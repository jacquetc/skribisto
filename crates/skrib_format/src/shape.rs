// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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
        f.read(&mut magic)
            .with_context(|| format!("reading '{path}'"))?
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
        p.parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        p.to_path_buf()
    }
}

/// The one spelling of a project's path everything keyed on that path agrees on.
///
/// A folder-shaped project can be named two ways: by its folder, or by the
/// `project.skrib` manifest inside it. The second is what a file dialog can pick
/// (a dialog picks files), the first is what Save As ▸ folder records — and every
/// consumer that keys on the path has to agree, or the same book is two projects:
/// `WorkInfo.file_name`, the recents list, the open registry, a window's
/// persistence id, where a backup goes and what it is named. Before this
/// existed, a folder project created in one session and reopened through its
/// manifest in the next was loaded a second time into an independent Work, with
/// two sessions autosaving one folder, and a backup written "next to the
/// project" landed *inside* it.
///
/// Collapses the manifest spelling onto the folder; every other path (a zip, a
/// legacy file, a path that does not exist yet) comes back unchanged. Purely
/// syntactic — no filesystem access — so it answers the same for a path that is
/// not on disk yet and is cheap enough to call at every door.
pub fn canonical_project_path(path: &str) -> String {
    let p = Path::new(path);
    if p.file_name().and_then(|n| n.to_str()) == Some(MANIFEST_NAME)
        && let Some(parent) = p.parent().filter(|d| !d.as_os_str().is_empty())
    {
        return parent.to_string_lossy().into_owned();
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_manifest_path_collapses_onto_its_folder() {
        assert_eq!(
            canonical_project_path("/home/j/Textes/raphaël-et-mireïa/project.skrib"),
            "/home/j/Textes/raphaël-et-mireïa"
        );
    }

    #[test]
    fn every_other_spelling_is_returned_unchanged() {
        for p in [
            "/home/j/Textes/raphaël-et-mireïa",
            "/home/j/Novel.skrib",
            "relative/Novel.skrib",
            "project.skrib",
        ] {
            assert_eq!(canonical_project_path(p), p, "{p}");
        }
    }

    #[test]
    fn it_is_idempotent() {
        let once = canonical_project_path("/x/Novel/project.skrib");
        assert_eq!(canonical_project_path(&once), once);
    }
}
