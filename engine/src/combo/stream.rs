use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};

use crate::job_control::{self, JobControl};

pub fn stream_lines<F>(path: &Path, control: Option<&JobControl>, mut on_line: F) -> Result<u64>
where
    F: FnMut(&str) -> Result<()>,
{
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let reader = BufReader::with_capacity(256 * 1024, file);
    let mut lines = 0u64;
    for line in reader.lines() {
        if job_control::checkpoint(control) {
            anyhow::bail!("stopped");
        }
        let line = line.with_context(|| format!("read {}", path.display()))?;
        on_line(&line)?;
        lines += 1;
    }
    Ok(lines)
}
