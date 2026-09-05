use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use heed::Env;

use crate::db::HashDb;
use crate::job_control::JobControl;
use crate::parser::{is_trash_password, parse_input_line};

#[derive(Clone, Default)]
pub struct Progress {
    pub processed: u64,
    pub total: u64,
    pub found: u64,
    pub nohash: u64,
    pub bad: u64,
    pub trash: u64,
    pub elapsed_ms: u128,
    pub file: String,
    pub done: bool,
    pub stopped: bool,
    pub good_path: String,
    pub nohash_path: String,
    pub trash_path: String,
}

pub fn output_paths(input: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let dir = input.parent().unwrap_or_else(|| Path::new("."));
    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    (
        dir.join(format!("{stem}_good.txt")),
        dir.join(format!("{stem}_nohash.txt")),
        dir.join(format!("{stem}_bad.txt")),
        dir.join(format!("{stem}_trash.txt")),
    )
}

fn count_lines(path: &Path) -> Result<u64> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::with_capacity(8 * 1024 * 1024, file);
    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .count() as u64)
}

fn lookup_batch(env: &Env, rows: &[crate::parser::InputRow]) -> Vec<(Option<String>, bool)> {
    let Ok(txn) = env.read_txn() else {
        tracing::error!("LMDB read_txn failed for batch");
        return rows
            .iter()
            .map(|row| {
                if row.bad || row.hash.is_empty() {
                    (None, true)
                } else {
                    (None, false)
                }
            })
            .collect();
    };
    let Ok(db) = HashDb::open_db(env, &txn) else {
        tracing::error!("LMDB open_db failed for batch");
        return rows
            .iter()
            .map(|row| (None, !row.bad && !row.hash.is_empty()))
            .collect();
    };

    rows.iter()
        .map(|row| {
            if row.bad || row.hash.is_empty() {
                return (None, true);
            }
            let pass = HashDb::lookup_in_txn(&db, &txn, &row.hash)
                .unwrap_or_else(|e| {
                    tracing::warn!(hash = %row.hash, error = %e, "lookup failed");
                    None
                });
            (pass, false)
        })
        .collect()
}

fn format_good_line(row: &crate::parser::InputRow, pass: &str) -> String {
    if row.prefix.is_empty() {
        format!("{}:{pass}", row.hash)
    } else {
        format!("{}{pass}", row.prefix)
    }
}

pub fn process_file(
    db: Arc<HashDb>,
    input: PathBuf,
    threads: usize,
    control: Arc<JobControl>,
    mut on_progress: impl FnMut(Progress) + Send,
) -> Result<()> {
    let (good_path, nohash_path, bad_path, trash_path) = output_paths(&input);
    let _ = File::create(&good_path);
    let _ = File::create(&nohash_path);
    let _ = File::create(&bad_path);
    let _ = File::create(&trash_path);

    let total = count_lines(&input)?;
    let started = Instant::now();
    let file_name = input.file_name().unwrap().to_string_lossy().into_owned();
    let thread_count = threads.clamp(1, 512);

    let env = db
        .env()
        .ok_or_else(|| anyhow::anyhow!("LMDB not open — выберите базу и нажмите «Открыть»"))?;

    tracing::info!(
        file = %input.display(),
        total,
        batch = thread_count,
        "process start"
    );

    let file = File::open(&input)?;
    let reader = std::io::BufReader::with_capacity(16 * 1024 * 1024, file);

    let mut batch = Vec::with_capacity(thread_count);
    let mut processed: u64 = 0;
    let mut found: u64 = 0;
    let mut nohash: u64 = 0;
    let mut bad: u64 = 0;
    let mut trash: u64 = 0;

    let mut good_w = BufWriter::with_capacity(8 * 1024 * 1024, OpenOptions::new().append(true).open(&good_path)?);
    let mut nohash_w =
        BufWriter::with_capacity(8 * 1024 * 1024, OpenOptions::new().append(true).open(&nohash_path)?);
    let mut bad_w =
        BufWriter::with_capacity(8 * 1024 * 1024, OpenOptions::new().append(true).open(&bad_path)?);
    let mut trash_w =
        BufWriter::with_capacity(8 * 1024 * 1024, OpenOptions::new().append(true).open(&trash_path)?);

    let flush = |rows: Vec<crate::parser::InputRow>,
                 env: &Env,
                 found: &mut u64,
                 nohash: &mut u64,
                 bad: &mut u64,
                 trash: &mut u64,
                 good_w: &mut BufWriter<File>,
                 nohash_w: &mut BufWriter<File>,
                 bad_w: &mut BufWriter<File>,
                 trash_w: &mut BufWriter<File>|
     -> Result<()> {
        let trash_rows: Vec<_> = rows.iter().filter(|r| r.trash).cloned().collect();
        let lookup_rows: Vec<_> = rows.into_iter().filter(|r| !r.trash).collect();

        for row in trash_rows {
            *trash += 1;
            writeln!(trash_w, "{}", row.raw)?;
        }

        let results = lookup_batch(env, &lookup_rows);

        for (i, (pass, is_bad)) in results.into_iter().enumerate() {
            let row = &lookup_rows[i];
            if is_bad {
                *bad += 1;
                writeln!(bad_w, "{}", row.raw)?;
                continue;
            }
            if let Some(p) = pass {
                let line = format_good_line(row, &p);
                if is_trash_password(&p) {
                    *trash += 1;
                    writeln!(trash_w, "{line}")?;
                } else {
                    *found += 1;
                    writeln!(good_w, "{line}")?;
                }
            } else {
                *nohash += 1;
                if !row.prefix.is_empty() {
                    writeln!(nohash_w, "{}NULL", row.prefix)?;
                } else {
                    writeln!(nohash_w, "{}:NULL", row.raw)?;
                }
            }
        }
        Ok(())
    };

    for line in reader.lines() {
        if control.checkpoint() {
            tracing::info!("process stopped by user");
            break;
        }
        let line = line?;
        let Some(row) = parse_input_line(&line) else {
            continue;
        };
        batch.push(row);
        if batch.len() >= thread_count {
            let chunk: Vec<_> = batch.drain(..).collect();
            let n = chunk.len() as u64;
            flush(
                chunk,
                &env,
                &mut found,
                &mut nohash,
                &mut bad,
                &mut trash,
                &mut good_w,
                &mut nohash_w,
                &mut bad_w,
                &mut trash_w,
            )?;
            processed += n;
            if processed.is_multiple_of(100_000) {
                tracing::info!(
                    processed,
                    total,
                    found,
                    nohash,
                    bad,
                    trash,
                    "progress"
                );
            }
            on_progress(Progress {
                processed,
                total,
                found,
                nohash,
                bad,
                trash,
                elapsed_ms: started.elapsed().as_millis(),
                file: file_name.clone(),
                done: false,
                stopped: false,
                good_path: good_path.display().to_string(),
                nohash_path: nohash_path.display().to_string(),
                trash_path: trash_path.display().to_string(),
            });
        }
    }

    if !batch.is_empty() && !control.is_stopped() {
        let n = batch.len() as u64;
        flush(
            batch,
            &env,
            &mut found,
            &mut nohash,
            &mut bad,
            &mut trash,
            &mut good_w,
            &mut nohash_w,
            &mut bad_w,
            &mut trash_w,
        )?;
        processed += n;
    }

    good_w.flush()?;
    nohash_w.flush()?;
    bad_w.flush()?;
    trash_w.flush()?;

    tracing::info!(
        processed,
        found,
        nohash,
        bad,
        trash,
        good = %good_path.display(),
        nohash_out = %nohash_path.display(),
        trash_out = %trash_path.display(),
        "process done"
    );

    on_progress(Progress {
        processed,
        total,
        found,
        nohash,
        bad,
        trash,
        elapsed_ms: started.elapsed().as_millis(),
        file: file_name,
        done: true,
        stopped: control.is_stopped(),
        good_path: good_path.display().to_string(),
        nohash_path: nohash_path.display().to_string(),
        trash_path: trash_path.display().to_string(),
    });

    Ok(())
}
