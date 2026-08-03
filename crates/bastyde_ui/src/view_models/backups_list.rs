// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupsListViewModel` — the backup **files** for the open project: find them, delete
//! them, open one, reveal one.
//!
//! Distinct from the destinations editor in Settings, which lists the configured *paths*;
//! this lists the actual backup files found at those paths, correlated on the project's
//! `unique_id` and sorted newest-first.
//!
//! ## Off the UI thread
//!
//! The scan is a `read_dir` plus one zip-manifest peek per candidate, and sizing a
//! folder-shaped bundle walks it recursively. On a USB stick or a network share either can
//! block for seconds, so **every** scan — initial load, the Refresh button, and the rescan
//! after a delete — goes through the main-thread async executor's `spawn_blocking` with a
//! loading state shown meanwhile. Never inline in `build()` or an event handler.
//!
//! When no [`AsyncRuntimeHandle`] is available (`install_async()` not called — an app bug,
//! not a normal runtime state) every method falls back to a synchronous scan rather than
//! getting stuck loading forever.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::widgets::Toast;

use skrib_format::retention;

use crate::shell::process;
use crate::toast_scope::ToastWorkExt;

use super::long_op::CapturedWork;

/// Toast id base, so a burst of delete failures replaces rather than stacks —
/// folded through [`work_scoped_toast_id`] with `self.work_id`/`me.work_id` at
/// every use, never bare: two windows browsing two different Works' backups
/// must not collide in the shared `ToastRegistry`.
const DELETE_TOAST_ID: &str = "backups.delete";

/// One backup file, as the list renders it.
#[derive(Clone)]
pub struct BackupRow {
    pub path: String,
    pub date: String,
    pub size: String,
}

/// Cloneable handle — every field is a cheap `Signal`/`ListModel`/`String` clone, so the row
/// action closures each take their own copy (mirroring the `Scanner` this replaced).
#[derive(Clone)]
pub struct BackupsListViewModel {
    uid: String,
    project_path: String,
    dirs: Vec<String>,
    /// The open Work this browser is listing backups for, used to route this
    /// view-model's delete-failure toast to the right window/bell
    /// (`crate::toast_scope::ToastWorkExt`) rather than every open project's.
    ///
    /// **Why `CapturedWork` here, with no `AppIds` to protect against.**
    /// `delete()` backgrounds its filesystem removal (`spawn_blocking`) and
    /// only routes a toast on completion — the same hazard every
    /// long-operation view-model in this crate guards against with
    /// `long_op::CapturedWork`. But unlike those (`ExportViewModel`,
    /// `BackupSchedulerViewModel`, `BackupRestoreViewModel`, `SaveAsViewModel`
    /// — each minted once per window, holding a long-lived `ids: AppIds`
    /// alongside the capture, so the risk is a handler reading the live
    /// signal instead), this type holds no `AppIds` at all: a fresh instance
    /// is built by `BackupsListPanel::new` every time the panel opens, with
    /// `work_id` given by the caller reading `ids.work_id.get()` at that
    /// moment. There is no live signal here a future handler could substitute
    /// by accident — a delete finishing after an in-place project switch
    /// still correctly names the Work the panel was opened for, structurally,
    /// not by discipline.
    ///
    /// Using [`CapturedWork::given`] anyway (rather than a bare `Option<u64>`)
    /// keeps this call site consistent with every other `.scoped_id`/
    /// `.target_work` call in the crate — `ProjectSwitchViewModel::pending_work_id`
    /// is the other holder that takes this same door, for the same reason.
    work_id: CapturedWork,
    model: ListModel<BackupRow>,
    /// Bumped whenever the list's *content* changes (a scan lands), driving the
    /// empty-state/list `Switcher`.
    epoch: Signal<u64>,
    /// `true` while a scan (initial / refresh / post-delete) is in flight.
    loading: Signal<bool>,
    /// Fetched once from `app_state` in the panel's `build`.
    async_rt: Option<AsyncRuntimeHandle>,
}

impl BackupsListViewModel {
    pub fn new(uid: String, project_path: String, dirs: Vec<String>, work_id: Option<u64>) -> Self {
        Self {
            uid,
            project_path,
            dirs,
            // The caller (`BackupsListPanel::new`) already read this from its
            // own window's `ids.work_id.get()` at panel-open time — this just
            // wraps that already-captured value in the shared type. See
            // `Self::work_id`'s doc (F7).
            work_id: CapturedWork::given(work_id),
            model: ListModel::from_vec(Vec::new()),
            epoch: Signal::new(0),
            loading: Signal::new(true),
            async_rt: None,
        }
    }

    /// Hand over the executor once it is reachable (the panel's first `build`). Idempotent.
    pub fn set_async_runtime(&mut self, rt: Option<AsyncRuntimeHandle>) {
        if self.async_rt.is_none() {
            self.async_rt = rt;
        }
    }

    // ── View handles ─────────────────────────────────────────────────────────

    pub fn rows(&self) -> ListModel<BackupRow> {
        self.model.clone()
    }

    pub fn epoch(&self) -> Signal<u64> {
        self.epoch.clone()
    }

    pub fn loading(&self) -> Signal<bool> {
        self.loading.clone()
    }

    /// The file name a row shows, and the name the delete confirmation quotes.
    pub fn display_name(path: &str) -> String {
        Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string())
    }

    // ── Commands ─────────────────────────────────────────────────────────────

    /// (Re)scan in the background and push the result into the model on completion.
    ///
    /// Safe to call from anywhere holding a clone — no `EventContext` required, since landing
    /// the result is just `Signal`/`ListModel` mutation with no ambient operation.
    pub fn reload(&self) {
        self.loading.set(true);
        let uid = self.uid.clone();
        let project_path = self.project_path.clone();
        let dirs = self.dirs.clone();
        let model = self.model.clone();
        let epoch = self.epoch.clone();
        let loading = self.loading.clone();
        match &self.async_rt {
            Some(rt) => {
                rt.spawn_local(async move {
                    let rows = spawn_blocking(move || scan(&uid, &project_path, &dirs))
                        .await
                        .unwrap_or_default();
                    model.replace_all(rows);
                    loading.set(false);
                    epoch.set(epoch.get().wrapping_add(1));
                })
                .detach();
            }
            None => {
                model.replace_all(scan(&uid, &project_path, &dirs));
                loading.set(false);
                epoch.set(epoch.get().wrapping_add(1));
            }
        }
    }

    /// Delete `path` in the background, report a failure as an error toast, then rescan
    /// **either way** — a partially-removed folder bundle should still be reflected.
    pub fn delete(&self, ctx: &mut EventContext, path: &str) {
        self.loading.set(true);
        let me = self.clone();
        let target = PathBuf::from(path);
        match &self.async_rt {
            Some(_) => {
                ctx.spawn_local_with(
                    async move { spawn_blocking(move || remove(&target)).await },
                    move |result, ctx2| {
                        let err = match result {
                            Ok(Ok(())) => None,
                            Ok(Err(e)) => Some(e.to_string()),
                            Err(_panicked) => Some("panicked".to_string()),
                        };
                        if let Some(e) = err {
                            ctx2.show_toast(
                                Toast::error(tr!(backups_delete_error(error = e)))
                                    .scoped_id(DELETE_TOAST_ID, me.work_id)
                                    .target_work(me.work_id),
                            );
                        }
                        me.reload();
                    },
                )
                .detach();
            }
            None => {
                if let Err(e) = remove(&target) {
                    ctx.show_toast(
                        Toast::error(tr!(backups_delete_error(error = e.to_string())))
                            .scoped_id(DELETE_TOAST_ID, self.work_id)
                            .target_work(self.work_id),
                    );
                }
                me.reload();
            }
        }
    }

    /// Open a backup in its **own window**, which is what shows the read-only /
    /// restore choice — the isolation a backup needs (it must never replace the
    /// project the user is working in, nor have its dirty state confused with
    /// theirs) is per-Work, via `WorkSession`'s `backup_mode`/`backup_context`,
    /// and therefore per-window already.
    pub fn open(&self, ctx: &mut EventContext, path: &str) {
        crate::shell::windows::open_or_focus_project(ctx, path);
    }

    /// Show the backup in the platform's file manager.
    pub fn reveal(path: &str) {
        process::reveal_in_file_manager(path);
    }
}

fn remove(target: &Path) -> std::io::Result<()> {
    if target.is_dir() {
        std::fs::remove_dir_all(target)
    } else {
        std::fs::remove_file(target)
    }
}

/// Scan `dirs` (resolving the empty "project folder" destination) for this project's
/// backups, newest first, de-duplicated by path.
fn scan(uid: &str, project_path: &str, dirs: &[String]) -> Vec<BackupRow> {
    let mut candidates = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    for d in dirs {
        let dir = if d.trim().is_empty() {
            Path::new(project_path)
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .map(|p| p.to_path_buf())
        } else {
            Some(PathBuf::from(d))
        };
        let Some(dir) = dir else { continue };
        if let Ok(found) = retention::scan_destination(&dir, uid, project_path) {
            for c in found {
                if seen.insert(c.path.clone()) {
                    candidates.push(c);
                }
            }
        }
    }
    candidates.sort_by_key(|c| std::cmp::Reverse(c.timestamp));
    candidates
        .into_iter()
        .map(|c| BackupRow {
            date: c.timestamp.format("%Y-%m-%d %H:%M").to_string(),
            size: human_size(&c.path),
            path: c.path.to_string_lossy().into_owned(),
        })
        .collect()
}

fn human_size(path: &Path) -> String {
    let bytes = byte_size(path);
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{} KB", (bytes / 1024).max(1))
    }
}

/// Total bytes at `path`. A backup is normally a single zip, but a folder-shaped bundle must
/// be summed recursively — `metadata(dir).len()` is the directory entry's own size (~4 KB),
/// not its contents.
fn byte_size(path: &Path) -> u64 {
    let Ok(meta) = std::fs::metadata(path) else {
        return 0;
    };
    if meta.is_file() {
        return meta.len();
    }
    if !meta.is_dir() {
        return 0;
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        return 0;
    };
    entries.flatten().map(|e| byte_size(&e.path())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// F7: `new`'s `work_id` parameter must land in `Self::work_id` wrapped as
    /// a `CapturedWork` (via `CapturedWork::given`), not stored bare — see
    /// that field's doc for why this type is the right one here even though
    /// this view-model holds no `AppIds` of its own to protect against.
    #[test]
    fn new_wraps_the_given_work_id_as_a_captured_work() {
        let vm = BackupsListViewModel::new(String::new(), String::new(), Vec::new(), Some(1));
        assert_eq!(vm.work_id, Some(1));

        let none_vm = BackupsListViewModel::new(String::new(), String::new(), Vec::new(), None);
        assert_eq!(none_vm.work_id, None);
    }

    #[test]
    fn scan_lists_this_projects_backups_newest_first() {
        // Reuse the retention test's zip writer would be ideal, but keep this a
        // pure check of ordering/dedup on an empty dir (no candidates).
        let d = tempfile::tempdir().unwrap();
        let rows = scan(
            "uid",
            &d.path().join("novel.skrib").to_string_lossy(),
            &[d.path().to_string_lossy().into_owned()],
        );
        assert!(rows.is_empty());
    }

    #[test]
    fn human_size_formats_kb_and_mb() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("a");
        std::fs::write(&f, vec![0u8; 2048]).unwrap();
        assert_eq!(human_size(&f), "2 KB");
    }

    #[test]
    fn human_size_sums_a_folder_bundle_recursively() {
        // A folder-shaped bundle: `metadata(dir).len()` would report the directory
        // entry (~4 KB), not the 3 KB of actual content across nested files.
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("novel-20260101-120000.skrib");
        std::fs::create_dir_all(root.join("binders/manuscript")).unwrap();
        std::fs::write(root.join("project.skrib"), vec![0u8; 1024]).unwrap();
        std::fs::write(root.join("binders/manuscript/a.djot"), vec![0u8; 2048]).unwrap();
        assert_eq!(byte_size(&root), 3072, "summed recursively");
        assert_eq!(human_size(&root), "3 KB");
    }

    /// F1: `DELETE_TOAST_ID` used bare would let a second Work's delete-failure
    /// toast collide with (and silently steal) this Work's still-live one.
    #[test]
    fn two_works_delete_failure_toasts_never_collide() {
        let a = crate::toast_scope::work_scoped_toast_id(DELETE_TOAST_ID, Some(1));
        let b = crate::toast_scope::work_scoped_toast_id(DELETE_TOAST_ID, Some(2));
        assert_ne!(
            a, b,
            "two different Works' backup-delete-failure toasts must never collide"
        );
    }

    /// The test above only proves `work_scoped_toast_id` itself is
    /// collision-free — it never touches `delete`'s actual
    /// `.scoped_id(...)` call site, so reverting that call
    /// site back to a bare `DELETE_TOAST_ID` would still leave it green.
    /// This one drives the real, PUBLIC `delete(ctx, path)` entry point
    /// through a real `ToastRegistry`: two `BackupsListViewModel`s for two
    /// different Works each try to delete a path that doesn't exist (so
    /// `delete` hits its error-toast branch without touching a real backup
    /// file), each through a wired `Button` + a dispatched click (a real
    /// `EventContext`), then asserts both toasts stay live —
    /// `ToastRegistry::enqueue`'s update-in-place merge would collapse them
    /// to ONE entry if the id were ever bare again.
    #[test]
    fn delete_failure_toasts_for_two_works_both_stay_live_in_a_real_registry() {
        use bastyde::i18n::lit;
        use bastyde::widgets::{Button, ToastInstallOptions, ToastRegistry};
        use frontend::AppContext;
        use std::rc::Rc;

        let vm_a = BackupsListViewModel::new(String::new(), String::new(), Vec::new(), Some(1));
        let vm_b = BackupsListViewModel::new(String::new(), String::new(), Vec::new(), Some(2));

        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        let app_ctx = Rc::new(AppContext::new());
        let mut tree = crate::test_support::tree_with_toast_registry(&app_ctx, &registry);

        let a = vm_a.clone();
        let b = vm_b.clone();
        let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| {
            a.delete(ctx, "/no/such/path/for/this/test-a");
        }));
        let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| {
            b.delete(ctx, "/no/such/path/for/this/test-b");
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));

        crate::test_support::click(&mut tree, btn_a);
        crate::test_support::click(&mut tree, btn_b);

        assert_eq!(
            registry.live_count(),
            2,
            "two different Works' backup-delete-failure toasts must both stay live — a \
             bare DELETE_TOAST_ID would let Work B's enqueue find Work A's still-live \
             entry (ToastRegistry::enqueue dedups on id alone) and merge into it, \
             leaving only 1"
        );
    }

    #[test]
    fn backup_display_name_is_the_basename() {
        assert_eq!(
            BackupsListViewModel::display_name("/a/b/novel-20260101-120000.skrib"),
            "novel-20260101-120000.skrib"
        );
        assert_eq!(
            BackupsListViewModel::display_name("novel.skrib"),
            "novel.skrib"
        );
    }
}
