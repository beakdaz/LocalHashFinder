//! TextMerger — merge password wordlists, clean garbage, dedupe (standalone binary).

#[path = "../parser.rs"]
mod parser;

#[path = "../text_merge/mod.rs"]
mod text_merge;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use text_merge::{run_merge, MergeOptions};

#[derive(Parser)]
#[command(
    name = "TextMerger",
    about = "Merge .txt wordlists into plain passwords only (no combos/hashes), dedupe"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Merge all .txt files from a folder into one cleaned output
    Merge {
        /// Input folder with .txt files
        #[arg(long)]
        input: PathBuf,
        /// Output merged file
        #[arg(long)]
        output: PathBuf,
        /// Scan subfolders recursively
        #[arg(long)]
        recursive: bool,
        /// Drop lines shorter than N chars after trim (default 3)
        #[arg(long, default_value_t = 3)]
        min_len: usize,
        /// Case-insensitive dedupe
        #[arg(long)]
        case_insensitive: bool,
        /// Worker threads (0 = auto)
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Merge {
            input,
            output,
            recursive,
            min_len,
            case_insensitive,
            threads,
        } => {
            run_merge(&MergeOptions {
                input,
                output,
                recursive,
                min_len,
                case_insensitive,
                threads,
            })?;
        }
    }
    Ok(())
}
