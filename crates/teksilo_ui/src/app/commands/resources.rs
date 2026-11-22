// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `resources.open` and `resources.insert_dropped_picture`: the two Resources commands a
//! text fires.
//!
//! Ctrl+click on a `resource:` link in any text sends [`AppIntent::OpenResource`]
//! (`resources::links::activate_link`), answered here by the window's
//! [`ResourcesViewModel`](crate::resources::ResourcesViewModel). A picture resource
//! dropped on a text and chosen as a picture is parked on the window's
//! `FormatViewModel` by the drop (`resources::text_drop`) and inserted here, where the
//! project's pictures folder and its undo stack are in hand. The dock's own `*.show`
//! command comes from `docks::panels`, like every panel's.

use teksilo::prelude::*;

use crate::intents::AppIntent;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    {
        let vm = deps.resources.clone();
        ctx.register_action_global(Action::new("resources.open").on_invoke(move |i, c| {
            if let Some(AppIntent::OpenResource { uid }) = AppIntent::from_intent(i)
                && let Ok(uid) = uuid::Uuid::parse_str(uid)
            {
                vm.open_uid(c, uid);
            }
        }));
    }
    {
        let vm = deps.resources.clone();
        ctx.register_action_global(Action::new("resources.attach").on_invoke(move |i, c| {
            if let Some(AppIntent::AttachResources {
                item_id,
                resource_ids,
            }) = AppIntent::from_intent(i)
            {
                vm.attach_dropped(c, *item_id, resource_ids);
            }
        }));
    }
    {
        let vm = deps.resources.clone();
        let format = deps.format.clone();
        ctx.register_action_global(
            Action::new(crate::resources::text_drop::INSERT_DROPPED_PICTURE).on_invoke(
                move |_i, c| {
                    if let Some(dropped) = format.take_dropped_resource() {
                        vm.insert_picture(c, &dropped.handle, dropped.at, &dropped.row);
                    }
                },
            ),
        );
    }
}
