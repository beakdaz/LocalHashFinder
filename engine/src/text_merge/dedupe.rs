//! Hash-partitioned dedupe — bounded RAM for GB-scale wordlists.

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rayon::prelude::*;

use super::filter::strip_bom;
use super::stats::MergeStats;

const BUCKET_COUNT: usize = 256;
const READ_BUF: usize = 256 * 1024;
const WRITE_BUF: usize = 256 * 1024;

fn bucket_id(line: &str, case_insensitive: bool) -> u8 {
    use std::hash::{Hash, Hasher};
    let key = if case_insensitive {
        line.to_ascii_lowercase()
    } else {
        line.to_string()
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() % BUCKET_COUNT as u64) as u8
}

fn dedupe_key(line: &str, case_insensitive: bool) -> String {
    if case_insensitive {
        line.to_ascii_lowercase()
    } else {
        line.to_string()
    }
}

/// Stream `source`, filter garbage, append clean lines into hash bucket files.
pub fn partition_into_buckets(
    source: &Path,
    bucket_dir: &Path,
    case_insensitive: bool,
    stats: &mut MergeStats,
) -> Result<()> {
    let file = File::open(source).with_context(|| format!("open {}", source.display()))?;
    let reader = BufReader::with_capacity(READ_BUF, file);

    let mut writers: Vec<BufWriter<File>> = (0..BUCKET_COUNT)
        .map(|i| {
            let path = bucket_dir.join(format!("bucket_{i:03}.txt"));
            Ok(BufWriter::with_capacity(
                WRITE_BUF,
                File::options().create(true).append(true).open(path)?,
            ))
        })
        .collect::<Result<_>>()?;

    for line in reader.lines() {
        let line = line.with_context(|| format!("read {}", source.display()))?;
        stats.lines_read += 1;
        let line = strip_bom(&line);
        let trimmed = line.trim();
        if let Some(kind) = super::filter::classify_garbage(trimmed, stats.min_len) {
            stats.record_garbage(kind);
            continue;
        }
        let id = bucket_id(trimmed, case_insensitive) as usize;
        writeln!(writers[id], "{trimmed}")?;
        stats.lines_kept += 1;

        if stats.lines_read.is_multiple_of(500_000) {
            stats.report_progress(false);
        }
    }

    for mut w in writers {
        w.flush()?;
    }
    Ok(())
}

/// Dedupe each bucket in parallel, append unique lines to `output`.
pub fn dedupe_buckets(
    bucket_dir: &Path,
    output: &Path,
    case_insensitive: bool,
    stats: &mut MergeStats,
) -> Result<()> {
    let bucket_paths: Vec<PathBuf> = (0..BUCKET_COUNT)
        .map(|i| bucket_dir.join(format!("bucket_{i:03}.txt")))
        .collect();

    let dup_counts: Vec<u64> = bucket_paths
        .par_iter()
        .map(|bucket_path| {
            if !bucket_path.exists() {
                return Ok(0u64);
            }
            if fs::metadata(bucket_path)?.len() == 0 {
                return Ok(0u64);
            }
            dedupe_single_bucket(bucket_path, output, case_insensitive)
        })
        .collect::<Result<Vec<_>>>()?;

    stats.duplicates += dup_counts.iter().sum::<u64>();
    Ok(())
}

fn dedupe_single_bucket(
    bucket_path: &Path,
    output: &Path,
    case_insensitive: bool,
) -> Result<u64> {
    let file = File::open(bucket_path)?;
    let reader = BufReader::with_capacity(READ_BUF, file);
    let mut seen = HashSet::new();
    let mut dupes = 0u64;
    let mut unique_lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let key = dedupe_key(trimmed, case_insensitive);
        if !seen.insert(key) {
            dupes += 1;
            continue;
        }
        unique_lines.push(trimmed.to_string());
    }

    if unique_lines.is_empty() {
        return Ok(dupes);
    }

    let mut out = BufWriter::with_capacity(
        WRITE_BUF,
        File::options().create(true).append(true).open(output)?,
    );
    for line in unique_lines {
        writeln!(out, "{line}")?;
    }
    out.flush()?;
    Ok(dupes)
}
