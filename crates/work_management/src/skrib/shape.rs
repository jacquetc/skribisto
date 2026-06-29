//! Detect what a path holds: a zipped `.skrib`, an exploded folder, or a legacy
//! SQLite `.skrib`.

use anyhow::{Context, Result};
use std::io::Read;
use std::path::{Path, PathBuf};

pub const MANIFEST_NAME: &str = "project.skrib";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkribShape {
    ZipFile,
    ExplodedFolder,
    LegacySqlite,
}

/// Classify `path`. Accepts a folder, a folder's `project.skrib`, a zip `.skrib`,
/// or a legacy SQLite `.skrib`.
pub fn detect_shape(path: &str) -> Result<SkribShape> {
    let p = Path::new(path);

    if p.is_dir() {
        return if p.join(MANIFEST_NAME).is_file() {
            Ok(SkribShape::ExplodedFolder)
        } else {
            anyhow::bail!("directory '{path}' is not a Skribisto project (no {MANIFEST_NAME})")
        };
    }

    if !p.exists() {
        anyhow::bail!("path '{path}' does not exist");
    }

    // A folder project's entry point is `project.skrib` (RON text).
    if p.file_name().and_then(|n| n.to_str()) == Some(MANIFEST_NAME) {
        return Ok(SkribShape::ExplodedFolder);
    }

    let mut magic = [0u8; 16];
    let n = {
        let mut f = std::fs::File::open(p).with_context(|| format!("opening '{path}'"))?;
        f.read(&mut magic).with_context(|| format!("reading '{path}'"))?
    };
    let head = &magic[..n];

    if head.starts_with(b"PK\x03\x04") || head.starts_with(b"PK\x05\x06") {
        return Ok(SkribShape::ZipFile);
    }
    if head.starts_with(b"SQLite format 3\0") {
        return Ok(SkribShape::LegacySqlite);
    }
    anyhow::bail!("'{path}' is not a recognised .skrib file (unknown header)")
}

/// The bundle root directory for an exploded-folder path (the folder itself, or
/// the parent of a `project.skrib`).
pub fn folder_root(path: &str) -> PathBuf {
    let p = Path::new(path);
    if p.is_dir() {
        p.to_path_buf()
    } else if p.file_name().and_then(|n| n.to_str()) == Some(MANIFEST_NAME) {
        p.parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."))
    } else {
        p.to_path_buf()
    }
}
