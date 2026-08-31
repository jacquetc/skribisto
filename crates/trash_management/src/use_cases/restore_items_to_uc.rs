// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: restore arbitrary trashed BinderItems — roots OR
// descendants of a larger trashed subtree — to a chosen destination binder and
// position. Because a descendant's ancestors are still trashed (or its whole
// binder is trashed), restoring it in place would leave it hidden, so this op
// always RELOCATES the reactivated block to an active destination.
//
// For each requested item, the currently-trashed contiguous subtree hanging off
// it is reactivated (activated = true) and moved out of its source binder into
// the destination at the resolved anchor/indent. A post-pass sweep then unlinks
// from Work.trash_infos any TrashInfo whose trashed_binder_item is now active or
// gone — which consumes a fully-restored root and LEAVES a root whose subtree
// was only partially peeled. Whole-binder (trashed_binder) TrashInfos are never
// touched.
//
// Undoable via a Work-scoped snapshot/restore (spans Work.trash_infos + the
// affected binder orders + item flags, all inside the Work trunk).
use crate::RestoreItemsToDto;
use crate::RestoreItemsToResultDto;
use crate::dtos::DropPosition;
use anyhow::{Result, anyhow};
use binder_ordering::{DropPlace, anchor_for_binder_target, insert_block, resolve_item_target};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{Binder, BinderItem, BinderItemRole, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

/// Map the feature-local `DropPosition` DTO enum onto the shared
/// `binder_ordering::DropPlace` (Qleany DTO enums can't be shared across crates).
fn to_drop_place(place: &DropPosition) -> DropPlace {
    match place {
        DropPosition::Before => DropPlace::Before,
        DropPosition::After => DropPlace::After,
        DropPosition::Into => DropPlace::Into,
    }
}

pub trait RestoreItemsToUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn RestoreItemsToUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/restore_items_to_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "Get")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait RestoreItemsToUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_restore_items_to_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct RestoreItemsToUseCase {
    uow_factory: Box<dyn RestoreItemsToUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

/// A resolved restore target: the requested item, its source binder (None = a
/// dangling orphan linked to no binder), the reactivatable trashed subtree, and
/// the root's current indent (for the reindent delta).
struct Plan {
    src_binder: Option<EntityId>,
    subtree: Vec<EntityId>,
    root_old_indent: i64,
}

impl RestoreItemsToUseCase {
    pub fn new(uow_factory: Box<dyn RestoreItemsToUnitOfWorkFactoryTrait>) -> Self {
        RestoreItemsToUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &RestoreItemsToDto) -> Result<RestoreItemsToResultDto> {
        if dto.binder_item_ids.is_empty() {
            return Ok(RestoreItemsToResultDto {
                restored_count: 0,
                orphaned: false,
            });
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // dto.work_id must be validated against the open Works: it scopes
        // both the undo/redo snapshot and the post-pass sweep of
        // Work.trash_infos.
        let work_id = work_id(uow.as_ref(), dto.work_id as EntityId)?;

        // --- destination resolution (read-only) ---
        let dest_binder_id = dto.destination_binder_id;
        let dest_binder = uow.get_binder(&dest_binder_id)?.ok_or_else(|| {
            anyhow!("restore_items_to: destination binder {dest_binder_id} not found")
        })?;
        if !dest_binder.activated {
            return Err(anyhow!(
                "restore_items_to: destination binder {dest_binder_id} is itself trashed"
            ));
        }
        let dest_order =
            uow.get_binder_relationship(&dest_binder_id, &BinderRelationshipField::BinderItems)?;
        let mut dest_indent: HashMap<EntityId, i64> = HashMap::new();
        for it in uow
            .get_binder_item_multi(&dest_order)?
            .into_iter()
            .flatten()
        {
            dest_indent.insert(it.id, it.indent);
        }

        // --- per-input resolution: gather each item's currently-trashed subtree.
        // A descendant already covered by an earlier peel in this batch is skipped.
        let mut orphaned = false;
        let mut plan: Vec<Plan> = Vec::new();
        let mut all_moving: HashSet<EntityId> = HashSet::new();

        for &item_id in &dto.binder_item_ids {
            if all_moving.contains(&item_id) {
                continue;
            }
            let item = match uow.get_binder_item(&item_id)? {
                Some(it) => it,
                None => {
                    orphaned = true; // entity gone entirely
                    continue;
                }
            };
            if item.activated {
                orphaned = true; // already active — nothing to restore
                continue;
            }
            let src_binder = uow
                .get_binder_relationships_from_right_ids(
                    &BinderRelationshipField::BinderItems,
                    &[item_id],
                )?
                .into_iter()
                .next()
                .map(|(b, _)| b);
            match src_binder {
                Some(src) => {
                    let order =
                        uow.get_binder_relationship(&src, &BinderRelationshipField::BinderItems)?;
                    let mut indent: HashMap<EntityId, i64> = HashMap::new();
                    let mut activated: HashMap<EntityId, bool> = HashMap::new();
                    for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
                        indent.insert(it.id, it.indent);
                        activated.insert(it.id, it.activated);
                    }
                    let subtree = trashed_subtree_of(&order, &indent, &activated, item_id);
                    let root_old_indent = *indent.get(&item_id).unwrap_or(&0);
                    for &id in &subtree {
                        all_moving.insert(id);
                    }
                    plan.push(Plan {
                        src_binder: Some(src),
                        subtree,
                        root_old_indent,
                    });
                }
                None => {
                    // exists, deactivated, linked to no binder → recoverable as a
                    // singleton (its descendants, if any, are unrecoverable — the
                    // order that defined them is gone).
                    all_moving.insert(item_id);
                    plan.push(Plan {
                        src_binder: None,
                        subtree: vec![item_id],
                        root_old_indent: item.indent,
                    });
                }
            }
        }

        if let Some(anchor) = dto.anchor_item_id
            && all_moving.contains(&anchor)
        {
            return Err(anyhow!(
                "restore_items_to: cannot anchor onto the subtree being restored"
            ));
        }

        // Ownership check: every binder this op touches -- each item's source
        // binder (resolved above from the item itself, with no reference to
        // `work_id`) and the destination -- must belong to the named Work.
        // Every check so far cross-references binder/item ids only against
        // each other, never against the Work, so without this a caller
        // handing in Work B's ids under Work A's work_id would move Work B's
        // rows while the undo/redo snapshot stayed scoped to Work A.
        let owned_binders: HashSet<EntityId> = uow
            .get_work_relationship(&work_id, &WorkRelationshipField::Binders)?
            .into_iter()
            .collect();
        let mut touched_binders: HashSet<EntityId> =
            plan.iter().filter_map(|p| p.src_binder).collect();
        touched_binders.insert(dest_binder_id);
        let foreign: Vec<EntityId> = touched_binders
            .iter()
            .copied()
            .filter(|id| !owned_binders.contains(id))
            .collect();
        if !foreign.is_empty() {
            return Err(anyhow!(
                "restore_items_to: binder(s) {foreign:?} do not belong to work {work_id}"
            ));
        }

        // Snapshot the Work trunk before any mutation. (Taken unconditionally so
        // undo/redo work even when nothing was recoverable.)
        let snap_before = uow.snapshot_work(&[work_id])?;

        // Resolve the destination anchor + base indent ONCE, shared across the
        // batch; repeated insert against the same anchor stacks the subtrees in
        // dto order.
        let drop = to_drop_place(&dto.drop_position);
        let (base_indent, anchor_id): (i64, Option<EntityId>) = match dto.anchor_item_id {
            None => (0, anchor_for_binder_target(&dest_order, drop, &all_moving)),
            Some(a) => {
                let anchor_item = uow
                    .get_binder_item(&a)?
                    .ok_or_else(|| anyhow!("restore_items_to: anchor item {a} not found"))?;
                if !dest_order.contains(&a) {
                    return Err(anyhow!(
                        "restore_items_to: anchor item {a} not in destination binder"
                    ));
                }
                let is_folder = anchor_item.role == BinderItemRole::Folder;
                let (base_indent, anchor) = resolve_item_target(
                    &dest_order,
                    &dest_indent,
                    a,
                    anchor_item.indent,
                    is_folder,
                    drop,
                    &all_moving,
                )?;

                // **A Book restored inside another Book is allowed**, the same as moving
                // one there: a book runs from its own marker to the next, so a Book row
                // nested by indent starts the next book rather than being contained by
                // anything. See `binder_item_management::move_items_uc`, where the
                // matching guard and its reasoning were removed first.

                (base_indent, anchor)
            }
        };

        // --- mutate ---
        let mut restored_count: i64 = 0;
        let mut touched: Vec<EntityId> = Vec::new();

        // 1) Reactivate + reindent every moving subtree (one write).
        let mut updated: Vec<BinderItem> = Vec::new();
        for p in &plan {
            let delta = base_indent - p.root_old_indent;
            for it in uow.get_binder_item_multi(&p.subtree)?.into_iter().flatten() {
                let mut it = it;
                it.activated = true;
                it.indent += delta;
                updated.push(it);
            }
            touched.extend(p.subtree.iter().copied());
            restored_count += 1;
        }
        if !updated.is_empty() {
            uow.update_binder_item_multi(&updated)?;
        }

        // 2) The moving block (all subtrees, in plan order).
        let block: Vec<EntityId> = plan
            .iter()
            .flat_map(|p| p.subtree.iter().copied())
            .collect();
        let moving_set = &all_moving;

        // 3) Remove the moving ids from every SOURCE binder (except the
        // destination, rebuilt in step 4).
        let src_binders: HashSet<EntityId> = plan.iter().filter_map(|p| p.src_binder).collect();
        for &src in &src_binders {
            if src == dest_binder_id {
                continue;
            }
            let order = uow.get_binder_relationship(&src, &BinderRelationshipField::BinderItems)?;
            let new_order: Vec<EntityId> = order
                .into_iter()
                .filter(|id| !moving_set.contains(id))
                .collect();
            uow.set_binder_relationship(&src, &BinderRelationshipField::BinderItems, &new_order)?;
        }

        // 4) Destination binder: drop any moving ids it already held (self-move)
        // then splice the block in at the anchor.
        let base_dest: Vec<EntityId> = dest_order
            .into_iter()
            .filter(|id| !moving_set.contains(id))
            .collect();
        let final_dest = insert_block(&base_dest, &block, anchor_id);
        uow.set_binder_relationship(
            &dest_binder_id,
            &BinderRelationshipField::BinderItems,
            &final_dest,
        )?;

        // 5) Post-pass sweep: consume any TrashInfo whose item became active or
        // vanished; keep partially-peeled roots and every whole-binder entry.
        let mut consumed: HashSet<EntityId> = HashSet::new();
        let trash_infos =
            uow.get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?;
        for info_id in &trash_infos {
            if uow
                .get_trash_info_relationship(info_id, &TrashInfoRelationshipField::TrashedBinder)?
                .into_iter()
                .next()
                .is_some()
            {
                continue; // whole-binder entry — never touched by this op
            }
            match uow
                .get_trash_info_relationship(
                    info_id,
                    &TrashInfoRelationshipField::TrashedBinderItem,
                )?
                .into_iter()
                .next()
            {
                None => {
                    consumed.insert(*info_id); // stale (neither relationship set)
                }
                Some(item_id) => match uow.get_binder_item(&item_id)? {
                    None => {
                        consumed.insert(*info_id); // entity gone
                    }
                    Some(it) if it.activated => {
                        consumed.insert(*info_id); // fully restored
                    }
                    Some(_) => {} // still trashed → keep
                },
            }
        }
        if !consumed.is_empty() {
            let remaining: Vec<EntityId> = trash_infos
                .into_iter()
                .filter(|id| !consumed.contains(id))
                .collect();
            uow.set_work_relationship(&work_id, &WorkRelationshipField::TrashInfos, &remaining)?;
        }

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_restore_items_to_event(touched, None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(RestoreItemsToResultDto {
            restored_count,
            orphaned,
        })
    }
}

/// The contiguous CURRENTLY-TRASHED subtree rooted at `root`: `root` plus every
/// following item whose indent is strictly greater AND whose `activated` is
/// still false — stopping at the first item at/above the root's indent, or one
/// already reactivated. Distinct from `binder_ordering::subtree_of` because it's
/// the one place ordering logic needs the `activated` flag (peeling a still-
/// trashed run out of a larger trashed subtree).
fn trashed_subtree_of(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    activated: &HashMap<EntityId, bool>,
    root: EntityId,
) -> Vec<EntityId> {
    let Some(pos) = order.iter().position(|&x| x == root) else {
        return Vec::new();
    };
    let root_indent = *indent.get(&root).unwrap_or(&0);
    let mut out = vec![root];
    let mut j = pos + 1;
    while j < order.len() {
        let id = order[j];
        if *indent.get(&id).unwrap_or(&0) <= root_indent {
            break;
        }
        if *activated.get(&id).unwrap_or(&true) {
            break;
        }
        out.push(id);
        j += 1;
    }
    out
}

// `get_work_relationship` doesn't validate that `id` is a real, open Work, so
// check `dto.work_id` against the open Works first (see `empty_trash_uc.rs`).
fn work_id(uow: &dyn RestoreItemsToUnitOfWorkTrait, requested: EntityId) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .find(|w| w.id == requested)
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for RestoreItemsToUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("restore_items_to: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("restore_items_to: nothing to redo"))?;
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
