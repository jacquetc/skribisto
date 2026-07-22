// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Phase 0.5 + 0.6 acceptance tests: the use cases that gained a `work_id` must actually
//! act on the Work the caller named, not on whichever Work a `HashMap`-backed store
//! happens to iterate first. Phase 0.5 covered 7 use cases (`scan_mentions` through
//! `empty_trash` below); Phase 0.6 adds the 6 that were misclassified as already-safe —
//! `trash_binder_items`, `trash_binder`, `restore_items`, `restore_items_to`,
//! `delete_trash_entries`, `merge_two_scenes` — each of which had its own private
//! `get_all_work().next()` work_id() helper despite the DTO already carrying an id for
//! something else.
//!
//! Every test here builds **two Works in one store** — the only way this class of bug can
//! be observed at all with one Work open, `all_work().next()` and "the one the caller
//! asked for" are the same row by accident, and the bug is invisible. `seed_second_work`
//! adds Work B onto the same `AppContext` that `new_work` already populated with Work A,
//! bypassing `new_work`/`load_work`'s own "close every other open Work first" sweep the
//! same way `work_management`'s own `load_additional_work` test helper does (see
//! `work_management::save_load_test::a_second_work_never_perturbs_the_first_through_mutate_save_close`)
//! — but through the *public* generated entity commands only (`work_commands`,
//! `binder_commands`, `search_commands`, `work_info_commands`, `system_commands`,
//! `root_commands`), since those `pub(crate)` helpers are not reachable from here.
//!
//! `empty_trash` is the data-loss-shaped one the whole migration exists for: with two
//! Works open, "empty the trash" used to have no defined subject at all.
//!
//! **Why every test below runs the operation against BOTH Works, not just one:**
//! `im::HashMap` iteration order is seeded from a per-process random key (`RandomState`),
//! not insertion order — an earlier draft of this file assumed "the second-created Work
//! sorts after the first" and picked that one as the single discriminating target. That
//! assumption is false: `store.works.read().unwrap().values()` can hand back either Work
//! first depending on the process's random seed, so a single-direction test (act on B,
//! assert A untouched) only fails a `get_all_work().next()`-style regression on the runs
//! where the random seed happens to put A first — observed at ~50% failure detection
//! across repeated runs, i.e. a coin flip, not a guard.
//!
//! The fix is structural, not statistical: within ONE test (one store, one fixed random
//! seed for its whole lifetime), invoke the use case once naming Work A and once naming
//! Work B, and assert the effect landed on the NAMED Work both times. A regression that
//! ignores `dto.work_id` and always resolves to whichever Work its `.next()` picks can
//! match the caller's intent for AT MOST one of those two calls (there are only two
//! Works, and `.next()` returns one fixed one for the run) — the other call is
//! guaranteed to act on the wrong Work and trip an assertion. This holds regardless of
//! which Work the random seed favours, so every test here fails deterministically on
//! every run when the bug is present, not on half of them.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands,
    mention_management_commands, progress_management_commands, root_commands, search_commands,
    search_management_commands, system_commands, tag_management_commands, trash_info_commands,
    trash_management_commands, work_commands, work_info_commands, work_management_commands,
};
use frontend::common::direct_access::root::RootRelationshipField;
use frontend::common::direct_access::system::SystemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::direct_access::work_info::WorkInfoRelationshipField;
use frontend::common::entities::{
    BinderItemRole, BinderItemSubRole, ChapterMode, ContentRole, WorkShape,
};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    CreateBinderDto, CreateContentDto, CreateSearchDto, CreateTrashInfoDto, CreateWorkDto,
    CreateWorkInfoDto, RootRelationshipDto, SystemRelationshipDto, WorkInfoRelationshipDto,
    WorkRelationshipDto,
};
use binder_item_management::MergeTwoScenesDto;
use mention_management::ScanMentionsDto;
use progress_management::{CountWordsDto, RecordProgressSnapshotDto};
use search_management::{ReplaceInProjectDto, RunSearchDto};
use tag_management::ImportTagsDto;
use trash_management::{
    DeleteTrashEntriesDto, DropPosition, EmptyTrashDto, RestoreItemsDto, RestoreItemsToDto,
    TrashBinderDto, TrashBinderItemsDto,
};
use work_management::{NewWorkDto, NewWorkTemplate};

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

/// Work A, via the real public `new_work` path — this also seeds the process-wide
/// `Root`/`System` singletons that `seed_second_work` reuses.
fn ctx_with_work_a() -> (AppContext, EntityId) {
    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-multiwork-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
        },
    )
    .expect("new_work A");
    let _ = std::fs::remove_dir_all(&dir);
    let work_a = work_commands::get_all_work(&ctx).unwrap().pop().unwrap().id;
    (ctx, work_a)
}

struct SecondWork {
    work_id: EntityId,
    work_info_id: EntityId,
    binder_id: EntityId,
}

/// Seed a second Work directly onto `ctx`'s already-populated store, via nothing but
/// public generated entity commands — replicating exactly what `new_work`/`load_work`'s
/// own trunk-building does, minus the "close every other open Work" sweep neither of
/// those two use cases can be asked to skip today. Legal per the entity model: `Root.works`
/// is `one_to_many`, so more than one `Work` coexisting is a supported shape, just not one
/// the app-level lifecycle commands can reach yet.
fn seed_second_work(ctx: &AppContext, title: &str) -> SecondWork {
    let n = now();

    let work = work_commands::create_orphan_work(
        ctx,
        None,
        &CreateWorkDto {
            created_at: n,
            updated_at: n,
            title: title.to_string(),
            author_name: String::new(),
            dict_language: vec!["en-US".to_string()],
            unique_id: format!("{title}-uid"),
            chapter_mode: ChapterMode::default(),
            binders: vec![],
            tags: vec![],
            dict_words: vec![],
            trash_infos: vec![],
            paces: vec![],
        },
    )
    .unwrap_or_else(|e| panic!("create work {title}: {e}"));

    let binder = binder_commands::create_orphan_binder(
        ctx,
        None,
        &CreateBinderDto {
            created_at: n,
            updated_at: n,
            uid: common::uid::fixture_uid(work.id * 1000),
            name: "Manuscript".to_string(),
            activated: true,
            binder_items: vec![],
        },
    )
    .unwrap_or_else(|e| panic!("create binder for {title}: {e}"));
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work.id,
            field: WorkRelationshipField::Binders,
            right_ids: vec![binder.id],
        },
    )
    .expect("attach binder to work");

    let search = search_commands::create_orphan_search(
        ctx,
        &CreateSearchDto {
            created_at: n,
            updated_at: n,
            ..Default::default()
        },
    )
    .expect("create search");

    let work_info = work_info_commands::create_orphan_work_info(
        ctx,
        &CreateWorkInfoDto {
            created_at: n,
            updated_at: n,
            file_name: None,
            shape: WorkShape::Folder,
            work: None,
            search: 0,
            progress_snapshots: vec![],
        },
    )
    .expect("create work_info");
    work_info_commands::set_work_info_relationship(
        ctx,
        &WorkInfoRelationshipDto {
            id: work_info.id,
            field: WorkInfoRelationshipField::Search,
            right_ids: vec![search.id],
        },
    )
    .expect("attach search to work_info");
    work_info_commands::set_work_info_relationship(
        ctx,
        &WorkInfoRelationshipDto {
            id: work_info.id,
            field: WorkInfoRelationshipField::Work,
            right_ids: vec![work.id],
        },
    )
    .expect("attach work_info to work");

    // System and Root are process-wide singletons already seeded by Work A's `new_work` —
    // reused, not recreated, mirroring `load_work_uc::create_trunk`'s own "reuse if present"
    // branch.
    let system_id = system_commands::get_all_system(ctx).unwrap().pop().unwrap().id;
    let mut work_infos = system_commands::get_system_relationship(
        ctx,
        &system_id,
        &SystemRelationshipField::WorkInfos,
    )
    .unwrap();
    work_infos.push(work_info.id);
    system_commands::set_system_relationship(
        ctx,
        &SystemRelationshipDto {
            id: system_id,
            field: SystemRelationshipField::WorkInfos,
            right_ids: work_infos,
        },
    )
    .expect("attach work_info to system");

    let root_id = root_commands::get_all_root(ctx).unwrap().pop().unwrap().id;
    let mut works =
        root_commands::get_root_relationship(ctx, &root_id, &RootRelationshipField::Works)
            .unwrap();
    works.push(work.id);
    root_commands::set_root_relationship(
        ctx,
        &RootRelationshipDto {
            id: root_id,
            field: RootRelationshipField::Works,
            right_ids: works,
        },
    )
    .expect("attach work to root — Root.works must APPEND, not replace (Phase 0)");

    SecondWork {
        work_id: work.id,
        work_info_id: work_info.id,
        binder_id: binder.id,
    }
}

/// A discoverable note + a scene naming it, for `scan_mentions`. Returns the scene id (the
/// one whose roster the scan should report a hit for).
fn seed_mentionable_scene(ctx: &AppContext, work_id: EntityId, binder_id: EntityId, needle: &str) -> EntityId {
    use frontend::direct_access::{BinderItemRelationshipDto, CreateBinderItemDto, CreateBinderTagDto};
    use frontend::commands::{binder_item_commands, binder_tag_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

    let n = now();
    let mk = |sub_role, title: &str| CreateBinderItemDto {
        uid: common::uid::fixture_uid(
            title.len() as u64 * 7919 + title.chars().map(|c| c as u64).sum::<u64>(),
        ),
        created_at: n,
        updated_at: n,
        title: title.to_string(),
        sub_title: String::new(),
        role: BinderItemRole::Item,
        sub_role,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: vec![needle.to_string()],
        contents: vec![],
        references: vec![],
        tags: vec![],
    };
    let mut character = mk(BinderItemSubRole::Note, &format!("{needle} Sarraute"));
    character.aliases = vec![needle.to_string()];
    let scene = mk(BinderItemSubRole::Scene, &format!("Scene about {needle}"));
    let created = binder_item_commands::create_binder_item_multi(
        ctx,
        None,
        &[character, scene],
        binder_id,
        -1,
    )
    .expect("create character+scene");
    let character_id = created[0].id;
    let scene_id = created[1].id;

    let tag = binder_tag_commands::create_orphan_binder_tag(
        ctx,
        None,
        &CreateBinderTagDto {
            created_at: n,
            updated_at: n,
            name: "character".into(),
            color: "#4477aa".into(),
            details: String::new(),
            discoverable: true,
        },
    )
    .expect("create tag")
    .id;
    // The tag must be attached to BOTH the Work's own palette (`run_scan` builds its
    // `discoverable` set from `g.tags`, which `gather` hydrates from `Work.tags`) AND the
    // item that carries it — attaching only one half is the mistake `scan_writes_no_entities`
    // in `mention_scan_test.rs` guards the working example against.
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work_id,
            field: WorkRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .expect("attach tag to work palette");
    binder_item_commands::set_binder_item_relationship(
        ctx,
        None,
        &BinderItemRelationshipDto {
            id: character_id,
            field: BinderItemRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .expect("tag the character");

    frontend::commands::content_commands::create_content_multi(
        ctx,
        None,
        &[CreateContentDto {
            created_at: n,
            updated_at: n,
            activated: true,
            role: ContentRole::SceneText,
            data: format!("{needle} climbed the stair."),
        }],
        scene_id,
        -1,
    )
    .expect("create scene prose");

    scene_id
}

/// Add one prose scene (no mentions machinery) under `binder_id`, holding `words` words —
/// for `count_words`.
fn seed_prose_scene(ctx: &AppContext, binder_id: EntityId, words: usize) {
    let n = now();
    let created = binder_item_commands::create_binder_item_multi(
        ctx,
        None,
        &[frontend::direct_access::CreateBinderItemDto {
            uid: common::uid::fixture_uid(binder_id * 13 + words as u64),
            created_at: n,
            updated_at: n,
            title: "Scene".to_string(),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            label: String::new(),
            activated: true,
            is_favorite: false,
            is_exportable: true,
            indent: 0,
            word_count_goal: 0,
            char_count_goal: 0,
            dict_language: Vec::new(),
            aliases: Vec::new(),
            contents: vec![],
            references: vec![],
            tags: vec![],
        }],
        binder_id,
        -1,
    )
    .expect("create scene");
    let prose = (0..words).map(|_| "word").collect::<Vec<_>>().join(" ");
    frontend::commands::content_commands::create_content_multi(
        ctx,
        None,
        &[CreateContentDto {
            created_at: n,
            updated_at: n,
            activated: true,
            role: ContentRole::SceneText,
            data: prose,
        }],
        created[0].id,
        -1,
    )
    .expect("create prose");
}

/// Trash one freshly created BinderItem directly (bypassing `trash_binder_items`, which
/// still resolves its own Work via the unscoped `get_all_work().next()` — see the scouts'
/// classification corrections — so it is not a safe fixture builder for a two-Works test).
/// Returns `(trash_info_id, binder_item_id)`.
fn seed_trashed_item(ctx: &AppContext, work_id: EntityId, binder_id: EntityId) -> (EntityId, EntityId) {
    let n = now();
    let created = binder_item_commands::create_binder_item_multi(
        ctx,
        None,
        &[frontend::direct_access::CreateBinderItemDto {
            uid: common::uid::fixture_uid(binder_id * 97 + 1),
            created_at: n,
            updated_at: n,
            title: "Deleted scene".to_string(),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            label: String::new(),
            activated: false,
            is_favorite: false,
            is_exportable: true,
            indent: 0,
            word_count_goal: 0,
            char_count_goal: 0,
            dict_language: Vec::new(),
            aliases: Vec::new(),
            contents: vec![],
            references: vec![],
            tags: vec![],
        }],
        binder_id,
        -1,
    )
    .expect("create item to trash");
    let item_id = created[0].id;

    let trash_info = trash_info_commands::create_orphan_trash_info(
        ctx,
        None,
        &CreateTrashInfoDto {
            created_at: n,
            updated_at: n,
            trashed_at: n,
            origin_binder_id: binder_id as i64,
            trashed_binder: None,
            trashed_binder_item: Some(item_id),
        },
    )
    .expect("create trash_info");
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work_id,
            field: WorkRelationshipField::TrashInfos,
            right_ids: vec![trash_info.id],
        },
    )
    .expect("index trash_info under Work.trash_infos");

    (trash_info.id, item_id)
}

// ───────────────────────────── the 7 use cases ─────────────────────────────

/// `scan_mentions`: scanning a Work must report THAT Work's own mention, never the other
/// one's — checked in both directions (see the file-level doc comment for why).
#[test]
fn scan_mentions_only_scans_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let scene_a = seed_mentionable_scene(&ctx, work_a, binder_a, "Alpha");

    let b = seed_second_work(&ctx, "Work B");
    let scene_b = seed_mentionable_scene(&ctx, b.work_id, b.binder_id, "Bravo");

    let scan = |work_id: EntityId| {
        let op = mention_management_commands::scan_mentions(&ctx, &ScanMentionsDto { work_id })
            .expect("start scan");
        let finished = ctx
            .long_operation_manager
            .lock()
            .unwrap()
            .wait_for_operation(&op, Some(std::time::Duration::from_secs(30)));
        assert!(finished, "scan must complete");
        mention_management_commands::get_scan_mentions_result(&ctx, &op)
            .expect("get result")
            .expect("a completed scan has a result")
    };

    let result_a = scan(work_a);
    let mention_management::MentionHits::Found(hits_a) = result_a.hits else {
        panic!("expected Found hits for Work A");
    };
    assert!(
        hits_a.iter().any(|h| matches!(
            h,
            mention_management::MentionHit::Found { owner_id, .. } if *owner_id == scene_a
        )),
        "Work A's own mention must be found when Work A is the named Work"
    );
    assert!(
        !hits_a.iter().any(|h| matches!(
            h,
            mention_management::MentionHit::Found { owner_id, .. } if *owner_id == scene_b
        )),
        "Work B's scene must never appear in a scan named against Work A"
    );

    let result_b = scan(b.work_id);
    let mention_management::MentionHits::Found(hits_b) = result_b.hits else {
        panic!("expected Found hits for Work B");
    };
    assert!(
        hits_b.iter().any(|h| matches!(
            h,
            mention_management::MentionHit::Found { owner_id, .. } if *owner_id == scene_b
        )),
        "Work B's own mention must be found when Work B is the named Work"
    );
    assert!(
        !hits_b.iter().any(|h| matches!(
            h,
            mention_management::MentionHit::Found { owner_id, .. } if *owner_id == scene_a
        )),
        "Work A's scene must never appear in a scan named against Work B"
    );
}

/// `count_words`: counting a Work must return THAT Work's own total — each Work is
/// deliberately given a different, distinguishable word count — checked in both
/// directions (see the file-level doc comment for why).
#[test]
fn count_words_only_counts_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    seed_prose_scene(&ctx, binder_a, 3);

    let b = seed_second_work(&ctx, "Work B");
    seed_prose_scene(&ctx, b.binder_id, 11);

    let count = |work_id: EntityId| {
        let op = progress_management_commands::count_words(&ctx, &CountWordsDto { work_id })
            .expect("start count");
        let finished = ctx
            .long_operation_manager
            .lock()
            .unwrap()
            .wait_for_operation(&op, Some(std::time::Duration::from_secs(30)));
        assert!(finished);
        progress_management_commands::get_count_words_result(&ctx, &op)
            .expect("get result")
            .expect("a completed count has a result")
            .total_word_count
    };

    assert_eq!(
        count(work_a),
        3,
        "must count Work A's 3 words when Work A is the named Work"
    );
    assert_eq!(
        count(b.work_id),
        11,
        "must count Work B's 11 words when Work B is the named Work"
    );
}

/// `record_progress_snapshot`: recording against a Work's `WorkInfo` must not touch the
/// other Work's snapshot history — checked in both directions (see the file-level doc
/// comment for why): recording onto Work A first, then onto Work B, must leave Work A's
/// own snapshot from the first call untouched by the second.
#[test]
fn record_progress_snapshot_records_onto_the_requested_works_workinfo() {
    let (ctx, _work_a) = ctx_with_work_a();
    let work_info_a = work_info_commands::get_all_work_info(&ctx).unwrap().pop().unwrap();

    let b = seed_second_work(&ctx, "Work B");

    progress_management_commands::record_progress_snapshot(
        &ctx,
        &RecordProgressSnapshotDto {
            work_id: work_info_a.work.expect("Work A's WorkInfo must point back at Work A"),
            day: now(),
            total_word_count: 333,
            total_char_count: 1665,
            book_item_ids: vec![],
            book_word_counts: vec![],
        },
    )
    .expect("record onto A");

    let snaps_a_after_first = work_info_commands::get_work_info_relationship(
        &ctx,
        &work_info_a.id,
        &WorkInfoRelationshipField::ProgressSnapshots,
    )
    .unwrap();
    assert_eq!(
        snaps_a_after_first.len(),
        1,
        "Work A's WorkInfo must gain exactly one snapshot when Work A is named"
    );
    let snaps_b_after_first = work_info_commands::get_work_info_relationship(
        &ctx,
        &b.work_info_id,
        &WorkInfoRelationshipField::ProgressSnapshots,
    )
    .unwrap();
    assert!(
        snaps_b_after_first.is_empty(),
        "Work B's WorkInfo must gain no snapshot from a call naming Work A"
    );

    progress_management_commands::record_progress_snapshot(
        &ctx,
        &RecordProgressSnapshotDto {
            work_id: b.work_id,
            day: now(),
            total_word_count: 555,
            total_char_count: 2775,
            book_item_ids: vec![],
            book_word_counts: vec![],
        },
    )
    .expect("record onto B");

    let snaps_a_after_second = work_info_commands::get_work_info_relationship(
        &ctx,
        &work_info_a.id,
        &WorkInfoRelationshipField::ProgressSnapshots,
    )
    .unwrap();
    assert_eq!(
        snaps_a_after_second.len(),
        1,
        "Work A's WorkInfo must still hold exactly its own one snapshot after a call naming Work B"
    );

    let snaps_b_after_second = work_info_commands::get_work_info_relationship(
        &ctx,
        &b.work_info_id,
        &WorkInfoRelationshipField::ProgressSnapshots,
    )
    .unwrap();
    assert_eq!(
        snaps_b_after_second.len(),
        1,
        "Work B's WorkInfo must gain exactly one snapshot when Work B is named"
    );
}

/// `import_tags`: importing into a Work must not add a single row to the other Work's
/// palette — checked in both directions (see the file-level doc comment for why):
/// importing into Work A first, then Work B, must leave Work A's earlier tag alone.
#[test]
fn import_tags_only_imports_into_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let b = seed_second_work(&ctx, "Work B");

    let result_a = tag_management_commands::import_tags(
        &ctx,
        None,
        &ImportTagsDto {
            work_id: work_a,
            names: vec!["status/draft".to_string()],
            colors: vec!["#607d8b".to_string()],
            details: vec![String::new()],
            discoverables: vec![false],
        },
    )
    .expect("import into A");
    assert_eq!(result_a.created_ids.len(), 1);

    let tags_a_after_first =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Tags).unwrap();
    assert_eq!(
        tags_a_after_first, result_a.created_ids,
        "the new tag must land on Work A when Work A is named"
    );
    let tags_b_after_first =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::Tags).unwrap();
    assert!(
        tags_b_after_first.is_empty(),
        "Work B's palette must stay empty after a call naming Work A"
    );

    let result_b = tag_management_commands::import_tags(
        &ctx,
        None,
        &ImportTagsDto {
            work_id: b.work_id,
            names: vec!["status/final".to_string()],
            colors: vec!["#2e7d32".to_string()],
            details: vec![String::new()],
            discoverables: vec![false],
        },
    )
    .expect("import into B");
    assert_eq!(result_b.created_ids.len(), 1);

    let tags_a_after_second =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Tags).unwrap();
    assert_eq!(
        tags_a_after_second, result_a.created_ids,
        "Work A's palette must still hold only its own tag after a call naming Work B"
    );

    let tags_b_after_second =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::Tags).unwrap();
    assert_eq!(
        tags_b_after_second, result_b.created_ids,
        "the new tag must land on Work B when Work B is named"
    );
}

/// One prose scene under `binder_id` whose body is exactly `needle` repeated once — for
/// `run_search`/`replace_in_project`'s per-Work distinguishable corpus.
fn seed_needle_scene(ctx: &AppContext, binder_id: EntityId, title: &str, prose: &str) -> EntityId {
    let n = now();
    let created = binder_item_commands::create_binder_item_multi(
        ctx,
        None,
        &[frontend::direct_access::CreateBinderItemDto {
            uid: common::uid::fixture_uid(binder_id * 61 + prose.len() as u64),
            created_at: n,
            updated_at: n,
            title: title.to_string(),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            label: String::new(),
            activated: true,
            is_favorite: false,
            is_exportable: true,
            indent: 0,
            word_count_goal: 0,
            char_count_goal: 0,
            dict_language: Vec::new(),
            aliases: Vec::new(),
            contents: vec![],
            references: vec![],
            tags: vec![],
        }],
        binder_id,
        -1,
    )
    .unwrap()[0]
        .id;
    frontend::commands::content_commands::create_content_multi(
        ctx,
        None,
        &[CreateContentDto {
            created_at: n,
            updated_at: n,
            activated: true,
            role: ContentRole::SceneText,
            data: prose.to_string(),
        }],
        created,
        -1,
    )
    .unwrap();
    created
}

/// `run_search`: searching a Work must find THAT Work's own prose and never the other
/// one's, even though both contain body text — checked in both directions (see the
/// file-level doc comment for why).
#[test]
fn run_search_only_searches_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    seed_needle_scene(&ctx, binder_a, "A scene", "unobtainable veins ran through the rock");

    let b = seed_second_work(&ctx, "Work B");
    seed_needle_scene(&ctx, b.binder_id, "B scene", "unobtainium veins ran through the rock");

    let search = |work_id: EntityId, query: &str| {
        search_management_commands::run_search(
            &ctx,
            &RunSearchDto {
                work_id,
                query: query.to_string(),
                case_sensitive: false,
                whole_word: false,
                diacritic_sensitive: false,
                facets: vec![],
                search_body: true,
                search_titles: true,
                search_synopsis: true,
                search_labels: false,
                include_trashed: false,
            },
        )
        .expect("run_search")
    };
    let search_id_for = |work_id: EntityId| {
        work_info_commands::get_all_work_info(&ctx)
            .unwrap()
            .into_iter()
            .find(|wi| wi.work == Some(work_id))
            .unwrap()
            .search
    };

    let out_a = search(work_a, "unobtainium");
    assert_eq!(
        out_a.match_count, 0,
        "Work A's corpus has no exact 'unobtainium', only 'unobtainable' — a search named \
         against Work A must not somehow find Work B's word"
    );
    let out_a2 = search(work_a, "unobtainable");
    assert!(out_a2.match_count > 0, "must find Work A's own word when Work A is named");
    let search_b_id = search_id_for(b.work_id);
    let search_b = search_commands::get_search(&ctx, &search_b_id).unwrap().unwrap();
    assert!(
        search_b.query.is_empty(),
        "Work B's Search must be untouched by a search run against Work A"
    );

    let out_b = search(b.work_id, "unobtainium");
    assert!(out_b.match_count > 0, "must find Work B's own word when Work B is named");
    let search_a_id = search_id_for(work_a);
    let search_a = search_commands::get_search(&ctx, &search_a_id).unwrap().unwrap();
    assert_eq!(
        search_a.query, "unobtainable",
        "Work A's Search must still hold its own last query after a search run against Work B"
    );
}

/// `replace_in_project`: replacing in a Work must rewrite only THAT Work's prose —
/// checked in both directions (see the file-level doc comment for why): replacing in
/// Work A first, then Work B, must leave Work A's already-rewritten prose alone.
#[test]
fn replace_in_project_only_replaces_in_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    seed_needle_scene(&ctx, binder_a, "A scene", "Aurelien walked in A.");

    let b = seed_second_work(&ctx, "Work B");
    seed_needle_scene(&ctx, b.binder_id, "B scene", "Aurelien walked in B.");

    // Matched by the unaffected tail of each sentence (" in A."/" in B."), which survives
    // the "Aurelien" → "Irene" replacement byte-for-byte, rather than by owner id — Content
    // carries no back-pointer to its owning BinderItem on the DTO itself.
    let content_of = |marker: &str| {
        frontend::commands::content_commands::get_all_content(&ctx)
            .unwrap()
            .into_iter()
            .find(|c| c.data.contains(marker))
            .map(|c| c.data)
    };
    let search_and_replace = |work_id: EntityId| {
        search_management_commands::run_search(
            &ctx,
            &RunSearchDto {
                work_id,
                query: "aurelien".to_string(),
                case_sensitive: false,
                whole_word: false,
                diacritic_sensitive: false,
                facets: vec![],
                search_body: true,
                search_titles: false,
                search_synopsis: false,
                search_labels: false,
                include_trashed: false,
            },
        )
        .expect("run_search");
        search_management_commands::replace_in_project(
            &ctx,
            None,
            &ReplaceInProjectDto {
                work_id,
                replacement: "Irene".to_string(),
                preserve_case: false,
                excluded_result_ids: vec![],
            },
        )
        .expect("replace_in_project");
    };

    search_and_replace(work_a);
    let content_a_after_first = content_of(" in A.").expect("A's content must still exist");
    assert!(
        content_a_after_first.to_lowercase().contains("irene"),
        "Work A's prose must be rewritten when Work A is named, got: {:?}",
        content_a_after_first
    );
    let content_b_after_first = content_of(" in B.").expect("B's content must still exist");
    assert!(
        content_b_after_first.contains("Aurelien"),
        "Work B's prose must be untouched by a replace naming Work A, got: {:?}",
        content_b_after_first
    );

    search_and_replace(b.work_id);
    let content_a_after_second = content_of(" in A.").expect("A's content must still exist");
    assert!(
        content_a_after_second.to_lowercase().contains("irene"),
        "Work A's already-rewritten prose must survive a replace naming Work B, got: {:?}",
        content_a_after_second
    );
    let content_b_after_second = content_of(" in B.").expect("B's content must still exist");
    assert!(
        content_b_after_second.to_lowercase().contains("irene"),
        "Work B's prose must actually have been rewritten when Work B is named, got: {:?}",
        content_b_after_second
    );
}

/// `empty_trash` — the data-loss-shaped one. Emptying Work A's trash must leave Work B's
/// trash (its `TrashInfo` row AND the trashed `BinderItem` it points at) completely intact.
#[test]
fn empty_trash_only_empties_the_requested_works_trash() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let (_trash_info_a, item_a) = seed_trashed_item(&ctx, work_a, binder_a);

    let b = seed_second_work(&ctx, "Work B");
    let (_trash_info_b, item_b) = seed_trashed_item(&ctx, b.work_id, b.binder_id);

    // Empty Work A's trash first — Work B's trash (row AND trashed item) must survive
    // completely untouched.
    trash_management_commands::empty_trash(&ctx, None, &EmptyTrashDto { work_id: work_a })
        .expect("empty Work A's trash");

    let trash_a_after_first =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a_after_first.is_empty(),
        "Work A's trash index must be empty when Work A is named"
    );
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_a).unwrap().is_none(),
        "Work A's trashed item must actually be gone"
    );
    let trash_b_after_first =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b_after_first,
        vec![_trash_info_b],
        "Work B's trash index must survive emptying Work A's trash completely unchanged"
    );
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().is_some(),
        "Work B's trashed item must survive — emptying A's trash must never reach into B"
    );

    // Now empty Work B's trash — this is the data-loss-shaped assertion the whole
    // migration exists for: Work A's (already-emptied) trash must not be disturbed a
    // second time, and Work B's own trash must actually go.
    trash_management_commands::empty_trash(&ctx, None, &EmptyTrashDto { work_id: b.work_id })
        .expect("empty Work B's trash");

    let trash_b_after_second =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b_after_second.is_empty(),
        "Work B's trash index must be empty when Work B is named"
    );
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().is_none(),
        "Work B's trashed item must actually be gone"
    );
    let trash_a_after_second =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a_after_second.is_empty(),
        "Work A's trash index must remain empty after a call naming Work B"
    );
}

// ───────────────────────── Phase 0.6: the 6 misclassified use cases ─────────────────────────
//
// Each of these had its own private `work_id(uow) -> get_all_work().next()` helper — the
// exact bug shape above, just not yet wired to a DTO field. Same "both directions" proof
// as every test above: within ONE store (one fixed random iteration order for its whole
// lifetime), the operation is invoked once naming Work A and once naming Work B. A
// regression that ignores `dto.work_id` resolves to whichever Work `.next()` fixes on for
// the run, which can match the caller's intent for AT MOST one of the two calls — so
// exactly one of the two directions is guaranteed to trip an assertion, on every run,
// regardless of which Work the process's random seed favours.

/// One active `Item/Scene`, appended to `binder_id` — a plain trash/merge target with no
/// content, distinguishable from other seeded items via `uid_seed`.
fn seed_active_item(ctx: &AppContext, binder_id: EntityId, uid_seed: u64) -> EntityId {
    use frontend::direct_access::CreateBinderItemDto;
    let n = now();
    binder_item_commands::create_binder_item_multi(
        ctx,
        None,
        &[CreateBinderItemDto {
            uid: common::uid::fixture_uid(uid_seed),
            created_at: n,
            updated_at: n,
            title: "Scene".to_string(),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            label: String::new(),
            activated: true,
            is_favorite: false,
            is_exportable: true,
            indent: 0,
            word_count_goal: 0,
            char_count_goal: 0,
            dict_language: Vec::new(),
            aliases: Vec::new(),
            contents: vec![],
            references: vec![],
            tags: vec![],
        }],
        binder_id,
        -1,
    )
    .expect("create item")[0]
        .id
}

/// Two adjacent, prose-bearing `Item/Scene` rows appended together to `binder_id` — a
/// `merge_two_scenes` (target, source) pair. Adjacency holds because both are created in
/// one call and appended in list order.
fn seed_scene_pair(
    ctx: &AppContext,
    binder_id: EntityId,
    uid_seed: u64,
    target_text: &str,
    source_text: &str,
) -> (EntityId, EntityId) {
    use frontend::direct_access::CreateBinderItemDto;
    let n = now();
    let mk = |i: u64, title: &str| CreateBinderItemDto {
        uid: common::uid::fixture_uid(uid_seed + i),
        created_at: n,
        updated_at: n,
        title: title.to_string(),
        sub_title: String::new(),
        role: BinderItemRole::Item,
        sub_role: BinderItemSubRole::Scene,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: Vec::new(),
        contents: vec![],
        references: vec![],
        tags: vec![],
    };
    let created = binder_item_commands::create_binder_item_multi(
        ctx,
        None,
        &[mk(1, "Target scene"), mk(2, "Source scene")],
        binder_id,
        -1,
    )
    .expect("create scene pair");
    let (target, source) = (created[0].id, created[1].id);
    for (id, text) in [(target, target_text), (source, source_text)] {
        frontend::commands::content_commands::create_content_multi(
            ctx,
            None,
            &[CreateContentDto {
                created_at: n,
                updated_at: n,
                activated: true,
                role: ContentRole::SceneText,
                data: text.to_string(),
            }],
            id,
            -1,
        )
        .expect("create scene prose");
    }
    (target, source)
}

/// `trash_binder_items`: trashing items must file the new `TrashInfo` under the NAMED
/// Work's index, not whichever Work `.next()` picks — checked in both directions.
#[test]
fn trash_binder_items_only_indexes_under_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let item_a = seed_active_item(&ctx, binder_a, 4001);

    let b = seed_second_work(&ctx, "Work B");
    let item_b = seed_active_item(&ctx, b.binder_id, 4002);

    trash_management_commands::trash_binder_items(
        &ctx,
        None,
        &TrashBinderItemsDto {
            work_id: work_a,
            binder_item_ids: vec![item_a as i64],
            origin_binder_id: binder_a as i64,
        },
    )
    .expect("trash in A");

    assert!(
        !binder_item_commands::get_binder_item(&ctx, &item_a).unwrap().unwrap().activated,
        "Work A's item must be trashed when Work A is named"
    );
    let trash_a =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_a.len(),
        1,
        "the new TrashInfo must be indexed under Work A when Work A is named"
    );
    let trash_b =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b.is_empty(),
        "Work B's trash index must stay empty after a call naming Work A"
    );
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().unwrap().activated,
        "Work B's item must be untouched by a call naming Work A"
    );

    trash_management_commands::trash_binder_items(
        &ctx,
        None,
        &TrashBinderItemsDto {
            work_id: b.work_id,
            binder_item_ids: vec![item_b as i64],
            origin_binder_id: b.binder_id as i64,
        },
    )
    .expect("trash in B");

    assert!(
        !binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().unwrap().activated,
        "Work B's item must be trashed when Work B is named"
    );
    let trash_b_after =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b_after.len(),
        1,
        "the new TrashInfo must be indexed under Work B when Work B is named"
    );
    let trash_a_after =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_a_after.len(),
        1,
        "Work A's trash index must remain exactly its own one entry after a call naming Work B"
    );
}

/// `trash_binder`: trashing a whole binder must file the new `TrashInfo` under the NAMED
/// Work's index — checked in both directions.
#[test]
fn trash_binder_only_indexes_under_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();

    let b = seed_second_work(&ctx, "Work B");

    trash_management_commands::trash_binder(
        &ctx,
        None,
        &TrashBinderDto {
            work_id: work_a,
            binder_id: binder_a as i64,
        },
    )
    .expect("trash binder in A");

    assert!(
        !binder_commands::get_binder(&ctx, &binder_a).unwrap().unwrap().activated,
        "Work A's binder must be trashed when Work A is named"
    );
    let trash_a =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_a.len(),
        1,
        "the new TrashInfo must be indexed under Work A when Work A is named"
    );
    let trash_b =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b.is_empty(),
        "Work B's trash index must stay empty after a call naming Work A"
    );
    assert!(
        binder_commands::get_binder(&ctx, &b.binder_id).unwrap().unwrap().activated,
        "Work B's binder must be untouched by a call naming Work A"
    );

    trash_management_commands::trash_binder(
        &ctx,
        None,
        &TrashBinderDto {
            work_id: b.work_id,
            binder_id: b.binder_id as i64,
        },
    )
    .expect("trash binder in B");

    assert!(
        !binder_commands::get_binder(&ctx, &b.binder_id).unwrap().unwrap().activated,
        "Work B's binder must be trashed when Work B is named"
    );
    let trash_b_after =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b_after.len(),
        1,
        "the new TrashInfo must be indexed under Work B when Work B is named"
    );
    let trash_a_after =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_a_after.len(),
        1,
        "Work A's trash index must remain exactly its own one entry after a call naming Work B"
    );
}

/// `restore_items`: restoring must unlink the consumed `TrashInfo` from the NAMED Work's
/// index, leaving the other Work's index completely untouched — checked in both
/// directions. (The item reactivation itself is resolved via the TrashInfo/binder ids
/// directly, so it is Work-independent either way; the index unlink is the part that a
/// wrong `work_id` corrupts — see `restore_items_uc`'s doc comment.)
#[test]
fn restore_items_only_unlinks_from_the_requested_works_index() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let (trash_info_a, item_a) = seed_trashed_item(&ctx, work_a, binder_a);

    let b = seed_second_work(&ctx, "Work B");
    let (trash_info_b, item_b) = seed_trashed_item(&ctx, b.work_id, b.binder_id);

    let res_a = trash_management_commands::restore_items(
        &ctx,
        None,
        &RestoreItemsDto {
            work_id: work_a,
            trash_info_ids: vec![trash_info_a as i64],
        },
    )
    .expect("restore in A");
    assert_eq!(res_a.restored_count, 1);
    assert!(!res_a.orphaned);
    let trash_a =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a.is_empty(),
        "Work A's trash index must be emptied of its consumed entry when Work A is named"
    );
    let trash_b =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b,
        vec![trash_info_b],
        "Work B's trash index must survive a restore naming Work A completely unchanged"
    );
    assert!(
        !binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().unwrap().activated,
        "Work B's item must stay trashed — a restore naming Work A must never reach into B"
    );

    let res_b = trash_management_commands::restore_items(
        &ctx,
        None,
        &RestoreItemsDto {
            work_id: b.work_id,
            trash_info_ids: vec![trash_info_b as i64],
        },
    )
    .expect("restore in B");
    assert_eq!(res_b.restored_count, 1);
    assert!(!res_b.orphaned);
    let trash_b_after =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b_after.is_empty(),
        "Work B's trash index must be emptied when Work B is named"
    );
    let trash_a_after =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a_after.is_empty(),
        "Work A's trash index must remain empty after a call naming Work B"
    );
    let _ = item_a; // reactivation is Work-independent; kept for readability of the fixture
}

/// `restore_items_to`: the post-pass sweep must unlink the consumed `TrashInfo` from the
/// NAMED Work's index only — checked in both directions.
#[test]
fn restore_items_to_only_sweeps_the_requested_works_index() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let (trash_info_a, item_a) = seed_trashed_item(&ctx, work_a, binder_a);

    let b = seed_second_work(&ctx, "Work B");
    let (trash_info_b, item_b) = seed_trashed_item(&ctx, b.work_id, b.binder_id);

    let res_a = trash_management_commands::restore_items_to(
        &ctx,
        None,
        &RestoreItemsToDto {
            work_id: work_a,
            binder_item_ids: vec![item_a],
            destination_binder_id: binder_a,
            anchor_item_id: None,
            drop_position: DropPosition::After,
        },
    )
    .expect("restore_to in A");
    assert_eq!(res_a.restored_count, 1);
    assert!(!res_a.orphaned);
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_a).unwrap().unwrap().activated,
        "Work A's item must be reactivated when Work A is named"
    );
    let trash_a =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a.is_empty(),
        "Work A's trash index must be swept of its consumed entry when Work A is named"
    );
    let trash_b =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b,
        vec![trash_info_b],
        "Work B's trash index must survive a restore_to naming Work A completely unchanged"
    );

    let res_b = trash_management_commands::restore_items_to(
        &ctx,
        None,
        &RestoreItemsToDto {
            work_id: b.work_id,
            binder_item_ids: vec![item_b],
            destination_binder_id: b.binder_id,
            anchor_item_id: None,
            drop_position: DropPosition::After,
        },
    )
    .expect("restore_to in B");
    assert_eq!(res_b.restored_count, 1);
    assert!(!res_b.orphaned);
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().unwrap().activated,
        "Work B's item must be reactivated when Work B is named"
    );
    let trash_b_after =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b_after.is_empty(),
        "Work B's trash index must be swept when Work B is named"
    );
    let trash_a_after =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a_after.is_empty(),
        "Work A's trash index must remain empty after a call naming Work B"
    );
    let _ = trash_info_a; // superseded by the `trash_a`/`trash_b` index reads above
}

/// `delete_trash_entries`: the sneakiest of the six — a wrong Work here filters the
/// caller's ids against an index that never contains them, so the existing "stale id →
/// no-op, not error" tolerance swallows the bug with zero error. The discriminating
/// assertion is therefore that the entity was ACTUALLY hard-removed, not merely that the
/// call returned `Ok(())` — checked in both directions.
#[test]
fn delete_trash_entries_only_purges_from_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let (trash_info_a, item_a) = seed_trashed_item(&ctx, work_a, binder_a);

    let b = seed_second_work(&ctx, "Work B");
    let (trash_info_b, item_b) = seed_trashed_item(&ctx, b.work_id, b.binder_id);

    trash_management_commands::delete_trash_entries(
        &ctx,
        None,
        &DeleteTrashEntriesDto {
            work_id: work_a,
            trash_info_ids: vec![trash_info_a],
        },
    )
    .expect("delete in A");

    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_a).unwrap().is_none(),
        "Work A's trashed item must actually be hard-removed when Work A is named — a \
         silent no-op here (Ok(()) but nothing deleted) is exactly the bug this guards \
         against"
    );
    let trash_a =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a.is_empty(),
        "Work A's trash index must be emptied of its purged entry when Work A is named"
    );
    let trash_b =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b,
        vec![trash_info_b],
        "Work B's trash index must survive a delete naming Work A completely unchanged"
    );
    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().is_some(),
        "Work B's trashed item must survive — a delete naming Work A must never reach into B"
    );

    trash_management_commands::delete_trash_entries(
        &ctx,
        None,
        &DeleteTrashEntriesDto {
            work_id: b.work_id,
            trash_info_ids: vec![trash_info_b],
        },
    )
    .expect("delete in B");

    assert!(
        binder_item_commands::get_binder_item(&ctx, &item_b).unwrap().is_none(),
        "Work B's trashed item must actually be hard-removed when Work B is named"
    );
    let trash_b_after =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b_after.is_empty(),
        "Work B's trash index must be emptied when Work B is named"
    );
    let trash_a_after =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_a_after.is_empty(),
        "Work A's trash index must remain empty after a call naming Work B"
    );
}

/// `merge_two_scenes`: the absorbed source scene's new `TrashInfo` must be filed under the
/// NAMED Work's index — checked in both directions.
#[test]
fn merge_two_scenes_only_indexes_under_the_requested_work() {
    let (ctx, work_a) = ctx_with_work_a();
    let binder_a = work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let (target_a, source_a) = seed_scene_pair(&ctx, binder_a, 5001, "Alpha one.", "Alpha two.");

    let b = seed_second_work(&ctx, "Work B");
    let (target_b, source_b) = seed_scene_pair(&ctx, b.binder_id, 5101, "Bravo one.", "Bravo two.");

    binder_item_management_commands::merge_two_scenes(
        &ctx,
        None,
        &MergeTwoScenesDto {
            work_id: work_a,
            target_id: target_a,
            source_id: source_a,
        },
    )
    .expect("merge in A");

    assert!(
        !binder_item_commands::get_binder_item(&ctx, &source_a).unwrap().unwrap().activated,
        "Work A's absorbed scene must be trashed when Work A is named"
    );
    let trash_a =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_a.len(),
        1,
        "the new TrashInfo must be indexed under Work A when Work A is named"
    );
    let trash_b =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert!(
        trash_b.is_empty(),
        "Work B's trash index must stay empty after a merge naming Work A"
    );
    assert!(
        binder_item_commands::get_binder_item(&ctx, &source_b).unwrap().unwrap().activated,
        "Work B's scene must be untouched by a merge naming Work A"
    );

    binder_item_management_commands::merge_two_scenes(
        &ctx,
        None,
        &MergeTwoScenesDto {
            work_id: b.work_id,
            target_id: target_b,
            source_id: source_b,
        },
    )
    .expect("merge in B");

    assert!(
        !binder_item_commands::get_binder_item(&ctx, &source_b).unwrap().unwrap().activated,
        "Work B's absorbed scene must be trashed when Work B is named"
    );
    let trash_b_after =
        work_commands::get_work_relationship(&ctx, &b.work_id, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_b_after.len(),
        1,
        "the new TrashInfo must be indexed under Work B when Work B is named"
    );
    let trash_a_after =
        work_commands::get_work_relationship(&ctx, &work_a, &WorkRelationshipField::TrashInfos)
            .unwrap();
    assert_eq!(
        trash_a_after.len(),
        1,
        "Work A's trash index must remain exactly its own one entry after a merge naming Work B"
    );
}
