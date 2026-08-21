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
    ScrollBarMode, Segment, SegmentSizing, SegmentedControl, Slider, Spacer, StandardListItem,
    Switcher, TextWidget, TraversalScopePolicy, VStack,
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
    /// What is currently in `model`, so an unchanged axis is not re-pushed.
    ///
    /// The **heights** and not just the labels, which is a bug this had: a
    /// bucketed bar is labelled after the *first* moment in its period and keeps
    /// that name as the period fills, while its height, its date and the moment
    /// it stands for are all overwritten by each later one. So a backup taken
    /// into a month the band already draws left every label identical, this
    /// returned early, and the chart went on drawing the size the project was
    /// before that backup — under a bar that had already moved on.
    drawn: RefCell<Vec<(String, u64)>>,
    /// The series name currently in `model`.
    ///
    /// Localised, so it cannot simply be set once in `new`: the writer can change
    /// the app's language without this panel being rebuilt from scratch. Kept
    /// here so `build` can push it only when it actually differs — `rename_series`
    /// bumps the model's style version, and doing that every frame is a repaint
    /// loop.
    named: RefCell<String>,
    /// The bar index this panel last wrote into `selection`, which is how a
    /// writer's click is told apart from our own mirroring of the slider.
    pushed: Cell<Option<usize>>,
    root_child: Option<WidgetId>,
}

impl TimelinePanel {
    fn new(vm: TimelineViewModel) -> Self {
        let model: ChartModel<String> = ChartModel::new();
        // **Named, though the legend is off.** A one-series chart has nothing to
        // distinguish itself from, so this was the empty string — and the name is
        // not only for a legend: `BarChart` builds its hover tooltip as
        // "{series}: {category} = {value}" and its screen-reader description as
        // "{series}, {category}: {value}". Anonymous, a writer hovering March got
        // a tooltip beginning with a bare colon, and a screen reader announced a
        // bare comma. `sync_name` keeps this in step with the app's language.
        let series = model.add_series(String::new());
        let selection = ChartSelection::attached(SelectionMode::Single, &model);
        Self {
            vm,
            model,
            series,
            selection,
            drawn: RefCell::new(Vec::new()),
            named: RefCell::new(String::new()),
            pushed: Cell::new(None),
            root_child: None,
        }
    }

    /// Keep the series' name in the app's current language.
    fn sync_name(&self) {
        let name = tr!(timeline_series_name()).resolve_now();
        if *self.named.borrow() == name {
            return;
        }
        self.model.rename_series(self.series, name.clone());
        *self.named.borrow_mut() = name;
    }

    /// Push the axis into the chart's model, if it is not already there.
    fn sync_chart(&self, axis: &Axis) {
        // Labels *and* heights — see the field docs for the period whose height
        // moved under an unchanged name.
        let drawn: Vec<(String, u64)> = axis
            .bars
            .iter()
            .map(|b| (b.label.clone(), b.bytes))
            .collect();
        if *self.drawn.borrow() == drawn {
            return;
        }
        self.model.replace_series_data(
            self.series,
            axis.bars
                .iter()
                .map(|b| ChartDatum::new(b.label.clone(), b.bytes as f32))
                .collect(),
        );
        *self.drawn.borrow_mut() = drawn;
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
        // **The app's language, because one string here is not a `tr!` the
        // framework re-resolves.** The chart's series name has to be handed to
        // `ChartModel` as a plain `String` (it is what `BarChart` splices into the
        // hover tooltip and the screen-reader description), so `sync_name` freezes
        // it with `resolve_now` — and `sync_name` runs from this function. None of
        // the bindings above is the locale, so without this the band went on
        // announcing "Project size" to a writer who had switched to French, in a
        // window where every other word had changed.
        ctx.locale_signal().bind_to(sid, reg, BindingLevel::Rebuild);
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
        self.sync_name();
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
            // The formatter is *not* dead weight beside `show_labels(false)`:
            // `BarChart` runs the hover tooltip's value through `axis_y` too, and
            // without one it prints the raw `f32` — so hovering March read
            // "= 482143", a number a novelist cannot use and will read as a word
            // count. It is not one: these are Djot source bytes, markup included,
            // and in French every accented character costs two of them, so it
            // sits several percent above the status bar's count. Through
            // `human_bytes` it becomes a size with its unit attached, which is
            // what it has always been, and it is the same wording the backup
            // settings already use for the same quantity.
            .axis_y(
                AxisConfig::new()
                    .show_labels(false)
                    .show_axis_line(false)
                    .formatter(size_label),
            )
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
        let moment = vm.selected();
        // **Which of the two records this moment came from decides what the list
        // beside it can possibly say**, and the list used to keep that to itself.
        // A backup is a whole bundle and answers all four kinds; the in-project
        // history log holds prose and nothing else, so against a log moment only
        // "edited" is ever reported — see `TimelineViewModel`'s module docs.
        //
        // Unsaid, a writer reads the difference as a fact about their book:
        // picking this morning's save and seeing nothing but edits says *nothing
        // was deleted or moved this morning*, which is a claim about the record
        // and not about the manuscript. Saying so costs one line.
        let prose_only = vm.selected_is_prose_only();
        if changes.is_empty() {
            // Not an error and not an empty project: the manuscript simply has
            // not moved since. Saying so beats an empty box — but "nothing has
            // changed" is more than a prose-only record can promise, so it says
            // the narrower true thing instead.
            return Box::new(note(if prose_only {
                tr!(timeline_no_text_changes())
            } else {
                tr!(timeline_no_changes())
            }));
        }
        // The moment the list is *against*. Empty only in the impossible case
        // where there are changes but no moment to have produced them, and an
        // empty date is better than refusing to draw the list.
        let when = moment
            .as_ref()
            .map(|m| m.at.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default();
        let rows: Vec<ChangeRow> = changes.iter().enumerate().map(ChangeRow::new).collect();
        let model = teksilo::data::ListModel::from_vec(rows);
        let open_vm = vm.clone();
        let mut column = VStack::new().spacing(0.0);
        column = column.child(
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
        );
        if prose_only {
            // Under the count, above the rows: the sentence has to be read
            // *before* the list is, because it is what the list's silences mean.
            column = column.child(
                Padding::symmetric(0.0, 10.0).child(
                    TextWidget::new(tr!(timeline_prose_only_record()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .overflow(TextOverflow::Wrap),
                ),
            );
        }
        column = column.child(
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
        );
        Box::new(column)
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

/// A bar's height, in words a writer means.
///
/// The same wording the backup settings already use for the same quantity, so the
/// two readouts of "how much is this" cannot disagree. A negative can only come
/// from a chart's own axis padding, never from a byte count.
fn size_label(v: f32) -> String {
    crate::backup_paths::human_bytes(v.max(0.0) as u64)
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

/// What the reader is showing, which is not decided until the archive is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReaderMode {
    /// The bundle is still being opened.
    Loading,
    /// The recording could not be read at all.
    ///
    /// Its own state, not an empty document: a bundle swept away by retention or
    /// sitting on an unplugged drive is **evidence of nothing**, and the empty
    /// string it used to become was then compared against the live text and
    /// rendered as "every word of this was written since". The band already holds
    /// that line elsewhere — a moment that could not be read must never be turned
    /// into a claim about the manuscript.
    Unreadable,
    /// The recorded text, on its own.
    Recorded,
    /// The recorded text set against what the row says now.
    Compared,
}

/// Show one row's prose as it was at the selected moment, read-only.
///
/// A modal rather than a pane in the band: the band is 180 dp of height, and the
/// point of opening a row is to *read* it — and, for a row that no longer exists,
/// to copy it back out by hand. `read_only` and not merely a disabled editor,
/// because the two are not the same promise: a read-only editor still selects and
/// copies, which is the whole reason to open this.
///
/// ## An edited row opens on a comparison, not on a mystery
///
/// "Edited since" is a label; the question behind it is *edited how*. For a
/// [`ChangeKind::Changed`] row the recorded text is set against what the row says
/// now through [`crate::versions::version_diff`] — the same struck-through and
/// underlined rendering the Versions dock and the import wizard already use, so
/// there is one way a comparison looks in this app. A removed row has no "now" to
/// compare with and opens on its text plain, which is exactly right: it is the
/// text itself the writer came for.
///
/// The fallback is deliberate rather than incidental. A row is *changed* when the
/// digest over **all** its prose moves, so a scene whose synopsis was rewritten is
/// a changed row whose body is untouched; comparing then yields nothing, and an
/// empty comparison under a row insisting something happened is worse than no
/// comparison at all. That case falls back to the recorded text and says so.
///
/// ## Opening the bundle does not block the frame
///
/// `read_prose` opens a zip and scans its central directory. Called straight from
/// the row's activation handler it stalled the frame on the click, which the
/// sibling Versions dock had already learned not to do. The modal is presented
/// immediately and its text arrives when the archive does; the comparison itself
/// is computed back on the UI thread, because rendering an elision needs the
/// app's language and the diff over one row's prose is microseconds.
fn open_past(ctx: &mut EventContext, vm: &TimelineViewModel, index: usize) {
    let changes = vm.changes().get();
    let Some(change) = changes.get(index) else {
        return;
    };
    let title = change.title.clone();
    let Some(past) = change.source.clone() else {
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
    // What the row says *now*, and only for a row that has a "now" and is claimed
    // to differ from it. Read here, on the UI thread, because the store is
    // `Rc`-backed and cannot cross one — and it costs nothing, being in memory.
    let live = match change.kind {
        ChangeKind::Changed => vm.live_prose(change.uid, &past.role),
        _ => None,
    };
    let handle = vm.project().get();
    let when = past.from.taken_at.format("%Y-%m-%d %H:%M").to_string();
    let gone = change.is_gone();

    // Two documents, not one swapped between: the writer switches back and forth
    // to check a passage against the comparison, and reparsing on every press
    // would throw away where they had scrolled to each time.
    let docs = ReaderDocs {
        diff: TextDocument::new(),
        text: TextDocument::new(),
    };
    let mode = Signal::new(ReaderMode::Loading);
    let view = Signal::new(0usize);
    {
        let (docs, mode, view) = (docs.clone(), mode.clone(), view.clone());
        ctx.present_modal(
            ModalRequest::deferred(move |t| {
                t.add(PastReader::new(
                    title.clone(),
                    when.clone(),
                    gone,
                    docs.clone(),
                    mode.clone(),
                    view.clone(),
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

    // Back on the UI thread once the bundle is open: decide what the reader is
    // showing, fill it, and say which.
    let fill = move |recorded: Option<String>| {
        let Some(recorded) = recorded else {
            mode.set(ReaderMode::Unreadable);
            return;
        };
        let (djot, showing) = compare_or_show(recorded.clone(), live.as_deref());
        // Throwaway view documents — no undo stack to clear, no comment anchors
        // to strand — which is the one place `set_djot_sync` is the right call.
        //
        // **The recorded text is always loaded, comparison or not.** A comparison
        // answers "what changed"; a writer reaching a scene they deleted in March
        // is asking the other question, and wants the paragraph as they wrote it,
        // clean enough to select and copy back out. The toggle is only reachable
        // when the two differ, but the plain side is always there behind it.
        let _ = docs.text.set_djot_sync(&recorded);
        if showing == ReaderMode::Compared {
            let _ = docs.diff.set_djot_sync(&djot);
        }
        mode.set(showing);
    };
    let read = move || read_prose(&handle, &past.from, &past.blob);
    match ctx.app_state::<AsyncRuntimeHandle>().cloned() {
        Some(rt) => rt
            .spawn_local(async move {
                // A dead worker is as unreadable as a dead file, and says so.
                fill(spawn_blocking(read).await.ok().flatten());
            })
            .detach(),
        // No runtime — the headless path. Straight through, as the view-model's
        // own fallback does.
        None => fill(read()),
    }
}

/// The document the reader shows, and what it is.
///
/// Split out of [`open_past`] so the fallback has a test of its own: an edited row
/// whose *body* did not move must not open on an empty comparison.
fn compare_or_show(recorded: String, live: Option<&str>) -> (String, ReaderMode) {
    let Some(now) = live else {
        return (recorded, ReaderMode::Recorded);
    };
    // Both sides empty is not a comparison, it is two blank pages.
    if recorded.trim().is_empty() && now.trim().is_empty() {
        return (recorded, ReaderMode::Recorded);
    }
    let diff = crate::versions::version_diff::diff_djot(&recorded, now);
    if diff.is_empty() {
        return (recorded, ReaderMode::Recorded);
    }
    let rendered = crate::versions::version_diff::render(
        &diff,
        Some(crate::versions::version_diff::CollapseRule::default()),
        &|n| tr!(versions_hidden_paragraphs(count = n as i64)).resolve_now(),
    );
    (rendered.djot, ReaderMode::Compared)
}

/// The reader's own size, since the modal request's is not read for an in-tree
/// presentation. Roomy: the point of opening a row is to read it, and for a row
/// that no longer exists, to copy it back out by hand.
const READER_W: f32 = 760.0;
const READER_H: f32 = 620.0;

/// Read one blob out of whichever source recorded it.
///
/// `None` when the bundle could not be opened — swept away by retention, on a
/// drive that is not plugged in, corrupt. **Not an empty string**, which is what
/// this returned and what turned an unreadable archive into a comparison saying
/// the whole scene had been written since.
fn read_prose(handle: &ProjectHandle, from: &VersionRef, blob: &str) -> Option<String> {
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
    source.prose(from, blob).ok()
}

/// The two things a reader can be showing, each in its own document.
///
/// Both are filled by [`open_past`] once the bundle is open; `diff` stays empty
/// unless there was something to compare with.
#[derive(Clone)]
struct ReaderDocs {
    diff: TextDocument,
    text: TextDocument,
}

/// Which of [`ReaderDocs`] is on screen. The order the segments are declared in.
const VIEW_DIFF: usize = 0;
const VIEW_TEXT: usize = 1;

/// The line under the title: what this reader is showing, right now.
///
/// **Its own widget so that [`PastReader`] does not have to react to `view`.**
/// The stamp has to change when the writer switches between the comparison and
/// the plain text; the reader must not, because rebuilding it destroys the
/// `Switcher` under it and with it both pages' scroll positions. Two `TextWidget`s
/// rebuilding on a switch is the cheap half of that trade.
struct ReaderStamp {
    when: String,
    mode: Signal<ReaderMode>,
    view: Signal<usize>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for ReaderStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReaderStamp").finish()
    }
}

impl Widget for ReaderStamp {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let (sid, reg) = (ctx.self_id(), ctx.binding_registry());
        self.mode.bind_to(sid, reg, BindingLevel::Rebuild);
        self.view.bind_to(sid, reg, BindingLevel::Rebuild);
        // The stamp says what the document under it actually is. A reader that
        // announced a comparison while holding the recorded text on its own —
        // which is what an edited row whose body did not move opens on, and what
        // the writer gets the moment they press "Text" — would be telling them
        // their scene was rewritten when it was not.
        let text = match (self.mode.get(), self.view.get()) {
            (ReaderMode::Loading, _) => tr!(timeline_reader_loading()),
            (ReaderMode::Unreadable, _) => tr!(timeline_reader_unreadable()),
            (ReaderMode::Compared, VIEW_DIFF) => {
                tr!(timeline_reader_compared(date = self.when.clone()))
            }
            _ => tr!(timeline_reader_stamp(date = self.when.clone())),
        };
        let id = ctx.add(
            TextWidget::new(text)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .overflow(TextOverflow::Wrap),
        );
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

/// What strikethrough and underline mean — shown only while they are on screen.
///
/// A sibling of [`ReaderStamp`] and for the same reason: it depends on `view`,
/// and [`PastReader`] must not.
struct ReaderLegend {
    mode: Signal<ReaderMode>,
    view: Signal<usize>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for ReaderLegend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReaderLegend").finish()
    }
}

impl Widget for ReaderLegend {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let (sid, reg) = (ctx.self_id(), ctx.binding_registry());
        self.mode.bind_to(sid, reg, BindingLevel::Rebuild);
        self.view.bind_to(sid, reg, BindingLevel::Rebuild);
        // Shape rather than colour carries the two sides, so the legend has to
        // name the shapes — the same pair the Versions dock draws. Only over the
        // comparison: on the plain text there are no marks to explain, and a
        // legend for marks that are not on screen is noise.
        let showing = self.mode.get() == ReaderMode::Compared && self.view.get() == VIEW_DIFF;
        let id = if showing {
            ctx.add(
                Padding::symmetric(6.0, 0.0).child(
                    TextWidget::new(tr!(timeline_reader_diff_legend()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .overflow(TextOverflow::Wrap),
                ),
            )
        } else {
            ctx.add(VStack::new())
        };
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

/// The modal: a header saying what, when, and what it is showing, then the prose.
struct PastReader {
    title: String,
    when: String,
    gone: bool,
    /// Filled by [`open_past`] once the bundle is open — the reader is presented
    /// before the archive has been read, so these start empty on purpose.
    docs: ReaderDocs,
    /// What was found, so the stamp under the title cannot claim a comparison the
    /// documents do not hold. Written by the same completion that fills them, and
    /// bound here so the header redraws with it.
    mode: Signal<ReaderMode>,
    /// Which of the two the writer is looking at — [`VIEW_DIFF`] or [`VIEW_TEXT`].
    ///
    /// Fresh per opening, and deliberately not remembered: a reader is opened to
    /// answer one question, and the next row opened is usually a different
    /// question. (It is also why this is a plain index rather than a `SegmentId`
    /// derived from a string — nothing here is persisted, so there is no id to
    /// keep stable across launches.)
    view: Signal<usize>,
    root_child: Option<WidgetId>,
}

impl PastReader {
    fn new(
        title: String,
        when: String,
        gone: bool,
        docs: ReaderDocs,
        mode: Signal<ReaderMode>,
        view: Signal<usize>,
    ) -> Self {
        Self {
            title,
            when,
            gone,
            docs,
            mode,
            view,
            root_child: None,
        }
    }

    /// One read-only editor over `doc`.
    ///
    /// `read_only` and not merely a disabled editor: the two are not the same
    /// promise, and a read-only editor still selects and copies, which is the
    /// whole reason to open this.
    fn page(doc: &TextDocument) -> impl Widget + use<> {
        RichTextEditor::read_only(doc.clone())
            .content_padding_symmetric(8.0, 4.0)
            .h_scroll_policy(ScrollPolicy::AlwaysOff)
    }

    /// **Comparison or plain text — the writer's choice, when there is one.**
    ///
    /// A comparison answers "what changed since". It is the better default, and
    /// it is the wrong thing to be stuck with: someone who has found the scene
    /// they cut in March wants the paragraph as they wrote it, without
    /// strikethrough running through every sentence they are trying to read and
    /// copy back out.
    ///
    /// Absent, not disabled, when there is nothing to compare — a removed row, a
    /// row whose body never moved, a bundle still opening. A two-way control with
    /// one reachable side is a question the writer cannot answer.
    fn switch(&self, comparing: bool) -> Box<dyn Widget> {
        if !comparing {
            return Box::new(Spacer::new());
        }
        let control = self.pages().into_iter().fold(
            SegmentedControl::indexed(self.view.clone())
                .label(tr!(timeline_reader_view_label()))
                .sizing(SegmentSizing::Fit)
                .text_style(TextStyleRole::Small),
            |c, (_, label, _)| c.segment(Segment::new(label)),
        );
        Box::new(control)
    }

    /// The prose itself: both pages behind a `Switcher` when there is a
    /// comparison, so switching keeps where each was scrolled to; the recorded
    /// text alone when there is not.
    fn body(&self, comparing: bool) -> Box<dyn Widget> {
        if !comparing {
            return Box::new(Self::page(&self.docs.text));
        }
        let switcher = self
            .pages()
            .into_iter()
            .fold(Switcher::new(self.view.clone()), |sw, (_, _, doc)| {
                sw.child_boxed(Box::new(Self::page(doc)))
            });
        Box::new(switcher)
    }

    /// The reader's two pages, in the order the switch declares them.
    ///
    /// One list builds the control and the switcher both, because a
    /// `SegmentedControl::indexed` writes a *position*: two lists in two places
    /// is the arrangement where pressing "Text" shows the comparison, and it is
    /// the same trap `tabs::shared::segments::RememberSegment` exists to close
    /// for the container bars.
    fn pages(&self) -> [(usize, teksilo::prelude::LocalizedString, &TextDocument); 2] {
        [
            (VIEW_DIFF, tr!(timeline_reader_view_diff()), &self.docs.diff),
            (VIEW_TEXT, tr!(timeline_reader_view_text()), &self.docs.text),
        ]
    }
}

impl std::fmt::Debug for PastReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PastReader").finish()
    }
}

impl Widget for PastReader {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // **`mode` only — never `view`.** A `BindingLevel::Rebuild` binding here
        // rebuilds this widget, and a rebuild destroys every child it has: the
        // `Switcher` holding both pages would come back new and empty, losing
        // where the writer had scrolled to in each. `Switcher` preserves a mounted
        // page across its *own* rebuilds and binds `view` itself for exactly that,
        // but nothing can save it from its parent being torn down. So the two
        // header lines that do depend on `view` are their own small widgets below,
        // and this one reacts only to the text arriving.
        self.mode
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let showing = self.mode.get();
        let comparing = showing == ReaderMode::Compared;
        // The stamp says what the document under it actually is. A reader that
        // announced a comparison while holding the recorded text on its own —
        // which is what an edited row whose body did not move opens on — would be
        // telling the writer their scene was rewritten when it was not.
        let stamp = ReaderStamp {
            when: self.when.clone(),
            mode: self.mode.clone(),
            view: self.view.clone(),
            root: None,
        };
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
                        child: stamp
                    }
                }
                child: crate::tabs::Boxed::new(self.switch(comparing))
                IconButton::clear() {
                    tooltip: tr!(timeline_reader_close())
                    on_activate_fn: |ctx| ctx.dismiss_modal()
                }
            }
        );
        let mut column = VStack::new()
            .spacing(0.0)
            .child(header)
            .child(ReaderLegend {
                mode: self.mode.clone(),
                view: self.view.clone(),
                root: None,
            });
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
                        Expand::vertical()
                            .child(crate::tabs::Boxed::new(self.body(comparing))),
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
        let id = tree.add_boxed(Box::new(reader(
            "Cut opening",
            "The scene that did not survive the second draft.",
            true,
            ReaderMode::Recorded,
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
        let id = tree.add_boxed(Box::new(reader(
            "Chapter 5",
            "There were six warships in the armada.",
            false,
            ReaderMode::Recorded,
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

    /// The reader as `open_past` builds it once the archive is open: a document
    /// already filled, and a mode saying what is in it.
    fn reader(title: &str, text: &str, gone: bool, mode: ReaderMode) -> PastReader {
        reader_viewing(title, text, gone, mode, VIEW_DIFF)
    }

    fn reader_viewing(
        title: &str,
        text: &str,
        gone: bool,
        mode: ReaderMode,
        view: usize,
    ) -> PastReader {
        let docs = ReaderDocs {
            diff: TextDocument::new(),
            text: TextDocument::new(),
        };
        let _ = docs.text.set_djot_sync(text);
        if mode == ReaderMode::Compared {
            let _ = docs.diff.set_djot_sync(text);
        }
        PastReader::new(
            title.into(),
            "2026-03-14 09:00".into(),
            gone,
            docs,
            Signal::new(mode),
            Signal::new(view),
        )
    }

    /// **The point of comparing at all.** "Edited since" is a label; a writer
    /// wants to know edited *how*, and the app already owns the rendering that
    /// says so.
    #[test]
    fn an_edited_row_opens_on_a_comparison_with_what_it_says_now() {
        let (djot, mode) =
            compare_or_show("The lamp went out.".to_string(), Some("The lamp guttered."));
        assert_eq!(mode, ReaderMode::Compared);
        assert!(
            djot.contains("{+") && djot.contains("{-"),
            "a comparison has to carry both sides: {djot}",
        );
    }

    /// **The fallback, and it is not a detail.** A row counts as changed when the
    /// digest over *all* its prose moves, so a scene whose synopsis was rewritten
    /// is a changed row whose body never moved. Comparing then yields nothing, and
    /// an empty pane under a row insisting something happened is worse than no
    /// comparison at all.
    #[test]
    fn a_row_whose_body_did_not_move_opens_on_its_text_rather_than_an_empty_comparison() {
        let same = "The lamp went out.";
        let (djot, mode) = compare_or_show(same.to_string(), Some(same));
        assert_eq!(mode, ReaderMode::Recorded);
        assert_eq!(djot, same, "the recorded text, whole, not a blank pane");
    }

    /// **The comparison a writer would most want, and the one a filter hid.** A
    /// scene whose text has since been emptied reads back as the empty string,
    /// not as "no text to compare with" — and showing the recorded prose plain
    /// there says nothing, where the comparison says *all of this is gone*.
    #[test]
    fn a_scene_that_has_since_been_emptied_shows_the_whole_thing_struck_through() {
        let (djot, mode) = compare_or_show("The lamp went out.".to_string(), Some(""));
        assert_eq!(mode, ReaderMode::Compared);
        assert!(
            djot.contains("{-"),
            "every word went, and the comparison has to say so: {djot}",
        );
    }

    /// …but two blank pages are not a comparison.
    #[test]
    fn a_row_that_was_empty_and_still_is_opens_plain() {
        let (_, mode) = compare_or_show(String::new(), Some(""));
        assert_eq!(mode, ReaderMode::Recorded);
    }

    /// A removed row has no "now" to be set against, and its text is the whole
    /// reason this surface exists.
    #[test]
    fn a_row_that_is_gone_opens_on_its_text_alone() {
        let (djot, mode) = compare_or_show("The cut chapter.".to_string(), None);
        assert_eq!(mode, ReaderMode::Recorded);
        assert_eq!(djot, "The cut chapter.");
    }

    /// The reader is presented before the bundle is open, so it has to lay out
    /// with nothing in it — otherwise every click flashes a collapsed panel.
    #[test]
    fn the_reader_lays_out_while_the_bundle_is_still_being_opened() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(reader(
            "Chapter 5",
            "",
            false,
            ReaderMode::Loading,
        )));
        tree.layout(SizeProposal::unspecified());
        assert!(tree.bounds(id).height >= READER_H);
    }

    /// Every line of text the reader draws, however deep.
    fn labels(tree: &WidgetTree, id: WidgetId) -> usize {
        // The name is fully qualified — `teksilo_widgets::primitives::…::TextWidget`.
        let mine = usize::from(
            tree.widget_type_name(id)
                .is_some_and(|n| n.ends_with("TextWidget")),
        );
        mine + tree
            .children(id)
            .into_iter()
            .map(|c| labels(tree, c))
            .sum::<usize>()
    }

    fn laid_out(mode: ReaderMode) -> usize {
        laid_out_viewing(mode, VIEW_DIFF)
    }

    fn laid_out_viewing(mode: ReaderMode, view: usize) -> usize {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(reader_viewing(
            "Chapter 5",
            "text",
            false,
            mode,
            view,
        )));
        tree.layout(SizeProposal::unspecified());
        labels(&tree, id)
    }

    /// A comparison is carried by shape, not colour — strikethrough and underline
    /// — so the reader has to name the shapes, and only while it is showing one.
    /// Counted rather than read: `TextWidget`'s `Debug` says nothing about its
    /// content.
    #[test]
    fn the_legend_appears_only_when_a_comparison_is_on_screen() {
        let plain = laid_out(ReaderMode::Recorded);
        assert_eq!(
            laid_out(ReaderMode::Compared),
            // The legend, plus the switch's two segment labels — which a
            // comparison also earns and a plain reading does not.
            plain + 3,
            "a comparison has to explain what struck through and underlined mean",
        );
        assert_eq!(
            laid_out(ReaderMode::Loading),
            plain,
            "nothing is being compared yet, so nothing may say it is",
        );
    }

    /// **The way out of a comparison.** A comparison answers "what changed"; a
    /// writer who has found the scene they cut in March is asking the other
    /// question, and wants the paragraph as they wrote it — not strikethrough
    /// running through every sentence they are trying to copy back out.
    #[test]
    fn a_comparison_can_be_switched_to_the_plain_recorded_text() {
        let mut tree = WidgetTree::new();
        let view = Signal::new(VIEW_DIFF);
        let docs = ReaderDocs {
            diff: TextDocument::new(),
            text: TextDocument::new(),
        };
        let _ = docs
            .diff
            .set_djot_sync("The lamp {-went out-}{+guttered+}.");
        let _ = docs.text.set_djot_sync("The lamp went out.");
        let id = tree.add_boxed(Box::new(PastReader::new(
            "Chapter 5".into(),
            "2026-03-14 09:00".into(),
            false,
            docs,
            Signal::new(ReaderMode::Compared),
            view.clone(),
        )));
        tree.layout(SizeProposal::unspecified());
        let with_legend = labels(&tree, id);

        view.set(VIEW_TEXT);
        tree.layout(SizeProposal::unspecified());
        assert_eq!(
            labels(&tree, id),
            with_legend - 1,
            "on the plain text there are no marks to explain, so the legend goes",
        );
        assert!(tree.bounds(id).height >= READER_H, "and the panel holds");
    }

    /// **The regression this shape exists to prevent.** Binding `view` on the
    /// reader made switching rebuild it, and a rebuild destroys every child —
    /// including the `Switcher` holding both pages, which came back new and empty
    /// with both scroll positions at the top. `Switcher` keeps a mounted page
    /// across its *own* rebuilds and binds `view` itself for exactly that, but it
    /// cannot survive its parent being torn down.
    ///
    /// Asserted as identity: the widget the reader mounted for the prose has to
    /// be the same one after a switch.
    #[test]
    fn switching_views_does_not_rebuild_the_pages_underneath() {
        let mut tree = WidgetTree::new();
        let view = Signal::new(VIEW_DIFF);
        let docs = ReaderDocs {
            diff: TextDocument::new(),
            text: TextDocument::new(),
        };
        let _ = docs
            .diff
            .set_djot_sync("The lamp {-went out-}{+guttered+}.");
        let _ = docs.text.set_djot_sync("The lamp went out.");
        let id = tree.add_boxed(Box::new(PastReader::new(
            "Chapter 5".into(),
            "2026-03-14 09:00".into(),
            false,
            docs,
            Signal::new(ReaderMode::Compared),
            view.clone(),
        )));
        tree.layout(SizeProposal::unspecified());
        let before = switcher_id(&tree, id).expect("a comparison mounts a Switcher");

        view.set(VIEW_TEXT);
        tree.layout(SizeProposal::unspecified());
        assert_eq!(
            switcher_id(&tree, id),
            Some(before),
            "the pages were rebuilt, so both scroll positions were thrown away",
        );
    }

    fn switcher_id(tree: &WidgetTree, id: WidgetId) -> Option<WidgetId> {
        if tree
            .widget_type_name(id)
            .is_some_and(|n| n.ends_with("Switcher"))
        {
            return Some(id);
        }
        tree.children(id)
            .into_iter()
            .find_map(|c| switcher_id(tree, c))
    }

    /// A recording that could not be read is **evidence of nothing**. It used to
    /// become the empty string, which was then compared against the live text and
    /// rendered as "every word of this was written since" — a claim about the
    /// manuscript invented out of an unplugged drive.
    #[test]
    fn a_recording_that_cannot_be_read_makes_no_claim_about_the_manuscript() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(reader(
            "Chapter 5",
            "",
            false,
            ReaderMode::Unreadable,
        )));
        tree.layout(SizeProposal::unspecified());
        assert!(tree.bounds(id).height >= READER_H, "and it still lays out");
        assert_eq!(
            segments(&tree, id),
            0,
            "there is nothing to compare, so nothing may offer a comparison",
        );
    }

    /// The control is absent, not disabled, when there is nothing to compare: a
    /// two-way switch with one reachable side is a question a writer cannot
    /// answer.
    #[test]
    fn there_is_no_switch_when_there_is_nothing_to_switch_to() {
        for mode in [
            ReaderMode::Recorded,
            ReaderMode::Loading,
            ReaderMode::Unreadable,
        ] {
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(Box::new(reader("Chapter 5", "text", false, mode)));
            tree.layout(SizeProposal::unspecified());
            let found = segments(&tree, id);
            assert_eq!(found, 0, "{mode:?} offered a choice it cannot honour");
        }
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(reader(
            "Chapter 5",
            "text",
            false,
            ReaderMode::Compared,
        )));
        tree.layout(SizeProposal::unspecified());
        assert_eq!(segments(&tree, id), 1, "a comparison offers both views");
    }

    /// **The trap one list closes.** A `SegmentedControl::indexed` writes a
    /// position, so the segments and the switcher's children have to be built in
    /// the same order — two lists in two places is the arrangement where pressing
    /// "Text" shows the comparison.
    #[test]
    fn the_switch_and_the_pages_are_built_from_one_ordered_list() {
        let r = reader("Chapter 5", "text", false, ReaderMode::Compared);
        let order: Vec<usize> = r.pages().iter().map(|(i, _, _)| *i).collect();
        assert_eq!(
            order,
            vec![VIEW_DIFF, VIEW_TEXT],
            "the list's positions have to be the indices the control writes",
        );
    }

    /// `SegmentedControl` nodes anywhere under `id`.
    fn segments(tree: &WidgetTree, id: WidgetId) -> usize {
        let mine = usize::from(
            tree.widget_type_name(id)
                .is_some_and(|n| n.ends_with("SegmentedControl")),
        );
        mine + tree
            .children(id)
            .into_iter()
            .map(|c| segments(tree, c))
            .sum::<usize>()
    }

    /// What the chart is actually drawing, bar by bar.
    fn drawn_bars(panel: &TimelinePanel) -> Vec<(String, f32)> {
        panel
            .model
            .with_series_view(panel.series, |v| {
                v.points
                    .iter()
                    .map(|d| (d.category.clone(), d.value))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// **The staleness this had.** A bucketed bar is named after the *first*
    /// moment of its period and keeps that name as the period fills, while its
    /// height is overwritten by each later one. The chart decided whether to
    /// redraw by comparing labels alone — so a backup taken into a month the band
    /// already draws changed no label, the push was skipped, and the bar went on
    /// showing the size the project was before that backup.
    #[test]
    fn a_backup_landing_in_a_period_the_band_already_draws_moves_its_bar() {
        let vm = TimelineViewModel::new();
        let mut history = moments(300, 730);
        vm.seed_moments_for_test(history.clone());

        let panel = TimelinePanel::new(vm.clone());
        let before = vm.axis();
        assert!(before.opens(), "precondition: two years is bucketed");
        panel.sync_chart(&before);
        let drawn_before = drawn_bars(&panel);
        assert!(!drawn_before.is_empty());

        // One more backup, minutes after the newest and inside the same period.
        let newest = history.last().cloned().expect("a history to extend");
        history.push(crate::timeline::Moment {
            at: newest.at + chrono::Duration::minutes(5),
            bytes: newest.bytes + 50_000,
            ..newest
        });
        vm.seed_moments_for_test(history);

        let after = vm.axis();
        let names = |bars: &[(String, f32)]| -> Vec<String> {
            bars.iter().map(|(l, _)| l.clone()).collect()
        };
        assert_eq!(
            after
                .bars
                .iter()
                .map(|b| b.label.clone())
                .collect::<Vec<_>>(),
            names(&drawn_before),
            "precondition: not one label moved, which is what made this invisible",
        );

        panel.sync_chart(&after);
        assert_ne!(
            drawn_bars(&panel),
            drawn_before,
            "the last period grew by 50 kB and the bar has to grow with it",
        );
    }

    /// **What a writer sees when they hover a bar.** `BarChart` builds the
    /// tooltip as "{series}: {category} = {value}" and its screen-reader
    /// description as "{series}, {category}: {value}", running the value through
    /// `axis_y`'s formatter. Anonymous and unformatted, hovering March read
    /// ": 2026-03 = 482143" — a leading colon and a raw byte count a novelist
    /// will read as a word count, which it is not: these are Djot source bytes,
    /// markup included, and in French every accented character costs two of them.
    #[test]
    fn hovering_a_bar_names_what_it_measures_and_gives_the_size_a_unit() {
        let panel = TimelinePanel::new(TimelineViewModel::new());
        panel.sync_name();
        let name = panel
            .model
            .with_series(panel.series, |name, _, _| name.to_string())
            .expect("the chart has its one series");
        assert!(
            !name.trim().is_empty(),
            "an anonymous series opens the tooltip on a bare colon",
        );

        let shown = size_label(482_143.0);
        assert!(
            shown.chars().any(|c| c.is_alphabetic()),
            "a bare number is the thing being fixed, and this is {shown}",
        );
        assert_ne!(shown, "482143");
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
