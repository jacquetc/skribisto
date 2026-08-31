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
    ctx.subscribe_event_with_ctx(
        Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
        move |event: &Event, c: &mut EventContext| {
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
}
