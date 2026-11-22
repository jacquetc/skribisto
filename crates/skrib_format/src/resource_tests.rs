// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resources in the bundle: `resources.ron`, the stored files under
//! `resources/<uid>/<name>`, the items' links, and the linked-file paths.

use std::path::Path;

use common::entities::ResourceKind;

use super::bundle::{ShapeTag, WorkBundle};
use super::tests::{build_bundle, sample_resource_blobs};
use super::{SkribShape, read_bundle, write_bundle};

/// The fixture's stored resource, its uid as `resource_blobs` keys it.
fn stored_key() -> String {
    common::uid::fixture_uid(60).hyphenated().to_string()
}

/// Write and read back, keeping the folder: a bundle read from it points at its files.
fn round_trip(shape: SkribShape, bundle: &WorkBundle) -> (tempfile::TempDir, WorkBundle) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Novel.skrib");
    let path = path.to_str().unwrap();
    write_bundle(path, shape, bundle).unwrap();
    let back = read_bundle(path).unwrap();
    (dir, back)
}

fn attached(bundle: &WorkBundle) -> Vec<u64> {
    bundle
        .binders
        .iter()
        .flat_map(|b| b.items.iter())
        .flat_map(|i| i.item.resource_ids.iter().copied())
        .collect()
}

#[test]
fn resources_survive_a_folder_round_trip_with_their_files_and_links() {
    let bundle = build_bundle(ShapeTag::Folder);
    assert_eq!(
        bundle.resources.len(),
        3,
        "the fixture carries one of each kind"
    );
    let (dir, back) = round_trip(SkribShape::ExplodedFolder, &bundle);

    assert_eq!(back.resources, bundle.resources);
    assert_eq!(attached(&back), vec![60, 61, 62]);
    let file = dir
        .path()
        .join("Novel.skrib")
        .join("resources")
        .join(stored_key())
        .join("Village map.pdf");
    assert_eq!(std::fs::read(&file).unwrap(), b"%PDF-1.7");
    assert_eq!(
        back.resource_blobs
            .get(&stored_key())
            .map(|b| b.read().unwrap()),
        Some(b"%PDF-1.7".to_vec())
    );
    // Only the stored one has a file; the other two are rows.
    assert_eq!(back.resource_blobs.len(), 1);
}

#[test]
fn resources_survive_a_zip_round_trip() {
    let bundle = build_bundle(ShapeTag::Zip);
    let (_dir, back) = round_trip(SkribShape::ZipFile, &bundle);
    assert_eq!(back.resources, bundle.resources);
    assert_eq!(attached(&back), vec![60, 61, 62]);
    assert_eq!(
        back.resource_blobs
            .get(&stored_key())
            .map(|b| b.read().unwrap()),
        Some(b"%PDF-1.7".to_vec())
    );
}

#[test]
fn a_stored_resource_whose_file_is_missing_keeps_its_card() {
    // A file the save could not find: the row is the writer's (title, description,
    // attachments) and must not go with the bytes.
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.resource_blobs.clear();
    let (_dir, back) = round_trip(SkribShape::ExplodedFolder, &bundle);
    let stored = back
        .resources
        .iter()
        .find(|r| r.kind == ResourceKind::StoredFile)
        .expect("the stored resource's row survives");
    assert_eq!(stored.title, "Village map");
    assert!(back.resource_blobs.is_empty());
    assert_eq!(attached(&back), vec![60, 61, 62]);
}

#[test]
fn a_removed_resource_keeps_its_file_until_the_project_is_next_opened() {
    // Undo after a save must find the bytes: the folder writer never prunes
    // `resources/`; `media::prune_resources_dir` does, when the project opens.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("Novel");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();
    let mut removed = read_bundle(path).unwrap();
    removed
        .resources
        .retain(|r| r.kind != ResourceKind::StoredFile);
    removed.resource_blobs.clear();
    write_bundle(path, SkribShape::ExplodedFolder, &removed).unwrap();
    let file = root
        .join("resources")
        .join(stored_key())
        .join("Village map.pdf");
    assert!(
        file.is_file(),
        "a save must not delete a removed resource's file"
    );

    let keep: Vec<(uuid::Uuid, String)> = Vec::new();
    super::media::prune_resources_dir(&root.join("resources"), &keep);
    assert!(!file.exists(), "the next open removes it");
}

#[test]
fn a_resource_file_is_modelled_not_carried() {
    let (dir, back) = round_trip(SkribShape::ExplodedFolder, &build_bundle(ShapeTag::Folder));
    assert!(
        back.carried.keys().all(|k| !k.starts_with("resources")),
        "carried: {:?}",
        back.carried.keys().collect::<Vec<_>>()
    );
    drop(dir);
}

#[test]
fn a_crafted_file_name_stays_inside_its_resource_folder() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    for r in &mut bundle.resources {
        if r.kind == ResourceKind::StoredFile {
            r.file_name = "../../../escaped.sh".into();
        }
    }
    let (dir, _back) = round_trip(SkribShape::ExplodedFolder, &bundle);
    let folder = dir
        .path()
        .join("Novel.skrib")
        .join("resources")
        .join(stored_key());
    let names: Vec<String> = std::fs::read_dir(&folder)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, vec!["_.._.._escaped.sh".to_string()]);
    assert!(!dir.path().join("escaped.sh").exists());
}

#[test]
fn the_read_floor_is_17_only_with_resources() {
    let with = build_bundle(ShapeTag::Folder);
    assert_eq!(super::version_gate::compute_min_read_version(&with), 17);
    let mut without = build_bundle(ShapeTag::Folder);
    super::tests::strip_resources(&mut without);
    assert!(super::version_gate::compute_min_read_version(&without) < 17);
}

#[test]
fn an_item_link_to_a_resource_of_no_project_is_not_written() {
    // The item names 60, 61 and 62; the Work only owns 60 and 61 (62 is another
    // project's, or a stale id). Only the two it owns are written.
    let mut s = super::tests::sample_inputs();
    s.resources.retain(|r| r.id != 62);
    let bundle = super::from_entities(
        &s.work,
        &s.tags,
        &s.dict_words,
        &[],
        &s.note_templates,
        &s.statuses,
        &s.assets,
        Default::default(),
        &s.resources,
        sample_resource_blobs(),
        Some(&s.smart_punctuation),
        &s.trash,
        &[],
        &[],
        &[],
        &[],
        &s.binders,
        ShapeTag::Folder,
    );
    assert_eq!(attached(&bundle), vec![60, 61]);
}

#[test]
fn a_linked_path_is_stored_relative_to_the_folder_holding_the_project() {
    use super::media::{link_base, relative_link, resolve_link};
    let dir = tempfile::tempdir().unwrap();
    let books = dir.path().join("Books");
    std::fs::create_dir_all(books.join("Novel")).unwrap();
    std::fs::write(books.join("Novel").join("project.skrib"), "()").unwrap();

    // A folder project and a zip beside it share one base: the folder that holds them.
    let folder_base = link_base(&books.join("Novel")).unwrap();
    let zip_base = link_base(&books.join("Novel.skrib")).unwrap();
    assert_eq!(folder_base, books);
    assert_eq!(zip_base, books);

    let target = dir.path().join("Research").join("map.pdf");
    let stored = relative_link(&target, &books);
    assert_eq!(stored, "../Research/map.pdf");
    assert_eq!(resolve_link(&stored, &books), target);

    // Somewhere else on another computer, the same pair still resolves.
    let elsewhere = Path::new("/srv/writer/Books");
    assert_eq!(
        resolve_link(&stored, elsewhere),
        Path::new("/srv/writer/Research/map.pdf")
    );
}

#[test]
fn a_path_inside_the_projects_folder_is_stored_without_climbing() {
    let base = Path::new("/home/a/Books");
    assert_eq!(
        super::media::relative_link(Path::new("/home/a/Books/Notes/x.txt"), base),
        "Notes/x.txt"
    );
}

#[test]
fn a_resolved_link_never_climbs_past_the_root() {
    let resolved = super::media::resolve_link("../../../../x.pdf", Path::new("/a"));
    assert_eq!(resolved, Path::new("/x.pdf"));
}

#[test]
fn a_resource_with_no_uid_or_a_shared_one_gets_its_own_and_keeps_its_file() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    let key = stored_key();
    let blob = bundle.resource_blobs.remove(&key).unwrap();
    // Two stored rows, both nil, sharing the nil folder's file.
    let nil = uuid::Uuid::nil();
    bundle
        .resource_blobs
        .insert(nil.hyphenated().to_string(), blob);
    let mut twin = bundle.resources[0].clone();
    twin.file_id = 63;
    bundle.resources[0].uid = nil;
    twin.uid = nil;
    bundle.resources.push(twin);

    bundle.heal_resource_uids();
    let uids: Vec<uuid::Uuid> = bundle.resources.iter().map(|r| r.uid).collect();
    assert!(uids.iter().all(|u| !u.is_nil()));
    let distinct: std::collections::BTreeSet<_> = uids.iter().collect();
    assert_eq!(distinct.len(), uids.len());
    for r in bundle
        .resources
        .iter()
        .filter(|r| r.kind == ResourceKind::StoredFile)
    {
        let b = bundle.resource_blobs.get(&r.uid.hyphenated().to_string());
        assert_eq!(b.map(|b| b.read().unwrap()), Some(b"%PDF-1.7".to_vec()));
    }
}

#[test]
fn prune_keeps_a_file_whose_name_only_changed_case() {
    let dir = tempfile::tempdir().unwrap();
    let uid = common::uid::fixture_uid(60);
    let folder = dir.path().join(uid.hyphenated().to_string());
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("village map.pdf"), b"x").unwrap();
    super::media::prune_resources_dir(dir.path(), &[(uid, "Village map.pdf".into())]);
    assert!(folder.join("village map.pdf").exists());
}

#[test]
fn a_dot_dot_in_a_linked_path_is_applied_not_dropped() {
    let stored = super::media::relative_link(
        Path::new("/data/Books/../Research/map.pdf"),
        Path::new("/data/Books"),
    );
    assert_eq!(stored, "../Research/map.pdf");
}

#[test]
fn a_zip_working_folder_is_its_own_for_each_copy_of_a_project() {
    let data = Path::new("/data/media");
    let a = super::media::resources_dir(Path::new("/books/Novel.skrib"), "uid-1", data, "uid-1");
    let b = super::media::resources_dir(Path::new("/books/Copy.skrib"), "uid-1", data, "uid-1");
    assert_ne!(a, b);
    assert_eq!(
        a,
        super::media::resources_dir(Path::new("/books/Novel.skrib"), "uid-1", data, "uid-1")
    );
}
