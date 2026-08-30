// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End to end: real files on disk, through both halves, into a real store.
//!
//! The unit tests in `document_ingest` pin the pipeline and the ones beside the
//! use cases pin their pieces, but a use case nobody calls end to end is not
//! meaningfully tested — the two halves are only correct *together*, and the
//! seam between them (a plan crossing the DTO boundary and coming back as rows
//! to create) is precisely where a mistake would survive both unit suites.
//!
//! Everything here goes through the real controllers, the real transaction, and
//! the real undo stack.

use std::sync::Arc;

use common::database::db_context::DbContext;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{ChapterMode, ContentRole};
use common::event::EventHub;
use common::long_operation::LongOperationManager;
use common::types::EntityId;
use common::undo_redo::UndoRedoManager;
use direct_access::binder::binder_controller;
use direct_access::binder::dtos::CreateBinderDto;
use direct_access::binder_item::binder_item_controller;
use direct_access::binder_item::dtos::{BinderItemDto, UpdateBinderItemDto};
use direct_access::content::content_controller;
use direct_access::root::dtos::CreateRootDto;
use direct_access::root::root_controller;
use direct_access::smart_punctuation::dtos::CreateSmartPunctuationDto;
use direct_access::smart_punctuation::smart_punctuation_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;
use import_management::import_management_controller;
use import_management::{
    AnalyzeDocumentImportDto, ApplyDocumentImportDto, ApplyDocumentImportResultDto, ApplyImportRow,
    ApplyImportRows, DocumentImportRow, DocumentImportRows, DropPosition, ImportComment,
    ImportCommentKind, ImportDiagnosticRows, ImportOrphanReason, ImportRowKind,
};

/// A read DTO as the update DTO of the same row, changing nothing.
///
/// The generated update DTO carries every field, so a test that wants to flip one has to
/// restate all of them; this keeps that in one place.
fn to_update_dto(item: BinderItemDto) -> UpdateBinderItemDto {
    UpdateBinderItemDto {
        id: item.id,
        created_at: item.created_at,
        updated_at: item.updated_at,
        uid: item.uid,
        title: item.title,
        sub_title: item.sub_title,
        role: item.role,
        sub_role: item.sub_role,
        label: item.label,
        activated: item.activated,
        is_favorite: item.is_favorite,
        is_exportable: item.is_exportable,
        exclude_from_numbering: item.exclude_from_numbering,
        indent: item.indent,
        word_count_goal: item.word_count_goal,
        char_count_goal: item.char_count_goal,
        dict_language: item.dict_language,
        aliases: item.aliases,
    }
}
// M-S7 round-trip tests: build a real "returning" `.docx`/`.odt` via
// `text-document`'s own writer, carrying real local `Comment`/`CommentReply`
// uids — see the `Ctx::write_bytes` doc and the tests themselves, grouped under
// the "M-S7: recognition on re-import" section near the bottom of this file.
use text_document::{
    CommentReply as TdCommentReply, DocumentComment, DocumentComments, DocxExportOptions,
    FindOptions, OdtExportOptions, TextDocument,
};

struct Ctx {
    db: DbContext,
    hub: Arc<EventHub>,
    undo: UndoRedoManager,
    long_ops: LongOperationManager,
    work_id: EntityId,
    binder_id: EntityId,
    _dir: tempfile::TempDir,
}

impl Ctx {
    fn new() -> Self {
        let db = DbContext::new().expect("in-memory store");
        let hub = Arc::new(EventHub::new());
        let mut undo = UndoRedoManager::new();
        undo.set_event_hub(&hub);
        let long_ops = LongOperationManager::new();

        let root_id = root_controller::create_orphan(&db, &hub, &CreateRootDto::default())
            .expect("root")
            .id;
        let smart_punctuation = smart_punctuation_controller::create_orphan(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateSmartPunctuationDto::default(),
        )
        .expect("smart_punctuation")
        .id;
        let work_id = work_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                smart_punctuation,
                chapter_mode: ChapterMode::Folder,
                // A durable id, as every saved project has. `Default` leaves it
                // empty — which is what an **unsaved** project carries, and what
                // makes it the wrong fixture for anything asking which project
                // something happened to.
                unique_id: "document-import-fixture-uid".to_string(),
                ..Default::default()
            },
            root_id,
            -1,
        )
        .expect("work")
        .id;
        let binder_id = binder_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateBinderDto {
                activated: true,
                ..Default::default()
            },
            work_id,
            -1,
        )
        .expect("binder")
        .id;

        Ctx {
            db,
            hub,
            undo,
            long_ops,
            work_id,
            binder_id,
            _dir: tempfile::tempdir().expect("scratch dir"),
        }
    }

    fn write(&self, name: &str, body: &str) -> String {
        let path = self._dir.path().join(name);
        std::fs::write(&path, body).expect("write fixture");
        path.to_string_lossy().to_string()
    }

    /// As [`Ctx::write`], for a binary container (`.docx`/`.odt`) — the M-S7
    /// round-trip tests build these fresh via `text-document`'s own writer
    /// rather than reading a checked-in fixture, so they can carry the exact
    /// local `Comment`/`CommentReply` uids a given test needs to recognise.
    fn write_bytes(&self, name: &str, bytes: &[u8]) -> String {
        let path = self._dir.path().join(name);
        std::fs::write(&path, bytes).expect("write fixture");
        path.to_string_lossy().to_string()
    }

    /// Run ANALYSE to completion and hand back its plan.
    ///
    /// The long-operation manager runs it on its own thread, so this polls for
    /// the result the way the UI does rather than reaching past the framework.
    /// Analyse into the binder itself (the top level).
    fn analyse(&mut self, paths: Vec<String>, _start: ImportRowKind) -> Vec<DocumentImportRow> {
        self.analyse_at(paths, 0, DropPosition::Into)
    }

    /// Analyse against a destination, as the wizard does.
    ///
    /// `start_kind` and `base_indent` are no longer the caller's to choose — they are
    /// derived from where the import is going, which is the whole of finding #1.
    fn analyse_at(
        &mut self,
        paths: Vec<String>,
        anchor_item_id: EntityId,
        drop_position: DropPosition,
    ) -> Vec<DocumentImportRow> {
        let dto = AnalyzeDocumentImportDto {
            work_id: self.work_id,
            binder_id: self.binder_id,
            source_paths: paths,
            anchor_item_id,
            drop_position,
        };
        let op = import_management_controller::analyze_document_import(
            &self.db,
            &self.hub,
            &mut self.long_ops,
            &dto,
        )
        .expect("start analyse");

        // Deadlined, not a bare spin: a long operation that fails without ever
        // reporting a result would otherwise hang the whole test binary with no clue
        // which test did it — which is exactly what happened the first time an
        // analyse started refusing a destination.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        let plan = loop {
            if let Some(result) = import_management_controller::get_analyze_document_import_result(
                &self.long_ops,
                &op,
            )
            .expect("analyse result")
            {
                break result;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "analyse never produced a result"
            );
            std::thread::sleep(std::time::Duration::from_millis(5));
        };

        match plan.rows {
            DocumentImportRows::Found(rows) => rows,
            DocumentImportRows::Empty => Vec::new(),
        }
    }

    fn apply(&mut self, rows: Vec<DocumentImportRow>, anchor: EntityId) -> Vec<EntityId> {
        self.apply_at(rows, anchor, DropPosition::Into)
    }

    fn apply_at(
        &mut self,
        rows: Vec<DocumentImportRow>,
        anchor: EntityId,
        drop_position: DropPosition,
    ) -> Vec<EntityId> {
        let create_rows: Vec<ApplyImportRow> = rows
            .into_iter()
            .filter_map(|r| match r {
                DocumentImportRow::Found {
                    indent,
                    kind,
                    title,
                    djot,
                    epigraph,
                    comments,
                    included,
                    source_uid_tag,
                    origin,
                    source_file_digest,
                    ..
                } if included => Some(ApplyImportRow::Create {
                    indent,
                    kind,
                    title,
                    djot,
                    // Handed straight back, exactly as the UI does: this helper is
                    // "accept the whole plan", and a plan carrying comments that
                    // silently did not get created would make every comment test
                    // pass for the wrong reason. The row's own identity travels the
                    // same way, and for the same reason — and so does its epigraph,
                    // which is a second `Content` and not part of `djot`.
                    epigraph,
                    comments,
                    source_uid_tag,
                    // …and its provenance, narrowed from the path to the name here
                    // exactly as `PlanRowView::source_file_name` does it. A helper
                    // that sent the whole path would pass while the real UI sent
                    // something else.
                    source_file_name: std::path::Path::new(&origin)
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default(),
                    source_file_digest,
                }),
                _ => None,
            })
            .collect();

        import_management_controller::apply_document_import(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &ApplyDocumentImportDto {
                work_id: self.work_id,
                binder_id: self.binder_id,
                anchor_item_id: anchor,
                drop_position,
                row: ApplyImportRow::Empty,
                rows: ApplyImportRows::Create(create_rows),
            },
        )
        .expect("apply")
        .created_ids
    }

    fn binder_order(&self) -> Vec<EntityId> {
        binder_controller::get_relationship(
            &self.db,
            &self.binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .expect("binder order")
    }

    /// The prose stored on an item, if it has any.
    fn prose_of(&self, item_id: EntityId) -> Option<String> {
        let content_ids = binder_item_controller::get_relationship(
            &self.db,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .expect("contents");
        content_ids.first().map(|id| {
            content_controller::get(&self.db, id)
                .expect("content")
                .expect("content row")
                .data
        })
    }

    fn title_of(&self, item_id: EntityId) -> String {
        binder_item_controller::get(&self.db, &item_id)
            .expect("item")
            .expect("item row")
            .title
    }

    /// The `Content` row id an item's prose lives on, if it has any — what a
    /// recognised `Comment::content` should be repointed at after a re-import
    /// lands on a *different* row than the one it was first attached to.
    fn content_id_of(&self, item_id: EntityId) -> Option<EntityId> {
        binder_item_controller::get_relationship(
            &self.db,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .expect("contents")
        .first()
        .copied()
    }

    /// Copy a committed container fixture into the scratch directory.
    ///
    /// `document_ingest`'s own tests read these files too, and read them harder —
    /// this is the half those cannot reach: whether the comments they recovered
    /// actually become rows in a store.
    fn copy_fixture(&self, name: &str) -> String {
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../document_ingest/tests/fixtures")
            .join(name);
        let target = self._dir.path().join(name);
        std::fs::copy(&source, &target)
            .unwrap_or_else(|e| panic!("{name} is missing — run its generate.py ({e})"));
        target.to_string_lossy().to_string()
    }

    /// Every comment on this Work, with its replies, in creation order.
    fn comments(
        &self,
    ) -> Vec<(
        direct_access::comment::dtos::CommentDto,
        Vec<direct_access::comment_reply::dtos::CommentReplyDto>,
    )> {
        work_controller::get_relationship(
            &self.db,
            &self.work_id,
            &common::direct_access::work::WorkRelationshipField::Comments,
        )
        .expect("work comments")
        .into_iter()
        .map(|id| {
            let comment = direct_access::comment::comment_controller::get(&self.db, &id)
                .expect("comment")
                .expect("comment row");
            let replies = direct_access::comment_reply::comment_reply_controller::get_multi(
                &self.db,
                &comment.replies,
            )
            .expect("replies")
            .into_iter()
            .flatten()
            .collect();
            (comment, replies)
        })
        .collect()
    }

    /// The plain text of the prose an item stores, in the coordinate space a
    /// comment anchor is measured in.
    fn plain_of(&self, item_id: EntityId) -> String {
        let djot = self.prose_of(item_id).unwrap_or_default();
        skrib_format::djot_plain_text(&djot)
            .expect("the stored Djot parses")
            .0
    }
}

const NOVEL: &str = "\
# The Long Novel

## Chapter 1: The Storm

She turned the corner.

* * *

The fog had not lifted.

## Chapter 2: Morning

Morning came late.
";

#[test]
fn a_manuscript_travels_through_both_halves_into_the_binder() {
    let mut ctx = Ctx::new();
    let path = ctx.write("novel.md", NOVEL);

    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    assert_eq!(rows.len(), 3, "one Book and two Chapters");

    let created = ctx.apply(rows, 0);
    assert_eq!(created.len(), 3);
    assert_eq!(ctx.binder_order(), created, "the block lands in order");

    assert_eq!(ctx.title_of(created[0]), "The Long Novel");
    // The ordinal came off the chapter titles; the manuscript's own order
    // supplies the number, so carrying "Chapter 1" into the title would put a
    // second, competing numbering beside it.
    assert_eq!(ctx.title_of(created[1]), "The Storm");
    assert_eq!(ctx.title_of(created[2]), "Morning");

    // A Book holds no prose; its chapters do, breaks preserved inline.
    assert_eq!(ctx.prose_of(created[0]), None);
    let chapter = ctx.prose_of(created[1]).expect("chapter prose");
    assert!(chapter.contains("turned the corner"));
    assert!(
        chapter.contains(skribisto_model::scene_break::canonical_djot(
            skribisto_model::scene_break::SceneBreakTier::Minor
        )),
        "the scene break must survive as an escaped marker: {chapter:?}"
    );
}

/// The whole import is one undo entry, and undoing it leaves nothing behind.
#[test]
fn the_import_is_a_single_undo_entry() {
    let mut ctx = Ctx::new();
    let path = ctx.write("novel.md", NOVEL);
    let before = ctx.binder_order();

    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    ctx.apply(rows, 0);
    assert_eq!(ctx.binder_order().len(), before.len() + 3);

    ctx.undo.undo(None).expect("undo");
    assert_eq!(
        ctx.binder_order(),
        before,
        "one undo must remove the whole import"
    );

    ctx.undo.redo(None).expect("redo");
    assert_eq!(
        ctx.binder_order().len(),
        before.len() + 3,
        "redo restores it"
    );
}

/// Created rows must be exportable and activated. Both default to `false`, and a
/// row that is neither is present in the binder yet invisible to export and to
/// chapter numbering — with no gap left behind, because chapters renumber to
/// fill it.
#[test]
fn created_rows_are_exportable_and_activated() {
    let mut ctx = Ctx::new();
    let path = ctx.write("novel.md", NOVEL);
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    let created = ctx.apply(rows, 0);

    for id in created {
        let item = binder_item_controller::get(&ctx.db, &id)
            .expect("item")
            .expect("item row");
        assert!(
            item.is_exportable,
            "row {id} would vanish from every export"
        );
        assert!(item.activated, "row {id} would vanish from the binder");
        assert!(!item.uid.is_nil(), "row {id} has no durable identity");
    }
}

/// **A note starts out of the export, whichever door made it.**
///
/// The review step lets the writer retype any row to Note, and a note is their own
/// workings. Every other door that makes one (the + Create button, the corkboard's and the
/// stream's "+", the Notebook template, the Plume importer, the story-bible door) creates it
/// with `is_exportable: false`. If this one did not, the same note would print or not print
/// under a preset with `include_notes` on depending only on which button made it.
#[test]
fn a_row_retyped_to_note_is_created_out_of_the_export() {
    let mut ctx = Ctx::new();
    let path = ctx.write("clipping.md", "A research clipping about ferries.");
    let rows = ctx.analyse(vec![path], ImportRowKind::Scene);
    assert!(!rows.is_empty(), "the file produced a row to retype");

    // Exactly what the review step's row-type combo does to the plan.
    let retyped: Vec<DocumentImportRow> = rows
        .into_iter()
        .map(|r| match r {
            DocumentImportRow::Found { title, djot, .. } => DocumentImportRow::Found {
                indent: 0,
                kind: ImportRowKind::Note,
                title,
                stripped_ordinal: String::new(),
                djot,
                epigraph: String::new(),
                scene_breaks: 0,
                word_count: 0,
                comments: Vec::new(),
                origin: String::new(),
                included: true,
                source_uid_tag: String::new(),
                source_digest: String::new(),
                source_file_digest: String::new(),
            },
            DocumentImportRow::Empty => DocumentImportRow::Empty,
        })
        .collect();

    let created = ctx.apply(retyped, 0);
    assert_eq!(created.len(), 1);
    let item = binder_item_controller::get(&ctx.db, &created[0])
        .expect("item")
        .expect("item row");
    assert_eq!(item.sub_role, common::entities::BinderItemSubRole::Note);
    assert!(
        !item.is_exportable,
        "an imported note must start out of the export, like every other note"
    );
    assert!(item.activated, "it is still in the binder");
}

/// Many files, handed over out of order, land in the order their names imply.
#[test]
fn many_files_land_in_their_own_order_not_the_callers() {
    let mut ctx = Ctx::new();
    let mut paths: Vec<String> = (1..=12)
        .map(|i| {
            ctx.write(
                &format!("{i:02}_scene-{i}.md"),
                &format!("Scene {i} prose."),
            )
        })
        .collect();
    paths.reverse();

    let rows = ctx.analyse(paths, ImportRowKind::Scene);
    let created = ctx.apply(rows, 0);

    assert_eq!(created.len(), 12);
    let titles: Vec<String> = created.iter().map(|id| ctx.title_of(*id)).collect();
    assert_eq!(titles[0], "scene 1");
    assert_eq!(titles[11], "scene 12");
}

/// An anchor puts the block where the writer said, not at the end.
#[test]
fn an_anchor_places_the_block_where_it_was_asked_to_go() {
    let mut ctx = Ctx::new();
    let first = ctx.write("a.md", "# One\n\nProse.");
    let created_first = {
        let rows = ctx.analyse(vec![first], ImportRowKind::Scene);
        ctx.apply(rows, 0)
    };
    let second = ctx.write("b.md", "# Two\n\nProse.");
    let rows = ctx.analyse(vec![second], ImportRowKind::Scene);
    let created_second = ctx.apply(rows, created_first[0]);

    let order = ctx.binder_order();
    let at_anchor = order.iter().position(|id| *id == created_first[0]).unwrap();
    assert_eq!(order[at_anchor + 1], created_second[0]);
}

/// One unreadable file among many must not take the batch down.
#[test]
fn a_missing_file_is_reported_and_the_rest_still_import() {
    let mut ctx = Ctx::new();
    let good = ctx.write("01_good.md", "# Good\n\nProse.");
    let missing = ctx
        ._dir
        .path()
        .join("02_gone.md")
        .to_string_lossy()
        .to_string();

    let dto = AnalyzeDocumentImportDto {
        work_id: ctx.work_id,
        binder_id: ctx.binder_id,
        source_paths: vec![good, missing],
        anchor_item_id: 0,
        drop_position: DropPosition::Into,
    };
    let op = import_management_controller::analyze_document_import(
        &ctx.db,
        &ctx.hub,
        &mut ctx.long_ops,
        &dto,
    )
    .expect("start");
    let plan = loop {
        if let Some(r) =
            import_management_controller::get_analyze_document_import_result(&ctx.long_ops, &op)
                .expect("result")
        {
            break r;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    };

    let DocumentImportRows::Found(rows) = plan.rows else {
        panic!("expected rows");
    };
    assert_eq!(rows.len(), 1, "the good file still imports");

    let ImportDiagnosticRows::Reported(diags) = plan.diagnostics else {
        panic!("expected diagnostics");
    };
    assert!(
        diags.iter().any(|d| matches!(
            d,
            import_management::ImportDiagnosticRow::Reported { key, .. } if key == "file-unreadable"
        )),
        "the missing file must be named, not silently skipped"
    );
}

/// Prose whose row cannot hold it is refused here rather than dropped silently
/// at the next save, which is what `content_allowed` would do on its own.
#[test]
fn prose_on_a_type_that_cannot_hold_it_is_refused_not_swallowed() {
    let mut ctx = Ctx::new();
    let result = import_management_controller::apply_document_import(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &ApplyDocumentImportDto {
            work_id: ctx.work_id,
            binder_id: ctx.binder_id,
            anchor_item_id: 0,
            drop_position: DropPosition::Into,
            row: ApplyImportRow::Empty,
            rows: ApplyImportRows::Create(vec![ApplyImportRow::Create {
                indent: 0,
                kind: ImportRowKind::Book,
                title: "A Book".into(),
                djot: "Prose a Book cannot hold.".into(),
                epigraph: String::new(),
                comments: Vec::new(),
                source_uid_tag: String::new(),
                source_file_name: String::new(),
                source_file_digest: String::new(),
            }]),
        },
    );
    assert!(result.is_err(), "a Book holding prose must be refused");
    assert!(
        ctx.binder_order().is_empty(),
        "a refused import must leave nothing behind"
    );
}

// ── The two measurements the plan called for ────────────────────────────────
//
// Both were in M2's exit criteria and neither got written, so two decisions the
// design rests on were argued rather than measured. They are cheap, they run in
// the ordinary suite, and their budgets are deliberately loose: this is a
// regression tripwire for an order-of-magnitude change, not a benchmark. A CI
// box under load must not turn a correct build red.

/// **The gate on the snapshot decision.**
///
/// Undo for an import is `snapshot_binder` / `restore_binder` — O(the whole
/// binder), not O(what changed). That is fine for a hundred items and was never
/// checked for a real manuscript. The plan named `begin_composite` as the escape
/// hatch if this measured badly; this is the measurement that would tell us.
///
/// A 4,000-item binder is a long novel with every scene split out, i.e. the
/// upper end of what anyone actually has.
#[test]
fn undoing_an_import_into_a_large_binder_stays_interactive() {
    use std::time::Instant;

    let mut ctx = Ctx::new();

    // A binder of 4,000 items, created the cheap way — this is the *setting*
    // for the measurement, not part of it.
    let existing: Vec<ApplyImportRow> = (0..4_000)
        .map(|i| ApplyImportRow::Create {
            indent: 0,
            kind: ImportRowKind::Scene,
            title: format!("Scene {i}"),
            djot: String::new(),
            epigraph: String::new(),
            comments: Vec::new(),
            source_uid_tag: String::new(),
            source_file_name: String::new(),
            source_file_digest: String::new(),
        })
        .collect();
    import_management_controller::apply_document_import(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &ApplyDocumentImportDto {
            work_id: ctx.work_id,
            binder_id: ctx.binder_id,
            anchor_item_id: 0,
            drop_position: DropPosition::Into,
            row: ApplyImportRow::Empty,
            rows: ApplyImportRows::Create(existing),
        },
    )
    .expect("seed");
    assert_eq!(ctx.binder_order().len(), 4_000);

    // Now the import being measured: a small one, into that large binder. The
    // snapshot is of the *binder*, so its cost tracks the binder's size and not
    // the import's — which is exactly the property under test.
    let path = ctx.write("novel.md", NOVEL);
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    ctx.apply(rows, 0);
    assert_eq!(ctx.binder_order().len(), 4_003);

    let started = Instant::now();
    ctx.undo.undo(None).expect("undo");
    let elapsed = started.elapsed();
    eprintln!("restore_binder over 4,000 items: {elapsed:?}");

    assert_eq!(ctx.binder_order().len(), 4_000, "the import is gone");
    // 2 s in an unoptimized debug build. Release is far faster; what this rules
    // out is the shape where restore is quadratic in binder size and a large
    // project's undo takes a minute.
    assert!(
        elapsed < std::time::Duration::from_secs(2),
        "restore_binder over a 4,000-item binder took {elapsed:?} — \
         if this is a real regression, `begin_composite` is the escape hatch \
         the plan named (undo cost O(what changed) instead of O(the binder))"
    );
}

/// **The event-coalescing check — and what it found.**
///
/// The question the plan asked was whether `create_multi` /
/// `set_binder_item_relationship_multi` coalesce their events. **They do not:**
/// a 200-row import publishes exactly 200 `BinderItem` events, a 20-row one
/// exactly 20. Measured, not argued — which was the point of writing this.
///
/// What that costs, concretely: `EventBuffer` defers every event to commit, so
/// they arrive as one burst rather than interleaved with the writing — good.
/// But `models::binder_binder_items_tree_model` calls a full `reload()` on each
/// one, with no throttle, and a reload re-queries the whole binder. So importing
/// 200 rows into a 4,000-item project re-reads and rebuilds that binder 200
/// times, in one burst, while the writer watches.
///
/// This is not fixable here — the per-item events come out of the generated
/// `direct_access` controllers, and an event that names one entity is what makes
/// it useful to everything else that listens. **It is fixed on the reading side**
/// instead: `teksilo_ui::models::coalesced_reload` collapses a burst into one
/// reload per frame, for the binder tree, the trash tree and the Overview alike,
/// so trash, restore, duplicate and move all stopped paying it too.
///
/// The assertion is the tripwire that keeps the *publishing* side from getting
/// worse: at most one event per row. A future apply that looped single creates
/// plus a relationship call each would double or triple it, and would otherwise
/// look fine.
#[test]
fn a_batched_import_fires_at_most_one_event_per_row() {
    use common::event::{DirectAccessEntity, Origin};
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn events_for(rows: usize) -> usize {
        let mut ctx = Ctx::new();
        let seen = Arc::new(AtomicUsize::new(0));

        // The hub is MPMC — one receiver, drained on its own thread, rather than
        // several competing for the same events.
        let rx = ctx.hub.subscribe_receiver();
        let counter = seen.clone();
        let drain = std::thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                if matches!(
                    event.origin,
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(_))
                ) {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }
        });

        let create: Vec<ApplyImportRow> = (0..rows)
            .map(|i| ApplyImportRow::Create {
                indent: 0,
                kind: ImportRowKind::Scene,
                title: format!("Scene {i}"),
                djot: String::new(),
                epigraph: String::new(),
                comments: Vec::new(),
                source_uid_tag: String::new(),
                source_file_name: String::new(),
                source_file_digest: String::new(),
            })
            .collect();
        import_management_controller::apply_document_import(
            &ctx.db,
            &ctx.hub,
            &mut ctx.undo,
            None,
            &ApplyDocumentImportDto {
                work_id: ctx.work_id,
                binder_id: ctx.binder_id,
                anchor_item_id: 0,
                drop_position: DropPosition::Into,
                row: ApplyImportRow::Empty,
                rows: ApplyImportRows::Create(create),
            },
        )
        .expect("apply");

        // Events cross a channel; give them a moment to land, then close the
        // hub's sender side by dropping the context so the drain thread ends.
        std::thread::sleep(std::time::Duration::from_millis(300));
        let counted = seen.load(Ordering::Relaxed);
        drop(ctx);
        let _ = drain.join();
        counted
    }

    let small = events_for(20);
    let large = events_for(200);
    eprintln!("BinderItem events: 20 rows -> {small}, 200 rows -> {large}");

    assert!(
        small <= 20,
        "a 20-row import fired {small} BinderItem events — more than one per row"
    );
    assert!(
        large <= 200,
        "a 200-row import fired {large} BinderItem events — more than one per row. \
         Each one costs the binder tree a full reload, so this multiplies a cost \
         that is already the biggest in a bulk import"
    );
}

// ── imported comments ───────────────────────────────────────────────────────

/// The full journey for an editor's note: out of a `.docx`, through the plan, into
/// a `Comment` row anchored to the words it was about.
///
/// `document_ingest`'s own tests prove the scanner reads the file; this proves the
/// half they cannot see — that what it read becomes rows in a store, pointing where
/// it said, with its thread intact.
#[test]
fn an_editors_comments_survive_the_whole_journey_into_the_store() {
    let mut ctx = Ctx::new();
    let path = ctx.copy_fixture("word-shaped.docx");
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    let created = ctx.apply(rows, 0);

    let comments = ctx.comments();
    // Three threads: the ranged comment (with its reply), the whole-paragraph
    // note, and the unanchored, richly-formatted comment M-S4's fixture carries
    // (`word-shaped.docx`'s comment 4 — see `tests/fixtures/generate.py`) to
    // prove a comment's own bold/italic survives the *whole* journey into the
    // store, not merely the scanner `document_ingest`'s own tests already cover.
    assert_eq!(comments.len(), 3, "three threads: {comments:#?}");

    let (ranged, replies) = comments
        .iter()
        .find(|(c, _)| c.body == "Is this the right word?")
        .expect("the ranged comment");

    assert_eq!(ranged.author_name, "Editor");
    assert_eq!(ranged.kind, common::entities::CommentAnchorKind::Range);
    assert!(
        !ranged.orphaned,
        "the quote was proved before it was stored"
    );
    assert_eq!(ranged.quote_exact, "the street was gone");
    assert_eq!(
        ranged.created_at.to_rfc3339(),
        "2026-01-02T03:04:05+00:00",
        "the author's own date, not the moment of import"
    );

    assert_eq!(replies.len(), 1, "the thread is one level deep");
    assert_eq!(replies[0].author_name, "Writer");
    assert_eq!(replies[0].body, "Yes, I meant it.");

    // The anchor has to point at that text in the prose *as stored*, which is the
    // only claim that matters: everything up to here could be right while the
    // offsets pointed at the wrong sentence.
    let owner = created
        .iter()
        .find(|id| ctx.plain_of(**id).contains("the street was gone"))
        .expect("the row holding the quoted prose");
    let plain: Vec<char> = ctx.plain_of(*owner).chars().collect();
    let start = ranged.range_start as usize;
    let end = start + ranged.range_length as usize;
    assert!(end <= plain.len(), "the anchor runs past the prose");
    assert_eq!(
        plain[start..end].iter().collect::<String>(),
        "the street was gone",
        "the stored offsets point somewhere else"
    );

    // The rich comment: its Djot markers must reach the `Comment` row itself,
    // not just `document_ingest`'s own `SourceAnnotation` — proving
    // `apply_document_import_uc` carries the body through unflattened.
    let (rich, _) = comments
        .iter()
        .find(|(c, _)| c.body.contains("real") && c.body.contains("italics"))
        .expect("the richly formatted comment");
    assert!(
        rich.body.contains("*real*"),
        "bold did not survive into the stored Comment: {:?}",
        rich.body
    );
    assert!(
        rich.body.contains("_italics_"),
        "italic did not survive into the stored Comment: {:?}",
        rich.body
    );
}

/// Resolved in Word, resolved in Skribisto — and the whole-paragraph comment keeps
/// the paragraph it was on.
#[test]
fn a_resolved_paragraph_comment_arrives_resolved_and_covers_its_paragraph() {
    let mut ctx = Ctx::new();
    let path = ctx.copy_fixture("word-shaped.docx");
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    let created = ctx.apply(rows, 0);

    let comments = ctx.comments();
    let (paragraph, _) = comments
        .iter()
        .find(|(c, _)| c.body == "A whole-paragraph note.")
        .expect("the paragraph comment");

    assert!(paragraph.resolved, "w15:done was not carried across");
    assert_eq!(
        paragraph.kind,
        common::entities::CommentAnchorKind::Paragraph
    );

    let owner = created
        .iter()
        .find(|id| {
            ctx.plain_of(**id)
                .contains("The second chapter opens quietly.")
        })
        .expect("the row holding the second chapter");
    let plain: Vec<char> = ctx.plain_of(*owner).chars().collect();
    let start = paragraph.range_start as usize;
    let end = start + paragraph.range_length as usize;
    assert_eq!(
        plain[start..end].iter().collect::<String>(),
        "The second chapter opens quietly."
    );
}

/// Undo must take the comments back with the rows.
///
/// The undo snapshot is `Binder`-scoped and a `Comment` hangs off the `Work`, so
/// restoring the binder alone would leave every imported note behind — attached to
/// `Content` rows that no longer exist, and visible in the comments dock as notes
/// about a manuscript the writer just undid.
#[test]
fn undoing_an_import_takes_its_comments_back_too() {
    let mut ctx = Ctx::new();
    let path = ctx.copy_fixture("word-shaped.docx");
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    ctx.apply(rows, 0);
    // Three threads — see `an_editors_comments_survive_the_whole_journey_into_the_store`
    // for why the fixture carries a third, unanchored one.
    assert_eq!(ctx.comments().len(), 3);

    ctx.undo.undo(None).expect("undo");
    assert!(
        ctx.comments().is_empty(),
        "the comments outlived the rows they annotated: {:#?}",
        ctx.comments()
    );

    ctx.undo.redo(None).expect("redo");
    assert_eq!(ctx.comments().len(), 3, "redo must put them back");
}

/// An `.odt` from LibreOffice takes the same journey — different spelling, same
/// destination. Worth its own test rather than a loop: if only one format were
/// broken, a shared assertion would name neither.
#[test]
fn an_odt_from_libreoffice_lands_its_comments_too() {
    let mut ctx = Ctx::new();
    let path = ctx.copy_fixture("libreoffice.odt");
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let comments = ctx.comments();
    // Three threads — see the DOCX test's own note on the fixture's third,
    // richly-formatted comment (M-S4).
    assert_eq!(comments.len(), 3, "{comments:#?}");
    let (ranged, replies) = comments
        .iter()
        .find(|(c, _)| c.body == "Is this the right word?")
        .expect("the ranged comment");
    assert_eq!(ranged.quote_exact, "the street was gone");
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0].body, "Yes, I meant it.");

    let (rich, _) = comments
        .iter()
        .find(|(c, _)| c.body.contains("real") && c.body.contains("italics"))
        .expect("the richly formatted comment");
    assert!(
        rich.body.contains("*real*"),
        "bold did not survive into the stored Comment: {:?}",
        rich.body
    );
    assert!(
        rich.body.contains("_italics_"),
        "italic did not survive into the stored Comment: {:?}",
        rich.body
    );
}

/// Markdown has no comments, and importing one must not invent any.
#[test]
fn a_markdown_import_creates_no_comments() {
    let mut ctx = Ctx::new();
    let path = ctx.write("novel.md", NOVEL);
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    ctx.apply(rows, 0);
    assert!(ctx.comments().is_empty());
}

// ── the destination's own depth ─────────────────────────────────────────────

/// Every pre-existing row's parent, derived the way `binder_ordering` derives it:
/// the nearest preceding row with a strictly smaller indent.
///
/// Deliberately re-derived here rather than imported. This asserts what the *binder*
/// means, and a helper shared with the code under test would agree with that code
/// even when both were wrong.
fn parents(ctx: &Ctx) -> std::collections::HashMap<EntityId, Option<EntityId>> {
    let order = ctx.binder_order();
    let indent: Vec<i64> = order
        .iter()
        .map(|id| {
            binder_item_controller::get(&ctx.db, id)
                .expect("item")
                .expect("row")
                .indent
        })
        .collect();

    let mut out = std::collections::HashMap::new();
    for (i, id) in order.iter().enumerate() {
        let mut parent = None;
        for j in (0..i).rev() {
            if indent[j] < indent[i] {
                parent = Some(order[j]);
                break;
            }
        }
        out.insert(*id, parent);
    }
    out
}

/// **Regression.** An import must add rows; it must never re-home one that was
/// already there.
///
/// `apply_document_import` splices its block immediately after the anchor *row* —
/// not after that row's subtree — and every imported row arrives at the plan's
/// `base_indent`, which the wizard hardcodes to 0 whatever destination the writer
/// chose. Land an indent-0 block between a chapter and its scenes and the scenes'
/// nearest preceding smaller indent becomes the *imported* row: they silently
/// change parent. "Import here…" made that reachable from any row in the binder.
///
/// Stated as an invariant rather than as an expected shape on purpose: it holds
/// whatever indent the fix decides an import should start at, so it cannot be
/// satisfied by teaching it the answer.
#[test]
fn importing_into_a_chapter_never_re_parents_what_was_already_there() {
    let mut ctx = Ctx::new();

    // A book with a chapter that has a scene of its own.
    let existing = ctx.write(
        "existing.md",
        "# The Book\n\n## Chapter One\n\n### A scene\n\nProse.\n",
    );
    let created = {
        let rows = ctx.analyse(vec![existing], ImportRowKind::Book);
        ctx.apply(rows, 0)
    };
    assert_eq!(created.len(), 3, "book, chapter, scene");
    let chapter = created[1];
    let scene = created[2];

    let before = parents(&ctx);
    assert_eq!(
        before[&scene],
        Some(chapter),
        "the fixture itself must be nested, or this test proves nothing"
    );

    // Import again, pointing at the chapter — what "Import here…" does.
    let incoming = ctx.write("incoming.md", "# Another Book\n\nProse.\n");
    let rows = ctx.analyse(vec![incoming], ImportRowKind::Book);
    ctx.apply(rows, chapter);

    let after = parents(&ctx);
    for (id, parent) in &before {
        assert_eq!(
            after.get(id),
            Some(parent),
            "row {id} changed parent: was {parent:?}, now {:?} — an import re-homed \
             a row that was already in the binder",
            after.get(id)
        );
    }
}

/// The other half of the same fix: an import *into* a container lands inside it, at
/// the right depth, after whatever was already there.
///
/// The invariant test above proves nothing is broken; this proves something is right.
/// Both are needed — "changes no parents" is also satisfied by refusing to import.
#[test]
fn importing_into_a_chapter_lands_inside_it_after_its_existing_scenes() {
    let mut ctx = Ctx::new();
    let existing = ctx.write(
        "existing.md",
        "# The Book\n\n## Chapter One\n\n### First scene\n\nProse.\n",
    );
    let created = {
        let rows = ctx.analyse(vec![existing], ImportRowKind::Book);
        ctx.apply(rows, 0)
    };
    let (chapter, first_scene) = (created[1], created[2]);

    let incoming = ctx.write("incoming.md", "# Later scene\n\nMore prose.\n");
    let rows = ctx.analyse_at(vec![incoming], chapter, DropPosition::Into);
    let imported = ctx.apply_at(rows, chapter, DropPosition::Into);
    assert_eq!(imported.len(), 1);

    // Inside the chapter, and a sibling of the scene already there.
    let after = parents(&ctx);
    assert_eq!(
        after[&imported[0]],
        Some(chapter),
        "the import must land inside the chapter the writer pointed at"
    );
    assert_eq!(after[&first_scene], Some(chapter), "beside what was there");

    // …and after it, not before: a container's existing children keep their order and
    // the new material appends.
    let order = ctx.binder_order();
    let at = |id| order.iter().position(|x| *x == id).unwrap();
    assert!(
        at(first_scene) < at(imported[0]),
        "the import appends after the chapter's existing scenes"
    );
}

/// Landing *beside* a leaf makes the import its sibling, at the leaf's own depth —
/// not a child of it, and not back at the top level.
#[test]
fn importing_beside_a_scene_makes_it_a_sibling() {
    let mut ctx = Ctx::new();
    let existing = ctx.write(
        "existing.md",
        "# The Book\n\n## Chapter One\n\n### First scene\n\nProse.\n",
    );
    let created = {
        let rows = ctx.analyse(vec![existing], ImportRowKind::Book);
        ctx.apply(rows, 0)
    };
    let (chapter, first_scene) = (created[1], created[2]);

    let incoming = ctx.write("incoming.md", "# Later scene\n\nMore prose.\n");
    let rows = ctx.analyse_at(vec![incoming], first_scene, DropPosition::After);
    let imported = ctx.apply_at(rows, first_scene, DropPosition::After);

    let after = parents(&ctx);
    assert_eq!(
        after[&imported[0]],
        Some(chapter),
        "a sibling of the scene shares the scene's parent"
    );
}

// ── M-S7: recognition on re-import ──────────────────────────────────────────
//
// The decisive proof the milestone exists for: importing a `.docx`/`.odt`
// Skribisto itself exported must RECOGNISE a comment it already has (matched by
// `Comment.uid`, the writer's own `skrb:uid` attribute) and update that row in
// place, never create a second copy of it. Every test below builds its
// "returning file" fresh via `text-document`'s own writer — the same dev-only
// wiring `document_ingest`'s own `docx_writer_roundtrip.rs`/
// `odt_writer_roundtrip.rs` use — rather than reading a checked-in fixture,
// because the whole point is to carry a *real* local `Comment`/`CommentReply`
// uid this Work already has, which no static fixture could ever do (a uid is
// minted at runtime, on the first import).

/// No heading, so the whole thing becomes one leading row named after the
/// document — the same shape `text-document`'s own `docx_comment_export_tests`/
/// `odt_writer_roundtrip.rs` comment fixtures use.
const MANUSCRIPT: &str = "\
This manuscript opens with a sentence that needs review.

A second, unrelated paragraph follows.
";

/// `[start, end)` of `needle`'s first occurrence in `doc`, in the addressable
/// character space `DocumentComment::start`/`end` are defined in.
fn find_range(doc: &TextDocument, needle: &str) -> (u32, u32) {
    let m = doc
        .find(needle, 0, &FindOptions::default())
        .expect("find")
        .unwrap_or_else(|| panic!("{needle:?} not found in the document"));
    (m.position as u32, (m.position + m.length) as u32)
}

/// Build `djot` into a real document, hand it to `make_comments` (which can
/// call [`find_range`] against it before deciding what to anchor), and export
/// the result to a real `.docx`. Returns the file's bytes.
fn build_docx(
    djot: &str,
    make_comments: impl FnOnce(&TextDocument) -> DocumentComments,
) -> Vec<u8> {
    let doc = TextDocument::new();
    doc.set_djot_sync(djot).expect("set_djot_sync");
    let comments = make_comments(&doc);

    // A private directory per call, never a shared name in `temp_dir()`.
    //
    // This used to build the file name from `{pid}_{nanos}`, which collides in two ways
    // once `cargo test` runs these in parallel — they share a process, so the pid is a
    // constant, and `duration_since(UNIX_EPOCH).map(..).unwrap_or_default()` yields `0`
    // for *every* caller the moment that call errors. Two tests then write and read the
    // same path, and the reader gets a zip the writer is still filling in: the failure
    // surfaces as `docx-rs` panicking with "Invalid checksum" deep inside `read_zip`,
    // several frames from anything that names this function. `TempDir` cannot collide,
    // and it removes itself on drop, so the explicit `remove_file` goes too.
    let dir = tempfile::tempdir().expect("temp dir for the exported docx");
    let path = dir.path().join("roundtrip.docx");
    doc.to_docx_with_options(
        &path.to_string_lossy(),
        DocxExportOptions {
            comments,
            ..Default::default()
        },
    )
    .expect("to_docx_with_options")
    .wait()
    .expect("docx export completes");
    std::fs::read(&path).expect("read exported docx")
}

/// As [`build_docx`], for `.odt`.
fn build_odt(djot: &str, make_comments: impl FnOnce(&TextDocument) -> DocumentComments) -> Vec<u8> {
    let doc = TextDocument::new();
    doc.set_djot_sync(djot).expect("set_djot_sync");
    let comments = make_comments(&doc);

    let path = std::env::temp_dir().join(format!(
        "import_mgmt_odt_roundtrip_{}_{}.odt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));
    doc.to_odt_with_options(
        &path.to_string_lossy(),
        OdtExportOptions {
            comments,
            ..Default::default()
        },
    )
    .expect("to_odt_with_options")
    .wait()
    .expect("odt export completes");
    let bytes = std::fs::read(&path).expect("read exported odt");
    let _ = std::fs::remove_file(&path);
    bytes
}

/// One root comment on "needs review" (no uid — an editor's own first pass) with
/// one reply, exported to `.docx`.
fn first_returning_docx() -> Vec<u8> {
    build_docx(MANUSCRIPT, |doc| {
        let range = find_range(doc, "needs review");
        let mut root = DocumentComment {
            start: range.0,
            end: range.1,
            uid: String::new(),
            author: "Editor".to_string(),
            author_initials: "ED".to_string(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: false,
            body: "Please look at this.".to_string(),
            replies: Vec::new(),
        };
        root.replies.push(TdCommentReply {
            uid: String::new(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "Will do.".to_string(),
        });
        let mut comments = DocumentComments::new();
        comments.insert(root);
        comments
    })
}

/// As [`first_returning_docx`], for `.odt`.
fn first_returning_odt() -> Vec<u8> {
    build_odt(MANUSCRIPT, |doc| {
        let range = find_range(doc, "needs review");
        let mut root = DocumentComment {
            start: range.0,
            end: range.1,
            uid: String::new(),
            author: "Editor".to_string(),
            author_initials: String::new(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: false,
            body: "Please look at this.".to_string(),
            replies: Vec::new(),
        };
        root.replies.push(TdCommentReply {
            uid: String::new(),
            author: "Writer".to_string(),
            author_initials: String::new(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "Will do.".to_string(),
        });
        let mut comments = DocumentComments::new();
        comments.insert(root);
        comments
    })
}

/// A `.docx` carrying the SAME thread again, keyed by `comment_uid`/`reply_uid` —
/// simulating Skribisto's own re-export of an already-recognised comment coming
/// back from a further round of editing, with a changed body on both turns.
fn returning_docx_for(comment_uid: uuid::Uuid, reply_uid: uuid::Uuid, resolved: bool) -> Vec<u8> {
    build_docx(MANUSCRIPT, |doc| {
        let range = find_range(doc, "needs review");
        let mut root = DocumentComment {
            start: range.0,
            end: range.1,
            uid: comment_uid.to_string(),
            author: "Editor".to_string(),
            author_initials: "ED".to_string(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved,
            body: "Please look at this *closely*.".to_string(),
            replies: Vec::new(),
        };
        root.replies.push(TdCommentReply {
            uid: reply_uid.to_string(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "Will do, on it now.".to_string(),
        });
        let mut comments = DocumentComments::new();
        comments.insert(root);
        comments
    })
}

/// A second reader's copy, cut before the writer settled the thread, must not
/// re-open it.
///
/// `resolved` is the one file-authoritative field that is monotone. Every other
/// one describes what the passage *says*, which the returning file is the best
/// authority on; this one describes where the writer has got to, which a copy
/// taken in the past cannot know. With several readers holding copies at once,
/// taking it verbatim re-opens settled threads on every return after the first.
#[test]
fn a_stale_returning_file_cannot_reopen_a_thread_the_writer_resolved() {
    let mut ctx = Ctx::new();

    let first_path = ctx.write_bytes("returned-1.docx", &first_returning_docx());
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let before = ctx.comments();
    let (comment, replies) = &before[0];
    let (uid, reply_uid) = (comment.uid, replies[0].uid);

    // A reader resolves it.
    let resolving = ctx.write_bytes("returned-2.docx", &returning_docx_for(uid, reply_uid, true));
    let rows = ctx.analyse(vec![resolving], ImportRowKind::Book);
    ctx.apply(rows, 0);
    assert!(ctx.comments()[0].0.resolved, "the resolve must land");

    // Another reader's copy, cut before that, still says open.
    let stale = ctx.write_bytes(
        "returned-3.docx",
        &returning_docx_for(uid, reply_uid, false),
    );
    let rows = ctx.analyse(vec![stale], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let after = ctx.comments();
    assert_eq!(after.len(), 1, "still one thread: {after:#?}");
    assert!(
        after[0].0.resolved,
        "a copy cut before the resolution re-opened the thread"
    );
}

/// The decisive proof, for DOCX: a returning file carrying an already-recognised
/// comment's real uid updates that row in place — same id, same uid, refreshed
/// body/resolved state — and repoints it at the NEW import's own `Content` row,
/// never leaving it on the row the first import created.
#[test]
fn reimporting_a_returning_docx_recognises_the_comment_instead_of_duplicating_it() {
    let mut ctx = Ctx::new();

    let first_path = ctx.write_bytes("returned-1.docx", &first_returning_docx());
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let before = ctx.comments();
    assert_eq!(
        before.len(),
        1,
        "one thread from the first import: {before:#?}"
    );
    let (first_comment, first_replies) = &before[0];
    assert_ne!(
        first_comment.uid,
        uuid::Uuid::nil(),
        "apply_document_import_uc must mint a uid for an editor-authored comment"
    );
    let local_uid = first_comment.uid;
    let local_id = first_comment.id;
    assert_eq!(first_replies.len(), 1);
    let reply_uid = first_replies[0].uid;
    let reply_id = first_replies[0].id;

    let second_bytes = returning_docx_for(local_uid, reply_uid, true);
    let second_path = ctx.write_bytes("returned-2.docx", &second_bytes);
    let rows = ctx.analyse(vec![second_path], ImportRowKind::Book);
    let second_created = ctx.apply(rows, 0);

    let after = ctx.comments();
    assert_eq!(
        after.len(),
        1,
        "the recognised comment must be updated, not duplicated: {after:#?}"
    );
    let (updated_comment, updated_replies) = &after[0];
    assert_eq!(updated_comment.id, local_id, "same row, not a new one");
    assert_eq!(updated_comment.uid, local_uid, "identity never changes");
    assert!(
        updated_comment.resolved,
        "the returning file's resolved state is authoritative"
    );
    assert!(
        updated_comment.body.contains("closely"),
        "the returning file's body is authoritative: {:?}",
        updated_comment.body
    );
    assert_eq!(
        updated_comment.author_initials, "ED",
        "w:initials must survive the whole journey, not just the scanner"
    );

    assert_eq!(updated_replies.len(), 1, "still one reply, not two");
    assert_eq!(
        updated_replies[0].id, reply_id,
        "the reply row is the same one, updated in place"
    );
    assert_eq!(updated_replies[0].uid, reply_uid);
    assert_eq!(updated_replies[0].body, "Will do, on it now.");

    // The comment's `content` must now point at the SECOND import's row, not
    // the first's.
    let second_content_id = ctx.content_id_of(second_created[0]);
    assert!(
        second_content_id.is_some(),
        "the second row must carry prose"
    );
    assert_eq!(
        updated_comment.content, second_content_id,
        "a recognised comment repoints at whichever import most recently touched it"
    );
}

/// The ODT twin of the DOCX test above — same recognition, different container.
/// `author_initials` is not asserted here: ODF has no carrier for it at all (a
/// documented format ceiling — see `export_odt_uc`'s module doc), so it is
/// always empty regardless of what either returning file claimed.
#[test]
fn reimporting_a_returning_odt_recognises_the_comment_instead_of_duplicating_it() {
    let mut ctx = Ctx::new();

    let first_path = ctx.write_bytes("returned-1.odt", &first_returning_odt());
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let before = ctx.comments();
    assert_eq!(before.len(), 1, "{before:#?}");
    let (first_comment, first_replies) = &before[0];
    let local_uid = first_comment.uid;
    let local_id = first_comment.id;
    let reply_uid = first_replies[0].uid;
    let reply_id = first_replies[0].id;

    let second_bytes = build_odt(MANUSCRIPT, |doc| {
        let range = find_range(doc, "needs review");
        let mut root = DocumentComment {
            start: range.0,
            end: range.1,
            uid: local_uid.to_string(),
            author: "Editor".to_string(),
            author_initials: String::new(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: true,
            body: "Please look at this *closely*.".to_string(),
            replies: Vec::new(),
        };
        root.replies.push(TdCommentReply {
            uid: reply_uid.to_string(),
            author: "Writer".to_string(),
            author_initials: String::new(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "Will do, on it now.".to_string(),
        });
        let mut comments = DocumentComments::new();
        comments.insert(root);
        comments
    });
    let second_path = ctx.write_bytes("returned-2.odt", &second_bytes);
    let rows = ctx.analyse(vec![second_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let after = ctx.comments();
    assert_eq!(after.len(), 1, "must not duplicate: {after:#?}");
    let (updated_comment, updated_replies) = &after[0];
    assert_eq!(updated_comment.id, local_id);
    assert_eq!(updated_comment.uid, local_uid);
    assert!(updated_comment.resolved);
    assert!(updated_comment.body.contains("closely"));
    assert_eq!(updated_replies.len(), 1);
    assert_eq!(updated_replies[0].id, reply_id);
}

/// A second, immediately-following round trip must still not duplicate — the
/// bug this milestone exists to fix was exactly "export -> edit -> import
/// TWICE", not merely once.
#[test]
fn a_second_consecutive_round_trip_still_does_not_duplicate() {
    let mut ctx = Ctx::new();

    let first_path = ctx.write_bytes("returned-1.docx", &first_returning_docx());
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let seed = ctx.comments();
    let local_uid = seed[0].0.uid;
    let local_id = seed[0].0.id;
    let reply_uid = seed[0].1[0].uid;

    for (name, resolved) in [("returned-2.docx", false), ("returned-3.docx", true)] {
        let bytes = returning_docx_for(local_uid, reply_uid, resolved);
        let path = ctx.write_bytes(name, &bytes);
        let rows = ctx.analyse(vec![path], ImportRowKind::Book);
        ctx.apply(rows, 0);

        let comments = ctx.comments();
        assert_eq!(
            comments.len(),
            1,
            "round trip {name} duplicated the comment: {comments:#?}"
        );
        assert_eq!(
            comments[0].0.id, local_id,
            "round trip {name} changed the row identity"
        );
        assert_eq!(
            comments[0].1.len(),
            1,
            "round trip {name} duplicated the reply"
        );
    }
}

/// Undoing a re-import that RECOGNISED a comment must restore that row's
/// PREVIOUS state, not merely detach it — the `updated_comments`/
/// `updated_replies` machinery `execute`'s own module doc describes, as
/// distinct from `set_comments_attached` (which only ever applies to a row this
/// transaction *created*). A detach here would make an already-existing,
/// previously-visible comment vanish, which is a correctness bug undo must
/// never have.
#[test]
fn undoing_a_recognised_update_restores_its_previous_state_not_just_detaches_it() {
    let mut ctx = Ctx::new();

    let first_path = ctx.write_bytes("returned-1.docx", &first_returning_docx());
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    let first_created = ctx.apply(rows, 0);

    let seed = ctx.comments();
    let local_uid = seed[0].0.uid;
    let local_id = seed[0].0.id;
    let reply_uid = seed[0].1[0].uid;
    let reply_id = seed[0].1[0].id;
    let first_content_id = ctx.content_id_of(first_created[0]);

    let second_bytes = returning_docx_for(local_uid, reply_uid, true);
    let second_path = ctx.write_bytes("returned-2.docx", &second_bytes);
    let rows = ctx.analyse(vec![second_path], ImportRowKind::Book);
    let second_created = ctx.apply(rows, 0);
    let second_content_id = ctx.content_id_of(second_created[0]);

    // Sanity: the update really did happen before undo gets to work.
    let updated = ctx.comments();
    assert_eq!(updated.len(), 1);
    assert!(updated[0].0.resolved);
    assert!(updated[0].0.body.contains("closely"));
    assert_eq!(updated[0].0.content, second_content_id);

    ctx.undo
        .undo(None)
        .expect("undo the second (recognising) import");

    let after_undo = ctx.comments();
    assert_eq!(
        after_undo.len(),
        1,
        "the comment must still be there — undo must not detach a row that \
         already existed before this import ran: {after_undo:#?}"
    );
    let (reverted_comment, reverted_replies) = &after_undo[0];
    assert_eq!(reverted_comment.id, local_id, "same row throughout");
    assert_eq!(reverted_comment.uid, local_uid);
    assert!(
        !reverted_comment.resolved,
        "undo must restore the PRE-second-import resolved state"
    );
    assert_eq!(
        reverted_comment.body, "Please look at this.",
        "undo must restore the pre-second-import body"
    );
    assert_eq!(
        reverted_comment.content, first_content_id,
        "undo must repoint content back at the row it was on before this import"
    );
    assert_eq!(reverted_replies.len(), 1);
    assert_eq!(reverted_replies[0].id, reply_id);
    assert_eq!(reverted_replies[0].body, "Will do.");

    ctx.undo
        .redo(None)
        .expect("redo the second (recognising) import");
    let after_redo = ctx.comments();
    assert_eq!(after_redo.len(), 1);
    assert!(after_redo[0].0.resolved, "redo must re-apply the update");
    assert!(after_redo[0].0.body.contains("closely"));
    assert_eq!(after_redo[0].0.content, second_content_id);
    assert_eq!(after_redo[0].1[0].body, "Will do, on it now.");
}

/// A comment the file carries no recognisable uid for — an editor's own brand
/// new remark, added alongside an already-recognised one — is genuinely new: it
/// is created, not merged into the recognised thread, and mints its own fresh
/// uid distinct from every uid already in play.
#[test]
fn an_editors_own_new_comment_with_no_uid_is_created_and_gets_a_fresh_uid() {
    let mut ctx = Ctx::new();

    let first_path = ctx.write_bytes("returned-1.docx", &first_returning_docx());
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let seed = ctx.comments();
    let local_uid = seed[0].0.uid;
    let reply_uid = seed[0].1[0].uid;

    let second_bytes = build_docx(MANUSCRIPT, |doc| {
        let recognised_range = find_range(doc, "needs review");
        let mut recognised = DocumentComment {
            start: recognised_range.0,
            end: recognised_range.1,
            uid: local_uid.to_string(),
            author: "Editor".to_string(),
            author_initials: "ED".to_string(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: false,
            body: "Please look at this.".to_string(),
            replies: Vec::new(),
        };
        recognised.replies.push(TdCommentReply {
            uid: reply_uid.to_string(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "Will do.".to_string(),
        });

        let new_range = find_range(doc, "second, unrelated paragraph");
        let brand_new = DocumentComment {
            start: new_range.0,
            end: new_range.1,
            uid: String::new(),
            author: "New Editor".to_string(),
            author_initials: "NE".to_string(),
            date: "2026-02-01T00:00:00Z".to_string(),
            resolved: false,
            body: "Totally new remark.".to_string(),
            replies: Vec::new(),
        };

        let mut comments = DocumentComments::new();
        comments.insert(recognised);
        comments.insert(brand_new);
        comments
    });
    let second_path = ctx.write_bytes("returned-2.docx", &second_bytes);
    let rows = ctx.analyse(vec![second_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let after = ctx.comments();
    assert_eq!(
        after.len(),
        2,
        "the recognised comment stays one row, the new remark becomes a second: {after:#?}"
    );
    let (new_comment, _) = after
        .iter()
        .find(|(c, _)| c.body == "Totally new remark.")
        .expect("the brand new comment");
    assert_ne!(
        new_comment.uid,
        uuid::Uuid::nil(),
        "a freshly created comment must mint a real uid"
    );
    assert_ne!(
        new_comment.uid, local_uid,
        "the new comment's uid must not collide with the recognised one's"
    );
    assert!(
        after.iter().any(|(c, _)| c.uid == local_uid),
        "the recognised comment must still be there, untouched in identity"
    );
}

/// A reply an editor inserts in the MIDDLE of a thread must not make the reply
/// that used to sit in that position — and every reply after it — look like a
/// new one. Matching is by uid, never by position: R1 stays R1, R2 stays R2,
/// wherever they now sit in the file's own order, and only the truly new,
/// uid-less middle reply gets created.
#[test]
fn a_reply_inserted_mid_conversation_does_not_duplicate_the_replies_after_it() {
    let mut ctx = Ctx::new();

    let first_bytes = build_docx(MANUSCRIPT, |doc| {
        let range = find_range(doc, "needs review");
        let mut root = DocumentComment {
            start: range.0,
            end: range.1,
            uid: String::new(),
            author: "Editor".to_string(),
            author_initials: "ED".to_string(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: false,
            body: "Please look at this.".to_string(),
            replies: Vec::new(),
        };
        root.replies.push(TdCommentReply {
            uid: String::new(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "First reply.".to_string(),
        });
        root.replies.push(TdCommentReply {
            uid: String::new(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T02:00:00Z".to_string(),
            body: "Second reply.".to_string(),
        });
        let mut comments = DocumentComments::new();
        comments.insert(root);
        comments
    });
    let first_path = ctx.write_bytes("returned-1.docx", &first_bytes);
    let rows = ctx.analyse(vec![first_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let seed = ctx.comments();
    assert_eq!(
        seed[0].1.len(),
        2,
        "two replies from the first import: {seed:#?}"
    );
    let comment_uid = seed[0].0.uid;
    let r1 = &seed[0].1[0];
    let r2 = &seed[0].1[1];
    assert_eq!(r1.body, "First reply.");
    assert_eq!(r2.body, "Second reply.");
    let (r1_id, r1_uid) = (r1.id, r1.uid);
    let (r2_id, r2_uid) = (r2.id, r2.uid);

    // The editor's return: R1 first, then a brand-new reply with no uid, then
    // R2 — in the file's own order, exactly as an editor inserting a reply
    // mid-thread in Word or LibreOffice would produce.
    let second_bytes = build_docx(MANUSCRIPT, |doc| {
        let range = find_range(doc, "needs review");
        let mut root = DocumentComment {
            start: range.0,
            end: range.1,
            uid: comment_uid.to_string(),
            author: "Editor".to_string(),
            author_initials: "ED".to_string(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: false,
            body: "Please look at this.".to_string(),
            replies: Vec::new(),
        };
        root.replies.push(TdCommentReply {
            uid: r1_uid.to_string(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T01:00:00Z".to_string(),
            body: "First reply.".to_string(),
        });
        root.replies.push(TdCommentReply {
            uid: String::new(),
            author: "Editor".to_string(),
            author_initials: "ED".to_string(),
            date: "2026-01-01T01:30:00Z".to_string(),
            body: "Inserted reply.".to_string(),
        });
        root.replies.push(TdCommentReply {
            uid: r2_uid.to_string(),
            author: "Writer".to_string(),
            author_initials: "WR".to_string(),
            date: "2026-01-01T02:00:00Z".to_string(),
            body: "Second reply.".to_string(),
        });
        let mut comments = DocumentComments::new();
        comments.insert(root);
        comments
    });
    let second_path = ctx.write_bytes("returned-2.docx", &second_bytes);
    let rows = ctx.analyse(vec![second_path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let after = ctx.comments();
    assert_eq!(after.len(), 1, "still one thread: {after:#?}");
    let replies = &after[0].1;
    assert_eq!(
        replies.len(),
        3,
        "two recognised replies plus one genuinely new one: {replies:#?}"
    );

    assert_eq!(replies[0].id, r1_id, "R1 keeps its row");
    assert_eq!(replies[0].uid, r1_uid, "R1 keeps its identity");
    assert_eq!(replies[0].body, "First reply.");

    assert_eq!(
        replies[1].body, "Inserted reply.",
        "the new reply lands in the middle"
    );
    assert_ne!(replies[1].id, r1_id);
    assert_ne!(
        replies[1].id, r2_id,
        "the inserted reply must be a genuinely new row"
    );
    assert_ne!(replies[1].uid, uuid::Uuid::nil(), "it must mint a real uid");

    assert_eq!(
        replies[2].id, r2_id,
        "R2 keeps its row even though it no longer sits at index 1 — matched by uid, not position"
    );
    assert_eq!(replies[2].uid, r2_uid, "R2 keeps its identity");
    assert_eq!(replies[2].body, "Second reply.");
}

// ── The carrier that actually survives an editor ────────────────────────────
//
// Everything in the M-S7 section above recognises a returning comment by its
// `skrb:uid` — an attribute **both Word and LibreOffice delete on save**. Measured
// against a real returning file: a manuscript exported from this app, commented in
// LibreOffice 25.8 and saved, came back with every `skrb:uid` gone and the namespace
// declaration with them. So those tests describe a file nobody has opened, and the
// path they cover never runs in practice.
//
// What survives is a **bookmark**, and these are the tests for the recognition that
// rests on one. The returning files below carry an empty `uid` — exactly what the
// scanner sees after an editor's save — and identify their comment only through a
// `skrb_c…` mark naming the local row's own tag.

/// As [`build_odt`], plus the round-trip marks a real export writes beside the comments.
fn build_odt_marked(
    djot: &str,
    make: impl FnOnce(&TextDocument) -> (DocumentComments, text_document::DocumentMarks),
) -> Vec<u8> {
    let doc = TextDocument::new();
    doc.set_djot_sync(djot).expect("set_djot_sync");
    let (comments, marks) = make(&doc);

    let path = std::env::temp_dir().join(format!(
        "import_mgmt_odt_marked_{}_{}.odt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));
    doc.to_odt_with_options(
        &path.to_string_lossy(),
        OdtExportOptions {
            comments,
            marks,
            ..Default::default()
        },
    )
    .expect("to_odt_with_options")
    .wait()
    .expect("odt export completes");
    let bytes = std::fs::read(&path).expect("read exported odt");
    let _ = std::fs::remove_file(&path);
    bytes
}

/// A returning `.odt` whose comment is identified **only** by its mark: the uid is
/// empty, the way an editor's save leaves it.
fn returning_odt_marked_only(
    comment_uid: uuid::Uuid,
    body: &str,
    replies: Vec<(&str, &str, &str)>,
) -> Vec<u8> {
    let replies: Vec<(String, String, String)> = replies
        .into_iter()
        .map(|(a, d, b)| (a.to_string(), d.to_string(), b.to_string()))
        .collect();
    let body = body.to_string();
    build_odt_marked(MANUSCRIPT, move |doc| {
        let range = find_range(doc, "needs review");
        let root = DocumentComment {
            start: range.0,
            end: range.1,
            // Gone, as an editor's save leaves it.
            uid: String::new(),
            author: "Editor".to_string(),
            author_initials: String::new(),
            date: "2026-01-01T00:00:00Z".to_string(),
            resolved: false,
            body,
            replies: replies
                .into_iter()
                .map(|(author, date, body)| TdCommentReply {
                    uid: String::new(),
                    author,
                    author_initials: String::new(),
                    date,
                    body,
                })
                .collect(),
        };
        let mut comments = DocumentComments::new();
        comments.insert(root);

        let mut marks = text_document::DocumentMarks::new();
        marks.insert(text_document::DocumentMark::range(
            range.0,
            range.1,
            skribisto_model::round_trip::comment_mark_name(&comment_uid),
        ));
        (comments, marks)
    })
}

/// The decisive proof for the carrier that survives: a returning file whose comment
/// has **no uid at all** still updates the row it names, because its mark does.
///
/// Without this, every editorial round trip duplicates every comment it brings home —
/// and the uid-based tests above would all still pass.
#[test]
fn a_comment_with_no_uid_is_recognised_by_its_round_trip_mark() {
    let mut ctx = Ctx::new();
    let first = ctx.write_bytes("first.odt", &first_returning_odt());
    let rows = ctx.analyse(vec![first], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let before = ctx.comments();
    assert_eq!(before.len(), 1, "the first import creates one comment");
    let local_uid = before[0].0.uid;
    let local_id = before[0].0.id;

    let again = ctx.write_bytes(
        "again.odt",
        &returning_odt_marked_only(local_uid, "Please look at this *closely*.", vec![]),
    );
    let rows = ctx.analyse(vec![again], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let after = ctx.comments();
    assert_eq!(
        after.len(),
        1,
        "the mark must identify the existing comment, not add a second: {after:#?}"
    );
    assert_eq!(after[0].0.id, local_id, "same row");
    assert_eq!(
        after[0].0.uid, local_uid,
        "identity is local, never the file's"
    );
    assert_eq!(
        after[0].0.body, "Please look at this *closely*.",
        "the editor's revised wording is what the file is for"
    );
}

/// A reply has no mark of its own — both formats anchor it to the thread's range —
/// so on a real returning file it arrives with no identity whatsoever.
///
/// It is recognised by its natural key instead: the author who wrote it and the moment
/// they did, both carried natively by ODF and OOXML. Without that, the *second* round
/// trip re-creates every reply the first brought home, and a thread grows a duplicate
/// of itself on every exchange.
#[test]
fn a_reply_with_no_uid_is_recognised_by_its_author_and_date() {
    let mut ctx = Ctx::new();
    let first = ctx.write_bytes("first.odt", &first_returning_odt());
    let rows = ctx.analyse(vec![first], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let before = ctx.comments();
    let local_uid = before[0].0.uid;
    assert_eq!(before[0].1.len(), 1, "the first import creates one reply");
    let reply_id = before[0].1[0].id;

    // The same thread coming home again: the original reply (same author, same
    // instant) plus a new one the editor added.
    let again = ctx.write_bytes(
        "again.odt",
        &returning_odt_marked_only(
            local_uid,
            "Please look at this.",
            vec![
                ("Writer", "2026-01-01T01:00:00Z", "Will do."),
                ("Editor", "2026-02-02T02:00:00Z", "Thanks."),
            ],
        ),
    );
    let rows = ctx.analyse(vec![again], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let after = ctx.comments()[0].1.clone();
    assert_eq!(
        after.len(),
        2,
        "the first reply must be recognised, not duplicated: {after:#?}"
    );
    assert!(
        after.iter().any(|r| r.id == reply_id),
        "the existing reply kept its row"
    );
    assert!(
        after.iter().any(|r| r.body == "Thanks."),
        "the editor's new reply arrived"
    );
}

// ── Bringing a returning file home to the rows it came from ─────────────────
//
// Every test above creates. These update: the writer sends a draft out, the editor
// marks it up, and the file comes back to *the book it left from* rather than
// landing beside it as a second copy. `ApplyImportRow::Update` names its target by
// the round-trip mark the file carried; `replace_prose` says whether the editor's
// wording is wanted, or only their remarks.

impl Ctx {
    /// Apply hand-built rows — the update half, which no analysis produces on its own
    /// because choosing between "take their wording" and "take only their notes" is a
    /// decision the writer makes in the wizard.
    fn apply_rows(&mut self, rows: Vec<ApplyImportRow>) -> Vec<EntityId> {
        self.try_apply_rows(rows)
            .expect("apply")
            .created_ids
            .clone()
    }

    /// As [`Ctx::apply_rows`], for the cases where refusing *is* the behaviour under test.
    fn try_apply_rows(
        &mut self,
        rows: Vec<ApplyImportRow>,
    ) -> anyhow::Result<ApplyDocumentImportResultDto> {
        import_management_controller::apply_document_import(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &ApplyDocumentImportDto {
                work_id: self.work_id,
                binder_id: self.binder_id,
                anchor_item_id: 0,
                drop_position: DropPosition::Into,
                row: ApplyImportRow::Empty,
                rows: ApplyImportRows::Create(rows),
            },
        )
    }

    /// Trash a row the way the trash model does: `activated` flips, the row stays put.
    fn trash(&mut self, item_id: EntityId) {
        let item = binder_item_controller::get(&self.db, &item_id)
            .expect("item")
            .expect("item row");
        binder_item_controller::update(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &UpdateBinderItemDto {
                activated: false,
                ..to_update_dto(item)
            },
        )
        .expect("trash the row");
    }

    /// The one item in this binder whose title is `title`.
    fn item_named(&self, title: &str) -> EntityId {
        self.binder_order()
            .into_iter()
            .find(|id| self.title_of(*id) == title)
            .unwrap_or_else(|| panic!("no row titled {title:?}"))
    }
}

/// One ordinary import, and the mark tag its row would be named by on the way back.
fn a_project_with_one_row(ctx: &mut Ctx) -> (EntityId, String) {
    let path = ctx.write("chapter.md", "# Chapter One\n\nThe original wording.\n");
    let rows = ctx.analyse(vec![path], ImportRowKind::Book);
    ctx.apply(rows, 0);

    let item_id = ctx.item_named("Chapter One");
    let uid = binder_item_controller::get(&ctx.db, &item_id)
        .expect("item")
        .expect("item row")
        .uid;
    assert!(!uid.is_nil(), "a created row mints a durable identity");
    (item_id, skribisto_model::round_trip::uid_tag(&uid))
}

/// A Book: a row the constraint matrix gives no prose `Content` at all.
fn a_row_that_stores_no_prose(ctx: &mut Ctx) -> EntityId {
    ctx.apply_rows(vec![ApplyImportRow::Create {
        indent: 0,
        kind: ImportRowKind::Book,
        title: "A Book".into(),
        djot: String::new(),
        epigraph: String::new(),
        comments: Vec::new(),
        source_uid_tag: String::new(),
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }]);
    let book = ctx.item_named("A Book");
    assert!(
        ctx.prose_of(book).is_none(),
        "a Book must store no prose for these tests to mean anything"
    );
    book
}

fn update_row(
    tag: &str,
    replace_prose: bool,
    djot: &str,
    comments: Vec<ImportComment>,
) -> ApplyImportRow {
    ApplyImportRow::Update {
        target_uid_tag: tag.to_string(),
        replace_prose,
        djot: djot.to_string(),
        // These tests are about prose and comments coming home; `update_row_with_epigraph`
        // below is the one that exercises the second `Content`.
        epigraph: String::new(),
        comments,
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }
}

fn an_editors_comment(body: &str, exact: &str, prefix: &str, suffix: &str) -> ImportComment {
    ImportComment::Found {
        kind: ImportCommentKind::Range,
        uid: None,
        uid_tag: String::new(),
        author_name: "Editor".into(),
        author_initials: "ED".into(),
        created_at: "2026-01-01T00:00:00Z".into(),
        body: body.into(),
        resolved: false,
        orphaned: false,
        orphan_reason: ImportOrphanReason::NotOrphaned,
        range_start: prefix.chars().count() as i64,
        range_length: exact.chars().count() as i64,
        quote_prefix: prefix.into(),
        quote_exact: exact.into(),
        quote_exact_truncated: false,
        quote_suffix: suffix.into(),
        block_ordinal_hint: 0,
        replies: Vec::new(),
    }
}

/// The case the whole feature exists for: the editor's remarks come home and not one
/// word of the manuscript is touched.
#[test]
fn comments_only_brings_the_notes_and_leaves_the_prose_alone() {
    let mut ctx = Ctx::new();
    let (item_id, tag) = a_project_with_one_row(&mut ctx);

    ctx.apply_rows(vec![update_row(
        &tag,
        false,
        "PROSE THAT MUST NOT BE WRITTEN",
        vec![an_editors_comment(
            "Is this the right word?",
            "original",
            "The ",
            " wording.",
        )],
    )]);

    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The original wording."),
        "comments-only must not touch a word of the manuscript"
    );
    let comments = ctx.comments();
    assert_eq!(comments.len(), 1, "the editor's remark came home");
    assert_eq!(comments[0].0.body, "Is this the right word?");
    assert_eq!(
        comments[0].0.content,
        ctx.content_id_of(item_id),
        "and is anchored to the row it was made on"
    );
}

/// The other half: the editor rewrote the chapter and the writer wants their wording.
#[test]
fn take_import_replaces_the_prose_of_the_row_the_mark_names() {
    let mut ctx = Ctx::new();
    let (item_id, tag) = a_project_with_one_row(&mut ctx);
    let before = ctx.binder_order().len();

    ctx.apply_rows(vec![update_row(
        &tag,
        true,
        "The editor's better wording.",
        Vec::new(),
    )]);

    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The editor's better wording.")
    );
    assert_eq!(
        ctx.binder_order().len(),
        before,
        "an update must not create a second copy of the row"
    );
}

/// A tag naming a row this project no longer has is skipped, not fatal: the writer may
/// have deleted the chapter between sending the draft and reading it back, which is an
/// ordinary thing to do — and the rest of the file still has to land.
#[test]
fn an_update_for_a_row_that_is_gone_is_skipped_not_fatal() {
    let mut ctx = Ctx::new();
    let (item_id, tag) = a_project_with_one_row(&mut ctx);
    let stale = skribisto_model::round_trip::uid_tag(&uuid::Uuid::from_u128(0xdead_beef));
    assert_ne!(stale, tag);

    ctx.apply_rows(vec![update_row(&stale, true, "Nowhere to go.", Vec::new())]);

    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The original wording."),
        "a stale tag must not write into some other row"
    );
}

/// A trashed row is not a place to put the editor's work.
///
/// Trashing flips `activated` and leaves the row exactly where it was, so its tag still
/// resolves in the binder's ordered list. The wizard filters trashed rows out when it builds
/// the merge, but a `Work` can be open in several windows — the writer can trash a chapter in
/// one while the import wizard sits on the merge step in another. An update that landed then
/// would put the editor's prose and remarks somewhere the outline does not show, to be found
/// only by restoring, or lost by emptying the trash.
#[test]
fn an_update_never_writes_into_a_trashed_row() {
    let mut ctx = Ctx::new();
    let (item_id, tag) = a_project_with_one_row(&mut ctx);

    // Between the merge and the Import button.
    ctx.trash(item_id);

    ctx.apply_rows(vec![update_row(
        &tag,
        true,
        "The editor's better wording.",
        vec![an_editors_comment(
            "Is this the right word?",
            "original",
            "The ",
            " wording.",
        )],
    )]);

    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The original wording."),
        "a trashed row keeps whatever it had"
    );
    assert!(
        ctx.comments().is_empty(),
        "and gains no remark it would hide"
    );
}

/// A row that stores no prose cannot quietly swallow a returning one.
///
/// A Book, a Part or a bare folder has no prose `Content` to write into or anchor against. An
/// update naming one used to be skipped in silence — prose and every comment on it gone, with
/// a result DTO that counts only creations, so nothing downstream could tell. The creation
/// pass refuses the identical shape outright; this is the same refusal on the other path.
///
/// Nothing has been written when it refuses: the import is one transaction, so the writer
/// lands back on the wizard with the row still there to untick or retype.
#[test]
fn an_update_onto_a_row_that_stores_no_prose_refuses_rather_than_dropping_it() {
    let mut ctx = Ctx::new();
    let (_, _) = a_project_with_one_row(&mut ctx);

    let book = a_row_that_stores_no_prose(&mut ctx);
    let uid = binder_item_controller::get(&ctx.db, &book)
        .expect("item")
        .expect("item row")
        .uid;
    let tag = skribisto_model::round_trip::uid_tag(&uid);

    let err = ctx
        .try_apply_rows(vec![update_row(
            &tag,
            true,
            "Front matter the editor rewrote.",
            vec![an_editors_comment("Fix this", "matter", "Front ", " the")],
        )])
        .expect_err("an update with nowhere to put its prose must not report success");
    let text = format!("{err:#}");
    assert!(
        text.contains("stores none"),
        "the refusal has to say what was about to be lost: {text}"
    );

    // And nothing landed: one transaction, refused whole.
    assert!(ctx.comments().is_empty(), "no comment was written");
}

/// Comments-only onto a prose-less row is not a loss, so it must not refuse.
///
/// The refusal above asks what would be *lost*, which is not what the row carries: a
/// comments-only update never writes its prose — the writer asked for the remarks and nothing
/// else — so prose it was never going to store is nothing to lose. Refusing over it aborted
/// an ordinary re-import, and the wizard could not have warned the writer first: its own
/// blocking check cannot see this shape, so Import stayed enabled and failed on press.
#[test]
fn comments_only_onto_a_prose_less_row_does_not_refuse_over_prose_it_would_not_write() {
    let mut ctx = Ctx::new();
    let (chapter, _) = a_project_with_one_row(&mut ctx);

    let book = a_row_that_stores_no_prose(&mut ctx);
    let uid = binder_item_controller::get(&ctx.db, &book)
        .expect("item")
        .expect("item row")
        .uid;
    let tag = skribisto_model::round_trip::uid_tag(&uid);

    // Carries prose, and is explicitly not going to write it.
    ctx.try_apply_rows(vec![update_row(
        &tag,
        false,
        "Front matter the row cannot hold anyway.",
        Vec::new(),
    )])
    .expect("a comments-only update must not abort over prose it would not write");

    assert_eq!(
        ctx.prose_of(chapter).as_deref(),
        Some("The original wording."),
        "and nothing else moved"
    );
}

/// The same shape carrying nothing is not an error — an empty row matched an empty row, and
/// there is no loss to report.
#[test]
fn an_update_onto_a_prose_less_row_carrying_nothing_is_simply_skipped() {
    let mut ctx = Ctx::new();
    let (chapter, _) = a_project_with_one_row(&mut ctx);

    let book = a_row_that_stores_no_prose(&mut ctx);
    let uid = binder_item_controller::get(&ctx.db, &book)
        .expect("item")
        .expect("item row")
        .uid;
    let tag = skribisto_model::round_trip::uid_tag(&uid);

    ctx.apply_rows(vec![update_row(&tag, true, "   ", Vec::new())]);

    assert_eq!(
        ctx.prose_of(chapter).as_deref(),
        Some("The original wording."),
        "and the rest of the project is untouched"
    );
}

/// Undo has to put the previous prose back.
///
/// The binder snapshot the import already takes is whole-store and restores the
/// destination subtree — which reaches Contents. This is the test that says so, because
/// "it should already be covered" is exactly the kind of claim that is wrong once.
#[test]
fn undoing_an_update_restores_the_prose_it_overwrote() {
    let mut ctx = Ctx::new();
    let (item_id, tag) = a_project_with_one_row(&mut ctx);

    ctx.apply_rows(vec![update_row(
        &tag,
        true,
        "The editor's better wording.",
        Vec::new(),
    )]);
    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The editor's better wording.")
    );

    ctx.undo.undo(None).expect("undo");
    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The original wording."),
        "undo must restore the writer's own prose"
    );
}

// ── Epigraphs ───────────────────────────────────────────────────────────────
//
// A Part's or a Chapter's epigraph is a `Content` of its own — `ContentRole::EpigraphText`
// beside the row's `SceneText`, never folded into it. Folding is the bug the reader-side
// fix closes: an epigraph inside the manuscript is duplicated on every round trip (the
// stored `EpigraphText` is untouched, so the next export writes it again) and its words
// are counted as the writer's, which `skribisto_model`'s
// `an_epigraph_is_never_counted_as_prose` pins against.

/// Every `Content` on an item, as (role, data), so a test can say which is which.
fn contents_of(ctx: &Ctx, item_id: EntityId) -> Vec<(ContentRole, String)> {
    binder_item_controller::get_relationship(
        &ctx.db,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .expect("contents")
    .into_iter()
    .map(|id| {
        let c = content_controller::get(&ctx.db, &id)
            .expect("content")
            .expect("content row");
        (c.role, c.data)
    })
    .collect()
}

fn epigraph_of(ctx: &Ctx, item_id: EntityId) -> Option<String> {
    contents_of(ctx, item_id)
        .into_iter()
        .find(|(role, _)| *role == ContentRole::EpigraphText)
        .map(|(_, data)| data)
}

/// The mark tag a row would be named by on the way back out.
fn tag_of(ctx: &Ctx, item_id: EntityId) -> String {
    let uid = binder_item_controller::get(&ctx.db, &item_id)
        .expect("item")
        .expect("item row")
        .uid;
    skribisto_model::round_trip::uid_tag(&uid)
}

fn create_chapter_with_epigraph(title: &str, djot: &str, epigraph: &str) -> ApplyImportRow {
    ApplyImportRow::Create {
        indent: 0,
        kind: ImportRowKind::Chapter,
        title: title.into(),
        djot: djot.into(),
        epigraph: epigraph.into(),
        comments: Vec::new(),
        source_uid_tag: String::new(),
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }
}

#[test]
fn a_created_chapters_epigraph_becomes_its_own_content_row() {
    let mut ctx = Ctx::new();
    ctx.apply_rows(vec![create_chapter_with_epigraph(
        "Chapter One",
        "The city held its breath.",
        "> Every winter asks twice.",
    )]);

    let item = ctx.item_named("Chapter One");
    assert_eq!(
        epigraph_of(&ctx, item).as_deref(),
        Some("> Every winter asks twice."),
        "the quotation must be stored under EpigraphText"
    );
    // The half that actually matters: it is *not* also in the manuscript. This is the
    // duplication the whole feature exists to stop.
    assert_eq!(
        ctx.prose_of(item).as_deref(),
        Some("The city held its breath."),
        "the prose must be the row's own words and nothing else"
    );
}

/// The prose is still reachable through `Contents` after the epigraph joins it.
///
/// `prose_content_of` and `Ctx::prose_of` both read that relationship, and an epigraph
/// written with a *second* `set_binder_item_relationship` call rather than one would have
/// replaced the prose's id instead of joining it. That failure leaves a `Content` row
/// alive in the store with nothing pointing at it — invisible in the app and gone at the
/// next save, which is the worst shape a loss can take.
#[test]
fn adding_an_epigraph_does_not_detach_the_rows_prose() {
    let mut ctx = Ctx::new();
    ctx.apply_rows(vec![create_chapter_with_epigraph(
        "Chapter One",
        "The city held its breath.",
        "> Every winter asks twice.",
    )]);

    let item = ctx.item_named("Chapter One");
    let roles: Vec<ContentRole> = contents_of(&ctx, item)
        .into_iter()
        .map(|(role, _)| role)
        .collect();
    assert_eq!(
        roles,
        vec![ContentRole::SceneText, ContentRole::EpigraphText],
        "both contents must survive, prose first"
    );
}

/// The review step lets the writer retype a row, so a chapter's epigraph can arrive on a
/// Scene — which the matrix gives no `EpigraphText`. The quotation moves to the head of
/// the prose rather than the import being refused: refusing would abort every other row
/// over one deliberate retype, and the text has somewhere sensible to go.
#[test]
fn an_epigraph_retyped_onto_a_row_that_cannot_hold_one_is_folded_into_its_prose() {
    let mut ctx = Ctx::new();
    ctx.apply_rows(vec![ApplyImportRow::Create {
        indent: 0,
        kind: ImportRowKind::Scene,
        title: "Scene A".into(),
        djot: "The city held its breath.".into(),
        epigraph: "> Every winter asks twice.".into(),
        comments: Vec::new(),
        source_uid_tag: String::new(),
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }]);

    let item = ctx.item_named("Scene A");
    assert!(
        epigraph_of(&ctx, item).is_none(),
        "a Scene must store no EpigraphText"
    );
    assert_eq!(
        ctx.prose_of(item).as_deref(),
        Some("> Every winter asks twice.\n\nThe city held its breath."),
        "the quotation opens the prose it was heading, rather than being lost"
    );
}

/// A returning file's epigraph rides `replace_prose` with the manuscript: an editor who
/// corrected the attribution under a chapter's quotation edited the book.
#[test]
fn an_update_writes_the_editors_epigraph_when_it_takes_their_wording() {
    let mut ctx = Ctx::new();
    ctx.apply_rows(vec![create_chapter_with_epigraph(
        "Chapter One",
        "The original wording.",
        "> As first written.",
    )]);
    let item = ctx.item_named("Chapter One");
    let tag = tag_of(&ctx, item);

    ctx.apply_rows(vec![ApplyImportRow::Update {
        target_uid_tag: tag,
        replace_prose: true,
        djot: "The editor's better wording.".into(),
        epigraph: "> As the editor corrected it.".into(),
        comments: Vec::new(),
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }]);

    assert_eq!(
        epigraph_of(&ctx, item).as_deref(),
        Some("> As the editor corrected it.")
    );
    assert_eq!(
        ctx.prose_of(item).as_deref(),
        Some("The editor's better wording.")
    );
}

/// A comments-only update leaves the epigraph exactly as it leaves the manuscript.
///
/// That is the case the whole re-import feature exists for — bring the editor's remarks
/// home and touch not a word — and an epigraph quietly overwritten by it would be the same
/// betrayal as a paragraph quietly overwritten by it.
#[test]
fn a_comments_only_update_leaves_the_epigraph_alone() {
    let mut ctx = Ctx::new();
    ctx.apply_rows(vec![create_chapter_with_epigraph(
        "Chapter One",
        "The original wording.",
        "> As first written.",
    )]);
    let item = ctx.item_named("Chapter One");
    let tag = tag_of(&ctx, item);

    ctx.apply_rows(vec![ApplyImportRow::Update {
        target_uid_tag: tag,
        replace_prose: false,
        djot: "The editor's better wording.".into(),
        epigraph: "> As the editor corrected it.".into(),
        comments: Vec::new(),
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }]);

    assert_eq!(
        epigraph_of(&ctx, item).as_deref(),
        Some("> As first written."),
        "the writer asked for remarks only"
    );
}

/// An absent epigraph never deletes the one the project holds.
///
/// A returning file carries none whenever the export it came from had
/// `Preset::include_epigraphs` off — a choice made about the *export*, which says nothing
/// about the book. Reading that silence as "delete it" would destroy content on the
/// strength of an absence, and the writer would have no way to know it had happened.
#[test]
fn an_update_carrying_no_epigraph_does_not_delete_the_one_already_there() {
    let mut ctx = Ctx::new();
    ctx.apply_rows(vec![create_chapter_with_epigraph(
        "Chapter One",
        "The original wording.",
        "> As first written.",
    )]);
    let item = ctx.item_named("Chapter One");
    let tag = tag_of(&ctx, item);

    ctx.apply_rows(vec![update_row(
        &tag,
        true,
        "The editor's better wording.",
        Vec::new(),
    )]);

    assert_eq!(
        epigraph_of(&ctx, item).as_deref(),
        Some("> As first written."),
        "an export that did not carry epigraphs must not delete them"
    );
}

/// A chapter that had none gains one, wired into `Contents` beside its prose.
#[test]
fn an_update_can_give_a_row_its_first_epigraph() {
    let mut ctx = Ctx::new();
    let (item_id, tag) = a_project_with_one_row(&mut ctx);
    assert!(epigraph_of(&ctx, item_id).is_none(), "none to begin with");

    ctx.apply_rows(vec![ApplyImportRow::Update {
        target_uid_tag: tag,
        replace_prose: true,
        djot: "The editor's better wording.".into(),
        epigraph: "> Newly added by the editor.".into(),
        comments: Vec::new(),
        source_file_name: String::new(),
        source_file_digest: String::new(),
    }]);

    assert_eq!(
        epigraph_of(&ctx, item_id).as_deref(),
        Some("> Newly added by the editor.")
    );
    assert_eq!(
        ctx.prose_of(item_id).as_deref(),
        Some("The editor's better wording."),
        "and the prose it was appended beside is still reachable"
    );
}

// ── Where each row's words came from ────────────────────────────────────────
//
// The completion event's payload, and the reason it exists: the plan knows which
// file a row was read out of, the store never will, and the moment the import
// commits is the only one where both halves are in hand. Everything below is
// about what an outside reader can honestly conclude from it.

/// Run `rows` and return the origins payload the completion event carried.
///
/// Drained on this thread rather than on one of its own, unlike `events_for`
/// above: that helper counts events over a whole run and has to keep listening,
/// this one wants a single event that is already in the channel by the time
/// `apply_rows` returns. No sleep to be flaky about, no thread to outlive the
/// test, and an event that never comes fails here instead of turning into an
/// assertion about `None` somewhere further down.
fn origins_for(
    ctx: &mut Ctx,
    rows: Vec<ApplyImportRow>,
) -> import_management::events::ImportOrigins {
    use common::event::{ImportManagementEvent, Origin};

    let rx = ctx.hub.subscribe_receiver();
    // ⚠ **Drain what is already queued first.** The channel buffers, and a test
    // that set its scene with an earlier `apply_rows` would otherwise read *that*
    // import's payload and assert happily against the wrong one — which is
    // exactly how the returning-file tests below first "passed" while reporting
    // `Created` for an update.
    while rx.try_recv().is_ok() {}

    ctx.apply_rows(rows);

    while let Ok(event) = rx.recv_timeout(std::time::Duration::from_secs(5)) {
        if matches!(
            event.origin,
            Origin::ImportManagement(ImportManagementEvent::ApplyDocumentImport)
        ) {
            let data = event
                .data
                .as_deref()
                .expect("the apply event must carry a payload");
            return import_management::events::ImportOrigins::from_payload(data)
                .expect("…and it must be an origins payload this build can read");
        }
    }
    panic!("no ApplyDocumentImport event arrived");
}

fn create_from(title: &str, djot: &str, file: &str, digest: &str) -> ApplyImportRow {
    ApplyImportRow::Create {
        indent: 0,
        kind: ImportRowKind::Scene,
        title: title.into(),
        djot: djot.into(),
        epigraph: String::new(),
        comments: Vec::new(),
        source_uid_tag: String::new(),
        source_file_name: file.into(),
        source_file_digest: digest.into(),
    }
}

/// **The false-positive control, and the whole reason this exists.** A 90,000-word
/// import produces the same shape as a manuscript written in an afternoon. What
/// tells them apart is a record able to say *"these rows arrived from this file"*,
/// and this is where that becomes possible at all.
#[test]
fn every_created_row_says_which_file_its_words_arrived_in() {
    let mut ctx = Ctx::new();
    let origins = origins_for(
        &mut ctx,
        vec![
            create_from("One", "First scene.", "novel-draft-3.docx", "aaa111"),
            create_from("Two", "Second scene.", "novel-draft-3.docx", "aaa111"),
            create_from("Three", "From elsewhere.", "notes.md", "bbb222"),
        ],
    );

    assert_eq!(
        origins.version,
        import_management::events::IMPORT_ORIGINS_VERSION
    );
    assert_eq!(origins.rows.len(), 3);
    assert!(
        origins
            .rows
            .iter()
            .all(|r| r.action == import_management::events::ImportAction::Created)
    );
    assert_eq!(
        origins
            .rows
            .iter()
            .map(|r| r.source_file_name.as_str())
            .collect::<Vec<_>>(),
        vec!["novel-draft-3.docx", "novel-draft-3.docx", "notes.md"],
        "a multi-file import must not collapse to one source"
    );
    assert_eq!(origins.rows[2].source_file_digest, "bbb222");
    assert_eq!(
        origins.rows[0].char_count,
        "First scene.".chars().count() as u64
    );
}

/// **The uid, never the store id.** An `EntityId` is re-minted by every
/// `load_work`, so a listener that cached one would be naming an unrelated row
/// the next time the project opened — silently, and with every figure it drew
/// from it rendering perfectly.
#[test]
fn the_payload_names_rows_by_their_durable_uid() {
    let mut ctx = Ctx::new();
    let origins = origins_for(&mut ctx, vec![create_from("One", "Prose.", "a.md", "aaa")]);

    let uid = origins.rows[0]
        .item_uid
        .parse::<uuid::Uuid>()
        .expect("a real uid, hyphenated");
    assert!(!uid.is_nil(), "a created row mints its own identity");

    let created = ctx.binder_order();
    let item = binder_item_controller::get(
        &ctx.db,
        created.last().expect("the row that was just created"),
    )
    .expect("item")
    .expect("item row");
    assert_eq!(item.uid, uid, "and the payload names that very row");
}

/// A row the writer typed into the review step came from no file, and the record
/// must say so rather than inherit the name of whatever else was in the import.
#[test]
fn a_row_that_came_from_no_file_carries_no_file_name() {
    let mut ctx = Ctx::new();
    let origins = origins_for(
        &mut ctx,
        vec![
            create_from("From a file", "Prose.", "chapter.docx", "aaa"),
            create_from("Invented here", "Typed in the wizard.", "", ""),
        ],
    );
    assert_eq!(origins.rows[0].source_file_name, "chapter.docx");
    assert_eq!(origins.rows[1].source_file_name, "");
    assert_eq!(origins.rows[1].source_file_digest, "");
}

/// **The payload has to name the project itself.** Several `Work`s are open at
/// once; the event's own `ids` are the created rows and say nothing about whose
/// binder they joined. A listener holding a list of rows and no project to attach
/// them to has nothing it can honestly record.
#[test]
fn the_payload_names_the_project_the_import_landed_in() {
    let mut ctx = Ctx::new();
    let expected = work_controller::get(&ctx.db, &ctx.work_id)
        .expect("work")
        .expect("work row")
        .unique_id;

    let origins = origins_for(&mut ctx, vec![create_from("One", "Prose.", "a.md", "aaa")]);
    assert!(!expected.is_empty(), "the fixture project has a durable id");
    assert_eq!(origins.work_unique_id, expected);
}

/// **A returning file that touched no prose must not read as text arriving.**
/// Bringing an editor's remarks home without a word of the manuscript changing is
/// the whole point of the returning-file feature, and a record that counted those
/// rows as imported would be describing something that did not happen.
#[test]
fn a_returning_file_that_only_brought_remarks_reports_no_characters() {
    let mut ctx = Ctx::new();
    // A row to bring home to, and the mark that names it.
    let created = ctx.apply_rows(vec![create_from(
        "Chapter one",
        "The writer's own words.",
        "first-draft.md",
        "aaa",
    )]);
    let item = binder_item_controller::get(&ctx.db, created.last().expect("created"))
        .expect("item")
        .expect("item row");
    let tag = skribisto_model::round_trip::uid_tag(&item.uid);

    let origins = origins_for(
        &mut ctx,
        vec![ApplyImportRow::Update {
            target_uid_tag: tag,
            // The case the feature exists for: their notes, not their wording.
            replace_prose: false,
            djot: "The editor's rather different wording.".into(),
            epigraph: String::new(),
            comments: Vec::new(),
            source_file_name: "chapter-1-editor.docx".into(),
            source_file_digest: "eee555".into(),
        }],
    );

    assert_eq!(origins.rows.len(), 1);
    assert_eq!(
        origins.rows[0].action,
        import_management::events::ImportAction::CommentsOnly
    );
    assert_eq!(
        origins.rows[0].char_count, 0,
        "no prose was written, so no characters arrived — whatever the file carried"
    );
    assert_eq!(origins.rows[0].source_file_name, "chapter-1-editor.docx");
    assert_eq!(
        ctx.prose_of(item.id).as_deref(),
        Some("The writer's own words."),
        "…and the manuscript really is untouched, or the assertion above means nothing"
    );
}

/// …and the same file taken *with* its wording is prose arriving, counted.
#[test]
fn a_returning_file_taken_with_its_wording_reports_the_characters_it_wrote() {
    let mut ctx = Ctx::new();
    let created = ctx.apply_rows(vec![create_from(
        "Chapter one",
        "The writer's own words.",
        "first-draft.md",
        "aaa",
    )]);
    let item = binder_item_controller::get(&ctx.db, created.last().expect("created"))
        .expect("item")
        .expect("item row");
    let tag = skribisto_model::round_trip::uid_tag(&item.uid);
    let editors_wording = "The editor's rather different wording.";

    let origins = origins_for(
        &mut ctx,
        vec![ApplyImportRow::Update {
            target_uid_tag: tag,
            replace_prose: true,
            djot: editors_wording.into(),
            epigraph: String::new(),
            comments: Vec::new(),
            source_file_name: "chapter-1-editor.docx".into(),
            source_file_digest: "eee555".into(),
        }],
    );

    assert_eq!(
        origins.rows[0].action,
        import_management::events::ImportAction::ProseReplaced
    );
    assert_eq!(
        origins.rows[0].char_count,
        editors_wording.chars().count() as u64
    );
    assert_eq!(origins.rows[0].item_uid, item.uid.to_string());
}
