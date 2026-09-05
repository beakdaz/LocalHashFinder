use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use regex::{Captures, Regex, RegexBuilder};

use crate::job_control::{self, JobControl};
use crate::sql_io::{self, SqlStatementStream};

#[derive(Clone, Debug)]
pub struct RegexExtractConfig {
    pub pattern: String,
    pub output_template: String,
    pub case_insensitive: bool,
    pub multiline: bool,
    pub dot_matches_newline: bool,
    pub dedupe: bool,
    pub skip_empty: bool,
}

impl Default for RegexExtractConfig {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            output_template: "$1".into(),
            case_insensitive: true,
            multiline: false,
            dot_matches_newline: false,
            dedupe: true,
            skip_empty: true,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RegexExtractStats {
    pub lines_scanned: u64,
    pub match_hits: u64,
    pub written: u64,
    pub skipped_empty: u64,
    pub duplicates: u64,
    pub output_path: String,
}

#[derive(Clone, Copy, Debug)]
pub struct RegexPreset {
    pub name: &'static str,
    pub pattern: &'static str,
    pub template: &'static str,
    pub case_insensitive: bool,
}

pub const PRESETS: &[RegexPreset] = &[
    RegexPreset {
        name: "email:md5",
        pattern: r"(?i)([a-z0-9][a-z0-9._%+-]*@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+):([a-f0-9]{32})\b",
        template: "$1:$2",
        case_insensitive: true,
    },
    RegexPreset {
        name: "email:sha1",
        pattern: r"(?i)([a-z0-9][a-z0-9._%+-]*@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+):([a-f0-9]{40})\b",
        template: "$1:$2",
        case_insensitive: true,
    },
    RegexPreset {
        name: "hash:pass",
        pattern: r"([a-f0-9]{32}):(\S+)",
        template: "$1:$2",
        case_insensitive: false,
    },
    RegexPreset {
        name: "email only",
        pattern: r"(?i)([a-z0-9][a-z0-9._%+-]*@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+)",
        template: "$1",
        case_insensitive: true,
    },
];

pub fn default_output(source: &Path) -> PathBuf {
    let dir = source.parent().unwrap_or_else(|| Path::new("."));
    let stem = source.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    dir.join(format!("{stem}_regex.txt"))
}

pub fn compile_regex(config: &RegexExtractConfig) -> Result<Regex> {
    let pattern = config.pattern.trim();
    if pattern.is_empty() {
        bail!("regex pattern is empty");
    }
    RegexBuilder::new(pattern)
        .case_insensitive(config.case_insensitive)
        .multi_line(config.multiline)
        .dot_matches_new_line(config.dot_matches_newline)
        .build()
        .with_context(|| format!("invalid regex: {pattern}"))
}

/// Подставляет `$0`, `$1`, `${name}`, `$$` в шаблон вывода.
pub fn render_template(template: &str, caps: &Captures<'_>) -> String {
    let mut out = String::with_capacity(template.len() + 32);
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            i += 1;
            if i >= chars.len() {
                out.push('$');
                break;
            }
            if chars[i] == '$' {
                out.push('$');
                i += 1;
                continue;
            }
            if chars[i] == '{' {
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }
                let name: String = chars[start..i.min(chars.len())].iter().collect();
                if i < chars.len() {
                    i += 1;
                }
                if let Some(m) = caps.name(&name) {
                    out.push_str(m.as_str());
                }
                continue;
            }
            if chars[i] == '0' {
                if let Some(m) = caps.get(0) {
                    out.push_str(m.as_str());
                }
                i += 1;
                continue;
            }
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let num: String = chars[start..i].iter().collect();
                if let Ok(n) = num.parse::<usize>() {
                    if let Some(m) = caps.get(n) {
                        out.push_str(m.as_str());
                    }
                }
                continue;
            }
            out.push('$');
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn process_chunk(
    chunk: &str,
    re: &Regex,
    config: &RegexExtractConfig,
    writer: &mut BufWriter<File>,
    stats: &mut RegexExtractStats,
    seen: &mut HashSet<String>,
) -> Result<()> {
    for caps in re.captures_iter(chunk) {
        stats.match_hits += 1;
        let rendered = render_template(&config.output_template, &caps);
        if config.skip_empty && rendered.trim().is_empty() {
            stats.skipped_empty += 1;
            continue;
        }
        if config.dedupe && !seen.insert(rendered.clone()) {
            stats.duplicates += 1;
            continue;
        }
        stats.written += 1;
        writeln!(writer, "{rendered}")?;
    }

    Ok(())
}

/// Сканирует файл построчно, применяет regex и пишет результат по шаблону.
pub fn extract_with_regex(
    source: &Path,
    output: &Path,
    config: &RegexExtractConfig,
    control: Option<&JobControl>,
) -> Result<RegexExtractStats> {
    let re = compile_regex(config)?;
    if config.output_template.trim().is_empty() {
        bail!("output template is empty");
    }

    tracing::info!(
        source = %source.display(),
        pattern = %config.pattern,
        template = %config.output_template,
        "regex extract start"
    );

    let mut stream = SqlStatementStream::open(source)?;
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, File::create(output)?);

    let mut stats = RegexExtractStats {
        output_path: output.display().to_string(),
        ..Default::default()
    };
    let mut seen = HashSet::new();

    while let Some(stmt) = stream.next_statement()? {
        if job_control::checkpoint(control) {
            break;
        }
        stats.lines_scanned = stream.lines_scanned();
        for chunk in sql_io::extraction_chunks(&stmt) {
            process_chunk(
                &chunk,
                &re,
                config,
                &mut writer,
                &mut stats,
                &mut seen,
            )?;
        }

        if stats.lines_scanned.is_multiple_of(1_000_000) {
            tracing::info!(
                "regex extract: scanned {} M lines, written {}",
                stats.lines_scanned / 1_000_000,
                stats.written
            );
        }
    }

    writer.flush()?;
    tracing::info!(
        written = stats.written,
        matches = stats.match_hits,
        output = %stats.output_path,
        "regex extract done"
    );
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("lhf_regex_{name}_{nanos}.txt"))
    }

    #[test]
    fn render_template_groups() {
        let re = Regex::new(r"(?i)(user@mail.com):([a-f0-9]{32})").unwrap();
        let caps = re.captures("x user@mail.com:0123456789abcdef0123456789abcdef y").unwrap();
        assert_eq!(
            render_template("$1:$2", &caps),
            "user@mail.com:0123456789abcdef0123456789abcdef"
        );
        assert_eq!(render_template("$0", &caps), "user@mail.com:0123456789abcdef0123456789abcdef");
        assert_eq!(render_template("$$ $1", &caps), "$ user@mail.com");
    }

    #[test]
    fn extract_with_regex_file() -> Result<()> {
        let source = temp_path("src");
        let output = temp_path("out");
        std::fs::write(
            &source,
            "a admin@test.com:0123456789abcdef0123456789abcdef b\n\
             c admin@test.com:0123456789abcdef0123456789abcdef d\n",
        )?;

        let config = RegexExtractConfig {
            pattern: PRESETS[0].pattern.into(),
            output_template: PRESETS[0].template.into(),
            case_insensitive: true,
            dedupe: true,
            ..Default::default()
        };
        let stats = extract_with_regex(&source, &output, &config, None)?;
        assert_eq!(stats.match_hits, 2);
        assert_eq!(stats.written, 1);
        assert_eq!(stats.duplicates, 1);

        let text = std::fs::read_to_string(&output)?;
        assert!(text.contains("admin@test.com:0123456789abcdef0123456789abcdef"));

        let _ = std::fs::remove_file(source);
        let _ = std::fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn extract_one_line_insert_with_multiple_tuples() -> Result<()> {
        let source = temp_path("oneline");
        let output = temp_path("oneline_out");
        let sql = "INSERT INTO t VALUES ('a@x.com','0123456789abcdef0123456789abcdef'), ('b@y.com','fedcba9876543210fedcba9876543210');";
        std::fs::write(&source, sql)?;

        let config = RegexExtractConfig {
            pattern: r"'([^']+@[^']+)',\s*'([a-f0-9]{32})'".into(),
            output_template: "$1:$2".into(),
            case_insensitive: true,
            dedupe: true,
            ..Default::default()
        };
        let stats = extract_with_regex(&source, &output, &config, None)?;
        assert_eq!(stats.written, 2);

        let text = std::fs::read_to_string(&output)?;
        assert!(text.contains("a@x.com:0123456789abcdef0123456789abcdef"));
        assert!(text.contains("b@y.com:fedcba9876543210fedcba9876543210"));

        let _ = std::fs::remove_file(source);
        let _ = std::fs::remove_file(output);
        Ok(())
    }
}
