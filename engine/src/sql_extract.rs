use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use anyhow::{Context, Result};
use encoding_rs::WINDOWS_1252;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;

use crate::job_control::{self, JobControl};

const UTF8_BOM: &[u8] = b"\xEF\xBB\xBF";
const ENCODING_SAMPLE: usize = 64 * 1024;
const LINE_BATCH: usize = 4096;
const MAX_SQL_THREADS: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SqlTextEncoding {
    Utf8,
    Windows1252,
}

fn strip_utf8_bom(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(UTF8_BOM).unwrap_or(bytes)
}

fn trim_crlf(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len();
    if end > 0 && bytes[end - 1] == b'\n' {
        end -= 1;
    }
    if end > 0 && bytes[end - 1] == b'\r' {
        end -= 1;
    }
    &bytes[..end]
}

fn detect_encoding(sample: &[u8]) -> SqlTextEncoding {
    let sample = strip_utf8_bom(sample);
    if sample.is_empty() || std::str::from_utf8(sample).is_ok() {
        SqlTextEncoding::Utf8
    } else {
        SqlTextEncoding::Windows1252
    }
}

fn decode_latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

fn decode_non_utf8_line(bytes: &[u8]) -> String {
    let (cow, _, _) = WINDOWS_1252.decode(bytes);
    let decoded = cow.into_owned();
    if !decoded.is_empty() || bytes.is_empty() {
        return decoded;
    }
    decode_latin1(bytes)
}

fn decode_line_bytes(bytes: &[u8], encoding: SqlTextEncoding) -> String {
    let bytes = trim_crlf(bytes);
    match encoding {
        SqlTextEncoding::Utf8 => std::str::from_utf8(bytes)
            .map(str::to_owned)
            .unwrap_or_else(|_| decode_non_utf8_line(bytes)),
        SqlTextEncoding::Windows1252 => WINDOWS_1252.decode(bytes).0.into_owned(),
    }
}

fn split_complete_lines(bytes: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    if let Some(pos) = bytes.iter().rposition(|&b| b == b'\n') {
        let (complete, tail) = bytes.split_at(pos + 1);
        let lines = complete
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect();
        (lines, tail.to_vec())
    } else {
        (Vec::new(), bytes.to_vec())
    }
}

fn read_encoding_sample(reader: &mut BufReader<File>) -> Result<Vec<u8>> {
    let mut sample = vec![0_u8; ENCODING_SAMPLE];
    let n = reader.read(&mut sample)?;
    sample.truncate(n);
    Ok(sample)
}

fn process_decoded_line(
    line: &str,
    writer: &mut BufWriter<File>,
    trash_writer: &mut BufWriter<File>,
    stats: &mut ExtractStats,
    seen: &mut HashSet<String>,
    trash_seen: &mut HashSet<String>,
) -> Result<()> {
    stats.lines_scanned += 1;
    let (pairs, trash) = collect_from_line(line);
    apply_collected(&pairs, &trash, writer, trash_writer, stats, seen, trash_seen)?;

    if stats.lines_scanned.is_multiple_of(1_000_000) {
        tracing::info!(
            "scanned {} M lines, found {} email:hash",
            stats.lines_scanned / 1_000_000,
            stats.total
        );
    }

    Ok(())
}

fn collect_from_line(line: &str) -> (Vec<(String, String)>, Vec<String>) {
    let mut pairs = Vec::new();
    for cap in email_hash_re().captures_iter(line) {
        let email = cap
            .get(1)
            .map(|m| m.as_str().to_ascii_lowercase())
            .unwrap_or_default();
        let hash = cap
            .get(2)
            .map(|m| m.as_str().to_ascii_lowercase())
            .unwrap_or_default();
        if !email.is_empty() && !hash.is_empty() {
            pairs.push((email, hash));
        }
    }

    let emails = unique_emails(line);
    let hashes = unique_quoted_hashes(line);
    for (email, hash) in pair_sql_fields(&emails, &hashes) {
        pairs.push((email, hash));
    }

    let mut trash = Vec::new();
    for cap in email_null_inline_re().captures_iter(line) {
        let email = cap
            .get(1)
            .map(|m| m.as_str().to_ascii_lowercase())
            .unwrap_or_default();
        if email.contains('@') {
            trash.push(format!("{email}:null"));
        }
    }

    for cap in sql_email_null_re().captures_iter(line) {
        let email = cap
            .get(1)
            .map(|m| m.as_str().to_ascii_lowercase())
            .unwrap_or_default();
        if email.contains('@') {
            trash.push(format!("{email}:null"));
        }
    }

    for m in email_re().find_iter(line) {
        let email = m.as_str().to_ascii_lowercase();
        let rest = &line[m.end()..];
        if !rest.starts_with(':') {
            continue;
        }
        let after_colon = &rest[1..];
        if after_colon.trim().is_empty() {
            trash.push(format!("{email}:"));
        }
    }

    (pairs, trash)
}

fn apply_collected(
    pairs: &[(String, String)],
    trash: &[String],
    writer: &mut BufWriter<File>,
    trash_writer: &mut BufWriter<File>,
    stats: &mut ExtractStats,
    seen: &mut HashSet<String>,
    trash_seen: &mut HashSet<String>,
) -> Result<()> {
    for (email, hash) in pairs {
        write_pair(writer, email, hash, stats, seen)?;
    }
    for line in trash {
        write_trash(trash_writer, line, stats, trash_seen)?;
    }
    Ok(())
}

fn flush_line_batch(
    batch: &mut Vec<String>,
    writer: &mut BufWriter<File>,
    trash_writer: &mut BufWriter<File>,
    stats: &mut ExtractStats,
    seen: &mut HashSet<String>,
    trash_seen: &mut HashSet<String>,
    threads: usize,
) -> Result<()> {
    if batch.is_empty() {
        return Ok(());
    }

    let collected: Vec<(Vec<(String, String)>, Vec<String>)> = if threads <= 1 {
        batch.iter().map(|line| collect_from_line(line)).collect()
    } else {
        batch.par_iter().map(|line| collect_from_line(line)).collect()
    };

    stats.lines_scanned += collected.len() as u64;
    for (pairs, trash) in collected {
        apply_collected(&pairs, &trash, writer, trash_writer, stats, seen, trash_seen)?;
    }
    batch.clear();
    Ok(())
}

/// `email@domain.tld:32hex` (MD5) or `email@domain.tld:40hex` (SHA1)
fn email_hash_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)([a-z0-9][a-z0-9._%+-]*@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+):([a-f0-9]{32}|[a-f0-9]{40})\b",
        )
        .expect("email:hash regex")
    })
}

fn email_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)[a-z0-9][a-z0-9._%+-]*@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+",
        )
        .expect("email regex")
    })
}

/// Quoted MD5/SHA1 in SQL INSERT: `'abc123...'`
fn quoted_hash_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"'([a-f0-9]{32}|[a-f0-9]{40})'").expect("quoted hash regex")
    })
}

const EMAIL_PAT: &str = r"[a-z0-9][a-z0-9._%+-]*@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+";

/// `email:null` в тексте дампа.
fn email_null_inline_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(&format!(r"(?i)({EMAIL_PAT}):null\b")).expect("email null regex")
    })
}

/// `'email@x.com', NULL` в SQL INSERT.
fn sql_email_null_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)'([^']+@[^']*)'\s*,\s*null\b").expect("sql email null regex")
    })
}

#[derive(Clone, Default, Debug)]
pub struct ExtractStats {
    pub lines_scanned: u64,
    pub md5: u64,
    pub sha1: u64,
    pub total: u64,
    pub trash: u64,
    pub output_path: String,
    pub trash_path: String,
}

pub fn default_output(sql: &Path) -> PathBuf {
    let dir = sql.parent().unwrap_or_else(|| Path::new("."));
    let stem = sql.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    dir.join(format!("{stem}_emails.txt"))
}

pub fn default_trash_output(sql: &Path) -> PathBuf {
    let dir = sql.parent().unwrap_or_else(|| Path::new("."));
    let stem = sql.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    dir.join(format!("{stem}_trash.txt"))
}

pub fn outputs_in_subdir(out_dir: &Path) -> (PathBuf, PathBuf) {
    (out_dir.join("emails.txt"), out_dir.join("trash.txt"))
}

#[derive(Clone, Default, Debug)]
pub struct FolderExtractStats {
    pub files_total: u32,
    pub files_ok: u32,
    pub files_failed: u32,
    pub lines_scanned: u64,
    pub md5: u64,
    pub sha1: u64,
    pub total: u64,
    pub trash: u64,
    pub output_root: String,
    pub errors: Vec<String>,
}

impl FolderExtractStats {
    pub fn summary(&self, lang: crate::i18n::Lang) -> String {
        crate::i18n::folder_sql_summary(
            lang,
            &self.output_root,
            self.files_ok,
            self.files_failed,
            self.files_total,
            self.total,
            self.md5,
            self.sha1,
            self.trash,
            &self.errors,
        )
    }
}

/// Process every .sql/.txt/.dump in `folder`; results → `{stem}_emails.txt` рядом с каждым дампом.
pub fn extract_folder(
    folder: &Path,
    progress: Option<&std::sync::Arc<std::sync::Mutex<crate::dump_batch::BatchLiveProgress>>>,
    control: Option<&JobControl>,
    threads: usize,
    lang: crate::i18n::Lang,
) -> Result<FolderExtractStats> {
    use crate::dump_batch::{list_dump_files, update_live};

    let files = list_dump_files(folder)?;
    if files.is_empty() {
        anyhow::bail!("{}", crate::i18n::folder_no_dumps(lang));
    }

    let total = files.len() as u32;
    let threads = threads.clamp(1, MAX_SQL_THREADS);
    update_live(progress, |p| {
        p.reset();
        p.lang = lang;
    });

    if threads <= 1 {
        let mut stats = FolderExtractStats {
            files_total: total,
            output_root: folder.display().to_string(),
            ..Default::default()
        };

        for (i, source) in files.iter().enumerate() {
            if job_control::checkpoint(control) {
                stats.errors.push(crate::i18n::stopped_by_user(lang).into());
                break;
            }
            process_folder_file(
                source,
                (i + 1) as u32,
                total,
                &mut stats,
                progress,
                control,
                1,
            )?;
        }

        update_live(progress, |p| p.active = false);
        return Ok(stats);
    }

    let stats = Arc::new(Mutex::new(FolderExtractStats {
        files_total: total,
        output_root: folder.display().to_string(),
        ..Default::default()
    }));
    let stopped = Arc::new(Mutex::new(false));
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .context("sql thread pool")?;

    pool.install(|| {
        files.par_iter().enumerate().for_each(|(i, source)| {
            if *stopped.lock().unwrap() || job_control::checkpoint(control) {
                *stopped.lock().unwrap() = true;
                return;
            }
            let idx = (i + 1) as u32;
            let name = source
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| source.display().to_string());
            update_live(progress, |p| p.begin_file(idx, total, &name));

            let output = default_output(source);
            let trash = default_trash_output(source);
            let file_result =
                extract_from_sql_with_trash(source, &output, &trash, control, 1);

            let mut stats = stats.lock().unwrap();
            match file_result {
                Ok(s) => {
                    stats.files_ok += 1;
                    stats.lines_scanned += s.lines_scanned;
                    stats.total += s.total;
                    stats.md5 += s.md5;
                    stats.sha1 += s.sha1;
                    stats.trash += s.trash;
                    update_live(progress, |p| {
                        p.files_ok = stats.files_ok;
                        p.files_failed = stats.files_failed;
                        p.lines_scanned = stats.lines_scanned;
                        p.total = stats.total;
                        p.md5 = stats.md5;
                        p.sha1 = stats.sha1;
                        p.trash = stats.trash;
                    });
                }
                Err(e) => {
                    stats.files_failed += 1;
                    stats.errors.push(format!("{name}: {e}"));
                    update_live(progress, |p| {
                        p.files_failed = stats.files_failed;
                    });
                }
            }
        });
    });

    update_live(progress, |p| p.active = false);
    Arc::try_unwrap(stats)
        .map_err(|_| anyhow::anyhow!("folder stats still shared"))?
        .into_inner()
        .map_err(|e| anyhow::anyhow!("folder stats poisoned: {e}"))
}

fn process_folder_file(
    source: &Path,
    idx: u32,
    total: u32,
    stats: &mut FolderExtractStats,
    progress: Option<&std::sync::Arc<std::sync::Mutex<crate::dump_batch::BatchLiveProgress>>>,
    control: Option<&JobControl>,
    file_threads: usize,
) -> Result<()> {
    use crate::dump_batch::update_live;

    let name = source
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| source.display().to_string());

    update_live(progress, |p| p.begin_file(idx, total, &name));

    let output = default_output(source);
    let trash = default_trash_output(source);

    match extract_from_sql_with_trash(source, &output, &trash, control, file_threads) {
        Ok(s) => {
            stats.files_ok += 1;
            stats.lines_scanned += s.lines_scanned;
            stats.total += s.total;
            stats.md5 += s.md5;
            stats.sha1 += s.sha1;
            stats.trash += s.trash;
            update_live(progress, |p| {
                p.files_ok = stats.files_ok;
                p.files_failed = stats.files_failed;
                p.lines_scanned = stats.lines_scanned;
                p.total = stats.total;
                p.md5 = stats.md5;
                p.sha1 = stats.sha1;
                p.trash = stats.trash;
            });
        }
        Err(e) => {
            stats.files_failed += 1;
            stats.errors.push(format!("{name}: {e}"));
            update_live(progress, |p| {
                p.files_failed = stats.files_failed;
            });
        }
    }

    Ok(())
}

fn unique_emails(line: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for m in email_re().find_iter(line) {
        let e = m.as_str().to_ascii_lowercase();
        if e.len() > 3 && seen.insert(e.clone()) {
            out.push(e);
        }
    }
    out
}

fn unique_quoted_hashes(line: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for cap in quoted_hash_re().captures_iter(line) {
        let h = cap.get(1).map(|m| m.as_str().to_ascii_lowercase()).unwrap_or_default();
        if seen.insert(h.clone()) {
            out.push(h);
        }
    }
    out
}

/// Pair email + hash fields from SQL INSERT row (separate columns).
fn pair_sql_fields(emails: &[String], hashes: &[String]) -> Vec<(String, String)> {
    if emails.is_empty() || hashes.is_empty() {
        return Vec::new();
    }

    let mut pairs = Vec::new();

    if hashes.len() == 1 {
        for e in emails {
            pairs.push((e.clone(), hashes[0].clone()));
        }
    } else if emails.len() == 1 {
        for h in hashes {
            pairs.push((emails[0].clone(), h.clone()));
        }
    } else if emails.len() <= 4 && hashes.len() <= 4 {
        for e in emails {
            for h in hashes {
                pairs.push((e.clone(), h.clone()));
            }
        }
    }

    pairs
}

fn write_pair(
    writer: &mut BufWriter<File>,
    email: &str,
    hash: &str,
    stats: &mut ExtractStats,
    seen: &mut HashSet<String>,
) -> Result<()> {
    let key = format!("{email}:{hash}");
    if !seen.insert(key) {
        return Ok(());
    }
    match hash.len() {
        32 => stats.md5 += 1,
        40 => stats.sha1 += 1,
        _ => return Ok(()),
    }
    stats.total += 1;
    writeln!(writer, "{email}:{hash}")?;
    Ok(())
}

fn write_trash(
    trash_writer: &mut BufWriter<File>,
    line: &str,
    stats: &mut ExtractStats,
    trash_seen: &mut HashSet<String>,
) -> Result<()> {
    if !trash_seen.insert(line.to_string()) {
        return Ok(());
    }
    stats.trash += 1;
    writeln!(trash_writer, "{line}")?;
    Ok(())
}

/// Scan `.sql` (or any text) and write `email:md5` / `email:sha1`.
pub fn extract_from_sql(source: &Path, output: &Path) -> Result<ExtractStats> {
    let trash_output = default_trash_output(source);
    extract_from_sql_with_trash(source, output, &trash_output, None, 1)
}

pub fn extract_from_sql_with_trash(
    source: &Path,
    output: &Path,
    trash_output: &Path,
    control: Option<&JobControl>,
    threads: usize,
) -> Result<ExtractStats> {
    let threads = threads.clamp(1, MAX_SQL_THREADS);
    tracing::info!(
        "extract email:hash from {} (threads={})",
        source.display(),
        threads
    );

    let file = File::open(source).with_context(|| format!("open {}", source.display()))?;
    let mut reader = BufReader::with_capacity(16 * 1024 * 1024, file);
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, File::create(output)?);
    let mut trash_writer =
        BufWriter::with_capacity(8 * 1024 * 1024, File::create(trash_output)?);

    let sample = read_encoding_sample(&mut reader)?;
    let encoding = detect_encoding(&sample);
    tracing::debug!(?encoding, "sql text encoding");

    let mut stats = ExtractStats {
        output_path: output.display().to_string(),
        trash_path: trash_output.display().to_string(),
        ..Default::default()
    };
    let mut seen = HashSet::new();
    let mut trash_seen = HashSet::new();
    let mut line_buf = Vec::with_capacity(4096);
    let mut line_batch: Vec<String> = Vec::with_capacity(LINE_BATCH);

    let (initial_lines, mut pending) = split_complete_lines(strip_utf8_bom(&sample));
    for line_bytes in initial_lines {
        if job_control::checkpoint(control) {
            break;
        }
        let line = decode_line_bytes(&line_bytes, encoding);
        if threads <= 1 {
            process_decoded_line(
                &line,
                &mut writer,
                &mut trash_writer,
                &mut stats,
                &mut seen,
                &mut trash_seen,
            )?;
        } else {
            line_batch.push(line);
            if line_batch.len() >= LINE_BATCH {
                flush_line_batch(
                    &mut line_batch,
                    &mut writer,
                    &mut trash_writer,
                    &mut stats,
                    &mut seen,
                    &mut trash_seen,
                    threads,
                )?;
            }
        }
    }

    loop {
        if job_control::checkpoint(control) {
            break;
        }
        line_buf.clear();
        line_buf.append(&mut pending);
        let bytes_read = reader.read_until(b'\n', &mut line_buf)?;
        if bytes_read == 0 && line_buf.is_empty() {
            break;
        }

        let line = decode_line_bytes(&line_buf, encoding);
        if threads <= 1 {
            process_decoded_line(
                &line,
                &mut writer,
                &mut trash_writer,
                &mut stats,
                &mut seen,
                &mut trash_seen,
            )?;
        } else {
            line_batch.push(line);
            if line_batch.len() >= LINE_BATCH {
                flush_line_batch(
                    &mut line_batch,
                    &mut writer,
                    &mut trash_writer,
                    &mut stats,
                    &mut seen,
                    &mut trash_seen,
                    threads,
                )?;
            }
        }

        if bytes_read == 0 {
            break;
        }
    }

    if threads > 1 {
        flush_line_batch(
            &mut line_batch,
            &mut writer,
            &mut trash_writer,
            &mut stats,
            &mut seen,
            &mut trash_seen,
            threads,
        )?;
    }

    writer.flush()?;
    trash_writer.flush()?;
    tracing::info!(
        "extract done: {} total (md5={}, sha1={}), trash={} -> {} / {}",
        stats.total,
        stats.md5,
        stats.sha1,
        stats.trash,
        stats.output_path,
        stats.trash_path
    );
    Ok(stats)
}

/// Returns true if hash length is MD5 (32) or SHA1 (40).
pub fn is_md5_or_sha1_hex(hash: &str) -> bool {
    let h = hash.trim();
    (h.len() == 32 || h.len() == 40) && h.bytes().all(|b| b.is_ascii_hexdigit())
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
        std::env::temp_dir().join(format!("lhf_sql_{name}_{nanos}.sql"))
    }

    #[test]
    fn decode_line_bytes_handles_non_utf8() {
        let bytes = b"INSERT INTO users VALUES ('test@test.com', '\x80abc');";
        let line = decode_line_bytes(bytes, SqlTextEncoding::Windows1252);
        assert!(line.contains("test@test.com"));

        let invalid = b"user@test.com:0123456789abcdef0123456789abcdef\xff";
        let decoded = decode_line_bytes(invalid, SqlTextEncoding::Utf8);
        assert!(decoded.contains("user@test.com"));
    }

    #[test]
    fn extract_from_sql_accepts_windows_1252() -> Result<()> {
        let source = temp_path("win1252");
        let output = temp_path("win1252_out.txt");

        let mut file = File::create(&source)?;
        file.write_all(
            b"INSERT INTO t VALUES ('admin@example.com', '0123456789abcdef0123456789abcdef', '\x80');\n",
        )?;
        drop(file);

        let stats = extract_from_sql(&source, &output)?;
        assert_eq!(stats.lines_scanned, 1);
        assert_eq!(stats.total, 1);

        let text = std::fs::read_to_string(&output)?;
        assert!(text.contains("admin@example.com:0123456789abcdef0123456789abcdef"));

        let _ = std::fs::remove_file(source);
        let _ = std::fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn extract_from_sql_strips_utf8_bom() -> Result<()> {
        let source = temp_path("bom");
        let output = temp_path("bom_out.txt");

        let mut file = File::create(&source)?;
        file.write_all(UTF8_BOM)?;
        write!(
            file,
            "INSERT INTO t VALUES ('bom@example.com', '0123456789abcdef0123456789abcdef');\n"
        )?;
        drop(file);

        let stats = extract_from_sql(&source, &output)?;
        assert_eq!(stats.total, 1);

        let text = std::fs::read_to_string(&output)?;
        assert!(text.contains("bom@example.com:0123456789abcdef0123456789abcdef"));

        let _ = std::fs::remove_file(source);
        let _ = std::fs::remove_file(output);
        Ok(())
    }
}
