// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for the Full Chapter view's new backend use cases —
//! `merge_two_scenes` and `split_scene` — against a real in-memory store.

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, SplitSceneDto};
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    handling_app_lifecycle_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{
    CreateBinderDto, CreateBinderItemDto, CreateContentDto, CreateWorkDto,
};

/// A fresh store with one Work + one Binder; returns (ctx, work_id, binder_id).
fn setup() -> (AppContext, u64, u64) {
    let ctx = AppContext::new();
    let root = handling_app_lifecycle_commands::initialize_app(&ctx)
        .unwrap()
        .root_id;
    let work = work_commands::create_work(
        &ctx,
        None,
        &CreateWorkDto {
            title: "Test".into(),
            ..Default::default()
        },
        root,
        -1,
    )
    .unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "Manuscript".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        -1,
    )
    .unwrap();
    (ctx, work.id, binder.id)
}

/// Append a `Scene` item with `SceneText = text` to the binder; return its id.
fn make_scene(ctx: &AppContext, binder: u64, title: &str, text: &str) -> u64 {
    make_item(
        ctx,
        binder,
        BinderItemRole::Item,
        BinderItemSubRole::Scene,
        title,
        text,
        "",
    )
}

/// Append a chapter **folder** — the container encoding of a chapter, which carries its
/// own prose exactly like the flat `Item/ChapterScene` it promotes to.
fn make_chapter_folder(ctx: &AppContext, binder: u64, title: &str, text: &str) -> u64 {
    make_item(
        ctx,
        binder,
        BinderItemRole::Folder,
        BinderItemSubRole::ChapterScene,
        title,
        text,
        "",
    )
}

/// Append a `Scene` carrying **both** writing roles — for the split-from-synopsis
/// tests, which need a scene whose prose and synopsis are independently checkable.
fn make_scene_with_synopsis(
    ctx: &AppContext,
    binder: u64,
    title: &str,
    text: &str,
    synopsis: &str,
) -> u64 {
    make_item(
        ctx,
        binder,
        BinderItemRole::Item,
        BinderItemSubRole::Scene,
        title,
        text,
        synopsis,
    )
}

/// Append an item of `(role, sub_role)`, with the given (optional) `SceneText` /
/// `SynopsisText` rows. An empty string means "no row for that role".
///
/// The role is explicit because a chapter has two encodings that share one sub_role:
/// `Folder/ChapterScene` (extent by containment) and `Item/ChapterScene` (by marker).
fn make_item(
    ctx: &AppContext,
    binder: u64,
    role: BinderItemRole,
    sub_role: BinderItemSubRole,
    title: &str,
    text: &str,
    synopsis: &str,
) -> u64 {
    let item = binder_item_commands::create_binder_item(
        ctx,
        None,
        &CreateBinderItemDto {
            title: title.into(),
            role,
            sub_role,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
        binder,
        -1,
    )
    .unwrap();
    for (role, data) in [
        (ContentRole::SceneText, text),
        (ContentRole::SynopsisText, synopsis),
    ] {
        if data.is_empty() {
            continue;
        }
        content_commands::create_content(
            ctx,
            None,
            &CreateContentDto {
                activated: true,
                role,
                data: data.into(),
                ..Default::default()
            },
            item.id,
            -1,
        )
        .unwrap();
    }
    item.id
}

/// The item's `SceneText` data.
fn scene_text(ctx: &AppContext, item: u64) -> String {
    content_of(ctx, item, ContentRole::SceneText)
}

/// The item's `SynopsisText` data.
fn synopsis_text(ctx: &AppContext, item: u64) -> String {
    content_of(ctx, item, ContentRole::SynopsisText)
}

/// The item's data for `role` — empty when the row doesn't exist.
fn content_of(ctx: &AppContext, item: u64, role: ContentRole) -> String {
    let ids = binder_item_commands::get_binder_item_relationship(
        ctx,
        &item,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    content_commands::get_content_multi(ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| c.role == role)
        .map(|c| c.data)
        .unwrap_or_default()
}

fn binder_order(ctx: &AppContext, binder: u64) -> Vec<u64> {
    binder_commands::get_binder_relationship(ctx, &binder, &BinderRelationshipField::BinderItems)
        .unwrap()
}

#[test]
fn merge_two_scenes_concats_text_and_trashes_source() {
    let (ctx, work_id, binder) = setup();
    let a = make_scene(&ctx, binder, "A", "Alpha");
    let b = make_scene(&ctx, binder, "B", "Beta");

    binder_item_management_commands::merge_two_scenes(
        &ctx,
        None,
        &MergeTwoScenesDto {
            work_id,
            target_id: a,
            source_id: b,
        },
    )
    .unwrap();

    // A absorbed B's text after a blank line.
    assert_eq!(scene_text(&ctx, a), "Alpha\n\nBeta");
    // B is trashed (deactivated, so excluded from the flat stream).
    let b_dto = binder_item_commands::get_binder_item(&ctx, &b)
        .unwrap()
        .unwrap();
    assert!(!b_dto.activated, "merged-away scene should be trashed");
}

/// Merge is an *adjacent*-scene operation. Two scenes separated by a chapter
/// boundary must be rejected by the use case itself, not merely hidden by the UI —
/// the Full Part / Full Book streams are the first views whose row list spans
/// several chapters.
#[test]
fn merge_two_scenes_rejects_non_adjacent() {
    let (ctx, work_id, binder) = setup();
    let a = make_scene(&ctx, binder, "A", "Alpha");
    make_chapter_folder(&ctx, binder, "Chapter Two", "");
    let b = make_scene(&ctx, binder, "B", "Beta");

    let res = binder_item_management_commands::merge_two_scenes(
        &ctx,
        None,
        &MergeTwoScenesDto {
            work_id,
            target_id: a,
            source_id: b,
        },
    );
    assert!(
        res.is_err(),
        "merging across a chapter boundary must be rejected"
    );
    // Nothing was mutated.
    assert_eq!(scene_text(&ctx, a), "Alpha");
    assert_eq!(scene_text(&ctx, b), "Beta");
    assert!(
        binder_item_commands::get_binder_item(&ctx, &b)
            .unwrap()
            .unwrap()
            .activated
    );
}

/// A `Folder/Chapter` carries its own `SceneText` (the matrix says so — that is
/// what makes promote/demote to `Item/ChapterScene` lossless), so its prose is
/// splittable like any other prose-bearing row. The scene cut out of it must land
/// *inside* the chapter (indent + 1), as its first child — not after it.
#[test]
fn split_scene_works_on_a_chapter_folders_own_prose() {
    let (ctx, _work_id, binder) = setup();
    let ch = make_chapter_folder(&ctx, binder, "The Long Road", "FirstBeatSecondBeat");
    let ch_indent = binder_item_commands::get_binder_item(&ctx, &ch)
        .unwrap()
        .unwrap()
        .indent;

    binder_item_management_commands::split_scene(
        &ctx,
        None,
        &SplitSceneDto {
            source_id: ch,
            before_text: "FirstBeat".into(),
            after_text: "SecondBeat".into(),
            before_synopsis: String::new(),
            after_synopsis: String::new(),
            new_title: "New Scene".into(),
        },
    )
    .expect("a chapter folder's own prose is splittable");

    assert_eq!(scene_text(&ctx, ch), "FirstBeat");
    let order = binder_order(&ctx, binder);
    let new_id = order[order.iter().position(|&x| x == ch).unwrap() + 1];
    let new_item = binder_item_commands::get_binder_item(&ctx, &new_id)
        .unwrap()
        .unwrap();
    assert_eq!(scene_text(&ctx, new_id), "SecondBeat");
    assert_eq!(new_item.sub_role, BinderItemSubRole::Scene);
    assert_eq!(
        new_item.indent,
        ch_indent + 1,
        "a scene split out of a chapter folder becomes its child, not its sibling"
    );
}

/// The inverse: that same scene merges straight back into the chapter folder.
/// Split → merge must round-trip.
#[test]
fn merge_absorbs_a_scene_back_into_its_chapter_folder() {
    let (ctx, work_id, binder) = setup();
    let ch = make_chapter_folder(&ctx, binder, "The Long Road", "FirstBeat");
    let sc = make_scene(&ctx, binder, "Scene", "SecondBeat");

    binder_item_management_commands::merge_two_scenes(
        &ctx,
        None,
        &MergeTwoScenesDto {
            work_id,
            target_id: ch,
            source_id: sc,
        },
    )
    .expect("a chapter folder can absorb its first scene");

    assert_eq!(scene_text(&ctx, ch), "FirstBeat\n\nSecondBeat");
    assert!(
        !binder_item_commands::get_binder_item(&ctx, &sc)
            .unwrap()
            .unwrap()
            .activated
    );
}

/// Merging *away* a row that opens a structural section would destroy the
/// structure: a `ChapterScene` source would delete a chapter boundary, and a
/// `Folder/Chapter` source would orphan its child scenes. Both are rejected by the
/// use case, not merely hidden by the UI.
#[test]
fn merge_refuses_to_trash_a_structural_opener() {
    // Both encodings of a chapter open a section: the flat marker and the folder.
    for role in [BinderItemRole::Item, BinderItemRole::Folder] {
        let (ctx, work_id, binder) = setup();
        let a = make_scene(&ctx, binder, "A", "Alpha");
        let b = make_item(
            &ctx,
            binder,
            role.clone(),
            BinderItemSubRole::ChapterScene,
            "Chapter Two",
            "Beta",
            "",
        );

        let res = binder_item_management_commands::merge_two_scenes(
            &ctx,
            None,
            &MergeTwoScenesDto {
                work_id,
                target_id: a,
                source_id: b,
            },
        );
        assert!(
            res.is_err(),
            "merging away a {role:?}/ChapterScene would destroy a chapter boundary"
        );
        assert_eq!(scene_text(&ctx, a), "Alpha", "nothing was mutated");
        assert!(
            binder_item_commands::get_binder_item(&ctx, &b)
                .unwrap()
                .unwrap()
                .activated
        );
    }
}

#[test]
fn split_scene_splits_at_offset_into_two() {
    let (ctx, _work_id, binder) = setup();
    let a = make_scene_with_synopsis(&ctx, binder, "A", "HelloWorld", "the whole synopsis");

    // Split from the *prose* editor: prose is cut at the caret; the synopsis is
    // passed whole to the source and empty to the new scene.
    binder_item_management_commands::split_scene(
        &ctx,
        None,
        &SplitSceneDto {
            source_id: a,
            before_text: "Hello".into(),
            after_text: "World".into(),
            before_synopsis: "the whole synopsis".into(),
            after_synopsis: String::new(),
            new_title: "New Scene".into(),
        },
    )
    .unwrap();

    // Source keeps the before-text; a new scene follows it with the after-text.
    assert_eq!(scene_text(&ctx, a), "Hello");
    let order = binder_order(&ctx, binder);
    let pos = order.iter().position(|&x| x == a).unwrap();
    let new_id = order[pos + 1];
    assert_eq!(scene_text(&ctx, new_id), "World");
    // The synopsis stayed entirely on the source.
    assert_eq!(synopsis_text(&ctx, a), "the whole synopsis");
    assert_eq!(synopsis_text(&ctx, new_id), "");
}

/// The mirror image: splitting from the **synopsis** editor cuts the synopsis at
/// the caret and leaves the prose whole on the original scene.
#[test]
fn split_scene_splits_synopsis_leaving_prose_intact() {
    let (ctx, _work_id, binder) = setup();
    let a = make_scene_with_synopsis(&ctx, binder, "A", "the whole prose", "TheyMeetTheyFight");

    binder_item_management_commands::split_scene(
        &ctx,
        None,
        &SplitSceneDto {
            source_id: a,
            // The caller passes the untouched role whole to the source, empty to
            // the new scene — that *is* the encoding of "it stays on the source".
            before_text: "the whole prose".into(),
            after_text: String::new(),
            before_synopsis: "TheyMeet".into(),
            after_synopsis: "TheyFight".into(),
            new_title: "New Scene".into(),
        },
    )
    .unwrap();

    let order = binder_order(&ctx, binder);
    let pos = order.iter().position(|&x| x == a).unwrap();
    let new_id = order[pos + 1];

    // Source: prose intact, synopsis cut at the caret.
    assert_eq!(scene_text(&ctx, a), "the whole prose");
    assert_eq!(synopsis_text(&ctx, a), "TheyMeet");
    // New scene: the after-caret synopsis, and *no* prose.
    assert_eq!(synopsis_text(&ctx, new_id), "TheyFight");
    assert_eq!(
        scene_text(&ctx, new_id),
        "",
        "a synopsis split must not copy the source's prose onto the new scene"
    );
}

/// An emptied half must genuinely empty its row, not be skipped as a no-op —
/// otherwise a scene that already had prose would keep it after a synopsis split
/// moved the prose away. Exercises the update-existing-row branch with empty text.
#[test]
fn split_scene_empties_a_role_when_its_half_is_empty() {
    let (ctx, _work_id, binder) = setup();
    let a = make_scene_with_synopsis(&ctx, binder, "A", "prose", "AB");

    // Everything goes to the new scene: the source is left with empty halves.
    binder_item_management_commands::split_scene(
        &ctx,
        None,
        &SplitSceneDto {
            source_id: a,
            before_text: String::new(),
            after_text: "prose".into(),
            before_synopsis: String::new(),
            after_synopsis: "AB".into(),
            new_title: "New Scene".into(),
        },
    )
    .unwrap();

    assert_eq!(scene_text(&ctx, a), "", "existing row must be emptied");
    assert_eq!(synopsis_text(&ctx, a), "", "existing row must be emptied");
    let order = binder_order(&ctx, binder);
    let new_id = order[order.iter().position(|&x| x == a).unwrap() + 1];
    assert_eq!(scene_text(&ctx, new_id), "prose");
    assert_eq!(synopsis_text(&ctx, new_id), "AB");
}
