// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The prologue lever: keep the row in the book, drop only its numeral.

use frontend::direct_access::BinderItemDto;
use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{TextWidget, Toggle, VStack};

use super::Inspector;
use crate::singles::SingleBinderItem;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    ctx: &mut BuildContext,
    d: &BinderItemDto,
) -> VStack {
    // Per-item **numbering** toggle: the prologue lever. Mounted only on rows
    // that actually open a structural level — a Scene or a Note has no ordinal
    // to suppress, and an affordance that does nothing is worse than none.
    //
    // Deliberately *not* folded into the export toggle above: that one takes
    // the row out of the book entirely (no prose, no heading, no word count),
    // while this keeps all of it and removes only the numeral — and the slot it
    // would have consumed, which is the half that matters. A prologue is in the
    // book; it is simply not chapter one.
    if skribisto_model::numbering::level_of(&d.sub_role).is_some() {
        let value = Signal::new(!d.exclude_from_numbering);
        let item_probe = SingleBinderItem::new(panel.app_ctx.clone());
        item_probe.set_id(Some(d.id));
        let stack = panel.outline.ids().stack_id.get();
        {
            let probe = item_probe.clone();
            // Same guarded-write shape as the export toggle: write only on a
            // genuine change, never on the seed or the post-write echo.
            ctx.effect(&value, move |on| {
                let want_excluded = !*on;
                if probe.dto().map(|d| d.exclude_from_numbering) != Some(want_excluded)
                    && let Err(e) = probe.set_excluded_from_numbering(want_excluded, stack)
                {
                    eprintln!("inspector: set numbered failed: {e}");
                }
            });
        }
        col = col.child(
            TextWidget::new(tr!(inspector_numbering()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
        // The label is one word, so the *rule* — that switching this off also
        // stops the row consuming a number, which is the whole point for a
        // prologue — lives in the tooltip rather than in a label nobody can
        // read at a glance.
        col = col.child(
            Toggle::new(value)
                .label(tr!(inspector_numbered()))
                .rich_tooltip_content(
                    TooltipContent::new("inspector.numbered", tr!(inspector_numbered_tip()))
                        .with_more(tr!(inspector_numbered_tip_more())),
                ),
        );
    }
    col
}
