//! Integration tests for the Full Chapter view's new backend use cases —
//! `merge_two_scenes` and `split_scene` — against a real in-memory store.

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, SplitSceneDto};
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    handling_app_lifecycle_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{
    CreateBinderDto, CreateBinderItemDto, CreateContentDto, CreateWorkDto,
};

/// A fresh store with one Work + one Binder; returns (ctx, binder_id).
fn setup() -> (AppContext, u64) {
    let ctx = AppContext::new();
    let root = handling_app_lifecycle_commands::initialize_app(&ctx)
        .unwrap()
        .root_id;
    let work = work_commands::create_work(
        &ctx,
        None,
        &CreateWorkDto {
            title: "Test".into(),
            ..Default::default()
        },
        root,
        -1,
    )
    .unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "Manuscript".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        -1,
    )
    .unwrap();
    (ctx, binder.id)
}

/// Append a `Scene` item with `SceneText = text` to the binder; return its id.
fn make_scene(ctx: &AppContext, binder: u64, title: &str, text: &str) -> u64 {
    let item = binder_item_commands::create_binder_item(
        ctx,
        None,
        &CreateBinderItemDto {
            title: title.into(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            is_printable: true,
            ..Default::default()
        },
        binder,
        -1,
    )
    .unwrap();
    content_commands::create_content(
        ctx,
        None,
        &CreateContentDto {
            activated: true,
            role: ContentRole::SceneText,
            data: text.into(),
            ..Default::default()
        },
        item.id,
        -1,
    )
    .unwrap();
    item.id
}

/// The item's `SceneText` data.
fn scene_text(ctx: &AppContext, item: u64) -> String {
    let ids = binder_item_commands::get_binder_item_relationship(
        ctx,
        &item,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    content_commands::get_content_multi(ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| c.role == ContentRole::SceneText)
        .map(|c| c.data)
        .unwrap_or_default()
}

fn binder_order(ctx: &AppContext, binder: u64) -> Vec<u64> {
    binder_commands::get_binder_relationship(ctx, &binder, &BinderRelationshipField::BinderItems)
        .unwrap()
}

#[test]
fn merge_two_scenes_concats_text_and_trashes_source() {
    let (ctx, binder) = setup();
    let a = make_scene(&ctx, binder, "A", "Alpha");
    let b = make_scene(&ctx, binder, "B", "Beta");

    binder_item_management_commands::merge_two_scenes(
        &ctx,
        None,
        &MergeTwoScenesDto {
            target_id: a,
            source_id: b,
        },
    )
    .unwrap();

    // A absorbed B's text after a blank line.
    assert_eq!(scene_text(&ctx, a), "Alpha\n\nBeta");
    // B is trashed (deactivated, so excluded from the flat stream).
    let b_dto = binder_item_commands::get_binder_item(&ctx, &b)
        .unwrap()
        .unwrap();
    assert!(!b_dto.activated, "merged-away scene should be trashed");
}

#[test]
fn split_scene_splits_at_offset_into_two() {
    let (ctx, binder) = setup();
    let a = make_scene(&ctx, binder, "A", "HelloWorld");

    binder_item_management_commands::split_scene(
        &ctx,
        None,
        &SplitSceneDto {
            source_id: a,
            before_text: "Hello".into(),
            after_text: "World".into(),
            new_title: "New Scene".into(),
        },
    )
    .unwrap();

    // Source keeps the before-text; a new scene follows it with the after-text.
    assert_eq!(scene_text(&ctx, a), "Hello");
    let order = binder_order(&ctx, binder);
    let pos = order.iter().position(|&x| x == a).unwrap();
    let new_id = order[pos + 1];
    assert_eq!(scene_text(&ctx, new_id), "World");
}
