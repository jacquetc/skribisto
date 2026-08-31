// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The per-item spell-check language override.
//!
//! Nothing inherits from a container — an item's tag reaches only that item (see
//! `skribisto_model::language`) — so "Apply to children" beside the field is the
//! *only* way a language spreads down a subtree.

use std::rc::Rc;

use frontend::direct_access::BinderItemDto;
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, TextWidget, VStack};

use super::Inspector;
use crate::singles::SingleBinderItem;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    ctx: &mut BuildContext,
    d: &BinderItemDto,
) -> VStack {
    // Per-item language override (Step 9): the pill field over this item's own
    // `dict_language`, with the Work's list as the placeholder shown when the item
    // declares none of its own. Nothing inherits from a container — an item's tag
    // reaches only that item (see `skribisto_model::language`) — so "Apply to
    // children" beside it is the *only* way a language spreads down a subtree.
    if let Some(spell) = ctx
        .app_state::<crate::spellcheck::SpellcheckService>()
        .cloned()
    {
        let inherited = Some(panel.open_docs.effective_language(d.id));
        let value = Signal::new(d.dict_language.clone());
        // A probe fixed to *this* item, so the write targets it even after focus
        // moves on (unlike the shared `panel.probe`).
        let item_probe = SingleBinderItem::new(panel.app_ctx.clone());
        item_probe.set_id(Some(d.id));
        let stack = panel.outline.ids().stack_id.get();
        let set: crate::spellcheck::language_pill_field::SetLanguages = {
            let value = value.clone();
            // Same "only echo a landed write" reasoning as `set_tags` above.
            Rc::new(
                move |new: Vec<String>, _c| match item_probe.set_dict_language(&new, stack) {
                    Ok(()) => value.set(new),
                    Err(e) => {
                        eprintln!("inspector: set dictionary language failed: {e}")
                    }
                },
            )
        };
        col = col
            .child(
                TextWidget::new(tr!(inspector_dict_language()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(
                crate::spellcheck::language_pill_field::LanguagePillField::new(
                    value.clone(),
                    set,
                    spell,
                    inherited.clone(),
                    panel.open_docs.clone(),
                ),
            );
        // Push this language down the subtree, one undo step (shown only when the
        // item actually has a subtree — the same gate the export toggle uses).
        //
        // It applies the list the pills **display**, not the raw field: when the
        // item declares nothing of its own the pills show the Work's language, and
        // stamping something else than what the writer is looking at would be a
        // lie. The consequence is deliberate — the descendants end up carrying a
        // real tag, so a later change to the Work's language no longer reaches
        // them. That is what "apply" means here, and one Undo takes it back.
        if !panel.outline.subtree_descendants(d.id).is_empty() {
            let outline = panel.outline.clone();
            let id = d.id;
            let value = value.clone();
            let placeholder = panel.open_docs.effective_language(d.id);
            col = col.child(
                Button::new(tr!(inspector_apply_language_to_children()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |_c| {
                        let raw = value.get();
                        let tags = if raw.iter().all(|t| t.trim().is_empty()) {
                            placeholder.clone()
                        } else {
                            raw
                        };
                        outline.apply_dict_language_to_subtree(id, &tags);
                    }),
            );
        }
    }
    col
}
