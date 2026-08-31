// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A hand-run harness for the editor round trip: export a real project to `.docx`/`.odt`
//! from the command line, so the file can be opened in a real Word or LibreOffice, marked
//! up by a real person, and read back — then read the marked-up file back against the project
//! it left from and print the merge the import wizard would draw.
//!
//! **Every test here is `#[ignore]`d, and that is the point.** They take paths from the
//! environment; they assert almost nothing, because what they produce is meant to be inspected
//! by a human or fed to the importer, not checked here. The automated proof that a round trip
//! works lives in `document_ingest`'s own suite, over a fixture this repo owns.
//!
//! ```bash
//! SKRIB_IN=~/Documents/tests/elise.skrib SKRIB_OUT=/tmp/elise.odt \
//!   cargo test -p skribisto-frontend --test round_trip_harness \
//!   -- --ignored --nocapture export_odt
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
    binder_item_commands, export_management_commands, import_management_commands, work_commands,
    work_management_commands,
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

/// The other half of the trip: read a marked-up file back against the project it left from and
/// print the merge the wizard's fourth step would draw.
///
/// ```bash
/// SKRIB_IN=~/Documents/tests/elise.skrib SKRIB_BACK=~/Documents/tests/elise-reply.odt \
///   cargo test -p skribisto-frontend --test round_trip_harness \
///   -- --ignored --nocapture merge_back
/// ```
///
/// This runs the same two computations the reconcile step runs — the destination's rows on one
/// side, the plan's on the other, through `reconcile::align` — with none of the wizard around
/// them. What a person cannot see by reading the panel is whether the *pairing* worked on a
/// real manuscript: a table of rows all reading "New" looks exactly like a table of rows all
/// reading "Identical" until you count them.
#[test]
#[ignore = "hand-run: needs SKRIB_IN and SKRIB_BACK"]
fn merge_back() {
    use import_management::{AnalyzeDocumentImportDto, DocumentImportRow, DocumentImportRows};
    use skribisto_model::CreateType;
    use skribisto_model::reconcile::{self, ExistingRow, IncomingRow, RowStatus};

    let (Some(input), Some(back)) = (env_or_skip("SKRIB_IN"), env_or_skip("SKRIB_BACK")) else {
        return;
    };
    assert!(
        std::path::Path::new(&input).exists(),
        "no project at {input}"
    );
    assert!(
        std::path::Path::new(&back).exists(),
        "no returning file at {back}"
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

    let work = frontend::commands::work_commands::get_all_work(&ctx).expect("get_all_work")[0].id;
    let binder =
        frontend::commands::binder_commands::get_all_binder(&ctx).expect("get_all_binder")[0].id;

    // ── what the project holds ──────────────────────────────────────────────
    //
    // Whole binder, exactly as the wizard's "the book itself" destination resolves to — and
    // read through the binder's own relationship, which is the **ordered** stream.
    // `get_all_binder_item` returns the store's rows in no order at all, and an aligner whose
    // whole job is preserving order cannot be fed an unordered stream: every row comes back
    // flagged as moved.
    use frontend::common::direct_access::binder::BinderRelationshipField;
    let ordered_ids = frontend::commands::binder_commands::get_binder_relationship(
        &ctx,
        &binder,
        &BinderRelationshipField::BinderItems,
    )
    .expect("binder items");
    let existing: Vec<(String, ExistingRow)> =
        binder_item_commands::get_binder_item_multi(&ctx, &ordered_ids)
            .expect("get_binder_item_multi")
            .into_iter()
            .flatten()
            .filter(|it| it.activated && !it.uid.is_nil())
            .filter_map(|it| {
                let create_type = CreateType::of(&it.role, &it.sub_role)?;
                Some((
                    it.title.clone(),
                    ExistingRow {
                        uid_tag: skribisto_model::round_trip::uid_tag(&it.uid),
                        title: it.title.clone(),
                        create_type,
                        digest: digest_of_djot(&item_prose(&ctx, it.id).unwrap_or_default()),
                    },
                ))
            })
            .collect();

    // ── what the file brings ────────────────────────────────────────────────
    let op_id = import_management_commands::analyze_document_import(
        &ctx,
        &AnalyzeDocumentImportDto {
            work_id: work,
            binder_id: binder,
            source_paths: vec![back.clone()],
            anchor_item_id: 0,
            drop_position: import_management::DropPosition::After,
        },
    )
    .expect("analyze dispatch");
    let completion = ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    assert!(
        completion.wait_for(&op_id, Some(Duration::from_secs(300))),
        "the analysis did not finish"
    );
    let plan = import_management_commands::get_analyze_document_import_result(&ctx, &op_id)
        .expect("analysis result")
        .expect("a finished analysis has a result");

    let DocumentImportRows::Found(rows) = plan.rows else {
        panic!("{back} produced no rows");
    };
    let incoming: Vec<IncomingRow> = rows
        .iter()
        .filter_map(|r| match r {
            DocumentImportRow::Found {
                kind,
                title,
                djot,
                source_uid_tag,
                source_digest,
                included,
                ..
            } if *included => Some(IncomingRow {
                source_uid_tag: (!source_uid_tag.is_empty()).then(|| source_uid_tag.clone()),
                source_digest: (!source_digest.is_empty()).then(|| source_digest.clone()),
                title: title.clone(),
                create_type: create_type_of(kind),
                digest: digest_of_djot(djot),
            }),
            _ => None,
        })
        .collect();

    // ── the merge ───────────────────────────────────────────────────────────
    let existing_rows: Vec<ExistingRow> = existing.iter().map(|(_, r)| r.clone()).collect();
    let merged = reconcile::align(&existing_rows, &incoming);

    println!(
        "{} existing row(s), {} incoming, {} merged",
        existing_rows.len(),
        incoming.len(),
        merged.len()
    );
    for m in &merged {
        let current = m
            .current
            .and_then(|j| existing.get(j))
            .map(|(t, _)| t.as_str())
            .unwrap_or("—");
        let inc = m
            .incoming
            .and_then(|i| incoming.get(i))
            .map(|r| r.title.as_str())
            .unwrap_or("—");
        println!(
            "  {:<40} | {:<40} | {:?}{}",
            truncate(current),
            truncate(inc),
            m.status,
            if m.moved { " (moved)" } else { "" }
        );
    }

    let paired = merged
        .iter()
        .filter(|m| m.current.is_some() && m.incoming.is_some())
        .count();
    println!("{paired} row(s) came home; the rest are additions or absences");

    // The one assertion worth making, and the one the freeze hid: a file this project
    // exported must pair with it. Every row reading `New` means the marks did not survive,
    // did not match, or were never written — and the wizard would silently import a second
    // copy of the book.
    assert!(
        paired > 0,
        "nothing in {back} paired with {input} — the returning file brought no usable identity"
    );
    assert!(
        merged.iter().any(|m| m.status != RowStatus::New),
        "every row is new"
    );
}

fn truncate(s: &str) -> String {
    if s.chars().count() <= 38 {
        return s.to_string();
    }
    s.chars().take(37).collect::<String>() + "…"
}

/// The prose stored on one binder item, the same way the import wizard reads it.
fn item_prose(ctx: &AppContext, item_id: u64) -> Option<String> {
    use frontend::commands::content_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;

    let ids = binder_item_commands::get_binder_item_relationship(
        ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .ok()?;
    content_commands::get_content_multi(ctx, &ids)
        .ok()?
        .into_iter()
        .flatten()
        .find(|c| {
            matches!(
                c.role,
                ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText
            )
        })
        .map(|c| c.data)
}

fn digest_of_djot(djot: &str) -> String {
    let plain = skrib_format::djot_plain_text(djot)
        .map(|(t, _)| t)
        .unwrap_or_default();
    skribisto_model::round_trip::digest(&plain)
}

fn create_type_of(kind: &import_management::ImportRowKind) -> skribisto_model::CreateType {
    use import_management::ImportRowKind as K;
    use skribisto_model::CreateType as C;
    match kind {
        K::Book => C::Book,
        K::Part => C::Part,
        K::Chapter => C::Chapter,
        K::Scene => C::Scene,
        K::Note => C::Note,
        K::NoteFolder => C::NoteFolder,
        K::Folder => C::Folder,
        K::Paratext => C::Paratext,
        K::ParatextFolder => C::ParatextFolder,
        K::EndOfBook => C::EndOfBook,
    }
}
