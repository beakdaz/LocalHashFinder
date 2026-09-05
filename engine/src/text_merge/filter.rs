//! Line filtering — keep only plain passwords (one per line, no colons).

use crate::parser::is_md5_or_sha1_hex;

/// Garbage category for a rejected line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GarbageKind {
    Empty,
    Short,
    Null,
    Hash,
    Combo,
    Comment,
    SpecialOnly,
}

/// True when the line is hash-related junk for a password dictionary.
pub fn is_hash_garbage(line: &str) -> bool {
    let t = line.trim();
    if is_md5_or_sha1_hex(t) {
        return true;
    }
    if let Some(i) = t.find(':') {
        let head = t[..i].trim();
        let tail = t[i + 1..].trim();
        if is_md5_or_sha1_hex(head) {
            return true;
        }
        if head.contains('@') && is_md5_or_sha1_hex(tail) {
            return true;
        }
    }
    false
}

/// Classify a line after trim. Returns `None` if the line is a plain password to keep.
pub fn classify_garbage(line: &str, min_len: usize) -> Option<GarbageKind> {
    let t = line.trim();
    if t.is_empty() {
        return Some(GarbageKind::Empty);
    }
    if t.starts_with('#') || t.starts_with(';') {
        return Some(GarbageKind::Comment);
    }
    if t.len() < min_len {
        return Some(GarbageKind::Short);
    }
    if t.to_ascii_uppercase().contains(":NULL") {
        return Some(GarbageKind::Null);
    }
    if is_hash_garbage(t) {
        return Some(GarbageKind::Hash);
    }
    if t.contains(':') {
        return Some(GarbageKind::Combo);
    }
    if is_special_only(t) {
        return Some(GarbageKind::SpecialOnly);
    }
    None
}

/// Returns `true` when the line should be discarded.
pub fn is_garbage_line(line: &str, min_len: usize) -> bool {
    classify_garbage(line, min_len).is_some()
}

fn is_special_only(s: &str) -> bool {
    !s.chars().any(|c| c.is_alphanumeric())
}

/// Strip UTF-8 BOM if present.
pub fn strip_bom(s: &str) -> &str {
    s.strip_prefix('\u{FEFF}').unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_whitespace() {
        assert_eq!(classify_garbage("", 3), Some(GarbageKind::Empty));
        assert_eq!(classify_garbage("   ", 3), Some(GarbageKind::Empty));
    }

    #[test]
    fn short_lines() {
        assert_eq!(classify_garbage("ab", 3), Some(GarbageKind::Short));
        assert_eq!(classify_garbage("abc", 3), None);
    }

    #[test]
    fn null_trash_case_insensitive() {
        assert_eq!(
            classify_garbage("user@gmail.com:NULL", 3),
            Some(GarbageKind::Null)
        );
        assert_eq!(classify_garbage("hash:null", 3), Some(GarbageKind::Null));
    }

    #[test]
    fn hash_trash() {
        assert_eq!(
            classify_garbage("5f4dcc3b5aa765d61d8327deb882cf99", 3),
            Some(GarbageKind::Hash)
        );
        assert_eq!(
            classify_garbage("5f4dcc3b5aa765d61d8327deb882cf99:password", 3),
            Some(GarbageKind::Hash)
        );
        assert_eq!(
            classify_garbage("user@test.com:0123456789abcdef0123456789abcdef", 3),
            Some(GarbageKind::Hash)
        );
    }

    #[test]
    fn combo_trash() {
        assert_eq!(
            classify_garbage("user@gmail.com:pass123", 3),
            Some(GarbageKind::Combo)
        );
        assert_eq!(classify_garbage("admin:qwerty", 3), Some(GarbageKind::Combo));
        assert_eq!(
            classify_garbage("domain.com:password", 3),
            Some(GarbageKind::Combo)
        );
    }

    #[test]
    fn plain_passwords_kept() {
        assert_eq!(classify_garbage("password123", 3), None);
        assert_eq!(classify_garbage("qwerty", 3), None);
        assert_eq!(classify_garbage("MyP@ss!2024", 3), None);
    }

    #[test]
    fn only_plain_passwords_remain() {
        let keep = ["password123", "qwerty", "MyP@ss!2024", "abc"];
        for line in keep {
            assert_eq!(classify_garbage(line, 3), None, "should keep: {line}");
        }

        let drop = [
            ("user:pass", GarbageKind::Combo),
            ("a@b.com:secret", GarbageKind::Combo),
            (
                "5f4dcc3b5aa765d61d8327deb882cf99:pass",
                GarbageKind::Hash,
            ),
            (
                "5f4dcc3b5aa765d61d8327deb882cf99",
                GarbageKind::Hash,
            ),
            ("user@gmail.com:NULL", GarbageKind::Null),
            ("ab", GarbageKind::Short),
            ("", GarbageKind::Empty),
            (":::", GarbageKind::Combo),
        ];
        for (line, kind) in drop {
            assert_eq!(
                classify_garbage(line, 3),
                Some(kind),
                "should drop as {kind:?}: {line}"
            );
        }
    }

    #[test]
    fn comments_skipped() {
        assert_eq!(classify_garbage("# header", 3), Some(GarbageKind::Comment));
    }
}
