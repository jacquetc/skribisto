// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Projects nested exactly to the XML ceiling, and one level past it.
//!
//! `roxmltree` recurses once per element, and a stack overflow aborts the process
//! rather than unwinding: a `world.opml` a few kilobytes long used to take every
//! window down with it. Each project here is generated in the test and imported on
//! a thread with less than a fifth of the 2 MiB stack a long operation gets, in the
//! debug build the test suite runs in, which is where the margin is thinnest. The
//! rest of the 2 MiB is room for platforms whose frames are larger than Linux's:
//! see [`SMALL_STACK`].
//!
//! At the ceiling the project imports, which is the measurement behind
//! `skrib_format::MAX_XML_DEPTH`: every reader and every walk after it fits. One
//! level past it the import is refused with the typed `XmlTooDeep`, naming the
//! member, before any parser has seen a byte of it. Nesting could also hide inside
//! an entity, since the readers allow a DTD; a member declaring one is refused for
//! that before its depth is measured (see `hostile_entities.rs`).
//!
//! A row's body is Markdown or HTML, with ceilings of its own
//! (`skrib_format::MAX_MARKDOWN_DEPTH`, `skrib_format::MAX_HTML_DEPTH`). A body past
//! them does not refuse the project: its words are kept as plain text and the
//! writer is told which row.

use std::io::Write;
use std::sync::atomic::AtomicBool;

use manuskript_import::map::Names;
use manuskript_import::{ImportSummary, import_with_progress};
use skrib_format::{BundledItem, FoldersTooDeep, MAX_XML_DEPTH, XmlDeclaresEntities, XmlTooDeep};

/// The stack every import here runs on: 384 KiB, where a long operation's worker
/// gets 2 MiB from `std::thread::spawn`.
///
/// A frame is not the same size on every platform. On macOS the standard library's
/// `DirEntry` carries a 1 KiB name buffer, and the recursive folder walk this file
/// once passed on Linux at 1.7 KiB a level needed 8.7 KiB a level there, so a
/// project nested to the ceiling aborted the macOS test run. An import needs about
/// 190 KiB of this whatever the nesting, mostly the zip writer's fixed-size deflate
/// state, which costs the same on every platform; what is left cannot hold 256
/// levels of anything that recurses on the input. A pass on Linux therefore still
/// holds on a platform whose frames are five times larger. The full account is in
/// `skrib_format::xml_depth`'s module note.
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

/// Import `source` into `out` on a thread of its own, the way the use case runs it,
/// with [`SMALL_STACK`] rather than the long operation's 2 MiB. An overflow would
/// abort the test binary, not fail the test, which is exactly the failure this file
/// exists to rule out.
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

/// A format-1 folder project holding nothing but `world.opml`.
fn folder_project(root: &std::path::Path, world: &str) -> String {
    let project = root.join("Hostile");
    std::fs::create_dir_all(&project).expect("project folder");
    std::fs::write(project.join("MANUSKRIPT"), "1").expect("marker");
    std::fs::write(project.join("world.opml"), world).expect("world.opml");
    project.to_string_lossy().into_owned()
}

/// `world.opml` whose deepest `<outline>` sits `depth` levels down, `<opml>`
/// counting as one and `<body>` as two.
fn world_nested_to(depth: usize) -> String {
    let outlines = depth - 2;
    let mut xml =
        String::from("<?xml version='1.0' encoding='UTF-8'?>\n<opml version=\"1.0\"><body>");
    for i in 0..outlines {
        xml.push_str(&format!("<outline name=\"Place {i}\" ID=\"{i}\">"));
    }
    xml.push_str(&"</outline>".repeat(outlines));
    xml.push_str("</body></opml>");
    xml
}

/// A format-0 zipped project whose `outline.xml` nests `<outlineItem>` `depth`
/// levels deep, the root holder counting as one. Every level is a folder but the
/// last, which carries the prose.
fn format_zero_project(root: &std::path::Path, depth: usize) -> String {
    let mut outline = String::from("<outlineItem title=\"Root\" type=\"folder\">");
    for i in 1..depth - 1 {
        outline.push_str(&format!(
            "<outlineItem title=\"Level {i}\" ID=\"{i}\" type=\"folder\">"
        ));
    }
    outline.push_str(&format!(
        "<outlineItem title=\"Bottom\" ID=\"{depth}\" type=\"md\" text=\"The words at the bottom.\"/>"
    ));
    outline.push_str(&"</outlineItem>".repeat(depth - 1));

    let path = root.join("hostile.msk");
    let file = std::fs::File::create(&path).expect("zip");
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default();
    zip.start_file("outline.xml", options).expect("member");
    zip.write_all(outline.as_bytes()).expect("outline.xml");
    zip.finish().expect("finish");
    path.to_string_lossy().into_owned()
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

fn entity_refusal_of(result: anyhow::Result<ImportSummary>) -> XmlDeclaresEntities {
    let error = match result {
        Ok(_) => panic!("a project hiding nesting in an entity must be refused"),
        Err(error) => error,
    };
    skrib_format::xml_depth::declares_entities(&error)
        .cloned()
        .unwrap_or_else(|| panic!("the refusal must be the typed one, got: {error:#}"))
}

#[test]
fn a_world_tree_nested_to_the_ceiling_imports_from_a_long_operation_stack() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = folder_project(dir.path(), &world_nested_to(MAX_XML_DEPTH));
    let out = output_in(dir.path());

    import_on_a_long_operation_stack(source, out.clone()).expect("at the ceiling it imports");

    // The first `<outline>` sits three levels down and the last at the ceiling,
    // so the entries run from "Place 0" to "Place 253", each one level inside the
    // one before it. The deepest is the entry a walk stopping one level short
    // would lose, so it is the one to look for, at its own depth.
    let rows = rows_of(&out);
    let outlines = MAX_XML_DEPTH - 2;
    let top = row(&rows, "Place 0");
    let bottom = row(&rows, &format!("Place {}", outlines - 1));
    assert_eq!(
        bottom.item.indent - top.item.indent,
        (outlines - 1) as i64,
        "the deepest entry must arrive nested under all the others"
    );
}

#[test]
fn a_world_tree_one_level_past_the_ceiling_is_refused_by_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = folder_project(dir.path(), &world_nested_to(MAX_XML_DEPTH + 1));

    let refused = refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "world.opml");
    assert_eq!(refused.depth, MAX_XML_DEPTH + 1);
}

/// Manuskript's readers allow a DTD, and `roxmltree` expands an entity's value in
/// place, at the depth of the reference. So a file whose own tags stop far short
/// of the ceiling could still nest past it once expanded. It never gets that far:
/// a member declaring an entity refuses the project before any depth is measured.
#[test]
fn nesting_hidden_in_an_entity_is_refused_with_the_entity() {
    let dir = tempfile::tempdir().expect("tempdir");
    let hidden = format!(
        "{}{}",
        "<outline name=\"x\">".repeat(200),
        "</outline>".repeat(200)
    );
    let world = format!(
        "<!DOCTYPE opml [<!ENTITY deep '{hidden}'>]><opml version=\"1.0\"><body>{}&deep;{}</body></opml>",
        "<outline name=\"y\">".repeat(60),
        "</outline>".repeat(60)
    );
    let source = folder_project(dir.path(), &world);

    let refused = entity_refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "world.opml");
}

/// Format 0 keeps the whole manuscript in `outline.xml`, so this one reaches the
/// deepest walk the importer has: every level becomes a row, nested in the one
/// above it, through the same mapper a real project goes through.
#[test]
fn a_format_zero_outline_nested_to_the_ceiling_imports_from_a_long_operation_stack() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = format_zero_project(dir.path(), MAX_XML_DEPTH);
    let out = output_in(dir.path());

    import_on_a_long_operation_stack(source, out.clone()).expect("at the ceiling it imports");

    // "Bottom" sits at the ceiling and is the only row carrying prose, so a walk
    // stopping one level short would lose the one piece of writing in the file.
    let rows = rows_of(&out);
    let top = row(&rows, "Level 1");
    let bottom = row(&rows, "Bottom");
    assert_eq!(
        bottom.item.indent - top.item.indent,
        (MAX_XML_DEPTH - 2) as i64,
        "the deepest row must arrive nested under all the others"
    );
    assert!(
        bottom
            .prose
            .values()
            .any(|text| text.contains("The words at the bottom.")),
        "the deepest row must keep its prose, got {:?}",
        bottom.prose
    );
}

#[test]
fn a_format_zero_outline_one_level_past_the_ceiling_is_refused_by_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = format_zero_project(dir.path(), MAX_XML_DEPTH + 1);

    let refused = refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "outline.xml");
    assert_eq!(refused.depth, MAX_XML_DEPTH + 1);
}

/// `roxmltree` goes on building the tree where an entity's value left it, so an
/// entity that opens an `<outline>` and never closes it nests everything after
/// each reference one level further. No single expansion is deep; a thousand of
/// them are. This used to import, with every level past the ceiling silently
/// dropped by the walk's own guard. Now the project is refused for the
/// declarations.
#[test]
fn nesting_built_from_entities_left_open_is_refused_with_the_entities() {
    let dir = tempfile::tempdir().expect("tempdir");
    let world = format!(
        "<!DOCTYPE opml [<!ENTITY o '<outline name=\"Deep\">'>\
         <!ENTITY c '<outline name=\"Bottom\"/></outline>'>]>\
         <opml version=\"1.0\"><body>{}{}</body></opml>",
        "&o;".repeat(1000),
        "&c;".repeat(1000)
    );
    let source = folder_project(dir.path(), &world);

    let refused = entity_refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, "world.opml");
}

// ---------------------------------------------------------------------------
// Format 1 keeps the outline as folders, one per level
// ---------------------------------------------------------------------------

/// The member path of a text item sitting `folders` folders deep: `outline/`,
/// then one folder per level down to it, the last level being the item itself.
fn outline_member(folders: usize) -> String {
    let mut path = String::from("outline/");
    for level in 1..folders {
        path.push_str(&format!("0-Level_{level}/"));
    }
    path.push_str("0-Bottom.md");
    path
}

const BOTTOM_ITEM: &str = "title:          Bottom\nID:             1\ntype:           md\n\n\n\
                           The words at the bottom.";

/// A format-1 zipped project holding one text item `folders` folders deep.
fn deep_outline_zip(root: &std::path::Path, folders: usize) -> String {
    let path = root.join("hostile.msk");
    let file = std::fs::File::create(&path).expect("zip");
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default();
    zip.start_file("MANUSKRIPT", options).expect("marker");
    zip.write_all(b"1").expect("marker");
    zip.start_file(outline_member(folders), options)
        .expect("member");
    zip.write_all(BOTTOM_ITEM.as_bytes()).expect("item");
    zip.finish().expect("finish");
    path.to_string_lossy().into_owned()
}

/// The longest path, in bytes, a folder project built here may put below its
/// temporary directory.
///
/// macOS refuses any path argument longer than 1,024 bytes (`MAXPATHLEN`), before
/// it looks at a single folder, and its temporary directory
/// (`/var/folders/…/T/.tmpXXXXXX/`) takes about 70 of them. The folders a walk
/// recurses through are what these tests measure, not how long their names are,
/// so the names are kept short enough to leave that directory room to spare on
/// every platform the suite runs on.
const FOLDER_PROJECT_PATH_BUDGET: usize = 640;

/// The member path of a text item sitting `folders` folders deep in a project
/// folder on disk: the top folder named as [`outline_member`] names it, so the
/// test can find it by title, and every folder below it one letter long.
fn compact_outline_member(folders: usize) -> String {
    let mut path = String::from("outline/0-Level_1/");
    for _ in 2..folders {
        path.push_str("a/");
    }
    path.push_str("0-Bottom.md");
    path
}

/// The same project as a folder on disk, `folders` folders deep, built from
/// [`compact_outline_member`] so its deepest path fits every platform's limit.
fn deep_outline_folder(root: &std::path::Path, folders: usize) -> String {
    let project = root.join("Hostile");
    let member = compact_outline_member(folders);
    let below_temp = format!("Hostile/{member}");
    assert!(
        below_temp.len() <= FOLDER_PROJECT_PATH_BUDGET,
        "a {folders}-folder project needs {} bytes below the temporary directory, more \
         than the {FOLDER_PROJECT_PATH_BUDGET} a macOS path leaves it",
        below_temp.len()
    );
    let item = project.join(&member);
    let Some(parent) = item.parent() else {
        panic!("{} has a parent", item.display());
    };
    std::fs::create_dir_all(parent).expect("the folders");
    std::fs::write(project.join("MANUSKRIPT"), "1").expect("marker");
    std::fs::write(&item, BOTTOM_ITEM).expect("item");
    project.to_string_lossy().into_owned()
}

fn folder_refusal_of(result: anyhow::Result<ImportSummary>) -> FoldersTooDeep {
    let error = match result {
        Ok(_) => panic!("a project nested past the ceiling must be refused"),
        Err(error) => error,
    };
    skrib_format::xml_depth::folders_too_deep(&error)
        .cloned()
        .unwrap_or_else(|| panic!("the refusal must be the typed one, got: {error:#}"))
}

/// The outline item at the bottom must arrive nested under every folder above
/// it, with its prose, which is what a walk stopping one level short would lose.
fn assert_the_bottom_arrived(out: &str, folders: usize) {
    let rows = rows_of(out);
    let top = row(&rows, "Level 1");
    let bottom = row(&rows, "Bottom");
    assert_eq!(
        bottom.item.indent - top.item.indent,
        (folders - 1) as i64,
        "the deepest item must arrive nested under all the others"
    );
    assert!(
        bottom
            .prose
            .values()
            .any(|text| text.contains("The words at the bottom.")),
        "the deepest item must keep its prose, got {:?}",
        bottom.prose
    );
}

/// A zip member's name can hold 64 KiB of folders, and the outline reader, the
/// mapper and the tree they build recurse once per level of it. At the ceiling
/// the project imports from a long operation's stack.
#[test]
fn an_outline_nested_to_the_ceiling_in_folders_imports_from_a_long_operation_stack() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = deep_outline_zip(dir.path(), MAX_XML_DEPTH);
    let out = output_in(dir.path());

    import_on_a_long_operation_stack(source, out.clone()).expect("at the ceiling it imports");

    assert_the_bottom_arrived(&out, MAX_XML_DEPTH);
}

#[test]
fn an_outline_one_folder_past_the_ceiling_is_refused_by_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = deep_outline_zip(dir.path(), MAX_XML_DEPTH + 1);

    let refused = folder_refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    assert_eq!(refused.part, outline_member(MAX_XML_DEPTH + 1));
    assert_eq!(refused.depth, MAX_XML_DEPTH + 1);
}

/// A project folder is walked before any member is read, one level per folder, on
/// the long operation's own stack. At the ceiling it imports. The walk used to
/// recurse, and on macOS its frames alone outgrew the long operation's 2 MiB before
/// it reached the ceiling.
#[test]
fn a_project_folder_nested_to_the_ceiling_imports_from_a_long_operation_stack() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = deep_outline_folder(dir.path(), MAX_XML_DEPTH);
    let out = output_in(dir.path());

    import_on_a_long_operation_stack(source, out.clone()).expect("at the ceiling it imports");

    assert_the_bottom_arrived(&out, MAX_XML_DEPTH);
}

/// One folder more and the walk stops where it is, naming the folder it would
/// have entered.
#[test]
fn a_project_folder_one_level_past_the_ceiling_is_refused_by_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = deep_outline_folder(dir.path(), MAX_XML_DEPTH + 1);

    let refused = folder_refusal_of(import_on_a_long_operation_stack(
        source,
        output_in(dir.path()),
    ));

    let member = compact_outline_member(MAX_XML_DEPTH + 1);
    let Some((folder, _item)) = member.rsplit_once('/') else {
        panic!("{member} sits in a folder");
    };
    assert_eq!(refused.part, folder);
    assert_eq!(refused.depth, MAX_XML_DEPTH + 1);
}

// ---------------------------------------------------------------------------
// Bodies nested past what a project can hold
// ---------------------------------------------------------------------------

/// A format-1 folder project holding one text item, `Scene`, whose body is
/// `body`, read as `declared` (`md`, or the pre-0.3.0 `html`).
fn project_with_body(root: &std::path::Path, declared: &str, body: &str) -> String {
    let project = root.join("Bodies");
    std::fs::create_dir_all(project.join("outline")).expect("outline folder");
    std::fs::write(project.join("MANUSKRIPT"), "1").expect("marker");
    std::fs::write(
        project.join("outline/0-Scene.md"),
        format!("title:          Scene\nID:             1\ntype:           {declared}\n\n\n{body}"),
    )
    .expect("item");
    project.to_string_lossy().into_owned()
}

/// The scene's stored prose, after importing `body` read as `declared` on
/// [`SMALL_STACK`], and the warnings the import raised.
fn import_body(declared: &str, body: &str) -> (String, Vec<String>) {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = project_with_body(dir.path(), declared, body);
    let out = output_in(dir.path());
    let summary = import_on_a_long_operation_stack(source, out.clone()).expect("it imports");
    let rows = rows_of(&out);
    let prose = row(&rows, "Scene")
        .prose
        .values()
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    (prose, summary.warnings)
}

/// Whether the import told the writer the scene's text was kept as its words,
/// naming the file it came from.
fn told_about_the_scene(warnings: &[String]) -> bool {
    warnings
        .iter()
        .any(|w| w.contains("0-Scene.md") && w.contains("nested deeper"))
}

/// A Markdown body is converted on the import's parser stack, but `text-document`
/// reads it on a thread of its own with 2 MiB of stack, which five thousand nested
/// blockquotes aborted. At the ceiling the body keeps its structure; past it the
/// words are kept as plain text, which the next load accepts, and the writer is
/// told which row.
#[test]
fn a_markdown_body_nested_past_the_ceiling_keeps_its_words_and_names_the_row() {
    let at = format!(
        "{}Deep words.",
        "> ".repeat(skrib_format::MAX_MARKDOWN_DEPTH)
    );
    let (prose, warnings) = import_body("md", &at);
    assert!(
        prose.contains(&"> ".repeat(skrib_format::MAX_MARKDOWN_DEPTH)),
        "{prose:.200}"
    );
    assert!(!told_about_the_scene(&warnings), "{warnings:?}");

    for levels in [skrib_format::MAX_MARKDOWN_DEPTH + 1, 5_000] {
        let past = format!("Opening words.\n\n{}Deep words.", "> ".repeat(levels));
        let (prose, warnings) = import_body("md", &past);
        assert!(prose.contains("Deep words."), "{levels}: {prose:.200}");
        assert!(skrib_format::djot_depth::check(&prose).is_ok(), "{levels}");
        assert!(told_about_the_scene(&warnings), "{levels}: {warnings:?}");
    }
}

/// An HTML body nested past the ceiling used to convert to nothing, without a
/// word: `text-document`'s HTML reader stops at its own depth and drops the rest.
/// Now its words are kept, and the writer is told which row.
#[test]
fn an_html_body_nested_past_the_ceiling_keeps_its_words_and_names_the_row() {
    let nested = |levels: usize| {
        format!(
            "<p>Opening words.</p>{}<p>Deep words.</p>{}",
            "<div>".repeat(levels),
            "</div>".repeat(levels)
        )
    };
    let (prose, warnings) = import_body("html", &nested(skrib_format::MAX_HTML_DEPTH - 1));
    assert!(prose.contains("Deep words."), "{prose:.200}");
    assert!(!told_about_the_scene(&warnings), "{warnings:?}");

    for levels in [5_000, 200, skrib_format::MAX_HTML_DEPTH + 1] {
        let (prose, warnings) = import_body("html", &nested(levels));
        assert!(prose.contains("Opening words."), "{levels}: {prose:.200}");
        assert!(prose.contains("Deep words."), "{levels}: {prose:.200}");
        assert!(told_about_the_scene(&warnings), "{levels}: {warnings:?}");
    }
}
