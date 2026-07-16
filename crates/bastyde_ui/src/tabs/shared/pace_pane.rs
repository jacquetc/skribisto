//! The Book's **Pace** segment — the writing-schedule planner.
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
use bastyde::data::{ChartModel, ChartSeries};
use bastyde::prelude::*;
use bastyde::tokens::CornerRadius;
use bastyde::widgets::{
    Button, Center, DateEdit, FixedSize, FormLayout, RectWidget, ScrollArea, SpinBox, StepType,
    Switcher, TextWidget, Toggle, VStack, Wrap, ZStack,
};
use bastyde_charts::{BarChart, LineChart};

use chrono::{Datelike, Duration, NaiveDate, Utc};
use jiff::civil::Date;

use crate::date_convert::{jiff_to_naive, naive_to_jiff, naive_to_jiff_opt};
use crate::tabs::ContentTab;
use crate::view_models::PaceViewModel;

use super::{centered, tab_backdrop, vspace};

/// (bit, ftl label) for the seven weekday chips. Mon = 1, … Sun = 64 — the mask
/// convention `is_scheduled_day` uses.
fn weekdays() -> [(i64, LocalizedString); 7] {
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
        // unreachable — fall back to the shell text rather than panic.
        return Box::new(
            Center::new().child(TextWidget::new(tr!(pace_placeholder())).color(TextRole::Secondary)),
        );
    };
    let cw = tab.column_width.clone();

    // Local mirror signals for the two-way-bound fields, seeded from the VM and
    // kept in sync by `PaceWire` (external edits) / written back to the VM by it.
    let goal_local = Signal::new(vm.goal_words().get().max(0));
    let end_local = Signal::new(naive_to_jiff_opt(vm.end().get()));
    let active_local = Signal::new(vm.active().get());
    // Switcher index (a real signal, not derived): 0 = no schedule yet, 1 = planner.
    let has_pace = Signal::new(vm.pace_id().get().is_some() as usize);

    let planner: Box<dyn Widget> = Box::new(planner(&vm, &goal_local, &end_local, &active_local));
    let empty: Box<dyn Widget> = Box::new(empty_state(&vm));

    let body = VStack::new()
        .spacing(0.0)
        .child(PaceWire::new(
            vm,
            goal_local,
            end_local,
            active_local,
            has_pace.clone(),
        ))
        .child(vspace(18.0))
        .child(Switcher::new(has_pace).child_boxed(empty).child_boxed(planner))
        .child(vspace(28.0));

    tab_backdrop(ScrollArea::new().child(centered(body, &cw)))
}

/// A section heading — normal case (never all-caps, per house style), spaced above.
fn section(label: LocalizedString) -> impl Widget {
    VStack::new()
        .child(vspace(6.0))
        .child(
            TextWidget::new(label)
                .style(TextStyleRole::BodyBold)
                .color(TextRole::Primary),
        )
        .child(vspace(2.0))
}

/// The "no schedule yet" state — an invitation to create one. `Start planning`
/// sets a default 90-day deadline, which lazily creates the Pace.
fn empty_state(vm: &PaceViewModel) -> impl Widget {
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
        .child(Button::new(tr!(pace_start_planning())).on_activate_fn(move |_c| {
            let today = Utc::now().date_naive();
            vm.set_dates(today, today + Duration::days(90));
        }))
}

/// The schedule form + statistics, shown once a Pace exists.
fn planner(
    vm: &PaceViewModel,
    goal_local: &Signal<i64>,
    end_local: &Signal<Option<Date>>,
    active_local: &Signal<bool>,
) -> impl Widget {
    let today = Utc::now().date_naive();

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

    let schedule = FormLayout::new()
        .row_spacing(10.0)
        .line(field_label(tr!(pace_goal())), goal_field)
        .line(field_label(tr!(pace_deadline())), deadline_field)
        .full_width(WeekdayChips::new(vm.clone()))
        .full_width(Toggle::new(active_local.clone()).label(tr!(pace_active())));

    VStack::new()
        .spacing(6.0)
        .child(section(tr!(pace_section_schedule())))
        .child(schedule)
        .child(section(tr!(pace_section_progress())))
        .child(PaceStats::new(vm.clone(), today))
        .child(vspace(10.0))
        .child(PaceCharts::new(vm.clone()))
}

/// A short, locale-neutral day label for a chart category (`7/16`).
fn day_label(date: NaiveDate) -> String {
    format!("{}/{}", date.month(), date.day())
}

/// A form-row label — small, secondary, single line (matches the settings panes).
fn field_label(label: LocalizedString) -> impl Widget {
    FixedSize::new().width(120.0).child(
        TextWidget::new(label)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
            .single_line(),
    )
}

// ── PaceWire: wire the view-model + the two-way binding effects ──────────────

/// Zero-size child that, on build, wires the view-model (its event subscriptions)
/// and registers the effects syncing the local field mirrors with the view-model
/// — the one place in the pane's tree that has a `BuildContext`.
struct PaceWire {
    vm: PaceViewModel,
    goal_local: Signal<i64>,
    end_local: Signal<Option<Date>>,
    active_local: Signal<bool>,
    has_pace: Signal<usize>,
}

impl PaceWire {
    fn new(
        vm: PaceViewModel,
        goal_local: Signal<i64>,
        end_local: Signal<Option<Date>>,
        active_local: Signal<bool>,
        has_pace: Signal<usize>,
    ) -> Self {
        Self { vm, goal_local, end_local, active_local, has_pace }
    }
}

impl std::fmt::Debug for PaceWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceWire").finish()
    }
}

impl Widget for PaceWire {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);

        // Switcher index mirrors "does a Pace exist yet".
        {
            let s = self.has_pace.clone();
            ctx.effect(&self.vm.pace_id(), move |id| {
                let v = id.is_some() as usize;
                if s.get() != v {
                    s.set(v);
                }
            });
        }
        // Goal: VM → local (external edits, e.g. the Inspector / Goals settings).
        // Local → VM is the SpinBox's `on_value_changed`, so no local effect here.
        {
            let l = self.goal_local.clone();
            ctx.effect(&self.vm.goal_words(), move |g| {
                let v = (*g).max(0);
                if l.get() != v {
                    l.set(v);
                }
            });
        }
        // Active: two-way.
        {
            let l = self.active_local.clone();
            ctx.effect(&self.vm.active(), move |a| {
                if l.get() != *a {
                    l.set(*a);
                }
            });
        }
        {
            let vm = self.vm.clone();
            ctx.effect(&self.active_local, move |a| vm.set_active(*a));
        }
        // Deadline: two-way (chrono ↔ jiff at the boundary).
        {
            let l = self.end_local.clone();
            ctx.effect(&self.vm.end(), move |e| {
                let jd = naive_to_jiff_opt(*e);
                if l.get() != jd {
                    l.set(jd);
                }
            });
        }
        {
            let vm = self.vm.clone();
            ctx.effect(&self.end_local, move |e| {
                if let Some(jd) = *e {
                    let end = jiff_to_naive(jd);
                    // Keep the existing start; a Pace with no start yet begins today.
                    let start = vm.start().get().unwrap_or_else(|| Utc::now().date_naive());
                    vm.set_dates(start, end);
                }
            });
        }
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

// ── WeekdayChips: the seven counted-day toggles ─────────────────────────────

/// A wrapping row of seven weekday chips over the Pace's `weekday_mask`. Rebuilds
/// on any mask change (so a chip's fill reflects the current bit); a click
/// toggles that day's bit through the view-model.
struct WeekdayChips {
    vm: PaceViewModel,
    root: Option<WidgetId>,
}

impl WeekdayChips {
    fn new(vm: PaceViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for WeekdayChips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeekdayChips").finish()
    }
}

impl Widget for WeekdayChips {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .weekday_mask()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let mask = self.vm.weekday_mask().get();

        let mut row = Wrap::new().spacing(6.0).line_spacing(6.0);
        for (bit, label) in weekdays() {
            let on = mask & bit != 0;
            let (bg, fg) = if on {
                (SurfaceRole::Accent, TextRole::OnAccent)
            } else {
                (SurfaceRole::Container, TextRole::Secondary)
            };
            let vm = self.vm.clone();
            let chip = ZStack::new()
                .child(
                    RectWidget::new()
                        .background(bg)
                        .corner_radius(CornerRadius::uniform(8.0)),
                )
                .child(Center::new().child(TextWidget::new(label).color(fg).single_line()))
                .on_tap(move |_e, _c| {
                    let m = vm.weekday_mask().get();
                    vm.set_weekday_mask(m ^ bit);
                });
            row = row.child(FixedSize::new().width(46.0).height(30.0).child(chip));
        }
        self.root = Some(ctx.add(row));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

// ── PaceCharts: progression (actual vs target) + words-per-day ──────────────

/// The two progress charts, rebuilt on the view-model's `version` (a new snapshot
/// or a schedule edit): a line chart of the recorded cumulative words against the
/// even-pace target, and a bar chart of words written per day. Until any progress
/// is recorded there is nothing to plot, so it shows a hint instead.
struct PaceCharts {
    vm: PaceViewModel,
    root: Option<WidgetId>,
}

impl PaceCharts {
    fn new(vm: PaceViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for PaceCharts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceCharts").finish()
    }
}

impl Widget for PaceCharts {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .version()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let actual = self.vm.actual_series();
        if actual.is_empty() {
            self.root = Some(ctx.add(
                TextWidget::new(tr!(pace_charts_empty())).color(TextRole::Secondary),
            ));
            return self.root.into_iter().collect();
        }

        // Progression: recorded cumulative vs the even-pace target.
        let mut actual_series = ChartSeries::new(tr!(pace_series_actual()).resolve_now());
        for (date, words) in &actual {
            actual_series.push(day_label(*date), *words as f32);
        }
        let mut line_series = vec![actual_series];
        // Only plot the target once a start/end/goal exist.
        if actual.iter().any(|(d, _)| self.vm.target_for(*d).is_some()) {
            let mut target = ChartSeries::new(tr!(pace_series_target()).resolve_now());
            for (date, _) in &actual {
                target.push(day_label(*date), self.vm.target_for(*date).unwrap_or(0) as f32);
            }
            line_series.push(target);
        }
        let line = LineChart::new(ChartModel::from_series_vec(line_series))
            .points(true)
            .grid(true)
            .legend(true);

        // Words written per day.
        let mut per_day = ChartSeries::new(tr!(pace_series_words_per_day()).resolve_now());
        for (date, words) in self.vm.words_per_day() {
            per_day.push(day_label(date), words as f32);
        }
        let bars = BarChart::new(ChartModel::from_series_vec(vec![per_day]))
            .grid(true)
            .legend(false);

        let col = VStack::new()
            .spacing(12.0)
            .child(
                TextWidget::new(tr!(pace_chart_progression()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(FixedSize::new().height(180.0).child(line))
            .child(
                TextWidget::new(tr!(pace_chart_words_per_day()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(FixedSize::new().height(140.0).child(bars));
        self.root = Some(ctx.add(col));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

// ── PaceStats: the reactive statistics readout ──────────────────────────────

/// The derived statistics. A composing widget bound to the view-model's `version`
/// at `Rebuild`, so the localized readout recomputes on any change (an edit, or a
/// freshly recorded snapshot). `today` is captured at build — it only matters
/// across a midnight roll-over, which a rebuild resolves.
struct PaceStats {
    vm: PaceViewModel,
    today: NaiveDate,
    root: Option<WidgetId>,
}

impl PaceStats {
    fn new(vm: PaceViewModel, today: NaiveDate) -> Self {
        Self { vm, today, root: None }
    }

    fn line(text: LocalizedString, dim: bool) -> impl Widget {
        TextWidget::new(text).color(if dim { TextRole::Secondary } else { TextRole::Primary })
    }
}

impl std::fmt::Debug for PaceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceStats").finish()
    }
}

impl Widget for PaceStats {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .version()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let vm = &self.vm;
        let today = self.today;

        // Progress toward the goal.
        let progress = match vm.percent_done() {
            Some(_) => Self::line(
                tr!(pace_stat_words(
                    current = vm.current_words(),
                    goal = vm.goal_words().get()
                )),
                false,
            ),
            None => Self::line(tr!(pace_stat_no_goal()), true),
        };
        let pct = vm
            .percent_done()
            .map(|p| Self::line(tr!(pace_stat_percent(pct = (p * 100.0).round() as i64)), true));

        // Streak.
        let streak = match vm.streak(today) {
            0 => Self::line(tr!(pace_stat_streak_none()), true),
            n => Self::line(tr!(pace_stat_streak(days = n as i64)), false),
        };

        // Deadline-derived: days left, needed rate, ahead/behind.
        let (days_left, rate, delta) = if vm.end().get().is_some() {
            let dl = vm.writing_days_left(today);
            let rate = match vm.words_per_writing_day(today) {
                Some(r) => Self::line(tr!(pace_stat_rate(words = r)), false),
                None => Self::line(tr!(pace_stat_no_goal()), true),
            };
            let delta = match vm.ahead_behind(today) {
                Some(d) if d > 0 => Self::line(tr!(pace_stat_ahead(words = d)), false),
                Some(d) if d < 0 => Self::line(tr!(pace_stat_behind(words = -d)), false),
                Some(_) => Self::line(tr!(pace_stat_on_track()), false),
                None => Self::line(tr!(pace_stat_no_goal()), true),
            };
            (
                Self::line(tr!(pace_stat_days_left(days = dl as i64)), false),
                rate,
                delta,
            )
        } else {
            (
                Self::line(tr!(pace_stat_no_deadline()), true),
                Self::line(tr!(pace_stat_no_deadline()), true),
                Self::line(tr!(pace_stat_no_deadline()), true),
            )
        };

        let mut col = VStack::new().spacing(4.0).child(progress);
        if let Some(pct) = pct {
            col = col.child(pct);
        }
        col = col.child(streak).child(days_left).child(rate).child(delta);

        self.root = Some(ctx.add(col));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
