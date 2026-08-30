// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading real Manuskript projects — ones nobody on this side authored.
//!
//! Gated on `SKRIBISTO_MANUSKRIPT_CORPUS`, a directory holding one or more
//! projects (a Manuskript checkout's `sample-projects/` is the obvious one).
//! Skipped, loudly, when it is unset.
//!
//! Every other test in this crate runs against a fixture we wrote, which proves
//! the reader agrees with our reading of the format. Only this one can catch the
//! reading itself being wrong. It deliberately asserts shape rather than exact
//! content, so it does not break when the corpus is a different project.

use std::path::{Path, PathBuf};

use manuskript_import::model::OutlineItem;
use manuskript_import::read_project;
use manuskript_import::source::ManuskriptSource;

fn corpus_root() -> Option<PathBuf> {
    std::env::var_os("SKRIBISTO_MANUSKRIPT_CORPUS").map(PathBuf::from)
}

/// Every project under `root`: a `.msk` of either kind, or a directory holding a
/// `MANUSKRIPT` marker.
fn projects(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("msk") {
            found.push(path);
        } else if path.is_dir() {
            if path.join("MANUSKRIPT").is_file() {
                found.push(path.clone());
            }
            found.extend(projects(&path));
        }
    }
    found.sort();
    found.dedup();
    found
}

fn count(items: &[OutlineItem], folders: &mut usize, texts: &mut usize, words: &mut usize) {
    for item in items {
        if item.is_folder() {
            *folders += 1;
        } else {
            *texts += 1;
            *words += item.text.split_whitespace().count();
        }
        count(&item.children, folders, texts, words);
    }
}

#[test]
fn every_real_project_reads_into_a_manuscript_with_prose_in_it() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "SKRIBISTO_MANUSKRIPT_CORPUS unset — skipping the real-project read. \
             Point it at a Manuskript checkout's sample-projects/ to run it."
        );
        return;
    };
    let found = projects(&root);
    assert!(
        !found.is_empty(),
        "no Manuskript project under {} — expected a .msk or a folder with a MANUSKRIPT marker",
        root.display()
    );

    for path in &found {
        let src = match ManuskriptSource::open(&path.to_string_lossy()) {
            Ok(src) => src,
            Err(e) => panic!("opening {}: {e:#}", path.display()),
        };
        let container = src.container;
        let format = src.format;
        let project = read_project(&src);

        let (mut folders, mut texts, mut words) = (0usize, 0usize, 0usize);
        count(&project.outline, &mut folders, &mut texts, &mut words);

        assert!(
            texts > 0,
            "{} read as {} but has no text items",
            path.display(),
            container.label()
        );
        assert!(
            words > 0,
            "{} has {texts} text items and not one word of prose",
            path.display()
        );
        // A row Manuskript wrote always has an id; a project full of rows without
        // one means the header parser is dropping the key.
        let mut all = Vec::new();
        for item in &project.outline {
            item.walk(true, &mut all);
        }
        let idless = all.iter().filter(|(i, _)| i.id.is_none()).count();
        assert_eq!(
            idless,
            0,
            "{}: {idless} of {} rows lost their ID",
            path.display(),
            all.len()
        );
        // Titles likewise: an empty one means the metadata never reached us.
        let untitled = all
            .iter()
            .filter(|(i, _)| i.title.trim().is_empty())
            .count();
        assert_eq!(
            untitled,
            0,
            "{}: {untitled} rows have no title",
            path.display()
        );

        // Every vocabulary index an item cites must resolve, or the 1-based
        // convention is being read wrongly.
        for (item, _) in &all {
            if let Some(index) = item.label {
                assert!(
                    project.label_at(index).is_some(),
                    "{}: '{}' cites label {index} of {}",
                    path.display(),
                    item.title,
                    project.labels.len()
                );
            }
            if let Some(index) = item.status {
                assert!(
                    project.status_at(index).is_some(),
                    "{}: '{}' cites status {index} of {}",
                    path.display(),
                    item.title,
                    project.statuses.len()
                );
            }
            // A POV names a character that has to exist.
            if let Some(pov) = item.pov.as_deref() {
                assert!(
                    project
                        .characters
                        .iter()
                        .any(|c| c.id.as_deref() == Some(pov)),
                    "{}: '{}' is told through character {pov}, who is not in the cast",
                    path.display(),
                    item.title
                );
            }
        }

        // A plot's characters are ids, not names.
        for plot in &project.plots {
            for id in &plot.characters {
                assert!(
                    project
                        .characters
                        .iter()
                        .any(|c| c.id.as_deref() == Some(id.as_str())),
                    "{}: plot '{}' names character {id}, who is not in the cast",
                    path.display(),
                    plot.name
                );
            }
        }

        eprintln!(
            "{}: {} {:?} — {folders} folders, {texts} scenes, {words} words, \
             {} characters, {} world rows, {} plots, {} revisions, {} labels, {} statuses",
            path.display(),
            container.label(),
            format,
            project.characters.len(),
            project.world.len(),
            project.plots.len(),
            project.revisions.len(),
            project.labels.len(),
            project.statuses.len(),
        );
        for notice in &project.notices {
            eprintln!("    note: {notice}");
        }
    }
}

/// Test names, in the shape the app supplies them.
fn names() -> manuskript_import::map::Names {
    manuskript_import::map::Names {
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

#[test]
fn every_real_project_converts_to_a_skrib_that_reads_back() {
    let Some(root) = corpus_root() else {
        eprintln!("SKRIBISTO_MANUSKRIPT_CORPUS unset — skipping the real-project conversion");
        return;
    };
    let dir = tempfile::tempdir().expect("tempdir");

    for (n, path) in projects(&root).iter().enumerate() {
        let out = dir.path().join(format!("imported-{n}.skrib"));
        let out = out.to_string_lossy().into_owned();
        let summary = manuskript_import::import_with_progress(
            &path.to_string_lossy(),
            &out,
            false,
            &names(),
            &|_, _| {},
            &std::sync::atomic::AtomicBool::new(false),
        );
        let summary = match summary {
            Ok(s) => s,
            Err(e) => panic!("importing {}: {e:#}", path.display()),
        };

        // The proof that matters: the bundle we wrote is one this project's own
        // reader accepts.
        let bundle = match skrib_format::read_bundle(&out) {
            Ok(b) => b,
            Err(e) => panic!("reading back {}: {e}", path.display()),
        };

        let manuscript = bundle.binders.first().expect("a manuscript binder");
        let scenes = manuscript
            .items
            .iter()
            .filter(|i| i.item.sub_role == common::entities::BinderItemSubRole::Scene)
            .count();
        let books = manuscript
            .items
            .iter()
            .filter(|i| i.item.sub_role == common::entities::BinderItemSubRole::Book)
            .count();
        assert_eq!(books, 1, "{}: expected exactly one Book", path.display());
        assert!(scenes > 0, "{}: no scenes arrived", path.display());

        eprintln!(
            "{} -> {} items, {} revisions, {} binders, {} tags, {} statuses",
            path.display(),
            summary.imported_items,
            summary.imported_revisions,
            bundle.binders.len(),
            bundle.tags.len(),
            bundle.statuses.len(),
        );
        for w in &summary.warnings {
            eprintln!("    note: {w}");
        }
    }
}
