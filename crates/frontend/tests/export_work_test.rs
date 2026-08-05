// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End-to-end test for the `export_work` long operation: load a real project into the
//! store, export the whole thing through the frozen-read → gather → compile → render path,
//! and assert a non-empty file lands on disk.

use std::time::Duration;

use export_management::{ExportFormat, ExportScopeKind, ExportWorkDto};
use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, export_management_commands, work_commands, work_management_commands,
};
use work_management::LoadWorkDto;

fn fixture_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// A minimal export style — the rest of the fields default via serde. `include_notes` is on
/// so the whole project renders regardless of how the fixture splits prose vs. notes.
const PRESET_JSON: &str = r#"{
    "id": "test",
    "name": "Test",
    "font_family": "Serif",
    "font_size_pt": 12.0,
    "include_notes": true
}"#;

#[test]
fn export_work_writes_the_whole_project_to_html() {
    let ctx = AppContext::new();
    let fixture = fixture_path();
    assert!(
        std::path::Path::new(&fixture).exists(),
        "fixture missing: {fixture}"
    );

    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: fixture,
        },
    )
    .expect("load_work should succeed");

    let work_id = work_commands::get_all_work(&ctx).expect("get_all_work")[0].id as i64;
    // Export everything: a Custom scope of every item id (robust to the fixture's structure).
    let ids: Vec<i64> = binder_item_commands::get_all_binder_item(&ctx)
        .expect("get_all_binder_item")
        .into_iter()
        .map(|it| it.id as i64)
        .collect();
    assert!(!ids.is_empty(), "the fixture should have binder items");

    let out_path =
        std::env::temp_dir().join(format!("skrib-export-e2e-{}.html", std::process::id()));
    let dto = ExportWorkDto {
        media_dir: String::new(),
        work_id,
        output_path: out_path.to_string_lossy().into_owned(),
        format: ExportFormat::Html,
        scope_kind: ExportScopeKind::Custom,
        preset_json: PRESET_JSON.to_string(),
        binder_item_ids: ids,
    };

    let op_id = export_management_commands::export_work(&ctx, &dto).expect("export_work dispatch");

    // Take the completion signal and release the manager lock before blocking — waiting
    // while holding it would stall every other operation query for the export's whole
    // duration (see the qleany 1.9.0 migration guide's long-operation section).
    let completion = ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    let finished = completion.wait_for(&op_id, Some(Duration::from_secs(10)));
    assert!(finished, "export_work should complete within the timeout");

    let result = export_management_commands::get_export_work_result(&ctx, &op_id)
        .expect("export result")
        .expect("a finished export has a result");

    assert!(
        result.exported_count > 0,
        "some items should have been exported"
    );
    assert_eq!(result.output_path, dto.output_path);
    let bytes = std::fs::read(&out_path).expect("the export file should exist");
    assert!(!bytes.is_empty(), "the export file should be non-empty");
    let html = String::from_utf8_lossy(&bytes);
    assert!(
        html.contains("<body>"),
        "should be an HTML document: {}",
        &html[..html.len().min(200)]
    );

    let _ = std::fs::remove_file(&out_path);
}

/// A manuscript style, whole-book: the combination the DOCX assertions below are about.
const SHUNN_JSON: &str = r#"{
    "id": "manuscript-shunn",
    "name": "Standard Manuscript (Shunn)",
    "font_family": "Times New Roman",
    "font_size_pt": 12.0,
    "book_title_page": true,
    "title_page_word_count": true
}"#;

/// The end of the chain, on a real file: load a project, export it as DOCX, unzip the
/// result and read the XML.
///
/// Everything below this point is only observable in the written file — a compiled
/// `TextDocument` cannot tell you whether Word will find a definition for `Heading1`, and
/// the export panel's old live preview could not either. That is precisely why these
/// three things were broken without anything failing.
#[test]
fn a_docx_export_paginates_and_defines_the_styles_it_uses() {
    let ctx = AppContext::new();
    let fixture = fixture_path();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: fixture,
        },
    )
    .expect("load_work should succeed");

    let work_id = work_commands::get_all_work(&ctx).expect("get_all_work")[0].id as i64;
    let ids: Vec<i64> = binder_item_commands::get_all_binder_item(&ctx)
        .expect("get_all_binder_item")
        .into_iter()
        .map(|it| it.id as i64)
        .collect();

    let out_path =
        std::env::temp_dir().join(format!("skrib-export-docx-{}.docx", std::process::id()));
    let dto = ExportWorkDto {
        media_dir: String::new(),
        work_id,
        output_path: out_path.to_string_lossy().into_owned(),
        format: ExportFormat::Docx,
        scope_kind: ExportScopeKind::Custom,
        preset_json: SHUNN_JSON.to_string(),
        binder_item_ids: ids,
    };
    let op_id = export_management_commands::export_work(&ctx, &dto).expect("export_work dispatch");
    let completion = ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    assert!(
        completion.wait_for(&op_id, Some(Duration::from_secs(30))),
        "the DOCX export should complete within the timeout"
    );

    let bytes = std::fs::read(&out_path).expect("the export file should exist");
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("a .docx is a zip");
    let read = |zip: &mut zip::ZipArchive<std::io::Cursor<Vec<u8>>>, name: &str| -> String {
        use std::io::Read;
        let mut part = zip
            .by_name(name)
            .unwrap_or_else(|_| panic!("{name} missing"));
        let mut s = String::new();
        part.read_to_string(&mut s).expect("utf-8");
        s
    };
    let document = read(&mut zip, "word/document.xml");
    let styles = read(&mut zip, "word/styles.xml");

    // 1. Pages. There was no page-break concept anywhere in the pipeline, so a book
    //    arrived as one unbroken column.
    assert!(
        document.contains("<w:pageBreakBefore"),
        "the manuscript must paginate"
    );

    // 2. The styles the document references must exist in it. Referencing an undefined
    //    style id is legal OOXML and silently resolves to the reader's own, which is how a
    //    title asking to be a title arrived as whatever Word had.
    for level in 1..=3 {
        let id = format!("w:styleId=\"Heading{level}\"");
        if document.contains(&format!("w:val=\"Heading{level}\"")) {
            assert!(
                styles.contains(&id),
                "Heading{level} is referenced by the document but not defined in styles.xml"
            );
        }
    }

    // 3. The title page. Centred, and carrying the rounded word count an editor reads
    //    first — neither of which the exporter emitted before.
    assert!(
        document.contains("<w:jc w:val=\"center\""),
        "the title page must be centred"
    );
    // The fixture is a French project, and the count is generated furniture, so it is
    // localized — asserting the English wording would only ever have tested the fixture's
    // language. `<w:jc w:val="right"/>` above it is the manuscript convention.
    assert!(
        document.contains("mots</w:t>") || document.contains("words</w:t>"),
        "a submission title page carries its word count: {}",
        &document[..document.len().min(2500)]
    );
    assert!(
        document.contains("<w:jc w:val=\"right\""),
        "the word count sits at the top right"
    );

    // 4. The title's drop down the page. A heading never reaches the body-paragraph
    //    styling, so its own space-above used to be dropped and the title page opened
    //    flush at the top.
    assert!(
        document.contains("<w:spacing w:before="),
        "the title must be dropped down the page"
    );

    let _ = std::fs::remove_file(&out_path);
}
