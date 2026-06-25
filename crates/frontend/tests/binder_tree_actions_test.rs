//! Integration tests for the binder-tree action use cases: move_items,
//! duplicate, and the four trash_management use cases — plus their undo/redo.
//!
//! Fixtures are built synthetically via the direct-access create commands (no
//! `.skrib` file needed). The fixture is created on a dedicated `setup` undo
//! stack; each action runs on its own fresh stack so `undo`/`redo` touch only
//! the action under test.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    system_commands, trash_info_commands, trash_management_commands, undo_redo_commands,
    work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::system::SystemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    BinderRelationshipDto, CreateBinderDto, CreateBinderItemDto, CreateContentDto, CreateSystemDto,
    CreateWorkDto, WorkRelationshipDto,
};

use binder_item_management::{DuplicateDto, MoveDto, MovePlace};
use trash_management::{RestoreItemsDto, TrashBinderDto, TrashBinderItemsDto};

// ───────────────────────────── fixture helpers ─────────────────────────────

struct Fixture {
    ctx: AppContext,
    setup: u64,
    system: EntityId,
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
        is_printable: true,
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

    Fixture {
        ctx,
        setup,
        system,
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
    binder_item_commands::set_binder_item_relationship(
        &fx.ctx,
        Some(fx.setup),
        &frontend::direct_access::BinderItemRelationshipDto {
            id: item_id,
            field: BinderItemRelationshipField::Contents,
            right_ids: vec![cid],
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.c, fx.a, fx.a1, fx.a2, fx.b, fx.b1]);
    assert_eq!(indent(&fx.ctx, fx.c), 0);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]);
    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.c, fx.a, fx.a1, fx.a2, fx.b, fx.b1]);
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.c, fx.b, fx.b1]);
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.c, fx.b, fx.b1]);
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.c, fx.a2, fx.b, fx.b1]);
    assert_eq!(indent(&fx.ctx, fx.c), 1, "Into a leaf == After it (sibling indent)");
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.b, fx.b1, fx.a, fx.a1, fx.a2, fx.c]);
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]);
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]);
}

// ─────────────────────────────── duplicate ───────────────────────────────

#[test]
fn duplicate_folder_subtree_clones_items_and_content() {
    let fx = make_fixture();
    add_content(&fx, fx.a1, ContentRole::SceneText, "hello scene");
    let before_items = binder_item_commands::get_all_binder_item(&fx.ctx).unwrap().len();
    let before_contents = content_commands::get_all_content(&fx.ctx).unwrap().len();

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let ret = binder_item_management_commands::duplicate(
        &fx.ctx,
        Some(stack),
        &DuplicateDto { item_ids: vec![fx.a] },
    )
    .expect("duplicate");

    assert_eq!(ret.new_item_ids.len(), 1, "one new subtree root");
    // 3 new items (A,A1,A2 clones) and 1 new content (A1's clone).
    let after_items = binder_item_commands::get_all_binder_item(&fx.ctx).unwrap().len();
    let after_contents = content_commands::get_all_content(&fx.ctx).unwrap().len();
    assert_eq!(after_items, before_items + 3);
    assert_eq!(after_contents, before_contents + 1);

    // New subtree is spliced directly after the source subtree.
    let ord = order(&fx.ctx, fx.binder1);
    let new_root = ret.new_item_ids[0];
    let src_end = ord.iter().position(|&x| x == fx.a2).unwrap();
    assert_eq!(ord[src_end + 1], new_root, "clone follows the source subtree");
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
    assert_ne!(new_contents[0], orig_contents[0], "content must be deep-copied");

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(
        binder_item_commands::get_all_binder_item(&fx.ctx).unwrap().len(),
        before_items
    );
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]);
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
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]);
    // Exactly one TrashInfo, pointing at the root.
    let infos = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].trashed_binder_item, Some(fx.a));
    let indexed = system_commands::get_system_relationship(
        &fx.ctx,
        &fx.system,
        &SystemRelationshipField::TrashInfos,
    )
    .unwrap_or_default();
    assert_eq!(indexed, vec![infos[0].id]);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert!(activated(&fx.ctx, fx.a));
    assert!(activated(&fx.ctx, fx.a1));
    assert!(trash_info_commands::get_all_trash_info(&fx.ctx).unwrap().is_empty());

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert!(!activated(&fx.ctx, fx.a));
    assert_eq!(trash_info_commands::get_all_trash_info(&fx.ctx).unwrap().len(), 1);
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
        &RestoreItemsDto { trash_info_ids: vec![info as i64] },
    )
    .expect("restore");
    assert_eq!(res.restored_count, 1);
    assert!(!res.orphaned);
    assert!(activated(&fx.ctx, fx.c));
    // Index emptied.
    let indexed = system_commands::get_system_relationship(
        &fx.ctx,
        &fx.system,
        &SystemRelationshipField::TrashInfos,
    )
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
    wire_binder(&fx.ctx, fx.setup, fx.binder1, &[fx.a, fx.a1, fx.a2, fx.b, fx.b1]);

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = trash_management_commands::restore_items(
        &fx.ctx,
        Some(s2),
        &RestoreItemsDto { trash_info_ids: vec![info as i64] },
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
        &TrashBinderDto { binder_id: fx.binder1 as i64 },
    )
    .expect("trash binder");

    assert!(!binder_commands::get_binder(&fx.ctx, &fx.binder1).unwrap().unwrap().activated);
    assert!(!activated(&fx.ctx, fx.a));
    assert!(!activated(&fx.ctx, fx.c));
    let infos = trash_info_commands::get_all_trash_info(&fx.ctx).unwrap();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].trashed_binder, Some(fx.binder1));

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert!(binder_commands::get_binder(&fx.ctx, &fx.binder1).unwrap().unwrap().activated);
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
    assert!(binder_item_commands::get_binder_item(&fx.ctx, &fx.a).unwrap().is_none());
    assert!(binder_item_commands::get_binder_item(&fx.ctx, &fx.a1).unwrap().is_none());
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.b, fx.b1, fx.c]);
    // Their content gone too.
    assert!(content_commands::get_all_content(&fx.ctx).unwrap().is_empty());
    // Index cleared.
    assert!(trash_info_commands::get_all_trash_info(&fx.ctx).unwrap().is_empty());

    undo_redo_commands::undo(&fx.ctx, Some(s2)).expect("undo empty");
    assert!(binder_item_commands::get_binder_item(&fx.ctx, &fx.a).unwrap().is_some());
    assert_eq!(order(&fx.ctx, fx.binder1), vec![fx.a, fx.a1, fx.a2, fx.b, fx.b1, fx.c]);
}

#[test]
fn empty_trash_removes_trashed_binder_from_work() {
    let fx = make_fixture();
    let s1 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::trash_binder(
        &fx.ctx,
        Some(s1),
        &TrashBinderDto { binder_id: fx.binder2 as i64 },
    )
    .expect("trash binder");

    let s2 = undo_redo_commands::create_new_stack(&fx.ctx);
    trash_management_commands::empty_trash(&fx.ctx, Some(s2)).expect("empty");

    assert!(binder_commands::get_binder(&fx.ctx, &fx.binder2).unwrap().is_none());
    let binders = work_commands::get_work_relationship(
        &fx.ctx,
        &fx.work,
        &WorkRelationshipField::Binders,
    )
    .unwrap();
    assert_eq!(binders, vec![fx.binder1], "trashed binder dropped from work");
}
