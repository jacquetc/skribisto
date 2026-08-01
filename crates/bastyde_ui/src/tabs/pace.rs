// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Book's **Pace** segment - the writing-schedule planner.
//!
//! Only the Book container shows it (see `folder_book`): the writer sets a
//! word-count goal and a deadline, picks which weekdays count, and Skribisto
//! derives a pace and shows the statistics (streak, % done, days left, needed
//! rate, ahead/behind) from the recorded `ProgressSnapshot` history. All state +
//! logic live in [`PaceViewModel`](crate::view_models::PaceViewModel); this pane
//! is a thin reactive view over it.
//!
//! Milestones, holidays and the progression / words-per-day charts land in M4d.

use bastyde::core::BindingLevel;
use bastyde::data::{ChartDatum, ChartModel, ChartSeries};
use bastyde::prelude::*;
use bastyde::tokens::{CornerRadius, FontWeight, TextStyle};
use bastyde::widgets::{
    Button, Center, ColumnFlow, DateEdit, DateRange, DateRangeEdit, Expand, FixedSize, FormLayout,
    HStack, IconButton, Padding, Panel, RectWidget, ScrollArea, SpinBox, StepType, TextInput,
    TextWidget, Toggle, VStack, Wrap, ZStack,
};
use bastyde_charts::{BarChart, LineChart};

use chrono::{Datelike, Duration, NaiveDate, Utc};
use jiff::civil::Date;

use crate::date_convert::{jiff_to_naive, naive_to_jiff, naive_to_jiff_opt};
use crate::tabs::ContentTab;
use crate::view_models::PaceViewModel;

use super::shared::{tab_backdrop, vspace};

mod editors;
mod planner;
mod stats;
mod wire;

use editors::*;
use planner::*;
use stats::*;
#[allow(unused_imports)]
use wire::*;

/// (bit, ftl label) for the seven weekday chips. Mon = 1, … Sun = 64 - the mask
/// convention `is_scheduled_day` uses.
pub(super) fn weekdays() -> [(i64, LocalizedString); 7] {
    [
        (1, tr!(pace_day_mon())),
        (2, tr!(pace_day_tue())),
        (4, tr!(pace_day_wed())),
        (8, tr!(pace_day_thu())),
        (16, tr!(pace_day_fri())),
        (32, tr!(pace_day_sat())),
        (64, tr!(pace_day_sun())),
    ]
}

pub fn pace_pane(tab: &ContentTab) -> Box<dyn Widget> {
    let Some(vm) = tab.pace().cloned() else {
        // Only a Book has a Pace view-model; the segment is Book-only, so this is
        // unreachable - fall back to the shell text rather than panic.
        return Box::new(
            Center::new()
                .child(TextWidget::new(tr!(pace_placeholder())).color(TextRole::Secondary)),
        );
    };
    // Local mirror signals for the two-way-bound fields, seeded from the VM and
    // kept in sync by `PaceWire` (external edits) / written back to the VM by it.
    let goal_local = Signal::new(vm.goal_words().get().max(0));
    let end_local = Signal::new(naive_to_jiff_opt(vm.end().get()));
    let active_local = Signal::new(vm.active().get());
    // Switcher index (a real signal, not derived): 0 = no schedule yet, 1 = planner.
    let has_pace = Signal::new(vm.pace_id().get().is_some() as usize);

    let body = VStack::new()
        .spacing(0.0)
        .child(PaceWire::new(
            vm.clone(),
            goal_local.clone(),
            end_local.clone(),
            active_local.clone(),
            has_pace.clone(),
        ))
        .child(vspace(18.0))
        // The empty/planner swap is a layout-forwarding composing widget, **not** a
        // `Switcher`: a Switcher measures *all* its children with an unbounded width (to
        // size to the largest), which pins the planner's `ColumnFlow` to its maximum
        // column count at every viewport width. `PaceBody` builds only the active child
        // and forwards layout to it, so the scroll viewport's bounded width reaches the
        // flow and it reflows.
        .child(PaceBody::new(
            vm,
            goal_local,
            end_local,
            active_local,
            has_pace,
        ))
        .child(vspace(28.0));

    // A stats dashboard, not a prose column: fill the scroll viewport's width and let the
    // planner's `ColumnFlow` reflow into as many columns as it affords, rather than hugging
    // to a centred reading column (which is what collapsed it to one column before). Just
    // horizontal breathing room on the sides.
    tab_backdrop(
        tab.backdrop_role(),
        ScrollArea::new().child(Padding::symmetric(0.0, 24.0).child(body)),
    )
}

/// The Pace pane's body below the wiring: the empty "start planning" state, or the
/// planner dashboard once a Pace exists. A **layout-forwarding** composing widget rather
/// than a `Switcher` (see [`pace_pane`]): it builds only the active child and forwards
/// layout straight to it, so the scroll viewport's bounded width reaches the planner's
/// `ColumnFlow` and it reflows. Rebuilds on `has_pace` (1 = a Pace exists), which
/// [`PaceWire`] keeps in step with `vm.pace_id()`.
pub(super) struct PaceBody {
    vm: PaceViewModel,
    goal_local: Signal<i64>,
    end_local: Signal<Option<Date>>,
    active_local: Signal<bool>,
    has_pace: Signal<usize>,
    root: Option<WidgetId>,
}

impl PaceBody {
    fn new(
        vm: PaceViewModel,
        goal_local: Signal<i64>,
        end_local: Signal<Option<Date>>,
        active_local: Signal<bool>,
        has_pace: Signal<usize>,
    ) -> Self {
        Self {
            vm,
            goal_local,
            end_local,
            active_local,
            has_pace,
            root: None,
        }
    }
}

impl std::fmt::Debug for PaceBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceBody").finish()
    }
}

impl Widget for PaceBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.has_pace
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let child: Box<dyn Widget> = if self.has_pace.get() == 1 {
            Box::new(planner(
                &self.vm,
                &self.goal_local,
                &self.end_local,
                &self.active_local,
            ))
        } else {
            Box::new(empty_state(&self.vm))
        };
        let id = ctx.add_boxed(child);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// One dashboard section: a titled Panel wrapping its body.
pub(super) fn panel_section(title: LocalizedString, body: impl Widget + 'static) -> impl Widget {
    Panel::new().child(
        Padding::uniform(14.0).child(
            VStack::new()
                .spacing(10.0)
                .child(
                    TextWidget::new(title)
                        .style(TextStyleRole::BodyBold)
                        .color(TextRole::Primary),
                )
                .child(body),
        ),
    )
}

/// The "no schedule yet" state - an invitation to create one. `Start planning`
/// sets a default 90-day deadline, which lazily creates the Pace.
pub(super) fn empty_state(vm: &PaceViewModel) -> impl Widget {
    let vm = vm.clone();
    VStack::new()
        .spacing(10.0)
        .child(
            TextWidget::new(tr!(pace_empty_title()))
                .style(TextStyleRole::BodyBold)
                .color(TextRole::Primary),
        )
        .child(TextWidget::new(tr!(pace_empty_body())).color(TextRole::Secondary))
        .child(vspace(4.0))
        .child(
            Button::new(tr!(pace_start_planning())).on_activate_fn(move |_c| {
                let today = Utc::now().date_naive();
                vm.set_dates(today, today + Duration::days(90));
            }),
        )
}

/// The planner, once a Pace exists: the schedule, statistics, charts, holidays and
/// milestones as section Panels flowed through a [`ColumnFlow`], which reflows them into
/// as many columns as the width affords (one when narrow, up to three when wide) and
/// re-partitions to balance the columns' heights. Each section drives its own reactivity,
/// so the flow itself never rebuilds - it only relayouts when a section's height changes.
pub(super) fn planner(
    vm: &PaceViewModel,
    goal_local: &Signal<i64>,
    end_local: &Signal<Option<Date>>,
    active_local: &Signal<bool>,
) -> impl Widget {
    let today = Utc::now().date_naive();

    // Schedule section: goal + deadline + weekdays + active.
    let goal_field = FixedSize::new().width(160.0).child(
        SpinBox::new(goal_local.clone(), 0_i64, 100_000_000)
            .step_type(StepType::Adaptive)
            .on_value_changed({
                let vm = vm.clone();
                move |v, _c| vm.set_goal_words(v)
            }),
    );
    let deadline_field = FixedSize::new().width(200.0).child(
        DateEdit::new(end_local.clone())
            .min_date(naive_to_jiff(today).unwrap_or_else(|| Date::new(2000, 1, 1).unwrap())),
    );
    let schedule_body = VStack::new()
        .spacing(10.0)
        .child(
            FormLayout::new()
                .row_spacing(10.0)
                .line(field_label(tr!(pace_goal())), goal_field)
                .line(field_label(tr!(pace_deadline())), deadline_field),
        )
        .child(WeekdayChips::new(vm.clone()))
        .child(Toggle::new(active_local.clone()).label(tr!(pace_active())));

    ColumnFlow::new()
        .min_column_width(300.0)
        .max_columns(3)
        .column_spacing(12.0)
        .item_spacing(12.0)
        .child(panel_section(tr!(pace_section_schedule()), schedule_body))
        .child(panel_section(
            tr!(pace_section_progress()),
            StatCards::new(vm.clone(), today),
        ))
        .child(panel_section(
            tr!(pace_advancement()),
            PaceCharts::new(vm.clone()),
        ))
        .child(panel_section(
            tr!(pace_section_holidays()),
            HolidayEditor::new(vm.clone()),
        ))
        .child(panel_section(
            tr!(pace_section_milestones()),
            MilestoneList::new(vm.clone()),
        ))
}

/// A short, locale-neutral day label for a chart category (`7/16`).
pub(super) fn day_label(date: NaiveDate) -> String {
    format!("{}/{}", date.month(), date.day())
}

/// A form-row label - small, secondary, single line (matches the settings panes).
pub(super) fn field_label(label: LocalizedString) -> impl Widget {
    FixedSize::new().width(120.0).child(
        TextWidget::new(label)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
            .single_line(),
    )
}
