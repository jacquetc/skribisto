// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Importing the committed fixture projects end to end.
//!
//! These are written by `tests/fixtures/generate.py` in the shape Manuskript's own
//! save code produces, and cover the three ways a project reaches a reader: the
//! modern folder beside its one-byte `.msk`, the single-file zip, and a zip whose
//! member names carry Windows separators. See the NOTICE beside them for why the
//! fixtures are written rather than borrowed.
//!
//! The unit tests in the crate prove each reader against text it was handed. This
//! file proves the whole path: a project on disk, in one of its real containers,
//! through the importer, into a `.skrib` that this project's own reader accepts.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use common::entities::{BinderItemRole as Role, BinderItemSubRole as SubRole, ContentRole};
use manuskript_import::map::Names;
use manuskript_import::{ImportSummary, import_with_progress};
use skrib_format::{BundledItem, WorkBundle};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn names() -> Names {
    Names {
        manuscript_binder: "Manuscript".into(),
        story_bible_binder: "Story bible".into(),
        characters_group: "Characters".into(),
        world_group: "World".into(),
        plots_group: "Plots".into(),
        project_info_note: "Project information".into(),
        summary_note: "Summary".into(),
        importance: ["Minor".into(), "Secondary".into(), "Main".into()],
    }
}

/// Import a fixture and read the result back through the real reader.
fn import(name: &str) -> (ImportSummary, WorkBundle, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    let out = dir
        .path()
        .join("imported.skrib")
        .to_string_lossy()
        .into_owned();
    // The reporter is an `Fn`, so the ticks land in a cell rather than a
    // captured `mut`.
    let ticks: std::cell::RefCell<Vec<f32>> = std::cell::RefCell::new(Vec::new());
    let summary = import_with_progress(
        &fixture(name).to_string_lossy(),
        &out,
        false,
        &names(),
        &|percent, _| ticks.borrow_mut().push(percent),
        &AtomicBool::new(false),
    );
    let summary = match summary {
        Ok(s) => s,
        Err(e) => panic!("importing {name}: {e:#}"),
    };
    let ticks = ticks.into_inner();
    assert!(
        ticks.windows(2).all(|w| w[0] <= w[1]),
        "progress went backwards: {ticks:?}"
    );
    assert_eq!(
        ticks.last().copied(),
        Some(100.0),
        "progress reached the end"
    );
    let bundle = match skrib_format::read_bundle(&out) {
        Ok(b) => b,
        Err(e) => panic!("reading back {name}: {e}"),
    };
    (summary, bundle, dir)
}

fn rows(bundle: &WorkBundle, binder: usize) -> &[BundledItem] {
    &bundle.binders[binder].items
}

fn find<'a>(bundle: &'a WorkBundle, binder: usize, title: &str) -> &'a BundledItem {
    rows(bundle, binder)
        .iter()
        .find(|i| i.item.title == title)
        .unwrap_or_else(|| panic!("no row titled '{title}'"))
}

fn prose(item: &BundledItem, role: ContentRole) -> String {
    let reference = item
        .item
        .prose_refs
        .iter()
        .find(|p| p.role == role)
        .unwrap_or_else(|| panic!("'{}' has no {role:?}", item.item.title));
    item.prose
        .get(&reference.file_id)
        .cloned()
        .unwrap_or_default()
}

#[test]
fn the_folder_project_imports_with_its_whole_shape() {
    let (summary, bundle, _dir) = import("tour-du-monde");
    assert_eq!(bundle.binders.len(), 2, "a manuscript and a story bible");

    let manuscript = rows(&bundle, 0);
    let books = manuscript
        .iter()
        .filter(|i| i.item.sub_role == SubRole::Book)
        .count();
    let parts = manuscript
        .iter()
        .filter(|i| i.item.sub_role == SubRole::Part)
        .count();
    let chapters = manuscript
        .iter()
        .filter(|i| i.item.sub_role == SubRole::ChapterScene)
        .count();
    let scenes = manuscript
        .iter()
        .filter(|i| i.item.sub_role == SubRole::Scene)
        .count();
    assert_eq!(books, 1, "one synthesised Book");
    // Three top-level folders become three parts: the two real ones, and the
    // "Chutes" directory the fixture leaves without a `folder.txt`, which
    // Manuskript would have skipped along with everything inside it. A recovered
    // folder takes the rung its depth gives it, like any other; the writer can
    // retype it.
    assert_eq!(
        parts, 3,
        "two parts, plus the folder recovered from a bare directory"
    );
    assert_eq!(chapters, 12, "twelve chapter folders");
    assert!(scenes >= 12, "a scene per chapter, plus the loose ones");
    assert_eq!(
        manuscript.last().map(|i| i.item.sub_role.clone()),
        Some(SubRole::BookEnd),
        "the book is closed"
    );
    assert!(summary.imported_items as usize >= manuscript.len());
}

/// The title is read from the metadata, never from the file name: `slugify`
/// turns every accented character into a dash.
#[test]
fn french_titles_survive_a_slug_that_destroyed_them() {
    let (_, bundle, _dir) = import("tour-du-monde");
    let titles: Vec<&str> = rows(&bundle, 0)
        .iter()
        .map(|i| i.item.title.as_str())
        .collect();
    assert!(
        titles.iter().any(|t| t.contains("Première partie")),
        "the accented part title came from the metadata: {titles:?}"
    );
    assert!(titles.iter().any(|t| t.starts_with("Chapitre 1")));
}

#[test]
fn the_cast_the_world_and_the_plots_all_arrive() {
    let (_, bundle, _dir) = import("tour-du-monde");
    let bible = rows(&bundle, 1);
    for group in ["Characters", "World", "Plots"] {
        let folder = find(&bundle, 1, group);
        assert_eq!(folder.item.role, Role::Folder);
        assert_eq!(folder.item.sub_role, SubRole::Note, "{group}");
    }
    assert!(bible.iter().any(|i| i.item.title == "Phileas Fogg"));
    assert!(bible.iter().any(|i| i.item.title == "Lieux"));
    assert!(bible.iter().any(|i| i.item.title == "Le pari"));
    assert!(
        bible.iter().any(|i| i.item.title == "Le jour gagné"),
        "a plot's beats"
    );

    // The lead is Main; the swatch had nowhere to go and was reported.
    assert_eq!(find(&bundle, 1, "Phileas Fogg").item.label, "Main");
    // A field the writer added, and the second `Color` that is one too.
    let sheet = prose(find(&bundle, 1, "Phileas Fogg"), ContentRole::NoteText);
    assert!(sheet.contains("Ville d’origine"), "{sheet}");
    assert!(
        sheet.contains("yeux gris"),
        "a second Color is a user field: {sheet}"
    );
}

#[test]
fn a_point_of_view_and_an_inline_mention_become_real_links() {
    let (_, bundle, _dir) = import("tour-du-monde");
    let fogg = find(&bundle, 1, "Phileas Fogg").item.file_id;
    let chapter = find(&bundle, 0, "Chapitre 1");
    assert_eq!(chapter.item.point_of_view_ids, [fogg]);

    let scene = rows(&bundle, 0)
        .iter()
        .find(|i| i.item.sub_role == SubRole::Scene && !i.item.prose_refs.is_empty())
        .expect("a scene");
    let text = prose(scene, ContentRole::SceneText);
    assert!(!text.contains("{C:"), "the marker is gone: {text}");
    assert!(
        text.contains("Phileas Fogg"),
        "replaced by the name: {text}"
    );
    assert!(scene.item.reference_ids.contains(&fogg), "and left a link");
}

#[test]
fn the_vocabularies_arrive_as_tags_and_a_ladder_in_the_projects_own_language() {
    let (_, bundle, _dir) = import("tour-du-monde");
    let tag_names: Vec<&str> = bundle.tags.iter().map(|t| t.name.as_str()).collect();
    assert!(tag_names.contains(&"Chapitre"), "{tag_names:?}");
    assert!(tag_names.contains(&"Documentation"), "{tag_names:?}");
    let chapitre = bundle
        .tags
        .iter()
        .find(|t| t.name == "Chapitre")
        .expect("tag");
    assert_eq!(chapitre.color, "#0000ff", "the colour the writer chose");

    let rungs: Vec<&str> = bundle.statuses.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(rungs, ["Plan", "Brouillon", "À relire", "Terminé"]);
    let work = &bundle.manifest.work;
    assert_eq!(work.dict_language, ["fr-FR"], "the project's own language");
    assert_eq!(work.author_name, "Jules Verne");
}

/// Only `0` excludes, and the fixture switches one scene off in each chapter that
/// is a multiple of five.
#[test]
fn a_scene_the_writer_switched_off_stays_off() {
    let (_, bundle, _dir) = import("tour-du-monde");
    let excluded = rows(&bundle, 0)
        .iter()
        .filter(|i| i.item.sub_role == SubRole::Scene && !i.item.is_exportable)
        .count();
    assert!(excluded >= 2, "the switched-off scenes stayed off");
}

#[test]
fn the_recorded_revisions_become_a_version_history() {
    let (summary, bundle, _dir) = import("tour-du-monde");
    assert!(summary.imported_revisions > 0);
    assert_eq!(
        bundle.history.entries.len() as u64,
        summary.imported_revisions
    );
    for entry in &bundle.history.entries {
        assert!(
            bundle.history.blobs.contains_key(&entry.hash),
            "every entry has its blob"
        );
        assert!(
            rows(&bundle, 0)
                .iter()
                .any(|i| i.item.uid == entry.item_uid),
            "every entry names a row that exists"
        );
    }
}

/// Manuskript skips a directory with no `folder.txt` and everything under it, and
/// drops a `.md` with no `ID:` without a word. Both are kept here, and said.
#[test]
fn the_rows_manuskript_would_have_lost_arrive_and_are_reported() {
    let (summary, bundle, _dir) = import("tour-du-monde");
    let titles: Vec<&str> = rows(&bundle, 0)
        .iter()
        .map(|i| i.item.title.as_str())
        .collect();
    assert!(titles.contains(&"Un fragment"), "{titles:?}");
    assert!(titles.contains(&"Une page sans identité"), "{titles:?}");
    assert!(
        titles.contains(&"Chutes"),
        "the folder with no metadata: {titles:?}"
    );

    let joined = summary.warnings.join("\n");
    assert!(joined.contains("no folder.txt"), "{joined}");
    assert!(joined.contains("no ID of its own"), "{joined}");
}

/// The three containers are three ways of holding one project, and must produce
/// the same one.
#[test]
fn every_container_shape_yields_the_same_project() {
    let (folder_summary, folder, _a) = import("tour-du-monde");
    let (zip_summary, zipped, _b) = import("tour-du-monde-zip.msk");
    let (windows_summary, windows, _c) = import("tour-du-monde-windows.msk");

    let shape = |b: &WorkBundle| {
        (
            b.binders.len(),
            b.binders.iter().map(|x| x.items.len()).collect::<Vec<_>>(),
            b.tags.len(),
            b.statuses.len(),
        )
    };
    assert_eq!(shape(&folder), shape(&zipped), "folder against single file");
    assert_eq!(
        shape(&zipped),
        shape(&windows),
        "a Windows-written archive must read the same on this host"
    );
    assert_eq!(folder_summary.imported_items, zip_summary.imported_items);
    assert_eq!(zip_summary.imported_items, windows_summary.imported_items);
    assert_eq!(
        zip_summary.imported_revisions,
        windows_summary.imported_revisions
    );
}

/// The `.msk` in folder mode is one byte pointing at the directory beside it, so
/// either path opens the same project.
#[test]
fn the_one_byte_msk_and_its_folder_are_two_doors_to_one_project() {
    let (via_stub, stub_bundle, _a) = import("tour-du-monde.msk");
    let (via_folder, folder_bundle, _b) = import("tour-du-monde");
    assert_eq!(via_stub.imported_items, via_folder.imported_items);
    assert_eq!(
        stub_bundle.binders[0].items.len(),
        folder_bundle.binders[0].items.len()
    );
    // And the summary says which copy it read.
    assert!(
        via_stub
            .warnings
            .first()
            .is_some_and(|w| w.contains("folder")),
        "{:?}",
        via_stub.warnings
    );
}

#[test]
fn an_existing_target_is_refused_unless_overwrite_was_asked_for() {
    let dir = tempfile::tempdir().expect("tempdir");
    let out = dir.path().join("taken.skrib");
    std::fs::write(&out, b"ORIGINAL").expect("seed");
    let source = fixture("tour-du-monde");

    let refused = import_with_progress(
        &source.to_string_lossy(),
        &out.to_string_lossy(),
        false,
        &names(),
        &|_, _| {},
        &AtomicBool::new(false),
    );
    assert!(
        refused.is_err(),
        "an existing target is not overwritten silently"
    );
    assert_eq!(std::fs::read(&out).ok(), Some(b"ORIGINAL".to_vec()));

    let allowed = import_with_progress(
        &source.to_string_lossy(),
        &out.to_string_lossy(),
        true,
        &names(),
        &|_, _| {},
        &AtomicBool::new(false),
    );
    assert!(
        allowed.is_ok(),
        "{:?}",
        allowed.err().map(|e| e.to_string())
    );
    assert!(skrib_format::read_bundle(&out.to_string_lossy()).is_ok());
}

/// Cancelling must leave nothing behind: not a partial `.skrib`, not the temp file
/// it is written through, and not a damaged copy of a target being replaced.
#[test]
fn cancelling_leaves_the_target_and_the_directory_untouched() {
    let dir = tempfile::tempdir().expect("tempdir");
    let out = dir.path().join("taken.skrib");
    std::fs::write(&out, b"ORIGINAL").expect("seed");

    let cancelled = import_with_progress(
        &fixture("tour-du-monde").to_string_lossy(),
        &out.to_string_lossy(),
        true,
        &names(),
        &|_, _| {},
        // Already set: the first checkpoint stops it.
        &AtomicBool::new(true),
    );
    assert!(cancelled.is_err());
    assert_eq!(std::fs::read(&out).ok(), Some(b"ORIGINAL".to_vec()));
    let leftovers: Vec<_> = std::fs::read_dir(dir.path())
        .expect("read dir")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(leftovers, ["taken.skrib"], "no temp file was left behind");
}

/// Reading is all this does. The fixture is committed, so a test that wrote to it
/// would show up as a dirty tree; this proves it in the test instead.
#[test]
fn the_source_project_is_never_written_to() {
    fn fingerprint(dir: &std::path::Path) -> Vec<(String, u64)> {
        let mut out = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(next) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&next) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if let Ok(meta) = entry.metadata() {
                    out.push((path.to_string_lossy().into_owned(), meta.len()));
                }
            }
        }
        out.sort();
        out
    }
    let source = fixture("tour-du-monde");
    let before = fingerprint(&source);
    let (_, _, _dir) = import("tour-du-monde");
    assert_eq!(before, fingerprint(&source), "the project was modified");
}

/// `read_bundle` proves the files parse. This proves the next layer: that the
/// bundle becomes the object graph `load_work` builds from it, with every
/// relationship resolving.
///
/// It is the step where a dangling id shows up. A tag, a status, a point of view
/// or a reference that names a row the bundle does not contain parses perfectly
/// and only fails here.
#[test]
fn the_produced_bundle_becomes_a_loadable_work() {
    let (_, bundle, dir) = import("tour-du-monde");
    let path = dir
        .path()
        .join("imported.skrib")
        .to_string_lossy()
        .into_owned();
    let loaded = match skrib_format::bundle_to_loaded(bundle, &path) {
        Ok(w) => w,
        Err(e) => panic!("the bundle does not load: {e:#}"),
    };
    assert_eq!(loaded.binders.len(), 2);
    assert!(!loaded.work.title.is_empty());
    let with_prose = loaded
        .binders
        .iter()
        .flat_map(|b| &b.items)
        .filter(|i| i.contents.iter().any(|c| !c.data.trim().is_empty()))
        .count();
    assert!(with_prose > 10, "the prose survived into the loaded graph");

    // Every id an item cites has to resolve against what the same bundle carries.
    let tags: Vec<u64> = loaded.tags.iter().map(|t| t.id).collect();
    let statuses: Vec<u64> = loaded.statuses.iter().map(|s| s.id).collect();
    for binder in &loaded.binders {
        for item in &binder.items {
            for tag in &item.tag_ids {
                assert!(
                    tags.contains(tag),
                    "'{}' cites a tag that is gone",
                    item.item.title
                );
            }
            if let Some(status) = item.status_id {
                assert!(
                    statuses.contains(&status),
                    "'{}' cites a status that is gone",
                    item.item.title
                );
            }
            // Point of view and cross-references are not on `LoadedItem`: this
            // layer carries the per-item vocabulary only, and the self-referencing
            // links are hydrated later, by `load_work` itself. `mapping.rs` asserts
            // them on the bundle instead, which is where this crate can see them.
        }
    }
}

// ── Format 0, the 2016 shape ────────────────────────────────────────────────

/// The whole old format, end to end. There was no fixture for it, and that is
/// exactly why an `html` scene could import as an empty document unnoticed: the
/// unit tests exercised the reader against text they were handed, and nothing
/// carried a real format-0 project through the mapper.
#[test]
fn the_2016_format_imports_with_its_prose_intact() {
    let (summary, bundle, _dir) = import("tour-du-monde-2016.msk");

    let manuscript = rows(&bundle, 0);
    let scenes: Vec<&BundledItem> = manuscript
        .iter()
        .filter(|i| i.item.sub_role == SubRole::Scene)
        .collect();
    assert!(scenes.len() >= 4, "the chapters arrived");

    // Every scene has words in it. The failure this guards is silent: a body
    // handed to the wrong parser comes back empty and the row still exists.
    for scene in &scenes {
        let text = prose(scene, ContentRole::SceneText);
        assert!(
            !text.trim().is_empty(),
            "'{}' imported with no prose at all",
            scene.item.title
        );
    }

    // The first chapter is the `type="html"` one.
    let html_scene = scenes
        .iter()
        .find(|s| prose(s, ContentRole::SceneText).contains("ainsi"))
        .expect("the html scene");
    let text = prose(html_scene, ContentRole::SceneText);
    assert!(
        !text.contains("<p>"),
        "the markup was converted, not carried: {text}"
    );
    assert!(
        text.contains("*ainsi*"),
        "and its bold survived as bold rather than being demoted: {text}"
    );

    // The misspelled summary key of that era is read.
    assert!(
        scenes
            .iter()
            .any(|s| !prose(s, ContentRole::SynopsisText).trim().is_empty()),
        "summarySentance was read"
    );

    assert!(summary.imported_revisions > 0, "its revisions came too");
    let joined = summary.warnings.join("\n");
    assert!(joined.contains("original 2016 format"), "{joined}");
    assert!(
        joined.contains("CVE-2021-35196"),
        "the pickle was refused: {joined}"
    );
}

/// The cast and the vocabularies come out of the generic `<model>` dumps, and
/// the empty leading row those carry must not become a rung.
#[test]
fn the_2016_vocabularies_and_cast_arrive_without_their_empty_rows() {
    let (_, bundle, _dir) = import("tour-du-monde-2016.msk");

    let tag_names: Vec<&str> = bundle.tags.iter().map(|t| t.name.as_str()).collect();
    assert!(tag_names.contains(&"Chapitre"), "{tag_names:?}");
    assert!(
        !tag_names.iter().any(|n| n.trim().is_empty()),
        "the empty 'none' row is not a tag: {tag_names:?}"
    );
    // Format 0 writes #aarrggbb; the alpha must not be read as red.
    let chapitre = bundle
        .tags
        .iter()
        .find(|t| t.name == "Chapitre")
        .expect("tag");
    assert_eq!(chapitre.color, "#0000ff");

    let rungs: Vec<&str> = bundle.statuses.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(rungs, ["Plan", "Brouillon", "À relire", "Terminé"]);

    assert!(
        rows(&bundle, 1)
            .iter()
            .any(|i| i.item.title == "Phileas Fogg"),
        "the cast came out of perso.xml"
    );
}
