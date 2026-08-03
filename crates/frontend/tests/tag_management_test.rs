// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for tag lifecycle: `tag_management::import_tags`, and the *generated*
//! delete path it deliberately does not wrap.
//!
//! There is no `delete_tag` use case: `BinderTagRepository::remove` already cleans the
//! `jn_binder_tag_from_binder_item_tags` junction via `impl_leaf_entity_table!`'s
//! `delete_from_backward_junction` (see `binder_tag_table.rs`), and scoped restore puts it
//! back on undo. The deletion tests below pin that behaviour on the generic command, since
//! nothing else did.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_tag_commands, tag_management_commands,
    undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    BinderItemRelationshipDto, CreateBinderDto, CreateBinderItemDto, CreateBinderTagDto,
    CreateWorkDto,
};
use tag_management::ImportTagsDto;

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

struct Fixture {
    ctx: AppContext,
    setup: u64,
    work: EntityId,
}

/// A Work with one binder, built on a dedicated undo stack so the arrange phase never
/// pollutes the stack the action under test runs on.
fn make_fixture() -> Fixture {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);

    let work = work_commands::create_orphan_work(
        &ctx,
        Some(setup),
        &CreateWorkDto {
            created_at: now(),
            updated_at: now(),
            title: "The Lighthouse".into(),
            ..Default::default()
        },
    )
    .expect("create work")
    .id;

    let binder = binder_commands::create_orphan_binder(
        &ctx,
        Some(setup),
        &CreateBinderDto {
            created_at: now(),
            updated_at: now(),
            name: "Manuscript".into(),
            activated: true,
            ..Default::default()
        },
    )
    .expect("create binder")
    .id;
    work_commands::set_work_relationship(
        &ctx,
        Some(setup),
        &frontend::direct_access::WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::Binders,
            right_ids: vec![binder],
        },
    )
    .expect("wire binder");

    Fixture { ctx, setup, work }
}

fn mk_tag(fx: &Fixture, name: &str, discoverable: bool) -> EntityId {
    let tag = binder_tag_commands::create_orphan_binder_tag(
        &fx.ctx,
        Some(fx.setup),
        &CreateBinderTagDto {
            created_at: now(),
            updated_at: now(),
            name: name.into(),
            color: "#f00".into(),
            details: String::new(),
            discoverable,
        },
    )
    .expect("create tag")
    .id;
    let mut ids = work_tags(fx);
    ids.push(tag);
    set_work_tags(fx, &ids);
    tag
}

fn mk_item(fx: &Fixture, title: &str) -> EntityId {
    binder_item_commands::create_orphan_binder_item(
        &fx.ctx,
        Some(fx.setup),
        &CreateBinderItemDto {
            created_at: now(),
            updated_at: now(),
            title: title.into(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
    )
    .expect("create item")
    .id
}

fn work_tags(fx: &Fixture) -> Vec<EntityId> {
    work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::Tags)
        .expect("work tags")
}

fn set_work_tags(fx: &Fixture, ids: &[EntityId]) {
    work_commands::set_work_relationship(
        &fx.ctx,
        Some(fx.setup),
        &frontend::direct_access::WorkRelationshipDto {
            id: fx.work,
            field: WorkRelationshipField::Tags,
            right_ids: ids.to_vec(),
        },
    )
    .expect("set work tags");
}

fn item_tags(fx: &Fixture, item: EntityId) -> Vec<EntityId> {
    binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &item,
        &BinderItemRelationshipField::Tags,
    )
    .expect("item tags")
}

fn tag_item(fx: &Fixture, item: EntityId, tags: &[EntityId]) {
    binder_item_commands::set_binder_item_relationship(
        &fx.ctx,
        Some(fx.setup),
        &BinderItemRelationshipDto {
            id: item,
            field: BinderItemRelationshipField::Tags,
            right_ids: tags.to_vec(),
        },
    )
    .expect("tag item");
}

/// Deleting a tag that is still in use must leave no id without a row behind it — the
/// generated `remove_multi` scrubs the item junction as well as the owning palette list.
///
/// This is the assertion a redundant `delete_tag` use case was once written to guarantee.
/// Keeping it here, against the generic command, is what proves the wrapper is unnecessary.
#[test]
fn deleting_a_tag_in_use_leaves_no_dangling_junction() {
    let fx = make_fixture();
    let doomed = mk_tag(&fx, "Draft", false);
    let keeper = mk_tag(&fx, "character", true);
    let a = mk_item(&fx, "Scene A");
    let b = mk_item(&fx, "Scene B");
    tag_item(&fx, a, &[doomed, keeper]);
    tag_item(&fx, b, &[doomed]);

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_tag_commands::remove_binder_tag_multi(&fx.ctx, Some(stack), &[doomed])
        .expect("remove tag");

    assert_eq!(item_tags(&fx, a), vec![keeper], "the other tag is untouched");
    assert!(item_tags(&fx, b).is_empty());
    assert!(
        !work_tags(&fx).contains(&doomed),
        "the palette must drop it too"
    );
    assert!(
        binder_tag_commands::get_binder_tag(&fx.ctx, &doomed)
            .unwrap()
            .is_none(),
        "the row itself is gone"
    );
    // The real assertion: no item references an id with no row behind it.
    for item in [a, b] {
        for id in item_tags(&fx, item) {
            assert!(
                binder_tag_commands::get_binder_tag(&fx.ctx, &id)
                    .unwrap()
                    .is_some(),
                "item {item} still references deleted tag {id}"
            );
        }
    }
}

/// Undo restores the row AND puts it back in every junction it was in, in order — scoped
/// restore reconciles the weak backrefs, so no bespoke inverse is needed.
#[test]
fn undoing_a_tag_delete_restores_the_row_and_its_assignments() {
    let fx = make_fixture();
    let doomed = mk_tag(&fx, "Draft", false);
    let keeper = mk_tag(&fx, "character", true);
    let a = mk_item(&fx, "Scene A");
    let b = mk_item(&fx, "Scene B");
    tag_item(&fx, a, &[doomed, keeper]);
    tag_item(&fx, b, &[doomed]);

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_tag_commands::remove_binder_tag_multi(&fx.ctx, Some(stack), &[doomed])
        .expect("remove tag");

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");

    let restored = binder_tag_commands::get_binder_tag(&fx.ctx, &doomed)
        .unwrap()
        .expect("the tag row is back");
    assert_eq!(restored.name, "Draft");
    assert!(work_tags(&fx).contains(&doomed), "back in the palette");
    assert_eq!(
        item_tags(&fx, a),
        vec![doomed, keeper],
        "assignments come back in order"
    );
    assert_eq!(item_tags(&fx, b), vec![doomed]);

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert!(
        binder_tag_commands::get_binder_tag(&fx.ctx, &doomed)
            .unwrap()
            .is_none()
    );
    assert_eq!(item_tags(&fx, a), vec![keeper]);
}

/// Deleting several tags at once is one undo step, so the settings pane needs no wrapper
/// to batch them.
#[test]
fn deleting_several_tags_is_one_undo_step() {
    let fx = make_fixture();
    let one = mk_tag(&fx, "Draft", false);
    let two = mk_tag(&fx, "Revise", false);
    let keeper = mk_tag(&fx, "character", true);
    let a = mk_item(&fx, "Scene A");
    tag_item(&fx, a, &[one, two, keeper]);

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_tag_commands::remove_binder_tag_multi(&fx.ctx, Some(stack), &[one, two])
        .expect("remove tags");
    assert_eq!(item_tags(&fx, a), vec![keeper]);
    assert_eq!(work_tags(&fx), vec![keeper]);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert_eq!(
        item_tags(&fx, a),
        vec![one, two, keeper],
        "one undo brings both back"
    );
}

/// The count the delete confirmation needs ("detaches it from N items"), and the usage
/// column in the settings pane, are both a plain read — the other reason a dedicated use
/// case earned nothing.
///
/// Note the shape: the repository has `get_relationships_from_right_ids`, which would
/// answer this in one reverse-index lookup, but it is **not exposed** through
/// `binder_item_commands`. So the UI must scan items and filter, as below. That is
/// acceptable here (a settings pane is not a hot path, and `docks/inspector.rs` already
/// scans items the same way), but if the tag chips ever need per-tag counts on a hot path,
/// exposing that command is the fix rather than scanning harder.
#[test]
fn items_carrying_a_tag_are_discoverable_before_deleting_it() {
    let fx = make_fixture();
    let doomed = mk_tag(&fx, "Draft", false);
    let a = mk_item(&fx, "Scene A");
    let b = mk_item(&fx, "Scene B");
    let untagged = mk_item(&fx, "Scene C");
    tag_item(&fx, a, &[doomed]);
    tag_item(&fx, b, &[doomed]);

    let carriers: Vec<EntityId> = binder_item_commands::get_all_binder_item(&fx.ctx)
        .expect("all items")
        .into_iter()
        .filter(|i| item_tags(&fx, i.id).contains(&doomed))
        .map(|i| i.id)
        .collect();

    assert_eq!(carriers.len(), 2, "exactly the two tagged items");
    assert!(carriers.contains(&a) && carriers.contains(&b));
    assert!(!carriers.contains(&untagged));
}

fn import(fx: &Fixture, stack: u64, rows: &[(&str, &str, &str, bool)]) -> tag_management::ImportTagsResultDto {
    tag_management_commands::import_tags(
        &fx.ctx,
        Some(stack),
        &ImportTagsDto {
            work_id: fx.work,
            names: rows.iter().map(|r| r.0.to_string()).collect(),
            colors: rows.iter().map(|r| r.1.to_string()).collect(),
            details: rows.iter().map(|r| r.2.to_string()).collect(),
            discoverables: rows.iter().map(|r| r.3).collect(),
        },
    )
    .expect("import_tags")
}

/// A preset must apply as ONE undoable action — twelve undos to reverse one click would be
/// worse than not offering presets.
#[test]
fn importing_tags_is_a_single_undo_step() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = import(
        &fx,
        stack,
        &[
            ("status/draft", "#888", "Not yet revised", false),
            ("character", "#0a0", "A person in the story", true),
            ("place", "#00a", "", true),
        ],
    );

    assert_eq!(res.created_ids.len(), 3);
    assert!(res.skipped_names.is_empty());
    assert_eq!(work_tags(&fx).len(), 3, "all three joined the palette");

    let created = binder_tag_commands::get_binder_tag(&fx.ctx, &res.created_ids[1])
        .unwrap()
        .expect("tag");
    assert_eq!(created.name, "character");
    assert_eq!(created.details, "A person in the story");
    assert!(created.discoverable, "discoverability survives the import");

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    assert!(
        work_tags(&fx).is_empty(),
        "one undo reverts the whole preset"
    );
    for id in &res.created_ids {
        assert!(
            binder_tag_commands::get_binder_tag(&fx.ctx, id)
                .unwrap()
                .is_none()
        );
    }

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(work_tags(&fx).len(), 3);
}

/// Re-applying a preset (or re-importing an edited CSV) must not double the palette.
/// Comparison is case-insensitive and trimmed, matching the UI's duplicate-name warning.
#[test]
fn importing_skips_names_already_in_the_palette() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    import(&fx, stack, &[("character", "#0a0", "", true)]);

    let stack2 = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = import(
        &fx,
        stack2,
        &[
            ("  CHARACTER  ", "#f00", "different colour", false),
            ("place", "#00a", "", true),
        ],
    );

    assert_eq!(res.created_ids.len(), 1, "only `place` is new");
    assert_eq!(res.skipped_names, vec!["  CHARACTER  ".to_string()]);
    assert_eq!(work_tags(&fx).len(), 2);
}

/// A batch containing the same name twice imports it once — the seen-set grows as rows are
/// accepted, not only from what was already in the palette.
#[test]
fn importing_deduplicates_within_one_batch() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let res = import(
        &fx,
        stack,
        &[
            ("place", "#00a", "", true),
            ("Place", "#0a0", "", false),
            ("  ", "#000", "blank name", false),
        ],
    );

    assert_eq!(res.created_ids.len(), 1);
    assert_eq!(res.skipped_names.len(), 2, "the dupe and the blank");
    assert_eq!(work_tags(&fx).len(), 1);
}

/// The four DTO vectors are one table transposed; a ragged set means the caller built them
/// inconsistently and would otherwise silently truncate.
#[test]
fn importing_rejects_ragged_columns() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let err = tag_management_commands::import_tags(
        &fx.ctx,
        Some(stack),
        &ImportTagsDto {
            work_id: fx.work,
            names: vec!["a".into(), "b".into()],
            colors: vec!["#f00".into()],
            details: vec![String::new(), String::new()],
            discoverables: vec![false, false],
        },
    )
    .expect_err("should refuse");
    assert!(
        format!("{err:#}").contains("column lengths differ"),
        "unexpected error: {err:#}"
    );
}
