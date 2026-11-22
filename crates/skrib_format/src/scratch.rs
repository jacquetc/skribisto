// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where a zip read extracts and a zip write stages.
//!
//! Both used to go to the system's temporary folder, which on many Linux systems
//! is a RAM file system: a project with a few hundred megabytes of pictures and
//! attached files cost that much memory on every open and on every autosave.
//! The app points this at a folder under its own data directory instead
//! ([`set_root`](crate::scratch::set_root)), which is on a disk, and on the same
//! file system as the media directory, so a [`Blob`] moves between the two as a
//! hard link rather than a copy.
//!
//! A program that never calls [`set_root`](crate::scratch::set_root) (a test, a
//! command-line tool) keeps the system's temporary folder.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime};

use tempfile::TempDir;

/// Name prefix of every scratch folder, so the sweep removes only its own.
const PREFIX: &str = "skrib-scratch-";

/// How old a scratch folder must be before the sweep takes it for one a crash
/// left behind. Far longer than any read or write runs.
const STALE_AFTER: Duration = Duration::from_secs(24 * 60 * 60);

static ROOT: OnceLock<PathBuf> = OnceLock::new();

/// Put every scratch folder under `dir` from now on, and remove the ones a crash
/// left there.
///
/// Set once per process: a second call keeps the first folder, since folders
/// already handed out live under it.
pub fn set_root(dir: &Path) -> io::Result<()> {
    std::fs::create_dir_all(dir)?;
    if ROOT.set(dir.to_path_buf()).is_ok() {
        sweep(dir);
    }
    Ok(())
}

/// A fresh scratch folder, removed when the returned handle is dropped.
pub fn tempdir() -> io::Result<TempDir> {
    let builder = {
        let mut b = tempfile::Builder::new();
        b.prefix(PREFIX);
        b
    };
    match ROOT.get() {
        Some(root) => builder.tempdir_in(root),
        None => builder.tempdir(),
    }
}

/// Remove the scratch folders under `dir` older than [`STALE_AFTER`]. Best effort:
/// a folder that cannot be removed now is tried again on the next start.
fn sweep(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        let name = entry.file_name();
        if !name.to_string_lossy().starts_with(PREFIX) {
            continue;
        }
        let old = entry
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| now.duration_since(t).ok())
            .is_some_and(|age| age > STALE_AFTER);
        if old {
            let _ = std::fs::remove_dir_all(entry.path());
        }
    }
}
