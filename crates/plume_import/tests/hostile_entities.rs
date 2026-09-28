// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A project whose XML declares an entity and names it over and over.
//!
//! Every Plume member carries a DOCTYPE, so its reader allows one, and `roxmltree`
//! then expands every entity the DOCTYPE declares. It bounds the references made
//! inside an entity's value (its billion-laughs guard) but not the ones the
//! document itself makes: one 64 KiB entity named a thousand times is a 64 MiB
//! title from a 68 KB file, and a member within the importer's size limits could
//! ask for more memory than the computer has, which aborts every window at once.
//!
//! No Plume Creator version writes an entity declaration: its DOCTYPE is always
//! a bare `<!DOCTYPE plume-tree>`. So a member declaring one is refused on its
//! text, before any parser runs, and the whole project with it. Each test here
//! measures the heap while it imports (see `skrib_format::xml_depth::fixtures`):
//! a refusal that came after the expansion would be no refusal at all.

use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use plume_import::{ImportSummary, import_with_progress};
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
/// expansion. Opening the archive and reading its member costs a few mebibytes;
/// holding the expansion even once costs all of it.
const REFUSAL_BUDGET: usize = EXPANDED / 16;

/// Long enough for a loaded machine running a debug build, and far longer than a
/// refusal made on the text needs.
const REFUSAL_TIME: Duration = Duration::from_secs(10);

/// The stack every import here runs on, as in `hostile_depth.rs`: less than a
/// fifth of what a long operation gets.
const SMALL_STACK: usize = 384 * 1024;

/// A `.plume` zip holding `members`.
fn plume(root: &std::path::Path, members: &[(&str, String)]) -> String {
    let path = root.join("hostile.plume");
    let file = std::fs::File::create(&path).expect("zip");
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default();
    for (name, content) in members {
        zip.start_file(*name, options).expect("member");
        zip.write_all(content.as_bytes()).expect("member bytes");
    }
    zip.finish().expect("finish");
    path.to_string_lossy().into_owned()
}

/// The internal subset declaring `e`, and the run of references naming it.
fn bomb() -> (String, String) {
    (
        format!("[<!ENTITY e \"{}\">]", "a".repeat(ENTITY_BYTES)),
        "&e;".repeat(REFERENCES),
    )
}

/// A plain `tree`: one book, one chapter, one scene.
fn plain_tree() -> String {
    "<!DOCTYPE plume-tree><plume-tree version=\"0.5\" projectName=\"Plain\">\
     <book number=\"1\" name=\"Book\"><chapter number=\"2\" name=\"Chapter\">\
     <scene number=\"3\" name=\"Scene\"/></chapter></book></plume-tree>"
        .to_string()
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

/// Import `source` into `out` on a thread of its own, the way the use case runs it.
fn import_on_a_long_operation_stack(source: String, out: String) -> anyhow::Result<ImportSummary> {
    std::thread::Builder::new()
        .stack_size(SMALL_STACK)
        .spawn(move || {
            import_with_progress(
                &source,
                &out,
                false,
                "Manuscript",
                "Story Bible",
                &[],
                &|_, _| {},
                &AtomicBool::new(false),
            )
        })
        .expect("spawn the import thread")
        .join()
        .expect("the import must not unwind")
}

/// Import `members` as a `.plume` and check that the heap never came near the
/// expansion and the import came back quickly; returns what the import returned.
#[track_caller]
fn import_measured(members: &[(&str, String)]) -> anyhow::Result<ImportSummary> {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = plume(dir.path(), members);
    let member_bytes: usize = members.iter().map(|(_, text)| text.len()).sum();
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
        "the entity was expanded: the heap rose {:.1} MiB for {member_bytes} bytes of XML \
         whose references expand to {} MiB (budget {} MiB), in {elapsed:.2?}; the import \
         {outcome}",
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

/// The outline is the one member every project has, and the one whose names
/// become every title.
#[test]
fn a_tree_naming_an_entity_over_and_over_is_refused_before_it_is_expanded() {
    let (subset, references) = bomb();
    let tree = format!(
        "<!DOCTYPE plume-tree {subset}><plume-tree version=\"0.5\" projectName=\"Bomb\">\
         <book number=\"1\" name=\"Book\"><chapter number=\"2\" name=\"Chapter\">\
         <scene number=\"3\" name=\"{references}\"/></chapter></book></plume-tree>"
    );

    let refused = entity_refusal_of(import_measured(&[("tree", tree)]));

    assert_eq!(refused.part, "tree");
    assert_eq!(refused.line, 1);
}

/// `info` is optional metadata, and a malformed one only costs the title. One
/// declaring an entity refuses the project all the same: no Plume version writes
/// one, so it is not damaged metadata but a file built to exhaust memory.
#[test]
fn an_info_member_naming_an_entity_over_and_over_refuses_the_project() {
    let (subset, references) = bomb();
    let info = format!(
        "<!DOCTYPE plume-information {subset}><plume-information version=\"0.3\">\
         <prj name=\"{references}\"/></plume-information>"
    );

    let refused = entity_refusal_of(import_measured(&[("tree", plain_tree()), ("info", info)]));

    assert_eq!(refused.part, "info");
}
