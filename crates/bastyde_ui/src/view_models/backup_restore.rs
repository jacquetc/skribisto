// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupRestoreViewModel` — "restore this project to this backup".
//!
//! A backup is opened read-only-file in its own window (see [`crate::backup`]).
//! Restoring writes the restored content — this window's store, the backup's
//! content plus any in-session edits — over the **original** project
//! (`backup_of`), reusing the existing `save_as` command. Before overwriting,
//! the current on-disk original is copied aside as a safety backup (and
//! promoted to a *real*, prunable backup — T2-7, see [`mark_existing_as_backup`]
//! below), so a restore is always reversible. If the original is open in
//! another window, the user is asked to close it there first (with a "Focus
//! that window" shortcut); restore never force-closes a peer.
//!
//! **T2-6 — crash-safe for folder-shape projects.** `save_as` writes to a
//! **temp sibling** path next to the original, never in place. Only once that
//! write fully succeeds does [`Self::on_long_op_completed`] swap it over the
//! real target — a same-filesystem `rename`, atomic for a zip file, and a
//! 3-step rename-aside/rename-in/remove-old dance for a folder bundle (`rename`
//! refuses to replace a non-empty directory directly). A mid-write failure
//! therefore never touches the original at all — it's still sitting untouched
//! at `target` while the (now-abandoned) temp holds whatever got written — so
//! `on_long_op_failed`'s "state unchanged" is now literally true for both
//! shapes, not just the zip one.
//!
//! Self-contained: it triggers `save_as` directly (not via `SaveAsViewModel`,
//! whose completion would repoint `WorkInfo` at the *temp* path) and handles that
//! op's completion itself — recording the new path/shape into `WorkInfo`, then
//! clearing backup mode so the window becomes the live restored project in place
//! (no reload).
//!
//! Because it bypasses `SaveAsViewModel::begin`, it owns the same
//! flush-before-serialize invariant itself: [`Self::set_flush_hook`] installs
//! `editors.flush_all()`, and [`Self::do_restore`] runs it before the read-only
//! background op reads the store. Typing only marks a doc dirty until an explicit
//! flush, so without it a restore would write the *pre-edit* prose — silently
//! dropping everything typed in the backup window since the last flush boundary,
//! which is exactly the content the user is restoring *in order to keep*.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton, Toast};

use frontend::AppContext;
use frontend::commands::{work_info_commands, work_management_commands};
use frontend::common::entities::WorkShape;
use frontend::common::event::Event;
use frontend::direct_access::UpdateWorkInfoDto;
use frontend::work_management::SaveAsDto;
use skrib_format::mark_existing_as_backup;

use crate::app_ids::AppIds;
use crate::backup::BackupContext;
use crate::singles::SingleWork;

use super::long_op::{event_id, parse_payload};

struct BackupRestorePending {
    op_id: String,
    /// The real project path being restored over.
    target: String,
    /// Where `save_as` actually wrote (T2-6): a temp sibling of `target`, never
    /// `target` itself — swapped in atomically once the write is confirmed done.
    temp_target: String,
    as_folder: bool,
    work_info_id: Option<u64>,
    safety_backup_path: Option<String>,
}

#[derive(Clone)]
pub struct BackupRestoreViewModel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    single_work: SingleWork,
    backup_mode: Signal<bool>,
    backup_context: Signal<Option<BackupContext>>,
    pending: Rc<RefCell<Option<BackupRestorePending>>>,
    /// Pushes the live editor buffers into the store (`editors.flush_all()`),
    /// installed by `App::build`. See the module docs: a restore serializes the
    /// store, so it must flush first. Shared cell → visible on every clone.
    flush_hook: Rc<RefCell<Rc<dyn Fn()>>>,
}

impl BackupRestoreViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        single_work: SingleWork,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<BackupContext>>,
    ) -> Self {
        Self {
            app_ctx,
            ids,
            single_work,
            backup_mode,
            backup_context,
            pending: Rc::new(RefCell::new(None)),
            flush_hook: Rc::new(RefCell::new(Rc::new(|| {}) as Rc<dyn Fn()>)),
        }
    }

    /// Install the real flush hook (`editors.flush_all()`). Called once from
    /// `App::build`; visible on every existing clone.
    pub fn set_flush_hook(&self, hook: Rc<dyn Fn()>) {
        *self.flush_hook.borrow_mut() = hook;
    }

    /// Copy the live editor buffers into the store. Cheap when nothing is dirty.
    fn flush(&self) {
        let hook = self.flush_hook.borrow().clone();
        hook();
    }

    /// Begin the restore flow (from the choice modal or the banner button).
    pub fn begin(&self, ctx: &mut EventContext) {
        let Some(bc) = self.backup_context.get() else {
            return;
        };
        // Resolve the original. If it can't be located, the user's escape hatch is
        // Save As (still available in backup mode) — say so rather than guessing.
        let Some(target) = bc.backup_of.clone().filter(|p| Path::new(p).exists()) else {
            ctx.show_toast(Toast::error(tr!(backup_restore_original_missing())));
            return;
        };
        self.check_open_elsewhere(ctx, target);
    }

    /// Refuse to overwrite an original that a *different* process has open; ask the
    /// user to close it there (offering to focus that window), then retry.
    fn check_open_elsewhere(&self, ctx: &mut EventContext, target: String) {
        let canon = crate::shell::open_registry::canonical(&target);
        let peer = crate::shell::open_registry::scan().into_iter().find(|e| {
            e.pid != crate::shell::open_registry::my_pid()
                && crate::shell::open_registry::canonical(&e.path) == canon
        });
        let Some(entry) = peer else {
            return self.confirm(ctx, target);
        };
        let pid = entry.pid;
        let me = self.clone();
        MessageBox::warning(tr!(backup_restore_close_elsewhere_title()))
            .text(tr!(backup_restore_close_elsewhere_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Open)
                    .label(tr!(backup_restore_focus_window())),
                MessageBoxButton::standard(StandardButton::Retry),
                MessageBoxButton::standard(StandardButton::Cancel),
            ]))
            .default_button(StandardButton::Retry)
            .escape_button(StandardButton::Cancel)
            .on_result(move |r, c| match r.button {
                // Focus the other window (best-effort raise), then let the user
                // retry once they've closed it there.
                StandardButton::Open => {
                    c.request_activation_token_self(Box::new(move |tok| {
                        let _ = crate::shell::ipc::send_raise(pid, tok);
                    }));
                }
                StandardButton::Retry => me.check_open_elsewhere(c, target.clone()),
                _ => {}
            })
            .present(ctx);
    }

    /// Final confirmation — the current on-disk version will be safety-copied first.
    fn confirm(&self, ctx: &mut EventContext, target: String) {
        let me = self.clone();
        MessageBox::question(tr!(backup_restore_confirm_title()))
            .text(tr!(backup_restore_confirm_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Ok)
                    .label(tr!(backup_restore_confirm_ok())),
                MessageBoxButton::standard(StandardButton::Cancel),
            ]))
            .default_button(StandardButton::Ok)
            .escape_button(StandardButton::Cancel)
            .on_result(move |r, c| {
                if r.button == StandardButton::Ok {
                    me.do_restore(c, target.clone());
                }
            })
            .present(ctx);
    }

    /// Safety-copy the original, then write the restored content to a temp
    /// sibling (T2-6) — never in place.
    fn do_restore(&self, ctx: &mut EventContext, target: String) {
        // The background `save_as` below is read-only: it serializes the store as
        // it finds it. Push the live editor buffers in first, or the restored file
        // is written from the *pre-edit* prose (see the module docs). Cheap when
        // nothing is dirty; must happen on the UI thread, before the op starts.
        self.flush();

        // Re-check the peer race just before writing (advisory, best-effort).
        let canon = crate::shell::open_registry::canonical(&target);
        if crate::shell::open_registry::scan().into_iter().any(|e| {
            e.pid != crate::shell::open_registry::my_pid()
                && crate::shell::open_registry::canonical(&e.path) == canon
        }) {
            return self.check_open_elsewhere(ctx, target);
        }

        // 1) Copy the current on-disk original aside (completes before any write).
        let safety_backup_path = match safety_copy(&target) {
            Ok(p) => p,
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(backup_restore_error(
                    error = e.to_string()
                ))));
                return;
            }
        };
        // T2-7: the safety copy is a raw byte copy, so as written it still
        // carries `kind: Regular` — retention would never prune it and the
        // backups list would never show it, a permanent invisible orphan on
        // every restore. Mark it as a real backup of `target` in place.
        if let Some(path) = &safety_backup_path {
            let when = chrono::Utc::now();
            if let Err(e) = mark_existing_as_backup(path, &target, when) {
                eprintln!("restore: could not mark the safety copy '{path}' as a backup: {e:#}");
            }
        }

        // 2) Write the restored content to a TEMP SIBLING of the original
        // (T2-6), preserving its shape — never in place. A mid-write failure
        // therefore can't touch `target` at all; the swap happens only once
        // this write is confirmed done (`on_long_op_completed`).
        let as_folder = matches!(
            skrib_format::detect_shape(&target),
            Ok(skrib_format::SkribShape::ExplodedFolder)
        );
        let temp_target = temp_sibling_path(Path::new(&target), "restore-tmp");
        match work_management_commands::save_as(
            &self.app_ctx,
            &SaveAsDto {
                work_id: self.ids.work_id.get().unwrap_or_default(),
                file_name: temp_target.to_string_lossy().into_owned(),
                as_folder,
            },
        ) {
            Ok(op_id) => {
                *self.pending.borrow_mut() = Some(BackupRestorePending {
                    op_id,
                    target,
                    temp_target: temp_target.to_string_lossy().into_owned(),
                    as_folder,
                    work_info_id: self.ids.work_info_id.get(),
                    safety_backup_path,
                });
            }
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(backup_restore_error(
                    error = e.to_string()
                ))));
            }
        }
    }

    /// The write to the temp sibling finished: swap it over the real target
    /// (T2-6 — a rename, never an in-place overwrite), then record the new
    /// path/shape into `WorkInfo` and leave backup mode — this window becomes
    /// the live restored project in place.
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        let pending = {
            let mut slot = self.pending.borrow_mut();
            match slot.as_ref() {
                Some(p) if p.op_id == op_id => slot.take().unwrap(),
                _ => return, // not our restore op
            }
        };

        // The write into the temp sibling succeeded — swap it over the
        // original now. A rename (or, for a non-empty folder, a 3-step
        // rename-aside/rename-in/remove-old dance), so a crash here can never
        // leave a torn tree: either the swap didn't happen (original intact,
        // temp still holds the full write) or it did.
        if let Err(e) = atomic_replace(
            Path::new(&pending.temp_target),
            Path::new(&pending.target),
            pending.as_folder,
        ) {
            // The restored content is safe on disk at `temp_target` — only the
            // swap failed, so nothing has been lost. Leave it in place (don't
            // delete it) so the user can retry or recover it manually, and
            // leave the original untouched — still viewing the backup here.
            ctx.show_toast(Toast::error(tr!(backup_restore_error(
                error = e.to_string()
            ))));
            return;
        }

        // Update WorkInfo (mirrors SaveAsViewModel) so the window points at the
        // restored file with the right shape. `cur.file_name`, read here before
        // this update overwrites it, is THIS window's own previous claim — the
        // backup's path (or an earlier restored path) — captured for the
        // release-then-claim pair below. See the Phase-3 fix note there.
        let mut previous_path: Option<String> = None;
        if let Some(id) = pending.work_info_id
            && let Ok(Some(cur)) = work_info_commands::get_work_info(&self.app_ctx, &id)
        {
            previous_path = cur.file_name.clone();
            let dto = UpdateWorkInfoDto {
                id,
                created_at: cur.created_at,
                updated_at: chrono::Utc::now(),
                file_name: Some(pending.target.clone()),
                shape: if pending.as_folder {
                    WorkShape::Folder
                } else {
                    WorkShape::Zip
                },
            };
            let _ = work_info_commands::update_work_info(&self.app_ctx, &dto);
        }

        // Leave backup mode: this window is now the live restored project. It
        // was holding a claim on the *backup's* path (or an earlier restored
        // path) with no `LoadWork`/`CloseWork` in between, so release THIS
        // window's own previous claim (`previous_path`, captured above) and
        // claim the new one. NOT `replace_claim`/`release_all()` (T1-5's
        // original choice): with a second Work open in a second window,
        // dropping every claim the whole *process* holds would silently
        // un-claim that sibling's untouched, still-open project too — see
        // `project_lifecycle::claim`'s doc for the identical Phase-3 fix.
        self.backup_mode.set(false);
        self.backup_context.set(None);
        if let Some(prev) = previous_path.as_deref() {
            crate::shell::open_registry::release(prev);
        }
        crate::shell::open_registry::claim(&pending.target, &self.single_work.title().get());

        let msg = match &pending.safety_backup_path {
            Some(p) => tr!(backup_restored_with_safety(path = p.clone())),
            None => tr!(backup_restored_ok()),
        };
        ctx.show_toast(Toast::success(msg));
    }

    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        let pending = {
            let mut slot = self.pending.borrow_mut();
            match slot.as_ref() {
                Some(p) if p.op_id == op_id => slot.take(),
                _ => return,
            }
        };
        // The write into the temp sibling failed before it could reach the
        // swap — the original was never touched. Best-effort clean up
        // whatever partial temp content the failed write left behind.
        if let Some(p) = &pending {
            if p.as_folder {
                let _ = std::fs::remove_dir_all(&p.temp_target);
            } else {
                let _ = std::fs::remove_file(&p.temp_target);
            }
        }
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        ctx.show_toast(Toast::error(tr!(backup_restore_error(error = error))));
        // State unchanged — still viewing the backup in backup mode.
    }
}

/// Copy the current on-disk project at `path` to a timestamped safety backup
/// beside it, returning the new path (or `None` if `path` doesn't exist).
fn safety_copy(path: &str) -> std::io::Result<Option<String>> {
    let src = Path::new(path);
    if !src.exists() {
        return Ok(None);
    }
    let stem = src
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("project");
    let dir = src.parent().unwrap_or_else(|| Path::new("."));
    let stamp = chrono::Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let mut dst = dir.join(format!("{stem}-{stamp}.skrib"));
    for n in 2u32..=u32::MAX {
        if !dst.exists() {
            break;
        }
        dst = dir.join(format!("{stem}-{stamp}-{n}.skrib"));
    }
    if src.is_dir() {
        copy_dir_recursive(src, &dst)?;
    } else {
        std::fs::copy(src, &dst)?;
    }
    Ok(Some(dst.to_string_lossy().into_owned()))
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// A sibling path next to `path`, tagged and disambiguated by this process's
/// pid (two restores in two windows never collide) — used both for the T2-6
/// temp write target and the atomic-replace "old" aside.
fn temp_sibling_path(path: &Path, tag: &str) -> PathBuf {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("project");
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    dir.join(format!("{name}.{tag}-{}", std::process::id()))
}

/// Atomically replace `target` with the freshly-written `temp` (same directory
/// ⇒ same filesystem, so `rename` is atomic) — T2-6.
///
/// A zip bundle is a single `rename`: POSIX `rename()` replaces an existing
/// destination *file* in one atomic step, so this is the whole operation.
///
/// A folder bundle needs a 3-step swap, because `rename()` refuses to replace
/// a non-empty destination *directory*: move the current folder aside, rename
/// the new one into `target`, then remove the old one. If the second rename
/// fails, the first is rolled back (the original folder moves right back to
/// `target`) before the error is returned — so the only failure window is a
/// single `rename` syscall, never a half-written tree.
fn atomic_replace(temp: &Path, target: &Path, as_folder: bool) -> std::io::Result<()> {
    if as_folder && target.exists() {
        let aside = temp_sibling_path(target, "restore-old");
        std::fs::rename(target, &aside)?;
        if let Err(e) = std::fs::rename(temp, target) {
            let _ = std::fs::rename(&aside, target); // put the original back
            return Err(e);
        }
        let _ = std::fs::remove_dir_all(&aside); // best-effort cleanup
        Ok(())
    } else {
        std::fs::rename(temp, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // ── Phase 3: cross-Work isolation ────────────────────────────────────────
    //
    // `BackupRestoreViewModel` is minted fresh per window from that window's
    // own `WorkSession` (see `shell::windows::ProjectWindowFactory::window_config`),
    // sourcing `ids`/`single_work`/`backup_mode`/`backup_context` off the
    // session instead of a shared, process-wide signal (see `WorkSession`'s
    // module doc). These tests prove the seam that would silently leak if
    // that fix were ever reverted: a restore vm built for one Work can never
    // resolve a different Work's id/title, and a Work that never opened a
    // backup never sees another Work's `backup_context`.

    fn restore_vm_for(
        session: &crate::sessions::WorkSession,
        work_id: u64,
    ) -> BackupRestoreViewModel {
        session.ids.work_id.set(Some(work_id));
        BackupRestoreViewModel::new(
            Rc::new(AppContext::new()),
            session.ids.clone(),
            session.single_work.clone(),
            session.backup_mode.clone(),
            session.backup_context.clone(),
        )
    }

    #[test]
    fn a_restore_vm_never_resolves_a_different_works_id_or_backup_context() {
        let session_a = crate::sessions::WorkSession::for_test();
        let session_b = crate::sessions::WorkSession::for_test();
        let restore_a = restore_vm_for(&session_a, 1);
        let restore_b = restore_vm_for(&session_b, 2);

        // Work A opened a backup; Work B never did.
        session_a.backup_context.set(Some(BackupContext {
            path: "/tmp/a.skrib".to_string(),
            backup_of: Some("/tmp/a-original.skrib".to_string()),
            backup_created_at: None,
            authoritative: true,
        }));

        assert!(
            restore_a.backup_context.get().is_some(),
            "Work A's own restore vm must see the backup it opened"
        );
        assert!(
            restore_b.backup_context.get().is_none(),
            "Work B's restore vm must never see Work A's backup_context — it never opened one, \
             so its own `begin()` returns immediately and can never write over Work A's original"
        );
        assert_ne!(
            restore_a.ids.work_id.get(),
            restore_b.ids.work_id.get(),
            "each restore vm must resolve its own Work's id, never a sibling's — this is what \
             `do_restore` puts into the SaveAsDto that ultimately names the write target"
        );
    }

    #[test]
    fn flipping_one_works_backup_mode_never_flips_a_siblings() {
        let session_a = crate::sessions::WorkSession::for_test();
        let session_b = crate::sessions::WorkSession::for_test();
        let restore_a = restore_vm_for(&session_a, 1);
        let restore_b = restore_vm_for(&session_b, 2);

        restore_a.backup_mode.set(true);

        assert!(restore_a.backup_mode.get());
        assert!(
            !restore_b.backup_mode.get(),
            "Work B must stay writable (Save enabled, no banner) while only Work A is in \
             backup mode — the exact regression a shared, process-wide backup_mode caused"
        );
    }

    #[test]
    fn safety_copy_duplicates_a_zip_file_beside_it() {
        let d = tempdir().unwrap();
        let proj = d.path().join("novel.skrib");
        std::fs::write(&proj, b"zip-bytes").unwrap();
        let out = safety_copy(proj.to_str().unwrap()).unwrap().unwrap();
        assert!(Path::new(&out).exists());
        assert!(out.contains("novel-"));
        assert_eq!(std::fs::read(&out).unwrap(), b"zip-bytes");
    }

    #[test]
    fn safety_copy_none_when_missing() {
        assert!(safety_copy("/no/such/path.skrib").unwrap().is_none());
    }

    #[test]
    fn safety_copy_recurses_a_folder_project() {
        let d = tempdir().unwrap();
        let proj = d.path().join("folder-proj");
        std::fs::create_dir_all(proj.join("binders")).unwrap();
        std::fs::write(proj.join("project.skrib"), b"m").unwrap();
        std::fs::write(proj.join("binders/items.ron"), b"i").unwrap();
        let out = safety_copy(proj.to_str().unwrap()).unwrap().unwrap();
        assert!(Path::new(&out).join("binders/items.ron").exists());
    }

    // ── T2-6: crash-safe atomic replace ──────────────────────────────────────

    #[test]
    fn temp_sibling_path_is_next_to_the_original_and_tagged() {
        let d = tempdir().unwrap();
        let proj = d.path().join("novel.skrib");
        let tmp = temp_sibling_path(&proj, "restore-tmp");
        assert_eq!(tmp.parent(), Some(d.path()));
        assert!(
            tmp.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("novel.skrib.restore-tmp-")
        );
    }

    #[test]
    fn atomic_replace_swaps_a_zip_file_in_one_rename() {
        let d = tempdir().unwrap();
        let target = d.path().join("novel.skrib");
        std::fs::write(&target, b"old").unwrap();
        let temp = d.path().join("novel.skrib.restore-tmp-x");
        std::fs::write(&temp, b"new").unwrap();

        atomic_replace(&temp, &target, false).unwrap();

        assert_eq!(std::fs::read(&target).unwrap(), b"new");
        assert!(!temp.exists());
    }

    #[test]
    fn atomic_replace_swaps_a_non_empty_folder_and_cleans_up_the_old_one() {
        let d = tempdir().unwrap();
        let target = d.path().join("proj");
        std::fs::create_dir_all(target.join("binders")).unwrap();
        std::fs::write(target.join("project.skrib"), b"old-manifest").unwrap();
        std::fs::write(target.join("binders/items.ron"), b"old-items").unwrap();

        let temp = d.path().join("proj.restore-tmp-x");
        std::fs::create_dir_all(&temp).unwrap();
        std::fs::write(temp.join("project.skrib"), b"new-manifest").unwrap();

        atomic_replace(&temp, &target, true).unwrap();

        assert_eq!(
            std::fs::read(target.join("project.skrib")).unwrap(),
            b"new-manifest"
        );
        assert!(
            !target.join("binders").exists(),
            "the old folder's content must be fully replaced, not merged"
        );
        assert!(!temp.exists(), "the temp folder was consumed by the swap");
        // No leftover "-restore-old-" sibling — cleaned up on success.
        let leftovers: Vec<_> = std::fs::read_dir(d.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains("restore-old"))
            .collect();
        assert!(leftovers.is_empty(), "leftovers: {leftovers:?}");
    }

    #[test]
    fn atomic_replace_writes_a_fresh_folder_when_target_does_not_exist_yet() {
        let d = tempdir().unwrap();
        let target = d.path().join("brand-new-proj");
        let temp = d.path().join("brand-new-proj.restore-tmp-x");
        std::fs::create_dir_all(&temp).unwrap();
        std::fs::write(temp.join("project.skrib"), b"m").unwrap();

        atomic_replace(&temp, &target, true).unwrap();

        assert!(target.join("project.skrib").exists());
        assert!(!temp.exists());
    }
}
