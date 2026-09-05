use std::path::PathBuf;

use anyhow::Result;

use crate::combo::{LineMode, LineToolOptions, run_line_tool};
use crate::job_control::JobControl;

use super::archive::materialize_input;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MiscOp {
    Merge,
    Split,
    Filter,
}

pub struct MiscStats {
    pub input_lines: u64,
    pub output_lines: u64,
    pub output_files: Vec<PathBuf>,
    pub message: String,
}

pub fn run_misc(
    op: MiscOp,
    input: &str,
    output: &str,
    filter: &str,
    lines_per_file: usize,
    control: Option<&JobControl>,
) -> Result<MiscStats> {
    let (files, _temp) = materialize_input(input)?;
    if files.is_empty() {
        anyhow::bail!("no input files");
    }

    let mode = match op {
        MiscOp::Merge => LineMode::Merge,
        MiscOp::Split => LineMode::Split,
        MiscOp::Filter => LineMode::Filter,
    };

    // Merge uses first file as anchor path; run_line_tool merge only reads one input in current impl.
    // For folder input, merge all materialized files sequentially via temp merged input.
    let input_path = if files.len() == 1 {
        files[0].clone()
    } else if op == MiscOp::Merge {
        merge_to_temp(&files)?
    } else {
        files[0].clone()
    };

    let opts = LineToolOptions {
        mode,
        input: input_path,
        output: PathBuf::from(output),
        filter: filter.to_string(),
        use_regex: false,
        lines_per_file: if lines_per_file == 0 {
            100_000
        } else {
            lines_per_file
        },
    };

    let result = run_line_tool(&opts, control)?;
    let label = match op {
        MiscOp::Merge => "Misc merge",
        MiscOp::Split => "Misc split",
        MiscOp::Filter => "Misc filter",
    };
    Ok(MiscStats {
        input_lines: result.input_lines,
        output_lines: result.output_lines,
        output_files: result.output_files,
        message: format!(
            "{label}: {} → {} lines",
            result.input_lines, result.output_lines
        ),
    })
}

fn merge_to_temp(files: &[PathBuf]) -> Result<PathBuf> {
    use std::fs::File;
    use std::io::{BufWriter, Write};

    let path = std::env::temp_dir().join(format!("lhf_ulp_merge_{}.txt", std::process::id()));
    let mut w = BufWriter::with_capacity(256 * 1024, File::create(&path)?);
    for f in files {
        let text = std::fs::read_to_string(f)?;
        for line in text.lines() {
            writeln!(w, "{line}")?;
        }
    }
    w.flush()?;
    Ok(path)
}
