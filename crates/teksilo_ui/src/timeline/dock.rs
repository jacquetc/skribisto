// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Timeline** band: the whole project's past, and the only way back to a
//! row that no longer exists.
//!
//! Its own bottom activity rather than a second pane beside Search Preview. A
//! sole-pane dock *is* its activity, so it gets the whole band; sharing would
//! leave a slider, a sparkline and a change list in half of 180 dp. The two share
//! only the side's height, which the writer can drag and the workspace layout
//! persists.
//!
//! ## Shaped to the band, which is wide and short
//!
//! **Left, the timeline.** A `teksilo_charts::BarChart` over the recorded
//! moments, with a slider above it addressing the same bars — the chart for the
//! pointer, the slider for the keyboard, kept in step by `sync_selection`.
//!
//! The bars were once a hand-rolled `HStack` of rectangles, which is the thing
//! this file most wants remembered: it spent 4 dp of spacing per bar *before*
//! any bar was given width, so a project with three hundred backups had 1,196 dp
//! of gaps in a 450 dp band and every bar laid out to zero. Two years of history
//! drew an empty strip. A chart is not decoration here; the framework already
//! owned this problem.
//!
//! **Too many versions to draw becomes too many to draw one by one.** Above
//! [`crate::timeline::timeline_axis::MAX_BARS`] the axis groups moments into
//! calendar periods and a bar becomes a month, a week or a day — see that module
//! for why bucketing rather than the scroll-per-datum answer Pace and Analysis
//! use. A period is opened to reach the versions inside it, and the date filter
//! reaches any period directly.
//!
//! **Right, what changed.** A flat, wide, scannable list of rows compared against
//! now, using the vocabulary `restic diff` settled on: added, removed, changed,
//! moved. A tree is tall-and-narrow and belongs on the leading rail; "what changed
//! here" is short and wide and belongs in a band.
//!
//! Opening a row shows its prose at that moment, read-only, in a modal with room
//! to read and copy out of. **A deleted row is reachable here and nowhere else** —
//! the Versions dock hangs off a live `BinderItem`, and a deleted scene has none.
//!
//! ## What this deliberately does not do
//!
//! Drive the *Outline* dock into a historical mode. It is feasible and it was
//! considered; it makes the app's most-used surface modal, and every historical
//! row then has to be inert against rename, drag, indent and trash or it targets
//! live ids. That is the same trap as a warning banner that does not actually
//! guard the content beneath it, on the worst possible surface.

use std::cell::{Cell, RefCell};

use teksilo::core::BindingLevel;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::data::{ChartDatum, ChartModel, ChartSelection, SelectionMode, SeriesId};
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    Button, ButtonVariant, DateRangeEdit, DockOpenLocation, DockSide, DockWidget, DockWidgetId,
    Expand, FixedSize, FocusScope, HStack, IconButton, IconButtonSize, ListView, Padding, Panel,
    ScrollBarMode, Slider, Spacer, StandardListItem, TextWidget, TraversalScopePolicy, VStack,
};
use teksilo_charts::{AxisConfig, BarChart};

use skrib_format::retention::BucketUnit;
use skrib_format::versions::{BackupVersions, LogVersions, SourceKind, VersionRef, VersionSource};

use crate::timeline::{Axis, ChangeKind, RowChange, TimelineViewModel};
use crate::versions::ProjectHandle;

/// How far back the one preset reaches.
///
/// Named where both docks can see it, so the button and its label cannot drift
/// apart — a "Last 30 days" button that set 31 would be a small lie told often.
pub const RECENT_DAYS: u32 = 30;

/// Package the timeline as a bottom `DockWidget`.
pub fn timeline_dock(vm: TimelineViewModel, dock_id: DockWidgetId) -> DockWidget {
    DockWidget::new(dock_id, tr!(timeline_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(TimelinePanel::new(vm.clone()))
    })
    .icon(crate::icons::activity::timeline_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Bottom).new_tab())
}

struct TimelinePanel {
    vm: TimelineViewModel,
    /// The chart's data, **owned by the panel** rather than rebuilt.
    ///
    /// `ChartModel` is a share-by-clone handle, and its `SeriesId` is an arena
    /// key: a fresh model on every build would mint a fresh id, and the selection
    /// — which is keyed on `(series, index)` — would stop matching the bars it
    /// was made from. Keeping one model and replacing its points is what lets a
    /// clicked bar stay clicked across a rebuild.
    model: ChartModel<String>,
    series: SeriesId,
    selection: ChartSelection,
    /// The labels currently in `model`, so an unchanged axis is not re-pushed.
    drawn: RefCell<Vec<String>>,
    /// The bar index this panel last wrote into `selection`, which is how a
    /// writer's click is told apart from our own mirroring of the slider.
    pushed: Cell<Option<usize>>,
    root_child: Option<WidgetId>,
}

impl TimelinePanel {
    fn new(vm: TimelineViewModel) -> Self {
        let model: ChartModel<String> = ChartModel::new();
        // Anonymous: the legend is off, and a one-series chart has nothing to
        // distinguish itself from.
        let series = model.add_series("");
        let selection = ChartSelection::attached(SelectionMode::Single, &model);
        Self {
            vm,
            model,
            series,
            selection,
            drawn: RefCell::new(Vec::new()),
            pushed: Cell::new(None),
            root_child: None,
        }
    }

    /// Push the axis into the chart's model, if it is not already there.
    fn sync_chart(&self, axis: &Axis) {
        let labels: Vec<String> = axis.bars.iter().map(|b| b.label.clone()).collect();
        if *self.drawn.borrow() == labels {
            return;
        }
        self.model.replace_series_data(
            self.series,
            axis.bars
                .iter()
                .map(|b| ChartDatum::new(b.label.clone(), b.bytes as f32))
                .collect(),
        );
        *self.drawn.borrow_mut() = labels;
        // The bars changed under the cursor; whatever was selected referred to
        // the old ones.
        self.selection.clear();
        self.pushed.set(None);
    }

    /// Keep the chart's selection and the slider's position saying the same thing.
    ///
    /// One direction per build, decided by which of them moved: a bar the writer
    /// clicked is one this panel did not write, so it wins and drives the slider;
    /// otherwise the slider is the source and the selection follows it. Without
    /// the `pushed` record the two would each keep re-asserting themselves and
    /// the band would flicker between them.
    fn sync_selection(&self, axis: &Axis) {
        if axis.is_empty() {
            return;
        }
        let picked = self
            .selection
            .selected_points()
            .first()
            .map(|(_, i)| *i)
            .filter(|i| *i < axis.bars.len());
        match picked {
            Some(i) if self.pushed.get() != Some(i) => {
                self.pushed.set(Some(i));
                self.vm.position().set(i as f32);
            }
            _ => {
                let i = self.vm.index();
                if picked != Some(i) {
                    self.selection.select_point(self.series, i);
                    self.pushed.set(Some(i));
                }
            }
        }
    }
}

impl std::fmt::Debug for TimelinePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimelinePanel").finish()
    }
}

impl Widget for TimelinePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.project().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.moments().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.position().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.changes().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.loading().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.error().bind_to(sid, reg, BindingLevel::Rebuild);

        self.vm.window().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.range().bind_to(sid, reg, BindingLevel::Rebuild);
        // A clicked bar has to reach the slider and the change list, and the only
        // way it announces itself is this signal.
        self.selection
            .selection_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        self.vm
            .set_async_runtime(ctx.app_state::<AsyncRuntimeHandle>().cloned());
        // All no-ops when nothing they depend on moved, which is what makes them
        // safe to call from a function their own writes rebuild.
        self.vm.scan();
        self.vm.sync_window();

        let axis = self.vm.axis();
        self.sync_chart(&axis);
        self.sync_selection(&axis);
        // After the selection, which can move the slider, which decides which
        // moment the change list is of.
        self.vm.sync_changes();

        let id = ctx.add(crate::tabs::Boxed::new(self.body(&axis)));
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

impl TimelinePanel {
    fn body(&self, axis: &Axis) -> Box<dyn Widget> {
        let vm = &self.vm;
        if !vm.error().get().is_empty() {
            return Box::new(note(tr!(versions_error())));
        }
        if vm.loading().get() {
            return Box::new(note(tr!(timeline_loading())));
        }
        if vm.moments().get().is_empty() {
            return Box::new(note(tr!(timeline_empty())));
        }
        if axis.is_empty() {
            // A filter narrower than the history. Not the same sentence as "no
            // history", and offering the way back out is the point.
            return Box::new(
                VStack::new()
                    .spacing(0.0)
                    .child(note(tr!(timeline_range_empty())))
                    .child(
                        Padding::symmetric(4.0, 10.0)
                            .child(crate::tabs::Boxed::new(self.window_control(axis))),
                    ),
            );
        }
        Box::new(
            HStack::new()
                .spacing(0.0)
                // Two to one, not half and half. The two halves do not want width
                // equally: the axis holds a chart, a slider and a date-range
                // field with two controls beside it, and every one of those has a
                // floor; the change list holds row titles, which ellipsise
                // gracefully and are read one at a time. Split evenly, a narrow
                // window starved the side that could not give.
                .child(
                    Expand::horizontal()
                        .flex(2.0)
                        .child(crate::tabs::Boxed::new(Box::new(self.axis(axis)))),
                )
                .child(teksilo::widgets::Divider::vertical())
                .child(
                    Expand::horizontal()
                        .flex(1.0)
                        .child(crate::tabs::Boxed::new(self.change_list())),
                ),
        )
    }

    /// Filter, slider, chart and dates — the left half.
    fn axis(&self, axis: &Axis) -> impl Widget + use<> {
        let vm = self.vm.clone();
        let last = axis.bars.len().saturating_sub(1);

        // Enough to move: a slider over one bar has nowhere to go, and a
        // disabled control says that better than a live one that does nothing.
        let movable = last > 0;
        let slider = Slider::new(vm.position(), 0.0, last.max(1) as f32)
            .step(1.0)
            .enabled(movable)
            // `label` is the accessible name and never reaches the screen; the
            // tooltip is the same sentence for someone who can see the control
            // but cannot tell what it picks.
            .label(tr!(timeline_slider_label()))
            .tooltip(tr!(timeline_slider_label()));

        let coverage = match vm.coverage() {
            Some((n, oldest)) => tr!(timeline_coverage(
                count = n as i64,
                oldest = oldest.format("%Y-%m-%d").to_string()
            )),
            None => tr!(timeline_empty()),
        };
        // What the bars *are* depends on how much history there is, so the
        // sentence explaining them has to say which. A caption reading "each bar
        // is a recorded version" over a row of months would be a lie.
        let caption = match axis.unit {
            None => tr!(timeline_bars_caption()),
            Some(unit) => tr!(timeline_bars_caption_periods(period = unit_word(unit))),
        };
        // Every row here has to *yield* width, not claim it. The band is half a
        // window wide and its neighbour is a list of row titles: a header that
        // simply laid its children out at their natural sizes overflowed the
        // half and drew the filter and the button on top of the change list.
        // The band is 180 dp by default and this half has six things to say, so
        // the order is: everything fixed gets one row and one line, and the chart
        // takes what is left. Sharing a row was tried and cost more than it
        // saved — the coverage sentence ellipsised down to "300 versions
        // recorded, goin…", and the caption wrapped to three lines, which
        // between them starved the chart to 31 dp of axis labels and no bars.
        teksu!(
            Padding::symmetric(4.0, 10.0) {
                VStack {
                    spacing: 2.0
                    TextWidget::new(coverage) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                        overflow: TextOverflow::Ellipsis(EllipsisMode::Trailing)
                    }
                    child: slider
                    Expand::vertical {
                        child: self.chart()
                    }
                    TextWidget::new(caption) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                        overflow: TextOverflow::Wrap
                    }
                    HStack {
                        spacing: 6.0
                        child: self.range_filter()
                        child: self.recent_button()
                        child: crate::tabs::Boxed::new(self.window_control(axis))
                        Spacer
                    }
                }
            }
        )
    }

    /// The bars, as a real chart.
    ///
    /// `teksilo_charts::BarChart` rather than a row of rectangles, which is what
    /// this was: a hand-rolled `HStack` spent 4 dp of spacing per bar before any
    /// of them got width, so at three hundred versions the gaps alone exceeded
    /// the band and every bar laid out to zero. A writer with two years of
    /// backups saw an empty strip.
    ///
    /// The chart is handed a *bucketed* axis, so it is never asked to draw more
    /// than [`crate::timeline::timeline_axis::MAX_BARS`]; the scroll-and-real-pitch answer the Pace and Analysis
    /// tabs use (`tabs::shared::charts::wide_chart`) is right for a chart you
    /// read and wrong for one you aim at, because 300 bars at that pitch is 8,400
    /// dp of scrolling to reach a date.
    fn chart(&self) -> impl Widget + use<> {
        // The model and its selection are the panel's, filled by `sync_chart` —
        // see the field docs for why they cannot be rebuilt here.
        BarChart::new(self.model.clone())
            .grid(false)
            .legend(false)
            // A band is short. The y axis would spend a third of the height on
            // numbers of bytes, which is not what anyone is reading this for —
            // the caption says what height means and the shape carries the rest.
            .axis_y(AxisConfig::new().show_labels(false).show_axis_line(false))
            .axis_x(AxisConfig::new().show_axis_line(false))
            .hover_tooltip(true)
            .selection(self.selection.clone())
    }

    /// The date filter — the way to reach March without walking through the
    /// months in between.
    fn range_filter(&self) -> impl Widget + use<> {
        teksu!(
            FixedSize::new() {
                width: 220.0
                DateRangeEdit::new(self.vm.range()) {
                    tooltip: tr!(timeline_range_filter())
                    label: tr!(timeline_range_filter())
                }
            }
        )
    }

    /// "Last 30 days" — the one range a writer asks for often enough to deserve
    /// a button, and one `DateRangeEdit` cannot express on its own.
    ///
    /// Icon-only, and that is a width decision as much as a visual one: this row
    /// also holds a date-range field, in a half-band that is 300 dp wide once
    /// the window is small. Two word-buttons beside it did not fit.
    fn recent_button(&self) -> impl Widget + use<> {
        let vm = self.vm.clone();
        teksu!(
            IconButton::new(crate::icons::versions::recent()) {
                size: IconButtonSize::Compact
                tooltip: tr!(versions_last_30_days())
                on_activate_fn: move |_| vm.set_last_days(RECENT_DAYS)
            }
        )
    }

    /// One contextual control: open the selected period, or come back out.
    ///
    /// A button rather than a double-click on the bar, because `BarChart` reports
    /// *selection*, not activation — and a keyboard user reaching the bars
    /// through the slider needs a real control to press either way.
    ///
    /// "Open this period" keeps its words: it is the one step in this surface
    /// that is not guessable, and it appears only while there is a period to
    /// open. "Show the whole history" is a reset, which has a glyph everyone
    /// already reads, so it becomes one.
    fn window_control(&self, axis: &Axis) -> Box<dyn Widget> {
        let vm = self.vm.clone();
        if axis.opens() {
            let open = move |_: &mut EventContext| vm.open_selected();
            return Box::new(teksu!(
                Button::new(tr!(timeline_open_period())) {
                    variant: ButtonVariant::Plain
                    text_style: TextStyleRole::Small
                    on_activate_fn: open
                }
            ));
        }
        if self.vm.window().get().is_some() {
            let back = move |_: &mut EventContext| vm.show_all();
            return Box::new(teksu!(
                IconButton::new(crate::icons::versions::reset()) {
                    size: IconButtonSize::Compact
                    tooltip: tr!(timeline_show_all())
                    on_activate_fn: back
                }
            ));
        }
        Box::new(Spacer::new())
    }

    /// What changed between the selected moment and now — the right half.
    fn change_list(&self) -> Box<dyn Widget> {
        let vm = self.vm.clone();
        let changes = vm.changes().get();
        if changes.is_empty() {
            // Not an error and not an empty project: the manuscript simply has
            // not moved since. Saying so beats an empty box.
            return Box::new(note(tr!(timeline_no_changes())));
        }
        // The moment the list is *against*. Empty only in the impossible case
        // where there are changes but no moment to have produced them, and an
        // empty date is better than refusing to draw the list.
        let when = vm
            .selected()
            .map(|m| m.at.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default();
        let rows: Vec<ChangeRow> = changes.iter().enumerate().map(ChangeRow::new).collect();
        let model = teksilo::data::ListModel::from_vec(rows);
        let open_vm = vm.clone();
        Box::new(
            VStack::new()
                .spacing(0.0)
                .child(
                    Padding::symmetric(4.0, 10.0).child(
                        // Named on both sides. "58 items have changed since"
                        // ended on a preposition with nothing after it, and a
                        // writer had no way to tell whether "since" meant the
                        // moment they had selected, the last backup, or the last
                        // time they opened the app.
                        TextWidget::new(tr!(timeline_changed_since(
                            count = changes.len() as i64,
                            date = when,
                        )))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .overflow(TextOverflow::Wrap),
                    ),
                )
                .child(
                    Expand::vertical().child(
                        ListView::new(model, move |_i, row: &ChangeRow, selected| {
                            Box::new(
                                StandardListItem::new(lit!(row.title.clone()))
                                    .subtitle(lit!(row.detail.clone()))
                                    .selected(selected)
                                    .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                                    .leading_slot(
                                        TextWidget::new(lit!(row.mark.to_string()))
                                            .color(row.tint)
                                            .style(TextStyleRole::SmallBold)
                                            // The mark restates the subtitle,
                                            // which `StandardListItem` already
                                            // exposes as the row's description.
                                            // Left visible it announced itself a
                                            // second time as a bare "−" or "→".
                                            .a11y_hidden(),
                                    ),
                            ) as Box<dyn Widget>
                        })
                        .item_height(40.0)
                        // On the view, not on the row: `on_activate` is what both
                        // Enter and a double-click reach, so the keyboard route to a
                        // deleted scene is the same one the mouse takes.
                        // The list is the view-model's change list verbatim — no
                        // filter, no sort — so the row position *is* the index the
                        // view-model keys on. `ChangeRow::index` carries it anyway,
                        // so the day a filter arrives this does not silently reopen
                        // the wrong row.
                        .on_activate(move |i, ctx| open_past(ctx, &open_vm, i))
                        .scroll_bar_style(ScrollBarMode::Overlay),
                    ),
                ),
        )
    }
}

/// One change-list row, flattened so the delegate does no work.
#[derive(Clone, PartialEq)]
struct ChangeRow {
    index: usize,
    title: String,
    detail: String,
    /// The `restic diff` vocabulary, which is as close to a convention as this
    /// has: `+` written since, `−` gone, `M` edited, `→` moved.
    mark: &'static str,
    tint: TextRole,
}

impl ChangeRow {
    /// Carries the row's index in the *view-model's* change list, which is what
    /// reopening it keys on — never the list view's own position, which a future
    /// filter or sort would renumber out from under it.
    fn new((index, change): (usize, &RowChange)) -> Self {
        let (mark, detail, tint) = match change.kind {
            ChangeKind::Added => ("+", tr!(timeline_kind_added()), TextRole::Success),
            ChangeKind::Removed => ("−", tr!(timeline_kind_removed()), TextRole::Error),
            ChangeKind::Changed => ("M", tr!(timeline_kind_changed()), TextRole::Accent),
            ChangeKind::Moved => ("→", tr!(timeline_kind_moved()), TextRole::Secondary),
        };
        Self {
            index,
            title: change.title.clone(),
            detail: detail.resolve_now(),
            mark,
            tint,
        }
    }
}

/// What one bucketed bar covers, in a word — "months", "weeks", "days", "hours".
fn unit_word(unit: BucketUnit) -> String {
    match unit {
        BucketUnit::Hour => tr!(timeline_unit_hour()),
        BucketUnit::Day => tr!(timeline_unit_day()),
        BucketUnit::Week => tr!(timeline_unit_week()),
        BucketUnit::Month => tr!(timeline_unit_month()),
    }
    .resolve_now()
}

use crate::shared::text::dock_note as note;

/// Show one row's prose as it was at the selected moment, read-only.
///
/// A modal rather than a pane in the band: the band is 180 dp of height, and the
/// point of opening a row is to *read* it — and, for a row that no longer exists,
/// to copy it back out by hand. `read_only` and not merely a disabled editor,
/// because the two are not the same promise: a read-only editor still selects and
/// copies, which is the whole reason to open this.
fn open_past(ctx: &mut EventContext, vm: &TimelineViewModel, index: usize) {
    let changes = vm.changes().get();
    let Some(change) = changes.get(index) else {
        return;
    };
    let title = change.title.clone();
    let Some((from, blob)) = change.source.clone() else {
        // Nothing to show — but *why* there is nothing is two different
        // sentences, and saying the wrong one sends a writer looking for a
        // history that never existed. An "added since" row genuinely post-dates
        // the moment; a folder or a chapter heading was there all along and
        // simply has no prose of its own.
        let why = if change.kind == ChangeKind::Added {
            tr!(timeline_not_yet_written())
        } else {
            tr!(timeline_no_text_of_its_own())
        };
        ctx.show_toast(Toast::info(why).auto_dismiss_after(std::time::Duration::from_secs(5)));
        return;
    };
    let handle = vm.project().get();
    let text = read_prose(&handle, &from, &blob);
    let when = from.taken_at.format("%Y-%m-%d %H:%M").to_string();
    let gone = change.is_gone();

    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(PastReader::new(
                title.clone(),
                when.clone(),
                text.clone(),
                gone,
            ))
        })
        .presentation(ModalPresentation::InTree)
        // No `.title()` and no `.size()`. Both are read only by the *native
        // window* branch of `present_modal`; an in-tree modal is a centred
        // overlay sized entirely by what it contains, and `present_in_tree_
        // modal_request` never looks at either field. Asking for 760×620 here
        // and believing it is what put a bare, dated header in the middle of the
        // window with no room left for the prose under it. The size now lives on
        // the widget, where it is the thing that actually decides, and the title
        // is drawn inside the panel, where it can be seen.
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

/// The reader's own size, since the modal request's is not read for an in-tree
/// presentation. Roomy: the point of opening a row is to read it, and for a row
/// that no longer exists, to copy it back out by hand.
const READER_W: f32 = 760.0;
const READER_H: f32 = 620.0;

/// Read one blob out of whichever source recorded it.
fn read_prose(handle: &ProjectHandle, from: &VersionRef, blob: &str) -> String {
    let backups = BackupVersions {
        directories: handle.destinations.clone(),
        work_unique_id: handle.unique_id.clone(),
        project_path: handle.path.clone(),
    };
    let log = LogVersions::open(&handle.path);
    let source: &dyn VersionSource = match from.source {
        SourceKind::Backup => &backups,
        SourceKind::Log => &log,
    };
    source.prose(from, blob).unwrap_or_default()
}

/// The modal: a header saying what and when, then the prose.
struct PastReader {
    title: String,
    when: String,
    gone: bool,
    doc: TextDocument,
    root_child: Option<WidgetId>,
}

impl PastReader {
    fn new(title: String, when: String, text: String, gone: bool) -> Self {
        let doc = TextDocument::new();
        // A throwaway view document: no undo stack to clear, no comment anchors
        // to strand — the one place `set_djot_sync` is the right call.
        let _ = doc.set_djot_sync(&text);
        Self {
            title,
            when,
            gone,
            doc,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for PastReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PastReader").finish()
    }
}

impl Widget for PastReader {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Title, date, and a way out that is not Escape. `ModalRequest::title`
        // is only read when a modal becomes a native window, so an in-tree modal
        // that wants a title has to draw one.
        let header = teksu!(
            HStack {
                spacing: 8.0
                Expand::horizontal {
                    VStack {
                        spacing: 2.0
                        TextWidget::new(lit!(self.title.clone())) {
                            style: TextStyleRole::BodyBold
                        }
                        TextWidget::new(tr!(timeline_reader_stamp(date = self.when.clone()))) {
                            style: TextStyleRole::Small
                            color: TextRole::Secondary
                        }
                    }
                }
                IconButton::clear() {
                    tooltip: tr!(timeline_reader_close())
                    on_activate_fn: |ctx| ctx.dismiss_modal()
                }
            }
        );
        let mut column = VStack::new().spacing(0.0).child(header);
        if self.gone {
            // The one row this surface exists for. Said plainly, because a
            // writer who deleted a scene months ago will not otherwise realise
            // that what they are reading is unreachable from anywhere else.
            column =
                column.child(Padding::symmetric(6.0, 0.0).child(
                    TextWidget::new(tr!(timeline_reader_deleted())).color(TextRole::Warning),
                ));
        }
        // `FixedSize` wrapping a raised `Panel`, exactly as `panels::about` does
        // it, and both halves were missing. The size, because the modal
        // request's `.size()` is ignored for an in-tree presentation: an unsized
        // panel is measured at its intrinsic height, and an `Expand::vertical`
        // asked for its intrinsic height reports nothing — which is why this
        // opened as a bare header with no prose under it. The panel, because a
        // modal that paints no surface of its own is transparent: the header sat
        // directly over the manuscript behind it, and the two sets of words
        // interleaved.
        let id = teksu!(ctx => FixedSize {
            width: READER_W
            height: READER_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                Padding::uniform(16.0) {
                    child: column.child(
                        Expand::vertical().child(
                            RichTextEditor::read_only(self.doc.clone())
                                .content_padding_symmetric(8.0, 4.0)
                                .h_scroll_policy(ScrollPolicy::AlwaysOff),
                        ),
                    )
                }
            }
        });
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    /// The band lays out in every state it can be in.
    #[test]
    fn the_timeline_band_lays_out_in_every_state() {
        let cases: Vec<(&str, TimelineViewModel)> = vec![
            ("no project", TimelineViewModel::new()),
            ("missing project", {
                let vm = TimelineViewModel::new();
                vm.set_project(ProjectHandle {
                    path: "/nonexistent/Novel.skrib".into(),
                    unique_id: "u".into(),
                    destinations: vec!["/nonexistent".into()],
                    revision: 0,
                });
                vm
            }),
            ("failed scan", {
                let vm = TimelineViewModel::new();
                vm.error().set("could not look".into());
                vm
            }),
        ];
        for (name, vm) in cases {
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(Box::new(TimelinePanel::new(vm)));
            tree.layout(SizeProposal::exact(1200.0, 180.0));
            assert!(
                tree.bounds(id).width > 0.0,
                "the timeline band laid out to zero width in the '{name}' state",
            );
        }
    }

    fn moments(n: usize, days: i64) -> Vec<crate::timeline::Moment> {
        use skrib_format::versions::VersionRef;
        let start = chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        (0..n)
            .map(|i| {
                let at =
                    start + chrono::Duration::seconds(days * 86_400 * i as i64 / n.max(1) as i64);
                crate::timeline::Moment {
                    at,
                    source: SourceKind::Backup,
                    from: VersionRef {
                        path: std::path::PathBuf::from("/x.skrib"),
                        taken_at: at,
                        source: SourceKind::Backup,
                    },
                    bytes: 1000 + i as u64,
                }
            })
            .collect()
    }

    /// **The bug this band existed to have.** Three hundred backups over two
    /// years drew *nothing*: the old hand-rolled row spent 4 dp of `HStack`
    /// spacing per bar, so at that count the gaps alone exceeded the band's width
    /// and every bar laid out to zero. The chart is now handed a bucketed axis,
    /// which is bounded whatever the history.
    #[test]
    fn two_years_of_backups_still_draw_a_band() {
        let vm = TimelineViewModel::new();
        vm.seed_moments_for_test(moments(300, 730));
        let axis = vm.axis();
        assert!(axis.opens(), "300 versions have to collapse into periods");

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TimelinePanel::new(vm)));
        tree.layout(SizeProposal::exact(1200.0, 180.0));
        assert!(
            tree.bounds(id).width > 0.0,
            "the band laid out to nothing at 300 versions",
        );
    }

    /// Opening a period narrows the band to it, and the way back restores the
    /// whole history — the two halves of the only navigation this surface has.
    #[test]
    fn opening_a_period_narrows_the_band_and_the_way_back_widens_it() {
        let vm = TimelineViewModel::new();
        vm.seed_moments_for_test(moments(300, 730));
        assert!(vm.window().get().is_none(), "opens on the whole history");

        vm.open_selected();
        let window = vm.window().get().expect("opening a period sets a window");
        let inside = vm.visible_moments();
        assert!(
            !inside.is_empty() && inside.len() < 300,
            "a period holds some of the history and not all of it: {} rows",
            inside.len(),
        );
        assert!(
            inside.iter().all(|m| m.at >= window.0 && m.at <= window.1),
            "every visible moment has to be inside the window",
        );

        vm.show_all();
        assert!(vm.window().get().is_none());
        assert_eq!(vm.visible_moments().len(), 300);
    }

    /// The preset narrows the band to the recent end of a long history, which is
    /// the whole point of it: reaching last month in two years of monthly bars
    /// otherwise means opening a period and hoping.
    #[test]
    fn the_recent_preset_narrows_the_band_to_the_last_thirty_days() {
        let vm = TimelineViewModel::new();
        // A history ending *now*, so "the last thirty days" has something in it.
        let now = chrono::Utc::now();
        let mut history = moments(300, 730);
        let shift = now - history.last().unwrap().at;
        for m in &mut history {
            m.at += shift;
        }
        vm.seed_moments_for_test(history);
        assert!(vm.axis().opens(), "precondition: two years is bucketed");

        vm.set_last_days(RECENT_DAYS);
        let inside = vm.visible_moments();
        assert!(
            !inside.is_empty() && inside.len() < 300,
            "thirty days of a two-year history is {} moments",
            inside.len(),
        );
        let earliest = inside.first().expect("some moments are recent").at;
        assert!(
            now - earliest <= chrono::Duration::days(RECENT_DAYS as i64),
            "the window reached further back than it says",
        );

        vm.show_all();
        assert_eq!(vm.visible_moments().len(), 300);
    }

    /// A short history is not bucketed at all — the bars are the versions, and
    /// selecting one picks it rather than opening anything.
    #[test]
    fn a_short_history_is_not_bucketed() {
        let vm = TimelineViewModel::new();
        vm.seed_moments_for_test(moments(9, 30));
        let axis = vm.axis();
        assert_eq!(axis.bars.len(), 9);
        assert!(!axis.opens());
        // And "open this period" is inert rather than doing something arbitrary.
        vm.open_selected();
        assert!(vm.window().get().is_none());
    }

    /// The date filter and opening a period write the same window, so "show the
    /// whole history" means one thing however the band was narrowed.
    #[test]
    fn the_date_filter_narrows_the_same_window_a_period_does() {
        use teksilo::widgets::DateRange;
        let vm = TimelineViewModel::new();
        vm.seed_moments_for_test(moments(300, 730));
        let all = vm.visible_moments();
        let first = crate::date_convert::to_jiff_date(all[0].at).unwrap();
        let mid = crate::date_convert::to_jiff_date(all[all.len() / 2].at).unwrap();

        vm.range().set(Some(DateRange::new(first, mid)));
        vm.sync_window();
        let narrowed = vm.visible_moments();
        assert!(
            narrowed.len() < all.len() && !narrowed.is_empty(),
            "a range covering half the history shows {} of {}",
            narrowed.len(),
            all.len(),
        );

        vm.show_all();
        assert!(
            vm.range().get().is_none(),
            "going back clears the filter too"
        );
        assert_eq!(vm.visible_moments().len(), 300);
    }

    /// The reader is what reaches a deleted row, so it has to stand up with
    /// nothing but text — no project, no store, no live item.
    #[test]
    fn the_past_reader_lays_out_for_a_row_that_no_longer_exists() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(PastReader::new(
            "Cut opening".into(),
            "2026-03-14 09:00".into(),
            "The scene that did not survive the second draft.".into(),
            true,
        )));
        tree.layout(SizeProposal::exact(760.0, 620.0));
        assert!(tree.bounds(id).width > 0.0);
    }

    /// **The bug this guards.** An in-tree modal is a centred overlay sized by
    /// its content — `ModalRequest::size` is read only when the modal becomes a
    /// native window. So the reader is measured with nothing proposed, and the
    /// `Expand::vertical` holding the prose reported no height at all: a titled,
    /// dated panel with the text missing, which is exactly what it looked like.
    #[test]
    fn the_reader_carries_its_own_size_because_the_modal_request_does_not() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(PastReader::new(
            "Chapter 5".into(),
            "2026-03-14 09:00".into(),
            "There were six warships in the armada.".into(),
            false,
        )));
        // Unconstrained on both axes — what a centred overlay actually proposes.
        tree.layout(SizeProposal::unspecified());
        let bounds = tree.bounds(id);
        assert!(
            bounds.height >= READER_H,
            "with nothing proposed the reader collapsed to {}dp of height, \
             which is the panel without its prose",
            bounds.height,
        );
        assert!(
            bounds.width >= READER_W,
            "and to {}dp of width",
            bounds.width
        );
    }

    #[test]
    fn each_kind_of_change_carries_its_own_mark() {
        let mut marks = std::collections::BTreeSet::new();
        for kind in [
            ChangeKind::Added,
            ChangeKind::Removed,
            ChangeKind::Changed,
            ChangeKind::Moved,
        ] {
            let row = ChangeRow::new((
                0,
                &RowChange {
                    uid: uuid::Uuid::nil(),
                    title: "x".into(),
                    kind,
                    source: None,
                },
            ));
            marks.insert(row.mark);
        }
        assert_eq!(marks.len(), 4, "two kinds share a mark: {marks:?}");
    }
}
