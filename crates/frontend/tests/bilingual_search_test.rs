// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A manuscript is not monolingual, and the language decides what folding **means**.
//!
//! One `Work`, two scenes: one French, one Turkish. In Turkish the dotted `i` and the
//! dotless `ı` are **different letters** — `Ilse` and `İlse` are different words, and the
//! uppercase of `i` is `İ`, not `I`. So a single search must fold the two scenes under
//! *different* rules, and a single rename must uppercase them under different rules too.
//!
//! Get either half wrong and there is no error, no warning, and no way for the writer to
//! notice until they read it: their Turkish prose has simply been rewritten into other
//! words. That is why the language is resolved **per item** (its own tag, else the Work's) and
//! not per search.
//!
//! The user's toggles (`case_sensitive`, `diacritic_sensitive`) stay **global** throughout:
//! the language decides *how* to fold, never *whether* to — or the same checkbox would mean
//! different things in different chapters of one book.

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, content_commands, search_commands, search_management_commands,
    search_result_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::search::SearchRelationshipField;
use frontend::direct_access::{BinderItemDto, ContentDto};
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

fn search(ctx: &AppContext, query: &str) -> RunSearchDto {
    let work_id = work_commands::get_all_work(ctx).expect("get_all_work").pop().unwrap().id;
    RunSearchDto {
        work_id,
        query: query.to_string(),
        case_sensitive: false,
        whole_word: false,
        diacritic_sensitive: false,
        facets: vec![],
        search_body: true,
        search_titles: false,
        search_synopsis: false,
        search_labels: false,
        include_trashed: false,
    }
}

/// Give the Work a language.
fn set_work_language(ctx: &AppContext, tag: &str) {
    let mut work = work_commands::get_all_work(ctx).unwrap().pop().unwrap();
    work.dict_language = skribisto_model::language::parse_legacy_list(tag);
    work_commands::update_work(ctx, None, &work.into()).expect("update_work");
}

/// Tag one item with its own language.
fn set_item_language(ctx: &AppContext, item: &BinderItemDto, tag: &str) {
    let mut item = item.clone();
    item.dict_language = skribisto_model::language::parse_legacy_list(tag);
    binder_item_commands::update_binder_item(ctx, None, &item.into()).expect("update_binder_item");
}

/// Overwrite one scene's prose.
fn write_prose(ctx: &AppContext, content: &ContentDto, djot: &str) {
    let mut content = content.clone();
    content.data = djot.to_string();
    content_commands::update_content(ctx, None, &content.into()).expect("update_content");
}

/// The first `Content` of an item that holds body prose.
fn body_of(ctx: &AppContext, item: &BinderItemDto) -> ContentDto {
    content_commands::get_content_multi(ctx, &item.contents)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| {
            matches!(
                c.role,
                frontend::common::entities::ContentRole::SceneText
                    | frontend::common::entities::ContentRole::NoteText
            )
        })
        .expect("the item must have a body Content row")
}

/// Two items that both carry body prose — one will be French, one Turkish.
fn two_prose_items(ctx: &AppContext) -> (BinderItemDto, BinderItemDto) {
    let mut items: Vec<BinderItemDto> = binder_item_commands::get_all_binder_item(ctx)
        .unwrap()
        .into_iter()
        .filter(|i| i.activated)
        .filter(|i| {
            content_commands::get_content_multi(ctx, &i.contents)
                .map(|cs| {
                    cs.into_iter().flatten().any(|c| {
                        matches!(
                            c.role,
                            frontend::common::entities::ContentRole::SceneText
                                | frontend::common::entities::ContentRole::NoteText
                        )
                    })
                })
                .unwrap_or(false)
        })
        .collect();
    items.sort_by_key(|i| i.id);
    assert!(
        items.len() >= 2,
        "the fixture must have at least two items with prose"
    );
    let second = items.pop().unwrap();
    let first = items.remove(0);
    (first, second)
}

fn result_rows(ctx: &AppContext) -> Vec<frontend::direct_access::SearchResultDto> {
    let wi = frontend::commands::work_info_commands::get_all_work_info(ctx)
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let ids = search_commands::get_search_relationship(
        ctx,
        &wi.search,
        &SearchRelationshipField::Results,
    )
    .unwrap();
    search_result_commands::get_search_result_multi(ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .collect()
}

/// Set the scene: a French Work, one scene tagged Turkish. Both scenes contain the same
/// three spellings, so *only* the language can explain any difference in what is found.
fn bilingual_ctx() -> (AppContext, BinderItemDto, BinderItemDto) {
    let ctx = loaded_ctx();
    set_work_language(&ctx, "fr-FR");

    let (french, turkish) = two_prose_items(&ctx);
    set_item_language(&ctx, &turkish, "tr-TR");

    // Identical prose in both scenes. `İlse` and `İLSE` are the dotted word; `Ilse` is the
    // dotless one, which in Turkish is a different word entirely.
    let prose = "İlse geldi. İLSE bağırdı. Ilse başka bir kelime.";
    write_prose(&ctx, &body_of(&ctx, &french), prose);
    write_prose(&ctx, &body_of(&ctx, &turkish), prose);

    (ctx, french, turkish)
}

/// One search, two languages. The French scene folds `I` and `İ` together and finds all
/// three; the Turkish scene keeps them apart and finds only the two dotted ones.
#[test]
fn one_search_folds_two_scenes_under_their_own_rules() {
    let (ctx, french, turkish) = bilingual_ctx();

    search_management_commands::run_search(&ctx, &search(&ctx, "ilse")).expect("run_search");
    let rows = result_rows(&ctx);

    let french_row = rows
        .iter()
        .find(|r| r.binder_item_id == french.id)
        .expect("the French scene must match");
    let turkish_row = rows
        .iter()
        .find(|r| r.binder_item_id == turkish.id)
        .expect("the Turkish scene must match");

    assert_eq!(
        french_row.occurrence_count, 3,
        "untailored, `I` and `İ` both fold onto `i` — all three spellings are the same word"
    );
    assert_eq!(
        turkish_row.occurrence_count, 2,
        "in Turkish the dotless `Ilse` is a DIFFERENT word: only `İlse` and `İLSE` match"
    );
}

/// …and the rename that follows uppercases each scene under its own rules. The untailored
/// uppercase of `i` is `I`; in Turkish it is `İ`. A case-preserver blind to that would write
/// `IRENE` into Turkish prose — the capital of a different letter, and a different word.
#[test]
fn one_rename_uppercases_each_scene_under_its_own_rules() {
    let (ctx, french, turkish) = bilingual_ctx();

    search_management_commands::run_search(&ctx, &search(&ctx, "ilse")).expect("run_search");
    let out = search_management_commands::replace_in_project(
        &ctx,
        None,
        &ReplaceInProjectDto {
            work_id: work_commands::get_all_work(&ctx).expect("get_all_work").pop().unwrap().id,
            replacement: "irene".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
        },
    )
    .expect("replace_in_project");
    assert_eq!(out.items_changed, 2);

    let french_prose = content_commands::get_content(&ctx, &body_of(&ctx, &french).id)
        .unwrap()
        .unwrap()
        .data;
    let turkish_prose = content_commands::get_content(&ctx, &body_of(&ctx, &turkish).id)
        .unwrap()
        .unwrap()
        .data;

    assert_eq!(
        french_prose.trim(),
        "Irene geldi. IRENE bağırdı. Irene başka bir kelime.",
        "untailored: all three renamed, and `i` uppercases to `I`"
    );
    assert_eq!(
        turkish_prose.trim(),
        "İrene geldi. İRENE bağırdı. Ilse başka bir kelime.",
        "Turkish: `i` uppercases to `İ`, and the dotless `Ilse` is a different word — it \
         must be left standing"
    );
}

/// The fold reaches the plain-string fields too, and they are folded under the item's
/// language just like its prose. A title is not a document — it takes a string rewrite, not
/// a parser splice — so it is the easiest place for the language to be quietly dropped.
#[test]
fn an_items_title_is_folded_under_the_items_own_language() {
    let ctx = loaded_ctx();
    set_work_language(&ctx, "fr-FR");
    let (french, turkish) = two_prose_items(&ctx);
    set_item_language(&ctx, &turkish, "tr-TR");

    let mut fr = french.clone();
    fr.title = "Ilse rentre".to_string();
    binder_item_commands::update_binder_item(&ctx, None, &fr.into()).unwrap();
    let mut tr = turkish.clone();
    tr.title = "Ilse döndü".to_string();
    tr.dict_language = skribisto_model::language::parse_legacy_list("tr-TR");
    binder_item_commands::update_binder_item(&ctx, None, &tr.into()).unwrap();

    let titles_only = RunSearchDto {
        search_body: false,
        search_titles: true,
        ..search(&ctx, "ilse")
    };
    search_management_commands::run_search(&ctx, &titles_only).expect("run_search");
    let rows = result_rows(&ctx);

    assert!(
        rows.iter().any(|r| r.binder_item_id == french.id),
        "the French title `Ilse rentre` must match `ilse`"
    );
    assert!(
        !rows.iter().any(|r| r.binder_item_id == turkish.id),
        "the Turkish title spells the DOTLESS `Ilse`, which is not the word `ilse` — the \
         language must reach the plain-string fields too, not just the parsed prose"
    );
}

/// A malformed language tag must degrade to an untailored search, never break one. It comes
/// from a writer's project settings, and a typo there must not stop them finding anything.
#[test]
fn a_malformed_language_tag_still_searches() {
    let ctx = loaded_ctx();
    set_work_language(&ctx, "!! not a tag !!");
    let (item, _) = two_prose_items(&ctx);
    set_item_language(&ctx, &item, "-----");
    write_prose(&ctx, &body_of(&ctx, &item), "Le café était froid.");

    search_management_commands::run_search(&ctx, &search(&ctx, "cafe")).expect("run_search");
    let rows = result_rows(&ctx);
    assert!(
        rows.iter().any(|r| r.binder_item_id == item.id),
        "a malformed tag must fold untailored — `cafe` still finds `café`"
    );
}
