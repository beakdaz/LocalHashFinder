// LocalHashFinder GUI + CLI entry
mod i18n;
mod dump_batch;
mod job_control;
mod tab_log;
mod tab_results;
mod combo;
mod ulp;
mod app;
mod config;
mod db;
mod logging;
mod merger;
mod parser;
mod processor;
mod sql_io;
mod regex_extract;
mod sql_columns;
mod sql_extract;
mod wordlist_hash;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::{ensure_lmdb_parent_dirs, load_lmdb_path, resolve_lmdb_path, save_lmdb_path};
use crate::db::HashDb;
use crate::job_control::JobControl;

#[derive(Parser)]
#[command(name = "LocalHashFinder", about = "Offline hash lookup from local LMDB")]
struct Cli {
    /// LMDB folder (hashdb.lmdb), data/, or any folder — saved to LocalHashFinder.cfg
    #[arg(long, global = true)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Import hash:password text file(s) into LMDB (one-time, can take hours)
    Import {
        /// Source file(s), e.g. D:\db\hashes.txt
        #[arg(required = true)]
        sources: Vec<PathBuf>,
        /// LMDB map size in GB (set >= your txt size + 80). For 200GB use 280
        #[arg(long, default_value_t = 280)]
        map_gb: u64,
    },
    /// Append hash:pass to existing LMDB (skip duplicate keys)
    Append {
        /// New hash:pass file(s) to add
        #[arg(required = true)]
        sources: Vec<PathBuf>,
        #[arg(long, default_value_t = 280)]
        map_gb: u64,
    },
    /// Open GUI (default)
    Gui,
    /// Merge mail:hashedpass + hash:dehashedpass → mail:plainpass
    Merge {
        /// mail:hash file (email list with password hashes)
        #[arg(long)]
        mail: PathBuf,
        /// hash:plainpass file (_good.txt or dehash dump)
        #[arg(long)]
        dehash: PathBuf,
    },
    /// Extract email:md5 / email:sha1 from .sql dump via regex
    ExtractSql {
        /// Source .sql file
        source: PathBuf,
        /// Output .txt (default: {name}_emails.txt next to source)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Extract login:password from SQL by column names (username/password, etc.)
    ExtractSqlColumns {
        source: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Check if hex hash exists in LMDB
    Lookup {
        /// One or more 32/40-char hex hashes
        #[arg(required = true)]
        hashes: Vec<String>,
    },
    /// Process input file → _good.txt / _nohash.txt
    Process {
        input: PathBuf,
        #[arg(short, long, default_value_t = 64)]
        threads: usize,
    },
    /// Hash plaintext wordlist -> hash:pass (MD5 / SHA1) for LMDB import
    WordlistHash {
        /// One password per line (.txt wordlist)
        source: PathBuf,
        /// Output file (required if both algos; else {random}_{name}_md5.txt / _sha1.txt)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// md5, sha1, or both (default: md5)
        #[arg(long, default_value = "md5")]
        algo: String,
        #[arg(short, long, default_value_t = 0)]
        threads: usize,
    },
    /// Extract lines with custom regex and output template
    ExtractRegex {
        source: PathBuf,
        #[arg(long)]
        pattern: String,
        #[arg(long)]
        template: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        case_insensitive: bool,
        #[arg(long)]
        multiline: bool,
        #[arg(long)]
        dotall: bool,
        #[arg(long)]
        no_dedupe: bool,
    },
}

fn init_db(cli: &Cli) -> Result<std::sync::Arc<HashDb>> {
    let lmdb = resolve_lmdb_path(cli.db.clone());
    if cli.db.is_some() {
        save_lmdb_path(&lmdb)?;
        tracing::info!("LMDB path saved: {}", lmdb.display());
    } else if load_lmdb_path().is_none() {
        save_lmdb_path(&lmdb)?;
        tracing::info!("LMDB default path saved: {}", lmdb.display());
    }
    ensure_lmdb_parent_dirs(&lmdb)?;
    tracing::info!("LMDB: {}", lmdb.display());
    let db = HashDb::new(lmdb);
    db.open_existing()?;
    Ok(db)
}

fn parse_wordlist_algos(raw: &str) -> Result<Vec<wordlist_hash::HashAlgo>> {
    let key = raw.trim().to_ascii_lowercase();
    match key.as_str() {
        "md5" => Ok(vec![wordlist_hash::HashAlgo::Md5]),
        "sha1" => Ok(vec![wordlist_hash::HashAlgo::Sha1]),
        "both" | "all" => Ok(vec![
            wordlist_hash::HashAlgo::Md5,
            wordlist_hash::HashAlgo::Sha1,
        ]),
        _ => anyhow::bail!("unknown algo '{raw}' — use md5, sha1, or both"),
    }
}

fn main() -> Result<()> {
    logging::init().map_err(|e| anyhow::anyhow!("log init: {e}"))?;

    let cli = Cli::parse();

    if matches!(cli.command, Some(Commands::WordlistHash { .. })) {
        let Commands::WordlistHash {
            source,
            output,
            algo,
            threads,
        } = cli.command.unwrap()
        else {
            unreachable!()
        };
        let algos = parse_wordlist_algos(&algo)?;
        if algos.len() > 1 && output.is_some() {
            anyhow::bail!("use separate default outputs for both algos, or pass --algo md5|sha1 with -o");
        }
        let threads = if threads == 0 {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(8)
        } else {
            threads
        };
        let stats = wordlist_hash::run_wordlist_hash(wordlist_hash::WordlistHashOptions {
            source,
            output,
            algos,
            threads,
        })?;
        println!(
            "Done: read={} written={} in {:.1}s ({:.0} lines/s)",
            stats.lines_read,
            stats.lines_written,
            stats.elapsed_secs,
            stats.lines_read as f64 / stats.elapsed_secs.max(0.001)
        );
        return Ok(());
    }

    let db = init_db(&cli)?;

    match cli.command {
        Some(Commands::Import { sources, map_gb }) => {
            println!("Importing into {}", db.lmdb_path().display());
            println!("Map size: ~{} GB — ensure free disk space", map_gb + 80);
            let n = db.import(&sources, map_gb)?;
            save_lmdb_path(&db.lmdb_path())?;
            println!("Done: {n} hashes in {}", db.lmdb_path().display());
        }
        Some(Commands::Append { sources, map_gb }) => {
            println!("Appending to {}", db.lmdb_path().display());
            let stats = db.append(&sources, map_gb)?;
            save_lmdb_path(&db.lmdb_path())?;
            println!(
                "Done: added={} skipped={} bad_lines={} final={}",
                stats.added, stats.skipped, stats.bad_lines, stats.final_count
            );
        }
        Some(Commands::Merge { mail, dehash }) => {
            let stats = merger::merge_files(&mail, &dehash, None)?;
            println!(
                "Done: merged={} nohash={} bad={} trash={} total={}",
                stats.merged, stats.nohash, stats.bad, stats.trash, stats.total
            );
            println!("{}", stats.plain_path);
            println!("{}", stats.nohash_path);
            println!("{}", stats.trash_path);
        }
        Some(Commands::ExtractSql { source, output }) => {
            let out = output.unwrap_or_else(|| sql_extract::default_output(&source));
            let stats = sql_extract::extract_from_sql(&source, &out)?;
            println!(
                "Done: {} email:hash (md5={}, sha1={}), trash={} from {} lines",
                stats.total, stats.md5, stats.sha1, stats.trash, stats.lines_scanned
            );
            println!("{}", stats.output_path);
            println!("{}", stats.trash_path);
        }
        Some(Commands::ExtractSqlColumns { source, output }) => {
            let out = output.unwrap_or_else(|| sql_columns::default_output(&source));
            let stats = sql_columns::extract_from_sql_columns(&source, &out, None)?;
            println!(
                "Done: written={} skipped={} tables={} inserts={} from {} lines",
                stats.written,
                stats.skipped,
                stats.tables_found,
                stats.inserts_parsed,
                stats.lines_scanned
            );
            println!("{}", stats.output_path);
        }
        Some(Commands::Lookup { hashes }) => {
            db.open_existing()?;
            for h in hashes {
                let hex = h.trim().to_ascii_lowercase();
                match db.lookup(&hex)? {
                    Some(pass) => println!("{hex} => {pass}"),
                    None => println!("{hex} => NOT FOUND"),
                }
            }
        }
        Some(Commands::Process { input, threads }) => {
            let count = db.open_existing()?;
            if count == 0 {
                anyhow::bail!("LMDB empty or missing — open database first");
            }
            println!("Processing {} (DB: {count} entries)", input.display());
            let control = JobControl::new_shared();
            let mut last = processor::Progress::default();
            processor::process_file(db, input, threads, control, |p| last = p.clone())?;
            println!(
                "Done: good={} nohash={} bad={} trash={}",
                last.found, last.nohash, last.bad, last.trash
            );
            println!("{}", last.good_path);
            println!("{}", last.nohash_path);
            println!("{}", last.trash_path);
        }
        Some(Commands::ExtractRegex {
            source,
            pattern,
            template,
            output,
            case_insensitive,
            multiline,
            dotall,
            no_dedupe,
        }) => {
            let out = output.unwrap_or_else(|| regex_extract::default_output(&source));
            let config = regex_extract::RegexExtractConfig {
                pattern,
                output_template: template,
                case_insensitive,
                multiline,
                dot_matches_newline: dotall,
                dedupe: !no_dedupe,
                skip_empty: true,
            };
            let stats = regex_extract::extract_with_regex(&source, &out, &config, None)?;
            println!(
                "Done: written={} matches={} dup={} skipped_empty={} from {} lines",
                stats.written,
                stats.match_hits,
                stats.duplicates,
                stats.skipped_empty,
                stats.lines_scanned
            );
            println!("{}", stats.output_path);
        }
        Some(Commands::WordlistHash { .. }) => unreachable!(),
        Some(Commands::Gui) | None => {
            let count = db.open_existing().unwrap_or(0);
            tracing::info!("LMDB entries: {count}");
            app::run(db)?;
        }
    }

    Ok(())
}
