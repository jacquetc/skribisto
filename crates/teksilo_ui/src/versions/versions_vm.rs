// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `VersionsViewModel` — one row's recorded past, for the Versions dock.
//!
//! Scoped to whatever the trailing rail is already scoped to: the active editor
//! tab's item, the same `EditorsViewModel::active_item()` signal the Inspector,
//! the per-document comments dock and the footnotes dock all take. "This document"
//! means the same thing on every trailing tab or it means nothing.
//!
//! ## Why a list, and not the slider first
//!
//! The obvious shape for time is a slider, and it is the wrong primary control
//! here. Versions are discrete and unevenly spaced — the condition usability
//! guidance names as disqualifying for sliders — and the labels that make them
//! meaningful (a date, which content changed, how much) do not fit under a thumb.
//! Every scrubber that shipped successfully also ships a discrete, non-drag way to
//! reach the same states; here the list *is* that way, and a slider can sit beside
//! it later as a scrub affordance bound to this same selection.
//!
//! ## Threading, and why the prose is read up front
//!
//! Scanning destinations and opening archives is filesystem work, so it runs
//! through `spawn_blocking` on the shared executor and lands in signals — the same
//! shape `BackupsListViewModel::reload` uses, including its degradation to a
//! synchronous scan when no executor is reachable, so a headless test still gets a
//! real answer instead of a permanent "loading".
//!
//! That one pass also reads each change's prose and compares adjacent pairs. It
//! costs a handful of small reads — the timeline holds only the moments a row
//! *changed*, not every backup — and it buys two things worth more than the reads:
//! the magnitude every list row shows, and a diff pane that opens instantly
//! instead of going to disk on every click.
//!
//! Not a Layer-A model: this touches no store entity and needs no real/mock `mod
//! imp` split. Under `--features mocks` the sources simply find no files and every
//! surface shows its empty state, which is the honest answer for a build with no
//! backend.

use std::cell::RefCell;
use std::rc::Rc;

use skrib_format::changes::{Timeline, timeline_for};
use skrib_format::versions::{BackupVersions, LogVersions, SourceKind, VersionSource};
use teksilo::data::{SelectionMode, SelectionModel};
use teksilo::prelude::{AsyncRuntimeHandle, Signal, spawn_blocking};
use teksilo::widgets::DateRange;

use common::entities::ContentRole;

use super::version_diff::{VersionDiff, diff_djot};
use skrib_format::changes::Change;

/// Reading and writing a backup's pin.
///
/// Two closures rather than a handle to the backup settings: pins live in an
/// app-level settings file, and a view-model that reached for a peer would close
/// exactly the dependency the layer's rules exist to keep open. Supplied by
/// `app::project_shell`.
/// Reads whether a backup file is pinned.
pub type IsPinned = Rc<dyn Fn(&std::path::Path) -> bool>;
/// Pins or unpins a backup file.
pub type SetPinned = Rc<dyn Fn(&std::path::Path, bool)>;

#[derive(Clone)]
pub struct Pins {
    pub is_pinned: IsPinned,
    pub set: SetPinned,
}

/// Which prose of a row the dock is showing history for.
///
/// Every writing row owns a main text *and* a synopsis, so a timeline is only
/// meaningful once you say which. The synopsis leads because it is short enough to
/// read at a glance — a 3,000-word scene is a wall, and the row you are checking
/// is usually the one you just changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VersionScope {
    #[default]
    Synopsis,
    Prose,
}

impl VersionScope {
    /// The content role this scope reads, for a row of the given kind.
    ///
    /// A Note's body is `NoteText` and a Scene's is `SceneText`; the dock does not
    /// need to know which kind of row it is looking at, only which content the
    /// row actually recorded — so both are offered and whichever the timeline has
    /// is used.
    pub fn roles(self) -> &'static [ContentRole] {
        match self {
            VersionScope::Synopsis => &[ContentRole::SynopsisText],
            VersionScope::Prose => &[ContentRole::SceneText, ContentRole::NoteText],
        }
    }
}

/// What the dock needs to know about the project to find its past.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct ProjectHandle {
    /// Absolute path of the `.skrib` (or its folder). Empty when nothing is open.
    pub path: String,
    pub unique_id: String,
    /// Resolved backup destinations — see `crate::backup_paths`.
    pub destinations: Vec<String>,
    /// How many times this session has recorded a new version of the project: a
    /// completed backup, or a save (which appends to the in-project history log).
    ///
    /// Not a location, and the one field here that is not about *where* to look.
    /// It is here because every cache in this feature is keyed on the handle, and
    /// a new version has to invalidate them exactly as a new project does.
    /// Without it, "Back up now" wrote a version that no open pane would look at
    /// again — the Versions dock only noticed on the next tab switch (which
    /// changes its other key), and the Timeline band, whose only key is this
    /// handle, never noticed at all.
    ///
    /// Bumped by `app::project_shell`, which is the one place that can see both
    /// the backup scheduler and the save state.
    pub revision: u64,
}

/// A row's timeline, with everything the dock needs to draw it.
#[derive(Clone, Default, PartialEq)]
pub struct TimelineView {
    pub timeline: Timeline,
    /// Each change's prose, aligned with `timeline.changes`. An entry is empty
    /// when that moment's blob could not be read after all.
    pub texts: Vec<String>,
    /// How much each change moved, in `0.0..=1.0`.
    ///
    /// `None` for the **oldest** recorded state when nothing older is known: the
    /// row may well have existed before the earliest backup, and drawing a full
    /// bar there would claim an edit that was never observed.
    pub magnitudes: Vec<Option<f32>>,
    /// The content role this timeline is actually of.
    ///
    /// A scope offers several roles and the first one with anything in it wins,
    /// so which it was is not derivable from the scope afterwards — and a restore
    /// has to name the role the version recorded, not the one the caller hoped
    /// for. `None` when nothing was found for any of them.
    pub role: Option<ContentRole>,
}

impl TimelineView {
    pub fn is_empty(&self) -> bool {
        self.timeline.is_empty()
    }

    /// The prose immediately older than `index`, and whether it is a real
    /// predecessor rather than "nothing was there".
    ///
    /// Returns `None` when `index` is the oldest recorded state and the row's
    /// creation was never observed — the one case where a diff would be a guess.
    fn predecessor(&self, index: usize) -> Option<&str> {
        match self.texts.get(index + 1) {
            Some(older) => Some(older.as_str()),
            // Past the oldest change: only a version in which the row was
            // *absent* proves it was created here.
            None => self.timeline.absent_at.is_some().then_some(""),
        }
    }
}

/// The identity of a load, so a rebuild does not start the same scan again.
#[derive(Clone, PartialEq, Eq)]
struct LoadKey {
    uid: Option<uuid::Uuid>,
    scope: usize,
    project: ProjectHandle,
}

#[derive(Clone)]
pub struct VersionsViewModel {
    view: Signal<TimelineView>,
    loading: Signal<bool>,
    /// Non-empty when the last load failed outright, for the dock to show instead
    /// of an empty list — "we could not look" is not "there is nothing".
    error: Signal<String>,
    /// The segmented control's index, and the only home of the scope.
    ///
    /// `SegmentedControl` writes its own `Signal<usize>` rather than emitting a
    /// change callback, so giving it this one directly avoids a second signal
    /// mirrored back through an effect — two signals that can disagree, and an
    /// effect re-registered on every rebuild.
    scope_index: Signal<usize>,
    /// Which version's diff is shown. Held here rather than built in the dock so
    /// the highlight survives a rebuild.
    selection: SelectionModel,
    diff: Signal<Option<VersionDiff>>,
    /// Whether the diff pane shows the untouched middle of the row.
    show_unchanged: Signal<bool>,
    /// Narrow the list to versions recorded between two dates.
    ///
    /// A calendar range, because the widget speaks `jiff::civil::Date` and a
    /// writer thinks in days. The list is virtualised so length is not the
    /// problem here that it is in the Timeline band — reaching last March in two
    /// hundred rows is.
    range: Signal<Option<DateRange>>,
    /// Show only the versions the writer pinned.
    ///
    /// The mechanic that makes an automatic stream navigable: a timeline is a
    /// record of everything, and "the three I marked" is a different list from
    /// "the forty that happened".
    pinned_only: Signal<bool>,
    /// Reads and writes the pin on a backup file. Supplied by the shell, because
    /// pins live in the app's backup settings and a view-model does not import a
    /// peer.
    pins: Rc<RefCell<Option<Pins>>>,
    /// What the timeline currently in `view` was loaded for.
    ///
    /// Load-bearing: the dock calls [`Self::load_for`] from `build`, and `build`
    /// re-runs whenever any signal it binds changes — including the ones a load
    /// writes. Without this the first scan would schedule the second.
    loaded: Rc<RefCell<Option<LoadKey>>>,
    /// Which change `diff` was computed for, to the same end.
    diffed: Rc<RefCell<Option<(LoadKey, usize)>>>,
    /// Where this project's past is kept.
    ///
    /// A **signal**, not a snapshot, and that is not decoration: `WorkInfo` is
    /// filled in by an event that lands *after* the project shell is built, so a
    /// value read at build time is `None` and stays `None` for the session. The
    /// dock binds this, so the path arriving is what makes the first scan run.
    project: Signal<ProjectHandle>,
    async_rt: Rc<RefCell<Option<AsyncRuntimeHandle>>>,
}

impl Default for VersionsViewModel {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionsViewModel {
    pub fn new() -> Self {
        Self {
            view: Signal::new(TimelineView::default()),
            loading: Signal::new(false),
            error: Signal::new(String::new()),
            scope_index: Signal::new(0),
            // Single: a diff compares one version with the one before it, so a
            // multi-select cursor would have nothing to mean.
            selection: SelectionModel::new(SelectionMode::Single),
            diff: Signal::new(None),
            show_unchanged: Signal::new(false),
            pinned_only: Signal::new(false),
            range: Signal::new(None),
            pins: Rc::new(RefCell::new(None)),
            loaded: Rc::new(RefCell::new(None)),
            diffed: Rc::new(RefCell::new(None)),
            project: Signal::new(ProjectHandle::default()),
            async_rt: Rc::new(RefCell::new(None)),
        }
    }

    // ── view handles ────────────────────────────────────────────────────────

    pub fn view(&self) -> Signal<TimelineView> {
        self.view.clone()
    }

    pub fn timeline(&self) -> Timeline {
        self.view.get().timeline
    }

    pub fn loading(&self) -> Signal<bool> {
        self.loading.clone()
    }

    pub fn error(&self) -> Signal<String> {
        self.error.clone()
    }

    /// Bind this straight into a `SegmentedControl`.
    pub fn scope_index(&self) -> Signal<usize> {
        self.scope_index.clone()
    }

    pub fn selection(&self) -> SelectionModel {
        self.selection.clone()
    }

    pub fn diff(&self) -> Signal<Option<VersionDiff>> {
        self.diff.clone()
    }

    pub fn show_unchanged(&self) -> Signal<bool> {
        self.show_unchanged.clone()
    }

    pub fn toggle_unchanged(&self) {
        let now = self.show_unchanged.get();
        self.show_unchanged.set(!now);
    }

    pub fn pinned_only(&self) -> Signal<bool> {
        self.pinned_only.clone()
    }

    pub fn toggle_pinned_only(&self) {
        let now = self.pinned_only.get();
        self.pinned_only.set(!now);
        // The cursor is positional and the list is about to renumber.
        self.selection.clear();
    }

    /// Install the pin store (once, from the shell).
    pub fn set_pins(&self, pins: Pins) {
        let mut slot = self.pins.borrow_mut();
        if slot.is_none() {
            *slot = Some(pins);
        }
    }

    /// Whether a change can be pinned at all, and whether it is.
    ///
    /// Only a **backup** can be: a pin protects a file from a retention sweep,
    /// and a log version is not a file — it is an entry inside the project
    /// itself, thinned by a different mechanism at save time. Offering a control
    /// that silently did nothing on half the rows would be worse than not
    /// offering it there.
    pub fn pin_state(&self, change: &Change) -> Option<bool> {
        if change.source != SourceKind::Backup {
            return None;
        }
        let pins = self.pins.borrow();
        let pins = pins.as_ref()?;
        Some((pins.is_pinned)(&change.from.path))
    }

    /// Whether the list on screen holds a version that cannot carry a pin.
    ///
    /// What the panel's pin note answers to. [`Self::pin_state`] leaves a log row
    /// with no control at all — the right call, since a disabled one invites the
    /// question "why?" without answering it — but the absence is then the *only*
    /// thing saying so, and an absence teaches nothing. Meanwhile the tooltip on
    /// the rows that do have one promises automatic cleanup will never delete the
    /// version, and the toolbar offers "show only pinned", so both surfaces teach
    /// that a version is a pinnable thing. A writer who pins the wording they love
    /// and then finds no pin on yesterday's rows concludes the feature is broken.
    ///
    /// Keyed on the **source**, not on `pin_state`, and deliberately: `pin_state`
    /// is also `None` before the shell has installed the store, and the note would
    /// then appear over a list of backups where every pin is merely late.
    pub fn shows_unpinnable(&self) -> bool {
        let view = self.view.get();
        self.visible_indices()
            .into_iter()
            .filter_map(|i| view.timeline.changes.get(i))
            .any(|c| c.source != SourceKind::Backup)
    }

    /// Whether "pinned only" is what emptied the list, with nothing pinned to find.
    ///
    /// Separated from the general "the filters exclude everything" case because
    /// the two need different sentences. A date range that matches nothing is a
    /// range the writer chose and can widen; an empty pinned list is a filter
    /// nothing in this row's past satisfies, and saying only "no version matches
    /// the filters you've set" invites them to go looking for the pins they think
    /// they made.
    pub fn pinned_filter_found_nothing(&self) -> bool {
        if !self.pinned_only.get() {
            return false;
        }
        !self
            .view
            .get()
            .timeline
            .changes
            .iter()
            .any(|c| self.pin_state(c) == Some(true))
    }

    /// Whether this row's past holds a version no pin can reach.
    ///
    /// Distinct from [`Self::shows_unpinnable`], which asks about the rows
    /// **on screen**: under "pinned only" there are none, so it would answer no
    /// for every empty list. This asks about the whole timeline, which is what
    /// decides whether "nothing is pinned" needs a reason attached. On a list of
    /// backups nobody has pinned yet, the reason is true and irrelevant, and
    /// reads as an explanation for an emptiness it did not cause.
    pub fn has_unpinnable_versions(&self) -> bool {
        self.view
            .get()
            .timeline
            .changes
            .iter()
            .any(|c| c.source != SourceKind::Backup)
    }

    /// Seed a hand-built timeline, and mark it loaded so no scan is scheduled.
    ///
    /// `pub(crate)` and test-only: the dock's own render tests need a view the
    /// filesystem cannot supply — a row whose past has been thinned, a list
    /// mixing a backup with the project's own history — and `versions_vm`'s
    /// tests reach the private fields directly because they live inside it.
    #[cfg(test)]
    pub(crate) fn seed_for_test(&self, uid: uuid::Uuid, view: TimelineView) {
        *self.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uid),
            scope: self.scope_index.get(),
            project: self.project.get(),
        });
        self.view.set(view);
    }

    /// Bind this straight into a `DateRangeEdit`.
    pub fn range(&self) -> Signal<Option<DateRange>> {
        self.range.clone()
    }

    /// Whether anything is narrowing the list right now.
    pub fn is_filtered(&self) -> bool {
        self.pinned_only.get() || self.range.get().is_some()
    }

    /// Narrow to the last `days` days, ending today.
    ///
    /// The preset `DateRangeEdit` does not have: it carries no open-ended range,
    /// so "recently" can only be said by computing both ends. A no-op if the
    /// clock reports a date outside what the widget can hold, which is a corrupt
    /// system clock rather than anything a writer did.
    pub fn set_last_days(&self, days: u32) {
        let Some(today) = crate::date_convert::today_utc() else {
            return;
        };
        self.range
            .set(Some(crate::date_convert::last_days(today, days)));
        // The list is about to renumber under a positional cursor.
        self.selection.clear();
    }

    /// Drop every filter. The one control that has to exist once a filter can
    /// empty the list: an empty pane with no way out reads as lost history.
    pub fn clear_filters(&self) {
        if self.pinned_only.get() {
            self.pinned_only.set(false);
        }
        if self.range.get().is_some() {
            self.range.set(None);
        }
        self.selection.clear();
    }

    /// Whether `at` falls inside the date filter, if one is set.
    ///
    /// Both days inclusive: a writer who picks the 3rd to the 5th means the whole
    /// of the 5th, and `DateRangeEdit` only carries the day.
    fn in_range(&self, at: chrono::DateTime<chrono::Utc>) -> bool {
        let Some(range) = self.range.get() else {
            return true;
        };
        let Some(day) = crate::date_convert::to_jiff_date(at) else {
            // A date jiff cannot represent is a corrupt stamp, not a match.
            return false;
        };
        day >= range.start && day <= range.end
    }

    /// The timeline indices the list is currently showing, in list order.
    ///
    /// The list is a **projection**: "pinned only" and the date filter each show
    /// a subset, so the cursor's position in it is not the position in the
    /// timeline. Everything that acts on the selection — the diff, and above all
    /// the restore — has to come back through here first, or a writer who filters
    /// the list and then presses Restore gets a different version's text than the
    /// one they read.
    ///
    /// **Public because the dock has to build its rows from exactly this list.**
    /// It used to apply its own filter, and the two drifted: the dock filtered on
    /// "pinned only" alone while this also applied the date range, so with a date
    /// filter on, every row the writer clicked resolved to a *different* version —
    /// the one the restore would have written back. A projection with two
    /// definitions is a projection with none.
    pub fn visible_indices(&self) -> Vec<usize> {
        let view = self.view.get();
        let only_pinned = self.pinned_only.get();
        view.timeline
            .changes
            .iter()
            .enumerate()
            .filter(|(_, c)| !only_pinned || self.pin_state(c) == Some(true))
            .filter(|(_, c)| self.in_range(c.at))
            .map(|(i, _)| i)
            .collect()
    }

    /// How many rows the list is showing, after every filter.
    pub fn visible_count(&self) -> usize {
        self.visible_indices().len()
    }

    /// Where in the **timeline** the cursor is, translated out of the list's own
    /// positional selection. `None` when nothing is selected.
    pub fn selected_index(&self) -> Option<usize> {
        let at = self.selection.selected_indices().first().copied()?;
        self.visible_indices().get(at).copied()
    }

    /// Flip the pin on the change at `index`, if it is one that can carry one.
    ///
    /// `index` is a **timeline** index — the dock's rows carry it explicitly for
    /// that reason, rather than relying on their position in a filtered list.
    pub fn toggle_pin(&self, index: usize) {
        let view = self.view.get();
        let Some(change) = view.timeline.changes.get(index) else {
            return;
        };
        let Some(now) = self.pin_state(change) else {
            return;
        };
        if let Some(pins) = self.pins.borrow().as_ref() {
            (pins.set)(&change.from.path, !now);
        }
        // Nothing about the timeline changed, but every row's pin glyph and the
        // "pinned only" projection did.
        self.view.set(view);
    }

    pub fn scope(&self) -> VersionScope {
        match self.scope_index.get() {
            0 => VersionScope::Synopsis,
            _ => VersionScope::Prose,
        }
    }

    // ── wiring ──────────────────────────────────────────────────────────────

    /// Hand over the executor once it is reachable (the dock's first build).
    /// Idempotent, matching `BackupsListViewModel::set_async_runtime`.
    pub fn set_async_runtime(&self, rt: Option<AsyncRuntimeHandle>) {
        let mut slot = self.async_rt.borrow_mut();
        if slot.is_none() {
            *slot = rt;
        }
    }

    /// Bind this so the dock rebuilds when the project's identity lands.
    pub fn project(&self) -> Signal<ProjectHandle> {
        self.project.clone()
    }

    /// Tell the view-model which project it is reading the past of.
    ///
    /// Idempotent by value, so the shell may push on every build and on every
    /// change to the signals it is derived from without causing a rebuild storm.
    pub fn set_project(&self, handle: ProjectHandle) {
        if self.project.get() != handle {
            self.project.set(handle);
        }
    }

    pub fn set_scope(&self, scope: VersionScope) {
        let want = match scope {
            VersionScope::Synopsis => 0,
            VersionScope::Prose => 1,
        };
        if self.scope_index.get() != want {
            self.scope_index.set(want);
        }
    }

    /// Load the timeline for `uid`, or clear it when nothing is focused.
    ///
    /// Safe to call from `build`: a repeat call for the same row, scope and
    /// project does nothing at all.
    pub fn load_for(&self, uid: Option<uuid::Uuid>) {
        let key = LoadKey {
            uid,
            scope: self.scope_index.get(),
            project: self.project.get(),
        };
        if self.loaded.borrow().as_ref() == Some(&key) {
            return;
        }
        *self.loaded.borrow_mut() = Some(key.clone());
        // A new target invalidates the diff *and* the cursor: index 3 of the
        // previous row's timeline is not index 3 of this one.
        self.selection.clear();
        *self.diffed.borrow_mut() = None;
        self.diff.set(None);

        let Some(uid) = uid else {
            self.finish(TimelineView::default(), String::new());
            return;
        };
        if key.project.path.is_empty() {
            // An unsaved project has nowhere to have kept a past.
            self.finish(TimelineView::default(), String::new());
            return;
        }

        let roles: Vec<ContentRole> = self.scope().roles().to_vec();
        let handle = key.project.clone();
        let work = move || collect(&handle, uid, &roles);

        self.loading.set(true);
        let rt = self.async_rt.borrow().clone();
        match rt {
            Some(rt) => {
                let this = self.clone();
                rt.spawn_local(async move {
                    match spawn_blocking(work).await {
                        Ok(Ok(v)) => this.finish(v, String::new()),
                        Ok(Err(e)) => this.finish(TimelineView::default(), e),
                        Err(_) => this.finish(
                            TimelineView::default(),
                            "the scan did not finish".to_string(),
                        ),
                    }
                })
                .detach();
            }
            None => match work() {
                Ok(v) => self.finish(v, String::new()),
                Err(e) => self.finish(TimelineView::default(), e),
            },
        }
    }

    fn finish(&self, view: TimelineView, error: String) {
        self.view.set(view);
        self.error.set(error);
        self.loading.set(false);
    }

    /// Recompute the diff for the currently selected version, if it changed.
    ///
    /// Pure CPU over prose already in memory, so it runs inline — and, like
    /// [`Self::load_for`], is a no-op when nothing moved, because the dock calls
    /// it from `build`.
    pub fn sync_diff(&self) {
        let Some(key) = self.loaded.borrow().clone() else {
            return;
        };
        let Some(index) = self.selected_index() else {
            if self.diff.get().is_some() {
                *self.diffed.borrow_mut() = None;
                self.diff.set(None);
            }
            return;
        };
        if self.diffed.borrow().as_ref() == Some(&(key.clone(), index)) {
            return;
        }
        *self.diffed.borrow_mut() = Some((key, index));

        let view = self.view.get();
        let Some(newer) = view.texts.get(index) else {
            self.diff.set(None);
            return;
        };
        match view.predecessor(index) {
            Some(older) => self.diff.set(Some(diff_djot(older, newer))),
            // The earliest thing on record, with nothing behind it to compare
            // against. Saying so is the only honest answer; inventing an empty
            // predecessor would report the whole scene as written that day.
            None => self.diff.set(None),
        }
    }

    /// What restoring the selected version would ask for, for the live row
    /// `item_id`.
    ///
    /// `None` when nothing is selected or the timeline never resolved a role —
    /// there is no honest request to make in either case.
    pub fn restore_request(&self, item_id: u64) -> Option<super::RestoreRequest> {
        let index = self.selected_index()?;
        let view = self.view.get();
        let change = view.timeline.changes.get(index)?;
        Some(super::RestoreRequest {
            item_id,
            recorded_role: view.role.clone()?,
            past: view.texts.get(index)?.clone(),
            taken_at: change.at,
        })
    }

    /// Whether the selected version is the earliest on record with no observed
    /// predecessor — the state in which there is nothing to diff *and* nothing
    /// went wrong.
    pub fn selection_is_earliest(&self) -> bool {
        let Some(index) = self.selected_index() else {
            return false;
        };
        let view = self.view.get();
        index < view.texts.len() && view.predecessor(index).is_none()
    }
}

/// The blocking half: build both sources, merge their timelines, read the prose.
///
/// Tries each role in the scope and keeps the first that produced anything, so a
/// Scene and a Note both answer "the body" without the caller knowing which it is
/// holding.
fn collect(
    handle: &ProjectHandle,
    uid: uuid::Uuid,
    roles: &[ContentRole],
) -> Result<TimelineView, String> {
    let backups = BackupVersions {
        directories: handle.destinations.clone(),
        work_unique_id: handle.unique_id.clone(),
        project_path: handle.path.clone(),
    };
    let log = LogVersions::open(&handle.path);

    let mut best = Timeline::default();
    let mut best_role = None;
    for role in roles {
        let sources: [&dyn VersionSource; 2] = [&log, &backups];
        match timeline_for(&sources, uid, role) {
            Ok(t) => {
                if !t.is_empty() {
                    return Ok(measure(t, role.clone(), &backups, &log));
                }
                // Keep whatever boundaries a role reported even with no changes,
                // so "did not exist yet" still reaches the writer.
                if best.absent_at.is_none() && best.deleted_after.is_none() {
                    best = t;
                    best_role = Some(role.clone());
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(TimelineView {
        role: best_role,
        ..measure(
            best,
            roles.first().cloned().unwrap_or(ContentRole::SceneText),
            &backups,
            &log,
        )
    })
}

/// Read each change's prose and compare adjacent pairs.
fn measure(
    timeline: Timeline,
    role: ContentRole,
    backups: &BackupVersions,
    log: &LogVersions,
) -> TimelineView {
    let texts: Vec<String> = timeline
        .changes
        .iter()
        .map(|c| {
            let source: &dyn VersionSource = match c.source {
                SourceKind::Backup => backups,
                SourceKind::Log => log,
            };
            source.prose(&c.from, &c.blob_path).unwrap_or_default()
        })
        .collect();

    let mut view = TimelineView {
        timeline,
        texts,
        magnitudes: Vec::new(),
        role: Some(role),
    };
    view.magnitudes = (0..view.texts.len())
        .map(|i| {
            view.predecessor(i)
                .map(|older| diff_djot(older, &view.texts[i]).magnitude())
        })
        .collect();
    view
}

#[cfg(test)]
mod tests;
