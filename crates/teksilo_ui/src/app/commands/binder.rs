// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The outline tree's verbs. Each drives an `OutlineViewModel` method; the tree's own
//! context menu and key handlers call those same methods directly, so there is one
//! implementation per verb and the command surface is a thin naming layer over it.

use teksilo::prelude::*;
use teksilo::widgets::message_box::EventContextMessageBoxExt;
use teksilo::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton};

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
                // drilled-into container id explicitly. The intent carries a store id, so
                // it is resolved to the tree's durable key here — and an anchor whose row
                // has left the tree falls back to the selection rather than to nothing.
                outline.add_recommended(
                    anchor_item_id.and_then(|id| outline.key_for_item(id)),
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
                if let Some(AppIntent::RevealInOutline { item_id }) = AppIntent::from_intent(i)
                    && let Some(key) = outline.key_for_item(*item_id)
                {
                    outline.reveal_item(key);
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
    {
        // Clear the titles that merely restate their own number.
        //
        // Previewed and confirmed, never silent: this rewrites stored prose-adjacent data
        // across the whole manuscript, and a wrong string match here would destroy a title
        // rather than merely mis-display it. The ids are captured with the preview, so what
        // the writer agreed to is what changes. One undo step covers the lot.
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("numbering.tidy_titles").on_invoke(move |_i, c| {
                let found = outline.redundant_number_titles();
                if found.is_empty() {
                    c.present_message_box(
                        MessageBox::information(tr!(tidy_titles_title()))
                            .text(tr!(tidy_titles_none())),
                    );
                    return;
                }
                // The question stays short — the count and what clearing means — because a
                // manuscript can offer forty of these and an inlined list grew the box past
                // the bottom of the screen. **Every** affected title goes in
                // `detailed_text`, the collapsible the framework already scrolls, so the
                // writer can check the whole list before agreeing rather than a truncated
                // sample of it.
                let details: Vec<String> =
                    found.iter().map(|(_, t)| format!("\u{2022} {t}")).collect();
                let ids: Vec<u64> = found.iter().map(|(id, _)| *id).collect();
                let outline = outline.clone();
                c.present_message_box(
                    MessageBox::question(tr!(tidy_titles_title()))
                        .text(tr!(tidy_titles_lead(count = found.len() as i64)))
                        .informative_text(tr!(tidy_titles_explain()))
                        .detailed_text(lit!(details.join("\n")))
                        .buttons(MessageBoxButtons::Custom(vec![
                            MessageBoxButton::standard(StandardButton::Ok),
                            MessageBoxButton::standard(StandardButton::Cancel),
                        ]))
                        .default_button(StandardButton::Cancel)
                        .escape_button(StandardButton::Cancel)
                        .on_result(move |r, _ctx| {
                            if r.button == StandardButton::Ok {
                                outline.clear_number_titles(&ids);
                            }
                        }),
                );
            }),
        );
    }
}
