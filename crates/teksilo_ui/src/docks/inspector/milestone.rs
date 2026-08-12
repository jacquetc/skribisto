// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A target date for this Part or Chapter, pinned on the Book's Pace timeline.

use jiff::civil::Date;

use frontend::common::entities::BinderItemSubRole;
use frontend::direct_access::BinderItemDto;
use skribisto_model::compile::{StreamLevel, enclosing_head};
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, DateEdit, HStack, TextWidget, VStack};

use super::{Inspector, live_item_metas};
use crate::date_convert::{jiff_to_naive, naive_to_jiff_opt};
use crate::singles::SingleMilestone;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    ctx: &mut BuildContext,
    d: &BinderItemDto,
    metas: &Option<Vec<skribisto_model::compile::ItemMeta>>,
) -> VStack {
    // Per-Part/Chapter **milestone** (M5): a target date pinned on the Book's
    // Pace timeline, set right where the writer plans the section. A milestone
    // only makes sense for a compile-stream Part or Chapter, and only inside a
    // Book — so gate on the sub_role, then resolve the enclosing Book head.
    if matches!(
        d.sub_role,
        BinderItemSubRole::Part | BinderItemSubRole::ChapterScene
    ) {
        let metas = metas
            .clone()
            .unwrap_or_else(|| live_item_metas(&panel.app_ctx, &panel.outline.ids()));
        if let Some(book_id) = metas
            .iter()
            .position(|m| m.id == d.id)
            .and_then(|pos| enclosing_head(&metas, pos, StreamLevel::Book))
            .map(|head| metas[head].id)
        {
            let probe = SingleMilestone::new(panel.app_ctx.clone(), panel.outline.ids());
            probe.set_book_and_item(Some(book_id), Some(d.id));
            probe.wire(ctx);
            // jiff mirror for the DateEdit; two guarded effects bridge the
            // entity's chrono date <-> the widget's jiff date, each writing only
            // on a genuine change so neither echoes the other into a loop.
            let local: Signal<Option<Date>> =
                Signal::new(naive_to_jiff_opt(probe.date_signal().get()));
            {
                let local = local.clone();
                ctx.effect(&probe.date_signal(), move |d| {
                    let jd = naive_to_jiff_opt(*d);
                    if local.get() != jd {
                        local.set(jd);
                    }
                });
            }
            {
                let probe = probe.clone();
                ctx.effect(&local, move |jd| {
                    let want = jd.map(jiff_to_naive);
                    if probe.date() != want {
                        probe.set_date(want);
                    }
                });
            }
            let has_date = local.map(|d| d.is_some());
            col = col
                .child(
                    TextWidget::new(tr!(inspector_milestone()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                )
                .child(
                    HStack::new()
                        .spacing(8.0)
                        .child(
                            DateEdit::new(local.clone())
                                .placeholder(tr!(inspector_milestone_none())),
                        )
                        .child(
                            Button::new(tr!(inspector_milestone_clear()))
                                .variant(ButtonVariant::Plain)
                                .enabled(has_date)
                                .on_activate_fn({
                                    let local = local.clone();
                                    move |_c| local.set(None)
                                }),
                        ),
                );
        }
    }
    col
}
