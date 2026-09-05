//! Shared SQL streaming: statements by `;` and INSERT rows by `(…), (…)`.

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::Result;
use encoding_rs::WINDOWS_1252;

const UTF8_BOM: &[u8] = b"\xEF\xBB\xBF";
const ENCODING_SAMPLE: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Windows1252,
}

#[derive(Clone, Debug)]
pub struct InsertParts {
    pub table: String,
    pub columns: Option<Vec<String>>,
    pub values_part: String,
}

pub struct SqlStatementStream {
    reader: BufReader<File>,
    encoding: TextEncoding,
    pending: Vec<u8>,
    stmt_buf: String,
    lines_scanned: u64,
    eof: bool,
}

impl SqlStatementStream {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::with_capacity(16 * 1024 * 1024, file);
        let mut sample = vec![0_u8; ENCODING_SAMPLE];
        let n = reader.read(&mut sample)?;
        sample.truncate(n);
        let encoding = detect_encoding(&sample);
        let (initial, pending) = split_complete_lines(strip_utf8_bom(&sample));
        let initial_line_count = initial.len() as u64;
        let mut stmt_buf = String::new();
        for line in initial {
            stmt_buf.push_str(&decode_line_bytes(&line, encoding));
            stmt_buf.push('\n');
        }
        Ok(Self {
            reader,
            encoding,
            pending,
            stmt_buf,
            lines_scanned: initial_line_count,
            eof: false,
        })
    }

    pub fn lines_scanned(&self) -> u64 {
        self.lines_scanned
    }

    /// Next complete SQL statement (ends with `;` outside quotes), or `None` at EOF.
    pub fn next_statement(&mut self) -> Result<Option<String>> {
        loop {
            if let Some(stmt) = take_one_statement(&mut self.stmt_buf) {
                return Ok(Some(stmt));
            }

            if self.eof {
                let tail = self.stmt_buf.trim();
                if tail.is_empty() {
                    return Ok(None);
                }
                let rest = std::mem::take(&mut self.stmt_buf);
                return Ok(Some(rest));
            }

            let mut line_buf = Vec::new();
            line_buf.append(&mut self.pending);
            let bytes_read = self.reader.read_until(b'\n', &mut line_buf)?;
            if bytes_read == 0 && line_buf.is_empty() {
                self.eof = true;
                continue;
            }

            self.lines_scanned += 1;
            let line = decode_line_bytes(&line_buf, self.encoding);
            self.stmt_buf.push_str(&line);
            if bytes_read == 0 {
                self.eof = true;
            }
        }
    }
}

/// Split buffered text into complete statements; leaves incomplete tail in `buf`.
pub fn drain_complete_statements(buf: &mut String) -> Vec<String> {
    let mut out = Vec::new();
    while let Some(stmt) = take_one_statement(buf) {
        out.push(stmt);
    }
    out
}

fn take_one_statement(buf: &mut String) -> Option<String> {
    let mut in_str = None;
    let mut escape = false;
    let mut stmt_start = 0usize;
    let bytes = buf.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if in_str.is_some() {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if Some(c) == in_str {
                in_str = None;
            }
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' {
            in_str = Some(c);
            i += 1;
            continue;
        }
        if c == ';' {
            let piece = buf[stmt_start..i].trim();
            if !piece.is_empty() {
                let stmt = piece.to_string();
                buf.drain(..=i);
                return Some(stmt);
            }
            stmt_start = i + 1;
        }
        i += 1;
    }
    if stmt_start > 0 {
        buf.drain(..stmt_start);
    }
    None
}

/// `(a,b), (c,d)` → `["(a,b)", "(c,d)"]`
pub fn split_value_tuples(values_part: &str) -> Vec<String> {
    let mut tuples = Vec::new();
    let s = values_part.trim().trim_end_matches(';');
    let mut start = None;
    let mut depth = 0i32;
    let mut in_str = None;
    let mut escape = false;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        let c = bytes[i] as char;
        if in_str.is_some() {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if Some(c) == in_str {
                in_str = None;
            }
            continue;
        }
        if c == '\'' || c == '"' {
            in_str = Some(c);
        } else if c == '(' {
            if depth == 0 {
                start = Some(i);
            }
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                if let Some(st) = start {
                    tuples.push(s[st..=i].to_string());
                    start = None;
                }
            }
        }
    }
    tuples
}

/// Chunks to scan — each INSERT row separately; other statements as one block.
pub fn extraction_chunks(statement: &str) -> Vec<String> {
    if let Some(parts) = parse_insert_statement(statement) {
        let tuples = split_value_tuples(&parts.values_part);
        if !tuples.is_empty() {
            return tuples;
        }
    }
    vec![statement.to_string()]
}

pub fn parse_insert_statement(sql: &str) -> Option<InsertParts> {
    let upper = sql.to_ascii_uppercase();
    let insert_at = upper.find("INSERT INTO")?;
    let rest = &sql[insert_at + "INSERT INTO".len()..];
    let rest = rest.trim_start();

    let (table, after_table) = read_ident(rest)?;
    let after_table = after_table.trim_start();

    let (columns, after_cols) = if after_table.starts_with('(') {
        let close = find_matching_paren(after_table, 0)?;
        let col_body = &after_table[1..close];
        let cols = split_top_level_commas(col_body)
            .into_iter()
            .filter_map(|p| read_ident(p.trim()).map(|(id, _)| normalize_ident(&id)))
            .collect::<Vec<_>>();
        let after = after_table[close + 1..].trim_start();
        (Some(cols), after)
    } else {
        (None, after_table)
    };

    let upper_cols = after_cols.to_ascii_uppercase();
    let values_at = upper_cols.find("VALUES")?;
    let values_part = after_cols[values_at + "VALUES".len()..].trim();
    if values_part.is_empty() {
        return None;
    }

    Some(InsertParts {
        table: normalize_ident(&table),
        columns: columns.filter(|c| !c.is_empty()),
        values_part: values_part.to_string(),
    })
}

fn normalize_ident(s: &str) -> String {
    s.trim().trim_matches('`').trim_matches('"').to_ascii_lowercase()
}

fn read_ident(s: &str) -> Option<(String, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    if s.starts_with('`') {
        let end = s[1..].find('`')? + 1;
        return Some((s[1..end].to_string(), &s[end + 1..]));
    }
    if s.starts_with('"') {
        let end = s[1..].find('"')? + 1;
        return Some((s[1..end].to_string(), &s[end + 1..]));
    }
    let end = s
        .char_indices()
        .find(|(_, c)| !c.is_ascii_alphanumeric() && *c != '_')
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    Some((s[..end].to_string(), &s[end..]))
}

pub fn find_matching_paren(s: &str, open: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut in_str = None;
    let mut escape = false;
    let mut i = open;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if in_str.is_some() {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if Some(c) == in_str {
                in_str = None;
            }
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' {
            in_str = Some(c);
        } else if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

pub fn split_top_level_commas(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    let mut in_str = None;
    let mut escape = false;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        let c = bytes[i] as char;
        if in_str.is_some() {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if Some(c) == in_str {
                in_str = None;
            }
            continue;
        }
        if c == '\'' || c == '"' {
            in_str = Some(c);
        } else if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
        } else if c == ',' && depth == 0 {
            parts.push(s[start..i].to_string());
            start = i + 1;
        }
    }
    parts.push(s[start..].to_string());
    parts
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

fn detect_encoding(sample: &[u8]) -> TextEncoding {
    let sample = strip_utf8_bom(sample);
    if sample.is_empty() || std::str::from_utf8(sample).is_ok() {
        TextEncoding::Utf8
    } else {
        TextEncoding::Windows1252
    }
}

pub fn decode_line_bytes(bytes: &[u8], encoding: TextEncoding) -> String {
    let bytes = trim_crlf(bytes);
    match encoding {
        TextEncoding::Utf8 => std::str::from_utf8(bytes)
            .map(str::to_owned)
            .unwrap_or_else(|_| WINDOWS_1252.decode(bytes).0.into_owned()),
        TextEncoding::Windows1252 => WINDOWS_1252.decode(bytes).0.into_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_statements_on_one_line() {
        let mut buf = "CREATE TABLE t (id int); INSERT INTO t VALUES (1,'a');".into();
        let stmts = drain_complete_statements(&mut buf);
        assert_eq!(stmts.len(), 2);
        assert!(stmts[0].contains("CREATE TABLE"));
        assert!(stmts[1].contains("INSERT INTO"));
        assert!(buf.is_empty());
    }

    #[test]
    fn split_multiple_insert_tuples_one_line() {
        let sql = "INSERT INTO u (login, pass) VALUES ('a@x.com','p1'), ('b@y.com','p2'), ('c@z.com','p3');";
        let parts = parse_insert_statement(sql).unwrap();
        let tuples = split_value_tuples(&parts.values_part);
        assert_eq!(tuples.len(), 3);
        assert!(tuples[0].contains("a@x.com"));
        assert!(tuples[2].contains("c@z.com"));
    }

    #[test]
    fn extraction_chunks_per_row() {
        let sql = "INSERT INTO t VALUES ('mail1','hash1'), ('mail2','hash2');";
        let chunks = extraction_chunks(sql);
        assert_eq!(chunks.len(), 2);
    }

    #[test]
    fn split_statements_one_physical_line_no_newline() {
        let path = std::env::temp_dir().join(format!(
            "lhf_sql_io_oneline_{}.sql",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(
            &path,
            "CREATE TABLE t (id int); INSERT INTO t VALUES (1,'a'), (2,'b');",
        )
        .unwrap();
        let mut stream = SqlStatementStream::open(&path).unwrap();
        let first = stream.next_statement().unwrap().unwrap();
        assert!(first.contains("CREATE TABLE"));
        let second = stream.next_statement().unwrap().unwrap();
        assert!(second.contains("INSERT INTO"));
        assert!(second.contains("(2,'b')"));
        assert!(stream.next_statement().unwrap().is_none());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn semicolon_inside_string() {
        let mut buf = "INSERT INTO t VALUES ('a;b', 'c');".into();
        let stmts = drain_complete_statements(&mut buf);
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].contains("a;b"));
    }
}
