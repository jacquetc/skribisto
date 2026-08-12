// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use std::collections::{HashMap, HashSet};

use teksilo::data::{KeyedSelectionModel, NodeId, SelectionMode, TreeModel};

use super::nav::{GroupKind, Root, all_panes, ancestors_of, children_of};
use super::*;
use skribisto_model::counting::CountingMethodSetting;

// NOTE: `build()` reads `ctx.settings()` / `ctx.theme_signal()`, which a bare
// `WidgetTree` can't provide (no way to register a `SettingsStore` app-state
// from outside `teksilo-core`), so the full panel is verified live via
// `scripts/automation_settings.py` rather than headlessly — the same boundary
// the settings-dependent `WelcomePanel` sits on.

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
        Pane::MenusToolbars,
        Pane::Notifications,
        Pane::SceneTypography,
        Pane::SynopsisTypography,
        Pane::NotesTypography,
        Pane::EditorBehavior,
        Pane::Goals,
        Pane::Games,
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

/// Opening straight at a page reveals it: the deep-link opens
/// (`open_to_dictionaries`, `open_to_games`, `open_to_backup`) and the default
/// landing page all sit under a parent that starts collapsed or two levels down.
#[test]
fn a_pages_ancestors_are_every_row_that_must_be_expanded_to_see_it() {
    let spec = full_spec();
    assert_eq!(
        ancestors_of(&spec, Pane::SceneTypography),
        vec![
            Pane::Section(Sec::Editor),
            Pane::Group(GroupKind::Typography)
        ],
        "the default landing page is two levels down"
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
    assert_eq!(children_of(&open, Pane::Section(Sec::Work)).len(), 9);
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

/// A link on a parent's page moves the tree's highlight *and* switches the pane.
///
/// Doing only the second is the failure mode worth a test: the right-hand pane
/// would change while the tree went on highlighting the parent, which is the exact
/// disagreement between window and navigation that these pages were added to fix.
#[test]
fn following_a_link_moves_the_highlight_and_the_pane() {
    let selection = KeyedSelectionModel::<NodeId>::new(SelectionMode::Single);
    let model: TreeModel<Pane> = TreeModel::new();
    let scene = model.insert_root(0, Pane::SceneTypography);
    let nav = Navigator {
        nodes: Rc::new(HashMap::from([(Pane::SceneTypography, scene)])),
        selection: selection.clone(),
        selected_pane: Signal::new(Pane::Section(Sec::Editor)),
    };

    nav.go(Pane::SceneTypography);
    assert_eq!(nav.selected_pane.get(), Pane::SceneTypography);
    assert_eq!(selection.selected_keys(), vec![scene]);

    // A per-project page resolves to no node when nothing is open. The pane still
    // switches — to that page's "no project" placeholder — rather than the click
    // doing nothing at all.
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

    let spec = full_spec();
    let nav = Navigator {
        nodes: Rc::new(HashMap::new()),
        selection: KeyedSelectionModel::<NodeId>::new(SelectionMode::Single),
        selected_pane: Signal::new(Pane::Section(Sec::Editor)),
    };
    for parent in all_panes(&spec)
        .into_iter()
        .filter(|p| matches!(p, Pane::Section(_) | Pane::Group(_)))
    {
        let mut tree = WidgetTree::new();
        let built = tree.add_boxed(Box::new(crate::tabs::Boxed::new(Box::new(
            panes::overview::overview_pane(
                parent,
                parent.label(),
                None,
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
