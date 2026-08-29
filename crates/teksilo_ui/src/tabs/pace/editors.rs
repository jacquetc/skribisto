// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The holiday and milestone editors.

#[allow(unused_imports)]
use super::*;

// ── The add rows' field widths ──────────────────────────────────────────────
//
// **Both add rows are a `Wrap`, not an `HStack`, and every field carries a width.**
// They were an `HStack` whose name field was an `Expand::horizontal` and whose date and
// number fields were fixed — which is fine at the pane's full width and broken at a
// dashboard column's. `ColumnFlow` reflows these sections into columns as narrow as
// `min_column_width` (300 dp), the fixed fields alone already add up to more than that,
// and an `Expand` in an over-constrained row surrenders every pixel: the milestone name
// field measured **21 dp wide** and rendered its placeholder as "…". A `Wrap` breaks the
// row onto a second line instead, so a narrow column costs height — which the column has —
// rather than a field nobody can type in.
//
// The name field is fixed too, for the same reason: a flexible child in a `Wrap` would be
// asked for its intrinsic width and a `TextInput`'s is its content's, so an empty one
// would collapse to the placeholder and jump about as the writer typed.

/// The "what is this called" field, in both rows.
const NAME_FIELD_WIDTH: f32 = 180.0;
/// A holiday's start-plus-optional-end control — one popover, so it is the widest field.
const RANGE_FIELD_WIDTH: f32 = 220.0;
/// A milestone's single date.
const DATE_FIELD_WIDTH: f32 = 150.0;
/// A milestone's word target, with its stepper.
const TARGET_FIELD_WIDTH: f32 = 130.0;

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
            Wrap::new()
                .spacing(8.0)
                .line_spacing(8.0)
                .child(FixedSize::new().width(NAME_FIELD_WIDTH).child(
                    TextInput::new(self.label.clone()).placeholder(tr!(pace_holiday_label())),
                ))
                .child(
                    FixedSize::new()
                        .width(RANGE_FIELD_WIDTH)
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
            Wrap::new()
                .spacing(8.0)
                .line_spacing(8.0)
                .child(FixedSize::new().width(NAME_FIELD_WIDTH).child(
                    TextInput::new(self.label.clone()).placeholder(tr!(milestone_add_label())),
                ))
                .child(FixedSize::new().width(DATE_FIELD_WIDTH).child(
                    DateEdit::new(self.date.clone()).placeholder(tr!(inspector_milestone_none())),
                ))
                .child(
                    FixedSize::new().width(TARGET_FIELD_WIDTH).child(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use std::rc::Rc as StdRc;
    use teksilo::core::widget_tree::WidgetTree;

    /// The narrowest a dashboard section ever gets: `ColumnFlow::min_column_width`
    /// in `tabs::pace::planner`, less the card's own padding on both sides.
    const NARROW: f32 = 300.0 - 2.0 * crate::pace::CARD_PADDING;

    /// Below this, a field has been crushed rather than merely made small. Sits well
    /// clear on both sides: the narrowest field the rows *declare* is
    /// [`TARGET_FIELD_WIDTH`] (130 dp), and the collapse this guards against measured
    /// **21 dp** in the shipped `HStack`.
    const CRUSH_FLOOR: f32 = 100.0;

    /// A Pace view-model over an empty store. Every list it exposes is empty, which
    /// is exactly the state these rows are laid out in: the add row is what a
    /// section with nothing in it consists of.
    fn empty_book_vm() -> PaceViewModel {
        PaceViewModel::new(
            StdRc::new(AppContext::new()),
            AppIds::new(),
            1,
            &BinderItemRole::Folder,
            &BinderItemSubRole::Book,
        )
        .expect("a Folder/Book container has a Pace view-model")
    }

    /// Lay `w` out in a `NARROW`-wide box with a real text metric, and report the
    /// narrowest `TextInput` in it plus the box's own laid-out size.
    ///
    /// A real backend for the usual reason (`crate::text_overflow`): the no-backend
    /// fallback reports rigid sizes, so an over-constrained row would measure as if
    /// it had fitted.
    fn narrowest_input(w: impl Widget + 'static) -> (f32, teksilo::canvas::Size) {
        let mut tree = WidgetTree::new().with_text_backend(StdRc::new(std::cell::RefCell::new(
            teksilo::canvas::MockTextBackend::new(),
        )));
        let root = tree.add(w);
        tree.layout(SizeProposal::exact(NARROW, 600.0));

        fn walk(tree: &WidgetTree, id: WidgetId, out: &mut f32) {
            if tree
                .widget_type_name(id)
                .is_some_and(|n| n.ends_with("::TextInput"))
            {
                *out = out.min(tree.bounds(id).size().width);
            }
            for c in tree.children(id) {
                walk(tree, c, out);
            }
        }
        let mut narrowest = f32::MAX;
        walk(&tree, root, &mut narrowest);
        (narrowest, tree.bounds(root).size())
    }

    /// **The collapse bug.** Both add rows were an `HStack` whose name field was an
    /// `Expand::horizontal` next to fixed-width date and number fields that already
    /// added up to more than a dashboard column. An `Expand` in an over-constrained
    /// row surrenders everything it has: the milestone name field measured 21 dp and
    /// drew its placeholder as "…".
    ///
    /// A `Wrap` spends height instead, which a column has. The guard is therefore
    /// two-sided — the field keeps its width, *and* the row does not simply overflow
    /// the column to get it.
    #[test]
    fn a_narrow_column_wraps_the_milestone_add_row_rather_than_crushing_its_name_field() {
        let (narrowest, size) = narrowest_input(AddMilestoneRow::new(empty_book_vm()));
        assert!(
            narrowest >= CRUSH_FLOOR,
            "no field may be crushed: narrowest input was {narrowest} dp in a \
             {NARROW} dp column"
        );
        assert!(
            size.width <= NARROW + 0.5,
            "…and the row must stay inside the column, not overflow it (took {} dp)",
            size.width
        );
    }

    /// The holiday add row is the same shape and carries the same guard: its
    /// date-range control is wider still, so it collapses sooner.
    #[test]
    fn a_narrow_column_wraps_the_holiday_add_row_too() {
        let (narrowest, size) = narrowest_input(HolidayEditor::new(empty_book_vm()));
        assert!(
            narrowest >= CRUSH_FLOOR,
            "no field may be crushed: narrowest input was {narrowest} dp in a \
             {NARROW} dp column"
        );
        assert!(
            size.width <= NARROW + 0.5,
            "…and the row must stay inside the column (took {} dp)",
            size.width
        );
    }
}
