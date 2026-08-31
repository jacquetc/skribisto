// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Line up a returning manuscript against the one the project already holds.
//!
//! When a writer sends a draft out and reads the marked-up file back, the question is not
//! "what does this file contain" — the importer already answers that — but **"which of these
//! rows are the ones I already have, and who changed what"**. This module answers it, as a
//! pure function over two ordered lists.
//!
//! Store-free and headless on purpose, the way `binder_ordering` and
//! [`analysis`](crate::analysis) are: the caller reads the binder and the plan, and this
//! decides. That keeps the interesting logic — the part that is easy to get subtly wrong and
//! impossible to eyeball in a UI — testable without a database, a document or a window.
//!
//! # Two passes
//!
//! **Pair** each incoming row with at most one existing row, by a ladder of decreasing
//! confidence (see `pair`). **Align** the two streams into one ordered list of
//! `MergeRow`s, each holding a current side, an incoming side, or both.
//!
//! The alignment is what makes the interesting case representable at all: *a chapter the
//! editor inserted between two existing chapters*. It is not a row on either side alone — it
//! is a gap in the current stream at a definite position — and a design that returned a flat
//! list of matches could not say where it goes.
//!
//! # What it deliberately does not do
//!
//! Nothing here deletes, and nothing here decides. A row present locally and absent from the
//! returning file comes back as `RowStatus::Missing` — *shown*, never acted on. An editor
//! who deleted a chapter in Word may have been tidying their copy; the writer says what
//! happens to theirs.

use crate::CreateType;

/// One row the project already has, reduced to what pairing needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistingRow {
    /// `round_trip::uid_tag(item.uid)` — precomputed by the caller, because this module has
    /// no opinion about uuids and the caller has to hash them anyway.
    pub uid_tag: String,
    pub title: String,
    pub create_type: CreateType,
    /// [`round_trip::digest`](crate::round_trip::digest) of this row's prose **as the project
    /// holds it now**.
    pub digest: String,
}

/// One row the returning file would create, reduced to what pairing needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingRow {
    /// The tag from this row's round-trip mark. `None` for a first arrival, a file from
    /// anywhere else, or a row the editor added inside the file.
    pub source_uid_tag: Option<String>,
    /// The digest that mark carried — this row's prose **as it was exported**. The baseline
    /// of the three-way comparison, and `None` exactly when `source_uid_tag` is.
    pub source_digest: Option<String>,
    pub title: String,
    pub create_type: CreateType,
    /// Digest of the prose **in the returning file**.
    pub digest: String,
}

/// What happened to a row, as far as the two sides and the baseline can tell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowStatus {
    /// Both sides hold the same prose. Nothing to merge.
    Identical,
    /// The editor changed it and the writer did not — the ordinary case for a draft that went
    /// out and came back marked up.
    EditorEdited,
    /// The writer changed it and the editor did not. Taking the import would undo their work.
    YouEdited,
    /// Both changed it, differently. The one case where no default is safe.
    Conflict,
    /// The two sides differ and there is no baseline to say who moved — a file from another
    /// tool, or one exported before round-trip marks existed.
    Different,
    /// Only in the returning file: the editor added it.
    New,
    /// Only in the project: the returning file does not have it.
    Missing,
}

impl RowStatus {
    /// Whether this row needs the writer to look before anything is done to it.
    pub fn needs_attention(self) -> bool {
        matches!(self, RowStatus::Conflict | RowStatus::Different)
    }
}

/// What may be done with one merge row. Ordered most- to least-common, so a UI can offer the
/// first as its default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowAction {
    /// Bring the editor's comments across and leave the prose alone.
    CommentsOnly,
    /// Replace the project's prose with the file's.
    TakeImport,
    /// Leave the project's row exactly as it is.
    KeepCurrent,
    /// Create it as a new row rather than updating anything.
    CreateNew,
    /// Do nothing at all with this row.
    Ignore,
}

/// One line of the merge: what the project has, what the file brings, and what may be done.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeRow {
    /// Index into the `existing` slice, or `None` when the editor added this row.
    pub current: Option<usize>,
    /// Index into the `incoming` slice, or `None` when the file no longer has this row.
    pub incoming: Option<usize>,
    pub status: RowStatus,
    /// True when the row is paired but the two sides disagree about where it sits — the
    /// editor moved a chapter. Orthogonal to [`status`](Self::status), which is about prose.
    pub moved: bool,
    /// The actions this row's shape allows, most-likely first.
    pub actions: Vec<RowAction>,
}

impl MergeRow {
    /// The action a UI should preselect.
    ///
    /// Always the first offered, which is why [`RowAction`]'s own order is meaningful: the
    /// safe choice leads for every shape, and the writer overrides it rather than being asked
    /// to construct it.
    pub fn default_action(&self) -> RowAction {
        self.actions.first().copied().unwrap_or(RowAction::Ignore)
    }
}

/// Compare two titles the way a human would: case-insensitively, ignoring surrounding space
/// and any decorative punctuation an editor's word processor may have introduced.
fn same_title(a: &str, b: &str) -> bool {
    fn key(s: &str) -> String {
        s.trim()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .flat_map(char::to_lowercase)
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    !a.trim().is_empty() && key(a) == key(b)
}

/// Pair each incoming row with at most one existing row.
///
/// Returns, for each incoming row, the index of the existing row it *is* — or `None`.
///
/// # The ladder, and why it stops where it does
///
/// 1. **The round-trip mark.** Exact, and the only rung that is *evidence* rather than
///    inference: the file says which row this was, in a bookmark the editor's own application
///    preserved. Everything below is a guess.
/// 2. **Same type and same title**, disambiguated by taking the nearest unclaimed candidate
///    in stream order. This is for files that carry no marks — another tool's `.docx`, or an
///    export from before marks existed.
/// 3. **Nothing.** There is deliberately no prose-similarity rung. Two chapters of one
///    manuscript share an author, a cast and a vocabulary, and "these are 80% alike" is how a
///    reconciliation confidently pairs chapter 4 with chapter 5 and offers to overwrite one
///    with the other. An unpaired row shows as `New`, which the writer can see and correct;
///    a wrongly paired one shows as a plausible diff, which they cannot.
pub fn pair(existing: &[ExistingRow], incoming: &[IncomingRow]) -> Vec<Option<usize>> {
    let mut out: Vec<Option<usize>> = vec![None; incoming.len()];
    let mut claimed = vec![false; existing.len()];

    // Rung 1, over every incoming row first: an exact match must never lose a candidate to a
    // title guess made earlier in the list.
    for (i, inc) in incoming.iter().enumerate() {
        let Some(tag) = inc.source_uid_tag.as_deref().filter(|t| !t.is_empty()) else {
            continue;
        };
        if let Some(j) = existing
            .iter()
            .position(|e| !e.uid_tag.is_empty() && e.uid_tag == tag)
            && !claimed[j]
        {
            out[i] = Some(j);
            claimed[j] = true;
        }
    }

    // Rung 2. `last` walks forward so equal titles pair in the order they appear rather than
    // all landing on the first candidate — the shape of a manuscript with several chapters
    // called "Untitled".
    let mut last = 0usize;
    for (i, inc) in incoming.iter().enumerate() {
        if out[i].is_some() {
            continue;
        }
        let hit = (last..existing.len()).chain(0..last).find(|&j| {
            !claimed[j]
                && existing[j].create_type == inc.create_type
                && same_title(&existing[j].title, &inc.title)
        });
        if let Some(j) = hit {
            out[i] = Some(j);
            claimed[j] = true;
            last = j + 1;
        }
    }

    out
}

/// Indices of a longest strictly-increasing subsequence of `values`.
///
/// The anchors of the alignment: the paired rows whose order the two sides agree on. Anything
/// paired but outside this set is a row the editor moved, and is reported as such rather than
/// being torn into a delete and an insert — which would lose the identity the mark went to
/// such lengths to carry.
fn longest_increasing(values: &[usize]) -> Vec<usize> {
    if values.is_empty() {
        return Vec::new();
    }
    // `tails[k]` = index into `values` of the smallest tail of an increasing run of length k+1.
    let mut tails: Vec<usize> = Vec::new();
    let mut prev: Vec<Option<usize>> = vec![None; values.len()];
    for i in 0..values.len() {
        let pos = tails.partition_point(|&t| values[t] < values[i]);
        if pos > 0 {
            prev[i] = Some(tails[pos - 1]);
        }
        if pos == tails.len() {
            tails.push(i);
        } else {
            tails[pos] = i;
        }
    }
    let mut out = Vec::with_capacity(tails.len());
    let mut cur = tails.last().copied();
    while let Some(i) = cur {
        out.push(i);
        cur = prev[i];
    }
    out.reverse();
    out
}

/// Line the two streams up into one ordered list.
///
/// The order is the **incoming** file's, with every unmatched existing row emitted at the
/// place it belongs relative to its neighbours. That is the right way round: the returning
/// file is what the writer is reading, and a row it no longer has still has to appear
/// somewhere they can see it.
pub fn align(existing: &[ExistingRow], incoming: &[IncomingRow]) -> Vec<MergeRow> {
    let pairs = pair(existing, incoming);

    // The anchors: paired rows in an order both sides agree on.
    let paired: Vec<(usize, usize)> = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.map(|j| (i, j)))
        .collect();
    let anchor_positions = longest_increasing(&paired.iter().map(|(_, j)| *j).collect::<Vec<_>>());
    let mut is_anchor = vec![false; incoming.len()];
    for &p in &anchor_positions {
        is_anchor[paired[p].0] = true;
    }

    // Every existing row some incoming row pairs with, anchor or not.
    //
    // The forward-fill below has to consult this and not only `emitted`. A row the editor
    // *moved* is paired out of order: it is emitted where the file puts it, which may be after
    // an anchor whose fill sweeps past it. Asking "has it been emitted yet" gets `false` for a
    // row that simply has not been reached, and the walk announces a chapter as deleted that
    // the returning file is still carrying — then emits it a second time when its own pairing
    // comes round. Moving the *first* row to the end is enough to do it.
    let mut claimed = vec![false; existing.len()];
    for j in pairs.iter().flatten() {
        claimed[*j] = true;
    }

    let mut out = Vec::with_capacity(incoming.len() + existing.len());
    let mut emitted = vec![false; existing.len()];
    let mut next_existing = 0usize;

    for (i, inc) in incoming.iter().enumerate() {
        match pairs[i] {
            Some(j) if is_anchor[i] => {
                // Everything the current stream holds before this anchor and the file no
                // longer has, in its own order.
                while next_existing < j {
                    if !emitted[next_existing] && !claimed[next_existing] {
                        out.push(missing_row(next_existing));
                        emitted[next_existing] = true;
                    }
                    next_existing += 1;
                }
                out.push(paired_row(i, j, &existing[j], inc, false));
                emitted[j] = true;
                next_existing = j + 1;
            }
            Some(j) => {
                // Paired, but out of order: the editor moved it. Emitted where the file puts
                // it, flagged, and *not* also emitted as missing further down.
                out.push(paired_row(i, j, &existing[j], inc, true));
                emitted[j] = true;
            }
            None => out.push(new_row(i)),
        }
    }

    // Whatever the current stream still holds past the last anchor.
    for (j, done) in emitted.iter().enumerate() {
        if !done {
            out.push(missing_row(j));
        }
    }

    out
}

fn paired_row(
    i: usize,
    j: usize,
    current: &ExistingRow,
    inc: &IncomingRow,
    moved: bool,
) -> MergeRow {
    let status = status_of(current, inc);
    MergeRow {
        current: Some(j),
        incoming: Some(i),
        status,
        moved,
        actions: actions_for(status),
    }
}

fn new_row(i: usize) -> MergeRow {
    MergeRow {
        current: None,
        incoming: Some(i),
        status: RowStatus::New,
        moved: false,
        actions: vec![RowAction::CreateNew, RowAction::Ignore],
    }
}

fn missing_row(j: usize) -> MergeRow {
    MergeRow {
        current: Some(j),
        incoming: None,
        status: RowStatus::Missing,
        moved: false,
        // Nothing but "leave it alone". A returning file that no longer mentions a chapter is
        // not evidence the chapter should go — see the module doc.
        actions: vec![RowAction::KeepCurrent],
    }
}

/// The three-way comparison, or the two-way one when the file carries no baseline.
fn status_of(current: &ExistingRow, inc: &IncomingRow) -> RowStatus {
    let Some(baseline) = inc.source_digest.as_deref().filter(|d| !d.is_empty()) else {
        // No baseline: the two sides can be compared to each other and to nothing else.
        return if current.digest == inc.digest {
            RowStatus::Identical
        } else {
            RowStatus::Different
        };
    };
    let local_changed = current.digest != baseline;
    let incoming_changed = inc.digest != baseline;
    match (local_changed, incoming_changed) {
        (false, false) => RowStatus::Identical,
        (false, true) => RowStatus::EditorEdited,
        (true, false) => RowStatus::YouEdited,
        // Both moved. If they landed on the same text they have converged, and calling that a
        // conflict would ask the writer to adjudicate between two identical passages.
        (true, true) if current.digest == inc.digest => RowStatus::Identical,
        (true, true) => RowStatus::Conflict,
    }
}

/// What may be done with a paired row, safest-plausible first.
fn actions_for(status: RowStatus) -> Vec<RowAction> {
    use RowAction::*;
    match status {
        // Nothing to merge, so the only thing the file can still bring is the editor's notes.
        RowStatus::Identical => vec![CommentsOnly, TakeImport, KeepCurrent, CreateNew, Ignore],
        // The ordinary case, and the only one where taking the import is the obvious default.
        RowStatus::EditorEdited => vec![TakeImport, CommentsOnly, KeepCurrent, CreateNew, Ignore],
        // The writer moved on since they sent the draft. Overwriting would discard their own
        // work, so it is offered but never preselected.
        RowStatus::YouEdited => vec![KeepCurrent, CommentsOnly, TakeImport, CreateNew, Ignore],
        // No safe default exists, so the default is the one that changes nothing.
        RowStatus::Conflict | RowStatus::Different => {
            vec![KeepCurrent, CommentsOnly, TakeImport, CreateNew, Ignore]
        }
        RowStatus::New => vec![CreateNew, Ignore],
        RowStatus::Missing => vec![KeepCurrent],
    }
}

#[cfg(test)]
mod tests;
