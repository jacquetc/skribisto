// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A project whose file names crossed a Unicode-normalisation boundary.
//!
//! The scenario behind every test here: a book written on macOS, carried to Linux by a
//! synchronisation client that emits decomposed (NFD) names, so every accented name on
//! disk differs byte-for-byte from the one `items.ron` records. See [`crate::locate`].
//! On a filesystem that normalises names itself (APFS) the renames these tests perform
//! are not observable and each test returns early: the situation cannot arise there.

use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use unicode_normalization::UnicodeNormalization;

use super::bundle::{ShapeTag, WorkBundle};
use super::folder_io::write_folder;
use super::locate::nfc;
use super::reader::read_bundle;
use super::shape::MANIFEST_NAME;
use super::tests::{SampleInputs, assert_round_trip, build_bundle_with, prose_paths};

/// The fixture with an accent in every name the writer derives from user text: the
/// binder directories, every titled row (and so the untitled rows that borrow a title),
/// and the note templates.
fn accented_bundle(tweak: impl FnOnce(&mut SampleInputs)) -> WorkBundle {
    build_bundle_with(ShapeTag::Folder, |s| {
        for binder in &mut s.binders {
            binder.binder.name = format!("Première partie {}", binder.binder.name);
            for (i, entry) in binder.items.iter_mut().enumerate() {
                if !entry.item.title.trim().is_empty() {
                    entry.item.title = format!("Élément {i} – l'été");
                }
            }
        }
        for t in &mut s.note_templates {
            t.name = format!("Fiche été {}", t.name);
        }
        tweak(s);
    })
}

fn manifest(root: &Path) -> String {
    root.join(MANIFEST_NAME).to_str().unwrap().to_string()
}

/// Every path under `root`, relative, as the filesystem spells it.
fn listing(root: &Path) -> BTreeSet<PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .flatten()
        .filter_map(|e| e.path().strip_prefix(root).ok().map(Path::to_path_buf))
        .filter(|p| !p.as_os_str().is_empty())
        .collect()
}

/// The `.djot` file names under `binders/`, as the filesystem spells them.
fn prose_names_on_disk(root: &Path) -> Vec<String> {
    listing(root)
        .into_iter()
        .filter(|p| p.starts_with("binders") && p.extension() == Some(OsStr::new("djot")))
        .filter_map(|p| p.file_name().and_then(OsStr::to_str).map(str::to_string))
        .collect()
}

/// What a sync client that emits NFD does to a whole project: rename every file and
/// directory under `root` with a distinct decomposed spelling to it, deepest first.
/// Returns the relative paths it renamed, or `None` on a filesystem that normalises
/// names itself, where the precomposed path still resolves after the rename.
fn decompose_tree(root: &Path) -> Option<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = listing(root).into_iter().collect();
    paths.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    let mut renamed = Vec::new();
    for rel in paths {
        let full = root.join(&rel);
        let name = full
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string();
        let decomposed: String = name.nfd().collect();
        if decomposed == name {
            continue;
        }
        if fs::rename(&full, full.with_file_name(&decomposed)).is_err() || full.exists() {
            return None;
        }
        renamed.push(rel);
    }
    Some(renamed)
}

#[test]
fn a_decomposed_tree_reads_back_losslessly() {
    let bundle = accented_bundle(|_| {});
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("livre");
    write_folder(&root, &bundle).unwrap();
    let Some(renamed) = decompose_tree(&root) else {
        return;
    };

    // The fixture has to have exercised every kind of derived name, or a pass here
    // proves less than it claims.
    type Is = fn(&Path) -> bool;
    let kinds: [(&str, Is); 4] = [
        ("a binder directory", |p: &Path| {
            p.starts_with("binders") && p.components().count() == 2
        }),
        ("a prose file", |p: &Path| {
            p.extension() == Some(OsStr::new("djot")) && p.starts_with("binders")
        }),
        ("a comments sidecar", |p: &Path| {
            p.to_str().is_some_and(|s| s.ends_with(".comments.ron"))
        }),
        ("a note-template body", |p: &Path| {
            p.starts_with("templates")
        }),
    ];
    for (what, is) in kinds {
        assert!(
            renamed.iter().any(|p| is(p)),
            "the fixture never decomposed {what}: {renamed:?}"
        );
    }

    let read = read_bundle(&manifest(&root)).unwrap();
    assert_round_trip(&bundle, &read);
}

#[test]
fn saving_into_a_decomposed_tree_edits_in_place_without_twins_or_renames() {
    let bundle = accented_bundle(|_| {});
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("livre");
    write_folder(&root, &bundle).unwrap();
    if decompose_tree(&root).is_none() {
        return;
    }
    let before = listing(&root);

    // Edit one accented scene's prose, as a writer would between two autosaves.
    let mut edited = bundle.clone();
    let (file_id, manifest_path) = edited.binders[0]
        .items
        .iter()
        .flat_map(|i| i.item.prose_refs.iter())
        .find(|pr| pr.path != pr.path.nfd().collect::<String>())
        .map(|pr| (pr.file_id, pr.path.clone()))
        .expect("an accented prose path");
    let item = edited.binders[0]
        .items
        .iter_mut()
        .find(|i| i.item.prose_refs.iter().any(|pr| pr.file_id == file_id))
        .unwrap();
    item.prose
        .insert(file_id, "Un texte modifié.\n".to_string());
    write_folder(&root, &edited).unwrap();

    assert_eq!(
        listing(&root),
        before,
        "a save must neither add a precomposed twin nor respell a decomposed name"
    );
    let decomposed_on_disk = root.join(manifest_path.nfd().collect::<String>());
    assert_eq!(
        fs::read_to_string(&decomposed_on_disk).unwrap(),
        "Un texte modifié.\n",
        "the edit lands in the file as it is spelled on disk"
    );
    let binder_dirs = fs::read_dir(root.join("binders")).unwrap().count();
    assert_eq!(
        binder_dirs,
        bundle.binders.len(),
        "one directory per binder, no twins"
    );

    let read = read_bundle(&manifest(&root)).unwrap();
    assert_round_trip(&edited, &read);
}

#[test]
fn a_stale_decomposed_prose_file_is_still_pruned() {
    let bundle = accented_bundle(|_| {});
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("livre");
    write_folder(&root, &bundle).unwrap();
    if decompose_tree(&root).is_none() {
        return;
    }
    let before = prose_names_on_disk(&root);

    // Retitling a row renames its prose files; the old names become stale and must go,
    // decomposed or not — tolerance for a spelling is not a licence to hoard.
    let retitled = accented_bundle(|s| {
        s.binders[0].items[2].item.title = "Un titre tout autre".to_string();
    });
    let kept = prose_paths(&retitled);
    let gone: Vec<String> = prose_paths(&bundle)
        .into_iter()
        .filter(|p| !kept.contains(p))
        .filter_map(|p| Path::new(&p).file_name()?.to_str().map(str::to_string))
        .collect();
    assert!(
        !gone.is_empty(),
        "the retitle must rename at least one prose file"
    );
    write_folder(&root, &retitled).unwrap();

    let after = prose_names_on_disk(&root);
    for stale in &gone {
        assert!(
            !after.iter().any(|n| nfc(n) == nfc(stale)),
            "{stale} is stale and must be pruned; on disk: {after:?}"
        );
    }
    assert_eq!(after.len(), before.len(), "one new name per pruned one");
    let read = read_bundle(&manifest(&root)).unwrap();
    assert_round_trip(&retitled, &read);
}
