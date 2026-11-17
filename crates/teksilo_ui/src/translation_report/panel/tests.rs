// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::test_support::with_shipped_messages;
use teksilo::core::widget_tree::WidgetTree;

/// The window's smallest size.
fn size() -> SizeProposal {
    SizeProposal::exact(760.0, 540.0)
}

/// A tree that measures text as a window does: wrapped text wraps. With no text
/// backend every text is measured on one line, which makes a column look wider than
/// any real window would draw it.
fn tree() -> WidgetTree {
    WidgetTree::new().with_text_backend(std::rc::Rc::new(std::cell::RefCell::new(
        teksilo::canvas::MockTextBackend::new(),
    )))
}

fn mounted(vm: &TranslationReportViewModel) -> WidgetTree {
    mounted_at(vm, size())
}

fn mounted_at(vm: &TranslationReportViewModel, at: SizeProposal) -> WidgetTree {
    let mut tree = tree();
    tree.add(TranslationReportPanel::new(vm.clone()));
    tree.layout(at);
    tree
}

/// Every control a writer or a screen reader reaches is there by its name, at the
/// window's smallest size.
#[test]
fn every_control_is_there_by_name() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let tree = mounted(&vm);
        for label in [
            tr!(translation_report_language()),
            tr!(translation_report_find()),
            tr!(translation_report_not_found()),
            tr!(translation_report_kind()),
            tr!(translation_report_explain()),
            tr!(translation_report_suggestion()),
            tr!(translation_report_public()),
            tr!(translation_report_copy_text()),
            tr!(translation_report_close()),
            tr!(translation_report_continue()),
        ] {
            let label = label.resolve_now();
            assert!(
                tree.find_by_label(&label).is_some(),
                "no control named {label:?}"
            );
        }
    });
}

/// Typing in the search field lists the matches and leaves the field in place: a
/// field rebuilt under the writer's fingers keeps only the last character.
#[test]
fn typing_a_query_lists_the_matches_and_keeps_the_field() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        let field = tree
            .find_by_label(&tr!(translation_report_find()).resolve_now())
            .expect("the search field");
        tree.focus(field);
        tree.type_text(field, "Report a Problem");
        tree.layout(size());
        assert_eq!(vm.query().get(), "Report a Problem");
        assert!(!vm.rows().is_empty());
        assert_eq!(
            vm.results_page().get(),
            super::super::translation_report_vm::RESULTS_LIST
        );
        assert!(
            tree.find_by_label(&tr!(translation_report_results()).resolve_now())
                .is_some(),
            "the list is shown, by its name"
        );
    });
}

/// Picking a text shows its card, whose heading is the text itself.
#[test]
fn picking_a_text_shows_its_card() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        vm.query().set("Report a Problem…".to_string());
        tree.layout(size());
        vm.pick_row(0);
        tree.layout(size());
        let card = vm.card().expect("a card");
        assert!(
            tree.find_by_label(&card.text).is_some(),
            "the card's heading"
        );
    });
}

/// A text not found asks where it was seen, by name.
#[test]
fn a_text_not_found_asks_where_it_was_seen() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        vm.cannot_find();
        tree.layout(size());
        assert!(
            tree.find_by_label(&tr!(translation_report_seen_at()).resolve_now())
                .is_some()
        );
    });
}

/// After Continue, Copy link and Report another appear beside the next steps.
#[test]
fn after_continue_the_link_can_be_copied() {
    with_shipped_messages("en-US", || {
        let _serial = crate::identity::lock_for_test();
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        let copy_link = tr!(translation_report_copy_link()).resolve_now();
        assert!(
            tree.find_by_label(&copy_link).is_none(),
            "nothing to copy yet"
        );
        vm.query().set("Report a Problem…".to_string());
        vm.pick_row(0);
        vm.choose_kind(ProblemKind::Unnatural);
        let plan = vm.prepare_continue().expect("complete");
        vm.continue_with(&plan, None, |_| Ok(()));
        tree.layout(size());
        assert!(tree.find_by_label(&copy_link).is_some());
        assert!(
            tree.find_by_label(&tr!(translation_report_another()).resolve_now())
                .is_some()
        );
    });
}

/// Every widget in the tree whose accessible name is `label`, in tree order.
fn all_named(tree: &WidgetTree, label: &str) -> Vec<WidgetId> {
    let mut found = Vec::new();
    let mut stack: Vec<WidgetId> = tree.roots().into_iter().rev().collect();
    while let Some(id) = stack.pop() {
        if tree.accessibility_node(id).name() == Some(label) {
            found.push(id);
        }
        stack.extend(tree.children(id).into_iter().rev());
    }
    found
}

/// The control named `label` that is not its visible label: the one whose role is
/// not a plain label.
fn control_named(tree: &WidgetTree, label: &str) -> WidgetId {
    all_named(tree, label)
        .into_iter()
        .find(|&id| tree.accessibility_node(id).role() != Role::Label)
        .unwrap_or_else(|| panic!("no control named {label:?}"))
}

/// Each field of the report has a visible label above it, not only an accessible
/// name: a sighted writer cannot hear a name. The text label is a separate widget
/// from the control, and sits above it.
fn assert_labelled_above(tree: &WidgetTree, label: &str) {
    let named = all_named(tree, label);
    let control = control_named(tree, label);
    let visible = named
        .iter()
        .copied()
        .find(|&id| tree.accessibility_node(id).role() == Role::Label)
        .unwrap_or_else(|| panic!("{label:?} has no visible label"));
    let (label_box, control_box) = (tree.bounds(visible), tree.bounds(control));
    assert!(
        label_box.y + label_box.height <= control_box.y + 0.5,
        "{label:?}: the label {label_box:?} is not above its control {control_box:?}"
    );
}

#[test]
fn the_kind_and_the_two_texts_are_labelled_above_their_fields() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let tree = mounted(&vm);
        for label in [
            tr!(translation_report_kind()),
            tr!(translation_report_explain()),
            tr!(translation_report_suggestion()),
        ] {
            assert_labelled_above(&tree, &label.resolve_now());
        }
    });
}

/// A text used in several places asks which one, under a visible label.
#[test]
fn the_place_picker_is_labelled_above_it() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        vm.query().set(String::from("annuler"));
        tree.layout(size());
        let index = (0..vm.rows().len())
            .find(|&i| vm.rows().with_item(i, |r| r.title == "Annuler") == Some(true))
            .expect("a row reading Annuler");
        vm.pick_row(index);
        tree.layout(size());
        assert!(vm.card().is_some_and(|c| c.places.len() > 1));
        assert_labelled_above(&tree, &tr!(translation_report_place()).resolve_now());
    });
}

/// The two editors fill the report column and grow with the window: an editor
/// sizes to its content, so one left unwrapped is a sliver as wide as its text.
#[test]
fn the_editors_fill_the_report_column() {
    with_shipped_messages("en-US", || {
        let widths = |width: f32| {
            let vm = TranslationReportViewModel::new();
            let tree = mounted_at(&vm, SizeProposal::exact(width, 540.0));
            [
                tr!(translation_report_explain()),
                tr!(translation_report_suggestion()),
            ]
            .map(|label| {
                tree.bounds(control_named(&tree, &label.resolve_now()))
                    .width
            })
        };
        let (narrow, wide) = (widths(760.0), widths(980.0));
        for (small, large) in narrow.into_iter().zip(wide) {
            // The report column is what is left beside the search column, less its
            // padding and room for a scroll bar.
            assert!(
                small >= 760.0 - SEARCH_WIDTH - 2.0 * 16.0 - 24.0,
                "an editor is {small} wide in a 760 window"
            );
            assert!(
                (large - small - 220.0).abs() < 1.0,
                "an editor grew from {small} to {large}, not with the window"
            );
        }
    });
}

/// "What will be public" is a disclosure: closed, it says so to a screen reader
/// and shows no body; activated, it says it is open and shows the exact title.
#[test]
fn what_will_be_public_opens_and_says_so() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        vm.query().set("Report a Problem…".to_string());
        vm.pick_row(0);
        vm.choose_kind(ProblemKind::Unnatural);
        tree.layout(size());
        let name = tr!(translation_report_public()).resolve_now();
        let toggle = control_named(&tree, &name);
        assert!(!tree.accessibility_node(toggle).is_expanded());
        let report = vm.report().expect("complete");
        let title = tr!(translation_report_public_title(title = report.title())).resolve_now();
        assert!(tree.find_by_label(&title).is_none(), "closed, no body");

        tree.click(toggle);
        tree.layout(size());
        let toggle = control_named(&tree, &name);
        assert!(tree.accessibility_node(toggle).is_expanded());
        assert!(
            tree.find_by_label(&title).is_some(),
            "open, the exact title"
        );
    });
}

/// Every widget that paints something wider than the window, with its name.
fn wider_than_the_window(tree: &WidgetTree, width: f32) -> Vec<String> {
    let mut found = Vec::new();
    let mut stack = tree.roots();
    while let Some(id) = stack.pop() {
        let b = tree.bounds(id);
        if b.width > 0.0 && b.x + b.width > width + 0.5 {
            found.push(format!(
                "{} {:?} at {}..{}",
                tree.widget_type_name(id)
                    .and_then(|n| n.rsplit("::").next())
                    .unwrap_or("?"),
                tree.accessibility_node(id).name(),
                b.x,
                b.x + b.width
            ));
        }
        stack.extend(tree.children(id));
    }
    found
}

/// At the window's smallest width nothing paints past its right edge, whatever the
/// report holds: the search column keeps its width, and the longest kind, a card
/// with a long path and every open block wrap or shorten inside the report column.
/// German is in the list for its long labels, French for its spaced punctuation.
#[test]
fn nothing_paints_past_the_window_at_its_smallest() {
    for locale in ["en-US", "de-DE", "fr-FR"] {
        with_shipped_messages(locale, || {
            let vm = TranslationReportViewModel::new();
            let mut tree = mounted(&vm);
            let check = |tree: &mut WidgetTree, state: &str| {
                tree.layout(size());
                let wide = wider_than_the_window(tree, 760.0);
                assert!(wide.is_empty(), "{locale}, {state}: {wide:#?}");
            };
            check(&mut tree, "empty");

            vm.open_for(Some(Scope::HelpPage(String::from(
                "help-report-translation-errors",
            ))));
            check(&mut tree, "scoped to a help page");
            vm.open_for(None);

            vm.query().set(String::from("e"));
            check(&mut tree, "a list of results");
            vm.pick_row(0);
            let longest = ProblemKind::ALL
                .into_iter()
                .max_by_key(|k| kind_label(*k).resolve_now().chars().count())
                .unwrap_or(ProblemKind::Other);
            vm.choose_kind(longest);
            check(&mut tree, "a card and the longest kind");

            let toggle = control_named(&tree, &tr!(translation_report_public()).resolve_now());
            tree.click(toggle);
            check(&mut tree, "what will be public, open");

            if let Some(plan) = vm.prepare_continue() {
                vm.continue_with(&plan, None, |_| Ok(()));
                check(&mut tree, "continued");
            }

            vm.report_another();
            vm.cannot_find();
            check(&mut tree, "not found");
        });
    }
}

/// The results take the height left in the search column, and the "can't find it"
/// button sits under them. A column given no height laid the list out zero pixels
/// tall: its rows spilled downwards and the button was drawn over the first match,
/// and over the prompt before anything was typed.
#[test]
fn the_matches_have_room_and_nothing_is_drawn_over_them() {
    with_shipped_messages("en-US", || {
        let vm = TranslationReportViewModel::new();
        let mut tree = mounted(&vm);
        let button = |tree: &WidgetTree| {
            tree.bounds(control_named(
                tree,
                &tr!(translation_report_not_found()).resolve_now(),
            ))
        };
        let prompt = tree
            .find_by_label(&tr!(translation_report_type_to_search()).resolve_now())
            .expect("the prompt");
        let (prompt, below) = (tree.bounds(prompt), button(&tree));
        assert!(
            prompt.y + prompt.height <= below.y + 0.5,
            "the button {below:?} is drawn over the prompt {prompt:?}"
        );

        vm.query().set("Cancel".to_string());
        tree.layout(size());
        let list = tree.bounds(control_named(
            &tree,
            &tr!(translation_report_results()).resolve_now(),
        ));
        let below = button(&tree);
        assert!(
            list.height > 200.0,
            "the list has {} px in a 540 px window",
            list.height
        );
        assert!(
            list.y + list.height <= below.y + 0.5,
            "the button {below:?} is drawn over the list {list:?}"
        );
    });
}
