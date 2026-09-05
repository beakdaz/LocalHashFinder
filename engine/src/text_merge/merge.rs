use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use rayon::ThreadPoolBuilder;
use tempfile::TempDir;

use super::dedupe::{dedupe_buckets, partition_into_buckets};
use super::stats::MergeStats;

const TXT_EXT: &str = "txt";

#[derive(Clone, Debug)]
pub struct MergeOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub recursive: bool,
    pub min_len: usize,
    pub case_insensitive: bool,
    pub threads: usize,
}

pub fn run_merge(opts: &MergeOptions) -> Result<MergeStats> {
    if !opts.input.is_dir() {
        bail!("input must be an existing folder: {}", opts.input.display());
    }

    let threads = if opts.threads == 0 {
        rayon::current_num_threads()
    } else {
        opts.threads
    };
    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .ok();

    let files = collect_txt_files(&opts.input, opts.recursive, &opts.output)?;
    if files.is_empty() {
        bail!("no .txt files found in {}", opts.input.display());
    }

    eprintln!(
        "[TextMerger] {} file(s), threads={}, min_len={}, case_insensitive={}",
        files.len(),
        threads,
        opts.min_len,
        opts.case_insensitive,
    );

    if let Some(parent) = opts.output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
    }
    if opts.output.exists() {
        fs::remove_file(&opts.output)
            .with_context(|| format!("remove old {}", opts.output.display()))?;
    }
    File::create(&opts.output).with_context(|| format!("create {}", opts.output.display()))?;

    let tmp = TempDir::new().context("create temp dir for buckets")?;
    let bucket_dir = tmp.path().join("buckets");
    fs::create_dir_all(&bucket_dir)?;

    let mut stats = MergeStats::new(opts.min_len, files.len() as u64);

    for (i, file) in files.iter().enumerate() {
        eprintln!(
            "[TextMerger] ({}/{}) {}",
            i + 1,
            files.len(),
            file.display()
        );
        partition_into_buckets(file, &bucket_dir, opts.case_insensitive, &mut stats)?;
    }

    stats.report_progress(true);
    dedupe_buckets(&bucket_dir, &opts.output, opts.case_insensitive, &mut stats)?;

    stats.lines_written = count_lines(&opts.output)?;
    stats.print_final(&opts.output.display().to_string());
    Ok(stats)
}

fn count_lines(path: &Path) -> Result<u64> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(256 * 1024, file);
    Ok(reader.lines().count() as u64)
}

fn collect_txt_files(input: &Path, recursive: bool, output: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_txt_files_inner(input, recursive, output, &mut files)?;
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_txt_files_inner(
    dir: &Path,
    recursive: bool,
    output: &Path,
    out: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if recursive {
                collect_txt_files_inner(&path, recursive, output, out)?;
            }
            continue;
        }
        if !is_txt_file(&path) {
            continue;
        }
        if paths_same_file(&path, output) {
            continue;
        }
        out.push(path);
    }
    Ok(())
}

fn is_txt_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case(TXT_EXT))
        .unwrap_or(false)
}

fn paths_same_file(a: &Path, b: &Path) -> bool {
    fs::canonicalize(a)
        .ok()
        .zip(fs::canonicalize(b).ok())
        .map(|(a, b)| a == b)
        .unwrap_or(false)
}
