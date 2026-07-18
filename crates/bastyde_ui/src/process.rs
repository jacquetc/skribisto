// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Launching another instance of this app.
//!
//! Skribisto is **one process per project** (see [`crate::open_registry`] and [`crate::ipc`]),
//! so "open that project" from anywhere other than this window means spawning a fresh
//! process. Four unrelated surfaces need that — the project switcher popover, the Launcher's
//! recents, the open-a-backup redirect, and the backups list — which is why it lives here
//! rather than in whichever one happened to implement it first.
//!
//! It sat in `project_switcher_button.rs` (a title-bar widget) and was imported from there by
//! `app.rs`, `view_models::welcome` and `backups_list_panel.rs`: three features reaching into
//! a button's module for process-launch infrastructure.

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

/// Launch a fresh Skribisto process to open `path`, forwarding an activation `token` so the
/// new window comes up focused.
///
/// The token is the Wayland `xdg_activation_v1` handshake: without it the compositor treats
/// the new window as an unsolicited pop-up and (on KWin) leaves it behind the current one.
/// The callers obtain it with `request_activation_token_self` and hand it straight here; the
/// new process reads it back via `activate_from_env`. Both env vars are set because
/// compositors disagree on which they honour.
///
/// Best-effort throughout: a failure to resolve our own executable, or to spawn, is silently
/// dropped. There is no useful recovery — and the user's own next action (retrying, or
/// opening the file from their file manager) is a better remedy than a toast about `argv[0]`.
pub(crate) fn spawn_new_process(path: &str, token: Option<String>) {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let mut cmd = std::process::Command::new(exe);
    cmd.arg(path);
    if let Some(tok) = token {
        cmd.env("XDG_ACTIVATION_TOKEN", &tok);
        cmd.env("DESKTOP_STARTUP_ID", &tok);
    }
    let _ = cmd.spawn();
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
