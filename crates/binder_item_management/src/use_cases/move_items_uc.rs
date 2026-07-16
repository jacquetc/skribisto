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
// order (never raw entity-scan order).
use crate::MoveDto;
use crate::MovePlace;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::{Binder, BinderItem, BinderItemRole};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

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
        let mut full_move_ids: Vec<EntityId> = Vec::new();
        let mut move_set: HashSet<EntityId> = HashSet::new();
        let mut i = 0usize;
        while i < src_order.len() {
            let id = src_order[i];
            if requested.contains(&id) && !move_set.contains(&id) {
                let root_indent = *indent.get(&id).unwrap_or(&0);
                let mut j = i;
                loop {
                    let cur = src_order[j];
                    full_move_ids.push(cur);
                    move_set.insert(cur);
                    j += 1;
                    if j >= src_order.len() {
                        break;
                    }
                    if *indent.get(&src_order[j]).unwrap_or(&0) <= root_indent {
                        break;
                    }
                }
                i = j;
            } else {
                i += 1;
            }
        }
        if full_move_ids.is_empty() {
            return Err(anyhow!("move_items: nothing to move"));
        }
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
            // Into / Before a binder = top of the list; After = bottom.
            let anchor = match dto.move_place {
                MovePlace::After => None,
                _ => dest_order.iter().copied().find(|id| !move_set.contains(id)),
            };
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
            let target_pos = dest_order
                .iter()
                .position(|&x| x == target_id)
                .ok_or_else(|| anyhow!("move_items: target not found in its binder order"))?;
            let target_indent = target_item.indent;
            let into_folder = matches!(dto.move_place, MovePlace::Into)
                && target_item.role == BinderItemRole::Folder;
            // Into a leaf item is meaningless → fall back to After it.
            let effective = match dto.move_place.clone() {
                MovePlace::Into if !into_folder => MovePlace::After,
                other => other,
            };
            let base_indent = if into_folder {
                target_indent + 1
            } else {
                target_indent
            };
            let idx = match effective {
                MovePlace::Before => target_pos,
                MovePlace::After | MovePlace::Into => {
                    subtree_end(&dest_order, &indent, target_pos, target_indent)
                }
            };
            let anchor = dest_order[idx..]
                .iter()
                .copied()
                .find(|id| !move_set.contains(id));
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

/// First index `j > pos` whose item indent is `<= base_indent`, or `len` — i.e.
/// the end (exclusive) of the subtree rooted at `pos`.
fn subtree_end(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    pos: usize,
    base_indent: i64,
) -> usize {
    let mut j = pos + 1;
    while j < order.len() {
        if *indent.get(&order[j]).unwrap_or(&0) <= base_indent {
            break;
        }
        j += 1;
    }
    j
}

/// Insert `block` into `base` immediately before `anchor` (or at the end when
/// `anchor` is `None`). `block` ids are assumed absent from `base`.
fn insert_block(base: &[EntityId], block: &[EntityId], anchor: Option<EntityId>) -> Vec<EntityId> {
    let mut out = Vec::with_capacity(base.len() + block.len());
    let mut inserted = false;
    for &id in base {
        if Some(id) == anchor {
            out.extend_from_slice(block);
            inserted = true;
        }
        out.push(id);
    }
    if !inserted {
        out.extend_from_slice(block);
    }
    out
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
