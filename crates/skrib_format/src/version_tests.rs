// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Read-path tests: pulling one row's past out of a real bundle.
//!
//! These need bundles actually written to disk, for the same reason
//! `history_tests` does — the whole contract is about what can be recovered from
//! a file, and the zip and folder shapes recover it by different mechanics.

use super::bundle::{ShapeTag, WorkBundle};
use super::history;
use super::{SkribShape, read_bundle, write_bundle};

/// A two-scene project, built through the same helper the format's own tests use.
fn bundle() -> WorkBundle {
    super::tests::build_bundle(ShapeTag::Zip)
}

/// Write `b`, read it back.
fn round_trip(b: &WorkBundle, shape: SkribShape) -> (tempfile::TempDir, String, WorkBundle) {
    let dir = tempfile::tempdir().expect("tmp");
    let path = dir
        .path()
        .join("Novel.skrib")
        .to_string_lossy()
        .into_owned();
    write_bundle(&path, shape, b).expect("write");
    let back = read_bundle(&path).expect("read");
    (dir, path, back)
}

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339("2026-08-07T12:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc)
}

use super::versions::{BackupVersions, LogVersions, SourceKind, VersionSource};

/// Write a bundle and return a source that reads versions out of that directory.
fn backup_source(dir: &std::path::Path, project: &str) -> BackupVersions {
    BackupVersions {
        directories: vec![dir.to_string_lossy().into_owned()],
        work_unique_id: String::new(),
        project_path: project.to_string(),
    }
}

#[test]
fn indexing_a_bundle_reads_rows_without_extracting_it() {
    for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
        let b = bundle();
        let (_dir, path, _back) = round_trip(&b, shape);

        let src = backup_source(std::path::Path::new("/unused"), &path);
        let v = super::versions::VersionRef {
            path: std::path::PathBuf::from(&path),
            taken_at: now(),
            source: SourceKind::Backup,
        };
        let index = src.index(&v).expect("index");

        let expected: usize = b.binders.iter().map(|bb| bb.items.len()).sum();
        assert_eq!(index.rows.len(), expected, "{shape:?} row count");
        assert!(
            index.rows.iter().all(|r| !r.uid.is_nil()),
            "{shape:?}: every row must carry the durable uid the timeline correlates on",
        );
        assert!(
            index.rows.iter().any(|r| !r.prose.is_empty()),
            "{shape:?}: at least one row must expose a prose blob",
        );
    }
}

#[test]
fn every_index_reports_a_blobs_size_without_decompressing_it() {
    let b = bundle();

    let (_zd, zpath, _zb) = round_trip(&b, SkribShape::ZipFile);
    let zsrc = backup_source(std::path::Path::new("/unused"), &zpath);
    let zv = super::versions::VersionRef {
        path: std::path::PathBuf::from(&zpath),
        taken_at: now(),
        source: SourceKind::Backup,
    };
    let zrows = zsrc.index(&zv).expect("zip index").rows;
    let zstamp = zrows
        .iter()
        .flat_map(|r| &r.prose)
        .map(|(_, _, s)| *s)
        .next()
        .expect("a prose blob");
    assert!(
        zstamp.bytes > 0,
        "a zip entry's uncompressed size comes straight from the central \
         directory, with nothing decompressed",
    );

    let (_fd, fpath, _fb) = round_trip(&b, SkribShape::ExplodedFolder);
    let fsrc = backup_source(std::path::Path::new("/unused"), &fpath);
    let fv = super::versions::VersionRef {
        path: std::path::PathBuf::from(&fpath),
        taken_at: now(),
        source: SourceKind::Backup,
    };
    let frows = fsrc.index(&fv).expect("folder index").rows;
    let fstamp = frows
        .iter()
        .flat_map(|r| &r.prose)
        .map(|(_, _, s)| *s)
        .next()
        .expect("a prose blob");
    assert!(fstamp.bytes > 0, "and a folder bundle reports one too");
}

#[test]
fn prose_is_readable_from_a_version_by_its_blob_path() {
    for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
        let b = bundle();
        let (_dir, path, _back) = round_trip(&b, shape);
        let src = backup_source(std::path::Path::new("/unused"), &path);
        let v = super::versions::VersionRef {
            path: std::path::PathBuf::from(&path),
            taken_at: now(),
            source: SourceKind::Backup,
        };
        let index = src.index(&v).expect("index");
        let (_, blob, _) = index
            .rows
            .iter()
            .flat_map(|r| &r.prose)
            .next()
            .expect("a prose blob");

        let text = src.prose(&v, blob).expect("prose");
        assert!(
            !text.is_empty(),
            "{shape:?}: reading one blob by its bundle-relative path must work — \
             that path IS the zip entry name",
        );
    }
}

#[test]
fn a_renamed_binder_still_indexes_because_entries_are_enumerated_not_addressed() {
    // The binder's directory name derives from its own title, so a rename moves
    // every prose path with it. Enumeration is what makes that a non-event — a
    // by-name lookup could never have worked, since the name that forms the
    // directory lives *inside* the file you would be looking for.
    //
    // Renamed consistently (name *and* the paths that derive from it), because
    // that is what `from_entities` would produce; changing only the name would
    // build a bundle the writer could never actually have.
    let mut b = bundle();
    let old_dir = super::slug::binder_dir_name(0, &b.binders[0].binder.name);
    b.binders[0].binder.name = "A Completely Different Name".to_string();
    let new_dir = super::slug::binder_dir_name(0, &b.binders[0].binder.name);
    assert_ne!(
        old_dir, new_dir,
        "the rename must actually move the directory"
    );
    for item in b.binders[0].items.iter_mut() {
        for pr in item.item.prose_refs.iter_mut() {
            pr.path = pr.path.replace(&old_dir, &new_dir);
        }
    }
    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);

    let src = backup_source(std::path::Path::new("/unused"), &path);
    let v = super::versions::VersionRef {
        path: std::path::PathBuf::from(&path),
        taken_at: now(),
        source: SourceKind::Backup,
    };
    let index = src.index(&v).expect("index after rename");
    assert!(!index.rows.is_empty());
    assert!(index.rows.iter().all(|r| !r.uid.is_nil()));
}

/// **A timeline reports what its sources have thrown away.**
///
/// The one boundary fact that cannot be read off the walk: a thinned moment
/// leaves nothing to examine, and `RowAt::Silent` is deliberately inert so that a
/// routine sweep does not read as a gap. Without the log's own tally reaching
/// `Timeline`, a row down to three surviving states after a year of daily edits
/// would be indistinguishable from one that never had more.
#[test]
fn a_timeline_carries_what_the_log_thinned_away() {
    let mut b = bundle();
    let (uid, fid) = a_scene(&b);

    // Forty states of one scene inside one hour, each recorded.
    for n in 0..40 {
        let item = b
            .binders
            .iter_mut()
            .flat_map(|bb| &mut bb.items)
            .find(|i| i.item.uid == uid)
            .expect("the scene");
        item.prose.insert(fid, format!("wording {n}"));
        history::record(&mut b, now() + chrono::Duration::minutes(n));
    }
    let recorded = b.history.entries.len();
    history::thin(
        &mut b.history,
        &history::DEFAULT_POLICY,
        history::DEFAULT_MIN_KEEP,
        now() + chrono::Duration::hours(2),
    );
    let dropped = recorded - b.history.entries.len();
    assert!(dropped > 0, "the fixture must actually thin");

    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);
    let log = LogVersions::open(&path);
    let t = timeline_for(&[&log], uid, &ContentRole::SceneText).expect("timeline");
    assert_eq!(
        t.thinned_away, dropped as u32,
        "the timeline must report exactly what the log removed for this row",
    );
}

/// And says nothing when nothing was removed — the half that keeps the line it
/// drives from becoming permanent background noise.
#[test]
fn a_timeline_claims_no_thinning_when_none_happened() {
    let mut b = bundle();
    let (uid, _fid) = a_scene(&b);
    history::record(&mut b, now());

    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);
    let log = LogVersions::open(&path);
    let t = timeline_for(&[&log], uid, &ContentRole::SceneText).expect("timeline");
    assert_eq!(t.thinned_away, 0);
}

#[test]
fn the_log_source_lists_one_version_per_distinct_save_moment() {
    let mut b = bundle();
    history::record(&mut b, now());
    history::record(&mut b, now()); // same instant, no edits — adds nothing

    // Edit and record again, an hour later.
    let item = b
        .binders
        .iter_mut()
        .flat_map(|bb| &mut bb.items)
        .find(|i| !i.item.prose_refs.is_empty())
        .expect("a row with prose");
    let fid = item.item.prose_refs[0].file_id;
    item.prose.insert(fid, "rewritten".to_string());
    history::record(&mut b, now() + chrono::Duration::hours(1));

    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);
    let src = LogVersions::open(&path);
    let list = src.list().expect("list");

    assert_eq!(list.len(), 2, "two distinct moments were recorded");
    assert!(
        list[0].taken_at > list[1].taken_at,
        "versions list newest first",
    );
    assert!(list.iter().all(|v| v.source == SourceKind::Log));
}

#[test]
fn a_log_version_reports_each_rows_newest_state_at_or_before_that_moment() {
    let mut b = bundle();
    history::record(&mut b, now());
    let first_moment = now();

    let item = b
        .binders
        .iter_mut()
        .flat_map(|bb| &mut bb.items)
        .find(|i| !i.item.prose_refs.is_empty())
        .expect("a row with prose");
    let uid = item.item.uid;
    let fid = item.item.prose_refs[0].file_id;
    item.prose.insert(fid, "the later wording".to_string());
    history::record(&mut b, now() + chrono::Duration::hours(1));

    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);
    let src = LogVersions::open(&path);
    let list = src.list().expect("list");

    let earlier = list.iter().find(|v| v.taken_at == first_moment).unwrap();
    let idx = src.index(earlier).expect("index");
    let row = idx
        .row(uid)
        .expect("the edited row exists at the earlier moment");
    let (_, blob, _) = &row.prose[0];
    assert_ne!(
        src.prose(earlier, blob).expect("prose"),
        "the later wording",
        "asking for an earlier moment must not hand back the newer text",
    );

    let latest = &list[0];
    let idx = src.index(latest).expect("index");
    let row = idx.row(uid).expect("row");
    let (_, blob, _) = &row.prose[0];
    assert_eq!(src.prose(latest, blob).expect("prose"), "the later wording");
}

/// **The log is a witness to the moments it recorded, and to nothing else.**
///
/// It answers [`RowAt::Present`] exactly where it wrote an entry, and
/// [`RowAt::Silent`] everywhere else — never [`RowAt::Absent`]. `index`, which
/// reconstructs "the project as of then" for the Timeline band, deliberately still
/// carries a state forward; this does not, and the two are different questions.
#[test]
fn the_log_speaks_only_for_the_moments_it_recorded() {
    use super::versions::RowAt;

    let mut b = bundle();
    let item = b
        .binders
        .iter_mut()
        .flat_map(|bb| &mut bb.items)
        .find(|i| !i.item.prose_refs.is_empty())
        .expect("a row with prose");
    let uid = item.item.uid;
    let role = item.item.prose_refs[0].role.clone();
    history::record(&mut b, now());

    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);
    let src = LogVersions::open(&path);
    let at = |when| super::versions::VersionRef {
        path: std::path::PathBuf::from(&path),
        taken_at: when,
        source: SourceKind::Log,
    };

    assert!(
        matches!(
            src.row_at(&at(now()), uid, &role).expect("row_at"),
            RowAt::Present { .. }
        ),
        "the moment it recorded is the moment it can speak for",
    );
    assert_eq!(
        src.row_at(&at(now() - chrono::Duration::days(365)), uid, &role)
            .expect("row_at"),
        RowAt::Silent,
        "before its record begins, the log knows nothing — and `thin` is designed \
         to drop old states, so this is not evidence the row did not exist",
    );
    assert_eq!(
        src.row_at(&at(now() + chrono::Duration::days(365)), uid, &role)
            .expect("row_at"),
        RowAt::Silent,
        "and after, carrying the state forward would report a deleted row as still \
         present at every later save",
    );

    // `index` is the other question, and still answers it.
    let idx = src
        .index(&at(now() + chrono::Duration::days(365)))
        .expect("index");
    assert!(
        idx.row(uid).is_some(),
        "'the project as of then' still carries the newest earlier state forward",
    );
}

/// **The bug this guards.** A backup proves a row was deleted; the writer must be
/// told. The log records prose, so a deleted row simply stops gaining entries and
/// its last recorded state sits there forever. Read as testimony about *existence*,
/// that state reported the row as present at every subsequent save — and since
/// saves outnumber backups, the newest examined moment was always one of them, so
/// the deletion boundary the backups had established was cleared again every time.
#[test]
fn a_deletion_a_backup_proves_is_not_erased_by_the_logs_stale_last_state() {
    let dir = tempfile::tempdir().expect("tmp");
    let full = bundle();
    let (uid, _fid) = a_scene(&full);

    // The row exists, is saved (so the log records it), and is then removed.
    let mut recorded = full.clone();
    history::record(&mut recorded, now());
    let mut later = recorded.clone();
    for bb in later.binders.iter_mut() {
        bb.items.retain(|i| i.item.uid != uid);
    }
    // A save after the deletion: the log gains a moment, but nothing for this row.
    let other = later
        .binders
        .iter_mut()
        .flat_map(|bb| &mut bb.items)
        .find(|i| !i.item.prose_refs.is_empty())
        .expect("another row with prose");
    let fid = other.item.prose_refs[0].file_id;
    other
        .prose
        .insert(fid, "some other row moved on".to_string());
    history::record(&mut later, now() + chrono::Duration::days(3));

    write_backup(&recorded, dir.path(), now() + chrono::Duration::days(1), 1);
    write_backup(&later, dir.path(), now() + chrono::Duration::days(2), 2);

    let (_pdir, project, _back) = round_trip(&later, SkribShape::ZipFile);
    let log = LogVersions::open(&project);
    let backups = backups_in(dir.path(), &full);
    let sources: [&dyn super::versions::VersionSource; 2] = [&log, &backups];
    let t = timeline_for(&sources, uid, &ContentRole::SceneText).expect("timeline");

    assert!(
        t.deleted_after.is_some(),
        "a row a backup proves was removed must still say so once the log — which \
         cannot see deletions at all — is merged in beside it",
    );
}

/// A moment a source declines to speak for must be **transparent**, not a gap.
///
/// Treating it as a gap (the way a genuinely unreadable backup is treated) would
/// clear the run and re-enter unchanged prose as a fresh change. Saves are far more
/// frequent than backups and most saves touch other rows, so a silent moment sits
/// between almost every pair of backups — padding every timeline with duplicates,
/// which is the exact failure `changes` exists to prevent.
#[test]
fn a_silent_moment_does_not_split_one_change_into_two() {
    let dir = tempfile::tempdir().expect("tmp");
    let mut b = bundle();
    let (uid, _fid) = a_scene(&b);

    // A save that records *another* row, between the two backups. The log lists
    // that moment; for our row it has nothing to say about it.
    let other = b
        .binders
        .iter_mut()
        .flat_map(|bb| &mut bb.items)
        .find(|i| !i.item.prose_refs.is_empty() && i.item.uid != uid)
        .expect("another row with prose");
    let fid = other.item.prose_refs[0].file_id;
    other
        .prose
        .insert(fid, "a different row changed".to_string());
    history::record(&mut b, now() + chrono::Duration::hours(36));

    write_backup(&b, dir.path(), now() + chrono::Duration::days(1), 1);
    write_backup(&b, dir.path(), now() + chrono::Duration::days(2), 2);

    let (_pdir, project, _back) = round_trip(&b, SkribShape::ZipFile);
    let log = LogVersions::open(&project);
    let backups = backups_in(dir.path(), &b);
    let sources: [&dyn super::versions::VersionSource; 2] = [&log, &backups];
    let t = timeline_for(&sources, uid, &ContentRole::SceneText).expect("timeline");

    let hashes: Vec<_> = t.changes.iter().map(|c| c.hash.clone()).collect();
    assert_eq!(
        hashes.len(),
        hashes
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        "the same prose must not be entered twice because a source stayed silent \
         between two backups holding it: {hashes:?}",
    );
    assert_eq!(
        t.absent_at, None,
        "and a silence must not be mistaken for proof the row did not exist",
    );
}

#[test]
fn historical_comments_come_back_with_the_prose_they_annotate() {
    // Nothing else in the category offers this, and it costs one extra entry read:
    // the sidecar's name is derivable from the blob's own.
    let b = bundle();
    let (_dir, path, _back) = round_trip(&b, SkribShape::ZipFile);
    let src = backup_source(std::path::Path::new("/unused"), &path);
    let v = super::versions::VersionRef {
        path: std::path::PathBuf::from(&path),
        taken_at: now(),
        source: SourceKind::Backup,
    };
    let index = src.index(&v).expect("index");
    let (_, blob, _) = index
        .rows
        .iter()
        .flat_map(|r| &r.prose)
        .next()
        .expect("a prose blob");

    // The fixture has no comments; the contract is that this is an empty answer,
    // never an error, because "no sidecar" is the common case.
    assert!(src.comments(&v, blob).expect("comments").is_empty());
}

// ── change detection ───────────────────────────────────────────────────────────

use super::changes::{Timeline, timeline_for};
use common::entities::ContentRole;

/// Write `b` as a *backup* into `dir`, stamped at `at`, and return its path.
///
/// Backups are correlated on `Work.unique_id` and dated from the manifest, so a
/// realistic fixture has to go through `mark_as_backup` rather than just dropping
/// a `.skrib` in a folder.
fn write_backup(
    b: &WorkBundle,
    dir: &std::path::Path,
    at: chrono::DateTime<chrono::Utc>,
    n: u32,
) -> String {
    let mut copy = b.clone();
    super::mark_as_backup(&mut copy, "/original/Novel.skrib".to_string(), at);
    let path = dir
        .join(format!("Novel-2026080{n}-100000.skrib"))
        .to_string_lossy()
        .into_owned();
    write_bundle(&path, SkribShape::ZipFile, &copy).expect("write backup");
    path
}

/// The uid and role of a row that actually has scene prose in the fixture.
fn a_scene(b: &WorkBundle) -> (uuid::Uuid, u64) {
    for bb in &b.binders {
        for item in &bb.items {
            for pr in &item.item.prose_refs {
                if pr.role == ContentRole::SceneText {
                    return (item.item.uid, pr.file_id);
                }
            }
        }
    }
    panic!("the fixture must contain a scene");
}

fn set_scene(b: &mut WorkBundle, fid: u64, text: &str) {
    for bb in b.binders.iter_mut() {
        for item in bb.items.iter_mut() {
            if item.prose.contains_key(&fid) {
                item.prose.insert(fid, text.to_string());
            }
        }
    }
}

/// Correlate on the fixture's own `unique_id`, which is how the real thing works:
/// `manifest_matches` only falls back to the recorded original path when the
/// bundle carries no uid at all, and every real project carries one.
fn backups_in(dir: &std::path::Path, b: &WorkBundle) -> BackupVersions {
    BackupVersions {
        directories: vec![dir.to_string_lossy().into_owned()],
        work_unique_id: b.manifest.work.unique_id.clone(),
        project_path: "/original/Novel.skrib".to_string(),
    }
}

#[test]
fn three_backups_of_an_untouched_scene_collapse_to_one_change() {
    // This is the whole point of the module: a scene nobody edited appears,
    // unchanged, in every backup taken since it was written. Listing all three is
    // the "excessive scrolling through timestamps" failure.
    let dir = tempfile::tempdir().expect("tmp");
    let b = bundle();
    let (uid, _fid) = a_scene(&b);
    for n in 1..=3 {
        write_backup(&b, dir.path(), now() + chrono::Duration::days(n as i64), n);
    }

    let src = backups_in(dir.path(), &b);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert_eq!(
        t.changes.len(),
        1,
        "three identical recordings are one state, not three versions",
    );
    assert!(t.unreadable.is_empty());
}

#[test]
fn each_real_edit_adds_exactly_one_change_newest_first() {
    let dir = tempfile::tempdir().expect("tmp");
    let mut b = bundle();
    let (uid, fid) = a_scene(&b);

    set_scene(&mut b, fid, "the first wording");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(1), 1);
    write_backup(&b, dir.path(), now() + chrono::Duration::days(2), 2); // no edit
    set_scene(&mut b, fid, "the second wording");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(3), 3);

    let src = backups_in(dir.path(), &b);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert_eq!(t.changes.len(), 2, "two wordings, two changes");
    assert!(
        t.changes[0].at > t.changes[1].at,
        "a timeline reads newest first",
    );
    assert_ne!(t.changes[0].hash, t.changes[1].hash);
}

#[test]
fn a_reverted_edit_is_a_change_in_its_own_right() {
    // Going back to earlier wording is something the writer did, at a moment, and
    // must appear as such — even though the content matches an older state.
    let dir = tempfile::tempdir().expect("tmp");
    let mut b = bundle();
    let (uid, fid) = a_scene(&b);

    set_scene(&mut b, fid, "original");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(1), 1);
    set_scene(&mut b, fid, "experiment");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(2), 2);
    set_scene(&mut b, fid, "original");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(3), 3);

    let src = backups_in(dir.path(), &b);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert_eq!(t.changes.len(), 3, "revert included");
    assert_eq!(
        t.changes[0].hash, t.changes[2].hash,
        "the revert restores the original content exactly",
    );
}

/// The boundary is stated out loud — and it names the moment that **proves** it.
///
/// The date reported must be the backup the row was missing from, never the one it
/// first appeared in. Between those two moments the row may have been written at
/// any time, so "didn't exist" said of the later one is a claim about a stretch
/// nothing on disk covers. Getting this wrong is invisible in a one-day-apart test
/// fixture and wrong by weeks in a real project, which is why the two backups here
/// are three days apart and the assertion is on the value, not on `is_some`.
#[test]
fn a_row_absent_from_the_oldest_backups_reports_the_moment_it_was_proved_absent() {
    let dir = tempfile::tempdir().expect("tmp");
    let full = bundle();
    let (uid, _fid) = a_scene(&full);

    // An older backup in which that row simply is not there yet.
    let mut earlier = full.clone();
    for bb in earlier.binders.iter_mut() {
        bb.items.retain(|i| i.item.uid != uid);
    }
    let absent_moment = now() + chrono::Duration::days(1);
    let seen_moment = now() + chrono::Duration::days(4);
    write_backup(&earlier, dir.path(), absent_moment, 1);
    write_backup(&full, dir.path(), seen_moment, 2);

    let src = backups_in(dir.path(), &full);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert_eq!(t.changes.len(), 1);
    assert_eq!(
        t.absent_at,
        Some(absent_moment),
        "the boundary must name the backup the row was missing from — reporting \
         the first sighting instead claims it did not exist during the three days \
         between, which nothing examined here can support",
    );
    assert_eq!(
        t.changes[0].at, seen_moment,
        "precondition: the two moments really are distinguishable",
    );
    assert_eq!(t.deleted_after, None);
}

#[test]
fn a_row_missing_from_the_newest_backup_reports_that_it_was_deleted() {
    let dir = tempfile::tempdir().expect("tmp");
    let full = bundle();
    let (uid, _fid) = a_scene(&full);

    let mut later = full.clone();
    for bb in later.binders.iter_mut() {
        bb.items.retain(|i| i.item.uid != uid);
    }
    write_backup(&full, dir.path(), now() + chrono::Duration::days(1), 1);
    write_backup(&later, dir.path(), now() + chrono::Duration::days(2), 2);

    let src = backups_in(dir.path(), &full);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert_eq!(
        t.changes.len(),
        1,
        "its one recorded state is still readable"
    );
    assert!(
        t.deleted_after.is_some(),
        "a row that vanished must say so — this is the case the per-item dock \
         cannot show at all, since it hangs off a row that no longer exists",
    );
}

#[test]
fn a_backup_whose_prose_blob_is_gone_is_reported_rather_than_silently_skipped() {
    // The realistic corruption, and the only one that can be *attributed*: a
    // folder backup that half-synced, so `items.ron` still lists a blob whose file
    // never arrived. (A zip truncated badly enough to lose its central directory
    // cannot be identified as this project's backup at all — it is not listed, and
    // reporting every unreadable file in a shared destination would be wrong.)
    let dir = tempfile::tempdir().expect("tmp");
    let b = bundle();
    let (uid, _fid) = a_scene(&b);

    let mut copy = b.clone();
    super::mark_as_backup(
        &mut copy,
        "/original/Novel.skrib".to_string(),
        now() + chrono::Duration::days(1),
    );
    let root = dir.path().join("Novel-20260801-100000.skrib");
    let path = root.to_string_lossy().into_owned();
    write_bundle(&path, SkribShape::ExplodedFolder, &copy).expect("write backup");

    // Remove the scene's prose file, leaving its `items.ron` entry behind.
    let blob = copy
        .binders
        .iter()
        .flat_map(|bb| &bb.items)
        .find(|i| i.item.uid == uid)
        .and_then(|i| {
            i.item
                .prose_refs
                .iter()
                .find(|pr| pr.role == ContentRole::SceneText)
        })
        .map(|pr| pr.path.clone())
        .expect("the scene's blob path");
    std::fs::remove_file(root.join(&blob)).expect("remove blob");

    let src = backups_in(dir.path(), &b);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert!(
        t.changes.is_empty(),
        "a state whose prose cannot be produced must not be invented",
    );
    assert!(
        !t.unreadable.is_empty(),
        "a gap the writer cannot see is a gap they will assume is data loss",
    );
}

/// **An unreadable backup is evidence of nothing**, and must never become the
/// boundary the timeline states as fact.
///
/// The older backup here cannot be read at all; the newer one has the scene. If
/// the two kinds of gap are conflated — "the version says the row was not there"
/// and "the version could not be read" — the walk concludes the row was created
/// at the newer moment and the dock tells the writer "Didn't exist before
/// <date>", confidently and falsely, about a scene they may have written years
/// earlier.
#[test]
fn an_unreadable_backup_is_not_mistaken_for_proof_that_a_row_did_not_exist() {
    let dir = tempfile::tempdir().expect("tmp");
    let b = bundle();
    let (uid, _fid) = a_scene(&b);

    // The older moment: a folder bundle whose `items.ron` is corrupt, so indexing
    // it fails outright. (Deleting the whole `binders` tree would *not* do — a
    // bundle with no binders is legal, if odd, and indexes to zero rows, which is
    // a genuine absence rather than an unreadable one.)
    let mut older = b.clone();
    super::mark_as_backup(
        &mut older,
        "/original/Novel.skrib".to_string(),
        now() - chrono::Duration::days(2),
    );
    let older_root = dir.path().join("Novel-20260801-100000.skrib");
    write_bundle(
        &older_root.to_string_lossy(),
        SkribShape::ExplodedFolder,
        &older,
    )
    .expect("write older backup");
    let items = std::fs::read_dir(older_root.join("binders"))
        .expect("binders")
        .flatten()
        .map(|e| e.path().join("items.ron"))
        .find(|p| p.is_file())
        .expect("an items.ron to corrupt");
    std::fs::write(&items, b"this is not RON").expect("corrupt the older backup");

    // The newer moment: readable, and it has the scene.
    let mut newer = b.clone();
    super::mark_as_backup(
        &mut newer,
        "/original/Novel.skrib".to_string(),
        now() - chrono::Duration::days(1),
    );
    write_bundle(
        &dir.path()
            .join("Novel-20260802-100000.skrib")
            .to_string_lossy(),
        SkribShape::ExplodedFolder,
        &newer,
    )
    .expect("write newer backup");

    let src = backups_in(dir.path(), &b);
    let t = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");

    assert!(
        !t.changes.is_empty(),
        "precondition: the readable backup still yields a state",
    );
    assert_eq!(
        t.absent_at, None,
        "a backup that could not be read says nothing about when the row was \
         written, and must not be reported as the moment before it existed",
    );
    assert!(
        !t.unreadable.is_empty(),
        "…it is surfaced as an unreadable moment instead, which is the truth",
    );
}

#[test]
fn the_log_and_the_backups_merge_without_double_counting_the_same_edit() {
    // The same edit is normally in both: the log recorded it at save time, and the
    // next backup copied it forward. One edit, one row in the timeline.
    let dir = tempfile::tempdir().expect("tmp");
    let mut b = bundle();
    let (uid, fid) = a_scene(&b);
    set_scene(&mut b, fid, "the only wording");
    history::record(&mut b, now());

    let project = dir
        .path()
        .join("Novel.skrib")
        .to_string_lossy()
        .into_owned();
    write_bundle(&project, SkribShape::ZipFile, &b).expect("write project");
    let bdir = dir.path().join("backups");
    std::fs::create_dir_all(&bdir).expect("mkdir");
    write_backup(&b, &bdir, now() + chrono::Duration::days(1), 1);

    let log = LogVersions::open(&project);
    let backups = backups_in(&bdir, &b);
    let t = timeline_for(&[&log, &backups], uid, &ContentRole::SceneText).expect("timeline");

    assert_eq!(
        t.changes.len(),
        1,
        "one edit present in two sources is one change, got {:?}",
        t.changes
            .iter()
            .map(|c| (c.at, c.source))
            .collect::<Vec<_>>(),
    );
}

#[test]
fn an_empty_timeline_is_a_normal_answer_for_a_project_with_no_past() {
    let dir = tempfile::tempdir().expect("tmp");
    let b = bundle();
    let (uid, _fid) = a_scene(&b);
    let src = backups_in(dir.path(), &b); // no backups written at all
    let t: Timeline = timeline_for(&[&src], uid, &ContentRole::SceneText).expect("timeline");
    assert!(t.is_empty());
    assert!(t.unreadable.is_empty());
}

// ── one row, read once ─────────────────────────────────────────────────────────

use super::versions::{OneRow, RowAt, VersionIndex, VersionRef};

/// A source that counts how often it is listed and how often a version's index
/// is read, and otherwise answers as the source it wraps.
struct Counted<'a> {
    inner: &'a dyn VersionSource,
    lists: std::cell::Cell<usize>,
    indexes: std::cell::Cell<usize>,
}

impl<'a> Counted<'a> {
    fn new(inner: &'a dyn VersionSource) -> Self {
        Self {
            inner,
            lists: std::cell::Cell::new(0),
            indexes: std::cell::Cell::new(0),
        }
    }
}

impl VersionSource for Counted<'_> {
    fn list(&self) -> anyhow::Result<Vec<VersionRef>> {
        self.lists.set(self.lists.get() + 1);
        self.inner.list()
    }
    fn index(&self, v: &VersionRef) -> anyhow::Result<VersionIndex> {
        self.indexes.set(self.indexes.get() + 1);
        self.inner.index(v)
    }
    fn prose(&self, v: &VersionRef, blob_path: &str) -> anyhow::Result<String> {
        self.inner.prose(v, blob_path)
    }
    fn comments(
        &self,
        v: &VersionRef,
        blob_path: &str,
    ) -> anyhow::Result<Vec<super::bundle::CommentFile>> {
        self.inner.comments(v, blob_path)
    }
}

/// Asking for each of a row's roles in turn reads each backup once, and every
/// role gets the timeline the backups themselves give, an unreadable backup
/// included.
#[test]
fn one_row_reads_each_backup_once_and_answers_every_role_as_the_backups_do() {
    let dir = tempfile::tempdir().expect("tmp");
    let mut b = bundle();
    let (uid, fid) = a_scene(&b);
    set_scene(&mut b, fid, "the first wording");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(1), 1);
    set_scene(&mut b, fid, "the second wording");
    write_backup(&b, dir.path(), now() + chrono::Duration::days(2), 2);
    // A third that is listed but cannot be indexed: a folder backup whose
    // `items.ron` is corrupt, as in
    // `an_unreadable_backup_is_not_mistaken_for_proof_that_a_row_did_not_exist`.
    let mut broken = b.clone();
    super::mark_as_backup(
        &mut broken,
        "/original/Novel.skrib".to_string(),
        now() + chrono::Duration::days(3),
    );
    let broken_root = dir.path().join("Novel-20260803-100000.skrib");
    write_bundle(
        &broken_root.to_string_lossy(),
        SkribShape::ExplodedFolder,
        &broken,
    )
    .expect("write the broken backup");
    let items = std::fs::read_dir(broken_root.join("binders"))
        .expect("binders")
        .flatten()
        .map(|e| e.path().join("items.ron"))
        .find(|p| p.is_file())
        .expect("an items.ron to corrupt");
    std::fs::write(&items, b"this is not RON").expect("corrupt the backup");

    let backups = backups_in(dir.path(), &b);
    let counted = Counted::new(&backups);
    let once = OneRow::new(&counted, uid);
    let roles = [
        ContentRole::SceneText,
        ContentRole::NoteText,
        ContentRole::ParatextText,
        ContentRole::EpigraphText,
        ContentRole::SynopsisText,
    ];
    let versions = backups.list().expect("list").len();
    for role in &roles {
        let direct = timeline_for(&[&backups], uid, role).expect("timeline");
        let through = timeline_for(&[&once], uid, role).expect("timeline");
        assert_eq!(through, direct, "{role:?} answers as the backups do");
    }
    let scene = timeline_for(&[&once], uid, &ContentRole::SceneText).expect("timeline");
    assert_eq!(scene.changes.len(), 2, "precondition: the scene has a past");
    assert_eq!(
        scene.unreadable.len(),
        1,
        "precondition: the broken backup is listed, and reported as unreadable",
    );
    assert_eq!(counted.lists.get(), 1, "the destinations are listed once");
    assert_eq!(
        counted.indexes.get(),
        versions,
        "each of the {versions} backups is read once, not once per role",
    );

    // Another row is not this one's to answer from what was kept.
    let other = uuid::Uuid::from_u128(7);
    assert_eq!(
        once.row_at(
            &backups.list().expect("list")[1],
            other,
            &ContentRole::SceneText
        )
        .expect("a readable backup"),
        RowAt::Absent,
    );
    assert_eq!(
        counted.indexes.get(),
        versions + 1,
        "a question about another row goes to the backups themselves",
    );
}

/// A past version comes out of a backup, which travels exactly as a project does, and
/// the diff pane parses what `prose` hands it. Every shape of nesting past the Djot
/// ceiling is refused by name there. Before the ceiling counted every kind of
/// container, all but the first shape was handed over, and the diff pane's parse
/// aborted the process: a blob that gets through is parsed here the same way.
#[test]
fn a_backup_blob_nested_past_the_djot_ceiling_is_refused_by_name() {
    for (name, deep) in super::djot_depth::tests::past_the_parsers_limit() {
        let dir = tempfile::tempdir().expect("tmp");
        let mut b = bundle();
        let (uid, fid) = a_scene(&b);
        set_scene(&mut b, fid, &deep);
        let path = write_backup(&b, dir.path(), now(), 1);
        let src = backups_in(dir.path(), &b);
        let v = super::versions::VersionRef {
            path: std::path::PathBuf::from(&path),
            taken_at: now(),
            source: SourceKind::Backup,
        };
        let index = src.index(&v).expect("index");
        let Some((blob, _)) = index
            .row(uid)
            .and_then(|row| row.prose_for(&ContentRole::SceneText))
        else {
            panic!("{name}: the scene has a blob in the backup");
        };

        match src.prose(&v, blob) {
            Err(e) => {
                let msg = format!("{e:#}");
                assert!(msg.contains(blob), "{name}: names the blob: {msg}");
                assert!(msg.contains("nests"), "{name}: {msg}");
            }
            Ok(text) => {
                let parsed = super::djot_depth::tests::parse_on_a_long_operation_stack(text);
                panic!("{name}: a blob past the ceiling was handed over: {parsed:?}");
            }
        }
    }
}

/// Write `b` as a backup into `dir` in `shape`, stamped at `at`, and return its path.
fn write_backup_shaped(
    b: &WorkBundle,
    dir: &std::path::Path,
    at: chrono::DateTime<chrono::Utc>,
    shape: SkribShape,
) -> String {
    let mut copy = b.clone();
    super::mark_as_backup(&mut copy, "/original/Novel.skrib".to_string(), at);
    let path = dir
        .join("Novel-20260801-100000.skrib")
        .to_string_lossy()
        .into_owned();
    write_bundle(&path, shape, &copy).expect("write backup");
    path
}

/// Put `text` in the body of the first comment beside a prose blob, or of the first
/// reply there, and return that blob's path and its comment sidecar's.
fn plant_in_a_thread(b: &mut WorkBundle, reply: bool, text: &str) -> (String, String) {
    for bb in &mut b.binders {
        for item in &mut bb.items {
            for (content, thread) in &mut item.comments {
                let Some(comment) = thread.iter_mut().find(|c| !reply || !c.replies.is_empty())
                else {
                    continue;
                };
                if reply {
                    comment.replies[0].body = text.to_string();
                } else {
                    comment.body = text.to_string();
                }
                let Some(prose_ref) = item.item.prose_refs.iter().find(|p| p.file_id == *content)
                else {
                    panic!("a comment sidecar sits beside a prose blob");
                };
                let blob = prose_ref.path.clone();
                let sidecar = super::folder_io::comments_file_name(&blob);
                return (blob, sidecar);
            }
        }
    }
    panic!("the fixture must hold a comment thread with a reply");
}

/// Parse every body in `threads` from a long operation's stack, as the comment cards
/// and the exporter do.
fn parse_every_body(threads: &[super::CommentFile]) -> Result<(), String> {
    for comment in threads {
        let bodies = std::iter::once(&comment.body).chain(comment.replies.iter().map(|r| &r.body));
        for body in bodies {
            super::djot_depth::tests::parse_on_a_long_operation_stack(body.clone())?;
        }
    }
    Ok(())
}

/// A backup carries its comment sidecars, and a comment's body and each reply's are
/// Djot the comment cards parse, exactly as the diff pane parses a past version's
/// prose. Every shape of nesting past the ceiling, in either body, in either shape of
/// backup, is refused by naming the sidecar. Before `comments` held the bodies to the
/// ceiling it handed every one of them over, and the parse below aborted the process.
#[test]
fn a_backup_comment_nested_past_the_djot_ceiling_is_refused_by_name() {
    for (name, deep) in super::djot_depth::tests::past_the_parsers_limit() {
        for shape in [SkribShape::ZipFile, SkribShape::ExplodedFolder] {
            for reply in [false, true] {
                let dir = tempfile::tempdir().expect("tmp");
                let mut b = bundle();
                let (blob, sidecar) = plant_in_a_thread(&mut b, reply, &deep);
                let path = write_backup_shaped(&b, dir.path(), now(), shape);
                let src = backups_in(dir.path(), &b);
                let v = super::versions::VersionRef {
                    path: std::path::PathBuf::from(&path),
                    taken_at: now(),
                    source: SourceKind::Backup,
                };

                match src.comments(&v, &blob) {
                    Err(e) => {
                        let msg = format!("{e:#}");
                        assert!(
                            msg.contains(&sidecar),
                            "{name}, {shape:?}: names the sidecar: {msg}"
                        );
                        assert!(msg.contains("nests"), "{name}, {shape:?}: {msg}");
                        assert!(
                            msg.contains(if reply {
                                "reply 1 to comment"
                            } else {
                                "comment 1"
                            }),
                            "{name}, {shape:?}: names the body: {msg}"
                        );
                    }
                    Ok(threads) => {
                        let parsed = parse_every_body(&threads);
                        panic!(
                            "{name}, {shape:?}: a body past the ceiling was handed over: {parsed:?}"
                        );
                    }
                }
            }
        }
    }
}

/// The same bodies at the ceiling come back whole, from either shape of backup, and
/// parse from a long operation's stack.
#[test]
fn a_backup_comment_at_the_djot_ceiling_comes_back_and_parses() {
    for marker in ["- ", "1. ", "(iv) ", "- [ ] ", "[^a]: ", ": ", "> "] {
        let text = super::djot_depth::tests::one_line(marker, super::MAX_DJOT_DEPTH);
        for shape in [SkribShape::ZipFile, SkribShape::ExplodedFolder] {
            let dir = tempfile::tempdir().expect("tmp");
            let mut b = bundle();
            let (blob, _) = plant_in_a_thread(&mut b, false, &text);
            plant_in_a_thread(&mut b, true, &text);
            let path = write_backup_shaped(&b, dir.path(), now(), shape);
            let src = backups_in(dir.path(), &b);
            let v = super::versions::VersionRef {
                path: std::path::PathBuf::from(&path),
                taken_at: now(),
                source: SourceKind::Backup,
            };

            let threads = match src.comments(&v, &blob) {
                Ok(threads) => threads,
                Err(e) => panic!("{marker:?}, {shape:?}: {e:#}"),
            };
            let at_the_ceiling = threads
                .iter()
                .flat_map(|c| std::iter::once(&c.body).chain(c.replies.iter().map(|r| &r.body)))
                .filter(|body| **body == text)
                .count();
            assert_eq!(
                at_the_ceiling, 2,
                "{marker:?}, {shape:?}: both bodies come back"
            );
            if let Err(e) = parse_every_body(&threads) {
                panic!("{marker:?}, {shape:?}: the parser refused a body: {e}");
            }
        }
    }
}
