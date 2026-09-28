// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A project whose XML declares an entity and names it over and over.
//!
//! Manuskript's readers allow a DTD, since a hand-edited file may carry one, and
//! `roxmltree` then expands every entity the DOCTYPE declares. It bounds the
//! references made inside an entity's value (its billion-laughs guard) but not the
//! ones the document itself makes: one 64 KiB entity named a thousand times is a
//! 64 MiB name from a 68 KB file, and a member within the importer's size limits
//! could ask for more memory than the computer has, which aborts every window at
//! once.
//!
//! Manuskript writes its XML with lxml and no DOCTYPE at all, so no project it
//! wrote declares an entity. A project with a member that does is refused on that
//! member's text, before any parser runs, like one nested past the XML ceiling.
//! Each test here measures the heap while it imports (see
//! `skrib_format::xml_depth::fixtures`): a refusal that came after the expansion
//! would be no refusal at all.

use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use manuskript_import::map::Names;
use manuskript_import::{ImportSummary, import_with_progress};
use skrib_format::XmlDeclaresEntities;
use skrib_format::xml_depth::fixtures::{MIB, PeakAllocator, peak_growth};

#[global_allocator]
static ALLOCATOR: PeakAllocator = PeakAllocator;

/// The declared entity's value, in bytes.
const ENTITY_BYTES: usize = 64 * 1024;
/// How many times the document names it.
const REFERENCES: usize = 1024;
/// What those references expand to: 64 MiB, from a file of about 68 KB.
const EXPANDED: usize = ENTITY_BYTES * REFERENCES;

/// The most the heap may rise while a bomb is refused: a sixteenth of the
/// expansion. Opening the project and reading its members costs a few mebibytes;
/// holding the expansion even once costs all of it.
const REFUSAL_BUDGET: usize = EXPANDED / 16;

/// Long enough for a loaded machine running a debug build, and far longer than a
/// refusal made on the text needs.
const REFUSAL_TIME: Duration = Duration::from_secs(10);

/// The stack every import here runs on, as in `hostile_depth.rs`: less than a
/// fifth of what a long operation gets.
const SMALL_STACK: usize = 384 * 1024;

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

/// The internal subset declaring `e`, and the run of references naming it.
fn bomb() -> (String, String) {
    (
        format!("[<!ENTITY e \"{}\">]", "a".repeat(ENTITY_BYTES)),
        "&e;".repeat(REFERENCES),
    )
}

/// A format-1 folder project holding nothing but `world.opml`.
fn folder_project(root: &std::path::Path, world: &str) -> String {
    let project = root.join("Hostile");
    std::fs::create_dir_all(&project).expect("project folder");
    std::fs::write(project.join("MANUSKRIPT"), "1").expect("marker");
    std::fs::write(project.join("world.opml"), world).expect("world.opml");
    project.to_string_lossy().into_owned()
}

/// A format-0 zipped project holding nothing but `outline.xml`.
fn format_zero_project(root: &std::path::Path, outline: &str) -> String {
    let path = root.join("hostile.msk");
    let file = std::fs::File::create(&path).expect("zip");
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default();
    zip.start_file("outline.xml", options).expect("member");
    zip.write_all(outline.as_bytes()).expect("outline.xml");
    zip.finish().expect("finish");
    path.to_string_lossy().into_owned()
}

/// The typed refusal `result` must be.
#[track_caller]
fn entity_refusal_of(result: anyhow::Result<ImportSummary>) -> XmlDeclaresEntities {
    let error = match result {
        Ok(_) => panic!("a project declaring an entity must be refused"),
        Err(error) => error,
    };
    skrib_format::xml_depth::declares_entities(&error)
        .cloned()
        .unwrap_or_else(|| panic!("the refusal must be the typed one, got: {error:#}"))
}

/// How many bytes the project at `path` holds on disk: the file, or every file of
/// the folder.
fn source_size(path: &std::path::Path) -> u64 {
    match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter_map(|entry| entry.metadata().ok())
            .map(|metadata| metadata.len())
            .sum(),
        Err(_) => std::fs::metadata(path).map_or(0, |metadata| metadata.len()),
    }
}

/// Import `source` into `out` on a thread of its own, the way the use case runs it.
fn import_on_a_long_operation_stack(source: String, out: String) -> anyhow::Result<ImportSummary> {
    std::thread::Builder::new()
        .stack_size(SMALL_STACK)
        .spawn(move || {
            import_with_progress(
                &source,
                &out,
                false,
                &names(),
                &|_, _| {},
                &AtomicBool::new(false),
            )
        })
        .expect("spawn the import thread")
        .join()
        .expect("the import must not unwind")
}

/// Import the project `build` writes into a scratch folder, and check that the
/// heap never came near the expansion and the import came back quickly; returns
/// what the import returned.
#[track_caller]
fn import_measured(
    build: impl FnOnce(&std::path::Path) -> String,
) -> anyhow::Result<ImportSummary> {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = build(dir.path());
    let source_bytes = source_size(std::path::Path::new(&source));
    let out = dir
        .path()
        .join("imported.skrib")
        .to_string_lossy()
        .into_owned();

    // Timed inside the measure, which waits for any other measured test first.
    let ((result, elapsed), growth) = peak_growth(|| {
        let started = Instant::now();
        let result = import_on_a_long_operation_stack(source, out.clone());
        (result, started.elapsed())
    });
    let growth = growth.expect("PeakAllocator is this binary's global allocator");

    let outcome = match &result {
        Ok(summary) => format!("imported {} rows", summary.imported_items),
        Err(error) => format!("failed: {:.200}", format!("{error:#}")),
    };
    assert!(
        growth < REFUSAL_BUDGET,
        "the entity was expanded: the heap rose {:.1} MiB for a project of {source_bytes} \
         bytes on disk whose references expand to {} MiB (budget {} MiB), in \
         {elapsed:.2?}; the import {outcome}",
        growth as f64 / MIB as f64,
        EXPANDED / MIB,
        REFUSAL_BUDGET / MIB,
    );
    assert!(
        elapsed < REFUSAL_TIME,
        "the refusal took {elapsed:.2?}; the import {outcome}"
    );
    assert!(
        !std::path::Path::new(&out).exists(),
        "a refused import must write nothing"
    );
    result
}

/// `world.opml` is read on its own, and a member that cannot be read otherwise
/// only costs the world entries. This one refuses the project.
#[test]
fn a_world_file_naming_an_entity_over_and_over_is_refused_before_it_is_expanded() {
    let (subset, references) = bomb();
    let world = format!(
        "<?xml version='1.0' encoding='UTF-8'?>\n<!DOCTYPE opml {subset}>\
         <opml version=\"1.0\"><body><outline name=\"{references}\" ID=\"1\"/></body></opml>"
    );

    let refused = entity_refusal_of(import_measured(|root| folder_project(root, &world)));

    assert_eq!(refused.part, "world.opml");
    assert_eq!(refused.line, 2);
}

/// Format 0 keeps the whole manuscript in `outline.xml`, read by a reader of its
/// own.
#[test]
fn a_format_zero_outline_naming_an_entity_over_and_over_is_refused_before_it_is_expanded() {
    let (subset, references) = bomb();
    let outline = format!(
        "<!DOCTYPE outlineItem {subset}><outlineItem title=\"Root\" type=\"folder\">\
         <outlineItem title=\"Scene\" ID=\"1\" type=\"md\" text=\"{references}\"/></outlineItem>"
    );

    let refused = entity_refusal_of(import_measured(|root| format_zero_project(root, &outline)));

    assert_eq!(refused.part, "outline.xml");
}
