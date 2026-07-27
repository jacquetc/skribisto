// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Regression test for the code-review finding: `merge_two_scenes` takes an
//! explicit `work_id` (Phase 0.6) but never checked it against the binder the
//! two scenes actually live in. `work_id()` (the `dto.work_id` -> open-Work
//! lookup) only validates the *scalar* id; every other check in the use case
//! cross-references `target`/`source` against EACH OTHER and against the
//! binder they share, never against the Work.
//!
//! This test builds two real, independently open Works straight through
//! `direct_access` (bypassing `work_management`'s single-open-Work sweep,
//! which would otherwise make a second open Work impossible to construct at
//! all) and proves that merging two scenes that legitimately share Work B's
//! binder is REJECTED when called under Work A's work_id, rather than
//! silently merging Work B's content and trashing Work B's row while the
//! undo/redo snapshot stays scoped to Work A.
//!
//! Without the fix in `use_cases/merge_two_scenes_uc.rs`, this test fails:
//! the call returns `Ok(())` and quietly mutates Work B's tree.

use std::sync::Arc;

use binder_item_management::MergeTwoScenesDto;
use binder_item_management::binder_item_management_controller;
use common::database::db_context::DbContext;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItemRole, BinderItemSubRole};
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
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;

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
        let root_id = root_controller::create_orphan(&db, &hub, &CreateRootDto::default())
            .expect("root")
            .id;
        Ctx { db, hub, undo, root_id }
    }

    /// A fresh, independently-open Work with one Binder — built straight
    /// through `direct_access`, NOT `work_management::new_work` (which closes
    /// every other open Work first — the Phase-0 tripwire documented on
    /// `new_work_uc.rs`). This is the only way to get two Works open at once
    /// today, which is exactly the state this bug needs.
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
            &CreateWorkDto { smart_punctuation: smart_punctuation_id, ..Default::default() },
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
            &CreateBinderDto { activated: true, ..Default::default() },
            work_id,
            -1,
        )
        .expect("binder")
        .id;
        (work_id, binder_id)
    }

    /// A plain `Item/Scene` row — prose-bearing (carries `SceneText`) and not
    /// a structural opener, so it is a legal `target` AND a legal `source`
    /// for `merge_two_scenes` (see `skribisto_model::COMBINATIONS`).
    fn new_scene(&mut self, binder_id: EntityId) -> EntityId {
        binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                ..Default::default()
            },
            binder_id,
            -1,
        )
        .expect("binder_item")
        .id
    }
}

#[test]
fn merge_two_scenes_rejects_scenes_owned_by_a_different_work() {
    let mut ctx = Ctx::new();
    let (work_a, _binder_a) = ctx.new_project();
    let (_work_b, binder_b) = ctx.new_project();

    // Two adjacent, mergeable scenes -- both legitimately in Work B's binder.
    let target_b = ctx.new_scene(binder_b);
    let source_b = ctx.new_scene(binder_b);

    // Work A's work_id, but target/source both belong to Work B.
    let result = binder_item_management_controller::merge_two_scenes(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &MergeTwoScenesDto {
            work_id: work_a as u64,
            target_id: target_b as u64,
            source_id: source_b as u64,
        },
    );

    assert!(
        result.is_err(),
        "two scenes belonging to Work B must not be mergeable under Work A's work_id"
    );

    // Work B's source row must still be active (not trashed by the rejected call).
    let source = binder_item_controller::get(&ctx.db, &source_b).unwrap().unwrap();
    assert!(source.activated, "the rejected call must not have trashed Work B's source scene");

    // Work B's target row must not have gained any content from the rejected merge.
    let target_contents = binder_item_controller::get_relationship(
        &ctx.db,
        &target_b,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    assert!(target_contents.is_empty(), "the rejected call must not have merged content into Work B's target");
}
