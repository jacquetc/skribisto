// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The panel had no tests at all until its sections were split out.
//!
//! It is the densest widget in the application — six built-in sections, each
//! gated on the focused item's sub-role, plus however many an extension has
//! contributed — and until now the only thing standing behind "it still builds
//! under a Part" was somebody opening the app and clicking a Part.

use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, GoalUnit};
use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
use skribisto_model::counting::CountingMethodSetting;
use teksilo::prelude::*;
use teksilo::widgets::FixedSize;

use super::super::inspector_sections::{
    InspectorSectionSpec, register_inspector_section, registered_for,
};
use super::Inspector;

/// A Work with one binder holding one item of `sub_role`, and the item's id.
fn work_with_item(ctx: &Rc<AppContext>, sub_role: BinderItemSubRole) -> (u64, u64) {
    let work = work_commands::create_orphan_work(ctx, None, &CreateWorkDto::default()).unwrap();
    let binder = binder_commands::create_binder(
        ctx,
        None,
        &CreateBinderDto {
            name: "Manuscript".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .unwrap();
    let item = binder_item_commands::create_binder_item(
        ctx,
        None,
        &CreateBinderItemDto {
            title: "A row".into(),
            role: if matches!(
                sub_role,
                BinderItemSubRole::Scene
                    | BinderItemSubRole::Note
                    | BinderItemSubRole::ChapterScene
            ) {
                BinderItemRole::Item
            } else {
                BinderItemRole::Folder
            },
            sub_role,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
        binder.id,
        0,
    )
    .unwrap();
    (work.id, item.id)
}

fn panel(ctx: &Rc<AppContext>, work_id: u64, focus: Signal<Option<u64>>) -> Inspector {
    let ids = crate::app_ids::AppIds::new();
    ids.work_id.set(Some(work_id));
    let outline = crate::view_models::OutlineViewModel::new_default(ctx.clone(), ids.clone());
    Inspector::new(
        ctx.clone(),
        outline,
        focus,
        crate::view_models::TagsViewModel::new(
            crate::models::WorkTagsListModel::new(ctx.clone(), ids.clone()),
            ids.clone(),
        ),
        crate::view_models::MentionIndex::new(ctx.clone(), ids.clone()),
        crate::models::OpenDocsStore::new(ctx.clone()),
        Signal::new(CountingMethodSetting::default()),
        Signal::new(GoalUnit::Words),
    )
}

/// The sub-role the panel will actually see for `item_id`.
///
/// Asked of a probe rather than assumed from what was seeded: under the `mocks`
/// feature `SingleBinderItem` fabricates its answer and never reads the store,
/// so a test that registered a section against the *seeded* sub-role would be
/// testing nothing there — and would fail, which is how this was found.
fn focused_sub_role(ctx: &Rc<AppContext>, item_id: u64) -> BinderItemSubRole {
    let probe = crate::singles::SingleBinderItem::new(ctx.clone());
    probe.set_id(Some(item_id));
    probe.dto().expect("the focused item resolves").sub_role
}

/// Build and lay the panel out, and hand back the tree with its root.
///
/// `tree_with_events`, never a bare `WidgetTree`: the panel calls
/// `ctx.subscribe_event` on every build, which *panics* with no event source
/// registered (see `crate::test_support`).
fn laid_out(
    ctx: &Rc<AppContext>,
    work_id: u64,
    focus: Option<u64>,
) -> (teksilo::core::widget_tree::WidgetTree, WidgetId) {
    let mut tree = crate::test_support::tree_with_events(ctx);
    let id = tree.add_boxed(Box::new(panel(ctx, work_id, Signal::new(focus))));
    tree.layout(SizeProposal::exact(300.0, 4000.0));
    (tree, id)
}

/// Every widget in the subtree that has no children of its own.
///
/// The panel fills whatever height it is given (`layout_response` defers to the
/// proposal), so the *panel's* bounds say nothing about how much is in it —
/// measuring it was the first version of these tests and it passed on an empty
/// panel. What is actually in it is the leaves.
fn leaves(tree: &teksilo::core::widget_tree::WidgetTree, root: WidgetId) -> Vec<(WidgetId, Rect)> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let kids = tree.children(id);
        if kids.is_empty() {
            out.push((id, tree.bounds(id)));
        } else {
            stack.extend(kids);
        }
    }
    out
}

/// Every sub-role the writing model allows must reach a laid-out panel.
///
/// The six built-in sections are each gated on the focused row's sub_role, and
/// three of those gates are `matches!` over an explicit variant list — the shape
/// that goes quietly wrong when the matrix gains a row. A panic here is the
/// point; the height assertion is the cheap extra.
#[test]
fn the_panel_builds_for_every_sub_role() {
    for sub_role in [
        BinderItemSubRole::Text,
        BinderItemSubRole::None,
        BinderItemSubRole::Note,
        BinderItemSubRole::Book,
        BinderItemSubRole::Part,
        BinderItemSubRole::Scene,
        BinderItemSubRole::ChapterScene,
        BinderItemSubRole::BookBegin,
        BinderItemSubRole::BookEnd,
        BinderItemSubRole::Paratext,
    ] {
        let ctx = Rc::new(AppContext::new());
        let (work_id, item_id) = work_with_item(&ctx, sub_role.clone());
        let (tree, root) = laid_out(&ctx, work_id, Some(item_id));
        assert!(
            !leaves(&tree, root).is_empty(),
            "{sub_role:?} laid out to nothing"
        );
    }
}

/// With nothing focused the panel is the placeholder, not an error and not a
/// zero-height nothing.
#[test]
fn an_unfocused_panel_is_the_placeholder() {
    let ctx = Rc::new(AppContext::new());
    let (work_id, item_id) = work_with_item(&ctx, BinderItemSubRole::Scene);
    let (empty_tree, empty_root) = laid_out(&ctx, work_id, None);
    let empty = leaves(&empty_tree, empty_root).len();
    let (tree, root) = laid_out(&ctx, work_id, Some(item_id));
    let focused = leaves(&tree, root).len();
    assert_eq!(empty, 1, "the placeholder is one line of text");
    assert!(
        focused > empty,
        "a focused Scene ({focused} leaves) should carry more than the placeholder ({empty})"
    );
}

/// A contributed section renders **under** everything the application builds,
/// never above it and never instead of it.
///
/// The ordering is the registry's stated contract and the panel's half of it is
/// one `for` loop placed after the built-in body — which is exactly the kind of
/// thing a refactor moves by accident. Measured rather than asserted on the
/// registry: `registered_for` returning the section proves the *registry* works,
/// not that the panel put it last.
#[test]
fn a_contributed_section_lands_after_the_built_in_body() {
    let ctx = Rc::new(AppContext::new());
    let (work_id, item_id) = work_with_item(&ctx, BinderItemSubRole::Scene);

    const TALL: f32 = 400.0;
    let _handle = register_inspector_section(
        "test.after",
        InspectorSectionSpec {
            id: "test.after".into(),
            label: Rc::new(|| lit!("Contributed".to_string())),
            view: Rc::new(|_cx| Box::new(FixedSize::new().width(100.0).height(TALL))),
            shows_on: {
                let shown = focused_sub_role(&ctx, item_id);
                Rc::new(move |s: &BinderItemSubRole| *s == shown)
            },
        },
    )
    .expect("a free namespace and a free id");

    let (tree, root) = laid_out(&ctx, work_id, Some(item_id));
    let all = leaves(&tree, root);
    // Nothing the panel builds is 400 px tall, so the height identifies it.
    let body = all
        .iter()
        .find(|(_, b)| (b.height - TALL).abs() < 0.5)
        .map(|(_, b)| *b)
        .expect("the contributed section is in the tree");

    // Last means bottom-most: every other leaf starts above this one.
    for (_, other) in &all {
        if (other.height - TALL).abs() < 0.5 {
            continue;
        }
        assert!(
            other.y <= body.y,
            "a built-in leaf at y={} sits below the contributed section at y={}",
            other.y,
            body.y
        );
    }
}

/// `shows_on` reaches the panel: a section that declines a sub-role is absent
/// from it, not merely absent from the registry's own filtered list.
#[test]
fn a_section_that_declines_a_sub_role_is_not_built() {
    let ctx = Rc::new(AppContext::new());
    let (work_id, note_id) = work_with_item(&ctx, BinderItemSubRole::Note);

    const TALL: f32 = 400.0;
    let _handle = register_inspector_section(
        "test.scene_only",
        InspectorSectionSpec {
            id: "test.scene_only".into(),
            label: Rc::new(|| lit!("Scenes only".to_string())),
            view: Rc::new(|_cx| Box::new(FixedSize::new().width(100.0).height(TALL))),
            shows_on: {
                let refused = focused_sub_role(&ctx, note_id);
                Rc::new(move |s: &BinderItemSubRole| *s != refused)
            },
        },
    )
    .expect("a free namespace and a free id");

    let shown = focused_sub_role(&ctx, note_id);
    assert!(registered_for(&shown).is_empty());

    // Look for the section's own body rather than counting leaves: under the
    // `mocks` feature the fabricated models do not answer identically on every
    // build, and a count would call that difference a contributed section.
    let (tree, root) = laid_out(&ctx, work_id, Some(note_id));
    assert!(
        !leaves(&tree, root)
            .iter()
            .any(|(_, b)| (b.height - TALL).abs() < 0.5),
        "a Scene-only section was built under a focused Note"
    );
}
