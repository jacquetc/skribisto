// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Projects nested exactly to the XML ceiling, and one level past it.
//!
//! `roxmltree` recurses once per element, and a stack overflow aborts the process
//! rather than unwinding. Every project here is generated in the test and imported
//! on a thread with the 2 MiB stack a long operation gets, in the debug build the
//! test suite runs in, which is where the margin is thinnest.
//!
//! At the ceiling the project imports, which is the measurement behind
//! `skrib_format::MAX_XML_DEPTH`: the reader and every walk after it fit. One level
//! past it the import is refused with the typed `XmlTooDeep`, naming the member,
//! before any parser has seen a byte of it. Every Plume member carries a DOCTYPE,
//! so this is also the reader where nesting can hide inside an entity.

use std::io::Write;
use std::sync::atomic::AtomicBool;

use plume_import::{ImportSummary, import_with_progress};
use skrib_format::{BundledItem, MAX_XML_DEPTH, XmlTooDeep};

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 * 1024 * 1024;

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

/// A `tree` whose deepest element sits `depth` levels down, `<plume-tree>`
/// counting as one: a book, acts inside acts, and a scene at the bottom. Acts,
/// because an act is a container the mapper descends into whatever it holds, so
/// every level reaches the deepest walk the importer has.
fn tree_nested_to(depth: usize) -> String {
    let acts = depth - 3;
    let mut xml = String::from(
        "<!DOCTYPE plume-tree><plume-tree version=\"0.5\" projectName=\"Hostile\">\
         <book number=\"1\" name=\"Book\">",
    );
    for i in 0..acts {
        xml.push_str(&format!("<act number=\"{}\" name=\"Act {i}\">", i + 2));
    }
    xml.push_str(&format!("<scene number=\"{}\" name=\"Bottom\"/>", acts + 2));
    xml.push_str(&"</act>".repeat(acts));
    xml.push_str("</book></plume-tree>");
    xml
}

/// Import `source` into `out` on a thread with a long operation's stack. An
/// overflow would abort the test binary, not fail the test, which is exactly the
/// failure this file exists to rule out.
fn import_on_a_long_operation_stack(source: String, out: String) -> anyhow::Result<ImportSummary> {
    std::thread::Builder::new()
        .stack_size(LONG_OPERATION_STACK)
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

/// Where a test writes its import: a `.skrib` inside `dir`.
fn output_in(dir: &std::path::Path) -> String {
    dir.join("imported.skrib").to_string_lossy().into_owned()
}

/// Every row of the project written to `out`, across both binders.
fn rows_of(out: &str) -> Vec<BundledItem> {
    let bundle = match skrib_format::read_bundle(out) {
        Ok(bundle) => bundle,
        Err(e) => panic!("reading back the imported project: {e}"),
    };
    bundle
        .binders
        .into_iter()
        .flat_map(|binder| binder.items)
        .collect()
}

/// The row titled `title`, which must be there.
#[track_caller]
fn row<'a>(rows: &'a [BundledItem], title: &str) -> &'a BundledItem {
    match rows.iter().find(|row| row.item.title == title) {
        Some(row) => row,
        None => panic!("no row titled '{title}': the deepest level was dropped"),
    }
}

fn refusal_of(result: anyhow::Result<ImportSummary>) -> XmlTooDeep {
    let error = match result {
        Ok(_) => panic!("a project nested past the ceiling must be refused"),
        Err(error) => error,
    };
    skrib_format::xml_depth::too_deep(&error)
        .cloned()
        .unwrap_or_else(|| panic!("the refusal must be the typed one, got: {error:#}"))
}

#[test]
fn a_tree_nested_to_the_ceiling_imports_from_a_long_operation_stack() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = plume(dir.path(), &[("tree", tree_nested_to(MAX_XML_DEPTH))]);
    let out = output_in(dir.path());

    import_on_a_long_operation_stack(source, out.clone()).expect("at the ceiling it imports");

    // The scene sits at the ceiling, one level inside the last act. It is the
    // node a walk stopping one level short would lose, so it is the one to look
    // for, at its own depth.
    let rows = rows_of(&out);
    let first_act = row(&rows, "Act 0");
    let bottom = row(&rows, "Bottom");
    assert_eq!(
        bottom.item.indent - first_act.item.indent,
        (MAX_XML_DEPTH - 3) as i64,
        "the scene must arrive nested under every act"
    );
}

#[test]
fn a_tree_one_level_past_the_ceiling_is_refused_by_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = plume(dir.path(), &[("tree", tree_nested_to(MAX_XML_DEPTH + 1))]);

    let refused = refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "tree");
    assert_eq!(refused.depth, MAX_XML_DEPTH + 1);
}

/// `roxmltree` expands an entity's value in place, at the depth of the
/// reference, so a tree whose own tags stop far short of the ceiling can still
/// nest past it once expanded.
#[test]
fn nesting_hidden_in_an_entity_is_counted_where_it_is_expanded() {
    let dir = tempfile::tempdir().expect("tempdir");
    let hidden = format!(
        "{}{}",
        "<chapter number=\"1\" name=\"x\">".repeat(200),
        "</chapter>".repeat(200)
    );
    let tree = format!(
        "<!DOCTYPE plume-tree [<!ENTITY deep '{hidden}'>]><plume-tree version=\"0.5\">{}&deep;{}</plume-tree>",
        "<chapter number=\"2\" name=\"y\">".repeat(60),
        "</chapter>".repeat(60)
    );
    let source = plume(dir.path(), &[("tree", tree)]);

    let refused = refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "tree");
}

/// `roxmltree` goes on building the tree where an entity's value left it, so an
/// entity that opens an act and never closes it nests everything after each
/// reference one level further, and a second entity closes them again. No single
/// expansion is deep; a thousand of them are, and used to import with every level
/// past the ceiling silently dropped by the walk's own guard.
#[test]
fn nesting_built_from_entities_left_open_is_counted_as_it_builds() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tree = format!(
        "<!DOCTYPE plume-tree [<!ENTITY o '<act number=\"2\" name=\"Deep\">'>\
         <!ENTITY c '<scene number=\"3\" name=\"Bottom\"/></act>'>]>\
         <plume-tree version=\"0.5\"><book number=\"1\" name=\"Book\">{}{}</book></plume-tree>",
        "&o;".repeat(1000),
        "&c;".repeat(1000)
    );
    let source = plume(dir.path(), &[("tree", tree)]);

    let refused = refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "tree");
}

/// `info` is optional metadata, and a malformed one only costs the title. One
/// nested past the ceiling refuses the project all the same: it is not damaged
/// metadata but a file built to crash its reader.
#[test]
fn an_info_member_past_the_ceiling_refuses_the_project() {
    let dir = tempfile::tempdir().expect("tempdir");
    let info = format!(
        "<!DOCTYPE plume-information><plume-information version=\"0.3\">{}{}</plume-information>",
        "<prj>".repeat(MAX_XML_DEPTH),
        "</prj>".repeat(MAX_XML_DEPTH)
    );
    let source = plume(dir.path(), &[("tree", tree_nested_to(4)), ("info", info)]);

    let refused = refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "info");
}
