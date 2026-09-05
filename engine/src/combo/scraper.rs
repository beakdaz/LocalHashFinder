use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use regex::Regex;
use std::sync::LazyLock;

use crate::job_control::JobControl;

use super::cred::{email_domain, parse_email_pass};
use super::stream::stream_lines;

static EMAIL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)[a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,}").unwrap());
static COMBO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)([a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,})\s*[:;]\s*(\S+)").unwrap()
});

#[derive(Clone, Debug, Default)]
pub struct ScraperOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub include_sql: bool,
    pub include_json: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ScraperResult {
    pub emails: u64,
    pub combos: u64,
}

pub fn scrape_credentials(opts: &ScraperOptions, control: Option<&JobControl>) -> Result<ScraperResult> {
    if opts.input.as_os_str().is_empty() || opts.output.as_os_str().is_empty() {
        bail!("input and output are required");
    }
    let files = collect_files(&opts.input, opts.include_sql, opts.include_json)?;
    if files.is_empty() {
        bail!("no input files found");
    }

    let mut out = BufWriter::with_capacity(256 * 1024, File::create(&opts.output)?);
    let mut seen = HashSet::new();
    let mut emails = 0u64;
    let mut combos = 0u64;

    for file in files {
        if crate::job_control::checkpoint(control) {
            bail!("stopped");
        }
        if file.extension().and_then(|e| e.to_str()) == Some("json") {
            let text = fs::read_to_string(&file)?;
            scrape_text(&text, &mut out, &mut seen, &mut emails, &mut combos)?;
        } else {
            stream_lines(&file, control, |line| {
                scrape_text(line, &mut out, &mut seen, &mut emails, &mut combos)
            })?;
        }
    }
    out.flush()?;
    Ok(ScraperResult { emails, combos })
}

fn scrape_text(
    text: &str,
    out: &mut BufWriter<File>,
    seen: &mut HashSet<String>,
    emails: &mut u64,
    combos: &mut u64,
) -> Result<()> {
    for cap in COMBO_RE.captures_iter(text) {
        let entry = format!("{}:{}", &cap[1], &cap[2]).to_lowercase();
        if seen.insert(entry.clone()) {
            writeln!(out, "{entry}")?;
            *combos += 1;
        }
    }
    for m in EMAIL_RE.find_iter(text) {
        let em = m.as_str().to_lowercase();
        if seen.insert(em.clone()) {
            writeln!(out, "{em}")?;
            *emails += 1;
        }
    }
    Ok(())
}

fn collect_files(path: &Path, sql: bool, json: bool) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            continue;
        }
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        let ok = matches!(ext, "txt" | "log" | "lst" | "ulp")
            || (sql && ext == "sql")
            || (json && ext == "json");
        if ok {
            files.push(p);
        }
    }
    Ok(files)
}

#[derive(Clone, Debug, Default)]
pub struct AnalyzeOptions {
    pub input: PathBuf,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct AnalyzeRow {
    pub domain: String,
    pub count: u64,
}

#[derive(Clone, Debug, Default)]
pub struct AnalyzeResult {
    pub lines: u64,
    pub domains: usize,
    pub top: Vec<AnalyzeRow>,
}

pub fn analyze_provider(opts: &AnalyzeOptions, control: Option<&JobControl>) -> Result<AnalyzeResult> {
    if opts.input.as_os_str().is_empty() || opts.output_dir.as_os_str().is_empty() {
        bail!("input and output dir are required");
    }
    fs::create_dir_all(&opts.output_dir)?;
    let by_domain = opts.output_dir.join("by_domain");
    fs::create_dir_all(&by_domain)?;

    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut buckets: HashMap<String, Vec<String>> = HashMap::new();
    let mut lines = 0u64;

    stream_lines(&opts.input, control, |line| {
        lines += 1;
        let email = parse_email_pass(line)
            .map(|(e, _)| e)
            .or_else(|| EMAIL_RE.find(line).map(|m| m.as_str().to_string()));
        let Some(email) = email else {
            return Ok(());
        };
        let Some(domain) = email_domain(&email) else {
            return Ok(());
        };
        *counts.entry(domain.clone()).or_default() += 1;
        let bucket = buckets.entry(domain).or_default();
        if bucket.len() < 5000 {
            bucket.push(line.trim().to_string());
        }
        Ok(())
    })?;

    for (domain, lines) in &buckets {
        let safe = domain.replace('/', "_");
        let path = by_domain.join(format!("{safe}.txt"));
        let mut f = BufWriter::new(File::create(path)?);
        for ln in lines {
            writeln!(f, "{ln}")?;
        }
        f.flush()?;
    }

    let domain_count = counts.len();
    let mut top: Vec<AnalyzeRow> = counts
        .into_iter()
        .map(|(domain, count)| AnalyzeRow { domain, count })
        .collect();
    top.sort_by(|a, b| b.count.cmp(&a.count));
    top.truncate(20);

    Ok(AnalyzeResult {
        lines,
        domains: domain_count,
        top,
    })
}
