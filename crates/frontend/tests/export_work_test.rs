//! End-to-end test for the `export_work` long operation: load a real project into the
//! store, export the whole thing through the frozen-read → gather → compile → render path,
//! and assert a non-empty file lands on disk.

use std::time::Duration;

use export_management::{ExportFormat, ExportScopeKind, ExportWorkDto};
use frontend::AppContext;
use frontend::commands::{binder_item_commands, export_management_commands, work_commands, work_management_commands};
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
    assert!(std::path::Path::new(&fixture).exists(), "fixture missing: {fixture}");

    work_management_commands::load_work(&ctx, &LoadWorkDto { file_name: fixture })
        .expect("load_work should succeed");

    let work_id = work_commands::get_all_work(&ctx).expect("get_all_work")[0].id as i64;
    // Export everything: a Custom scope of every item id (robust to the fixture's structure).
    let ids: Vec<i64> = binder_item_commands::get_all_binder_item(&ctx)
        .expect("get_all_binder_item")
        .into_iter()
        .map(|it| it.id as i64)
        .collect();
    assert!(!ids.is_empty(), "the fixture should have binder items");

    let out_path = std::env::temp_dir().join(format!("skrib-export-e2e-{}.html", std::process::id()));
    let dto = ExportWorkDto {
        work_id,
        output_path: out_path.to_string_lossy().into_owned(),
        format: ExportFormat::Html,
        scope_kind: ExportScopeKind::Custom,
        preset_json: PRESET_JSON.to_string(),
        binder_item_ids: ids,
    };

    let op_id = export_management_commands::export_work(&ctx, &dto).expect("export_work dispatch");

    // Poll the long op to completion (it runs on its own thread), with a timeout.
    let mut result = None;
    for _ in 0..500 {
        if let Some(r) =
            export_management_commands::get_export_work_result(&ctx, &op_id).expect("export result")
        {
            result = Some(r);
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let result = result.expect("export_work should complete within the timeout");

    assert!(result.exported_count > 0, "some items should have been exported");
    assert_eq!(result.output_path, dto.output_path);
    let bytes = std::fs::read(&out_path).expect("the export file should exist");
    assert!(!bytes.is_empty(), "the export file should be non-empty");
    let html = String::from_utf8_lossy(&bytes);
    assert!(html.contains("<body>"), "should be an HTML document: {}", &html[..html.len().min(200)]);

    let _ = std::fs::remove_file(&out_path);
}
