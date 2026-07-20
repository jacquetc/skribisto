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

/// The Book's milestones, shown along the pace. Set on a Part/Chapter in the
/// Inspector (M5); here they are listed with a remove affordance. Version-bound.
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
pub(super) fn commafy(n: i64) -> String {
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
