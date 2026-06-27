//! Integration test for the ported `load_work` use case.
//!
//! Loads the bundled legacy fixture (`tbl_tree` schema, db version 1.8) and
//! asserts the migration mapping populated the in-memory store.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, content_commands, trash_info_commands, work_commands,
    work_management_commands,
};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use skribisto_model::validate_item;
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
    assert_eq!(
        works.len(),
        1,
        "expected exactly one Work, got {}",
        works.len()
    );

    // Binders: the manuscript folder and the "Notes" note_folder. There is NO
    // Trash binder — trashed items stay in place (activated=false), so the legacy
    // Trash folder is not reproduced.
    let binders = binder_commands::get_all_binder(&ctx).expect("get_all_binder");
    let names: Vec<&str> = binders.iter().map(|b| b.name.as_str()).collect();
    assert!(
        names.contains(&"Notes"),
        "expected a 'Notes' binder, found: {names:?}"
    );
    assert!(
        !names.contains(&"Trash"),
        "there should be no 'Trash' binder, found: {names:?}"
    );
    assert!(
        binders.len() >= 2,
        "expected at least 2 binders, got {names:?}"
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

    // The fixture is v1.8 (Qt HTML content). The 1.9→2.0 step must have converted
    // it to Markdown via text-document — no Qt rich-text HTML should survive.
    for c in &contents {
        assert!(
            !c.data.contains("<!DOCTYPE")
                && !c.data.contains("qrichtext")
                && !c.data.contains("<p "),
            "content still looks like Qt HTML (HTML→Markdown did not run): {:?}",
            &c.data[..c.data.len().min(80)]
        );
    }
    assert!(
        contents.iter().any(|c| !c.data.trim().is_empty()),
        "expected at least one non-empty content row"
    );

    // Trashed entities are invisible (activated == false) and indexed by TrashInfo
    // so a trash view can list them. The fixture has trashed rows, so there must be
    // a one-to-one correspondence between activated=false items and TrashInfo rows.
    let trashed_item_ids: std::collections::HashSet<_> = items
        .iter()
        .filter(|i| !i.activated)
        .map(|i| i.id)
        .collect();
    assert!(
        !trashed_item_ids.is_empty(),
        "fixture contains trashed rows; expected some activated=false items"
    );
    let trash_infos = trash_info_commands::get_all_trash_info(&ctx).expect("get_all_trash_info");
    let indexed_item_ids: std::collections::HashSet<_> = trash_infos
        .iter()
        .filter_map(|t| t.trashed_binder_item)
        .collect();
    assert_eq!(
        indexed_item_ids, trashed_item_ids,
        "every trashed BinderItem must have exactly one TrashInfo, and vice versa"
    );

    // Every migrated item must satisfy the writing-model constraint matrix:
    // a valid (role, sub_role) pair carrying only permitted content roles.
    for item in &items {
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &ctx,
            &item.id,
            &BinderItemRelationshipField::Contents,
        )
        .expect("binder_item Contents relationship");
        let content_roles: Vec<_> = content_commands::get_content_multi(&ctx, &content_ids)
            .expect("get_content_multi")
            .into_iter()
            .flatten()
            .map(|c| c.role)
            .collect();
        validate_item(&item.role, &item.sub_role, &content_roles).unwrap_or_else(|e| {
            panic!(
                "migrated item {:?} violates the writing model: {e}",
                item.title
            )
        });
    }
}
