// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Keeping an open editor honest when something rewrites its prose underneath.
//!
//! Not a view-model — one subscriber, installed once per window.
//!
//! # Why one subscriber rather than a call per command
//!
//! Several commands rewrite a `Content` row that a tab may have open: restoring
//! a trashed subtree, `split_scene`, `merge_two_scenes`, `apply_document_import`,
//! `replace_in_project` — and now **undo and redo of any of them**. An open
//! editor deliberately does not follow its own row (`SingleContent::wire`:
//! *"the editor owns its live document, so it does NOT wire this for an open
//! tab"*), so each of those commands had to remember to reload the tabs it
//! touched. Two of them did, by hand, in two different files. Anything added
//! later would have had to remember too.
//!
//! The unit of work emits `Content(Updated)` with real ids on commit, so one
//! subscriber covers every one of them — and every one added after this.
//!
//! # The trap this exists to avoid
//!
//! Reloading on that event *unconditionally* would be worse than the bug it
//! fixes. `OpenDoc::reload` calls `set_djot`, and every whole-document setter in
//! text-document clears that document's undo history. This window's own autosave
//! also emits `Content(Updated)` — every few seconds — so a naive subscriber
//! would delete the writer's typing history continuously. Hence
//! `reload_if_diverged`, which compares the stored prose against what this
//! window last wrote and does nothing when they agree.
//!
//! When it *does* reload, the writer is told: a typing history vanishing in
//! silence reads as a bug, and this is the one place that knows it happened.

use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use teksilo::prelude::*;
use teksilo::widgets::Toast;

use crate::models::OpenDocsStore;
use crate::toast_scope::ToastWorkExt;

/// Install the subscriber. Called once from `App::build`.
pub(crate) fn install(ctx: &mut BuildContext, docs: OpenDocsStore) {
    let docs_updated = docs.clone();
    ctx.subscribe_event_with_ctx(
        Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
        move |event: &Event, c: &mut EventContext| {
            let docs = &docs_updated;
            let touched = docs.items_owning_contents(&event.ids);
            if touched.is_empty() {
                return;
            }
            let reloaded = docs.reload_if_diverged(&touched);
            if reloaded.is_empty() {
                return;
            }
            // `set_djot_sync` only queues a document event; without a frame the
            // editor keeps painting the old prose.
            c.request_frame();
            c.show_toast(
                Toast::info(tr!(prose_history_reset_title()))
                    .body(tr!(prose_history_reset_body(count = reloaded.len() as i64)))
                    // One toast for the whole burst: a Replace All undo can touch
                    // forty scenes, and forty snackbars is not a notification.
                    .scoped_id("prose.history-reset", None),
            );
        },
    );
    // A command can also *create* a row an open document shows: a merge folding a
    // scratchpad or a synopsis into a scene that had none. Its id names no open field
    // yet, so the subscriber above cannot find it. The docs with a field still waiting
    // for its row are the only ones it can belong to, and among those only the items
    // whose stored contents now list it are re-read, and in them only the fields with
    // no row: re-reading by `(item, role)` binds such a field to the new row and shows
    // its text. Without it the field kept showing nothing, and its next flush created a
    // second row of the same role. A field that already has a row is the subscriber
    // above's, which says so when it resets a history.
    //
    // No toast: a field with no row has had nothing saved, so there is no history in
    // it to lose. A field still holding unsaved typing is left alone here (reloading
    // would drop it); its next save finds the new row and writes its text after the
    // row's (`SingleContent::save`), so it still ends on one row.
    let docs_created = docs.clone();
    ctx.subscribe_event_with_ctx(
        Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Created)),
        move |event: &Event, c: &mut EventContext| {
            if !docs_created.bind_created_rows(&event.ids).is_empty() {
                c.request_frame();
            }
        },
    );
    // And a command can take away the row an open field writes to: an undo removing a
    // row made since its command, or a redo joining it onto the row the command brings
    // back (`common::prose_keep`). The field is bound to whatever row of its role the
    // item holds now and shows its text; left alone, it kept showing the old text and
    // its next flush wrote to a row that no longer exists. A field still holding unsaved
    // typing keeps it, and its next save goes to the item's row of its role
    // (`SingleContent::save`).
    let docs_removed = docs.clone();
    ctx.subscribe_event_with_ctx(
        Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Removed)),
        move |event: &Event, c: &mut EventContext| {
            let reloaded = docs_removed.rebind_removed_rows(&event.ids);
            if reloaded.is_empty() {
                return;
            }
            c.request_frame();
            c.show_toast(
                Toast::info(tr!(prose_history_reset_title()))
                    .body(tr!(prose_history_reset_body(count = reloaded.len() as i64)))
                    .scoped_id("prose.history-reset", None),
            );
        },
    );
}
