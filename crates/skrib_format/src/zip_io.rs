// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Zip read/write. The zip is a single binary file, so it is always rewritten
//! whole (diff-minimal only matters for the exploded folder under git). Open
//! extracts to a tempdir and reuses the folder reader; save writes the folder
//! form into a tempdir, packs it deterministically, and atomically renames the
//! finished archive over the target.

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

use super::bundle::WorkBundle;
use super::folder_io::{read_folder, write_folder};
use super::writer::persist_durably;

pub fn write_zip(target: &Path, bundle: &WorkBundle) -> Result<()> {
    let staging = tempfile::tempdir().context("creating staging dir for zip save")?;
    write_folder(staging.path(), bundle)?;

    let parent = target.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(p) = parent {
        std::fs::create_dir_all(p).with_context(|| format!("creating {}", p.display()))?;
    }
    let mut tmp = match parent {
        Some(p) => NamedTempFile::new_in(p),
        None => NamedTempFile::new(),
    }
    .context("temp file for zip")?;

    // Write through the `NamedTempFile`'s own fd (not a second, independent
    // `File::create` on the same path) so the bytes we fsync in
    // `persist_durably` are demonstrably the bytes we just wrote.
    zip_dir(staging.path(), tmp.as_file_mut())?;
    persist_durably(tmp, target)
}

pub fn read_zip(path: &Path) -> Result<WorkBundle> {
    let dir = tempfile::tempdir().context("creating extract dir")?;
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut archive =
        zip::ZipArchive::new(file).with_context(|| format!("reading zip {}", path.display()))?;
    archive
        .extract(dir.path())
        .with_context(|| format!("extracting zip {}", path.display()))?;
    read_folder(dir.path())
}

/// Pack `src_dir`'s tree into a fresh zip, written through `target` (an
/// already-open file — typically a [`NamedTempFile`]'s own fd, so the bytes
/// fsynced afterwards are demonstrably the bytes just written, rather than
/// through a second independent handle on the same path).
pub fn zip_dir(src_dir: &Path, target: &mut File) -> Result<()> {
    let mut zw = zip::ZipWriter::new(target);
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    // Already-compressed payloads are stored, not deflated again.
    //
    // A JPEG, PNG or WebP is entropy-coded already: running deflate over one
    // spends real CPU to gain a fraction of a percent, occasionally growing the
    // entry. That cost lands on the autosave path, which rebuilds the whole
    // archive every few seconds, so for a project with photographs in it this is
    // the difference between an autosave that is proportional to the prose and
    // one that is proportional to the pictures.
    let stored = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for entry in WalkDir::new(src_dir).sort_by_file_name() {
        let entry = entry.context("walking staging dir")?;
        let path = entry.path();
        let rel = path
            .strip_prefix(src_dir)
            .context("stripping staging prefix")?;
        if rel.as_os_str().is_empty() {
            continue;
        }
        let name = rel
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non-UTF8 path {}", rel.display()))?
            .replace('\\', "/");
        if entry.file_type().is_dir() {
            zw.add_directory(format!("{name}/"), opts)?;
        } else {
            let already_compressed = matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("png" | "jpg" | "jpeg" | "webp" | "gif")
            );
            zw.start_file(name, if already_compressed { stored } else { opts })?;
            let bytes =
                std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
            zw.write_all(&bytes)?;
        }
    }
    zw.finish()?;
    Ok(())
}
