//! Track output files per tab: open folder, delete, zip.

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result, bail};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub struct TabResults {
    paths: Mutex<Vec<PathBuf>>,
}

impl TabResults {
    pub fn shared() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            paths: Mutex::new(Vec::new()),
        })
    }

    pub fn set_paths<I: IntoIterator<Item = PathBuf>>(&self, paths: I) {
        *self.paths.lock().unwrap() = paths
            .into_iter()
            .filter(|p| !p.as_os_str().is_empty())
            .collect();
    }

    pub fn set_from_strings<I: IntoIterator<Item = String>>(&self, paths: I) {
        self.set_paths(paths.into_iter().map(PathBuf::from));
    }

    pub fn has_files(&self) -> bool {
        !self.paths.lock().unwrap().is_empty()
    }

    pub fn folder(&self) -> Option<PathBuf> {
        self.paths
            .lock()
            .unwrap()
            .first()
            .and_then(|p| p.parent().map(Path::to_path_buf))
    }

    pub fn existing_files(&self) -> Vec<PathBuf> {
        self.paths
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.is_file())
            .cloned()
            .collect()
    }

    pub fn delete_all(&self) -> Result<usize> {
        let paths: Vec<PathBuf> = self.paths.lock().unwrap().drain(..).collect();
        let mut n = 0usize;
        for p in paths {
            if p.is_file() {
                std::fs::remove_file(&p).with_context(|| format!("delete {}", p.display()))?;
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn zip_pack(&self, empty_msg: &str) -> Result<PathBuf> {
        let files: Vec<PathBuf> = self.existing_files();
        if files.is_empty() {
            bail!("{empty_msg}");
        }
        let folder = files[0]
            .parent()
            .context("no parent folder")?
            .to_path_buf();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let zip_path = folder.join(format!("lhf_results_{stamp}.zip"));

        let out = File::create(&zip_path)
            .with_context(|| format!("создание архива {}", zip_path.display()))?;
        let mut zip = ZipWriter::new(out);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

        for path in &files {
            if path == &zip_path {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .with_context(|| format!("имя файла {}", path.display()))?;
            zip.start_file(name, options)
                .with_context(|| format!("добавление в архив {}", path.display()))?;
            let mut input = File::open(path)
                .with_context(|| format!("чтение {}", path.display()))?;
            std::io::copy(&mut input, &mut zip)
                .with_context(|| format!("запись в архив {}", path.display()))?;
        }

        zip.finish()
            .context("завершение архива")?;
        Ok(zip_path)
    }

    /// Concatenate non-trash `.txt` results into one file next to the outputs.
    pub fn merge_text(&self) -> Result<PathBuf> {
        let files: Vec<PathBuf> = self
            .existing_files()
            .into_iter()
            .filter(|p| is_mergeable_result(p))
            .collect();
        if files.is_empty() {
            bail!("нет текстовых результатов для склейки");
        }

        let folder = files[0]
            .parent()
            .context("no parent folder")?
            .to_path_buf();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let out_path = folder.join(format!("lhf_merged_{stamp}.txt"));

        let mut out = File::create(&out_path)
            .with_context(|| format!("создание {}", out_path.display()))?;
        let mut lines_total = 0u64;

        for (i, path) in files.iter().enumerate() {
            if i > 0 {
                writeln!(out)?;
            }
            let reader = BufReader::new(
                File::open(path).with_context(|| format!("чтение {}", path.display()))?,
            );
            for line in reader.lines() {
                let line = line.with_context(|| format!("строка в {}", path.display()))?;
                writeln!(out, "{line}")?;
                lines_total += 1;
            }
        }

        tracing::info!(
            "merged {} files, {} lines -> {}",
            files.len(),
            lines_total,
            out_path.display()
        );
        Ok(out_path)
    }
}

fn is_trash_result(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.to_ascii_lowercase().contains("_trash"))
}

fn is_generated_output(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| {
            let lower = n.to_ascii_lowercase();
            lower.starts_with("lhf_merged_") || lower.starts_with("lhf_results_")
        })
}

fn is_mergeable_result(path: &Path) -> bool {
    if is_trash_result(path) || is_generated_output(path) {
        return false;
    }
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("txt"))
}

pub fn collect_suffixes_in_folder(folder: &Path, suffixes: &[&str]) -> Vec<PathBuf> {
    let Ok(read) = std::fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = read
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && suffixes.iter().any(|s| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(s))
                })
        })
        .collect();
    out.sort();
    out
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
        std::env::temp_dir().join(format!("lhf_zip_{name}_{nanos}"))
    }

    #[test]
    fn zip_pack_creates_archive_with_files() -> Result<()> {
        let dir = temp_dir("pack");
        std::fs::create_dir_all(&dir)?;
        let a = dir.join("a_good.txt");
        let b = dir.join("b_nohash.txt");
        std::fs::write(&a, "line1\n")?;
        std::fs::write(&b, "line2\n")?;

        let results = TabResults {
            paths: Mutex::new(vec![a.clone(), b.clone()]),
        };
        let zip_path = results.zip_pack("no result files")?;
        assert!(zip_path.is_file());
        assert!(zip_path.file_name().unwrap().to_str().unwrap().starts_with("lhf_results_"));

        let _ = std::fs::remove_file(zip_path);
        let _ = std::fs::remove_file(a);
        let _ = std::fs::remove_file(b);
        let _ = std::fs::remove_dir(dir);
        Ok(())
    }

    #[test]
    fn merge_text_skips_trash_and_concatenates() -> Result<()> {
        let dir = temp_dir("merge");
        std::fs::create_dir_all(&dir)?;
        let good = dir.join("a_good.txt");
        let nohash = dir.join("a_nohash.txt");
        let trash = dir.join("a_trash.txt");
        std::fs::write(&good, "found:pass\n")?;
        std::fs::write(&nohash, "missing\n")?;
        std::fs::write(&trash, "junk\n")?;

        let results = TabResults {
            paths: Mutex::new(vec![good, nohash, trash]),
        };
        let merged = results.merge_text()?;
        let body = std::fs::read_to_string(&merged)?;
        assert!(body.contains("found:pass"));
        assert!(body.contains("missing"));
        assert!(!body.contains("junk"));

        let _ = std::fs::remove_file(merged);
        let _ = std::fs::remove_file(dir.join("a_good.txt"));
        let _ = std::fs::remove_file(dir.join("a_nohash.txt"));
        let _ = std::fs::remove_file(dir.join("a_trash.txt"));
        let _ = std::fs::remove_dir(dir);
        Ok(())
    }
}
