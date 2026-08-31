// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Carry-through: a build must not destroy bundle files it does not model.
//!
//! The scenario every test here stands in for: a project written by a build with
//! a feature this one lacks — a newer version, or a different edition — is
//! opened here, saved, and opened again there. Before carry-through, that
//! round trip silently deleted the feature's data, because the zip writer
//! repacks from a fresh staging directory and the folder writer prunes down to
//! what the bundle lists.
//!
//! The files planted below are deliberately shaped like plausible future data
//! rather than like junk: a root manifest, one inside a binder directory, and
//! one sitting next to a prose blob in `text/` where the sidecar prune runs.

use super::bundle::ShapeTag;
use super::{SkribShape, read_bundle, write_bundle};

/// Plant `extra` files into a bundle on disk, then read it back.
///
/// Writes a normal project first so the planted files land beside real content
/// rather than in an empty directory — the prunes only run where there is
/// something to prune.
fn plant_and_read(
    shape: SkribShape,
    extra: &[(&str, &[u8])],
) -> (tempfile::TempDir, String, Vec<(String, Vec<u8>)>) {
    let dir = tempfile::tempdir().expect("tmp");
    let target = dir.path().join("Novel.skrib");
    let path = target.to_string_lossy().into_owned();
    let tag = match shape {
        SkribShape::ZipFile => ShapeTag::Zip,
        _ => ShapeTag::Folder,
    };
    let fixture = super::tests::build_bundle(tag);
    // The binder's directory name is derived from its index and title
    // (`slug::binder_dir_name`), so a hard-coded one would be planting files in
    // a directory for a binder that does not exist — which `prune_binder_dirs`
    // is *right* to delete, and which would test the wrong thing.
    let binder_dir = super::slug::binder_dir_name(0, &fixture.binders[0].binder.name);
    let extra: Vec<(String, Vec<u8>)> = extra
        .iter()
        .map(|(rel, bytes)| (rel.replace("{binder}", &binder_dir), bytes.to_vec()))
        .collect();
    write_bundle(&path, shape, &fixture).expect("write");

    match shape {
        SkribShape::ExplodedFolder => {
            let root = super::shape::folder_root(&path);
            for (rel, bytes) in &extra {
                let p = root.join(rel);
                std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                std::fs::write(&p, bytes).unwrap();
            }
        }
        SkribShape::ZipFile => {
            // Rebuild the archive with the extra entries appended: the writer
            // under test is the only thing allowed to be clever here.
            let staging = tempfile::tempdir().unwrap();
            let file = std::fs::File::open(&path).unwrap();
            zip::ZipArchive::new(file)
                .unwrap()
                .extract(staging.path())
                .unwrap();
            for (rel, bytes) in &extra {
                let p = staging.path().join(rel);
                std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                std::fs::write(&p, bytes).unwrap();
            }
            let mut out = std::fs::File::create(&path).unwrap();
            super::zip_io::zip_dir(staging.path(), &mut out).unwrap();
        }
        SkribShape::LegacySqlite => unreachable!("not a write target"),
    }
    (dir, path, extra)
}

/// The one that matters: read a project with unmodelled files in it, save it
/// back exactly as an ordinary build would, and check they are all still there.
fn round_trip_preserves(shape: SkribShape, extra: &[(&str, &[u8])]) {
    let (_dir, path, extra) = plant_and_read(shape, extra);

    let loaded = read_bundle(&path).expect("read");
    for (rel, bytes) in &extra {
        let carried = loaded
            .carried
            .get(rel)
            .unwrap_or_else(|| panic!("`{rel}` was not carried out of the bundle"));
        assert_eq!(&carried.bytes, bytes, "`{rel}` read back with wrong bytes");
    }

    // Save it straight back — no edits, the cheapest possible thing a writer
    // could do to someone else's project.
    write_bundle(&path, shape, &loaded).expect("rewrite");

    let again = read_bundle(&path).expect("reread");
    for (rel, bytes) in &extra {
        let carried = again
            .carried
            .get(rel)
            .unwrap_or_else(|| panic!("`{rel}` was destroyed by a save"));
        assert_eq!(&carried.bytes, bytes, "`{rel}` survived a save but changed");
    }
}

/// Paths chosen for where they land, not for variety:
/// - root: the simplest case, and where a whole-project manifest would go.
/// - inside a binder directory: survives `prune_binder_dirs` for a live binder.
/// - inside `text/`, ending `.ron`: the sharp one. `prune_dir(&tdir, …, "ron")`
///   deletes every `.ron` in that directory that is not a known sidecar, so
///   without the carry check this file is removed on the very first save.
const FUTURE_FILES: &[(&str, &[u8])] = &[
    ("structure.ron", b"([(beat: \"midpoint\", at: 0.5)])"),
    ("binders/{binder}/structure.ron", b"(bound: [1, 2, 3])"),
    ("binders/{binder}/text/plot.beats.ron", b"(beats: [])"),
];

#[test]
fn unmodelled_files_survive_a_folder_round_trip() {
    round_trip_preserves(SkribShape::ExplodedFolder, FUTURE_FILES);
}

#[test]
fn unmodelled_files_survive_a_zip_round_trip() {
    round_trip_preserves(SkribShape::ZipFile, FUTURE_FILES);
}

#[test]
fn a_carried_ron_beside_prose_outlives_the_sidecar_prune() {
    // Isolated because it is the case a naive implementation gets wrong: the
    // file is in the directory the sidecar prune sweeps, has the extension it
    // sweeps for, and is not in its keep-set.
    round_trip_preserves(
        SkribShape::ExplodedFolder,
        &[("binders/{binder}/text/plot.beats.ron", b"(beats: [])")],
    );
}

#[test]
fn binary_bytes_are_carried_unchanged() {
    // Carrying must not assume UTF-8: the whole point is that the bytes are
    // opaque. A lossy read would corrupt a future build's binary sidecar and
    // still "pass" a string comparison.
    round_trip_preserves(
        SkribShape::ZipFile,
        &[("plot.bin", &[0x00, 0xFF, 0xFE, 0x80, 0x7F, 0x00])],
    );
}

#[test]
fn a_project_with_nothing_unmodelled_carries_nothing() {
    // The no-op guarantee: carrying must not invent files, or every project
    // would grow one on first save and the exploded-folder diff would churn.
    let (_dir, path, _) = plant_and_read(SkribShape::ExplodedFolder, &[]);
    let loaded = read_bundle(&path).expect("read");
    assert!(
        loaded.carried.is_empty(),
        "a plain project carried {:?}",
        loaded.carried.keys().collect::<Vec<_>>()
    );
}

#[test]
fn an_orphaned_prose_blob_is_still_pruned() {
    // The regression this design exists to avoid. A stale `.djot` in `text/` is
    // *modelled in shape* but referenced by nothing, and the prune is what
    // collects it. If carrying were defined as "whatever the reader did not
    // read", this file would be carried instead — preserved forever, and the
    // format's garbage collection quietly disabled.
    let (_dir, path, planted) = plant_and_read(
        SkribShape::ExplodedFolder,
        &[("binders/{binder}/text/99-ghost.djot", b"orphaned")],
    );
    let ghost = &planted[0].0;
    let loaded = read_bundle(&path).expect("read");
    assert!(
        !loaded.carried.contains_key(ghost),
        "an orphaned prose blob must stay prunable, not become carried"
    );

    write_bundle(&path, SkribShape::ExplodedFolder, &loaded).expect("rewrite");
    let root = super::shape::folder_root(&path);
    assert!(
        !root.join(ghost).exists(),
        "the orphan prune stopped collecting stale prose blobs"
    );
}

#[test]
fn a_carried_file_changes_the_content_fingerprint() {
    // `CarriedFile::bytes` is `#[serde(skip)]`, so only the digest reaches the
    // fingerprint. If that digest were skipped too, backup skip-if-unchanged
    // would treat a project whose *only* change was a carried file as
    // unmodified, and it would silently stop being backed up.
    let (_dir, path, _) = plant_and_read(
        SkribShape::ExplodedFolder,
        &[("structure.ron", b"(version: 1)")],
    );
    let before = super::content_fingerprint(&read_bundle(&path).expect("read"));

    let root = super::shape::folder_root(&path);
    std::fs::write(root.join("structure.ron"), b"(version: 2)").unwrap();
    let after = super::content_fingerprint(&read_bundle(&path).expect("reread"));

    assert_ne!(
        before, after,
        "a changed carried file must change the fingerprint, or dedup skips its backup"
    );
}
