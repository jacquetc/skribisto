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
//! entry only for a row that had text, and [`LogVersions::index`] synthesises its
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

use bastyde::prelude::{AsyncRuntimeHandle, Signal, spawn_blocking};
use bastyde::widgets::DateRange;
use chrono::{DateTime, Utc};

use common::entities::ContentRole;
use skrib_format::versions::{
    BackupVersions, LogVersions, SourceKind, VersionRef, VersionRow, VersionSource,
};

use crate::models::{LiveRow, digest_of};

use super::versions::ProjectHandle;

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

/// One row of the change list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowChange {
    pub uid: uuid::Uuid,
    pub title: String,
    pub kind: ChangeKind,
    /// The bundle and blob this row's prose can be read back from.
    ///
    /// `None` in the two cases where there is genuinely nothing to read:
    /// [`ChangeKind::Added`], which by definition has no recorded past at that
    /// moment, and a row that existed then but held no prose — a Book, a folder,
    /// a chapter heading. The two are different sentences to a writer, which is
    /// why [`Self::kind`] is what tells them apart and not this being `None`.
    pub source: Option<(VersionRef, String)>,
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
    out.sort_by_key(|m| m.at);
    // Two sources can hold the same instant; one tick per moment.
    out.dedup_by(|a, b| a.at == b.at && a.bytes == b.bytes);
    Ok(out)
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
                title: row.title.clone(),
                kind: ChangeKind::Removed,
                source: recorded_at(moment, row),
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
            }),
            Some(live) if moved(&row.uid) => out.push(RowChange {
                uid: row.uid,
                title: live.title.clone(),
                kind: ChangeKind::Moved,
                source: recorded_at(moment, row),
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
fn recorded_at(moment: &Moment, row: &VersionRow) -> Option<(VersionRef, String)> {
    main_blob(row).map(|blob| (moment.from.clone(), blob))
}

fn main_blob(row: &VersionRow) -> Option<String> {
    READING_ORDER
        .iter()
        .find_map(|role| row.prose_for(role))
        .map(|(blob, _)| blob.to_string())
        .filter(|blob| !blob.is_empty())
}

/// Every prose role there is, best-first.
///
/// Exhaustive on purpose, and held to it by
/// [`tests::every_prose_role_is_something_this_can_open`]: a role missing from
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
mod tests {
    use super::*;
    use skrib_format::versions::BlobStamp;
    use std::path::PathBuf;

    fn uid(n: u128) -> uuid::Uuid {
        uuid::Uuid::from_u128(n)
    }

    fn live(n: u128, title: &str, order: usize, digest: &str) -> LiveRow {
        LiveRow {
            uid: uid(n),
            title: title.to_string(),
            order,
            digest: digest.to_string(),
        }
    }

    fn version_row(n: u128, title: &str) -> VersionRow {
        VersionRow {
            uid: uid(n),
            title: title.to_string(),
            sub_role: Default::default(),
            indent: 0,
            prose: vec![(
                ContentRole::SceneText,
                format!("binders/01/text/{n}.scene.djot"),
                BlobStamp { bytes: 10 },
            )],
        }
    }

    /// `compare` needs a filesystem, so its *decision table* is exercised through
    /// this pure re-statement of it — the same four rules, over data, including
    /// the rank-among-shared-rows definition of "moved".
    fn classify(
        then: &[(usize, VersionRow, String)],
        now: &[LiveRow],
    ) -> Vec<(String, ChangeKind)> {
        classify_from(then, now, true)
    }

    /// [`classify`], with `ordered` off for a source that has no order — the log.
    fn classify_from(
        then: &[(usize, VersionRow, String)],
        now: &[LiveRow],
        ordered: bool,
    ) -> Vec<(String, ChangeKind)> {
        let shared: HashSet<uuid::Uuid> = then
            .iter()
            .map(|(_, r, _)| r.uid)
            .filter(|u| now.iter().any(|l| &l.uid == u))
            .collect();
        let rank_then: HashMap<uuid::Uuid, usize> = then
            .iter()
            .map(|(_, r, _)| r.uid)
            .filter(|u| shared.contains(u))
            .enumerate()
            .map(|(i, u)| (u, i))
            .collect();
        let mut ordered_now: Vec<&LiveRow> =
            now.iter().filter(|l| shared.contains(&l.uid)).collect();
        ordered_now.sort_by_key(|l| l.order);
        let rank_now: HashMap<uuid::Uuid, usize> = ordered_now
            .iter()
            .enumerate()
            .map(|(i, l)| (l.uid, i))
            .collect();
        let moved = |u: &uuid::Uuid| ordered && rank_then.get(u) != rank_now.get(u);

        let mut out = Vec::new();
        for (_, row, digest) in then {
            match now.iter().find(|l| l.uid == row.uid) {
                None if ordered => out.push((row.title.clone(), ChangeKind::Removed)),
                None => {}
                Some(l) if &l.digest != digest => out.push((l.title.clone(), ChangeKind::Changed)),
                Some(l) if moved(&row.uid) => out.push((l.title.clone(), ChangeKind::Moved)),
                Some(_) => {}
            }
        }
        if ordered {
            for l in now {
                if !then.iter().any(|(_, r, _)| r.uid == l.uid) {
                    out.push((l.title.clone(), ChangeKind::Added));
                }
            }
        }
        out
    }

    #[test]
    fn a_row_deleted_since_is_reported_as_removed() {
        let then = vec![(0, version_row(1, "The lost scene"), "d1".to_string())];
        let got = classify(&then, &[]);
        assert_eq!(
            got,
            vec![("The lost scene".to_string(), ChangeKind::Removed)]
        );
    }

    #[test]
    fn a_row_written_since_is_reported_as_added() {
        let got = classify(&[], &[live(2, "A new chapter", 0, "d2")]);
        assert_eq!(got, vec![("A new chapter".to_string(), ChangeKind::Added)]);
    }

    #[test]
    fn a_row_whose_text_moved_on_is_reported_as_changed() {
        let then = vec![(0, version_row(1, "Chapter 1"), "old".to_string())];
        let got = classify(&then, &[live(1, "Chapter 1", 0, "new")]);
        assert_eq!(got, vec![("Chapter 1".to_string(), ChangeKind::Changed)]);
    }

    /// A reorder is not a rewrite, and calling it one would send a writer looking
    /// for an edit that never happened.
    #[test]
    fn a_row_that_only_changed_place_is_reported_as_moved() {
        let then = vec![
            (0, version_row(1, "Chapter 1"), "a".to_string()),
            (1, version_row(2, "Chapter 2"), "b".to_string()),
        ];
        let got = classify(
            &then,
            &[live(2, "Chapter 2", 0, "b"), live(1, "Chapter 1", 1, "a")],
        );
        assert_eq!(got.len(), 2, "both ends of a swap moved: {got:?}");
        assert!(got.iter().all(|(_, k)| *k == ChangeKind::Moved));
    }

    /// **Rank among the rows both sides share**, not absolute position. One scene
    /// written since would otherwise shift every row after it and report the
    /// whole book as rearranged.
    #[test]
    fn a_row_written_since_does_not_report_everything_after_it_as_moved() {
        let then = vec![
            (0, version_row(1, "Chapter 1"), "a".to_string()),
            (1, version_row(2, "Chapter 2"), "b".to_string()),
        ];
        let got = classify(
            &then,
            &[
                live(9, "A new opening", 0, "n"),
                live(1, "Chapter 1", 1, "a"),
                live(2, "Chapter 2", 2, "b"),
            ],
        );
        assert_eq!(
            got,
            vec![("A new opening".to_string(), ChangeKind::Added)],
            "an insertion is one addition, not an addition plus a rearranged book",
        );
    }

    /// **The bug this caught live.** The history log records *prose*, so its rows
    /// come out of a map keyed by uid — an order nothing ever had — and it holds
    /// no entry at all for a row without text. Asked the structural questions, it
    /// reported the whole manuscript as rearranged and every folder as written
    /// since, against a moment minutes old.
    #[test]
    fn a_source_that_recorded_only_prose_makes_no_structural_claims() {
        let then = vec![
            (0, version_row(1, "Chapter 1"), "a".to_string()),
            (1, version_row(2, "Chapter 2"), "b".to_string()),
        ];
        // Reordered, one row gone from the record, one folder the log never knew.
        let now = [
            live(2, "Chapter 2", 0, "b"),
            live(1, "Chapter 1", 1, "a"),
            live(7, "Front matter", 2, ""),
        ];
        let structural = classify_from(&then, &now, true);
        assert!(
            structural.len() >= 3,
            "precondition: a bundle sees the moves and the addition: {structural:?}",
        );
        assert!(
            classify_from(&then, &now, false).is_empty(),
            "a prose-only record cannot say what existed or where it sat",
        );
    }

    /// …and it still says the one thing it does know.
    #[test]
    fn a_prose_only_source_still_reports_a_changed_text() {
        let then = vec![(0, version_row(1, "Chapter 1"), "old".to_string())];
        let got = classify_from(&then, &[live(1, "Chapter 1", 0, "new")], false);
        assert_eq!(got, vec![("Chapter 1".to_string(), ChangeKind::Changed)]);
    }

    /// The failure this exists to avoid: a project-wide list that repeats every
    /// untouched scene is the per-row timeline's "forty identical backups"
    /// problem, one level up.
    #[test]
    fn an_untouched_row_is_not_listed_at_all() {
        let then = vec![(0, version_row(1, "Chapter 1"), "same".to_string())];
        assert!(classify(&then, &[live(1, "Chapter 1", 0, "same")]).is_empty());
    }

    /// A row is named by what it is called *now* when it still exists, because
    /// that is the name in the binder the writer is looking at.
    #[test]
    fn a_renamed_row_is_listed_under_the_name_it_has_today() {
        let then = vec![(0, version_row(1, "Working title"), "old".to_string())];
        let got = classify(&then, &[live(1, "The Garden Gate", 0, "new")]);
        assert_eq!(got[0].0, "The Garden Gate");
    }

    /// …and a row that is gone keeps the only name anyone ever gave it.
    #[test]
    fn a_deleted_row_keeps_the_name_it_had_when_it_existed() {
        let then = vec![(0, version_row(1, "Cut opening"), "d".to_string())];
        let got = classify(&then, &[]);
        assert_eq!(got[0].0, "Cut opening");
    }

    // ── the view-model's own state ──────────────────────────────────────────

    #[test]
    fn an_unsaved_project_scans_to_nothing_rather_than_to_an_error() {
        let vm = TimelineViewModel::new();
        vm.set_project(ProjectHandle::default());
        vm.scan();
        assert!(vm.moments().get().is_empty());
        assert!(vm.error().get().is_empty());
        assert!(!vm.loading().get());
    }

    #[test]
    fn a_missing_project_scans_synchronously_with_no_executor() {
        let vm = TimelineViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        });
        vm.scan();
        assert!(!vm.loading().get(), "the scan must have completed inline");
        assert!(vm.moments().get().is_empty());
    }

    /// **The bug this guards, and the worse half of it.** The Versions dock at
    /// least came right on the next tab switch, because its key carries the
    /// focused row. This band's only key is the project, so a backup taken
    /// mid-session was invisible to it for the rest of the session — the writer
    /// pressed "Back up now" and the band went on saying the project had no
    /// recorded past at all.
    #[test]
    fn a_newly_recorded_version_makes_the_band_scan_again() {
        let vm = TimelineViewModel::new();
        let base = ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        };
        vm.set_project(base.clone());
        vm.scan();
        let first = vm.scanned.borrow().clone();
        assert!(first.is_some(), "precondition: the first scan ran");

        vm.set_project(ProjectHandle {
            revision: 1,
            ..base
        });
        vm.scan();
        assert!(
            first.as_ref() != vm.scanned.borrow().as_ref(),
            "a recorded version has to invalidate the scan key",
        );
    }

    /// The dock calls `scan` from `build`, and a scan writes signals `build`
    /// binds. Without the guard the first would schedule the second, forever.
    #[test]
    fn rescanning_the_same_project_does_nothing() {
        let vm = TimelineViewModel::new();
        vm.set_project(ProjectHandle {
            path: "/nonexistent/Novel.skrib".into(),
            unique_id: "u".into(),
            destinations: vec!["/nonexistent".into()],
            revision: 0,
        });
        vm.scan();
        vm.scan();
        assert!(!vm.loading().get());
    }

    #[test]
    fn the_slider_lands_on_the_newest_moment_and_clamps_to_what_exists() {
        let vm = TimelineViewModel::new();
        assert_eq!(vm.index(), 0, "an empty timeline has no index to be out of");
        vm.finish(
            (0..3)
                .map(|i| Moment {
                    at: Utc::now() + chrono::Duration::minutes(i),
                    source: SourceKind::Backup,
                    from: VersionRef {
                        path: PathBuf::from("/x"),
                        taken_at: Utc::now(),
                        source: SourceKind::Backup,
                    },
                    bytes: 100,
                })
                .collect(),
            String::new(),
        );
        assert_eq!(
            vm.index(),
            2,
            "opening on the newest is opening on the least"
        );

        // A stale thumb from a longer timeline must not index past the end.
        vm.position().set(99.0);
        assert_eq!(vm.index(), 2);
    }

    /// The sentence most writers will get most of the value from, since most of
    /// them open these surfaces two or three times a year.
    #[test]
    fn coverage_says_how_much_is_kept_and_how_far_back_it_reaches() {
        let vm = TimelineViewModel::new();
        assert_eq!(
            vm.coverage(),
            None,
            "with nothing recorded the caller has to say *that*, not '0 since never'",
        );

        let oldest = Utc::now() - chrono::Duration::days(40);
        vm.finish(
            (0..3)
                .map(|i| Moment {
                    at: oldest + chrono::Duration::days(i),
                    source: SourceKind::Backup,
                    from: VersionRef {
                        path: PathBuf::from("/x"),
                        taken_at: oldest,
                        source: SourceKind::Backup,
                    },
                    bytes: 10,
                })
                .collect(),
            String::new(),
        );
        let (count, back_to) = vm.coverage().expect("three moments are coverage");
        assert_eq!(count, 3);
        assert_eq!(
            back_to, oldest,
            "the reach is the *oldest* moment, which is the reassuring end of the list",
        );
    }

    #[test]
    fn the_body_is_what_opening_a_row_shows_and_a_synopsis_is_the_fallback() {
        let mut row = version_row(1, "Chapter 1");
        assert!(main_blob(&row).is_some_and(|b| b.ends_with(".scene.djot")));

        row.prose = vec![(
            ContentRole::SynopsisText,
            "binders/01/text/1.synopsis.djot".to_string(),
            BlobStamp { bytes: 4 },
        )];
        assert!(
            main_blob(&row).is_some_and(|b| b.ends_with(".synopsis.djot")),
            "a row with only a synopsis opens on it rather than on nothing",
        );
    }

    /// **The bug this caught.** A Part or a chapter can carry an epigraph and
    /// nothing else. The preference list left that role out, so the row resolved
    /// to no blob — and, because the caller took an empty string for a path, it
    /// opened a titled, dated, completely empty reader.
    #[test]
    fn a_row_whose_only_prose_is_an_epigraph_still_opens_on_it() {
        let mut row = version_row(1, "Part One");
        row.prose = vec![(
            ContentRole::EpigraphText,
            "binders/01/text/1.epigraph.djot".to_string(),
            BlobStamp { bytes: 40 },
        )];
        assert!(
            main_blob(&row).is_some_and(|b| b.ends_with(".epigraph.djot")),
            "an epigraph is prose, and it is the only prose this row has",
        );
    }

    /// …and a row that recorded nothing says so, rather than handing on a path
    /// that is not one.
    #[test]
    fn a_row_with_no_recorded_prose_offers_nothing_to_read() {
        let mut row = version_row(1, "Front matter");
        row.prose = Vec::new();
        assert_eq!(main_blob(&row), None);

        let moment = Moment {
            at: Utc::now(),
            source: SourceKind::Backup,
            from: VersionRef {
                path: PathBuf::from("/x.skrib"),
                taken_at: Utc::now(),
                source: SourceKind::Backup,
            },
            bytes: 0,
        };
        assert_eq!(
            recorded_at(&moment, &row),
            None,
            "a real bundle paired with an empty blob path is not a readable source",
        );
    }

    /// The guard that keeps the epigraph bug from happening again under a
    /// different name: every role that is prose has to be a role this can open.
    #[test]
    fn every_prose_role_is_something_this_can_open() {
        let all = [
            ContentRole::SceneText,
            ContentRole::NoteText,
            ContentRole::SynopsisText,
            ContentRole::EpigraphText,
            ContentRole::ParatextText,
            ContentRole::BookTitle,
            ContentRole::BookSubtitle,
            ContentRole::PartTitle,
            ContentRole::ChapterTitle,
        ];
        for role in all {
            // The same definition of "is this prose" the digest uses, so the two
            // sides cannot drift into disagreeing about what a row holds.
            if skrib_format::slug::prose_kind(&role).is_none() {
                continue;
            }
            let row = VersionRow {
                uid: uid(1),
                title: "x".into(),
                sub_role: Default::default(),
                indent: 0,
                prose: vec![(
                    role.clone(),
                    "binders/01/text/1.djot".to_string(),
                    BlobStamp { bytes: 1 },
                )],
            };
            assert!(
                main_blob(&row).is_some(),
                "{role:?} is prose but a row carrying only it opens on nothing",
            );
        }
    }
}
