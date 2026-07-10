// Custom implementation: soft-delete (trash) a set of BinderItems and their
// subtrees. Trashed items stay in place in the binder; `activated` flips to
// false and one TrashInfo per requested root is indexed under Work.trash_infos.
//
// Undoable via a targeted inverse (this is a high-frequency op): undo reactivates
// exactly the cascade it trashed and removes exactly the TrashInfo rows it
// created. Post-reparent TrashInfo lives in the Work trunk alongside the items,
// so no cross-trunk snapshot is needed.
//
// Work resolution is get_all_work (single Work today); when in-process multi-Work
// lands this resolves origin_binder -> its owning Work.
use crate::TrashBinderItemsDto;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, TrashInfo, Work};
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

pub trait TrashBinderItemsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn TrashBinderItemsUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/trash_binder_items_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait TrashBinderItemsUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_trash_binder_items_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct TrashBinderItemsUseCase {
    uow_factory: Box<dyn TrashBinderItemsUnitOfWorkFactoryTrait>,
    // Undo/redo state — targeted inverse.
    origin_binder: EntityId,
    work_id: EntityId,
    trashed_at: DateTime<Utc>,
    roots: Vec<EntityId>,
    cascade: Vec<EntityId>,
    created_trash: Vec<EntityId>,
}

impl TrashBinderItemsUseCase {
    pub fn new(uow_factory: Box<dyn TrashBinderItemsUnitOfWorkFactoryTrait>) -> Self {
        TrashBinderItemsUseCase {
            uow_factory,
            origin_binder: 0,
            work_id: 0,
            trashed_at: Utc::now(),
            roots: Vec::new(),
            cascade: Vec::new(),
            created_trash: Vec::new(),
        }
    }

    pub fn execute(&mut self, dto: &TrashBinderItemsDto) -> Result<()> {
        if dto.binder_item_ids.is_empty() {
            return Err(anyhow!("trash_binder_items: no items"));
        }
        let origin_binder = dto.origin_binder_id as EntityId;
        let requested: HashSet<EntityId> =
            dto.binder_item_ids.iter().map(|&x| x as EntityId).collect();

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Resolve the requested roots + their contiguous cascade from binder order.
        let order =
            uow.get_binder_relationship(&origin_binder, &BinderRelationshipField::BinderItems)?;
        let mut indent: HashMap<EntityId, i64> = HashMap::new();
        for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
            indent.insert(it.id, it.indent);
        }
        let (roots, cascade) = roots_and_cascade(&order, &indent, &requested);
        if cascade.is_empty() {
            return Err(anyhow!(
                "trash_binder_items: no matching items in binder {origin_binder}"
            ));
        }

        self.origin_binder = origin_binder;
        self.work_id = work_id(uow.as_ref())?;
        self.trashed_at = Utc::now();
        self.roots = roots.clone();
        self.cascade = cascade;
        self.apply(uow.as_ref())?;

        uow.commit()?;
        uow.publish_trash_binder_items_event(roots, None);
        Ok(())
    }

    /// Forward (execute + redo): flip the cascade to trashed and index one
    /// TrashInfo per root under Work.trash_infos, recording the new ids.
    fn apply(&mut self, uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<()> {
        set_activated(uow, &self.cascade, false)?;

        let mut index = uow.get_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos)?;
        let mut created = Vec::with_capacity(self.roots.len());
        for root in &self.roots {
            let info = uow.create_orphan_trash_info(&TrashInfo {
                created_at: self.trashed_at,
                updated_at: self.trashed_at,
                trashed_at: self.trashed_at,
                origin_binder_id: self.origin_binder as i64,
                ..Default::default()
            })?;
            uow.set_trash_info_relationship(
                &info.id,
                &TrashInfoRelationshipField::TrashedBinderItem,
                &[*root],
            )?;
            index.push(info.id);
            created.push(info.id);
        }
        uow.set_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos, &index)?;
        self.created_trash = created;
        Ok(())
    }

    /// Inverse (undo): reactivate the cascade and drop this command's TrashInfo
    /// rows, unlinking them from Work.trash_infos.
    fn revert(&self, uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<()> {
        set_activated(uow, &self.cascade, true)?;

        if !self.created_trash.is_empty() {
            let drop: HashSet<EntityId> = self.created_trash.iter().copied().collect();
            let remaining: Vec<EntityId> = uow
                .get_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos)?
                .into_iter()
                .filter(|id| !drop.contains(id))
                .collect();
            uow.set_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos, &remaining)?;
            uow.remove_trash_info_multi(&self.created_trash)?;
        }
        Ok(())
    }
}

/// Load the given items, set `activated`, write them back.
fn set_activated(
    uow: &dyn TrashBinderItemsUnitOfWorkTrait,
    ids: &[EntityId],
    value: bool,
) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut items: Vec<BinderItem> =
        uow.get_binder_item_multi(ids)?.into_iter().flatten().collect();
    for it in &mut items {
        it.activated = value;
    }
    uow.update_binder_item_multi(&items)?;
    Ok(())
}

/// Compute the requested *roots* (items not nested under another requested item)
/// and the full *cascade* (each root plus its contiguous subtree), in binder order.
pub(crate) fn roots_and_cascade(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    requested: &HashSet<EntityId>,
) -> (Vec<EntityId>, Vec<EntityId>) {
    let mut roots = Vec::new();
    let mut cascade = Vec::new();
    let mut covered: HashSet<EntityId> = HashSet::new();
    let mut i = 0usize;
    while i < order.len() {
        let id = order[i];
        if requested.contains(&id) && !covered.contains(&id) {
            roots.push(id);
            let root_indent = *indent.get(&id).unwrap_or(&0);
            let mut j = i;
            loop {
                cascade.push(order[j]);
                covered.insert(order[j]);
                j += 1;
                if j >= order.len() || *indent.get(&order[j]).unwrap_or(&0) <= root_indent {
                    break;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
    (roots, cascade)
}

pub(crate) fn work_id(uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .next()
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("trash: no Work entity in store"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for TrashBinderItemsUseCase {
    fn undo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.revert(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_binder_items_event(self.cascade.clone(), None);
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.apply(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_binder_items_event(self.roots.clone(), None);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
