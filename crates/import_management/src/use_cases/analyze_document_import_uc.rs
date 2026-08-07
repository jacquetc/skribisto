// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation (hand-maintained — do NOT blanket-regenerate): the read
// half of document import. It reads the chosen files, works out what tree they
// would make, and returns that plan for the writer to review. It writes nothing.
//
// This is Qleany's **Scenario 4** (`qleany docs flow`): a read-only long
// operation crunches and returns a preview, the writer reviews it, and
// `apply_document_import` — a regular undoable use case — creates what they
// accepted. The split is not a workaround for `undoable` being dropped under
// `long_operation`; it is what makes a review step possible at all, since
// nothing exists to undo until the writer has said yes.
//
// **Long operation for the IO, not the arithmetic.** Converting 78,000 words of
// Markdown to Djot measures at about 5 ms, so the parsing is never the cost. A
// dropped folder is hundreds of individual file reads, and a synchronous use
// case in this codebase runs on the UI's own thread. `cancel_flag` is polled per
// file so a writer who dropped the wrong directory can stop it.
//
// **Format-neutral by name and by shape.** Nothing here knows about Markdown:
// `document_ingest::ScannerRegistry` picks a scanner from each file's extension,
// and the plan rows carry nothing that says which one ran. Adding ODT or DOCX is
// a new `SourceScanner` in that crate — not a second use case, not a second
// regen, not a near-duplicate of the Apply DTO beside this one.
//
// The `#[macros::uow_action(...)]` list below is hand-trimmed to what this
// actually reads, and must stay in lockstep with the identical list in
// ../units_of_work/analyze_document_import_uow.rs.

use crate::AnalyzeDocumentImportDto;
use crate::DocumentImportPlanDto;
use crate::dtos::{
    DocumentImportRow, DocumentImportRows, ImportDiagnosticRow, ImportDiagnosticRows, ImportRowKind,
};
use anyhow::{Result, anyhow};
use common::database::QueryUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::{BinderItem, Work};
use common::long_operation::{LongOperation, OperationProgress};
use common::types::EntityId;
use std::sync::Arc;

use document_ingest::plan::PlannedRow;
use document_ingest::{
    ImportDiagnostic, ImportPlan, ScannerRegistry, SourceDocument, build_plan, infer_rules,
    sort_documents, structure,
};

pub trait AnalyzeDocumentImportUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn AnalyzeDocumentImportUnitOfWorkTrait>;
}

// Read-only unit of work: `*RO` actions only, never mixed with write actions.
//
// `Work` for its `chapter_mode`, which decides how a detected Chapter is encoded
// and has no per-item override. `Binder` + `BinderItem` to read the destination's
// existing titles, so a re-import of an edited file can be flagged before it
// doubles a manuscript — an ordinary workflow now that importing into the open
// project is the only path there is.
#[macros::uow_action(entity = "Work", action = "GetRO")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "BinderItem", action = "GetMultiRO")]
pub trait AnalyzeDocumentImportUnitOfWorkTrait: QueryUnitOfWork + Send + Sync {
    fn publish_analyze_document_import_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct AnalyzeDocumentImportUseCase {
    uow_factory: Box<dyn AnalyzeDocumentImportUnitOfWorkFactoryTrait>,
    dto: AnalyzeDocumentImportDto,
}

impl AnalyzeDocumentImportUseCase {
    pub fn new(
        uow_factory: Box<dyn AnalyzeDocumentImportUnitOfWorkFactoryTrait>,
        dto: &AnalyzeDocumentImportDto,
    ) -> Self {
        AnalyzeDocumentImportUseCase {
            uow_factory,
            dto: dto.clone(),
        }
    }
}

impl LongOperation for AnalyzeDocumentImportUseCase {
    type Output = DocumentImportPlanDto;

    fn execute(
        &self,
        progress_callback: Box<dyn Fn(OperationProgress) + Send>,
        cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<Self::Output> {
        use std::sync::atomic::Ordering;

        let uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Read the destination's context before touching the filesystem, so a
        // vanished Work fails fast rather than after a minute of parsing.
        let work = uow.get_work(&self.dto.work_id)?.ok_or_else(|| {
            anyhow!(
                "analyze_document_import: work {} not found",
                self.dto.work_id
            )
        })?;
        let chapter_mode = work.chapter_mode.clone();
        let existing_titles = self.existing_titles(uow.as_ref())?;

        uow.end_transaction()?;

        // ── read the files ──────────────────────────────────────────────────
        //
        // One file at a time, reporting as it goes and checking for cancellation
        // between each, because this is the part that can actually take a while.
        // A file that cannot be read becomes a diagnostic on its own document
        // rather than an error: five good files and one unreadable one must
        // import five, which is exactly what every importer in the survey behind
        // this feature got wrong.
        let registry = ScannerRegistry::with_builtin_scanners();
        let total = self.dto.source_paths.len().max(1);
        let mut docs: Vec<SourceDocument> = Vec::with_capacity(self.dto.source_paths.len());

        for (index, path) in self.dto.source_paths.iter().enumerate() {
            if cancel_flag.load(Ordering::Relaxed) {
                return Err(anyhow!("Operation was cancelled"));
            }
            progress_callback(OperationProgress::new(
                (index as f32 / total as f32) * 90.0,
                Some(path.clone()),
            ));

            let path_buf = std::path::PathBuf::from(path);
            match std::fs::read(&path_buf) {
                Ok(bytes) => docs.push(registry.scan_bytes(&path_buf, &bytes)),
                Err(err) => {
                    let mut doc = SourceDocument::new(
                        document_ingest::scanner::display_name_for(&path_buf),
                        path,
                    );
                    doc.diagnostics.push(ImportDiagnostic::FileUnreadable {
                        path: path.clone(),
                        reason: err.to_string(),
                    });
                    docs.push(doc);
                }
            }
        }

        // ── work out the tree ───────────────────────────────────────────────
        progress_callback(OperationProgress::new(
            92.0,
            Some("Working out the structure".to_string()),
        ));

        sort_documents(&mut docs);
        let rules = infer_rules(
            &structure::levels_used(&docs),
            kind_to_create_type(&self.dto.start_kind),
        );
        let mut plan = build_plan(&docs, &rules, chapter_mode, self.dto.base_indent);
        flag_existing_titles(&mut plan, &existing_titles);

        progress_callback(OperationProgress::new(100.0, Some("completed".to_string())));

        // The feature event alongside the framework's own `LongOperation(Completed)`,
        // the way the Plume importer does: the long-op event says an operation
        // finished, this one says *which* Work's import was analysed, which is what
        // a subscriber actually filters on.
        uow.publish_analyze_document_import_event(vec![self.dto.work_id], None);

        Ok(to_dto(plan))
    }
}

impl AnalyzeDocumentImportUseCase {
    /// Titles already present in the destination binder, lower-cased.
    ///
    /// A binder that cannot be read is not fatal: the warning it would have
    /// produced is a convenience, and refusing to import over it would be a much
    /// worse trade than importing without the warning.
    fn existing_titles(
        &self,
        uow: &dyn AnalyzeDocumentImportUnitOfWorkTrait,
    ) -> Result<Vec<String>> {
        if self.dto.binder_id == 0 {
            return Ok(Vec::new());
        }
        let item_ids = uow
            .get_binder_relationship(&self.dto.binder_id, &BinderRelationshipField::BinderItems)?;
        Ok(uow
            .get_binder_item_multi(&item_ids)?
            .into_iter()
            .flatten()
            .map(|i| i.title.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect())
    }
}

/// Warn when an imported title already exists at the destination.
///
/// This is what a second import of the same files looks like, and with import
/// into the open project being the only path, "fix one file and import again" is
/// an ordinary thing to do. Nothing else in the system would notice it doubling
/// a manuscript.
fn flag_existing_titles(plan: &mut ImportPlan, existing: &[String]) {
    if existing.is_empty() {
        return;
    }
    for row in &plan.rows {
        let key = row.title.trim().to_lowercase();
        if !key.is_empty() && existing.contains(&key) {
            plan.diagnostics.push(ImportDiagnostic::DuplicateTitle {
                title: row.title.clone(),
                occurrences: 2,
            });
        }
    }
}

/// The DTO's flat kind ↔ the model's create vocabulary.
///
/// Two enums for one idea, because a Qleany DTO cannot name a type from another
/// crate. Kept as an exhaustive `match` in both directions rather than a cast, so
/// adding a `CreateType` fails to compile here instead of silently importing
/// everything as a Scene.
pub(crate) fn kind_to_create_type(kind: &ImportRowKind) -> skribisto_model::CreateType {
    use skribisto_model::CreateType as C;
    match kind {
        ImportRowKind::Book => C::Book,
        ImportRowKind::Part => C::Part,
        ImportRowKind::Chapter => C::Chapter,
        ImportRowKind::Scene => C::Scene,
        ImportRowKind::Note => C::Note,
        ImportRowKind::NoteFolder => C::NoteFolder,
        ImportRowKind::Folder => C::Folder,
        ImportRowKind::Paratext => C::Paratext,
        ImportRowKind::ParatextFolder => C::ParatextFolder,
        ImportRowKind::EndOfBook => C::EndOfBook,
    }
}

pub(crate) fn create_type_to_kind(kind: skribisto_model::CreateType) -> ImportRowKind {
    use skribisto_model::CreateType as C;
    match kind {
        C::Book => ImportRowKind::Book,
        C::Part => ImportRowKind::Part,
        C::Chapter => ImportRowKind::Chapter,
        C::Scene => ImportRowKind::Scene,
        C::Note => ImportRowKind::Note,
        C::NoteFolder => ImportRowKind::NoteFolder,
        C::Folder => ImportRowKind::Folder,
        C::Paratext => ImportRowKind::Paratext,
        C::ParatextFolder => ImportRowKind::ParatextFolder,
        C::EndOfBook => ImportRowKind::EndOfBook,
    }
}

/// Flatten the plan into the wire shape.
///
/// Diagnostics travel as a key plus their data, never a rendered sentence: the
/// UI says them in the writer's own locale, which a pre-built English string
/// would make impossible.
fn to_dto(plan: ImportPlan) -> DocumentImportPlanDto {
    let rows: Vec<DocumentImportRow> = plan.rows.iter().map(row_to_dto).collect();

    let mut diagnostics: Vec<ImportDiagnosticRow> = plan
        .diagnostics
        .iter()
        .map(|d| diagnostic_to_dto(d, -1))
        .collect();
    for (index, row) in plan.rows.iter().enumerate() {
        for d in &row.diagnostics {
            diagnostics.push(diagnostic_to_dto(d, index as i64));
        }
    }

    DocumentImportPlanDto {
        row: DocumentImportRow::Empty,
        rows: DocumentImportRows::Found(rows),
        diagnostic: ImportDiagnosticRow::Empty,
        diagnostics: ImportDiagnosticRows::Reported(diagnostics),
    }
}

fn row_to_dto(row: &PlannedRow) -> DocumentImportRow {
    DocumentImportRow::Found {
        indent: row.indent,
        kind: create_type_to_kind(row.create_type),
        title: row.title.clone(),
        stripped_ordinal: row.stripped_ordinal.clone().unwrap_or_default(),
        djot: row.djot.clone(),
        scene_breaks: row.scene_breaks as i64,
        word_count: row.word_count as i64,
        origin: row.origin.clone(),
        included: row.included,
    }
}

fn diagnostic_to_dto(d: &ImportDiagnostic, row_index: i64) -> ImportDiagnosticRow {
    use ImportDiagnostic::*;

    // `detail` and `count` carry the variant's own payload, so the UI can
    // interpolate them into a translated sentence.
    let (detail, count) = match d {
        FileUnreadable { reason, .. } => (reason.clone(), 0),
        LossyDecode { replacements, .. } => (String::new(), *replacements as i64),
        DecodedFromBom { encoding, .. } => ((*encoding).to_string(), 0),
        EmptyFile { .. } | NoHeadings { .. } => (String::new(), 0),
        UnsupportedFormat { extension, .. } => (extension.clone(), 0),
        FrontMatterNotFlat { key, .. } => (key.clone(), 0),
        FootnotesDegraded { count, .. } | RawHtmlDropped { count, .. } => {
            (String::new(), *count as i64)
        }
        ImageNotIngested { target, .. } => (target.clone(), 0),
        DuplicateTitle { title, occurrences } => (title.clone(), *occurrences as i64),
        HeadingLevelJump { title, from, to } => (format!("{title}|{from}|{to}"), *to as i64),
        IllegalCombination { detail, .. } => (detail.clone(), 0),
    };

    ImportDiagnosticRow::Reported {
        key: d.key().to_string(),
        severity: format!("{:?}", d.severity()).to_lowercase(),
        path: d.path().unwrap_or_default().to_string(),
        detail,
        count,
        row_index,
    }
}
