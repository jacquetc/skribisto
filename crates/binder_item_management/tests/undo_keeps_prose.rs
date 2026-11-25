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

impl Ctx {
    /// The item's row of `role`, if it has one.
    fn row_of(
        &self,
        item: EntityId,
        role: ContentRole,
    ) -> Option<direct_access::content::dtos::ContentDto> {
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
            .find(|c| c.role == role)
    }

    /// Type into a scratchpad for the first time, the way an open editor's first
    /// flush does: the row is created then, off the project's stack.
    fn first_scratchpad_flush(&mut self, item: EntityId, text: &str) {
        assert!(
            self.row_of(item, ContentRole::ScratchpadText).is_none(),
            "precondition: no scratchpad row yet"
        );
        content_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            Some(UNTRACKED_STACK_ID),
            &CreateContentDto {
                role: ContentRole::ScratchpadText,
                activated: true,
                data: text.to_string(),
                ..Default::default()
            },
            item,
            -1,
        )
        .expect("scratchpad created on first flush");
    }

    /// Write into an existing scratchpad row, the way an open editor's flush does.
    fn scratchpad_flush(&mut self, item: EntityId, text: &str) {
        let row = self
            .row_of(item, ContentRole::ScratchpadText)
            .expect("a scratchpad row");
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
                data: text.to_string(),
            },
        )
        .expect("scratchpad flush");
    }

    /// How many scratchpad rows the item holds.
    fn scratchpad_rows(&self, item: EntityId) -> usize {
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
            .filter(|c| c.role == ContentRole::ScratchpadText)
            .count()
    }

    fn merge(&mut self, target: EntityId, source: EntityId) {
        let stack = Some(self.stack);
        feature::merge_two_scenes(
            &self.db,
            &self.hub,
            &mut self.undo,
            stack,
            &MergeTwoScenesDto {
                work_id: self.work_id,
                target_id: target,
                source_id: source,
            },
        )
        .expect("merge");
    }

    fn scratchpad(&self, item: EntityId) -> Option<String> {
        self.row_of(item, ContentRole::ScratchpadText)
            .map(|c| c.data)
    }
}

/// A scratchpad has no row until its first keystroke, so the first one typed after a
/// command is a row neither of the command's snapshots knows. The undo's restore used
/// to delete it with everything else created after the snapshot, losing the note.
#[test]
fn a_scratchpad_first_typed_after_a_move_survives_its_undo_and_redo() {
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
    ctx.first_scratchpad_flush(two, "Check the ferry timetable.");

    ctx.undo.undo(stack).expect("undo");
    assert_eq!(ctx.order(), vec![one, two], "the move is undone");
    assert_eq!(
        ctx.scratchpad(two).as_deref(),
        Some("Check the ferry timetable."),
        "the scratchpad typed after the move outlives its undo"
    );
    assert_eq!(ctx.prose(two), "Two.", "the scene's own prose is untouched");

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(ctx.order(), vec![two, one], "the move is redone");
    assert_eq!(
        ctx.scratchpad(two).as_deref(),
        Some("Check the ferry timetable.")
    );
}

/// The same across a merge, whose undo restores rows it rewrote: a scratchpad started
/// on the surviving scene after the merge is the writer's, not the merge's, and stays.
/// The merged scene text still goes back, since that rewrite is what the undo undoes.
#[test]
fn a_scratchpad_first_typed_after_a_merge_survives_its_undo() {
    let mut ctx = Ctx::new();
    let first = ctx.scene("First.");
    let second = ctx.scene("Second.");
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
    ctx.first_scratchpad_flush(first, "Merged: reread the join.");

    ctx.undo.undo(stack).expect("undo");
    assert_eq!(ctx.prose(first), "First.", "the merge itself is undone");
    assert_eq!(ctx.prose(second), "Second.");
    assert_eq!(
        ctx.scratchpad(first).as_deref(),
        Some("Merged: reread the join.")
    );
    assert_eq!(
        ctx.scratchpad(second),
        None,
        "nothing invented on the source"
    );

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(
        ctx.scratchpad(first).as_deref(),
        Some("Merged: reread the join."),
        "and the redo keeps it too"
    );
}

/// A merge folds the source's scratchpad into the target, which had none. Undone, the
/// target has no scratchpad again, and the writer starts one there. The redo brings
/// the merged scratchpad back on the target, and the new one has no place beside it
/// (one row per role): its text is joined onto the end of the merged one rather than
/// dropped, which is what the redo used to do with it.
#[test]
fn a_scratchpad_started_between_a_merges_undo_and_redo_joins_the_merged_one() {
    let mut ctx = Ctx::new();
    let first = ctx.scene("First.");
    let second = ctx.scene("Second.");
    let stack = Some(ctx.stack);
    ctx.first_scratchpad_flush(second, "Ask Marthe about the lighthouse.");

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
    assert_eq!(
        ctx.scratchpad(first).as_deref(),
        Some("Ask Marthe about the lighthouse."),
        "the merge folded the source's scratchpad into the target"
    );

    ctx.undo.undo(stack).expect("undo");
    assert_eq!(ctx.scratchpad(first), None, "the target had none before");
    ctx.first_scratchpad_flush(first, "The tide turns at four.");

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(
        ctx.scratchpad(first).as_deref(),
        Some("Ask Marthe about the lighthouse.\n\nThe tide turns at four."),
        "the note typed while the merge was undone is kept, after the merged one"
    );
    let ids = binder_item_controller::get_relationship(
        &ctx.db,
        &first,
        &BinderItemRelationshipField::Contents,
    )
    .expect("contents");
    let pads = content_controller::get_multi(&ctx.db, &ids)
        .expect("get_multi")
        .into_iter()
        .flatten()
        .filter(|c| c.role == ContentRole::ScratchpadText)
        .count();
    assert_eq!(pads, 1, "one scratchpad row on the target, never two");
}

/// The redo above joins the new note onto the row the merge made, and that row is in
/// the merge's "after" snapshot only, so the next undo removes it. The note typed
/// while the merge was undone was then in no snapshot and on no row: lost for good.
/// Every undo and redo after that keeps it, on the scene it was typed on.
#[test]
fn a_note_joined_by_a_merges_redo_survives_every_undo_and_redo_after_it() {
    let mut ctx = Ctx::new();
    let first = ctx.scene("First.");
    let second = ctx.scene("Second.");
    let stack = Some(ctx.stack);
    ctx.first_scratchpad_flush(second, "Ask Marthe about the lighthouse.");
    ctx.merge(first, second);
    ctx.undo.undo(stack).expect("undo");
    ctx.first_scratchpad_flush(first, "The tide turns at four.");
    ctx.undo.redo(stack).expect("redo");

    for round in 1..=2 {
        ctx.undo.undo(stack).expect("undo");
        assert_eq!(
            ctx.scratchpad(first).as_deref(),
            Some("The tide turns at four."),
            "round {round}: the note typed on the first scene is back on it"
        );
        assert_eq!(
            ctx.scratchpad(second).as_deref(),
            Some("Ask Marthe about the lighthouse."),
            "round {round}: the second scene has its own note again"
        );
        assert_eq!(ctx.scratchpad_rows(first), 1, "round {round}: one row");

        ctx.undo.redo(stack).expect("redo");
        assert_eq!(
            ctx.scratchpad(first).as_deref(),
            Some("Ask Marthe about the lighthouse.\n\nThe tide turns at four."),
            "round {round}: the redo joins it after the merged note again"
        );
        assert_eq!(ctx.scratchpad_rows(first), 1, "round {round}: one row");
    }
}

/// The scratchpad the merge made on the target is the writer's to type into. A
/// paragraph added at its end since is not the merge's: undoing the merge takes back
/// the folded note and keeps that paragraph on the target. A change inside the folded
/// note is the merge's text rewritten, and goes back with it, like the merged prose.
#[test]
fn undoing_a_merge_keeps_what_was_added_to_the_end_of_the_merged_note() {
    let mut ctx = Ctx::new();
    let first = ctx.scene("First.");
    let second = ctx.scene("Second.");
    let stack = Some(ctx.stack);
    ctx.first_scratchpad_flush(second, "Ask Marthe about the lighthouse.");
    ctx.merge(first, second);
    ctx.scratchpad_flush(
        first,
        "Ask Marthe about the lighthouse.\n\nShe kept the logbook.\n",
    );

    ctx.undo.undo(stack).expect("undo");
    assert_eq!(
        ctx.scratchpad(first).as_deref(),
        Some("She kept the logbook.\n"),
        "the paragraph added since the merge stays on the target"
    );
    assert_eq!(
        ctx.scratchpad(second).as_deref(),
        Some("Ask Marthe about the lighthouse."),
        "the folded note is back on the source"
    );

    ctx.undo.redo(stack).expect("redo");
    assert_eq!(
        ctx.scratchpad(first).as_deref(),
        Some("Ask Marthe about the lighthouse.\n\nShe kept the logbook.\n"),
        "the redo puts the two back together"
    );
    assert_eq!(ctx.scratchpad_rows(first), 1);

    ctx.scratchpad_flush(first, "Ask Marta about the lighthouse.");
    ctx.undo.undo(stack).expect("undo");
    assert_eq!(
        ctx.scratchpad(first),
        None,
        "a change inside the folded note is the merge's own text, undone with it"
    );
}
