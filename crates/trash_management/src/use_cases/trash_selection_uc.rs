// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: trash a mixed selection -- whole binders and loose
// items, spanning any number of this Work's binders -- as ONE undo entry.
//
// `trash_binder_items` is scoped to a single origin binder, so a caller holding
// a selection had to resolve each item's owning binder, group by it, and wrap
// the resulting calls in a composite. Four view-models were each doing that
// (outline, overview, corkboard, stream), and the grouping is not cosmetic: a
// flat call with a mixed list files items under the wrong origin and restores
// them to the wrong place. So the grouping happens here and callers pass
// ungrouped ids.
//
// The rules themselves -- `activated = !trashed`, one TrashInfo per requested
// root, the open-Work and ownership checks -- live in `crate::trash_ops`, which
// this shares with `trash_binder` and `trash_binder_items`. No use case calls
// another; they call the same module.
//
// Undoable via a targeted inverse, like its two neighbours: undo reactivates
// exactly the cascade this trashed and removes exactly the TrashInfo rows it
// created.
use crate::TrashSelectionDto;
use crate::TrashSelectionResultDto;
use crate::trash_ops::{
    self, BinderStore, ItemStore, TrashIndex, assert_work_owns, file_trash_infos, resolve_work,
    set_activated, set_binder_activated, unfile_trash_infos,
};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{Binder, BinderItem, TrashInfo, Work};
use common::types::EntityId;
use std::collections::HashSet;

pub trait TrashSelectionUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn TrashSelectionUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/trash_selection_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Binder", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "Update")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait TrashSelectionUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_trash_selection_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

impl TrashIndex for dyn TrashSelectionUnitOfWorkTrait + '_ {
    fn open_works(&self) -> Result<Vec<EntityId>> {
        Ok(self.get_all_work()?.into_iter().map(|w| w.id).collect())
    }
    fn work_binders(&self, work: EntityId) -> Result<Vec<EntityId>> {
        self.get_work_relationship(&work, &WorkRelationshipField::Binders)
    }
    fn trash_index(&self, work: EntityId) -> Result<Vec<EntityId>> {
        self.get_work_relationship(&work, &WorkRelationshipField::TrashInfos)
    }
    fn set_trash_index(&self, work: EntityId, ids: &[EntityId]) -> Result<()> {
        self.set_work_relationship(&work, &WorkRelationshipField::TrashInfos, ids)?;
        Ok(())
    }
    fn new_trash_info(&self, info: &TrashInfo) -> Result<EntityId> {
        Ok(self.create_orphan_trash_info(info)?.id)
    }
    fn link_trash_info(
        &self,
        info: EntityId,
        field: &TrashInfoRelationshipField,
        right: &[EntityId],
    ) -> Result<()> {
        self.set_trash_info_relationship(&info, field, right)?;
        Ok(())
    }
    fn drop_trash_infos(&self, ids: &[EntityId]) -> Result<()> {
        self.remove_trash_info_multi(ids)?;
        Ok(())
    }
}

impl ItemStore for dyn TrashSelectionUnitOfWorkTrait + '_ {
    fn binder_items(&self, binder: EntityId) -> Result<Vec<EntityId>> {
        self.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)
    }
    fn items(&self, ids: &[EntityId]) -> Result<Vec<BinderItem>> {
        Ok(self
            .get_binder_item_multi(ids)?
            .into_iter()
            .flatten()
            .collect())
    }
    fn save_items(&self, items: &[BinderItem]) -> Result<()> {
        self.update_binder_item_multi(items)?;
        Ok(())
    }
}

impl BinderStore for dyn TrashSelectionUnitOfWorkTrait + '_ {
    fn binder(&self, id: EntityId) -> Result<Option<Binder>> {
        self.get_binder(&id)
    }
    fn save_binder(&self, binder: &Binder) -> Result<()> {
        self.update_binder(binder)?;
        Ok(())
    }
}

pub struct TrashSelectionUseCase {
    uow_factory: Box<dyn TrashSelectionUnitOfWorkFactoryTrait>,
    work_id: EntityId,
    trashed_at: DateTime<Utc>,
    /// Whole binders, and every item inside each — trashed and reactivated together.
    binders: Vec<(EntityId, Vec<EntityId>)>,
    /// Loose item roots grouped by the binder they came from, with each group's
    /// full contiguous cascade.
    groups: Vec<ItemGroup>,
    created_trash: Vec<EntityId>,
}

struct ItemGroup {
    binder: EntityId,
    roots: Vec<EntityId>,
    cascade: Vec<EntityId>,
}

impl TrashSelectionUseCase {
    pub fn new(uow_factory: Box<dyn TrashSelectionUnitOfWorkFactoryTrait>) -> Self {
        TrashSelectionUseCase {
            uow_factory,
            work_id: 0,
            trashed_at: Utc::now(),
            binders: Vec::new(),
            groups: Vec::new(),
            created_trash: Vec::new(),
        }
    }

    pub fn execute(&mut self, dto: &TrashSelectionDto) -> Result<TrashSelectionResultDto> {
        let binder_ids: Vec<EntityId> = dedup(&dto.binder_ids);
        let item_ids: Vec<EntityId> = dedup(&dto.binder_item_ids);
        if binder_ids.is_empty() && item_ids.is_empty() {
            return Err(anyhow!("trash_selection: nothing selected"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let store: &dyn TrashSelectionUnitOfWorkTrait = uow.as_ref();

        self.work_id = resolve_work(store, dto.work_id as EntityId)?;

        // Whole binders first, so their contents are already accounted for when
        // the loose items are grouped below.
        assert_work_owns(store, self.work_id, &binder_ids)?;
        let mut binders = Vec::with_capacity(binder_ids.len());
        let mut inside_a_trashed_binder: HashSet<EntityId> = HashSet::new();
        for &binder in &binder_ids {
            if store.binder(binder)?.is_none() {
                return Err(anyhow!("trash_selection: binder {binder} not found"));
            }
            let items = store.binder_items(binder)?;
            inside_a_trashed_binder.extend(items.iter().copied());
            binders.push((binder, items));
        }

        // Group the loose items by the binder that actually lists them, rather
        // than trusting a caller to have worked it out.
        let mut groups: Vec<ItemGroup> = Vec::new();
        // A Vec, not a set: `item_ids` is already deduplicated and in the order
        // the caller gave, and feeding a HashSet's iteration order into the
        // lookup below would vary the TrashInfo ids run to run.
        let wanted: Vec<EntityId> = item_ids
            .iter()
            .copied()
            // An item inside a binder being trashed wholesale is already going;
            // filing a second TrashInfo for it would restore it twice.
            .filter(|id| !inside_a_trashed_binder.contains(id))
            .collect();
        if !wanted.is_empty() {
            let mut by_binder = store.get_binder_relationships_from_right_ids(
                &BinderRelationshipField::BinderItems,
                &wanted,
            )?;
            // Sorted so a selection spanning several binders files its entries
            // in the same order every time.
            by_binder.sort_by_key(|(binder, _)| *binder);

            let owning: Vec<EntityId> = by_binder.iter().map(|(binder, _)| *binder).collect();
            assert_work_owns(store, self.work_id, &owning)?;

            // Each entry carries the matched binder's **whole** item list, not the
            // subset that was asked about -- so it has to be intersected back with
            // `wanted`. Taking it at face value trashes every row in any binder the
            // selection touches, which is what selecting one scene in a four-scene
            // binder did until `the_cascade_reaches_a_grandchild` caught it.
            let asked: HashSet<EntityId> = wanted.iter().copied().collect();
            for (binder, listed) in by_binder {
                let order = store.binder_items(binder)?;
                let indent = trash_ops::indents(store, &order)?;
                let requested: HashSet<EntityId> =
                    listed.into_iter().filter(|id| asked.contains(id)).collect();
                if requested.is_empty() {
                    continue;
                }
                let (roots, cascade) = trash_ops::roots_and_cascade(&order, &indent, &requested);
                if !cascade.is_empty() {
                    groups.push(ItemGroup {
                        binder,
                        roots,
                        cascade,
                    });
                }
            }
        }

        if binders.is_empty() && groups.is_empty() {
            return Err(anyhow!(
                "trash_selection: nothing in the selection is in work {}",
                self.work_id
            ));
        }

        self.binders = binders;
        self.groups = groups;
        self.trashed_at = Utc::now();
        self.apply(store)?;

        let trashed_binder_ids: Vec<EntityId> = self.binders.iter().map(|(b, _)| *b).collect();
        let trashed_item_ids: Vec<EntityId> =
            self.groups.iter().flat_map(|g| g.roots.clone()).collect();

        uow.commit()?;
        let mut announced = trashed_binder_ids.clone();
        announced.extend(trashed_item_ids.iter().copied());
        uow.publish_trash_selection_event(announced, None);

        Ok(TrashSelectionResultDto {
            trashed_binder_ids,
            trashed_item_ids,
        })
    }

    /// Forward (execute + redo).
    fn apply(&mut self, store: &dyn TrashSelectionUnitOfWorkTrait) -> Result<()> {
        let mut created = Vec::new();
        for (binder, items) in &self.binders {
            set_binder_activated(store, *binder, false)?;
            set_activated(store, items, false)?;
            created.extend(file_trash_infos(
                store,
                self.work_id,
                &[*binder],
                &TrashInfoRelationshipField::TrashedBinder,
                // A whole binder has no parent binder to be restored into.
                &|_| 0,
                self.trashed_at,
            )?);
        }
        for group in &self.groups {
            set_activated(store, &group.cascade, false)?;
            let origin = group.binder as i64;
            created.extend(file_trash_infos(
                store,
                self.work_id,
                &group.roots,
                &TrashInfoRelationshipField::TrashedBinderItem,
                &|_| origin,
                self.trashed_at,
            )?);
        }
        self.created_trash = created;
        Ok(())
    }

    /// Inverse (undo).
    fn revert(&self, store: &dyn TrashSelectionUnitOfWorkTrait) -> Result<()> {
        for (binder, items) in &self.binders {
            set_binder_activated(store, *binder, true)?;
            set_activated(store, items, true)?;
        }
        for group in &self.groups {
            set_activated(store, &group.cascade, true)?;
        }
        unfile_trash_infos(store, self.work_id, &self.created_trash)
    }

    fn touched(&self) -> Vec<EntityId> {
        let mut ids: Vec<EntityId> = self.binders.iter().map(|(b, _)| *b).collect();
        ids.extend(self.groups.iter().flat_map(|g| g.cascade.iter().copied()));
        ids
    }
}

/// Preserve order, drop repeats. A selection can legitimately name the same row
/// twice (two views, one gesture), and a repeat must not file two `TrashInfo`s.
fn dedup(ids: &[u64]) -> Vec<EntityId> {
    let mut seen = HashSet::new();
    ids.iter()
        .map(|&id| id as EntityId)
        .filter(|id| seen.insert(*id))
        .collect()
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for TrashSelectionUseCase {
    fn undo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.revert(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_selection_event(self.touched(), None);
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let store: &dyn TrashSelectionUnitOfWorkTrait = uow.as_ref();
        self.apply(store)?;
        uow.commit()?;
        uow.publish_trash_selection_event(self.touched(), None);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
