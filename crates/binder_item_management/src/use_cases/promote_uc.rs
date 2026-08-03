// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: "promote" converts a binder item to another type. A
// folder may become any *other* kind of folder (plain, chapter, part, book,
// notes folder), letting a writer outline in bare folders and declare what
// each one is later. Among the leaves, Scene <-> Note and chapter-folder <->
// flat-chapter are pairs, and a Scene may also be raised into a flat chapter
// (and back).
//
// The caller names the target as a `skribisto_model::PromoteTarget` wire code. It is a
// *code*, not an index into a menu: this use case re-derives the legal targets from the
// item's CURRENT type and rejects anything not among them, so a stale menu can never
// promote an item to a type that was never offered for it.
//
// Content is remapped into the target's vocabulary so the writer's text survives: a
// title becomes the target's title (a chapter that becomes a part keeps its name, as a
// part title), and SceneText <-> NoteText for Scene <-> Note. A conversion that would
// leave non-empty text with nowhere to go (a chapter holding prose becoming a Part,
// which has no prose) is REFUSED rather than silently discarding it. Empty rows never
// block anything.
//
// The item keeps its place in the binder. Undoable via a scoped snapshot/restore of the
// item's own subtree (item + its Content rows). The caller (UI) enforces the "empty the
// folder before demoting a container to a leaf" rule.
use crate::PromoteDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItem, Content};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use skribisto_model::PromoteTarget;

pub trait PromoteUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn PromoteUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/promote_uow.rs.
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "Update")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "Snapshot")]
#[macros::uow_action(entity = "BinderItem", action = "Restore")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
pub trait PromoteUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_promote_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct PromoteUseCase {
    uow_factory: Box<dyn PromoteUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl PromoteUseCase {
    pub fn new(uow_factory: Box<dyn PromoteUnitOfWorkFactoryTrait>) -> Self {
        PromoteUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &PromoteDto) -> Result<()> {
        let item_id = dto.item_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let snap_before = uow.snapshot_binder_item(&[item_id])?;

        let item = uow
            .get_binder_item_multi(&[item_id])?
            .into_iter()
            .next()
            .flatten()
            .ok_or_else(|| anyhow!("promote: item not found"))?;

        // Resolve the requested target, and check it is one this item may actually
        // become *right now* — not merely one it could become when the menu was built.
        let target = PromoteTarget::from_code(dto.target)
            .ok_or_else(|| anyhow!("promote: unknown target code {}", dto.target))?;
        if !skribisto_model::promote_targets(&item.role, &item.sub_role).contains(&target) {
            return Err(anyhow!(
                "promote: {:?}/{:?} cannot become {:?}",
                item.role,
                item.sub_role,
                target
            ));
        }
        let (target_role, target_sub_role) = target.combo();

        let content_ids =
            uow.get_binder_item_relationship(&item_id, &BinderItemRelationshipField::Contents)?;
        let rows: Vec<Content> = uow
            .get_content_multi(&content_ids)?
            .into_iter()
            .flatten()
            .collect();

        // Refuse before mutating anything if the target has nowhere to put text the item
        // actually holds. Empty rows are ignored: an untouched prose slot must not stop a
        // chapter becoming a part.
        let non_empty: Vec<_> = rows
            .iter()
            .filter(|c| !c.data.trim().is_empty())
            .map(|c| c.role.clone())
            .collect();
        let lost =
            skribisto_model::promote_content_loss(&target_role, &target_sub_role, &non_empty);
        if !lost.is_empty() {
            return Err(anyhow!(
                "promote: {:?} has nowhere to keep {:?}; clear or move that text first",
                target,
                lost
            ));
        }

        let now = chrono::Utc::now();

        // 1. Flip the item's type (scalar update; relationships untouched).
        let mut updated = item.clone();
        updated.role = target_role.clone();
        updated.sub_role = target_sub_role.clone();
        updated.updated_at = now;
        uow.update_binder_item(&updated)?;

        // 2. Remap the content roles into the target's vocabulary so the text survives.
        //    Anything with no home here is empty (the guard above proved it), so it is
        //    simply left behind: it is invalid for the new type, and the `.skrib`
        //    serializer filters content by the constraint matrix on save.
        for mut row in rows {
            if let Some(new_role) =
                skribisto_model::remap_content(&target_role, &target_sub_role, &row.role)
                && new_role != row.role
            {
                row.role = new_role;
                row.updated_at = now;
                uow.update_content(&row)?;
            }
        }

        let snap_after = uow.snapshot_binder_item(&[item_id])?;
        uow.commit()?;
        uow.publish_promote_event(vec![item_id], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for PromoteUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("promote: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder_item(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("promote: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder_item(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
