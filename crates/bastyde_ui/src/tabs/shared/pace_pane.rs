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
    Button, Center, DateEdit, DateRange, DateRangeEdit, Expand, FixedSize, FormLayout, HStack,
    IconButton, MasonryLayout, Padding, Panel, RectWidget, ScrollArea, SpinBox, StepType, Switcher,
    TextInput, TextWidget, Toggle, VStack, Wrap, ZStack,
};
use bastyde_charts::{BarChart, LineChart};

use chrono::{Datelike, Duration, NaiveDate, Utc};
use jiff::civil::Date;

use crate::date_convert::{jiff_to_naive, naive_to_jiff, naive_to_jiff_opt};
use crate::tabs::ContentTab;
use crate::view_models::PaceViewModel;

use super::{centered, tab_backdrop, vspace};

/// (bit, ftl label) for the seven weekday chips. Mon = 1, … Sun = 64 - the mask
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
        // unreachable - fall back to the shell text rather than panic.
        return Box::new(
            Center::new().child(TextWidget::new(tr!(pace_placeholder())).color(TextRole::Secondary)),
        );
    };
    // The Pace pane is a stats dashboard, not a prose column, so it uses a wider
    // sensible max width than the reading-column width (which the form + charts +
    // stat-card masonry all read for their layout).
    let cw = Signal::new(880.0);

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

/// One dashboard section: a titled Panel wrapping its body.
fn panel_section(title: LocalizedString, body: impl Widget + 'static) -> impl Widget {
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

/// The planner, once a Pace exists: a responsive dashboard of section Panels.
fn planner(
    vm: &PaceViewModel,
    goal_local: &Signal<i64>,
    end_local: &Signal<Option<Date>>,
    active_local: &Signal<bool>,
) -> impl Widget {
    PaceDashboard::new(
        vm.clone(),
        goal_local.clone(),
        end_local.clone(),
        active_local.clone(),
    )
}

/// The dashboard: the schedule, statistics, charts, holidays and milestones as
/// section Panels laid out in a responsive [`MasonryLayout`]. A masonry *fills*
/// its offered width (dividing it into columns), so the whole pane uses the
/// available width - one column when narrow, two or three when wide - rather than
/// hugging the form's natural width the way a `VStack` would. Rebuilt on the
/// column count, derived from the measured width.
struct PaceDashboard {
    vm: PaceViewModel,
    goal_local: Signal<i64>,
    end_local: Signal<Option<Date>>,
    active_local: Signal<bool>,
    cols: Signal<usize>,
    root: Option<WidgetId>,
}

impl PaceDashboard {
    fn new(
        vm: PaceViewModel,
        goal_local: Signal<i64>,
        end_local: Signal<Option<Date>>,
        active_local: Signal<bool>,
    ) -> Self {
        Self {
            vm,
            goal_local,
            end_local,
            active_local,
            cols: Signal::new(1),
            root: None,
        }
    }

    /// Section panels want ~340px to be comfortable (a chart, a form).
    fn columns_for(width: f32) -> usize {
        ((width / 340.0).floor() as usize).clamp(1, 3)
    }
}

impl std::fmt::Debug for PaceDashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceDashboard").finish()
    }
}

impl Widget for PaceDashboard {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.cols
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let today = Utc::now().date_naive();
        let vm = &self.vm;

        // Schedule section: goal + deadline + weekdays + active.
        let goal_field = FixedSize::new().width(160.0).child(
            SpinBox::new(self.goal_local.clone(), 0_i64, 100_000_000)
                .step_type(StepType::Adaptive)
                .on_value_changed({
                    let vm = vm.clone();
                    move |v, _c| vm.set_goal_words(v)
                }),
        );
        let deadline_field = FixedSize::new().width(200.0).child(
            DateEdit::new(self.end_local.clone())
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
            .child(Toggle::new(self.active_local.clone()).label(tr!(pace_active())));

        let m = MasonryLayout::new(self.cols.get().max(1))
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
            ));

        self.root = Some(ctx.add(m));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        if let Some(w) = proposal.width {
            let want = Self::columns_for(w);
            if self.cols.get() != want {
                self.cols.set(want);
            }
        }
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// A short, locale-neutral day label for a chart category (`7/16`).
fn day_label(date: NaiveDate) -> String {
    format!("{}/{}", date.month(), date.day())
}

/// A form-row label - small, secondary, single line (matches the settings panes).
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
/// - the one place in the pane's tree that has a `BuildContext`.
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

        // Words written per day, each bar tinted red when that day fell below the
        // even-pace daily target (and accent otherwise).
        let rate = self.vm.target_daily_rate();
        let points: Vec<ChartDatum<String>> = self
            .vm
            .words_per_day()
            .into_iter()
            .map(|(date, words)| {
                let datum = ChartDatum::new(day_label(date), words as f32);
                match rate {
                    Some(r) if words < r => datum.with_color(SurfaceRole::StatusError),
                    _ => datum.with_color(SurfaceRole::Accent),
                }
            })
            .collect();
        let per_day = ChartSeries::new(tr!(pace_series_words_per_day()).resolve_now()).data(points);
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

/// A holiday's date span, compact (`7/16` or `7/16–7/20`).
fn holiday_span(start: NaiveDate, end: NaiveDate) -> String {
    if start == end {
        day_label(start)
    } else {
        format!("{}\u{2013}{}", day_label(start), day_label(end))
    }
}

// ── HolidayEditor: list of paused spans + an add row ────────────────────────

/// The Book's holidays - spans excluded from the schedule. A version-bound list
/// (each row removable) plus an add row (label + start + optional end). Mirrors
/// the settings `DestinationsEditor` pattern; the add-row inputs are widget
/// fields, so they persist across the list's rebuilds.
struct HolidayEditor {
    vm: PaceViewModel,
    label: Signal<String>,
    range: Signal<Option<DateRange>>,
    root: Option<WidgetId>,
}

impl HolidayEditor {
    fn new(vm: PaceViewModel) -> Self {
        Self {
            vm,
            label: Signal::new(String::new()),
            range: Signal::new(None),
            root: None,
        }
    }
}

impl std::fmt::Debug for HolidayEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HolidayEditor").finish()
    }
}

impl Widget for HolidayEditor {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .version()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let holidays = self.vm.holidays().get();
        let mut col = VStack::new().spacing(6.0);
        if holidays.is_empty() {
            col = col.child(
                TextWidget::new(tr!(pace_holidays_none()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
        }
        for h in &holidays {
            let vm = self.vm.clone();
            let id = h.id;
            let row = HStack::new()
                .spacing(8.0)
                .child(
                    Expand::horizontal()
                        .child(TextWidget::new(lit!(h.label.clone())).single_line()),
                )
                .child(
                    TextWidget::new(lit!(holiday_span(h.start, h.end)))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .single_line(),
                )
                .child(
                    IconButton::clear()
                        .tooltip(tr!(pace_remove()))
                        .on_activate_fn(move |_c| vm.remove_holiday(id)),
                );
            col = col.child(row);
        }

        // Add row: a name and a single date-range control (start plus optional
        // end, one popover) rather than two separate date pickers.
        let add = {
            let vm = self.vm.clone();
            let label = self.label.clone();
            let range = self.range.clone();
            Button::new(tr!(pace_add_holiday())).on_activate_fn(move |_c| {
                let text = label.get();
                if text.trim().is_empty() {
                    return;
                }
                let Some(r) = range.get() else {
                    return;
                };
                // A DateRange carries both ends; a single-day holiday is start == end.
                let end = if r.end == r.start { None } else { Some(jiff_to_naive(r.end)) };
                vm.add_holiday(text, jiff_to_naive(r.start), end);
                label.set(String::new());
                range.set(None);
            })
        };
        let add_row = HStack::new()
            .spacing(8.0)
            .child(
                Expand::horizontal()
                    .child(TextInput::new(self.label.clone()).placeholder(tr!(pace_holiday_label()))),
            )
            .child(FixedSize::new().width(220.0).child(DateRangeEdit::new(self.range.clone())))
            .child(add);
        col = col.child(vspace(4.0)).child(add_row);

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

// ── MilestoneList: the Book's milestones (set in the Inspector) ─────────────

/// The Book's milestones, shown along the pace. Set on a Part/Chapter in the
/// Inspector (M5); here they are listed with a remove affordance. Version-bound.
struct MilestoneList {
    vm: PaceViewModel,
    root: Option<WidgetId>,
}

impl MilestoneList {
    fn new(vm: PaceViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for MilestoneList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MilestoneList").finish()
    }
}

impl Widget for MilestoneList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .version()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let milestones = self.vm.milestones().get();
        let mut col = VStack::new().spacing(6.0);
        if milestones.is_empty() {
            col = col.child(
                TextWidget::new(tr!(pace_milestones_none()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
        }
        for m in &milestones {
            let vm = self.vm.clone();
            let id = m.id;
            let row = HStack::new()
                .spacing(8.0)
                .child(
                    Expand::horizontal()
                        .child(TextWidget::new(lit!(m.label.clone())).single_line()),
                )
                .child(
                    TextWidget::new(lit!(day_label(m.target_date)))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .single_line(),
                )
                .child(
                    IconButton::clear()
                        .tooltip(tr!(pace_remove()))
                        .on_activate_fn(move |_c| vm.remove_milestone(id)),
                );
            col = col.child(row);
        }
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

// ── StatCards: the statistics as a responsive masonry of cards ──────────────

/// A thousands-separated integer (`12,345`).
fn commafy(n: i64) -> String {
    let neg = n < 0;
    let digits = n.unsigned_abs().to_string();
    let bytes = digits.as_bytes();
    let mut out = String::new();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(*b as char);
    }
    if neg { format!("-{out}") } else { out }
}

/// One stat card: a big number over a small caption, inside a Panel.
fn stat_card(
    big: &TextStyle,
    number: String,
    caption: LocalizedString,
    number_color: TextRole,
) -> impl Widget {
    Panel::new().child(
        Padding::uniform(14.0).child(
            VStack::new()
                .spacing(2.0)
                .child(
                    TextWidget::new(lit!(number))
                        .style(big.clone())
                        .color(number_color)
                        .single_line(),
                )
                .child(
                    TextWidget::new(caption)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
        ),
    )
}

/// The derived statistics, laid out as a responsive [`MasonryLayout`] of cards.
/// Rebuilt on the view-model's `version` (a data change) and on the column count
/// (a width change). bastyde's masonry takes a fixed column count, so the count
/// is derived from the measured width in [`layout_response`](Self::layout_response),
/// which packs the varying-height cards efficiently.
struct StatCards {
    vm: PaceViewModel,
    today: NaiveDate,
    cols: Signal<usize>,
    root: Option<WidgetId>,
}

impl StatCards {
    fn new(vm: PaceViewModel, today: NaiveDate) -> Self {
        Self { vm, today, cols: Signal::new(2), root: None }
    }

    /// How many card columns a given width affords (each card stays readable down
    /// to ~165px, so a narrow split pane still shows two).
    fn columns_for(width: f32) -> usize {
        ((width / 175.0).floor() as usize).clamp(1, 4)
    }
}

impl std::fmt::Debug for StatCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatCards").finish()
    }
}

impl Widget for StatCards {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.version().bind_to(sid, reg, BindingLevel::Rebuild);
        self.cols.bind_to(sid, reg, BindingLevel::Rebuild);

        // A big, bold number style derived from the theme's body style.
        let big = TextStyle {
            size: 24.0,
            weight: FontWeight::BOLD,
            ..ctx.theme().typography.body_bold.clone()
        };
        let vm = &self.vm;
        let today = self.today;

        let mut m = MasonryLayout::new(self.cols.get().max(1))
            .column_spacing(10.0)
            .item_spacing(10.0);

        m = m.child(stat_card(
            &big,
            commafy(vm.current_words()),
            tr!(pace_card_written()),
            TextRole::Accent,
        ));
        if let Some(p) = vm.percent_done() {
            m = m.child(stat_card(
                &big,
                format!("{}%", (p * 100.0).round() as i64),
                tr!(pace_card_of_goal()),
                TextRole::Accent,
            ));
        }
        if let Some(r) = vm.words_per_writing_day(today) {
            m = m.child(stat_card(&big, commafy(r), tr!(pace_card_rate()), TextRole::Accent));
        }
        if vm.end().get().is_some() {
            m = m.child(stat_card(
                &big,
                vm.writing_days_left(today).to_string(),
                tr!(pace_card_days_left()),
                TextRole::Accent,
            ));
        }
        m = m.child(stat_card(
            &big,
            vm.streak(today).to_string(),
            tr!(pace_card_streak()),
            TextRole::Accent,
        ));
        if let Some(d) = vm.ahead_behind(today) {
            let (num, caption, color) = if d >= 0 {
                (commafy(d), tr!(pace_card_ahead()), TextRole::Success)
            } else {
                (commafy(-d), tr!(pace_card_behind()), TextRole::Warning)
            };
            m = m.child(stat_card(&big, num, caption, color));
        }

        self.root = Some(ctx.add(m));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Derive the column count from the available width; a change re-runs
        // build (bound above) on the next frame. The masonry fills whatever
        // width it is given regardless of the count, so this cannot oscillate.
        if let Some(w) = proposal.width {
            let want = Self::columns_for(w);
            if self.cols.get() != want {
                self.cols.set(want);
            }
        }
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
