// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where the book stands, shown once when a project with an active writing plan opens.
//!
//! A writer who has set a deadline has asked the app to hold them to it. Making them go
//! looking for the answer — open the Book, find the Pace segment — is the app declining the
//! job it was given. So it says so on the way in, once, and gets out of the way: Escape or a
//! click outside closes it, and a checkbox stops it coming back.
//!
//! **Fired after a fresh count, not before.** The numbers Pace draws come from the
//! `ProgressSnapshot` history, which is written on save — so at opening the newest one can
//! be days old, and a summary quoting it would be quietly wrong on exactly the morning it
//! matters. The wiring runs `count_words` first and presents this on its completion, which
//! also closes a real gap: a project opened and never saved used to leave a hole in the
//! streak and the chart.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, FixedSize, HStack, Padding, ScrollArea, Spacer, TextWidget,
    Toggle, VStack,
};

use frontend::AppContext;
use frontend::commands::{binder_item_commands, pace_commands, work_commands};
use frontend::common::direct_access::work::WorkRelationshipField;

use crate::app_ids::AppIds;
use crate::goals::format_goal;

/// "Open this Book's plan" — supplied by the caller, which is the only place that knows how
/// this window opens a tab.
type OpenPace = Rc<dyn Fn(u64, &mut EventContext)>;

const CARD_W: f32 = 520.0;
const CARD_H: f32 = 420.0;

/// One active plan's headline facts, resolved once when the panel is built.
struct PlanRow {
    book_item_id: u64,
    title: String,
    goal: i64,
    written: i64,
}

/// Every Book in this project with an **active** plan.
///
/// Empty means there is nothing to say, and the caller shows nothing at all rather than a
/// modal explaining that it has no news.
fn active_plans(app_ctx: &AppContext, ids: &AppIds) -> Vec<PlanRow> {
    let Some(work_id) = ids.work_id.get() else {
        return Vec::new();
    };
    let pace_ids =
        work_commands::get_work_relationship(app_ctx, &work_id, &WorkRelationshipField::Paces)
            .unwrap_or_default();
    let mut out = Vec::new();
    for pace in pace_commands::get_pace_multi(app_ctx, &pace_ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
    {
        if !pace.active {
            continue;
        }
        let Some(book_item_id) = pace.book_item else {
            continue;
        };
        let Ok(Some(book)) = binder_item_commands::get_binder_item(app_ctx, &book_item_id) else {
            continue;
        };
        // The Book's target is its own `BinderItem` field — the same number the Pace tab's
        // goal box edits and the Inspector shows, never a copy.
        let goal = book.word_count_goal;
        if goal <= 0 {
            continue;
        }
        // Live, from the one measurement service, so this panel cannot disagree with the
        // Book's own page about how far along it is.
        let written = crate::goals::measure::measure(
            app_ctx,
            work_id,
            book_item_id,
            skribisto_model::counting::CountingMethodSetting::default(),
            &frontend::common::entities::GoalUnit::Words,
        )
        .map_or(0, |m| m.subtree.words as i64);
        out.push(PlanRow {
            book_item_id,
            title: book.title,
            goal,
            written,
        });
    }
    out
}

/// Whether this project has anything worth opening the panel for.
pub fn has_active_plan(app_ctx: &AppContext, ids: &AppIds) -> bool {
    !active_plans(app_ctx, ids).is_empty()
}

/// Present the summary. The caller has already decided it should appear.
pub fn present(
    ctx: &mut EventContext,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    show_on_open: Signal<bool>,
    open_pace: OpenPace,
) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(PaceSummaryPanel {
                rows: active_plans(&app_ctx, &ids),
                show_on_open: show_on_open.clone(),
                open_pace: open_pace.clone(),
                root_child: None,
            })
        })
        .presentation(ModalPresentation::InTree)
        .title(tr!(pace_summary_title()))
        .size(CARD_W as u32, CARD_H as u32)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct PaceSummaryPanel {
    rows: Vec<PlanRow>,
    show_on_open: Signal<bool>,
    /// Open the Book's Pace planner — the panel's one forward action.
    open_pace: OpenPace,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for PaceSummaryPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceSummaryPanel").finish()
    }
}

impl Widget for PaceSummaryPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let mut list = VStack::new().spacing(14.0);
        for row in &self.rows {
            let open = self.open_pace.clone();
            let book = row.book_item_id;
            list = list
                .child(
                    VStack::new()
                        .spacing(6.0)
                        .child(
                            TextWidget::new(lit!(row.title.clone()))
                                .style(TextStyleRole::BodyBold)
                                .single_line(),
                        )
                        .child(crate::goals::readout::line(
                            row.written.max(0) as usize,
                            row.goal,
                            &frontend::common::entities::GoalUnit::Words,
                        ))
                        .child(
                            TextWidget::new(tr!(pace_summary_remaining(
                                words = format_goal((row.goal - row.written).max(0))
                            )))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                        )
                        .child(
                            HStack::new().child(
                                Button::new(tr!(pace_summary_open()))
                                    .variant(ButtonVariant::Plain)
                                    .rich_tooltip(crate::tooltip_registry::PACE_PLAN)
                                    .on_activate_fn(move |c| {
                                        open(book, c);
                                        c.dismiss_modal();
                                    }),
                            ),
                        ),
                )
                .child(Divider::new());
        }

        // The setting says "show it"; the checkbox asks not to. A local signal carries the
        // inverted sense, with one effect writing through — a derived signal is read-only,
        // and a `Toggle` needs somewhere to write.
        let hide = Signal::new(!self.show_on_open.get());
        {
            let show = self.show_on_open.clone();
            ctx.effect(&hide, move |off| {
                let want = !*off;
                if show.get() != want {
                    show.set(want);
                }
            });
        }
        let footer = HStack::new()
            .spacing(8.0)
            .child(Toggle::new(hide).label(tr!(pace_summary_dont_show())))
            .child(Spacer::new())
            .child(
                Button::new(tr!(pace_summary_close()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(|c| c.dismiss_modal()),
            );

        let root = ctx.add(
            FixedSize::new().width(CARD_W).height(CARD_H).child(
                Padding::symmetric(16.0, 16.0).child(
                    VStack::new()
                        .spacing(12.0)
                        .child(
                            teksilo::widgets::Expand::vertical()
                                .child(ScrollArea::new().child(list)),
                        )
                        .child(footer),
                ),
            ),
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}
