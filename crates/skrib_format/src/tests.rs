// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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
            tags: vec![10],
        };
        items.push(ItemWithContents { item, contents });
    }
    // A cross-reference: first item -> second item.
    items[0].item.references = vec![301];

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

fn build_bundle(shape: ShapeTag) -> WorkBundle {
    let s = sample_inputs();
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
    assert_eq!(bundle, read);
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

#[test]
fn a_bundle_from_a_newer_format_is_refused_not_silently_migrated() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = FORMAT_VERSION + 1;
    assert!(
        migration::migrate_bundle(&mut bundle).is_err(),
        "a newer .skrib must be refused, not downgraded"
    );
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
    // ...and the new field defaults rather than failing the parse.
    assert!(items[0].aliases.is_empty());
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
#[test]
fn a_bundle_from_a_newer_format_is_refused() {
    let mut bundle = build_bundle(ShapeTag::Folder);
    bundle.manifest.format_version = FORMAT_VERSION + 1;
    let err = crate::migration::migrate_bundle(&mut bundle).expect_err("newer must be refused");
    assert!(
        format!("{err:#}").contains("newer Skribisto"),
        "got: {err:#}"
    );
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
