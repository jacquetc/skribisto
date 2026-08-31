// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Round-trip and recording tests for the in-project history log.
//!
//! Kept beside `asset_tests` rather than inside `history.rs` because these need a
//! real bundle written to a real directory: the log's whole contract is about what
//! survives a write, and the zip shape in particular rebuilds from a fresh staging
//! directory, so only an actual round trip can prove anything.

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

#[test]
fn recording_captures_every_prose_row_the_first_time() {
    let mut b = bundle();
    assert!(b.history.is_empty(), "a fresh bundle carries no history");

    history::record(&mut b, now());

    let prose_rows: usize = b
        .binders
        .iter()
        .flat_map(|bb| &bb.items)
        .map(|i| i.item.prose_refs.len())
        .sum();
    assert!(prose_rows > 0, "the fixture must have prose to record");
    assert_eq!(
        b.history.entries.len(),
        prose_rows,
        "the first record captures one entry per prose row",
    );
}

#[test]
fn recording_twice_without_an_edit_appends_nothing() {
    let mut b = bundle();
    history::record(&mut b, now());
    let after_first = b.history.entries.len();

    history::record(&mut b, now() + chrono::Duration::hours(1));

    assert_eq!(
        b.history.entries.len(),
        after_first,
        "an unchanged save must not grow the log — otherwise every autosave tick \
         would add a version indistinguishable from the last",
    );
}

#[test]
fn editing_one_row_appends_exactly_one_entry() {
    let mut b = bundle();
    history::record(&mut b, now());
    let before = b.history.entries.len();

    // Change one prose blob.
    let item = b
        .binders
        .iter_mut()
        .flat_map(|bb| &mut bb.items)
        .find(|i| !i.item.prose_refs.is_empty())
        .expect("a row with prose");
    let file_id = item.item.prose_refs[0].file_id;
    item.prose
        .insert(file_id, "a rewritten opening".to_string());

    history::record(&mut b, now() + chrono::Duration::hours(1));

    assert_eq!(
        b.history.entries.len(),
        before + 1,
        "one edited row is one new entry, not a fresh snapshot of the whole project",
    );
}

#[test]
fn identical_prose_shares_one_blob() {
    let mut b = bundle();
    // Force two rows to hold byte-identical text.
    let same = "the very same words".to_string();
    let mut touched = 0;
    for bb in b.binders.iter_mut() {
        for item in bb.items.iter_mut() {
            if let Some(pr) = item.item.prose_refs.first() {
                let id = pr.file_id;
                item.prose.insert(id, same.clone());
                touched += 1;
            }
        }
    }
    assert!(touched >= 2, "need at least two prose rows to share a blob");

    history::record(&mut b, now());

    let hashes: std::collections::BTreeSet<&str> =
        b.history.entries.iter().map(|e| e.hash.as_str()).collect();
    assert!(
        b.history.blobs.len() <= hashes.len(),
        "blobs are content-addressed, so identical prose must not be stored twice",
    );
    assert!(
        b.history.blobs.len() < touched,
        "{touched} rows holding identical text must collapse to fewer blobs, got {}",
        b.history.blobs.len(),
    );
}

#[test]
fn the_log_survives_an_exploded_folder_round_trip() {
    let mut b = bundle();
    history::record(&mut b, now());
    let expected = b.history.entries.len();

    let (_dir, _path, back) = round_trip(&b, SkribShape::ExplodedFolder);

    assert_eq!(back.history.entries.len(), expected);
    for e in &back.history.entries {
        assert!(
            back.history.blobs.contains_key(&e.hash),
            "every surviving entry must come back with its prose",
        );
    }
}

#[test]
fn the_log_survives_a_zip_round_trip() {
    // The zip writer packs a *fresh* staging directory, so anything not carried in
    // `WorkBundle` is simply not in the next archive. This is the test that says
    // the history log is carried rather than left behind on disk.
    let mut b = bundle();
    history::record(&mut b, now());
    let expected: Vec<String> = b.history.entries.iter().map(|e| e.hash.clone()).collect();

    let (_dir, _path, back) = round_trip(&b, SkribShape::ZipFile);

    assert_eq!(
        back.history
            .entries
            .iter()
            .map(|e| e.hash.clone())
            .collect::<Vec<_>>(),
        expected,
    );
}

#[test]
fn load_reads_the_log_back_from_both_shapes_without_parsing_the_bundle() {
    for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
        let mut b = bundle();
        history::record(&mut b, now());
        let expected = b.history.entries.len();

        let (_dir, path, _back) = round_trip(&b, shape);
        let loaded = history::load(&path);

        assert_eq!(
            loaded.entries.len(),
            expected,
            "history::load must recover the log for {shape:?} — it is what every \
             save uses to carry history forward",
        );
        for e in &loaded.entries {
            assert!(loaded.blobs.contains_key(&e.hash));
        }
    }
}

#[test]
fn load_of_a_project_with_no_history_is_empty_not_an_error() {
    let (_dir, path, _back) = round_trip(&bundle(), SkribShape::ZipFile);
    assert!(history::load(&path).is_empty());
    assert!(history::load("/nonexistent/Novel.skrib").is_empty());
}

#[test]
fn a_corrupt_index_degrades_to_no_history_and_still_opens_the_project() {
    let mut b = bundle();
    history::record(&mut b, now());
    let dir = tempfile::tempdir().expect("tmp");
    let root = dir.path().join("Novel");
    let path = root.to_string_lossy().into_owned();
    write_bundle(&path, SkribShape::ExplodedFolder, &b).expect("write");

    std::fs::write(root.join(history::HISTORY_INDEX), b"{ this is not ron").expect("corrupt");

    // The manuscript is not a convenience: a broken history index must never be
    // able to lock a writer out of their book.
    let back = read_bundle(&path).expect("a corrupt history index must not fail the open");
    assert!(back.history.is_empty());
    assert!(
        !back.binders.is_empty(),
        "the manuscript itself must still be there",
    );
    assert!(history::load(&path).is_empty());
}

#[test]
fn an_emptied_log_removes_its_index_rather_than_leaving_a_stale_one() {
    let mut b = bundle();
    history::record(&mut b, now());
    let dir = tempfile::tempdir().expect("tmp");
    let root = dir.path().join("Novel");
    let path = root.to_string_lossy().into_owned();
    write_bundle(&path, SkribShape::ExplodedFolder, &b).expect("write");
    assert!(root.join(history::HISTORY_INDEX).exists());

    b.history = history::HistoryLog::default();
    write_bundle(&path, SkribShape::ExplodedFolder, &b).expect("rewrite");

    assert!(
        !root.join(history::HISTORY_INDEX).exists(),
        "an emptied log must not leave an index behind for the next load to resurrect",
    );
}
