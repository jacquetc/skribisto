// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Phase 0.1 vertical slice — the search pipe, end to end.
//!
//! Loads a real project, runs a search, and asserts the whole chain works before any
//! of the linguistic machinery goes in on top of it:
//!
//!   `load_work` → eager `Search` under `WorkInfo` → `run_search` → bulk-written
//!   `SearchResult` rows → readable back through the generated commands.
//!
//! The matcher itself is deliberately naive here (literal, ASCII); what these tests
//! pin down is the *structure*: that the entities exist and are reachable, that a
//! search rewrites the result set rather than appending to it, that rows are per
//! FIELD with an occurrence count (not per occurrence), that the cap reports
//! truncation honestly, and that trashed items stay out unless asked for.

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, search_commands, search_management_commands, search_result_commands,
    work_commands, work_info_commands, work_management_commands,
};
use frontend::common::direct_access::search::SearchRelationshipField;
use search_management::RunSearchDto;
use work_management::{CloseWorkDto, LoadWorkDto};

fn fixture_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// Load the bundled fixture and hand back a live context.
fn loaded_ctx() -> AppContext {
    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            file_name: fixture_path(),
        },
    )
    .expect("load_work should succeed on the fixture");
    ctx
}

/// A query with every scope on and nothing else — the defaults the UI will send.
fn dto(ctx: &AppContext, query: &str) -> RunSearchDto {
    let work_id = work_commands::get_all_work(ctx).expect("get_all_work").pop().unwrap().id;
    RunSearchDto {
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
    }
}

/// Every `SearchResult` currently hanging off the project's `Search`.
fn results(ctx: &AppContext) -> Vec<frontend::direct_access::search_result::dtos::SearchResultDto> {
    let work_info = work_info_commands::get_all_work_info(ctx)
        .expect("get_all_work_info")
        .into_iter()
        .next()
        .expect("a loaded project has exactly one WorkInfo");
    let ids = search_commands::get_search_relationship(
        ctx,
        &work_info.search,
        &SearchRelationshipField::Results,
    )
    .expect("Search.results");
    search_result_commands::get_search_result_multi(ctx, &ids)
        .expect("get_search_result_multi")
        .into_iter()
        .flatten()
        .collect()
}

/// `load_work` must create the Search eagerly — without it there is nothing for
/// `run_search` to attach results to, and the feature is dead on arrival for every
/// project opened before the search UI is touched.
#[test]
fn loading_a_project_creates_its_search_surface() {
    let ctx = loaded_ctx();
    let work_info = work_info_commands::get_all_work_info(&ctx)
        .expect("get_all_work_info")
        .into_iter()
        .next()
        .expect("one WorkInfo");
    let search = search_commands::get_search(&ctx, &work_info.search)
        .expect("get_search")
        .expect("WorkInfo.search must point at a live Search");
    assert!(
        search.query.is_empty(),
        "a freshly loaded project starts with no query"
    );
    assert!(results(&ctx).is_empty(), "and with no results");
}

/// The pipe: a search over a real manuscript finds real prose and writes rows the UI
/// can read back.
#[test]
fn run_search_finds_prose_and_writes_rows() {
    let ctx = loaded_ctx();

    // NB: the fixture's prose is Lorem ipsum, so it contains no English function
    // words — "the" finds nothing here. Query something that is actually in it.
    let out = search_management_commands::run_search(&ctx, &dto(&ctx, "ipsum")).expect("run_search");
    assert!(
        out.match_count > 0,
        "expected the fixture's Lorem-ipsum prose to contain 'ipsum'"
    );
    assert!(out.item_count > 0, "and to span at least one item");

    let rows = results(&ctx);
    assert!(!rows.is_empty(), "the rows must be readable back");
    assert_eq!(
        rows.len() as u64,
        // One row per matching FIELD. `match_count` counts OCCURRENCES, so it is
        // >= the row count — never equal by construction.
        rows.len() as u64,
        "sanity"
    );
    assert!(
        out.match_count >= rows.len() as u64,
        "occurrences ({}) must be at least the number of matching fields ({})",
        out.match_count,
        rows.len()
    );

    // Every row carries a live item and a snippet with the match in it.
    for row in &rows {
        assert!(row.binder_item_id > 0, "a row points at its item");
        assert!(
            row.occurrence_count > 0,
            "a row is only written when it hits"
        );
        assert!(
            row.snippet_match.to_lowercase().contains("ipsum"),
            "the snippet's matched span must be the match, got {:?}",
            row.snippet_match
        );
        assert!(!row.trashed, "trashed items are excluded by default");
        // The item id must resolve — the projection is honest about what it points at.
        binder_item_commands::get_binder_item(&ctx, &row.binder_item_id)
            .expect("get_binder_item")
            .expect("a result row points at a live BinderItem");
    }
}

/// A row counts every occurrence in its field — it is NOT one row per occurrence.
/// This is the two-tier model the whole UI rests on, so pin it.
#[test]
fn a_row_counts_every_occurrence_in_its_field() {
    let ctx = loaded_ctx();
    search_management_commands::run_search(&ctx, &dto(&ctx, "ipsum")).expect("run_search");
    let rows = results(&ctx);
    assert!(!rows.is_empty());
    assert!(
        rows.iter().any(|r| r.occurrence_count > 1),
        "expected at least one field to contain 'ipsum' more than once — otherwise \
         this fixture cannot distinguish per-field rows from per-occurrence rows"
    );
    // One row per (item, field), never two.
    let mut keys: Vec<(u64, String)> = rows
        .iter()
        .map(|r| (r.binder_item_id, format!("{:?}", r.match_field)))
        .collect();
    let before = keys.len();
    keys.sort();
    keys.dedup();
    assert_eq!(before, keys.len(), "the same field was written twice");
}

/// A search REPLACES the previous result set. If it appended, the rows would grow
/// without bound across a typing session — this is the single most important
/// structural property of the whole design.
#[test]
fn a_search_replaces_the_previous_result_set() {
    let ctx = loaded_ctx();

    search_management_commands::run_search(&ctx, &dto(&ctx, "ipsum")).expect("first search");
    let first = results(&ctx);
    assert!(!first.is_empty());

    search_management_commands::run_search(&ctx, &dto(&ctx, "ipsum")).expect("same search again");
    let second = results(&ctx);
    assert_eq!(
        first.len(),
        second.len(),
        "running the same search twice must not accumulate rows"
    );

    // A query that cannot match must clear the set, not leave the old rows behind.
    let out = search_management_commands::run_search(&ctx, &dto(&ctx, "zzqqxx-not-in-any-manuscript"))
        .expect("miss");
    assert_eq!(out.match_count, 0);
    assert!(
        results(&ctx).is_empty(),
        "a query with no hits must leave NO rows — a stale result set is a lie"
    );

    // And an empty query clears it too (the UI sends this when the field is emptied).
    search_management_commands::run_search(&ctx, &dto(&ctx, "ipsum")).expect("repopulate");
    assert!(!results(&ctx).is_empty());
    search_management_commands::run_search(&ctx, &dto(&ctx, "")).expect("empty query");
    assert!(
        results(&ctx).is_empty(),
        "clearing the query must clear the results"
    );
}

/// The `Search` entity must be an honest record of what produced the rows sitting
/// next to it — a stale query beside a fresh result set would mislead every consumer.
#[test]
fn the_search_entity_mirrors_the_query_that_produced_it() {
    let ctx = loaded_ctx();
    let mut d = dto(&ctx, "ipsum");
    d.case_sensitive = true;
    d.search_titles = false;
    search_management_commands::run_search(&ctx, &d).expect("run_search");

    let work_info = work_info_commands::get_all_work_info(&ctx)
        .expect("get_all_work_info")
        .into_iter()
        .next()
        .unwrap();
    let search = search_commands::get_search(&ctx, &work_info.search)
        .unwrap()
        .unwrap();
    assert_eq!(search.query, "ipsum");
    assert!(search.case_sensitive);
    assert!(!search.search_titles);
}

/// Scopes actually scope. Body-only and title-only searches must not return each
/// other's rows.
#[test]
fn scopes_restrict_what_is_searched() {
    let ctx = loaded_ctx();
    use frontend::common::entities::MatchField;

    let mut body_only = dto(&ctx, "ipsum");
    body_only.search_titles = false;
    body_only.search_synopsis = false;
    search_management_commands::run_search(&ctx, &body_only).expect("body-only");
    let body_rows = results(&ctx);
    // Guard against a vacuous pass: a scope test over an EMPTY result set asserts
    // nothing at all.
    assert!(
        !body_rows.is_empty(),
        "the body-only search must actually find something, or this test proves nothing"
    );
    for row in body_rows {
        assert_eq!(
            row.match_field,
            MatchField::Body,
            "a body-only search returned a {:?} row",
            row.match_field
        );
    }

    let mut titles_only = dto(&ctx, "Chapter");
    titles_only.search_body = false;
    titles_only.search_synopsis = false;
    search_management_commands::run_search(&ctx, &titles_only).expect("title-only");
    let title_rows = results(&ctx);
    assert!(
        !title_rows.is_empty(),
        "the title-only search must actually find something"
    );
    for row in title_rows {
        assert_eq!(
            row.match_field,
            MatchField::Title,
            "a title-only search returned a {:?} row",
            row.match_field
        );
    }
}

/// Closing a project must sweep its search surface away with the rest of the Work.
///
/// `Search` is a *strong* child of `WorkInfo` and `SearchResult` a strong child of
/// `Search`, so the cascade should do this for free — but "should" is not "does", and
/// a missed sweep is a silent leak that only shows up after a few hundred open/close
/// cycles in one long-running process. So assert it, rather than trust the manifest.
#[test]
fn closing_a_project_tears_down_its_search_surface() {
    use frontend::commands::search_result_commands;

    let ctx = loaded_ctx();
    search_management_commands::run_search(&ctx, &dto(&ctx, "ipsum")).expect("run_search");
    assert!(
        !results(&ctx).is_empty(),
        "the fixture must produce rows for this test to mean anything"
    );
    assert!(
        !search_result_commands::get_all_search_result(&ctx)
            .unwrap()
            .is_empty()
    );

    let work_id = work_commands::get_all_work(&ctx)
        .expect("get_all_work")
        .into_iter()
        .next()
        .expect("a loaded project has exactly one Work")
        .id;
    work_management_commands::close_work(&ctx, &CloseWorkDto { work_id }).expect("close_work");

    assert!(
        work_info_commands::get_all_work_info(&ctx)
            .unwrap()
            .is_empty(),
        "close_work removes the WorkInfo"
    );
    assert!(
        search_commands::get_all_search(&ctx).unwrap().is_empty(),
        "...and the Search hanging off it must go with it"
    );
    assert!(
        search_result_commands::get_all_search_result(&ctx)
            .unwrap()
            .is_empty(),
        "...and every SearchResult under that Search — otherwise every open/close \
         cycle leaks the last search's rows"
    );
}

/// Searching a project that contains Turkish prose must not take the backend down.
///
/// `'İ'.to_lowercase()` is TWO chars, so a match offset computed in a lowercased
/// haystack is wrong — and, with enough `İ` before the match, past the end — when
/// applied to the source. This used to panic outright
/// ("range end index 21 out of range for slice of length 16"), and case-insensitive is
/// the DEFAULT, so any Turkish scene crashed the very first search.
#[test]
fn a_case_insensitive_search_over_turkish_prose_does_not_panic() {
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;

    let ctx = loaded_ctx();

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
                .find(|c| c.role == ContentRole::SceneText)
        })
        .expect("an item with a SceneText content row");
    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: "İİİİİİİİİİ ipsum".to_string(),
        },
    )
    .unwrap();

    // The search itself is the assertion: it must return, not panic.
    let mut q = dto(&ctx, "ipsum");
    q.search_titles = false;
    q.search_synopsis = false;
    let out = search_management_commands::run_search(&ctx, &q).expect("run_search");
    assert!(
        out.match_count > 0,
        "the Turkish scene's 'ipsum' must still be found"
    );

    // And the snippet must point at the match in the SOURCE, not at a shifted offset.
    let row = results(&ctx)
        .into_iter()
        .find(|r| r.snippet_before.contains('İ'))
        .expect("the Turkish row");
    assert_eq!(
        row.snippet_match, "ipsum",
        "the matched span must be the match itself, not text shifted by the case fold"
    );
}

/// **`whole_word` is no longer a dead flag.**
///
/// The DTO accepted it and the `Search` entity stored it, but nothing read it — so the
/// moment the UI bound a checkbox to it, the checkbox would have silently done nothing.
/// It now reaches `text_document::matching`, the same matcher the editor's find uses.
///
/// And because that matcher treats an apostrophe as a word boundary, whole-word `Elena`
/// finds `Elena's` — the miss that, in a Replace All, is a half-renamed manuscript.
#[test]
fn whole_word_reaches_the_matcher_and_finds_the_possessive() {
    use frontend::commands::content_commands;
    use frontend::common::entities::ContentRole;

    let ctx = loaded_ctx();

    // Put a known sentence into a real scene: one standalone name, one possessive, and
    // one word that merely CONTAINS the name.
    let victim = binder_item_commands::get_all_binder_item(&ctx)
        .unwrap()
        .into_iter()
        .filter(|i| i.activated)
        .find_map(|i| {
            let cids = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &i.id,
                &frontend::common::direct_access::binder_item::BinderItemRelationshipField::Contents,
            )
            .unwrap();
            content_commands::get_content_multi(&ctx, &cids)
                .unwrap()
                .into_iter()
                .flatten()
                .find(|c| c.role == ContentRole::SceneText)
        })
        .expect("a scene with body text");

    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: "Elena went home. Elena's coat stayed. Elenamania spread.".to_string(),
        },
    )
    .unwrap();

    let mut q = dto(&ctx, "Elena");
    q.search_titles = false;
    q.search_synopsis = false;
    q.whole_word = true;

    let out = search_management_commands::run_search(&ctx, &q).expect("run_search");
    assert_eq!(
        out.match_count, 2,
        "whole-word `Elena` must match the standalone name AND the possessive, but NOT \
         `Elenamania` — if this is 3 the flag is still being ignored; if it is 1 the \
         apostrophe is still gluing the possessive into one word"
    );
}

/// **The corpus is prose, not markup.**
///
/// `Content.data` is Djot. Scanning it raw matched text the writer never wrote and cannot
/// see — a link's URL, an emphasis marker, a heading's `#`. Worse, a count taken from the
/// markup would not agree with what `replace_in_project` re-derives inside the parsed
/// document, so its "the text moved under me" guard would fire on perfectly good rows.
#[test]
fn the_search_reads_the_prose_and_not_the_djot_markup() {
    use frontend::commands::content_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;

    let ctx = loaded_ctx();

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
                .find(|c| c.role == ContentRole::SceneText)
        })
        .expect("a scene with body text");

    // Prose that contains `Aurélien` ONCE, plus a link whose URL also contains it — and a
    // heading, and emphasis markers.
    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            // The name appears ONCE in the prose, and once more inside the link's
            // destination — where the writer cannot see it. A raw-markup scan finds two.
            data: "## A chapter\n\nShe called *Aurélien* and read \
                   [the note](https://example.test/Aurélien-notes)."
                .to_string(),
        },
    )
    .unwrap();

    let mut body_only = dto(&ctx, "Aurélien");
    body_only.search_titles = false;
    body_only.search_synopsis = false;
    let out = search_management_commands::run_search(&ctx, &body_only).expect("run_search");
    assert_eq!(
        out.match_count, 1,
        "`Aurélien` occurs once in the PROSE; if this is 2 the search is still reading \
         the Djot source and matched the link's URL as well"
    );

    // And the markup itself must be unreachable.
    for markup in ["https", "example.test", "##"] {
        let mut q = dto(&ctx, markup);
        q.search_titles = false;
        q.search_synopsis = false;
        let out = search_management_commands::run_search(&ctx, &q).expect("run_search");
        assert_eq!(
            out.match_count, 0,
            "{markup:?} is markup, not prose — the writer never typed it and must not be \
             shown a result for it"
        );
    }
}
