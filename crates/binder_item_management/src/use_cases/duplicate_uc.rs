// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: deep-copy each selected BinderItem subtree (items +
// their Content rows + tag links, NOT references) and insert each new subtree
// immediately after its source subtree in the binder. Undoable via a scoped
// snapshot/restore of the source binder subtree.
use crate::DuplicateDto;
use crate::DuplicateReturnDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItem, Content};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

pub trait DuplicateUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn DuplicateUnitOfWorkTrait>;
}

// Read-write unit of work trait. The same macro set must appear on the impl
// block in ../units_of_work/duplicate_uow.rs.
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
pub trait DuplicateUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_duplicate_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct DuplicateUseCase {
    uow_factory: Box<dyn DuplicateUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl DuplicateUseCase {
    pub fn new(uow_factory: Box<dyn DuplicateUnitOfWorkFactoryTrait>) -> Self {
        DuplicateUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &DuplicateDto) -> Result<DuplicateReturnDto> {
        if dto.item_ids.is_empty() {
            return Err(anyhow!("duplicate: no items to duplicate"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // The scoped snapshot is taken below, once the source binder is known.

        // All requested items must share one source binder.
        let groups = uow.get_binder_relationships_from_right_ids(
            &BinderRelationshipField::BinderItems,
            &dto.item_ids,
        )?;
        if groups.len() != 1 {
            return Err(anyhow!(
                "duplicate: items span {} binders (must share one)",
                groups.len()
            ));
        }
        let (binder, found_items) = groups.into_iter().next().unwrap();
        let found: HashSet<EntityId> = found_items.into_iter().collect();
        if dto.item_ids.iter().any(|id| !found.contains(id)) {
            return Err(anyhow!(
                "duplicate: some items are not in the source binder"
            ));
        }

        // Scoped snapshot of the source binder subtree, before the first mutation.
        let snap_before = uow.snapshot_binder(&[binder])?;

        let order = uow.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)?;
        let mut indent: HashMap<EntityId, i64> = HashMap::new();
        for it in uow.get_binder_item_multi(&order)?.into_iter().flatten() {
            indent.insert(it.id, it.indent);
        }

        // Compute the top-level subtrees to duplicate (requested roots not nested
        // under another requested item), each as a contiguous run in `order`.
        let requested: HashSet<EntityId> = dto.item_ids.iter().copied().collect();
        let mut subtrees: Vec<Vec<EntityId>> = Vec::new(); // each: root..last, in order
        let mut covered: HashSet<EntityId> = HashSet::new();
        let mut i = 0usize;
        while i < order.len() {
            let id = order[i];
            if requested.contains(&id) && !covered.contains(&id) {
                let root_indent = *indent.get(&id).unwrap_or(&0);
                let mut sub: Vec<EntityId> = Vec::new();
                let mut j = i;
                loop {
                    sub.push(order[j]);
                    covered.insert(order[j]);
                    j += 1;
                    if j >= order.len() || *indent.get(&order[j]).unwrap_or(&0) <= root_indent {
                        break;
                    }
                }
                subtrees.push(sub);
                i = j;
            } else {
                i += 1;
            }
        }

        // Deep-copy each subtree; remember (source_last_id -> new block) so the
        // new block is spliced right after its source subtree.
        let mut block_after: HashMap<EntityId, Vec<EntityId>> = HashMap::new();
        let mut new_root_ids: Vec<EntityId> = Vec::new();
        let now = chrono::Utc::now();

        for sub in &subtrees {
            let src_items = uow.get_binder_item_multi(sub)?;
            let mut new_block: Vec<EntityId> = Vec::new();
            for maybe in src_items.into_iter() {
                let src = maybe.ok_or_else(|| anyhow!("duplicate: source item vanished"))?;

                // Copy the item's Content rows.
                let content_ids = uow.get_binder_item_relationship(
                    &src.id,
                    &BinderItemRelationshipField::Contents,
                )?;
                let mut new_content_ids: Vec<EntityId> = Vec::new();
                for c in uow.get_content_multi(&content_ids)?.into_iter().flatten() {
                    let created = uow.create_orphan_content(&Content {
                        created_at: now,
                        updated_at: now,
                        activated: c.activated,
                        role: c.role.clone(),
                        data: c.data.clone(),
                        ..Default::default()
                    })?;
                    new_content_ids.push(created.id);
                }

                // Copy the item itself (scalars incl. indent).
                let created_item = uow.create_orphan_binder_item(&BinderItem {
                    // A duplicate is a NEW row: it mints its own identity and
                    // must never inherit the source's, or the two would be
                    // indistinguishable to anything keyed by uid.
                    uid: common::uid::new_uid(),
                    created_at: now,
                    updated_at: now,
                    title: src.title.clone(),
                    sub_title: src.sub_title.clone(),
                    role: src.role.clone(),
                    sub_role: src.sub_role.clone(),
                    label: src.label.clone(),
                    activated: src.activated,
                    is_favorite: src.is_favorite,
                    is_exportable: src.is_exportable,
                    indent: src.indent,
                    word_count_goal: src.word_count_goal,
                    char_count_goal: src.char_count_goal,
                    dict_language: src.dict_language.clone(),
                    ..Default::default()
                })?;

                if !new_content_ids.is_empty() {
                    uow.set_binder_item_relationship(
                        &created_item.id,
                        &BinderItemRelationshipField::Contents,
                        &new_content_ids,
                    )?;
                }

                // Copy tag links (shared M2M); references are intentionally NOT copied.
                let tag_ids =
                    uow.get_binder_item_relationship(&src.id, &BinderItemRelationshipField::Tags)?;
                if !tag_ids.is_empty() {
                    uow.set_binder_item_relationship(
                        &created_item.id,
                        &BinderItemRelationshipField::Tags,
                        &tag_ids,
                    )?;
                }

                new_block.push(created_item.id);
            }
            new_root_ids.push(new_block[0]);
            block_after.insert(*sub.last().unwrap(), new_block);
        }

        // Splice each new block immediately after its source subtree's last item.
        let mut new_order: Vec<EntityId> = Vec::with_capacity(order.len() + new_root_ids.len());
        for &id in &order {
            new_order.push(id);
            if let Some(block) = block_after.remove(&id) {
                new_order.extend(block);
            }
        }
        uow.set_binder_relationship(&binder, &BinderRelationshipField::BinderItems, &new_order)?;

        let snap_after = uow.snapshot_binder(&[binder])?;
        uow.commit()?;
        uow.publish_duplicate_event(new_root_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(DuplicateReturnDto {
            new_item_ids: new_root_ids,
        })
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for DuplicateUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("duplicate: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("duplicate: nothing to redo"))?;
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
