// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: blank the titles of the named items in one undo step.
//
// A title has TWO homes and this writes both, always:
//
//   * `BinderItem.title` -- what the outline row and the tab show;
//   * the title `Content` row the constraint matrix gives that combination
//     (`skribisto_model::title_role_of` -> BookTitle / PartTitle /
//     ChapterTitle) -- what the exporter actually compiles.
//
// They are one title with two homes, and clearing one without the other leaves
// the manuscript printing a heading the binder says is gone. Pairing them is
// the reason this is a use case and not a loop of scalar updates: a caller has
// to know the matrix to find the second home, and a caller doing it per item is
// a caller who will eventually forget.
//
// Ids are given, not derived. Deciding *which* titles are worth clearing is a
// read-model question the caller answers and the writer confirms in a preview
// (see the outline's `redundant_number_titles`, which offers exactly the titles
// the exporter was already discarding); taking the ids back verbatim means the
// rows the writer saw are the rows that change, even if the tree moved in
// between.
//
// Clearing rather than rewriting: an empty title is a first-class state
// everywhere. The binder falls back to the numbering badge plus the item's
// type, and the exporter's `NumberAndTitle` renders an untitled chapter as its
// number alone -- so a writer who wanted "Chapter 3" printed still gets it,
// from the generator, wherever the export style asks for it.
//
// Undoable by scoped snapshot rather than a targeted inverse, unlike its two
// neighbours in this crate. Those write one scalar; this writes across two
// entity kinds, and `Content` is owned by `BinderItem`, so one
// `snapshot_binder_item` over the touched ids already covers both halves --
// the same reasoning as `promote_uc`, which retypes an item and remaps its
// content rows together.
use crate::ClearTitlesDto;
use crate::ClearTitlesResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItem, Content};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;

pub trait ClearTitlesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ClearTitlesUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/clear_titles_uow.rs.
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "Snapshot")]
#[macros::uow_action(entity = "BinderItem", action = "Restore")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
pub trait ClearTitlesUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_clear_titles_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct ClearTitlesUseCase {
    uow_factory: Box<dyn ClearTitlesUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
    cleared: Vec<EntityId>,
}

impl ClearTitlesUseCase {
    pub fn new(uow_factory: Box<dyn ClearTitlesUnitOfWorkFactoryTrait>) -> Self {
        ClearTitlesUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
            cleared: Vec::new(),
        }
    }

    pub fn execute(&mut self, dto: &ClearTitlesDto) -> Result<ClearTitlesResultDto> {
        let ids: Vec<EntityId> = dto.item_ids.iter().map(|&id| id as EntityId).collect();
        if ids.is_empty() {
            return Ok(ClearTitlesResultDto::default());
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Scoped to the requested ids, so undo reverts these subtrees and
        // nothing else. Taken before the first write; every read above it is
        // read-only, and an early `return Err` drops the uow and rolls back.
        let snap_before = uow.snapshot_binder_item(&ids)?;

        let mut by_id: std::collections::HashMap<EntityId, BinderItem> = uow
            .get_binder_item_multi(&ids)?
            .into_iter()
            .flatten()
            .map(|it| (it.id, it))
            .collect();

        let now = chrono::Utc::now();
        let mut items: Vec<BinderItem> = Vec::new();
        let mut cleared: Vec<EntityId> = Vec::new();
        for id in &ids {
            // A requested row that is gone is skipped, not raised: the preview
            // the caller is acting on was built a moment earlier, and failing
            // the whole step over one deleted row would clear none of the rest.
            let Some(mut item) = by_id.remove(id) else {
                continue;
            };

            // Home two, resolved from the matrix rather than guessed. `None`
            // means this combination carries no title row at all — a Scene, for
            // instance — so only the entity field is its title.
            let title_row = match skribisto_model::title_role_of(&item.role, &item.sub_role) {
                Some(role) => self.title_content(uow.as_ref(), *id, &role)?,
                None => None,
            };

            let entity_dirty = !item.title.is_empty();
            let content_dirty = title_row.as_ref().is_some_and(|c| !c.data.is_empty());
            if !entity_dirty && !content_dirty {
                continue;
            }

            if entity_dirty {
                item.title.clear();
                item.updated_at = now;
                items.push(item);
            }
            if let Some(mut row) = title_row
                && content_dirty
            {
                row.data.clear();
                row.updated_at = now;
                uow.update_content(&row)?;
            }
            cleared.push(*id);
        }

        if !items.is_empty() {
            uow.update_binder_item_multi(&items)?;
        }

        let snap_after = uow.snapshot_binder_item(&ids)?;
        uow.commit()?;
        uow.publish_clear_titles_event(cleared.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        self.cleared = cleared.clone();
        Ok(ClearTitlesResultDto {
            cleared_ids: cleared,
        })
    }

    /// The item's `Content` row for `role`, if it has one.
    ///
    /// Deliberately does **not** create one. Clearing a title that has no
    /// content row is complete once the entity field is blank; minting an empty
    /// row to hold nothing would put a row in the bundle that the writer never
    /// authored, and `.skrib`'s serializer would carry it forward.
    fn title_content(
        &self,
        uow: &dyn ClearTitlesUnitOfWorkTrait,
        item_id: EntityId,
        role: &common::entities::ContentRole,
    ) -> Result<Option<Content>> {
        let content_ids =
            uow.get_binder_item_relationship(&item_id, &BinderItemRelationshipField::Contents)?;
        Ok(uow
            .get_content_multi(&content_ids)?
            .into_iter()
            .flatten()
            .find(|c| &c.role == role))
    }

    fn restore(&self, snap: Option<&EntityTreeSnapshot>, what: &str) -> Result<()> {
        let snap = snap.ok_or_else(|| anyhow!("clear_titles: nothing to {what}"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder_item(snap)?;
        uow.commit()?;
        uow.publish_clear_titles_event(self.cleared.clone(), None);
        Ok(())
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for ClearTitlesUseCase {
    fn undo(&mut self) -> Result<()> {
        self.restore(self.snap_before.as_ref(), "undo")
    }

    fn redo(&mut self) -> Result<()> {
        self.restore(self.snap_after.as_ref(), "redo")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
