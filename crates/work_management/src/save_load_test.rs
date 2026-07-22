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
            aliases: if id % 2 == 0 {
                vec![format!("Alias{id}"), format!("Miss Bennet {id}")]
            } else {
                Vec::new()
            },
            contents: Vec::new(),
            references: Vec::new(),
            tags: Vec::new(),
        },
        contents,
    }
}

/// Two binders covering scene/note/folder/title items with prose + titles.
fn sample_bundle() -> WorkBundle {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    use ContentRole::*;

    let work = Work {
        id: 1,
        created_at: ts(),
        updated_at: ts(),
        title: "The Lighthouse".into(),
        author_name: "Jane".into(),
        dict_language: vec!["en-US".to_string()],
        unique_id: "the-lighthouse-uid".into(),
        // Non-default so the round-trip actually exercises chapter_mode persistence.
        chapter_mode: common::entities::ChapterMode::Flat,
        tags: vec![10, 11],
        dict_words: vec![20],
        binders: vec![100, 101],
        trash_infos: vec![],
        paces: vec![],
    };
    let tags = vec![
        BinderTag {
            id: 10,
            created_at: ts(),
            updated_at: ts(),
            name: "Important".into(),
            color: "#f00".into(),
            details: "Needs a second pass".into(),
            discoverable: false,
        },
        BinderTag {
            id: 11,
            created_at: ts(),
            updated_at: ts(),
            name: "Idea".into(),
            color: "#0f0".into(),
            details: String::new(),
            discoverable: true,
        },
    ];
    let dict_words = vec![DictWord {
        id: 20,
        created_at: ts(),
        updated_at: ts(),
        word: "Skribisto".into(),
    }];

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
        items: vec![
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
                    content(431, SceneText, "The light failed at midnight."),
                    content(432, SynopsisText, "The storm hits."),
                ],
            ),
        ],
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
        &trash,
        &[],
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
    tags: Vec<NormTag>,
    words: Vec<String>,
    binders: Vec<NormBinder>,
    trash: usize,
    refs: usize,
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
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work");

    // 3. Save the store back out to a different folder through the real
    //    long-operation path, then read it back.
    let resaved = store_to_bundle(&db, &hub, &dst);

    // 4. The resaved project must be structurally identical (ids/paths aside).
    assert_eq!(norm(&original), norm(&resaved));

    // The stable id survives the save → load → save round-trip through the store.
    assert_eq!(resaved.manifest.work.unique_id, "the-lighthouse-uid");
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
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load bundle with a pace");

    let out = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(out.paces.len(), 1, "the pace must round-trip through the store");
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
    // Weak back-links survive (ids are reassigned by the store, so just assert they resolve).
    assert!(p.book_item.is_some(), "book_item must resolve after id remap");
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
        &LoadWorkDto { file_name: a.to_str().unwrap().to_string() },
    )
    .unwrap();
    let b_path = dir.path().join("B");
    let bundle_b = store_to_bundle(&db1, &hub1, &b_path);
    assert_eq!(bundle_b.progress_snapshots.len(), 2, "first save must keep both days");

    // Cycle 2: load B (a *fresh* store + WorkInfo) → save → assert still intact.
    let db2 = DbContext::new().unwrap();
    let hub2 = Arc::new(EventHub::new());
    work_management_controller::load_work(
        &db2,
        &hub2,
        &LoadWorkDto {
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
    assert!(days[0].book_item_ids.len() == 1, "the per-Book id must remap and survive");
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
            file_name: path.to_string(),
            is_folder,
            template_kind: t,
            labels: labels(),
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
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
            file_name: dir.path().join("Authored.skrib").to_str().unwrap().to_string(),
            is_folder: false,
            template_kind: NewWorkTemplate::Novel,
            labels: labels(),
            language: vec!["en-US".to_string()],
            author_name: "A. Writer".to_string(),
            chapter_scene_mode: false,
        },
    )
    .expect("new_work");

    let b = store_to_bundle(&db, &hub, &dir.path().join("out"));
    assert_eq!(b.manifest.work.author_name, "A. Writer");
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
        worst, 1,
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
    let id1 = store_to_bundle(&db, &hub, &dir.path().join("o1"))
        .manifest
        .work
        .unique_id;

    // A second new_work replaces the first and mints a different id.
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
            file_name: src.to_str().unwrap().to_string(),
        },
    )
    .expect("load_work");

    // Creating a new (None) work must clear the old tree entirely.
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
fn plain_backup_dto(work_id: u64, directories: Vec<String>, last_known_hashes: Vec<String>) -> BackupNowDto {
    BackupNowDto {
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
        &plain_backup_dto(live_work_id(&db), vec![backup_dir.to_str().unwrap().to_string()], vec![]),
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
        let mut dto = plain_backup_dto(live_work_id(&db), 
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

    let dto1 = plain_backup_dto(live_work_id(&db), vec![dest.to_str().unwrap().to_string()], vec![]);
    let res1 = BackupNowUseCase::new(Box::new(BackupNowUnitOfWorkFactory::new(&db, &hub)), &dto1)
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("backup_now");
    assert_eq!(res1.succeeded_paths.len(), 1, "first run writes the backup");
    let written = res1.succeeded_paths[0].clone();

    // The backup is gone out from under the app (drive wiped, folder removed).
    std::fs::remove_file(&written).unwrap();
    assert!(!std::path::Path::new(&written).exists());

    // A matching hash AND the now-deleted path must NOT be enough to skip.
    let mut dto2 = plain_backup_dto(live_work_id(&db), 
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

    let mut dto = plain_backup_dto(live_work_id(&db), 
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

    let dto = plain_backup_dto(live_work_id(&db), vec![dest.to_str().unwrap().to_string()], vec![]);
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

    let mut dto = plain_backup_dto(live_work_id(&db), vec![dest.to_str().unwrap().to_string()], vec![]);
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
    assert!(finished, "the panicking operation should settle within the timeout");
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
    let loaded =
        skrib::bundle_to_loaded(bundle, path.to_str().unwrap()).expect("bundle_to_loaded");
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
    skrib::write_bundle(path_a.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle_a).unwrap();
    work_management_controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
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
    skrib::write_bundle(path_b.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle_b).unwrap();
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
        db.get_store().works.read().unwrap().get(&work_a_id).cloned(),
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
        db.get_store().works.read().unwrap().get(&work_a_id).cloned(),
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
        db.get_store().works.read().unwrap().get(&work_a_id).cloned(),
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
        db.get_store().works.read().unwrap().get(&work_b_id).is_none(),
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
        db.get_store().works.read().unwrap().get(&work_a_id).cloned(),
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
    first.begin_transaction().expect("first write transaction opens");

    let mut second = factory.create();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        second.begin_transaction()
    }));
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
