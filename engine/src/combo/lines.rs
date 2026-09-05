use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use regex::Regex;

use crate::job_control::JobControl;

use super::stream::stream_lines;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LineMode {
    #[default]
    Dedupe,
    Filter,
    Merge,
    Split,
}

#[derive(Clone, Debug, Default)]
pub struct LineToolOptions {
    pub mode: LineMode,
    pub input: PathBuf,
    pub output: PathBuf,
    pub filter: String,
    pub use_regex: bool,
    pub lines_per_file: usize,
}

#[derive(Clone, Debug, Default)]
pub struct LineToolResult {
    pub input_lines: u64,
    pub output_lines: u64,
    pub output_files: Vec<PathBuf>,
}

pub fn run_line_tool(opts: &LineToolOptions, control: Option<&JobControl>) -> Result<LineToolResult> {
    match opts.mode {
        LineMode::Dedupe => dedupe(opts, control),
        LineMode::Filter => filter_lines(opts, control),
        LineMode::Merge => merge_lines(opts, control),
        LineMode::Split => split_lines(opts, control),
    }
}

fn dedupe(opts: &LineToolOptions, control: Option<&JobControl>) -> Result<LineToolResult> {
    if opts.input.as_os_str().is_empty() || opts.output.as_os_str().is_empty() {
        bail!("input and output are required");
    }
    let mut seen = HashSet::new();
    let mut out = BufWriter::with_capacity(256 * 1024, File::create(&opts.output)?);
    let mut input_lines = 0u64;
    let mut output_lines = 0u64;

    stream_lines(&opts.input, control, |line| {
        input_lines += 1;
        let key = line.trim();
        if key.is_empty() || !seen.insert(key.to_string()) {
            return Ok(());
        }
        writeln!(out, "{key}")?;
        output_lines += 1;
        Ok(())
    })?;
    out.flush()?;
    Ok(LineToolResult {
        input_lines,
        output_lines,
        output_files: vec![opts.output.clone()],
    })
}

fn filter_lines(opts: &LineToolOptions, control: Option<&JobControl>) -> Result<LineToolResult> {
    if opts.filter.trim().is_empty() {
        bail!("filter text is required");
    }
    let re = if opts.use_regex {
        Some(Regex::new(opts.filter.trim())?)
    } else {
        None
    };
    let mut out = BufWriter::with_capacity(256 * 1024, File::create(&opts.output)?);
    let mut input_lines = 0u64;
    let mut output_lines = 0u64;

    stream_lines(&opts.input, control, |line| {
        input_lines += 1;
        let ok = if let Some(re) = &re {
            re.is_match(line)
        } else {
            line.to_lowercase().contains(&opts.filter.to_lowercase())
        };
        if ok {
            writeln!(out, "{}", line.trim())?;
            output_lines += 1;
        }
        Ok(())
    })?;
    out.flush()?;
    Ok(LineToolResult {
        input_lines,
        output_lines,
        output_files: vec![opts.output.clone()],
    })
}

fn merge_lines(opts: &LineToolOptions, control: Option<&JobControl>) -> Result<LineToolResult> {
    let mut out = BufWriter::with_capacity(256 * 1024, File::create(&opts.output)?);
    let mut input_lines = 0u64;
    stream_lines(&opts.input, control, |line| {
        input_lines += 1;
        writeln!(out, "{}", line.trim())?;
        Ok(())
    })?;
    out.flush()?;
    Ok(LineToolResult {
        input_lines,
        output_lines: input_lines,
        output_files: vec![opts.output.clone()],
    })
}

fn split_lines(opts: &LineToolOptions, control: Option<&JobControl>) -> Result<LineToolResult> {
    let per = if opts.lines_per_file == 0 {
        100_000
    } else {
        opts.lines_per_file
    };
    let stem = opts
        .output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("part");
    let parent = opts.output.parent().unwrap_or(Path::new("."));
    let mut part = 1usize;
    let mut in_part = 0usize;
    let mut input_lines = 0u64;
    let mut output_files = Vec::new();
    let mut writer: Option<BufWriter<File>> = None;

    let open_part = |part: usize, parent: &Path, stem: &str| -> Result<BufWriter<File>> {
        let path = parent.join(format!("{stem}_part{part:03}.txt"));
        Ok(BufWriter::with_capacity(256 * 1024, File::create(&path)?))
    };

    stream_lines(&opts.input, control, |line| {
        input_lines += 1;
        if writer.is_none() || in_part >= per {
            if let Some(mut w) = writer.take() {
                w.flush()?;
            }
            let path = parent.join(format!("{stem}_part{part:03}.txt"));
            output_files.push(path);
            writer = Some(open_part(part, parent, stem)?);
            part += 1;
            in_part = 0;
        }
        if let Some(w) = writer.as_mut() {
            writeln!(w, "{}", line.trim())?;
            in_part += 1;
        }
        Ok(())
    })?;
    if let Some(mut w) = writer.take() {
        w.flush()?;
    }
    Ok(LineToolResult {
        input_lines,
        output_lines: input_lines,
        output_files,
    })
}
