// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `replace_in_project`: the replace pipe, and its safety net.
//!
//! `run_search` → the writer reviews and unticks some rows → `replace_in_project`
//! rewrites the rest through the Djot parser → **one `undo` puts the manuscript back**.
//!
//! What these tests pin is the part that must never regress: that a bulk rewrite of
//! someone's prose is a single, reversible action, that it goes through the parser rather
//! than string-surgering the markup, and that it refuses to guess when the text moved
//! under it.

use frontend::AppContext;
use frontend::commands::{
    content_commands, search_commands, search_management_commands, search_result_commands,
    undo_redo_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::search::SearchRelationshipField;
use search_management::{ReplaceInProjectDto, RunSearchDto};
use work_management::LoadWorkDto;

fn fixture_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn loaded_ctx() -> AppContext {
    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: fixture_path(),
        },
    )
    .expect("load_work");
    ctx
}

fn work_id(ctx: &AppContext) -> u64 {
    work_commands::get_all_work(ctx)
        .expect("get_all_work")
        .pop()
        .unwrap()
        .id
}

fn search(ctx: &AppContext, query: &str) -> RunSearchDto {
    RunSearchDto {
        work_id: work_id(ctx),
        query: query.to_string(),
        case_sensitive: false,
        whole_word: false,
        diacritic_sensitive: false,
        facets: vec![],
        search_body: true,
        search_titles: true,
        search_synopsis: true,
        search_labels: false,
        search_comments: false,
        include_trashed: false,
    }
}

fn result_ids(ctx: &AppContext) -> Vec<u64> {
    let wi = frontend::commands::work_info_commands::get_all_work_info(ctx)
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    search_commands::get_search_relationship(ctx, &wi.search, &SearchRelationshipField::Results)
        .unwrap()
}

/// Every `Content` in the store, as one blob. The bluntest possible way to ask "did the
/// prose actually change, and did it actually change back".
fn all_prose(ctx: &AppContext) -> String {
    let mut all: Vec<String> = content_commands::get_all_content(ctx)
        .unwrap()
        .into_iter()
        .map(|c| c.data)
        .collect();
    all.sort();
    all.join("\u{1}")
}

/// The whole safety story, end to end: rewrite the manuscript, then take it back.
///
/// If this ever fails, Replace All must not ship — a bulk rewrite a writer cannot undo
/// is the single most destructive thing this app could do to them.
#[test]
fn a_replace_is_one_undoable_step() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let before = all_prose(&ctx);
    assert!(
        before.contains("ipsum"),
        "the fixture must contain the word"
    );

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "IPSUM-REPLACED".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");

    assert!(
        out.items_changed > 0,
        "the replace must have touched fields"
    );
    assert!(out.occurrences_replaced > 0);
    assert!(
        out.skipped_stale.is_empty(),
        "nothing moved under us: {:?}",
        out.skipped_stale
    );

    let after = all_prose(&ctx);
    assert!(
        after.contains("IPSUM-REPLACED"),
        "the replacement must be in the prose"
    );
    assert_ne!(before, after, "the manuscript must actually have changed");

    // ── and back, in ONE step ────────────────────────────────────────────────────
    undo_redo_commands::undo(&ctx, stack).expect("undo");

    let restored = all_prose(&ctx);
    assert_eq!(
        restored, before,
        "one undo must restore the manuscript EXACTLY — every scene, byte for byte"
    );
    assert!(
        !restored.contains("IPSUM-REPLACED"),
        "no trace of the replacement may survive the undo"
    );

    // Redo puts it back, so the writer can change their mind twice.
    undo_redo_commands::redo(&ctx, stack).expect("redo");
    assert_eq!(all_prose(&ctx), after, "redo must reapply the whole batch");
}

/// A row the writer unticked must be left alone. This is what makes "rename Aurélien,
/// but not in the dialogue where the nickname is right" possible at all.
#[test]
fn excluded_rows_are_left_untouched() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let ids = result_ids(&ctx);
    assert!(ids.len() >= 2, "need at least two rows to exclude one");

    // Exclude the first row; everything else gets rewritten.
    let excluded = ids[0];
    let excluded_row = search_result_commands::get_search_result(&ctx, &excluded)
        .unwrap()
        .unwrap();

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "ZZZ".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![excluded],
        },
    )
    .expect("replace_in_project");

    assert_eq!(
        out.items_changed as usize,
        ids.len() - 1,
        "every row except the excluded one must have been rewritten"
    );

    // The excluded field must still contain the original word.
    let item_contents = {
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        let cids = frontend::commands::binder_item_commands::get_binder_item_relationship(
            &ctx,
            &excluded_row.binder_item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap();
        content_commands::get_content_multi(&ctx, &cids)
            .unwrap()
            .into_iter()
            .flatten()
            .map(|c| c.data)
            .collect::<Vec<_>>()
            .join(" ")
    };
    assert!(
        item_contents.to_lowercase().contains("ipsum"),
        "the excluded row's field must still hold the original word, got: {item_contents:?}"
    );
}

/// If the text moved between the writer reviewing it and committing, the use case must
/// SKIP that field and say so — never rewrite words it was not shown.
#[test]
fn a_field_that_moved_under_us_is_skipped_and_reported() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let ids = result_ids(&ctx);
    let row = search_result_commands::get_search_result(&ctx, &ids[0])
        .unwrap()
        .unwrap();

    // Simulate the writer editing that very scene after reviewing the results: add one
    // more occurrence, so its count no longer matches what the row recorded.
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    let cids = frontend::commands::binder_item_commands::get_binder_item_relationship(
        &ctx,
        &row.binder_item_id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    let mut victim = content_commands::get_content_multi(&ctx, &cids)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| c.data.to_lowercase().contains("ipsum"))
        .expect("the row's field");
    victim
        .data
        .push_str("\n\nAnd one more ipsum, typed after the search ran.");
    content_commands::update_content(
        &ctx,
        stack,
        &frontend::content::dtos::UpdateContentDto {
            uid: victim.uid,
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: victim.data.clone(),
        },
    )
    .expect("update_content");

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "QQQ".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");

    assert!(
        !out.skipped_stale.is_empty(),
        "the edited field must be reported as stale, not silently rewritten"
    );

    let reloaded = content_commands::get_content(&ctx, &victim.id)
        .unwrap()
        .unwrap();
    assert!(
        !reloaded.data.contains("QQQ"),
        "a field that moved under us must be left completely alone"
    );
}

/// `preserve_case` was a DTO field nothing read — so a character rename would have
/// lowercased every capitalised occurrence. It is the whole point of a rename: renaming
/// Aurélien must leave AURÉLIEN as AURÉLIAN, not aurélian.
#[test]
fn preserve_case_keeps_the_case_it_found() {
    use frontend::commands::binder_item_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    // Pick a SCENE-TEXT content deliberately. `get_all_binder_item` iterates a HashMap,
    // so "the first item with a Content row" is nondeterministic — and if it landed on a
    // title Content, a body-scoped search would never read it and the test would pass or
    // fail depending on hash order.
    let victim = binder_item_commands::get_all_binder_item(&ctx)
        .unwrap()
        .into_iter()
        .filter(|i| i.activated)
        .find_map(|i| {
            let cids = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &i.id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap();
            content_commands::get_content_multi(&ctx, &cids)
                .unwrap()
                .into_iter()
                .flatten()
                .find(|c| c.role == frontend::common::entities::ContentRole::SceneText)
        })
        .expect("an item with a SceneText content row");
    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            uid: victim.uid,
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: "Aurélien, AURÉLIEN and aurélien walked on.".to_string(),
        },
    )
    .unwrap();

    let mut q = search(&ctx, "aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    search_management_commands::run_search(&ctx, &q).expect("run_search");

    search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");

    let out = content_commands::get_content(&ctx, &victim.id)
        .unwrap()
        .unwrap()
        .data;
    assert!(
        out.contains("Aurélian"),
        "Titlecase must stay titlecase: {out:?}"
    );
    assert!(
        out.contains("AURÉLIAN"),
        "ALL CAPS must stay ALL CAPS: {out:?}"
    );
    assert!(
        out.contains("aurélian"),
        "lowercase stays lowercase: {out:?}"
    );
    assert!(
        !out.to_lowercase().contains("aurélien"),
        "no occurrence of the old name may survive: {out:?}"
    );
}

/// **A2: the splice happens inside the document, not on the markup.**
///
/// `find_and_replace` splices at the offsets the parser reports and the exporter
/// re-serialises — so a rename never rewrites the query where it happens to appear in
/// markup (a link's URL, an image path) and never drops formatting under the match.
#[test]
fn a_rename_spares_the_markup_and_keeps_the_styling() {
    use frontend::commands::binder_item_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    // A scene where the name is emphasised in the prose AND appears inside a link's URL.
    let victim = binder_item_commands::get_all_binder_item(&ctx)
        .unwrap()
        .into_iter()
        .filter(|i| i.activated)
        .find_map(|i| {
            let cids = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &i.id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap();
            content_commands::get_content_multi(&ctx, &cids)
                .unwrap()
                .into_iter()
                .flatten()
                .find(|c| c.role == frontend::common::entities::ContentRole::SceneText)
        })
        .expect("a scene with body text");

    let source = "She called *Aurélien* home, then read \
                  [the note](https://example.test/Aurélien-notes).";
    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            uid: victim.uid,
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: source.to_string(),
        },
    )
    .unwrap();

    let mut q = search(&ctx, "Aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    search_management_commands::run_search(&ctx, &q).expect("run_search");

    search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");

    let out = content_commands::get_content(&ctx, &victim.id)
        .unwrap()
        .unwrap()
        .data;

    assert!(
        out.contains("*Aurélian*"),
        "the emphasis under the renamed name must survive — the old string rewrite dropped \
         it: {out:?}"
    );
    assert!(
        out.contains("https://example.test/Aurélien-notes"),
        "the link's DESTINATION is markup, not prose. The writer never typed it into their \
         sentence and cannot see it; a rename must not reach inside it: {out:?}"
    );
    assert!(
        !out.contains("*Aurélien*"),
        "the prose occurrence must actually have been renamed: {out:?}"
    );
}

/// A manuscript of `n` one-paragraph scenes, every one of them naming Aurélien — the shape
/// of the operation this feature exists for: renaming a character who is *in the book*.
///
/// Built rather than loaded from the bundled fixture, which is a handful of scenes: the
/// thing under test is what happens when the result set is bigger than a list a person
/// would scroll, and no small fixture can produce that.
fn manuscript_of(scenes: usize) -> AppContext {
    use frontend::commands::{binder_item_commands, work_commands};
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
    use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
    use work_management::{NewWorkDto, NewWorkTemplate};

    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-bigrename-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["fr-FR".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap();
    let binder =
        work_commands::get_work_relationship(&ctx, &work.id, &WorkRelationshipField::Binders)
            .unwrap()
            .pop()
            .expect("the new Work must have a binder");

    let now = chrono::Utc::now();
    let items: Vec<CreateBinderItemDto> = (0..scenes)
        .map(|i| CreateBinderItemDto {
            uid: common::uid::fixture_uid(i as u64),
            created_at: now,
            updated_at: now,
            title: format!("Scène {i}"),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
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
        })
        .collect();
    let created = binder_item_commands::create_binder_item_multi(&ctx, None, &items, binder, -1)
        .expect("create_binder_item_multi");

    for (i, item) in created.iter().enumerate() {
        content_commands::create_content_multi(
            &ctx,
            None,
            &[CreateContentDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                activated: true,
                role: ContentRole::SceneText,
                data: format!(
                    "Aurélien traversa la forêt, et le vent portait l'odeur du sel ({i})."
                ),
            }],
            item.id,
            -1,
        )
        .expect("create_content_multi");
    }

    let _ = std::fs::remove_dir_all(&dir);
    ctx
}

/// **A manuscript-wide rename must not be refused for being manuscript-wide.**
///
/// A result row is one matching *field*, and `run_search` stops at `RESULT_CAP` and reports
/// `truncated` — which the UI reads as "this scan cannot honestly claim completeness" and
/// switches Replace All **off** (`SearchReplaceViewModel::can_replace_all`). That cap used
/// to be 300, which is the size of a list a person can scroll rather than the size of a
/// novel: renaming a character who appears in 400 of a book's scenes tripped it, and the one
/// operation the search panel exists for went dark exactly when it was worth doing, leaving
/// the writer to retype the name scene by scene.
///
/// 400 scenes here, every one a hit — comfortably past the old cap — and the whole rename
/// must still go through, in one reversible step.
#[test]
fn a_rename_across_more_scenes_than_the_old_cap_still_goes_through() {
    const SCENES: usize = 400;

    let ctx = manuscript_of(SCENES);
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    let mut q = search(&ctx, "Aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    let scan = search_management_commands::run_search(&ctx, &q).expect("run_search");

    assert!(
        !scan.truncated,
        "{SCENES} matching scenes must produce a COMPLETE scan — a truncated one disables \
         Replace All, which is the whole feature"
    );
    assert_eq!(
        scan.item_count as usize, SCENES,
        "every scene names Aurélien, so every scene must be listed"
    );

    let before = all_prose(&ctx);
    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");

    assert_eq!(
        out.items_changed as usize, SCENES,
        "the rename must reach every scene, not the first few hundred"
    );
    assert_eq!(out.occurrences_replaced as usize, SCENES);
    assert!(
        out.skipped_stale.is_empty(),
        "nothing moved under us: {:?}",
        out.skipped_stale
    );

    let after = all_prose(&ctx);
    assert!(
        !after.contains("Aurélien"),
        "not one occurrence may be left behind — a rename that misses some is worse than one \
         that is refused, because nothing says which"
    );

    // …and it is still ONE undo, however many scenes it crossed. The snapshot is a
    // structural clone of the Work's entity tree, so 400 scenes cost what 4 do.
    undo_redo_commands::undo(&ctx, stack).expect("undo");
    assert_eq!(
        all_prose(&ctx),
        before,
        "one undo must put all {SCENES} scenes back, byte for byte"
    );
}
