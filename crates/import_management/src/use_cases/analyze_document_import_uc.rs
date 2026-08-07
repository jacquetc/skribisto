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
    DocumentImportRow, DocumentImportRows, DropPosition, ImportComment, ImportCommentKind,
    ImportDiagnosticRow, ImportDiagnosticRows, ImportOrphanReason, ImportReply, ImportRowKind,
};
use crate::kind_mapping::create_type_to_kind;
use anyhow::{Result, anyhow};
use common::database::QueryUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::{BinderItem, CommentAnchorKind, CommentOrphanReason, Work};
use common::long_operation::{LongOperation, OperationProgress};
use common::types::EntityId;
use std::sync::Arc;

use document_ingest::plan::{PlannedComment, PlannedRow};
use document_ingest::{
    ImportDiagnostic, ImportPlan, ScannerRegistry, SourceDocument, build_plan, infer_rules,
    sort_documents, structure,
};
use skribisto_model::CreateType;

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
#[macros::uow_action(entity = "BinderItem", action = "GetRO")]
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
        // Inside the transaction with the two reads above, not after it: the anchor is
        // a store read like any other, and the destination has to be resolved before
        // the filesystem work begins so a vanished one fails fast rather than after a
        // minute of parsing.
        let (start, base_indent) = self.destination_shape(uow.as_ref())?;

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
        let rules = infer_rules(&structure::levels_used(&docs), start);
        let mut plan = build_plan(&docs, &rules, chapter_mode, base_indent);
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
    /// What the chosen destination means: the kind its shallowest imported row
    /// should be, and the indent it should sit at.
    ///
    /// **Derived, never taken from the caller.** The wizard used to send
    /// `(Book, 0)` whatever the writer had pointed at, so importing into a chapter
    /// produced a book at top level — spliced between that chapter and its scenes,
    /// which then re-parented onto the import. `binder_ordering::resolve_item_target`
    /// is the same function `restore_items_to` resolves the identical question with;
    /// asking it here is what keeps the two features answering alike.
    ///
    /// The kind steps one rung down the ladder from what the destination *is* when
    /// importing **into** a container, and matches it when landing **beside** a row —
    /// exactly what `infer_rules`' own doc prescribes ("importing into a Book means
    /// the shallowest heading is a Part; importing at the root means it is a Book").
    ///
    /// This is a *preview*: `apply_document_import` resolves the indent again at
    /// write time and shifts the accepted rows to match, because the binder can move
    /// between the writer reviewing a plan and accepting it.
    fn destination_shape(
        &self,
        uow: &dyn AnalyzeDocumentImportUnitOfWorkTrait,
    ) -> Result<(CreateType, i64)> {
        // A whole binder: the top level, and the top of the ladder.
        if self.dto.anchor_item_id == 0 || self.dto.binder_id == 0 {
            return Ok((CreateType::Book, 0));
        }
        let Some(anchor) = uow.get_binder_item(&self.dto.anchor_item_id)? else {
            // The row went away between the writer pointing at it and this running.
            // Not fatal: the plan simply previews a top-level import, and apply
            // resolves it again against whatever is there by then.
            return Ok((CreateType::Book, 0));
        };
        let into = matches!(self.dto.drop_position, DropPosition::Into)
            && anchor.role == common::entities::BinderItemRole::Folder;

        let base_indent = if into {
            anchor.indent + 1
        } else {
            anchor.indent
        };
        // An anchor the create vocabulary does not describe, or one off the
        // structural ladder (a note, a paratext, a plain folder): there is no rung to
        // step from, so the import keeps the depth the writer pointed at and starts at
        // the top of the ladder. Better a flat import exactly where they asked than a
        // guessed nesting somewhere else.
        let start = CreateType::of(&anchor.role, &anchor.sub_role)
            .and_then(|kind| structure::start_kind_at(kind, into))
            .unwrap_or(CreateType::Book);
        Ok((start, base_indent))
    }

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
        // Declaration-only fields: these exist so the generated file declares their
        // types. What matters rides inside the rows.
        row_kind: ImportRowKind::default(),
        comment_kind: ImportCommentKind::default(),
        orphan_reason: ImportOrphanReason::default(),
        comment: ImportComment::Empty,
        reply: ImportReply::Empty,
        diagnostic: ImportDiagnosticRow::Empty,
        diagnostics: ImportDiagnosticRows::Reported(diagnostics),
    }
}

/// One planned comment on the wire.
///
/// Field for field, with no interpretation: `document_ingest` already proved this
/// anchor against the very Djot the row carries, using the same matcher the editor
/// re-anchors with. Anything decided again here would be a second opinion about a
/// question that has already been answered correctly.
fn comment_to_dto(comment: &PlannedComment) -> ImportComment {
    ImportComment::Found {
        kind: match comment.kind {
            CommentAnchorKind::Range => ImportCommentKind::Range,
            CommentAnchorKind::Paragraph => ImportCommentKind::Paragraph,
            CommentAnchorKind::Document => ImportCommentKind::Document,
        },
        author_name: comment.author.clone(),
        created_at: comment.created.map(|d| d.to_rfc3339()).unwrap_or_default(),
        body: comment.body.clone(),
        resolved: comment.resolved,
        orphaned: comment.orphaned,
        orphan_reason: match comment.orphan_reason {
            CommentOrphanReason::NotOrphaned => ImportOrphanReason::NotOrphaned,
            CommentOrphanReason::TextNotFound => ImportOrphanReason::TextNotFound,
            CommentOrphanReason::Ambiguous => ImportOrphanReason::Ambiguous,
            CommentOrphanReason::TargetDeleted => ImportOrphanReason::TargetDeleted,
        },
        range_start: comment.anchor.start as i64,
        range_length: comment.anchor.length as i64,
        quote_prefix: comment.anchor.prefix.clone(),
        quote_exact: comment.anchor.exact.clone(),
        quote_exact_truncated: comment.anchor.exact_truncated,
        quote_suffix: comment.anchor.suffix.clone(),
        block_ordinal_hint: comment.anchor.block_ordinal as i64,
        replies: comment
            .replies
            .iter()
            .map(|reply| ImportReply::Found {
                author_name: reply.author.clone(),
                created_at: reply.created.map(|d| d.to_rfc3339()).unwrap_or_default(),
                body: reply.body.clone(),
            })
            .collect(),
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
        comments: row.comments.iter().map(comment_to_dto).collect(),
        origin: row.origin.clone(),
        included: row.included,
    }
}

/// Map one diagnostic onto its wire row.
///
/// `pub(crate)` and re-exported (see `lib.rs`) rather than private, so the UI's
/// "every diagnostic reaches a translated sentence" test can drive the **real**
/// mapping instead of a second copy of it. A test that reimplements the mapping
/// it is checking proves only that the copy agrees with itself.
pub fn diagnostic_to_dto(d: &ImportDiagnostic, row_index: i64) -> ImportDiagnosticRow {
    use ImportDiagnostic::*;

    // `detail` and `count` carry the variant's own payload, so the UI can
    // interpolate them into a translated sentence.
    //
    // Neither field is ever a *separator-packed* pair. `HeadingLevelJump` used to
    // send `"{title}|{from}|{to}"` for the UI to split on `|` — which any title
    // containing a pipe would have broken, silently and only for that writer.
    // Every variant now maps to at most one string and one number, and anything
    // else the sentence needs is read off the row `row_index` names: a row-scoped
    // diagnostic's title and type are already in the plan, so sending them twice
    // only creates two places for them to disagree.
    let (detail, count) = match d {
        FileUnreadable { reason, .. } => (reason.clone(), 0),
        LossyDecode { replacements, .. } => (String::new(), *replacements as i64),
        DecodedFromBom { encoding, .. } => ((*encoding).to_string(), 0),
        EmptyFile { .. } | NoHeadings { .. } => (String::new(), 0),
        UnsupportedFormat { extension, .. } => (extension.clone(), 0),
        FrontMatterNotFlat { key, .. } => (key.clone(), 0),
        FootnotesDegraded { count, .. }
        | RawHtmlDropped { count, .. }
        | NestedBreakDropped { count, .. } => (String::new(), *count as i64),
        ImageNotIngested { target, .. } => (target.clone(), 0),
        TrackedChangesFlattened { count, .. }
        | TextBoxDropped { count, .. }
        | EmbeddedObjectDropped { count, .. }
        | FieldFlattened { count, .. }
        | CommentRepliesFlattened { count, .. } => (String::new(), *count as i64),
        UnknownStyleLevel { style, .. } => (style.clone(), 0),
        // The comment's own opening words, so the writer recognises which note the
        // importer could not place. Its body, not the prose it was about — a
        // sentence naming the prose reads as though the manuscript were at fault.
        CommentUnanchored { quote, .. } => (quote.clone(), 0),
        DuplicateTitle { title, occurrences } => (title.clone(), *occurrences as i64),
        // The two levels, as the two fields — no separator to parse. The title
        // comes from the row.
        HeadingLevelJump { from, to, .. } => (from.to_string(), *to as i64),
        // Nothing: the row carries both the title and the offending type.
        IllegalCombination { .. } => (String::new(), 0),
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
