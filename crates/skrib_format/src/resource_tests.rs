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
    // Isolated to the resource axis: the fixture's scratchpads would hold it at 18.
    let mut with = build_bundle(ShapeTag::Folder);
    super::tests::strip_scratchpads(&mut with);
    assert_eq!(super::version_gate::compute_min_read_version(&with), 17);
    let mut without = build_bundle(ShapeTag::Folder);
    super::tests::strip_scratchpads(&mut without);
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
fn unknown_resource_entries_survive_open_and_save_as() {
    let (dir, _) = round_trip(SkribShape::ExplodedFolder, &build_bundle(ShapeTag::Folder));
    let root = dir.path().join("Novel.skrib");
    let extras = ["resources/research/map.pdf", "resources/notes.txt"];
    for name in extras {
        let path = root.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"research").unwrap();
    }
    super::media::prune_resources_dir(&root.join("resources"), &[]);
    let bundle = read_bundle(root.to_str().unwrap()).unwrap();
    let (_, copy) = round_trip(SkribShape::ZipFile, &bundle);
    for name in extras {
        assert_eq!(copy.carried[name].bytes, b"research");
    }
}

#[test]
fn an_older_projects_uuid_named_research_directory_survives_upgrade_and_backup() {
    let mut old = build_bundle(ShapeTag::Folder);
    old.resources.clear();
    old.resource_blobs.clear();
    old.manifest.format_version = 16;
    let (dir, _) = round_trip(SkribShape::ExplodedFolder, &old);
    let root = dir.path().join("Novel.skrib");
    let path = root.to_str().unwrap();
    let rel = format!("resources/{}/research.pdf", stored_key());
    let file = root.join(&rel);
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, b"research").unwrap();
    std::fs::remove_file(root.join("resources.ron")).unwrap();

    let mut back = read_bundle(path).unwrap();
    super::media::prune_resources_except(
        &root.join("resources"),
        &[],
        &back.manifest.legacy_resource_dirs,
    );
    assert_eq!(std::fs::read(file).unwrap(), b"research");
    back.manifest.format_version = super::FORMAT_VERSION;
    back.carried = super::carry::load_for_save(path, &back.resources);
    write_bundle(path, SkribShape::ExplodedFolder, &back).unwrap();
    let upgraded = read_bundle(path).unwrap();
    let (backup, restored) = round_trip(SkribShape::ZipFile, &upgraded);
    assert_eq!(restored.carried[&rel].bytes, b"research");
    assert_eq!(
        super::carry::load(backup.path().join("Novel.skrib").to_str().unwrap())[&rel].bytes,
        b"research"
    );
}

#[test]
fn a_decomposed_resource_name_is_found_after_open() {
    let dir = tempfile::tempdir().unwrap();
    let uid = common::uid::fixture_uid(60);
    let folder = dir.path().join(uid.hyphenated().to_string());
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("Cafe\u{301}.pdf"), b"map").unwrap();
    super::media::prune_resources_dir(dir.path(), &[(uid, "Café.pdf".into())]);
    let path = super::media::resource_file(dir.path(), uid, "Café.pdf");
    assert_eq!(std::fs::read(path).unwrap(), b"map");
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

// ── links in a folder project ──

/// The stored resource's folder in the folder project at `root`.
fn stored_folder(root: &Path) -> std::path::PathBuf {
    root.join("resources").join(stored_key())
}

/// A folder project whose stored file is a link to a file outside it is read without
/// that file, and a copy of it written as a single file does not carry its bytes: a
/// shared folder project cannot slip `~/.ssh/id_rsa` into the archive the writer sends.
#[cfg(unix)]
#[test]
fn a_stored_file_that_is_a_link_is_not_read_into_the_project() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("id_rsa");
    std::fs::write(&secret, b"PRIVATE KEY outside the project").unwrap();
    let root = dir.path().join("Novel");
    write_bundle(
        root.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();
    let file = stored_folder(&root).join("Village map.pdf");
    std::fs::remove_file(&file).unwrap();
    std::os::unix::fs::symlink(&secret, &file).unwrap();

    let back = read_bundle(root.to_str().unwrap()).unwrap();
    assert!(
        back.resource_blobs.is_empty(),
        "the link is left out: {:?}",
        back.resource_blobs.keys().collect::<Vec<_>>()
    );
    assert!(
        back.resources
            .iter()
            .any(|r| r.kind == ResourceKind::StoredFile),
        "its card stays, shown as missing"
    );

    let zip = dir.path().join("Copy.skrib");
    write_bundle(zip.to_str().unwrap(), SkribShape::ZipFile, &back).unwrap();
    let bytes = std::fs::read(&zip).unwrap();
    assert!(
        !bytes
            .windows(b"PRIVATE KEY".len())
            .any(|w| w == b"PRIVATE KEY"),
        "the outside file's bytes reached the archive"
    );
    let copy = read_bundle(zip.to_str().unwrap()).unwrap();
    assert!(copy.resource_blobs.is_empty());
}

/// The same with the resource's whole `<uid>` folder a link to a folder elsewhere.
#[cfg(unix)]
#[test]
fn a_stored_file_under_a_linked_folder_is_not_read_into_the_project() {
    let dir = tempfile::tempdir().unwrap();
    let elsewhere = dir.path().join("elsewhere");
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::write(elsewhere.join("Village map.pdf"), b"outside").unwrap();
    let root = dir.path().join("Novel");
    write_bundle(
        root.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();
    std::fs::remove_dir_all(stored_folder(&root)).unwrap();
    std::os::unix::fs::symlink(&elsewhere, stored_folder(&root)).unwrap();

    let back = read_bundle(root.to_str().unwrap()).unwrap();
    assert!(back.resource_blobs.is_empty());
}

/// A picture that is a link is left out the same way, and the project still opens.
#[cfg(unix)]
#[test]
fn a_picture_that_is_a_link_is_not_read_into_the_project() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("private.jpg");
    std::fs::write(&secret, b"outside").unwrap();
    let blobs = std::collections::BTreeMap::from([
        (
            "hash-of-cover".to_string(),
            crate::Blob::from(b"cover".to_vec()),
        ),
        (
            "hash-of-map".to_string(),
            crate::Blob::from(b"map".to_vec()),
        ),
    ]);
    let bundle = super::tests::build_bundle_with_bytes(ShapeTag::Folder, blobs, |_| {});
    let root = dir.path().join("Novel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    let map = bundle
        .assets
        .iter()
        .find(|a| a.content_hash == "hash-of-map")
        .expect("the map is written");
    let file = root.join(&map.path);
    std::fs::remove_file(&file).unwrap();
    std::os::unix::fs::symlink(&secret, &file).unwrap();

    let back = read_bundle(root.to_str().unwrap()).expect("the project still opens");
    assert!(back.asset_blobs.contains_key("hash-of-cover"));
    assert!(
        !back.asset_blobs.contains_key("hash-of-map"),
        "the linked picture is left out"
    );
}

/// Opening a project prunes its resources folder, and a live resource's folder that is
/// a link is not listed: the prune would otherwise delete every file in the folder it
/// points at that the resource does not name.
#[cfg(unix)]
#[test]
fn the_prune_never_deletes_through_a_linked_resource_folder() {
    let dir = tempfile::tempdir().unwrap();
    let documents = dir.path().join("Documents");
    std::fs::create_dir_all(&documents).unwrap();
    std::fs::write(documents.join("Thesis.odt"), b"the writer's thesis").unwrap();
    let resources = dir.path().join("resources");
    std::fs::create_dir_all(&resources).unwrap();
    let uid = common::uid::fixture_uid(60);
    std::os::unix::fs::symlink(&documents, resources.join(uid.hyphenated().to_string())).unwrap();

    super::media::prune_resources_dir(&resources, &[(uid, "Village map.pdf".into())]);
    assert_eq!(
        std::fs::read(documents.join("Thesis.odt")).unwrap(),
        b"the writer's thesis"
    );
}

#[cfg(unix)]
#[test]
fn a_link_anywhere_below_the_folder_is_found_and_the_folder_itself_is_not_judged() {
    use super::media::passes_through_link;
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().join("real");
    std::fs::create_dir_all(real.join("uid")).unwrap();
    std::fs::write(real.join("uid").join("a.pdf"), b"a").unwrap();
    std::os::unix::fs::symlink(
        real.join("uid").join("a.pdf"),
        real.join("uid").join("b.pdf"),
    )
    .unwrap();
    std::os::unix::fs::symlink(real.join("uid"), real.join("linked-uid")).unwrap();
    // The project folder reached through a link of the user's own.
    let alias = dir.path().join("alias");
    std::os::unix::fs::symlink(&real, &alias).unwrap();

    assert!(!passes_through_link(&real, &real.join("uid").join("a.pdf")));
    assert!(passes_through_link(&real, &real.join("uid").join("b.pdf")));
    assert!(passes_through_link(
        &real,
        &real.join("linked-uid").join("a.pdf")
    ));
    assert!(!passes_through_link(
        &alias,
        &alias.join("uid").join("a.pdf")
    ));
    assert!(!passes_through_link(
        &real,
        &real.join("uid").join("gone.pdf")
    ));
    assert!(!passes_through_link(&real, Path::new("/elsewhere/a.pdf")));
}

// ── stored files under an earlier build's names ──

/// The fixture with its stored file named `COM¹.pdf` and a second one `CON .tar.gz`:
/// names the rule before today's let through, and today's reserves (see
/// `slug::legacy_safe_file_name`). With the bytes each holds.
fn with_earlier_names() -> (WorkBundle, Vec<(uuid::Uuid, String, Vec<u8>)>) {
    let mut bundle = build_bundle(ShapeTag::Folder);
    let first = bundle
        .resources
        .iter_mut()
        .find(|r| r.kind == ResourceKind::StoredFile)
        .expect("the fixture stores a file");
    first.file_name = "COM¹.pdf".into();
    let mut second = first.clone();
    second.file_id = 9090;
    second.uid = common::uid::fixture_uid(90);
    second.file_name = "CON .tar.gz".into();
    second.title = "Tide tables".into();
    let stored = vec![
        (first.uid, first.file_name.clone(), b"%PDF-1.7 map".to_vec()),
        (
            second.uid,
            second.file_name.clone(),
            b"tide tables".to_vec(),
        ),
    ];
    bundle.resources.push(second);
    bundle.resource_blobs = stored
        .iter()
        .map(|(uid, _, bytes)| {
            (
                uid.hyphenated().to_string(),
                crate::Blob::from(bytes.clone()),
            )
        })
        .collect();
    (bundle, stored)
}

/// `bundle`'s stored files each hold the bytes `stored` says.
fn holds(bundle: &WorkBundle, stored: &[(uuid::Uuid, String, Vec<u8>)], what: &str) {
    for (uid, name, bytes) in stored {
        let blob = bundle
            .resource_blobs
            .get(&uid.hyphenated().to_string())
            .unwrap_or_else(|| panic!("{what}: {name} is not found"));
        assert_eq!(&blob.read().unwrap(), bytes, "{what}: {name}");
    }
}

/// A project an earlier build saved holds its stored files under the names that
/// build's rule gave them: `COM¹.pdf` where today's rule says `_COM¹.pdf`. They are
/// still found, in a folder and in a single file, and every save writes them under
/// today's name, from which they read back. Opening the folder again (its prune) then
/// leaves only today's name.
#[cfg(unix)]
#[test]
fn stored_files_under_an_earlier_builds_names_are_found_and_saved_under_todays() {
    use super::slug::{legacy_safe_file_name, safe_file_name};
    let dir = tempfile::tempdir().unwrap();
    let (bundle, stored) = with_earlier_names();
    let old = dir.path().join("Old");
    write_bundle(old.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    // As the earlier build wrote them.
    for (uid, name, _) in &stored {
        let folder = old.join("resources").join(uid.hyphenated().to_string());
        let earlier = legacy_safe_file_name(name).expect("today's rule renames it");
        std::fs::rename(folder.join(safe_file_name(name)), folder.join(&earlier)).unwrap();
    }
    let old_zip = dir.path().join("Old.skrib");
    super::zip_io::zip_dir(&old, &mut std::fs::File::create(&old_zip).unwrap()).unwrap();

    for (n, path) in [&old, &old_zip].into_iter().enumerate() {
        let back = read_bundle(path.to_str().unwrap()).expect("opens");
        holds(&back, &stored, &format!("{} as read", path.display()));
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            let new = dir.path().join(format!("New-{n}-{shape:?}.skrib"));
            write_bundle(new.to_str().unwrap(), shape, &back).unwrap();
            let again = read_bundle(new.to_str().unwrap()).unwrap();
            holds(
                &again,
                &stored,
                &format!("{} saved as {shape:?}", path.display()),
            );
            if shape == SkribShape::ExplodedFolder {
                for (uid, name, _) in &stored {
                    let folder = new.join("resources").join(uid.hyphenated().to_string());
                    assert!(folder.join(safe_file_name(name)).is_file(), "{name}");
                    assert!(!folder.join(legacy_safe_file_name(name).unwrap()).exists());
                }
            }
        }
    }

    // Saved in place, the folder holds both names until it is next opened.
    let back = read_bundle(old.to_str().unwrap()).unwrap();
    write_bundle(old.to_str().unwrap(), SkribShape::ExplodedFolder, &back).unwrap();
    let keep: Vec<(uuid::Uuid, String)> = stored
        .iter()
        .map(|(uid, name, _)| (*uid, name.clone()))
        .collect();
    super::media::prune_resources_dir(&old.join("resources"), &keep);
    for (uid, name, bytes) in &stored {
        let folder = old.join("resources").join(uid.hyphenated().to_string());
        assert_eq!(
            &std::fs::read(folder.join(safe_file_name(name))).unwrap(),
            bytes
        );
        assert!(!folder.join(legacy_safe_file_name(name).unwrap()).exists());
    }
    holds(
        &read_bundle(old.to_str().unwrap()).unwrap(),
        &stored,
        "reopened",
    );
}

/// Opening a folder an earlier build saved renames each such file to today's name in
/// its own folder (the prune that runs as a project opens), and `resource_file`, which
/// every other reader asks, finds it under whichever name it has.
#[cfg(unix)]
#[test]
fn opening_renames_a_stored_file_to_todays_name() {
    use super::slug::{legacy_safe_file_name, safe_file_name};
    let dir = tempfile::tempdir().unwrap();
    let resources = dir.path().join("resources");
    let uid = common::uid::fixture_uid(90);
    let folder = resources.join(uid.hyphenated().to_string());
    std::fs::create_dir_all(&folder).unwrap();
    let name = "COM¹.pdf";
    let earlier = folder.join(legacy_safe_file_name(name).unwrap());
    std::fs::write(&earlier, b"%PDF-1.7 map").unwrap();
    assert_eq!(
        super::media::resource_file(&resources, uid, name),
        earlier,
        "found under the earlier name"
    );
    assert_eq!(
        super::media::resource_file(&resources, uid, "CON.pdf"),
        folder.join("_CON.pdf"),
        "a name no rule change touched resolves as it always did"
    );

    super::media::prune_resources_dir(&resources, &[(uid, name.into())]);
    let today = folder.join(safe_file_name(name));
    assert_eq!(std::fs::read(&today).unwrap(), b"%PDF-1.7 map");
    assert!(!earlier.exists());
    assert_eq!(super::media::resource_file(&resources, uid, name), today);
}
