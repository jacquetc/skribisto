// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where the pictures an import brings wait while the writer reviews it.
//!
//! Not a view-model: a folder and its lifetime. The analysis writes each picture a document
//! shows into it (`import_management::analyze_document_import`, one `<hash>.<ext>` file
//! each), and `apply_document_import` copies the accepted ones into the project's media
//! directory. The folder is the wizard's, never the project's: nothing reaches a project's
//! own `assets/` before the writer presses *Import*.
//!
//! One folder per analysis, made when the analysis starts and removed when the
//! [`PictureStaging`] holding it is dropped: when the wizard is reset or starts another
//! analysis, when its window closes, and once an import has landed, the project's copies
//! being all that is needed from then on. The analysis writes into the folder and never
//! makes it: one still running after its wizard let the folder go would otherwise make it
//! again, and leave its pictures there. A folder left behind by a crash is swept the next
//! time an import starts, once it is a week old, a margin no review still open in another
//! window comes near.
//!
//! Under the app's data directory rather than the system's temporary one, which is often
//! memory: a manuscript's photographs can run to hundreds of megabytes.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// How old a folder left behind has to be before a new import removes it.
pub const STALE_AFTER: Duration = Duration::from_secs(7 * 24 * 60 * 60);

/// The folder every import's pictures wait in, one folder each below it.
pub fn default_root() -> PathBuf {
    crate::identity::app_paths()
        .map(|paths| paths.data_dir().join("import-pictures"))
        .unwrap_or_else(|| std::env::temp_dir().join("skribisto-import-pictures"))
}

/// One analysis's folder, removed with this.
///
/// Made here, empty, and never by the analysis (see the module doc). A folder that cannot
/// be made is not an error yet: an import bringing no picture never needs it, and one that
/// brings a picture is refused by the analysis for want of it.
#[derive(Debug)]
pub struct PictureStaging {
    dir: PathBuf,
}

impl PictureStaging {
    /// A fresh folder below `root`, sweeping the ones a crash left there first.
    pub fn new_in(root: &Path) -> Self {
        sweep_stale(root, SystemTime::now(), STALE_AFTER);
        let dir = root.join(uuid::Uuid::new_v4().to_string());
        let _ = std::fs::create_dir_all(&dir);
        PictureStaging { dir }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// The folder, as the DTOs carry it.
    pub fn dir_string(&self) -> String {
        self.dir.to_string_lossy().into_owned()
    }
}

impl Drop for PictureStaging {
    fn drop(&mut self) {
        // A folder never made, or already gone, is exactly what this was going to leave.
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// Remove the folders below `root` older than `max_age` at `now`.
///
/// Only folders: whatever else someone put there is not this module's. A folder whose age
/// cannot be read is left alone.
pub fn sweep_stale(root: &Path, now: SystemTime, max_age: Duration) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if !metadata.is_dir() {
            continue;
        }
        let stale = metadata
            .modified()
            .ok()
            .and_then(|modified| now.duration_since(modified).ok())
            .is_some_and(|age| age > max_age);
        if stale {
            let _ = std::fs::remove_dir_all(entry.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_goes_with_its_staging() {
        let root = tempfile::tempdir().expect("scratch");
        let staging = PictureStaging::new_in(root.path());
        assert!(
            staging.dir().is_dir(),
            "made for the analysis to write into"
        );
        std::fs::write(staging.dir().join("abc.png"), b"png").expect("a picture");
        let dir = staging.dir().to_path_buf();
        assert!(dir.starts_with(root.path()));
        drop(staging);
        assert!(!dir.exists(), "removed with its staging");
    }

    #[test]
    fn two_analyses_never_share_a_folder() {
        let root = tempfile::tempdir().expect("scratch");
        let a = PictureStaging::new_in(root.path());
        let b = PictureStaging::new_in(root.path());
        assert_ne!(a.dir(), b.dir());
    }

    #[test]
    fn a_folder_a_crash_left_is_swept_once_it_is_old() {
        let root = tempfile::tempdir().expect("scratch");
        let left = root.path().join("left-behind");
        std::fs::create_dir_all(&left).expect("folder");
        std::fs::write(root.path().join("not-ours.txt"), b"x").expect("file");

        sweep_stale(root.path(), SystemTime::now(), STALE_AFTER);
        assert!(
            left.exists(),
            "a recent folder may be another window's review"
        );

        sweep_stale(
            root.path(),
            SystemTime::now() + STALE_AFTER + Duration::from_secs(60),
            STALE_AFTER,
        );
        assert!(!left.exists(), "an old one is a crash's leftover");
        assert!(
            root.path().join("not-ours.txt").exists(),
            "only folders are swept"
        );
    }
}
