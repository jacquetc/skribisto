// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for `analysis_management::analyze_book`.
//!
//! Two properties, and the first is the one the whole feature rests on: **measuring a
//! manuscript never modifies it.**
//!
//! The reasoning is `mention_scan_test`'s, and applies here for the same reason: the Analysis
//! view runs over a whole Book on demand, and `teksilo_ui::app::mutation_origins()` turns any
//! `DirectAccess` entity event into "this project is unsaved". If a measurement ever wrote —
//! even touching `updated_at` — then merely *looking at* the analysis would dirty the
//! project, autosave forever, and leave undo entries the writer cannot explain.
//! `read_only: true`, `undoable: false` and a `QueryUnitOfWork` express that intent, but none
//! is enforced at compile time against a future edit that adds a write action to the uow
//! trait. This file is the enforcement.
//!
//! The second property is scope: the analysis reads what the writer *wrote*, which includes
//! rows they have marked non-printing. That is deliberately not the export scope, and it is
//! the kind of thing that regresses silently the moment someone reaches for the convenient
//! `resolve_scope` helper.

use analysis_management::{AnalyzeBookDto, BookAnalysisResultDto, SceneAnalyses, SceneAnalysis};
use frontend::AppContext;
use frontend::commands::{
    analysis_management_commands, binder_item_commands, content_commands, undo_redo_commands,
    work_commands,
};
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::event::{Event, Origin};
use frontend::common::types::EntityId;
use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
use work_management::{NewWorkDto, NewWorkTemplate};

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

fn item(sub_role: BinderItemSubRole, title: &str, role: BinderItemRole) -> CreateBinderItemDto {
    CreateBinderItemDto {
        uid: common::uid::fixture_uid(
            title.len() as u64 * 1000 + title.chars().map(|c| c as u64).sum::<u64>(),
        ),
        created_at: now(),
        updated_at: now(),
        title: title.to_string(),
        sub_title: String::new(),
        role,
        sub_role,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        exclude_from_numbering: false,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: Vec::new(),
        contents: vec![],
        references: vec![],
        point_of_view: vec![],
        tags: vec![],
    }
}

struct Fixture {
    ctx: AppContext,
    setup: u64,
    work: EntityId,
    book: EntityId,
    scene_a: EntityId,
    scene_b: EntityId,
    /// A scene the writer has marked non-printing. It must still be measured.
    scene_unprinted: EntityId,
}

/// Enough distinct prose that a scene clears the shingle floor and the measures have
/// something real to chew on.
fn prose(tag: &str, n: usize) -> String {
    let words: Vec<String> = (0..n).map(|i| format!("{tag}word{i}")).collect();
    // Several sentences and paragraphs, so sentence/paragraph stats are exercised too.
    words
        .chunks(12)
        .map(|c| format!("{}.", c.join(" ")))
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.join(" "))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn fixture() -> Fixture {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);

    let dir = std::env::temp_dir().join(format!("skrib-analysis-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    frontend::commands::work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");
    let _ = std::fs::remove_dir_all(&dir);

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap().id;
    let binder = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();

    let mut unprinted = item(
        BinderItemSubRole::Scene,
        "Cut for now",
        BinderItemRole::Item,
    );
    unprinted.is_exportable = false;

    let created = binder_item_commands::create_binder_item_multi(
        &ctx,
        Some(setup),
        &[
            item(BinderItemSubRole::Book, "The Book", BinderItemRole::Folder),
            item(
                BinderItemSubRole::Scene,
                "First light",
                BinderItemRole::Item,
            ),
            item(
                BinderItemSubRole::Scene,
                "Second light",
                BinderItemRole::Item,
            ),
            unprinted,
        ],
        binder,
        -1,
    )
    .unwrap();
    let (book, scene_a, scene_b, scene_unprinted) =
        (created[0].id, created[1].id, created[2].id, created[3].id);

    for (id, tag) in [
        (scene_a, "alpha"),
        (scene_b, "beta"),
        (scene_unprinted, "gamma"),
    ] {
        content_commands::create_content_multi(
            &ctx,
            Some(setup),
            &[CreateContentDto {
                uid: Default::default(),
                created_at: now(),
                updated_at: now(),
                activated: true,
                role: ContentRole::SceneText,
                data: prose(tag, 200),
            }],
            id,
            -1,
        )
        .unwrap();
    }

    Fixture {
        ctx,
        setup,
        work,
        book,
        scene_a,
        scene_b,
        scene_unprinted,
    }
}

/// Run an analysis to completion and return its result.
fn analyze(fx: &Fixture) -> BookAnalysisResultDto {
    let id = analysis_management_commands::analyze_book(
        &fx.ctx,
        &AnalyzeBookDto {
            work_id: fx.work,
            scope_item_id: fx.book,
        },
    )
    .expect("start analysis");
    // Release the manager lock before blocking, per the qleany 1.9.0 long-operation guidance.
    let completion = fx
        .ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    let finished = completion.wait_for(&id, Some(std::time::Duration::from_secs(60)));
    assert!(finished, "the analysis did not finish within 60s");

    analysis_management_commands::get_analyze_book_result(&fx.ctx, &id)
        .expect("analysis result")
        .expect("a finished analysis has a result")
}

fn drain(ctx: &AppContext) -> Vec<Event> {
    ctx.event_hub.subscribe_receiver().try_iter().collect()
}

fn measured(dto: &BookAnalysisResultDto) -> Vec<(EntityId, String, i64)> {
    match &dto.scenes {
        SceneAnalyses::Measured(rows) => rows
            .iter()
            .filter_map(|r| match r {
                SceneAnalysis::Measured {
                    item_id,
                    title,
                    words,
                    ..
                } => Some((*item_id, title.clone(), *words)),
                SceneAnalysis::Empty => None,
            })
            .collect(),
        SceneAnalyses::Empty => vec![],
    }
}

/// The guarantee named in `analyze_book_uc`'s header.
///
/// The first assertion matters: without it the test passes vacuously on an analysis that
/// measured nothing, and "wrote nothing" is trivially true of an analysis that did nothing.
#[test]
fn analysis_writes_no_entities() {
    let fx = fixture();
    let before_stack = undo_redo_commands::get_stack_size(&fx.ctx, fx.setup);

    // Discard everything the fixture itself emitted, so the drain below sees only the analysis.
    let _ = drain(&fx.ctx);

    let dto = analyze(&fx);

    assert!(
        dto.total_words > 0,
        "the fixture has prose, so a real measurement happened — if this fails the rest of \
         the test proves nothing"
    );
    assert!(!measured(&dto).is_empty(), "scenes were measured");

    // The actual guarantee. A write of any entity — even one the UI currently ignores —
    // announces itself here.
    let emitted = drain(&fx.ctx);
    let entity_events: Vec<&Event> = emitted
        .iter()
        .filter(|e| matches!(e.origin, Origin::DirectAccess(_)))
        .collect();
    assert!(
        entity_events.is_empty(),
        "measuring a manuscript must not write to it; got {entity_events:?}"
    );
    assert_eq!(
        undo_redo_commands::get_stack_size(&fx.ctx, fx.setup),
        before_stack,
        "an analysis must not push an undo entry"
    );
}

/// Scope is "everything the writer wrote", not "everything that prints".
///
/// `resolve_scope` — the export path — drops `is_exportable == false` rows via `push_swept`.
/// Reaching for it here would silently stop measuring the scenes a writer has parked, which
/// are exactly the ones they are most likely to be checking for duplication against.
#[test]
fn a_non_printing_scene_is_still_measured() {
    let fx = fixture();
    let dto = analyze(&fx);
    let ids: Vec<EntityId> = measured(&dto).into_iter().map(|(id, _, _)| id).collect();

    assert!(ids.contains(&fx.scene_a), "printing scenes are measured");
    assert!(
        ids.contains(&fx.scene_unprinted),
        "a scene marked non-printing is still something the writer wrote; got {ids:?}"
    );
}

/// A Part heading is a structural marker, not a scene.
///
/// Regression, and it produced a visible false finding rather than merely a wrong number.
/// `compile::row_indices` answers the *stream view*'s question — what gets a row in Full
/// Book — which legitimately includes Part and Chapter headings. But per `COMBINATIONS` a
/// Part owns a title and a synopsis and never `SceneText`, so admitting one as a scene
/// produced a phantom zero-word row that dragged the word-count median toward zero and was
/// then reported to the writer as a scene whose prose does not match its synopsis — when it
/// has no prose to match at all.
#[test]
fn a_part_heading_is_not_measured_as_a_scene() {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);

    let dir = std::env::temp_dir().join(format!("skrib-analysis-part-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    frontend::commands::work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");
    let _ = std::fs::remove_dir_all(&dir);

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap().id;
    let binder = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();

    let created = binder_item_commands::create_binder_item_multi(
        &ctx,
        Some(setup),
        &[
            item(BinderItemSubRole::Book, "The Book", BinderItemRole::Folder),
            item(BinderItemSubRole::Part, "Part One", BinderItemRole::Folder),
            item(
                BinderItemSubRole::Scene,
                "A real scene",
                BinderItemRole::Item,
            ),
        ],
        binder,
        -1,
    )
    .unwrap();
    let (book, part, scene) = (created[0].id, created[1].id, created[2].id);

    // The Part gets a synopsis but, by construction, no scene text.
    content_commands::create_content_multi(
        &ctx,
        Some(setup),
        &[CreateContentDto {
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            activated: true,
            role: ContentRole::SynopsisText,
            data: "Carnelian halyard trebuchet.".to_string(),
        }],
        part,
        -1,
    )
    .unwrap();
    content_commands::create_content_multi(
        &ctx,
        Some(setup),
        &[CreateContentDto {
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            activated: true,
            role: ContentRole::SceneText,
            data: prose("delta", 200),
        }],
        scene,
        -1,
    )
    .unwrap();

    let fx = Fixture {
        ctx,
        setup,
        work,
        book,
        scene_a: scene,
        scene_b: scene,
        scene_unprinted: scene,
    };
    let dto = analyze(&fx);
    let ids: Vec<EntityId> = measured(&dto).into_iter().map(|(id, _, _)| id).collect();

    assert!(ids.contains(&scene), "the real scene is measured");
    assert!(
        !ids.contains(&part),
        "a Part heading owns no prose and must not appear as a scene; got {:?}",
        measured(&dto)
    );
    // The Part carries a synopsis and no prose, which is exactly the row that used to
    // slip in as a phantom zero-word scene: it would drag the word-count median down and
    // then be reported as a scene whose prose does not match its synopsis. `counts_prose`
    // is what keeps it out, so the total must be the real scene's alone.
    assert_eq!(
        dto.total_words, 200,
        "the Part's synopsis must contribute nothing to the scope's word total"
    );
}

/// The container the view is mounted on bounds the walk, and the head itself is never a row.
#[test]
fn the_scope_head_is_not_measured_as_a_scene() {
    let fx = fixture();
    let dto = analyze(&fx);
    let ids: Vec<EntityId> = measured(&dto).into_iter().map(|(id, _, _)| id).collect();
    assert!(
        !ids.contains(&fx.book),
        "the Book folder is the pane header, not a scene"
    );
    assert!(ids.contains(&fx.scene_b));
}

/// A scope id that is not in this work's stream is an error, not an empty result — an empty
/// result would read as "this book has nothing in it".
#[test]
fn an_unknown_scope_is_an_error_not_an_empty_analysis() {
    let fx = fixture();
    let id = analysis_management_commands::analyze_book(
        &fx.ctx,
        &AnalyzeBookDto {
            work_id: fx.work,
            scope_item_id: 999_999,
        },
    )
    .expect("start analysis");
    let completion = fx
        .ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    completion.wait_for(&id, Some(std::time::Duration::from_secs(60)));

    let result = analysis_management_commands::get_analyze_book_result(&fx.ctx, &id);
    assert!(
        result.is_err() || matches!(result, Ok(None)),
        "an unknown scope must not yield a confident empty analysis"
    );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// The bundled example
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// The shipped Starforgers example opens, and the analysis has real material to work with.
///
/// Nothing else in the workspace loads this file, so until now it could have been corrupted
/// — by an enrichment pass, a format migration, anything — and the first person to find out
/// would have been a user clicking it on the welcome screen.
///
/// It doubles as the only test running the measurements over real novel-length prose rather
/// than generated filler: 85,000 words across 33 chapters, where the synthetic fixtures above
/// are a few hundred words of `alphaword1 alphaword2`.
#[test]
fn the_bundled_example_opens_and_analyses() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../resources/examples/Starforgers.skrib"
    );

    let ctx = AppContext::new();
    frontend::commands::work_management_commands::load_work(
        &ctx,
        &work_management::LoadWorkDto {
            media_root: String::new(),
            file_name: path.to_string(),
        },
    )
    .expect("the bundled example must open");

    let work = work_commands::get_all_work(&ctx)
        .unwrap()
        .pop()
        .expect("a work")
        .id;

    // The Book container is the analysis scope.
    let binder = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Binders)
        .unwrap()
        .into_iter()
        .next()
        .expect("a manuscript binder");
    let item_ids = frontend::commands::binder_commands::get_binder_relationship(
        &ctx,
        &binder,
        &frontend::common::direct_access::binder::BinderRelationshipField::BinderItems,
    )
    .unwrap();
    let items = binder_item_commands::get_binder_item_multi(&ctx, &item_ids).unwrap();
    let book = items
        .into_iter()
        .flatten()
        .find(|i| i.sub_role == BinderItemSubRole::Book)
        .expect("the example has a Book container")
        .id;

    let id = analysis_management_commands::analyze_book(
        &ctx,
        &AnalyzeBookDto {
            work_id: work,
            scope_item_id: book,
        },
    )
    .expect("start analysis");
    let completion = ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    assert!(
        completion.wait_for(&id, Some(std::time::Duration::from_secs(300))),
        "analysing the bundled example did not finish"
    );
    let dto = analysis_management_commands::get_analyze_book_result(&ctx, &id)
        .expect("analysis result")
        .expect("a finished analysis has a result");

    let rows = measured(&dto);
    assert!(
        rows.len() >= 30,
        "the example has 33 prose chapters; measured {}",
        rows.len()
    );
    assert!(
        dto.total_words > 50_000,
        "a novel-length manuscript should measure tens of thousands of words, got {}",
        dto.total_words
    );

    // The enrichment pass added a synopsis to every chapter. Asserted against the store
    // rather than against the analysis result, because the result no longer carries a
    // synopsis measure — and this was never really a claim about `analyze_book`. It is a
    // claim about the shipped fixture: a synopsis per chapter is what makes the example
    // demonstrate the corkboard, the outline's synopsis column and every synopsis-reading
    // surface, in this application and in anything built on it. A future enrichment pass
    // that dropped them would leave all of those looking empty on the app's flagship file.
    let with_synopsis = item_ids
        .iter()
        .filter(|id| {
            let content_ids = binder_item_commands::get_binder_item_relationship(
                &ctx,
                id,
                &frontend::common::direct_access::binder_item::BinderItemRelationshipField::Contents,
            )
            .unwrap_or_default();
            content_commands::get_content_multi(&ctx, &content_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .any(|c| c.role == ContentRole::SynopsisText && !c.data.trim().is_empty())
        })
        .count();
    assert!(
        with_synopsis >= 30,
        "the example ships a synopsis per chapter; found {with_synopsis}"
    );
}
