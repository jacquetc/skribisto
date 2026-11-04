// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Undoing a structural command keeps the prose written since it.
//!
//! An open editor writes its prose through `UNTRACKED_STACK_ID`, so nothing on the
//! project's stack records it. The undo of a command restores a snapshot of the
//! rows it covered, and used to restore their `Content` rows with them: moving a
//! chapter, typing into it, then undoing the move erased the typing. These tests
//! write prose the way the editor does, between a command and its undo, and check
//! that the undo and the redo both leave it there — while a row the command itself
//! rewrote (a merge) still goes back.

use std::sync::Arc;

use binder_item_management::binder_item_management_controller as feature;
use binder_item_management::{MergeTwoScenesDto, MoveDto, MovePlace};
use common::database::db_context::DbContext;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use common::event::EventHub;
use common::types::EntityId;
use common::undo_redo::{UNTRACKED_STACK_ID, UndoRedoManager};
use direct_access::binder::binder_controller;
use direct_access::binder::dtos::CreateBinderDto;
use direct_access::binder_item::binder_item_controller;
use direct_access::binder_item::dtos::CreateBinderItemDto;
use direct_access::content::content_controller;
use direct_access::content::dtos::{CreateContentDto, UpdateContentDto};
use direct_access::root::dtos::CreateRootDto;
use direct_access::root::root_controller;
use direct_access::smart_punctuation::dtos::CreateSmartPunctuationDto;
use direct_access::smart_punctuation::smart_punctuation_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;

/// One open Work, one Binder, and the scenes a test pushes into it.
struct Ctx {
    db: DbContext,
    hub: Arc<EventHub>,
    undo: UndoRedoManager,
    work_id: EntityId,
    binder_id: EntityId,
    stack: u64,
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
        let smart_punctuation = smart_punctuation_controller::create_orphan(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateSmartPunctuationDto::default(),
        )
        .expect("smart_punctuation")
        .id;
        let work_id = work_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                smart_punctuation,
                ..Default::default()
            },
            root_id,
            -1,
        )
        .expect("work")
        .id;
        let binder_id = binder_controller::create(
            &db,
            &hub,
            &mut undo,
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
        let stack = undo.create_new_stack();
        Ctx {
            db,
            hub,
            undo,
            work_id,
            binder_id,
            stack,
        }
    }

    /// A top-level scene holding `prose` in its `SceneText` row.
    fn scene(&mut self, prose: &str) -> EntityId {
        let binder_id = self.binder_id;
        let item = binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                status: None,
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder_id,
            -1,
        )
        .expect("binder_item")
        .id;
        content_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateContentDto {
                role: ContentRole::SceneText,
                activated: true,
                data: prose.to_string(),
                ..Default::default()
            },
            item,
            -1,
        )
        .expect("content");
        item
    }

    fn prose_row(&self, item: EntityId) -> Option<direct_access::content::dtos::ContentDto> {
        let ids = binder_item_controller::get_relationship(
            &self.db,
            &item,
            &BinderItemRelationshipField::Contents,
        )
        .expect("contents");
        content_controller::get_multi(&self.db, &ids)
            .expect("get_multi")
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::SceneText)
    }

    fn prose(&self, item: EntityId) -> String {
        self.prose_row(item).expect("a SceneText row").data
    }

    /// Write prose the way an open editor's flush does: off the project's stack.
    fn type_into(&mut self, item: EntityId, prose: &str) {
        let row = self.prose_row(item).expect("a SceneText row");
        content_controller::update(
            &self.db,
            &self.hub,
            &mut self.undo,
            Some(UNTRACKED_STACK_ID),
            &UpdateContentDto {
                id: row.id,
                created_at: row.created_at,
                updated_at: chrono::Utc::now(),
                uid: row.uid,
                activated: row.activated,
                role: row.role,
                data: prose.to_string(),
            },
        )
        .expect("prose flush");
    }

    fn order(&self) -> Vec<EntityId> {
        binder_controller::get_relationship(
            &self.db,
            &self.binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .expect("binder_items")
    }
}

#[test]
fn undoing_a_move_keeps_what_was_typed_into_the_moved_scene() {
    let mut ctx = Ctx::new();
    let one = ctx.scene("One.");
    let two = ctx.scene("Two.");
    let three = ctx.scene("Three.");
    let stack = Some(ctx.stack);

    feature::move_items(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        stack,
        &MoveDto {
            item_ids: vec![three],
            target_id: Some(one),
            target_is_binder: false,
            move_place: MovePlace::Before,
        },
    )
    .expect("move");
    assert_eq!(ctx.order(), vec![three, one, two]);

    ctx.type_into(three, "Three, and a sentence written after the move.");
    ctx.type_into(one, "One, edited too.");

    ctx.undo.undo(stack).expect("undo");
    assert_eq!(ctx.order(), vec![one, two, three], "the move is undone");
    assert_eq!(
        ctx.prose(three),
        "Three, and a sentence written after the move."
    );
    assert_eq!(ctx.prose(one), "One, edited too.");
    assert_eq!(ctx.prose(two), "Two.");

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(ctx.order(), vec![three, one, two], "the move is redone");
    assert_eq!(
        ctx.prose(three),
        "Three, and a sentence written after the move."
    );
    assert_eq!(ctx.prose(one), "One, edited too.");
}

#[test]
fn typing_between_an_undo_and_its_redo_survives_the_redo() {
    let mut ctx = Ctx::new();
    let one = ctx.scene("One.");
    let two = ctx.scene("Two.");
    let stack = Some(ctx.stack);

    feature::move_items(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        stack,
        &MoveDto {
            item_ids: vec![two],
            target_id: Some(one),
            target_is_binder: false,
            move_place: MovePlace::Before,
        },
    )
    .expect("move");
    ctx.undo.undo(stack).expect("undo");
    ctx.type_into(two, "Two, written while the move was undone.");

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(ctx.order(), vec![two, one]);
    assert_eq!(ctx.prose(two), "Two, written while the move was undone.");
}

#[test]
fn undoing_a_merge_puts_back_the_merged_prose_and_keeps_the_rest() {
    let mut ctx = Ctx::new();
    let first = ctx.scene("First.");
    let second = ctx.scene("Second.");
    let other = ctx.scene("Elsewhere.");
    let stack = Some(ctx.stack);

    feature::merge_two_scenes(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        stack,
        &MergeTwoScenesDto {
            work_id: ctx.work_id,
            target_id: first,
            source_id: second,
        },
    )
    .expect("merge");
    let merged = ctx.prose(first);
    assert_ne!(merged, "First.", "the merge rewrote the target's prose");

    ctx.type_into(other, "Elsewhere, typed after the merge.");

    ctx.undo.undo(stack).expect("undo");
    // The merge's own rewrite is what this undo undoes.
    assert_eq!(ctx.prose(first), "First.");
    assert_eq!(ctx.prose(second), "Second.");
    // A scene the merge never touched keeps its new prose.
    assert_eq!(ctx.prose(other), "Elsewhere, typed after the merge.");

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(ctx.prose(first), merged);
    assert_eq!(ctx.prose(other), "Elsewhere, typed after the merge.");
}
