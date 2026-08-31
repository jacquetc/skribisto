// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Window-persistence string ids, and the one path→window resolver every open
//! and raise path must share.
//!
//! A project's first window is keyed by [`window_id_for`]; every further one
//! (Work ▸ New Window) is keyed by [`attached_window_id_for`]. Resolving "is this
//! project already open?" by the bare first id alone misses a project whose only
//! live window is a secondary — and used to, on the raise path, while open had
//! already grown a registry fallback. That divergence is exactly what
//! [`resolve_project_window`] closes: one function, used by open *and* raise.

use crate::sessions::WorkRegistry;

/// Stable window-persistence string_id for the Launcher window. One process
/// only ever shows one Launcher at a time (it closes the moment a project
/// opens), so a fixed id — not per-project hashing — is correct here; see
/// [`window_id_for`] for project windows.
pub const LAUNCHER_WINDOW_ID: &str = "launcher";

/// Per-project window-persistence id: `work-{hash(canonical_path)}`.
///
/// Canonicalizing first collapses different spellings of the same path
/// (symlinks, `..`, relative vs. absolute, a trailing slash) onto one id, so
/// a project's remembered geometry keys on the file it actually is, not the
/// string a particular launch path happened to spell it as.
///
/// Falls back to the raw string when the path doesn't exist yet (a New Work
/// target that hasn't been written to disk).
///
/// Hashed with **blake3**, not `std::hash::DefaultHasher`: the result is
/// *persisted* (as the key of a `window_state.toml` row), and `DefaultHasher`'s
/// algorithm is explicitly not guaranteed stable across Rust releases.
/// Truncated to 16 hex chars to match the previous `work-{:016x}` id width.
pub fn window_id_for(project: &str) -> String {
    let canon = crate::shell::process::canon(project);
    format!("work-{}", &blake3::hash(canon.as_bytes()).to_hex()[..16])
}

/// The window-persistence id of the **`ordinal`-th** window on `project` —
/// [`window_id_for`] for the first, `{base}-w{ordinal}` for every further one
/// opened by Work ▸ New Window.
///
/// Ordinal 1 deliberately yields the bare [`window_id_for`] value: the first
/// window on a project must keep the id its geometry has always been saved
/// under, and it is the one [`resolve_project_window`] prefers when several
/// windows show the same project.
pub fn attached_window_id_for(project: &str, ordinal: usize) -> String {
    let base = window_id_for(project);
    if ordinal <= 1 {
        base
    } else {
        format!("{base}-w{ordinal}")
    }
}

/// Any live window showing the project at `path`.
///
/// Tries the first window's string id, then falls back through
/// [`WorkRegistry`] so a project whose only open window is a secondary
/// (`-wN`) is still found. The longest-standing window on the Work is the one
/// returned (`windows_for` is ordinal-ordered).
///
/// **The one resolver.** Open, raise, and the project switcher all go through
/// here — never a bare `find_window(window_id_for(..))` — so the three paths
/// cannot drift apart about what "this project is already open" means.
pub fn resolve_project_window(
    ctx: &teksilo::prelude::EventContext,
    path: &str,
) -> Option<teksilo::prelude::TeksiloWindowId> {
    if let Some(id) = ctx.find_window(&window_id_for(path)) {
        return Some(id);
    }
    find_open_project_window(ctx, path)
}

/// Open `path` in a project window of **this** process, or focus a window
/// already showing it. Returns the window's id, or `None` if no
/// [`super::windows::ProjectWindowFactory`] is registered (only in a headless
/// test context).
///
/// **The one door** for "get me a window on this project". Uses
/// [`resolve_project_window`] so a secondary-only open project is focused
/// rather than loaded a second time into an independent Work.
pub fn open_or_focus_project(
    ctx: &mut teksilo::prelude::EventContext,
    path: &str,
) -> Option<teksilo::prelude::TeksiloWindowId> {
    if let Some(id) = resolve_project_window(ctx, path) {
        ctx.focus_window(id);
        return Some(id);
    }
    let factory = ctx.app_state::<super::windows::ProjectWindowFactory>()?;
    let (config, _state) = factory.window_config(crate::app::PendingAction::Load(path.to_string()));
    Some(ctx.open_window(config))
}

/// Registry fallback for [`resolve_project_window`]: any live window whose
/// Work's path canonicalizes to the same id as `path`.
fn find_open_project_window(
    ctx: &teksilo::prelude::EventContext,
    path: &str,
) -> Option<teksilo::prelude::TeksiloWindowId> {
    let registry = ctx.app_state::<WorkRegistry>()?;
    let wanted = window_id_for(path);
    registry.open_work_ids().into_iter().find_map(|work_id| {
        let session = registry.session_for(work_id)?;
        let open_path = session.single_work_info.file_name().get()?;
        (window_id_for(&open_path) == wanted)
            .then(|| registry.windows_for(work_id).first().copied())
            .flatten()
    })
}
