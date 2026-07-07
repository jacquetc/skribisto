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
            is_printable: true,
            indent: 0,
            word_count_goal: 500,
            char_count_goal: 2000,
            dict_language: "en-US".into(),
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
        dict_language: "en-US".into(),
        unique_id: "the-lighthouse-uid".into(),
        tags: vec![10, 11],
        dict_words: vec![20],
        binders: vec![100, 101],
    };
    let tags = vec![
        BinderTag {
            id: 10,
            created_at: ts(),
            updated_at: ts(),
            name: "Important".into(),
            color: "#f00".into(),
            text_color: "#fff".into(),
        },
        BinderTag {
            id: 11,
            created_at: ts(),
            updated_at: ts(),
            name: "Idea".into(),
            color: "#0f0".into(),
            text_color: "#000".into(),
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
                Chapter,
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
    is_printable: bool,
    wcg: i64,
    inline: Vec<(String, String)>,
    prose: Vec<(String, String)>,
}
#[derive(Debug, PartialEq)]
struct NormBinder {
    name: String,
    activated: bool,
    items: Vec<NormItem>,
}
#[derive(Debug, PartialEq)]
struct Norm {
    title: String,
    author: String,
    lang: String,
    unique_id: String,
    tags: Vec<(String, String, String)>,
    words: Vec<String>,
    binders: Vec<NormBinder>,
    trash: usize,
    refs: usize,
}

fn norm(b: &WorkBundle) -> Norm {
    let mut tags: Vec<_> = b
        .tags
        .iter()
        .map(|t| (t.name.clone(), t.color.clone(), t.text_color.clone()))
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
                        is_printable: f.is_printable,
                        wcg: f.word_count_goal,
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
        "/../../resources/test/skribisto_test_project.skrib"
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
            language: "en-US".to_string(),
            chapter_scene_mode: false,
        },
    )
    .expect("new_work");
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
    assert_eq!(b.manifest.work.dict_language, "en-US");
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
        .filter(|i| i.item.sub_role == BinderItemSubRole::Chapter)
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
