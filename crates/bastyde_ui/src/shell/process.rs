// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Path helpers shared by the surfaces that reach outside this window.
//!
//! This module used to own `spawn_new_process`, and existed because **four** unrelated
//! surfaces needed it — the project switcher popover, the Launcher's recents, the
//! open-a-backup redirect and the backups list — back when Skribisto was one process per
//! project and "open that project" meant launching a second copy of itself.
//!
//! Phase 4 removed all four. Skribisto is single-instance now: a spawned child would elect,
//! find this very process as the primary, hand the path straight back over a socket and exit,
//! so every one of those callers goes through
//! [`crate::shell::windows::open_or_focus_project`] instead. What is left here is the two
//! helpers that were never about spawning at all.

use std::path::Path;

/// Best-effort canonical form for comparing project paths across the open registry (which
/// stores canonical paths) and the recents list.
///
/// Falls back to the input unchanged when the path does not resolve — an unreachable network
/// mount or a deleted project still has to compare *somehow*, and comparing the raw string is
/// better than dropping the entry.
pub(crate) fn canon(path: &str) -> String {
    std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

/// Open the file manager at `path`'s containing folder (best-effort, per platform).
pub(crate) fn reveal_in_file_manager(path: &str) {
    let target = Path::new(path)
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(&target).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(&target).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(&target).spawn();
}
