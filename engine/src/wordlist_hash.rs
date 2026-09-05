use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use rayon::prelude::*;
use sha1::{Digest, Sha1};

/// Lines read per batch — bounded RAM regardless of file size.
const BATCH_SIZE: usize = 50_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashAlgo {
    Md5,
    Sha1,
}

pub struct WordlistHashOptions {
    pub source: PathBuf,
    pub output: Option<PathBuf>,
    pub algos: Vec<HashAlgo>,
    pub threads: usize,
}

pub struct WordlistHashStats {
    pub lines_read: u64,
    pub lines_written: u64,
    pub elapsed_secs: f64,
}

pub fn hash_password_md5(password: &str) -> String {
    format!("{:x}", md5::compute(password.as_bytes()))
}

pub fn hash_password_sha1(password: &str) -> String {
    hex::encode(Sha1::digest(password.as_bytes()))
}

fn default_output(source: &Path, algo: HashAlgo) -> PathBuf {
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("wordlist");
    let parent = source.parent().unwrap_or_else(|| Path::new("."));
    let suffix = match algo {
        HashAlgo::Md5 => "_md5.txt",
        HashAlgo::Sha1 => "_sha1.txt",
    };
    parent.join(format!("{stem}{suffix}"))
}

fn is_skippable_line(trimmed: &str) -> bool {
    trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';')
}

fn hash_line(algo: HashAlgo, pass: &str) -> String {
    let hash = match algo {
        HashAlgo::Md5 => hash_password_md5(pass),
        HashAlgo::Sha1 => hash_password_sha1(pass),
    };
    format!("{hash}:{pass}")
}

fn process_batch(
    pool: &rayon::ThreadPool,
    batch: &[String],
    writers: &mut [(HashAlgo, BufWriter<File>)],
) -> Result<u64> {
    let mut written = 0u64;
    for (algo, writer) in writers.iter_mut() {
        let rows: Vec<String> = pool.install(|| {
            batch
                .par_iter()
                .map(|pass| hash_line(*algo, pass))
                .collect()
        });
        for row in rows {
            writeln!(writer, "{row}").context("write hash:pass line")?;
            written += 1;
        }
    }
    Ok(written)
}

fn stream_hash_pass_files(
    source: &Path,
    outputs: &[(HashAlgo, PathBuf)],
    threads: usize,
) -> Result<(u64, u64)> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .build()
        .context("thread pool")?;

    let mut writers: Vec<(HashAlgo, BufWriter<File>)> = outputs
        .iter()
        .map(|(algo, path)| {
            let file = File::create(path).with_context(|| format!("create {}", path.display()))?;
            Ok((*algo, BufWriter::new(file)))
        })
        .collect::<Result<_>>()?;

    let file = File::open(source).with_context(|| format!("open {}", source.display()))?;
    let reader = BufReader::new(file);

    let mut batch = Vec::with_capacity(BATCH_SIZE);
    let mut lines_read = 0u64;
    let mut lines_written = 0u64;

    for line in reader.lines() {
        let line = line.with_context(|| format!("read {}", source.display()))?;
        let trimmed = line.trim();
        if is_skippable_line(trimmed) {
            continue;
        }
        batch.push(trimmed.to_string());
        lines_read += 1;

        if batch.len() >= BATCH_SIZE {
            lines_written += process_batch(&pool, &batch, &mut writers)?;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        lines_written += process_batch(&pool, &batch, &mut writers)?;
    }

    for (_, writer) in &mut writers {
        writer.flush().context("flush hash:pass output")?;
    }

    Ok((lines_read, lines_written))
}

pub fn run_wordlist_hash(opts: WordlistHashOptions) -> Result<WordlistHashStats> {
    let started = Instant::now();
    let threads = opts.threads.max(1);

    let outputs: Vec<(HashAlgo, PathBuf)> = opts
        .algos
        .iter()
        .map(|algo| {
            let out = opts
                .output
                .clone()
                .filter(|_| opts.algos.len() == 1)
                .unwrap_or_else(|| default_output(&opts.source, *algo));
            (*algo, out)
        })
        .collect();

    let (lines_read, lines_written) = stream_hash_pass_files(&opts.source, &outputs, threads)?;

    for (algo, out) in &outputs {
        println!(
            "{}: {} lines -> {}",
            match algo {
                HashAlgo::Md5 => "MD5",
                HashAlgo::Sha1 => "SHA1",
            },
            lines_read,
            out.display()
        );
    }

    Ok(WordlistHashStats {
        lines_read,
        lines_written,
        elapsed_secs: started.elapsed().as_secs_f64(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_password123() {
        assert_eq!(
            hash_password_md5("password123"),
            "482c811da5d5b4bc6d497ffa98491e38"
        );
    }

    #[test]
    fn sha1_password123() {
        assert_eq!(
            hash_password_sha1("password123"),
            "cbfdac6008f9cab4083784cbd1874f76618d2a97"
        );
    }
}
