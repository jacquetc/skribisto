// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Symbolic links in a folder project, one kind of file at a time: what the reader
//! refuses, what it leaves out, and that a save never writes through one (see
//! `folder_io`'s module docs). The stored files and pictures are in `resource_tests`.
#![cfg(unix)]

use std::path::{Path, PathBuf};

use super::bundle::{ShapeTag, WorkBundle};
use super::versions::{BackupVersions, SourceKind, VersionRef, VersionSource};
use super::{SkribShape, read_bundle, write_bundle};

/// A folder project with comments, footnotes, note templates and a history, written
/// under `dir`, and the bundle it was written from.
fn project(dir: &Path) -> (PathBuf, WorkBundle) {
    let mut bundle = super::tests::build_bundle_with_footnotes(ShapeTag::Folder);
    super::history::record(&mut bundle, chrono::Utc::now());
    let root = dir.join("Novel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    (root, bundle)
}

/// Move `path` (a file or a folder) out of the project into `dir/elsewhere` and leave
/// a link to it in its place. The bytes stay the same: what is judged is the link.
fn relink(dir: &Path, path: &Path) -> PathBuf {
    let elsewhere = dir.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).unwrap();
    let moved = elsewhere.join(path.file_name().unwrap());
    std::fs::rename(path, &moved).unwrap();
    std::os::unix::fs::symlink(&moved, path).unwrap();
    moved
}

/// The first prose file of the bundle, relative to its root.
fn first_prose(bundle: &WorkBundle) -> String {
    bundle.binders[0].items[0].item.prose_refs[0].path.clone()
}

/// The prose path whose content carries a sidecar of the kind `has` picks.
fn prose_with(
    bundle: &WorkBundle,
    has: impl Fn(&super::bundle::BundledItem, u64) -> bool,
) -> String {
    bundle
        .binders
        .iter()
        .flat_map(|b| &b.items)
        .flat_map(|i| i.item.prose_refs.iter().map(move |pr| (i, pr)))
        .find(|(i, pr)| has(i, pr.file_id))
        .map(|(_, pr)| pr.path.clone())
        .expect("the fixture has one")
}

fn sidecar(prose: &str, kind: &str) -> String {
    format!("{}.{kind}.ron", prose.strip_suffix(".djot").unwrap())
}

fn refusal(root: &Path) -> String {
    match read_bundle(root.to_str().unwrap()) {
        Ok(_) => panic!("the project opened"),
        Err(e) => format!("{e:#}"),
    }
}

/// Every file the project cannot open without is refused when it is a link, naming
/// the file, rather than read as missing: read as missing, the next save would write
/// the list or the scene back empty.
#[test]
fn a_linked_manifest_list_prose_footnote_or_template_refuses_the_project() {
    let probe = tempfile::tempdir().unwrap();
    let (_, bundle) = project(probe.path());
    let mut kinds: Vec<String> = [
        "project.skrib",
        "tags.ron",
        "templates.ron",
        "resources.ron",
        "statuses.ron",
    ]
    .map(String::from)
    .to_vec();
    kinds.push(first_prose(&bundle));
    kinds.push(sidecar(
        &prose_with(&bundle, |i, id| i.footnotes.contains_key(&id)),
        "footnotes",
    ));
    kinds.push(bundle.note_templates[0].path.clone());
    let items = Path::new(&first_prose(&bundle))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .join("items.ron");
    kinds.push(items.to_str().unwrap().to_string());

    for rel in kinds {
        let dir = tempfile::tempdir().unwrap();
        let (root, _) = project(dir.path());
        let outside = relink(dir.path(), &root.join(&rel));
        let before = std::fs::read(&outside).unwrap();
        let message = refusal(&root);
        assert!(
            message.contains("symbolic link") && message.contains(&rel),
            "{rel}: {message}"
        );
        assert_eq!(
            std::fs::read(&outside).unwrap(),
            before,
            "{rel} is untouched"
        );
    }
}

/// The two readers of the manifest alone, the version gate and the backup sniff,
/// refuse it the same way.
#[test]
fn a_linked_manifest_is_refused_by_its_light_readers_too() {
    let dir = tempfile::tempdir().unwrap();
    let (root, _) = project(dir.path());
    relink(dir.path(), &root.join("project.skrib"));
    let err = super::peek_manifest(root.to_str().unwrap()).unwrap_err();
    assert!(format!("{err:#}").contains("symbolic link"), "{err:#}");
    let err =
        super::version_gate::check_version_gate(root.to_str().unwrap(), SkribShape::ExplodedFolder)
            .unwrap_err();
    assert!(err.to_string().contains("symbolic link"), "{err}");
}

/// A whole binder folder that is a link refuses the project: its `items.ron` and its
/// prose are under the link.
#[test]
fn a_linked_binder_folder_refuses_the_project() {
    let dir = tempfile::tempdir().unwrap();
    let (root, bundle) = project(dir.path());
    let binder = Path::new(&first_prose(&bundle))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf();
    relink(dir.path(), &root.join(&binder));
    assert!(refusal(&root).contains("symbolic link"));
}

/// A comment list that is a link is left out and the book opens whole; the next save
/// removes the link alone, and the file it pointed at is still there.
#[test]
fn a_linked_comment_list_is_left_out_and_a_save_removes_only_the_link() {
    let dir = tempfile::tempdir().unwrap();
    let (root, bundle) = project(dir.path());
    let prose = prose_with(&bundle, |i, id| i.comments.contains_key(&id));
    let rel = sidecar(&prose, "comments");
    let outside = relink(dir.path(), &root.join(&rel));
    let before = std::fs::read(&outside).unwrap();

    let back = read_bundle(root.to_str().unwrap()).expect("the project opens");
    let with_comments = back
        .binders
        .iter()
        .flat_map(|b| &b.items)
        .flat_map(|i| i.item.prose_refs.iter().map(move |pr| (i, pr)))
        .find(|(_, pr)| pr.path == prose)
        .expect("the scene is there");
    assert!(
        with_comments.0.comments.is_empty(),
        "the linked comments are left out"
    );
    assert!(
        with_comments.0.prose.values().any(|t| !t.is_empty()),
        "the prose beside them is read"
    );

    super::folder_io::write_folder(&root, &back).unwrap();
    assert!(
        std::fs::symlink_metadata(root.join(&rel)).is_err(),
        "the link is pruned"
    );
    assert_eq!(std::fs::read(&outside).unwrap(), before);
}

/// The history index or one of its blobs as a link is left out, by the folder reader
/// and by `history::load` (what a save carries forward and the Versions pane reads).
#[test]
fn a_linked_history_index_or_blob_is_left_out() {
    let dir = tempfile::tempdir().unwrap();
    let (root, bundle) = project(dir.path());
    let path = root.to_str().unwrap();
    assert!(!bundle.history.is_empty(), "the fixture has a history");
    let hash = bundle.history.entries[0].hash.clone();
    relink(dir.path(), &root.join(super::history::blob_relpath(&hash)));

    let back = read_bundle(path).expect("the project opens");
    assert!(!back.history.blobs.contains_key(&hash));
    assert!(back.history.entries.iter().all(|e| e.hash != hash));
    assert!(!super::history::load(path).blobs.contains_key(&hash));

    relink(dir.path(), &root.join(super::history::HISTORY_INDEX));
    assert!(read_bundle(path).expect("opens").history.is_empty());
    assert!(super::history::load(path).is_empty());
}

/// A file this build does not model is carried through a save, unless it is a link
/// or under a linked folder: its bytes are not the project's.
#[test]
fn a_linked_unmodelled_file_is_not_carried() {
    let dir = tempfile::tempdir().unwrap();
    let (root, _) = project(dir.path());
    let secret = dir.path().join("id_rsa");
    std::fs::write(&secret, b"PRIVATE KEY").unwrap();
    std::fs::create_dir_all(root.join("notes")).unwrap();
    std::fs::write(root.join("notes/own.txt"), b"mine").unwrap();
    std::os::unix::fs::symlink(&secret, root.join("notes/key.txt")).unwrap();
    std::os::unix::fs::symlink(dir.path(), root.join("extras")).unwrap();

    let path = root.to_str().unwrap();
    let back = read_bundle(path).expect("the project opens");
    let keys: Vec<&String> = back.carried.keys().collect();
    assert_eq!(keys, vec!["notes/own.txt"]);
    let keys: Vec<String> = super::carry::load(path).unwrap().into_keys().collect();
    assert_eq!(keys, vec!["notes/own.txt".to_string()]);
}

/// A past version is read out of a backup the same way: a linked prose file in a
/// folder backup is refused, and a linked binder list leaves its rows out.
#[test]
fn a_folder_backup_is_not_read_through_a_link() {
    let dir = tempfile::tempdir().unwrap();
    let (root, bundle) = project(dir.path());
    let path = root.to_str().unwrap().to_string();
    let source = BackupVersions {
        directories: Vec::new(),
        work_unique_id: String::new(),
        project_path: path.clone(),
    };
    let v = VersionRef {
        path: root.clone(),
        taken_at: chrono::Utc::now(),
        source: SourceKind::Backup,
    };
    let prose = first_prose(&bundle);
    assert!(source.prose(&v, &prose).is_ok());
    relink(dir.path(), &root.join(&prose));
    let err = source.prose(&v, &prose).unwrap_err();
    assert!(format!("{err:#}").contains("symbolic link"), "{err:#}");

    let rows = source.index(&v).unwrap().rows.len();
    let items = Path::new(&prose)
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .join("items.ron");
    relink(dir.path(), &root.join(items));
    assert!(source.index(&v).unwrap().rows.len() < rows);
}

// ── the writer ──

/// A prose file replaced by a link to a file outside the project holding the very
/// same text: the save writes the project's own file in its place rather than
/// keeping the link because the bytes match, and the outside file is untouched.
#[test]
fn a_save_replaces_a_linked_prose_file_with_the_projects_own() {
    let dir = tempfile::tempdir().unwrap();
    let (root, bundle) = project(dir.path());
    let prose = root.join(first_prose(&bundle));
    let outside = relink(dir.path(), &prose);
    let before = std::fs::read(&outside).unwrap();

    super::folder_io::write_folder(&root, &bundle).unwrap();
    let meta = std::fs::symlink_metadata(&prose).unwrap();
    assert!(
        meta.is_file(),
        "the link is replaced by a file of the project"
    );
    assert_eq!(std::fs::read(&prose).unwrap(), before);
    assert_eq!(std::fs::read(&outside).unwrap(), before, "untouched");
    read_bundle(root.to_str().unwrap()).expect("and the project opens again");
}

/// A folder the save writes into, a binder's `text/` or a stored resource's folder,
/// replaced by a link to a folder elsewhere: nothing is written there, and the
/// project gets a real folder with its files in it.
#[test]
fn a_save_never_writes_into_a_linked_folder() {
    let dir = tempfile::tempdir().unwrap();
    let mut bundle = super::tests::build_bundle(ShapeTag::Folder);
    bundle.resource_blobs = super::tests::sample_resource_blobs();
    let root = dir.path().join("Novel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    let text = root
        .join(first_prose(&bundle))
        .parent()
        .unwrap()
        .to_path_buf();
    let stored = root
        .join("resources")
        .join(common::uid::fixture_uid(60).hyphenated().to_string());
    let listing = |p: &Path| -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(p)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    };
    for folder in [&text, &stored] {
        let outside = dir.path().join("outside");
        std::fs::create_dir_all(&outside).unwrap();
        if folder.exists() {
            std::fs::remove_dir_all(folder).unwrap();
        }
        std::os::unix::fs::symlink(&outside, folder).unwrap();

        super::folder_io::write_folder(&root, &bundle).unwrap();
        assert!(
            listing(&outside).is_empty(),
            "{}: written through the link: {:?}",
            folder.display(),
            listing(&outside)
        );
        assert!(std::fs::symlink_metadata(folder).unwrap().is_dir());
        assert!(!listing(folder).is_empty());
        std::fs::remove_dir_all(&outside).unwrap();
    }
    let back = read_bundle(root.to_str().unwrap()).expect("opens");
    assert_eq!(back.resource_blobs.len(), bundle.resource_blobs.len());
}

/// A folder the save has nothing to write into but prunes, linked to a folder
/// elsewhere: the prune does not list it, so the files there stay.
#[test]
fn a_prune_never_deletes_through_a_linked_folder() {
    let dir = tempfile::tempdir().unwrap();
    let mut bundle = super::tests::build_bundle(ShapeTag::Folder);
    bundle.note_templates.clear();
    bundle.note_template_bodies.clear();
    assert!(bundle.history.is_empty());
    let root = dir.path().join("Novel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let documents = dir.path().join("Documents");
    std::fs::create_dir_all(&documents).unwrap();
    std::fs::write(documents.join("draft.djot"), b"the writer's other book").unwrap();
    std::fs::write(documents.join("index.ron"), b"[]").unwrap();
    for linked in ["templates", "history"] {
        let _ = std::fs::remove_dir_all(root.join(linked));
        std::os::unix::fs::symlink(&documents, root.join(linked)).unwrap();
    }

    super::folder_io::write_folder(&root, &bundle).unwrap();
    assert_eq!(
        std::fs::read(documents.join("draft.djot")).unwrap(),
        b"the writer's other book"
    );
    assert!(documents.join("index.ron").exists());
}

/// Opening a project prunes its resources folder, and a folder project's whole
/// `resources/` that is a link is not listed: every folder in the one it points at
/// that no resource names would go.
#[test]
fn the_resources_prune_never_lists_a_linked_resources_folder() {
    let dir = tempfile::tempdir().unwrap();
    let documents = dir.path().join("Documents");
    std::fs::create_dir_all(documents.join("Thesis")).unwrap();
    std::fs::write(documents.join("Thesis/thesis.odt"), b"the writer's thesis").unwrap();
    let resources = dir.path().join("Novel").join("resources");
    std::fs::create_dir_all(resources.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&documents, &resources).unwrap();

    super::media::prune_resources_dir(&resources, &[]);
    assert!(documents.join("Thesis/thesis.odt").exists());
}
