// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: bulk-create note templates as ONE undoable action.
//
// Two callers share this: importing one or more `.md`/`.djot` files, and applying a named
// preset (Character sheet, Location, …). Both must be a single Ctrl+Z — applying a
// six-template preset that takes six undos to reverse would be worse than not offering
// presets at all. That single-action requirement is the whole reason this use case exists
// rather than the UI looping over the generic `create_note_template` command.
//
// **Colliding names are suffixed, never skipped** — the one place this deliberately
// diverges from `import_tags`, which it otherwise mirrors closely. A skipped tag costs a
// label and a colour; a skipped template silently discards an entire document the writer
// explicitly picked a file for, and it makes "re-import my edited character-sheet.md" a
// permanent no-op with no way to tell that nothing happened. So a second "Character sheet"
// lands as "Character sheet (2)" and the new name is reported back. That keeps the
// invariant the rest of the feature relies on — names are unique within a Work, so the
// insert menu is unambiguous — without ever throwing content away.
//
// Undo strategy: a Work-scoped snapshot/restore, matching `import_tags` and `empty_trash`.
// Snapshots are O(1) (structural sharing), so scoping to the Work costs nothing and covers
// both the new rows and the `Work.note_templates` list they were appended to in one restore.
use crate::ImportNoteTemplatesDto;
use crate::ImportNoteTemplatesResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{NoteTemplate, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::HashSet;

pub trait ImportNoteTemplatesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ImportNoteTemplatesUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/import_note_templates_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "NoteTemplate", action = "GetMulti")]
#[macros::uow_action(entity = "NoteTemplate", action = "CreateOrphan")]
pub trait ImportNoteTemplatesUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_import_note_templates_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

/// The comparison key for "already in this project". Trimmed and lowercased, matching the
/// duplicate check the settings pane and the save-as-template dialog apply while typing, so
/// import and the UI agree on what counts as a collision.
fn name_key(name: &str) -> String {
    name.trim().to_lowercase()
}

/// The first free `"<base> (n)"` for a name already taken, starting at 2.
///
/// Terminates by construction: every iteration tests a distinct candidate and `taken` is
/// finite, so the search ends at worst one past the number of rows already present.
fn disambiguate(base: &str, taken: &HashSet<String>) -> String {
    let mut n = 2usize;
    loop {
        let candidate = format!("{base} ({n})");
        if !taken.contains(&name_key(&candidate)) {
            return candidate;
        }
        n += 1;
    }
}

pub struct ImportNoteTemplatesUseCase {
    uow_factory: Box<dyn ImportNoteTemplatesUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl ImportNoteTemplatesUseCase {
    pub fn new(uow_factory: Box<dyn ImportNoteTemplatesUnitOfWorkFactoryTrait>) -> Self {
        ImportNoteTemplatesUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(
        &mut self,
        dto: &ImportNoteTemplatesDto,
    ) -> Result<ImportNoteTemplatesResultDto> {
        // The three vectors are one table transposed into columns (DTOs carry only
        // primitives, so a `Vec<NoteTemplateSpec>` is not expressible). A length mismatch
        // means the caller built them inconsistently and would otherwise silently truncate.
        let n = dto.names.len();
        if dto.bodies.len() != n || dto.starreds.len() != n {
            return Err(anyhow!(
                "import_note_templates: column lengths differ (names {}, bodies {}, starreds {})",
                n,
                dto.bodies.len(),
                dto.starreds.len()
            ));
        }
        if n == 0 {
            return Err(anyhow!("import_note_templates: nothing to import"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Import into the caller-named Work only — several may be open at once.
        let work_id = dto.work_id as EntityId;
        uow.get_all_work()?
            .into_iter()
            .find(|w| w.id == work_id)
            .ok_or_else(|| anyhow!("work {work_id} is not open"))?;

        let mut template_ids =
            uow.get_work_relationship(&work_id, &WorkRelationshipField::NoteTemplates)?;
        let existing: Vec<NoteTemplate> = uow
            .get_note_template_multi(&template_ids)?
            .into_iter()
            .flatten()
            .collect();
        // Seeded with what the project already has, then grown as rows are accepted, so a
        // batch that names the same template twice suffixes the second one rather than
        // creating two rows nothing downstream could tell apart.
        let mut taken: HashSet<String> = existing.iter().map(|t| name_key(&t.name)).collect();

        let snap_before = uow.snapshot_work(&[work_id])?;

        let now = chrono::Utc::now();
        let mut created_ids: Vec<EntityId> = Vec::new();
        let mut renamed_to: Vec<String> = Vec::new();

        for i in 0..n {
            let raw = dto.names[i].trim();
            // A blank name would leave an unpickable row in the insert menu. It is the one
            // case with nothing to disambiguate *to*, so it is the only thing skipped here.
            if raw.is_empty() {
                continue;
            }
            let name = if taken.contains(&name_key(raw)) {
                let fresh = disambiguate(raw, &taken);
                renamed_to.push(fresh.clone());
                fresh
            } else {
                raw.to_string()
            };
            taken.insert(name_key(&name));

            let created = uow.create_orphan_note_template(&NoteTemplate {
                created_at: now,
                updated_at: now,
                name,
                body: dto.bodies[i].clone(),
                starred: dto.starreds[i],
                ..Default::default()
            })?;
            created_ids.push(created.id);
        }

        if !created_ids.is_empty() {
            // Appended, never prepended: the relationship is ordered, and an import must
            // not disturb the arrangement the writer chose for the rows already there.
            template_ids.extend(created_ids.iter().copied());
            uow.set_work_relationship(
                &work_id,
                &WorkRelationshipField::NoteTemplates,
                &template_ids,
            )?;
        }

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_import_note_templates_event(created_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(ImportNoteTemplatesResultDto {
            created_ids,
            renamed_to,
        })
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for ImportNoteTemplatesUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("import_note_templates: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("import_note_templates: nothing to redo"))?;
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
