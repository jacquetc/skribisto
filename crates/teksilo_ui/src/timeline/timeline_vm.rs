// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TimelineViewModel` — the **project-wide** past, and the only way back to a
//! row that no longer exists.
//!
//! The Versions dock answers "what did *this row* say before", and it can only
//! ever ask that of a row that is still there: it hangs off the focused editor
//! tab, and a deleted scene has no tab to focus. This answers the other question
//! — "what did the *project* look like then, and what has changed since" — and in
//! doing so becomes the one surface that reaches a row which was written,
//! recorded, and later deleted. Effectively a trash for what already left the
//! trash.
//!
//! ## What "changed" means here
//!
//! Every recorded moment is compared against **now**, by durable `uid`:
//!
//! * in the version, gone from the project → **removed**
//! * in the project, absent from the version → **added since**
//! * in both, different text → **changed**
//! * in both, same text, different position → **moved**
//!
//! Rows that match on all three are not listed at all. A project-wide list that
//! repeated every untouched scene would be the same failure the per-row timeline
//! avoids by collapsing unchanged states, one level up.
//!
//! ## What each source can honestly be asked
//!
//! Only a **backup** answers all four. A backup is a whole bundle: its
//! `items.ron` files carry every row the project had and the order it had them
//! in. The in-project history log records *prose and nothing else* — it holds an
//! entry only for a row that had text, and `LogVersions::index` synthesises its
//! rows from a map keyed by uid, so they emerge in uid order, which is not an
//! order anything ever had.
//!
//! So against a **log** moment only **changed** is reported. The other three are
//! structural claims a prose-only record cannot make, and making them anyway is
//! not a small inaccuracy: asked of a log moment taken minutes earlier, "moved"
//! fired for every row in the manuscript, and "added since" fired for every
//! folder — because a folder has no prose, so the log had never heard of it.
//!
//! Even for a backup, position is compared as a **rank among the rows both sides
//! share**, not as an absolute index: one scene written since would otherwise
//! shift every row after it and report the whole book as rearranged.
//!
//! Both sides digest their text through [`crate::models::digest_of`], so the two
//! can only disagree about the text itself and never about how each happened to
//! concatenate it.
//!
//! ## Threading
//!
//! The live side has to be read on the UI thread — `AppContext` is `Rc`-based and
//! cannot cross a thread — so it is captured first, as plain data, and moved into
//! the blocking half that opens the archives. That ordering is also what makes
//! the comparison honest: "now" is sampled once, before the slow part, rather
//! than drifting under it.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use teksilo::prelude::{AsyncRuntimeHandle, EventContext, Signal, spawn_blocking};
use teksilo::widgets::DateRange;

use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use skrib_format::versions::{
    BackupVersions, LogVersions, SourceKind, VersionRef, VersionRow, VersionSource,
};

use crate::models::{LiveRow, digest_of};
use crate::versions::ProjectHandle;

/// One recorded moment of the whole project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Moment {
    pub at: DateTime<Utc>,
    pub source: SourceKind,
    pub from: VersionRef,
    /// Total prose bytes across every row at that moment — the sparkline's height.
    ///
    /// **Size, not words.** These are Djot source bytes, markup included, summed
    /// straight from each blob's recorded length (for a zip, the uncompressed size
    /// in the central directory — no decompression). A word count would need every
    /// blob read and parsed at every point, and the project already keeps a real
    /// per-book word series in `ProgressSnapshot`; two charts that disagreed about
    /// "how big is the book" would be worse than one that is clearly labelled.
    pub bytes: u64,
}

/// What one row did between a recorded moment and now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// Present then, not in the project now.
    Removed,
    /// In the project now, absent then.
    Added,
    /// Present in both, different text.
    Changed,
    /// Present in both with the same text, in a different place.
    Moved,
}

/// Where one row's recorded prose can be read back from, and which of its texts
/// that is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PastProse {
    /// The bundle holding it.
    pub from: VersionRef,
    /// The bundle-relative blob path.
    pub blob: String,
    /// Which of the row's texts this blob is.
    ///
    /// Carried rather than re-derived at the point of use, because the *live*
    /// side has to be asked for the same one. A row counts as changed when the
    /// digest over **all** its prose moves, so a scene whose synopsis was edited
    /// is a changed row whose body did not move — and comparing the recorded body
    /// against the live synopsis would not be a comparison of anything.
    pub role: ContentRole,
}

/// Everything needed to put a removed row back into the binder.
///
/// Only ever built for a [`ChangeKind::Removed`] row, which by construction comes
/// from a **backup**: `compare` gates the removed verdict on `structural`, and only
/// a backup is structural. That is what makes this recoverable at all — the
/// project's own history log records prose and nothing else, so a row it alone
/// remembers has no title, no type and no place to be put back into.
///
/// It carries the *whole* row, not the one text the reader happens to be showing:
/// a writer bringing back a cut chapter means the chapter, its synopsis and its
/// epigraph, not whichever of them was on screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoneRow {
    /// Container or leaf. Not derivable from `sub_role` — see
    /// [`skrib_format::versions::VersionRow::role`].
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    /// The name it had at that moment. There is no live row to take one from,
    /// which is exactly why [`RowChange::title`] cannot be used here: for every
    /// other kind that field is deliberately the row's *current* name.
    pub title: String,
    /// A Book's subtitle. Empty for every other kind, and empty is a legal value
    /// there — see [`skrib_format::versions::VersionRow::sub_title`].
    pub sub_title: String,
    /// Its depth in the binder as recorded. A hint for the destination, not an
    /// instruction: the tree it was indented against may be long gone.
    pub indent: i64,
    /// Every prose blob the moment recorded, `(role, bundle-relative path)`.
    pub prose: Vec<(ContentRole, String)>,
    /// The bundle those paths are relative to.
    pub from: VersionRef,
}

/// One row of the change list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowChange {
    pub uid: uuid::Uuid,
    pub title: String,
    pub kind: ChangeKind,
    /// The recorded prose this row can be read back from.
    ///
    /// `None` in the two cases where there is genuinely nothing to read:
    /// [`ChangeKind::Added`], which by definition has no recorded past at that
    /// moment, and a row that existed then but held no prose — a Book, a folder,
    /// a chapter heading. The two are different sentences to a writer, which is
    /// why [`Self::kind`] is what tells them apart and not this being `None`.
    pub source: Option<PastProse>,
    /// What it would take to put this row back — `Some` only for
    /// [`ChangeKind::Removed`].
    ///
    /// Held here rather than re-read at the click, because the index this comes
    /// from is a bundle read: the change list already paid for it, and paying
    /// again on the UI thread is the stall `open_past` had to be moved off.
    pub gone: Option<GoneRow>,
}

impl RowChange {
    /// Whether this row still exists in the project.
    ///
    /// The rows that do not are the whole reason this surface exists: they are
    /// unreachable from every other version surface in the app.
    pub fn is_gone(&self) -> bool {
        self.kind == ChangeKind::Removed
    }
}

/// The identity of a scan, so a rebuild does not start the same one again.
#[derive(Clone, PartialEq, Eq)]
struct ScanKey {
    project: ProjectHandle,
}

/// The span of history the band is currently showing.
///
/// `None` is everything. A window is set two ways, and they are deliberately the
/// same state: the writer types a range into the date filter, or opens one of the
/// periods on the axis. Two mechanisms writing one window means "back" means the
/// same thing however you got there.
type Window = Option<(DateTime<Utc>, DateTime<Utc>)>;

/// Reads the live manuscript. Supplied by the shell, because a view-model does
/// not reach into the store — the same seam the Versions dock's `UidLookup` uses.
pub type LiveManuscriptFn = Rc<dyn Fn() -> Vec<LiveRow>>;

/// Reads **one** live row's text for one content role — the "now" half of the
/// comparison the reader draws when an edited row is opened.
///
/// Separate from [`LiveManuscriptFn`] rather than a field on [`LiveRow`], and
/// deliberately: the manuscript reducer runs every time the band's selection
/// moves, and a `LiveRow` carrying its prose would mean holding the whole book in
/// memory on every move so that the occasional opened row could be diffed. This
/// runs once, when a row is actually opened.
pub type LiveProseFn = Rc<dyn Fn(uuid::Uuid, &ContentRole) -> Option<String>>;

/// Puts a **deleted** row back into the binder.
///
/// Supplied by the shell for the same reason the two readers above are, and more
/// so: recreating a row needs the open-documents store, this Work's undo stack
/// and the binder tree at once, none of which a view-model may reach for. The
/// band hands over what it recorded and knows nothing about what happens next —
/// including the destination picker, which is the shell's to raise.
///
/// `None` in a build with no such wiring (mocks, headless tests), where the
/// reader simply offers no way back. The type is deliberately the whole
/// [`crate::app::DeletedRow`] rather than a uid: re-reading the moment's index to
/// recover the row's type and title would be a second bundle open, on the UI
/// thread, for facts the change list has already paid to know.
pub type RecreateFn = Rc<dyn Fn(&mut EventContext, crate::app::DeletedRow)>;

#[derive(Clone)]
pub struct TimelineViewModel {
    /// Every recorded moment, **oldest first** — a timeline reads left to right.
    moments: Signal<Vec<Moment>>,
    /// The slider's own signal, and the only home of the selected index.
    ///
    /// `f32` because that is what `Slider` writes; the dock rounds it and snaps.
    /// One signal rather than a slider signal mirrored into an index avoids the
    /// two ever disagreeing about where the thumb is.
    position: Signal<f32>,
    changes: Signal<Vec<RowChange>>,
    loading: Signal<bool>,
    error: Signal<String>,
    project: Signal<ProjectHandle>,
    /// The span of history on screen — see [`Window`].
    window: Signal<Window>,
    /// The date filter's own signal, bound straight into a `DateRangeEdit`.
    ///
    /// A calendar range, not an instant range: the widget speaks
    /// `jiff::civil::Date` and a writer thinks in days. Converted at the edge by
    /// [`Self::sync_window`], which is also what keeps this and `window` from
    /// being two states that can disagree — the filter writes the window, the
    /// window never writes the filter back.
    range: Signal<Option<DateRange>>,
    live: Rc<RefCell<Option<LiveManuscriptFn>>>,
    live_prose: Rc<RefCell<Option<LiveProseFn>>>,
    /// Puts a removed row back — see [`RecreateFn`]. `None` until the shell
    /// installs one, which is what the reader's "Bring this back" is gated on.
    recreate: Rc<RefCell<Option<RecreateFn>>>,
    scanned: Rc<RefCell<Option<ScanKey>>>,
    compared: Rc<RefCell<Option<(ScanKey, usize)>>>,
    /// The range last folded into `window`, so a rebuild does not redo it.
    ranged: Rc<RefCell<Option<Option<DateRange>>>>,
    async_rt: Rc<RefCell<Option<AsyncRuntimeHandle>>>,
}

impl Default for TimelineViewModel {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineViewModel {
    pub fn new() -> Self {
        Self {
            moments: Signal::new(Vec::new()),
            position: Signal::new(0.0),
            changes: Signal::new(Vec::new()),
            loading: Signal::new(false),
            error: Signal::new(String::new()),
            project: Signal::new(ProjectHandle::default()),
            window: Signal::new(None),
            range: Signal::new(None),
            live: Rc::new(RefCell::new(None)),
            live_prose: Rc::new(RefCell::new(None)),
            recreate: Rc::new(RefCell::new(None)),
            scanned: Rc::new(RefCell::new(None)),
            compared: Rc::new(RefCell::new(None)),
            ranged: Rc::new(RefCell::new(None)),
            async_rt: Rc::new(RefCell::new(None)),
        }
    }

    // ── view handles ────────────────────────────────────────────────────────

    pub fn moments(&self) -> Signal<Vec<Moment>> {
        self.moments.clone()
    }

    /// Bind this straight into a `Slider`.
    pub fn position(&self) -> Signal<f32> {
        self.position.clone()
    }

    pub fn changes(&self) -> Signal<Vec<RowChange>> {
        self.changes.clone()
    }

    pub fn loading(&self) -> Signal<bool> {
        self.loading.clone()
    }

    pub fn error(&self) -> Signal<String> {
        self.error.clone()
    }

    pub fn project(&self) -> Signal<ProjectHandle> {
        self.project.clone()
    }

    /// Bind this straight into a `DateRangeEdit`.
    pub fn range(&self) -> Signal<Option<DateRange>> {
        self.range.clone()
    }

    pub fn window(&self) -> Signal<Window> {
        self.window.clone()
    }

    /// The moments inside the current window, oldest first.
    pub fn visible_moments(&self) -> Vec<Moment> {
        let all = self.moments.get();
        match self.window.get() {
            None => all,
            Some((start, end)) => all
                .into_iter()
                .filter(|m| m.at >= start && m.at <= end)
                .collect(),
        }
    }

    /// What the axis draws: one bar per visible moment, or one per period when
    /// there are too many. See [`super::timeline_axis`].
    pub fn axis(&self) -> super::Axis {
        super::axis_for(&self.visible_moments())
    }

    /// The selected **bar**'s index, clamped into the axis that exists.
    ///
    /// The slider and the chart both address bars, not raw moments: once the axis
    /// is bucketed there are far fewer bars than moments, and a position read
    /// against the whole history would point somewhere off the end of what is
    /// drawn.
    pub fn index(&self) -> usize {
        let n = self.axis().bars.len();
        if n == 0 {
            return 0;
        }
        (self.position.get().round().max(0.0) as usize).min(n - 1)
    }

    /// The moment the selected bar stands for — for a period, its newest.
    pub fn selected(&self) -> Option<Moment> {
        let axis = self.axis();
        let bar = axis.bars.get(self.index())?;
        self.visible_moments().get(bar.moment).cloned()
    }

    /// Whether the selected moment comes from a record that holds prose alone.
    ///
    /// The one thing about a moment the band has to say out loud. Only a backup
    /// is a whole bundle; the in-project history log records text and nothing
    /// else, so against a log moment the change list can report "edited" and
    /// nothing more — see this module's docs. Unsaid, a writer reads that silence
    /// as a fact about their book ("nothing was deleted or moved this morning")
    /// when it is a fact about the record.
    ///
    /// A view-model method rather than a `match` in the view, because it is the
    /// rule the sentence rests on and it is worth a test of its own.
    pub fn selected_is_prose_only(&self) -> bool {
        self.selected().is_some_and(|m| m.source == SourceKind::Log)
    }

    /// Open the selected period, narrowing the window to it.
    ///
    /// A no-op when the axis is already showing moments — there is nothing
    /// further in, and the caller's control is hidden in that state anyway.
    pub fn open_selected(&self) {
        let axis = self.axis();
        if !axis.opens() {
            return;
        }
        let Some(span) = axis.bars.get(self.index()).and_then(|b| b.span) else {
            return;
        };
        self.set_window(Some(span));
    }

    /// Narrow the band to the last `days` days, ending today.
    ///
    /// The preset `DateRangeEdit` does not have — see
    /// [`crate::date_convert::last_days`]. Writes the filter and folds it
    /// straight into the window rather than waiting for the next build, so the
    /// band has moved by the time the button's own frame is drawn.
    pub fn set_last_days(&self, days: u32) {
        let Some(today) = crate::date_convert::today_utc() else {
            return;
        };
        self.range
            .set(Some(crate::date_convert::last_days(today, days)));
        self.sync_window();
    }

    /// Back to the whole history, clearing the date filter with it.
    ///
    /// Both, deliberately: the writer sees one narrowed band, and leaving the
    /// filter set while the window reopened would put it straight back.
    pub fn show_all(&self) {
        if self.range.get().is_some() {
            self.range.set(None);
        }
        *self.ranged.borrow_mut() = Some(None);
        self.set_window(None);
    }

    fn set_window(&self, window: Window) {
        if self.window.get() == window {
            return;
        }
        self.window.set(window);
        // A new window is a new axis, so the old bar index means nothing. Land on
        // the newest bar, as the first scan does.
        let last = super::axis_for(&self.visible_moments())
            .bars
            .len()
            .saturating_sub(1) as f32;
        self.position.set(last);
        *self.compared.borrow_mut() = None;
    }

    /// Fold the date filter into the window, if it moved.
    ///
    /// Safe to call from `build`, like the scan: a repeat call for the same range
    /// does nothing.
    pub fn sync_window(&self) {
        let range = self.range.get();
        if self.ranged.borrow().as_ref() == Some(&range) {
            return;
        }
        *self.ranged.borrow_mut() = Some(range);
        match range {
            None => self.set_window(None),
            // Inclusive of both days: a writer who picks 3rd–5th means the whole
            // of the 5th, and the widget only carries the day.
            Some(r) => {
                let start = crate::date_convert::from_jiff_date(r.start);
                let end = crate::date_convert::from_jiff_date(r.end) + chrono::Duration::days(1)
                    - chrono::Duration::seconds(1);
                self.set_window(Some((start, end)));
            }
        }
    }

    /// How much past is kept, and how far back it reaches.
    ///
    /// The quiet statement of safety this whole feature is mostly worth to
    /// people: the research behind it is unambiguous that version history gets
    /// built and then forgotten — *"I mostly take snapshots and never look at
    /// them again, I just like knowing I have all my versions."* Most writers
    /// will open these surfaces two or three times a year, and what they want the
    /// rest of the time is one sentence saying they are covered.
    ///
    /// `None` when nothing is recorded, so the caller says *that* rather than
    /// "0 moments since never".
    pub fn coverage(&self) -> Option<(usize, DateTime<Utc>)> {
        let moments = self.moments.get();
        let oldest = moments.first()?.at;
        Some((moments.len(), oldest))
    }

    // ── wiring ──────────────────────────────────────────────────────────────

    pub fn set_async_runtime(&self, rt: Option<AsyncRuntimeHandle>) {
        let mut slot = self.async_rt.borrow_mut();
        if slot.is_none() {
            *slot = rt;
        }
    }

    /// Idempotent by value, so the shell may push on every build.
    pub fn set_project(&self, handle: ProjectHandle) {
        if self.project.get() != handle {
            self.project.set(handle);
        }
    }

    /// Seed the moments a scan would have found, for a test that needs a history
    /// without a filesystem.
    ///
    /// `pub(crate)` and test-only: the real path is [`Self::scan`], and a caller
    /// that set this in the app would be writing a past the project does not have.
    #[cfg(test)]
    pub(crate) fn seed_moments_for_test(&self, moments: Vec<Moment>) {
        self.finish(moments, String::new());
    }

    /// Install the live-manuscript reader (once, from the shell).
    pub fn set_live_source(&self, live: LiveManuscriptFn) {
        let mut slot = self.live.borrow_mut();
        if slot.is_none() {
            *slot = Some(live);
        }
    }

    /// Install the one-row live-prose reader (once, from the shell).
    pub fn set_live_prose_source(&self, live: LiveProseFn) {
        let mut slot = self.live_prose.borrow_mut();
        if slot.is_none() {
            *slot = Some(live);
        }
    }

    /// Install the recreate sink (once, from the shell).
    pub fn set_recreate_sink(&self, recreate: RecreateFn) {
        let mut slot = self.recreate.borrow_mut();
        if slot.is_none() {
            *slot = Some(recreate);
        }
    }

    /// The recreate sink, if the shell installed one.
    ///
    /// Read rather than called through a wrapper, because the caller is a button
    /// handler that must also know whether to *draw* the button: an affordance
    /// that appears and then does nothing is worse than one that never appeared.
    pub fn recreate_sink(&self) -> Option<RecreateFn> {
        self.recreate.borrow().clone()
    }

    /// The live text of one row's `role`, if the shell installed a reader and the
    /// row still holds that text.
    ///
    /// `None` covers every reason the comparison cannot be drawn — no reader
    /// (a mocks build), a row that is gone, a role it no longer has — and the
    /// caller falls back to showing the recorded text on its own.
    ///
    /// **`Some("")` is not one of those reasons**, and the difference cost a
    /// live run to find: a row whose text has since been emptied returns the
    /// empty string, and rejecting it as "nothing to compare with" hid the one
    /// comparison a writer would most want — the whole scene struck through.
    /// A row that never had that text at all returns `None` instead, from the
    /// reader itself, so the two stay distinguishable.
    pub fn live_prose(&self, uid: uuid::Uuid, role: &ContentRole) -> Option<String> {
        let read = self.live_prose.borrow().clone()?;
        read(uid, role)
    }

    /// Scan every source for recorded moments, if the project changed.
    ///
    /// Safe to call from `build`: a repeat call for the same project does
    /// nothing, which is what keeps a scan from scheduling the next one through
    /// the signals `build` binds.
    pub fn scan(&self) {
        let key = ScanKey {
            project: self.project.get(),
        };
        if self.scanned.borrow().as_ref() == Some(&key) {
            return;
        }
        *self.scanned.borrow_mut() = Some(key.clone());
        *self.compared.borrow_mut() = None;
        self.changes.set(Vec::new());

        if key.project.path.is_empty() {
            self.finish(Vec::new(), String::new());
            return;
        }
        let handle = key.project.clone();
        let work = move || collect_moments(&handle);

        self.loading.set(true);
        let rt = self.async_rt.borrow().clone();
        match rt {
            Some(rt) => {
                let this = self.clone();
                rt.spawn_local(async move {
                    match spawn_blocking(work).await {
                        Ok(Ok(m)) => this.finish(m, String::new()),
                        Ok(Err(e)) => this.finish(Vec::new(), e),
                        Err(_) => this.finish(Vec::new(), "the scan did not finish".to_string()),
                    }
                })
                .detach();
            }
            None => match work() {
                Ok(m) => self.finish(m, String::new()),
                Err(e) => self.finish(Vec::new(), e),
            },
        }
    }

    fn finish(&self, moments: Vec<Moment>, error: String) {
        self.moments.set(moments);
        // Land on the newest **bar**, which is the one a writer opening this is
        // most likely to be asking about — and the one whose change list is
        // shortest, so the surface does not open on a wall. Bars, not moments:
        // a bucketed axis has far fewer, and the raw count would put the thumb
        // off the end of what is drawn.
        let last = super::axis_for(&self.visible_moments())
            .bars
            .len()
            .saturating_sub(1) as f32;
        self.position.set(last);
        self.error.set(error);
        self.loading.set(false);
    }

    /// Recompute the change list for the selected moment, if it moved.
    ///
    /// The live side is sampled **here**, on the UI thread, before the blocking
    /// half starts — see the module docs.
    pub fn sync_changes(&self) {
        let Some(key) = self.scanned.borrow().clone() else {
            return;
        };
        // The moment the selected bar stands for — the newest inside it when the
        // axis is bucketed, which is a real recorded state and so a comparison a
        // writer can act on.
        let Some(moment) = self.selected() else {
            if !self.changes.get().is_empty() {
                self.changes.set(Vec::new());
            }
            return;
        };
        let index = self.index();
        if self.compared.borrow().as_ref() == Some(&(key.clone(), index)) {
            return;
        }
        *self.compared.borrow_mut() = Some((key, index));

        let now: Vec<LiveRow> = match self.live.borrow().as_ref() {
            Some(read) => read(),
            None => Vec::new(),
        };
        let handle = self.project.get();
        let work = move || compare(&handle, &moment, &now);

        let out = self.changes.clone();
        let rt = self.async_rt.borrow().clone();
        match rt {
            Some(rt) => {
                rt.spawn_local(async move {
                    if let Ok(rows) = spawn_blocking(work).await {
                        out.set(rows);
                    }
                })
                .detach();
            }
            None => out.set(work()),
        }
    }
}

/// Every recorded moment across both sources, oldest first, with its size.
fn collect_moments(handle: &ProjectHandle) -> Result<Vec<Moment>, String> {
    let backups = BackupVersions {
        directories: handle.destinations.clone(),
        work_unique_id: handle.unique_id.clone(),
        project_path: handle.path.clone(),
    };
    let log = LogVersions::open(&handle.path);
    let sources: [&dyn VersionSource; 2] = [&log, &backups];

    let mut out: Vec<Moment> = Vec::new();
    for src in sources {
        let Ok(refs) = src.list() else { continue };
        for v in refs {
            // A moment that cannot be indexed is skipped rather than shown at
            // zero: a dip to nothing in the sparkline would read as "the book was
            // empty that day", which is a far worse lie than a missing tick.
            let Ok(index) = src.index(&v) else { continue };
            out.push(Moment {
                at: v.taken_at,
                source: v.source,
                from: v,
                bytes: index
                    .rows
                    .iter()
                    .flat_map(|r| r.prose.iter())
                    .map(|(_, _, stamp)| stamp.bytes)
                    .sum(),
            });
        }
    }
    one_tick_per_moment(&mut out);
    Ok(out)
}

/// Order the moments oldest-first and collapse the ones that describe the same
/// state, keeping the record that can say the most about it.
///
/// Sorted by instant, then by size, then **backup before log** — and the last of
/// those three is not cosmetic. Two sources routinely hold one instant (a "Back
/// up now" straight after a save records the same manuscript twice), and the
/// dedup keeps whichever comes first. Log entries are collected first, so a
/// stable sort on the instant alone kept the *log*: the tick silently lost the
/// bundle standing behind it, and with it every structural claim, because
/// [`compare`] gates removed/added/moved on the source that survived. The richer
/// record has to win.
///
/// Sorting on `bytes` as well is what makes the collapse complete rather than
/// approximate: `dedup_by` only ever looks at *adjacent* pairs, so three moments
/// on one instant sized 100, 200, 100 left two identical-looking bars standing.
fn one_tick_per_moment(out: &mut Vec<Moment>) {
    out.sort_by(|a, b| {
        a.at.cmp(&b.at)
            .then(a.bytes.cmp(&b.bytes))
            .then(source_rank(a.source).cmp(&source_rank(b.source)))
    });
    out.dedup_by(|a, b| a.at == b.at && a.bytes == b.bytes);
}

/// Which record to keep when two describe the same instant, smallest first.
///
/// A backup carries the whole bundle and can answer all four kinds of change; the
/// history log carries prose alone and can only ever say "edited". Given the
/// choice, the band keeps the one that can say more.
fn source_rank(source: SourceKind) -> u8 {
    match source {
        SourceKind::Backup => 0,
        SourceKind::Log => 1,
    }
}

/// Compare one recorded moment against the live manuscript.
fn compare(handle: &ProjectHandle, moment: &Moment, now: &[LiveRow]) -> Vec<RowChange> {
    let backups = BackupVersions {
        directories: handle.destinations.clone(),
        work_unique_id: handle.unique_id.clone(),
        project_path: handle.path.clone(),
    };
    let log = LogVersions::open(&handle.path);
    let source: &dyn VersionSource = match moment.source {
        SourceKind::Backup => &backups,
        SourceKind::Log => &log,
    };
    let Ok(index) = source.index(&moment.from) else {
        return Vec::new();
    };

    let then: Vec<((), &VersionRow, String)> = index
        .rows
        .iter()
        .map(|row| {
            let roles: Vec<(String, String)> = row
                .prose
                .iter()
                .filter_map(|(role, blob, _)| {
                    let kind = skrib_format::slug::prose_kind(role)?;
                    let text = source.prose(&moment.from, blob).ok()?;
                    Some((kind.to_string(), text))
                })
                .collect();
            ((), row, digest_of(&roles))
        })
        .collect();

    // Whether this source recorded the project's *structure* — which rows there
    // were, and in what order — or only their prose. See the module docs.
    let structural = moment.source == SourceKind::Backup;

    // Rank among the rows both sides have, so an insertion does not renumber
    // everything after it into a false "moved". `None` for a log version, which
    // has no order to speak of at all.
    let ranks: Option<(HashMap<uuid::Uuid, usize>, HashMap<uuid::Uuid, usize>)> =
        structural.then(|| {
            let shared: HashSet<uuid::Uuid> = then
                .iter()
                .map(|(_, r, _)| r.uid)
                .filter(|u| now.iter().any(|l| &l.uid == u))
                .collect();
            let rank_then = then
                .iter()
                .map(|(_, r, _)| r.uid)
                .filter(|u| shared.contains(u))
                .enumerate()
                .map(|(i, u)| (u, i))
                .collect();
            let rank_now = now
                .iter()
                .map(|l| l.uid)
                .filter(|u| shared.contains(u))
                .enumerate()
                .map(|(i, u)| (u, i))
                .collect();
            (rank_then, rank_now)
        });
    let moved = |uid: &uuid::Uuid| match &ranks {
        Some((a, b)) => a.get(uid) != b.get(uid),
        None => false,
    };

    let mut out = Vec::new();
    for (_, row, digest) in &then {
        match now.iter().find(|l| l.uid == row.uid) {
            // A row the source knew and the project has not. From a backup that
            // means deleted; from the log it means only that the row has no
            // recorded prose, which is not the same claim at all.
            None if structural => out.push(RowChange {
                uid: row.uid,
                // The name it had *then* — there is no other. Every other arm
                // takes the live one deliberately (see below); this row has no
                // live side at all, which is the whole point of it.
                title: row.title.clone(),
                kind: ChangeKind::Removed,
                source: recorded_at(moment, row),
                gone: Some(gone_row(moment, row)),
            }),
            None => {}
            Some(live) if &live.digest != digest => out.push(RowChange {
                uid: row.uid,
                // The name it has *now*, not the one it had then: this row is a
                // way back to a document the writer still owns, and naming it by
                // a title they have since changed would send them looking for
                // something that is not in their binder.
                title: live.title.clone(),
                kind: ChangeKind::Changed,
                source: recorded_at(moment, row),
                gone: None,
            }),
            Some(live) if moved(&row.uid) => out.push(RowChange {
                uid: row.uid,
                title: live.title.clone(),
                kind: ChangeKind::Moved,
                source: recorded_at(moment, row),
                gone: None,
            }),
            Some(_) => {}
        }
    }
    if structural {
        for live in now {
            if !then.iter().any(|(_, r, _)| r.uid == live.uid) {
                out.push(RowChange {
                    uid: live.uid,
                    title: live.title.clone(),
                    kind: ChangeKind::Added,
                    source: None,
                    gone: None,
                });
            }
        }
    }
    out
}

/// The blob holding a row's *body*, which is what opening it should show.
///
/// A synopsis is a note about the text, not the text; a row whose only recorded
/// prose is a synopsis falls back to it rather than opening on nothing. So does
/// one whose only prose is an epigraph — which an earlier shape of this list
/// left out, so a Part or a chapter carrying nothing but its epigraph resolved
/// to no blob at all.
///
/// `None`, not an empty string, when the row recorded no prose whatsoever. That
/// is a real and ordinary state — a Book, a folder, an item never written into —
/// and the difference matters: an empty path handed onward looks exactly like a
/// path, and the reader opened on it, showing a titled panel with nothing in it.
/// The preferred order runs first; anything else the row happens to carry is
/// better than nothing, so it is taken rather than discarded.
/// Where this row's prose can be read back from at `moment`, if it had any.
///
/// The one place `None` is minted, so no caller can accidentally pair a real
/// `VersionRef` with a blob path that is not one.
fn recorded_at(moment: &Moment, row: &VersionRow) -> Option<PastProse> {
    main_blob(row).map(|(role, blob)| PastProse {
        from: moment.from.clone(),
        blob,
        role,
    })
}

/// The whole of a removed row, as the moment recorded it.
///
/// Every prose role it carried, not just the one the reader opens on: a row is
/// put back as itself, and dropping its synopsis on the way would be a silent
/// second loss on top of the one being recovered. An empty blob path is dropped
/// for the same reason [`main_blob`] filters one out — it is not a path.
fn gone_row(moment: &Moment, row: &VersionRow) -> GoneRow {
    GoneRow {
        role: row.role.clone(),
        sub_role: row.sub_role.clone(),
        title: row.title.clone(),
        sub_title: row.sub_title.clone(),
        indent: row.indent,
        prose: row
            .prose
            .iter()
            .filter(|(_, blob, _)| !blob.is_empty())
            .map(|(role, blob, _)| (role.clone(), blob.clone()))
            .collect(),
        from: moment.from.clone(),
    }
}

fn main_blob(row: &VersionRow) -> Option<(ContentRole, String)> {
    READING_ORDER
        .iter()
        .find_map(|role| {
            row.prose_for(role)
                .map(|(blob, _)| (role.clone(), blob.to_string()))
        })
        .filter(|(_, blob)| !blob.is_empty())
}

/// Every prose role there is, best-first.
///
/// Exhaustive on purpose, and held to it by
/// `every_prose_role_is_something_this_can_open`: a role missing from
/// here does not degrade gracefully, it makes rows that carry only that role
/// unopenable. Adding a prose role to `ContentRole` should fail that test rather
/// than quietly ship an empty reader.
const READING_ORDER: [ContentRole; 5] = [
    ContentRole::SceneText,
    ContentRole::NoteText,
    ContentRole::ParatextText,
    ContentRole::EpigraphText,
    ContentRole::SynopsisText,
];

#[cfg(test)]
mod tests;
