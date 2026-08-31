// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared purge core for `empty_trash` (purge ALL current trash entries) and
//! `delete_trash_entries` (purge a caller-chosen subset). A plain shared module,
//! not a use case — neither use case calls the other.
//!
//! The two use cases hold different generated unit-of-work trait objects
//! (`EmptyTrashUnitOfWorkTrait` / `DeleteTrashEntriesUnitOfWorkTrait`) with an
//! identical method set. [`PurgeAccess`] is a thin forwarding trait implemented
//! for each `dyn`, so [`plan_purge`]/[`apply_purge`] can be generic over either.

use anyhow::Result;
use binder_ordering::subtree_of;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::BinderItem;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

use crate::use_cases::delete_trash_entries_uc::DeleteTrashEntriesUnitOfWorkTrait;
use crate::use_cases::empty_trash_uc::EmptyTrashUnitOfWorkTrait;

/// The UoW surface both purge callers expose identically — their generated
/// `#[macros::uow_action]` sets must cover every method here.
pub(crate) trait PurgeAccess {
    fn get_work_relationship(
        &self,
        id: &EntityId,
        field: &WorkRelationshipField,
    ) -> Result<Vec<EntityId>>;
    fn set_work_relationship(
        &self,
        id: &EntityId,
        field: &WorkRelationshipField,
        ids: &[EntityId],
    ) -> Result<()>;
    fn get_trash_info_relationship(
        &self,
        id: &EntityId,
        field: &TrashInfoRelationshipField,
    ) -> Result<Vec<EntityId>>;
    fn get_binder_relationship(
        &self,
        id: &EntityId,
        field: &BinderRelationshipField,
    ) -> Result<Vec<EntityId>>;
    fn set_binder_relationship(
        &self,
        id: &EntityId,
        field: &BinderRelationshipField,
        ids: &[EntityId],
    ) -> Result<()>;
    fn get_binder_relationships_from_right_ids(
        &self,
        field: &BinderRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>>;
    fn get_binder_item_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderItem>>>;
    fn get_binder_item_relationship(
        &self,
        id: &EntityId,
        field: &BinderItemRelationshipField,
    ) -> Result<Vec<EntityId>>;
    fn remove_content_multi(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_binder_item_multi(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_binder_multi(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_trash_info_multi(&self, ids: &[EntityId]) -> Result<()>;
}

/// Forward every `PurgeAccess` method to the same-named method on the generated
/// unit-of-work trait `$t` (each use case's `dyn` uow already implements it).
macro_rules! forward_purge_access {
    ($t:path) => {
        impl PurgeAccess for dyn $t {
            fn get_work_relationship(
                &self,
                id: &EntityId,
                field: &WorkRelationshipField,
            ) -> Result<Vec<EntityId>> {
                <Self as $t>::get_work_relationship(self, id, field)
            }
            fn set_work_relationship(
                &self,
                id: &EntityId,
                field: &WorkRelationshipField,
                ids: &[EntityId],
            ) -> Result<()> {
                <Self as $t>::set_work_relationship(self, id, field, ids)
            }
            fn get_trash_info_relationship(
                &self,
                id: &EntityId,
                field: &TrashInfoRelationshipField,
            ) -> Result<Vec<EntityId>> {
                <Self as $t>::get_trash_info_relationship(self, id, field)
            }
            fn get_binder_relationship(
                &self,
                id: &EntityId,
                field: &BinderRelationshipField,
            ) -> Result<Vec<EntityId>> {
                <Self as $t>::get_binder_relationship(self, id, field)
            }
            fn set_binder_relationship(
                &self,
                id: &EntityId,
                field: &BinderRelationshipField,
                ids: &[EntityId],
            ) -> Result<()> {
                <Self as $t>::set_binder_relationship(self, id, field, ids)
            }
            fn get_binder_relationships_from_right_ids(
                &self,
                field: &BinderRelationshipField,
                right_ids: &[EntityId],
            ) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
                <Self as $t>::get_binder_relationships_from_right_ids(self, field, right_ids)
            }
            fn get_binder_item_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderItem>>> {
                <Self as $t>::get_binder_item_multi(self, ids)
            }
            fn get_binder_item_relationship(
                &self,
                id: &EntityId,
                field: &BinderItemRelationshipField,
            ) -> Result<Vec<EntityId>> {
                <Self as $t>::get_binder_item_relationship(self, id, field)
            }
            fn remove_content_multi(&self, ids: &[EntityId]) -> Result<()> {
                <Self as $t>::remove_content_multi(self, ids)
            }
            fn remove_binder_item_multi(&self, ids: &[EntityId]) -> Result<()> {
                <Self as $t>::remove_binder_item_multi(self, ids)
            }
            fn remove_binder_multi(&self, ids: &[EntityId]) -> Result<()> {
                <Self as $t>::remove_binder_multi(self, ids)
            }
            fn remove_trash_info_multi(&self, ids: &[EntityId]) -> Result<()> {
                <Self as $t>::remove_trash_info_multi(self, ids)
            }
        }
    };
}

forward_purge_access!(EmptyTrashUnitOfWorkTrait);
forward_purge_access!(DeleteTrashEntriesUnitOfWorkTrait);

/// A resolved purge plan over exactly the given TrashInfo ids.
pub(crate) struct PurgePlan {
    pub remove_contents: Vec<EntityId>,
    pub remove_items: Vec<EntityId>,
    pub remove_binders: Vec<EntityId>,
    /// Surviving binders that lose a trashed item subtree from their order.
    pub drop_from_binder: HashMap<EntityId, Vec<EntityId>>,
    /// Other still-indexed TrashInfos whose target is swept up as collateral
    /// (its binder/item is inside `remove_binders`/`remove_items`). Consumed
    /// alongside the requested ids so no dangling trash row is left behind.
    pub collateral: Vec<EntityId>,
}

/// Read-only planning pass over exactly `trash_info_ids` (never implicitly "all
/// of Work.trash_infos" — the caller decides the subset), plus the collateral
/// scan over the rest of the index.
pub(crate) fn plan_purge<U: PurgeAccess + ?Sized>(
    uow: &U,
    work_id: EntityId,
    trash_info_ids: &[EntityId],
) -> Result<PurgePlan> {
    let mut remove_contents: Vec<EntityId> = Vec::new();
    let mut remove_items: Vec<EntityId> = Vec::new();
    let mut remove_binders: Vec<EntityId> = Vec::new();
    let mut drop_from_binder: HashMap<EntityId, Vec<EntityId>> = HashMap::new();

    for info in trash_info_ids {
        let trashed_binder = uow
            .get_trash_info_relationship(info, &TrashInfoRelationshipField::TrashedBinder)?
            .into_iter()
            .next();
        let trashed_item = uow
            .get_trash_info_relationship(info, &TrashInfoRelationshipField::TrashedBinderItem)?
            .into_iter()
            .next();

        if let Some(binder_id) = trashed_binder {
            let items =
                uow.get_binder_relationship(&binder_id, &BinderRelationshipField::BinderItems)?;
            for it in &items {
                let cs =
                    uow.get_binder_item_relationship(it, &BinderItemRelationshipField::Contents)?;
                remove_contents.extend(cs);
            }
            remove_items.extend(items);
            remove_binders.push(binder_id);
        } else if let Some(item_id) = trashed_item
            && let Some((binder_id, _)) = uow
                .get_binder_relationships_from_right_ids(
                    &BinderRelationshipField::BinderItems,
                    &[item_id],
                )?
                .into_iter()
                .next()
        {
            let order =
                uow.get_binder_relationship(&binder_id, &BinderRelationshipField::BinderItems)?;
            let mut indent: HashMap<EntityId, i64> = HashMap::new();
            for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
                indent.insert(it.id, it.indent);
            }
            let subtree = subtree_of(&order, &indent, item_id);
            for it in &subtree {
                let cs =
                    uow.get_binder_item_relationship(it, &BinderItemRelationshipField::Contents)?;
                remove_contents.extend(cs);
            }
            remove_items.extend(subtree.iter().copied());
            drop_from_binder
                .entry(binder_id)
                .or_default()
                .extend(subtree);
        }
    }

    // Collateral scan: any OTHER still-indexed TrashInfo whose target falls
    // inside what we're about to hard-remove would otherwise dangle. A subset
    // purge can hit this; the all-purge (empty_trash) never does.
    let removed_binders: HashSet<EntityId> = remove_binders.iter().copied().collect();
    let removed_items: HashSet<EntityId> = remove_items.iter().copied().collect();
    let requested: HashSet<EntityId> = trash_info_ids.iter().copied().collect();
    let mut collateral: Vec<EntityId> = Vec::new();
    for info in uow.get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)? {
        if requested.contains(&info) {
            continue;
        }
        let tb = uow
            .get_trash_info_relationship(&info, &TrashInfoRelationshipField::TrashedBinder)?
            .into_iter()
            .next();
        let ti = uow
            .get_trash_info_relationship(&info, &TrashInfoRelationshipField::TrashedBinderItem)?
            .into_iter()
            .next();
        let doomed = match (tb, ti) {
            (Some(b), _) => removed_binders.contains(&b),
            (None, Some(i)) => removed_items.contains(&i),
            (None, None) => false,
        };
        if doomed {
            collateral.push(info);
        }
    }

    Ok(PurgePlan {
        remove_contents,
        remove_items,
        remove_binders,
        drop_from_binder,
        collateral,
    })
}

/// Apply a previously-computed plan: prune subtrees from surviving binders' order,
/// drop removed binders from their Works, hard-remove content → items → binders,
/// remove the requested + collateral TrashInfos, and unlink them all from
/// `Work.trash_infos`. Returns `remove_items` (for the caller's event payload).
pub(crate) fn apply_purge<U: PurgeAccess + ?Sized>(
    uow: &U,
    work_id: EntityId,
    trash_info_ids: &[EntityId],
    plan: &PurgePlan,
) -> Result<Vec<EntityId>> {
    // Drop removed item subtrees from their surviving binders' order.
    for (binder_id, dropped) in &plan.drop_from_binder {
        let dropped_set: HashSet<EntityId> = dropped.iter().copied().collect();
        let order =
            uow.get_binder_relationship(binder_id, &BinderRelationshipField::BinderItems)?;
        let new: Vec<EntityId> = order
            .into_iter()
            .filter(|id| !dropped_set.contains(id))
            .collect();
        uow.set_binder_relationship(binder_id, &BinderRelationshipField::BinderItems, &new)?;
    }

    // Drop removed binders from their Work. A trashed Binder always belongs to
    // the same Work whose trash is being purged (TrashInfo lives in the Work
    // trunk), so this can go straight to `work_id`.
    if !plan.remove_binders.is_empty() {
        let rb: HashSet<EntityId> = plan.remove_binders.iter().copied().collect();
        let binders = uow.get_work_relationship(&work_id, &WorkRelationshipField::Binders)?;
        let new: Vec<EntityId> = binders.into_iter().filter(|b| !rb.contains(b)).collect();
        uow.set_work_relationship(&work_id, &WorkRelationshipField::Binders, &new)?;
    }

    // Hard-remove entities (contents → items → binders).
    if !plan.remove_contents.is_empty() {
        uow.remove_content_multi(&plan.remove_contents)?;
    }
    if !plan.remove_items.is_empty() {
        uow.remove_binder_item_multi(&plan.remove_items)?;
    }
    if !plan.remove_binders.is_empty() {
        uow.remove_binder_multi(&plan.remove_binders)?;
    }

    // Remove the requested + collateral TrashInfos and unlink them from the index.
    let mut removed_infos: Vec<EntityId> = trash_info_ids.to_vec();
    removed_infos.extend(plan.collateral.iter().copied());
    if !removed_infos.is_empty() {
        uow.remove_trash_info_multi(&removed_infos)?;
    }
    let removed_set: HashSet<EntityId> = removed_infos.iter().copied().collect();
    let remaining: Vec<EntityId> = uow
        .get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?
        .into_iter()
        .filter(|id| !removed_set.contains(id))
        .collect();
    uow.set_work_relationship(&work_id, &WorkRelationshipField::TrashInfos, &remaining)?;

    Ok(plan.remove_items.clone())
}
