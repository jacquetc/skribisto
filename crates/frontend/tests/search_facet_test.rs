// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Show me only the scenes."
//!
//! The constraint matrix has **twelve** `(role, sub_role)` combinations. A writer filtering
//! their search results does not think in twelve — they think "scenes", "notes", "chapters".
//! And a chapter is a chapter whether the project stores its chapters flat or as folders,
//! which is a storage decision they made once, when they created the project, and should
//! never have to remember again.
//!
//! `skribisto_model::SearchFacet` is where those twelve collapse onto six, and it is derived
//! *from* the matrix so a thirteenth combination cannot quietly become unfindable. These tests
//! pin that the collapse actually reaches the search.

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, content_commands, search_commands, search_management_commands,
    search_result_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::search::SearchRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
use search_management::RunSearchDto;
use skribisto_model::SearchFacet;
use work_management::{NewWorkDto, NewWorkTemplate};

/// The same word, in every kind of thing the matrix allows to hold prose or a title.
const NEEDLE: &str = "Aurélien";

fn item(role: BinderItemRole, sub_role: BinderItemSubRole, title: &str) -> CreateBinderItemDto {
    let now = chrono::Utc::now();
    CreateBinderItemDto {
        // Derived from the title: this helper builds every row in the
        // fixture, so a single constant would give them all one identity.
        uid: common::uid::fixture_uid(title.len() as u64 * 1000 + title.chars().map(|c| c as u64).sum::<u64>()),
        created_at: now,
        updated_at: now,
        title: title.to_string(),
        sub_title: String::new(),
        role,
        sub_role,
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
    }
}

/// One project holding **one of each kind**, every one of them named after the same
/// character. Whatever a facet filters out, it must filter out on purpose.
fn ctx_with_one_of_each() -> (AppContext, Vec<(BinderItemSubRole, u64)>) {
    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-facet-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["fr-FR".to_string()],
            chapter_scene_mode: false,
        },
    )
    .expect("new_work");
    let _ = std::fs::remove_dir_all(&dir);

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap();
    let binder =
        work_commands::get_work_relationship(&ctx, &work.id, &WorkRelationshipField::Binders)
            .unwrap()
            .pop()
            .unwrap();

    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole as S;
    // Every row of the matrix that a writer can meet, in stream order.
    let kinds = [
        (Folder, S::Book),
        (Item, S::BookBegin),
        (Folder, S::Part),
        (Item, S::Part),
        (Folder, S::ChapterScene),
        (Item, S::ChapterScene),
        (Item, S::Scene),
        (Item, S::Note),
        (Folder, S::Note),
        (Folder, S::None),
        (Item, S::Text),
        (Item, S::BookEnd),
    ];

    let dtos: Vec<CreateBinderItemDto> = kinds
        .iter()
        .map(|(r, s)| item(r.clone(), s.clone(), &format!("{NEEDLE} — {s:?}")))
        .collect();
    let created =
        binder_item_commands::create_binder_item_multi(&ctx, None, &dtos, binder, -1).unwrap();

    // The scene gets prose too, so both the title scope and the body scope are exercised.
    let scene = created
        .iter()
        .zip(kinds.iter())
        .find(|(_, (_, s))| *s == S::Scene)
        .map(|(c, _)| c.id)
        .unwrap();
    let now = chrono::Utc::now();
    content_commands::create_content_multi(
        &ctx,
        None,
        &[CreateContentDto {
            created_at: now,
            updated_at: now,
            activated: true,
            role: ContentRole::SceneText,
            data: format!("{NEEDLE} traversa la forêt."),
        }],
        scene,
        -1,
    )
    .unwrap();

    let ids = created
        .iter()
        .zip(kinds.iter())
        .map(|(c, (_, s))| (s.clone(), c.id))
        .collect();
    (ctx, ids)
}

fn search(facets: Vec<i64>) -> RunSearchDto {
    RunSearchDto {
        query: "aurelien".to_string(), // plain ASCII — the fold finds the accented name
        case_sensitive: false,
        whole_word: false,
        diacritic_sensitive: false,
        facets,
        search_body: true,
        search_titles: true,
        search_synopsis: true,
        search_labels: false,
        include_trashed: false,
    }
}

fn hit_ids(ctx: &AppContext) -> Vec<u64> {
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
    let mut out: Vec<u64> = search_result_commands::get_search_result_multi(ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .map(|r| r.binder_item_id)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// **No chips ticked means no filter.** A filter with nothing selected does not select
/// nothing — it selects everything, which is what a reader means by it. Get this backwards and
/// the search panel opens showing zero results and the writer concludes it is broken.
#[test]
fn an_empty_facet_list_filters_nothing() {
    let (ctx, kinds) = ctx_with_one_of_each();
    search_management_commands::run_search(&ctx, &search(vec![])).unwrap();
    assert_eq!(
        hit_ids(&ctx).len(),
        kinds.len(),
        "every kind is named after the character, so every kind must be found"
    );
}

/// One chip: only that kind.
#[test]
fn a_single_chip_keeps_only_that_kind() {
    let (ctx, kinds) = ctx_with_one_of_each();
    search_management_commands::run_search(&ctx, &search(vec![SearchFacet::Scene.code() as i64]))
        .unwrap();

    let scene = kinds
        .iter()
        .find(|(s, _)| *s == BinderItemSubRole::Scene)
        .map(|(_, id)| *id)
        .unwrap();
    assert_eq!(hit_ids(&ctx), vec![scene]);
}

/// **A chapter is a chapter in either encoding.** The `Chapter` chip must find both the flat
/// `Item/ChapterScene` and the `Folder/ChapterScene` — the difference between them is a
/// storage decision the writer made once and does not think about.
#[test]
fn the_chapter_chip_finds_both_encodings_of_a_chapter() {
    let (ctx, kinds) = ctx_with_one_of_each();
    search_management_commands::run_search(&ctx, &search(vec![SearchFacet::Chapter.code() as i64]))
        .unwrap();

    let mut expected: Vec<u64> = kinds
        .iter()
        .filter(|(s, _)| *s == BinderItemSubRole::ChapterScene)
        .map(|(_, id)| *id)
        .collect();
    expected.sort_unstable();
    assert_eq!(expected.len(), 2, "the fixture has both encodings");
    assert_eq!(hit_ids(&ctx), expected);
}

/// …and the `Book` chip covers the container **and** both flat markers.
#[test]
fn the_book_chip_covers_the_container_and_the_markers() {
    let (ctx, kinds) = ctx_with_one_of_each();
    search_management_commands::run_search(&ctx, &search(vec![SearchFacet::Book.code() as i64]))
        .unwrap();

    let mut expected: Vec<u64> = kinds
        .iter()
        .filter(|(s, _)| {
            matches!(
                s,
                BinderItemSubRole::Book | BinderItemSubRole::BookBegin | BinderItemSubRole::BookEnd
            )
        })
        .map(|(_, id)| *id)
        .collect();
    expected.sort_unstable();
    assert_eq!(expected.len(), 3);
    assert_eq!(hit_ids(&ctx), expected);
}

/// Chips are a multi-select, and they union.
#[test]
fn several_chips_union() {
    let (ctx, kinds) = ctx_with_one_of_each();
    search_management_commands::run_search(
        &ctx,
        &search(vec![
            SearchFacet::Scene.code() as i64,
            SearchFacet::Note.code() as i64,
        ]),
    )
    .unwrap();

    let mut expected: Vec<u64> = kinds
        .iter()
        .filter(|(s, _)| matches!(s, BinderItemSubRole::Scene | BinderItemSubRole::Note))
        .map(|(_, id)| *id)
        .collect();
    expected.sort_unstable();
    assert_eq!(expected.len(), 3, "one scene, two kinds of note");
    assert_eq!(hit_ids(&ctx), expected);
}

/// Every one of the six chips finds something in a project holding one of each kind. A chip
/// that always came back empty would be a dead control the writer could not tell from a
/// genuine "no matches".
#[test]
fn every_chip_finds_something() {
    let (ctx, _) = ctx_with_one_of_each();
    for facet in SearchFacet::ALL {
        search_management_commands::run_search(&ctx, &search(vec![facet.code() as i64])).unwrap();
        assert!(
            !hit_ids(&ctx).is_empty(),
            "the {facet:?} chip found nothing in a project that holds one of every kind"
        );
    }
}

/// A code naming no facet is **ignored**, never an error. It can only come from a stale
/// `search.toml` written by a version whose codes differed.
#[test]
fn an_unknown_facet_code_is_ignored_not_fatal() {
    let (ctx, kinds) = ctx_with_one_of_each();

    // A real chip alongside a code that names nothing: the real chip still works.
    search_management_commands::run_search(
        &ctx,
        &search(vec![SearchFacet::Scene.code() as i64, 9999]),
    )
    .expect("an unknown code must not fail the search");
    let scene = kinds
        .iter()
        .find(|(s, _)| *s == BinderItemSubRole::Scene)
        .map(|(_, id)| *id)
        .unwrap();
    assert_eq!(hit_ids(&ctx), vec![scene]);
}

/// **An unrecognisable filter is no filter.** Dropping unknown codes one at a time is not
/// enough: if they are ALL unknown, the list is still non-empty, filtering still runs, and it
/// matches nothing — so a `search.toml` left over from an upgrade would silently leave the
/// writer unable to find their own prose, which is exactly what "ignored, never an error" is
/// supposed to promise.
#[test]
fn a_filter_of_nothing_but_unknown_codes_filters_nothing() {
    let (ctx, kinds) = ctx_with_one_of_each();

    search_management_commands::run_search(&ctx, &search(vec![9999, -1, 0])).expect("run_search");
    assert_eq!(
        hit_ids(&ctx).len(),
        kinds.len(),
        "no code named a facet, so there is no filter — every kind must still be found"
    );
}

/// An item whose (role, sub_role) is not in the matrix cannot be classified — and **must not
/// be hidden**. Filtering it out would make it findable with no chips ticked and invisible
/// with any chip ticked, which reads as the filter being broken rather than the item being
/// malformed, and leaves the writer no way to reach the thing and fix it.
#[test]
fn an_item_of_an_unclassifiable_kind_is_never_hidden_by_a_facet() {
    let (ctx, _) = ctx_with_one_of_each();

    // `Folder/Scene` is not a row of the matrix — `search_facet_of` returns None for it.
    // (The Plume importer and the legacy upgrader both synthesise sub_roles, so a shape the
    // matrix does not contain is not merely hypothetical.)
    assert_eq!(
        skribisto_model::search_facet_of(&BinderItemRole::Folder, &BinderItemSubRole::Scene),
        None,
        "the fixture depends on this combination being unclassifiable"
    );

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap();
    let binder =
        work_commands::get_work_relationship(&ctx, &work.id, &WorkRelationshipField::Binders)
            .unwrap()
            .pop()
            .unwrap();
    let odd = binder_item_commands::create_binder_item_multi(
        &ctx,
        None,
        &[item(
            BinderItemRole::Folder,
            BinderItemSubRole::Scene,
            &format!("{NEEDLE} — malformé"),
        )],
        binder,
        -1,
    )
    .unwrap()
    .pop()
    .unwrap();

    for facet in SearchFacet::ALL {
        search_management_commands::run_search(&ctx, &search(vec![facet.code() as i64])).unwrap();
        assert!(
            hit_ids(&ctx).contains(&odd.id),
            "the {facet:?} chip hid an item whose kind we cannot name — it is now unreachable"
        );
    }
}
