use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};

use crate::combo::stream_lines;
use crate::job_control::JobControl;

use super::archive::materialize_input;
use super::cred::{email_domain, is_email_login, parse_line, Cred};

#[derive(Clone, Debug, Default)]
pub struct SortCountryStats {
    pub files: usize,
    pub lines_read: u64,
    pub tld_buckets: usize,
    pub domain_buckets: usize,
    pub output_dir: String,
    pub message: String,
}

pub fn sort_by_country(
    input: &str,
    output_dir: &str,
    control: Option<&JobControl>,
) -> Result<SortCountryStats> {
    let out = Path::new(output_dir.trim());
    if output_dir.trim().is_empty() {
        anyhow::bail!("output directory required");
    }
    fs::create_dir_all(out).context("create output dir")?;

    let tld_dir = out.join("by_tld");
    let domain_dir = out.join("by_domain");
    fs::create_dir_all(&tld_dir)?;
    fs::create_dir_all(&domain_dir)?;

    let (files, _temp) = materialize_input(input)?;
    if files.is_empty() {
        anyhow::bail!("no input files found");
    }

    let mut by_tld: HashMap<String, Vec<String>> = HashMap::new();
    let mut by_domain: HashMap<String, Vec<String>> = HashMap::new();
    let mut lines_read = 0u64;

    for file in &files {
        stream_lines(file, control, |line| {
            lines_read += 1;
            if let Some((domain, tld)) = line_country_keys(line) {
                by_domain
                    .entry(domain.clone())
                    .or_default()
                    .push(line.to_string());
                by_tld.entry(tld).or_default().push(line.to_string());
            }
            Ok(())
        })?;
    }

    write_bucket_dir(&tld_dir, &by_tld)?;
    write_bucket_dir(&domain_dir, &by_domain)?;

    let summary_path = out.join("summary.txt");
    write_summary(&summary_path, lines_read, &by_tld, &by_domain)?;

    let msg = format!(
        "Sort country: {} lines → {} TLDs, {} domains → {}",
        lines_read,
        by_tld.len(),
        by_domain.len(),
        out.display()
    );
    Ok(SortCountryStats {
        files: files.len(),
        lines_read,
        tld_buckets: by_tld.len(),
        domain_buckets: by_domain.len(),
        output_dir: output_dir.to_string(),
        message: msg,
    })
}

fn line_country_keys(line: &str) -> Option<(String, String)> {
    let cred = parse_line(line)?;
    let domain = email_domain_from_cred(&cred)?;
    let tld = tld_of(&domain);
    Some((domain, tld))
}

fn email_domain_from_cred(c: &Cred) -> Option<String> {
    let login = c.login.trim();
    if is_email_login(login) {
        return email_domain(login);
    }
    if !c.url.trim().is_empty() {
        let host = super::cred::strip_url_protocol(&c.url);
        let host = host.split(['/', ':', '?', '#']).next()?.trim().to_lowercase();
        if host.contains('.') {
            return Some(host);
        }
    }
    None
}

pub fn tld_of(domain: &str) -> String {
    let domain = domain.trim().to_lowercase();
    let parts: Vec<&str> = domain.split('.').filter(|p| !p.is_empty()).collect();
    if parts.len() >= 3 {
        let last2 = format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]);
        if matches!(last2.as_str(), "co.uk" | "com.br" | "com.au" | "co.jp" | "co.nz") {
            return last2;
        }
    }
    parts.last().map(|s| (*s).to_string()).unwrap_or_else(|| "unknown".to_string())
}

fn write_bucket_dir(dir: &Path, buckets: &HashMap<String, Vec<String>>) -> Result<()> {
    for (key, lines) in buckets {
        let safe = key.replace(['/', '\\', ':'], "_");
        let path = dir.join(format!("{safe}.txt"));
        let mut w = BufWriter::with_capacity(256 * 1024, File::create(&path)?);
        for line in lines {
            writeln!(w, "{line}")?;
        }
        w.flush()?;
    }
    Ok(())
}

fn write_summary(
    path: &Path,
    lines: u64,
    by_tld: &HashMap<String, Vec<String>>,
    by_domain: &HashMap<String, Vec<String>>,
) -> Result<()> {
    let mut w = BufWriter::new(File::create(path)?);
    writeln!(w, "lines={lines}")?;
    writeln!(w, "tlds={}", by_tld.len())?;
    writeln!(w, "domains={}", by_domain.len())?;
    writeln!(w, "\n# top TLDs")?;
    let mut tld_counts: Vec<_> = by_tld.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    tld_counts.sort_by(|a, b| b.1.cmp(&a.1));
    for (tld, count) in tld_counts.into_iter().take(30) {
        writeln!(w, "{tld}\t{count}")?;
    }
    w.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tld_handles_co_uk() {
        assert_eq!(tld_of("mail.co.uk"), "co.uk");
        assert_eq!(tld_of("gmail.com"), "com");
    }
}
