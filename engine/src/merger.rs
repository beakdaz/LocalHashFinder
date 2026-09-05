use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::job_control::{self, JobControl};
use crate::parser::{is_trash_input_line, is_trash_password, parse_hash_pass_line, parse_mail_hash_line};

#[derive(Clone, Default, Debug)]
pub struct MergeStats {
    pub total: u64,
    pub merged: u64,
    pub nohash: u64,
    pub bad: u64,
    pub trash: u64,
    pub plain_path: String,
    pub nohash_path: String,
    pub trash_path: String,
}

pub fn output_paths(mail: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let dir = mail.parent().unwrap_or_else(|| Path::new("."));
    let stem = mail.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    (
        dir.join(format!("{stem}_plain.txt")),
        dir.join(format!("{stem}_plain_nohash.txt")),
        dir.join(format!("{stem}_trash.txt")),
    )
}

/// Load `hash:plainpass` map from dehash / _good file.
pub fn load_dehash_map(path: &Path) -> Result<HashMap<String, String>> {
    tracing::info!("loading dehash map from {}", path.display());
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let reader = std::io::BufReader::with_capacity(16 * 1024 * 1024, file);
    let mut map = HashMap::new();
    let mut first_line = true;
    for line in reader.lines() {
        let mut line = line?;
        if first_line {
            first_line = false;
            if let Some(stripped) = line.strip_prefix('\u{feff}') {
                line = stripped.to_string();
            }
        }
        if let Some((hash, pass)) = parse_hash_pass_line(&line) {
            map.insert(hash, pass);
        }
    }
    tracing::info!("dehash map: {} entries", map.len());
    Ok(map)
}

/// Merge `mail:hashedpass` + `hash:dehashedpass` → `mail:plainpass`.
pub fn merge_files(
    mail_path: &Path,
    dehash_path: &Path,
    control: Option<&JobControl>,
) -> Result<MergeStats> {
    let map = load_dehash_map(dehash_path)?;
    let (plain_path, nohash_path, trash_path) = output_paths(mail_path);

    let mail_file = File::open(mail_path)
        .with_context(|| format!("open mail file {}", mail_path.display()))?;
    let reader = std::io::BufReader::with_capacity(16 * 1024 * 1024, mail_file);

    let mut plain_w = BufWriter::with_capacity(8 * 1024 * 1024, File::create(&plain_path)?);
    let mut nohash_w = BufWriter::with_capacity(8 * 1024 * 1024, File::create(&nohash_path)?);
    let mut trash_w = BufWriter::with_capacity(8 * 1024 * 1024, File::create(&trash_path)?);

    let mut stats = MergeStats {
        plain_path: plain_path.display().to_string(),
        nohash_path: nohash_path.display().to_string(),
        trash_path: trash_path.display().to_string(),
        ..Default::default()
    };

    let mut first_line = true;
    for line in reader.lines() {
        if job_control::checkpoint(control) {
            break;
        }
        let mut line = line?;
        if first_line {
            first_line = false;
            if let Some(stripped) = line.strip_prefix('\u{feff}') {
                line = stripped.to_string();
            }
        }
        let raw = line.trim();
        if raw.is_empty() {
            continue;
        }
        stats.total += 1;

        if is_trash_input_line(raw) {
            stats.trash += 1;
            writeln!(trash_w, "{raw}")?;
            continue;
        }

        let Some((email, hash)) = parse_mail_hash_line(raw) else {
            stats.bad += 1;
            writeln!(trash_w, "{raw}")?;
            continue;
        };

        if let Some(pass) = map.get(&hash) {
            let out = format!("{email}:{pass}");
            if is_trash_password(pass) {
                stats.trash += 1;
                writeln!(trash_w, "{out}")?;
            } else {
                stats.merged += 1;
                writeln!(plain_w, "{out}")?;
            }
        } else {
            stats.nohash += 1;
            writeln!(nohash_w, "{email}:NULL")?;
        }
    }

    plain_w.flush()?;
    nohash_w.flush()?;
    trash_w.flush()?;
    Ok(stats)
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
        std::env::temp_dir().join(format!("lhf_merge_{name}_{nanos}"))
    }

    #[test]
    fn merge_sha1_mail_with_dehash_map() -> Result<()> {
        let dir = temp_dir("sha1");
        std::fs::create_dir_all(&dir)?;
        let mail = dir.join("mails.txt");
        let dehash = dir.join("found_good.txt");
        std::fs::write(
            &mail,
            "anton@test.com:dabd7bfb00119f1ee6baaddbb5e2150308b70599\n",
        )?;
        std::fs::write(
            &dehash,
            "dabd7bfb00119f1ee6baaddbb5e2150308b70599:MyPassword\n",
        )?;

        let stats = merge_files(&mail, &dehash, None)?;
        assert_eq!(stats.merged, 1);
        assert_eq!(stats.nohash, 0);
        assert_eq!(stats.bad, 0);

        let plain = std::fs::read_to_string(stats.plain_path)?;
        assert!(plain.contains("anton@test.com:MyPassword"));

        let _ = std::fs::remove_dir_all(dir);
        Ok(())
    }
}
