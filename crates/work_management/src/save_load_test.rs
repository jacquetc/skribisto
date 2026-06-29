//! End-to-end backend test for the save loop: write a new-format folder, load
//! it into a real in-memory store via `load_work`, save it back out via
//! `save_work`, and assert the project survives the store round-trip (ids are
//! reassigned by the store, so the comparison is id-free / structural).

use crate::LoadWorkDto;
use crate::SaveWorkDto;
use crate::skrib::{self, BinderWithItems, ItemWithContents, ShapeTag, SkribShape, WorkBundle};
use crate::units_of_work::save_work_uow::SaveWorkUnitOfWorkFactory;
use crate::use_cases::save_work_uc::SaveWorkUseCase;
use crate::work_management_controller;
use chrono::{DateTime, Utc};
use common::database::db_context::DbContext;
use common::entities::{
    Binder, BinderItem, BinderItemRole, BinderItemSubRole, BinderTag, Content, ContentRole,
    DictWord, TrashInfo, Work,
};
use common::event::EventHub;
use common::long_operation::LongOperation;
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
        tags: vec![10, 11],
        dict_words: vec![20],
        binders: vec![100, 101],
    };
    let tags = vec![
        BinderTag { id: 10, created_at: ts(), updated_at: ts(), name: "Important".into(), color: "#f00".into(), text_color: "#fff".into() },
        BinderTag { id: 11, created_at: ts(), updated_at: ts(), name: "Idea".into(), color: "#0f0".into(), text_color: "#000".into() },
    ];
    let dict_words = vec![DictWord { id: 20, created_at: ts(), updated_at: ts(), word: "Skribisto".into() }];

    let manuscript = BinderWithItems {
        binder: Binder { id: 100, created_at: ts(), updated_at: ts(), name: "Manuscript".into(), activated: true, binder_items: Vec::new() },
        items: vec![
            item(300, "The Lighthouse", Folder, Book, vec![
                content(400, BookTitle, "The Lighthouse"),
                content(401, BookSubtitle, "A Novel"),
                content(402, SynopsisText, "A keeper and a storm."),
            ]),
            item(301, "Chapter One", Folder, Chapter, vec![
                content(410, ChapterTitle, "Chapter One"),
                content(411, SynopsisText, "Arrival."),
            ]),
            item(302, "The ferry", Item, Scene, vec![
                content(420, SceneText, "The ferry pitched in the swell."),
                content(421, SynopsisText, "They cross."),
            ]),
            item(303, "Into the Dark", Item, ChapterScene, vec![
                content(430, ChapterTitle, "Chapter Two"),
                content(431, SceneText, "The light failed at midnight."),
                content(432, SynopsisText, "The storm hits."),
            ]),
        ],
    };
    let characters = BinderWithItems {
        binder: Binder { id: 101, created_at: ts(), updated_at: ts(), name: "Characters".into(), activated: true, binder_items: Vec::new() },
        items: vec![item(320, "Mara Vance", Item, Note, vec![
            content(500, NoteText, "The keeper's daughter."),
            content(501, SynopsisText, "Protagonist."),
        ])],
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

    skrib::from_entities(&work, &tags, &dict_words, &trash, &[manuscript, characters], ShapeTag::Folder)
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
        &LoadWorkDto { file_name: src.to_str().unwrap().to_string() },
    )
    .expect("load_work");

    // 3. Save the store back out to a different folder (run the long op inline).
    let uc = SaveWorkUseCase::new(
        Box::new(SaveWorkUnitOfWorkFactory::new(&db, &hub)),
        &SaveWorkDto { file_name: dst.to_str().unwrap().to_string(), overwrite: true },
    );
    let result = uc
        .execute(Box::new(|_| {}), Arc::new(AtomicBool::new(false)))
        .expect("save_work");
    assert_eq!(result.output_path, dst.to_str().unwrap());

    // 4. The resaved project must be structurally identical (ids/paths aside).
    let resaved = skrib::read_bundle(dst.to_str().unwrap()).unwrap();
    assert_eq!(norm(&original), norm(&resaved));
}
