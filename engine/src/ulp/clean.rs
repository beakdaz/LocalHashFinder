use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::combo::stream_lines;
use crate::job_control::JobControl;

use super::cred::{parse_line, strip_url_protocol};
use super::extract::rebuild_without_protocol;
use super::input::resolve_input_files;
use super::swifty_data::{CAPTURE_PREFIXES, DOMAIN_BLACKLIST, WEAK_PASSWORDS};

pub struct CleanStats {
    pub input_lines: u64,
    pub output_lines: u64,
    pub output_file: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CleanOp {
    Dedupe,
    EmptyLines,
    Junk,
    Blacklist,
    EmptyChars,
    Weak,
    Protocols,
    Capture,
}

pub fn run_clean(
    op: CleanOp,
    input: &str,
    output: &str,
    control: Option<&JobControl>,
) -> Result<CleanStats> {
    let output = output.trim();
    if output.is_empty() {
        anyhow::bail!("output required");
    }
    let files = resolve_input_files(input)?;
    if files.is_empty() {
        anyhow::bail!("no input files");
    }

    let file = File::create(output).with_context(|| format!("create {output}"))?;
    let mut writer = BufWriter::with_capacity(256 * 1024, file);
    let mut input_lines = 0u64;
    let mut output_lines = 0u64;

    match op {
        CleanOp::Dedupe => {
            let mut seen = std::collections::HashSet::new();
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    let key = line.to_ascii_lowercase();
                    if seen.insert(key) {
                        writeln!(writer, "{line}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::EmptyLines => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    if !line.trim().is_empty() {
                        writeln!(writer, "{line}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::Junk => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    if is_printable_line(line) {
                        writeln!(writer, "{line}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::Blacklist => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    if !is_blacklisted_line(line) {
                        writeln!(writer, "{line}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::EmptyChars => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    let cleaned = strip_control_chars(line);
                    if !cleaned.trim().is_empty() {
                        writeln!(writer, "{cleaned}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::Weak => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    if !is_weak_credential(line) {
                        writeln!(writer, "{line}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::Protocols => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    if let Some(out) = strip_protocols_line(line) {
                        writeln!(writer, "{out}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
        CleanOp::Capture => {
            for path in &files {
                stream_lines(path, control, |line| {
                    input_lines += 1;
                    if !is_capture_line(line) {
                        writeln!(writer, "{line}")?;
                        output_lines += 1;
                    }
                    Ok(())
                })?;
            }
        }
    }

    writer.flush()?;
    let label = clean_label(op);
    Ok(CleanStats {
        input_lines,
        output_lines,
        output_file: output.to_string(),
        message: format!("{label}: {input_lines} → {output_lines} lines"),
    })
}

fn clean_label(op: CleanOp) -> &'static str {
    match op {
        CleanOp::Dedupe => "Dedupe",
        CleanOp::EmptyLines => "Remove empty",
        CleanOp::Junk => "Remove junk",
        CleanOp::Blacklist => "Remove blacklist",
        CleanOp::EmptyChars => "Remove empty chars",
        CleanOp::Weak => "Remove weak",
        CleanOp::Protocols => "Remove protocols",
        CleanOp::Capture => "Remove capture",
    }
}

fn is_printable_line(line: &str) -> bool {
    line.chars().all(|c| {
        c == '\t' || c == ' ' || (!c.is_control() && c.is_ascii())
    })
}

fn is_blacklisted_line(line: &str) -> bool {
    let low = line.to_ascii_lowercase();
    for domain in DOMAIN_BLACKLIST {
        if low.contains(domain) {
            return true;
        }
    }
    false
}

fn strip_control_chars(line: &str) -> String {
    line.chars()
        .filter(|c| !c.is_control() || *c == '\t')
        .collect()
}

fn is_weak_credential(line: &str) -> bool {
    let Some(c) = parse_line(line) else {
        return false;
    };
    let pass = c.pass.trim().to_ascii_lowercase();
    if pass.len() < 4 {
        return true;
    }
    WEAK_PASSWORDS.iter().any(|w| pass == *w)
}

fn strip_protocols_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(c) = parse_line(line) {
        if !c.url.trim().is_empty() {
            return Some(rebuild_without_protocol(&c));
        }
    }
    if let Some(i) = trimmed.find("://") {
        if let Some(colon) = trimmed[i..].find(':') {
            let rest = &trimmed[i + 3 + colon..];
            let host_part = strip_url_protocol(&trimmed[..i + 3 + colon]);
            return Some(format!("{host_part}{rest}"));
        }
    }
    Some(trimmed.to_string())
}

fn is_capture_line(line: &str) -> bool {
    let t = line.trim();
    if t.is_empty() {
        return false;
    }
    let low = t.to_ascii_lowercase();
    if low.contains("capture") {
        return true;
    }
    for prefix in CAPTURE_PREFIXES {
        if low.starts_with(prefix) {
            return true;
        }
    }
    if t.starts_with('[') && t.contains(']') && !t.contains(':') {
        return true;
    }
    if !t.contains(':') && t.len() > 180 {
        return true;
    }
    false
}

pub fn resolve_clean_inputs(input: &str) -> Result<Vec<PathBuf>> {
    resolve_input_files(input)
}
