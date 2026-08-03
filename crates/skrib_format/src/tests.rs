// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Round-trip, validation, and diff-minimal tests for the `.skrib` serializer.

use super::bundle::{BinderWithItems, CommentWithReplies, ItemWithContents};
use super::*;
use chrono::{DateTime, Utc};
use common::entities::{
    Binder, BinderItem, BinderItemRole, BinderItemSubRole, BinderTag, Comment, CommentAnchorKind,
    CommentOrphanReason, CommentReply, Content, ContentRole, DictWord, TrashInfo, Work,
};
use skribisto_model::{allowed_content, validate_item};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn ts() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).unwrap()
}

/// Assert a write→read cycle preserved everything, allowing for the **one** field the
/// writer legitimately computes rather than copies.
///
/// `folder_io::write_folder` stamps `format_min_read_version` from the bundle's real
/// content at the manifest commit (see [`crate::version_gate`]), which is why neither
/// producer sets it — so a bundle handed in with `None` reads back carrying its actual
/// floor. That difference is the feature working, not loss, so this asserts the stamp is
/// right and then compares everything else exactly.
fn assert_round_trip(input: &WorkBundle, read: &WorkBundle) {
    assert_eq!(
        read.manifest.format_min_read_version,
        Some(crate::version_gate::compute_min_read_version(input)),
        "the writer must stamp the content-derived read floor"
    );
    let mut normalized = read.clone();
    normalized.manifest.format_min_read_version = input.manifest.format_min_read_version;
    assert_eq!(
        input, &normalized,
        "everything but the computed floor must round-trip unchanged"
    );
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
        (Item, Note),
        (Item, Text),
        (Folder, None),
        (Folder, ChapterScene),
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
        // Two quotations in one row, separated by a genuine blank line — a `>`-only
        // continuation would fold them into a single blockquote. The round-trip test
        // is what pins that they stay two.
        ContentRole::EpigraphText => {
            "> The sea is not a place; it is a going.\n>\n> {alignment=right}\n> — Anon., *Tidewater*\n\n> Salt is the only honest preservative.\n>\n> {alignment=right}\n> — M. Ferrand"
                .to_string()
        }
    }
}

/// Build a fixture covering every combination, with each item carrying exactly
/// its allowed content roles. `content_id` is bumped to keep ids unique.
struct SampleInputs {
    work: Work,
    tags: Vec<BinderTag>,
    dict_words: Vec<DictWord>,
    note_templates: Vec<common::entities::NoteTemplate>,
    smart_punctuation: common::entities::SmartPunctuation,
    trash: Vec<TrashInfo>,
    binders: Vec<BinderWithItems>,
}

fn sample_inputs() -> SampleInputs {
    let now = ts();
    let work = Work {
        id: 1,
        created_at: now,
        updated_at: now,
        title: "My Novel".into(),
        author_name: "Jane".into(),
        dict_language: vec!["en-US".to_string()],
        unique_id: "test-unique-id-abc".into(),
        chapter_mode: common::entities::ChapterMode::Flat,
        custom_replacement_rules_enabled: false,
        tags: vec![10, 11],
        dict_words: vec![20, 21],
        text_replacement_rules: vec![],
        note_templates: vec![40, 41],
        smart_punctuation: 30,
        binders: vec![100],
        trash_infos: vec![],
        paces: vec![],
        comments: vec![],
    };
    // Every field deliberately OFF-default, for the reason spelled out on the
    // tags below: the round-trip tests compare whole bundles, and a row left at
    // its derived defaults would compare equal even if the field were dropped
    // end to end. That is exactly how `chapter_mode` was silently lost.
    // Two templates, both off-default (starred differs between them, bodies are
    // non-empty multi-block Djot) for the reason the comment above gives: a row left
    // at its defaults would compare equal even if the field were dropped end to end.
    // These ride the whole-bundle round-trip assertions below, which is what makes
    // "someone forgot the `gather()` fetch" a failing test rather than silent data loss.
    let note_templates = vec![
        common::entities::NoteTemplate {
            id: 40,
            created_at: now,
            updated_at: now,
            name: "Character sheet".into(),
            body: "# Character sheet\n\n## Identity\n\n- Full name:\n- Age:\n".into(),
            starred: true,
        },
        common::entities::NoteTemplate {
            id: 41,
            created_at: now,
            updated_at: now,
            name: "Location".into(),
            body: "# Location\n\nSensory detail:\n".into(),
            starred: false,
        },
    ];
    let smart_punctuation = common::entities::SmartPunctuation {
        id: 30,
        created_at: now,
        updated_at: now,
        override_app_default: true,
        dashes: true,
        ellipsis: true,
        quotes: true,
        quote_style: common::entities::QuoteStyle::Guillemets,
        pre_punctuation_spacing: true,
        dialogue_marker: true,
    };
    let tags = vec![
        // Non-default `details`/`discoverable` on purpose: the folder and zip
        // round-trip tests compare whole bundles, so these two rows are what proves
        // the new fields actually survive a write/read cycle.
        BinderTag {
            id: 10,
            created_at: now,
            updated_at: now,
            name: "Important".into(),
            color: "#f00".into(),
            details: "Needs a second pass before the beta read".into(),
            discoverable: false,
        },
        BinderTag {
            id: 11,
            created_at: now,
            updated_at: now,
            name: "Idea".into(),
            color: "#0f0".into(),
            details: String::new(),
            discoverable: true,
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
    let mut content_id = 1000u64;
    for (i, (role, sub_role)) in all_combinations().into_iter().enumerate() {
        let item_id = 300 + i as u64;
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
            // Distinct per row: this is inside the `for … enumerate()` above, so a
            // single literal would give every item the SAME identity.
            uid: common::uid::fixture_uid(item_id),
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
            is_exportable: true,
            indent: (i % 3) as i64,
            word_count_goal: 1000,
            char_count_goal: 5000,
            dict_language: vec!["en-US".to_string()],
            // One item carries multi-word aliases and the rest carry none, so the
            // round-trip covers both the populated and the empty case. Multi-word is
            // the point of `Vec<String>`: a space-separated string could not hold
            // "Miss Bennet" as one alias.
            aliases: if i == 0 {
                vec!["Lizzy".into(), "Miss Bennet".into()]
            } else {
                Vec::new()
            },
            contents: Vec::new(),
            references: Vec::new(),
            point_of_view: Vec::new(),
            tags: vec![10],
        };
        items.push(ItemWithContents { item, contents });
    }
    // A cross-reference: first item -> second item.
    items[0].item.references = vec![301];
    // ...and a point of view on the same pair, which is a different question
    // (who appears here vs. whose eyes this is told through) travelling the same road.
    items[0].item.point_of_view = vec![301];

    let binders = vec![BinderWithItems {
        binder: Binder {
            uid: common::uid::fixture_uid(2),
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

    SampleInputs {
        work,
        tags,
        dict_words,
        note_templates,
        smart_punctuation,
        trash,
        binders,
    }
}

/// The `SceneText` content id of the fixture's Item/Scene row (`items[2]`) — the
/// same row `writes_are_diff_minimal` edits.
fn scene_text_content_id(binders: &[BinderWithItems]) -> u64 {
    binders[0].items[2]
        .contents
        .iter()
        .find(|c| c.role == ContentRole::SceneText)
        .expect("the Item/Scene fixture row has SceneText")
        .id
}

/// Comment fixtures with **no field left at its default**, deliberately: an
/// all-default row round-trips equal even when a field has been dropped somewhere
/// in the mapping, which is exactly how a persistence bug hides. Same reasoning the
/// `chapter_mode` / `smart_punctuation` fixtures above already record.
///
/// Covers all three shapes that persist differently: an anchored `Range` comment
/// with a reply thread, an anchored `Paragraph` comment, and an **orphan**
/// (`content: None`) which has no sidecar to live in and must survive via the
/// bundle-root orphanage.
fn sample_comments(binders: &[BinderWithItems]) -> Vec<CommentWithReplies> {
    let now = ts();
    let scene = scene_text_content_id(binders);
    vec![
        CommentWithReplies {
            comment: Comment {
                id: 5000,
                created_at: now,
                updated_at: now,
                content: Some(scene),
                kind: CommentAnchorKind::Range,
                author_name: "Jane".into(),
                body: "Is this too on-the-nose?".into(),
                resolved: false,
                orphaned: false,
                orphan_reason: CommentOrphanReason::NotOrphaned,
                range_start: 6,
                range_length: 5,
                quote_prefix: "Scene ".into(),
                quote_exact: "prose".into(),
                quote_exact_truncated: false,
                quote_suffix: " with ".into(),
                block_ordinal_hint: 0,
                replies: vec![5001, 5002],
            },
            replies: vec![
                CommentReply {
                    id: 5001,
                    created_at: now,
                    updated_at: now,
                    author_name: "Jane".into(),
                    body: "Maybe. Sleep on it.".into(),
                },
                CommentReply {
                    id: 5002,
                    created_at: now,
                    updated_at: now,
                    author_name: "Marc".into(),
                    body: "Keep it — it lands.".into(),
                },
            ],
        },
        CommentWithReplies {
            comment: Comment {
                id: 5010,
                created_at: now,
                updated_at: now,
                content: Some(scene),
                kind: CommentAnchorKind::Paragraph,
                author_name: "Jane".into(),
                body: "This whole paragraph drags.".into(),
                resolved: true,
                orphaned: false,
                orphan_reason: CommentOrphanReason::NotOrphaned,
                range_start: 0,
                range_length: 0,
                quote_prefix: String::new(),
                quote_exact: "Scene prose with *emphasis*".into(),
                quote_exact_truncated: true,
                quote_suffix: String::new(),
                block_ordinal_hint: 1,
                replies: vec![],
            },
            replies: vec![],
        },
        CommentWithReplies {
            comment: Comment {
                id: 5020,
                created_at: now,
                updated_at: now,
                // No anchor: the Content this once pointed at is gone. Without the
                // orphanage this row would be silently destroyed by a save.
                content: None,
                kind: CommentAnchorKind::Range,
                author_name: "Marc".into(),
                body: "Whatever this was about, it is gone now.".into(),
                resolved: false,
                orphaned: true,
                orphan_reason: CommentOrphanReason::TargetDeleted,
                range_start: 42,
                range_length: 7,
                quote_prefix: "the ".into(),
                quote_exact: "vanished".into(),
                quote_exact_truncated: false,
                quote_suffix: " line".into(),
                block_ordinal_hint: 3,
                replies: vec![],
            },
            replies: vec![],
        },
    ]
}

fn build_bundle(shape: ShapeTag) -> WorkBundle {
    let s = sample_inputs();
    let comments = sample_comments(&s.binders);
    from_entities(
        &s.work,
        &s.tags,
        &s.dict_words,
        &[],
        &s.note_templates,
        Some(&s.smart_punctuation),
        &s.trash,
        &[],
        &[],
        &comments,
        &s.binders,
        shape,
    )
}

#[test]
fn folder_round_trip_is_lossless() {
    let bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    let read = read_bundle(root.to_str().unwrap()).unwrap();
    assert_round_trip(&bundle, &read);
}

/// Point of view survives a save→load, and stays distinct from `references`.
///
/// `folder_round_trip_is_lossless` covers this too, by whole-bundle equality — but it would
/// report a regression as a diff of two large structures. This one names the property, and
/// in particular pins that the two relationships do not bleed into each other: they travel
/// the same road through the format and answer different questions, so a copy-paste slip in
/// the mapping layer would otherwise show up as "POV works" while silently writing it into
/// the cast list.
#[test]
fn point_of_view_round_trips_and_stays_distinct_from_references() {
    let bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("PovNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    let read = read_bundle(root.to_str().unwrap()).unwrap();

    let written = &bundle.binders[0].items[0].item;
    let loaded = &read.binders[0].items[0].item;
    assert_eq!(written.point_of_view_ids, vec![301], "the fixture sets a POV to begin with");
    assert_eq!(
        loaded.point_of_view_ids, written.point_of_view_ids,
        "point of view must survive the round trip"
    );
    assert_eq!(
        loaded.reference_ids, written.reference_ids,
        "references must survive it independently"
    );

    // An item with no POV must come back with none, not with its cast copied in.
    let unassigned = read.binders[0]
        .items
        .iter()
        .map(|b| &b.item)
        .find(|i| i.file_id != written.file_id)
        .expect("the fixture has more than one item");
    assert!(
        unassigned.point_of_view_ids.is_empty(),
        "an unassigned scene must stay unassigned; got {:?}",
        unassigned.point_of_view_ids
    );
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
    assert_round_trip(&bundle, &read);
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
        is_exportable: true,
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
            uid: common::uid::fixture_uid(1),
            id: 100,
            created_at: now,
            updated_at: now,
            name: "M".into(),
            activated: true,
            binder_items: Vec::new(),
        },
        items: vec![ItemWithContents { item, contents }],
    }];
    let bundle = from_entities(
        &work,
        &[],
        &[],
        &[],
        &[],
        None,
        &[],
        &[],
        &[],
        &[],
        &binders,
        ShapeTag::Folder,
    );
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

// ---------------------------------------------------------------------------
// Comments: per-Content sidecars, the orphanage, and diff-minimality
// ---------------------------------------------------------------------------

#[test]
fn a_comment_lands_in_a_sidecar_beside_the_prose_it_annotates() {
    let bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    // Find the prose blob for the annotated scene, then assert its sidecar sits
    // right next to it — that adjacency is what identifies the owning Content, and
    // is why `Content` needs no `uid`.
    let binders = sample_inputs().binders;
    let scene = scene_text_content_id(&binders);
    let prose: Vec<_> = walkdir::WalkDir::new(&root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("djot"))
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.file_name().unwrap().to_str().unwrap().starts_with(&format!("{scene}-")))
        .collect();
    assert_eq!(prose.len(), 1, "expected exactly one .djot for the scene");

    let sidecar = prose[0].with_extension("").to_string_lossy().to_string() + ".comments.ron";
    let sidecar = std::path::Path::new(&sidecar);
    assert!(
        sidecar.is_file(),
        "expected a comments sidecar at {}",
        sidecar.display()
    );
    let text = fs::read_to_string(sidecar).unwrap();
    assert!(text.contains("Is this too on-the-nose?"));
    assert!(text.contains("Keep it — it lands."), "replies must round-trip");
}

#[test]
fn an_uncommented_project_writes_no_comment_files_at_all() {
    let s = sample_inputs();
    let bundle = from_entities(
        &s.work,
        &s.tags,
        &s.dict_words,
        &[],
        &s.note_templates,
        Some(&s.smart_punctuation),
        &s.trash,
        &[],
        &[],
        &[], // no comments
        &s.binders,
        ShapeTag::Folder,
    );
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    let stray: Vec<String> = walkdir::WalkDir::new(&root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().to_string())
        .filter(|p| p.contains("comments"))
        .collect();
    assert!(
        stray.is_empty(),
        "a project with no comments should grow no comment files, found {stray:?}"
    );
}

#[test]
fn an_orphaned_comment_survives_a_round_trip_via_the_orphanage() {
    let bundle = build_bundle(ShapeTag::Folder);
    assert_eq!(
        bundle.orphan_comments.len(),
        1,
        "the fixture's anchorless comment belongs in the orphanage, not a sidecar"
    );

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    assert!(root.join("orphan_comments.ron").is_file());

    let read = read_bundle(root.to_str().unwrap()).unwrap();
    assert_eq!(read.orphan_comments.len(), 1);
    let o = &read.orphan_comments[0];
    assert_eq!(o.body, "Whatever this was about, it is gone now.");
    assert!(o.orphaned);
    assert_eq!(o.orphan_reason, CommentOrphanReason::TargetDeleted);
    // The quote is the only thing that could ever re-anchor it, so it must survive
    // even though the anchor itself is dead.
    assert_eq!(o.quote_exact, "vanished");
}

#[test]
fn the_orphanage_file_disappears_once_the_last_orphan_is_gone() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    assert!(root.join("orphan_comments.ron").is_file());

    // The writer deletes the orphan. An empty list must remove the file rather than
    // leave `[]` behind, so a project that has never lost an anchor and one that has
    // recovered look identical on disk.
    bundle.orphan_comments.clear();
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    assert!(!root.join("orphan_comments.ron").exists());
}

#[test]
fn deleting_the_last_comment_prunes_its_sidecar() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    let rs = root.to_str().unwrap();
    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();

    let sidecars = |root: &std::path::Path| -> usize {
        walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().to_string_lossy().ends_with(".comments.ron"))
            .count()
    };
    assert_eq!(sidecars(&root), 1);

    for item in &mut bundle.binders[0].items {
        item.comments.clear();
    }
    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    assert_eq!(
        sidecars(&root),
        0,
        "a sidecar whose last comment was deleted must be pruned, not left stale"
    );
}

#[test]
fn editing_one_comment_touches_only_its_own_sidecar() {
    // The comment counterpart of `writes_are_diff_minimal`: this is the property
    // that forced per-Content sidecars instead of one project-wide comments.ron.
    let mut bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    let rs = root.to_str().unwrap();

    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    let before = snapshot(&root);

    // Re-writing an identical bundle must still touch nothing, comments included.
    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    for (rel, (mtime, _)) in &before {
        assert_eq!(*mtime, snapshot(&root)[rel].0, "no-op save rewrote {rel}");
    }

    for item in &mut bundle.binders[0].items {
        for list in item.comments.values_mut() {
            for c in list {
                c.body = "Rewritten note.".into();
            }
        }
    }
    write_bundle(rs, SkribShape::ExplodedFolder, &bundle).unwrap();
    let after = snapshot(&root);

    let changed: Vec<&String> = before
        .keys()
        .filter(|rel| before[*rel].1 != after[*rel].1)
        .collect();
    assert_eq!(
        changed.len(),
        1,
        "editing one comment should rewrite exactly one file, got {changed:?}"
    );
    assert!(
        changed[0].ends_with(".comments.ron"),
        "the changed file should be the sidecar, got {}",
        changed[0]
    );
}

#[test]
fn a_bundle_written_before_comments_existed_still_reads() {
    // Additive-and-optional is what lets this ship without a FORMAT_VERSION bump:
    // deleting every comment artefact must read back as "no comments", not as an error.
    let bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();

    fs::remove_file(root.join("orphan_comments.ron")).unwrap();
    for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        if entry.path().to_string_lossy().ends_with(".comments.ron") {
            fs::remove_file(entry.path()).unwrap();
        }
    }

    let read = read_bundle(root.to_str().unwrap()).unwrap();
    assert!(read.orphan_comments.is_empty());
    assert!(
        read.binders[0].items.iter().all(|i| i.comments.is_empty()),
        "a pre-feature bundle must load with zero comments rather than failing"
    );
}

// ---------------------------------------------------------------------------
// T2-1: durable persist (fsync before rename)
// ---------------------------------------------------------------------------

#[test]
fn persist_durably_writes_a_readable_fsynced_file() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("durable.bin");
    let mut tmp = tempfile::NamedTempFile::new_in(dir.path()).unwrap();
    {
        use std::io::Write;
        tmp.write_all(b"durable payload").unwrap();
    }
    super::writer::persist_durably(tmp, &target).unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"durable payload");
}

#[test]
fn write_zip_round_trips_through_the_durable_persist_path() {
    // write_zip now writes through the NamedTempFile's own fd and fsyncs
    // before/after rename (persist_durably) rather than a second independent
    // File::create handle on the same path; the round trip must still be
    // lossless.
    let bundle = build_bundle(ShapeTag::Zip);
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("Durable.skrib");
    write_bundle(target.to_str().unwrap(), SkribShape::ZipFile, &bundle).unwrap();
    assert!(target.exists());
    let read = read_bundle(target.to_str().unwrap()).unwrap();
    assert_round_trip(&bundle, &read);
}

// ---------------------------------------------------------------------------
// T2-2: verify_backup_at
// ---------------------------------------------------------------------------

#[test]
fn verify_backup_at_accepts_real_backup_and_rejects_the_rest() {
    let mut bundle = build_bundle(ShapeTag::Zip);
    let dir = tempfile::tempdir().unwrap();

    // A plain Regular save must be rejected (not a backup at all).
    let regular_path = dir.path().join("Regular.skrib");
    write_bundle(regular_path.to_str().unwrap(), SkribShape::ZipFile, &bundle).unwrap();
    assert!(verify_backup_at(regular_path.to_str().unwrap(), "").is_err());

    // Mark + rewrite as a real backup: must be accepted, with no expected
    // unique_id and with the correct one.
    let backup_path = dir.path().join("Regular-20260101-120000.skrib");
    mark_as_backup(
        &mut bundle,
        regular_path.to_string_lossy().into_owned(),
        ts(),
    );
    write_bundle(backup_path.to_str().unwrap(), SkribShape::ZipFile, &bundle).unwrap();
    verify_backup_at(backup_path.to_str().unwrap(), "").unwrap();
    verify_backup_at(backup_path.to_str().unwrap(), "test-unique-id-abc").unwrap();

    // Wrong expected unique_id must be rejected.
    assert!(verify_backup_at(backup_path.to_str().unwrap(), "some-other-project").is_err());

    // Garbage (unparseable, not even a real zip) must be rejected.
    let garbage_path = dir.path().join("garbage.skrib");
    fs::write(&garbage_path, b"not a skrib file at all").unwrap();
    assert!(verify_backup_at(garbage_path.to_str().unwrap(), "").is_err());
}

// ---------------------------------------------------------------------------
// T2-5: filename-fallback sniff must not hijack a legacy project
// ---------------------------------------------------------------------------

#[test]
fn sniff_backup_filename_fallback_requires_the_original_to_exist() {
    let bundle = build_bundle(ShapeTag::Zip);
    let dir = tempfile::tempdir().unwrap();

    // Stamped-looking name, unreadable "manifest" (garbage bytes), but its
    // guessed original ("ghost.skrib") does NOT exist on disk -> must NOT be
    // treated as a backup (the legacy-project-genuinely-named-like-a-backup
    // case).
    let orphan_stamp_path = dir.path().join("ghost-20260101-120000.skrib");
    fs::write(&orphan_stamp_path, b"not a parseable skrib file").unwrap();
    let sniff = sniff_backup(orphan_stamp_path.to_str().unwrap());
    assert!(
        !sniff.is_backup,
        "a stamped file whose guessed original doesn't exist must not be a backup"
    );

    // Same unreadable file, but now its guessed original DOES exist on disk
    // -> accepted as a non-authoritative backup guess.
    let original_path = dir.path().join("ghost.skrib");
    write_bundle(
        original_path.to_str().unwrap(),
        SkribShape::ZipFile,
        &bundle,
    )
    .unwrap();
    let sniff2 = sniff_backup(orphan_stamp_path.to_str().unwrap());
    assert!(sniff2.is_backup);
    assert!(!sniff2.authoritative);
    assert_eq!(
        sniff2.backup_of.as_deref(),
        Some(original_path.to_str().unwrap())
    );

    // A readable `Regular` manifest always wins, even when the filename also
    // matches the stamp pattern.
    let regular_stamped_path = dir.path().join("mynovel-20260101-120000.skrib");
    write_bundle(
        regular_stamped_path.to_str().unwrap(),
        SkribShape::ZipFile,
        &bundle,
    )
    .unwrap();
    let sniff3 = sniff_backup(regular_stamped_path.to_str().unwrap());
    assert!(!sniff3.is_backup);
    assert!(sniff3.authoritative);
}

// ---------------------------------------------------------------------------
// T2-7: mark_existing_as_backup
// ---------------------------------------------------------------------------

#[test]
fn mark_existing_as_backup_turns_a_regular_bundle_into_a_backup_in_place() {
    let bundle = build_bundle(ShapeTag::Zip);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Project.skrib");
    write_bundle(path.to_str().unwrap(), SkribShape::ZipFile, &bundle).unwrap();

    let before = peek_manifest(path.to_str().unwrap()).unwrap();
    assert_eq!(before.kind, BundleKind::Regular);

    mark_existing_as_backup(path.to_str().unwrap(), "/orig/Project.skrib", ts()).unwrap();

    let after = peek_manifest(path.to_str().unwrap()).unwrap();
    assert_eq!(after.kind, BundleKind::Backup);
    assert_eq!(after.backup_of.as_deref(), Some("/orig/Project.skrib"));
    // The work's identity must be preserved, not reset.
    assert_eq!(after.work.unique_id, "test-unique-id-abc");

    // Content preserved (shape + data round trip through read/write).
    let reread = read_bundle(path.to_str().unwrap()).unwrap();
    assert_eq!(reread.binders.len(), bundle.binders.len());
    assert_eq!(reread.manifest.work.title, bundle.manifest.work.title);
    assert_eq!(reread.tags, bundle.tags);
}

// ── uid: durable per-row identity (format v3) ───────────────────────────────

#[test]
fn uids_survive_a_write_read_round_trip_unchanged() {
    // The whole point: unlike `file_id` (the store id at save time, re-minted on
    // the next save), a uid written to disk must come back identical.
    let bundle = build_bundle(ShapeTag::Folder);
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("MyNovel");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    let read = read_bundle(root.to_str().unwrap()).unwrap();

    let before: Vec<uuid::Uuid> = bundle
        .binders
        .iter()
        .flat_map(|b| std::iter::once(b.binder.uid).chain(b.items.iter().map(|i| i.item.uid)))
        .collect();
    let after: Vec<uuid::Uuid> = read
        .binders
        .iter()
        .flat_map(|b| std::iter::once(b.binder.uid).chain(b.items.iter().map(|i| i.item.uid)))
        .collect();
    assert!(!before.is_empty(), "fixture must carry uids");
    assert_eq!(before, after, "every uid must round-trip byte-identical");
}

#[test]
fn migrating_a_pre_v3_bundle_mints_a_uid_for_every_row() {
    // A v2 bundle has no uids at all. `migrate_bundle` must fill in every row --
    // one missed row is an empty key that collides with every other empty key.
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = 2;
    for b in &mut bundle.binders {
        b.binder.uid = uuid::Uuid::nil();
        for i in &mut b.items {
            i.item.uid = uuid::Uuid::nil();
        }
    }

    migration::migrate_bundle(&mut bundle).unwrap();

    assert_eq!(bundle.manifest.format_version, FORMAT_VERSION);
    let mut seen = std::collections::HashSet::new();
    for b in &bundle.binders {
        assert!(!b.binder.uid.is_nil(), "binder left without a uid");
        assert!(seen.insert(b.binder.uid), "duplicate uid minted");
        for i in &b.items {
            assert!(!i.item.uid.is_nil(), "item left without a uid");
            assert!(seen.insert(i.item.uid), "duplicate uid minted");
        }
    }
}

#[test]
fn migration_is_idempotent_and_never_re_mints_an_existing_uid() {
    // Re-running the step (or meeting a partially-migrated bundle) must not
    // change identities that already exist -- that would orphan every
    // reference to them.
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = 2;
    // Only the FIRST item loses its uid: the rest must survive untouched.
    let kept: Vec<uuid::Uuid> = bundle.binders[0].items.iter().map(|i| i.item.uid).collect();
    bundle.binders[0].items[0].item.uid = uuid::Uuid::nil();

    migration::migrate_bundle(&mut bundle).unwrap();

    let after: Vec<uuid::Uuid> = bundle.binders[0].items.iter().map(|i| i.item.uid).collect();
    assert!(!after[0].is_nil(), "the nil one was filled");
    assert_ne!(after[0], kept[0], "…with a fresh value");
    assert_eq!(
        after[1..],
        kept[1..],
        "every already-identified row must keep its uid"
    );
}

/// A bundle needing a format newer than ours must be refused, not silently downgraded.
///
/// **Retargeted from `migrate_bundle` to the gate**, deliberately. The refusal moved:
/// `migrate_bundle` ran after the whole bundle was parsed, which made it unreachable for
/// the only forward-incompatible change this project actually makes (a new enum variant
/// blows up in `ron::from_str` several frames earlier), and it compared the raw writer
/// stamp rather than the content floor — so it would have refused exactly the files the
/// floor scheme exists to keep openable. The coverage is kept; only its target changed.
#[test]
fn a_bundle_from_a_newer_format_is_refused_not_silently_migrated() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("FromTheFuture");
    let bundle = build_bundle(ShapeTag::Folder);
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    set_manifest_floor(&root.join("project.skrib"), FORMAT_VERSION + 1);

    let err = read_bundle(root.to_str().unwrap()).expect_err("a newer .skrib must be refused");
    assert!(
        matches!(err, SkribFormatError::TooNew { .. }),
        "expected TooNew, got: {err:?}"
    );
}

/// Rewrite an already-written manifest's read floor to `floor`, in place.
///
/// A too-new bundle **cannot** be produced through `write_bundle` — the writer computes
/// the floor from content, so by construction it never emits a file it could not read
/// back. That is the right property, and it means every "from the future" test has to
/// doctor the manifest afterwards. Textual rather than parse-edit-reserialize, because
/// several tests below deliberately hand the reader a manifest it *cannot* fully parse,
/// and going through `ProjectManifest` would defeat them.
fn set_manifest_floor(manifest_path: &Path, floor: u32) {
    let text = fs::read_to_string(manifest_path).unwrap();
    let start = text
        .find("format_min_read_version:")
        .expect("the writer must have stamped a floor");
    let end = start
        + text[start..]
            .find(',')
            .expect("the field must be comma-terminated");
    let replaced = format!(
        "{}format_min_read_version: Some({floor}){}",
        &text[..start],
        &text[end..]
    );
    fs::write(manifest_path, replaced).unwrap();
}

#[test]
fn migrating_a_v1_bundle_walks_the_whole_chain() {
    // The v1 path is the one with non-obvious control flow: two loop
    // iterations, v1→v2 then v2→v3. Only v2 was covered, so a regression that
    // broke the oldest files -- the ones most likely still in the wild --
    // would have passed.
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = 1;
    for b in &mut bundle.binders {
        b.binder.uid = uuid::Uuid::nil();
        for i in &mut b.items {
            i.item.uid = uuid::Uuid::nil();
        }
    }

    migration::migrate_bundle(&mut bundle).unwrap();

    assert_eq!(bundle.manifest.format_version, FORMAT_VERSION);
    for b in &bundle.binders {
        assert!(!b.binder.uid.is_nil(), "v1 binder left unidentified");
        for i in &b.items {
            assert!(!i.item.uid.is_nil(), "v1 item left unidentified");
        }
    }
}

/// A `work.ron` written before `author_name` existed must still parse.
///
/// `author_name` was added to `WorkFile` without `#[serde(default)]`, unlike every
/// other additive field on that struct — which made it *required*, so every project
/// saved before it landed became unopenable. Nothing caught it because the four
/// hand-written "this is what an older build emitted" fixtures below had each been
/// given an `author_name:` line an old file could not possibly contain.
///
/// The failure is a parse error, not a migration gap: serde runs before
/// `migration`, so a manifest that will not deserialize never reaches a step that
/// could heal it. That is why the fix is `default`, not a version bump.
#[test]
fn a_work_written_before_author_name_existed_still_parses() {
    let old = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "Old Novel",
        dict_language: ["fr-FR"],
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "abc",
    )"#;
    let w: WorkFile = ron::from_str(old).expect("a work.ron without author_name must still parse");
    assert_eq!(w.title, "Old Novel");
    // Absent means unset, which is a legal state — the field is optional.
    assert_eq!(w.author_name, "");
}

/// A `work.ron` written before the punctuation house style existed must still parse.
///
/// Same hazard as `author_name` above, and the same one-line guard: without
/// `#[serde(default)]` on `WorkFile.smart_punctuation`, every project saved before
/// this feature landed would fail to deserialize — before `migration` ever runs, so
/// no step could heal it.
#[test]
fn a_work_written_before_smart_punctuation_existed_still_parses() {
    let old = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "Old Novel",
        author_name: "Jane",
        dict_language: ["fr-FR"],
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "abc",
    )"#;
    let w: WorkFile =
        ron::from_str(old).expect("a work.ron without smart_punctuation must still parse");
    assert_eq!(w.title, "Old Novel");
    // `None`, and deliberately not an all-false row: absent means "this project
    // was never asked", which the loader turns into "follow the app default".
    // An all-false row would instead mean "the writer switched everything off".
    assert!(w.smart_punctuation.is_none());
}

/// An unknown quote style degrades to the locale default rather than failing.
///
/// The style is written as a string precisely so a bundle from a build that knows
/// a style this one does not stays readable. Reading it back as `LocaleDefault` is
/// the honest fallback: it is what the locale would have picked anyway.
#[test]
fn an_unrecognised_quote_style_falls_back_instead_of_failing() {
    let future = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "From The Future",
        author_name: "Jane",
        dict_language: ["en-US"],
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "abc",
        smart_punctuation: Some(SmartPunctuationFile(
            created_at: "2023-11-14T22:13:20Z",
            updated_at: "2023-11-14T22:13:20Z",
            override_app_default: true,
            dashes: true,
            ellipsis: true,
            quotes: true,
            quote_style: "corner_brackets",
            pre_punctuation_spacing: false,
            dialogue_marker: false,
        )),
    )"#;
    let w: WorkFile = ron::from_str(future).expect("an unknown quote style must not break parsing");
    let sp = w.smart_punctuation.expect("the row is present");
    assert_eq!(sp.quote_style, "corner_brackets", "stored verbatim");
    // The rest of the row still round-trips — one unknown value must not
    // discard the settings around it.
    assert!(sp.override_app_default);
    assert!(sp.dashes);
}

/// A bundle written before `details`/`discoverable`/`aliases` existed must still parse.
///
/// The round-trip tests above always write with the current code, so they can never
/// exercise the `#[serde(default)]` attributes those three fields carry — a dropped
/// `default` would sail through them and only fail on a real pre-existing project. The
/// RON below is hand-written to match exactly what an older build emitted: `struct_names`
/// on, `BinderTagFile.text_color` present (now unknown, and silently ignored because this
/// crate sets `deny_unknown_fields` nowhere), and the three new fields absent.
#[test]
fn parses_a_bundle_written_before_these_fields_existed() {
    // `r##"…"##`: the colour literals contain `"#`, which would close an `r#"…"#`.
    let old_tags = r##"[
        BinderTagFile(
            file_id: 10,
            created_at: "2023-11-14T22:13:20Z",
            updated_at: "2023-11-14T22:13:20Z",
            name: "Important",
            color: "#f00",
            text_color: "#fff",
        ),
    ]"##;
    let tags: Vec<BinderTagFile> = ron::from_str(old_tags).expect("old tags.ron must parse");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "Important");
    assert_eq!(tags[0].color, "#f00");
    // Defaulted, not carried over from the dropped `text_color`.
    assert_eq!(tags[0].details, "");
    assert!(!tags[0].discoverable);

    let old_items = r##"[
        BinderItemFile(
            file_id: 300,
            created_at: "2023-11-14T22:13:20Z",
            updated_at: "2023-11-14T22:13:20Z",
            title: "The ferry",
            sub_title: "",
            role: Item,
            sub_role: Scene,
            label: "",
            activated: true,
            is_favorite: false,
            is_exportable: true,
            indent: 0,
            word_count_goal: 0,
            char_count_goal: 0,
            dict_language: "en-US",
            inline_contents: [],
            prose_refs: [],
            reference_ids: [301],
            tag_ids: [10],
        ),
    ]"##;
    let items: Vec<BinderItemFile> = ron::from_str(old_items).expect("old items.ron must parse");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "The ferry");
    // The pre-existing relationships must survive untouched...
    assert_eq!(items[0].reference_ids, vec![301]);
    assert_eq!(items[0].tag_ids, vec![10]);
    // ...and the new fields default rather than failing the parse. This is what
    // justifies leaving FORMAT_VERSION alone for `point_of_view_ids`: an empty POV
    // is an ordinary, legal state (an unassigned scene), so `default` reads a v4
    // file back correctly. Contrast `uid`, where nil was never valid and the
    // addition therefore needed both a version bump and a heal step.
    assert!(items[0].aliases.is_empty());
    assert!(items[0].point_of_view_ids.is_empty());
}

// ── dict_language: the pre-v4 string form (format v4) ───────────────────────

/// A project written before `dict_language` became a list must still open.
///
/// This is the one guarantee standing between the change and every existing project being
/// unopenable, and it cannot be covered by the round-trip tests above: those always write
/// with the current code, so they only ever produce the list form. A type change also cannot
/// be handled by `migration`, which runs *after* serde — a v3 file would fail to parse long
/// before reaching it. So the tolerance lives in the deserializer, and this pins it with RON
/// hand-written to match exactly what an older build emitted.
#[test]
fn a_pre_v4_space_separated_language_still_parses_as_a_list() {
    let old = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "Old Novel",
        dict_language: "fr-FR en-US",
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "abc",
    )"#;
    let w: WorkFile = ron::from_str(old).expect("a pre-v4 work must still parse");
    assert_eq!(
        w.dict_language,
        vec!["fr-FR".to_string(), "en-US".to_string()],
        "the space-separated grammar is split into the list it always meant"
    );
}

/// A single tag — by far the common case — becomes a one-element list, not one element
/// containing a space-padded string.
#[test]
fn a_pre_v4_single_language_becomes_one_element() {
    let old = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "T",
        dict_language: "  fr-FR  ",
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "",
    )"#;
    let w: WorkFile = ron::from_str(old).expect("must parse");
    assert_eq!(w.dict_language, vec!["fr-FR".to_string()]);
}

/// An untagged pre-v4 project yields no tags at all, rather than one empty string — which
/// would make `primary` return "" while looking like a real entry.
#[test]
fn a_pre_v4_empty_language_yields_no_tags() {
    let old = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "T",
        dict_language: "",
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "",
    )"#;
    let w: WorkFile = ron::from_str(old).expect("must parse");
    assert!(w.dict_language.is_empty());
}

/// …and the current form round-trips as itself.
#[test]
fn the_v4_list_form_parses_unchanged() {
    let current = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "T",
        author_name: "A",
        dict_language: ["fr-FR", "en-US"],
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "",
    )"#;
    let w: WorkFile = ron::from_str(current).expect("must parse");
    assert_eq!(
        w.dict_language,
        vec!["fr-FR".to_string(), "en-US".to_string()]
    );
}

/// The shipped fixture — a real project written long before this change — opens through the
/// real read path, and its pre-v4 language survives as a list.
///
/// `read_bundle` migrates internally (reader.rs), so this asserts the *post-migration* state:
/// an old project arrives fully current. An earlier version of this test called
/// `migrate_bundle` afterwards and asserted "before migration", which was never true and
/// would have passed even if the split had been moved into the migration step — breaking
/// `peek_manifest`, which does not migrate. That path is covered separately below.
///
/// Note what this deliberately does **not** assert: the fixture's on-disk `format_version`.
/// That number moves whenever the app legitimately re-saves the project (it went 2 → 3
/// exactly that way), and a test pinning it fails for a reason that has nothing to do with
/// what it checks. What matters is that whatever version ships, it opens and arrives current.
const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../resources/test/skribisto_test_project.skrib"
);

#[test]
fn the_shipped_fixture_opens_and_keeps_its_language() {
    let on_disk = peek_manifest(FIXTURE).expect("the fixture must have a readable manifest");
    assert!(
        on_disk.format_version < FORMAT_VERSION,
        "this fixture earns its keep by being OLD — at v{} it no longer exercises migration, \
         so either keep an older copy or retire this test",
        on_disk.format_version
    );

    let bundle = read_bundle(FIXTURE).expect("an old project must still open");
    assert_eq!(
        bundle.manifest.format_version, FORMAT_VERSION,
        "read_bundle migrates, so what comes back is current"
    );
    assert_eq!(
        bundle.manifest.work.dict_language,
        vec!["fr".to_string()],
        "the pre-v4 string \"fr\" arrives as a one-element list"
    );
}

/// …and `peek_manifest`, which deliberately does **not** migrate, still gets a usable list.
///
/// This is the path the backup sniff and the retention scan take over many files. If the
/// string-to-list split ever moved out of the deserializer and into `migrate_bundle`, this is
/// the test that would fail — the one above would not.
///
/// It writes its own manifest rather than reading the shipped project, because the property
/// under test is "peek does not migrate", and borrowing a shared file to check that couples
/// the test to a version number that legitimately changes underneath it.
#[test]
fn peek_manifest_parses_a_pre_v4_language_without_migrating() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("Old Novel");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join(crate::shape::MANIFEST_NAME),
        r#"ProjectManifest(
    format_version: 2,
    shape: Folder,
    work: WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "T",
        dict_language: "fr-FR en-US",
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "",
    ),
    binder_order: [],
)"#,
    )
    .unwrap();

    let manifest = peek_manifest(root.to_str().unwrap()).expect("must parse unmigrated");
    assert_eq!(
        manifest.format_version, 2,
        "peek must report what is on disk, not bump it"
    );
    assert_eq!(
        manifest.work.dict_language,
        vec!["fr-FR".to_string(), "en-US".to_string()],
        "the split happens at parse time, so a caller that never migrates still gets a list"
    );
}

/// A malformed value says what was expected instead of naming an internal enum.
///
/// The exploded-folder shape is meant to be hand-edited and diffed, so a typo there has to
/// be legible. `#[serde(untagged)]` reported every such value as "data did not match any
/// variant of untagged enum Either", which tells the writer nothing.
#[test]
fn a_malformed_language_names_what_was_expected() {
    let bad = r#"WorkFile(
        file_id: 1,
        created_at: "2023-11-14T22:13:20Z",
        updated_at: "2023-11-14T22:13:20Z",
        title: "T",
        author_name: "A",
        dict_language: 42,
        tag_ids: [],
        dict_word_ids: [],
        unique_id: "",
    )"#;
    let err = ron::from_str::<WorkFile>(bad).expect_err("42 is not a language");
    let msg = err.to_string();
    assert!(
        msg.contains("language tags"),
        "the error should say what was expected, got: {msg}"
    );
    assert!(
        !msg.contains("untagged"),
        "and should not leak an internal enum, got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// Note templates
// ---------------------------------------------------------------------------

/// The bodies live in `templates/`, not inline in `templates.ron` — the split that makes
/// a template edit diff as a prose change in an exploded project.
#[test]
fn template_bodies_are_written_as_sibling_djot_blobs() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("p");
    write_bundle(
        root.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    let index = std::fs::read_to_string(root.join("templates.ron")).unwrap();
    assert!(
        index.contains("Character sheet"),
        "the index carries the name"
    );
    assert!(
        !index.contains("## Identity"),
        "but never the body — that belongs in the blob, got:\n{index}"
    );

    let blob = root.join("templates/40-character-sheet.djot");
    assert!(blob.is_file(), "expected a blob at {}", blob.display());
    assert!(
        std::fs::read_to_string(&blob)
            .unwrap()
            .contains("## Identity"),
        "the body is the blob's content"
    );
}

/// A rename changes the slug and therefore the blob's filename. Without the prune in
/// `write_folder` the old file would survive forever, so this is the regression guard
/// for that specific omission.
#[test]
fn renaming_a_template_prunes_its_old_blob() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("p");
    let path = root.to_str().unwrap();

    let mut bundle = build_bundle(ShapeTag::Folder);
    write_bundle(path, SkribShape::ExplodedFolder, &bundle).unwrap();
    assert!(root.join("templates/40-character-sheet.djot").is_file());

    bundle.note_templates[0].name = "Dramatis persona".into();
    bundle.note_templates[0].path = crate::slug::note_template_relpath(40, "Dramatis persona");
    write_bundle(path, SkribShape::ExplodedFolder, &bundle).unwrap();

    assert!(
        !root.join("templates/40-character-sheet.djot").exists(),
        "the pre-rename blob must be pruned, not orphaned"
    );
    assert!(root.join("templates/40-dramatis-persona.djot").is_file());
}

/// Deleting the last template leaves no orphan blobs behind.
#[test]
fn deleting_templates_prunes_every_blob() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("p");
    let path = root.to_str().unwrap();

    let mut bundle = build_bundle(ShapeTag::Folder);
    write_bundle(path, SkribShape::ExplodedFolder, &bundle).unwrap();

    bundle.note_templates.clear();
    bundle.note_template_bodies.clear();
    write_bundle(path, SkribShape::ExplodedFolder, &bundle).unwrap();

    let leftovers: Vec<_> = std::fs::read_dir(root.join("templates"))
        .map(|d| {
            d.flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    assert!(
        leftovers.is_empty(),
        "expected no orphan blobs, found {leftovers:?}"
    );
}

/// A name that is not a legal file name still lands on one safe path segment — the
/// export/import surfaces take user-typed names, so this must not be able to escape.
#[test]
fn a_hostile_template_name_still_yields_one_safe_path_segment() {
    for hostile in ["../../etc/passwd", "CON", "a/b\\c", "  ..  ", "Fiche/perso"] {
        let rel = crate::slug::note_template_relpath(7, hostile);
        assert_eq!(
            std::path::Path::new(&rel).components().count(),
            2,
            "'{hostile}' must stay `templates/<one-segment>`, got '{rel}'"
        );
        assert!(rel.starts_with("templates/"), "got '{rel}'");
        assert!(!rel.contains(".."), "got '{rel}'");
    }
}

/// A project with no `templates.ron` at all (every bundle written before v5) loads as
/// zero templates rather than failing — the additive half of the format change.
#[test]
fn a_bundle_without_templates_ron_loads_as_empty() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("p");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    std::fs::remove_file(root.join("templates.ron")).unwrap();
    std::fs::remove_dir_all(root.join("templates")).unwrap();

    let reread = read_bundle(path).expect("a pre-v5 project must still open");
    assert!(reread.note_templates.is_empty());
}

/// A listed template whose blob has gone missing is a **hard error**, never a silently
/// empty body. Degrading quietly would let autosave rewrite `templates.ron` from that
/// empty state seconds later and destroy the text for good.
#[test]
fn a_missing_template_blob_fails_the_load_rather_than_emptying_it() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("p");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    std::fs::remove_file(root.join("templates/40-character-sheet.djot")).unwrap();

    let err = read_bundle(path).expect_err("a missing body blob must not load as empty");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("40-character-sheet.djot"),
        "the error should name the missing blob, got: {msg}"
    );
}

/// The v5 bump exists so an older build refuses the file instead of silently dropping
/// its templates on the next save. Guard the refusal itself.
///
/// Retargeted at the gate for the reasons on
/// `a_bundle_from_a_newer_format_is_refused_not_silently_migrated`, and rephrased in the
/// terms the gate actually judges: a template-bearing bundle's *floor* is what makes it
/// unopenable by a pre-v5 build, so that is what this asserts.
#[test]
fn a_bundle_from_a_newer_format_is_refused() {
    // Isolated to the template axis: the fixture also carries epigraphs, which would
    // hold the floor at 6 and stop this asserting the v5 case it is named for.
    let mut bundle = build_bundle(ShapeTag::Folder);
    strip_epigraphs(&mut bundle);
    assert!(
        !bundle.note_templates.is_empty(),
        "the fixture must carry templates for this to be the v5 case"
    );
    assert_eq!(
        crate::version_gate::compute_min_read_version(&bundle),
        5,
        "templates are what raise the floor to 5"
    );

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("HasTemplates");
    write_bundle(root.to_str().unwrap(), SkribShape::ExplodedFolder, &bundle).unwrap();
    // Stand in for a build older than the floor by raising the floor above ours instead
    // — the comparison the gate makes is identical either way.
    set_manifest_floor(&root.join("project.skrib"), FORMAT_VERSION + 1);

    let err = read_bundle(root.to_str().unwrap()).expect_err("newer must be refused");
    assert!(
        matches!(err, SkribFormatError::TooNew { requires_at_least, supported, .. }
                 if requires_at_least == FORMAT_VERSION + 1 && supported == FORMAT_VERSION),
        "got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// The pre-flight version gate (`version_gate`)
// ---------------------------------------------------------------------------

/// The gate must fire **before** the binder manifests are parsed.
///
/// This is the whole bug: `migrate_bundle` owned the refusal and ran last, so a bundle
/// from a future format — which in practice means one carrying an enum variant we do not
/// know — died in `ron::from_str` on `items.ron` with a raw `Unexpected variant named
/// "…"`, several call frames before any version was compared. Simulated here with an
/// `items.ron` that cannot parse at all: if the ordering ever regresses, this reports a
/// RON error instead of `TooNew`.
#[test]
fn the_gate_refuses_a_too_new_folder_bundle_without_parsing_its_binders() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("FromTheFuture");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    set_manifest_floor(&root.join("project.skrib"), FORMAT_VERSION + 1);
    for entry in fs::read_dir(root.join("binders")).unwrap() {
        fs::write(
            entry.unwrap().path().join("items.ron"),
            "ItemsFile(binder: NotAThing(sub_role: EpigraphFromTheFuture))",
        )
        .unwrap();
    }

    let err = read_bundle(path).expect_err("a too-new bundle must be refused");
    assert!(
        matches!(err, SkribFormatError::TooNew { .. }),
        "expected the version refusal to win over the parse error, got: {err:?}"
    );
}

/// Same for the zip shape, and one step stronger: the archive here **cannot be
/// extracted**, so reaching `TooNew` proves the gate never called `extract`.
///
/// That matters beyond ordering. `read_zip` unpacks the entire archive — every `.djot`
/// blob — into a tempdir before a single field is read, so gating afterwards means paying
/// the full cost of a read that was always going to be refused. The gate's zip arm
/// streams only the `project.skrib` entry via `by_name`.
#[test]
fn the_gate_refuses_a_too_new_zip_without_extracting_it() {
    use std::io::Write as _;

    let dir = tempfile::tempdir().unwrap();

    // Start from a manifest the writer really produced, then raise its floor — so this
    // exercises the actual on-disk spelling rather than a hand-written approximation.
    let staging = dir.path().join("staging");
    write_bundle(
        staging.to_str().unwrap(),
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Zip),
    )
    .unwrap();
    set_manifest_floor(&staging.join("project.skrib"), FORMAT_VERSION + 3);
    let manifest_text = fs::read_to_string(staging.join("project.skrib")).unwrap();

    // Hand-build an archive whose one non-manifest entry is *stored* (uncompressed) and
    // then overwritten in place with the same number of bytes: the zip stays structurally
    // valid and `by_name` still works, but the recorded CRC32 no longer matches, so any
    // attempt to extract fails.
    let target = dir.path().join("FromTheFuture.skrib");
    let payload = b"AAAAAAAAAAAAAAAA";
    {
        let f = fs::File::create(&target).unwrap();
        let mut zw = zip::ZipWriter::new(f);
        let stored = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zw.start_file("project.skrib", stored).unwrap();
        zw.write_all(manifest_text.as_bytes()).unwrap();
        zw.start_file("binders/00-b/items.ron", stored).unwrap();
        zw.write_all(payload).unwrap();
        zw.finish().unwrap();
    }
    let mut bytes = fs::read(&target).unwrap();
    let at = bytes
        .windows(payload.len())
        .position(|w| w == payload)
        .expect("the stored payload must be findable verbatim");
    bytes[at..at + payload.len()].fill(b'B');
    fs::write(&target, &bytes).unwrap();

    // Control: extraction really is fatal for this fixture, so the assertion below is
    // about the gate's behaviour and not about a lenient reader.
    assert!(
        zip::ZipArchive::new(fs::File::open(&target).unwrap())
            .unwrap()
            .extract(dir.path().join("control"))
            .is_err(),
        "the fixture must be an archive that cannot be extracted"
    );

    let err = read_bundle(target.to_str().unwrap()).expect_err("a too-new zip must be refused");
    assert!(
        matches!(err, SkribFormatError::TooNew { requires_at_least, .. }
                 if requires_at_least == FORMAT_VERSION + 3),
        "expected TooNew without extracting, got: {err:?}"
    );
}

/// The direct regression test for the ceiling deletion.
///
/// A newer build that saves a project containing nothing new stamps `format_version`
/// above ours but a floor we understand. That file must open. While `migrate_bundle`
/// kept its own `v > FORMAT_VERSION` check it would have refused exactly this file —
/// *after* the gate admitted it and the whole bundle was parsed.
#[test]
fn a_floor_we_understand_opens_even_when_format_version_is_ahead() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("NewerWriterOldContent");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    let manifest_path = root.join("project.skrib");
    let text = fs::read_to_string(&manifest_path).unwrap();
    fs::write(
        &manifest_path,
        text.replacen(
            &format!("format_version: {FORMAT_VERSION}"),
            &format!("format_version: {}", FORMAT_VERSION + 1),
            1,
        ),
    )
    .unwrap();
    // Floor stays at what the writer computed, i.e. something this build implements.

    let bundle = read_bundle(path).expect("a floor we understand must open");
    assert_eq!(
        bundle.manifest.format_version,
        FORMAT_VERSION + 1,
        "migration must leave a from-the-future stamp alone rather than downgrade it"
    );
}

/// Every `.skrib` in a real user's hands predates this field. Absent → fall back to
/// `format_version`, which is precisely the refuse-if-greater rule the crate always had.
#[test]
fn an_absent_floor_falls_back_to_format_version() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("Legacyish");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    // Strip the field entirely, exactly as a pre-scheme manifest has it.
    let manifest_path = root.join("project.skrib");
    let text = fs::read_to_string(&manifest_path).unwrap();
    let start = text.find("format_min_read_version:").unwrap();
    let end = start + text[start..].find(',').unwrap() + 1;
    fs::write(
        &manifest_path,
        format!("{}{}", &text[..start], &text[end..].trim_start()),
    )
    .unwrap();
    assert!(!fs::read_to_string(&manifest_path)
        .unwrap()
        .contains("format_min_read_version"));

    read_bundle(path).expect("a manifest without the field must open exactly as before");
}

/// `format_version: 0` is refused before anything else is parsed.
#[test]
fn format_version_zero_is_refused_pre_parse() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("Bogus");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    let manifest_path = root.join("project.skrib");
    let text = fs::read_to_string(&manifest_path).unwrap();
    fs::write(
        &manifest_path,
        text.replacen(&format!("format_version: {FORMAT_VERSION}"), "format_version: 0", 1),
    )
    .unwrap();
    for entry in fs::read_dir(root.join("binders")).unwrap() {
        fs::write(entry.unwrap().path().join("items.ron"), "not ron at all").unwrap();
    }

    let err = read_bundle(path).expect_err("format_version 0 must be refused");
    assert!(
        matches!(err, SkribFormatError::InvalidVersion),
        "expected InvalidVersion before any binder parsing, got: {err:?}"
    );
}

/// The probe must survive a manifest from an arbitrarily distant future — and this is the
/// test that would have caught reusing `peek_manifest` for it.
///
/// `peek_manifest` parses the whole `ProjectManifest`, including `shape: ShapeTag` and
/// `kind: BundleKind` — plain derived enums with no `#[serde(other)]`. A future third
/// `ShapeTag` would hard-fail *the probe itself*, reopening the very bug the gate closes,
/// one level up. The control assertion pins that difference rather than describing it.
#[test]
fn the_probe_survives_an_unrecognised_shape_tag_variant() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("FutureShape");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    let manifest_path = root.join("project.skrib");
    let text = fs::read_to_string(&manifest_path).unwrap();
    let doctored = text.replacen("shape: Folder", "shape: HolographicCrystal", 1);
    assert_ne!(doctored, text, "the shape field must have been rewritten");
    fs::write(&manifest_path, &doctored).unwrap();

    // Control: the full-manifest reader cannot cope with it…
    assert!(
        peek_manifest(path).is_err(),
        "peek_manifest must choke on an unknown ShapeTag — that is why it is not the probe"
    );
    // …while the gate reads its two integers regardless. (The full read still fails
    // afterwards, as it must; what matters is that the *gate* got its answer.)
    assert!(
        crate::version_gate::check_version_gate(path, SkribShape::ExplodedFolder).is_ok(),
        "the narrow probe must be immune to unknown enum values elsewhere in the manifest"
    );
}

/// The probe is coupled to `ProjectManifest`'s **type name**, because `to_ron` writes
/// with `struct_names(true)` and RON checks that name before any field. Round-trip a real
/// serialized manifest rather than a hand-written string, so a rename of the manifest
/// type breaks this test rather than every project open.
#[test]
fn the_probe_reads_a_real_serialized_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("Real");
    let path = root.to_str().unwrap();
    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();

    assert!(
        fs::read_to_string(root.join("project.skrib"))
            .unwrap()
            .starts_with("ProjectManifest("),
        "the writer emits the struct name; the probe's rename depends on it"
    );
    assert!(crate::version_gate::check_version_gate(path, SkribShape::ExplodedFolder).is_ok());
}

/// `migrate_bundle`'s narrowed contract: a stamp above ours is no longer its business.
#[test]
fn migrate_bundle_no_longer_bails_on_a_stamp_above_current() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = FORMAT_VERSION + 1;
    crate::migration::migrate_bundle(&mut bundle)
        .expect("the gate, not the migration chain, judges what is too new");
    assert_eq!(
        bundle.manifest.format_version,
        FORMAT_VERSION + 1,
        "the chain must no-op rather than downgrade"
    );
}

// ---------------------------------------------------------------------------
// The content-derived floor (`compute_min_read_version`)
// ---------------------------------------------------------------------------

/// Drop every epigraph row (and its blob) from a bundle, so a test can isolate one
/// floor axis from the other. Epigraph text is prose-role, so it lives in `prose_refs`
/// with its body in `BundledItem::prose`, keyed by the same `file_id`.
fn strip_epigraphs(bundle: &mut WorkBundle) {
    for binder in &mut bundle.binders {
        for item in &mut binder.items {
            let dropped: Vec<u64> = item
                .item
                .prose_refs
                .iter()
                .filter(|p| p.role == ContentRole::EpigraphText)
                .map(|p| p.file_id)
                .collect();
            item.item
                .prose_refs
                .retain(|p| p.role != ContentRole::EpigraphText);
            for id in dropped {
                item.prose.remove(&id);
            }
        }
    }
}

/// Each content kind raises the floor only when actually present, and each axis is
/// independent of the others. This is the payoff of the two-number scheme:
/// `format_version` is stamped unconditionally on every write, autosave included, so a
/// single number would lock a plain project out of the previous build the instant one
/// tick landed.
///
/// Peeled one axis at a time — epigraphs (v6) then templates (v5) — because the axes
/// have to be separable to be worth anything: a project with templates but no epigraph
/// must still claim 5, not 6, or every v5 build loses files it can read perfectly well.
#[test]
fn the_floor_rises_only_for_content_that_needs_it() {
    let everything = build_bundle(ShapeTag::Folder);
    assert_eq!(
        crate::version_gate::compute_min_read_version(&everything),
        6,
        "epigraphs are the v6 axis and the fixture carries them"
    );

    let mut no_epigraphs = build_bundle(ShapeTag::Folder);
    strip_epigraphs(&mut no_epigraphs);
    assert_eq!(
        crate::version_gate::compute_min_read_version(&no_epigraphs),
        5,
        "templates still hold the floor at 5 once the epigraphs are gone"
    );

    let mut without = build_bundle(ShapeTag::Folder);
    strip_epigraphs(&mut without);
    without.note_templates.clear();
    without.note_template_bodies.clear();
    assert_eq!(crate::version_gate::compute_min_read_version(&without), 4);
}

/// The floor is recomputed from content at every write, never carried over from what was
/// loaded — otherwise deleting the content that justified it would leave the project
/// permanently pinned to a version it no longer needs.
#[test]
fn the_floor_is_recomputed_at_every_write_not_carried_over() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("Shedding");
    let path = root.to_str().unwrap();

    write_bundle(
        path,
        SkribShape::ExplodedFolder,
        &build_bundle(ShapeTag::Folder),
    )
    .unwrap();
    assert_eq!(
        peek_manifest(path).unwrap().format_min_read_version,
        Some(6),
        "epigraphs must have raised the stamped floor"
    );

    let mut shed = read_bundle(path).unwrap();
    assert_eq!(shed.manifest.format_min_read_version, Some(6));
    strip_epigraphs(&mut shed);
    shed.note_templates.clear();
    shed.note_template_bodies.clear();
    write_bundle(path, SkribShape::ExplodedFolder, &shed).unwrap();

    assert_eq!(
        peek_manifest(path).unwrap().format_min_read_version,
        Some(4),
        "dropping the templates must drop the floor back — nothing is sticky"
    );
}

/// The floor must be scored on what actually reaches disk. `from_entities` drops content
/// failing `content_allowed` before bundling it, so scoring the pre-filter store entities
/// would count rows that were never written and needlessly refuse readers.
#[test]
fn the_floor_ignores_content_dropped_by_content_allowed() {
    let mut s = sample_inputs();

    // A Folder/Book may not carry SceneText — `from_entities` filters it out.
    assert!(
        !allowed_content(&BinderItemRole::Folder, &BinderItemSubRole::Book)
            .contains(&ContentRole::SceneText),
        "the fixture must actually be an invalid triple"
    );
    let victim = &mut s.binders[0].items[0];
    victim.item.role = BinderItemRole::Folder;
    victim.item.sub_role = BinderItemSubRole::Book;
    victim.contents = vec![Content {
        id: 9001,
        created_at: ts(),
        updated_at: ts(),
        activated: true,
        role: ContentRole::SceneText,
        data: "should never be written".into(),
    }];

    let bundle = from_entities(
        &s.work,
        &s.tags,
        &s.dict_words,
        &[],
        &s.note_templates,
        Some(&s.smart_punctuation),
        &s.trash,
        &[],
        &[],
        &[], // no comments
        &s.binders,
        ShapeTag::Folder,
    );

    let written = &bundle.binders[0].items[0].item;
    assert!(
        written.inline_contents.is_empty() && written.prose_refs.is_empty(),
        "the filter must have dropped the invalid content before it could be scored"
    );
    // Walking the bundle rather than the pre-filter store input is what makes that hold:
    // the dropped row is simply not there to score.
    crate::version_gate::compute_min_read_version(&bundle);
}

/// The floor is a pure function of content already hashed elsewhere in the same bundle,
/// so it must not participate in the fingerprint. Otherwise every future refinement of
/// the scoring logic — with not one word of prose edited — re-triggers a backup cascade
/// on the next save of every project.
#[test]
fn the_content_fingerprint_ignores_the_floor() {
    let a = build_bundle(ShapeTag::Folder);
    let mut b = a.clone();
    b.manifest.format_min_read_version = Some(FORMAT_VERSION + 7);
    assert_eq!(content_fingerprint(&a), content_fingerprint(&b));
}

/// A v4 bundle migrates forward to v5 with no templates and no complaint.
#[test]
fn a_v4_bundle_migrates_to_v5() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = 4;
    bundle.note_templates.clear();
    bundle.note_template_bodies.clear();
    crate::migration::migrate_bundle(&mut bundle).expect("v4 must migrate");
    assert_eq!(bundle.manifest.format_version, FORMAT_VERSION);
}
