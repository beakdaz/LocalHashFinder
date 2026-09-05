use regex::Regex;
use std::sync::LazyLock;

static EMAIL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^[^@\s]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap());

/// Parse `email:password` or ULP-style line; returns `(email, password)`.
pub fn parse_email_pass(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    if let Some((login, pass)) = line.rsplit_once(':') {
        let login = login.trim();
        let pass = pass.trim();
        if EMAIL_RE.is_match(login) && !pass.is_empty() {
            return Some((login.to_string(), pass.to_string()));
        }
    }
    None
}

pub fn email_domain(email: &str) -> Option<String> {
    let at = email.rfind('@')?;
    let domain = email[at + 1..].trim().to_lowercase();
    if domain.is_empty() {
        None
    } else {
        Some(domain)
    }
}
