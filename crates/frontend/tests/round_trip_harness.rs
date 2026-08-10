// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A hand-run harness for the editor round trip: export a real project to `.docx`/`.odt`
//! from the command line, so the file can be opened in a real Word or LibreOffice, marked
//! up by a real person, and read back.
//!
//! **Both tests here are `#[ignore]`d, and that is the point.** They take a project path and
//! an output path from the environment; they assert almost nothing, because what they produce
//! is meant to be inspected by a human or fed to the importer, not checked here. The automated
//! proof that a round trip works lives in `document_ingest`'s own suite, over a fixture this
//! repo owns.
//!
//! ```bash
//! SKRIB_IN=~/Documents/tests/elise.skrib SKRIB_OUT=/tmp/elise.odt \
//!   cargo test -p frontend --test round_trip_harness -- --ignored --nocapture export_odt
//! ```
//!
//! `SKRIB_STYLE` optionally names a JSON export preset file; the default below keeps comments
//! and round-trip marks on, which is what makes the produced file worth marking up.
//!
//! Why this exists rather than driving the app: exporting through the UI means opening a
//! panel, choosing a format and a scope, and steering a file dialog. That is a fine thing for
//! a person to do in ten seconds and an expensive thing to automate; this reaches the same
//! `export_work` use case the panel calls, with none of it.

use std::time::Duration;

use export_management::{ExportFormat, ExportScopeKind, ExportWorkDto};
use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, export_management_commands, work_commands, work_management_commands,
};
use work_management::LoadWorkDto;

/// Comments and marks explicitly on. Both default to on, and both are named here anyway: the
/// whole purpose of a file this harness produces is that it can be marked up and read back,
/// and a silent default is not something to rest that on.
const ROUND_TRIP_STYLE: &str = r#"{
    "id": "round-trip",
    "name": "Round trip",
    "font_family": "Literata",
    "font_size_pt": 12.0,
    "include_comments": true,
    "include_round_trip_marks": true,
    "include_notes": false,
    "include_synopses": false
}"#;

fn env_or_skip(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(v) if !v.trim().is_empty() => Some(v),
        _ => {
            eprintln!("skipping: set {key} to run this harness");
            None
        }
    }
}

fn export(format: ExportFormat, default_ext: &str) {
    let Some(input) = env_or_skip("SKRIB_IN") else {
        return;
    };
    let output = std::env::var("SKRIB_OUT").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join(format!("skrib-round-trip.{default_ext}"))
            .to_string_lossy()
            .into_owned()
    });
    let style = std::env::var("SKRIB_STYLE")
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .unwrap_or_else(|| ROUND_TRIP_STYLE.to_string());

    assert!(
        std::path::Path::new(&input).exists(),
        "no project at {input}"
    );

    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: input.clone(),
        },
    )
    .expect("load_work");

    let work_id = work_commands::get_all_work(&ctx).expect("get_all_work")[0].id as i64;
    // Every item, as a Custom scope: robust to whatever shape the project has, and the same
    // thing "Export Book" resolves to for a single-book manuscript.
    let ids: Vec<i64> = binder_item_commands::get_all_binder_item(&ctx)
        .expect("get_all_binder_item")
        .into_iter()
        .map(|it| it.id as i64)
        .collect();
    assert!(!ids.is_empty(), "{input} has no binder items");

    let carries_marks = matches!(format, ExportFormat::Docx | ExportFormat::Odt);
    let dto = ExportWorkDto {
        media_dir: String::new(),
        work_id,
        output_path: output.clone(),
        format,
        scope_kind: ExportScopeKind::Custom,
        preset_json: style,
        binder_item_ids: ids,
    };

    let op_id = export_management_commands::export_work(&ctx, &dto).expect("export_work dispatch");
    let completion = ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    assert!(
        completion.wait_for(&op_id, Some(Duration::from_secs(300))),
        "the export did not finish"
    );

    let result = export_management_commands::get_export_work_result(&ctx, &op_id)
        .expect("export result")
        .expect("a finished export has a result");

    let bytes = std::fs::read(&output).expect("the export file exists");
    println!("wrote {output} ({} bytes)", bytes.len());
    println!(
        "  {} item(s), {} comment(s) written, {} dropped",
        result.exported_count, result.comments_written, result.comments_orphaned
    );

    // The one thing worth asserting: the identity actually reached the file. Everything else
    // about this run is for a human to look at.
    //
    // Only for the two formats that carry marks at all — `export_djot` below writes a plain
    // text file, which is here to *read* what the compiler produced before either container
    // writer touches it, and has nothing to count.
    if carries_marks {
        let marks = count_marks(&bytes, &output);
        println!("  {marks} round-trip mark(s)");
        assert!(
            marks > 0,
            "no round-trip marks in {output} — a returning file could not be matched back"
        );
    }
}

/// How many of our bookmark names are in the packed container.
fn count_marks(bytes: &[u8], path: &str) -> usize {
    let part = if path.ends_with(".docx") {
        "word/document.xml"
    } else {
        "content.xml"
    };
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("a packed container");
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut zip.by_name(part).expect("body part"), &mut xml)
        .expect("body part is utf-8");
    xml.matches(skribisto_model::round_trip::ROW_PREFIX).count()
        + xml
            .matches(skribisto_model::round_trip::COMMENT_PREFIX)
            .count()
}

#[test]
#[ignore = "hand-run: needs SKRIB_IN"]
fn export_odt() {
    export(ExportFormat::Odt, "odt");
}

#[test]
#[ignore = "hand-run: needs SKRIB_IN"]
fn export_docx() {
    export(ExportFormat::Docx, "docx");
}

#[test]
#[ignore = "hand-run: needs SKRIB_IN"]
fn export_djot() {
    export(ExportFormat::Djot, "djot");
}
