use regex::Regex;
use std::sync::LazyLock;

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^[^@\s]+@[a-z0-9.-]+\.[a-z]{2,}$").expect("email re")
});

static PHONE_PASS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[^:]+:([a-zA-Z][a-zA-Z0-9_-]{2,19}):(\d{10,15})$").expect("phone re")
});

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cred {
    pub url: String,
    pub login: String,
    pub pass: String,
    pub raw: String,
}

impl Cred {
    pub fn line(&self) -> String {
        let login = self.login.trim();
        let pass = self.pass.trim();
        if !self.url.trim().is_empty() {
            format!("{}:{login}:{pass}", self.url.trim())
        } else {
            format!("{login}:{pass}")
        }
    }
}

fn normalize_url(raw: &str) -> String {
    let raw = raw.trim();
    if raw.is_empty() {
        return String::new();
    }
    if raw.contains("://") {
        raw.to_string()
    } else {
        format!("https://{raw}")
    }
}

fn finish_entry(raw: &str, url: &str, login: &str, pass: &str) -> Option<Cred> {
    let url = url.trim();
    let login = login.trim();
    let pass = pass.trim();
    if url.is_empty() || login.is_empty() {
        return None;
    }
    let host = url_host(url)?;
    if host.is_empty() {
        return None;
    }
    Some(Cred {
        raw: raw.to_string(),
        url: url.to_string(),
        login: login.to_string(),
        pass: pass.to_string(),
    })
}

fn url_host(raw: &str) -> Option<String> {
    let norm = normalize_url(raw);
    let without_scheme = norm
        .split("://")
        .nth(1)
        .unwrap_or(norm.as_str());
    let host = without_scheme
        .split(['/', ':', '?', '#'])
        .next()
        .unwrap_or("")
        .trim();
    if host.is_empty() || !host.contains('.') {
        None
    } else {
        Some(host.to_string())
    }
}

/// Parse `url:login:pass` and related ULP formats.
pub fn parse_ulp_line(line: &str) -> Option<Cred> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    // url login:pass
    if let Some(i) = line.find(' ') {
        if i > 0 {
            let (left, right) = line.split_at(i);
            let left = left.trim();
            let right = right.trim();
            if let Some((login, pass)) = right.split_once(':') {
                return finish_entry(line, left, login, pass);
            }
        }
    }

    // url,login,pass
    if line.contains(',') {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let pass = parts[2..].join(",");
            return finish_entry(line, parts[0], parts[1], &pass);
        }
    }

    // url:login:pass (last two colons)
    if line.matches(':').count() >= 2 {
        if let Some(last) = line.rfind(':') {
            if let Some(mid) = line[..last].rfind(':') {
                if mid > 0 {
                    return finish_entry(
                        line,
                        &line[..mid],
                        &line[mid + 1..last],
                        &line[last + 1..],
                    );
                }
            }
        }
    }

    None
}

/// Parse `login:pass` without URL.
pub fn parse_login_pass(line: &str) -> Option<Cred> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    if line.matches(':').count() >= 2 {
        if line.contains("://")
            || line
                .find(':')
                .zip(line.find('.'))
                .is_some_and(|(c, d)| c < d)
        {
            if let Some(c) = parse_ulp_line(line) {
                return Some(c);
            }
        }
    }
    let (login, pass) = line.rsplit_once(':')?;
    let login = login.trim();
    let pass = pass.trim();
    if login.is_empty() || pass.is_empty() {
        return None;
    }
    Some(Cred {
        raw: line.to_string(),
        login: login.to_string(),
        pass: pass.to_string(),
        ..Default::default()
    })
}

/// Auto-detect ULP or login:pass.
pub fn parse_line(line: &str) -> Option<Cred> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    parse_ulp_line(line).or_else(|| parse_login_pass(line))
}

pub fn is_email_login(s: &str) -> bool {
    EMAIL_RE.is_match(s.trim())
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

pub fn match_phone_pass(line: &str) -> bool {
    PHONE_PASS_RE.is_match(line.trim())
}

pub fn strip_url_protocol(url: &str) -> String {
    let url = url.trim();
    for prefix in ["https://", "http://", "ftp://", "ftps://"] {
        if url.len() >= prefix.len() && url[..prefix.len()].eq_ignore_ascii_case(prefix) {
            return url[prefix.len()..].trim_start_matches('/').to_string();
        }
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ulp_triple_colon() {
        let c = parse_ulp_line("https://site.com:user@mail.com:secret").unwrap();
        assert_eq!(c.url, "https://site.com");
        assert_eq!(c.login, "user@mail.com");
        assert_eq!(c.pass, "secret");
    }

    #[test]
    fn parse_login_pass_only() {
        let c = parse_login_pass("user@gmail.com:pass123").unwrap();
        assert_eq!(c.login, "user@gmail.com");
        assert_eq!(c.pass, "pass123");
    }
}
