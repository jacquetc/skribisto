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
mod tests {
    use super::*;

    #[test]
    fn synopsis_leads_because_it_can_be_read_at_a_glance() {
        assert_eq!(VersionScope::default(), VersionScope::Synopsis);
        assert_eq!(VersionScope::Synopsis.roles(), &[ContentRole::SynopsisText]);
    }

    #[test]
    fn the_body_scope_covers_both_a_scene_and_a_note() {
        // The dock must not need to know which kind of row it is showing.
        let roles = VersionScope::Prose.roles();
        assert!(roles.contains(&ContentRole::SceneText));
        assert!(roles.contains(&ContentRole::NoteText));
    }

    #[test]
    fn nothing_focused_clears_the_timeline_without_touching_the_disk() {
        let vm = VersionsViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/definitely/not/a/project.skrib".into(),
            ..Default::default()
        });
        vm.load_for(None);
        assert!(vm.view().get().is_empty());
        assert!(vm.error().get().is_empty(), "no target is not an error");
        assert!(!vm.loading().get());
    }

    #[test]
    fn an_unsaved_project_reports_no_past_rather_than_an_error() {
        // A project that has never been written has nowhere to have kept history,
        // which is a normal state and must not read as a failure.
        let vm = VersionsViewModel::new();
        vm.set_project(ProjectHandle::default());
        vm.load_for(Some(uuid::Uuid::from_u128(1)));
        assert!(vm.view().get().is_empty());
        assert!(vm.error().get().is_empty());
        assert!(!vm.loading().get());
    }

    #[test]
    fn a_missing_project_file_yields_an_empty_timeline_synchronously() {
        // With no executor the scan runs inline, so a headless caller gets a real
        // answer instead of a permanent "loading" — the same degradation
        // `BackupsListViewModel` relies on.
        let vm = VersionsViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        });
        vm.load_for(Some(uuid::Uuid::from_u128(1)));
        assert!(!vm.loading().get(), "the load must have completed inline");
        assert!(vm.view().get().is_empty());
    }

    /// The dock calls `load_for` from `build`, and a load writes signals `build`
    /// binds. Without the guard the first scan schedules the second, forever.
    #[test]
    fn repeating_a_load_for_the_same_row_does_not_scan_again() {
        let vm = VersionsViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        });
        let uid = Some(uuid::Uuid::from_u128(1));
        vm.load_for(uid);
        let first = vm.view().get();
        // A second call must not even reach `loading`, which is the signal that
        // would retrigger the build that called it.
        vm.load_for(uid);
        assert!(!vm.loading().get());
        assert!(vm.view().get() == first);
    }

    /// **The bug this guards.** A backup taken mid-session writes a new version
    /// of every row, and nothing about *where* the project lives changes — so a
    /// dock that had already loaded kept showing its stale answer. It only ever
    /// came right by accident, when a tab switch changed the other half of the
    /// key.
    #[test]
    fn a_newly_recorded_version_makes_the_dock_look_again() {
        let vm = VersionsViewModel::new();
        let base = ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        };
        vm.set_project(base.clone());
        let uid = Some(uuid::Uuid::from_u128(1));
        vm.load_for(uid);
        assert!(
            vm.loaded.borrow().is_some(),
            "precondition: the first load ran",
        );

        // Same row, same scope, same project — and the guard must still let this
        // one through, because the past itself moved.
        vm.set_project(ProjectHandle {
            revision: 1,
            ..base.clone()
        });
        let before = vm.loaded.borrow().clone();
        vm.load_for(uid);
        assert!(
            before != *vm.loaded.borrow(),
            "a recorded version has to invalidate the load key, not just the path",
        );
    }

    #[test]
    fn changing_the_scope_does_start_a_new_scan() {
        let vm = VersionsViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        });
        let uid = Some(uuid::Uuid::from_u128(1));
        vm.load_for(uid);
        vm.set_scope(VersionScope::Prose);
        vm.load_for(uid);
        // Reaching the end without the guard swallowing it is the assertion; the
        // scan itself finds nothing, which the previous test already covers.
        assert!(vm.error().get().is_empty());
    }

    // ── diffing ─────────────────────────────────────────────────────────────

    /// A hand-built view, so the diff logic is testable without a filesystem.
    ///
    /// Each change's `from.path` is its own index, so a stub pin store can answer
    /// "is this one pinned" without touching a disk.
    fn view_with(texts: &[&str], absent_at: Option<chrono::DateTime<chrono::Utc>>) -> TimelineView {
        let now = chrono::Utc::now();
        let changes: Vec<Change> = (0..texts.len())
            .map(|i| Change {
                at: now - chrono::Duration::minutes(i as i64),
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
            timeline: Timeline {
                absent_at,
                changes,
                ..Default::default()
            },
            texts: texts.iter().map(|t| t.to_string()).collect(),
            magnitudes: Vec::new(),
            role: Some(ContentRole::SceneText),
        }
    }

    #[test]
    fn a_version_is_compared_with_the_one_before_it_not_with_the_current_text() {
        // Newest first, so index 1 is what index 0 replaced.
        let view = view_with(&["the lamp guttered", "the lamp went out"], None);
        assert_eq!(view.predecessor(0), Some("the lamp went out"));
    }

    /// The trap this guards: the oldest recorded state is not necessarily the
    /// moment the row was written.
    #[test]
    fn the_earliest_state_is_not_reported_as_newly_written_unless_it_was() {
        let unknown = view_with(&["the lamp went out"], None);
        assert_eq!(
            unknown.predecessor(0),
            None,
            "with nothing older on record, a diff would be a guess",
        );

        let known = view_with(&["the lamp went out"], Some(chrono::Utc::now()));
        assert_eq!(
            known.predecessor(0),
            Some(""),
            "a version in which the row was absent proves it was created here",
        );
    }

    #[test]
    fn selecting_a_version_produces_a_diff_against_its_predecessor() {
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        vm.view
            .set(view_with(&["the lamp guttered", "the lamp went out"], None));
        vm.selection.select(0);
        vm.sync_diff();

        let diff = vm.diff().get().expect("a version with a predecessor diffs");
        assert!(diff.summary.words_added > 0);
        assert!(diff.summary.words_removed > 0);
        assert!(!vm.selection_is_earliest());
    }

    #[test]
    fn the_earliest_version_offers_no_diff_and_says_which_state_it_is_in() {
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        vm.view.set(view_with(&["the lamp went out"], None));
        vm.selection.select(0);
        vm.sync_diff();

        assert!(vm.diff().get().is_none());
        assert!(
            vm.selection_is_earliest(),
            "the pane must be able to tell 'nothing older' from 'nothing changed'",
        );
    }

    #[test]
    fn repeating_the_diff_for_the_same_selection_recomputes_nothing() {
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        vm.view
            .set(view_with(&["the lamp guttered", "the lamp went out"], None));
        vm.selection.select(0);
        vm.sync_diff();
        let first = vm.diff().get();
        vm.sync_diff();
        assert!(vm.diff().get() == first);
    }

    /// **The bug this guards.** With "pinned only" on, the list is a projection:
    /// the cursor's position in it is not the position in the timeline. Read
    /// straight through, selecting the first *visible* row diffed — and would
    /// have restored — a completely different version's text than the one on
    /// screen.
    #[test]
    fn a_filtered_list_still_selects_the_version_the_writer_can_see() {
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        // Newest first; only the third is pinned.
        vm.view
            .set(view_with(&["newest", "middle", "oldest"], None));
        let pinned = std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, true]));
        {
            let by_path = pinned.clone();
            vm.set_pins(Pins {
                is_pinned: Rc::new(move |p: &std::path::Path| {
                    let i: usize = p.to_string_lossy().parse().unwrap_or(0);
                    by_path.borrow().get(i).copied().unwrap_or(false)
                }),
                set: Rc::new(|_, _| {}),
            });
        }

        assert_eq!(vm.visible_indices(), vec![0, 1, 2], "unfiltered: all three");
        vm.toggle_pinned_only();
        assert_eq!(
            vm.visible_indices(),
            vec![2],
            "filtered: only the pinned one is on screen",
        );

        // The writer selects the one row they can see — list position 0.
        vm.selection.select(0);
        assert_eq!(
            vm.selected_index(),
            Some(2),
            "position 0 of a filtered list is the *third* version, not the first",
        );
        let req = vm
            .restore_request(7)
            .expect("the selected version can be restored");
        assert_eq!(
            req.past, "oldest",
            "restore must write back the version the writer actually looked at",
        );
    }

    /// The same trap as "pinned only", one filter later: a date range is another
    /// projection, and everything acting on the selection has to come back
    /// through it or Restore writes a version the writer never looked at.
    #[test]
    fn a_date_filtered_list_still_restores_the_version_on_screen() {
        use teksilo::widgets::DateRange;
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        // `view_with` dates them one minute apart, newest first.
        vm.view
            .set(view_with(&["newest", "middle", "oldest"], None));
        assert_eq!(vm.visible_count(), 3);

        // A range covering only the oldest — two minutes back, one day wide.
        let oldest = vm.view.get().timeline.changes[2].at;
        let day = crate::date_convert::to_jiff_date(oldest).unwrap();
        vm.range().set(Some(DateRange::new(day, day)));

        // All three fall on the same day here, so narrow to a day that holds none
        // and check the dock is told, rather than shown an empty list.
        let elsewhere =
            crate::date_convert::to_jiff_date(oldest - chrono::Duration::days(30)).unwrap();
        vm.range().set(Some(DateRange::new(elsewhere, elsewhere)));
        assert_eq!(vm.visible_count(), 0, "the filter excludes every version");
        assert!(
            vm.is_filtered(),
            "and the dock can say why the list is empty"
        );
        assert_eq!(
            vm.selected_index(),
            None,
            "nothing on screen means nothing selected, so nothing to restore",
        );

        vm.clear_filters();
        assert_eq!(vm.visible_count(), 3);
        assert!(!vm.is_filtered());
    }

    /// The preset sets a real range, and — like every other filter here — drops
    /// a cursor that pointed into the unfiltered list.
    #[test]
    fn the_recent_preset_sets_a_range_and_clears_the_cursor() {
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        vm.view
            .set(view_with(&["newest", "middle", "oldest"], None));
        vm.selection.select(1);
        assert!(!vm.is_filtered());

        vm.set_last_days(30);
        let range = vm.range().get().expect("the preset sets a range");
        assert!(vm.is_filtered());
        assert!(
            vm.selection().selected_indices().is_empty(),
            "the list is about to renumber under a positional cursor",
        );

        // `view_with` dates its changes minutes ago, so all three are inside the
        // last thirty days — the preset narrows without hiding recent work.
        assert_eq!(vm.visible_count(), 3);
        let today = crate::date_convert::today_utc().expect("a sane clock");
        assert_eq!(range.end, today, "the window ends today");
        assert!(range.start < today, "and reaches back before it");
    }

    /// A filter that hides the selected row must not leave the cursor pointing at
    /// a version the writer can no longer see.
    #[test]
    fn narrowing_the_range_does_not_leave_a_hidden_version_selected() {
        use teksilo::widgets::DateRange;
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        vm.view
            .set(view_with(&["newest", "middle", "oldest"], None));
        vm.selection.select(2);
        assert_eq!(vm.selected_index(), Some(2));

        let far = crate::date_convert::to_jiff_date(
            vm.view.get().timeline.changes[0].at - chrono::Duration::days(365),
        )
        .unwrap();
        vm.range().set(Some(DateRange::new(far, far)));
        assert_eq!(
            vm.selected_index(),
            None,
            "position 2 of an empty projection is not a version",
        );
        assert!(
            vm.restore_request(7).is_none(),
            "and there is nothing to put back"
        );
    }

    #[test]
    fn clearing_the_selection_clears_the_diff() {
        let vm = VersionsViewModel::new();
        *vm.loaded.borrow_mut() = Some(LoadKey {
            uid: Some(uuid::Uuid::from_u128(1)),
            scope: 0,
            project: ProjectHandle::default(),
        });
        vm.view
            .set(view_with(&["the lamp guttered", "the lamp went out"], None));
        vm.selection.select(0);
        vm.sync_diff();
        assert!(vm.diff().get().is_some());

        vm.selection.clear();
        vm.sync_diff();
        assert!(vm.diff().get().is_none());
    }

    /// Loading a different row must not leave the previous row's cursor behind:
    /// index 3 of one timeline is not index 3 of another.
    #[test]
    fn loading_another_row_drops_the_previous_selection_and_diff() {
        let vm = VersionsViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        });
        vm.load_for(Some(uuid::Uuid::from_u128(1)));
        vm.selection.select(0);
        vm.load_for(Some(uuid::Uuid::from_u128(2)));
        assert!(vm.selection().selected_indices().is_empty());
        assert!(vm.diff().get().is_none());
    }

    #[test]
    fn magnitudes_line_up_with_the_changes_they_describe() {
        let view = measure_texts(&["a b c d e f g h", "a b c d e f g X", "a b c d e f g h"]);
        assert_eq!(view.magnitudes.len(), 3);
        assert!(
            view.magnitudes[0].is_some_and(|m| m > 0.0),
            "one word changed against the state below it",
        );
        assert_eq!(
            view.magnitudes[2], None,
            "the oldest state has nothing behind it to measure against",
        );
    }

    /// `measure` without touching a filesystem: the reading half is exercised by
    /// the timeline tests, the comparing half is what matters here.
    fn measure_texts(texts: &[&str]) -> TimelineView {
        let mut view = TimelineView {
            timeline: Timeline::default(),
            texts: texts.iter().map(|t| t.to_string()).collect(),
            magnitudes: Vec::new(),
            role: Some(ContentRole::SceneText),
        };
        view.magnitudes = (0..view.texts.len())
            .map(|i| {
                view.predecessor(i)
                    .map(|older| diff_djot(older, &view.texts[i]).magnitude())
            })
            .collect();
        view
    }
}
