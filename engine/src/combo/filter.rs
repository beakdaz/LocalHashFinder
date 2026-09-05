use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{Result, bail};

use crate::job_control::JobControl;

use super::cred::parse_email_pass;
use super::stream::stream_lines;

#[derive(Clone, Debug, Default)]
pub struct ComboFilterOptions {
    pub input: String,
    pub output: String,
    pub name_min: usize,
    pub name_max: usize,
    pub pass_min: usize,
    pub pass_max: usize,
    pub allow_name_specials: bool,
    pub allow_pass_specials: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ComboFilterResult {
    pub input_lines: u64,
    pub output_lines: u64,
}

#[derive(Clone, Debug, Default)]
pub struct EmailFilterOptions {
    pub input: String,
    pub output: String,
}

#[derive(Clone, Debug, Default)]
pub struct SplitNamePassOptions {
    pub input: String,
    pub names_file: String,
    pub passwords_file: String,
    pub use_local_part: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SplitNamePassResult {
    pub lines: u64,
}

pub fn filter_combos(opts: &ComboFilterOptions, control: Option<&JobControl>) -> Result<ComboFilterResult> {
    if opts.input.is_empty() || opts.output.is_empty() {
        bail!("input and output are required");
    }
    let mut out = BufWriter::with_capacity(256 * 1024, File::create(&opts.output)?);
    let mut input_lines = 0u64;
    let mut output_lines = 0u64;

    stream_lines(std::path::Path::new(&opts.input), control, |line| {
        input_lines += 1;
        let Some((email, pass)) = parse_email_pass(line) else {
            return Ok(());
        };
        let Some(at) = email.rfind('@') else {
            return Ok(());
        };
        let local = &email[..at];
        if !len_ok(local, opts.name_min, opts.name_max) || !len_ok(&pass, opts.pass_min, opts.pass_max) {
            return Ok(());
        }
        if !chars_ok(local, opts.allow_name_specials) || !chars_ok(&pass, opts.allow_pass_specials) {
            return Ok(());
        }
        writeln!(out, "{email}:{pass}")?;
        output_lines += 1;
        Ok(())
    })?;

    out.flush()?;
    Ok(ComboFilterResult {
        input_lines,
        output_lines,
    })
}

pub fn filter_email_combo(opts: &EmailFilterOptions, control: Option<&JobControl>) -> Result<ComboFilterResult> {
    filter_combos(
        &ComboFilterOptions {
            input: opts.input.clone(),
            output: opts.output.clone(),
            allow_name_specials: true,
            allow_pass_specials: true,
            ..Default::default()
        },
        control,
    )
}

pub fn split_name_pass(opts: &SplitNamePassOptions, control: Option<&JobControl>) -> Result<SplitNamePassResult> {
    if opts.input.is_empty() || opts.names_file.is_empty() || opts.passwords_file.is_empty() {
        bail!("input, names and passwords outputs are required");
    }
    let mut names = BufWriter::new(File::create(&opts.names_file)?);
    let mut passes = BufWriter::new(File::create(&opts.passwords_file)?);
    let mut lines = 0u64;

    stream_lines(std::path::Path::new(&opts.input), control, |line| {
        let Some((email, pass)) = parse_email_pass(line) else {
            return Ok(());
        };
        let name = if opts.use_local_part {
            email.rsplit_once('@').map(|(l, _)| l).unwrap_or(&email)
        } else {
            &email
        };
        writeln!(names, "{name}")?;
        writeln!(passes, "{pass}")?;
        lines += 1;
        Ok(())
    })?;

    names.flush()?;
    passes.flush()?;
    Ok(SplitNamePassResult { lines })
}

fn len_ok(s: &str, min: usize, max: usize) -> bool {
    let n = s.chars().count();
    if min > 0 && n < min {
        return false;
    }
    if max > 0 && n > max {
        return false;
    }
    true
}

fn chars_ok(s: &str, allow_specials: bool) -> bool {
    for c in s.chars() {
        if c.is_whitespace() {
            return false;
        }
        if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '+') {
            continue;
        }
        if allow_specials {
            continue;
        }
        return false;
    }
    true
}
