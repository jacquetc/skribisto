// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The in-project **history log**: what each writing row said, at each save.
//!
//! A project's routine backups already carry history, but only at the cadence
//! backups run — under the shipped policy that is on close and every couple of
//! hours, which answers *"what did this scene say last week"* and cannot answer
//! *"what did it say this morning"*. This log fills that gap from the other end:
//! it is written on **every save**, costs one blake3 per prose blob, and stores
//! only the blobs that actually changed.
//!
//! ## Shape on disk
//!
//! ```text
//! history/index.ron        Vec<HistoryEntry>, append-only, oldest first
//! history/<blake3>.djot    the prose itself, content-addressed and shared
//! ```
//!
//! Content-addressing is what makes this cheap: a scene edited fifty times but
//! reverted to an earlier wording costs one blob, not fifty, and two rows that
//! happen to hold identical text (an empty synopsis, a duplicated note) share
//! one file. It is deliberately the same design the `assets/` directory already
//! uses, for the same reasons — see [`crate::media`].
//!
//! ## Two invariants that are not obvious
//!
//! 1. **`HistoryLog` must never enter the content fingerprint.** It is
//!    `#[serde(skip)]` on [`WorkBundle`], exactly as `asset_bytes` is, and
//!    `fingerprint::strip_volatile`'s own comment states the rule: *"nothing may
//!    be added that would include them"*. If the log were hashed, every save
//!    would change the fingerprint, `skip_if_unchanged` would never skip again,
//!    and every close would write a fresh full backup of an unchanged project.
//! 2. **Only `save_work` records.** `save_as` writes a copy of the same state and
//!    `backup_now` snapshots it; recording in either would stamp a second entry
//!    for prose that never changed. Both still *carry* the log, because they copy
//!    the whole bundle — which is what lets history survive into a backup and back
//!    out of a restore.
//!
//! ## Reading it back
//!
//! A corrupt `index.ron` degrades to an empty log rather than refusing to open the
//! project. History is a convenience; the manuscript is not, and no writer should
//! lose access to their book because a convenience file got truncated.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::bundle::WorkBundle;
use super::retention::{RetentionPolicy, policy_keep_indices};
use common::entities::ContentRole;

/// Directory holding the history blobs and index, relative to the bundle root.
pub const HISTORY_DIR: &str = "history";

/// Bundle-root-relative path of the history index.
pub const HISTORY_INDEX: &str = "history/index.ron";

/// Bundle-root-relative path of one history blob.
pub fn blob_relpath(hash: &str) -> String {
    format!("{HISTORY_DIR}/{hash}.djot")
}

/// How much per-row history a project keeps.
///
/// The same GFS shape backups use, and for the same reason: a writer wants dense
/// history for today, thinning as it ages, not an unbounded log. It is applied
/// **per `(row, role)`** though, so these numbers describe one scene's own past,
/// not the project's — twenty-four hourly states of the scene you are working on,
/// then a daily state for a week, weekly for a month, monthly for a year.
///
/// Deliberately a constant rather than a setting: the cost is bounded and small
/// (prose, content-addressed and deduplicated), and a writer asked to tune their
/// own safety net will mostly get it wrong in the direction that loses work.
pub const DEFAULT_POLICY: RetentionPolicy = RetentionPolicy::Gfs {
    hourly: 24,
    daily: 7,
    weekly: 4,
    monthly: 12,
};

/// The newest N states of any row are kept whatever the calendar math says, so a
/// row edited twice in one minute can still be stepped back through.
pub const DEFAULT_MIN_KEEP: u32 = 3;

/// One recorded state of one row's prose, at one moment.
///
/// Keyed on `(item_uid, role)` and **never** on a content `file_id` or a file
/// path: `file_id` is the store's `EntityId`, re-minted on every load, and a
/// prose path embeds both that id and the item's title slug. Only the uid is
/// durable across a save→load cycle, which is what makes two entries recorded in
/// different sessions comparable at all.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// RFC3339, UTC.
    pub at: String,
    pub item_uid: uuid::Uuid,
    pub role: ContentRole,
    /// blake3 hex of the prose — also the blob's file-name stem.
    pub hash: String,
    /// Uncompressed byte length, so a size-over-time reading needs no blob reads.
    pub bytes: u64,
    /// How many older states of this `(row, role)` retention has already removed.
    ///
    /// Carried by the **oldest surviving entry** of the key and zero on every
    /// other, so the running total outlives each sweep without a second file to
    /// keep in step with this one. [`thin`] is the only writer.
    ///
    /// It exists because thinning is otherwise **unprovable after the fact**. The
    /// survivors of a GFS sweep are one per bucket, which is exactly what a row
    /// edited once per bucket and never thinned at all looks like: nothing in the
    /// surviving set distinguishes them. So a surface wanting to say "there was
    /// more here once" could only have guessed, and the Versions dock's
    /// *"the earliest version on record"* had to be read as *"this is everything
    /// there ever was"*. This is the one fact that lets it say otherwise and be
    /// right — and, just as much, stay quiet on a row whose whole past is still
    /// on record.
    ///
    /// `#[serde(default)]`, so an `index.ron` written before this field existed
    /// reads back as zero — the honest value for it, since those sweeps recorded
    /// nothing about what they dropped and nothing is therefore claimed. Purely
    /// additive, so no `FORMAT_VERSION` bump, for the same reason
    /// `BinderItemFile::aliases` needed none.
    #[serde(default)]
    pub thinned_away: u32,
    //
    // There was a `pinned: bool` here, exempting one entry from `thin`, and it was
    // never set to `true` by anything: the shipped answer to "keep this version
    // whatever retention decides" is a pin on the **backup file**, held by path in
    // the app's backup settings. That is why the Versions dock offers no pin
    // control on a log row at all — a pin protects a file from a retention sweep,
    // and there is no file behind a log entry.
    //
    // So the flag was an unreachable branch inside the one function whose job is
    // deciding what to delete, exercised only by its own unit test. It is gone
    // rather than left as a promise the product does not make. An `index.ron`
    // written while it existed still loads: serde ignores the unknown field, which
    // `a_log_written_when_entries_carried_a_pin_still_loads` holds to.
}

/// The whole log: the ordered index plus the blobs it references.
///
/// `blobs` is loaded **eagerly** alongside the index, and that is load-bearing
/// rather than lazy-by-omission: `write_zip` packs a *fresh* staging directory, so
/// a blob left on disk instead of carried in memory would simply not be in the
/// next archive. An entry whose blob has gone missing is dropped on load, so the
/// invariant `write_folder` relies on — every referenced hash has bytes — always
/// holds. [`thin`] is what bounds the cost.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HistoryLog {
    /// Oldest first. Append-only in normal operation; [`thin`] is the only shrink.
    pub entries: Vec<HistoryEntry>,
    /// Prose keyed by blake3 hash. Written as `history/<hash>.djot`.
    pub blobs: BTreeMap<String, String>,
}

/// The hashable half of an entry's identity.
///
/// `ContentRole` is a generated entity enum deriving only `PartialEq`/`Eq` — no
/// `Hash`, no `Ord` — and adding a derive there would be stripped by the next
/// regeneration. So keys use the role's canonical **prose kind** instead
/// (`"scene"`, `"synopsis"`, …), which is already the on-disk authority for
/// naming a prose blob and is therefore exactly as stable as the file layout.
///
/// A non-prose role has no blob and never reaches here; `"other"` is a total
/// fallback rather than a silent panic.
type Key = (uuid::Uuid, &'static str);

fn role_key(uid: uuid::Uuid, role: &ContentRole) -> Key {
    (uid, super::slug::prose_kind(role).unwrap_or("other"))
}

impl HistoryLog {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The newest recorded hash for each `(uid, role)`.
    ///
    /// `entries` is oldest-first, so a later insert legitimately overwrites an
    /// earlier one and the last write wins.
    fn newest_by_key(&self) -> HashMap<Key, &str> {
        let mut out = HashMap::new();
        for e in &self.entries {
            out.insert(role_key(e.item_uid, &e.role), e.hash.as_str());
        }
        out
    }

    /// Every hash the index still references — what a blob GC must keep.
    pub fn referenced_hashes(&self) -> BTreeSet<String> {
        self.entries.iter().map(|e| e.hash.clone()).collect()
    }

    /// How many states of one `(row, role)` [`thin`] has removed from this log.
    ///
    /// `max` rather than "read the oldest entry", even though [`thin`]'s
    /// invariant is that only the oldest carries a non-zero tally: `entries` is
    /// append-only *in normal operation*, and a log carried through a restore has
    /// no such guarantee — the same reason [`crate::versions::LogVersions`]
    /// compares timestamps instead of trusting position. `max` is right under
    /// both orderings and cannot silently report zero for a row that has lost
    /// half its past.
    pub fn thinned_away(&self, uid: uuid::Uuid, role: &ContentRole) -> u32 {
        let key = role_key(uid, role);
        self.entries
            .iter()
            .filter(|e| role_key(e.item_uid, &e.role) == key)
            .map(|e| e.thinned_away)
            .max()
            .unwrap_or(0)
    }
}

/// Append an entry for every prose blob whose text differs from that row's newest
/// recorded state. Idempotent: recording twice without an edit in between adds
/// nothing.
///
/// Call from `save_work` only (see the module docs), after the bundle is built and
/// before it is written.
pub fn record(bundle: &mut WorkBundle, now: DateTime<Utc>) {
    let at = now.to_rfc3339();
    // Snapshot the comparison keys before touching `bundle.history`, so the
    // borrow of the old entries ends before the new ones are appended.
    let newest: HashMap<Key, String> = bundle
        .history
        .newest_by_key()
        .into_iter()
        .map(|(k, v)| (k, v.to_string()))
        .collect();

    let mut fresh: Vec<(HistoryEntry, String)> = Vec::new();
    for bb in &bundle.binders {
        for bi in &bb.items {
            let uid = bi.item.uid;
            for pr in &bi.item.prose_refs {
                let Some(text) = bi.prose.get(&pr.file_id) else {
                    continue; // a ref with no blob is `write_folder`'s error to raise
                };
                let hash = blake3::hash(text.as_bytes()).to_hex().to_string();
                if newest.get(&role_key(uid, &pr.role)) == Some(&hash) {
                    continue; // unchanged since the last save
                }
                fresh.push((
                    HistoryEntry {
                        at: at.clone(),
                        item_uid: uid,
                        role: pr.role.clone(),
                        hash,
                        bytes: text.len() as u64,
                        // A fresh state has lost nothing behind it; `thin` moves
                        // the key's running total onto whichever entry it leaves
                        // oldest, and a just-appended one is the newest.
                        thinned_away: 0,
                    },
                    text.clone(),
                ));
            }
        }
    }

    for (entry, text) in fresh {
        bundle
            .history
            .blobs
            .entry(entry.hash.clone())
            .or_insert(text);
        bundle.history.entries.push(entry);
    }
}

/// Thin the log **per `(item_uid, role)`** by the same GFS calendar math backup
/// retention uses, then drop every blob the surviving index no longer references.
///
/// Per-key rather than whole-log: a novel where one scene is rewritten forty times
/// and the rest is untouched must not lose the untouched rows' only recorded state
/// just because the busy scene filled the buckets.
///
/// Nothing here is exempt. Keeping one particular version whatever retention
/// decides is a promise about a **backup file**, made by pinning it in the Backups
/// list, and there is no file behind a log entry — see [`HistoryEntry`].
///
/// What it removes it also **counts**, into
/// [`HistoryEntry::thinned_away`](HistoryEntry#structfield.thinned_away) on
/// whichever entry the sweep leaves oldest for that key. Discarding an entry
/// outright, as this used to, made a thinned row indistinguishable from a row
/// that had never had more — which left every surface downstream unable to say
/// how far back its own record really reached.
pub fn thin(log: &mut HistoryLog, policy: &RetentionPolicy, min_keep: u32, now: DateTime<Utc>) {
    let mut by_key: HashMap<Key, Vec<usize>> = HashMap::new();
    for (i, e) in log.entries.iter().enumerate() {
        by_key
            .entry(role_key(e.item_uid, &e.role))
            .or_default()
            .push(i);
    }

    let mut keep: BTreeSet<usize> = BTreeSet::new();
    // What each key has lost in total — this sweep plus every sweep before it.
    // A key the sweep empties outright loses its tally with its last entry; that
    // needs an all-zero policy *and* `min_keep == 0`, under which the log holds
    // nothing to hang a count on and has nothing to say either.
    let mut lost: HashMap<Key, u32> = HashMap::new();
    for (key, indices) in &by_key {
        // `policy_keep_indices` speaks newest-first, and requires it: its bucket
        // walk keeps the **first** entry it meets in each of the most recent N
        // buckets, so a list that is not in that order keeps the wrong state and
        // deletes a legitimate one.
        //
        // Reversing `entries` only approximates it. The log is append-only in
        // normal operation, but a log carried through a restore has no such
        // guarantee — the same reason [`crate::versions::LogVersions`] compares
        // timestamps rather than trusting position, and the same reason this now
        // sorts. The reverse stays as the seed so that the stable sort's
        // tie-break is unchanged: a well-ordered log keeps exactly the survivors
        // it always did, and only a scrambled one behaves differently (better).
        // Parsed once and carried alongside the index. Sorting on a key computed
        // in the comparator would parse every entry again to rebuild `stamps`
        // from the sorted order — two full passes per key, on a sweep that runs
        // at every save.
        let mut newest_first: Vec<(usize, DateTime<Utc>)> = indices
            .iter()
            .rev()
            .map(|&i| (i, parse_at(&log.entries[i].at)))
            .collect();
        newest_first.sort_by_key(|&(_, at)| std::cmp::Reverse(at));
        let stamps: Vec<DateTime<Utc>> = newest_first.iter().map(|&(_, at)| at).collect();
        let survivors = policy_keep_indices(&stamps, policy, min_keep, now);
        // Carried forward rather than recomputed: an entry an earlier sweep
        // dropped left no other trace of itself, so the only surviving record of
        // it is the running total on whichever entry outlived it. `max` because
        // that entry may itself be among today's casualties.
        let carried = indices
            .iter()
            .map(|&i| log.entries[i].thinned_away)
            .max()
            .unwrap_or(0);
        lost.insert(
            *key,
            carried + indices.len().saturating_sub(survivors.len()) as u32,
        );
        for k in survivors {
            keep.insert(newest_first[k].0);
        }
    }

    let mut kept = Vec::with_capacity(keep.len());
    for (i, e) in log.entries.drain(..).enumerate() {
        if keep.contains(&i) {
            kept.push(e);
        }
    }
    log.entries = kept;

    // Exactly one entry per key carries the tally — the **oldest**, because that
    // is the edge a reader is standing at when they ask whether anything came
    // before it. Every other is cleared, so a later sweep that drops the oldest
    // cannot leave two entries each claiming the same losses. `entries` is
    // oldest-first, so the first sighting of a key is that key's oldest.
    let mut seen: HashMap<Key, ()> = HashMap::new();
    for e in log.entries.iter_mut() {
        let key = role_key(e.item_uid, &e.role);
        e.thinned_away = if seen.insert(key, ()).is_none() {
            lost.get(&key).copied().unwrap_or(0)
        } else {
            0
        };
    }

    let referenced = log.referenced_hashes();
    log.blobs.retain(|h, _| referenced.contains(h));
}

/// Read an existing bundle's history log, without parsing the rest of it.
///
/// **This is what makes the log survive a save at all.** The log lives in the
/// file, not in the store: `from_entities` builds a bundle out of store entities,
/// which have never heard of it, so every save starts from an empty log and would
/// overwrite the directory with nothing. Each save therefore has to read the log
/// back off the target and carry it forward — a read-modify-write, with the file
/// as the source of truth. That also means a crash between two saves costs at most
/// the entries the crashed save would have added, never the whole history.
///
/// A missing, unreadable, or legacy bundle yields an empty log rather than an
/// error: there is nothing to carry, which is a normal state (a brand-new project,
/// a first save after upgrading), not a failure.
pub fn load(path: &str) -> HistoryLog {
    use super::shape::{SkribShape, detect_shape, folder_root};
    match detect_shape(path) {
        Ok(SkribShape::ExplodedFolder) => load_folder(&folder_root(path)),
        Ok(SkribShape::ZipFile) => load_zip(std::path::Path::new(path)),
        _ => HistoryLog::default(),
    }
}

fn load_folder(root: &std::path::Path) -> HistoryLog {
    let Ok(text) = std::fs::read_to_string(root.join(HISTORY_INDEX)) else {
        return HistoryLog::default();
    };
    let entries: Vec<HistoryEntry> = match ron::from_str(&text) {
        Ok(e) => e,
        Err(_) => return HistoryLog::default(),
    };
    let mut blobs = BTreeMap::new();
    for hash in entries.iter().map(|e| e.hash.clone()) {
        if blobs.contains_key(&hash) {
            continue;
        }
        if let Ok(t) = std::fs::read_to_string(root.join(blob_relpath(&hash))) {
            blobs.insert(hash, t);
        }
    }
    prune_unbacked(entries, blobs)
}

fn load_zip(path: &std::path::Path) -> HistoryLog {
    use std::io::Read;
    let Ok(file) = std::fs::File::open(path) else {
        return HistoryLog::default();
    };
    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return HistoryLog::default();
    };
    let mut text = String::new();
    match archive.by_name(HISTORY_INDEX) {
        Ok(mut entry) => {
            if entry.read_to_string(&mut text).is_err() {
                return HistoryLog::default();
            }
        }
        Err(_) => return HistoryLog::default(), // no history in this bundle yet
    }
    let entries: Vec<HistoryEntry> = match ron::from_str(&text) {
        Ok(e) => e,
        Err(_) => return HistoryLog::default(),
    };
    let mut blobs = BTreeMap::new();
    for hash in entries.iter().map(|e| e.hash.clone()) {
        if blobs.contains_key(&hash) {
            continue;
        }
        // Each `ZipFile` holds an exclusive borrow of the archive, so it must be
        // dropped before the next `by_name` — which it is, at the end of this arm.
        if let Ok(mut entry) = archive.by_name(&blob_relpath(&hash)) {
            let mut t = String::new();
            if entry.read_to_string(&mut t).is_ok() {
                blobs.insert(hash, t);
            }
        }
    }
    prune_unbacked(entries, blobs)
}

/// Drop entries whose blob is missing, so an in-memory log never claims prose it
/// cannot produce — which is exactly what `write_folder` asserts before writing.
fn prune_unbacked(entries: Vec<HistoryEntry>, blobs: BTreeMap<String, String>) -> HistoryLog {
    let entries = entries
        .into_iter()
        .filter(|e| blobs.contains_key(&e.hash))
        .collect();
    HistoryLog { entries, blobs }
}

/// An unparseable timestamp sorts as the epoch rather than aborting a thin.
///
/// A single corrupt row must not be able to strand the whole log at its current
/// size forever; treating it as ancient makes it the first thing dropped.
fn parse_at(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| DateTime::UNIX_EPOCH)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(at: &str, uid: uuid::Uuid, role: ContentRole, hash: &str) -> HistoryEntry {
        HistoryEntry {
            at: at.to_string(),
            item_uid: uid,
            role,
            hash: hash.to_string(),
            bytes: hash.len() as u64,
            thinned_away: 0,
        }
    }

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-08-07T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn gfs() -> RetentionPolicy {
        RetentionPolicy::Gfs {
            hourly: 24,
            daily: 7,
            weekly: 4,
            monthly: 12,
        }
    }

    #[test]
    fn thinning_is_per_item_and_role_not_whole_log() {
        let busy = uuid::Uuid::from_u128(1);
        let quiet = uuid::Uuid::from_u128(2);
        let mut log = HistoryLog::default();
        // One row rewritten many times inside a single hour…
        for m in 0..40 {
            log.entries.push(entry(
                &format!("2026-08-07T09:{m:02}:00Z"),
                busy,
                ContentRole::SceneText,
                &format!("busy{m}"),
            ));
        }
        // …and another row touched once, long ago.
        log.entries.push(entry(
            "2026-02-01T09:00:00Z",
            quiet,
            ContentRole::SceneText,
            "quiet",
        ));

        thin(&mut log, &gfs(), 1, now());

        assert!(
            log.entries.iter().any(|e| e.hash == "quiet"),
            "a quiet row's only recorded state must survive a busy row's churn",
        );
        assert!(
            log.entries.iter().filter(|e| e.item_uid == busy).count() < 40,
            "the busy row must actually thin",
        );
    }

    /// **The fact a sweep would otherwise take with it.**
    ///
    /// After thinning, the survivors of a busy row are one per bucket — exactly
    /// what a row edited once per bucket and never thinned looks like. Without a
    /// tally recorded here, nothing downstream could tell the two apart, and
    /// "the earliest version on record" would read as "this is all there ever
    /// was" on a row that had lost thirty states.
    #[test]
    fn thinning_records_how_much_it_removed() {
        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        for m in 0..40 {
            log.entries.push(entry(
                &format!("2026-08-07T09:{m:02}:00Z"),
                uid,
                ContentRole::SceneText,
                &format!("h{m}"),
            ));
        }
        let before = log.entries.len();
        thin(&mut log, &gfs(), 1, now());
        let dropped = before - log.entries.len();
        assert!(dropped > 0, "the fixture must actually thin");
        assert_eq!(
            log.thinned_away(uid, &ContentRole::SceneText),
            dropped as u32,
            "the tally must be exactly what the sweep removed",
        );
    }

    /// Only the **oldest** survivor carries it, so a later sweep that drops the
    /// oldest cannot leave two entries each claiming the same losses.
    #[test]
    fn only_the_oldest_surviving_entry_carries_the_tally() {
        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        for m in 0..40 {
            log.entries.push(entry(
                &format!("2026-08-07T09:{m:02}:00Z"),
                uid,
                ContentRole::SceneText,
                &format!("h{m}"),
            ));
        }
        thin(&mut log, &gfs(), 1, now());
        let carrying: Vec<&HistoryEntry> =
            log.entries.iter().filter(|e| e.thinned_away > 0).collect();
        assert_eq!(carrying.len(), 1, "exactly one entry holds the count");
        assert_eq!(
            carrying[0].at, log.entries[0].at,
            "and it is the oldest survivor",
        );
    }

    /// **The invariant a running total lives or dies by.** A later sweep drops
    /// the entry the first one hung its count on, so the count has to move
    /// forward with it or every earlier sweep's losses vanish silently.
    #[test]
    fn the_tally_survives_the_sweep_that_drops_the_entry_holding_it() {
        fn monthly_entry(year: i32, month: u32, uid: uuid::Uuid) -> HistoryEntry {
            entry(
                &format!("{year}-{month:02}-01T09:00:00Z"),
                uid,
                ContentRole::SceneText,
                &format!("h{year}{month:02}"),
            )
        }
        fn at(s: &str) -> DateTime<Utc> {
            DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
        }

        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        // Twenty months of history, one state each. `monthly: 12` keeps the
        // twelve most recent month-buckets, so eight go.
        for (y, m) in (1..=12)
            .map(|m| (2025, m))
            .chain((1..=8).map(|m| (2026, m)))
        {
            log.entries.push(monthly_entry(y, m, uid));
        }
        thin(&mut log, &gfs(), 3, at("2026-08-07T12:00:00Z"));
        assert_eq!(log.entries.len(), 12, "twelve month-buckets survive");
        assert_eq!(log.thinned_away(uid, &ContentRole::SceneText), 8);
        let carrier = log.entries[0].hash.clone();

        // A year of further saves, then another sweep: the twelve newest months
        // are now 2026-09..2027-08, so every survivor of the first sweep goes —
        // including the one holding its count.
        for (y, m) in (9..=12)
            .map(|m| (2026, m))
            .chain((1..=8).map(|m| (2027, m)))
        {
            log.entries.push(monthly_entry(y, m, uid));
        }
        thin(&mut log, &gfs(), 3, at("2027-08-07T12:00:00Z"));
        assert!(
            !log.entries.iter().any(|e| e.hash == carrier),
            "the fixture must actually drop the entry carrying the tally",
        );
        assert_eq!(
            log.thinned_away(uid, &ContentRole::SceneText),
            20,
            "the running total must cover both sweeps, not just the last",
        );
    }

    /// **A log carried through a restore is not in append order**, and
    /// `policy_keep_indices` keeps the *first* entry it meets in each bucket —
    /// so feeding it position order deletes the state a writer would have wanted
    /// and keeps the one they would not.
    #[test]
    fn a_scrambled_log_still_keeps_its_newest_state_per_bucket() {
        let uid = uuid::Uuid::from_u128(1);
        let stamps = [
            "2026-08-07T09:10:00Z",
            "2026-08-07T09:50:00Z", // the newest of the 09:00 bucket…
            "2026-08-07T09:30:00Z",
        ];
        let mut log = HistoryLog::default();
        // …recorded in the middle, which append order would put in the middle too.
        for (n, at) in stamps.iter().enumerate() {
            log.entries
                .push(entry(at, uid, ContentRole::SceneText, &format!("h{n}")));
        }
        thin(&mut log, &gfs(), 1, now());
        assert_eq!(log.entries.len(), 1, "one hour bucket, one survivor");
        assert_eq!(
            log.entries[0].at, "2026-08-07T09:50:00Z",
            "the survivor must be the bucket's newest state, whatever order the \
             entries happen to sit in",
        );
        assert_eq!(log.thinned_away(uid, &ContentRole::SceneText), 2);
    }

    /// Thinning with nothing to drop must not invent a loss.
    #[test]
    fn a_sweep_that_removes_nothing_claims_nothing() {
        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        log.entries.push(entry(
            "2026-08-07T11:00:00Z",
            uid,
            ContentRole::SceneText,
            "only",
        ));
        thin(&mut log, &gfs(), 3, now());
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.thinned_away(uid, &ContentRole::SceneText), 0);
    }

    /// The tally is per `(row, role)` like the sweep itself: a busy scene body
    /// must not make its own synopsis look thinned.
    #[test]
    fn the_tally_is_per_row_and_role() {
        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        for m in 0..40 {
            log.entries.push(entry(
                &format!("2026-08-07T09:{m:02}:00Z"),
                uid,
                ContentRole::SceneText,
                &format!("body{m}"),
            ));
        }
        log.entries.push(entry(
            "2026-08-07T09:00:00Z",
            uid,
            ContentRole::SynopsisText,
            "synopsis",
        ));
        thin(&mut log, &gfs(), 1, now());
        assert!(log.thinned_away(uid, &ContentRole::SceneText) > 0);
        assert_eq!(
            log.thinned_away(uid, &ContentRole::SynopsisText),
            0,
            "an untouched role has lost nothing",
        );
    }

    /// An `index.ron` written before the tally existed reads back as zero — the
    /// honest value, since those sweeps recorded nothing about what they dropped.
    #[test]
    fn a_log_written_before_the_tally_existed_reads_back_as_no_claim() {
        let written = r#"[
            (
                at: "2026-08-07T09:00:00Z",
                item_uid: "00000000-0000-0000-0000-000000000001",
                role: SceneText,
                hash: "abc123",
                bytes: 12,
            ),
        ]"#;
        let entries: Vec<HistoryEntry> =
            ron::from_str(written).expect("a missing additive field must default");
        assert_eq!(entries[0].thinned_away, 0);
    }

    /// An `index.ron` written while [`HistoryEntry`] still carried a `pinned` flag
    /// must still load.
    ///
    /// This is the whole risk of having removed the field, and it is not a
    /// theoretical one: [`load`] degrades a parse failure to an **empty log**
    /// rather than an error, so getting it wrong would silently throw away every
    /// existing project's per-save history with nothing shown to the writer.
    #[test]
    fn a_log_written_when_entries_carried_a_pin_still_loads() {
        let written = r#"[
            (
                at: "2026-08-07T09:00:00Z",
                item_uid: "00000000-0000-0000-0000-000000000001",
                role: SceneText,
                hash: "abc123",
                bytes: 12,
                pinned: true,
            ),
        ]"#;
        let entries: Vec<HistoryEntry> =
            ron::from_str(written).expect("a field this version does not know must be ignored");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].hash, "abc123");
    }

    #[test]
    fn thinning_collects_blobs_the_index_no_longer_references() {
        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        for m in 0..10 {
            let h = format!("h{m}");
            log.entries.push(entry(
                &format!("2026-01-0{}T09:00:00Z", 1 + m % 9),
                uid,
                ContentRole::SceneText,
                &h,
            ));
            log.blobs.insert(h, format!("text {m}"));
        }
        log.blobs.insert("orphan".into(), "never referenced".into());

        thin(&mut log, &RetentionPolicy::KeepLastN { n: 2 }, 1, now());

        let referenced = log.referenced_hashes();
        assert_eq!(
            log.blobs.keys().cloned().collect::<BTreeSet<_>>(),
            referenced,
            "every surviving blob is referenced, and every referenced blob survives",
        );
        assert!(!log.blobs.contains_key("orphan"));
    }

    #[test]
    fn a_corrupt_timestamp_is_dropped_first_rather_than_stranding_the_log() {
        let uid = uuid::Uuid::from_u128(1);
        let mut log = HistoryLog::default();
        log.entries
            .push(entry("not-a-date", uid, ContentRole::SceneText, "bad"));
        log.entries.push(entry(
            "2026-08-07T11:00:00Z",
            uid,
            ContentRole::SceneText,
            "good",
        ));

        thin(&mut log, &RetentionPolicy::KeepLastN { n: 1 }, 1, now());

        assert_eq!(
            log.entries
                .iter()
                .map(|e| e.hash.as_str())
                .collect::<Vec<_>>(),
            vec!["good"],
            "the unparseable row sorts as ancient and goes first",
        );
    }
}
