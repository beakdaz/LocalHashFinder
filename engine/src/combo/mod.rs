//! ComboKit tools ported to Rust (compare, filter, scraper, MX, line tools, …).

mod compare;
mod cred;
mod filter;
mod lines;
mod mx;
mod scraper;
mod stream;

pub use compare::{CompareOptions, CompareResult, compare_lists};
pub use cred::{email_domain, parse_email_pass};
pub use filter::{
    filter_combos, filter_email_combo, split_name_pass, ComboFilterOptions, ComboFilterResult,
    EmailFilterOptions, SplitNamePassOptions, SplitNamePassResult,
};
pub use lines::{run_line_tool, LineMode, LineToolOptions, LineToolResult};
pub use mx::{check_mx, MxCheckOptions, MxCheckResult};
pub use scraper::{
    analyze_provider, scrape_credentials, AnalyzeOptions, AnalyzeResult, AnalyzeRow,
    ScraperOptions, ScraperResult,
};
pub use stream::stream_lines;

use std::fmt;

/// ComboKit tool selector (matches legacy ComboKit cards).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ComboTool {
    #[default]
    Compare,
    ComboFilter,
    EmailFilter,
    SplitNamePass,
    MxCheck,
    Scraper,
    Analyze,
    LineDedupe,
    LineFilter,
    LineMerge,
    LineSplit,
}

impl ComboTool {
    pub const ALL: [ComboTool; 11] = [
        Self::Compare,
        Self::ComboFilter,
        Self::EmailFilter,
        Self::SplitNamePass,
        Self::MxCheck,
        Self::Scraper,
        Self::Analyze,
        Self::LineDedupe,
        Self::LineFilter,
        Self::LineMerge,
        Self::LineSplit,
    ];

    pub fn label(self, lang: crate::i18n::Lang) -> &'static str {
        let t = crate::i18n::tr(lang);
        match self {
            Self::Compare => t.combo_tool_compare,
            Self::ComboFilter => t.combo_tool_filter,
            Self::EmailFilter => t.combo_tool_email,
            Self::SplitNamePass => t.combo_tool_namepw,
            Self::MxCheck => t.combo_tool_mx,
            Self::Scraper => t.combo_tool_scraper,
            Self::Analyze => t.combo_tool_analyze,
            Self::LineDedupe => t.combo_tool_dedupe,
            Self::LineFilter => t.combo_tool_line_filter,
            Self::LineMerge => t.combo_tool_merge,
            Self::LineSplit => t.combo_tool_split,
        }
    }
}

/// Unified result summary for the Combo tab UI.
#[derive(Clone, Debug, Default)]
pub struct ComboJobSummary {
    pub message: String,
}

impl fmt::Display for ComboJobSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// Run selected ComboKit tool; returns human-readable summary.
pub fn run_tool(
    tool: ComboTool,
    input: &str,
    input_b: &str,
    output: &str,
    output_dir: &str,
    filter: &str,
    use_regex: bool,
    lines_per_file: usize,
    control: Option<&crate::job_control::JobControl>,
) -> anyhow::Result<ComboJobSummary> {
    use anyhow::{Context, bail};
    use std::path::PathBuf;

    let msg = match tool {
        ComboTool::Compare => {
            let res = compare_lists(
                &CompareOptions {
                    file_a: PathBuf::from(input),
                    file_b: PathBuf::from(input_b),
                    output_dir: PathBuf::from(output_dir),
                },
                control,
            )?;
            format!(
                "Compare: A={} B={} · only A={} only B={} both={} → {}",
                res.lines_a,
                res.lines_b,
                res.only_a,
                res.only_b,
                res.both,
                res.output_dir.display()
            )
        }
        ComboTool::ComboFilter => {
            let res = filter_combos(
                &ComboFilterOptions {
                    input: input.into(),
                    output: output.into(),
                    allow_name_specials: true,
                    allow_pass_specials: true,
                    ..Default::default()
                },
                control,
            )?;
            format!("Filter: {} → {} lines → {}", res.input_lines, res.output_lines, output)
        }
        ComboTool::EmailFilter => {
            let res = filter_email_combo(
                &EmailFilterOptions {
                    input: input.into(),
                    output: output.into(),
                },
                control,
            )?;
            format!("Email filter: {} → {} → {}", res.input_lines, res.output_lines, output)
        }
        ComboTool::SplitNamePass => {
            let names = format!("{output_dir}/names.txt");
            let passes = format!("{output_dir}/passwords.txt");
            std::fs::create_dir_all(output_dir).context("create output dir")?;
            let res = split_name_pass(
                &SplitNamePassOptions {
                    input: input.into(),
                    names_file: names.clone(),
                    passwords_file: passes.clone(),
                    use_local_part: true,
                },
                control,
            )?;
            format!("Split {} combos → {names} + {passes}", res.lines)
        }
        ComboTool::MxCheck => {
            let res = check_mx(
                &MxCheckOptions {
                    input: input.into(),
                    output_dir: output_dir.into(),
                },
                control,
            )?;
            format!(
                "MX: {} domains · valid={} bad={} unknown={} → {output_dir}",
                res.domains, res.valid, res.bad, res.unknown
            )
        }
        ComboTool::Scraper => {
            let res = scrape_credentials(
                &ScraperOptions {
                    input: PathBuf::from(input),
                    output: PathBuf::from(output),
                    include_sql: true,
                    include_json: true,
                },
                control,
            )?;
            format!("Scrape: {} combos, {} emails → {output}", res.combos, res.emails)
        }
        ComboTool::Analyze => {
            let res = analyze_provider(
                &AnalyzeOptions {
                    input: PathBuf::from(input),
                    output_dir: PathBuf::from(output_dir),
                },
                control,
            )?;
            format!(
                "Analyze: {} lines, {} domains → {output_dir}/by_domain",
                res.lines, res.domains
            )
        }
        ComboTool::LineDedupe => {
            let res = run_line_tool(
                &LineToolOptions {
                    mode: LineMode::Dedupe,
                    input: PathBuf::from(input),
                    output: PathBuf::from(output),
                    ..Default::default()
                },
                control,
            )?;
            format!("Dedupe: {} → {} unique → {output}", res.input_lines, res.output_lines)
        }
        ComboTool::LineFilter => {
            if filter.trim().is_empty() {
                bail!("filter text required");
            }
            let res = run_line_tool(
                &LineToolOptions {
                    mode: LineMode::Filter,
                    input: PathBuf::from(input),
                    output: PathBuf::from(output),
                    filter: filter.into(),
                    use_regex,
                    ..Default::default()
                },
                control,
            )?;
            format!("Line filter: {} → {} → {output}", res.input_lines, res.output_lines)
        }
        ComboTool::LineMerge => {
            let res = run_line_tool(
                &LineToolOptions {
                    mode: LineMode::Merge,
                    input: PathBuf::from(input),
                    output: PathBuf::from(output),
                    ..Default::default()
                },
                control,
            )?;
            format!("Merge: {} lines → {output}", res.input_lines)
        }
        ComboTool::LineSplit => {
            let res = run_line_tool(
                &LineToolOptions {
                    mode: LineMode::Split,
                    input: PathBuf::from(input),
                    output: PathBuf::from(output),
                    lines_per_file,
                    ..Default::default()
                },
                control,
            )?;
            format!("Split: {} lines → {} files", res.input_lines, res.output_files.len())
        }
    };
    Ok(ComboJobSummary { message: msg })
}

#[cfg(test)]
mod tests {
    use super::compare::{compare_lists, CompareOptions};
    use super::cred::parse_email_pass;
    use std::fs;

    #[test]
    fn parse_email_pass_works() {
        let p = parse_email_pass("user@mail.com:secret123");
        assert_eq!(p, Some(("user@mail.com".into(), "secret123".into())));
        assert!(parse_email_pass("not-an-email:pass").is_none());
    }

    #[test]
    fn compare_lists_writes_outputs() {
        let dir = std::env::temp_dir().join("lhf_combo_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.txt");
        let b = dir.join("b.txt");
        fs::write(&a, "line1\nline2\n").unwrap();
        fs::write(&b, "line2\nline3\n").unwrap();
        let out = dir.join("out");
        let res = compare_lists(
            &CompareOptions {
                file_a: a,
                file_b: b,
                output_dir: out.clone(),
            },
            None,
        )
        .unwrap();
        assert_eq!(res.only_a, 1);
        assert_eq!(res.only_b, 1);
        assert_eq!(res.both, 1);
        assert!(out.join("only_a.txt").exists());
        assert!(out.join("only_b.txt").exists());
        assert!(out.join("both.txt").exists());
        let _ = fs::remove_dir_all(&dir);
    }
}
