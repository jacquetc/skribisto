// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The holiday and milestone editors.

#[allow(unused_imports)]
use super::*;

// ── HolidayEditor: list of paused spans + an add row ────────────────────────

/// The Book's holidays - spans excluded from the schedule. A version-bound list
/// (each row removable) plus an add row (label + start + optional end). Mirrors
/// the settings `DestinationsEditor` pattern; the add-row inputs are widget
/// fields, so they persist across the list's rebuilds.
pub(super) struct HolidayEditor {
    vm: PaceViewModel,
    label: Signal<String>,
    range: Signal<Option<DateRange>>,
    root: Option<WidgetId>,
}

impl HolidayEditor {
    pub(super) fn new(vm: PaceViewModel) -> Self {
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
                let end = if r.end == r.start {
                    None
                } else {
                    Some(jiff_to_naive(r.end))
                };
                vm.add_holiday(text, jiff_to_naive(r.start), end);
                label.set(String::new());
                range.set(None);
            })
        };
        let add_row =
            HStack::new()
                .spacing(8.0)
                .child(Expand::horizontal().child(
                    TextInput::new(self.label.clone()).placeholder(tr!(pace_holiday_label())),
                ))
                .child(
                    FixedSize::new()
                        .width(220.0)
                        .child(DateRangeEdit::new(self.range.clone())),
                )
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

/// The Book's milestones, of both kinds, with their progress and a remove affordance.
///
/// An **item** milestone is created on a Part or Chapter in the Inspector and shows that
/// item's own target, resolved live rather than copied — so editing the target updates the
/// milestone. A **book** milestone is added here, with a number the writer types, and is
/// measured against the Book's own cumulative count.
///
/// An item milestone whose target has been deleted says so rather than showing a number
/// nobody set: that state exists precisely because the reference is weak, and telling it
/// apart from a book milestone is what the stored kind buys.
///
/// Version-bound.
pub(super) struct MilestoneList {
    vm: PaceViewModel,
    root: Option<WidgetId>,
}

impl MilestoneList {
    pub(super) fn new(vm: PaceViewModel) -> Self {
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
        // The Book's own count now — what a book-cumulative waypoint is measured against.
        let current = self.vm.current_words();
        for m in &milestones {
            let vm = self.vm.clone();
            let id = m.id;
            let row = HStack::new()
                .spacing(8.0)
                .child(
                    Expand::horizontal()
                        .child(TextWidget::new(lit!(m.label.clone())).single_line()),
                )
                .child(milestone_target(m, current))
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
        col = col.child(AddMilestoneRow::new(self.vm.clone()));
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

/// The number beside a milestone, and what it is measured against.
///
/// A book-cumulative waypoint is compared with the Book's count *now*, so the row says
/// whether it has been passed. An item milestone shows the item's own target: comparing it
/// with the Book's total would be plainly wrong, and the item's own progress already has a
/// home on its own page and in the Overview.
fn milestone_target(m: &MilestoneRow, current: i64) -> impl Widget + use<> {
    if m.target_gone {
        return TextWidget::new(tr!(milestone_target_gone()))
            .style(TextStyleRole::Small)
            .color(TextRole::Error)
            .single_line();
    }
    let Some(target) = m.target_words else {
        // An item milestone on an item that carries no target of its own: a date with
        // nothing to reach by it, which is worth saying rather than leaving blank.
        return TextWidget::new(tr!(milestone_no_target()))
            .style(TextStyleRole::Small)
            .color(TextRole::Disabled)
            .single_line();
    };
    let color = match m.kind {
        MilestoneKind::BookCumulative => {
            crate::goals::target_role(crate::goals::ratio(current, target).unwrap_or(0.0))
        }
        MilestoneKind::Item => TextRole::Secondary,
    };
    TextWidget::new(lit!(crate::goals::format_goal(target)))
        .style(TextStyleRole::Small)
        .color(color)
        .single_line()
}

/// "＋ Add a waypoint": label, date and word count, for a **book-cumulative** milestone.
///
/// The only place that kind can be created. An *item* milestone is born where the item is —
/// the Inspector — because it is a statement about that section, and asking for one here
/// would mean asking the writer to name a chapter in a text field.
struct AddMilestoneRow {
    vm: PaceViewModel,
    label: Signal<String>,
    date: Signal<Option<jiff::civil::Date>>,
    words: Signal<i64>,
    root: Option<WidgetId>,
}

impl AddMilestoneRow {
    fn new(vm: PaceViewModel) -> Self {
        Self {
            vm,
            label: Signal::new(String::new()),
            date: Signal::new(None),
            words: Signal::new(0),
            root: None,
        }
    }
}

impl std::fmt::Debug for AddMilestoneRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddMilestoneRow").finish()
    }
}

impl Widget for AddMilestoneRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        // The button's enabled state follows the two fields that must be filled.
        self.label.bind_to(sid, reg, BindingLevel::Rebuild);
        self.date.bind_to(sid, reg, BindingLevel::Rebuild);
        self.words.bind_to(sid, reg, BindingLevel::Rebuild);

        let ready = !self.label.get().trim().is_empty()
            && self.date.get().is_some()
            && self.words.get() > 0;
        let vm = self.vm.clone();
        let (label, date, words) = (self.label.clone(), self.date.clone(), self.words.clone());
        let row =
            HStack::new()
                .spacing(8.0)
                .child(Expand::horizontal().child(
                    TextInput::new(self.label.clone()).placeholder(tr!(milestone_add_label())),
                ))
                .child(FixedSize::new().width(150.0).child(
                    DateEdit::new(self.date.clone()).placeholder(tr!(inspector_milestone_none())),
                ))
                .child(
                    FixedSize::new().width(130.0).child(
                        SpinBox::new(self.words.clone(), 0_i64, 100_000_000)
                            .single_step(1_000)
                            .special_value_text(tr!(milestone_no_target())),
                    ),
                )
                .child(
                    Button::new(tr!(milestone_add()))
                        .variant(ButtonVariant::Plain)
                        .enabled(ready)
                        .rich_tooltip(crate::tooltip_registry::GOAL_MILESTONE)
                        .on_activate_fn(move |_c| {
                            let Some(d) = date.get().map(crate::date_convert::jiff_to_naive) else {
                                return;
                            };
                            vm.add_milestone(label.get().trim().to_string(), d, Some(words.get()));
                            label.set(String::new());
                            date.set(None);
                            words.set(0);
                        }),
                );
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

// ── StatCards: the statistics as a responsive masonry of cards ──────────────

/// A thousands-separated integer (`12,345`).
pub(super) fn commafy(n: i64) -> String {
    let neg = n < 0;
    let digits = n.unsigned_abs().to_string();
    let bytes = digits.as_bytes();
    let mut out = String::new();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    if neg { format!("-{out}") } else { out }
}
