// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One-time work at launch, before the first window exists.
//!
//! Serving another instance's request over the socket, and pruning the
//! window-geometry rows of projects that are no longer anywhere — two things
//! that happen once per process and belong to no feature.

use teksilo::settings::WindowStateService;

use crate::models;
use crate::shell::{open_registry, windows};

/// The pure half of the F4(b) startup sweep: forget every `work-*` label in
/// `window_state` that doesn't hash (via [`windows::window_id_for`]) to one of
/// `known_paths`. Factored out from `main`'s production wiring (which resolves
/// `known_paths` from the real recents file + `open_registry::scan()`) so it's
/// directly testable against a temp-dir-backed `WindowStateService`, without
/// touching the real user's `window_state.toml` or recents file.
///
/// `"main"`, `"launcher"`, and any other label that doesn't start with
/// `"work-"` are never touched, regardless of `known_paths` — they are fixed,
/// not per-project.
/// Every project path this process can account for — the set the prune above
/// treats as "still wanted". Three sources, and **all three are load-bearing**:
///
/// 1. the recents MRU (the usual case);
/// 2. the open registry (a project another live instance is holding — it never
///    reaches *our* recents, but its geometry must survive);
/// 3. **`initial_project`** — the path handed to us on argv. This one is easy to
///    forget and is exactly the bug this function exists to make untestable-by-
///    omission: a file-manager double-click on a project that has aged out of the
///    30-entry MRU is in neither (1) nor (2), yet we are about to open it. Prune
///    without it and we delete the saved geometry of the very window we are
///    seconds away from restoring.
pub(crate) fn known_project_paths(initial_project: Option<&str>) -> Vec<String> {
    let mut known = models::RecentWorkListModel::all_raw_paths();
    known.extend(open_registry::scan().into_iter().map(|e| e.path));
    known.extend(initial_project.map(str::to_string));
    known
}

pub(crate) fn prune_orphaned_window_state(
    window_state: &WindowStateService,
    known_paths: &[String],
) {
    let known_labels: std::collections::HashSet<String> = known_paths
        .iter()
        .map(|p| windows::window_id_for(p))
        .collect();
    for label in window_state.labels() {
        if !label.starts_with("work-") || is_known_window_label(&label, &known_labels) {
            continue;
        }
        if let Err(e) = window_state.forget(&label) {
            eprintln!("skribisto: could not forget stale window state '{label}': {e}");
        }
    }
}

/// Does `label` name a window of a project we still know about?
///
/// Not a plain set lookup, because one project can own **several** geometry
/// rows since Work ▸ New Window: its first window saves under the bare
/// `windows::window_id_for(path)`, and each further one under
/// `{that}-w{ordinal}` (see `windows::attached_window_id_for`). Testing only for
/// exact membership would leave every `-w2`/`-w3` row unmatched and therefore
/// pruned — on *every* launch, so a second window's size and placement could
/// never survive one, and the loss would look like the geometry feature simply
/// not working for second windows rather than like a sweep deleting it.
///
/// The suffix must parse as a number rather than merely being present: `-w` is
/// otherwise the start of any string at all, and this decides what gets
/// deleted.
pub(crate) fn is_known_window_label(
    label: &str,
    known_labels: &std::collections::HashSet<String>,
) -> bool {
    if known_labels.contains(label) {
        return true;
    }
    match label.rsplit_once("-w") {
        Some((base, ordinal)) => {
            !ordinal.is_empty()
                && ordinal.bytes().all(|b| b.is_ascii_digit())
                && known_labels.contains(base)
        }
        None => false,
    }
}
