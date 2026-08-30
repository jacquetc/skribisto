// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use std::collections::{HashMap, HashSet};

use teksilo::data::{KeyedSelectionModel, NodeId, SelectionMode, TreeModel, TreeSlice};

use super::nav::{GroupKind, Root, all_panes, ancestors_of, children_of};
use super::*;
use skribisto_model::counting::CountingMethodSetting;

// NOTE: `build()` reads `ctx.settings()`, which a bare `WidgetTree` cannot
// provide — but `test_support::tree_with_settings` can, by registering a real
// `SettingsStore` over a throwaway file as `app_state`. So the panel IS built
// headlessly here (see `the_panel_builds_and_lays_out_with_no_project_open`);
// what stays out of reach is everything behind the app-level singletons the
// real process registers (`DictionariesViewModel`, `ExportStylesViewModel`, …),
// which resolve to `None` in a test and render their empty placeholder. Those
// pages are verified live via `scripts/automation_settings.py`.

/// Every pane's id is unique.
///
/// Replaces `pane_discriminants_are_a_gap_free_range`, which pinned a property that no
/// longer exists: the `Switcher` slot is now looked up in the pane list rather than
/// being `pane as usize`, so declaration order carries no meaning and there is no gap
/// to leave. What *does* still have to hold is that two pages cannot share an
/// identity — a duplicate would make the tree select one and the panel show the other.
///
/// Parents are pages too, and their ids are the ones most at risk: Backup & Sync and
/// the page inside it are both "backup" in English, which is why one carries a
/// `section-` prefix.
#[test]
fn every_pane_id_is_unique() {
    let all = every_built_in_pane();
    let mut seen = std::collections::HashSet::new();
    for pane in &all {
        assert!(
            seen.insert(pane.id()),
            "two settings pages share the id '{}' — the tree would select one and the \
                 panel show the other",
            pane.id()
        );
    }
    assert_eq!(seen.len(), all.len());
}

/// Every built-in page — leaves and parents — shared by the tests below.
///
/// Order is irrelevant: every test over it is about the set, so a page inserted
/// anywhere is fine as long as it is here. What it must be is *complete*, which
/// `the_tree_places_every_built_in_page_exactly_once` is what proves.
fn every_built_in_pane() -> Vec<Pane> {
    vec![
        Pane::Section(Sec::AppearanceBehaviour),
        Pane::Section(Sec::Editor),
        Pane::Section(Sec::Spelling),
        Pane::Section(Sec::BackupSync),
        Pane::Section(Sec::CompileExport),
        Pane::Section(Sec::Work),
        Pane::Section(Sec::Extensions),
        Pane::Group(GroupKind::Typography),
        Pane::User,
        Pane::Appearance,
        Pane::Notifications,
        Pane::SceneTypography,
        Pane::SynopsisTypography,
        Pane::NotesTypography,
        Pane::EditorBehavior,
        Pane::Goals,
        Pane::Games,
        Pane::MarginLane,
        Pane::Corkboard,
        Pane::Dictionaries,
        Pane::Autosave,
        Pane::ExportFormats,
        Pane::Paratext,
        Pane::Keymap,
        Pane::WorkStructure,
        Pane::WorkPunctuation,
        Pane::Punctuation,
        Pane::Backup,
        Pane::WorkBackup,
        Pane::WorkLanguage,
        Pane::WorkDictionary,
        Pane::Spellcheck,
        Pane::WorkTags,
        Pane::WorkStatuses,
        Pane::WorkAuthor,
        Pane::WorkTextReplacements,
        Pane::DistractionFree,
        Pane::DistractionFreeThemes,
        Pane::WorkTemplates,
    ]
}

/// The tree spec with everything switched on: a project open, one extension page.
fn full_spec() -> Vec<Root> {
    tree_spec(true, &["t.extension.page"])
}

/// **The tree places every page, and places it once.**
///
/// A `Pane` variant that never reaches [`tree_spec`] is a page with a body in the
/// `Switcher` and no row to select it — invisible, and only reachable by search if
/// somebody remembered to index it. One placed twice gives two rows fighting over
/// one highlight. Both used to be possible: the tree was a run of `insert_child`
/// calls with nothing checking them against the enum.
///
/// `Pane::Games` is why this test exists in this shape — it was the one page the
/// old hand-kept `every_built_in_pane` list had drifted away from.
#[test]
fn the_tree_places_every_built_in_page_exactly_once() {
    let placed = all_panes(&full_spec());

    let mut seen = HashSet::new();
    for pane in &placed {
        assert!(
            seen.insert(*pane),
            "'{}' appears twice in the settings tree",
            pane.id()
        );
    }

    let mut expected: HashSet<Pane> = every_built_in_pane().into_iter().collect();
    expected.insert(Pane::Extension("t.extension.page"));

    let missing: Vec<&str> = expected.difference(&seen).map(|p| p.id()).collect();
    assert!(
        missing.is_empty(),
        "pages with no row in the tree: {missing:?}"
    );
    let unexpected: Vec<&str> = seen.difference(&expected).map(|p| p.id()).collect();
    assert!(
        unexpected.is_empty(),
        "the tree places pages the shared list has never heard of: {unexpected:?}"
    );
}

/// **Every page is listed by exactly one parent, and every parent lists something.**
///
/// This is the property the parent pages exist for: a page that no parent links to
/// can only be found by expanding the tree and reading, which is the state the
/// window was in before. A parent with an empty list would render a title, a lead
/// line and nothing at all.
#[test]
fn every_page_is_linked_from_exactly_one_parent() {
    let spec = full_spec();
    // User and Keymap are roots of their own — deliberately, each is one page
    // and needs no section — so they are the pages with no parent to list them.
    let roots: HashSet<Pane> = [Pane::User, Pane::Keymap].into_iter().collect();

    let mut listed_by: HashMap<Pane, Vec<Pane>> = HashMap::new();
    let mut parents = 0;
    for parent in all_panes(&spec) {
        let children = children_of(&spec, parent);
        if matches!(parent, Pane::Section(_) | Pane::Group(_)) {
            parents += 1;
            assert!(
                !children.is_empty(),
                "'{}' is a parent with nothing under it",
                parent.id()
            );
        } else {
            assert!(
                children.is_empty(),
                "'{}' is a leaf and must have no children",
                parent.id()
            );
        }
        for child in children {
            listed_by.entry(child).or_default().push(parent);
        }
    }
    assert_eq!(parents, 8, "seven sections plus the Typography group");

    for pane in all_panes(&spec) {
        if roots.contains(&pane) || matches!(pane, Pane::Section(_)) {
            continue;
        }
        let parents = listed_by.get(&pane).map(Vec::as_slice).unwrap_or_default();
        assert_eq!(
            parents.len(),
            1,
            "'{}' is listed by {} parents, not one",
            pane.id(),
            parents.len()
        );
    }
}

/// A nested group is **one** entry on its section's page, not six.
///
/// Flattening it would undo the reason the group exists (six typography pages
/// crowd the Editor list), and would leave the group's own page linked from
/// nowhere.
#[test]
fn a_section_lists_a_nested_group_as_one_entry() {
    let spec = full_spec();
    let editor = children_of(&spec, Pane::Section(Sec::Editor));
    assert_eq!(
        editor.first().copied(),
        Some(Pane::Group(GroupKind::Typography)),
        "Typography leads the Editor section"
    );
    assert!(
        !editor.contains(&Pane::SceneTypography),
        "the group's pages must not also be listed by its section"
    );
    assert_eq!(
        children_of(&spec, Pane::Group(GroupKind::Typography)).len(),
        6
    );
}

/// Opening straight at a page reveals it: every deep-link open
/// (`open_to_dictionaries`, `open_to_games`, `open_to_backup`) sits under a
/// parent that starts collapsed, and Scene sits two levels down.
#[test]
fn a_pages_ancestors_are_every_row_that_must_be_expanded_to_see_it() {
    let spec = full_spec();
    assert_eq!(
        ancestors_of(&spec, Pane::SceneTypography),
        vec![
            Pane::Section(Sec::Editor),
            Pane::Group(GroupKind::Typography)
        ],
        "Scene is two levels down — which is exactly why it is no longer the \
         landing page (see `the_landing_pane_has_no_collapsed_ancestor`)"
    );
    assert_eq!(
        ancestors_of(&spec, Pane::Dictionaries),
        vec![Pane::Section(Sec::Spelling)]
    );
    assert_eq!(
        ancestors_of(&spec, Pane::Group(GroupKind::Typography)),
        vec![Pane::Section(Sec::Editor)],
        "a group is revealed by its section"
    );
    assert!(
        ancestors_of(&spec, Pane::Keymap).is_empty(),
        "a root needs nothing expanded"
    );
    assert!(
        ancestors_of(&spec, Pane::User).is_empty(),
        "the unsigned-comments toast deep-links straight here, so it must be \
             reachable with nothing expanded"
    );
    assert!(
        ancestors_of(&spec, Pane::Section(Sec::Editor)).is_empty(),
        "a section is a root"
    );

    // Every ancestor must itself be an expandable row, or the reveal loop in
    // `build_tree` would look up a node that cannot be expanded.
    for pane in all_panes(&spec) {
        for ancestor in ancestors_of(&spec, pane) {
            assert!(
                matches!(ancestor, Pane::Section(_) | Pane::Group(_)),
                "'{}' is not a row that expands",
                ancestor.id()
            );
        }
    }
}

/// **The page the window opens at must have no collapsed ancestor.**
///
/// This is the guard on defect #04, and it is the only thing that catches it:
/// `build_tree` reveals whatever pane the panel opened at, so a deep landing
/// page silently *undoes* the collapse in [`tree::DEFAULT_COLLAPSED`]. That is
/// not a visual nicety. The rail viewport is ~519 px — about 18 rows — and the
/// fully expanded tree is 33; with Editor and its Typography group re-expanded
/// the `Work: <title>` section is pushed below the fold on **every single
/// open**, and the Work pages (the tag palette, the status ladder, the template
/// library) have no other door in the whole app.
///
/// So the rule is a subset check rather than a spot assertion on one variant:
/// whichever page [`DEFAULT_PANE`] names, every row that has to be open for it
/// to be visible must already be open by design.
#[test]
fn the_landing_pane_has_no_collapsed_ancestor() {
    let spec = full_spec();
    let expanded: HashSet<Pane> = tree::DEFAULT_EXPANDED.iter().copied().collect();
    let collapsed: HashSet<Pane> = tree::DEFAULT_COLLAPSED.iter().copied().collect();

    // The two halves cannot overlap, or "starts open" is decided by whichever
    // loop in `build_tree` runs second.
    for pane in &expanded {
        assert!(
            !collapsed.contains(pane),
            "'{}' is in both DEFAULT_EXPANDED and DEFAULT_COLLAPSED",
            pane.id()
        );
    }

    for ancestor in ancestors_of(&spec, DEFAULT_PANE) {
        assert!(
            expanded.contains(&ancestor),
            "the Settings window opens at '{}', whose ancestor '{}' is not a row \
             that starts expanded. `build_tree` reveals the landing page, so \
             opening here re-expands '{}' on every single open — which is what \
             pushed the `Work: <title>` section below the fold of the ~519px \
             rail and made the tag palette, the status ladder and the template \
             library unreachable without scrolling. Either land on a page whose \
             ancestors are all in `tree::DEFAULT_EXPANDED`, or move '{}' into \
             that list and re-measure the rail.",
            DEFAULT_PANE.id(),
            ancestor.id(),
            ancestor.id(),
            ancestor.id(),
        );
    }

    // And the constructors really use it — the constant would otherwise be a
    // comment that compiles. `without_project` is the one buildable with no
    // `WorkSession`; `SettingsPanel::new` shares the same literal.
    assert_eq!(
        SettingsPanel::without_project().selected_pane.get(),
        DEFAULT_PANE,
        "the Launcher's Settings window does not open at DEFAULT_PANE"
    );
}

/// With no project open there is no Work section — so neither its page nor any of
/// its pages' links can be reached, rather than offering a link to a placeholder.
#[test]
fn the_work_section_is_absent_until_a_project_is_open() {
    let closed = tree_spec(false, &[]);
    assert!(!all_panes(&closed).contains(&Pane::Section(Sec::Work)));
    assert!(!all_panes(&closed).contains(&Pane::WorkTags));
    assert!(children_of(&closed, Pane::Section(Sec::Work)).is_empty());
    assert!(
        !all_panes(&closed).contains(&Pane::Section(Sec::Extensions)),
        "and no Extensions section when nothing is registered"
    );

    let open = tree_spec(true, &[]);
    assert!(all_panes(&open).contains(&Pane::Section(Sec::Work)));
    assert_eq!(
        children_of(&open, Pane::Section(Sec::Work)).len(),
        10,
        "author, structure, language, backup, dictionary, tags, statuses, templates, \
         text replacements, punctuation"
    );
}

/// With no project open the tree loses the Work section and **keeps every other
/// one** — the state the Launcher opens this window in, and the state the app
/// starts in on Linux and Windows.
///
/// The complement of the test above, and the half that matters for the Launcher:
/// the Work section going is expected, an app-level section going with it would
/// mean the writer still could not reach the theme, the dictionaries or the
/// keybindings before opening a project, which is the whole reason the window is
/// offered there.
#[test]
fn every_app_level_section_survives_with_no_project_open() {
    let panes = all_panes(&tree_spec(false, &[]));
    for sec in [
        Sec::AppearanceBehaviour,
        Sec::Editor,
        Sec::Spelling,
        Sec::BackupSync,
        Sec::CompileExport,
    ] {
        assert!(
            panes.contains(&Pane::Section(sec)),
            "the '{}' section vanished with the project",
            Pane::Section(sec).id()
        );
    }
    // One leaf from each, plus the two roots that are pages in their own right —
    // a section can survive as an empty row, which would be no better.
    for pane in [
        Pane::User,
        Pane::Keymap,
        Pane::Appearance,
        Pane::SceneTypography,
        Pane::Games,
        Pane::Dictionaries,
        Pane::Backup,
        Pane::ExportFormats,
    ] {
        assert!(
            panes.contains(&pane),
            "'{}' is unreachable until a project is open",
            pane.id()
        );
    }
    assert!(!panes.contains(&Pane::Section(Sec::Work)));
}

/// The whole window builds and lays out with **no project open**.
///
/// `SettingsPanel::without_project` is what the Launcher opens, and every
/// Work-scoped pane behind it has to reach its placeholder rather than a
/// `WorkSession` that is not there. A panic anywhere in that ~30-page build is
/// the failure this guards, and it is one no test on `tree_spec` alone can see:
/// the Work pages are still constructed and still mounted in the `Switcher`,
/// they are merely unreachable from the rail.
#[test]
fn the_panel_builds_and_lays_out_with_no_project_open() {
    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let mut tree = crate::test_support::tree_with_settings(&app_ctx);
    let root = tree.add(SettingsPanel::without_project());
    tree.layout(SizeProposal::exact(CARD_W, CARD_H));

    let card = tree.bounds(root);
    assert!(
        card.width >= MIN_CARD_W && card.height >= MIN_CARD_H,
        "the card collapsed: {}x{}",
        card.width,
        card.height
    );

    // Both halves of the two-pane window are really mounted. A build that bailed
    // early would still lay a `Panel` out at the full card size.
    let names = type_names(&tree, root);
    for needle in ["TreeView", "Switcher", "SearchField"] {
        assert!(
            names.iter().any(|n| n.contains(needle)),
            "no {needle} under the card with no project open"
        );
    }
}

/// Every widget type name at or under `root`, DFS pre-order.
fn type_names(tree: &teksilo::core::widget_tree::WidgetTree, root: WidgetId) -> Vec<&'static str> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if let Some(name) = tree.widget_type_name(id) {
            out.push(name);
        }
        stack.extend(tree.children(id));
    }
    out
}

/// Every built-in page says what it is for; only an extension's may not.
///
/// The gloss under a link is the whole reason a parent's page beats reading the
/// tree, so a page arriving without one is a page whose entry says nothing the
/// row above it didn't. An extension's is `None` by design — `SettingsPage`
/// carries no description for the app to show.
#[test]
fn every_built_in_page_says_what_it_is_for() {
    for pane in every_built_in_pane() {
        assert!(
            pane.description().is_some(),
            "'{}' has no description to put under its link",
            pane.id()
        );
    }
    assert!(Pane::Extension("t.extension.page").description().is_none());
}

/// A link on a parent's page **reveals** the target's row, moves the tree's
/// highlight onto it *and* switches the pane.
///
/// Doing only the last is the failure mode worth a test: the right-hand pane
/// would change while the tree went on highlighting the parent, which is the exact
/// disagreement between window and navigation that these pages were added to fix.
///
/// The reveal is the same failure one layer down, and it shipped: Editor and its
/// Typography group start collapsed (`tree::DEFAULT_COLLAPSED`), so selecting a
/// node inside them highlighted a row that is not rendered at all. It was masked
/// while the window opened at Editor ▸ Typography ▸ Scene, because landing there
/// expanded both rows as a side effect.
#[test]
fn following_a_link_reveals_the_row_and_moves_the_highlight_and_the_pane() {
    let spec = Rc::new(full_spec());
    let selection = KeyedSelectionModel::<NodeId>::new(SelectionMode::Single);
    let model: TreeModel<Pane> = TreeModel::new();
    // The real nesting, because `go` reads the trail off `spec`: Scene is a leaf
    // of the Typography group inside the Editor section.
    let editor = model.insert_root(0, Pane::Section(Sec::Editor));
    let typography = model.insert_child(editor, 0, Pane::Group(GroupKind::Typography));
    let scene = model.insert_child(typography, 0, Pane::SceneTypography);
    // The slice is what the `TreeView` owns in the real window, and it has to
    // outlive every handle taken off it — hence the binding rather than a
    // temporary.
    let slice = TreeSlice::new(model.clone());
    let nav = Navigator {
        nodes: Rc::new(HashMap::from([
            (Pane::Section(Sec::Editor), editor),
            (Pane::Group(GroupKind::Typography), typography),
            (Pane::SceneTypography, scene),
        ])),
        selection: selection.clone(),
        selected_pane: Signal::new(Pane::Section(Sec::Editor)),
        reveal: Some(slice.handle()),
        spec: spec.clone(),
    };
    assert!(
        !slice.is_expanded(editor) && !slice.is_expanded(typography),
        "the fixture must start folded, or the reveal below proves nothing"
    );

    nav.go(Pane::SceneTypography);
    assert!(
        slice.is_expanded(editor) && slice.is_expanded(typography),
        "the jump left the Scene row folded inside a collapsed branch — the pane \
         switched to a page whose highlighted row is not on screen"
    );
    assert_eq!(nav.selected_pane.get(), Pane::SceneTypography);
    assert_eq!(selection.selected_keys(), vec![scene]);

    // A per-project page resolves to no node when nothing is open. The pane still
    // switches — to that page's "no project" placeholder — rather than the click
    // doing nothing at all, and the missing node must not make the reveal panic.
    nav.go(Pane::WorkTags);
    assert_eq!(nav.selected_pane.get(), Pane::WorkTags);
    assert_eq!(
        selection.selected_keys(),
        vec![scene],
        "an unreachable page must not clear the highlight"
    );
}

/// A parent's page builds and lays out — title, lead line and one entry per child.
///
/// Headless, unlike the panel around it: `overview_pane` reads no settings store,
/// so it is one of the few settings surfaces a bare `WidgetTree` can host.
#[test]
fn a_parents_page_builds_and_lays_out() {
    use teksilo::core::widget_tree::WidgetTree;

    let spec = std::rc::Rc::new(full_spec());
    let nav = Navigator {
        nodes: Rc::new(HashMap::new()),
        selection: KeyedSelectionModel::<NodeId>::new(SelectionMode::Single),
        selected_pane: Signal::new(Pane::Section(Sec::Editor)),
        reveal: None,
        spec: spec.clone(),
    };
    let crumbs = Crumbs::new(spec.clone(), "", Some(nav.clone()));
    for parent in all_panes(&spec)
        .into_iter()
        .filter(|p| matches!(p, Pane::Section(_) | Pane::Group(_)))
    {
        let mut tree = WidgetTree::new();
        let built = tree.add_boxed(Box::new(crate::tabs::Boxed::new(Box::new(
            panes::overview::overview_pane(
                parent,
                parent.label(),
                &crumbs,
                children_of(&spec, parent),
                nav.clone(),
            ),
        ))));
        tree.layout(SizeProposal::exact(640.0, 520.0));
        let bounds = tree.bounds(built);
        assert!(
            bounds.width > 0.0 && bounds.height > 0.0,
            "'{}' laid out to nothing",
            parent.id()
        );
    }
}

/// The Goals pane bridges `CountingMethodSetting` to the `RadioGroup`'s `usize`
/// selection; the two conversions must round-trip and cover every variant, or a
/// stored method would render as the wrong (or default) radio.
#[test]
fn counting_method_index_bridge_round_trips_every_variant() {
    for m in [
        CountingMethodSetting::Auto,
        CountingMethodSetting::Whitespace,
        CountingMethodSetting::UnicodeWords,
        CountingMethodSetting::CjkHybrid,
    ] {
        assert_eq!(index_to_method(method_to_index(m)), m);
    }
    // The four radios map to 0..=3; anything else falls back to Auto (never panics).
    assert_eq!(method_to_index(CountingMethodSetting::Auto), 0);
    assert_eq!(method_to_index(CountingMethodSetting::CjkHybrid), 3);
    assert_eq!(index_to_method(99), CountingMethodSetting::Auto);
}

/// **What makes an extension page unable to collide with a built-in.**
///
/// `settings_ext::register_page` refuses an id with no dot in it, and every
/// built-in id here is a bare kebab-case word. That is the whole guarantee —
/// so if a built-in ever takes a dotted id, an extension could shadow it and
/// the tree would select one page while the panel showed another. This is the
/// test that would say so.
#[test]
fn no_built_in_page_id_is_namespaced() {
    for pane in every_built_in_pane() {
        assert!(
            !pane.id().contains('.'),
            "built-in page '{}' took a dotted id — an extension page can now shadow it",
            pane.id()
        );
    }
}

/// A contributed page keeps its registered label, and falls back to its id
/// rather than an empty row if the extension is gone.
#[test]
fn an_extension_page_labels_itself_and_degrades_visibly() {
    let pane = Pane::Extension("t.settings.page");
    assert_eq!(pane.id(), "t.settings.page");
    assert_eq!(
        pane.label().resolve_now(),
        "t.settings.page",
        "an unregistered page must still render something identifiable"
    );

    let _h = crate::settings_ext::register_page(
        "test.settings.label",
        crate::settings_ext::SettingsPage {
            id: "t.settings.page",
            label: Rc::new(|| lit!("Structure".to_string())),
            build: Rc::new(|_| Box::new(teksilo::widgets::Spacer::new())),
        },
    )
    .expect("register");
    assert_eq!(pane.label().resolve_now(), "Structure");
}

/// **A contributed page gets the window's own frame**, mounted in the window's
/// own geometry.
///
/// ## The bug this exists for
///
/// A registered page used to be pushed into the content `Switcher` **raw** — no
/// breadcrumb, no rule, and, the part that showed, no `ScrollArea` and no
/// insets. A page longer than the card was cut off at the bottom with nothing on
/// screen to say there was more, and a page with a paragraph in it ran off the
/// right-hand edge of the window.
///
/// The width half is the one worth spelling out, because it is invisible by
/// inspection. Wrap-mode text measured with no width offered lays out **on one
/// line**, however long the sentence, and `VStack` reports `max(offered, natural)`
/// on its cross axis so that over-constraint stays visible rather than being
/// swallowed. So one over-wide child inflates the whole page. `pane_frame`'s
/// `ScrollArea` is what re-proposes the real viewport width when the page is
/// *placed*, and a page without one keeps the width it was measured at.
///
/// Every built-in page survived only because its content is naturally narrow —
/// `FormLayout` rows, short labels, 300 px controls — which is exactly why
/// nothing here caught it. So the body below is deliberately the shape that
/// breaks: long wrap-mode paragraphs, and enough of them to overflow the card.
///
/// ## What the harness pins now
///
/// `SettingsPanel::build` used to wrap the pane in `Expand::horizontal { FixedSize
/// { height } }` — a **height-only** `FixedSize`, which forwards `width: None`, so
/// `ScrollArea` took its `natural_content_width` branch and measured the entire page
/// subtree unbounded (a user-shaped pane measured 1472 px, a paratext-shaped one
/// 1808 px) on every pass. Those pages still *placed* at 657 only because `Expand`
/// reports a zero flex basis when its parent leaves width open: one
/// `.respect_intrinsic()`, or one refactor dropping the `Expand`, and the card would
/// have blown open. The arrangement below is the fixed one — the pane's width is
/// **bound**, and each column claims its height from the card rather than from a
/// constant — and it is what this test reproduces.
#[test]
fn a_contributed_page_is_framed_like_a_built_in_one() {
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::{Divider, FixedSize, HStack, Spacer, Switcher, TextWidget};

    // `SettingsPanel`'s own constants. Private to that file, and repeated rather
    // than exported: what this test needs is the arrangement, and a shared
    // constant would not stop the arrangement itself from changing.
    const CARD_W: f32 = 920.0;
    const TREE_W: f32 = 262.0;
    /// The body's height for a full-size card — a *harness* number now, not the
    /// window's: the real body claims whatever the header leaves via
    /// `Expand::vertical`, so it follows a card that had to shrink.
    const BODY_H: f32 = 620.0 - 44.0 - 1.0;
    const PANE_W: f32 = CARD_W - TREE_W - super::RULE_W;
    const EPS: f32 = 0.5;

    let paragraph = "A contributed page may carry prose, and prose is the one thing this \
                     window measured with no width offered, so it laid out on a single line \
                     and ran off the right-hand edge of the card without anything reporting a \
                     problem at all.";
    let mut body = VStack::new().spacing(12.0);
    for _ in 0..12 {
        body = body.child(TextWidget::new(lit!(paragraph.to_string())));
    }

    let crumbs = Crumbs::new(std::rc::Rc::new(full_spec()), "", None);
    let page = super::content::extension_pane(
        &crumbs,
        "t.extension.page",
        lit!("A contributed page".to_string()),
        Box::new(body),
    );

    // ⚠ **A real text backend, or this asserts nothing.** With none,
    // `TextWidget` falls back to a single-line 8 px-per-character measure in
    // *every* mode: wrap text never wraps, every paragraph reports one 16 px
    // line, and a page that overflows in both directions measures as if it
    // fitted.
    let mut tree = WidgetTree::new().with_text_backend(std::rc::Rc::new(std::cell::RefCell::new(
        teksilo::canvas::MockTextBackend::new(),
    )));
    let root = tree.add(
        FixedSize::new().width(CARD_W).height(BODY_H).child(
            HStack::new()
                .spacing(0.0)
                // Each column claims the row's bound height and sits at its own
                // bound width — `Expand::vertical` inside an `HStack` reports no
                // horizontal flex, so the three widths are rigid and sum to the
                // card exactly.
                .child(
                    Expand::vertical().child(FixedSize::new().width(TREE_W).child(Spacer::new())),
                )
                .child(Expand::vertical().child(Divider::vertical()))
                .child(
                    Expand::vertical().child(
                        // **Width-bound**, not height-bound: this is what makes
                        // `ScrollArea` take its bounded branch instead of measuring
                        // the whole page subtree with no width offered at all.
                        FixedSize::new()
                            .width(PANE_W)
                            // The same `Switcher` the window switches with: it
                            // measures its child at the incoming proposal and
                            // places it at the bounds, and those are two
                            // different sizes here.
                            .child(Switcher::new(Signal::new(0usize)).child(page)),
                    ),
                ),
        ),
    );
    tree.layout(SizeProposal::exact(CARD_W, BODY_H));

    let hstack = tree.children(root)[0];
    let pane = tree.bounds(tree.children(hstack)[2]);
    assert!(
        (pane.width - PANE_W).abs() < EPS,
        "the reproduction is wrong: the pane came out {} wide, not {PANE_W}",
        pane.width
    );

    // The frame: breadcrumb row · rule · scrolling body.
    let framed = tree.children(tree.children(hstack)[2]);
    let page_root = descend(&tree, framed[0], &[0, 0]);
    let parts = tree.children(page_root);
    assert_eq!(
        parts.len(),
        3,
        "a contributed page is not sitting in `pane_frame` — it should be a breadcrumb row, \
         a rule and a scrolling body"
    );
    assert!(
        (tree.bounds(parts[0]).height - 34.0).abs() < EPS,
        "no breadcrumb header above a contributed page"
    );

    // Nothing crosses the pane's edge.
    let mut ids = Vec::new();
    subtree(&tree, page_root, &mut ids);
    let (left, right) = (pane.x, pane.x + pane.width);
    for id in &ids {
        let b = tree.bounds(*id);
        if b.width <= 0.0 {
            continue;
        }
        assert!(
            b.x >= left - EPS && b.x + b.width <= right + EPS,
            "a widget spans {:.1}..{:.1}, outside the pane's {left:.1}..{right:.1}",
            b.x,
            b.x + b.width
        );
    }

    // …and the page fills its slot exactly, with content taller than it. Both
    // halves: a page with no scroll area also "fills the slot", and a page whose
    // content fits also "does not overflow it".
    let root_bounds = tree.bounds(page_root);
    assert!(
        (root_bounds.height - pane.height).abs() < EPS,
        "the page is {:.1} tall in a {:.1} slot",
        root_bounds.height,
        pane.height
    );
    assert!(
        ids.iter().any(|id| tree.bounds(*id).height > pane.height),
        "nothing in the page is taller than the slot, so this asserts nothing about scrolling"
    );
}

/// Walk one child index at a time, panicking with the depth reached rather than
/// asserting about the wrong widget.
fn descend(
    tree: &teksilo::core::widget_tree::WidgetTree,
    from: WidgetId,
    path: &[usize],
) -> WidgetId {
    let mut id = from;
    for (depth, &i) in path.iter().enumerate() {
        let children = tree.children(id);
        id = *children
            .get(i)
            .unwrap_or_else(|| panic!("no child {i} at depth {depth} of {path:?}"));
    }
    id
}

/// Every widget in a subtree, root included.
fn subtree(tree: &teksilo::core::widget_tree::WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
    out.push(id);
    for child in tree.children(id) {
        subtree(tree, child, out);
    }
}

/// **A contributed page is findable by name.**
///
/// It was the one page in the window the search could not reach, and it is the
/// page that most needs to be reachable that way: the Extensions section is
/// last, the tree mounts a window of rows rather than all of them, and on a
/// fresh install its rows are below that window. Searching is how a writer gets
/// to a page they cannot see.
#[test]
fn a_contributed_page_is_searchable() {
    let _h = crate::settings_ext::register_page(
        "test.settings.search",
        crate::settings_ext::SettingsPage {
            id: "t.search.page",
            label: Rc::new(|| lit!("Record & check-in".to_string())),
            build: Rc::new(|_| Box::new(teksilo::widgets::Spacer::new())),
        },
    )
    .expect("register");

    let spec = tree_spec(true, &["t.search.page"]);
    let index = super::tree::search_index(&spec, "Starforgers");

    let found = index
        .iter()
        .find(|(_, pane)| *pane == Pane::Extension("t.search.page"));
    let (label, _) = found.expect(
        "a contributed page is missing from the search index, so the only way to it is finding \
         its row in a tree that does not scroll to it",
    );
    assert_eq!(
        label.resolve_now(),
        "Record & check-in",
        "the index must carry the page's registered label, not its id"
    );

    // The section above it is findable too — a writer who types the section name
    // lands on its overview page, which links to every page under it.
    assert!(
        index
            .iter()
            .any(|(_, pane)| *pane == Pane::Section(Sec::Extensions)),
        "the Extensions section itself is not searchable either"
    );
}

/// **The card fits the app's own minimum window.**
///
/// Project windows declare `.min_size(800, 600)` (`shell/windows.rs`), and the
/// card asked for a hard 920×620. `OverlayPlacement::Centered` clamps an overlay's
/// *bounds* to the viewport, so the card was never unclosable — but the body was
/// laid out against a `BODY_H` constant derived from `CARD_H`, so it stayed 575 px
/// tall however short the window was: header 44 + rule 1 + 575 = 620 into a
/// 600-tall box, losing the bottom 20 px of the 56 px footer band and cutting
/// *Done* in half. The same clip landed at 1366×768 @125 % (1092×614) and
/// 1920×1080 @175 % (1097×617), which is to say on two very ordinary desktops.
///
/// [`SettingsPanel::card_size`] is the answer, and its contract is worth pinning
/// rather than eyeballing: never larger than the slot, the preferred size when the
/// slot can hold it, and a [`VIEWPORT_MARGIN`] gutter given back the moment it
/// cannot.
#[test]
fn the_card_shrinks_into_the_window_it_was_given() {
    // A roomy desktop: the preferred card, no gutter taken.
    assert_eq!(SettingsPanel::card_size(1600.0, 1000.0), (CARD_W, CARD_H));

    // The app's own minimum window. The overlay hands us `min(preferred,
    // viewport)`, so the slot *is* the window on both axes here.
    assert_eq!(
        SettingsPanel::card_size(800.0, 600.0),
        (800.0 - VIEWPORT_MARGIN, 600.0 - VIEWPORT_MARGIN)
    );

    // The two scaled-desktop cases from the finding: wide enough, one row short.
    assert_eq!(
        SettingsPanel::card_size(920.0, 614.0),
        (CARD_W, 614.0 - VIEWPORT_MARGIN)
    );
    assert_eq!(
        SettingsPanel::card_size(920.0, 617.0),
        (CARD_W, 617.0 - VIEWPORT_MARGIN)
    );

    // Never larger than the slot — including a slot below the floor, where
    // clipping is the only remaining answer and the floor must not win.
    for (w, h) in [
        (300.0f32, 200.0f32),
        (560.0, 360.0),
        (640.0, 480.0),
        (800.0, 600.0),
        (920.0, 620.0),
        (2560.0, 1440.0),
    ] {
        let (cw, ch) = SettingsPanel::card_size(w, h);
        assert!(
            cw <= w + 0.001 && ch <= h + 0.001,
            "a {w}x{h} slot produced a {cw}x{ch} card"
        );
        assert!(
            cw <= CARD_W && ch <= CARD_H,
            "the card grew past its preferred size"
        );
    }

    // Stable: `layout_response` reports the *preferred* size on every pass, so the
    // slot the overlay hands back is `min(preferred, viewport)` every time — never
    // last frame's already-shrunken answer, which would shrink by another gutter
    // each frame until it hit the floor.
    let slot = (CARD_W.min(800.0), CARD_H.min(600.0));
    assert_eq!(
        SettingsPanel::card_size(slot.0, slot.1),
        SettingsPanel::card_size(CARD_W.min(800.0), CARD_H.min(600.0))
    );
}

/// **The footer band survives an 800×600 window, whole.**
///
/// The arrangement half of the finding above: `card_size` can return 568 all it
/// likes, but if the body still claims a constant 575 the footer is still clipped.
/// So this lays out the card's actual shape — header · rule · body, with the
/// footer at the bottom of the body's trailing column — inside the card size an
/// 800×600 window yields, and asserts the footer band is entirely inside it.
///
/// A replica of the shape rather than the panel itself, like
/// [`a_contributed_page_is_framed_like_a_built_in_one`] above: `SettingsPanel::build`
/// reads `ctx.settings()` / `ctx.theme_signal()`, which a bare `WidgetTree` cannot
/// provide.
#[test]
fn the_footer_band_is_whole_in_the_apps_minimum_window() {
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::{Divider, FixedSize, HStack, Spacer};

    const EPS: f32 = 0.5;
    let (card_w, card_h) = SettingsPanel::card_size(800.0, 600.0);
    let pane_w = pane_width(card_w);

    // The trailing column: scrolling content · rule · footer, exactly as
    // `SettingsPanel::build` assembles `right`.
    let right = VStack::new()
        .spacing(0.0)
        .child(Expand::vertical().child(Spacer::new()))
        .child(Expand::horizontal().child(Divider::new()))
        .child(FixedSize::new().height(FOOTER_H).child(Spacer::new()));

    // The `Panel` the real card wears is chrome, not arrangement, and its styled
    // body inserts a wrapper of its own — so the replica starts at the VStack.
    let card = VStack::new()
        .spacing(0.0)
        .child(Expand::horizontal().child(FixedSize::new().height(HEADER_H).child(Spacer::new())))
        .child(Expand::horizontal().child(Divider::new()))
        .child(
            Expand::vertical().child(
                HStack::new()
                    .spacing(0.0)
                    .child(
                        Expand::vertical()
                            .child(FixedSize::new().width(TREE_W).child(Spacer::new())),
                    )
                    .child(Expand::vertical().child(Divider::vertical()))
                    .child(Expand::vertical().child(FixedSize::new().width(pane_w).child(right))),
            ),
        );

    let mut tree = WidgetTree::new();
    let root = tree.add(FixedSize::new().width(card_w).height(card_h).child(card));
    tree.layout(SizeProposal::exact(card_w, card_h));

    // card > VStack > [header, rule, body] ; body > HStack > [rail, rule, pane] ;
    // pane > FixedSize > right VStack > [content, rule, footer].
    let footer = descend(&tree, root, &[0, 2, 0, 2, 0, 0, 2]);
    let fb = tree.bounds(footer);
    assert!(
        (fb.height - FOOTER_H).abs() < EPS,
        "the footer band is {:.1} tall, not {FOOTER_H}",
        fb.height
    );
    assert!(
        fb.y + fb.height <= card_h + EPS,
        "the footer runs to {:.1} in a {card_h:.1}-tall card — Done is clipped",
        fb.y + fb.height
    );
    assert!(
        (fb.y + fb.height - card_h).abs() < EPS,
        "the footer is not sitting on the card's bottom edge ({:.1} vs {card_h:.1})",
        fb.y + fb.height
    );

    // …and the rail and pane still tile the card's width exactly, with the pane
    // bound rather than left to an `Expand` that forwards `width: None`.
    let body = descend(&tree, root, &[0, 2, 0]);
    let columns = tree.children(body);
    assert_eq!(columns.len(), 3, "rail · rule · pane");
    let total: f32 = columns.iter().map(|c| tree.bounds(*c).width).sum();
    assert!(
        (total - card_w).abs() < EPS,
        "the three columns come to {total:.1} in a {card_w:.1}-wide card"
    );
    assert!(
        (tree.bounds(columns[2]).width - pane_w).abs() < EPS,
        "the pane column is {:.1} wide, not the bound {pane_w:.1}",
        tree.bounds(columns[2]).width
    );
}

/// **The card fits the Launcher's window — the one the writer cannot resize.**
///
/// Settings is now reachable from the Launcher, which makes that window a
/// Settings host, and it is unlike every other one: `launcher_window_config`
/// declares `min == max`, so 820×590 is not a *minimum* the writer can grow out
/// of — it is the only size that window will ever have. If the largest surface
/// in the app does not fit there, there is no gesture that makes it fit, and the
/// footer's *Done* is clipped for as long as the Launcher is on screen.
///
/// The existing [`the_card_shrinks_into_the_window_it_was_given`] pins the
/// project window's 800×600 floor, which the writer *can* grow; nothing pinned
/// this one. 590 is also the shortest viewport the card is offered anywhere in
/// the app, so it is the case where [`MIN_CARD_H`] would bite first.
#[test]
fn the_card_fits_the_launchers_fixed_window() {
    use crate::shell::launcher_window::{LAUNCHER_H, LAUNCHER_W};

    let (slot_w, slot_h) = (LAUNCHER_W as f32, LAUNCHER_H as f32);
    let (card_w, card_h) = SettingsPanel::card_size(slot_w, slot_h);

    // Inside the window, gutter and all — the card shrank rather than clipped.
    assert!(
        card_w <= slot_w && card_h <= slot_h,
        "a {card_w}x{card_h} card in the Launcher's {slot_w}x{slot_h} window"
    );
    assert_eq!(
        (card_w, card_h),
        (slot_w - VIEWPORT_MARGIN, slot_h - VIEWPORT_MARGIN),
        "the Launcher is smaller than the preferred card on both axes, so the \
         gutter is given back on both"
    );

    // …and it did not have to fall through to the floor to do it. Were the
    // Launcher ever shortened past `MIN_CARD_H + VIEWPORT_MARGIN`, the card
    // would stop shrinking and start clipping instead — silently, and in the
    // one window with no way out of it.
    assert!(
        card_w > MIN_CARD_W && card_h > MIN_CARD_H,
        "the card is at its floor ({card_w}x{card_h}) in the Launcher, so any \
         further shrink of that window clips it with no way to resize"
    );

    // The two-pane split still has a real pane column left at that width — the
    // rail is a fixed `TREE_W`, so this is what actually gets squeezed.
    assert!(
        pane_width(card_w) > TREE_W,
        "the pane column ({}) is narrower than the rail beside it",
        pane_width(card_w)
    );

    // The chrome the card owes at that height leaves the pages real room, so
    // the footer band is whole rather than eating the content column.
    assert!(
        card_h - HEADER_H - FOOTER_H > MIN_CARD_H / 2.0,
        "only {:.1} px left for the pages after header + footer",
        card_h - HEADER_H - FOOTER_H
    );
}
