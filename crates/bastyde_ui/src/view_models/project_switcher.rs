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
use crate::shell::open_registry::OpenEntry;
use crate::shell::process;

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
/// model, `my_pid` from `open_registry::my_pid()`, `my_path` from the CALLING window's own
/// `SingleWorkInfo::file_name()` (already canonicalized the same way `open_registry` claims
/// are — see the module doc — so a plain string comparison is enough).
///
/// **`is_self` is "this window's own open path", never "any entry from my process"** —
/// `is_self = e.pid == my_pid` alone would be wrong with two in-process Works sharing
/// one `my_pid()`: every entry from this process, including a SIBLING window's
/// different, still-open Work, would be marked `is_self` and its row would go inert
/// (`if is_self { return; }` in `project_switcher_button.rs`), so clicking it would do
/// nothing instead of raising that other window. `my_path` being `None` (this window
/// hasn't finished its own Load/New yet) marks nothing as self, rather than guessing.
pub fn sections(
    entries: Vec<OpenEntry>,
    recents: &[RecentWorkDto],
    my_pid: u32,
    my_path: Option<&str>,
) -> SwitcherSections {
    let my_canon = my_path.map(process::canon);
    let open: Vec<OpenRow> = entries
        .into_iter()
        .map(|e| {
            let is_self = e.pid == my_pid && my_canon.as_deref() == Some(e.path.as_str());
            OpenRow {
                is_self,
                pid: e.pid,
                path: e.path,
                title: e.title,
            }
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

/// Raise the window already showing `path`, rather than opening it twice.
///
/// **Two routes, and the split matters.** The common case is that the project is
/// open in a window of *this very process* — single-instance means a second launch
/// hands off rather than forking — so `pid == my_pid()` is resolved directly through
/// [`crate::shell::windows::resolve_project_window`] (first window string id, then registry
/// fallback for a secondary-only open) and `focus_window` raises it. Going out over a socket
/// to ourselves would arrive at the same handler by a longer road.
///
/// A foreign pid is a genuine peer — a `--new-instance` sibling, or an instance that ran
/// while this one was wedged — and still goes over its per-pid IPC socket, carrying `path` so
/// the peer raises the *right* window rather than whichever of its own opened last.
///
/// The activation token is minted from *this* (focused) window either way: on Wayland a
/// process cannot raise itself unprompted, and neither can a window that isn't the one the
/// user just clicked in.
pub fn raise_instance(ctx: &mut EventContext, pid: u32, path: &str) {
    if pid == crate::shell::open_registry::my_pid() {
        if let Some(id) = crate::shell::windows::resolve_project_window(ctx, path) {
            ctx.focus_window(id);
        }
        return;
    }
    let path = path.to_string();
    ctx.request_activation_token_self(Box::new(move |tok| {
        let _ = crate::shell::ipc::send_raise(pid, Some(path), tok);
    }));
}

/// Open `path` in a brand-new window of **this** process, leaving the current one alone.
///
/// Spawning a second `skribisto` process would be a round trip to nowhere under
/// single-instance: the child would elect, find this very process as the primary,
/// hand the path back over a socket and exit — so this does directly what that
/// handoff would ask for. Backed by the same `ProjectWindowFactory` every other
/// project window comes from, so window 2 gets its own
/// `WorkSession`/`AppIds`/undo stack, same as a second process would have given it.
pub fn open_in_new_window(ctx: &mut EventContext, path: &str) {
    crate::shell::windows::open_or_focus_project(ctx, path);
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
    fn this_windows_own_open_path_is_marked_and_the_others_are_not() {
        let s = sections(
            vec![entry(10, "/a.skrib"), entry(10, "/b.skrib")],
            &[],
            10,
            Some("/a.skrib"),
        );
        assert!(s.open[0].is_self, "this window's own path is us");
        assert!(
            !s.open[1].is_self,
            "a sibling in-process Work at a different path must never be marked self, \
             even though it shares this window's own pid"
        );
    }

    /// The regression this Phase-3 fix closes: with two Works open in ONE process (sharing one
    /// `my_pid`), `is_self` must key on THIS window's own open path, never on pid alone — or a
    /// sibling window's different, still-open Work would be marked self and its row would go
    /// inert instead of raising it.
    #[test]
    fn a_sibling_in_process_work_sharing_this_pid_is_never_marked_self() {
        let s = sections(
            vec![entry(10, "/mine.skrib"), entry(10, "/siblings.skrib")],
            &[],
            10,
            Some("/mine.skrib"),
        );
        let mine = s.open.iter().find(|r| r.path == "/mine.skrib").unwrap();
        let sibling = s.open.iter().find(|r| r.path == "/siblings.skrib").unwrap();
        assert!(mine.is_self);
        assert!(!sibling.is_self);
    }

    /// Before this window has finished its own Load/New, `my_path` is `None` — nothing should
    /// be guessed as self.
    #[test]
    fn no_open_path_yet_marks_nothing_as_self() {
        let s = sections(vec![entry(10, "/a.skrib")], &[], 10, None);
        assert!(!s.open[0].is_self);
    }

    /// The whole point of the split: a project already open somewhere must not also appear
    /// under Recent, or the popover offers to open it a second time.
    #[test]
    fn an_already_open_project_is_dropped_from_recent() {
        let s = sections(
            vec![entry(10, "/novel.skrib")],
            &[recent("/novel.skrib"), recent("/other.skrib")],
            10,
            Some("/novel.skrib"),
        );
        assert_eq!(s.open.len(), 1);
        assert_eq!(s.recent.len(), 1, "only the un-open project stays");
        assert_eq!(s.recent[0].absolute_path, "/other.skrib");
    }

    /// A project open in *another* instance is still dropped from Recent — the dedup is by
    /// path, not by "is it mine".
    #[test]
    fn a_peer_windows_project_is_also_dropped_from_recent() {
        let s = sections(
            vec![entry(99, "/novel.skrib")],
            &[recent("/novel.skrib")],
            10,
            Some("/mine.skrib"),
        );
        assert!(s.recent.is_empty());
        assert!(!s.open[0].is_self);
    }

    #[test]
    fn nothing_open_leaves_every_recent_in_place() {
        let s = sections(vec![], &[recent("/a.skrib"), recent("/b.skrib")], 10, None);
        assert!(s.open.is_empty());
        assert_eq!(s.recent.len(), 2);
    }

    #[test]
    fn both_empty_is_the_empty_state() {
        assert!(sections(vec![], &[], 10, None).is_empty());
        assert!(!sections(vec![entry(1, "/a.skrib")], &[], 1, None).is_empty());
    }
}
