// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::AppContext;
use frontend::import_management::{
    ScrivenerArg, ScrivenerFinding, ScrivenerFindingFamily, ScrivenerPlace, ScrivenerReportGroup,
    ScrivenerTotals,
};
use teksilo::core::widget_tree::WidgetTree;
use uuid::Uuid;

use crate::import_scrivener::ScrivenerImportResult;
use crate::import_scrivener::report::ImportReport;
use crate::import_scrivener::report_vm::ItemFinder;

const ROW: Uuid = Uuid::from_u128(42);

fn finding(key: &str, group: ScrivenerReportGroup, item: i64, title: &str) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Prose,
        group,
        place: ScrivenerPlace::Text,
        item,
        binder_path: vec!["Draft".into(), title.into()],
        item_uid: Some(if item == 3 {
            ROW
        } else {
            Uuid::from_u128(item as u128)
        }),
        args: vec![ScrivenerArg::Count {
            name: "count".into(),
            value: 2,
        }],
    }
}

/// A report with a line in each group, the first two with an item each.
fn report() -> ImportReport {
    ImportReport::from_result(&ScrivenerImportResult {
        output_path: "/books/The Lighthouse.skrib".into(),
        research_folder: String::new(),
        totals: ScrivenerTotals::Counted {
            rows: 12,
            words: 3400,
            footnotes: 1,
            comments: 5,
            pictures: 1,
            templates: 0,
            tags: 2,
            rungs: 4,
            trashed: 0,
            research_files: 0,
            synopses: 4,
            document_notes: 2,
        },
        findings: vec![
            finding(
                "scrivener-prose-unreadable",
                ScrivenerReportGroup::CouldNotBeRead,
                3,
                "Arrival",
            ),
            finding(
                "scrivener-prose-heading-repeats-title",
                ScrivenerReportGroup::CheckThese,
                4,
                "Storm",
            ),
            finding(
                "scrivener-prose-annotations-became-comments",
                ScrivenerReportGroup::BroughtAcrossDifferently,
                5,
                "Harbour",
            ),
            finding(
                "scrivener-prose-media-file-not-carried",
                ScrivenerReportGroup::LeftBehind,
                6,
                "Map.pdf",
            ),
        ],
        record: None,
    })
}

/// The dialog on a tree of its own, laid out in a `w` × `h` slot.
fn mount(find_item: Option<ItemFinder>, w: f32, h: f32) -> (WidgetTree, WidgetId) {
    let (tree, id, _) = mount_report(report(), find_item, w, h);
    (tree, id)
}

/// `report`'s dialog on a tree of its own, laid out in a `w` × `h` slot, with its
/// view-model.
fn mount_report(
    report: ImportReport,
    find_item: Option<ItemFinder>,
    w: f32,
    h: f32,
) -> (WidgetTree, WidgetId, ImportReportViewModel) {
    let app_ctx = std::rc::Rc::new(AppContext::new());
    let mut tree =
        crate::test_support::tree_with_events(&app_ctx).with_text_backend(std::rc::Rc::new(
            std::cell::RefCell::new(teksilo::canvas::MockTextBackend::new()),
        ));
    let vm = ImportReportViewModel::new(report, find_item);
    let id = tree.add_boxed(Box::new(ImportReportPanel::new(vm.clone())));
    tree.layout(SizeProposal::exact(w, h));
    (tree, id, vm)
}

/// The first node under `root` whose accessible name starts with `prefix`.
fn named_starting(tree: &WidgetTree, root: WidgetId, prefix: &str) -> Option<WidgetId> {
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if tree
            .accessibility_node(id)
            .name()
            .is_some_and(|n| n.starts_with(prefix))
        {
            return Some(id);
        }
        stack.extend(tree.children(id).into_iter().rev());
    }
    None
}

/// Every node under `root`.
fn every_node(tree: &WidgetTree, root: WidgetId) -> Vec<WidgetId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        out.push(id);
        stack.extend(tree.children(id));
    }
    out
}

/// The button under `root` named `name`: a disclosure's header, not the region it
/// controls, which a group names the same.
fn button_named(tree: &WidgetTree, root: WidgetId, name: &str) -> Option<WidgetId> {
    every_node(tree, root).into_iter().find(|n| {
        let node = tree.accessibility_node(*n);
        node.role() == Role::Button && node.name() == Some(name)
    })
}

/// A report whose one line names items with long binder paths, the first a row of the
/// open project.
fn long_paths() -> ImportReport {
    let long = |item: i64, scene: &str| ScrivenerFinding::Reported {
        key: "scrivener-prose-unreadable".into(),
        family: ScrivenerFindingFamily::Prose,
        group: ScrivenerReportGroup::CouldNotBeRead,
        place: ScrivenerPlace::Text,
        item,
        binder_path: vec![
            "Manuscript".into(),
            "Part One: The Long Winter Before the Lighthouse Keeper Came Home Again".into(),
            "Chapter 1: In Which the Harbour Master Refuses to Speak of the Storm".into(),
            scene.into(),
        ],
        item_uid: Some(if item == 3 {
            ROW
        } else {
            Uuid::from_u128(item as u128)
        }),
        args: Vec::new(),
    };
    ImportReport::from_result(&ScrivenerImportResult {
        output_path: "/books/The Lighthouse.skrib".into(),
        research_folder: String::new(),
        totals: ScrivenerTotals::Empty,
        findings: vec![
            long(
                3,
                "Arrival of the Inspector on the Last Ferry of an Endless Grey Afternoon",
            ),
            long(
                4,
                "Departure of the Inspector at Dawn, Without a Word to Anyone",
            ),
        ],
        record: None,
    })
}

/// The dialog builds at its size, says what it is, and shows its four groups, their
/// titles counting their lines, with the footer's three buttons.
#[test]
fn the_report_builds_and_names_its_groups() {
    crate::test_support::with_real_messages(|| {
        let (tree, id) = mount(None, REPORT_W, REPORT_H);
        let card = tree.bounds(id);
        assert!((card.width - REPORT_W).abs() < 0.5, "{card:?}");
        assert!((card.height - REPORT_H).abs() < 0.5, "{card:?}");
        assert!(
            tree.find_by_role(Role::Dialog).is_some(),
            "a named dialog for a screen reader"
        );
        for label in [
            "Could not be read (1)",
            "Check these (1)",
            "Brought across differently (1)",
            "Left behind (1)",
            "Copy report",
            "Open now",
            "Close",
        ] {
            assert!(tree.find_by_label(label).is_some(), "{label}");
        }
    });
}

/// In a window smaller than the dialog, it shrinks to fit rather than losing its footer.
#[test]
fn the_report_fits_a_small_window() {
    crate::test_support::with_real_messages(|| {
        let (tree, _id) = mount(None, 500.0, 400.0);
        let close = tree
            .find_by_label("Close")
            .expect("the footer's Close is there");
        let bounds = tree.bounds(close);
        assert!(
            bounds.y + bounds.height <= 400.0 && bounds.x + bounds.width <= 500.0,
            "Close at {bounds:?}, outside a 500 × 400 window"
        );
    });
}

/// An item links to its row only where the window shows the new project.
#[test]
fn items_link_only_where_the_project_is_shown() {
    crate::test_support::with_real_messages(|| {
        let (tree, id) = mount(None, REPORT_W, REPORT_H);
        assert!(crate::test_support::first_of_type(&tree, id, "::Link").is_none());
        assert!(tree.find_by_label("Open now").is_some());
        let hint = tr!(import_scrivener_report_links_hint()).resolve_now();
        assert!(tree_texts(&tree, id).contains(&hint));

        let finder: ItemFinder =
            std::rc::Rc::new(|uid| (uid == ROW).then(|| (7, "Arrival".to_string())));
        let (mut tree, id) = mount(Some(finder), REPORT_W, REPORT_H);
        assert!(
            tree.find_by_label("Open now").is_none(),
            "this window already shows the new project"
        );
        let link = crate::test_support::first_of_type(&tree, id, "::Link")
            .expect("the item with a row of the open project is a link");
        assert_eq!(
            shown_height(&tree, link, id),
            0.0,
            "the list is closed: its link is not on screen"
        );
        let header = named_starting(&tree, id, "1 item").expect("the first line's list of items");
        open(&mut tree, header);
        let height = tree.bounds(link).height;
        assert!(
            height > 0.0 && shown_height(&tree, link, id) >= height - 0.5,
            "once open, the whole link is on screen"
        );
        assert!(!tree_texts(&tree, id).contains(&hint));
    });
}

/// Longer than any accordion's opening, at any motion setting.
const PAST_THE_OPENING: std::time::Duration = std::time::Duration::from_secs(1);

/// Press the disclosure `header` and let its body finish opening, as a window would.
fn open(tree: &mut WidgetTree, header: WidgetId) {
    crate::test_support::click(tree, header);
    tree.layout(SizeProposal::exact(REPORT_W, REPORT_H));
    tree.advance_time(PAST_THE_OPENING);
    tree.layout(SizeProposal::exact(REPORT_W, REPORT_H));
}

/// How much of `node`'s height is on screen: what the disclosures it sits in, up to
/// `root`, leave of it once they clip it to the height they have opened to.
fn shown_height(tree: &WidgetTree, node: WidgetId, root: WidgetId) -> f32 {
    let b = tree.bounds(node);
    let (mut top, mut bottom) = (b.y, b.y + b.height);
    let mut at = tree.parent(node);
    while let Some(id) = at {
        if tree
            .widget_type_name(id)
            .is_some_and(|n| n.ends_with("::Collapse"))
        {
            let clip = tree.bounds(id);
            top = top.max(clip.y);
            bottom = bottom.min(clip.y + clip.height);
        }
        if id == root {
            break;
        }
        at = tree.parent(id);
    }
    (bottom - top).max(0.0)
}

/// Whether a screen reader reaches `node`: it is in the accessibility tree, and neither it
/// nor any node above it is hidden.
fn reached_by_at(tree: &WidgetTree, node: WidgetId) -> bool {
    use std::collections::HashMap;
    let snapshot = tree.accessibility_tree_snapshot();
    let mut parent = HashMap::new();
    for (id, n) in &snapshot.nodes {
        for child in n.children() {
            parent.insert(*child, *id);
        }
    }
    let mut at = Some(teksilo::core::accessibility::widget_id_to_node_id(node));
    if !snapshot.nodes.iter().any(|(id, _)| Some(*id) == at) {
        return false;
    }
    while let Some(id) = at {
        if snapshot
            .nodes
            .iter()
            .any(|(n, node)| *n == id && node.is_hidden())
        {
            return false;
        }
        at = parent.get(&id).copied();
    }
    true
}

/// Every text under `root`, as the accessibility tree reads it.
fn tree_texts(tree: &WidgetTree, root: WidgetId) -> String {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let node = tree.accessibility_node(id);
        if let Some(name) = node.name() {
            out.push(name.to_string());
        }
        stack.extend(tree.children(id));
    }
    out.join(" | ")
}

/// Opening a line's list of items shows them, and moves what follows down.
#[test]
fn opening_a_lines_items_shows_them() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id) = mount(None, REPORT_W, REPORT_H);
        let header = named_starting(&tree, id, "1 item").expect("the first line's list of items");
        let next = button_named(&tree, id, "Check these (1)").expect("the next group");
        let before = tree.bounds(next).y;
        open(&mut tree, header);
        let after = tree.bounds(next).y;
        assert!(
            after > before + 4.0,
            "the group below moved from {before} to {after}"
        );
    });
}

/// In a window without focus, a group and a line's list of items still open to their full
/// height. The scheduler of teksilo before 0.15.1 held every animation of such a window,
/// one-shot tweens included, so an accordion opened there by an automation client, an
/// assistive technology or a writer's click before the window took focus said it was
/// expanded and showed nothing.
#[test]
fn an_accordion_opens_in_a_window_without_focus() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id) = mount(None, REPORT_W, REPORT_H);
        tree.set_window_active(false);
        let group = button_named(&tree, id, "Brought across differently (1)")
            .expect("a group the report opens closed");
        let next = button_named(&tree, id, "Left behind (1)").expect("the group under it");
        let before = tree.bounds(next).y;
        open(&mut tree, group);
        let after = tree.bounds(next).y;
        assert!(
            after > before + 4.0,
            "the open group pushed the next one down: from {before} to {after}"
        );

        let header = named_starting(&tree, id, "1 item").expect("the first line's list of items");
        let next = button_named(&tree, id, "Check these (1)").expect("the group under the line");
        let before = tree.bounds(next).y;
        open(&mut tree, header);
        let item = named_starting(&tree, id, "Draft ▸ Arrival").expect("the item it lists");
        let shown = tree.bounds(item);
        assert!(shown.height > 0.0, "the item has a height: {shown:?}");
        assert!(
            tree.bounds(next).y >= before + shown.height - 0.5,
            "the open list pushed what follows down by at least the item it shows"
        );
    });
}

/// A long binder path shown as a link stays inside the dialog: the report keeps its width,
/// its lines wrap, and nothing scrolls sideways.
#[test]
fn a_long_linked_path_stays_inside_the_dialog() {
    crate::test_support::with_real_messages(|| {
        let finder: ItemFinder =
            std::rc::Rc::new(|uid| (uid == ROW).then(|| (7, "Arrival".to_string())));
        let (mut tree, id, _) = mount_report(long_paths(), Some(finder), REPORT_W, REPORT_H);
        let header = named_starting(&tree, id, "2 items").expect("the line's list of items");
        open(&mut tree, header);
        assert!(crate::test_support::first_of_type(&tree, id, "::Link").is_some());
        let card = tree.bounds(id);
        let right = card.x + REPORT_W;
        // Nodes with no area (a scroll bar at rest, a ring with nothing to draw) aside.
        for node in every_node(&tree, id) {
            let b = tree.bounds(node);
            if b.width <= 0.0 || b.height <= 0.0 {
                continue;
            }
            assert!(
                b.x >= card.x - 0.5 && b.x + b.width <= right + 0.5,
                "{node:?} at {b:?} runs past the dialog's {card:?}"
            );
        }
    });
}

/// Items whose paths wrap stay visibly apart: more room between two items than between
/// the lines of one.
#[test]
fn wrapped_items_stay_apart() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, _) = mount_report(long_paths(), None, REPORT_W, REPORT_H);
        let header = named_starting(&tree, id, "2 items").expect("the line's list of items");
        open(&mut tree, header);
        let first = named_starting(&tree, id, "Manuscript ▸ Part One").expect("first item");
        let second = tree
            .find_by_label(
                "Manuscript ▸ Part One: The Long Winter Before the Lighthouse Keeper Came Home \
                 Again ▸ Chapter 1: In Which the Harbour Master Refuses to Speak of the Storm \
                 ▸ Departure of the Inspector at Dawn, Without a Word to Anyone",
            )
            .expect("second item");
        let (a, b) = (tree.bounds(first), tree.bounds(second));
        assert!(a.height > 20.0, "the first path wraps: {a:?}");
        assert!(b.y - (a.y + a.height) >= 6.0, "{a:?} then {b:?}");
    });
}

/// In French, once the report is copied, the footer still fits a small window: Close
/// stays on the card.
#[test]
fn the_french_footer_fits_once_copied() {
    crate::test_support::with_shipped_messages("fr-FR", || {
        for (w, h) in [(560.0, 480.0), (480.0, 400.0)] {
            let (mut tree, id, vm) = mount_report(report(), None, w, h);
            vm.copy_status()
                .set(tr!(import_scrivener_report_copied()).resolve_now());
            tree.layout(SizeProposal::exact(w, h));
            let card = tree
                .children(id)
                .first()
                .map(|c| tree.bounds(*c))
                .expect("the card");
            // The footer's Close, not the header's cross, whose tooltip says the same.
            let close = every_node(&tree, id)
                .into_iter()
                .filter(|n| tree.accessibility_node(*n).name() == Some("Fermer"))
                .max_by(|a, b| tree.bounds(*a).y.total_cmp(&tree.bounds(*b).y))
                .expect("the footer's Close");
            let b = tree.bounds(close);
            assert!(
                b.x >= card.x && b.x + b.width <= card.x + card.width + 0.5,
                "Fermer at {b:?}, off the {card:?} card in a {w} × {h} window"
            );
        }
    });
}

/// A group and a line's list of items are each a button that controls a named region
/// while it is open, and a list's button says which line it belongs to.
#[test]
fn each_disclosure_controls_a_named_region() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id) = mount(None, REPORT_W, REPORT_H);
        // The button, not the region it controls, which carries the same name.
        let button = |tree: &WidgetTree, name: &str| {
            every_node(tree, id).into_iter().find(|n| {
                let node = tree.accessibility_node(*n);
                node.role() == Role::Button && node.name() == Some(name)
            })
        };
        let controlled_region = |tree: &WidgetTree, of: WidgetId, name: &str| {
            let snapshot = tree.accessibility_tree_snapshot();
            let wanted = teksilo::core::accessibility::widget_id_to_node_id(of);
            let controls = snapshot
                .nodes
                .iter()
                .find(|(n, _)| *n == wanted)
                .map(|(_, node)| node.controls().to_vec())
                .unwrap_or_default();
            snapshot.nodes.iter().any(|(n, node)| {
                node.role() == Role::Region && node.label() == Some(name) && controls.contains(n)
            })
        };
        let group = button(&tree, "Could not be read (1)").expect("the group's button");
        assert!(
            controlled_region(&tree, group, "Could not be read (1)"),
            "the group's button controls its region"
        );

        let line = tr!(import_scrivener_prose_unreadable_line(count = 1)).resolve_now();
        let named = format!("1 item: {line}");
        let toggle = button(&tree, &named).expect("the line's list of items, named with its line");
        let region = every_node(&tree, id)
            .into_iter()
            .find(|n| {
                let node = tree.accessibility_node(*n);
                node.role() == Role::Region && node.name() == Some(named.as_str())
            })
            .expect("the list's region");
        assert!(
            !reached_by_at(&tree, region),
            "a closed list's region is hidden from a screen reader"
        );
        open(&mut tree, toggle);
        assert!(
            controlled_region(&tree, toggle, &named),
            "the open list's button controls its region"
        );
        assert!(
            reached_by_at(&tree, region),
            "an open list's region is read"
        );
    });
}

/// No two regions share a name: each disclosure's region is named after what it holds,
/// a list of items with its line, so a screen reader's list of regions tells them apart.
#[test]
fn no_two_regions_share_a_name() {
    crate::test_support::with_real_messages(|| {
        let (tree, _id) = mount(None, REPORT_W, REPORT_H);
        let snapshot = tree.accessibility_tree_snapshot();
        let mut names: Vec<&str> = snapshot
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == Role::Region)
            .filter_map(|(_, node)| node.label())
            .collect();
        assert!(!names.is_empty(), "the disclosures have regions");
        names.sort_unstable();
        let total = names.len();
        names.dedup();
        assert_eq!(names.len(), total, "a region name repeats: {names:?}");
    });
}

/// What a closed disclosure holds is out of reach: not a Tab stop, not read by a screen
/// reader, until it opens. A list left open in a group then closed is out of reach too.
#[test]
fn closed_disclosures_are_out_of_reach() {
    crate::test_support::with_real_messages(|| {
        let finder: ItemFinder =
            std::rc::Rc::new(|uid| (uid == ROW).then(|| (7, "Arrival".to_string())));
        let (mut tree, id) = mount(Some(finder), REPORT_W, REPORT_H);
        let reachable = |tree: &WidgetTree, node: WidgetId| {
            (
                tree.tab_stops_within(id).contains(&node),
                reached_by_at(tree, node),
            )
        };
        let link = crate::test_support::first_of_type(&tree, id, "::Link").expect("the link");
        let toggle = named_starting(&tree, id, "1 item").expect("the open group's first list");
        assert_eq!(
            reachable(&tree, toggle),
            (true, true),
            "an open group's list button is in reach"
        );
        assert_eq!(
            reachable(&tree, link),
            (false, false),
            "the closed list's link is out of reach"
        );

        // In a group the report opens closed, the line's list button is out of reach.
        let changed = tr!(import_scrivener_prose_annotations_became_comments_line(
            count = 2
        ))
        .resolve_now();
        let hidden_toggle = every_node(&tree, id)
            .into_iter()
            .find(|n| {
                let node = tree.accessibility_node(*n);
                node.role() == Role::Button
                    && node.name() == Some(format!("1 item: {changed}").as_str())
            })
            .expect("the closed group's list button");
        assert_eq!(
            reachable(&tree, hidden_toggle),
            (false, false),
            "a closed group's list button is out of reach"
        );

        open(&mut tree, toggle);
        assert_eq!(reachable(&tree, link), (true, true), "the open list's link");

        // Closing the group around the open list takes the link out of reach again.
        let group = button_named(&tree, id, "Could not be read (1)").expect("the group's button");
        open(&mut tree, group);
        assert_eq!(
            reachable(&tree, link),
            (false, false),
            "a list left open in a closed group"
        );
    });
}

/// Focus opens on **Copy report**, never on the header's close cross, which a first
/// Enter would press.
#[test]
fn focus_opens_on_copy_report() {
    crate::test_support::with_real_messages(|| {
        let (tree, id) = mount(None, REPORT_W, REPORT_H);
        let hinted = tree
            .widget_initial_focus_hint(id)
            .expect("the dialog says where focus opens");
        assert_eq!(Some(hinted), tree.find_by_label("Copy report"));
    });
}
