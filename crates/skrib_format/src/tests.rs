//! Round-trip, validation, and diff-minimal tests for the `.skrib` serializer.

use super::bundle::{BinderWithItems, ItemWithContents};
use super::*;
use chrono::{DateTime, Utc};
use common::entities::{
    Binder, BinderItem, BinderItemRole, BinderItemSubRole, BinderTag, Content, ContentRole,
    DictWord, TrashInfo, Work,
};
use skribisto_model::{allowed_content, validate_item};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn ts() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).unwrap()
}

/// Every valid (role, sub_role) combination in the constraint matrix.
fn all_combinations() -> Vec<(BinderItemRole, BinderItemSubRole)> {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    vec![
        (Item, BookBegin),
        (Item, BookEnd),
        (Item, Scene),
        (Item, ChapterScene),
        (Item, Part),
        (Item, Chapter),
        (Item, Note),
        (Item, Text),
        (Folder, None),
        (Folder, Chapter),
        (Folder, Part),
        (Folder, Book),
        (Folder, Note),
    ]
}

fn prose_text(role: &ContentRole) -> String {
    match role {
        ContentRole::SceneText => "Scene prose with *emphasis* and ^super^.".to_string(),
        ContentRole::NoteText => "A note about the lighthouse keeper.".to_string(),
        ContentRole::SynopsisText => "They arrive; the storm is coming.".to_string(),
        ContentRole::BookTitle => "The Lighthouse".to_string(),
        ContentRole::BookSubtitle => "A Novel".to_string(),
        ContentRole::PartTitle => "Part One — Arrival".to_string(),
        ContentRole::ChapterTitle => "Chapter One".to_string(),
    }
}

/// Build a fixture covering every combination, with each item carrying exactly
/// its allowed content roles. `content_id` is bumped to keep ids unique.
fn sample_inputs() -> (
    Work,
    Vec<BinderTag>,
    Vec<DictWord>,
    Vec<TrashInfo>,
    Vec<BinderWithItems>,
) {
    let now = ts();
    let work = Work {
        id: 1,
        created_at: now,
        updated_at: now,
        title: "My Novel".into(),
        author_name: "Jane".into(),
        dict_language: "en-US".into(),
        unique_id: "test-unique-id-abc".into(),
        tags: vec![10, 11],
        dict_words: vec![20, 21],
        binders: vec![100],
    };
    let tags = vec![
        BinderTag {
            id: 10,
            created_at: now,
            updated_at: now,
            name: "Important".into(),
            color: "#f00".into(),
            text_color: "#fff".into(),
        },
        BinderTag {
            id: 11,
            created_at: now,
            updated_at: now,
            name: "Idea".into(),
            color: "#0f0".into(),
            text_color: "#000".into(),
        },
    ];
    let dict_words = vec![
        DictWord {
            id: 20,
            created_at: now,
            updated_at: now,
            word: "Skribisto".into(),
        },
        DictWord {
            id: 21,
            created_at: now,
            updated_at: now,
            word: "Bastyde".into(),
        },
    ];

    let mut items = Vec::new();
    let mut item_id = 300u64;
    let mut content_id = 1000u64;
    for (i, (role, sub_role)) in all_combinations().into_iter().enumerate() {
        let mut contents = Vec::new();
        for cr in allowed_content(&role, &sub_role) {
            contents.push(Content {
                id: content_id,
                created_at: now,
                updated_at: now,
                activated: true,
                role: cr.clone(),
                data: prose_text(cr),
            });
            content_id += 1;
        }
        let item = BinderItem {
            id: item_id,
            created_at: now,
            updated_at: now,
            title: format!("Item {i}"),
            sub_title: String::new(),
            role,
            sub_role,
            label: "1st plot point".into(),
            activated: true,
            is_favorite: i % 2 == 0,
            is_printable: true,
            indent: (i % 3) as i64,
            word_count_goal: 1000,
            char_count_goal: 5000,
            dict_language: "en-US".into(),
            contents: Vec::new(),
            references: Vec::new(),
            tags: vec![10],
        };
        items.push(ItemWithContents { item, contents });
        item_id += 1;
    }
    // A cross-reference: first item -> second item.
    items[0].item.references = vec![301];

    let binders = vec![BinderWithItems {
        binder: Binder {
            id: 100,
            created_at: now,
            updated_at: now,
            name: "Manuscript".into(),
            activated: true,
            binder_items: Vec::new(),
        },
        items,
    }];

    let trash = vec![
        TrashInfo {
            id: 200,
            created_at: now,
            updated_at: now,
            trashed_at: now,
            origin_binder_id: 100,
            trashed_binder: Option::None,
            trashed_binder_item: Some(305),
        },
        TrashInfo {
            id: 201,
            created_at: now,
            updated_at: now,
            trashed_at: now,
            origin_binder_id: 0,
            trashed_binder: Some(999),
            trashed_binder_item: Option::None,
        },
    ];

    (work, tags, dict_words, trash, binders)
}

fn build_bundle(shape: ShapeTag) -> WorkBundle {
    let (work, tags, dict_words, trash, binders) = sample_inputs();
    from_entities(&work, &tags, &dict_words, &trash, &binders, shape)
}

#[test]
fn folder_round_trip_is_lossless() {
    let bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    let read = read_bundle(root.to_str().unwrap()).unwrap();
    assert_eq!(bundle, read);
}

#[test]
fn folder_save_creates_named_subfolder_not_parent() {
    // Bug 1 regression: "Save as folder" passes `<picked>/<name>` as the target;
    // the bundle must land in that named subfolder, never loose in the picked
    // parent. `folder_root` returns a non-existent target as-is, so `write_folder`
    // creates it.
    let bundle = build_bundle(ShapeTag::Folder);
    let parent = tempfile::tempdir().unwrap();
    let target = parent.path().join("My Novel");
    write_bundle(
        target.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &bundle,
    )
    .unwrap();
    assert!(
        target.join("project.skrib").exists(),
        "manifest must be in the named subfolder"
    );
    assert!(
        !parent.path().join("project.skrib").exists(),
        "bundle must NOT be written loose in the picked parent"
    );
}

#[test]
fn zip_round_trip_matches_folder() {
    let bundle = build_bundle(ShapeTag::Zip);
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("MyNovel.skrib");
    write_bundle(target.to_str().unwrap(), SkribShape::ZipFile, &bundle).unwrap();
    assert_eq!(
        detect_shape(target.to_str().unwrap()).unwrap(),
        SkribShape::ZipFile
    );
    let read = read_bundle(target.to_str().unwrap()).unwrap();
    assert_eq!(bundle, read);
}

#[test]
fn every_item_validates() {
    let bundle = build_bundle(ShapeTag::Folder);
    for bb in &bundle.binders {
        for bi in &bb.items {
            let present: Vec<ContentRole> = bi
                .item
                .inline_contents
                .iter()
                .map(|c| c.role.clone())
                .chain(bi.item.prose_refs.iter().map(|p| p.role.clone()))
                .collect();
            validate_item(&bi.item.role, &bi.item.sub_role, &present).unwrap_or_else(|e| {
                panic!(
                    "{:?}/{:?} failed validation: {e:?}",
                    bi.item.role, bi.item.sub_role
                )
            });
        }
    }
}

#[test]
fn disallowed_content_is_dropped() {
    let now = ts();
    // An Item/Scene carrying an illegal NoteText row.
    let item = BinderItem {
        id: 300,
        created_at: now,
        updated_at: now,
        title: "Scene".into(),
        role: BinderItemRole::Item,
        sub_role: BinderItemSubRole::Scene,
        is_printable: true,
        ..Default::default()
    };
    let contents = vec![
        Content {
            id: 1,
            created_at: now,
            updated_at: now,
            activated: true,
            role: ContentRole::SceneText,
            data: "ok".into(),
        },
        Content {
            id: 2,
            created_at: now,
            updated_at: now,
            activated: true,
            role: ContentRole::NoteText,
            data: "illegal".into(),
        },
    ];
    let work = Work {
        id: 1,
        created_at: now,
        updated_at: now,
        binders: vec![100],
        ..Default::default()
    };
    let binders = vec![BinderWithItems {
        binder: Binder {
            id: 100,
            created_at: now,
            updated_at: now,
            name: "M".into(),
            activated: true,
            binder_items: Vec::new(),
        },
        items: vec![ItemWithContents { item, contents }],
    }];
    let bundle = from_entities(&work, &[], &[], &[], &binders, ShapeTag::Folder);
    let f = &bundle.binders[0].items[0].item;
    assert!(
        f.prose_refs
            .iter()
            .all(|p| p.role == ContentRole::SceneText)
    );
    assert!(f.inline_contents.is_empty());
    assert_eq!(
        f.prose_refs.len(),
        1,
        "NoteText must be dropped for a Scene"
    );
}

/// Recursively snapshot every file's bytes + mtime under `root`.
fn snapshot(root: &Path) -> BTreeMap<String, (std::time::SystemTime, Vec<u8>)> {
    let mut map = BTreeMap::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let rel = entry
                .path()
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let meta = entry.metadata().unwrap();
            map.insert(
                rel,
                (meta.modified().unwrap(), fs::read(entry.path()).unwrap()),
            );
        }
    }
    map
}

#[test]
fn writes_are_diff_minimal() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    let rs = root.to_str().unwrap();

    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    let before = snapshot(&root);

    // Re-writing the identical bundle must touch nothing (no mtime change).
    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    let after_noop = snapshot(&root);
    for (rel, (mtime, _)) in &before {
        assert_eq!(*mtime, after_noop[rel].0, "no-op save rewrote {rel}");
    }

    // Edit exactly one scene's prose; only that .djot blob may change.
    let target_id = {
        let item = &mut bundle.binders[0].items[2]; // Item/Scene
        let (cid, text) = item.prose.iter_mut().next().unwrap();
        *text = "Completely rewritten scene prose.".to_string();
        *cid
    };
    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    let after_edit = snapshot(&root);

    let changed: Vec<&String> = before
        .keys()
        .filter(|rel| before[*rel].1 != after_edit[*rel].1)
        .collect();
    assert_eq!(
        changed.len(),
        1,
        "exactly one file should change, got {changed:?}"
    );
    assert!(
        changed[0].contains(&target_id.to_string()),
        "the changed file {} should be the edited scene's .djot",
        changed[0]
    );
}
