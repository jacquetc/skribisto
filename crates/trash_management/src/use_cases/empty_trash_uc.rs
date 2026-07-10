// Custom implementation: permanently delete everything indexed by
// Work.trash_infos. Trashed binders (with their items + contents) are removed
// and dropped from their Work; trashed item subtrees (with their contents) are
// removed and dropped from their binder's order. All TrashInfos are removed and
// the index cleared. Undoable via a Work-scoped snapshot/restore: post-reparent
// TrashInfo lives in the Work trunk alongside the items/binders, so the whole
// Work is the undo scope.
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

pub trait EmptyTrashUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn EmptyTrashUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/empty_trash_uow.rs.
#[macros::uow_action(entity = "TrashInfo", action = "GetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "RemoveMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "RemoveMulti")]
#[macros::uow_action(entity = "Content", action = "RemoveMulti")]
pub trait EmptyTrashUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_empty_trash_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct EmptyTrashUseCase {
    uow_factory: Box<dyn EmptyTrashUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl EmptyTrashUseCase {
    pub fn new(uow_factory: Box<dyn EmptyTrashUnitOfWorkFactoryTrait>) -> Self {
        EmptyTrashUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // The Root-scoped snapshot is taken below, after the read-only planning loop.

        let work_id = work_id(uow.as_ref())?;
        let trash_infos =
            uow.get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?;

        let mut remove_contents: Vec<EntityId> = Vec::new();
        let mut remove_items: Vec<EntityId> = Vec::new();
        let mut remove_binders: Vec<EntityId> = Vec::new();
        // Surviving binders that lose a trashed item subtree from their order.
        let mut drop_from_binder: HashMap<EntityId, Vec<EntityId>> = HashMap::new();

        for info in &trash_infos {
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
                    let cs = uow
                        .get_binder_item_relationship(it, &BinderItemRelationshipField::Contents)?;
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
                    let cs = uow
                        .get_binder_item_relationship(it, &BinderItemRelationshipField::Contents)?;
                    remove_contents.extend(cs);
                }
                remove_items.extend(subtree.iter().copied());
                drop_from_binder
                    .entry(binder_id)
                    .or_default()
                    .extend(subtree);
            }
        }

        // Work-scoped snapshot, taken now (after the read-only planning loop above,
        // before the first mutation below).
        let snap_before = uow.snapshot_work(&[work_id])?;

        // Drop removed item subtrees from their surviving binders' order.
        for (binder_id, dropped) in &drop_from_binder {
            let dropped_set: HashSet<EntityId> = dropped.iter().copied().collect();
            let order =
                uow.get_binder_relationship(binder_id, &BinderRelationshipField::BinderItems)?;
            let new: Vec<EntityId> = order
                .into_iter()
                .filter(|id| !dropped_set.contains(id))
                .collect();
            uow.set_binder_relationship(binder_id, &BinderRelationshipField::BinderItems, &new)?;
        }

        // Drop removed binders from their Works.
        if !remove_binders.is_empty() {
            let rb: HashSet<EntityId> = remove_binders.iter().copied().collect();
            for work in uow.get_all_work()? {
                let binders =
                    uow.get_work_relationship(&work.id, &WorkRelationshipField::Binders)?;
                if binders.iter().any(|b| rb.contains(b)) {
                    let new: Vec<EntityId> =
                        binders.into_iter().filter(|b| !rb.contains(b)).collect();
                    uow.set_work_relationship(&work.id, &WorkRelationshipField::Binders, &new)?;
                }
            }
        }

        // Hard-remove entities (contents → items → binders).
        if !remove_contents.is_empty() {
            uow.remove_content_multi(&remove_contents)?;
        }
        if !remove_items.is_empty() {
            uow.remove_binder_item_multi(&remove_items)?;
        }
        if !remove_binders.is_empty() {
            uow.remove_binder_multi(&remove_binders)?;
        }

        // Remove the TrashInfos and clear the index.
        if !trash_infos.is_empty() {
            uow.remove_trash_info_multi(&trash_infos)?;
        }
        uow.set_work_relationship(&work_id, &WorkRelationshipField::TrashInfos, &[])?;

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_empty_trash_event(remove_items.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

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

fn work_id(uow: &dyn EmptyTrashUnitOfWorkTrait) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .next()
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("empty_trash: no Work entity in store"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for EmptyTrashUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("empty_trash: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("empty_trash: nothing to redo"))?;
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
