// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: restore trashed entities indexed by the given
// TrashInfos. A TrashedBinder reactivates the binder and all its items; a
// TrashedBinderItem reactivates the item and its contiguous subtree in place.
// Either way, a row that another entry still in the bin covers (a scene trashed
// before its chapter) stays trashed: it goes when that entry is purged, so it
// must not be visible.
// An item is reported `orphaned` — left indexed, for the caller to re-home
// through restore_items_to — when its binder no longer exists or is itself in
// the trash, when a row it is nested in is still in the trash, or when the row
// it was nested in is no longer a Folder (its container was collapsed to a leaf
// while it sat in the trash).
// Restored TrashInfos are unlinked from Work.trash_infos.
//
// Undoable via a Work-scoped snapshot/restore.
use crate::RestoreItemsDto;
use crate::RestoreResultDto;
use anyhow::{Result, anyhow};
use binder_ordering::{expand_to_subtrees, subtree_of};
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
    /// Read the prose a restore must not revert: see [`common::prose_keep`].
    fn keep_prose(
        &self,
        target: &common::snapshot::EntityTreeSnapshot,
        other: &common::snapshot::EntityTreeSnapshot,
    ) -> Result<common::prose_keep::KeptProse>;
    /// Write that prose back after the restore, in the same transaction.
    fn put_back_prose(&self, kept: common::prose_keep::KeptProse) -> Result<()>;

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
        // An entry listed twice is restored once. Counted twice, it would report two
        // restores for one subtree, and reactivate that subtree a second time.
        let mut seen: HashSet<EntityId> = HashSet::new();
        let info_ids: Vec<EntityId> = dto
            .trash_info_ids
            .iter()
            .map(|&x| x as EntityId)
            .filter(|id| seen.insert(*id))
            .collect();

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

        // What each requested entry points at. Read before any row changes.
        let mut targets: Vec<(EntityId, Target)> = Vec::with_capacity(info_ids.len());
        for info_id in &info_ids {
            targets.push((*info_id, target_of(uow.as_ref(), *info_id)?));
        }
        // Whole binders first, then items, each item after its ancestors: a row whose
        // container is restored in the same call must find that container live.
        targets.sort_by_key(|(_, t)| match t {
            Target::Binder(_) => (0, 0, 0),
            Target::Item {
                binder: Some((binder, pos)),
                ..
            } => (1, *binder, *pos),
            Target::Item { binder: None, .. } => (2, 0, 0),
            Target::Stale => (3, 0, 0),
        });

        // The entries that stay in the bin. A row one of them covers is still in the
        // trash, whatever else this call brings back: reactivating it would leave a
        // visible row with a trash entry, and the next "Empty trash" would delete it.
        let requested: HashSet<EntityId> = info_ids.iter().copied().collect();
        let mut staying_items: HashSet<EntityId> = HashSet::new();
        for info in indexed.iter().filter(|id| !requested.contains(id)) {
            if let Some(item) = uow
                .get_trash_info_relationship(info, &TrashInfoRelationshipField::TrashedBinderItem)?
                .into_iter()
                .next()
            {
                staying_items.insert(item);
            }
        }

        let mut restored_count: i64 = 0;
        let mut orphaned = false;
        let mut consumed: HashSet<EntityId> = HashSet::new(); // TrashInfos to drop from the index
        let mut touched: Vec<EntityId> = Vec::new();

        for (info_id, target) in &targets {
            match *target {
                Target::Binder(binder_id) => match uow.get_binder(&binder_id)? {
                    Some(mut binder) => {
                        binder.activated = true;
                        uow.update_binder(&binder)?;
                        let rows = Rows::read(uow.as_ref(), binder_id)?;
                        let back = rows.not_covered(&rows.order, &staying_items);
                        reactivate(uow.as_ref(), &back)?;
                        touched.push(binder_id);
                        touched.extend(back);
                        restored_count += 1;
                        consumed.insert(*info_id);
                    }
                    None => orphaned = true,
                },
                Target::Item {
                    item: item_id,
                    binder: Some((binder_id, _)),
                } => {
                    // A binder still in the trash hides the row whatever its flag says,
                    // and purging that binder takes the row with it. Reactivating in place
                    // would only look like a restore, so report it orphaned and leave its
                    // TrashInfo indexed for restore_items_to.
                    let binder_live = uow
                        .get_binder(&binder_id)?
                        .is_some_and(|binder| binder.activated);
                    let rows = Rows::read(uow.as_ref(), binder_id)?;
                    let ancestors = ancestors_of(&rows.order, &rows.indent, item_id);
                    // Only a Folder may hold nested rows, and the container this item
                    // sat in can have been collapsed to a leaf while it was trashed
                    // (the demote guard counts only live children). Reactivating in
                    // place would then strand it under a leaf.
                    let nested_under_a_leaf = ancestors
                        .first()
                        .and_then(|p| rows.role.get(p))
                        .is_some_and(|r| *r != BinderItemRole::Folder);
                    // A container still in the trash: the row would come back inside a
                    // hidden chapter, and go when that chapter is purged.
                    let inside_a_trashed_row = ancestors
                        .iter()
                        .any(|a| !*rows.activated.get(a).unwrap_or(&true));
                    if !binder_live || nested_under_a_leaf || inside_a_trashed_row {
                        orphaned = true;
                        continue;
                    }
                    let subtree = subtree_of(&rows.order, &rows.indent, item_id);
                    let back = rows.not_covered(&subtree, &staying_items);
                    reactivate(uow.as_ref(), &back)?;
                    touched.extend(back);
                    restored_count += 1;
                    consumed.insert(*info_id);
                }
                Target::Item { binder: None, .. } => orphaned = true,
                // A TrashInfo with neither relationship is stale: drop it.
                Target::Stale => {
                    consumed.insert(*info_id);
                }
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

/// What one TrashInfo points at.
enum Target {
    /// A whole binder.
    Binder(EntityId),
    /// One item and, when some binder still lists it, that binder and the item's
    /// position in it.
    Item {
        item: EntityId,
        binder: Option<(EntityId, usize)>,
    },
    /// Neither relationship is set.
    Stale,
}

fn target_of(uow: &dyn RestoreItemsUnitOfWorkTrait, info: EntityId) -> Result<Target> {
    if let Some(binder) = uow
        .get_trash_info_relationship(&info, &TrashInfoRelationshipField::TrashedBinder)?
        .into_iter()
        .next()
    {
        return Ok(Target::Binder(binder));
    }
    let Some(item) = uow
        .get_trash_info_relationship(&info, &TrashInfoRelationshipField::TrashedBinderItem)?
        .into_iter()
        .next()
    else {
        return Ok(Target::Stale);
    };
    // Trashed items stay in place, so the binder listing the item is the one it
    // comes back into.
    let Some((binder, _)) = uow
        .get_binder_relationships_from_right_ids(&BinderRelationshipField::BinderItems, &[item])?
        .into_iter()
        .next()
    else {
        return Ok(Target::Item { item, binder: None });
    };
    let order = uow.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)?;
    let pos = order
        .iter()
        .position(|&id| id == item)
        .unwrap_or(order.len());
    Ok(Target::Item {
        item,
        binder: Some((binder, pos)),
    })
}

/// One binder's rows as they are now: order, depth, role and trashed flag.
struct Rows {
    order: Vec<EntityId>,
    indent: HashMap<EntityId, i64>,
    role: HashMap<EntityId, BinderItemRole>,
    activated: HashMap<EntityId, bool>,
}

impl Rows {
    fn read(uow: &dyn RestoreItemsUnitOfWorkTrait, binder: EntityId) -> Result<Self> {
        let order = uow.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)?;
        let mut indent = HashMap::new();
        let mut role = HashMap::new();
        let mut activated = HashMap::new();
        for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
            indent.insert(it.id, it.indent);
            role.insert(it.id, it.role);
            activated.insert(it.id, it.activated);
        }
        Ok(Rows {
            order,
            indent,
            role,
            activated,
        })
    }

    /// `ids` minus every row inside the subtree of a `staying` root: those are still
    /// in the trash under their own entry.
    fn not_covered(&self, ids: &[EntityId], staying: &HashSet<EntityId>) -> Vec<EntityId> {
        let roots: HashSet<EntityId> = self
            .order
            .iter()
            .copied()
            .filter(|id| staying.contains(id))
            .collect();
        if roots.is_empty() {
            return ids.to_vec();
        }
        let covered: HashSet<EntityId> = expand_to_subtrees(&self.order, &self.indent, &roots)
            .into_iter()
            .collect();
        ids.iter()
            .copied()
            .filter(|id| !covered.contains(id))
            .collect()
    }
}

/// The rows `root` is nested inside, nearest first: walking back in binder order,
/// each row shallower than every row seen so far.
///
/// The model has no parent pointers; containment is encoded by position + indent alone,
/// so this walk *is* "who are my parents". Empty for a top-level row.
fn ancestors_of(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    root: EntityId,
) -> Vec<EntityId> {
    let Some(pos) = order.iter().position(|&x| x == root) else {
        return Vec::new();
    };
    let mut depth = *indent.get(&root).unwrap_or(&0);
    let mut out = Vec::new();
    for id in order[..pos].iter().rev() {
        let d = *indent.get(id).unwrap_or(&0);
        if d < depth {
            out.push(*id);
            depth = d;
        }
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
        let other = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("restore_items: the other end of this command is missing"))?;
        let kept = uow.keep_prose(snap, other)?;
        uow.restore_work(snap)?;
        uow.put_back_prose(kept)?;
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
        let other = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("restore_items: the other end of this command is missing"))?;
        let kept = uow.keep_prose(snap, other)?;
        uow.restore_work(snap)?;
        uow.put_back_prose(kept)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
