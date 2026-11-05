// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every Tab stop a writer can reach on an editor tab has somewhere to be.
//!
//! A control laid out at nothing is still a Tab stop unless something takes it out, and
//! nothing about it looks wrong in the code that builds it. NVDA found two on Windows: an
//! untagged chapter's tag button (no area, by design of the row around it) and the
//! corkboard's filter (no area, because the column was narrow). So this walks the tabs
//! rather than those two: a chapter, a book in each of its views, a corkboard, an overview.
//!
//! Real backend only: under `--features mocks` the tabs show fabricated rows of other shapes.

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use teksilo::core::widget_id::WidgetId;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::prelude::*;

use super::ContentTab;
use super::shared::flow_row::FILTER_MIN_WIDTH;
use super::shared::segments::{self, segment_id};
use super::{tab_for, tab_pane};
use crate::settings::{EditorTypography, EditorTypographySet};
use crate::test_support::{RealProject, first_of_type};

/// The centre column of a maximised window with its docks open.
const WIDE: (f32, f32) = (1000.0, 700.0);
/// The centre column measured under NVDA: a window about 1000 wide on Windows, with the
/// binder on one side and the Inspector on the other. The same project, the same controls.
const NARROW: (f32, f32) = (330.0, 800.0);

fn typography() -> EditorTypographySet {
    let bundle = |family: &str| EditorTypography {
        font_family: Signal::new(family.to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: crate::settings::TypographySizeRange::default(),
    };
    EditorTypographySet {
        scene: bundle("Literata"),
        synopsis: bundle("Literata"),
        notes: bundle("Inter"),
        corkboard: bundle("Literata"),
        distraction_free: bundle("Literata"),
    }
}

/// `(id, role, sub_role)` of the project's rows, in binder order.
fn rows(project: &RealProject) -> Vec<(u64, BinderItemRole, BinderItemSubRole)> {
    crate::models::binder_stream::ordered_binder_items(&project.app_ctx, project.work_id)
        .into_iter()
        .filter_map(|row| {
            frontend::commands::binder_item_commands::get_binder_item(&project.app_ctx, &row.id)
                .ok()
                .flatten()
                .map(|item| (item.id, item.role, item.sub_role))
        })
        .collect()
}

/// The ids of a row's views, for a row that has more than one. Asking a tab for a segment it
/// does not have falls back to one it does, so the extras cost a repeat and hide nothing.
fn views_of(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> Vec<&'static str> {
    use BinderItemSubRole as Sub;
    match (role, sub_role) {
        (BinderItemRole::Folder, Sub::Book) => vec![
            segments::SEG_OWN,
            segments::SEG_MANUSCRIPT,
            segments::SEG_SYNOPSIS,
            segments::SEG_PACE,
            segments::SEG_ANALYSIS,
            segments::SEG_CORKBOARD,
            segments::SEG_OVERVIEW,
        ],
        (BinderItemRole::Folder, Sub::ChapterScene | Sub::Part) => vec![
            segments::SEG_OWN,
            segments::SEG_MANUSCRIPT,
            segments::SEG_SYNOPSIS,
            segments::SEG_CORKBOARD,
            segments::SEG_OVERVIEW,
        ],
        (BinderItemRole::Folder, _) => vec![
            segments::SEG_NOTES,
            segments::SEG_STORY_BIBLE,
            segments::SEG_OVERVIEW,
        ],
        _ => vec![segments::SEG_OWN],
    }
}

fn open_tab(
    project: &RealProject,
    item: u64,
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
) -> ContentTab {
    tab_for(
        &project.app_ctx,
        item,
        role,
        sub_role,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        typography(),
        crate::settings::EditorViewMemory::detached(false),
        &project.ids,
    )
}

/// `tab` on `view`, laid out at `size`, with its virtualized rows realized.
fn mount(
    project: &RealProject,
    tab: &ContentTab,
    view: &str,
    (width, height): (f32, f32),
) -> (WidgetTree, WidgetId) {
    tab.segment.set(Some(segment_id(view)));
    let mut tree = crate::test_support::tree_with_events(&project.app_ctx);
    let root = tree.add_boxed(tab_pane(tab));
    let proposal = SizeProposal::exact(width, height);
    tree.layout(proposal);
    let _ = tree.render();
    tree.layout(proposal);
    (tree, root)
}

fn is_empty(tree: &WidgetTree, id: WidgetId) -> bool {
    let bounds = tree.bounds(id);
    bounds.width < 1.0 || bounds.height < 1.0
}

/// Every Tab stop under `root` with no area, even once it holds focus.
///
/// A stop is allowed to draw itself as focus arrives, which is the point of revealing a
/// control for the keyboard: what a writer meets there is what it looks like then, so that
/// is what is measured.
fn invisible_stops(
    tree: &mut WidgetTree,
    root: WidgetId,
    (width, height): (f32, f32),
) -> Vec<String> {
    let proposal = SizeProposal::exact(width, height);
    let stops = tree.focusables_in_tab_order(root);
    let mut invisible = Vec::new();
    for id in stops {
        // Out of the Tab order, or gone since a previous stop took and left focus.
        if !tree.tab_stop(id) || !tree.is_active(id) || !is_empty(tree, id) {
            continue;
        }
        tree.focus(id);
        tree.layout(proposal);
        if is_empty(tree, id) {
            invisible.push(format!(
                "{} at {:?}",
                tree.widget_type_name(id).unwrap_or("?"),
                tree.bounds(id)
            ));
        }
    }
    invisible
}

/// The sweep. It walks both widths because the filters were only empty in the narrow one and
/// the tag button in both.
#[test]
fn no_tab_stop_on_any_view_of_any_tab_is_invisible() {
    let project = RealProject::empty_novel();
    let mut walked = 0;
    let mut offenders = Vec::new();
    for (item, role, sub_role) in rows(&project) {
        for view in views_of(&role, &sub_role) {
            for size in [WIDE, NARROW] {
                let tab = open_tab(&project, item, &role, &sub_role);
                let (mut tree, root) = mount(&project, &tab, view, size);
                walked += tree.focusables_in_tab_order(root).len();
                for stop in invisible_stops(&mut tree, root, size) {
                    offenders.push(format!(
                        "{role:?}/{sub_role:?} on {view} at {}: {stop}",
                        size.0
                    ));
                }
            }
        }
    }
    assert!(walked > 100, "the walk saw only {walked} stops");
    assert!(
        offenders.is_empty(),
        "Tab stops a writer cannot see:\n{}",
        offenders.join("\n")
    );
}

fn folder(project: &RealProject, wanted: BinderItemSubRole) -> u64 {
    rows(project)
        .into_iter()
        .find(|(_, role, sub_role)| *role == BinderItemRole::Folder && *sub_role == wanted)
        .map(|(id, ..)| id)
        .expect("the novel has this folder")
}

/// The filter is the control the writer asked for, so having some area is not the bar: the
/// text in it has to have room, and the row must not outgrow its column to give it that.
#[test]
fn a_narrow_column_keeps_the_filters_usable() {
    let project = RealProject::empty_novel();
    let chapter = folder(&project, BinderItemSubRole::ChapterScene);
    for view in [segments::SEG_CORKBOARD, segments::SEG_OVERVIEW] {
        let tab = open_tab(
            &project,
            chapter,
            &BinderItemRole::Folder,
            &BinderItemSubRole::ChapterScene,
        );
        let (tree, root) = mount(&project, &tab, view, NARROW);
        let field = first_of_type(&tree, root, "SearchField").expect("the view has a filter");
        let bounds = tree.bounds(field);
        assert!(
            bounds.width >= FILTER_MIN_WIDTH,
            "{view}: the filter is down to {bounds:?}"
        );
        assert!(
            bounds.x + bounds.width <= NARROW.0,
            "{view}: the filter runs past the column: {bounds:?}"
        );
        let text = first_of_type(&tree, field, "TextInputField").expect("its text area");
        assert!(
            tree.bounds(text).width >= 80.0,
            "{view}: too little room to type: {:?}",
            tree.bounds(text)
        );
    }
}

/// With the header back inside its column, nothing past it pushes the cards out either: they
/// were being cut at the right edge of the same narrow window.
/// The same for a card: its dots come from the project's palette. A tagged scene's card drew
/// none after a launch from the Launcher.
#[test]
fn a_tagged_card_draws_its_dots_from_its_own_project_palette() {
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::direct_access::BinderItemRelationshipDto;

    let project = RealProject::empty_novel();
    let chapter = folder(&project, BinderItemSubRole::ChapterScene);
    let scene =
        crate::models::binder_stream::ordered_binder_items(&project.app_ctx, project.work_id)
            .into_iter()
            .find(|row| {
                frontend::commands::binder_item_commands::get_binder_item(&project.app_ctx, &row.id)
                    .ok()
                    .flatten()
                    .is_some_and(|i| i.sub_role == BinderItemSubRole::Scene)
            })
            .expect("the novel has a scene")
            .id;
    let tag = crate::tags::TagsViewModel::detached(project.app_ctx.clone(), project.ids.clone())
        .create("character", "#2980b9", "", false)
        .expect("the novel takes a tag");
    frontend::commands::binder_item_commands::set_binder_item_relationship(
        &project.app_ctx,
        None,
        &BinderItemRelationshipDto {
            id: scene,
            field: BinderItemRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .expect("the scene takes the tag");

    let tab = open_tab(
        &project,
        chapter,
        &BinderItemRole::Folder,
        &BinderItemSubRole::ChapterScene,
    );
    tab.tags().refresh();
    let (tree, root) = mount(&project, &tab, segments::SEG_CORKBOARD, WIDE);
    let grid = first_of_type(&tree, root, "CorkboardGrid").expect("the board has a grid");
    let mut stack = vec![grid];
    let mut drawn = false;
    while let Some(id) = stack.pop() {
        drawn |= tree
            .widget_type_name(id)
            .is_some_and(|name| name.ends_with("FixedDot"))
            && !is_empty(&tree, id);
        stack.extend(tree.children(id));
    }
    assert!(drawn, "the tagged scene's card draws no tag dot");
}

#[test]
fn a_narrow_column_keeps_the_corkboard_inside_it() {
    let project = RealProject::empty_novel();
    let chapter = folder(&project, BinderItemSubRole::ChapterScene);
    let tab = open_tab(
        &project,
        chapter,
        &BinderItemRole::Folder,
        &BinderItemSubRole::ChapterScene,
    );
    let (tree, root) = mount(&project, &tab, segments::SEG_CORKBOARD, NARROW);
    let grid = first_of_type(&tree, root, "CorkboardGrid").expect("the board has a grid");
    let bounds = tree.bounds(grid);
    assert!(
        bounds.x + bounds.width <= NARROW.0,
        "the board runs past the column: {bounds:?}"
    );
}

/// The keyboard's way to the tags of the page it is on, end to end: Tab from the title lands
/// on the picker, which is drawn from that moment, and leaving takes it away again.
#[test]
fn tabbing_past_a_chapter_title_finds_a_visible_tag_control() {
    let project = RealProject::empty_novel();
    let chapter = folder(&project, BinderItemSubRole::ChapterScene);
    let tab = open_tab(
        &project,
        chapter,
        &BinderItemRole::Folder,
        &BinderItemSubRole::ChapterScene,
    );
    let (mut tree, root) = mount(&project, &tab, segments::SEG_OWN, WIDE);
    let proposal = SizeProposal::exact(WIDE.0, WIDE.1);
    let trigger = first_of_type(&tree, root, "OverlayTrigger").expect("the subtitle's tag row");
    assert!(
        is_empty(&tree, trigger),
        "an untagged page draws nothing for a writer not asking"
    );

    tree.focus(trigger);
    tree.layout(proposal);
    assert!(!is_empty(&tree, trigger), "focused, it has to be seen");

    tree.press_key(Key::Tab, Modifiers::NONE);
    tree.layout(proposal);
    assert_ne!(tree.focused(), Some(trigger), "Tab moved on");
    assert!(is_empty(&tree, trigger), "and it folds away again");

    tree.press_key(Key::Tab, Modifiers::SHIFT);
    tree.layout(proposal);
    assert_eq!(tree.focused(), Some(trigger), "Shift+Tab comes back to it");
    assert!(!is_empty(&tree, trigger));
}

/// A manuscript view is one row per item. An untagged row keeps no stop, so a keyboard writer
/// is not walked through one invisible button per scene, and the Inspector is there for them.
#[test]
fn an_untagged_row_of_a_stream_is_no_stop_at_all() {
    let project = RealProject::empty_novel();
    let book = folder(&project, BinderItemSubRole::Book);
    let tab = open_tab(
        &project,
        book,
        &BinderItemRole::Folder,
        &BinderItemSubRole::Book,
    );
    let (tree, root) = mount(&project, &tab, segments::SEG_MANUSCRIPT, WIDE);
    let triggers: Vec<WidgetId> = tree
        .focusables_in_tab_order(root)
        .into_iter()
        .filter(|id| {
            tree.widget_type_name(*id)
                .is_some_and(|name| name.ends_with("OverlayTrigger"))
        })
        .collect();
    assert_eq!(triggers, Vec::new());
}

/// With no palette in `app_state`, which is what a launch starting at the Launcher amounts to:
/// that one is a throwaway on a Work never seeded. A tagged chapter resolved no tag there and
/// drew no dots, and that is the other way to a tag button with no area. The tree here has no
/// `app_state` palette at all, so the dots can only come from the tab's own.
#[test]
fn a_tagged_row_draws_its_dots_from_its_own_project_palette() {
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::direct_access::BinderItemRelationshipDto;

    let project = RealProject::empty_novel();
    let chapter = folder(&project, BinderItemSubRole::ChapterScene);
    let book = folder(&project, BinderItemSubRole::Book);
    let tag = crate::tags::TagsViewModel::detached(project.app_ctx.clone(), project.ids.clone())
        .create("character", "#2980b9", "", false)
        .expect("the novel takes a tag");
    frontend::commands::binder_item_commands::set_binder_item_relationship(
        &project.app_ctx,
        None,
        &BinderItemRelationshipDto {
            id: chapter,
            field: BinderItemRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .expect("the chapter takes the tag");

    for (item, sub_role, view) in [
        (chapter, BinderItemSubRole::ChapterScene, segments::SEG_OWN),
        (book, BinderItemSubRole::Book, segments::SEG_MANUSCRIPT),
    ] {
        let tab = open_tab(&project, item, &BinderItemRole::Folder, &sub_role);
        tab.tags().refresh();
        // A standalone tab's document has nobody to seed it, unlike a stream row's probe.
        if item == chapter {
            tab.open_doc.tags.set(vec![tag]);
        }
        let (tree, root) = mount(&project, &tab, view, WIDE);
        let drawn = tree.focusables_in_tab_order(root).into_iter().any(|id| {
            tree.widget_type_name(id)
                .is_some_and(|name| name.ends_with("OverlayTrigger"))
                && !is_empty(&tree, id)
        });
        assert!(drawn, "{sub_role:?} on {view} draws no tag dots");
    }
}
