// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The per-item word or character target.

use frontend::common::entities::{BinderItemSubRole, GoalUnit};
use frontend::direct_access::BinderItemDto;
use teksilo::prelude::*;
use teksilo::widgets::{SpinBox, TextWidget, VStack};

use super::{GoalReadout, Inspector};
use crate::singles::SingleBinderItem;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    ctx: &mut BuildContext,
    d: &BinderItemDto,
) -> VStack {
    // Per-item **word/character target**: how long this piece is meant to be.
    //
    // The field has existed on every `BinderItem` since the first commit of the
    // format, and the legacy `.skrib` upgrader has always migrated it — but
    // until now nothing could write it except the Book's Pace planner, so a
    // writer opening an old project had targets in the store and no way to see
    // or change a single one.
    //
    // Offered on every row except a Paratext, whose whole documented purpose is
    // to sit outside every statistic; a target there would be measured against
    // prose that is counted nowhere. The **Book** deliberately keeps it too,
    // alongside Pace's own field: both write this same number through the same
    // signal, so the two doors cannot disagree, and a writer who never opens
    // the Pace planner should still be able to say how long the book is.
    if !matches!(d.sub_role, BinderItemSubRole::Paratext) {
        let unit = panel.goal_unit.get();
        let current = match unit {
            GoalUnit::Words => d.word_count_goal,
            GoalUnit::Characters => d.char_count_goal,
        };
        let value = Signal::new(current);
        let item_probe = SingleBinderItem::new(panel.app_ctx.clone());
        item_probe.set_id(Some(d.id));
        let stack = panel.outline.ids().stack_id.get();
        {
            let probe = item_probe.clone();
            let unit = unit.clone();
            // Same guarded write as the toggles above: only on a genuine
            // change, never on the seed nor on the post-write echo (the entity
            // Updated event rebuilds this panel).
            ctx.effect(&value, move |v| {
                let seen = probe.dto().map(|d| match unit {
                    GoalUnit::Words => d.word_count_goal,
                    GoalUnit::Characters => d.char_count_goal,
                });
                if seen == Some(*v) {
                    return;
                }
                let wrote = match unit {
                    GoalUnit::Words => probe.set_word_count_goal(*v, stack),
                    GoalUnit::Characters => probe.set_char_count_goal(*v, stack),
                };
                if let Err(e) = wrote {
                    eprintln!("inspector: set target failed: {e}");
                }
            });
        }
        col = col.child(
            TextWidget::new(tr!(inspector_goal()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
        col = col.child(
            SpinBox::new(value, 0_i64, 100_000_000)
                .single_step(100)
                // `0` is the no-target sentinel this field has always used, so
                // the bottom of the range has to *read* as "none" rather than
                // as a target of zero words, which means nothing.
                .special_value_text(tr!(inspector_goal_none()))
                .rich_tooltip(crate::tooltip_registry::GOAL_TARGET),
        );
        // What the target is measured against, so the number above is never
        // shown without the thing it is compared to.
        col = col.child(GoalReadout::new(
            panel.app_ctx.clone(),
            panel.outline.ids().work_id.clone(),
            d.id,
            current,
            panel.counting_method.clone(),
            unit,
        ));
    }
    col
}
