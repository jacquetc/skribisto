// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End-to-end backend test for the save loop: write a new-format folder, load
//! it into a real in-memory store via `load_work`, save it back out via
//! `save_work`, and assert the project survives the store round-trip (ids are
//! reassigned by the store, so the comparison is id-free / structural).

use crate::LoadWorkDto;
use crate::SaveWorkDto;
use crate::units_of_work::save_work_uow::SaveWorkUnitOfWorkFactory;
use crate::use_cases::save_work_uc::SaveWorkUseCase;
use crate::work_management_controller;
use crate::{NewWorkDto, NewWorkTemplate};
use chrono::{DateTime, Utc};
use common::database::db_context::DbContext;
use common::entities::{
    Binder, BinderItem, BinderItemRole, BinderItemSubRole, BinderTag, Content, ContentRole,
    DictWord, TrashInfo, Work,
};
use common::event::EventHub;
use common::long_operation::LongOperation;
use skrib_format::{
    self as skrib, BinderWithItems, ItemWithContents, ShapeTag, SkribShape, WorkBundle,
};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

// ── Extra surface for the concurrency / robustness regression tests (F1–F4) ──
use crate::units_of_work::backup_now_uow::BackupNowUnitOfWorkFactory;
use crate::units_of_work::save_as_uow::SaveAsUnitOfWorkFactory;
use crate::use_cases::backup_now_uc::{self, BackupNowUseCase};
use crate::use_cases::save_as_uc::SaveAsUseCase;
use crate::use_cases::save_work_uc::SaveWorkUnitOfWorkFactoryTrait;
use crate::{BackupNowDto, RetentionMode, SaveAsDto};
use common::database::hashmap_store::HashMapStore;
use common::long_operation::{LongOperationManager, OperationProgress, OperationStatus};

// ── Extra surface for the Phase 0 two-Works isolation test ───────────────────
use crate::CloseWorkDto;
use crate::units_of_work::load_work_uow::LoadWorkUnitOfWorkFactory;
use crate::use_cases::load_work_uc::{self, LoadWorkUnitOfWorkFactoryTrait};

fn ts() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).unwrap()
}

fn content(id: u64, role: ContentRole, data: &str) -> Content {
    Content {
        id,
        uid: common::uid::fixture_uid(id),
        created_at: ts(),
        updated_at: ts(),
        activated: true,
        role,
        data: data.to_string(),
    }
}

fn item(
    id: u64,
    title: &str,
    role: BinderItemRole,
    sub_role: BinderItemSubRole,
    contents: Vec<Content>,
) -> ItemWithContents {
    ItemWithContents {
        item: BinderItem {
            uid: common::uid::fixture_uid(3),
            id,
            created_at: ts(),
            updated_at: ts(),
            title: title.to_string(),
            sub_title: String::new(),
            role,
            sub_role,
            label: "note".into(),
            activated: true,
            is_favorite: false,
            is_exportable: true,
            exclude_from_numbering: false,
            indent: 0,
            word_count_goal: 500,
            char_count_goal: 2000,
            dict_language: vec!["en-US".to_string()],
            // Derived from `id` so every item's aliases are distinct: `materialize`
            // rebuilds each item under a fresh id and remaps by `item_map`, so a
            // mis-assignment that handed one item another's aliases would be invisible
            // if every fixture row carried the same vector. Odd ids stay empty to cover
            // the absent case, and one entry is multi-word because that is the whole
            // point of `Vec<String>` over a space-separated field.
            aliases: if id.is_multiple_of(2) {
                vec![format!("Alias{id}"), format!("Miss Bennet {id}")]
            } else {
                Vec::new()
            },
            contents: Vec::new(),
            references: Vec::new(),
            point_of_view: Vec::new(),
            books: Vec::new(),
            tags: Vec::new(),
        },
        contents,
    }
}

/// Two binders covering scene/note/folder/title items with prose + titles.
///
/// `pub(crate)` so the unit tests in [`crate::bundle_contributors`] can
/// fingerprint a manuscript that actually has one, rather than growing a second
/// fixture that would drift away from this one.
pub(crate) fn sample_bundle() -> WorkBundle {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    use ContentRole::*;

    let work = Work {
        // Not the default: the store round trip has to prove it carries the field.
        goal_unit: common::entities::GoalUnit::Characters,
        id: 1,
        created_at: ts(),
        updated_at: ts(),
        title: "The Lighthouse".into(),
        author_name: "Jane".into(),
        dict_language: vec!["en-US".to_string()],
        unique_id: "the-lighthouse-uid".into(),
        // Non-default so the round-trip actually exercises chapter_mode persistence.
        chapter_mode: common::entities::ChapterMode::Flat,
        custom_replacement_rules_enabled: false,
        number_chapters: true,
        part_resets_chapter: false,
        tags: vec![10, 11],
        dict_words: vec![20],
        text_replacement_rules: vec![],
        note_templates: vec![],
        assets: vec![],
        footnotes: vec![],
        // Non-default too, and for the same reason as chapter_mode above: an
        // all-default row would round-trip equal even if the field were dropped.
        smart_punctuation: 30,
        binders: vec![100, 101],
        trash_infos: vec![],
        paces: vec![],
        comments: vec![],
    };
    let tags = vec![
        BinderTag {
            id: 10,
            uid: common::uid::fixture_uid(10),
            created_at: ts(),
            updated_at: ts(),
            name: "Important".into(),
            color: "#f00".into(),
            details: "Needs a second pass".into(),
            discoverable: false,
            // Filed: notes created under this tag land in item 300, the same Book the
            // fixture files an item under below. Not a trivial `None`, so the round
            // trip below actually proves a tag's own filing survives rather than
            // proving two empty values are equal.
            //
            // `Option::None` elsewhere in this file is qualified deliberately:
            // `BinderItemSubRole::None` is in scope under a glob import and shadows
            // the bare name.
            creates_in: Some(300),
            note_template: Option::None,
        },
        BinderTag {
            id: 11,
            uid: common::uid::fixture_uid(11),
            created_at: ts(),
            updated_at: ts(),
            name: "Idea".into(),
            color: "#0f0".into(),
            details: String::new(),
            discoverable: true,
            // `Option::None`, qualified: `BinderItemSubRole::None` is in scope
            // under a glob import here and shadows the bare name.
            creates_in: Option::None,
            note_template: Option::None,
        },
    ];
    let dict_words = vec![DictWord {
        id: 20,
        created_at: ts(),
        updated_at: ts(),
        word: "Skribisto".into(),
    }];

    let mut manuscript_items = vec![
        item(
            300,
            "The Lighthouse",
            Folder,
            Book,
            vec![
                content(400, BookTitle, "The Lighthouse"),
                content(401, BookSubtitle, "A Novel"),
                content(402, SynopsisText, "A keeper and a storm."),
            ],
        ),
        item(
            301,
            "Chapter One",
            Folder,
            ChapterScene,
            vec![
                content(410, ChapterTitle, "Chapter One"),
                content(411, SynopsisText, "Arrival."),
            ],
        ),
        item(
            302,
            "The ferry",
            Item,
            Scene,
            vec![
                content(420, SceneText, "The ferry pitched in the swell."),
                content(421, SynopsisText, "They cross."),
            ],
        ),
        item(
            303,
            "Into the Dark",
            Item,
            ChapterScene,
            vec![
                content(430, ChapterTitle, "Chapter Two"),
                // Carries a hyperlink. Prose is stored as Djot, so a link is a
                // character format on the way in and out and plain `[text](url)`
                // on disk. This is the one place the whole chain is asserted at
                // once, rather than each half separately.
                content(
                    431,
                    SceneText,
                    "The light failed at [midnight](https://example.com/logs).",
                ),
                content(432, SynopsisText, "The storm hits."),
            ],
        ),
    ];
    // A declaration, not a position: item 302 ("The ferry") sits inside this very
    // Book by containment already, but `books` is filed independently of that --
    // here it is filed under item 300, "The Lighthouse", the fixture's own
    // `Folder/Book` row. The store round-trip must carry this declaration through
    // `gather` (`tree_read.rs`) exactly as it already does for `references` and
    // `point_of_view`.
    manuscript_items[2].item.books = vec![300];
    let manuscript = BinderWithItems {
        binder: Binder {
            uid: common::uid::fixture_uid(2),
            id: 100,
            created_at: ts(),
            updated_at: ts(),
            name: "Manuscript".into(),
            activated: true,
            binder_items: Vec::new(),
        },
        items: manuscript_items,
    };
    let characters = BinderWithItems {
        binder: Binder {
            uid: common::uid::fixture_uid(1),
            id: 101,
            created_at: ts(),
            updated_at: ts(),
            name: "Characters".into(),
            activated: true,
            binder_items: Vec::new(),
        },
        items: vec![item(
            320,
            "Mara Vance",
            Item,
            Note,
            vec![
                content(500, NoteText, "The keeper's daughter."),
                content(501, SynopsisText, "Protagonist."),
            ],
        )],
    };

    let trash = vec![TrashInfo {
        id: 200,
        created_at: ts(),
        updated_at: ts(),
        trashed_at: ts(),
        origin_binder_id: 100,
        trashed_binder: Option::None,
        trashed_binder_item: Some(302),
    }];

    skrib::from_entities(
        &work,
        &tags,
        &dict_words,
        &[],
        &[],
        &[],
        Default::default(),
        Some(&common::entities::SmartPunctuation {
            id: 30,
            created_at: ts(),
            updated_at: ts(),
            override_app_default: true,
            dashes: true,
            ellipsis: true,
            quotes: true,
            quote_style: common::entities::QuoteStyle::Guillemets,
            pre_punctuation_spacing: true,
            dialogue_marker: true,
        }),
        &trash,
        &[],
        &[],
        &[],
        // No footnotes in this fixture — the sidecar round-trip has its own
        // coverage in `skrib_format`, and a default-valued row here would pass
        // whether or not the field survived.
        &[],
        &[manuscript, characters],
        ShapeTag::Folder,
    )
}

// Id-free structural projection for comparison across the store round-trip.
#[derive(Debug, PartialEq)]
struct NormItem {
    title: String,
    role: String,
    sub_role: String,
    label: String,
    is_exportable: bool,
    wcg: i64,
    aliases: Vec<String>,
    inline: Vec<(String, String)>,
    prose: Vec<(String, String)>,
}
#[derive(Debug, PartialEq)]
struct NormBinder {
    name: String,
    activated: bool,
    items: Vec<NormItem>,
}
/// A tag as compared across the round-trip. Named rather than a tuple: two adjacent
/// `String` fields in a tuple can be transposed in both the construction and the
/// assertion and still compare equal, silently checking nothing.
/// No text colour — it is derived from `color` at render time, not persisted.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct NormTag {
    name: String,
    color: String,
    details: String,
    discoverable: bool,
}
#[derive(Debug, PartialEq)]
struct Norm {
    title: String,
    author: String,
    lang: Vec<String>,
    unique_id: String,
    /// Compared, not merely set: `sample_bundle` deliberately uses the non-default
    /// `Flat`, but for as long as this field was missing from the projection the
    /// round trip silently reverted it to `Folder` and every test still passed.
    chapter_flat: bool,
    tags: Vec<NormTag>,
    words: Vec<String>,
    binders: Vec<NormBinder>,
    trash: usize,
    refs: usize,
    books: usize,
}

fn norm(b: &WorkBundle) -> Norm {
    let mut tags: Vec<_> = b
        .tags
        .iter()
        .map(|t| NormTag {
            name: t.name.clone(),
            color: t.color.clone(),
            details: t.details.clone(),
            discoverable: t.discoverable,
        })
        .collect();
    tags.sort();
    let mut words: Vec<_> = b.dict_words.iter().map(|w| w.word.clone()).collect();
    words.sort();

    let binders = b
        .binders
        .iter()
        .map(|bb| NormBinder {
            name: bb.binder.name.clone(),
            activated: bb.binder.activated,
            items: bb
                .items
                .iter()
                .map(|bi| {
                    let f = &bi.item;
                    let mut inline: Vec<_> = f
                        .inline_contents
                        .iter()
                        .map(|c| (format!("{:?}", c.role), c.text.clone()))
                        .collect();
                    inline.sort();
                    let mut prose: Vec<_> = f
                        .prose_refs
                        .iter()
                        .map(|p| {
                            (
                                format!("{:?}", p.role),
                                bi.prose.get(&p.file_id).cloned().unwrap_or_default(),
                            )
                        })
                        .collect();
                    prose.sort();
                    NormItem {
                        title: f.title.clone(),
                        role: format!("{:?}", f.role),
                        sub_role: format!("{:?}", f.sub_role),
                        label: f.label.clone(),
                        is_exportable: f.is_exportable,
                        wcg: f.word_count_goal,
                        aliases: f.aliases.clone(),
                        inline,
                        prose,
                    }
                })
                .collect(),
        })
        .collect();

    Norm {
        title: b.manifest.work.title.clone(),
        author: b.manifest.work.author_name.clone(),
        lang: b.manifest.work.dict_language.clone(),
        unique_id: b.manifest.work.unique_id.clone(),
        chapter_flat: b.manifest.work.chapter_flat,
        tags,
        words,
        binders,
        trash: b.trash_infos.len(),
        refs: b
            .binders
            .iter()
            .flat_map(|bb| &bb.items)
            .map(|bi| bi.item.reference_ids.len())
            .sum(),
        books: b
            .binders
            .iter()
            .flat_map(|bb| &bb.items)
            .map(|bi| bi.item.book_ids.len())
            .sum(),
    }
}

#[test]
fn save_load_round_trip_through_store() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("Original");
    let dst = dir.path().join("Resaved");

    // 1. Materialise a new-format folder on disk.
    let original = sample_bundle();
    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &original).unwrap();

    // 2. Load it into a real store.
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work");

    // 3. Save the store back out to a different folder through the real
    //    long-operation path, then read it back.
    let resaved = store_to_bundle(&db, &hub, &dst);

    // 4. The resaved project must be structurally identical (ids/paths aside).
    assert_eq!(norm(&original), norm(&resaved));
    // Not a trivial 0 == 0: the fixture actually files an item under a Book, so this
    // pins that `books` survives `load_work` and `gather` rather than both losing it
    // in a way `norm`'s equality check alone would not distinguish from the field
    // never being read on either side.
    assert_eq!(
        norm(&resaved).books,
        1,
        "the Book filing must survive the round trip"
    );
    // Same reasoning one level up, for a tag's own filing: the fixture points a tag at
    // a real folder, so this fails if `creates_in` is dropped by the bundle writer, the
    // bundle reader, `load_work`'s deferred relationship pass, or `gather`. Each of
    // those four is a place it silently could be, and the equality check above would
    // not tell them apart from the field never being read at all.
    let filed: Vec<_> = resaved
        .tags
        .iter()
        .filter(|t| t.creates_in.is_some())
        .collect();
    assert_eq!(
        filed.len(),
        1,
        "a tag's own destination must survive the round trip"
    );

    // The stable id survives the save → load → save round-trip through the store.
    assert_eq!(resaved.manifest.work.unique_id, "the-lighthouse-uid");
}

/// The end-to-end proof for comments: disk → store → disk, through the real
/// `load_work` materialiser and the real `save_work` gather.
///
/// The store round-trip is where a comment is most likely to be quietly lost,
/// because it is the only place its anchor has to be *re-pointed*: every
/// `EntityId` is re-minted on load, so the content id a comment was saved against
/// is meaningless until `materialize` remaps it. A comment that came back attached
/// to the wrong Content — or to none — would still round-trip "successfully" at the
/// file level, which is exactly why this asserts the anchor, not just the body.
#[test]
fn comments_survive_the_store_round_trip_and_stay_anchored() {
    use common::entities::{CommentAnchorKind, CommentOrphanReason};

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("Original");
    let dst = dir.path().join("Resaved");

    let mut original = sample_bundle();

    // Anchor a comment (with a reply) to the first prose row that exists, and add
    // one anchorless orphan alongside it.
    let (scene_content_id, _) = {
        let item = original.binders[0]
            .items
            .iter_mut()
            .find(|i| !i.item.prose_refs.is_empty())
            .expect("a prose row in the fixture");
        let pr = item.item.prose_refs[0].clone();
        item.comments.insert(
            pr.file_id,
            vec![skrib::CommentFile {
                file_id: 7001,
                uid: common::uid::fixture_uid(7001),
                created_at: "2020-01-01T00:00:00Z".into(),
                updated_at: "2020-01-01T00:00:00Z".into(),
                kind: CommentAnchorKind::Range,
                author_name: "Jane".into(),
                author_initials: "J".into(),
                body: "Does this land?".into(),
                resolved: false,
                orphaned: false,
                orphan_reason: CommentOrphanReason::NotOrphaned,
                range_start: 4,
                range_length: 5,
                quote_prefix: "The ".into(),
                quote_exact: "lamp!".into(),
                quote_exact_truncated: false,
                quote_suffix: " guttered".into(),
                block_ordinal_hint: 2,
                replies: vec![skrib::CommentReplyFile {
                    file_id: 7002,
                    uid: common::uid::fixture_uid(7002),
                    created_at: "2020-01-01T00:00:00Z".into(),
                    updated_at: "2020-01-01T00:00:00Z".into(),
                    author_name: "Marc".into(),
                    author_initials: "M".into(),
                    body: "It does.".into(),
                }],
            }],
        );
        (pr.file_id, ())
    };
    original.orphan_comments.push(skrib::CommentFile {
        file_id: 7010,
        uid: common::uid::fixture_uid(7010),
        created_at: "2020-01-01T00:00:00Z".into(),
        updated_at: "2020-01-01T00:00:00Z".into(),
        kind: CommentAnchorKind::Range,
        author_name: "Jane".into(),
        author_initials: "J".into(),
        body: "Homeless note.".into(),
        resolved: false,
        orphaned: true,
        orphan_reason: CommentOrphanReason::TargetDeleted,
        range_start: 1,
        range_length: 2,
        quote_prefix: "a".into(),
        quote_exact: "bc".into(),
        quote_exact_truncated: false,
        quote_suffix: "d".into(),
        block_ordinal_hint: 9,
        replies: vec![],
    });

    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &original).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work");

    let resaved = store_to_bundle(&db, &hub, &dst);

    // The anchored comment came back on a prose row — and on the SAME one, matched
    // by the prose text rather than by any id (every id was re-minted on load).
    let original_prose = original.binders[0]
        .items
        .iter()
        .find_map(|i| i.prose.get(&scene_content_id).cloned())
        .expect("original prose text");

    let mut found = None;
    for item in &resaved.binders[0].items {
        for (cid, list) in &item.comments {
            if item.prose.get(cid) == Some(&original_prose) {
                found = Some(list.clone());
            }
        }
    }
    let list = found.expect("the comment must come back attached to the same prose row");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].body, "Does this land?");
    assert_eq!(list[0].quote_exact, "lamp!");
    assert_eq!(list[0].range_start, 4);
    assert_eq!(list[0].block_ordinal_hint, 2);
    assert_eq!(
        list[0].replies.len(),
        1,
        "the reply thread must survive the store, not just the file"
    );
    assert_eq!(list[0].replies[0].body, "It does.");

    // And the orphan is still an orphan — kept, not silently discarded.
    assert_eq!(resaved.orphan_comments.len(), 1);
    assert_eq!(resaved.orphan_comments[0].body, "Homeless note.");
    assert!(resaved.orphan_comments[0].orphaned);
}

/// The typed "too new" refusal must survive the whole backend stack.
///
/// `skrib_format` decides it, the use case adds `reading project '…'`, the frontend
/// command adds `load_work`, and the UI leaf recovers it with `downcast_ref` to show an
/// actionable message instead of the generic one. That recovery is the only reason the
/// error is typed at all, and it holds only if every layer between propagates with `?`
/// rather than re-wrapping the message — which no unit test on either end can check.
///
/// It also pins the second half: the refusal happens **before** the bundle is parsed.
/// The binder manifests here are deliberately unparseable, standing in for the real
/// future-format case (an enum variant we do not know), so a regression in the ordering
/// surfaces as a RON error rather than as `TooNew`.
#[test]
fn a_too_new_project_is_refused_with_a_typed_error_all_the_way_up() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("FromTheFuture");
    let path = src.to_str().unwrap().to_string();

    skrib::write_bundle(&path, SkribShape::ExplodedFolder, &sample_bundle()).unwrap();

    // Raise the stamped read floor above what this build supports. A too-new bundle
    // cannot be produced through `write_bundle` — the writer derives the floor from
    // content, so it never emits a file it could not read back — hence the doctoring.
    let manifest_path = src.join("project.skrib");
    let text = std::fs::read_to_string(&manifest_path).unwrap();
    let start = text
        .find("format_min_read_version:")
        .expect("writer stamps a floor");
    let end = start + text[start..].find(',').unwrap();
    std::fs::write(
        &manifest_path,
        format!(
            "{}format_min_read_version: Some({}){}",
            &text[..start],
            skrib::FORMAT_VERSION + 1,
            &text[end..]
        ),
    )
    .unwrap();
    for entry in std::fs::read_dir(src.join("binders")).unwrap() {
        std::fs::write(entry.unwrap().path().join("items.ron"), "}{ not ron").unwrap();
    }

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    let err = work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect_err("a too-new project must be refused");

    match err.downcast_ref::<skrib::SkribFormatError>() {
        Some(skrib::SkribFormatError::TooNew {
            requires_at_least,
            supported,
            ..
        }) => {
            assert_eq!(*requires_at_least, skrib::FORMAT_VERSION + 1);
            assert_eq!(*supported, skrib::FORMAT_VERSION);
        }
        other => panic!("expected TooNew to survive the context layers, got: {other:?} / {err:#}"),
    }

    // And nothing was materialised — a refused open must not half-create a Work. The
    // gate runs in the use case's stage 1, before it opens a transaction at all, so this
    // holds for free; assert it anyway, because "refused" quietly meaning "refused after
    // writing half a project into the store" is exactly the failure nobody would look for.
    assert!(
        db.get_store().works.read().unwrap().is_empty(),
        "a refused open must leave the store untouched"
    );
}

// ── unique_id: migration / persistence / heal ────────────────────────────────

/// The 7 template labels in the documented order (values don't matter here).
fn labels() -> Vec<String> {
    [
        "Manuscript",
        "Notes",
        "Research",
        "Notebook",
        "Chapter",
        "Scene",
        "Note",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

/// Save the current store to `out` and read it straight back — a convenient way
/// to inspect what's in the store after a use case runs (`read_bundle` sniffs the
/// on-disk shape, so this works whether the work is a zip or a folder). The use
/// case is synchronous, so we run `execute` directly rather than through the
/// long-operation manager (which only adds a background thread + progress).
fn store_to_bundle(db: &DbContext, hub: &Arc<EventHub>, out: &std::path::Path) -> WorkBundle {
    let uc = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(db, hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: live_work_id(db),
            file_name: out.to_str().unwrap().to_string(),
            overwrite: true,
        },
    );
    let result = uc
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save_work");
    assert_eq!(result.output_path, out.to_str().unwrap());
    skrib::read_bundle(out.to_str().unwrap()).unwrap()
}

/// A legacy `.skrib` (SQLite) with a `t_project_unique_identifier` must carry
/// that id into the modern store, verbatim.
#[test]
fn legacy_load_preserves_unique_id() {
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../resources/test/skribisto_legacy_v2.skrib"
    );
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: fixture.to_string(),
        },
    )
    .expect("load legacy .skrib");

    let bundle = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(
        bundle.manifest.work.unique_id, "o7R0QHFMHp0p",
        "the legacy project's unique id must be preserved"
    );
}

/// A legacy `.skrib` (SQLite) storing per-item `word_count_goal` / `char_count_goal`
/// rows in `tbl_tree_property` must carry those goals into the modern `BinderItem`
/// fields — before this fix the legacy loader silently dropped them.
#[test]
fn legacy_load_preserves_word_and_char_count_goals() {
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../resources/test/skribisto_legacy_v2.skrib"
    );
    let dir = tempfile::tempdir().unwrap();
    // The fixture is read-only in the repo — copy it so we can inject goal rows.
    let copy = dir.path().join("with_goals.skrib");
    std::fs::copy(fixture, &copy).unwrap();
    {
        let conn = rusqlite::Connection::open(&copy).unwrap();
        // Stamp a goal on every real item (indent > 1); at least one survives as a
        // BinderItem regardless of which rows the migration keeps.
        conn.execute(
            "INSERT INTO tbl_tree_property (l_tree_code, t_name, m_value) \
             SELECT l_tree_id, 'word_count_goal', '1500' FROM tbl_tree WHERE l_indent > 1",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO tbl_tree_property (l_tree_code, t_name, m_value) \
             SELECT l_tree_id, 'char_count_goal', '9000' FROM tbl_tree WHERE l_indent > 1",
            [],
        )
        .unwrap();
    }

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: copy.to_str().unwrap().to_string(),
        },
    )
    .expect("load legacy .skrib with goals");

    let bundle = store_to_bundle(&db, &hub, &dir.path().join("out"));
    let goal_item = bundle
        .binders
        .iter()
        .flat_map(|bb| &bb.items)
        .map(|bi| &bi.item)
        .find(|f| f.word_count_goal == 1500)
        .expect("a migrated item must carry the legacy word_count_goal");
    assert_eq!(
        goal_item.char_count_goal, 9000,
        "the legacy char_count_goal must migrate on the same item"
    );
}

/// The per-project replacement lexicon must survive the full store round-trip —
/// written to a `.skrib`, loaded into the store (ids remapped), and saved back out —
/// with every rule's trigger, replacement and enabled flag intact, and the Work's
/// master switch with it.
///
/// Both halves are set to **non-default** values on purpose. A field that is never
/// materialised out of the loaded bundle reads back as its `Default`, so a fixture
/// left at the default would pass while the field was being silently dropped — which
/// is exactly how `chapter_mode` went unnoticed (`sample_bundle` sets it to `Flat`,
/// but `norm` never compares it, so nothing failed).
#[test]
fn text_replacement_rules_survive_a_save_load_round_trip() {
    const T: &str = "2020-01-01T00:00:00+00:00";
    let mut bundle = sample_bundle();
    bundle.manifest.work.custom_replacement_rules_enabled = true;
    bundle.manifest.work.text_replacement_rule_ids = vec![600, 601];
    bundle.text_replacement_rules = vec![
        skrib::TextReplacementRuleFile {
            file_id: 600,
            created_at: T.into(),
            updated_at: T.into(),
            trigger: "btw".into(),
            replacement: "by the way".into(),
            enabled: true,
        },
        // Disabled on purpose: `enabled` is the one field whose loss would be
        // invisible in the list but would silently start expanding a rule the
        // writer switched off.
        skrib::TextReplacementRuleFile {
            file_id: 601,
            created_at: T.into(),
            updated_at: T.into(),
            trigger: "teh".into(),
            replacement: "the".into(),
            enabled: false,
        },
    ];

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("WithReplacements");
    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load bundle with replacement rules");

    let out = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert!(
        out.manifest.work.custom_replacement_rules_enabled,
        "the per-project master switch must survive the round trip"
    );
    let mut rules: Vec<_> = out
        .text_replacement_rules
        .iter()
        .map(|r| (r.trigger.clone(), r.replacement.clone(), r.enabled))
        .collect();
    rules.sort();
    assert_eq!(
        rules,
        vec![
            ("btw".to_string(), "by the way".to_string(), true),
            ("teh".to_string(), "the".to_string(), false),
        ],
        "every rule must round-trip with its enabled flag"
    );
    assert_eq!(
        out.manifest.work.text_replacement_rule_ids.len(),
        2,
        "the Work must still own both rules after the id remap"
    );
}

/// The punctuation house style must survive the full store round-trip.
///
/// **Every flag is set to its non-default value on purpose.** `SmartPunctuation`
/// derives `Default` with all-`false`/`LocaleDefault`, so a fixture left at its
/// defaults would compare equal to a row that had been dropped entirely at any
/// point in the chain — written, read, materialised, gathered, written again.
/// That is precisely how `chapter_mode` was lost on every load for as long as
/// flat chapters existed, and this test is shaped to make the same mistake
/// impossible here.
#[test]
fn the_punctuation_house_style_survives_a_save_load_round_trip() {
    const T: &str = "2020-01-01T00:00:00+00:00";
    let mut bundle = sample_bundle();
    bundle.manifest.work.smart_punctuation = Some(skrib::SmartPunctuationFile {
        created_at: T.into(),
        updated_at: T.into(),
        override_app_default: true,
        dashes: true,
        ellipsis: true,
        quotes: true,
        quote_style: "guillemets".into(),
        pre_punctuation_spacing: true,
        dialogue_marker: true,
    });

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("WithPunctuation");
    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load bundle with a punctuation house style");

    let out = store_to_bundle(&db, &hub, &dir.path().join("out"));
    let sp = out
        .manifest
        .work
        .smart_punctuation
        .expect("the house style must still be there after the round trip");
    assert!(sp.override_app_default, "the master switch must survive");
    assert!(sp.dashes);
    assert!(sp.ellipsis);
    assert!(sp.quotes);
    assert_eq!(
        sp.quote_style, "guillemets",
        "the house quote style must survive, not fall back to the locale default"
    );
    assert!(sp.pre_punctuation_spacing);
    assert!(sp.dialogue_marker);
}

/// A project saved before the punctuation setting existed must load, and must come
/// back as "never configured" rather than as "the writer switched everything off".
///
/// The distinction is the whole reason the field is an `Option` end to end: a Work
/// whose `override_app_default` is false follows the application preference, which
/// is what someone who has never opened the setting should get.
#[test]
fn a_project_without_a_punctuation_style_loads_and_follows_the_app_default() {
    let mut bundle = sample_bundle();
    bundle.manifest.work.smart_punctuation = None;

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("NoPunctuation");
    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("a bundle with no punctuation style must still load");

    let out = store_to_bundle(&db, &hub, &dir.path().join("out"));
    // A row IS minted on load — the relationship is one-to-one, so it cannot be
    // absent in the store — but it must be an inert one.
    let sp = out
        .manifest
        .work
        .smart_punctuation
        .expect("the loader mints a row, because a one-to-one child cannot be absent");
    assert!(
        !sp.override_app_default,
        "a pre-feature project must follow the app default, not adopt a house style"
    );
    assert!(!sp.dashes && !sp.ellipsis && !sp.quotes);
    assert_eq!(sp.quote_style, "locale_default");
}

/// A per-Book Pace (with a Holiday and a Milestone) must survive the full store
/// round-trip — written to a `.skrib`, loaded into the store (ids remapped), and saved
/// back out — with its dates, weekday mask, children and weak back-links intact.
#[test]
fn paces_survive_a_save_load_round_trip() {
    const T: &str = "2020-01-01T00:00:00+00:00";
    let mut bundle = sample_bundle();
    // Item 300 is the "The Lighthouse" Book folder; 302 is a manuscript item.
    bundle.paces = vec![skrib::PaceFile {
        file_id: 400,
        created_at: T.into(),
        updated_at: T.into(),
        book_item: Some(300),
        start_date: "2020-02-01T00:00:00+00:00".into(),
        end_date: "2020-06-01T00:00:00+00:00".into(),
        weekday_mask: 31, // Mon–Fri
        active: true,
        holidays: vec![skrib::HolidayFile {
            file_id: 410,
            created_at: T.into(),
            updated_at: T.into(),
            label: "Spring break".into(),
            start_date: "2020-03-01T00:00:00+00:00".into(),
            end_date: Some("2020-03-08T00:00:00+00:00".into()),
        }],
        milestones: vec![skrib::MilestoneFile {
            // Not the default, so the round trip proves the field travels. A milestone
            // that also names a target item is an inconsistent pair the loader must not
            // silently "correct": the stored kind is the authority precisely because a
            // weak target reference can vanish.
            kind: common::entities::MilestoneKind::BookCumulative,
            file_id: 420,
            created_at: T.into(),
            updated_at: T.into(),
            label: "Act I done".into(),
            target_item: Some(302),
            target_date: "2020-04-01T00:00:00+00:00".into(),
            target_word_count: Some(20_000),
        }],
    }];

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("WithPace");
    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load bundle with a pace");

    let out = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(
        out.paces.len(),
        1,
        "the pace must round-trip through the store"
    );
    let p = &out.paces[0];
    assert_eq!(p.weekday_mask, 31);
    assert!(p.active);
    assert!(p.start_date.starts_with("2020-02-01"));
    assert!(p.end_date.starts_with("2020-06-01"));
    assert_eq!(p.holidays.len(), 1);
    assert_eq!(p.holidays[0].label, "Spring break");
    assert!(p.holidays[0].end_date.is_some());
    assert_eq!(p.milestones.len(), 1);
    assert_eq!(p.milestones[0].label, "Act I done");
    assert_eq!(p.milestones[0].target_word_count, Some(20_000));
    assert_eq!(
        p.milestones[0].kind,
        common::entities::MilestoneKind::BookCumulative,
        "the stored milestone kind must survive the store, not be re-derived from target_item"
    );
    // Weak back-links survive (ids are reassigned by the store, so just assert they resolve).
    assert!(
        p.book_item.is_some(),
        "book_item must resolve after id remap"
    );
    assert!(
        p.milestones[0].target_item.is_some(),
        "milestone target_item must resolve after id remap"
    );
}

/// ProgressSnapshots hang off WorkInfo — which is torn down and rebuilt fresh on every
/// load and normally never round-trips. They must nevertheless survive a **double** cycle
/// (load A → save → load B → save): a single cycle wouldn't catch a hydration that only
/// works the first time (this is the invariant-break the design deliberately makes).
#[test]
fn progress_snapshots_survive_a_double_round_trip() {
    const T: &str = "2020-01-01T00:00:00+00:00";
    let mut bundle = sample_bundle();
    bundle.progress_snapshots = vec![
        skrib::ProgressSnapshotFile {
            file_id: 500,
            created_at: T.into(),
            updated_at: T.into(),
            day: "2020-05-01T00:00:00+00:00".into(),
            total_word_count: 1200,
            total_char_count: Some(6800),
            book_item_ids: vec![300], // the "The Lighthouse" Book folder
            book_word_counts: vec![1200],
        },
        skrib::ProgressSnapshotFile {
            file_id: 501,
            created_at: T.into(),
            updated_at: T.into(),
            day: "2020-05-02T00:00:00+00:00".into(),
            total_word_count: 1850,
            total_char_count: None,
            book_item_ids: vec![],
            book_word_counts: vec![],
        },
    ];

    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("A");
    skrib::write_bundle(a.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    // Cycle 1: load A → save → bundle B.
    let db1 = DbContext::new().unwrap();
    let hub1 = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db1,
        &hub1,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: a.to_str().unwrap().to_string(),
        },
    )
    .unwrap();
    let b_path = dir.path().join("B");
    let bundle_b = store_to_bundle(&db1, &hub1, &b_path);
    assert_eq!(
        bundle_b.progress_snapshots.len(),
        2,
        "first save must keep both days"
    );

    // Cycle 2: load B (a *fresh* store + WorkInfo) → save → assert still intact.
    let db2 = DbContext::new().unwrap();
    let hub2 = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db2,
        &hub2,
        &LoadWorkDto {
            media_root: String::new(),
            // store_to_bundle wrote B as a folder next to the out path.
            file_name: b_path.to_str().unwrap().to_string(),
        },
    )
    .unwrap();
    let out = store_to_bundle(&db2, &hub2, &dir.path().join("C"));

    assert_eq!(
        out.progress_snapshots.len(),
        2,
        "both days must survive TWO load→save cycles (the WorkInfo-rehydration path)"
    );
    let mut days: Vec<_> = out.progress_snapshots.iter().collect();
    days.sort_by_key(|s| s.day.clone());
    assert!(days[0].day.starts_with("2020-05-01"));
    assert_eq!(days[0].total_word_count, 1200);
    assert_eq!(days[0].total_char_count, Some(6800));
    assert_eq!(days[0].book_word_counts, vec![1200]);
    assert!(
        days[0].book_item_ids.len() == 1,
        "the per-Book id must remap and survive"
    );
    assert!(days[1].day.starts_with("2020-05-02"));
    assert_eq!(days[1].total_word_count, 1850);
    assert_eq!(days[1].total_char_count, None);
}

/// A pre-v2 bundle whose `project.skrib` lacks `unique_id` must still load
/// (serde default) and be *healed* — the store gets a freshly minted id.
#[test]
fn bundle_without_unique_id_is_healed() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("Legacyish");
    skrib::write_bundle(
        src.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &sample_bundle(),
    )
    .unwrap();

    // Simulate an older new-format file: drop the `unique_id` line from the
    // manifest and stamp it as format_version 1.
    let manifest_path = src.join("project.skrib");
    let text = std::fs::read_to_string(&manifest_path).unwrap();
    let stripped: String = text
        .lines()
        .filter(|l| !l.contains("unique_id"))
        .map(|l| {
            if l.contains("format_version") {
                "    format_version: 1,".to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&manifest_path, stripped).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load pre-v2 bundle");

    let bundle = store_to_bundle(&db, &hub, &dir.path().join("out"));
    let id = &bundle.manifest.work.unique_id;
    assert!(!id.is_empty(), "a missing unique_id must be healed on load");
    assert!(
        uuid::Uuid::parse_str(id).is_ok(),
        "the minted id should be a UUID, got {id:?}"
    );
}

// ── NewWork ───────────────────────────────────────────────────────────────────

fn new_work(db: &DbContext, hub: &Arc<EventHub>, path: &str, is_folder: bool, t: NewWorkTemplate) {
    work_management_controller::new_work(
        db,
        hub,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: path.to_string(),
            is_folder,
            template_kind: t,
            labels: labels(),
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");
}

/// The author supplied at creation must reach the `Work` entity and then the
/// saved manifest. Before this was wired, `new_work_uc` built its `Work` with
/// `..Default::default()`, so the name was accepted by the dialog and silently
/// dropped on the way to disk.
#[test]
fn new_work_persists_the_author_to_the_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    work_management_controller::new_work(
        &db,
        &hub,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: dir
                .path()
                .join("Authored.skrib")
                .to_str()
                .unwrap()
                .to_string(),
            is_folder: false,
            template_kind: NewWorkTemplate::Novel,
            labels: labels(),
            language: vec!["en-US".to_string()],
            author_name: "A. Writer".to_string(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(b.manifest.work.author_name, "A. Writer");
}

/// **The same `..Default::default()` trap as the author name above, one field over.**
///
/// `Work` derives `Default`, so `bool::default()` leaves `number_chapters` **false** —
/// and `new_work_uc` builds its `Work` with `..Default::default()`. Without the explicit
/// `number_chapters: true` there, every project this app creates would export with no
/// chapter numbers at all, while the template it ships still writes the literal titles
/// "Chapter 1".."Chapter N" into those same chapters. Nothing in the UI would say so; the
/// writer would find out from the exported file.
///
/// This is exactly the hazard that made `BinderItem.exclude_from_numbering` the *negative*
/// spelling — there, the safe legacy state and `Default` agree, so no such line is needed
/// and no such test could fail.
#[test]
fn new_work_numbers_its_chapters() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    new_work(
        &db,
        &hub,
        dir.path().join("Numbered.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::Novel,
    );

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert!(
        b.manifest.work.number_chapters,
        "a new project must number its chapters"
    );
    assert!(
        !b.manifest.work.part_resets_chapter,
        "and run them continuously across parts"
    );
    // The per-item opt-out starts clear on every row the template creates.
    assert!(
        b.binders
            .iter()
            .flat_map(|bin| bin.items.iter())
            .all(|i| !i.item.exclude_from_numbering),
        "no template row starts life excluded from numbering"
    );
}

/// **`new_work` must mint the punctuation house-style row, not only `load_work`.**
///
/// The existing round-trip tests prove a *loaded* project keeps its row — but the
/// loader mints one to satisfy the one-to-one relationship, so they would pass
/// even if `new_work` created a Work with a dangling `smart_punctuation: 0`. This
/// pins the creation path directly: a freshly made project already has an inert
/// row on disk, following the application default until someone gives it a house
/// style of its own.
#[test]
fn new_work_mints_a_punctuation_house_style() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    new_work(
        &db,
        &hub,
        dir.path().join("Fresh.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::Novel,
    );

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    let sp = b
        .manifest
        .work
        .smart_punctuation
        .expect("new_work must mint a SmartPunctuation row that reaches the manifest");
    assert!(
        !sp.override_app_default,
        "a new project follows the app default rather than adopting a house style"
    );
    assert!(
        !sp.dashes
            && !sp.ellipsis
            && !sp.quotes
            && !sp.pre_punctuation_spacing
            && !sp.dialogue_marker,
        "every rule starts off on a new project"
    );
}

/// An empty author is a legal, common state — it must round-trip as empty
/// rather than failing or acquiring a placeholder.
#[test]
fn new_work_without_an_author_round_trips_empty() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    new_work(
        &db,
        &hub,
        dir.path().join("Anon.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::Novel,
    );

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(b.manifest.work.author_name, "");
}

#[test]
fn new_work_novel_builds_full_tree() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    new_work(
        &db,
        &hub,
        dir.path().join("My Novel.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::Novel,
    );

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));

    // A fresh UUID identity.
    assert!(
        uuid::Uuid::parse_str(&b.manifest.work.unique_id).is_ok(),
        "new work should get a UUID, got {:?}",
        b.manifest.work.unique_id
    );
    // Title derived from the file stem.
    assert_eq!(b.manifest.work.title, "My Novel");
    // The chosen default language is persisted as the work's dict_language.
    assert_eq!(b.manifest.work.dict_language, vec!["en-US".to_string()]);
    // Not saved as a folder.
    assert_eq!(b.manifest.shape, ShapeTag::Zip);

    // Three binders, named from the labels.
    let names: Vec<&str> = b.binders.iter().map(|bb| bb.binder.name.as_str()).collect();
    assert_eq!(names, ["Manuscript", "Notes", "Research"]);

    // Manuscript = Book + 20 chapters (each Folder/Chapter + Item/Scene) + BookEnd.
    let manuscript = &b.binders[0].items;
    assert_eq!(manuscript[0].item.sub_role, BinderItemSubRole::Book);
    let chapters = manuscript
        .iter()
        .filter(|i| i.item.sub_role == BinderItemSubRole::ChapterScene)
        .count();
    let scenes = manuscript
        .iter()
        .filter(|i| i.item.sub_role == BinderItemSubRole::Scene)
        .count();
    assert_eq!(chapters, 20);
    assert_eq!(scenes, 20);
    assert_eq!(
        manuscript.last().unwrap().item.sub_role,
        BinderItemSubRole::BookEnd
    );
    // Notes + Research are empty.
    assert!(b.binders[1].items.is_empty());
    assert!(b.binders[2].items.is_empty());
}

/// Every binder and item a new project mints must carry its OWN uid.
///
/// `new_work_mints_distinct_ids` above checks only the Work's `unique_id`, which left the
/// per-item uids untested — and they are what the UI keys by. `BinderTreeKey` is
/// `Binder(uid)` / `Item(uid)`, and `TreeDataSlice` builds its parent→children map keyed
/// by that. Give forty items one shared uid and they collapse onto a single key whose
/// child list contains its own row, so `flatten_node` — which has no cycle guard —
/// recurses until the stack overflows and the process aborts.
///
/// That is not hypothetical: it is what "create a new project from the Launcher" did,
/// reproduced under gdb with ~32k frames of `flatten_node(idx=2)` calling itself.
///
/// Reading a project back is immune because `heal_uid` replaces a nil uid on load, which
/// is exactly why the checked-in fixture opens fine and only a *freshly created,
/// never-saved* project crashed.
#[test]
fn new_work_gives_every_binder_and_item_its_own_uid() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    // The Novel template, because it is the one that mints many rows — the shape where a
    // shared uid actually produces a cycle rather than merely a duplicate.
    new_work(
        &db,
        &hub,
        dir.path().join("Uids.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::Novel,
    );
    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));

    let mut seen: std::collections::HashMap<uuid::Uuid, usize> = std::collections::HashMap::new();
    let mut nil = Vec::new();
    for bundled in &b.binders {
        let rows = std::iter::once((bundled.binder.name.clone(), bundled.binder.uid)).chain(
            bundled
                .items
                .iter()
                .map(|i| (i.item.title.clone(), i.item.uid)),
        );
        for (what, uid) in rows {
            if uid.is_nil() {
                nil.push(what);
            }
            *seen.entry(uid).or_default() += 1;
        }
    }

    assert!(
        nil.is_empty(),
        "{} row(s) were minted with a nil uid, e.g. {:?}",
        nil.len(),
        &nil[..nil.len().min(5)]
    );
    let worst = seen.values().copied().max().unwrap_or(0);
    assert_eq!(
        worst,
        1,
        "uids must be unique across a new project — {} distinct uid(s) cover {} rows, and one \
         uid is shared by {worst} of them",
        seen.len(),
        seen.values().sum::<usize>(),
    );
}

#[test]
fn new_work_folder_shape_is_honored() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    new_work(
        &db,
        &hub,
        dir.path().join("Notebook").to_str().unwrap(),
        true,
        NewWorkTemplate::NoteBook,
    );

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(
        b.manifest.shape,
        ShapeTag::Folder,
        "is_folder=true → folder shape"
    );
    assert_eq!(b.binders.len(), 1);
    assert_eq!(b.binders[0].binder.name, "Notebook");
    // Notes folder + a starter Note.
    assert_eq!(b.binders[0].items[0].item.sub_role, BinderItemSubRole::None);
    assert_eq!(b.binders[0].items[1].item.sub_role, BinderItemSubRole::Note);
}

#[test]
fn new_work_mints_distinct_ids() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    new_work(
        &db,
        &hub,
        dir.path().join("A.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::None,
    );
    let work_a_id = live_work_id(&db);
    let id1 = store_to_bundle(&db, &hub, &dir.path().join("o1"))
        .manifest
        .work
        .unique_id;

    // Phase 2 of the multi-Work migration: `new_work` no longer closes Work A
    // first — explicitly close it here so `store_to_bundle`'s `live_work_id`
    // (which assumes exactly one resident Work) keeps resolving unambiguously
    // to "the" work, matching this test's own single-work-at-a-time intent
    // (proving distinct ids, not multi-Work coexistence — see
    // `frontend::tests::multi_work_scoping_test` for that).
    work_management_controller::close_work(&db, &hub, &CloseWorkDto { work_id: work_a_id })
        .expect("close_work A");

    new_work(
        &db,
        &hub,
        dir.path().join("B.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::None,
    );
    let id2 = store_to_bundle(&db, &hub, &dir.path().join("o2"))
        .manifest
        .work
        .unique_id;

    assert_ne!(id1, id2, "each new project gets its own id");
}

#[test]
fn new_work_replaces_open_project() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    // Open an existing 2-binder project.
    let src = dir.path().join("Existing");
    skrib::write_bundle(
        src.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &sample_bundle(),
    )
    .unwrap();
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work");
    let existing_work_id = live_work_id(&db);

    // Phase 2 of the multi-Work migration: `new_work` no longer closes the
    // open Work first (see `frontend::tests::multi_work_scoping_test` for the
    // "both stay open" proof) — this test is about a *closed-then-reopened*
    // project starting from a clean tree, so it closes the existing project
    // explicitly first, exactly as a real "Close Work" then "New Work" would.
    work_management_controller::close_work(
        &db,
        &hub,
        &CloseWorkDto {
            work_id: existing_work_id,
        },
    )
    .expect("close_work Existing");

    // Creating a new (None) work over a closed project must start from a
    // clean tree, not inherit the old one's binders.
    new_work(
        &db,
        &hub,
        dir.path().join("Fresh.skrib").to_str().unwrap(),
        false,
        NewWorkTemplate::None,
    );

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(b.manifest.work.title, "Fresh");
    assert_eq!(b.binders.len(), 1, "old binders must be gone");
    assert_eq!(b.binders[0].binder.name, "Manuscript");
    assert!(b.binders[0].items.is_empty());
}

// ── Concurrency / robustness regression tests (save-system review F1–F4) ─────

/// Materialise the sample project (folder shape) on disk and load it into a
/// fresh store; returns the tempdir (keep it alive) + the ctx.
fn load_sample() -> (tempfile::TempDir, DbContext, Arc<EventHub>) {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("Original");
    skrib::write_bundle(
        src.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &sample_bundle(),
    )
    .unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work");
    (dir, db, hub)
}

/// Directly rewrite the open Work's title in the LIVE store — simulates a
/// concurrent UI-thread edit landing while a background long op reads/writes.
fn set_live_title(db: &DbContext, title: &str) {
    let store = db.get_store();
    let mut works = store.works.write().unwrap();
    let (id, mut w) = works
        .iter()
        .next()
        .map(|(k, v)| (*k, v.clone()))
        .expect("a work is open");
    w.title = title.to_string();
    works.insert(id, w);
}

fn live_title(db: &DbContext) -> String {
    db.get_store()
        .works
        .read()
        .unwrap()
        .values()
        .next()
        .unwrap()
        .title
        .clone()
}

/// The id of whichever Work is (first) open in the store — the scoped
/// `SaveWorkDto`/`SaveAsDto`/`BackupNowDto`/`CloseWorkDto.work_id` every real
/// caller now supplies. Every helper/test in this file that calls `load_sample`
/// or `load_work` puts exactly one Work in the store, so "first" is unambiguous
/// here — the two-Works isolation test below resolves ids explicitly instead of
/// through this helper, precisely because it is the one place that assumption
/// stops holding.
fn live_work_id(db: &DbContext) -> u64 {
    *db.get_store()
        .works
        .read()
        .unwrap()
        .keys()
        .next()
        .expect("a work is open")
}

/// F1: a frozen read transaction (what save now begins) sees one consistent
/// point-in-time view — a concurrent write to the live store is invisible to it.
#[test]
fn frozen_read_is_isolated_from_concurrent_writes() {
    let (_dir, db, hub) = load_sample();

    // Begin a frozen read exactly as `save_work`/`save_as`/`backup_now` do.
    let uow = SaveWorkUnitOfWorkFactory::new(&db, &hub).create();
    uow.begin_transaction().unwrap();
    let before = uow.get_all_work().unwrap()[0].title.clone();

    // A concurrent write lands in the LIVE store mid-read.
    set_live_title(&db, "MUTATED");

    // The frozen read still returns the pre-mutation state.
    let after = uow.get_all_work().unwrap()[0].title.clone();
    uow.end_transaction().unwrap();

    assert_eq!(
        before, after,
        "the frozen read must not observe the concurrent write"
    );
    assert_ne!(after, "MUTATED");
    // Sanity: the live store really did change.
    assert_eq!(live_title(&db), "MUTATED");
}

/// F2: a failed Save As never touches the store — the old whole-store-savepoint
/// rollback on the background thread would have reverted a concurrent UI edit.
#[test]
fn failed_save_as_does_not_roll_back_the_store() {
    let (dir, db, hub) = load_sample();

    // A concurrent UI edit lands in the store.
    set_live_title(&db, "EDIT");

    // Aim Save As at a path whose parent is a *file* → the write must fail.
    let not_a_dir = dir.path().join("iamafile");
    std::fs::write(&not_a_dir, b"x").unwrap();
    let target = not_a_dir.join("out.skrib");

    let uc = SaveAsUseCase::new(
        Box::new(SaveAsUnitOfWorkFactory::new(&db, &hub)),
        &SaveAsDto {
            media_root: String::new(),
            work_id: live_work_id(&db),
            file_name: target.to_str().unwrap().to_string(),
            as_folder: false,
        },
    );
    let res = uc.execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)));
    assert!(res.is_err(), "Save As to an unwritable path must fail");

    // The concurrent edit survived the failed Save As.
    assert_eq!(
        live_title(&db),
        "EDIT",
        "a failed Save As must not revert the store"
    );
}

/// A `BackupNowDto` with every retention/skip field at its "do nothing extra"
/// default (no pruning, no skip-if-unchanged) — the shape every pre-existing
/// test wants; tests that exercise T1-3/T1-7/T2-2/T2-11 build their own.
fn plain_backup_dto(
    work_id: u64,
    directories: Vec<String>,
    last_known_hashes: Vec<String>,
) -> BackupNowDto {
    BackupNowDto {
        media_root: String::new(),
        work_id,
        directories,
        last_known_hashes,
        last_known_paths: vec![],
        prune: false,
        retention_mode: RetentionMode::Tiered,
        keep_last_n: 0,
        gfs_hourly: 0,
        gfs_daily: 0,
        gfs_weekly: 0,
        gfs_monthly: 0,
        min_keep: 0,
        pinned_paths: vec![],
    }
}

/// F3: Backup serialises the current in-memory store (the source of truth), not
/// the possibly-stale on-disk artifact — an unsaved edit must appear in the
/// backup — and it is always a zip regardless of the project's shape.
#[test]
fn backup_serializes_current_store_not_disk() {
    let (dir, db, hub) = load_sample();

    // An unsaved in-memory edit (never written back to the on-disk folder).
    set_live_title(&db, "UNSAVED EDIT");

    let backup_dir = dir.path().join("backups");
    std::fs::create_dir_all(&backup_dir).unwrap();
    let uc = BackupNowUseCase::new(
        Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)),
        &plain_backup_dto(
            live_work_id(&db),
            vec![backup_dir.to_str().unwrap().to_string()],
            vec![],
        ),
    );
    let res = uc
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");

    assert_eq!(res.succeeded_paths.len(), 1, "one destination written");
    assert!(res.failed_directories.is_empty());
    let bundle = skrib::read_bundle(&res.succeeded_paths[0]).unwrap();
    assert_eq!(
        bundle.manifest.work.title, "UNSAVED EDIT",
        "the backup must reflect the store, not the stale on-disk file"
    );
    assert_eq!(
        bundle.manifest.shape,
        ShapeTag::Zip,
        "backups are always a single zip, even for a folder-shape project"
    );
    // The backup carries the authoritative marker so it is recognised on open.
    assert_eq!(bundle.manifest.kind, skrib::BundleKind::Backup);
    assert!(bundle.manifest.backup_created_at.is_some());
    assert!(bundle.manifest.backup_of.is_some());
}

/// A second destination that cannot be written must not sink the whole backup:
/// the good destination still gets a file, and the bad one is reported. And a
/// matching last-known hash makes a destination skip.
#[test]
fn backup_multi_destination_is_resilient_and_dedups() {
    let (dir, db, hub) = load_sample();

    let good = dir.path().join("good");
    std::fs::create_dir_all(&good).unwrap();
    // A path whose parent is a *file* — creating the zip there must fail.
    let blocker = dir.path().join("blocker");
    std::fs::write(&blocker, b"x").unwrap();
    let bad = blocker.join("nested"); // parent is a file ⇒ unwritable

    let run = |hashes: Vec<String>, paths: Vec<String>| {
        let mut dto = plain_backup_dto(
            live_work_id(&db),
            vec![
                good.to_str().unwrap().to_string(),
                bad.to_str().unwrap().to_string(),
            ],
            hashes,
        );
        dto.last_known_paths = paths;
        BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto)
            .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
            .expect("backup_now")
    };

    let res = run(vec![], vec![]);
    assert_eq!(
        res.succeeded_paths.len(),
        1,
        "the good destination is written"
    );
    assert_eq!(
        res.failed_directories.len(),
        1,
        "the bad destination is reported"
    );
    assert_eq!(res.failed_reasons.len(), 1);
    assert!(!res.content_hash.is_empty());

    // Feeding back the good destination's hash AND its still-existing path
    // makes it skip on the next run.
    let res2 = run(
        vec![res.content_hash.clone(), String::new()],
        vec![res.succeeded_paths[0].clone(), String::new()],
    );
    assert_eq!(
        res2.skipped_directories.len(),
        1,
        "unchanged good destination is skipped"
    );
    assert!(
        res2.succeeded_paths.is_empty(),
        "nothing written to the good destination"
    );
    assert_eq!(
        res2.failed_directories.len(),
        1,
        "the bad destination still fails"
    );
}

// ── backup: T1-3 (skip-if-gone) / T1-7 (retention joins the op) / T2-2
//    (verify-after-write) / T2-11 (atomic path reservation) regression tests ──

/// Write a fake single-file backup directly (bypassing `BackupNowUseCase`) so a
/// destination can be pre-seeded with backups carrying an arbitrary manifest
/// timestamp — including one in the FUTURE, simulating what a backwards system
/// clock produces for a genuinely older backup.
fn write_fake_backup(dir: &std::path::Path, name: &str, when: DateTime<Utc>) -> String {
    let mut bundle = sample_bundle();
    skrib::mark_as_backup(&mut bundle, "irrelevant-source".to_string(), when);
    let path = dir.join(name);
    skrib::write_bundle(path.to_str().unwrap(), SkribShape::ZipFile, &bundle).unwrap();
    path.to_str().unwrap().to_string()
}

/// Same as [`write_fake_backup`] but as a folder-shape bundle (a directory
/// containing `project.skrib`) — used to engineer a deletion that fails.
fn write_fake_folder_backup(
    dir: &std::path::Path,
    name: &str,
    when: DateTime<Utc>,
) -> std::path::PathBuf {
    let mut bundle = sample_bundle();
    skrib::mark_as_backup(&mut bundle, "irrelevant-source".to_string(), when);
    let path = dir.join(name);
    skrib::write_bundle(path.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    path
}

/// T1-3: skip-if-unchanged must not skip when the previously-written backup
/// file is gone (deleted folder, reformatted stick) — the app must not believe
/// a destination is "current" when it actually holds zero backups.
#[test]
fn skip_if_unchanged_does_not_skip_when_the_backup_file_is_gone() {
    let (dir, db, hub) = load_sample();
    let dest = dir.path().join("dest");
    std::fs::create_dir_all(&dest).unwrap();

    let dto1 = plain_backup_dto(
        live_work_id(&db),
        vec![dest.to_str().unwrap().to_string()],
        vec![],
    );
    let res1 = BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto1)
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");
    assert_eq!(res1.succeeded_paths.len(), 1, "first run writes the backup");
    let written = res1.succeeded_paths[0].clone();

    // The backup is gone out from under the app (drive wiped, folder removed).
    std::fs::remove_file(&written).unwrap();
    assert!(!std::path::Path::new(&written).exists());

    // A matching hash AND the now-deleted path must NOT be enough to skip.
    let mut dto2 = plain_backup_dto(
        live_work_id(&db),
        vec![dest.to_str().unwrap().to_string()],
        vec![res1.content_hash.clone()],
    );
    dto2.last_known_paths = vec![written.clone()];
    let res2 = BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto2)
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");

    assert!(
        res2.skipped_directories.is_empty(),
        "must not skip when the recorded backup file no longer exists on disk"
    );
    assert_eq!(
        res2.succeeded_paths.len(),
        1,
        "the destination must be rewritten"
    );
    assert!(std::path::Path::new(&res2.succeeded_paths[0]).exists());
}

/// A pinned backup survives a sweep that would otherwise delete it.
///
/// This is the data-loss-shaped hole in a feature about not losing things: GFS
/// is a policy about *how much past to keep in general*, and it has no way to
/// know that one of those files is the draft that went to an editor. The pin
/// rides the same `protected` guarantee the just-written backup does — by
/// identity, so a backwards clock cannot defeat it either.
#[test]
fn a_pinned_backup_survives_a_sweep_that_would_otherwise_delete_it() {
    let (dir, db, hub) = load_sample();
    let dest = dir.path().join("dest");
    std::fs::create_dir_all(&dest).unwrap();

    let now = Utc::now();
    let kept = write_fake_backup(
        &dest,
        "the-one-i-sent.skrib",
        now - chrono::Duration::days(40),
    );
    let ordinary = write_fake_backup(&dest, "ordinary.skrib", now - chrono::Duration::days(10));

    let sweep = |pinned: Vec<String>| {
        let mut dto = plain_backup_dto(
            live_work_id(&db),
            vec![dest.to_str().unwrap().to_string()],
            vec![],
        );
        dto.prune = true;
        dto.retention_mode = RetentionMode::KeepLastN;
        dto.keep_last_n = 1;
        dto.min_keep = 0;
        dto.pinned_paths = pinned;
        BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto)
            .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
            .expect("backup_now")
    };

    // Precondition: with no pin, the oldest is exactly what this policy deletes.
    let unpinned = sweep(vec![]);
    assert!(
        unpinned.deleted_paths.contains(&kept),
        "precondition: without a pin this backup is the first to go; deleted={:?}",
        unpinned.deleted_paths,
    );
    assert!(!std::path::Path::new(&kept).exists());

    // Put it back and sweep again, this time pinned.
    let kept = write_fake_backup(
        &dest,
        "the-one-i-sent.skrib",
        now - chrono::Duration::days(40),
    );
    let pinned = sweep(vec![kept.clone()]);
    assert!(
        !pinned.deleted_paths.contains(&kept),
        "a pinned backup must never be swept; deleted={:?}",
        pinned.deleted_paths,
    );
    assert!(
        std::path::Path::new(&kept).exists(),
        "the pinned backup must still be on disk",
    );
    // …and pinning one file must not turn retention off for everything else.
    let _ = ordinary;
    assert!(
        !pinned.deleted_paths.is_empty(),
        "retention must still sweep the versions that were not pinned",
    );
}

/// T1-7: retention now runs INSIDE the operation and joins the write — and the
/// backup this run just wrote survives an aggressive `KeepLastN { n: 1 }` even
/// when a sibling backup's manifest carries a simulated FUTURE timestamp
/// (exactly what a backwards system clock produces for an older backup): that
/// sibling sorts as "newest" so neither the newest-first rule nor `KeepLastN`
/// would save the one just written — only `protected` does. Exercised across
/// two destinations, to confirm `protected` is wired per-destination.
#[test]
fn retention_runs_inside_the_operation_and_protects_the_just_written_backup() {
    let (dir, db, hub) = load_sample();

    let dest_a = dir.path().join("dest_a");
    let dest_b = dir.path().join("dest_b");
    std::fs::create_dir_all(&dest_a).unwrap();
    std::fs::create_dir_all(&dest_b).unwrap();

    let now = Utc::now();
    let mut old_paths = Vec::new();
    let mut future_paths = Vec::new();
    for dest in [&dest_a, &dest_b] {
        old_paths.push(write_fake_backup(
            dest,
            "old.skrib",
            now - chrono::Duration::days(10),
        ));
        future_paths.push(write_fake_backup(
            dest,
            "future.skrib",
            now + chrono::Duration::days(30),
        ));
    }

    let mut dto = plain_backup_dto(
        live_work_id(&db),
        vec![
            dest_a.to_str().unwrap().to_string(),
            dest_b.to_str().unwrap().to_string(),
        ],
        vec![],
    );
    dto.prune = true;
    dto.retention_mode = RetentionMode::KeepLastN;
    dto.keep_last_n = 1;
    dto.min_keep = 0;

    let res = BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto)
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");

    assert_eq!(res.succeeded_paths.len(), 2, "both destinations written");
    assert!(res.delete_errors.is_empty());

    // The backup just written to each destination survives, whatever the
    // (simulated clock-skewed) sibling timestamps say.
    for p in &res.succeeded_paths {
        assert!(
            !res.deleted_paths.contains(p),
            "the backup just written must never be pruned: {p}"
        );
        assert!(
            std::path::Path::new(p).exists(),
            "the protected backup must still be on disk: {p}"
        );
    }
    // The genuinely stale backup in each destination is pruned (T1-7 also
    // populates `deleted_paths`, not just protects).
    for p in &old_paths {
        assert!(
            res.deleted_paths.contains(p),
            "a genuinely stale backup must be pruned: {p}, deleted={:?}",
            res.deleted_paths
        );
        assert!(!std::path::Path::new(p).exists());
    }
    // The simulated future-dated backup is also kept — it is the "newest" by
    // manifest timestamp, so the unconditional "always keep the newest" rule
    // covers it regardless of `protected`.
    for p in &future_paths {
        assert!(std::path::Path::new(p).exists());
    }
}

/// T2-2: a destination whose file writes but fails verification must be
/// reported as FAILED, not succeeded — and the invalid file must not be left
/// on disk. A genuine write-then-verify failure can't be triggered by
/// corrupting the filesystem out from under a write that just succeeded on
/// it, so this uses the `#[cfg(test)]` fault-injection seam in
/// `backup_now_uc` (production code always calls the real
/// `skrib::verify_backup_at`; the seam only overrides an otherwise-successful
/// outcome, and compiles to nothing outside tests).
#[test]
fn destination_failing_verification_is_reported_failed_not_succeeded() {
    let (dir, db, hub) = load_sample();
    let dest = dir.path().join("verify_fail_dest");
    std::fs::create_dir_all(&dest).unwrap();

    backup_now_uc::force_verify_failure_for(&dest);

    let dto = plain_backup_dto(
        live_work_id(&db),
        vec![dest.to_str().unwrap().to_string()],
        vec![],
    );
    let res = BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto)
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");

    assert!(
        res.succeeded_paths.is_empty(),
        "a destination that fails verification must not be reported as succeeded"
    );
    assert_eq!(res.failed_directories.len(), 1);
    assert_eq!(res.failed_reasons.len(), 1);
    assert!(
        res.failed_reasons[0].contains("forced verification failure"),
        "unexpected failure reason: {}",
        res.failed_reasons[0]
    );
    // A failed-verification write must not leave a (corrupt/unverified) file.
    let leftovers: Vec<_> = std::fs::read_dir(&dest).unwrap().collect();
    assert!(
        leftovers.is_empty(),
        "the unverified file must be cleaned up, found: {leftovers:?}"
    );
}

/// `deleted_paths` / `delete_errors`: a deletion that genuinely fails (here, a
/// permission-denied unlink) is surfaced in `delete_errors`, not silently
/// dropped like the old `let _ = apply_retention(..)` on the UI thread used to.
#[cfg(unix)]
#[test]
fn a_failed_deletion_is_reported_in_delete_errors() {
    use std::os::unix::fs::PermissionsExt;

    let (dir, db, hub) = load_sample();
    let dest = dir.path().join("delete_error_dest");
    std::fs::create_dir_all(&dest).unwrap();

    let stale = write_fake_folder_backup(
        &dest,
        "stale_folder_backup",
        Utc::now() - chrono::Duration::days(30),
    );
    // Deny write on the stale bundle's own directory: deleting its
    // `project.skrib` requires write permission on the directory containing
    // it, so this makes `remove_dir_all` fail with a permission error — while
    // `dest` itself stays writable, so the new backup still gets written.
    let mut perms = std::fs::metadata(&stale).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&stale, perms).unwrap();

    let mut dto = plain_backup_dto(
        live_work_id(&db),
        vec![dest.to_str().unwrap().to_string()],
        vec![],
    );
    dto.prune = true;
    dto.retention_mode = RetentionMode::KeepLastN;
    dto.keep_last_n = 1;
    dto.min_keep = 0;

    let res = BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto)
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");

    // Restore permissions immediately so the tempdir can clean itself up.
    let mut perms = std::fs::metadata(&stale).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&stale, perms).unwrap();

    assert_eq!(
        res.succeeded_paths.len(),
        1,
        "the new backup is still written"
    );
    assert!(
        !res.delete_errors.is_empty(),
        "a permission-denied delete must be surfaced, not silently dropped"
    );
    assert!(
        res.delete_errors
            .iter()
            .any(|e| e.contains("stale_folder_backup")),
        "delete_errors should name the path that failed: {:?}",
        res.delete_errors
    );
}

/// F4: a panicking long operation is reported `Failed`, not left stuck at
/// `Running` with its background thread silently dead.
#[test]
fn panicking_long_operation_is_reported_failed_not_stuck() {
    struct Panicky;
    impl LongOperation for Panicky {
        type Output = ();
        fn execute(
            &self,
            _p: Box<dyn Fn(OperationProgress) + Send>,
            _c: Arc<AtomicBool>,
        ) -> anyhow::Result<()> {
            panic!("boom");
        }
    }

    let mgr = LongOperationManager::new();
    let id = mgr.start_operation(Panicky);
    // Wait for the background thread to settle via the completion signal instead of
    // polling on a sleep loop — a panicking operation still publishes its id last,
    // after settling to `Failed` (see the qleany 1.9.0 migration guide's long-operation
    // section: "A panicking operation is now observable").
    let completion = mgr.completion_signal();
    let finished = completion.wait_for(&id, Some(std::time::Duration::from_secs(3)));
    assert!(
        finished,
        "the panicking operation should settle within the timeout"
    );
    assert!(
        matches!(
            mgr.get_operation_status(&id),
            Some(OperationStatus::Failed(_))
        ),
        "a panicking long op must be reported Failed, got {:?}",
        mgr.get_operation_status(&id)
    );
}

/// F4: the store's restore path recovers a poisoned lock instead of aborting —
/// so a panic under one table's write lock can't turn the Drop-time rollback
/// into a process-wide double-panic.
#[test]
fn restore_savepoint_recovers_a_poisoned_table_lock() {
    let store = HashMapStore::new();
    let sp = store.create_savepoint(); // clean snapshot, before poisoning

    // Poison the `works` write lock by panicking while holding it.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _g = store.works.write().unwrap();
        panic!("poison the lock");
    }));
    assert!(
        store.works.is_poisoned(),
        "precondition: the works lock is poisoned"
    );

    // `restore_savepoint` writes every table via `write_or_recover`; a plain
    // `.write().unwrap()` here would double-panic and abort the process.
    store.restore_savepoint(sp);

    // Recovery is permanent, not one-shot: write_or_recover cleared the poison,
    // so the lock is usable again and the (untouched, plain-`.unwrap()`) CRUD
    // paths won't re-panic on the next access.
    assert!(
        !store.works.is_poisoned(),
        "restore must clear the poison, not just recover once"
    );
    assert!(store.works.read().is_ok(), "the lock must be usable again");
    // A fresh savepoint cycle (snapshot() uses read_or_recover, create/restore
    // use write_or_recover) must also succeed on the recovered store.
    let sp2 = store.create_savepoint();
    store.restore_savepoint(sp2);
}

// ── Phase 0 acceptance test: two Works in one store, mutate/save/close ONE ──

/// Seed a SECOND Work into a store that already has one open, bypassing
/// `LoadWorkUseCase::execute`'s auto-close. `execute` deliberately still closes
/// whatever is open before materialising the new Work (see
/// `new_work_replaces_open_project` above, which pins that behaviour) — Phase 0
/// keeps the UI-visible "opening a project replaces the open one" contract
/// unchanged. The only way to get two Works to genuinely coexist in one store
/// for this test is therefore to drive the same two steps `execute` itself runs
/// — `materialize` (builds the entities) then `create_trunk` (builds the
/// non-undoable System/RecentWork/WorkInfo/Root frame, appending to
/// `Root.works` — the Phase 0 fix under test) — directly, inside their own
/// write transaction, with no `close_current_work` call in between. Returns the
/// new Work's id.
fn load_additional_work(db: &DbContext, hub: &Arc<EventHub>, path: &std::path::Path) -> u64 {
    let bundle = skrib::read_bundle(path.to_str().unwrap()).expect("read fixture bundle");
    let loaded = skrib::bundle_to_loaded(bundle, path.to_str().unwrap()).expect("bundle_to_loaded");
    let factory = LoadWorkUnitOfWorkFactory::new(db, hub);
    let mut uow = factory.create();
    uow.begin_transaction().expect("begin_transaction");
    let mat = load_work_uc::materialize(&*uow, &loaded).expect("materialize");
    load_work_uc::create_trunk(
        &*uow,
        &loaded,
        &mat,
        path.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        Utc::now(),
    )
    .expect("create_trunk");
    uow.commit().expect("commit");
    mat.work_id
}

/// A content fingerprint of exactly ONE Work's subtree, taken through the real,
/// now `work_id`-scoped save path (`SaveWorkUseCase` → the fixed `gather`) —
/// this is what makes "byte-identical throughout" a literal, not approximate,
/// assertion: `content_fingerprint` hashes the serialized, timestamp-stripped
/// bundle (the same helper `backup_now`'s own skip-if-unchanged logic uses).
fn fingerprint_of(
    db: &DbContext,
    hub: &Arc<EventHub>,
    work_id: u64,
    scratch: &std::path::Path,
) -> String {
    let uc = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(db, hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id,
            file_name: scratch.to_str().unwrap().to_string(),
            overwrite: true,
        },
    );
    let result = uc
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("scoped save_work must resolve exactly the requested Work");
    let bundle = skrib::read_bundle(&result.output_path).unwrap();
    skrib::content_fingerprint(&bundle)
}

/// The literal Phase 0 acceptance test: "open two Works in one store; mutate,
/// save and close ONE; assert the OTHER is byte-identical throughout."
///
/// Exercises, in one place, every Phase 0 backend fix: `Root.works` appends
/// rather than replaces (`load_additional_work` would clobber Work A otherwise);
/// `tree_read::gather` resolves the Work the DTO actually names, not whichever
/// the store returns first (`fingerprint_of` would silently target the wrong
/// Work otherwise, especially since B is *added after* A — the opposite of
/// insertion order a `.next()`-based bug would default to); `WorkCloser`/
/// `close_current_work` is scoped to one Work's subtree (closing A must not
/// touch B's rows); `CloseWorkUseCase` takes a real `work_id` and publishes it.
#[test]
fn a_second_work_never_perturbs_the_first_through_mutate_save_close() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    // Work A, via the real public `load_work` path.
    let mut bundle_a = sample_bundle();
    bundle_a.manifest.work.title = "First Project".into();
    bundle_a.manifest.work.unique_id = "first-project-uid".into();
    let path_a = dir.path().join("A");
    skrib::write_bundle(
        path_a.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &bundle_a,
    )
    .unwrap();
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path_a.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work A");
    let work_a_id = live_work_id(&db);
    let row_a_before = db
        .get_store()
        .works
        .read()
        .unwrap()
        .get(&work_a_id)
        .cloned()
        .expect("A resident");
    let fp_a_checkpoint0 = fingerprint_of(&db, &hub, work_a_id, &dir.path().join("fp0"));

    // Work B, seeded onto the SAME store, with ids intentionally allocated
    // AFTER A's — `all_work().next()` (the pre-Phase-0 bug) would still (by
    // accident of insertion order) return A here; B is the one that actually
    // proves `gather` resolves by requested id, not by iteration order.
    let mut bundle_b = sample_bundle();
    bundle_b.manifest.work.title = "Second Project".into();
    bundle_b.manifest.work.unique_id = "second-project-uid".into();
    let path_b = dir.path().join("B");
    skrib::write_bundle(
        path_b.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &bundle_b,
    )
    .unwrap();
    let work_b_id = load_additional_work(&db, &hub, &path_b);
    assert_ne!(work_a_id, work_b_id);

    // Root.works must now list BOTH — the append fix, not an overwrite.
    {
        let store = db.get_store();
        let root_id = *store.roots.read().unwrap().keys().next().expect("a root");
        let works_of_root = store
            .jn_work_from_root_works
            .read()
            .unwrap()
            .get(&root_id)
            .cloned()
            .unwrap_or_default();
        assert_eq!(
            works_of_root.len(),
            2,
            "Root.works must contain both A and B, not just the most recent"
        );
        assert!(works_of_root.contains(&work_a_id));
        assert!(works_of_root.contains(&work_b_id));
    }
    assert_eq!(db.get_store().works.read().unwrap().len(), 2);

    // Checkpoint 1: loading B must not perturb A at all.
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before.clone()),
        "A's row must be byte-for-byte the same object after B is loaded"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("fp1")),
        fp_a_checkpoint0,
        "A unaffected by B's load"
    );
    let fp_b_checkpoint0 = fingerprint_of(&db, &hub, work_b_id, &dir.path().join("fpb0"));

    // Mutate B directly in the live store (simulating a concurrent live edit) —
    // A must not move.
    {
        let store = db.get_store();
        let mut works = store.works.write().unwrap();
        let mut b = works.get(&work_b_id).unwrap().clone();
        b.title = "Mutated B".into();
        works.insert(work_b_id, b);
    }
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before.clone()),
        "A unaffected by B's in-store mutation"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("fp2")),
        fp_a_checkpoint0,
        "A's fingerprint unaffected by B's mutation"
    );

    // Save B (scoped by work_id) — A must not move.
    let save_b = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: work_b_id,
            file_name: dir.path().join("out_b").to_str().unwrap().to_string(),
            overwrite: true,
        },
    );
    let save_b_result = save_b
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save_work B");
    let saved_b_bundle = skrib::read_bundle(&save_b_result.output_path).unwrap();
    assert_eq!(
        saved_b_bundle.manifest.work.title, "Mutated B",
        "save_work(work_id=B) must save B's content, not A's or an arbitrary Work's"
    );
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before.clone()),
        "A unaffected by B's save"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("fp3")),
        fp_a_checkpoint0,
        "A's fingerprint unaffected by B's save"
    );

    // Close B (scoped by work_id) — A must survive intact, and B's rows must be
    // fully gone (not just its Work row: WorkInfo/Search/Binders/... too).
    work_management_controller::close_work(&db, &hub, &CloseWorkDto { work_id: work_b_id })
        .expect("close_work B");

    assert!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_b_id)
            .is_none(),
        "B's Work row must be fully removed"
    );
    assert_eq!(
        db.get_store().works.read().unwrap().len(),
        1,
        "only A remains resident"
    );
    assert!(
        db.get_store()
            .work_infos
            .read()
            .unwrap()
            .values()
            .all(|wi| wi.work != Some(work_b_id)),
        "B's WorkInfo must be fully removed too (the weak, one-way referrer WorkCloser \
         must find and remove explicitly)"
    );
    {
        let store = db.get_store();
        let root_id = *store.roots.read().unwrap().keys().next().expect("a root");
        let works_of_root = store
            .jn_work_from_root_works
            .read()
            .unwrap()
            .get(&root_id)
            .cloned()
            .unwrap_or_default();
        assert_eq!(
            works_of_root,
            vec![work_a_id],
            "Root.works must list only A once B is closed"
        );
    }
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before),
        "A survives B's whole lifecycle (load, mutate, save, close), byte-identical"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("fp4")),
        fp_a_checkpoint0,
        "A byte-identical throughout the whole two-Works scenario"
    );

    // Sanity on B's own numbers too (not just "A is fine"): B's pre-close
    // fingerprint should differ from its checkpoint-0 one, since it was
    // mutated — proving the fingerprint comparisons above are actually
    // sensitive to content, not vacuously equal.
    assert_ne!(
        skrib::content_fingerprint(&saved_b_bundle),
        fp_b_checkpoint0,
        "B's own content did change (the mutation) — the harness is sensitive"
    );

    // A must still be fully open/save-able afterward (no dangling relationship
    // left behind by B's close) — a full content check, not just a fingerprint.
    let final_a = store_to_bundle(&db, &hub, &dir.path().join("final_a"));
    assert_eq!(final_a.manifest.work.title, "First Project");
    assert_eq!(final_a.manifest.work.unique_id, "first-project-uid");
    assert_eq!(norm(&final_a), norm(&bundle_a));
}

/// **Phase 2 acceptance test.** The test above still seeds Work B through
/// `load_additional_work` (a direct `materialize`/`create_trunk` call), because
/// at Phase 0 the real, public `load_work` controller still ran the "close
/// every other open Work" sweep — the only way to get two Works open at once
/// was to bypass it. Phase 2 deletes that sweep (`load_work_uc.rs`/
/// `new_work_uc.rs`), so this test proves the identical "open A, open B,
/// mutate, save, close ONE, assert the OTHER is untouched" scenario through
/// nothing but the real, public `work_management_controller::load_work`/
/// `close_work` entry points — no bypass, no direct use-case/UoW construction
/// for B at all. This is the scenario a second project window actually drives.
#[test]
fn a_second_work_opened_through_the_real_load_work_path_never_perturbs_the_first() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    // Work A, via the real public `load_work` path — exactly as a first
    // project window would open it.
    let mut bundle_a = sample_bundle();
    bundle_a.manifest.work.title = "First Project".into();
    bundle_a.manifest.work.unique_id = "first-project-uid".into();
    let path_a = dir.path().join("A");
    skrib::write_bundle(
        path_a.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &bundle_a,
    )
    .unwrap();
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path_a.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work A");
    let work_a_id = live_work_id(&db);
    let row_a_before = db
        .get_store()
        .works
        .read()
        .unwrap()
        .get(&work_a_id)
        .cloned()
        .expect("A resident");
    let fp_a_checkpoint0 = fingerprint_of(&db, &hub, work_a_id, &dir.path().join("real-fp0"));

    // Work B, ALSO via the real public `load_work` path, on the SAME store,
    // while A is still open — exactly a second project window opening a
    // second Work. Before Phase 2 this call would have swept A away first;
    // it must not, any more.
    let mut bundle_b = sample_bundle();
    bundle_b.manifest.work.title = "Second Project".into();
    bundle_b.manifest.work.unique_id = "second-project-uid".into();
    let path_b = dir.path().join("B");
    skrib::write_bundle(
        path_b.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &bundle_b,
    )
    .unwrap();
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path_b.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work B — must NOT close Work A first");
    let work_b_id = *db
        .get_store()
        .works
        .read()
        .unwrap()
        .keys()
        .find(|&&id| id != work_a_id)
        .expect("a second, distinct Work must now be resident");

    assert_eq!(
        db.get_store().works.read().unwrap().len(),
        2,
        "both A and B must be resident after loading B through the real path"
    );
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before.clone()),
        "A's row must be byte-for-byte the same object after B loads through the real path"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("real-fp1")),
        fp_a_checkpoint0,
        "A unaffected by B's real load_work call"
    );

    // Mutate B directly (simulating a live edit), then save B through the
    // real, work_id-scoped save path — A must not move at any point.
    {
        let store = db.get_store();
        let mut works = store.works.write().unwrap();
        let mut b = works.get(&work_b_id).unwrap().clone();
        b.title = "Mutated B (real path)".into();
        works.insert(work_b_id, b);
    }
    let save_b = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: work_b_id,
            file_name: dir.path().join("real-out-b").to_str().unwrap().to_string(),
            overwrite: true,
        },
    );
    let save_b_result = save_b
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save_work B");
    let saved_b_bundle = skrib::read_bundle(&save_b_result.output_path).unwrap();
    assert_eq!(saved_b_bundle.manifest.work.title, "Mutated B (real path)");
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before.clone()),
        "A unaffected by B's mutate + save"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("real-fp2")),
        fp_a_checkpoint0,
        "A's fingerprint unaffected by B's mutate + save"
    );

    // Close B through the real, public `close_work` path — A must survive,
    // fully intact, and B's rows must be fully gone.
    work_management_controller::close_work(&db, &hub, &CloseWorkDto { work_id: work_b_id })
        .expect("close_work B");

    assert!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_b_id)
            .is_none(),
        "B's Work row must be fully removed by the real close_work path"
    );
    assert_eq!(
        db.get_store().works.read().unwrap().len(),
        1,
        "only A remains resident after B closes"
    );
    assert_eq!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .get(&work_a_id)
            .cloned(),
        Some(row_a_before),
        "A survives B's whole real-path lifecycle (load, mutate, save, close), byte-identical"
    );
    assert_eq!(
        fingerprint_of(&db, &hub, work_a_id, &dir.path().join("real-fp3")),
        fp_a_checkpoint0,
        "A byte-identical throughout — the whole two-Works scenario, driven only through \
         the real public load_work/close_work commands, exactly as two project windows would"
    );

    // A must still be fully open/save-able afterward — a full content check.
    let final_a = store_to_bundle(&db, &hub, &dir.path().join("real-final-a"));
    assert_eq!(final_a.manifest.work.title, "First Project");
    assert_eq!(final_a.manifest.work.unique_id, "first-project-uid");
    assert_eq!(norm(&final_a), norm(&bundle_a));
}

/// Regression guard for the write-transaction single-writer invariant
/// (`common::database::write_guard`): a second write transaction opened on the
/// SAME store while the first is still open must error/panic (RAII, not
/// merely a convention) — exercised here through the real `load_work`/
/// `new_work`/`close_work` guard wiring rather than the unit's own isolated
/// tests (see `crates/common/src/database/write_guard.rs`), to prove the guard
/// is actually reached from the use-case layer, not just correct in isolation.
#[test]
fn a_second_write_transaction_on_the_same_store_is_rejected_while_the_first_is_open() {
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    let factory = LoadWorkUnitOfWorkFactory::new(&db, &hub);
    let mut first = factory.create();
    first
        .begin_transaction()
        .expect("first write transaction opens");

    let mut second = factory.create();
    let result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| second.begin_transaction()));
    assert!(
        result.is_err(),
        "a second concurrent write transaction on the same store must panic (debug build)"
    );

    // Tear down the first cleanly (rollback releases its guard) and confirm the
    // slot is free again afterward.
    first.rollback().expect("rollback releases the guard");
    let mut third = factory.create();
    assert!(
        third.begin_transaction().is_ok(),
        "the slot must be free once the first transaction ended"
    );
    third.rollback().ok();
}

/// Note templates must survive the **full store round-trip**: written, read, materialised
/// into store rows, gathered back out, written again.
///
/// This is the guard for the one step in that chain the compiler cannot check. Every
/// other site that had to learn about `NoteTemplate` is a struct literal or a trait impl,
/// so forgetting one is a build error — but the relationship fetch in
/// `skrib_format::tree_read::gather` is a plain method call. Omit it and everything still
/// compiles, `gather` simply reports zero templates, and the very next save writes the
/// project with none. Values here are deliberately off-default (`starred` differs between
/// the two rows, the bodies are non-empty) for the reason the fixture comments above give.
#[test]
fn note_templates_survive_a_save_load_round_trip() {
    const T: &str = "2020-01-01T00:00:00+00:00";
    let mut bundle = sample_bundle();
    bundle.note_templates = vec![
        skrib::NoteTemplateFile {
            file_id: 700,
            uid: common::uid::fixture_uid(700),
            created_at: T.into(),
            updated_at: T.into(),
            name: "Character sheet".into(),
            starred: true,
            path: "templates/700-character-sheet.djot".into(),
        },
        skrib::NoteTemplateFile {
            file_id: 701,
            uid: common::uid::fixture_uid(701),
            created_at: T.into(),
            updated_at: T.into(),
            name: "Location".into(),
            starred: false,
            path: "templates/701-location.djot".into(),
        },
    ];
    bundle.note_template_bodies = [
        (700u64, "# Character sheet\n\n- Full name:\n".to_string()),
        (701u64, "# Location\n\nSensory detail:\n".to_string()),
    ]
    .into_iter()
    .collect();

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("WithTemplates");
    skrib::write_bundle(src.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load bundle with note templates");

    let out = store_to_bundle(&db, &hub, &dir.path().join("out"));

    let got: Vec<_> = out
        .note_templates
        .iter()
        .map(|t| {
            (
                t.name.clone(),
                t.starred,
                out.note_template_bodies
                    .get(&t.file_id)
                    .cloned()
                    .unwrap_or_default(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            (
                "Character sheet".to_string(),
                true,
                "# Character sheet\n\n- Full name:\n".to_string()
            ),
            (
                "Location".to_string(),
                false,
                "# Location\n\nSensory detail:\n".to_string()
            ),
        ],
        "every template must round-trip with its body and its starred flag, in order"
    );
}

/// A file the format does not model must survive a real `save_work`, not just a
/// direct `write_bundle`.
///
/// The unit test in `skrib_format` proves the writer preserves what the reader
/// hands it; this proves the *save use case* hands it anything at all. It is the
/// half that is easy to get wrong, because a save builds its bundle from store
/// entities — which have never heard of an unmodelled file — so without
/// `carry::load` reading the source back, `from_entities` would produce an empty
/// carry set and the write would delete the file with no error anywhere.
///
/// Saving **in place** is the case that matters: it is what autosave does every
/// few seconds, so a regression here destroys an extension's data in a project
/// within seconds of a community build opening it.
#[test]
fn an_unmodelled_file_survives_a_real_save_work() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("Novel");
    let path = project.to_str().unwrap().to_string();
    skrib::write_bundle(&path, SkribShape::ExplodedFolder, &sample_bundle()).unwrap();

    // Planted the way a build with a feature this one lacks would have left it —
    // and planted *inside* `binders/<b>/text/`, deliberately.
    //
    // A root-level file would prove nothing here: no prune runs at the bundle
    // root, so an exploded folder keeps one whether or not carrying works. The
    // sidecar prune does sweep this directory for stray `.ron`, which makes this
    // the shortest path from "carrying is broken" to "the writer's data is gone".
    let root = &project;
    let binder_dir = std::fs::read_dir(root.join("binders"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.is_dir())
        .expect("the sample project has a binder directory");
    let planted = binder_dir.join("text").join("plot.beats.ron");
    std::fs::write(&planted, b"(beats: [(at: 0.5)])").unwrap();

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");

    let uc = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: live_work_id(&db),
            file_name: path.clone(),
            overwrite: true,
        },
    );
    uc.execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save in place");

    assert_eq!(
        std::fs::read(&planted).ok().as_deref(),
        Some(b"(beats: [(at: 0.5)])".as_slice()),
        "save_work destroyed a bundle file it does not model"
    );
}

/// A registered contributor's files reach the bundle, and **override** the copy
/// already on disk.
///
/// The override direction is the whole point of the hook. `carry::load` reads
/// what the previous save left in the file; a contributor holds what is true
/// now. If the on-disk read won, every save would write the stale copy back
/// over the extension's live state — a bug that looks exactly like "my changes
/// don't save" and has no error anywhere to explain it.
#[test]
fn a_bundle_contributor_writes_into_the_project_and_beats_the_stale_copy() {
    use crate::bundle_contributors::{BundleContributor, SaveContext, register};
    use std::collections::BTreeMap;

    let mut bundle = sample_bundle();
    // A uid **this test alone** owns. Every fixture project shares
    // `sample_bundle`'s, and the contributor registry is process-wide with tests
    // running in parallel — so a contributor scoped to the shared uid still
    // leaks its file into a sibling test's bundle. That is not hypothetical: it
    // is what made `backup_multi_destination_is_resilient_and_dedups` flake once
    // backups started collecting contributor files, since the extra file moves
    // the content fingerprint that skip-if-unchanged compares.
    let uid = "contributor-e2e-uid".to_string();
    bundle.manifest.work.unique_id = uid.clone();

    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("Novel");
    let path = project.to_str().unwrap().to_string();
    skrib::write_bundle(&path, SkribShape::ExplodedFolder, &bundle).unwrap();
    // The previous save's version, as `carry::load` will find it.
    std::fs::create_dir_all(project.join("ext")).unwrap();
    std::fs::write(project.join("ext/data.ron"), b"(stale)").unwrap();

    /// Scoped to one project id on purpose: the registry is process-wide and
    /// tests run in parallel, so a contributor that answered for every project
    /// would leak files into unrelated tests' bundles.
    struct OneProject(String);
    impl BundleContributor for OneProject {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            let mut out = BTreeMap::new();
            if ctx.work_unique_id == self.0 {
                out.insert("ext/data.ron".to_string(), b"(live)".to_vec());
            }
            Ok(out)
        }
    }
    let _handle = register("test.e2e", Arc::new(OneProject(uid.clone())));

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");

    let uc = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: live_work_id(&db),
            file_name: path.clone(),
            overwrite: true,
        },
    );
    uc.execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save");

    assert_eq!(
        std::fs::read(project.join("ext/data.ron")).ok().as_deref(),
        Some(b"(live)".as_slice()),
        "the contributor's current state must win over the copy read off disk"
    );
}

/// **What a save tells its contributors, on the real write paths.**
///
/// Three claims, asserted together because each is worth little alone:
///
/// 1. Every write says *which* write it was, so a contributor can tell the
///    writer pressing save from a scheduled backup copying a state they had
///    already reached. Nothing exercised that fork before this test.
/// 2. Two saves of an unedited manuscript carry the **same** fingerprint, a
///    backup of it carries that same one again — despite being written as a zip
///    where the project is a folder — and one edit moves it.
/// 3. All of that holds while a *second* contributor writes different bytes on
///    every single call.
///
/// The third is the ordering claim and only the real path can make it: save #2's
/// `carry::load` reads the file save #1's contributor left on disk, so a
/// fingerprint taken after that read folds an extension's churn into "did the
/// book change?" and answers yes forever after. The unit tests in
/// `bundle_contributors` can only simulate that merge; this one lives it.
#[test]
fn every_write_tells_its_contributors_which_write_it_is_and_what_the_book_says() {
    use crate::bundle_contributors::{BundleContributor, SaveContext, register};
    use crate::lifecycle::SaveKind;
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicU64, Ordering};

    // A uid this test alone owns: the registry is process-wide and tests run in
    // parallel, so anything wider writes files into a sibling's bundle.
    let uid = "save-context-uid";
    let (dir, path) = write_sample_with_uid(uid);

    /// Different bytes on every call. The point of it is to be invisible to the
    /// contributor beside it.
    struct EverChanging(String, AtomicU64);
    impl BundleContributor for EverChanging {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            if ctx.work_unique_id != self.0 {
                return Ok(BTreeMap::new());
            }
            let n = self.1.fetch_add(1, Ordering::Relaxed);
            Ok(BTreeMap::from([(
                "ext/churn.ron".to_string(),
                format!("(call: {n})").into_bytes(),
            )]))
        }
    }

    /// Writes nothing; just records what it was told.
    struct Watch(String, std::sync::Mutex<Vec<(SaveKind, String)>>);
    impl BundleContributor for Watch {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            if ctx.work_unique_id == self.0 {
                self.1
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .push((ctx.kind, ctx.manuscript_fingerprint.clone()));
            }
            Ok(BTreeMap::new())
        }
    }

    let watch = Arc::new(Watch(uid.to_string(), std::sync::Mutex::new(Vec::new())));
    let _churn = register(
        "test.ctx.churn",
        Arc::new(EverChanging(uid.to_string(), AtomicU64::new(0))),
    );
    let _watch = register("test.ctx.watch", watch.clone());

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");
    let work_id = live_work_id(&db);

    let save = |label: &str| {
        SaveWorkUseCase::new(
            Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
            &SaveWorkDto {
                media_root: String::new(),
                work_id,
                file_name: path.clone(),
                overwrite: true,
            },
        )
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .unwrap_or_else(|e| panic!("{label}: {e}"));
    };

    save("first save");
    save("second save, nothing edited");

    let dest = dir.path().join("backups");
    std::fs::create_dir_all(&dest).unwrap();
    BackupNowUseCase::new(
        Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)),
        &plain_backup_dto(work_id, vec![dest.to_str().unwrap().to_string()], vec![]),
    )
    .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
    .expect("backup");

    // One real edit, straight into the live store — the simplest change that is
    // unambiguously *content* rather than bookkeeping.
    {
        let store = db.get_store();
        let mut works = store.works.write().unwrap();
        let mut w = works.get(&work_id).unwrap().clone();
        w.title = "The Lighthouse, Revised".into();
        works.insert(work_id, w);
    }
    save("save after an edit");

    let seen = watch.1.lock().unwrap_or_else(|e| e.into_inner()).clone();
    assert_eq!(
        seen.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
        vec![
            SaveKind::Save,
            SaveKind::Save,
            SaveKind::Backup,
            SaveKind::Save
        ],
        "each write path must name itself, and a backup must fire exactly once"
    );
    let fp: Vec<&String> = seen.iter().map(|(_, f)| f).collect();
    assert_eq!(
        fp[0], fp[1],
        "a second save of an unedited manuscript must fingerprint identically, even though \
         an extension wrote different bytes into the bundle in between"
    );
    assert_eq!(
        fp[1], fp[2],
        "a backup is a copy of the same book: being written as a zip where the project is a \
         folder is not an edit"
    );
    assert_ne!(
        fp[2], fp[3],
        "one word changed in the manuscript must move the fingerprint"
    );
    // …and the churn really did reach disk, or the test above proved nothing.
    let churned = skrib::read_bundle(&path).expect("reread the project");
    assert_eq!(
        churned
            .carried
            .get("ext/churn.ron")
            .map(|f| f.bytes.as_slice()),
        Some(&b"(call: 3)"[..]),
        "the fourth call's bytes are what the last save wrote, so every earlier save wrote \
         a different file and the fingerprints above were compared across real churn"
    );
}

/// **The fingerprint has to survive closing the project and opening it again**,
/// or it answers a question no extension asked.
///
/// The obvious doubt: every `*File` row carries a `file_id`, which is the store's
/// `EntityId` **at save time** and is re-minted by every `load_work`. If those
/// landed differently on the second load, a manuscript nobody edited would
/// fingerprint anew on the first save of every session — and anything comparing
/// against a stored value would read that as a change the writer never made,
/// once per session, forever.
#[test]
fn the_fingerprint_survives_a_close_and_a_reopen() {
    use crate::bundle_contributors::{BundleContributor, SaveContext, register};
    use std::collections::BTreeMap;

    let uid = "reopen-fingerprint-uid";
    let (_dir, path) = write_sample_with_uid(uid);

    struct Watch(String, std::sync::Mutex<Vec<String>>);
    impl BundleContributor for Watch {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            if ctx.work_unique_id == self.0 {
                self.1
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .push(ctx.manuscript_fingerprint.clone());
            }
            Ok(BTreeMap::new())
        }
    }
    let watch = Arc::new(Watch(uid.to_string(), std::sync::Mutex::new(Vec::new())));
    let _h = register("test.reopen", watch.clone());

    // Two whole store lifetimes, which is what a close and a reopen amount to:
    // fresh entity ids, materialised from the file all over again.
    for _session in 0..2 {
        let db = DbContext::new().unwrap();
        let hub = Arc::new(EventHub::new());
        work_management_controller::load_work(
            &db,
            &hub,
            &LoadWorkDto {
                media_root: String::new(),
                file_name: path.clone(),
            },
        )
        .expect("load");
        SaveWorkUseCase::new(
            Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
            &SaveWorkDto {
                media_root: String::new(),
                work_id: live_work_id(&db),
                file_name: path.clone(),
                overwrite: true,
            },
        )
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save");
    }

    let fp = watch.1.lock().unwrap_or_else(|e| e.into_inner()).clone();
    assert_eq!(fp.len(), 2, "one save per session");
    assert_eq!(
        fp[0], fp[1],
        "closing a project and opening it again is not an edit to the book"
    );
}

/// **The same number, asked of the file instead of the store.**
///
/// A contributor keeping a record across sessions has to find out, when a
/// project *opens*, whether the book moved while this build was not watching.
/// The only thing it has to ask is the bundle `read_bundle` just returned — so
/// that has to give the same answer a save gives, or the check reports a change
/// at the start of every session and there is no way to tell a real one from the
/// noise.
///
/// The file on disk always has `carried` populated (an extension's own data, at
/// minimum) where the store's bundle does not, which is exactly why
/// `manuscript_fingerprint` drops it rather than leaving that to a caller who
/// might forget.
#[test]
fn the_fingerprint_is_the_same_asked_of_the_saved_file() {
    use crate::bundle_contributors::{
        BundleContributor, SaveContext, manuscript_fingerprint, register,
    };
    use std::collections::BTreeMap;

    let uid = "reread-fingerprint-uid";
    let (_dir, path) = write_sample_with_uid(uid);

    /// Writes a file of its own, so the saved bundle has something in `carried`
    /// that the store's bundle never had.
    struct WatchAndWrite(String, std::sync::Mutex<Vec<String>>);
    impl BundleContributor for WatchAndWrite {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            if ctx.work_unique_id != self.0 {
                return Ok(BTreeMap::new());
            }
            self.1
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(ctx.manuscript_fingerprint.clone());
            Ok(BTreeMap::from([(
                "ext/record.ron".to_string(),
                b"(a record of some kind)".to_vec(),
            )]))
        }
    }
    let watch = Arc::new(WatchAndWrite(
        uid.to_string(),
        std::sync::Mutex::new(Vec::new()),
    ));
    let _h = register("test.reread", watch.clone());

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");
    SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: live_work_id(&db),
            file_name: path.clone(),
            overwrite: true,
        },
    )
    .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
    .expect("save");

    let told = watch.1.lock().unwrap_or_else(|e| e.into_inner())[0].clone();
    let from_disk = skrib::read_bundle(&path).expect("reread the project");
    assert!(
        from_disk.carried.contains_key("ext/record.ron"),
        "the contributor's file must really be in the bundle, or this proves nothing"
    );
    assert_eq!(
        manuscript_fingerprint(&from_disk),
        told,
        "the fingerprint of the saved file must be the one the save handed out"
    );
}

// ── The lifecycle hook (`crate::lifecycle`) through the real use cases ───────

/// A listener scoped to **one** project. The registry is process-wide and tests
/// run in parallel, so anything wider counts a sibling test's events as its own —
/// which is why every test below gives its project a uid it alone names.
struct LifecycleLog {
    uid: String,
    seen: std::sync::Mutex<Vec<crate::lifecycle::LifecycleEvent>>,
}

impl LifecycleLog {
    fn new(uid: &str) -> Arc<Self> {
        Arc::new(Self {
            uid: uid.to_string(),
            seen: std::sync::Mutex::new(Vec::new()),
        })
    }
    fn events(&self) -> Vec<crate::lifecycle::LifecycleEvent> {
        self.seen.lock().unwrap().clone()
    }
}

impl crate::lifecycle::LifecycleListener for LifecycleLog {
    fn on_event(&self, event: &crate::lifecycle::LifecycleEvent) {
        use crate::lifecycle::LifecycleEvent::*;
        let mine = match event {
            Opened { unique_id, .. } | Closed { unique_id } | Saved { unique_id, .. } => {
                unique_id == &self.uid
            }
        };
        if mine {
            self.seen.lock().unwrap().push(event.clone());
        }
    }
}

/// Write `sample_bundle` under a uid this test alone owns, so a process-wide
/// listener can tell its own project's events from a sibling's.
fn write_sample_with_uid(uid: &str) -> (tempfile::TempDir, String) {
    let mut bundle = sample_bundle();
    bundle.manifest.work.unique_id = uid.to_string();
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("Novel");
    let path = project.to_str().unwrap().to_string();
    skrib::write_bundle(&path, SkribShape::ExplodedFolder, &bundle).unwrap();
    (dir, path)
}

/// Opening a project fires exactly one `Opened`, naming the project by its
/// durable id and path, and carrying **every** live item uid — the prune list an
/// extension has no other way to build, since no cascade ever reaches one.
#[test]
fn loading_a_project_fires_opened_with_its_live_item_uids() {
    let log = LifecycleLog::new("lifecycle-open-uid");
    let _h = crate::lifecycle::register("test.lc.open", log.clone());

    let (_dir, path) = write_sample_with_uid("lifecycle-open-uid");
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");

    let events = log.events();
    assert_eq!(events.len(), 1, "exactly one Opened per load: {events:?}");
    match &events[0] {
        crate::lifecycle::LifecycleEvent::Opened {
            path: p,
            live_binder_item_uids,
            ..
        } => {
            assert_eq!(p, &path);
            let in_store = db.get_store().binder_items.read().unwrap().len();
            assert_eq!(
                live_binder_item_uids.len(),
                in_store,
                "every item in the project must be in the prune list"
            );
            assert!(
                live_binder_item_uids.iter().all(|u| !u.is_nil()),
                "a nil uid prunes nothing and matches everything"
            );
        }
        other => panic!("expected Opened, got {other:?}"),
    }
}

/// Closing fires exactly one `Closed`, and it names the project.
///
/// **The ordering that makes it work:** `unique_id` is read *before* the
/// teardown. Read after, it would come back empty — with no compiler error and
/// nothing failing — so every extension would keep that project's state resident
/// for the life of the process, and re-opening it would find the stale slot.
#[test]
fn closing_a_project_fires_closed_naming_the_project_that_closed() {
    let log = LifecycleLog::new("lifecycle-close-uid");
    let _h = crate::lifecycle::register("test.lc.close", log.clone());

    let (_dir, path) = write_sample_with_uid("lifecycle-close-uid");
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path,
        },
    )
    .expect("load");
    let work_id = live_work_id(&db);

    work_management_controller::close_work(&db, &hub, &CloseWorkDto { work_id }).expect("close");

    let closed: Vec<_> = log
        .events()
        .into_iter()
        .filter(|e| matches!(e, crate::lifecycle::LifecycleEvent::Closed { .. }))
        .collect();
    assert_eq!(closed.len(), 1, "exactly one Closed per close: {closed:?}");
    // …and it really did fire after the teardown, not before.
    assert!(
        db.get_store().works.read().unwrap().is_empty(),
        "close_work must have removed the subtree"
    );
}

/// A save fires exactly one `Saved`, tagged for the write that produced it and
/// naming the file the bytes went to.
#[test]
fn saving_fires_one_saved_tagged_with_the_kind_of_write() {
    let log = LifecycleLog::new("lifecycle-save-uid");
    let _h = crate::lifecycle::register("test.lc.save", log.clone());

    let (_dir, path) = write_sample_with_uid("lifecycle-save-uid");
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");

    SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto {
            media_root: String::new(),
            work_id: live_work_id(&db),
            file_name: path.clone(),
            overwrite: true,
        },
    )
    .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
    .expect("save");

    let saved: Vec<_> = log
        .events()
        .into_iter()
        .filter_map(|e| match e {
            crate::lifecycle::LifecycleEvent::Saved { kind, path, .. } => Some((kind, path)),
            _ => None,
        })
        .collect();
    assert_eq!(saved.len(), 1, "exactly one Saved per save: {saved:?}");
    assert_eq!(saved[0].0, crate::lifecycle::SaveKind::Save);
    assert_eq!(saved[0].1, path);
}

/// A backup writes one file **per destination**, so it fires one `Saved` per
/// destination — each naming its own backup file, never the project.
///
/// Cross-checked against what actually landed on disk, because the failure this
/// guards is a listener mirroring three destinations off one notification.
#[test]
fn a_backup_fires_one_saved_per_destination_written() {
    let log = LifecycleLog::new("lifecycle-backup-uid");
    let _h = crate::lifecycle::register("test.lc.backup", log.clone());

    let (dir, path) = write_sample_with_uid("lifecycle-backup-uid");
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");

    let dests: Vec<String> = (0..3)
        .map(|i| {
            let d = dir.path().join(format!("dest{i}"));
            std::fs::create_dir_all(&d).unwrap();
            d.to_str().unwrap().to_string()
        })
        .collect();
    let res = BackupNowUseCase::new(
        Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)),
        &plain_backup_dto(live_work_id(&db), dests, vec![]),
    )
    .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
    .expect("backup");
    assert_eq!(res.succeeded_paths.len(), 3);

    let mut announced: Vec<String> = log
        .events()
        .into_iter()
        .filter_map(|e| match e {
            crate::lifecycle::LifecycleEvent::Saved {
                kind: crate::lifecycle::SaveKind::Backup,
                path,
                ..
            } => Some(path),
            _ => None,
        })
        .collect();
    announced.sort();
    let mut written = res.succeeded_paths.clone();
    written.sort();
    assert_eq!(
        announced, written,
        "one Saved per backup actually written, naming that file"
    );
}

/// **A backup used to lose every unmodelled file.** It builds its bundle by hand
/// rather than through `work_io::serialize_and_write`, and so had neither
/// `carry::load` nor the contributor collection — so an extension's data (and
/// any carried file from a build that did not model it) was silently absent from
/// every backup, and restoring one wiped it from the project.
#[test]
fn a_backup_carries_unmodelled_files_and_the_contributors_current_state() {
    use crate::bundle_contributors::{BundleContributor, SaveContext, register};
    use std::collections::BTreeMap;

    let uid = "backup-carry-uid";
    let (dir, path) = write_sample_with_uid(uid);
    // A file the format does not model, as a previous save left it.
    std::fs::create_dir_all(std::path::Path::new(&path).join("ext")).unwrap();
    std::fs::write(
        std::path::Path::new(&path).join("ext/stale.ron"),
        b"(from disk)",
    )
    .unwrap();

    struct OneProject(String);
    impl BundleContributor for OneProject {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            let mut out = BTreeMap::new();
            if ctx.work_unique_id == self.0 {
                out.insert("ext/live.ron".to_string(), b"(from memory)".to_vec());
            }
            Ok(out)
        }
    }
    let _c = register("test.backup-carry", Arc::new(OneProject(uid.to_string())));

    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path.clone(),
        },
    )
    .expect("load");

    let dest = dir.path().join("backups");
    std::fs::create_dir_all(&dest).unwrap();
    let res = BackupNowUseCase::new(
        Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)),
        &plain_backup_dto(
            live_work_id(&db),
            vec![dest.to_str().unwrap().to_string()],
            vec![],
        ),
    )
    .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
    .expect("backup");

    let restored = skrib::read_bundle(&res.succeeded_paths[0]).expect("read the backup");
    assert_eq!(
        restored
            .carried
            .get("ext/stale.ron")
            .map(|f| f.bytes.as_slice()),
        Some(&b"(from disk)"[..]),
        "a backup dropped an unmodelled file, so restoring it would delete that file"
    );
    assert_eq!(
        restored
            .carried
            .get("ext/live.ron")
            .map(|f| f.bytes.as_slice()),
        Some(&b"(from memory)"[..]),
        "a backup must carry a contributor's CURRENT state, as a save does"
    );
}

/// **Two projects open at once, and each event names exactly one of them.**
///
/// The whole point of the payload being `unique_id` rather than a store id: a
/// listener keys its state by project, so an event misattributed to the wrong one
/// would load or evict the wrong plan. Opening B must not tell a listener anything
/// about A, and closing A must not tell it anything about B.
#[test]
fn lifecycle_events_name_exactly_the_project_they_are_about() {
    let log_a = LifecycleLog::new("multi-uid-a");
    let log_b = LifecycleLog::new("multi-uid-b");
    let _ha = crate::lifecycle::register("test.lc.multi.a", log_a.clone());
    let _hb = crate::lifecycle::register("test.lc.multi.b", log_b.clone());

    let (_dir_a, path_a) = write_sample_with_uid("multi-uid-a");
    let (_dir_b, path_b) = write_sample_with_uid("multi-uid-b");

    // One store, two Works open at once — the arrangement `Root.works` allows and
    // the one every multi-project bug in this seam has needed.
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path_a.clone(),
        },
    )
    .expect("load A");
    assert_eq!(log_a.events().len(), 1, "opening A tells A's listener once");
    assert!(
        log_b.events().is_empty(),
        "opening A must say nothing about B"
    );

    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: path_b.clone(),
        },
    )
    .expect("load B");
    assert_eq!(
        log_a.events().len(),
        1,
        "opening B must not fire a second Opened for A"
    );
    assert_eq!(log_b.events().len(), 1, "…and exactly one for B");

    // Close A only. Both Works are in the store, so this is the case where a
    // wrong `unique_id` — the trap `close_work_uc` reads BEFORE teardown to
    // avoid — would evict the surviving project's state.
    let work_a = *db
        .get_store()
        .works
        .read()
        .unwrap()
        .iter()
        .find(|(_, w)| w.unique_id == "multi-uid-a")
        .expect("A is open")
        .0;
    work_management_controller::close_work(&db, &hub, &CloseWorkDto { work_id: work_a })
        .expect("close A");

    let closed_a: Vec<_> = log_a
        .events()
        .into_iter()
        .filter(|e| matches!(e, crate::lifecycle::LifecycleEvent::Closed { .. }))
        .collect();
    assert_eq!(closed_a.len(), 1, "closing A tells A's listener once");
    assert!(
        !log_b
            .events()
            .iter()
            .any(|e| matches!(e, crate::lifecycle::LifecycleEvent::Closed { .. })),
        "closing A reported B as closed — a listener would have evicted the open project"
    );

    // …and B really is still open, so the event was not merely mislabelled.
    assert!(
        db.get_store()
            .works
            .read()
            .unwrap()
            .values()
            .any(|w| w.unique_id == "multi-uid-b"),
        "closing A removed B from the store"
    );
}
