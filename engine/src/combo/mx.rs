use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

use anyhow::{Result, bail};

use crate::job_control::JobControl;

use super::cred::{email_domain, parse_email_pass};
use super::stream::stream_lines;

#[derive(Clone, Debug, Default)]
pub struct MxCheckOptions {
    pub input: String,
    pub output_dir: String,
}

#[derive(Clone, Debug, Default)]
pub struct MxCheckResult {
    pub domains: usize,
    pub valid: u64,
    pub bad: u64,
    pub unknown: u64,
}

pub fn check_mx(opts: &MxCheckOptions, control: Option<&JobControl>) -> Result<MxCheckResult> {
    if opts.input.is_empty() || opts.output_dir.is_empty() {
        bail!("input and output dir are required");
    }
    std::fs::create_dir_all(&opts.output_dir)?;

    let mut valid_f = BufWriter::new(File::create(format!("{}/mx_valid.txt", opts.output_dir))?);
    let mut bad_f = BufWriter::new(File::create(format!("{}/mx_bad.txt", opts.output_dir))?);
    let mut unknown_f = BufWriter::new(File::create(format!("{}/mx_unknown.txt", opts.output_dir))?);

    let mut cache: std::collections::HashMap<String, i8> = std::collections::HashMap::new();
    let mut valid = 0u64;
    let mut bad = 0u64;
    let mut unknown = 0u64;

    stream_lines(std::path::Path::new(&opts.input), control, |line| {
        let Some((email, _)) = parse_email_pass(line) else {
            return Ok(());
        };
        let Some(domain) = email_domain(&email) else {
            return Ok(());
        };
        let status = *cache.entry(domain.clone()).or_insert_with(|| lookup_mx(&domain));
        let trimmed = line.trim();
        match status {
            1 => {
                writeln!(valid_f, "{trimmed}")?;
                valid += 1;
            }
            -1 => {
                writeln!(bad_f, "{trimmed}")?;
                bad += 1;
            }
            _ => {
                writeln!(unknown_f, "{trimmed}")?;
                unknown += 1;
            }
        }
        Ok(())
    })?;

    valid_f.flush()?;
    bad_f.flush()?;
    unknown_f.flush()?;

    Ok(MxCheckResult {
        domains: cache.len(),
        valid,
        bad,
        unknown,
    })
}

fn lookup_mx(domain: &str) -> i8 {
    #[cfg(windows)]
    {
        if let Ok(out) = Command::new("nslookup")
            .args(["-type=mx", domain])
            .output()
        {
            let text = String::from_utf8_lossy(&out.stdout).to_lowercase();
            if text.contains("mail exchanger") || text.contains("mx preference") {
                return 1;
            }
            if text.contains("non-existent domain") || text.contains("nxdomain") {
                return -1;
            }
        }
        if let Ok(out) = Command::new("nslookup").arg(domain).output() {
            let text = String::from_utf8_lossy(&out.stdout).to_lowercase();
            if text.contains("address:") || text.contains("addresses:") {
                return 1;
            }
        }
        0
    }
    #[cfg(not(windows))]
    {
        if let Ok(out) = Command::new("dig").args(["+short", "MX", domain]).output() {
            if !String::from_utf8_lossy(&out.stdout).trim().is_empty() {
                return 1;
            }
        }
        if let Ok(out) = Command::new("dig").args(["+short", "A", domain]).output() {
            if !String::from_utf8_lossy(&out.stdout).trim().is_empty() {
                return 1;
            }
        }
        0
    }
}
