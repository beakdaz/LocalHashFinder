use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use zip::read::ZipArchive;

const ARCHIVE_EXTS: &[&str] = &["zip", "7z", "rar"];

pub fn is_archive_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| ARCHIVE_EXTS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn is_text_ext(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .as_deref(),
        Some("txt") | Some("log") | Some("lst") | Some("ulp") | Some("csv")
    )
}

/// Extract archives under `input` into a fresh temp directory; returns paths to text files.
pub fn materialize_input(input: &str) -> Result<(Vec<PathBuf>, Option<tempfile::TempDir>)> {
    let path = Path::new(input.trim());
    if !path.exists() {
        bail!("path not found: {}", path.display());
    }

    if path.is_file() {
        if is_archive_path(path) {
            let temp = tempfile::TempDir::new().context("create temp dir for archive")?;
            extract_archive(path, temp.path())?;
            let files = collect_text_files_recursive(temp.path())?;
            return Ok((files, Some(temp)));
        }
        return Ok((vec![path.to_path_buf()], None));
    }

    let mut archives = Vec::new();
    let mut text_files = Vec::new();
    walk_input_tree(path, &mut text_files, &mut archives)?;

    if archives.is_empty() {
        text_files.sort();
        return Ok((text_files, None));
    }

    let temp = tempfile::TempDir::new().context("create temp dir for archives")?;
    for (i, arch) in archives.iter().enumerate() {
        let sub = temp.path().join(format!("arch_{i}"));
        fs::create_dir_all(&sub)?;
        if let Err(e) = extract_archive(arch, &sub) {
            tracing::warn!("skip archive {}: {e}", arch.display());
        }
    }
    for f in &text_files {
        if f.starts_with(path) {
            // keep direct text files from source tree
        }
    }
    let mut out = collect_text_files_recursive(temp.path())?;
    out.extend(text_files);
    out.sort();
    out.dedup();
    Ok((out, Some(temp)))
}

fn walk_input_tree(dir: &Path, text: &mut Vec<PathBuf>, archives: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            walk_input_tree(&p, text, archives)?;
        } else if is_archive_path(&p) {
            archives.push(p);
        } else if is_text_ext(&p) {
            text.push(p);
        }
    }
    Ok(())
}

fn collect_text_files_recursive(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            out.extend(collect_text_files_recursive(&p)?);
        } else if is_text_ext(&p) {
            out.push(p);
        }
    }
    Ok(out)
}

pub fn extract_archive(archive: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    let ext = archive
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "zip" => extract_zip(archive, dest),
        "7z" => extract_sevenz(archive, dest),
        "rar" => extract_rar(archive, dest),
        _ => bail!("unsupported archive: {}", archive.display()),
    }
}

fn extract_zip(archive: &Path, dest: &Path) -> Result<()> {
    let file = File::open(archive).with_context(|| format!("open {}", archive.display()))?;
    let mut zip = ZipArchive::new(file).context("read zip")?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).with_context(|| format!("zip entry {i}"))?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().replace('\\', "/");
        let out_path = dest.join(safe_archive_path(&name));
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if !is_text_ext(&out_path) {
            continue;
        }
        let mut out = File::create(&out_path)?;
        copy(&mut entry, &mut out)?;
    }
    Ok(())
}

fn extract_sevenz(archive: &Path, dest: &Path) -> Result<()> {
    sevenz_rust::decompress_file(archive, dest)
        .map_err(|e| anyhow::anyhow!("7z extract {}: {e}", archive.display()))
}

fn extract_rar(_archive: &Path, _dest: &Path) -> Result<()> {
    bail!(
        "RAR extraction is not enabled in this build (linker conflict with unrar_sys). \
         Extract the archive manually or convert to .zip/.7z."
    );
}

fn safe_archive_path(name: &str) -> PathBuf {
    let mut parts = Vec::new();
    for part in name.split(['/', '\\']) {
        if part.is_empty() || part == "." || part == ".." {
            continue;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        PathBuf::from("entry.txt")
    } else {
        parts.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn zip_extracts_text_file() {
        let dir = std::env::temp_dir().join("lhf_ulp_zip_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let zip_path = dir.join("test.zip");
        {
            use zip::write::SimpleFileOptions;
            let f = File::create(&zip_path).unwrap();
            let mut w = zip::ZipWriter::new(f);
            w.start_file("data/in.txt", SimpleFileOptions::default())
                .unwrap();
            w.write_all(b"user@gmail.com:pass\n").unwrap();
            w.finish().unwrap();
        }
        let out = dir.join("out");
        extract_zip(&zip_path, &out).unwrap();
        assert!(out.join("data/in.txt").exists());
        let _ = fs::remove_dir_all(&dir);
    }
}
