// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Path helpers shared by the surfaces that reach outside this window.
//!
//! Skribisto is single-instance: "open that project" goes through
//! [`crate::shell::windows::open_or_focus_project`], never a spawned second
//! process (a spawned child would just elect, find this process as the
//! primary, and hand the path back over a socket). What is left here are the
//! two helpers that were never about spawning at all.

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
    open_with_desktop(&target);
}

/// Hand `path` to whatever the desktop opens that kind of file with — a `.docx` to the word
/// processor, a `.pdf` to the viewer.
///
/// Best-effort and deliberately fire-and-forget: there is no portable way to learn that the
/// handler actually appeared, and a project must never be blocked waiting on one. A desktop
/// with nothing registered for the type simply does nothing, which is the same outcome as
/// not offering the button — so the failure mode costs the writer one click, not their work.
pub(crate) fn open_in_default_app(path: &str) {
    open_with_desktop(Path::new(path));
}

fn open_with_desktop(target: &Path) {
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(target).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(target).spawn();
    #[cfg(target_os = "windows")]
    // `explorer` is the launcher on Windows for both a folder and a document; `start` is a
    // shell builtin and has no executable to spawn.
    let _ = std::process::Command::new("explorer").arg(target).spawn();
}
