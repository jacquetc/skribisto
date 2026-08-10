// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where this installation keeps project media.
//!
//! Every backend call that reads or writes a project's image bytes takes a
//! *media root* — the app-level directory, not the project's own folder inside
//! it. The split matters at load time: which directory a project uses depends on
//! its `unique_id`, and that id is still inside the file being opened, so only
//! the use case can resolve it (see `skrib_format::media`).
//!
//! Not a view-model — a path helper with no state.

use std::path::PathBuf;

/// The app's media root, `<data_dir>/media`.
///
/// `data_dir` rather than `cache_dir`: bytes land here the moment a writer
/// inserts an image, before any save, so this is where an unsaved picture lives
/// until the project is written. A cache directory is something the OS may
/// reclaim, and reclaiming it would destroy work the writer has not yet saved.
///
/// Returns an empty path when the platform offers no data directory. The
/// backend treats that as "no media", so a project on such a platform still
/// opens — without its images — rather than failing to open at all.
pub fn media_root() -> PathBuf {
    crate::identity::app_paths()
        .map(|paths| paths.data_dir().join("media"))
        .unwrap_or_default()
}

/// The media root as the `String` the DTOs carry.
pub fn media_root_string() -> String {
    media_root().to_string_lossy().into_owned()
}
