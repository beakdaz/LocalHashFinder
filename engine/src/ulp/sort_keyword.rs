use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::combo::stream_lines;
use crate::job_control::JobControl;

use super::archive::materialize_input;
use super::cred::parse_line;

#[derive(Clone, Debug, Default)]
pub struct SortKeywordStats {
    pub files: usize,
    pub lines_read: u64,
    pub buckets: usize,
    pub matched_lines: u64,
    pub output_dir: String,
    pub message: String,
}

pub fn sort_by_keyword(
    input: &str,
    output_dir: &str,
    keywords: &[String],
    control: Option<&JobControl>,
) -> Result<SortKeywordStats> {
    let out = Path::new(output_dir.trim());
    if output_dir.trim().is_empty() {
        bail!("output directory required");
    }
    let keywords = normalize_keywords(keywords);
    if keywords.is_empty() {
        bail!("at least one keyword is required");
    }
    fs::create_dir_all(out).context("create output dir")?;

    let (files, _temp) = materialize_input(input)?;
    if files.is_empty() {
        bail!("no input files found");
    }

    let mut buckets: HashMap<String, Vec<String>> = HashMap::new();
    for kw in &keywords {
        buckets.insert(kw.clone(), Vec::new());
    }
    let mut lines_read = 0u64;
    let mut matched_lines = 0u64;

    for file in &files {
        stream_lines(file, control, |line| {
            lines_read += 1;
            let mut hit = false;
            for kw in &keywords {
                if line_matches_keyword(line, kw) {
                    buckets.get_mut(kw).unwrap().push(line.to_string());
                    hit = true;
                }
            }
            if hit {
                matched_lines += 1;
            }
            Ok(())
        })?;
    }

    for (kw, lines) in &buckets {
        let fname = safe_filename(kw);
        let path = out.join(format!("{fname}.txt"));
        let mut w = BufWriter::with_capacity(256 * 1024, File::create(&path)?);
        for line in lines {
            writeln!(w, "{line}")?;
        }
        w.flush()?;
    }

    let msg = format!(
        "Sort keyword: {} lines, {} matched → {} buckets → {}",
        lines_read,
        matched_lines,
        keywords.len(),
        out.display()
    );
    Ok(SortKeywordStats {
        files: files.len(),
        lines_read,
        buckets: keywords.len(),
        matched_lines,
        output_dir: output_dir.to_string(),
        message: msg,
    })
}

pub fn search_ulp(
    input: &str,
    output: &str,
    keywords: &[String],
    control: Option<&JobControl>,
) -> Result<SortKeywordStats> {
    let out = output.trim();
    if out.is_empty() {
        bail!("output file required");
    }
    let keywords = normalize_keywords(keywords);
    if keywords.is_empty() {
        bail!("at least one keyword is required");
    }

    let (files, _temp) = materialize_input(input)?;
    if files.is_empty() {
        bail!("no input files found");
    }

    let mut writer = BufWriter::with_capacity(256 * 1024, File::create(out)?);
    let mut lines_read = 0u64;
    let mut matched_lines = 0u64;

    for file in &files {
        stream_lines(file, control, |line| {
            lines_read += 1;
            if keywords.iter().any(|kw| line_matches_keyword(line, kw)) {
                writeln!(writer, "{line}")?;
                matched_lines += 1;
            }
            Ok(())
        })?;
    }
    writer.flush()?;

    Ok(SortKeywordStats {
        files: files.len(),
        lines_read,
        buckets: 1,
        matched_lines,
        output_dir: out.to_string(),
        message: format!("Search: {matched_lines} / {lines_read} lines → {out}"),
    })
}

fn normalize_keywords(keywords: &[String]) -> Vec<String> {
    keywords
        .iter()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
        .collect()
}

fn line_matches_keyword(line: &str, keyword: &str) -> bool {
    let line_low = line.to_ascii_lowercase();
    let kw = keyword.to_ascii_lowercase();
    if line_low.contains(&kw) {
        return true;
    }
    if let Some(c) = parse_line(line) {
        let url = c.url.to_ascii_lowercase();
        let login = c.login.to_ascii_lowercase();
        let pass = c.pass.to_ascii_lowercase();
        return url.contains(&kw) || login.contains(&kw) || pass.contains(&kw);
    }
    false
}

fn safe_filename(keyword: &str) -> String {
    let mut s: String = keyword
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() {
        s = "keyword".to_string();
    }
    if s.len() > 80 {
        s.truncate(80);
    }
    s
}
