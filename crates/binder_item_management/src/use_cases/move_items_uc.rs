// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: reorder / reparent a set of BinderItems (with their
// contiguous subtrees) within or across Binders. Undoable via a scoped
// snapshot/restore of the affected binder subtree(s) (v1.8 restore reverts only
// the subtree rooted at the snapshot's root ids).
//
// The binder's `binder_items` relationship is an ordered flat list; an item's
// subtree is the contiguous run of following items whose `indent` is strictly
// greater than the item's. All traversal uses the binder's relationship-vec
// order (never raw entity-scan order). The pure ordering math lives in the
// shared `binder_ordering` crate (also used by trash_management::restore_items_to).
use crate::MoveDto;
use crate::MovePlace;
use anyhow::{Result, anyhow};
use binder_ordering::{
    DropPlace, anchor_for_binder_target, expand_to_subtrees, insert_block, resolve_item_target,
};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::{Binder, BinderItem, BinderItemRole};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

/// Map the feature-local `MovePlace` DTO enum onto the shared `binder_ordering`
/// `DropPlace` (Qleany DTO enums can't be shared across crates).
fn to_drop_place(place: &MovePlace) -> DropPlace {
    match place {
        MovePlace::Before => DropPlace::Before,
        MovePlace::After => DropPlace::After,
        MovePlace::Into => DropPlace::Into,
    }
}

pub trait MoveItemsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn MoveItemsUnitOfWorkTrait>;
}

// Read-write unit of work trait. The same macro set must appear on the impl
// block in ../units_of_work/move_items_uow.rs.
#[macros::uow_action(entity = "Binder", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "Get")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait MoveItemsUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_move_items_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct MoveItemsUseCase {
    uow_factory: Box<dyn MoveItemsUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl MoveItemsUseCase {
    pub fn new(uow_factory: Box<dyn MoveItemsUnitOfWorkFactoryTrait>) -> Self {
        MoveItemsUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &MoveDto) -> Result<()> {
        if dto.item_ids.is_empty() {
            return Err(anyhow!("move_items: no items to move"));
        }
        let target_id = dto
            .target_id
            .ok_or_else(|| anyhow!("move_items: missing target"))?;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // The scoped snapshot is taken below, once the affected binders are known,
        // so undo/redo revert exactly those subtrees. All resolution above the
        // snapshot is read-only. On any early `return Err`, the uow is dropped and
        // its transaction auto-rolls back.

        // Resolve the single source binder shared by all requested items.
        let src_groups = uow.get_binder_relationships_from_right_ids(
            &BinderRelationshipField::BinderItems,
            &dto.item_ids,
        )?;
        if src_groups.len() != 1 {
            return Err(anyhow!(
                "move_items: items span {} binders (must share one source binder)",
                src_groups.len()
            ));
        }
        let (src_binder, found_items) = src_groups.into_iter().next().unwrap();
        let found: HashSet<EntityId> = found_items.into_iter().collect();
        if dto.item_ids.iter().any(|id| !found.contains(id)) {
            return Err(anyhow!(
                "move_items: some requested items are not in the source binder"
            ));
        }

        let src_order =
            uow.get_binder_relationship(&src_binder, &BinderRelationshipField::BinderItems)?;
        let mut indent: HashMap<EntityId, i64> = HashMap::new();
        for it in uow.get_binder_item_multi(&src_order)?.into_iter().flatten() {
            indent.insert(it.id, it.indent);
        }

        // Expand requested ids to their full contiguous subtrees, in src order,
        // deduplicating nested selections.
        let requested: HashSet<EntityId> = dto.item_ids.iter().copied().collect();
        let full_move_ids = expand_to_subtrees(&src_order, &indent, &requested);
        if full_move_ids.is_empty() {
            return Err(anyhow!("move_items: nothing to move"));
        }
        let move_set: HashSet<EntityId> = full_move_ids.iter().copied().collect();
        let root_old_indent = *indent.get(&full_move_ids[0]).unwrap_or(&0);
        // Resolve destination binder, the new base indent for the moved root,
        // and the anchor id to insert before (None = append at end).
        let (dest_binder, base_indent, anchor_id): (EntityId, i64, Option<EntityId>) = if dto
            .target_is_binder
        {
            if uow.get_binder(&target_id)?.is_none() {
                return Err(anyhow!("move_items: target binder {target_id} not found"));
            }
            let dest_order =
                uow.get_binder_relationship(&target_id, &BinderRelationshipField::BinderItems)?;
            let anchor =
                anchor_for_binder_target(&dest_order, to_drop_place(&dto.move_place), &move_set);
            (target_id, 0, anchor)
        } else {
            if move_set.contains(&target_id) {
                return Err(anyhow!("move_items: cannot move a subtree into itself"));
            }
            let target_item = uow
                .get_binder_item(&target_id)?
                .ok_or_else(|| anyhow!("move_items: target item {target_id} not found"))?;
            let dest_binder = uow
                .get_binder_relationships_from_right_ids(
                    &BinderRelationshipField::BinderItems,
                    &[target_id],
                )?
                .into_iter()
                .next()
                .map(|(b, _)| b)
                .ok_or_else(|| anyhow!("move_items: target item has no binder"))?;
            let dest_order = if dest_binder == src_binder {
                src_order.clone()
            } else {
                let order = uow
                    .get_binder_relationship(&dest_binder, &BinderRelationshipField::BinderItems)?;
                for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
                    indent.insert(it.id, it.indent);
                }
                order
            };
            let target_is_folder = target_item.role == BinderItemRole::Folder;
            let (base_indent, anchor) = resolve_item_target(
                &dest_order,
                &indent,
                target_id,
                target_item.indent,
                target_is_folder,
                to_drop_place(&dto.move_place),
                &move_set,
            )?;

            // **A Book inside a Book is allowed.** There is nothing to resolve: a
            // book runs from its own marker to the next one, so a Book row nested by
            // indent inside another simply *starts* the next book, exactly as it would
            // at the top level. Indent is a fact about how the writer arranged the
            // tree, not about where a book ends, and the compiler never consults it.
            //
            // This used to refuse, on the reasoning that "once the compiler walks past
            // the inner book's subtree, every row still part of the outer book by
            // indent gets folded into the inner book's total". That describes the flat
            // fold correctly and then calls it a fault: those rows *are* the inner
            // book's, and the only thing that made it look wrong was reading the tree
            // as containment. Nesting one book inside another reads badly, and it is
            // the writer's business whether to.

            (dest_binder, base_indent, anchor)
        };

        let delta = base_indent - root_old_indent;

        // Scoped snapshot of the affected binder subtree(s), taken now (after the
        // read-only resolution, before the first mutation below).
        let mut roots = vec![src_binder, dest_binder];
        roots.sort();
        roots.dedup();
        let snap_before = uow.snapshot_binder(&roots)?;

        // Apply the reorder (relationship vec) — same- and cross-binder.
        let filtered_src: Vec<EntityId> = src_order
            .iter()
            .copied()
            .filter(|id| !move_set.contains(id))
            .collect();
        if dest_binder == src_binder {
            let final_order = insert_block(&filtered_src, &full_move_ids, anchor_id);
            uow.set_binder_relationship(
                &src_binder,
                &BinderRelationshipField::BinderItems,
                &final_order,
            )?;
        } else {
            uow.set_binder_relationship(
                &src_binder,
                &BinderRelationshipField::BinderItems,
                &filtered_src,
            )?;
            let dest_order =
                uow.get_binder_relationship(&dest_binder, &BinderRelationshipField::BinderItems)?;
            let final_dest = insert_block(&dest_order, &full_move_ids, anchor_id);
            uow.set_binder_relationship(
                &dest_binder,
                &BinderRelationshipField::BinderItems,
                &final_dest,
            )?;
        }

        // Reindent the moved subtree (preserving its internal relative shape).
        if delta != 0 {
            let mut updated: Vec<BinderItem> = Vec::new();
            for it in uow
                .get_binder_item_multi(&full_move_ids)?
                .into_iter()
                .flatten()
            {
                let mut it = it;
                it.indent += delta;
                updated.push(it);
            }
            uow.update_binder_item_multi(&updated)?;
        }

        let snap_after = uow.snapshot_binder(&roots)?;
        uow.commit()?;
        uow.publish_move_items_event(full_move_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for MoveItemsUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("move_items: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("move_items: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
