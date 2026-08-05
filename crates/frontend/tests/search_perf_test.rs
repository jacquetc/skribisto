// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What does a keystroke actually cost?
//!
//! `run_search` runs on every keystroke of a search box (behind a debounce). It walks every
//! scene in the manuscript, extracts the prose from its Djot, folds it, and scans it. On a
//! real novel that is thousands of rows, and all of it happens **synchronously on the UI
//! thread**.
//!
//! This is the measurement the corpus cache is designed against — run it with
//! `--nocapture -- --ignored` to see the numbers. The two `#[test]`s below it are the
//! guards that keep the answer honest.

use std::time::Instant;

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, content_commands, search_management_commands, work_commands,
    work_management_commands,
};
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
use search_management::RunSearchDto;
use work_management::{NewWorkDto, NewWorkTemplate};

/// ~75 words of prose per scene, with a little Djot markup so the parse is not trivial, and
/// one occurrence of the query word in roughly one scene in ten.
fn scene_prose(i: usize) -> String {
    let hit = if i.is_multiple_of(10) {
        "Aurélien"
    } else {
        "Elena"
    };
    format!(
        "{hit} traversa la forêt où l'ombre s'étirait entre les *hêtres*, et le vent \
         portait l'odeur du sel. Elle songeait à la _promesse_ faite au bord de l'eau, \
         celle qu'elle n'avait jamais su tenir, et le [souvenir](https://exemple.test/{i}) \
         revenait comme une marée lente et patiente qui ne demandait rien.\n\n\
         Le café refroidissait doucement sur la table de chêne pendant qu'elle relisait la \
         lettre, encore une fois, sans y trouver ce qu'elle cherchait vraiment."
    )
}

const SCENES: usize = 4000;

/// Build a manuscript of `SCENES` scenes — roughly a 300k-word novel.
fn big_manuscript() -> AppContext {
    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-perf-{}", std::process::id()));
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
    let items: Vec<CreateBinderItemDto> = (0..SCENES)
        .map(|i| CreateBinderItemDto {
            // Distinct per scene: this is inside `(0..SCENES).map(..)`.
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
                created_at: now,
                updated_at: now,
                activated: true,
                role: ContentRole::SceneText,
                data: scene_prose(i),
            }],
            item.id,
            -1,
        )
        .expect("create_content_multi");
    }

    let _ = std::fs::remove_dir_all(&dir);
    ctx
}

fn query(ctx: &AppContext, q: &str) -> RunSearchDto {
    let work_id = work_commands::get_all_work(ctx)
        .expect("get_all_work")
        .pop()
        .unwrap()
        .id;
    RunSearchDto {
        work_id,
        query: q.to_string(),
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

/// The measurement. Not a guard — run it by hand:
///
/// ```text
/// cargo test -p skribisto-frontend --test search_perf_test -- --ignored --nocapture
/// ```
#[test]
#[ignore = "measurement, not an assertion — run with --release --ignored --nocapture"]
fn how_much_does_a_keystroke_cost() {
    let built = Instant::now();
    let ctx = big_manuscript();
    eprintln!(
        "\n  built {SCENES} scenes (~{}k words) in {:?}",
        SCENES * 75 / 1000,
        built.elapsed()
    );

    // The FIRST search after the panel opens: nothing is parsed, nothing is folded. This is
    // the one the writer waits for, and the only one that pays the full price.
    search_management::corpus_cache::clear();
    let t = Instant::now();
    search_management_commands::run_search(&ctx, &query(&ctx, "aurelien")).expect("run_search");
    eprintln!("  cold  (empty cache)               {:?}", t.elapsed());

    // …and then a search box produces one search per prefix of what is typed. This is what
    // the writer's fingers actually generate, and the prose has not changed between any two
    // of them.
    for q in ["a", "au", "aur", "aure", "aurel", "aureli", "aurelien"] {
        let t = Instant::now();
        let out =
            search_management_commands::run_search(&ctx, &query(&ctx, q)).expect("run_search");
        eprintln!(
            "  run_search({q:>9?}) -> {:>5} matches in {:>4} items   {:?}",
            out.match_count,
            out.item_count,
            t.elapsed()
        );
    }
    eprintln!(
        "\n  cache holds {:.1} MB\n",
        search_management::corpus_cache::heap_size() as f64 / 1_048_576.0
    );
}

/// The guard. `run_search` is **synchronous, on the UI thread**, on every keystroke — so its
/// cost is a stall in the writer's typing, not a slow feature.
///
/// A warm search is the common case by far: the prose does not change while someone types a
/// query into a box. 50 ms is deliberately loose against a measured ~9 ms — it is here to
/// catch an order-of-magnitude regression (the cache silently missing, the fold's ASCII fast
/// path lost), not to police milliseconds on a loaded CI box.
/// **Release only.** A debug build is 10-30x slower and its timings mean nothing — a
/// threshold loose enough to pass there would be too loose to catch anything here.
#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "a perf guard is meaningless in a debug build — run with --release"
)]
fn a_warm_search_over_a_whole_novel_costs_a_few_milliseconds() {
    let ctx = big_manuscript();

    search_management_commands::run_search(&ctx, &query(&ctx, "aurelien")).unwrap();

    let t = Instant::now();
    for _ in 0..5 {
        search_management_commands::run_search(&ctx, &query(&ctx, "aurelien")).unwrap();
    }
    let per_search = t.elapsed() / 5;

    assert!(
        per_search < std::time::Duration::from_millis(50),
        "a warm search over {SCENES} scenes took {per_search:?}. It re-parses and re-folds \
         nothing — if this regressed, the corpus cache is missing or the fold's ASCII fast \
         path is gone"
    );
    eprintln!("  warm search over {SCENES} scenes: {per_search:?}");
}

/// …and a **cold** search — the first one after the panel opens, with nothing cached — must
/// still come in under the debounce, or the very first thing the writer sees is a stall.
#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "a perf guard is meaningless in a debug build — run with --release"
)]
fn a_cold_search_still_comes_in_under_the_debounce() {
    let ctx = big_manuscript();
    search_management::corpus_cache::clear();

    let t = Instant::now();
    search_management_commands::run_search(&ctx, &query(&ctx, "aurelien")).unwrap();
    let cold = t.elapsed();

    assert!(
        cold < std::time::Duration::from_millis(300),
        "a cold search over {SCENES} scenes took {cold:?}, which is past the 300 ms debounce"
    );
    eprintln!("  cold search over {SCENES} scenes: {cold:?}");
}
