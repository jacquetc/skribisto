// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two tag-identity doors that `binder_tag_controller::with_identity` does **not**
//! cover, and that nothing else asserted.
//!
//! `binder_tag_identity_and_backrefs_test` pins the controller doors. Neither of the two
//! below goes through the controller at all:
//!
//! 1. **`tag_management::import_tags`** writes straight through its unit of work, so
//!    `with_identity` never runs on it. That use case is what every built-in preset
//!    ("Basic", "Sci-fi", …) and every CSV import lands through, so a fresh project's
//!    whole starting palette was born `00000000-0000-0000-0000-000000000000`. Nil uids all
//!    compare equal: `note_capture.toml`'s `recent_tags` collapses into one always-matching
//!    slot, and no out-of-tree consumer can name one preset tag rather than another.
//!
//! 2. **`load_work`** copied a stored tag uid verbatim while its three neighbours (note
//!    templates, binders, items) all pass through `common::uid::heal_uid`. `migrate_bundle`
//!    heals tags only on the v10 → v11 step, so a project written at the *current* format
//!    version with nil tag uids, exactly what door one produced, was never healed by
//!    anything, and stayed nil through every save and open for the rest of its life.
//!
//! Both failures still compile and neither shows up in the UI, which is why they need a
//! test rather than an inspection.

use frontend::AppContext;
use frontend::commands::{
    binder_tag_commands, tag_management_commands, undo_redo_commands, work_commands,
    work_management_commands,
};
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::direct_access::CreateWorkDto;
use std::time::Duration;
use tag_management::ImportTagsDto;
use work_management::{LoadWorkDto, NewWorkDto, NewWorkTemplate, SaveWorkDto};

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

/// Door one: the bulk import that presets and CSV both use.
///
/// Written against the public command, because that is the door the app uses. Before the
/// mint landed in `import_tags_uc`, every row here came back nil and the two assertions
/// below both failed: `is_nil` on each, and the pairwise distinctness that nil uids can
/// never satisfy.
#[test]
fn every_imported_tag_is_created_with_its_own_durable_identity() {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);
    let work = work_commands::create_orphan_work(
        &ctx,
        Some(setup),
        &CreateWorkDto {
            created_at: now(),
            updated_at: now(),
            title: "The Lighthouse".into(),
            ..Default::default()
        },
    )
    .expect("create work")
    .id;

    let stack = undo_redo_commands::create_new_stack(&ctx);
    let names = ["Character", "Place", "Object"];
    let out = tag_management_commands::import_tags(
        &ctx,
        Some(stack),
        &ImportTagsDto {
            work_id: work,
            names: names.iter().map(|n| n.to_string()).collect(),
            colors: vec!["#4488cc".to_string(); names.len()],
            details: vec![String::new(); names.len()],
            discoverables: vec![true; names.len()],
        },
    )
    .expect("import tags");
    assert_eq!(
        out.created_ids.len(),
        names.len(),
        "every name should have been accepted, skipped: {:?}",
        out.skipped_names
    );

    // Read back rather than trusting a return value: the mint has to survive the store.
    let mut seen = std::collections::HashSet::new();
    for id in &out.created_ids {
        let tag = binder_tag_commands::get_binder_tag(&ctx, id)
            .expect("read tag")
            .expect("the tag exists");
        assert!(
            !tag.uid.is_nil(),
            "imported tag {:?} was created without an identity, so it is indistinguishable \
             from every other preset tag to anything that keys by uid",
            tag.name
        );
        assert!(
            seen.insert(tag.uid),
            "two imported tags share the uid {}, which is what a nil mint looks like",
            tag.uid
        );
    }
}

/// Door two: a project already on disk carrying a nil tag uid.
///
/// The bundle is written by the real `new_work` path, so it is stamped at the current
/// `FORMAT_VERSION`, which is the whole point. `migrate_bundle` mints tag uids on the
/// v10 → v11 step only, so a bundle already at the current version walks past every
/// migration arm untouched and `load_work` is the last place the nil can be caught.
#[test]
fn a_stored_tag_without_an_identity_is_healed_on_load() {
    let dir = std::env::temp_dir().join(format!("skrib-tag-uid-heal-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    {
        let ctx = AppContext::new();
        work_management_commands::new_work(
            &ctx,
            &NewWorkDto {
                goal_unit: Default::default(),
                file_name: dir.to_string_lossy().to_string(),
                title: String::new(),
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
        .expect("new_work seeds the store");

        // `new_work` only populates the store; the folder appears on the first save.
        let work = work_commands::get_all_work(&ctx)
            .expect("get_all_work")
            .pop()
            .expect("new_work made a Work")
            .id;
        let op_id = work_management_commands::save_work(
            &ctx,
            &SaveWorkDto {
                work_id: work,
                file_name: dir.to_string_lossy().to_string(),
                overwrite: true,
                media_root: String::new(),
            },
        )
        .expect("save_work dispatch");
        // Take the completion signal and drop the manager lock before waiting, the way
        // `export_work_test` does: blocking while holding it stalls every other query.
        let completion = ctx
            .long_operation_manager
            .lock()
            .unwrap()
            .completion_signal();
        assert!(
            completion.wait_for(&op_id, Some(Duration::from_secs(30))),
            "save_work should finish within the timeout"
        );
        work_management_commands::get_save_work_result(&ctx, &op_id)
            .expect("save result")
            .expect("a finished save has a result");
    }

    // Stand in for a palette written by a build that created its tags nil-identified. The
    // `uid` is spelled out rather than omitted so the test states the defect it guards:
    // `#[serde(default)]` would produce the same nil silently.
    let tags_ron = r##"[
    (
        file_id: 9001,
        uid: "00000000-0000-0000-0000-000000000000",
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
        name: "Character",
        color: "#4488cc",
        details: "",
        discoverable: true,
        creates_in: None,
        note_template: None,
    ),
]
"##;
    let tags_path = dir.join("tags.ron");
    assert!(
        tags_path.exists(),
        "new_work should have written {}",
        tags_path.display()
    );
    std::fs::write(&tags_path, tags_ron).expect("rewrite tags.ron");

    // A store of its own, so nothing from the write side can be mistaken for a load result.
    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: dir.to_string_lossy().to_string(),
        },
    )
    .expect("load the bundle back");

    let work = work_commands::get_all_work(&ctx)
        .expect("get_all_work")
        .pop()
        .expect("one Work was loaded")
        .id;
    let tag_ids = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Tags)
        .expect("work tags");
    assert_eq!(tag_ids.len(), 1, "the hand-written palette has one tag");

    let tag = binder_tag_commands::get_binder_tag(&ctx, &tag_ids[0])
        .expect("read tag")
        .expect("the tag exists");
    assert_eq!(tag.name, "Character", "the loaded row is the one written");
    assert!(
        !tag.uid.is_nil(),
        "a stored nil tag uid must be healed on load, or a project saved by a build that \
         created tags nil-identified keeps them nil forever"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
