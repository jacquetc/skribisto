// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Whether this item is swept in when a structural scope compiles.

use frontend::direct_access::BinderItemDto;
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, TextWidget, Toggle, VStack};

use super::Inspector;
use crate::singles::SingleBinderItem;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    ctx: &mut BuildContext,
    d: &BinderItemDto,
) -> VStack {
    // Per-item **export** toggle (M3): whether this item is included when a
    // structural scope (Book / Chapter / Folder) sweeps it in. On by default; an
    // explicit Export Scene/Note or a checked Choose… item overrides it. Beside
    // it, "Apply to children" pushes this value across the whole subtree in one
    // undo step (shown only when the item actually has a subtree).
    {
        let value = Signal::new(d.is_exportable);
        let item_probe = SingleBinderItem::new(panel.app_ctx.clone());
        item_probe.set_id(Some(d.id));
        let stack = panel.outline.ids().stack_id.get();
        {
            let probe = item_probe.clone();
            // Write only on a genuine change — never on the initial seed nor the
            // post-write echo (the entity Updated event rebuilds this panel), so
            // the toggle can't feed back into itself.
            ctx.effect(&value, move |on| {
                if probe.dto().map(|d| d.is_exportable) != Some(*on) {
                    // The `Toggle` below is bound straight to `value`, so it
                    // already shows `*on` by the time we get here — there is
                    // no "don't update the mirror" option like the pill
                    // fields have. On failure the entity Updated event this
                    // panel rebuilds on never fires, so log it: the toggle
                    // stays wrong until something else (a focus change, a
                    // promote) forces a fresh read.
                    if let Err(e) = probe.set_exportable(*on, stack) {
                        eprintln!("inspector: set exportable failed: {e}");
                    }
                }
            });
        }
        col = col.child(
            TextWidget::new(tr!(inspector_export()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
        col = col.child(Toggle::new(value.clone()).label(tr!(inspector_exportable())));
        if !panel.outline.subtree_descendants(d.id).is_empty() {
            let outline = panel.outline.clone();
            let id = d.id;
            col = col.child(
                Button::new(tr!(inspector_apply_to_children()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |_c| outline.apply_exportable_to_subtree(id, value.get())),
            );
        }
    }
    col
}
