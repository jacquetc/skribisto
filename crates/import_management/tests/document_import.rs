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
use common::entities::ChapterMode;
use common::event::EventHub;
use common::long_operation::LongOperationManager;
use common::types::EntityId;
use common::undo_redo::UndoRedoManager;
use direct_access::binder::binder_controller;
use direct_access::binder::dtos::CreateBinderDto;
use direct_access::binder_item::binder_item_controller;
use direct_access::content::content_controller;
use direct_access::root::dtos::CreateRootDto;
use direct_access::root::root_controller;
use direct_access::smart_punctuation::dtos::CreateSmartPunctuationDto;
use direct_access::smart_punctuation::smart_punctuation_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;
use import_management::import_management_controller;
use import_management::{
    AnalyzeDocumentImportDto, ApplyDocumentImportDto, ApplyImportRow, ApplyImportRows,
    DocumentImportRow, DocumentImportRows, ImportDiagnosticRows, ImportRowKind,
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
                smart_punctuation,
                chapter_mode: ChapterMode::Folder,
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

    /// Run ANALYSE to completion and hand back its plan.
    ///
    /// The long-operation manager runs it on its own thread, so this polls for
    /// the result the way the UI does rather than reaching past the framework.
    fn analyse(&mut self, paths: Vec<String>, start: ImportRowKind) -> Vec<DocumentImportRow> {
        let dto = AnalyzeDocumentImportDto {
            work_id: self.work_id,
            binder_id: self.binder_id,
            source_paths: paths,
            start_kind: start,
            base_indent: 0,
        };
        let op = import_management_controller::analyze_document_import(
            &self.db,
            &self.hub,
            &mut self.long_ops,
            &dto,
        )
        .expect("start analyse");

        let plan = loop {
            if let Some(result) = import_management_controller::get_analyze_document_import_result(
                &self.long_ops,
                &op,
            )
            .expect("analyse result")
            {
                break result;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        };

        match plan.rows {
            DocumentImportRows::Found(rows) => rows,
            DocumentImportRows::Empty => Vec::new(),
        }
    }

    fn apply(&mut self, rows: Vec<DocumentImportRow>, anchor: EntityId) -> Vec<EntityId> {
        let create_rows: Vec<ApplyImportRow> = rows
            .into_iter()
            .filter_map(|r| match r {
                DocumentImportRow::Found {
                    indent,
                    kind,
                    title,
                    djot,
                    included,
                    ..
                } if included => Some(ApplyImportRow::Create {
                    indent,
                    kind,
                    title,
                    djot,
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
        start_kind: ImportRowKind::Scene,
        base_indent: 0,
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
            row: ApplyImportRow::Empty,
            rows: ApplyImportRows::Create(vec![ApplyImportRow::Create {
                indent: 0,
                kind: ImportRowKind::Book,
                title: "A Book".into(),
                djot: "Prose a Book cannot hold.".into(),
            }]),
        },
    );
    assert!(result.is_err(), "a Book holding prose must be refused");
    assert!(
        ctx.binder_order().is_empty(),
        "a refused import must leave nothing behind"
    );
}
