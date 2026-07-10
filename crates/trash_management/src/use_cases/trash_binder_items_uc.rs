// Custom implementation: soft-delete (trash) a set of BinderItems and their
// subtrees. Trashed items stay in place in the binder; `activated` flips to
// false and one TrashInfo per requested root is indexed under System.trash_infos.
//
// Undoable via a Root-scoped snapshot/restore. The op spans both the Work trunk
// (item `activated` flags) and the System trunk (TrashInfo + the index), so the
// whole tree is the undo scope. (This narrows to the Work once TrashInfo moves
// under Work in the deferred multi-Work reparent.)
use crate::TrashBinderItemsDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{BinderItem, Root, System, TrashInfo};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

pub trait TrashBinderItemsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn TrashBinderItemsUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/trash_binder_items_uow.rs.
#[macros::uow_action(entity = "Root", action = "GetAll")]
#[macros::uow_action(entity = "Root", action = "Snapshot")]
#[macros::uow_action(entity = "Root", action = "Restore")]
#[macros::uow_action(entity = "System", action = "GetAll")]
#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait TrashBinderItemsUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_trash_binder_items_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct TrashBinderItemsUseCase {
    uow_factory: Box<dyn TrashBinderItemsUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl TrashBinderItemsUseCase {
    pub fn new(uow_factory: Box<dyn TrashBinderItemsUnitOfWorkFactoryTrait>) -> Self {
        TrashBinderItemsUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
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

        // Root-scoped snapshot, taken after the read-only resolution above and
        // before the first mutation below.
        let root_id = root_id(uow.as_ref())?;
        let snap_before = uow.snapshot_root(&[root_id])?;

        // Flip `activated` to false for the whole cascade.
        let mut updated: Vec<BinderItem> = Vec::new();
        for it in uow.get_binder_item_multi(&cascade)?.into_iter().flatten() {
            let mut it = it;
            it.activated = false;
            updated.push(it);
        }
        uow.update_binder_item_multi(&updated)?;

        // Index one TrashInfo per requested root under System.trash_infos.
        let system = system_singleton(uow.as_ref())?;
        let mut trash_infos =
            uow.get_system_relationship(&system.id, &SystemRelationshipField::TrashInfos)?;
        let now = chrono::Utc::now();
        for root in &roots {
            let info = uow.create_orphan_trash_info(&TrashInfo {
                created_at: now,
                updated_at: now,
                trashed_at: now,
                origin_binder_id: origin_binder as i64,
                ..Default::default()
            })?;
            uow.set_trash_info_relationship(
                &info.id,
                &TrashInfoRelationshipField::TrashedBinderItem,
                &[*root],
            )?;
            trash_infos.push(info.id);
        }
        uow.set_system_relationship(
            &system.id,
            &SystemRelationshipField::TrashInfos,
            &trash_infos,
        )?;

        let snap_after = uow.snapshot_root(&[root_id])?;
        uow.commit()?;
        uow.publish_trash_binder_items_event(roots, None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
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

pub(crate) fn system_singleton(uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<System> {
    uow.get_all_system()?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("trash: no System entity in store"))
}

pub(crate) fn root_id(uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<EntityId> {
    uow.get_all_root()?
        .into_iter()
        .next()
        .map(|r: Root| r.id)
        .ok_or_else(|| anyhow!("trash: no Root entity in store"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for TrashBinderItemsUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("trash_binder_items: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_root(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("trash_binder_items: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_root(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
