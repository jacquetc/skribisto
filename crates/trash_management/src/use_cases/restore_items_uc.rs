// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: restore trashed entities indexed by the given
// TrashInfos. A TrashedBinder reactivates the binder and all its items; a
// TrashedBinderItem reactivates the item and its contiguous subtree in place.
// An item is reported `orphaned` — left indexed, for the caller to re-home
// through restore_items_to — when its binder no longer exists, or when the row
// it was nested in is no longer a Folder (its container was collapsed to a leaf
// while it sat in the trash).
// Restored TrashInfos are unlinked from Work.trash_infos.
//
// Undoable via a Work-scoped snapshot/restore.
use crate::RestoreItemsDto;
use crate::RestoreResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{Binder, BinderItem, BinderItemRole, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

pub trait RestoreItemsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn RestoreItemsUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/restore_items_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "Update")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait RestoreItemsUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_restore_items_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct RestoreItemsUseCase {
    uow_factory: Box<dyn RestoreItemsUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl RestoreItemsUseCase {
    pub fn new(uow_factory: Box<dyn RestoreItemsUnitOfWorkFactoryTrait>) -> Self {
        RestoreItemsUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &RestoreItemsDto) -> Result<RestoreResultDto> {
        if dto.trash_info_ids.is_empty() {
            return Ok(RestoreResultDto {
                restored_count: 0,
                orphaned: false,
            });
        }
        let info_ids: Vec<EntityId> = dto.trash_info_ids.iter().map(|&x| x as EntityId).collect();

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // dto.work_id must be validated against the open Works: it scopes both
        // the undo/redo snapshot and which Work's trash_infos index gets the
        // consumed ids unlinked.
        let work_id = work_id(uow.as_ref(), dto.work_id as EntityId)?;

        // Ownership check: every requested TrashInfo must be indexed under
        // THIS Work, not merely exist somewhere in the store -- `work_id()`
        // above only validates the scalar dto.work_id, not that
        // dto.trash_info_ids are this Work's own rows. Without this, an id
        // copied from a different open Work's trash bin would reactivate
        // whatever it points at while the snapshot/restore pair stays scoped
        // to `work_id`: the real owning Work's tree gets mutated with no undo
        // record.
        let indexed: HashSet<EntityId> = uow
            .get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?
            .into_iter()
            .collect();
        let foreign: Vec<EntityId> = info_ids
            .iter()
            .copied()
            .filter(|id| !indexed.contains(id))
            .collect();
        if !foreign.is_empty() {
            return Err(anyhow!(
                "restore_items: trash entries {foreign:?} do not belong to work {work_id}"
            ));
        }

        // Work-scoped snapshot, before the first mutation.
        let snap_before = uow.snapshot_work(&[work_id])?;

        let mut restored_count: i64 = 0;
        let mut orphaned = false;
        let mut consumed: HashSet<EntityId> = HashSet::new(); // TrashInfos to drop from the index
        let mut touched: Vec<EntityId> = Vec::new();

        for info_id in &info_ids {
            let trashed_binder = uow
                .get_trash_info_relationship(info_id, &TrashInfoRelationshipField::TrashedBinder)?
                .into_iter()
                .next();
            let trashed_item = uow
                .get_trash_info_relationship(
                    info_id,
                    &TrashInfoRelationshipField::TrashedBinderItem,
                )?
                .into_iter()
                .next();

            if let Some(binder_id) = trashed_binder {
                match uow.get_binder(&binder_id)? {
                    Some(mut binder) => {
                        binder.activated = true;
                        uow.update_binder(&binder)?;
                        let item_ids = uow.get_binder_relationship(
                            &binder_id,
                            &BinderRelationshipField::BinderItems,
                        )?;
                        reactivate(uow.as_ref(), &item_ids)?;
                        touched.push(binder_id);
                        touched.extend(item_ids);
                        restored_count += 1;
                        consumed.insert(*info_id);
                    }
                    None => orphaned = true,
                }
            } else if let Some(item_id) = trashed_item {
                // Resolve the item's current binder (trashed items stay in place).
                let binder = uow
                    .get_binder_relationships_from_right_ids(
                        &BinderRelationshipField::BinderItems,
                        &[item_id],
                    )?
                    .into_iter()
                    .next()
                    .map(|(b, _)| b);
                match binder {
                    Some(binder_id) => {
                        let order = uow.get_binder_relationship(
                            &binder_id,
                            &BinderRelationshipField::BinderItems,
                        )?;
                        let mut indent: HashMap<EntityId, i64> = HashMap::new();
                        let mut role: HashMap<EntityId, BinderItemRole> = HashMap::new();
                        for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
                            indent.insert(it.id, it.indent);
                            role.insert(it.id, it.role);
                        }
                        // Only a Folder may hold nested rows, and the container this item
                        // sat in can have been collapsed to a leaf while it was trashed
                        // (the demote guard counts only live children). Reactivating in
                        // place would then strand it under a leaf, so report it orphaned
                        // instead and leave its TrashInfo indexed for restore_items_to.
                        let nested_under_a_leaf = parent_of(&order, &indent, item_id)
                            .and_then(|p| role.get(&p))
                            .is_some_and(|r| *r != BinderItemRole::Folder);
                        if nested_under_a_leaf {
                            orphaned = true;
                            continue;
                        }
                        let subtree = subtree_of(&order, &indent, item_id);
                        reactivate(uow.as_ref(), &subtree)?;
                        touched.extend(subtree);
                        restored_count += 1;
                        consumed.insert(*info_id);
                    }
                    None => orphaned = true,
                }
            } else {
                // A TrashInfo with neither relationship is stale — drop it.
                consumed.insert(*info_id);
            }
        }

        // Unlink consumed TrashInfos from Work.trash_infos.
        if !consumed.is_empty() {
            let remaining: Vec<EntityId> = uow
                .get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?
                .into_iter()
                .filter(|id| !consumed.contains(id))
                .collect();
            uow.set_work_relationship(&work_id, &WorkRelationshipField::TrashInfos, &remaining)?;
        }

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_restore_items_event(touched, None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(RestoreResultDto {
            restored_count,
            orphaned,
        })
    }
}

fn reactivate(uow: &dyn RestoreItemsUnitOfWorkTrait, ids: &[EntityId]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut updated: Vec<BinderItem> = Vec::new();
    for it in uow.get_binder_item_multi(ids)?.into_iter().flatten() {
        let mut it = it;
        it.activated = true;
        updated.push(it);
    }
    if !updated.is_empty() {
        uow.update_binder_item_multi(&updated)?;
    }
    Ok(())
}

/// The row `root` is nested inside: the nearest one before it in binder order with a
/// strictly smaller indent. `None` for a top-level row — nothing shallower precedes it,
/// which is always a valid place to sit.
///
/// The model has no parent pointers; containment is encoded by position + indent alone,
/// so this walk *is* "who is my parent".
fn parent_of(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    root: EntityId,
) -> Option<EntityId> {
    let pos = order.iter().position(|&x| x == root)?;
    let root_indent = *indent.get(&root).unwrap_or(&0);
    order[..pos]
        .iter()
        .rev()
        .find(|id| *indent.get(id).unwrap_or(&0) < root_indent)
        .copied()
}

/// The contiguous subtree rooted at `root` (root plus following items whose
/// indent is strictly greater), in binder order.
fn subtree_of(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    root: EntityId,
) -> Vec<EntityId> {
    let Some(pos) = order.iter().position(|&x| x == root) else {
        return Vec::new();
    };
    let root_indent = *indent.get(&root).unwrap_or(&0);
    let mut out = vec![root];
    let mut j = pos + 1;
    while j < order.len() {
        if *indent.get(&order[j]).unwrap_or(&0) <= root_indent {
            break;
        }
        out.push(order[j]);
        j += 1;
    }
    out
}

// `get_work_relationship` doesn't validate that `id` is a real, open Work, so
// check `dto.work_id` against the open Works first (see `empty_trash_uc.rs`).
fn work_id(uow: &dyn RestoreItemsUnitOfWorkTrait, requested: EntityId) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .find(|w| w.id == requested)
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for RestoreItemsUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("restore_items: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_work(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("restore_items: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_work(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
