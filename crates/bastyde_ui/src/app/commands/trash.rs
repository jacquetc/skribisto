// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trash dock's verbs: reveal it, empty it, restore out of it, delete forever.
//!
//! Each destructive one goes through a `TrashViewModel::confirm_*` method rather than acting
//! directly — the confirmation is the view-model's business, not the command's.

use bastyde::prelude::*;

use crate::intents::AppIntent;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    {
        let docking = deps.outline.docking();
        let trash_dock = deps.trash_dock;
        ctx.register_action_global(Action::new("trash.show").on_invoke(move |_i, _c| {
            docking.reveal_dock(trash_dock);
        }));
    }
    {
        let trash = deps.trash.clone();
        ctx.register_action_global(
            Action::new("trash.empty").on_invoke(move |_i, c| trash.confirm_empty_trash(c)),
        );
    }
    {
        let trash = deps.trash.clone();
        ctx.register_action_global(Action::new("trash.restore").on_invoke(move |i, c| {
            if let Some(AppIntent::RestoreTrashed { trash_info_ids }) = AppIntent::from_intent(i) {
                trash.restore(c, trash_info_ids);
            }
        }));
    }
    {
        let trash = deps.trash.clone();
        ctx.register_action_global(Action::new("trash.restore_item").on_invoke(move |i, c| {
            if let Some(AppIntent::RestoreTrashedItem { item_id }) = AppIntent::from_intent(i) {
                trash.restore_item(c, *item_id);
            }
        }));
    }
    {
        let trash = deps.trash.clone();
        ctx.register_action_global(Action::new("trash.delete_forever").on_invoke(move |i, c| {
            if let Some(AppIntent::DeleteTrashForever { trash_info_ids }) = AppIntent::from_intent(i)
            {
                trash.confirm_delete_forever(c, trash_info_ids);
            }
        }));
    }
}
