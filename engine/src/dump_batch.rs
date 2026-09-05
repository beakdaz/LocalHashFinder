//! Shared helpers: list dump files in a folder, live batch progress.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};

use crate::i18n::{self, Lang};

pub const DUMP_EXTENSIONS: &[&str] = &["sql", "txt", "dump"];

#[derive(Clone, Default, Debug)]
pub struct BatchLiveProgress {
    pub active: bool,
    pub files_total: u32,
    pub file_index: u32,
    pub current_file: String,
    pub files_ok: u32,
    pub files_failed: u32,
    pub lines_scanned: u64,
    pub total: u64,
    pub md5: u64,
    pub sha1: u64,
    pub trash: u64,
    pub written: u64,
    pub skipped: u64,
    pub tables_found: u64,
    pub inserts_parsed: u64,
    pub lang: Lang,
}

impl BatchLiveProgress {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn begin_file(&mut self, index: u32, total: u32, name: &str) {
        self.active = true;
        self.files_total = total;
        self.file_index = index;
        self.current_file = name.to_string();
    }

    pub fn sql_status_lines(&self) -> Vec<String> {
        vec![
            i18n::batch_file_line(
                self.lang,
                self.file_index,
                self.files_total,
                &self.current_file,
            ),
            i18n::batch_sql_stats_line(
                self.lang,
                self.files_ok,
                self.files_failed,
                self.total,
                self.md5,
                self.sha1,
                self.trash,
            ),
            i18n::batch_lines_scanned(self.lang, self.lines_scanned),
        ]
    }

    pub fn columns_status_lines(&self) -> Vec<String> {
        vec![
            i18n::batch_file_line(
                self.lang,
                self.file_index,
                self.files_total,
                &self.current_file,
            ),
            i18n::batch_columns_stats_line(
                self.lang,
                self.files_ok,
                self.files_failed,
                self.written,
                self.skipped,
                self.tables_found,
            ),
            i18n::batch_inserts_lines(self.lang, self.inserts_parsed, self.lines_scanned),
        ]
    }
}

pub fn update_live(progress: Option<&Arc<Mutex<BatchLiveProgress>>>, f: impl FnOnce(&mut BatchLiveProgress)) {
    if let Some(p) = progress {
        f(&mut p.lock().unwrap());
    }
}

pub fn is_dump_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| {
            let ext = ext.to_ascii_lowercase();
            DUMP_EXTENSIONS.iter().any(|e| *e == ext)
        })
        .unwrap_or(false)
}

/// All dump files in `folder` (non-recursive), sorted by name.
pub fn list_dump_files(folder: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(folder)
        .with_context(|| format!("read dir {}", folder.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_dump_file(p))
        .collect();
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("lhf_batch_{name}_{nanos}"))
    }

    #[test]
    fn list_dump_files_skips_other_extensions() -> Result<()> {
        let dir = temp_dir("list");
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("a.sql"), "x")?;
        std::fs::write(dir.join("b.txt"), "x")?;
        std::fs::write(dir.join("c.log"), "x")?;
        std::fs::write(dir.join("readme.md"), "x")?;

        let files = list_dump_files(&dir)?;
        assert_eq!(files.len(), 2);

        let _ = std::fs::remove_dir_all(dir);
        Ok(())
    }

    #[test]
    fn live_progress_sql_lines() {
        let mut p = BatchLiveProgress::default();
        p.begin_file(2, 5, "site.sql");
        p.files_ok = 1;
        p.total = 100;
        p.md5 = 80;
        p.sha1 = 20;
        let lines = p.sql_status_lines();
        assert!(lines[0].contains("2/5"));
        assert!(lines[1].contains("email:hash: 100"));
    }
}
