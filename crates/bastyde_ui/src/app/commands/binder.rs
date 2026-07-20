// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The outline tree's verbs. Each drives an `OutlineViewModel` method; the tree's own
//! context menu and key handlers call those same methods directly, so there is one
//! implementation per verb and the command surface is a thin naming layer over it.

use bastyde::prelude::*;

use crate::intents::AppIntent;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(Action::new("binder.new_item").on_invoke(move |i, _c| {
            if let Some(AppIntent::NewItem {
                create_type,
                relation,
                anchor_item_id,
            }) = AppIntent::from_intent(i)
            {
                // `None` anchors on the current Outline selection; a corkboard passes its
                // drilled-into container id explicitly.
                outline.add_recommended(
                    anchor_item_id.map(crate::models::BinderTreeKey::Item),
                    &skribisto_model::Recommendation {
                        create_type: *create_type,
                        relation: *relation,
                    },
                );
            }
        }));
    }
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("binder.rename").on_invoke(move |_i, c| outline.rename_selected(c)),
        );
    }
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("binder.duplicate").on_invoke(move |_i, _c| outline.duplicate_selected()),
        );
    }
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("binder.trash_selected").on_invoke(move |_i, _c| outline.trash_selected()),
        );
    }
    {
        // Trash one specific binder (id in the intent payload) — fired from the switcher
        // popover's context menu after its confirmation.
        let outline = deps.outline.clone();
        ctx.register_action_global(Action::new("binder.trash").on_invoke(move |i, _c| {
            if let Some(AppIntent::TrashBinder { binder_id }) = AppIntent::from_intent(i) {
                outline.trash_binder(*binder_id as u64);
            }
        }));
    }
    {
        // Reveal an item in the outline — fired from the Overview table, which knows the
        // item but deliberately holds no reference to the outline view-model. `App` is
        // the mediator, so the view-model graph stays a DAG.
        let outline = deps.outline.clone();
        ctx.register_action_global(Action::new("binder.reveal_in_outline").on_invoke(
            move |i, _c| {
                if let Some(AppIntent::RevealInOutline { item_id }) = AppIntent::from_intent(i) {
                    outline.reveal_item(crate::models::BinderTreeKey::Item(*item_id));
                }
            },
        ));
    }
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("binder.indent").on_invoke(move |_i, _c| outline.indent_selected()),
        );
    }
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("binder.outdent").on_invoke(move |_i, _c| outline.outdent_selected()),
        );
    }
    ctx.register_shortcut_global(
        Shortcut::new("binder.duplicate")
            .name("Duplicate")
            .primary(KeyStroke::ctrl(Key::D))
            .build(),
    );
}
