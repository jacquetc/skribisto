// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Regression tests for the code-review finding: every use case in this crate
//! that takes an explicit `work_id` (Phase 0.6) must reject a `(work_id, ids)`
//! pair where the ids belong to a DIFFERENT, real, open Work rather than
//! silently mutating it. `work_id()` (the `dto.work_id` -> open-Work lookup
//! shared by every use case here) only validates the *scalar* id — it says
//! nothing about whether the accompanying entity ids are actually that Work's
//! own rows. Each test builds TWO real, independently open Works straight
//! through `direct_access` (bypassing `work_management`'s single-open-Work
//! sweep, which would otherwise make a second open Work impossible to
//! construct at all) and proves that feeding Work B's ids under Work A's
//! work_id is REJECTED, not silently applied to Work B's tree.
//!
//! Without the fix in each `use_cases/*.rs` file, every one of these tests
//! fails: the call returns `Ok(..)` and quietly mutates the *other* Work.

use std::sync::Arc;

use common::database::db_context::DbContext;
use common::event::EventHub;
use common::types::EntityId;
use common::undo_redo::UndoRedoManager;
use direct_access::binder::binder_controller;
use direct_access::binder::dtos::CreateBinderDto;
use direct_access::binder_item::binder_item_controller;
use direct_access::binder_item::dtos::CreateBinderItemDto;
use direct_access::root::dtos::CreateRootDto;
use direct_access::root::root_controller;
use direct_access::smart_punctuation::dtos::CreateSmartPunctuationDto;
use direct_access::smart_punctuation::smart_punctuation_controller;
use direct_access::trash_info::dtos::CreateTrashInfoDto;
use direct_access::trash_info::trash_info_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;
use trash_management::dtos::DropPosition;
use trash_management::trash_management_controller;
use trash_management::{
    DeleteTrashEntriesDto, RestoreItemsDto, RestoreItemsToDto, TrashBinderDto, TrashBinderItemsDto,
};

struct Ctx {
    db: DbContext,
    hub: Arc<EventHub>,
    undo: UndoRedoManager,
    root_id: EntityId,
}

impl Ctx {
    fn new() -> Self {
        let db = DbContext::new().expect("in-memory store");
        let hub = Arc::new(EventHub::new());
        let mut undo = UndoRedoManager::new();
        undo.set_event_hub(&hub);
        // Root is not undoable — no undo_redo_manager parameter.
        let root_id = root_controller::create_orphan(&db, &hub, &CreateRootDto::default())
            .expect("root")
            .id;
        Ctx {
            db,
            hub,
            undo,
            root_id,
        }
    }

    /// A fresh, independently-open Work with one activated Binder — built
    /// straight through `direct_access`, NOT `work_management::new_work`
    /// (which closes every other open Work first — the Phase-0 tripwire
    /// documented on `new_work_uc.rs`). This is the only way to get two Works
    /// open at once today, which is exactly the state this bug needs.
    fn new_project(&mut self) -> (EntityId, EntityId) {
        // `smart_punctuation` is a one_to_one strong FK on `Work`, seeded at
        // create time (see `new_work_uc.rs`): two Works can never legally
        // share a punctuation row, so `CreateWorkDto::default()` (which
        // defaults the field to the placeholder id `0`) would make this
        // helper's *second* call collide under the generated uniqueness
        // check the moment two projects are open at once — exactly the
        // scenario this test file exists to exercise.
        let smart_punctuation_id = smart_punctuation_controller::create_orphan(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateSmartPunctuationDto::default(),
        )
        .expect("smart_punctuation")
        .id;
        let work_id = work_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                smart_punctuation: smart_punctuation_id,
                ..Default::default()
            },
            self.root_id,
            -1,
        )
        .expect("work")
        .id;
        let binder_id = binder_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderDto {
                activated: true,
                ..Default::default()
            },
            work_id,
            -1,
        )
        .expect("binder")
        .id;
        (work_id, binder_id)
    }

    fn new_item(&mut self, binder_id: EntityId, activated: bool) -> EntityId {
        binder_item_management_item(self, binder_id, activated)
    }

    fn new_trash_info_for_item(
        &mut self,
        work_id: EntityId,
        item_id: EntityId,
        origin_binder_id: EntityId,
    ) -> EntityId {
        trash_info_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateTrashInfoDto {
                trashed_at: chrono::Utc::now(),
                origin_binder_id: origin_binder_id as i64,
                trashed_binder_item: Some(item_id),
                ..Default::default()
            },
            work_id,
            -1,
        )
        .expect("trash_info")
        .id
    }
}

fn binder_item_management_item(ctx: &mut Ctx, binder_id: EntityId, activated: bool) -> EntityId {
    binder_item_controller::create(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &CreateBinderItemDto {
            status: None,
            activated,
            ..Default::default()
        },
        binder_id,
        -1,
    )
    .expect("binder_item")
    .id
}

// -----------------------------------------------------------------------
// restore_items: trash_info_ids must belong to dto.work_id
// -----------------------------------------------------------------------

#[test]
fn restore_items_rejects_a_trash_info_owned_by_a_different_work() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();
    let (work_b, binder_b) = ctx.new_project();

    let item_b = ctx.new_item(binder_b, false); // already "trashed" in place
    let trash_info_b = ctx.new_trash_info_for_item(work_b, item_b, binder_b);

    // Work A's work_id, but the TrashInfo actually belongs to Work B.
    let result = trash_management_controller::restore_items(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &RestoreItemsDto {
            work_id: work_a,
            trash_info_ids: vec![trash_info_b as i64],
        },
    );

    assert!(
        result.is_err(),
        "a TrashInfo belonging to Work B must not be restorable under Work A's work_id"
    );

    // And Work B's item must be untouched by the rejected call.
    let item = direct_access::binder_item::binder_item_controller::get(&ctx.db, &item_b)
        .unwrap()
        .unwrap();
    assert!(
        !item.activated,
        "the rejected call must not have reactivated Work B's item"
    );
}

// -----------------------------------------------------------------------
// restore_items_to: binder_item_ids (and their source/destination binders)
// must belong to dto.work_id
// -----------------------------------------------------------------------

#[test]
fn restore_items_to_rejects_a_source_item_owned_by_a_different_work() {
    let mut ctx = Ctx::new();
    let (work_a, binder_a) = ctx.new_project(); // destination
    let (_work_b, binder_b) = ctx.new_project(); // owns the item

    let item_b = ctx.new_item(binder_b, false); // trashed in place

    let result = trash_management_controller::restore_items_to(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &RestoreItemsToDto {
            work_id: work_a,
            binder_item_ids: vec![item_b],
            destination_binder_id: binder_a,
            anchor_item_id: None,
            drop_position: DropPosition::Before,
        },
    );

    assert!(
        result.is_err(),
        "an item belonging to Work B must not be relocatable into Work A's binder under Work A's work_id"
    );

    // Work B's item must not have been moved into Work A's binder.
    let dest_items = direct_access::binder::binder_controller::get_relationship(
        &ctx.db,
        &binder_a,
        &common::direct_access::binder::BinderRelationshipField::BinderItems,
    )
    .unwrap();
    assert!(
        !dest_items.contains(&item_b),
        "the rejected call must not have moved Work B's item"
    );
}

// -----------------------------------------------------------------------
// delete_trash_entries: trash_info_ids must belong to dto.work_id (the
// "sneakiest variant" — a real id under the wrong, but also real/open, Work
// must be a hard error, not swallowed by the stale-id tolerance)
// -----------------------------------------------------------------------

#[test]
fn delete_trash_entries_rejects_a_trash_info_owned_by_a_different_work() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();
    let (work_b, binder_b) = ctx.new_project();

    let item_b = ctx.new_item(binder_b, false);
    let trash_info_b = ctx.new_trash_info_for_item(work_b, item_b, binder_b);

    let result = trash_management_controller::delete_trash_entries(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &DeleteTrashEntriesDto {
            work_id: work_a,
            trash_info_ids: vec![trash_info_b],
        },
    );

    assert!(
        result.is_err(),
        "a TrashInfo belonging to Work B must not be purgeable under Work A's work_id"
    );

    // Work B's TrashInfo must still exist — the rejected call purged nothing.
    assert!(
        direct_access::trash_info::trash_info_controller::get(&ctx.db, &trash_info_b)
            .unwrap()
            .is_some(),
        "the rejected call must not have purged Work B's trash entry"
    );
}

/// The rejection must name **the ids the caller sent**, and only those.
///
/// `get_work_relationships_from_right_ids` hands back each matched Work's whole
/// trash index rather than the subset asked about, so reporting it raw listed
/// every entry in Work B's bin — including ones Work A had never heard of. The
/// guard fired correctly either way, which is why nothing caught this until a
/// second entry existed to be wrongly named.
#[test]
fn delete_trash_entries_names_only_the_offending_ids() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();
    let (work_b, binder_b) = ctx.new_project();

    let item_one = ctx.new_item(binder_b, false);
    let item_two = ctx.new_item(binder_b, false);
    let named = ctx.new_trash_info_for_item(work_b, item_one, binder_b);
    // A second entry in the same bin, which the caller never mentions.
    let bystander = ctx.new_trash_info_for_item(work_b, item_two, binder_b);

    let err = trash_management_controller::delete_trash_entries(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &DeleteTrashEntriesDto {
            work_id: work_a,
            trash_info_ids: vec![named],
        },
    )
    .expect_err("Work B's entry must not be purgeable under Work A's work_id");

    let msg = format!("{err:#}");
    assert!(
        msg.contains(&named.to_string()),
        "the offending id must be named: {msg}"
    );
    assert!(
        !msg.contains(&bystander.to_string()),
        "an entry the caller never sent must not appear in the complaint: {msg}"
    );
}

/// Companion/control: a genuinely stale id (never existed at all) under the
/// CORRECT work_id must still be tolerated silently — the documented
/// "stale id -> no-op, not error" contract this file's fix must not break.
#[test]
fn delete_trash_entries_still_tolerates_a_truly_stale_id() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();

    let result = trash_management_controller::delete_trash_entries(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &DeleteTrashEntriesDto {
            work_id: work_a,
            trash_info_ids: vec![999_999],
        },
    );

    assert!(
        result.is_ok(),
        "an id that never existed anywhere must be tolerated, not an error"
    );
}

// -----------------------------------------------------------------------
// trash_binder_items: origin_binder_id (and the items in it) must belong to
// dto.work_id
// -----------------------------------------------------------------------

#[test]
fn trash_binder_items_rejects_an_origin_binder_owned_by_a_different_work() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();
    let (_work_b, binder_b) = ctx.new_project();

    let item_b = ctx.new_item(binder_b, true); // still active, in Work B

    let result = trash_management_controller::trash_binder_items(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &TrashBinderItemsDto {
            work_id: work_a,
            binder_item_ids: vec![item_b as i64],
            origin_binder_id: binder_b as i64,
        },
    );

    assert!(
        result.is_err(),
        "a binder belonging to Work B must not be trashable under Work A's work_id"
    );

    let item = direct_access::binder_item::binder_item_controller::get(&ctx.db, &item_b)
        .unwrap()
        .unwrap();
    assert!(
        item.activated,
        "the rejected call must not have trashed Work B's item"
    );
}

// -----------------------------------------------------------------------
// trash_binder: binder_id must belong to dto.work_id
// -----------------------------------------------------------------------

#[test]
fn trash_binder_rejects_a_binder_owned_by_a_different_work() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();
    let (_work_b, binder_b) = ctx.new_project();

    let result = trash_management_controller::trash_binder(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &TrashBinderDto {
            work_id: work_a,
            binder_id: binder_b as i64,
        },
    );

    assert!(
        result.is_err(),
        "a binder belonging to Work B must not be trashable under Work A's work_id"
    );

    let binder = direct_access::binder::binder_controller::get(&ctx.db, &binder_b)
        .unwrap()
        .unwrap();
    assert!(
        binder.activated,
        "the rejected call must not have trashed Work B's binder"
    );
}
