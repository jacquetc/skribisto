//! Phase 0.1b vertical slice — the replace pipe, and its safety net.
//!
//! `run_search` → the writer reviews and unticks some rows → `replace_in_project`
//! rewrites the rest through the Djot parser → **one `undo` puts the manuscript back**.
//!
//! The matcher is still the naive Phase-0.1 literal scan; what these tests pin is the
//! part that must never regress: that a bulk rewrite of someone's prose is a single,
//! reversible action, that it goes through the parser rather than string-surgering the
//! markup, and that it refuses to guess when the text moved under it.

use frontend::AppContext;
use frontend::commands::{
    content_commands, search_commands, search_management_commands, search_result_commands,
    undo_redo_commands, work_management_commands,
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
            file_name: fixture_path(),
        },
    )
    .expect("load_work");
    ctx
}

fn search(query: &str) -> RunSearchDto {
    RunSearchDto {
        query: query.to_string(),
        case_sensitive: false,
        whole_word: false,
        diacritic_sensitive: false,
        search_body: true,
        search_titles: true,
        search_synopsis: true,
        search_labels: false,
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

    search_management_commands::run_search(&ctx, &search("ipsum")).expect("run_search");
    let before = all_prose(&ctx);
    assert!(before.contains("ipsum"), "the fixture must contain the word");

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            replacement: "IPSUM-REPLACED".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");

    assert!(out.items_changed > 0, "the replace must have touched fields");
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

    search_management_commands::run_search(&ctx, &search("ipsum")).expect("run_search");
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

    search_management_commands::run_search(&ctx, &search("ipsum")).expect("run_search");
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
    victim.data.push_str("\n\nAnd one more ipsum, typed after the search ran.");
    content_commands::update_content(
        &ctx,
        stack,
        &frontend::content::dtos::UpdateContentDto {
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
