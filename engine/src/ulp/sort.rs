use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::combo::stream_lines;
use crate::job_control::JobControl;

use super::buckets::{Buckets, SortStats};
use super::input::resolve_input_files;

pub fn sort_ulp(
    input: &str,
    output_dir: &str,
    control: Option<&JobControl>,
) -> Result<SortStats> {
    let out = Path::new(output_dir.trim());
    if output_dir.trim().is_empty() {
        anyhow::bail!("output directory required");
    }
    fs::create_dir_all(out).context("create output dir")?;

    let files = resolve_input_files(input)?;
    if files.is_empty() {
        anyhow::bail!("no input files found");
    }

    let buckets = Buckets::new();
    let mut lines_read = 0u64;
    for file in &files {
        lines_read += stream_lines(file, control, |line| {
            buckets.ingest_line(line);
            Ok(())
        })?;
    }

    write_buckets(out, &buckets)?;
    let mut stats = SortStats::from_buckets(&buckets, files.len(), output_dir.trim());
    stats.lines_read = lines_read;
    Ok(stats)
}

fn write_buckets(out_dir: &Path, buckets: &Buckets) -> Result<()> {
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    buckets.each(|name, line| {
        files.entry(name.to_string()).or_default().push(line.to_string());
    });
    for (name, lines) in files {
        let path = out_dir.join(&name);
        let mut writer = BufWriter::with_capacity(
            256 * 1024,
            File::create(&path).with_context(|| format!("create {}", path.display()))?,
        );
        for line in lines {
            writeln!(writer, "{line}")?;
        }
        writer.flush()?;
    }
    Ok(())
}

pub fn resolve_input_files_only(input: &str) -> Result<Vec<PathBuf>> {
    resolve_input_files(input)
}
