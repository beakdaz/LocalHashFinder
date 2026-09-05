use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Result};
use regex::Regex;

use crate::job_control::{self, JobControl};
use crate::sql_io::{self, SqlStatementStream};

// no trash filters on this tab — plain login:password output only

const LOGIN_COLUMNS: &[&str] = &[
    "username",
    "users",
    "user",
    "login",
    "email",
    "emal",
    "mail",
    "mal",
    "customer",
    "customers",
    "member",
    "nickname",
];

const PASSWORD_COLUMNS: &[&str] = &["password", "passwords", "pass", "pwd"];

#[derive(Clone, Debug)]
struct TableSchema {
    columns: Vec<String>,
    login_idx: Option<usize>,
    pass_idx: Option<usize>,
}

#[derive(Clone, Default, Debug)]
pub struct SqlColumnsStats {
    pub lines_scanned: u64,
    pub tables_found: u64,
    pub inserts_parsed: u64,
    pub written: u64,
    pub skipped: u64,
    pub duplicates: u64,
    pub output_path: String,
}

fn login_names() -> &'static HashSet<String> {
    static SET: OnceLock<HashSet<String>> = OnceLock::new();
    SET.get_or_init(|| LOGIN_COLUMNS.iter().map(|s| (*s).to_string()).collect())
}

fn pass_names() -> &'static HashSet<String> {
    static SET: OnceLock<HashSet<String>> = OnceLock::new();
    SET.get_or_init(|| PASSWORD_COLUMNS.iter().map(|s| (*s).to_string()).collect())
}

fn normalize_col(name: &str) -> String {
    name.trim()
        .trim_matches('`')
        .trim_matches('"')
        .to_ascii_lowercase()
}

fn find_login_pass_indices(columns: &[String]) -> (Option<usize>, Option<usize>) {
    let logins = login_names();
    let passes = pass_names();
    let mut login_idx = None;
    let mut pass_idx = None;
    for (i, c) in columns.iter().enumerate() {
        if login_idx.is_none() && logins.contains(c) {
            login_idx = Some(i);
        }
        if pass_idx.is_none() && passes.contains(c) {
            pass_idx = Some(i);
        }
    }
    (login_idx, pass_idx)
}

pub fn default_output(sql: &Path) -> PathBuf {
    let dir = sql.parent().unwrap_or_else(|| Path::new("."));
    let stem = sql.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    dir.join(format!("{stem}_loginpass.txt"))
}

pub fn output_in_subdir(out_dir: &Path) -> PathBuf {
    out_dir.join("loginpass.txt")
}

#[derive(Clone, Default, Debug)]
pub struct FolderColumnsStats {
    pub files_total: u32,
    pub files_ok: u32,
    pub files_failed: u32,
    pub lines_scanned: u64,
    pub written: u64,
    pub skipped: u64,
    pub tables_found: u64,
    pub inserts_parsed: u64,
    pub output_root: String,
    pub errors: Vec<String>,
}

impl FolderColumnsStats {
    pub fn summary(&self, lang: crate::i18n::Lang) -> String {
        crate::i18n::folder_columns_summary(
            lang,
            &self.output_root,
            self.files_ok,
            self.files_failed,
            self.files_total,
            self.written,
            self.skipped,
            self.tables_found,
            &self.errors,
        )
    }
}

/// Process every dump in `folder`; results → `{stem}_loginpass.txt` рядом с каждым дампом.
pub fn extract_folder(
    folder: &Path,
    progress: Option<&std::sync::Arc<std::sync::Mutex<crate::dump_batch::BatchLiveProgress>>>,
    control: Option<&JobControl>,
    lang: crate::i18n::Lang,
) -> Result<FolderColumnsStats> {
    use crate::dump_batch::{list_dump_files, update_live};

    let files = list_dump_files(folder)?;
    if files.is_empty() {
        anyhow::bail!("{}", crate::i18n::folder_no_dumps(lang));
    }

    let total = files.len() as u32;
    let mut stats = FolderColumnsStats {
        files_total: total,
        output_root: folder.display().to_string(),
        ..Default::default()
    };

    update_live(progress, |p| {
        p.reset();
        p.lang = lang;
    });

    for (i, source) in files.iter().enumerate() {
        if job_control::checkpoint(control) {
            stats.errors.push(crate::i18n::stopped_by_user(lang).into());
            break;
        }
        let name = source
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| source.display().to_string());
        let idx = (i + 1) as u32;

        update_live(progress, |p| p.begin_file(idx, total, &name));

        let output = default_output(source);

        match extract_from_sql_columns(source, &output, control) {
            Ok(s) => {
                stats.files_ok += 1;
                stats.lines_scanned += s.lines_scanned;
                stats.written += s.written;
                stats.skipped += s.skipped;
                stats.tables_found += s.tables_found;
                stats.inserts_parsed += s.inserts_parsed;
                update_live(progress, |p| {
                    p.files_ok = stats.files_ok;
                    p.files_failed = stats.files_failed;
                    p.lines_scanned = stats.lines_scanned;
                    p.written = stats.written;
                    p.skipped = stats.skipped;
                    p.tables_found = stats.tables_found;
                    p.inserts_parsed = stats.inserts_parsed;
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
    }

    update_live(progress, |p| p.active = false);
    Ok(stats)
}

fn is_constraint_segment(part: &str) -> bool {
    let u = part.trim().to_ascii_uppercase();
    u.starts_with("PRIMARY KEY")
        || u.starts_with("UNIQUE KEY")
        || u.starts_with("UNIQUE ")
        || u.starts_with("KEY ")
        || u.starts_with("INDEX ")
        || u.starts_with("CONSTRAINT ")
        || u.starts_with("FOREIGN KEY")
        || u.starts_with("FULLTEXT ")
        || u.starts_with("SPATIAL ")
        || u.starts_with("CHECK ")
}

fn is_sql_type(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "tinyint"
            | "smallint"
            | "mediumint"
            | "int"
            | "integer"
            | "bigint"
            | "decimal"
            | "numeric"
            | "float"
            | "double"
            | "real"
            | "bit"
            | "bool"
            | "boolean"
            | "char"
            | "varchar"
            | "binary"
            | "varbinary"
            | "tinyblob"
            | "blob"
            | "mediumblob"
            | "longblob"
            | "tinytext"
            | "text"
            | "mediumtext"
            | "longtext"
            | "enum"
            | "set"
            | "json"
            | "date"
            | "datetime"
            | "timestamp"
            | "time"
            | "year"
            | "uuid"
            | "point"
            | "linestring"
            | "polygon"
            | "geometry"
            | "multipoint"
            | "multilinestring"
            | "multipolygon"
            | "geometrycollection"
    )
}

fn first_identifier(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() {
        return None;
    }
    if part.starts_with('`') {
        let end = part[1..].find('`')? + 1;
        return Some(normalize_col(&part[1..end]));
    }
    if part.starts_with('"') {
        let end = part[1..].find('"')? + 1;
        return Some(normalize_col(&part[1..end]));
    }
    let word: String = part
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if word.is_empty() {
        None
    } else {
        Some(normalize_col(&word))
    }
}

fn parse_create_table_body(body: &str) -> Vec<String> {
    let mut cols = Vec::new();
    for part in sql_io::split_top_level_commas(body) {
        if is_constraint_segment(&part) {
            continue;
        }
        let Some(name) = first_identifier(&part) else {
            continue;
        };
        let rest = part.trim();
        let after_name = rest
            .find(&name)
            .map(|pos| &rest[pos + name.len()..])
            .unwrap_or(rest);
        let type_word = after_name
            .trim()
            .trim_start_matches('`')
            .split(|c: char| !c.is_ascii_alphanumeric())
            .find(|s| !s.is_empty())
            .unwrap_or("");
        if is_sql_type(type_word) || type_word.is_empty() {
            cols.push(name);
        }
    }
    cols
}

fn create_table_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?is)CREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?[`'"]?(\w+)[`'"]?\s*\("#)
            .expect("create table regex")
    })
}

fn parse_create_table_statement(sql: &str) -> Option<(String, TableSchema)> {
    let cap = create_table_re().captures(sql)?;
    let table = normalize_col(cap.get(1)?.as_str());
    let matched = cap.get(0)?;
    let open = matched.start() + matched.as_str().rfind('(')?;
    let close = sql_io::find_matching_paren(sql, open)?;
    let body = &sql[open + 1..close];
    let columns = parse_create_table_body(body);
    if columns.is_empty() {
        return None;
    }
    let (login_idx, pass_idx) = find_login_pass_indices(&columns);
    Some((
        table,
        TableSchema {
            columns,
            login_idx,
            pass_idx,
        },
    ))
}

fn parse_sql_value(raw: &str) -> Option<String> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    if s.eq_ignore_ascii_case("NULL") {
        return None;
    }
    if s.starts_with('\'') {
        return Some(parse_quoted_sql(s, '\''));
    }
    if s.starts_with('"') {
        return Some(parse_quoted_sql(s, '"'));
    }
    Some(s.trim_end_matches(';').to_string())
}

fn parse_quoted_sql(s: &str, quote: char) -> String {
    let inner = s.trim();
    let mut out = String::new();
    let mut chars = inner.chars().peekable();
    if chars.next() != Some(quote) {
        return inner.to_string();
    }
    while let Some(c) = chars.next() {
        if c == quote {
            if chars.peek() == Some(&quote) {
                chars.next();
                out.push(quote);
            } else {
                break;
            }
        } else if c == '\\' {
            if let Some(n) = chars.next() {
                out.push(n);
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn parse_value_tuple(tuple: &str) -> Vec<Option<String>> {
    let inner = tuple.trim().trim_start_matches('(').trim_end_matches(')');
    sql_io::split_top_level_commas(inner)
        .into_iter()
        .map(|v| parse_sql_value(&v))
        .collect()
}

fn resolve_indices(
    schema: Option<&TableSchema>,
    insert_cols: Option<&[String]>,
) -> Option<(usize, usize)> {
    if let Some(cols) = insert_cols {
        let (login_idx, pass_idx) = find_login_pass_indices(cols);
        if login_idx.is_some() && pass_idx.is_some() {
            return Some((login_idx?, pass_idx?));
        }
    }
    if let Some(s) = schema {
        if let (Some(l), Some(p)) = (s.login_idx, s.pass_idx) {
            return Some((l, p));
        }
    }
    None
}

fn write_pair(
    login: &str,
    pass: &str,
    out: &mut BufWriter<File>,
    stats: &mut SqlColumnsStats,
    seen: &mut HashSet<String>,
) -> Result<()> {
    let login = login.trim();
    let pass = pass.trim();
    if login.is_empty() || pass.is_empty() {
        stats.skipped += 1;
        return Ok(());
    }
    let line = format!("{login}:{pass}");
    if !seen.insert(line.clone()) {
        stats.duplicates += 1;
        return Ok(());
    }
    stats.written += 1;
    writeln!(out, "{line}")?;
    Ok(())
}

fn process_insert_statement(
    sql: &str,
    schemas: &HashMap<String, TableSchema>,
    out: &mut BufWriter<File>,
    stats: &mut SqlColumnsStats,
    seen: &mut HashSet<String>,
) -> Result<()> {
    let Some(parts) = sql_io::parse_insert_statement(sql) else {
        return Ok(());
    };
    stats.inserts_parsed += 1;
    let schema = schemas.get(&parts.table);
    let insert_cols = parts.columns.as_deref();
    let Some((login_idx, pass_idx)) = resolve_indices(schema, insert_cols) else {
        return Ok(());
    };

    for tuple in sql_io::split_value_tuples(&parts.values_part) {
        let values = parse_value_tuple(&tuple);
        let login = values.get(login_idx).and_then(|v| v.clone()).unwrap_or_default();
        let pass = values.get(pass_idx).and_then(|v| v.clone()).unwrap_or_default();
        write_pair(&login, &pass, out, stats, seen)?;
    }
    Ok(())
}

fn register_create_table(
    sql: &str,
    schemas: &mut HashMap<String, TableSchema>,
    stats: &mut SqlColumnsStats,
) {
    if let Some((table, schema)) = parse_create_table_statement(sql) {
        if schema.login_idx.is_some() && schema.pass_idx.is_some() {
            stats.tables_found += 1;
            tracing::debug!(table = %table, "sql columns table schema");
        }
        schemas.insert(table, schema);
    }
}

/// Извлекает login:password из SQL по именам колонок (не обязательно рядом в INSERT).
pub fn extract_from_sql_columns(
    source: &Path,
    output: &Path,
    control: Option<&JobControl>,
) -> Result<SqlColumnsStats> {
    tracing::info!("sql columns extract from {}", source.display());

    let mut stream = SqlStatementStream::open(source)?;
    let mut out = BufWriter::with_capacity(8 * 1024 * 1024, File::create(output)?);

    let mut stats = SqlColumnsStats {
        output_path: output.display().to_string(),
        ..Default::default()
    };
    let mut schemas: HashMap<String, TableSchema> = HashMap::new();
    let mut seen = HashSet::new();

    while let Some(stmt) = stream.next_statement()? {
        if job_control::checkpoint(control) {
            break;
        }
        stats.lines_scanned = stream.lines_scanned();
        let upper = stmt.to_ascii_uppercase();
        if upper.contains("CREATE TABLE") {
            register_create_table(&stmt, &mut schemas, &mut stats);
        }
        if upper.contains("INSERT INTO") {
            process_insert_statement(&stmt, &schemas, &mut out, &mut stats, &mut seen)?;
        }

        if stats.lines_scanned.is_multiple_of(500_000) {
            tracing::info!(
                lines = stats.lines_scanned,
                written = stats.written,
                tables = stats.tables_found,
                "sql columns progress"
            );
        }
    }

    out.flush()?;
    tracing::info!(
        written = stats.written,
        skipped = stats.skipped,
        tables = stats.tables_found,
        output = %stats.output_path,
        "sql columns done"
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
        std::env::temp_dir().join(format!("lhf_sqlcol_{name}_{nanos}.sql"))
    }

    #[test]
    fn parse_values_non_adjacent_columns() {
        let sql = r"
CREATE TABLE `accounts` (
  `id` int(11) NOT NULL,
  `password` varchar(255) DEFAULT NULL,
  `status` int(11) DEFAULT NULL,
  `username` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
);
INSERT INTO `accounts` (`id`, `password`, `status`, `username`) VALUES
(1, 'secret123', 1, 'alice@test.com'),
(2, 'pass456', 1, 'bob@test.com');
";
        let path = temp_path("src");
        let out = temp_path("out");
        std::fs::write(&path, sql).unwrap();

        let stats = extract_from_sql_columns(&path, &out, None).unwrap();
        assert_eq!(stats.written, 2);
        let text = std::fs::read_to_string(&out).unwrap();
        assert!(text.contains("alice@test.com:secret123"));
        assert!(text.contains("bob@test.com:pass456"));

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(out);
    }

    #[test]
    fn parse_insert_without_column_list() {
        let sql = r"
CREATE TABLE users (
  id int,
  login varchar(100),
  pwd varchar(100)
);
INSERT INTO users VALUES (1,'user1@test.com','mypassword');
";
        let path = temp_path("src2");
        let out = temp_path("out2");
        std::fs::write(&path, sql).unwrap();

        let stats = extract_from_sql_columns(&path, &out, None).unwrap();
        assert_eq!(stats.written, 1);
        let text = std::fs::read_to_string(&out).unwrap();
        assert!(text.contains("user1@test.com:mypassword"));

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(out);
    }

    #[test]
    fn skips_hashed_password_without_trash_file() {
        let sql = r"
CREATE TABLE u (email varchar(100), password varchar(100));
INSERT INTO u VALUES ('a@test.com','0123456789abcdef0123456789abcdef');
INSERT INTO u VALUES ('b@test.com','plainpass');
";
        let path = temp_path("src3");
        let out = temp_path("out3");
        std::fs::write(&path, sql).unwrap();

        let stats = extract_from_sql_columns(&path, &out, None).unwrap();
        assert_eq!(stats.written, 2);
        let text = std::fs::read_to_string(&out).unwrap();
        assert!(text.contains("0123456789abcdef0123456789abcdef"));
        assert!(text.contains("b@test.com:plainpass"));

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(out);
    }

    #[test]
    fn parse_one_line_insert_with_multiple_tuples() {
        let sql = r"CREATE TABLE u (email varchar(100), password varchar(100)); INSERT INTO u (email, password) VALUES ('a@test.com','pass1'), ('b@test.com','pass2'), ('c@test.com','pass3');";
        let path = temp_path("oneline");
        let out = temp_path("oneline_out");
        std::fs::write(&path, sql).unwrap();

        let stats = extract_from_sql_columns(&path, &out, None).unwrap();
        assert_eq!(stats.written, 3);
        let text = std::fs::read_to_string(&out).unwrap();
        assert!(text.contains("a@test.com:pass1"));
        assert!(text.contains("b@test.com:pass2"));
        assert!(text.contains("c@test.com:pass3"));

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(out);
    }
}
