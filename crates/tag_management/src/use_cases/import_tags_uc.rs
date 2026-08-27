// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: bulk-create palette tags as ONE undoable action.
//
// Two callers share this: CSV import, and applying a named preset (Basic, Sci-fi, …).
// Both must be a single Ctrl+Z — applying a twelve-tag preset that takes twelve undos to
// reverse would be worse than not offering presets at all. That single-action requirement
// is the whole reason this use case exists rather than the UI looping over the generic
// `create_binder_tag` command.
//
// Names collide silently by design: the backend allows duplicate tag names (the UI warns,
// but does not forbid). Import is the one place that would turn a warning into a mess —
// re-applying a preset, or re-importing an edited CSV, would otherwise double every row —
// so a name already in the palette is skipped and reported, case-insensitively. That also
// makes applying "Sci-fi" after "Basic" add only the genuinely new tags.
//
// Undo strategy: a Work-scoped snapshot/restore, matching `empty_trash`. Snapshots are
// O(1) (structural sharing), so scoping to the Work costs nothing and covers both the new
// rows and the `Work.tags` list they were appended to in one restore.
use crate::ImportTagsDto;
use crate::ImportTagsResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderTag, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::HashSet;

pub trait ImportTagsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ImportTagsUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/import_tags_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "BinderTag", action = "GetMulti")]
#[macros::uow_action(entity = "BinderTag", action = "CreateOrphan")]
pub trait ImportTagsUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_import_tags_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

/// The comparison key for "already in the palette". Trimmed and lowercased, matching the
/// duplicate-name warning the UI shows while typing, so import and the UI agree on what
/// counts as a collision.
fn name_key(name: &str) -> String {
    name.trim().to_lowercase()
}

pub struct ImportTagsUseCase {
    uow_factory: Box<dyn ImportTagsUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl ImportTagsUseCase {
    pub fn new(uow_factory: Box<dyn ImportTagsUnitOfWorkFactoryTrait>) -> Self {
        ImportTagsUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &ImportTagsDto) -> Result<ImportTagsResultDto> {
        // The four vectors are one table transposed into columns (DTOs carry only
        // primitives, so a Vec<TagSpec> is not expressible). A length mismatch means the
        // caller built them inconsistently and would otherwise silently truncate.
        let n = dto.names.len();
        if dto.colors.len() != n || dto.details.len() != n || dto.discoverables.len() != n {
            return Err(anyhow!(
                "import_tags: column lengths differ (names {}, colors {}, details {}, discoverables {})",
                n,
                dto.colors.len(),
                dto.details.len(),
                dto.discoverables.len()
            ));
        }
        if n == 0 {
            return Err(anyhow!("import_tags: nothing to import"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Import into the caller-named Work only — several may be open at once.
        let work_id = dto.work_id as EntityId;
        uow.get_all_work()?
            .into_iter()
            .find(|w| w.id == work_id)
            .ok_or_else(|| anyhow!("work {work_id} is not open"))?;

        let mut tag_ids = uow.get_work_relationship(&work_id, &WorkRelationshipField::Tags)?;
        let existing: Vec<BinderTag> = uow
            .get_binder_tag_multi(&tag_ids)?
            .into_iter()
            .flatten()
            .collect();
        // Seeded with what is already in the palette, then grown as rows are accepted, so
        // a batch containing the same name twice imports it once.
        let mut seen: HashSet<String> = existing.iter().map(|t| name_key(&t.name)).collect();

        let snap_before = uow.snapshot_work(&[work_id])?;

        let now = chrono::Utc::now();
        let mut created_ids: Vec<EntityId> = Vec::new();
        let mut skipped_names: Vec<String> = Vec::new();

        for i in 0..n {
            let name = dto.names[i].trim();
            // A blank name would produce an unclickable, unnameable chip.
            if name.is_empty() {
                skipped_names.push(dto.names[i].clone());
                continue;
            }
            if !seen.insert(name_key(name)) {
                skipped_names.push(dto.names[i].clone());
                continue;
            }
            let created = uow.create_orphan_binder_tag(&BinderTag {
                // Minted here, not left to `..Default::default()`: this path writes through
                // the unit of work and so never reaches `binder_tag_controller`'s
                // `with_identity`. Every preset tag and every CSV row would otherwise be
                // born nil-identified, and nil uids all compare equal: `note_capture.toml`'s
                // `recent_tags` would collapse into one always-matching slot, and a save/open
                // round trip could not tell one preset tag from another.
                uid: common::uid::new_uid(),
                created_at: now,
                updated_at: now,
                name: name.to_string(),
                color: dto.colors[i].clone(),
                details: dto.details[i].clone(),
                discoverable: dto.discoverables[i],
                ..Default::default()
            })?;
            created_ids.push(created.id);
        }

        if !created_ids.is_empty() {
            tag_ids.extend(created_ids.iter().copied());
            uow.set_work_relationship(&work_id, &WorkRelationshipField::Tags, &tag_ids)?;
        }

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_import_tags_event(created_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(ImportTagsResultDto {
            created_ids,
            skipped_names,
        })
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for ImportTagsUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("import_tags: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("import_tags: nothing to redo"))?;
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
