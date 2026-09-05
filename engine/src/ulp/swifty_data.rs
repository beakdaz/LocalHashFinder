//! Recovered SwiftyULP string tables (via runtime nmKikxo27T + Ktqk decode).
//! Source: SwiftyULP.exe embedded resource IIcl2uv1D8DVp94l7q.HaonIHdScDIdOeJG68

/// Domain / token blacklist (Clean → Remove blacklisted lines).
pub const DOMAIN_BLACKLIST: &[&str] = &[
    // disposable / temp mail (heuristic defaults retained)
    "tempmail.org",
    "10minutemail.com",
    "guerrillamail.com",
    "mailinator.com",
    // recovered from SwiftyULP predefined word blacklist
    "[unknown]",
    "[default]",
    "[logs]",
    "t.me",
    "@celestialadmin",
    "@starlink",
    "@txt",
    "fake",
    "none",
    "null",
    "unknown",
];

/// Weak password dictionary (Clean → Remove weak credentials).
pub const WEAK_PASSWORDS: &[&str] = &[
    "000000",
    "0987654321",
    "111111",
    "123123",
    "1234",
    "12345",
    "123456",
    "1234567",
    "12345678",
    "123456789",
    "1234567890",
    "1q2w3e4r",
    "654321",
    "aa123456",
    "abc123",
    "admin",
    "administrator",
    "baseball",
    "dragon",
    "football",
    "guest",
    "iloveyou",
    "letmein",
    "login",
    "master",
    "monkey",
    "pass",
    "passw0rd",
    "password",
    "password1",
    "princess",
    "pwd",
    "qwerty",
    "qwerty123",
    "root",
    "shadow",
    "sunshine",
    "superman",
    "test",
    "welcome",
];

/// Misc / cleaning / extraction module labels from SwiftyULP UI.
pub const MISC_MODULE_NAMES: &[&str] = &[
    "General Cleaner",
    "Login:Pass Cleaner",
    "URL:Login:Pass Cleaner",
    "URL:Login:Pass Extractor",
    "URL:Login:Pass Sorter",
    "Credential Type",
    "Country",
    "Keyword",
    "Splitter",
    "Randomizer",
];

/// Stealer / capture line prefixes (Clean → Remove capture).
pub const CAPTURE_PREFIXES: &[&str] = &[
    "soft:",
    "browser:",
    "host:",
    "login:",
    "password:",
    "user:",
    "pass:",
    "url:",
];
