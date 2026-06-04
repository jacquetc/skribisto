//! Integration test for the ported `load_work` use case.
//!
//! Loads the bundled legacy fixture (`tbl_tree` schema, db version 1.8) and
//! asserts the migration mapping populated the in-memory store.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, content_commands, work_commands,
    work_management_commands,
};
use work_management::LoadWorkDto;

fn fixture_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

#[test]
fn load_legacy_fixture_populates_store() {
    let ctx = AppContext::new();
    let fixture = fixture_path();
    assert!(
        std::path::Path::new(&fixture).exists(),
        "fixture missing: {fixture}"
    );

    work_management_commands::load_work(&ctx, &LoadWorkDto { file_name: fixture })
        .expect("load_work should succeed on the legacy fixture");

    // Exactly one Work imported.
    let works = work_commands::get_all_work(&ctx).expect("get_all_work");
    assert_eq!(works.len(), 1, "expected exactly one Work, got {}", works.len());

    // Two binders: the "Writings" folder and the "Notes" note_folder.
    let binders = binder_commands::get_all_binder(&ctx).expect("get_all_binder");
    assert_eq!(
        binders.len(),
        2,
        "expected two binders (Writings + Notes), got {}",
        binders.len()
    );
    assert!(
        binders.iter().any(|b| b.name == "Notes"),
        "expected a 'Notes' binder, found: {:?}",
        binders.iter().map(|b| &b.name).collect::<Vec<_>>()
    );

    // Items and content rows were created (separators dropped, so just a lower bound).
    let items = binder_item_commands::get_all_binder_item(&ctx).expect("get_all_binder_item");
    assert!(
        items.len() >= 10,
        "expected several binder items, got {}",
        items.len()
    );

    let contents = content_commands::get_all_content(&ctx).expect("get_all_content");
    assert!(!contents.is_empty(), "expected some content rows");
}
