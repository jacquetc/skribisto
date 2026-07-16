// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for the binder-tree action use cases: move_items,
//! duplicate, and the four trash_management use cases — plus their undo/redo.
//!
//! Fixtures are built synthetically via the direct-access create commands (no
//! `.skrib` file needed). The fixture is created on a dedicated `setup` undo
//! stack; each action runs on its own fresh stack so `undo`/`redo` touch only
//! the action under test.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, binder_tag_commands,
    content_commands, root_commands, system_commands, trash_info_commands,
    trash_management_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    BinderItemRelationshipDto, BinderRelationshipDto, CreateBinderDto, CreateBinderItemDto,
    CreateBinderTagDto, CreateContentDto, CreateRootDto, CreateSystemDto, CreateWorkDto,
    WorkRelationshipDto,
};

use binder_item_management::{
    DuplicateDto, MergeTwoScenesDto, MoveDto, MovePlace, PromoteDto, SplitSceneDto,
};
use skribisto_model::PromoteTarget;
use trash_management::{RestoreItemsDto, TrashBinderDto, TrashBinderItemsDto};

// ───────────────────────────── fixture helpers ─────────────────────────────

struct Fixture {
    ctx: AppContext,
    setup: u64,
    work: EntityId,
    binder1: EntityId,
    binder2: EntityId,
    // binder1 items, in order:  a, a1, a2, b, b1, c  (indents 0,1,1,0,1,0)
    a: EntityId,
    a1: EntityId,
    a2: EntityId,
    b: EntityId,
    b1: EntityId,
    c: EntityId,
}

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

fn mk_item(
    ctx: &AppContext,
    stack: u64,
    title: &str,
    indent: i64,
    role: BinderItemRole,
) -> EntityId {
    let dto = CreateBinderItemDto {
        created_at: now(),
        updated_at: now(),
        title: title.to_string(),
        role,
        sub_role: BinderItemSubRole::Text,
        activated: true,
        is_exportable: true,
        indent,
        ..Default::default()
    };
    binder_item_commands::create_orphan_binder_item(ctx, Some(stack), &dto)
        .expect("create item")
        .id
}

fn wire_binder(ctx: &AppContext, stack: u64, binder: EntityId, items: &[EntityId]) {
    binder_commands::set_binder_relationship(
        ctx,
        Some(stack),
        &BinderRelationshipDto {
            id: binder,
            field: BinderRelationshipField::BinderItems,
            right_ids: items.to_vec(),
        },
    )
    .expect("wire binder");
}

fn order(ctx: &AppContext, binder: EntityId) -> Vec<EntityId> {
    binder_commands::get_binder_relationship(ctx, &binder, &BinderRelationshipField::BinderItems)
        .expect("order")
}

fn item(ctx: &AppContext, id: EntityId) -> frontend::direct_access::BinderItemDto {
    binder_item_commands::get_binder_item(ctx, &id)
        .expect("get item")
        .expect("item exists")
}

fn indent(ctx: &AppContext, id: EntityId) -> i64 {
    item(ctx, id).indent
}

fn make_fixture() -> Fixture {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);

    let system = system_commands::create_orphan_system(
        &ctx,
        &CreateSystemDto {
            created_at: now(),
            updated_at: now(),
            ..Default::default()
        },
    )
    .expect("create system")
    .id;

    let work = work_commands::create_orphan_work(
        &ctx,
        Some(setup),
        &CreateWorkDto {
            created_at: now(),
            updated_at: now(),
            title: "Test".into(),
            ..Default::default()
        },
    )
    .expect("create work")
    .id;

    let binder1 = binder_commands::create_orphan_binder(
        &ctx,
        Some(setup),
        &CreateBinderDto {
            created_at: now(),
            updated_at: now(),
            name: "Manuscript".into(),
            activated: true,
            binder_items: vec![],
        },
    )
    .expect("create binder1")
    .id;
    let binder2 = binder_commands::create_orphan_binder(
        &ctx,
        Some(setup),
        &CreateBinderDto {
            created_at: now(),
            updated_at: now(),
            name: "Notes".into(),
            activated: true,
            binder_items: vec![],
        },
    )
    .expect("create binder2")
    .id;

    let a = mk_item(&ctx, setup, "A", 0, BinderItemRole::Folder);
    let a1 = mk_item(&ctx, setup, "A1", 1, BinderItemRole::Item);
    let a2 = mk_item(&ctx, setup, "A2", 1, BinderItemRole::Item);
    let b = mk_item(&ctx, setup, "B", 0, BinderItemRole::Folder);
    let b1 = mk_item(&ctx, setup, "B1", 1, BinderItemRole::Item);
    let c = mk_item(&ctx, setup, "C", 0, BinderItemRole::Item);

    wire_binder(&ctx, setup, binder1, &[a, a1, a2, b, b1, c]);
    work_commands::set_work_relationship(
        &ctx,
        Some(setup),
        &WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::Binders,
            right_ids: vec![binder1, binder2],
        },
    )
    .expect("wire work");

    // A Root owning the System + Work, matching what initialize_app seeds in the
    // real app (empty_trash's undo restores the Root-scoped subtree).
    root_commands::create_orphan_root(
        &ctx,
        &CreateRootDto {
            created_at: now(),
            updated_at: now(),
            system,
            works: vec![work],
        },
    )
    .expect("create root");

    Fixture {
        ctx,
        setup,
        work,
        binder1,
        binder2,
        a,
        a1,
        a2,
        b,
        b1,
        c,
    }
}

/// Link a new `Content` row onto `item_id`, **keeping** the rows already there —
/// an item legitimately carries several roles at once (prose + synopsis), so this
/// appends rather than replacing the relationship.
fn add_content(fx: &Fixture, item_id: EntityId, role: ContentRole, data: &str) -> EntityId {
    let cid = content_commands::create_orphan_content(
        &fx.ctx,
        Some(fx.setup),
        &CreateContentDto {
            created_at: now(),
            updated_at: now(),
            activated: true,
            role,
            data: data.to_string(),
        },
    )
    .expect("create content")
    .id;
    let mut right_ids = binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .expect("contents");
    right_ids.push(cid);
    binder_item_commands::set_binder_item_relationship(
        &fx.ctx,
        Some(fx.setup),
        &frontend::direct_access::BinderItemRelationshipDto {
            id: item_id,
            field: BinderItemRelationshipField::Contents,
            right_ids,
        },
    )
    .expect("set contents");
    cid
}

// ───────────────────────────────── move ─────────────────────────────────

#[test]
fn move_before_sibling() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.c],
            target_id: Some(fx.a),
            target_is_binder: false,
            move_place: MovePlace::Before,
        },
    )
    .expect("move");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.c, fx.a, fx.a1, fx.a2, fx.b, fx.b1]
    );
    assert_eq!(indent(&fx.ctx, fx.c), 0);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]
    );
    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.c, fx.a, fx.a1, fx.a2, fx.b, fx.b1]
    );
}

#[test]
fn move_after_folder_lands_past_its_subtree() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.c],
            target_id: Some(fx.a),
            target_is_binder: false,
            move_place: MovePlace::After,
        },
    )
    .expect("move");
    // After folder A (subtree A,A1,A2) → before B.
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.c, fx.b, fx.b1]
    );
    assert_eq!(indent(&fx.ctx, fx.c), 0);
}

#[test]
fn move_into_folder_reindents() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.c],
            target_id: Some(fx.a),
            target_is_binder: false,
            move_place: MovePlace::Into,
        },
    )
    .expect("move");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.c, fx.b, fx.b1]
    );
    assert_eq!(indent(&fx.ctx, fx.c), 1, "C becomes a child of folder A");
}

#[test]
fn move_into_leaf_redirects_to_after() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.c],
            target_id: Some(fx.a1), // a leaf
            target_is_binder: false,
            move_place: MovePlace::Into,
        },
    )
    .expect("move");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.c, fx.a2, fx.b, fx.b1]
    );
    assert_eq!(
        indent(&fx.ctx, fx.c),
        1,
        "Into a leaf == After it (sibling indent)"
    );
}

#[test]
fn move_into_folder_propagates_indent_delta_to_subtree() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    // Move folder A (with A1,A2) Into folder B.
    binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.a],
            target_id: Some(fx.b),
            target_is_binder: false,
            move_place: MovePlace::Into,
        },
    )
    .expect("move");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.b, fx.b1, fx.a, fx.a1, fx.a2, fx.c]
    );
    assert_eq!(indent(&fx.ctx, fx.a), 1);
    assert_eq!(indent(&fx.ctx, fx.a1), 2);
    assert_eq!(indent(&fx.ctx, fx.a2), 2);
}

#[test]
fn move_folder_subtree_cross_binder_into_empty_binder() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.a],
            target_id: Some(fx.binder2),
            target_is_binder: true,
            move_place: MovePlace::Into,
        },
    )
    .expect("move");
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.b, fx.b1, fx.c]);
    assert_eq!(order(&fx.ctx, fx.binder2), vec![fx.a, fx.a1, fx.a2]);
    // Indents preserved (delta 0).
    assert_eq!(indent(&fx.ctx, fx.a), 0);
    assert_eq!(indent(&fx.ctx, fx.a1), 1);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]
    );
    assert!(order(&fx.ctx, fx.binder2).is_empty());
}

#[test]
fn move_into_own_subtree_is_rejected() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let err = binder_item_management_commands::move_items(
        &fx.ctx,
        Some(stack),
        &MoveDto {
            item_ids: vec![fx.a],
            target_id: Some(fx.a1), // a1 is inside a's subtree
            target_is_binder: false,
            move_place: MovePlace::Into,
        },
    );
    assert!(err.is_err(), "moving a subtree into itself must fail");
    // Tree unchanged.
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]
    );
}

// ─────────────────────────────── duplicate ───────────────────────────────

#[test]
fn duplicate_folder_subtree_clones_items_and_content() {
    let fx = make_fixture();
    add_content(&fx, fx.a1, ContentRole::SceneText, "hello scene");
    let before_items = binder_item_commands::get_all_binder_item(&fx.ctx)
        .unwrap()
        .len();
    let before_contents = content_commands::get_all_content(&fx.ctx).unwrap().len();

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let ret = binder_item_management_commands::duplicate(
        &fx.ctx,
        Some(stack),
        &DuplicateDto {
            item_ids: vec![fx.a],
        },
    )
    .expect("duplicate");

    assert_eq!(ret.new_item_ids.len(), 1, "one new subtree root");
    // 3 new items (A,A1,A2 clones) and 1 new content (A1's clone).
    let after_items = binder_item_commands::get_all_binder_item(&fx.ctx)
        .unwrap()
        .len();
    let after_contents = content_commands::get_all_content(&fx.ctx).unwrap().len();
    assert_eq!(after_items, before_items + 3);
    assert_eq!(after_contents, before_contents + 1);

    // New subtree is spliced directly after the source subtree.
    let ord = order(&fx.ctx, fx.binder1);
    let new_root = ret.new_item_ids[0];
    let src_end = ord.iter().position(|&x| x == fx.a2).unwrap();
    assert_eq!(
        ord[src_end + 1],
        new_root,
        "clone follows the source subtree"
    );
    // The clone's content is a *new* Content entity (not aliased).
    let new_a1 = ord[src_end + 2];
    let new_contents = binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &new_a1,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    let orig_contents = binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &fx.a1,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    assert_eq!(new_contents.len(), 1);
    assert_ne!(
        new_contents[0], orig_contents[0],
        "content must be deep-copied"
    );

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(
        binder_item_commands::get_all_binder_item(&fx.ctx)
            .unwrap()
            .len(),
        before_items
    );
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]
    );
}

// ───────────────────────────────── trash ─────────────────────────────────

fn activated(ctx: &AppContext, id: EntityId) -> bool {
    item(ctx, id).activated
}

#[test]
fn trash_items_cascades_and_indexes_trash_info() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder_items(
        &fx.ctx,
        Some(stack),
        &TrashBinderItemsDto {
            binder_item_ids: vec![fx.a as i64],
            origin_binder_id: fx.binder1 as i64,
        },
    )
    .expect("trash");

    // Cascade: A and its subtree deactivated; B/C untouched.
    assert!(!activated(&fx.ctx, fx.a));
    assert!(!activated(&fx.ctx, fx.a1));
    assert!(!activated(&fx.ctx, fx.a2));
    assert!(activated(&fx.ctx, fx.b));
    assert!(activated(&fx.ctx, fx.c));
    // Items stay in place.
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]
    );
    // Exactly one TrashInfo, pointing at the root.
    let infos = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].trashed_binder_item, Some(fx.a));
    let indexed =
        work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::TrashInfos)
            .unwrap_or_default();
    assert_eq!(indexed, vec![infos[0].id]);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert!(activated(&fx.ctx, fx.a));
    assert!(activated(&fx.ctx, fx.a1));
    assert!(
        trash_info_commands::get_all_trash_info(&fx.ctx)
            .unwrap()
            .is_empty()
    );

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert!(!activated(&fx.ctx, fx.a));
    assert_eq!(
        trash_info_commands::get_all_trash_info(&fx.ctx)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn restore_items_round_trip() {
    let fx = make_fixture();
    let s1 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder_items(
        &fx.ctx,
        Some(s1),
        &TrashBinderItemsDto {
            binder_item_ids: vec![fx.c as i64],
            origin_binder_id: fx.binder1 as i64,
        },
    )
    .expect("trash");
    let info = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap()[0].id;

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = trash_management_commands::restore_items(
        &fx.ctx,
        Some(s2),
        &RestoreItemsDto {
            trash_info_ids: vec![info as i64],
        },
    )
    .expect("restore");
    assert_eq!(res.restored_count, 1);
    assert!(!res.orphaned);
    assert!(activated(&fx.ctx, fx.c));
    // Index emptied.
    let indexed =
        work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::TrashInfos)
            .unwrap_or_default();
    assert!(indexed.is_empty());
}

#[test]
fn restore_reports_orphaned_when_binder_lost_the_item() {
    let fx = make_fixture();
    let s1 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder_items(
        &fx.ctx,
        Some(s1),
        &TrashBinderItemsDto {
            binder_item_ids: vec![fx.c as i64],
            origin_binder_id: fx.binder1 as i64,
        },
    )
    .expect("trash");
    let info = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap()[0].id;
    // Simulate the item's binder losing it (e.g. binder removed elsewhere).
    wire_binder(
        &fx.ctx,
        fx.setup,
        fx.binder1,
        &[fx.a, fx.a1, fx.a2, fx.b, fx.b1],
    );

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = trash_management_commands::restore_items(
        &fx.ctx,
        Some(s2),
        &RestoreItemsDto {
            trash_info_ids: vec![info as i64],
        },
    )
    .expect("restore");
    assert!(res.orphaned, "an item with no binder is orphaned");
    assert_eq!(res.restored_count, 0);
}

#[test]
fn trash_binder_deactivates_binder_and_items() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder(
        &fx.ctx,
        Some(stack),
        &TrashBinderDto {
            binder_id: fx.binder1 as i64,
        },
    )
    .expect("trash binder");

    assert!(
        !binder_commands::get_binder(&fx.ctx, &fx.binder1)
            .unwrap()
            .unwrap()
            .activated
    );
    assert!(!activated(&fx.ctx, fx.a));
    assert!(!activated(&fx.ctx, fx.c));
    let infos = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].trashed_binder, Some(fx.binder1));

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert!(
        binder_commands::get_binder(&fx.ctx, &fx.binder1)
            .unwrap()
            .unwrap()
            .activated
    );
    assert!(activated(&fx.ctx, fx.a));
}

#[test]
fn empty_trash_hard_removes_item_subtree_and_contents() {
    let fx = make_fixture();
    add_content(&fx, fx.a1, ContentRole::SceneText, "doomed");
    let s1 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder_items(
        &fx.ctx,
        Some(s1),
        &TrashBinderItemsDto {
            binder_item_ids: vec![fx.a as i64],
            origin_binder_id: fx.binder1 as i64,
        },
    )
    .expect("trash");

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::empty_trash(&fx.ctx, Some(s2)).expect("empty");

    // A subtree gone from the store and the binder order.
    assert!(
        binder_item_commands::get_binder_item(&fx.ctx, &fx.a)
            .unwrap()
            .is_none()
    );
    assert!(
        binder_item_commands::get_binder_item(&fx.ctx, &fx.a1)
            .unwrap()
            .is_none()
    );
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.b, fx.b1, fx.c]);
    // Their content gone too.
    assert!(
        content_commands::get_all_content(&fx.ctx)
            .unwrap()
            .is_empty()
    );
    // Index cleared.
    assert!(
        trash_info_commands::get_all_trash_info(&fx.ctx)
            .unwrap()
            .is_empty()
    );

    undo_redo_commands::undo(&fx.ctx, Some(s2)).expect("undo empty");
    assert!(
        binder_item_commands::get_binder_item(&fx.ctx, &fx.a)
            .unwrap()
            .is_some()
    );
    assert_eq!(
        order(&fx.ctx, fx.binder1),
        vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]
    );
}

#[test]
fn empty_trash_removes_trashed_binder_from_work() {
    let fx = make_fixture();
    let s1 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder(
        &fx.ctx,
        Some(s1),
        &TrashBinderDto {
            binder_id: fx.binder2 as i64,
        },
    )
    .expect("trash binder");

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::empty_trash(&fx.ctx, Some(s2)).expect("empty");

    assert!(
        binder_commands::get_binder(&fx.ctx, &fx.binder2)
            .unwrap()
            .is_none()
    );
    let binders =
        work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::Binders)
            .unwrap();
    assert_eq!(
        binders,
        vec![fx.binder1],
        "trashed binder dropped from work"
    );
}

// ──────────────────────── restore + merge undo/redo ────────────────────────

fn system_trash_index(fx: &Fixture) -> Vec<EntityId> {
    work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::TrashInfos)
        .unwrap_or_default()
}

fn mk_scene(fx: &Fixture, title: &str) -> EntityId {
    let dto = CreateBinderItemDto {
        created_at: now(),
        updated_at: now(),
        title: title.to_string(),
        role: BinderItemRole::Item,
        sub_role: BinderItemSubRole::Scene,
        activated: true,
        is_exportable: true,
        indent: 0,
        ..Default::default()
    };
    binder_item_commands::create_orphan_binder_item(&fx.ctx, Some(fx.setup), &dto)
        .expect("create scene")
        .id
}

fn scene_text(fx: &Fixture, item_id: EntityId) -> String {
    content_data(fx, item_id, ContentRole::SceneText)
}

fn synopsis_text(fx: &Fixture, item_id: EntityId) -> String {
    content_data(fx, item_id, ContentRole::SynopsisText)
}

fn content_data(fx: &Fixture, item_id: EntityId, role: ContentRole) -> String {
    let cids = binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .expect("contents");
    for cid in cids {
        if let Some(c) = content_commands::get_content(&fx.ctx, &cid).expect("get content")
            && c.role == role
        {
            return c.data;
        }
    }
    String::new()
}

/// The targeted inverse of `restore_items`: undo re-trashes and re-links the
/// index; redo restores again.
#[test]
fn restore_items_undo_redo() {
    let fx = make_fixture();
    let s1 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder_items(
        &fx.ctx,
        Some(s1),
        &TrashBinderItemsDto {
            binder_item_ids: vec![fx.c as i64],
            origin_binder_id: fx.binder1 as i64,
        },
    )
    .expect("trash");
    let info = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap()[0].id;

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::restore_items(
        &fx.ctx,
        Some(s2),
        &RestoreItemsDto {
            trash_info_ids: vec![info as i64],
        },
    )
    .expect("restore");
    assert!(activated(&fx.ctx, fx.c));
    assert!(system_trash_index(&fx).is_empty());

    // Undo restore → c re-trashed, the index re-links the TrashInfo.
    undo_redo_commands::undo(&fx.ctx, Some(s2)).expect("undo restore");
    assert!(!activated(&fx.ctx, fx.c));
    assert_eq!(system_trash_index(&fx), vec![info]);

    // Redo restore → c active again, index emptied.
    undo_redo_commands::redo(&fx.ctx, Some(s2)).expect("redo restore");
    assert!(activated(&fx.ctx, fx.c));
    assert!(system_trash_index(&fx).is_empty());
}

/// merge_two_scenes appends B's text into A and trashes B; the targeted inverse
/// must restore A's content exactly, reactivate B, and drop the TrashInfo.
#[test]
fn merge_two_scenes_round_trip() {
    let fx = make_fixture();
    let a = mk_scene(&fx, "SceneA");
    let b = mk_scene(&fx, "SceneB");
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[a, b]);
    add_content(&fx, a, ContentRole::SceneText, "Alpha.");
    add_content(&fx, b, ContentRole::SceneText, "Bravo.");

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::merge_two_scenes(
        &fx.ctx,
        Some(stack),
        &MergeTwoScenesDto {
            target_id: a,
            source_id: b,
        },
    )
    .expect("merge");

    // A absorbed B's text; B is trashed and indexed.
    assert_eq!(scene_text(&fx, a), "Alpha.\n\nBravo.");
    assert!(!activated(&fx.ctx, b));
    let infos = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].trashed_binder_item, Some(b));

    // Undo → A's content restored exactly, B reactivated, TrashInfo gone.
    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo merge");
    assert_eq!(scene_text(&fx, a), "Alpha.");
    assert!(activated(&fx.ctx, b));
    assert!(
        trash_info_commands::get_all_trash_info(&fx.ctx)
            .unwrap()
            .is_empty()
    );

    // Redo → merged again.
    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo merge");
    assert_eq!(scene_text(&fx, a), "Alpha.\n\nBravo.");
    assert!(!activated(&fx.ctx, b));
    assert_eq!(
        trash_info_commands::get_all_trash_info(&fx.ctx)
            .unwrap()
            .len(),
        1
    );
}

// ─────────────────────── promote + split_scene undo/redo ───────────────────────

fn has_content_role(fx: &Fixture, item_id: EntityId, role: ContentRole) -> bool {
    let cids = binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .expect("contents");
    cids.iter().any(|cid| {
        content_commands::get_content(&fx.ctx, cid)
            .ok()
            .flatten()
            .map(|c| c.role == role)
            .unwrap_or(false)
    })
}

/// promote toggles Scene<->Note and remaps SceneText<->NoteText; the scoped
/// (item-rooted) snapshot must revert both on undo and redo.
#[test]
fn promote_undo_redo() {
    let fx = make_fixture();
    let s = mk_scene(&fx, "Scene");
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[s]);
    add_content(&fx, s, ContentRole::SceneText, "prose");

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::promote(
        &fx.ctx,
        Some(stack),
        &PromoteDto {
            item_id: s,
            target: PromoteTarget::Note.code(),
        },
    )
    .expect("promote");

    assert_eq!(item(&fx.ctx, s).sub_role, BinderItemSubRole::Note);
    assert!(has_content_role(&fx, s, ContentRole::NoteText));

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(item(&fx.ctx, s).sub_role, BinderItemSubRole::Scene);
    assert!(has_content_role(&fx, s, ContentRole::SceneText));

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(item(&fx.ctx, s).sub_role, BinderItemSubRole::Note);
    assert!(has_content_role(&fx, s, ContentRole::NoteText));
}

/// The headline of multi-target promote: a plain folder becomes any other kind of
/// folder. It carries only a synopsis, which every folder type allows, so nothing is
/// lost and its text comes along.
#[test]
fn a_plain_folder_becomes_any_other_kind_of_folder() {
    for (target, want) in [
        (
            PromoteTarget::ChapterFolder,
            BinderItemSubRole::ChapterScene,
        ),
        (PromoteTarget::PartFolder, BinderItemSubRole::Part),
        (PromoteTarget::BookFolder, BinderItemSubRole::Book),
        (PromoteTarget::NoteFolder, BinderItemSubRole::Note),
    ] {
        let fx = make_fixture();
        // A plain grouping folder — `Folder/None`, the shape you outline in.
        let f = mk_item(&fx.ctx, fx.setup, "Draft", 0, BinderItemRole::Folder);
        set_sub_role(&fx, f, BinderItemSubRole::None);
        wire_binder(&fx.ctx, fx.setup, fx.binder2, &[f]);
        add_content(&fx, f, ContentRole::SynopsisText, "what happens here");

        binder_item_management_commands::promote(
            &fx.ctx,
            None,
            &PromoteDto {
                item_id: f,
                target: target.code(),
            },
        )
        .unwrap_or_else(|e| panic!("promote to {target:?}: {e}"));

        let dto = item(&fx.ctx, f);
        assert_eq!(dto.role, BinderItemRole::Folder);
        assert_eq!(dto.sub_role, want, "promoting to {target:?}");
        assert_eq!(
            content_data(&fx, f, ContentRole::SynopsisText),
            "what happens here",
            "the synopsis must survive the conversion to {target:?}"
        );
    }
}

/// A name outlives the kind of thing it names: a chapter that becomes a part keeps its
/// title, remapped into the part's vocabulary.
#[test]
fn a_title_is_carried_across_a_type_change() {
    let fx = make_fixture();
    let ch = mk_item(
        &fx.ctx,
        fx.setup,
        "The Long Road",
        0,
        BinderItemRole::Folder,
    );
    set_sub_role(&fx, ch, BinderItemSubRole::ChapterScene);
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[ch]);
    add_content(&fx, ch, ContentRole::ChapterTitle, "The Long Road");

    binder_item_management_commands::promote(
        &fx.ctx,
        None,
        &PromoteDto {
            item_id: ch,
            target: PromoteTarget::PartFolder.code(),
        },
    )
    .expect("an empty chapter becomes a part");

    assert_eq!(item(&fx.ctx, ch).sub_role, BinderItemSubRole::Part);
    assert_eq!(
        content_data(&fx, ch, ContentRole::PartTitle),
        "The Long Road",
        "the chapter title became the part title"
    );
}

/// A conversion that has nowhere to keep the writer's text is refused outright, not
/// quietly performed with the prose dropped. A Part carries no scene prose.
#[test]
fn promote_refuses_to_discard_text() {
    let fx = make_fixture();
    let ch = mk_item(&fx.ctx, fx.setup, "Chapter", 0, BinderItemRole::Folder);
    set_sub_role(&fx, ch, BinderItemSubRole::ChapterScene);
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[ch]);
    add_content(&fx, ch, ContentRole::SceneText, "words the writer typed");

    let res = binder_item_management_commands::promote(
        &fx.ctx,
        None,
        &PromoteDto {
            item_id: ch,
            target: PromoteTarget::PartFolder.code(),
        },
    );
    assert!(res.is_err(), "a Part has nowhere to keep scene prose");
    // Nothing moved.
    assert_eq!(item(&fx.ctx, ch).sub_role, BinderItemSubRole::ChapterScene);
    assert_eq!(
        content_data(&fx, ch, ContentRole::SceneText),
        "words the writer typed"
    );

    // An *empty* prose row never blocks the conversion.
    let fx2 = make_fixture();
    let ch2 = mk_item(&fx2.ctx, fx2.setup, "Chapter", 0, BinderItemRole::Folder);
    set_sub_role(&fx2, ch2, BinderItemSubRole::ChapterScene);
    wire_binder(&fx2.ctx, fx2.setup, fx2.binder2, &[ch2]);
    add_content(&fx2, ch2, ContentRole::SceneText, "");
    binder_item_management_commands::promote(
        &fx2.ctx,
        None,
        &PromoteDto {
            item_id: ch2,
            target: PromoteTarget::PartFolder.code(),
        },
    )
    .expect("an empty prose row must not block the conversion");
    assert_eq!(item(&fx2.ctx, ch2).sub_role, BinderItemSubRole::Part);
}

/// The DTO carries a stable *code*, not a menu index, and the use case re-derives the
/// legal targets from the item's current type: a target that was never offered for this
/// item is refused.
#[test]
fn promote_refuses_a_target_that_was_never_offered() {
    let fx = make_fixture();
    let s = mk_scene(&fx, "Scene");
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[s]);

    // A Scene may only become a Note.
    let res = binder_item_management_commands::promote(
        &fx.ctx,
        None,
        &PromoteDto {
            item_id: s,
            target: PromoteTarget::BookFolder.code(),
        },
    );
    assert!(res.is_err(), "a Scene cannot become a Book folder");
    assert_eq!(item(&fx.ctx, s).sub_role, BinderItemSubRole::Scene);

    // An unknown code is rejected too.
    assert!(
        binder_item_management_commands::promote(
            &fx.ctx,
            None,
            &PromoteDto {
                item_id: s,
                target: 9999,
            },
        )
        .is_err()
    );
}

/// An item's name lives in two places: `BinderItem.title`, which the outline tree and
/// the tab show, and a title `Content` row, which is what compiles into the manuscript.
/// They are one title with two homes. This pins the promote path's half of that: a
/// conversion carries the name across *and* keeps both homes in step.
#[test]
fn a_rename_keeps_both_homes_of_the_title_in_step() {
    let fx = make_fixture();
    let ch = mk_item(&fx.ctx, fx.setup, "Old name", 0, BinderItemRole::Folder);
    set_sub_role(&fx, ch, BinderItemSubRole::ChapterScene);
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[ch]);
    add_content(&fx, ch, ContentRole::ChapterTitle, "Old name");

    // Converting to a Book must carry the name into the *book's* title role, and the
    // entity field must still agree with it.
    binder_item_management_commands::promote(
        &fx.ctx,
        None,
        &PromoteDto {
            item_id: ch,
            target: PromoteTarget::BookFolder.code(),
        },
    )
    .expect("an empty chapter becomes a book");

    assert_eq!(item(&fx.ctx, ch).sub_role, BinderItemSubRole::Book);
    assert_eq!(
        content_data(&fx, ch, ContentRole::BookTitle),
        "Old name",
        "the chapter title became the book title"
    );
    assert_eq!(
        item(&fx.ctx, ch).title,
        "Old name",
        "the entity field the tree shows is unchanged by the conversion"
    );
    assert_eq!(
        content_data(&fx, ch, ContentRole::ChapterTitle),
        "",
        "the old title role is gone"
    );
}

/// Set an item's sub_role directly (the fixture helper builds Text items).
fn set_sub_role(fx: &Fixture, item_id: EntityId, sub_role: BinderItemSubRole) {
    let mut dto = item(&fx.ctx, item_id);
    dto.sub_role = sub_role;
    binder_item_commands::update_binder_item(
        &fx.ctx,
        Some(fx.setup),
        &frontend::direct_access::UpdateBinderItemDto {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            title: dto.title,
            sub_title: dto.sub_title,
            role: dto.role,
            sub_role: dto.sub_role,
            label: dto.label,
            activated: dto.activated,
            is_favorite: dto.is_favorite,
            is_exportable: dto.is_exportable,
            indent: dto.indent,
            word_count_goal: dto.word_count_goal,
            char_count_goal: dto.char_count_goal,
            dict_language: dto.dict_language,
        },
    )
    .expect("set sub_role");
}

/// split_scene creates a new scene after the source; the scoped (binder-rooted)
/// snapshot must delete it on undo (restoring the source text) and re-add it on redo.
#[test]
fn split_scene_undo_redo() {
    let fx = make_fixture();
    let s = mk_scene(&fx, "Full");
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[s]);
    add_content(&fx, s, ContentRole::SceneText, "AB");

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::split_scene(
        &fx.ctx,
        Some(stack),
        &SplitSceneDto {
            source_id: s,
            before_text: "A".into(),
            after_text: "B".into(),
            before_synopsis: String::new(),
            after_synopsis: String::new(),
            new_title: "Second".into(),
        },
    )
    .expect("split");

    // Source keeps "A"; a new scene follows carrying "B".
    assert_eq!(scene_text(&fx, s), "A");
    let after = order(&fx.ctx, fx.binder2);
    assert_eq!(after.len(), 2);
    assert_eq!(after[0], s);
    let new_scene = after[1];
    assert_eq!(scene_text(&fx, new_scene), "B");

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(scene_text(&fx, s), "AB");
    assert_eq!(order(&fx.ctx, fx.binder2), vec![s]);
    assert!(
        binder_item_commands::get_binder_item(&fx.ctx, &new_scene)
            .unwrap()
            .is_none()
    );

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(scene_text(&fx, s), "A");
    assert_eq!(order(&fx.ctx, fx.binder2), vec![s, new_scene]);
    assert_eq!(scene_text(&fx, new_scene), "B");
}

/// Splitting from the **synopsis** editor: the synopsis is cut at the caret, the
/// prose stays whole on the source, and the whole thing undoes/redoes cleanly —
/// the scoped snapshot must cover the `SynopsisText` rows too, including the one
/// created on the new scene.
#[test]
fn split_scene_from_synopsis_undo_redo() {
    let fx = make_fixture();
    let s = mk_scene(&fx, "Full");
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[s]);
    add_content(&fx, s, ContentRole::SceneText, "the whole prose");
    add_content(&fx, s, ContentRole::SynopsisText, "AB");

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_management_commands::split_scene(
        &fx.ctx,
        Some(stack),
        &SplitSceneDto {
            source_id: s,
            // The untouched role goes whole to the source, empty to the new scene.
            before_text: "the whole prose".into(),
            after_text: String::new(),
            before_synopsis: "A".into(),
            after_synopsis: "B".into(),
            new_title: "Second".into(),
        },
    )
    .expect("split");

    let after = order(&fx.ctx, fx.binder2);
    assert_eq!(after.len(), 2);
    let new_scene = after[1];
    assert_eq!(scene_text(&fx, s), "the whole prose");
    assert_eq!(synopsis_text(&fx, s), "A");
    assert_eq!(scene_text(&fx, new_scene), "");
    assert_eq!(synopsis_text(&fx, new_scene), "B");

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(scene_text(&fx, s), "the whole prose");
    assert_eq!(
        synopsis_text(&fx, s),
        "AB",
        "undo restores the whole synopsis"
    );
    assert_eq!(order(&fx.ctx, fx.binder2), vec![s]);
    assert!(
        binder_item_commands::get_binder_item(&fx.ctx, &new_scene)
            .unwrap()
            .is_none()
    );

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(synopsis_text(&fx, s), "A");
    assert_eq!(order(&fx.ctx, fx.binder2), vec![s, new_scene]);
    assert_eq!(synopsis_text(&fx, new_scene), "B");
    assert_eq!(scene_text(&fx, new_scene), "");
}

fn item_tags(fx: &Fixture, item_id: EntityId) -> Vec<EntityId> {
    binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &item_id,
        &BinderItemRelationshipField::Tags,
    )
    .expect("tags")
}

/// duplicate copies M2M tag links; undo (scoped restore) must delete the clone AND
/// its tag junction while leaving the shared BinderTag itself intact.
#[test]
fn duplicate_reverts_cloned_tag_links() {
    let fx = make_fixture();
    let tag = binder_tag_commands::create_orphan_binder_tag(
        &fx.ctx,
        Some(fx.setup),
        &CreateBinderTagDto {
            created_at: now(),
            updated_at: now(),
            name: "Important".into(),
            color: "#f00".into(),
            text_color: "#fff".into(),
        },
    )
    .expect("create tag")
    .id;
    let source = mk_scene(&fx, "Tagged");
    wire_binder(&fx.ctx, fx.setup, fx.binder2, &[source]);
    binder_item_commands::set_binder_item_relationship(
        &fx.ctx,
        Some(fx.setup),
        &BinderItemRelationshipDto {
            id: source,
            field: BinderItemRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .expect("tag the item");

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = binder_item_management_commands::duplicate(
        &fx.ctx,
        Some(stack),
        &DuplicateDto {
            item_ids: vec![source],
        },
    )
    .expect("duplicate");
    let clone = res.new_item_ids[0];

    // The clone shares the tag link.
    assert_eq!(item_tags(&fx, clone), vec![tag]);

    // Undo removes the clone and its tag junction; the shared tag entity survives.
    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert!(
        binder_item_commands::get_binder_item(&fx.ctx, &clone)
            .unwrap()
            .is_none()
    );
    assert!(item_tags(&fx, clone).is_empty(), "no dangling tag junction");
    assert!(
        binder_tag_commands::get_binder_tag(&fx.ctx, &tag)
            .unwrap()
            .is_some(),
        "shared tag must not be deleted"
    );
    assert_eq!(item_tags(&fx, source), vec![tag], "source keeps its tag");
}
