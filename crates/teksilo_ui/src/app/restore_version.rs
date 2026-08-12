// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Putting a past version back: the guarded sequence around the write.
//!
//! [`crate::versions::version_restore`] knows *how* to replace a row's text.
//! This knows what has to be true first, and in what order — which is the part
//! that makes the difference between a feature about not losing work and a
//! feature that loses it.
//!
//! ```text
//!   resolve the row  ─┐  gone?              → refuse, by name
//!   remap the role   ─┤  no home?           → refuse, by name
//!   count comments   ─┤
//!   confirm          ─┤  cancelled?         → nothing happened
//!   safety backup    ─┤  no copy made?      → refuse, and say so
//!   write            ─┤  failed?            → refuse, and say so
//!   save now         ─┘  then: undo is one Ctrl+Z away
//! ```
//!
//! The order is load-bearing at two points. The safety copy is taken **after**
//! the writer confirms (so a cancelled restore costs nothing) and **before** the
//! write (so there is something to fall back on) — and the write happens only
//! from the copy's success path, never merely after asking for one.
//!
//! And the save is immediate rather than left to the autosave debounce. The
//! replacement collapses into one large undo entry; leaving the window with that
//! entry unsaved means a crash loses the restore *and* the text it replaced.

use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction};

use frontend::AppContext;
use frontend::commands::undo_redo_commands;

use crate::backup::{BackupSchedulerViewModel, SafetyBlocker};
use crate::models::OpenDocsStore;
use crate::toast_scope::ToastWorkExt;
use crate::versions::version_restore::{
    self, RestoreRefusal, RestoreRequest, content_id_for, slot_for,
};
use crate::view_models::EditorsViewModel;

/// One toast per feature: a second restore replaces its own snackbar rather than
/// stacking a tower of them.
const RESTORE_TOAST_ID: &str = "versions.restored";

/// How long the "Restored — Undo" snackbar stays up.
const UNDO_GRACE: std::time::Duration = std::time::Duration::from_secs(10);

/// Ask to put `req`'s text back, then — if everything holds — do it.
#[allow(clippy::too_many_arguments)]
pub fn restore_version(
    app_ctx: &Rc<AppContext>,
    docs: &OpenDocsStore,
    scheduler: &BackupSchedulerViewModel,
    editors: &EditorsViewModel,
    stack_id: &Signal<Option<u64>>,
    ctx: &mut EventContext,
    req: RestoreRequest,
) {
    // ── the row, as it is *now* ──────────────────────────────────────────────
    let probe = crate::singles::SingleBinderItem::new(app_ctx.clone());
    probe.set_id(Some(req.item_id));
    let Some(item) = probe.dto() else {
        return refuse(ctx, RestoreRefusal::RowGone);
    };
    let target = match version_restore::resolve_role(&item.role, &item.sub_role, &req.recorded_role)
    {
        Ok(t) => t,
        Err(e) => return refuse(ctx, e),
    };

    // ── what the writer is about to lose ─────────────────────────────────────
    // Counted before anything is written, and named in the confirmation: a
    // whole-content replacement can orphan every comment anchored in the text at
    // once, and doing that in silence is how a writer discovers it much later.
    let comments_at_risk = slot_for(&target)
        .and_then(|slot| {
            let doc = docs.peek(req.item_id)?;
            let content_id = content_id_for(&doc, slot)?;
            let vm = docs.comments()?;
            Some(vm.model().rows_for_content(content_id).len())
        })
        .unwrap_or(0);

    // A blocker known *now* is worth saying now, rather than after the writer
    // has confirmed a destructive edit that was never going to happen.
    if let Some(blocked) = scheduler.safety_backup_blocker() {
        return refuse_backup(ctx, blocked);
    }

    let when = req.taken_at.format("%Y-%m-%d %H:%M").to_string();
    let body = if comments_at_risk > 0 {
        tr!(versions_restore_confirm_with_comments(
            date = when.clone(),
            count = comments_at_risk as i64
        ))
    } else {
        tr!(versions_restore_confirm_text(date = when.clone()))
    };

    let (docs, scheduler, editors, stack_id) = (
        docs.clone(),
        scheduler.clone(),
        editors.clone(),
        stack_id.clone(),
    );
    let app_ctx = app_ctx.clone();
    MessageBox::warning(tr!(versions_restore_confirm_title(date = when.clone())))
        .text(body)
        .informative_text(tr!(versions_restore_confirm_undo_note()))
        .buttons(MessageBoxButtons::Custom(vec![
            StandardButton::Ok.into(),
            StandardButton::Cancel.into(),
        ]))
        .default_button(StandardButton::Cancel)
        .escape_button(StandardButton::Cancel)
        .on_result(move |r, c| {
            if r.button != StandardButton::Ok {
                return;
            }
            let (docs, editors, stack_id, app_ctx) = (
                docs.clone(),
                editors.clone(),
                stack_id.clone(),
                app_ctx.clone(),
            );
            let (target, past, when) = (target.clone(), req.past.clone(), when.clone());
            let item_id = req.item_id;
            // The destructive edit fires from the safety copy's success handler
            // and from nowhere else. `backup_now` returns early on three
            // conditions with nothing but a toast, so "call it, then write"
            // would be a safety net that silently is not there.
            scheduler.backup_before(
                c,
                Rc::new(move |c: &mut EventContext, copied: bool| {
                    if !copied {
                        return refuse(c, RestoreRefusal::SafetyBackupDidNotRun);
                    }
                    // Threaded from `AppIds`, never defaulted: the undo stack id
                    // is a plain caller-supplied `Option<u64>` with nothing
                    // filling it in, so the wrong one bleeds undo across Works
                    // and `None` leaks into the never-cleared global stack 0.
                    let stack = stack_id.get();
                    if let Err(e) = version_restore::apply(&docs, item_id, &target, &past, stack) {
                        return refuse(c, e);
                    }
                    // Immediately, not on the autosave debounce.
                    editors.request_save();
                    let app_ctx = app_ctx.clone();
                    c.show_toast(
                        Toast::success(tr!(versions_restored_toast(date = when.clone())))
                            .scoped_id(RESTORE_TOAST_ID, 0)
                            .auto_dismiss_after(UNDO_GRACE)
                            .action(ToastAction::primary(tr!(versions_undo()), move |_c| {
                                let _ = undo_redo_commands::undo(&app_ctx, stack);
                            })),
                    );
                }),
            );
        })
        .present(ctx);
}

/// Say why, in the writer's terms. Every refusal is named — a restore that
/// silently does nothing is indistinguishable from one that silently did the
/// wrong thing.
fn refuse(ctx: &mut EventContext, why: RestoreRefusal) {
    let message = match why {
        RestoreRefusal::RowGone => tr!(versions_restore_row_gone()),
        RestoreRefusal::RoleHasNoHome { .. } | RestoreRefusal::NotEditableText { .. } => {
            tr!(versions_restore_no_home())
        }
        RestoreRefusal::SafetyBackupDidNotRun => tr!(versions_restore_no_safety_copy()),
        RestoreRefusal::WriteFailed { reason } => tr!(versions_restore_failed(error = reason)),
    };
    ctx.show_toast(
        Toast::error(message)
            .scoped_id(RESTORE_TOAST_ID, 0)
            .auto_dismiss_after(std::time::Duration::from_secs(8)),
    );
}

/// The three obstacles to a safety copy read very differently to a writer, so
/// each says its own thing rather than collapsing into one failure.
fn refuse_backup(ctx: &mut EventContext, blocked: SafetyBlocker) {
    let message = match blocked {
        SafetyBlocker::AlreadyRunning => tr!(versions_restore_backup_busy()),
        SafetyBlocker::BackupFileOpen => tr!(versions_restore_in_backup_file()),
        SafetyBlocker::NoProject => tr!(versions_restore_no_project()),
    };
    ctx.show_toast(
        Toast::warning(message)
            .scoped_id(RESTORE_TOAST_ID, 0)
            .auto_dismiss_after(std::time::Duration::from_secs(8)),
    );
}
