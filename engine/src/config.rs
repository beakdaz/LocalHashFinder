use std::fs;
use std::path::{Path, PathBuf};

/// Directory where LocalHashFinder.exe lives (portable config lives here too).
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn config_path() -> PathBuf {
    exe_dir().join("LocalHashFinder.cfg")
}

pub fn default_lmdb_path() -> PathBuf {
    exe_dir().join("data").join("hashdb.lmdb")
}

/// Ensure `data/` (parent of `hashdb.lmdb`) exists next to exe or config.
pub fn ensure_lmdb_parent_dirs(lmdb_path: &Path) -> std::io::Result<()> {
    if let Some(parent) = lmdb_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Accept LMDB dir, `data/` folder, project root, or any folder for a new DB.
pub fn normalize_lmdb_path(selected: &Path) -> PathBuf {
    if selected.file_name().and_then(|n| n.to_str()) == Some("hashdb.lmdb") {
        return selected.to_path_buf();
    }

    let direct = selected.join("hashdb.lmdb");
    if direct.is_dir() {
        return direct;
    }

    let nested = selected.join("data").join("hashdb.lmdb");
    if nested.is_dir() {
        return nested;
    }

    direct
}

pub fn load_lmdb_path() -> Option<PathBuf> {
    let text = fs::read_to_string(config_path()).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("lmdb_path=") {
            let p = PathBuf::from(rest.trim().trim_matches('"'));
            if !p.as_os_str().is_empty() {
                return Some(p);
            }
        }
    }
    None
}

pub fn save_lmdb_path(path: &Path) -> std::io::Result<()> {
    let content = format!(
        "# LocalHashFinder — путь к LMDB (рядом с exe лежит этот файл)\r\nlmdb_path={}\r\n",
        path.display()
    );
    fs::write(config_path(), content)
}

pub fn resolve_lmdb_path(cli_override: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_override {
        return normalize_lmdb_path(&p);
    }
    if let Some(p) = load_lmdb_path() {
        return p;
    }
    default_lmdb_path()
}

/// Fixed UI scale — not user-adjustable.
pub const UI_ZOOM_FIXED: f32 = 1.10;

pub fn load_ui_zoom() -> f32 {
    UI_ZOOM_FIXED
}

pub fn load_ui_lang() -> crate::i18n::Lang {
    let Ok(text) = fs::read_to_string(config_path()) else {
        return crate::i18n::Lang::Ru;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("ui_lang=") {
            return crate::i18n::Lang::parse(rest.trim());
        }
    }
    crate::i18n::Lang::Ru
}

pub fn save_ui_lang(lang: crate::i18n::Lang) -> std::io::Result<()> {
    let path = config_path();
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_else(|_| "# LocalHashFinder — settings\r\n".to_string())
        .lines()
        .map(str::to_string)
        .collect();
    let entry = format!("ui_lang={}", lang.code());
    if let Some(idx) = lines.iter().position(|l| l.trim_start().starts_with("ui_lang=")) {
        lines[idx] = entry;
    } else {
        lines.push(entry);
    }
    let body = if lines.last().is_some_and(|l| l.is_empty()) {
        lines.join("\r\n")
    } else {
        format!("{}\r\n", lines.join("\r\n"))
    };
    fs::write(path, body)
}

pub fn save_ui_zoom(_zoom: f32) -> std::io::Result<()> {
    let zoom = UI_ZOOM_FIXED;
    let path = config_path();
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_else(|_| {
            "# LocalHashFinder — настройки\r\n".to_string()
        })
        .lines()
        .map(str::to_string)
        .collect();
    let entry = format!("ui_zoom={zoom:.2}");
    if let Some(idx) = lines.iter().position(|l| l.trim_start().starts_with("ui_zoom=")) {
        lines[idx] = entry;
    } else {
        lines.push(entry);
    }
    let body = if lines.last().is_some_and(|l| l.is_empty()) {
        lines.join("\r\n")
    } else {
        format!("{}\r\n", lines.join("\r\n"))
    };
    fs::write(path, body)
}
