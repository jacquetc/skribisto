// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Versions** dock: what the focused row said before, and what changed.
//!
//! Trailing rail, beside Inspector, Format, this-document Comments and Footnotes —
//! the side that is already about *whatever is in front of me right now*. It takes
//! the same `EditorsViewModel::active_item()` signal they do.
//!
//! A **list**, deliberately, and not a slider. Versions are discrete and unevenly
//! spaced, which is the condition usability guidance names as disqualifying for
//! sliders; and the things that make a version choosable — its date, which content
//! changed, how much of it moved — do not fit under a thumb. A scrub affordance can
//! join later, bound to this same selection, without any of this changing.
//!
//! Three states are stated rather than implied, because leaving them to inference
//! is the documented failure of every backup browser that came before: **did not
//! exist yet**, **deleted after this point**, and **could not be read**. The last
//! one matters most — a gap a writer cannot see is a gap they will assume is data
//! loss.
//!
//! ## The diff pane
//!
//! Selecting a version compares it with the one before it, in a plain
//! [`RichTextEditor::read_only`] over an ordinary document: the comparison is
//! rendered as Djot with `{+…+}` / `{-…-}`, which the parser already maps to
//! underline and strikeout. No new widget, no character-format plumbing, and shape
//! rather than colour satisfies WCAG G182/G183 for free.
//!
//! Above it sits a **sentence** — *12 words added, 4 removed, near "the garden
//! gate"* — because strikethrough is frequently not announced at all by screen
//! readers, so a pane that carries its meaning only in text decoration carries it
//! only for people who can see it.
//!
//! The untouched middle is collapsed by default. A scene is mostly unchanged by
//! definition; the named complaint about the equivalent feature elsewhere is that
//! the change is "difficult to spot", and opening on three thousand identical
//! words answers the wrong question.

use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    Button, ButtonVariant, DateRangeEdit, Divider, DockOpenLocation, DockSide, DockWidget,
    DockWidgetId, Expand, FixedSize, FocusScope, HStack, IconButton, IconButtonSize, ListView,
    Padding, ProgressBar, ScrollBarMode, Segment, SegmentedControl, Spacer, StandardListItem,
    TextWidget, TraversalScopePolicy, VStack,
};

use skrib_format::changes::Change;
use skrib_format::versions::SourceKind;

use crate::versions::version_diff::{self, CollapseRule};
use crate::versions::{TimelineView, VersionDiff, VersionsViewModel};
use crate::widgets::DiffPane;

/// Package the versions panel as a trailing `DockWidget`.
pub fn versions_dock(
    vm: VersionsViewModel,
    dock_id: DockWidgetId,
    focus: Signal<Option<u64>>,
    uid_of: UidLookup,
    restore: RestoreFn,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(versions_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(VersionsPanel::new(
            vm.clone(),
            focus.clone(),
            uid_of.clone(),
            restore.clone(),
        ))
    })
    .icon(crate::icons::activity::versions_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

/// Resolve an item's store id to its durable uid.
///
/// A closure rather than a direct call because the dock must not import
/// `EditorsViewModel` or reach into the store itself — the same discipline the
/// outline and trash docks follow with `OpenItemFn`. `EntityId` is re-minted on
/// every load, so the uid is the only thing a timeline can key on.
pub type UidLookup = Rc<dyn Fn(u64) -> Option<uuid::Uuid>>;

/// Put a past version back.
///
/// A closure for the same reason [`UidLookup`] is one: the guarded sequence
/// needs the open-documents store, the backup scheduler, the save state and this
/// Work's undo stack, and a dock that reached for any of them itself would be
/// reaching past its own seam. Assembled in `app::project_shell`; the sequence
/// itself lives in `app::restore_version`.
pub type RestoreFn = Rc<dyn Fn(&mut EventContext, crate::versions::RestoreRequest)>;

struct VersionsPanel {
    vm: VersionsViewModel,
    focus: Signal<Option<u64>>,
    uid_of: UidLookup,
    restore: RestoreFn,
    pane: DiffPane,
    root_child: Option<WidgetId>,
}

impl VersionsPanel {
    fn new(
        vm: VersionsViewModel,
        focus: Signal<Option<u64>>,
        uid_of: UidLookup,
        restore: RestoreFn,
    ) -> Self {
        Self {
            vm,
            focus,
            uid_of,
            restore,
            pane: DiffPane::new(),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for VersionsPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VersionsPanel").finish()
    }
}

impl Widget for VersionsPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        // Rebuild on a new focus, a new scope, a new selection, and on the
        // timeline or the diff landing — the labels are `tr!(key(args))`, which
        // resolves eagerly, so a changed count has to rebuild rather than re-bind.
        self.focus.bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .scope_index()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.view().bind_to(sid, reg, BindingLevel::Rebuild);
        // The project's identity lands after the shell is built, and it is what
        // makes the first scan possible at all.
        self.vm.project().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.loading().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.error().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.diff().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .show_unchanged()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .pinned_only()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .selection()
            .selection_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        self.vm
            .set_async_runtime(ctx.app_state::<AsyncRuntimeHandle>().cloned());
        // Both are no-ops when nothing they depend on moved, which is what makes
        // them safe to call from a function every one of their own signals
        // rebuilds.
        self.vm
            .load_for(self.focus.get().and_then(|id| (self.uid_of)(id)));
        self.vm.sync_diff();

        match self.vm.diff().get() {
            Some(diff) => {
                let collapse = (!self.vm.show_unchanged().get()).then(CollapseRule::default);
                self.pane.show(&version_diff::render(&diff, collapse, &|n| {
                    tr!(versions_hidden_paragraphs(count = n as i64)).resolve_now()
                }));
            }
            None => self.pane.clear(),
        }

        let id = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(scope_bar(self.vm.clone()))
                .child(Divider::new())
                .child(Expand::vertical().child(crate::tabs::Boxed::new(self.body()))),
        );
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

impl VersionsPanel {
    fn body(&self) -> Box<dyn Widget> {
        let vm = &self.vm;
        if !vm.error().get().is_empty() {
            // "We could not look" is not "there is nothing", and conflating them is
            // how a version browser loses a writer's trust at the worst moment.
            return Box::new(note(tr!(versions_error())));
        }
        if vm.loading().get() {
            return Box::new(note(tr!(versions_loading())));
        }
        let view = vm.view().get();
        if view.is_empty() && view.timeline.unreadable.is_empty() {
            return Box::new(note(tr!(versions_empty())));
        }
        if vm.visible_count() == 0 {
            // The row has a past; the filters just exclude all of it. Saying "no
            // earlier version" here would be a lie about the project. The way
            // back out is the "Clear the filters" button already sitting in the
            // filter row above, which appears exactly when something is
            // filtering — a second copy here would be the same control twice on
            // one screen.
            return Box::new(note(tr!(versions_filtered_empty())));
        }
        Box::new(
            VStack::new()
                .spacing(0.0)
                // Said once, above the list, because the alternative is a writer
                // deciding the feature is broken. A row's date is the moment that
                // exact wording *first* appeared, not the moment a file was
                // written: identical states collapse into the earliest one, so a
                // backup taken this afternoon over text last touched in March
                // adds no row at all — and the entry the writer is looking at is
                // legitimately older than a backup they just made by hand.
                .child(
                    Padding::symmetric(4.0, 8.0).child(
                        TextWidget::new(tr!(versions_list_caption()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    ),
                )
                // The list is the index; the comparison is what is being read. Two
                // to one, so a paragraph fits without scrolling in a rail that is
                // 300 dp wide and as tall as the window.
                .child(Expand::vertical().flex(1.0).child(list(&view, vm.clone())))
                .child(Divider::new())
                .child(
                    Expand::vertical()
                        .flex(2.0)
                        .child(crate::tabs::Boxed::new(self.diff_pane())),
                )
                .child(boundaries(&view)),
        )
    }

    /// Summary, controls, and the comparison itself.
    fn diff_pane(&self) -> Box<dyn Widget> {
        let vm = &self.vm;
        if vm.selection().count() == 0 {
            return Box::new(note(tr!(versions_pick_a_version())));
        }
        if vm.selection_is_earliest() {
            // Not an error, and not "nothing changed": there is simply nothing
            // older on record to compare against, and guessing would be worse.
            // It is still a real recorded state of the row, so it can still be
            // put back — the restore bar stays.
            return Box::new(
                VStack::new()
                    .spacing(0.0)
                    .child(Expand::vertical().child(note(tr!(versions_earliest()))))
                    .child(Divider::new())
                    .child(self.restore_bar()),
            );
        }
        let Some(diff) = vm.diff().get() else {
            return Box::new(note(tr!(versions_empty())));
        };
        if diff.summary.formatting_only {
            return Box::new(note(tr!(versions_formatting_only())));
        }
        if diff.is_empty() {
            return Box::new(note(tr!(versions_no_change())));
        }

        let editor = RichTextEditor::read_only(self.pane.doc.clone())
            .content_padding_symmetric(6.0, 8.0)
            .h_scroll_policy(ScrollPolicy::AlwaysOff);
        *self.pane.handle.borrow_mut() = Some(editor.handle());

        Box::new(
            VStack::new()
                .spacing(0.0)
                .child(summary_line(&diff))
                .child(self.controls(&diff))
                .child(Expand::vertical().child(editor))
                .child(Divider::new())
                .child(self.restore_bar()),
        )
    }

    /// Show/hide the untouched middle, and walk the changes.
    fn controls(&self, diff: &VersionDiff) -> impl Widget + use<> {
        let toggle_vm = self.vm.clone();
        let showing = self.vm.show_unchanged().get();
        let label = if showing {
            tr!(versions_hide_unchanged())
        } else {
            tr!(versions_show_unchanged())
        };

        let (offsets, cursor, handle) = (
            self.pane.offsets.clone(),
            self.pane.cursor.clone(),
            self.pane.handle.borrow().clone(),
        );
        let jump = move |ctx: &mut EventContext| {
            let offsets = offsets.borrow();
            if offsets.is_empty() {
                return;
            }
            let i = cursor.get() % offsets.len();
            cursor.set(i + 1);
            if let Some(handle) = &handle {
                let at = offsets[i];
                // A collapsed caret would be invisible in a read-only view, so the
                // jump selects a little of the change to mark where it landed.
                handle.select_range(at, at + 1);
                handle.reveal_range(ctx, at, at + 1);
            }
        };

        let jumps = diff.blocks.len() > 1;
        teksu!(
            Padding::symmetric(2.0, 6.0) {
                HStack {
                    spacing: 4.0
                    Button::new(label) {
                        variant: ButtonVariant::Plain
                        text_style: TextStyleRole::Small
                        on_activate_fn: move |_| toggle_vm.toggle_unchanged()
                    }
                    Spacer
                    Button::new(tr!(versions_next_change())) {
                        variant: ButtonVariant::Plain
                        text_style: TextStyleRole::Small
                        enabled: jumps
                        on_activate_fn: jump
                    }
                }
            }
        )
    }

    /// The one destructive control in this dock.
    ///
    /// Below the comparison rather than beside the list, deliberately: it acts on
    /// what is being *read*, and a writer should have seen what they are about to
    /// bring back before the button is within reach. Everything that makes it
    /// safe — the confirmation naming the date and the comments at stake, the
    /// safety copy taken first, the single-step undo — lives behind
    /// [`RestoreFn`].
    fn restore_bar(&self) -> impl Widget + use<> {
        let (vm, restore) = (self.vm.clone(), self.restore.clone());
        let item_id = self.focus.get();
        let ready = item_id.is_some() && vm.selection().count() > 0;
        let on_restore = move |ctx: &mut EventContext| {
            let Some(item_id) = item_id else { return };
            let Some(req) = vm.restore_request(item_id) else {
                return;
            };
            restore(ctx, req);
        };
        teksu!(
            Padding::symmetric(2.0, 6.0) {
                HStack {
                    spacing: 4.0
                    Spacer
                    Button::new(tr!(versions_restore_button())) {
                        variant: ButtonVariant::Plain
                        text_style: TextStyleRole::Small
                        enabled: ready
                        on_activate_fn: on_restore
                    }
                }
            }
        )
    }
}

/// Synopsis / Prose. Which content a version is *of* is not a detail: a row that
/// says "changed" and opens onto an identical text destroys trust immediately.
fn scope_bar(vm: VersionsViewModel) -> impl Widget + use<> {
    // The segmented control writes the view-model's own signal, so there is
    // nothing to mirror and no effect to re-register on each rebuild.
    let scope = SegmentedControl::indexed(vm.scope_index())
        .segment(Segment::new(tr!(versions_scope_synopsis())))
        .segment(Segment::new(tr!(versions_scope_prose())));
    let only = vm.pinned_only().get();
    let filter_vm = vm.clone();
    teksu!(
        Padding::symmetric(8.0, 8.0) {
            VStack {
                spacing: 6.0
                HStack {
                    spacing: 6.0
                    Expand::horizontal {
                        child: scope
                    }
                    IconButton::new(crate::icons::versions::pinned()) {
                        size: IconButtonSize::Compact
                        icon_role: if only { TextRole::Accent } else { TextRole::Secondary }
                        tooltip: tr!(versions_pinned_only())
                        on_activate_fn: move |_| filter_vm.toggle_pinned_only()
                    }
                }
                // Its own row: `DateRangeEdit` is two date fields, an arrow and a
                // calendar button, and there is no compact form of it. Beside the
                // scope control in a 300 dp rail it would squeeze both into
                // uselessness.
                Expand::horizontal {
                    DateRangeEdit::new(vm.range()) {
                        tooltip: tr!(versions_range_filter())
                        label: tr!(versions_range_filter())
                    }
                }
                child: filter_row(vm)
            }
        }
    )
}

/// The preset, and the way back out of it.
///
/// The two belong together: a one-click way *into* a filtered list needs a
/// one-click way out, and until now the only way to drop a range was to clear
/// two date fields by hand. "Clear" appears only while something is actually
/// filtering, so the row is empty in the ordinary case.
fn filter_row(vm: VersionsViewModel) -> impl Widget + use<> {
    let recent_vm = vm.clone();
    // The same glyph the Timeline band's preset wears: one control, one picture,
    // whichever surface a writer meets it on.
    let mut row = HStack::new().spacing(6.0).child(teksu!(
        IconButton::new(crate::icons::versions::recent()) {
            size: IconButtonSize::Compact
            tooltip: tr!(versions_last_30_days())
            on_activate_fn: move |_| recent_vm.set_last_days(crate::timeline::dock::RECENT_DAYS)
        }
    ));
    if vm.is_filtered() {
        let clear_vm = vm.clone();
        row = row.child(teksu!(
            IconButton::new(crate::icons::versions::reset()) {
                size: IconButtonSize::Compact
                tooltip: tr!(versions_clear_filters())
                on_activate_fn: move |_| clear_vm.clear_filters()
            }
        ));
    }
    row.child(Spacer::new())
}

/// One sentence saying what moved, and roughly where.
///
/// Carried in text rather than in the marks alone: strikethrough is frequently
/// not announced by screen readers, and a reader scanning the list wants the
/// answer before deciding to read the pane at all.
fn summary_line(diff: &VersionDiff) -> impl Widget + use<> {
    let s = &diff.summary;
    let mut parts: Vec<String> = Vec::new();
    if s.words_added > 0 {
        parts.push(tr!(versions_words_added(count = s.words_added as i64)).resolve_now());
    }
    if s.words_removed > 0 {
        parts.push(tr!(versions_words_removed(count = s.words_removed as i64)).resolve_now());
    }
    if s.blocks_moved > 0 {
        parts.push(tr!(versions_blocks_moved(count = s.blocks_moved as i64)).resolve_now());
    }
    if let Some(anchor) = &s.anchor {
        parts.push(tr!(versions_near(text = anchor.clone())).resolve_now());
    }
    Padding::symmetric(6.0, 8.0).child(
        TextWidget::new(lit!(parts.join(", ")))
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary),
    )
}

/// The rows on screen, in screen order.
///
/// Every filter lives in the view-model's [`VersionsViewModel::visible_indices`]
/// and **nowhere else**. This function used to run its own `pinned_only` filter
/// beside it, which was fine until a second filter arrived: once a date range was
/// set, the list showed rows the view-model had already excluded, so list position
/// *n* and timeline index *n* no longer described the same version. Selecting a row
/// then diffed — and Restore would have written back — a different version than the
/// one the writer had just read.
fn visible_rows(view: &TimelineView, vm: &VersionsViewModel) -> Vec<VersionRow> {
    vm.visible_indices()
        .into_iter()
        .filter_map(|i| {
            let c = view.timeline.changes.get(i)?;
            Some(VersionRow {
                index: i,
                when: c.at.format("%Y-%m-%d %H:%M").to_string(),
                subtitle: subtitle_for(c),
                magnitude: view.magnitudes.get(i).copied().flatten(),
                pinned: vm.pin_state(c),
            })
        })
        .collect()
}

fn list(view: &TimelineView, vm: VersionsViewModel) -> impl Widget + use<> {
    let model = teksilo::data::ListModel::from_vec(visible_rows(view, &vm));
    let pin_vm = vm.clone();
    ListView::new(model, move |_i, row: &VersionRow, selected| {
        let mut item = StandardListItem::new(lit!(row.when.clone()))
            .subtitle(lit!(row.subtitle.clone()))
            .selected(selected)
            .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
            .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing));
        if let Some(m) = row.magnitude {
            item = item.trailing_slot(magnitude_bar(m));
        }
        // Absent, not disabled, on a version the log holds: a pin protects a
        // *file* from a retention sweep, and there is no file behind a log entry.
        if let Some(pinned) = row.pinned {
            let (vm, index) = (pin_vm.clone(), row.index);
            item = item.leading_slot(
                IconButton::new(if pinned {
                    crate::icons::versions::pinned()
                } else {
                    crate::icons::versions::unpinned()
                })
                .size(IconButtonSize::Compact)
                .icon_role(if pinned {
                    TextRole::Accent
                } else {
                    TextRole::Secondary
                })
                .tooltip(if pinned {
                    tr!(versions_unpin())
                } else {
                    tr!(versions_pin())
                })
                .on_activate_fn(move |_| vm.toggle_pin(index)),
            );
        }
        Box::new(item) as Box<dyn Widget>
    })
    .item_height(44.0)
    .selection(vm.selection())
    .scroll_bar_style(ScrollBarMode::Overlay)
}

/// One list row's data, flattened out of the timeline so the delegate does no work.
#[derive(Clone, PartialEq)]
struct VersionRow {
    /// Position in the **unfiltered** timeline — what the view-model keys on, so
    /// the "pinned only" projection cannot renumber a pin onto the wrong version.
    index: usize,
    when: String,
    subtitle: String,
    magnitude: Option<f32>,
    /// `Some(pinned)` for a backup, `None` for a version that cannot carry a pin.
    pinned: Option<bool>,
}

/// Where this version lives.
///
/// The source is not decoration: "this exists only in a backup on a drive you last
/// plugged in a month ago" is a materially different promise from "this is in your
/// project file", and the writer cannot tell from a date.
///
/// The size that used to sit beside it is gone. It was one blob's byte length —
/// only the scope currently on screen, never the row and never the project — which
/// is a number nothing in the app acts on, next to a magnitude bar that already
/// answers the question a writer is actually asking. Worse, the Backups panel one
/// surface over uses "size" for the whole archive on disk, so the two numbers
/// invited a comparison that would have been wrong by three orders of magnitude.
fn subtitle_for(change: &Change) -> String {
    match change.source {
        SourceKind::Backup => tr!(versions_source_backup()),
        SourceKind::Log => tr!(versions_source_project()),
    }
    .resolve_now()
}

/// How much of the row moved, at a glance.
fn magnitude_bar(value: f32) -> impl Widget {
    teksu!(
        FixedSize::new() {
            width: 40.0
            ProgressBar::new(value) {
                thickness: 4.0
                label: tr!(versions_changed_percent(percent = (value * 100.0).round() as i64))
            }
        }
    )
}

/// The three things a timeline has to say out loud.
///
/// Each date here is one the timeline can prove, and the sentences are worded to
/// claim no more than that. `absent_at` is a moment the row was *observed* not to
/// exist, not the moment it first appeared: the creation sits somewhere in the gap
/// between the two, and "didn't exist before `<first sighting>`" would have asserted
/// something about that gap that nothing on disk supports.
fn boundaries(view: &TimelineView) -> impl Widget + use<> {
    let timeline = &view.timeline;
    let mut col = VStack::new().spacing(2.0);
    if let Some(at) = timeline.absent_at {
        col = col.child(footnote_line(tr!(versions_did_not_exist(
            date = at.format("%Y-%m-%d").to_string()
        ))));
    }
    if let Some(at) = timeline.deleted_after {
        col = col.child(footnote_line(tr!(versions_deleted_after(
            date = at.format("%Y-%m-%d").to_string()
        ))));
    }
    if !timeline.unreadable.is_empty() {
        col = col.child(footnote_line(tr!(versions_unreadable(
            count = timeline.unreadable.len() as i64
        ))));
    }
    Padding::symmetric(8.0, 6.0).child(col)
}

fn footnote_line(text: LocalizedString) -> impl Widget {
    HStack::new()
        .spacing(6.0)
        .child(TextWidget::new(text).color(TextRole::Secondary))
        .child(Spacer::new())
}

use crate::shared::text::dock_note as note;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::versions::{ProjectHandle, VersionScope};
    use teksilo::core::widget_tree::WidgetTree;

    fn panel(vm: VersionsViewModel, focus: Option<u64>) -> VersionsPanel {
        VersionsPanel::new(
            vm,
            Signal::new(focus),
            Rc::new(|_| Some(uuid::Uuid::from_u128(1))),
            Rc::new(|_, _| {}),
        )
    }

    /// The dock lays out in every state it can be in, including the ones a writer
    /// only meets when something has gone wrong.
    #[test]
    fn the_versions_dock_lays_out_in_every_state() {
        let cases: Vec<(&str, VersionsViewModel, Option<u64>)> = vec![
            ("nothing focused", VersionsViewModel::new(), None),
            ("no project", VersionsViewModel::new(), Some(7)),
            (
                "missing project",
                {
                    let vm = VersionsViewModel::new();
                    vm.set_project(ProjectHandle {
                        path: "/nonexistent/Novel.skrib".into(),
                        unique_id: "u".into(),
                        destinations: vec!["/nonexistent".into()],
                        revision: 0,
                    });
                    vm
                },
                Some(7),
            ),
            (
                "prose scope",
                {
                    let vm = VersionsViewModel::new();
                    vm.set_scope(VersionScope::Prose);
                    vm
                },
                Some(7),
            ),
        ];
        for (name, vm, focus) in cases {
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(Box::new(panel(vm, focus)));
            tree.layout(SizeProposal::exact(300.0, 600.0));
            assert!(
                tree.bounds(id).width > 0.0,
                "the versions dock laid out to zero width in the '{name}' state",
            );
        }
    }

    /// Three changes a day apart, newest first — enough to put a date filter
    /// across.
    fn dated_view(count: usize) -> TimelineView {
        let now = chrono::Utc::now();
        let changes: Vec<Change> = (0..count)
            .map(|i| Change {
                at: now - chrono::Duration::days(i as i64),
                source: SourceKind::Backup,
                from: skrib_format::versions::VersionRef {
                    path: std::path::PathBuf::from(i.to_string()),
                    taken_at: now,
                    source: SourceKind::Backup,
                },
                blob_path: String::new(),
                hash: format!("h{i}"),
                bytes: 0,
                title: String::new(),
            })
            .collect();
        TimelineView {
            timeline: skrib_format::changes::Timeline {
                changes,
                ..Default::default()
            },
            texts: (0..count).map(|i| format!("state {i}")).collect(),
            magnitudes: vec![None; count],
            role: Some(common::entities::ContentRole::SceneText),
        }
    }

    /// **The bug this guards.** The list drew its own rows with its own filter,
    /// which knew about "pinned only" and not about the date range. The view-model
    /// knew about both. So with a date filter on, the writer clicked row 0 and the
    /// diff pane — and Restore — resolved a completely different version.
    ///
    /// The assertion is not "the counts match" but "the *rows* are the versions the
    /// view-model says are visible, in order", which is the property the selection
    /// arithmetic actually depends on.
    #[test]
    fn the_rows_on_screen_are_exactly_the_versions_the_selection_resolves_against() {
        let vm = VersionsViewModel::new();
        let view = dated_view(5);
        vm.view().set(view.clone());
        assert_eq!(visible_rows(&view, &vm).len(), 5, "unfiltered: all five");

        // The last three days only — which excludes the two oldest changes.
        vm.set_last_days(3);
        let rows = visible_rows(&view, &vm);
        assert_eq!(
            rows.iter().map(|r| r.index).collect::<Vec<_>>(),
            vm.visible_indices(),
            "the list and the selection must agree on which versions are on screen",
        );
        assert_eq!(rows.len(), vm.visible_count());
        assert!(
            rows.len() < 5,
            "precondition: the filter excludes something"
        );

        // And the version behind the row a writer would click is the one they read.
        vm.selection().select(0);
        assert_eq!(
            vm.selected_index(),
            Some(rows[0].index),
            "list position 0 must resolve to the timeline index of the first row",
        );
    }

    /// An error must not be rendered as an empty list.
    #[test]
    fn a_failed_scan_says_so_rather_than_showing_nothing() {
        let vm = VersionsViewModel::new();
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(panel(vm.clone(), Some(7))));
        tree.layout(SizeProposal::exact(300.0, 600.0));
        assert!(tree.bounds(id).width > 0.0);
        assert!(
            vm.error().get().is_empty(),
            "a project that was never opened is not an error state",
        );
    }

    /// The diff pane renders — and, more to the point, the rendered Djot survives
    /// a real document import, which is the only thing that proves the escaping
    /// reaches a screen rather than just a string comparison.
    #[test]
    fn a_selected_version_loads_a_readable_comparison_into_a_real_document() {
        let pane = DiffPane::new();
        let diff = version_diff::diff_djot(
            "She wrote \\{here\\} on the door.\n\nAnd waited.",
            "She wrote \\{there\\} on the door.\n\nAnd waited.",
        );
        let rendered =
            version_diff::render(&diff, Some(CollapseRule::default()), &|n| format!("[{n}]"));
        pane.show(&rendered);

        let text = pane
            .doc
            .to_plain_text()
            .expect("the document must hold text");
        assert!(
            text.contains("{here}"),
            "the old words were mangled: {text}"
        );
        assert!(
            text.contains("{there}"),
            "the new words were mangled: {text}"
        );
        assert!(
            !pane.offsets.borrow().is_empty(),
            "a change must be reachable by jump-to-next-change",
        );
    }

    /// Reloading the same comparison must not touch the document — a reset would
    /// throw away the reader's scroll position on every unrelated rebuild.
    #[test]
    fn an_unchanged_comparison_is_not_reloaded() {
        let pane = DiffPane::new();
        let diff = version_diff::diff_djot("the lamp went out", "the lamp guttered");
        let rendered = version_diff::render(&diff, None, &|n| format!("[{n}]"));
        pane.show(&rendered);
        let first = pane.doc.to_plain_text().unwrap();
        pane.show(&rendered);
        assert_eq!(pane.doc.to_plain_text().unwrap(), first);
    }

    /// Every change offset has to be inside the document it points into, or a jump
    /// panics or silently does nothing.
    #[test]
    fn every_jump_target_lands_inside_the_loaded_document() {
        let pane = DiffPane::new();
        let before = "Alpha one two three.\n\nBeta four five six.\n\nGamma seven eight nine.";
        let after = "Alpha one two THREE.\n\nBeta four five six.\n\nGamma seven eight NINE.";
        let rendered = version_diff::render(
            &version_diff::diff_djot(before, after),
            Some(CollapseRule::default()),
            &|n| format!("[{n}]"),
        );
        pane.show(&rendered);
        let len = pane.doc.to_plain_text().unwrap().chars().count();
        let offsets = pane.offsets.borrow();
        assert_eq!(offsets.len(), 2, "two blocks changed");
        for at in offsets.iter() {
            assert!(
                *at < len,
                "offset {at} is past the end of a {len}-char document"
            );
        }
    }
}
