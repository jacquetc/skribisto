// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Replace All — the confirmation dialog, the execution, and the undo.
//!
//! This is the one piece of the search feature that needs an `EventContext` (a
//! dialog, a toast), so it lives beside the view (not in the view-model): the VM
//! does the work ([`SearchReplaceViewModel::replace_all`] and friends), and this
//! orchestrates the *presentation* around it.
//!
//! **The undo is the toast's, not `Ctrl+Z`.** App-level undo is not wired to any
//! keystroke in `bastyde_ui` (the global shortcuts hold no `Ctrl+Z`; `Ctrl+Z`
//! reaches only a focused editor's own local buffer undo), so a `snapshot_work`
//! entry from the replace would sit on a stack nothing can pop. The completion
//! toast therefore carries its own **Undo** action, which calls the backend undo
//! directly on the project's stack. The confirmation dialog promises exactly that
//! — never `Ctrl+Z`.

use bastyde::prelude::*;
use bastyde::widgets::{
    MessageBox, MessageBoxButtons, MessageBoxResult, StandardButton, Toast, ToastAction,
};

use crate::toast_scope::ToastWorkExt;
use crate::view_models::SearchReplaceViewModel;

/// Confirm, then Replace All. Names both strings and both counts, and promises
/// the toast's Undo. Does nothing if the scan can't honestly be replaced over
/// (`can_replace_all` is false — the button that fires this is already disabled
/// then, but a keyboard activation could still reach it).
pub fn confirm_and_replace(vm: &SearchReplaceViewModel, ctx: &mut EventContext) {
    if !vm.can_replace_all() {
        return;
    }
    let query = vm.query_signal().get();
    let replacement = vm.replacement_signal().get();
    let occurrences = vm.match_count_signal().get();
    let items = vm.item_count_signal().get();

    let vm = vm.clone();
    MessageBox::warning(tr!(search_replace_confirm_title()))
        .text(tr!(search_replace_confirm_text(
            query = query,
            replacement = display_replacement(&replacement),
            occurrences = occurrences as i64,
            items = items as i64
        )))
        .buttons(MessageBoxButtons::OkCancel)
        .on_result(move |r: MessageBoxResult, ctx| {
            if r.button == StandardButton::Ok {
                execute(&vm, ctx);
            }
        })
        .present(ctx);
}

/// Run the replace, reload every touched open document, re-run the search, and
/// raise the completion toast (with Undo) — or an error toast if it failed.
fn execute(vm: &SearchReplaceViewModel, ctx: &mut EventContext) {
    // Capture what will be touched BEFORE the replace resets the result set, so
    // the reload (and the undo's reload) target the right items.
    let touched = vm.touched_item_ids();
    match vm.replace_all() {
        Ok(res) => {
            reload_and_rescan(vm, &touched, ctx);

            let occurrences = res.occurrences_replaced;
            let items = res.items_changed;
            let skipped = res.skipped_stale.len();

            let vm_undo = vm.clone();
            let undo_touched = touched.clone();
            let mut toast = Toast::success(tr!(search_replace_done_title()))
                .body(done_body(occurrences, items, skipped))
                .action(ToastAction::primary(
                    tr!(search_replace_undo()),
                    move |ctx| {
                        undo(&vm_undo, &undo_touched, ctx);
                    },
                ));
            // Keep it up a little longer than a default toast — the Undo is the
            // only path back, so the writer must have time to reach for it.
            toast = toast.id("search-replace-result");
            // Work-scoped: a replace-all edits this Work's own prose, so its
            // result (and Undo) belongs to this Work's window/bell, not every
            // open project's.
            ctx.show_toast(toast.target_work(vm.work_id()));
        }
        Err(e) => {
            ctx.show_toast(
                Toast::error(tr!(search_replace_failed_title()))
                    .id("search-replace-result")
                    .body(lit!(e.to_string()))
                    .target_work(vm.work_id()),
            );
        }
    }
}

/// Undo the last Replace All (the toast's action): reverse it on the project's
/// stack, then reload the touched open docs and re-run the search so the panel
/// and any open editors reflect the restored prose.
fn undo(vm: &SearchReplaceViewModel, touched: &[u64], ctx: &mut EventContext) {
    match vm.undo_last_replace() {
        Ok(()) => reload_and_rescan(vm, touched, ctx),
        Err(e) => {
            ctx.show_toast(
                Toast::error(tr!(search_replace_undo_failed_title()))
                    .body(lit!(e.to_string()))
                    .target_work(vm.work_id()),
            );
        }
    }
}

/// Reload the touched open documents (so an open tab shows the new/restored
/// text) and re-run the search (so the result set reflects the changed prose).
///
/// **`request_frame` is load-bearing.** `reload_touched` applies the new prose to
/// each open document via `set_djot().wait()` — durable in the store, and pushed
/// onto the editor's event queue — but a `RichTextEditor` only drains that queue
/// on frames the tree is *asked* to pump. A Replace All mutates the document from
/// *outside* the editor's own handlers, so nothing schedules that frame: the open
/// tab would keep painting the old prose until the writer's next interaction
/// (a click) happened to pump one. Requesting a frame here is what makes the
/// change appear immediately — the same pattern `StreamViewModel` uses after its
/// own reload.
fn reload_and_rescan(vm: &SearchReplaceViewModel, touched: &[u64], ctx: &mut EventContext) {
    vm.reload_touched(touched);
    vm.run_now();
    ctx.request_frame();
}

/// The completion toast body — with a trailing note when some fields were
/// skipped because their occurrence count no longer matched what was reviewed.
fn done_body(occurrences: u64, items: u64, skipped: usize) -> LocalizedString {
    if skipped > 0 {
        tr!(search_replace_done_skipped(
            occurrences = occurrences as i64,
            items = items as i64,
            skipped = skipped as i64
        ))
    } else {
        tr!(search_replace_done(
            occurrences = occurrences as i64,
            items = items as i64
        ))
    }
}

/// An empty replacement is legitimate ("delete every match"); name it explicitly
/// in the confirmation rather than showing empty quotes. Returns a plain `String`
/// (a `tr!` interpolation arg, not a nested `LocalizedString`).
fn display_replacement(replacement: &str) -> String {
    if replacement.is_empty() {
        tr!(search_replace_nothing()).resolve_now()
    } else {
        replacement.to_string()
    }
}
