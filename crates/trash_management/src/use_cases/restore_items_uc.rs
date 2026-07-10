// Custom implementation: restore trashed entities indexed by the given
// TrashInfos. A TrashedBinder reactivates the binder and all its items; a
// TrashedBinderItem reactivates the item and its contiguous subtree in place.
// If a trashed item's binder no longer exists, it is reported `orphaned`.
// The restored TrashInfos are unlinked from System.trash_infos.
//
// Undoable via a *targeted inverse* (not a whole-store snapshot): the command
// records exactly the binders/items it reactivated and the System.trash_infos
// index before/after, so undo re-trashes those rows and re-links the index.
use crate::RestoreItemsDto;
use crate::RestoreResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{Binder, BinderItem, System};
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

pub trait RestoreItemsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn RestoreItemsUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/restore_items_uow.rs.
#[macros::uow_action(entity = "System", action = "GetAll")]
#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "SetRelationship")]
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
    // Undo/redo state — targeted inverse, no whole-store snapshot.
    touched_binders: Vec<EntityId>,
    touched_items: Vec<EntityId>,
    index_before: Vec<EntityId>,
    index_after: Vec<EntityId>,
}

impl RestoreItemsUseCase {
    pub fn new(uow_factory: Box<dyn RestoreItemsUnitOfWorkFactoryTrait>) -> Self {
        RestoreItemsUseCase {
            uow_factory,
            touched_binders: Vec::new(),
            touched_items: Vec::new(),
            index_before: Vec::new(),
            index_after: Vec::new(),
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

        let system = system_singleton(uow.as_ref())?;
        let index_before =
            uow.get_system_relationship(&system.id, &SystemRelationshipField::TrashInfos)?;

        let mut restored_count: i64 = 0;
        let mut orphaned = false;
        let mut consumed: HashSet<EntityId> = HashSet::new(); // TrashInfos to drop from the index
        let mut touched_binders: Vec<EntityId> = Vec::new();
        let mut touched_items: Vec<EntityId> = Vec::new();

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
                        set_items_activated(uow.as_ref(), &item_ids, true)?;
                        touched_binders.push(binder_id);
                        touched_items.extend(item_ids);
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
                        for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
                            indent.insert(it.id, it.indent);
                        }
                        let subtree = subtree_of(&order, &indent, item_id);
                        set_items_activated(uow.as_ref(), &subtree, true)?;
                        touched_items.extend(subtree);
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

        // Unlink consumed TrashInfos from System.trash_infos.
        let index_after: Vec<EntityId> = index_before
            .iter()
            .copied()
            .filter(|id| !consumed.contains(id))
            .collect();
        if index_after.len() != index_before.len() {
            uow.set_system_relationship(
                &system.id,
                &SystemRelationshipField::TrashInfos,
                &index_after,
            )?;
        }

        uow.commit()?;
        uow.publish_restore_items_event(touched_event(&touched_binders, &touched_items), None);

        self.touched_binders = touched_binders;
        self.touched_items = touched_items;
        self.index_before = index_before;
        self.index_after = index_after;
        Ok(RestoreResultDto {
            restored_count,
            orphaned,
        })
    }

    fn set_state(&self, uow: &dyn RestoreItemsUnitOfWorkTrait, activated: bool, index: &[EntityId]) -> Result<()> {
        set_binders_activated(uow, &self.touched_binders, activated)?;
        set_items_activated(uow, &self.touched_items, activated)?;
        let system = system_singleton(uow)?;
        uow.set_system_relationship(&system.id, &SystemRelationshipField::TrashInfos, index)?;
        Ok(())
    }
}

fn touched_event(binders: &[EntityId], items: &[EntityId]) -> Vec<EntityId> {
    let mut all = binders.to_vec();
    all.extend_from_slice(items);
    all
}

fn set_binders_activated(
    uow: &dyn RestoreItemsUnitOfWorkTrait,
    ids: &[EntityId],
    value: bool,
) -> Result<()> {
    for id in ids {
        if let Some(mut binder) = uow.get_binder(id)? {
            binder.activated = value;
            uow.update_binder(&binder)?;
        }
    }
    Ok(())
}

fn set_items_activated(
    uow: &dyn RestoreItemsUnitOfWorkTrait,
    ids: &[EntityId],
    value: bool,
) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut updated: Vec<BinderItem> = Vec::new();
    for it in uow.get_binder_item_multi(ids)?.into_iter().flatten() {
        let mut it = it;
        it.activated = value;
        updated.push(it);
    }
    if !updated.is_empty() {
        uow.update_binder_item_multi(&updated)?;
    }
    Ok(())
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

fn system_singleton(uow: &dyn RestoreItemsUnitOfWorkTrait) -> Result<System> {
    uow.get_all_system()?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("restore_items: no System entity in store"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for RestoreItemsUseCase {
    fn undo(&mut self) -> Result<()> {
        // Re-trash: deactivate what we restored and re-link the prior index.
        let index = self.index_before.clone();
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.set_state(uow.as_ref(), false, &index)?;
        uow.commit()?;
        uow.publish_restore_items_event(
            touched_event(&self.touched_binders, &self.touched_items),
            None,
        );
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let index = self.index_after.clone();
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.set_state(uow.as_ref(), true, &index)?;
        uow.commit()?;
        uow.publish_restore_items_event(
            touched_event(&self.touched_binders, &self.touched_items),
            None,
        );
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
