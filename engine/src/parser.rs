use anyhow::Result;

/// MD5 hex length.
pub const MD5_HEX_LEN: usize = 32;
/// SHA1 hex length.
pub const SHA1_HEX_LEN: usize = 40;

/// Пароли длиной ≤ этого значения попадают в `_trash.txt`.
pub const MAX_TRASH_PASSWORD_LEN: usize = 4;

/// `32` or `40` lowercase/uppercase hex chars (MD5 / SHA1).
pub fn is_md5_or_sha1_hex(s: &str) -> bool {
    let h = s.trim();
    (h.len() == MD5_HEX_LEN || h.len() == SHA1_HEX_LEN)
        && h.bytes().all(|b| b.is_ascii_hexdigit())
}

fn is_hex_hash(s: &str) -> bool {
    is_md5_or_sha1_hex(s)
}

/// Parse `hash:password` line for DB import.
pub fn parse_db_line(line: &str) -> Option<(String, String)> {
    let raw = line.trim();
    if raw.is_empty() || raw.starts_with('#') || raw.starts_with(';') {
        return None;
    }

    let (hash_part, password) = if let Some(i) = raw.find(':') {
        (raw[..i].trim(), raw[i + 1..].to_string())
    } else if let Some(i) = raw.find('|') {
        (raw[..i].trim(), raw[i + 1..].to_string())
    } else if let Some(i) = raw.find('\t') {
        (raw[..i].trim(), raw[i + 1..].to_string())
    } else {
        let mut parts = raw.split_whitespace();
        let h = parts.next()?;
        let p = parts.next().unwrap_or("");
        (h, p.to_string())
    };

    let mut hash = hash_part.to_ascii_lowercase();
    if hash.contains('@') {
        if let Some(i) = hash.rfind(':') {
            hash = hash[i + 1..].to_string();
        }
    }

    if is_hex_hash(&hash) {
        Some((hash, password))
    } else {
        None
    }
}

#[derive(Clone)]
pub struct InputRow {
    pub raw: String,
    pub hash: String,
    pub prefix: String,
    pub bad: bool,
    pub trash: bool,
}

/// `email:null`, `email:` без хеша.
pub fn is_trash_input_line(line: &str) -> bool {
    let raw = line.trim();
    if raw.is_empty() || !raw.contains('@') {
        return false;
    }
    let Some(i) = raw.rfind(':') else {
        return false;
    };
    let head = raw[..i].trim();
    if !head.contains('@') {
        return false;
    }
    let tail = raw[i + 1..].trim();
    tail.is_empty() || tail.eq_ignore_ascii_case("null")
}

/// Слишком короткий или пустой пароль после расшифровки.
pub fn is_trash_password(pass: &str) -> bool {
    pass.is_empty() || pass.chars().count() <= MAX_TRASH_PASSWORD_LEN
}

/// Parse input file line (`email:hash`, bare hash, etc.).
pub fn parse_input_line(line: &str) -> Option<InputRow> {
    let raw = line.trim().to_string();
    if raw.is_empty() || raw.starts_with('#') || raw.starts_with(';') {
        return None;
    }

    if is_trash_input_line(&raw) {
        return Some(InputRow {
            raw,
            hash: String::new(),
            prefix: String::new(),
            bad: false,
            trash: true,
        });
    }

    if is_hex_hash(&raw) {
        return Some(InputRow {
            raw: raw.clone(),
            hash: raw.to_ascii_lowercase(),
            prefix: String::new(),
            bad: false,
            trash: false,
        });
    }

    if let Some(i) = raw.rfind(':') {
        let tail = raw[i + 1..].trim();
        if is_hex_hash(tail) {
            return Some(InputRow {
                raw: raw.clone(),
                hash: tail.to_ascii_lowercase(),
                prefix: raw[..=i].to_string(),
                bad: false,
                trash: false,
            });
        }
    }

    for part in raw.split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '|') {
        let p = part.trim();
        if is_hex_hash(p) {
            return Some(InputRow {
                raw: raw.clone(),
                hash: p.to_ascii_lowercase(),
                prefix: raw.clone(),
                bad: false,
                trash: false,
            });
        }
    }

    Some(InputRow {
        raw,
        hash: String::new(),
        prefix: String::new(),
        bad: true,
        trash: false,
    })
}

pub fn hash_to_key(hex_hash: &str) -> Result<[u8; 16]> {
    let hex_hash = hex_hash.trim();
    if hex_hash.len() != MD5_HEX_LEN {
        anyhow::bail!(
            "LMDB key expects {MD5_HEX_LEN}-char MD5 hex, got {}",
            hex_hash.len()
        );
    }
    let mut key = [0u8; 16];
    hex::decode_to_slice(hex_hash, &mut key)?;
    Ok(key)
}

/// `email:md5` or `email:sha1` — mail list with password hash.
pub fn parse_mail_hash_line(line: &str) -> Option<(String, String)> {
    let raw = line.trim();
    if raw.is_empty() || raw.starts_with('#') || raw.starts_with(';') {
        return None;
    }

    let colon = raw.rfind(':')?;
    let email = raw[..colon].trim();
    let hash = raw[colon + 1..].trim().to_ascii_lowercase();

    if email.contains('@') && is_hex_hash(&hash) {
        Some((email.to_string(), hash))
    } else {
        None
    }
}

/// `hash:plainpass` из _good.txt / файла расшифровки.
pub fn parse_hash_pass_line(line: &str) -> Option<(String, String)> {
    parse_db_line(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trash_input_lines() {
        assert!(is_trash_input_line("user@test.com:null"));
        assert!(is_trash_input_line("user@test.com:"));
        assert!(!is_trash_input_line(
            "user@test.com:0123456789abcdef0123456789abcdef"
        ));
        assert!(!is_trash_input_line(
            "user@test.com:dabd7bfb00119f1ee6baaddbb5e2150308b70599"
        ));
    }

    #[test]
    fn parse_mail_hash_md5_and_sha1() {
        let md5 = parse_mail_hash_line(
            "anton@test.com:0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        assert_eq!(md5.0, "anton@test.com");
        assert_eq!(md5.1, "0123456789abcdef0123456789abcdef");

        let sha1 = parse_mail_hash_line(
            "anton@test.com:dabd7bfb00119f1ee6baaddbb5e2150308b70599",
        )
        .unwrap();
        assert_eq!(sha1.0, "anton@test.com");
        assert_eq!(sha1.1, "dabd7bfb00119f1ee6baaddbb5e2150308b70599");
    }

    #[test]
    fn parse_hash_pass_sha1() {
        let (hash, pass) = parse_hash_pass_line(
            "dabd7bfb00119f1ee6baaddbb5e2150308b70599:secret123",
        )
        .unwrap();
        assert_eq!(hash, "dabd7bfb00119f1ee6baaddbb5e2150308b70599");
        assert_eq!(pass, "secret123");
    }

    #[test]
    fn trash_passwords() {
        assert!(is_trash_password(""));
        assert!(is_trash_password("11"));
        assert!(is_trash_password("123"));
        assert!(is_trash_password("1234"));
        assert!(is_trash_password("амп"));
        assert!(is_trash_password("zxcd"));
        assert!(!is_trash_password("12345"));
        assert!(!is_trash_password("password"));
    }
}
