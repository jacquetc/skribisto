// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `trash_selection` — one gesture, several binders, both kinds of row.
//!
//! Two things are being pinned here, and they are different in kind.
//!
//! The first is the **grouping**, which is the reason the use case exists: four
//! view-models were each resolving every item's origin binder by hand, and an
//! item filed under the wrong origin is restored to the wrong place. So the
//! fixture deliberately spans two binders in one Work.
//!
//! The second is the **Work scoping**, which is inherited rather than new: the
//! checks live in `crate::trash_ops` and are shared with `trash_binder` and
//! `trash_binder_items`, whose own `work_scoping.rs` covers them from the other
//! two doors. Re-covering them here is not redundancy — a new caller of shared
//! checks is exactly where a wiring mistake would show up, and this one accepts
//! ids for several binders at once, which neither of the others does.

use std::sync::Arc;

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
use direct_access::trash_info::trash_info_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;
use trash_management::TrashSelectionDto;
use trash_management::trash_management_controller as feature;

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
        Ctx {
            db,
            hub,
            undo,
            root_id,
        }
    }

    /// An independently open Work — built straight through `direct_access`, the
    /// only way to have two open at once (see `work_scoping.rs`).
    fn new_work(&mut self) -> EntityId {
        let smart_punctuation = smart_punctuation_controller::create_orphan(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateSmartPunctuationDto::default(),
        )
        .expect("smart_punctuation")
        .id;
        work_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                smart_punctuation,
                ..Default::default()
            },
            self.root_id,
            -1,
        )
        .expect("work")
        .id
    }

    fn new_binder(&mut self, work: EntityId) -> EntityId {
        binder_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderDto {
                activated: true,
                ..Default::default()
            },
            work,
            -1,
        )
        .expect("binder")
        .id
    }

    fn push(&mut self, binder: EntityId, indent: i64) -> EntityId {
        binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                status: None,
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                indent,
                ..Default::default()
            },
            binder,
            -1,
        )
        .expect("binder_item")
        .id
    }

    fn active(&self, id: EntityId) -> bool {
        binder_item_controller::get(&self.db, &id)
            .expect("get")
            .expect("row still present")
            .activated
    }

    fn binder_active(&self, id: EntityId) -> bool {
        binder_controller::get(&self.db, &id)
            .expect("get")
            .expect("binder still present")
            .activated
    }

    /// `(origin_binder_id, trashed_item_id)` for every TrashInfo under `work`.
    fn trash_entries(&self, work: EntityId) -> Vec<(i64, Option<EntityId>)> {
        work_controller::get_relationship(
            &self.db,
            &work,
            &common::direct_access::work::WorkRelationshipField::TrashInfos,
        )
        .expect("trash_infos")
        .into_iter()
        .map(|id| {
            let info = trash_info_controller::get(&self.db, &id)
                .expect("get")
                .expect("info");
            let item = trash_info_controller::get_relationship(
                &self.db,
                &id,
                &common::direct_access::trash_info::TrashInfoRelationshipField::TrashedBinderItem,
            )
            .expect("trashed item")
            .first()
            .copied();
            (info.origin_binder_id, item)
        })
        .collect()
    }

    fn trash(
        &mut self,
        work: EntityId,
        binder_ids: &[EntityId],
        item_ids: &[EntityId],
    ) -> anyhow::Result<trash_management::TrashSelectionResultDto> {
        feature::trash_selection(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &TrashSelectionDto {
                work_id: work,
                binder_ids: binder_ids.to_vec(),
                binder_item_ids: item_ids.to_vec(),
            },
        )
    }
}

#[test]
fn items_from_two_binders_go_in_one_gesture_each_filed_under_its_own_origin() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let left = ctx.new_binder(work);
    let right = ctx.new_binder(work);
    let a = ctx.push(left, 0);
    let b = ctx.push(right, 0);

    let out = ctx.trash(work, &[], &[a, b]).expect("trash");

    assert_eq!(out.trashed_item_ids.len(), 2);
    assert!(!ctx.active(a));
    assert!(!ctx.active(b));

    // The point of the whole use case: each row remembers the binder it came
    // from, so restore puts it back where it was.
    let mut entries = ctx.trash_entries(work);
    entries.sort();
    let mut expected = vec![(left as i64, Some(a)), (right as i64, Some(b))];
    expected.sort();
    assert_eq!(entries, expected);
}

#[test]
fn a_subtree_is_absorbed_into_its_root_rather_than_filed_twice() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let root = ctx.push(binder, 0);
    let child = ctx.push(binder, 1);

    // Both named explicitly — a selection can easily contain an item and its
    // parent, and restoring the child twice would duplicate it.
    let out = ctx.trash(work, &[], &[root, child]).expect("trash");

    assert_eq!(
        out.trashed_item_ids,
        vec![root],
        "the child is covered by the root"
    );
    assert!(!ctx.active(root));
    assert!(!ctx.active(child), "the cascade still goes down");
    assert_eq!(ctx.trash_entries(work).len(), 1);
}

#[test]
fn an_item_inside_a_binder_being_trashed_is_not_filed_separately() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let item = ctx.push(binder, 0);

    let out = ctx.trash(work, &[binder], &[item]).expect("trash");

    assert_eq!(out.trashed_binder_ids, vec![binder]);
    assert!(
        out.trashed_item_ids.is_empty(),
        "the binder's own entry already covers everything in it"
    );
    assert_eq!(ctx.trash_entries(work).len(), 1);
    assert!(!ctx.binder_active(binder));
    assert!(!ctx.active(item));
}

#[test]
fn a_repeated_id_files_one_entry() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let item = ctx.push(binder, 0);

    let out = ctx.trash(work, &[], &[item, item]).expect("trash");

    assert_eq!(out.trashed_item_ids, vec![item]);
    assert_eq!(ctx.trash_entries(work).len(), 1);
}

#[test]
fn undo_reactivates_everything_and_removes_the_entries() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let left = ctx.new_binder(work);
    let right = ctx.new_binder(work);
    let a = ctx.push(left, 0);
    let a_child = ctx.push(left, 1);
    let b = ctx.push(right, 0);
    ctx.trash(work, &[right], &[a]).expect("trash");

    ctx.undo.undo(None).expect("undo");

    assert!(ctx.active(a));
    assert!(ctx.active(a_child));
    assert!(ctx.active(b));
    assert!(ctx.binder_active(right));
    assert!(
        ctx.trash_entries(work).is_empty(),
        "the bin must not keep entries for rows that are back"
    );
}

#[test]
fn redo_puts_it_all_back_in_the_bin() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let item = ctx.push(binder, 0);
    ctx.trash(work, &[], &[item]).expect("trash");
    ctx.undo.undo(None).expect("undo");

    ctx.undo.redo(None).expect("redo");

    assert!(!ctx.active(item));
    assert_eq!(ctx.trash_entries(work).len(), 1);
}

#[test]
fn a_binder_belonging_to_another_open_work_is_refused() {
    let mut ctx = Ctx::new();
    let work_a = ctx.new_work();
    let work_b = ctx.new_work();
    let binder_b = ctx.new_binder(work_b);
    let item_b = ctx.push(binder_b, 0);

    // Work A's work_id, Work B's binder. Accepting this would trash Work B's
    // rows while filing the entry under Work A — B loses the item with nothing
    // in its bin to restore.
    let result = ctx.trash(work_a, &[binder_b], &[]);

    assert!(result.is_err(), "{result:?}");
    assert!(ctx.binder_active(binder_b));
    assert!(ctx.active(item_b));
    assert!(ctx.trash_entries(work_a).is_empty());
}

#[test]
fn an_item_belonging_to_another_open_work_is_refused() {
    let mut ctx = Ctx::new();
    let work_a = ctx.new_work();
    let work_b = ctx.new_work();
    let binder_a = ctx.new_binder(work_a);
    let keep = ctx.push(binder_a, 0);
    let binder_b = ctx.new_binder(work_b);
    let item_b = ctx.push(binder_b, 0);

    // The item route resolves its own binder, so the ownership check has to
    // happen on what it resolved — not on anything the caller supplied.
    let result = ctx.trash(work_a, &[], &[keep, item_b]);

    assert!(result.is_err(), "{result:?}");
    assert!(ctx.active(item_b));
    assert!(
        ctx.active(keep),
        "a rejected call must not have trashed the half that was legitimate"
    );
    assert!(ctx.trash_entries(work_a).is_empty());
}

#[test]
fn a_work_that_is_not_open_is_refused() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let item = ctx.push(binder, 0);

    let result = ctx.trash(work + 9_999, &[], &[item]);

    assert!(result.is_err(), "{result:?}");
    assert!(ctx.active(item));
}

#[test]
fn an_empty_selection_is_refused_rather_than_filing_nothing() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    assert!(ctx.trash(work, &[], &[]).is_err());
}

#[test]
fn the_cascade_reaches_a_grandchild() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let root = ctx.push(binder, 0);
    let child = ctx.push(binder, 1);
    let grand = ctx.push(binder, 2);
    let sibling = ctx.push(binder, 0);

    ctx.trash(work, &[], &[root]).expect("trash");

    assert!(!ctx.active(root));
    assert!(!ctx.active(child));
    assert!(!ctx.active(grand));
    assert!(
        ctx.active(sibling),
        "the cascade stops at the first row back up to the root's indent"
    );
}

#[test]
fn contents_travel_with_the_row_they_belong_to() {
    let mut ctx = Ctx::new();
    let work = ctx.new_work();
    let binder = ctx.new_binder(work);
    let item = ctx.push(binder, 0);

    ctx.trash(work, &[], &[item]).expect("trash");
    ctx.undo.undo(None).expect("undo");

    // Trashing is a flag, not a move: the row keeps its place and its children.
    let contents = binder_item_controller::get_relationship(
        &ctx.db,
        &item,
        &BinderItemRelationshipField::Contents,
    )
    .expect("contents");
    assert!(contents.is_empty());
    assert!(ctx.active(item));
}
