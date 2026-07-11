//! `RestoreViewModel` — "restore this project to this backup".
//!
//! A backup is opened read-only-file in its own window (see [`crate::backup`]).
//! Restoring writes that window's store (the backup's content, plus any in-session
//! edits) over the **original** project (`backup_of`), reusing the existing
//! `save_as` command. Before overwriting, the current on-disk original is copied
//! aside as a safety backup, so a restore is always reversible. If the original
//! is open in another window, the user is asked to close it there first (with a
//! "Focus that window" shortcut); restore never force-closes a peer.
//!
//! Self-contained: it triggers `save_as` directly (not via `SaveAsViewModel`) and
//! handles that op's completion itself — recording the new path/shape into
//! `WorkInfo`, then clearing backup mode so the window becomes the live restored
//! project in place (no reload).

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton, Toast};

use frontend::AppContext;
use frontend::commands::{work_info_commands, work_management_commands};
use frontend::common::entities::WorkShape;
use frontend::common::event::Event;
use frontend::direct_access::UpdateWorkInfoDto;
use frontend::work_management::SaveAsDto;

use crate::app_ids::AppIds;
use crate::backup::BackupContext;
use crate::singles::SingleWork;

use super::long_op::{event_id, parse_payload};

struct RestorePending {
    op_id: String,
    target: String,
    as_folder: bool,
    work_info_id: Option<u64>,
    safety_backup_path: Option<String>,
}

#[derive(Clone)]
pub struct RestoreViewModel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    single_work: SingleWork,
    backup_mode: Signal<bool>,
    backup_context: Signal<Option<BackupContext>>,
    pending: Rc<RefCell<Option<RestorePending>>>,
}

impl RestoreViewModel {
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
        }
    }

    /// Begin the restore flow (from the choice modal or the banner button).
    pub fn begin(&self, ctx: &mut EventContext) {
        let Some(bc) = self.backup_context.get() else {
            return;
        };
        // Resolve the original. If it can't be located, the user's escape hatch is
        // Save As (still available in backup mode) — say so rather than guessing.
        let Some(target) = bc.backup_of.clone().filter(|p| Path::new(p).exists()) else {
            ctx.show_toast(Toast::error(tr!(restore_original_missing())));
            return;
        };
        self.check_open_elsewhere(ctx, target);
    }

    /// Refuse to overwrite an original that a *different* process has open; ask the
    /// user to close it there (offering to focus that window), then retry.
    fn check_open_elsewhere(&self, ctx: &mut EventContext, target: String) {
        let canon = crate::open_registry::canonical(&target);
        let peer = crate::open_registry::scan().into_iter().find(|e| {
            e.pid != crate::open_registry::my_pid()
                && crate::open_registry::canonical(&e.path) == canon
        });
        let Some(entry) = peer else {
            return self.confirm(ctx, target);
        };
        let pid = entry.pid;
        let me = self.clone();
        MessageBox::warning(tr!(restore_close_elsewhere_title()))
            .text(tr!(restore_close_elsewhere_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Open).label(tr!(restore_focus_window())),
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
                        let _ = crate::ipc::send_raise(pid, tok);
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
        MessageBox::question(tr!(restore_confirm_title()))
            .text(tr!(restore_confirm_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Ok).label(tr!(restore_confirm_ok())),
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

    /// Safety-copy the original, then overwrite it via `save_as`.
    fn do_restore(&self, ctx: &mut EventContext, target: String) {
        // Re-check the peer race just before writing (advisory, best-effort).
        let canon = crate::open_registry::canonical(&target);
        if crate::open_registry::scan().into_iter().any(|e| {
            e.pid != crate::open_registry::my_pid()
                && crate::open_registry::canonical(&e.path) == canon
        }) {
            return self.check_open_elsewhere(ctx, target);
        }

        // 1) Copy the current on-disk original aside (completes before any write).
        let safety_backup_path = match safety_copy(&target) {
            Ok(p) => p,
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(restore_error(error = e.to_string()))));
                return;
            }
        };

        // 2) Overwrite the original with this window's store, preserving its shape.
        let as_folder = matches!(
            skrib_format::detect_shape(&target),
            Ok(skrib_format::SkribShape::ExplodedFolder)
        );
        match work_management_commands::save_as(
            &self.app_ctx,
            &SaveAsDto {
                file_name: target.clone(),
                as_folder,
            },
        ) {
            Ok(op_id) => {
                *self.pending.borrow_mut() = Some(RestorePending {
                    op_id,
                    target,
                    as_folder,
                    work_info_id: self.ids.work_info_id.get(),
                    safety_backup_path,
                });
            }
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(restore_error(error = e.to_string()))));
            }
        }
    }

    /// The restore write finished: record the new path/shape into `WorkInfo`, then
    /// leave backup mode — this window becomes the live restored project in place.
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

        // Update WorkInfo (mirrors SaveAsViewModel) so the window points at the
        // restored file with the right shape.
        if let Some(id) = pending.work_info_id
            && let Ok(Some(cur)) = work_info_commands::get_work_info(&self.app_ctx, &id)
        {
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

        // Leave backup mode: this window is now the live restored project.
        self.backup_mode.set(false);
        self.backup_context.set(None);
        crate::open_registry::claim(&pending.target, &self.single_work.title().get());

        let msg = match &pending.safety_backup_path {
            Some(p) => tr!(restored_with_safety(path = p.clone())),
            None => tr!(restored_ok()),
        };
        ctx.show_toast(Toast::success(msg));
    }

    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        {
            let mut slot = self.pending.borrow_mut();
            match slot.as_ref() {
                Some(p) if p.op_id == op_id => {
                    slot.take();
                }
                _ => return,
            }
        }
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        ctx.show_toast(Toast::error(tr!(restore_error(error = error))));
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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
}
