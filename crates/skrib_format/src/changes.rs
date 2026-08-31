// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning "forty-seven backups" into "six times this scene changed".
//!
//! Every source in [`crate::versions`] can say what a row looked like at some
//! moment. Almost none of those moments are interesting: a backup captures the
//! whole project, so a scene nobody touched appears, unchanged, in every backup
//! taken since it was written. Listing those verbatim is the failure mode the
//! whole feature has to avoid — it is what makes writers describe the equivalent
//! feature elsewhere as "excessive scrolling through timestamps" and stop opening
//! it.
//!
//! So a version becomes a **row in the timeline only where that row's prose
//! actually differs from the next-older recording of it**. Consecutive identical
//! states collapse into the earliest one, because the edit happened when the text
//! was first written, not when a later backup copied it forward.
//!
//! ## Why every state is hashed
//!
//! A zip entry carries a CRC-32 in its central directory, and an earlier shape
//! here used it as a pre-filter: equal CRC to the previous state ⇒ unchanged, skip
//! the read. That is the one direction a checksum **cannot** be trusted in. A CRC
//! *mismatch* is a proof of difference; a *match* is not a proof of sameness, and
//! the failure it admits is the worst one this feature has — a real edit silently
//! collapsed into the state before it, invisible in the timeline forever, with
//! nothing surfaced.
//!
//! Nor did trusting it buy much: a differing CRC still needs the hash, because the
//! hash is the identity the two sources are merged on. So every state is hashed,
//! and the comparison has one rule instead of two.
//!
//! ## What the boundaries mean
//!
//! **How much** a change moved is deliberately not answered here. A magnitude worth
//! showing needs a real word diff, which needs the Djot parser and a diff library;
//! it lives beside the diff itself, in `teksilo_ui::view_models::version_diff`, so
//! there is exactly one definition of "how much changed" rather than a cheap one
//! here and an accurate one there that quietly disagree.
//!
//! Walking the same list also yields the two facts a timeline has to state rather
//! than imply — the point before which a row *did not exist yet*, and the point
//! after which it was *gone*. Time Machine leaves both to inference, via an absence
//! and a greyed-out button, and has generated a decade of confused support threads
//! for it.

use std::collections::BTreeSet;

use anyhow::Result;
use chrono::{DateTime, Utc};

use super::versions::{RowAt, SourceKind, VersionRef, VersionSource};
use common::entities::ContentRole;

/// One entry in a row's timeline: a moment at which its prose became this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub at: DateTime<Utc>,
    pub source: SourceKind,
    /// The bundle this state can be read back from.
    pub from: VersionRef,
    /// The blob to read, relative to that bundle's root.
    pub blob_path: String,
    /// blake3 of the prose at this moment — the identity two sources are merged on.
    pub hash: String,
    pub bytes: u64,
    /// The row's title as of this moment, when the source records one. The history
    /// log stores prose rather than metadata, so it has none.
    pub title: String,
}

/// A row's whole recorded past, newest first, plus the edges of it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Timeline {
    /// Newest first.
    pub changes: Vec<Change>,
    /// A moment that was **read** and in which this row was **not there** — the
    /// newest such moment before it first appears. `None` when no examined version
    /// ever proved its absence.
    ///
    /// This is a timestamp the timeline can stand behind: at exactly this instant,
    /// the row demonstrably did not exist. It is deliberately *not* the moment the
    /// row was first seen. The row was created somewhere in the gap between the two
    /// and nothing here can narrow it further, so naming the later end would claim
    /// the row did not exist during a stretch in which it may well have. That is
    /// the same class of confident-and-false statement an unreadable backup used to
    /// produce, one step further in.
    pub absent_at: Option<DateTime<Utc>>,
    /// The newest moment in which the row was still **present**, when a later
    /// examined moment proved it gone — i.e. it was deleted some time after this.
    ///
    /// The mirror of [`Self::absent_at`], and provable for the same reason: this
    /// instant is one at which the row demonstrably *did* exist.
    pub deleted_after: Option<DateTime<Utc>>,
    /// Moments that could not be read at all (an unplugged drive, a truncated
    /// archive). Surfaced rather than swallowed: a gap a writer cannot see is a
    /// gap they will assume is data loss.
    pub unreadable: Vec<VersionRef>,
    /// How many states of this row's prose the sources have **removed** as they
    /// aged — today only the project's own history log, which thins per
    /// `(row, role)` and keeps the count (see
    /// [`crate::versions::VersionSource::thinned_away`]).
    ///
    /// The third boundary fact, and the one that could not be inferred. The other
    /// two are read off the walk below because a moment that was *examined* proves
    /// something; thinning leaves nothing to examine, and a silent slot is
    /// deliberately inert so a routine sweep does not read as a gap. Without a
    /// tally recorded at the moment of the deletion, a timeline whose oldest entry
    /// is all that survives of a year of daily edits is indistinguishable from one
    /// that has its row's whole past — and "the earliest version on record" then
    /// reads as a promise it cannot keep.
    ///
    /// It counts what the **log** dropped, not what is missing from this list: a
    /// backup may still hold one of those states, in which case the merge below
    /// puts it back on screen. That is why the sentence a surface draws from this
    /// has to name where the states were dropped from.
    pub thinned_away: u32,
}

impl Timeline {
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

/// Build one row's timeline for one content role, across every supplied source.
///
/// Sources are merged rather than concatenated: the same edit is usually present
/// in both the project's log and a backup taken afterwards, so states are
/// de-duplicated by content hash and the **earliest** moment holding a given hash
/// wins. A version that a source cannot read is recorded in
/// [`Timeline::unreadable`] rather than dropped.
pub fn timeline_for(
    sources: &[&dyn VersionSource],
    uid: uuid::Uuid,
    role: &ContentRole,
) -> Result<Timeline> {
    // Every version any source can offer, tagged with **which** source produced it
    // so reading a blob back later is a direct index rather than a search. (An
    // earlier shape looked the source up by re-listing each one per candidate,
    // which is quadratic in versions and does real I/O each time.)
    //
    // `row_at`, not `index`: this only ever wants one row, and asking for the whole
    // project's index per moment made the cost of one scene's timeline scale with
    // how much the writer had edited everything else. It is also the call that lets
    // a source distinguish "the row was not there" from "I cannot say".
    let mut seen: Vec<(usize, VersionRef, Option<RowAt>)> = Vec::new();
    let mut unreadable = Vec::new();
    for (si, src) in sources.iter().enumerate() {
        for v in src.list()? {
            match src.row_at(&v, uid, role) {
                Ok(found) => seen.push((si, v, Some(found))),
                Err(_) => {
                    unreadable.push(v.clone());
                    seen.push((si, v, None));
                }
            }
        }
    }
    // Oldest first: the walk below needs to know what came before.
    seen.sort_by_key(|(_, v, _)| v.taken_at);

    // What each moment says about the row, keeping "it was not there" apart from
    // "it could not be read" and from "this source cannot say" — three different
    // silences, only one of which is evidence. The boundaries below turn evidence
    // into a sentence shown to a writer, so conflating them is how the dock comes
    // to state something false with confidence.
    let mut states: Vec<Slot> = Vec::with_capacity(seen.len());
    for (si, v, found) in &seen {
        let Some(found) = found else {
            states.push(Slot::Unreadable);
            continue;
        };
        states.push(match found {
            RowAt::Present {
                blob_path,
                stamp,
                title,
            } => Slot::Candidate(Box::new(Candidate {
                at: v.taken_at,
                source_index: *si,
                from: v.clone(),
                blob_path: blob_path.clone(),
                bytes: stamp.bytes,
                title: title.clone(),
            })),
            // Read, and the row was genuinely not in it. The moment travels with
            // the verdict: it is the evidence behind `Timeline::absent_at`, and a
            // boundary is only worth stating if it can name the moment it proves.
            RowAt::Absent => Slot::Absent(v.taken_at),
            RowAt::Silent => Slot::Silent,
        });
    }

    // Resolve each candidate's true content hash, reusing the source that produced
    // it. Every state is hashed — see the module docs on why a CRC match is not a
    // shortcut this may take.
    let mut resolved: Vec<Slot> = Vec::with_capacity(states.len());
    for slot in states {
        let Slot::Candidate(c) = slot else {
            resolved.push(slot);
            continue;
        };
        match sources[c.source_index].prose(&c.from, &c.blob_path).ok() {
            Some(text) => {
                let hash = blake3::hash(text.as_bytes()).to_hex().to_string();
                resolved.push(Slot::Present(Box::new(c.to_change(hash))));
            }
            None => {
                // The index listed a blob the source cannot produce. Report the
                // moment as unreadable rather than inventing a state for it —
                // and, crucially, keep it distinguishable from absence.
                unreadable.push(c.from.clone());
                resolved.push(Slot::Unreadable);
            }
        }
    }

    let mut timeline = assemble(resolved, unreadable);
    // Asked of every source and maxed rather than summed: each reports what *it*
    // removed, and a row whose past two logs both thinned (a project restored from
    // a backup carries the backup's log) has not lost the two counts added
    // together — the states overlap, so the larger of the two is the claim that
    // holds under either reading.
    timeline.thinned_away = sources
        .iter()
        .map(|s| s.thinned_away(uid, role))
        .max()
        .unwrap_or(0);
    Ok(timeline)
}

/// What one examined moment had to say about the row.
///
/// Four states, not two. "The version says this row was not there", "the version
/// could not be read" and "this source has no record either way" look identical
/// from a bare `Option`, and treating them alike is how the boundaries below come
/// to make a confident false claim — see [`assemble`].
enum Slot {
    /// Found in the version's index, hash not yet resolved.
    Candidate(Box<Candidate>),
    /// Found, and its prose hashed — a real state of the row.
    Present(Box<Change>),
    /// Read successfully; the row was not in it, as of this moment.
    Absent(DateTime<Utc>),
    /// Read successfully, and the source declined to testify — it holds no record
    /// of this row at this moment for a reason unrelated to whether the row
    /// existed. [`crate::versions::RowAt::Silent`] explains when that happens.
    Silent,
    /// Could not be read at all, so it says nothing either way.
    Unreadable,
}

/// Collapse consecutive identical states and read the boundaries off the walk.
fn assemble(resolved: Vec<Slot>, unreadable: Vec<VersionRef>) -> Timeline {
    let mut changes: Vec<Change> = Vec::new();
    let mut absent_at = None;
    let mut deleted_after = None;
    let mut last_hash: Option<String> = None;
    let mut ever_present = false;

    for (i, slot) in resolved.iter().enumerate() {
        match slot {
            Slot::Present(c) => {
                if last_hash.as_deref() != Some(c.hash.as_str()) {
                    // Genuinely different prose (or the first sighting): a change.
                    changes.push((**c).clone());
                    last_hash = Some(c.hash.clone());
                }
                if !ever_present {
                    // The last moment before the first sighting that was **read**
                    // and did not contain the row. Only such a moment proves
                    // anything: a backup on an unplugged drive says nothing about
                    // whether the row existed, and letting it stand in here
                    // produced a confident "Didn't exist before <date>" that was
                    // simply false.
                    //
                    // The moment kept is the **absent** one, not this sighting.
                    // The row was created somewhere between the two, and the only
                    // instant this can name with proof behind it is the earlier
                    // end. Naming the sighting claimed the row did not exist right
                    // up to it — across a gap in which it may well have.
                    absent_at = resolved[..i].iter().rev().find_map(|s| match s {
                        Slot::Absent(at) => Some(*at),
                        _ => None,
                    });
                    ever_present = true;
                }
                deleted_after = None;
            }
            // Read, and the row was not there: that is a deletion boundary. The
            // moment recorded is the newest one it was still *present* in, which
            // is the provable half of the same pair.
            Slot::Absent(_) => {
                if ever_present && deleted_after.is_none() {
                    deleted_after = changes.last().map(|c| c.at);
                }
                last_hash = None;
            }
            // Unreadable: it is already in `unreadable`, where the writer can see
            // it. It must not also be read as evidence of anything — but it does
            // break the run, since two states either side of a gap are not known
            // to be consecutive.
            //
            // `Candidate` cannot reach here: every one is resolved into `Present`
            // or `Unreadable` before `assemble` is called. Matched rather than
            // wildcarded so adding a further state is a compile error here.
            Slot::Unreadable | Slot::Candidate(_) => {
                last_hash = None;
            }
            // Silent: **fully transparent**, and deliberately not treated like an
            // unreadable gap. A source with no record of this row at this moment
            // has not observed a gap; it has not observed anything, so the walk
            // must proceed exactly as if the moment had never been examined.
            //
            // Breaking the run here would be actively harmful rather than merely
            // cautious: thinning is routine, so a log with no surviving entry for
            // an old moment sits between two backups holding identical prose, and
            // clearing `last_hash` would enter that unchanged prose as a second
            // "change". A timeline padded with duplicate entries is the exact
            // failure this module exists to avoid.
            Slot::Silent => {}
        }
    }

    changes.reverse(); // newest first, the order a timeline is read in
    Timeline {
        changes,
        absent_at,
        deleted_after,
        unreadable: dedup_refs(unreadable),
        // Not readable off this walk — see the field's own docs. `timeline_for`
        // fills it from the sources themselves.
        thinned_away: 0,
    }
}

fn dedup_refs(mut v: Vec<VersionRef>) -> Vec<VersionRef> {
    let mut seen: BTreeSet<(std::path::PathBuf, DateTime<Utc>)> = BTreeSet::new();
    v.retain(|r| seen.insert((r.path.clone(), r.taken_at)));
    v
}

struct Candidate {
    at: DateTime<Utc>,
    /// Index into the caller's `sources`, so reading the blob back is direct.
    source_index: usize,
    from: VersionRef,
    blob_path: String,
    bytes: u64,
    title: String,
}

impl Candidate {
    fn to_change(&self, hash: String) -> Change {
        Change {
            at: self.at,
            source: self.from.source,
            from: self.from.clone(),
            blob_path: self.blob_path.clone(),
            hash,
            bytes: self.bytes,
            title: self.title.clone(),
        }
    }
}
