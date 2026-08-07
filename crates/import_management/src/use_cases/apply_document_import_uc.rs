// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation (hand-maintained — do NOT blanket-regenerate): the write
// half of document import. It takes the plan the writer reviewed and accepted and
// creates it, as one undo entry.
//
// Synchronous, and deliberately so. Qleany drops `undoable` under
// `long_operation`, and the expensive part — reading and converting files —
// already happened in `analyze_document_import`. Rows arrive with their Djot
// already built, so this opens a transaction, creates, splices and commits
// without parsing anything.
//
// Undo is `snapshot_binder` / `restore_binder`, the same pair `duplicate_uc`
// uses. It costs a walk of the whole destination binder rather than of what was
// imported; that is the accepted trade for reusing a proven primitive instead of
// building a new one.
//
// Three traps this has to step around, none of which the backend will catch:
//
//  1. `is_exportable` and `activated` both default to `false`, because their DTO
//     derives `Default` and `bool::default()` is false. A row created without
//     setting them is present in the binder and invisible to export and to
//     chapter numbering — with no gap left behind, since chapters renumber to
//     fill it. They are set explicitly on every row below.
//  2. `validate_item` is called only by the UI. Nothing at this layer rejects an
//     illegal `(role, sub_role)`, so this checks the combination itself.
//  3. `content_allowed` is enforced only at save time, where it *drops the row
//     silently*. Prose that its type cannot hold is refused here, while the
//     writer can still see why.
//
// The `#[macros::uow_action(...)]` list below is hand-trimmed to what this
// actually does, and must stay in lockstep with the identical list in
// ../units_of_work/apply_document_import_uow.rs.

use crate::ApplyDocumentImportDto;
use crate::ApplyDocumentImportResultDto;
use crate::dtos::{ApplyImportRow, ApplyImportRows};
use crate::use_cases::analyze_document_import_uc::kind_to_create_type;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItem, Content, ContentRole, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use skribisto_model::{allowed_content, content_allowed, is_valid_combination};

pub trait ApplyDocumentImportUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ApplyDocumentImportUnitOfWorkTrait>;
}

// `Work` for `chapter_mode` (how a Chapter is encoded, with no per-item
// override). `Binder` to read the destination order, splice into it, and
// snapshot it for undo. `BinderItem` + `Content` to create the rows themselves.
#[macros::uow_action(entity = "Work", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
pub trait ApplyDocumentImportUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_apply_document_import_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct ApplyDocumentImportUseCase {
    uow_factory: Box<dyn ApplyDocumentImportUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl ApplyDocumentImportUseCase {
    pub fn new(uow_factory: Box<dyn ApplyDocumentImportUnitOfWorkFactoryTrait>) -> Self {
        ApplyDocumentImportUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(
        &mut self,
        dto: &ApplyDocumentImportDto,
    ) -> Result<ApplyDocumentImportResultDto> {
        let ApplyImportRows::Create(rows) = &dto.rows else {
            return Err(anyhow!("apply_document_import: no rows to create"));
        };
        let rows: Vec<&ApplyImportRow> = rows
            .iter()
            .filter(|r| !matches!(r, ApplyImportRow::Empty))
            .collect();
        if rows.is_empty() {
            return Err(anyhow!("apply_document_import: no rows to create"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let work = uow
            .get_work(&dto.work_id)?
            .ok_or_else(|| anyhow!("apply_document_import: work {} not found", dto.work_id))?;
        let chapter_mode = work.chapter_mode.clone();

        let snap_before = uow.snapshot_binder(&[dto.binder_id])?;
        let order =
            uow.get_binder_relationship(&dto.binder_id, &BinderRelationshipField::BinderItems)?;

        // Everything the import creates gets one timestamp, the way every other
        // creation path in this codebase does — a row's `created_at` records the
        // import, not how long the parsing took.
        let now = chrono::Utc::now();
        let mut created_ids: Vec<EntityId> = Vec::with_capacity(rows.len());

        for row in &rows {
            let ApplyImportRow::Create {
                indent,
                kind,
                title,
                djot,
            } = row
            else {
                continue;
            };

            let (role, sub_role) = kind_to_create_type(kind).combo(chapter_mode.clone());

            // Trap 2: nothing below this layer checks the combination.
            if !is_valid_combination(&role, &sub_role) {
                return Err(anyhow!(
                    "apply_document_import: '{title}' resolves to an invalid {role:?}/{sub_role:?}"
                ));
            }

            // Trap 3: prose whose row cannot hold it would be dropped silently at
            // the next save. Refuse it here, where the writer can still be told.
            let content_id = if djot.trim().is_empty() {
                None
            } else {
                let content_role = prose_role_for(&role, &sub_role).ok_or_else(|| {
                    anyhow!(
                        "apply_document_import: '{title}' is a {role:?}/{sub_role:?}, \
                         which holds no prose"
                    )
                })?;
                let created = uow.create_orphan_content(&Content {
                    created_at: now,
                    updated_at: now,
                    activated: true,
                    role: content_role,
                    data: djot.clone(),
                    ..Default::default()
                })?;
                Some(created.id)
            };

            let item = uow.create_orphan_binder_item(&BinderItem {
                // A created row mints its own durable identity. Everything
                // persisted about an item keys on this — including, since the
                // prose-filename fix, its own file on disk.
                uid: common::uid::new_uid(),
                created_at: now,
                updated_at: now,
                title: title.clone(),
                role,
                sub_role,
                // Trap 1: both default to false, and a row that is neither
                // exportable nor activated is present but invisible to export and
                // to chapter numbering.
                activated: true,
                is_exportable: true,
                indent: *indent,
                ..Default::default()
            })?;

            if let Some(content_id) = content_id {
                uow.set_binder_item_relationship(
                    &item.id,
                    &BinderItemRelationshipField::Contents,
                    &[content_id],
                )?;
            }

            created_ids.push(item.id);
        }

        // Splice the new block in as one run, after the anchor or at the end.
        // Keeping it contiguous is what makes the import one movable, one
        // undoable thing rather than rows scattered through the binder.
        let new_order = splice_after(&order, dto.anchor_item_id, &created_ids)?;
        uow.set_binder_relationship(
            &dto.binder_id,
            &BinderRelationshipField::BinderItems,
            &new_order,
        )?;

        let snap_after = uow.snapshot_binder(&[dto.binder_id])?;
        uow.commit()?;
        uow.publish_apply_document_import_event(created_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(ApplyDocumentImportResultDto { created_ids })
    }
}

/// The `ContentRole` a row of this type stores its prose under, or `None` when
/// the type holds no prose at all.
///
/// Asks `skribisto_model` rather than deciding: the constraint matrix is the one
/// authority on what a combination may carry, and a second copy of that judgement
/// here would drift.
fn prose_role_for(
    role: &common::entities::BinderItemRole,
    sub_role: &common::entities::BinderItemSubRole,
) -> Option<ContentRole> {
    for candidate in [
        ContentRole::SceneText,
        ContentRole::NoteText,
        ContentRole::ParatextText,
    ] {
        if content_allowed(role, sub_role, &candidate) {
            return Some(candidate);
        }
    }
    // Nothing prose-shaped is allowed here. `allowed_content` is consulted so the
    // reason is visible in a debugger rather than inferred.
    let _ = allowed_content(role, sub_role);
    None
}

/// `order` with `block` inserted after `anchor`, or appended when `anchor` is 0.
///
/// An anchor that is not in the binder is an error rather than a silent append:
/// the writer chose a destination, and quietly putting a hundred rows somewhere
/// else is the kind of "it worked, just not where you said" outcome this whole
/// feature exists to avoid.
fn splice_after(order: &[EntityId], anchor: EntityId, block: &[EntityId]) -> Result<Vec<EntityId>> {
    if anchor == 0 {
        let mut new_order = order.to_vec();
        new_order.extend_from_slice(block);
        return Ok(new_order);
    }
    let at = order
        .iter()
        .position(|id| *id == anchor)
        .ok_or_else(|| anyhow!("apply_document_import: anchor {anchor} is not in this binder"))?;

    let mut new_order = Vec::with_capacity(order.len() + block.len());
    new_order.extend_from_slice(&order[..=at]);
    new_order.extend_from_slice(block);
    new_order.extend_from_slice(&order[at + 1..]);
    Ok(new_order)
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for ApplyDocumentImportUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("apply_document_import: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("apply_document_import: nothing to redo"))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_block_appends_when_there_is_no_anchor() {
        let order = vec![1, 2, 3];
        assert_eq!(
            splice_after(&order, 0, &[7, 8]).unwrap(),
            vec![1, 2, 3, 7, 8]
        );
    }

    #[test]
    fn a_block_lands_immediately_after_its_anchor() {
        let order = vec![1, 2, 3];
        assert_eq!(
            splice_after(&order, 2, &[7, 8]).unwrap(),
            vec![1, 2, 7, 8, 3]
        );
    }

    #[test]
    fn an_anchor_outside_the_binder_is_refused_rather_than_appended() {
        assert!(splice_after(&[1, 2, 3], 99, &[7]).is_err());
    }

    #[test]
    fn a_scene_stores_its_prose_as_scene_text() {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        assert_eq!(
            prose_role_for(&BinderItemRole::Item, &BinderItemSubRole::Scene),
            Some(ContentRole::SceneText)
        );
    }

    #[test]
    fn a_book_holds_no_prose_and_says_so() {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        assert_eq!(
            prose_role_for(&BinderItemRole::Folder, &BinderItemSubRole::Book),
            None
        );
    }
}
