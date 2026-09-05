use std::path::PathBuf;

use anyhow::Result;

use super::archive::{self, materialize_input};

pub fn resolve_input_files(input_path: &str) -> Result<Vec<PathBuf>> {
    let (files, _temp) = materialize_input(input_path)?;
    Ok(files)
}

pub fn is_text_ext(path: &std::path::Path) -> bool {
    archive::is_text_ext(path)
}

pub fn is_archive_path(path: &std::path::Path) -> bool {
    archive::is_archive_path(path)
}
