use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::job_control::JobControl;

#[derive(Clone, Debug, Default)]
pub struct CompareOptions {
    pub file_a: PathBuf,
    pub file_b: PathBuf,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct CompareResult {
    pub lines_a: u64,
    pub lines_b: u64,
    pub only_a: u64,
    pub only_b: u64,
    pub both: u64,
    pub output_dir: PathBuf,
}

pub fn compare_lists(opts: &CompareOptions, control: Option<&JobControl>) -> Result<CompareResult> {
    if opts.file_a.as_os_str().is_empty() || opts.file_b.as_os_str().is_empty() {
        bail!("both input files are required");
    }
    std::fs::create_dir_all(&opts.output_dir)?;

    let set_a = load_set(&opts.file_a, control)?;
    let set_b = load_set(&opts.file_b, control)?;
    let lines_a = set_a.len() as u64;
    let lines_b = set_b.len() as u64;

    let only_a_path = opts.output_dir.join("only_a.txt");
    let only_b_path = opts.output_dir.join("only_b.txt");
    let both_path = opts.output_dir.join("both.txt");

    let mut fa = File::create(&only_a_path)?;
    let mut fb = File::create(&only_b_path)?;
    let mut fboth = File::create(&both_path)?;

    let mut only_a = 0u64;
    let mut both = 0u64;
    for line in &set_a {
        if set_b.contains(line) {
            writeln!(fboth, "{line}")?;
            both += 1;
        } else {
            writeln!(fa, "{line}")?;
            only_a += 1;
        }
    }
    let mut only_b = 0u64;
    for line in &set_b {
        if !set_a.contains(line) {
            writeln!(fb, "{line}")?;
            only_b += 1;
        }
    }

    Ok(CompareResult {
        lines_a,
        lines_b,
        only_a,
        only_b,
        both,
        output_dir: opts.output_dir.clone(),
    })
}

fn load_set(path: &Path, control: Option<&JobControl>) -> Result<HashSet<String>> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let reader = BufReader::with_capacity(256 * 1024, file);
    let mut set = HashSet::new();
    for line in reader.lines() {
        if crate::job_control::checkpoint(control) {
            bail!("stopped");
        }
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            set.insert(line.to_string());
        }
    }
    Ok(set)
}
