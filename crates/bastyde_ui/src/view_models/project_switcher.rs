// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the project-switcher popover lists, and the two ways to reach a project.
//!
//! The popover has two sections — **Currently open** (one row per live instance, this window
//! marked) and **Recent** (everything else) — and getting the split right is the only real
//! logic in the feature: a project that is already open in some window must appear *once*, in
//! the open section, never again under Recent. That derivation used to be inline in
//! `OpenProjectsMenu::build` with no tests, even though it is pure over its two inputs.
//!
//! Path comparison goes through [`process::canon`] because the two sides disagree by
//! construction: the open registry stores canonicalized paths, while the recents list stores
//! whatever path the project was opened by (a relative path from argv, a symlinked home, a
//! `//server/share` spelling). Comparing them raw silently shows a project in both sections.
//!
//! The confirmation dialog ("open in a new window" vs "open here") stays beside the view in
//! `project_switcher_button.rs`: it needs an `EventContext`, which is the codebase's stated
//! reason for dialog logic living next to the widget (see `docks::search_replace_flow`).

use bastyde::prelude::*;

use frontend::direct_access::RecentWorkDto;

use crate::intents::AppIntent;
use crate::open_registry::OpenEntry;
use crate::process;

/// One row of the **Currently open** section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenRow {
    pub pid: u32,
    pub path: String,
    pub title: String,
    /// This very window — rendered as the current selection and inert on click.
    pub is_self: bool,
}

/// What the popover shows.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SwitcherSections {
    pub open: Vec<OpenRow>,
    pub recent: Vec<RecentWorkDto>,
}

impl SwitcherSections {
    /// Neither section has anything — the popover shows its "no recent works" note.
    pub fn is_empty(&self) -> bool {
        self.open.is_empty() && self.recent.is_empty()
    }
}

/// Split the registry's live instances and the recents list into the popover's two sections.
///
/// Pure over its inputs, so the dedup rule is testable without a registry, a filesystem or a
/// widget tree. `entries` comes from `open_registry::scan()`, `recents` from the recents
/// model, `my_pid` from `open_registry::my_pid()`.
pub fn sections(entries: Vec<OpenEntry>, recents: &[RecentWorkDto], my_pid: u32) -> SwitcherSections {
    let open: Vec<OpenRow> = entries
        .into_iter()
        .map(|e| OpenRow {
            is_self: e.pid == my_pid,
            pid: e.pid,
            path: e.path,
            title: e.title,
        })
        .collect();

    // Canonicalize both sides — see the module docs.
    let open_paths: std::collections::HashSet<&str> =
        open.iter().map(|r| r.path.as_str()).collect();
    let recent = recents
        .iter()
        .filter(|r| !open_paths.contains(process::canon(&r.absolute_path).as_str()))
        .cloned()
        .collect();

    SwitcherSections { open, recent }
}

/// Raise the instance that already holds a project, rather than opening it twice.
///
/// Mints an activation token from *this* (focused) window and hands it to the owning process
/// over the IPC socket, which is what lets that window actually come forward on Wayland — a
/// process cannot raise itself unprompted.
pub fn raise_instance(ctx: &mut EventContext, pid: u32) {
    ctx.request_activation_token_self(Box::new(move |tok| {
        let _ = crate::ipc::send_raise(pid, tok);
    }));
}

/// Open `path` in a brand-new window (a fresh process — one process per project).
pub fn open_in_new_window(ctx: &mut EventContext, path: &str) {
    let path = path.to_string();
    ctx.request_activation_token_self(Box::new(move |tok| {
        process::spawn_new_process(&path, tok);
    }));
}

/// Open `path` **in this window**, replacing the current project.
///
/// Goes through the `work.open_path` intent rather than `load_work`, so it passes the
/// unsaved-changes guard and loads only once the open project is saved or explicitly
/// discarded. It used to call `load_work` outright and bin those edits without asking.
pub fn open_here(ctx: &mut EventContext, path: &str) {
    ctx.send_intent(AppIntent::OpenWorkPath {
        path: path.to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(pid: u32, path: &str) -> OpenEntry {
        OpenEntry {
            pid,
            path: path.to_string(),
            title: format!("work-{pid}"),
        }
    }

    fn recent(path: &str) -> RecentWorkDto {
        RecentWorkDto {
            absolute_path: path.to_string(),
            title: "recent".into(),
            ..Default::default()
        }
    }

    #[test]
    fn this_window_is_marked_and_the_others_are_not() {
        let s = sections(vec![entry(10, "/a.skrib"), entry(20, "/b.skrib")], &[], 10);
        assert!(s.open[0].is_self, "pid 10 is us");
        assert!(!s.open[1].is_self);
    }

    /// The whole point of the split: a project already open somewhere must not also appear
    /// under Recent, or the popover offers to open it a second time.
    #[test]
    fn an_already_open_project_is_dropped_from_recent() {
        let s = sections(
            vec![entry(10, "/novel.skrib")],
            &[recent("/novel.skrib"), recent("/other.skrib")],
            10,
        );
        assert_eq!(s.open.len(), 1);
        assert_eq!(s.recent.len(), 1, "only the un-open project stays");
        assert_eq!(s.recent[0].absolute_path, "/other.skrib");
    }

    /// A project open in *another* instance is still dropped from Recent — the dedup is by
    /// path, not by "is it mine".
    #[test]
    fn a_peer_windows_project_is_also_dropped_from_recent() {
        let s = sections(vec![entry(99, "/novel.skrib")], &[recent("/novel.skrib")], 10);
        assert!(s.recent.is_empty());
        assert!(!s.open[0].is_self);
    }

    #[test]
    fn nothing_open_leaves_every_recent_in_place() {
        let s = sections(vec![], &[recent("/a.skrib"), recent("/b.skrib")], 10);
        assert!(s.open.is_empty());
        assert_eq!(s.recent.len(), 2);
    }

    #[test]
    fn both_empty_is_the_empty_state() {
        assert!(sections(vec![], &[], 10).is_empty());
        assert!(!sections(vec![entry(1, "/a.skrib")], &[], 1).is_empty());
    }
}
