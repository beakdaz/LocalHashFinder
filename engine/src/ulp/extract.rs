use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};

use crate::combo::stream_lines;
use crate::job_control::JobControl;

use super::cred::{parse_line, Cred};
use super::input::resolve_input_files;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtractFormat {
    UrlLoginPass,
    LoginPass,
    UserPass,
}

pub struct ExtractStats {
    pub input_lines: u64,
    pub output_lines: u64,
    pub output_file: String,
    pub message: String,
}

pub fn extract_swifty(
    input: &str,
    output: &str,
    format: ExtractFormat,
    keywords: &[String],
    control: Option<&JobControl>,
) -> Result<ExtractStats> {
    let input = input.trim();
    let output = output.trim();
    if input.is_empty() {
        anyhow::bail!("input required");
    }
    if output.is_empty() {
        anyhow::bail!("output required");
    }

    let files = resolve_input_files(input)?;
    if files.is_empty() {
        anyhow::bail!("no input files found");
    }

    let file = File::create(output).with_context(|| format!("create {output}"))?;
    let mut writer = BufWriter::with_capacity(256 * 1024, file);
    let mut seen = HashSet::new();
    let mut input_lines = 0u64;
    let mut output_lines = 0u64;

    for path in &files {
        stream_lines(path, control, |line| {
            input_lines += 1;
            let row = swifty_transform(line, format);
            if row.is_empty() {
                return Ok(());
            }
            if !keywords.is_empty() && !line_matches_keywords(&row, keywords) {
                return Ok(());
            }
            if !seen.insert(row.clone()) {
                return Ok(());
            }
            writeln!(writer, "{row}")?;
            output_lines += 1;
            Ok(())
        })?;
    }

    writer.flush()?;
    let fmt = match format {
        ExtractFormat::UrlLoginPass => "url:login:pass",
        ExtractFormat::LoginPass => "login:pass",
        ExtractFormat::UserPass => "user:pass",
    };
    Ok(ExtractStats {
        input_lines,
        output_lines,
        output_file: output.to_string(),
        message: format!(
            "Extract {fmt}: {input_lines} → {output_lines} lines → {}",
            Path::new(output)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(output)
        ),
    })
}

fn swifty_transform(line: &str, format: ExtractFormat) -> String {
    let line = line.trim();
    if line.is_empty() {
        return String::new();
    }
    let Some(c) = parse_line(line) else {
        return String::new();
    };
    match format {
        ExtractFormat::UrlLoginPass => {
            if !c.url.trim().is_empty() {
                c.line()
            } else {
                String::new()
            }
        }
        ExtractFormat::LoginPass => {
            if !c.login.is_empty() && !c.pass.is_empty() {
                format!("{}:{}", c.login.trim(), c.pass.trim())
            } else {
                String::new()
            }
        }
        ExtractFormat::UserPass => {
            let login = c.login.trim();
            if login.is_empty() || login.contains('@') {
                String::new()
            } else if !c.pass.is_empty() {
                format!("{login}:{}", c.pass.trim())
            } else {
                String::new()
            }
        }
    }
}

fn line_matches_keywords(line: &str, keywords: &[String]) -> bool {
    let low = line.to_ascii_lowercase();
    keywords.iter().any(|k| {
        let k = k.trim().to_ascii_lowercase();
        !k.is_empty() && low.contains(&k)
    })
}

pub fn rebuild_without_protocol(c: &Cred) -> String {
    if c.url.trim().is_empty() {
        return c.line();
    }
    let url = super::cred::strip_url_protocol(&c.url);
    format!("{}:{}:{}", url, c.login.trim(), c.pass.trim())
}
