// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The left rail: the category tree and the search field above it.
//!
//! Both read the same [`tree_spec`](super::nav::tree_spec) the rest of the
//! window does, and both hand back the pieces a [`Navigator`] is assembled from
//! — the selection model and the page → node map. They are free functions
//! rather than methods because the only thing they need from the panel is which
//! page is open, and taking that as an argument is what makes them reachable
//! from a test that never builds a `SettingsPanel`.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::data::{KeyedSelectionModel, NodeId, SelectionMode, TreeModel};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{SearchField, StandardTreeItem, TreeView};

use super::nav::{Branch, GroupKind, Navigator, Pane, Root, Sec, ancestors_of};
use super::section_title;

/// The category tree (left rail). Walks `spec` into a `TreeModel`, seeds
/// selection to the active page, and wires selection → `selected_pane`.
/// Returns the `TreeView`, its selection model, and the page → node map every
/// other way of reaching a page goes through ([`Navigator`]).
///
/// `work_title` fills the Work section's row label; it is ignored when the
/// spec carries no Work section.
pub(crate) fn build_tree(
    ctx: &mut BuildContext,
    selected_pane: &Signal<Pane>,
    spec: &[Root],
    work_title: String,
) -> (
    impl Widget,
    KeyedSelectionModel<NodeId>,
    HashMap<Pane, NodeId>,
) {
    let model: TreeModel<Pane> = TreeModel::new();
    let mut nodes: HashMap<Pane, NodeId> = HashMap::new();

    // The tree IS the spec — order, nesting and membership all come from it,
    // so the rows, the reveal-on-open expansion and each parent's own page
    // cannot describe three different trees.
    for (i, root) in spec.iter().enumerate() {
        match root {
            Root::Page(page) => {
                nodes.insert(*page, model.insert_root(i, *page));
            }
            Root::Section(sec, branches) => {
                let sec_pane = Pane::Section(*sec);
                let sec_node = model.insert_root(i, sec_pane);
                nodes.insert(sec_pane, sec_node);
                for (j, branch) in branches.iter().enumerate() {
                    match branch {
                        Branch::Page(page) => {
                            nodes.insert(*page, model.insert_child(sec_node, j, *page));
                        }
                        Branch::Group(group, pages) => {
                            let group_pane = Pane::Group(*group);
                            let group_node = model.insert_child(sec_node, j, group_pane);
                            nodes.insert(group_pane, group_node);
                            for (k, page) in pages.iter().enumerate() {
                                nodes.insert(*page, model.insert_child(group_node, k, *page));
                            }
                        }
                    }
                }
            }
        }
    }

    // Single-selection, seeded to the active page so the pane + highlight
    // agree on open (Editor ▸ Typography ▸ Scene by default).
    let selection = KeyedSelectionModel::<NodeId>::new(SelectionMode::Single);
    if let Some(id) = nodes.get(&selected_pane.get()) {
        selection.select(*id);
    }

    // Selection → active page. Every row is a page now, parents included:
    // selecting a section used to switch nothing, so the right-hand pane went
    // on showing whichever leaf was open last.
    {
        let selected_pane = selected_pane.clone();
        let model = model.clone();
        let sig = selection.selection_signal();
        ctx.effect(&sig, move |set: &HashSet<NodeId>| {
            if let Some(id) = set.iter().next().copied()
                && let Some(pane) = model.with_item(id, |p: &Pane| *p)
            {
                selected_pane.set(pane);
            }
        });
    }

    // A row-body click selects; the chevron (wired via `on_toggle_rc`)
    // expands/collapses a parent. Proven config (mirrors the framework's own
    // tree examples) — reliable for both mouse and synthetic input.
    let tree = TreeView::new_with_context(model, move |pane: &Pane, entry, selected, rowctx| {
        // The Work section's label is dynamic ("Work: `<title>`"); every
        // other row uses its static label.
        let label = match pane {
            Pane::Section(Sec::Work) => section_title(Sec::Work, &work_title),
            other => other.label(),
        };
        let mut row = StandardTreeItem::new(label)
            // Every other row here is a `tr!()` label of known length, but the Work
            // section's is "Work: <the project's title>" — as long as the writer named
            // their book. See `binder::dock`.
            .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
            .from_entry(entry)
            .selected(selected)
            .on_toggle_rc(rowctx.toggle_callback());
        if let Some(icon) = pane.icon() {
            row = row.leading_slot(icon);
        }
        Box::new(row)
    })
    .keyed_selection(selection.clone())
    .row_click_expands(false)
    .item_height(28.0);

    // Design-state expansion: the first two sections open, the rest closed —
    // regardless of the model's default (collapse is a no-op if already so).
    // The nested Typography group opens too — it holds the default landing
    // page (Scene), so it must never start collapsed under the always-open
    // Editor section. A pane the spec left out (Work with nothing open) has
    // no node, so its line is a no-op rather than a special case.
    let expand = |pane: Pane| {
        if let Some(id) = nodes.get(&pane) {
            tree.expand(*id);
        }
    };
    let collapse = |pane: Pane| {
        if let Some(id) = nodes.get(&pane) {
            tree.collapse(*id);
        }
    };
    expand(Pane::Section(Sec::AppearanceBehaviour));
    expand(Pane::Section(Sec::Editor));
    expand(Pane::Group(GroupKind::Typography));
    collapse(Pane::Section(Sec::Spelling));
    collapse(Pane::Section(Sec::BackupSync));
    collapse(Pane::Section(Sec::CompileExport));
    expand(Pane::Section(Sec::Work));
    // Reveal the page we opened at, so a deep-link open (e.g. the toast that
    // jumps straight to Dictionaries, under the otherwise-collapsed Spelling
    // section) shows its highlighted tree node. Idempotent with the
    // design-state expansion above.
    for ancestor in ancestors_of(spec, selected_pane.get()) {
        expand(ancestor);
    }

    (tree, selection, nodes)
}

/// Everything the search field can find: a searchable label, and the page it
/// lives on.
///
/// Separate from the widget below so it can be asserted on. What is in this list
/// *is* what a writer can reach without hunting through the tree, and the entry
/// that was missing from it — a contributed page — was the one page in the
/// window they had no other way to get to.
///
/// `spec` contributes the parents and the contributed pages: a section and the
/// Typography group are pages like any other now, and a search that could not
/// reach them would be the one place in the window still treating them as mere
/// branches.
///
/// Labels are `LocalizedString`s, resolved per keystroke, so the whole index
/// follows a runtime locale change.
pub(crate) fn search_index(spec: &[Root], work_title: &str) -> Vec<(LocalizedString, Pane)> {
    let mut idx: Vec<(LocalizedString, Pane)> = vec![
        (tr!(settings_page_user()), Pane::User),
        // Both fields by name too: someone hunting for this is far likelier to
        // type "initials" — the word they saw beside a comment — than "user".
        (tr!(settings_field_user_name()), Pane::User),
        (tr!(settings_field_user_initials()), Pane::User),
        (tr!(settings_page_appearance()), Pane::Appearance),
        (tr!(settings_page_menus()), Pane::MenusToolbars),
        (tr!(settings_page_notifications()), Pane::Notifications),
        (tr!(settings_page_scene()), Pane::SceneTypography),
        (tr!(settings_page_synopsis()), Pane::SynopsisTypography),
        (tr!(settings_page_notes()), Pane::NotesTypography),
        (tr!(settings_page_editor_behavior()), Pane::EditorBehavior),
        (tr!(settings_page_goals()), Pane::Goals),
        (tr!(settings_page_games()), Pane::Games),
        (tr!(settings_page_corkboard()), Pane::Corkboard),
        (tr!(settings_page_distraction_free()), Pane::DistractionFree),
        (
            tr!(settings_page_distraction_free_themes()),
            Pane::DistractionFreeThemes,
        ),
        (tr!(settings_page_dictionaries()), Pane::Dictionaries),
        (tr!(settings_page_autosave()), Pane::Autosave),
        (tr!(settings_page_backup()), Pane::Backup),
        (tr!(settings_page_export()), Pane::ExportFormats),
        (tr!(settings_page_keymap()), Pane::Keymap),
        // Per-project pages resolve to no tree node when nothing is open —
        // `on_select` skips the selection and still switches the pane, which
        // lands on that page's "no project" placeholder. Correct either way.
        (
            tr!(settings_page_text_replacements()),
            Pane::WorkTextReplacements,
        ),
        (tr!(settings_text_width()), Pane::EditorBehavior),
        (tr!(settings_synopsis_placement()), Pane::EditorBehavior),
        (tr!(settings_typewriter()), Pane::EditorBehavior),
        (tr!(settings_highlight_scope()), Pane::EditorBehavior),
        // The distraction-free settings are all on their own page now —
        // typography, the column width and the strip's toggles together —
        // so every one of them searches there.
        (
            tr!(settings_distraction_free_title()),
            Pane::DistractionFree,
        ),
        (
            tr!(settings_distraction_free_word_count()),
            Pane::DistractionFree,
        ),
        (
            tr!(settings_distraction_free_session()),
            Pane::DistractionFree,
        ),
        (tr!(settings_distraction_free_go()), Pane::DistractionFree),
        (
            tr!(settings_distraction_free_go_to()),
            Pane::DistractionFree,
        ),
        (tr!(settings_field_column_width()), Pane::DistractionFree),
        (tr!(settings_field_app_theme()), Pane::Appearance),
        (tr!(settings_field_text_scale()), Pane::Appearance),
        (tr!(settings_field_language()), Pane::Appearance),
        (tr!(settings_show_welcome()), Pane::Appearance),
        (tr!(settings_autosave()), Pane::Autosave),
    ];
    // Typeface / Size / Line height / First-line indent repeat on every
    // typography page, so disambiguate each by page ("Scene — Typeface") —
    // the index dedupes by resolved text, so bare labels would collide and
    // leave all but one page unreachable.
    for (page, pane) in [
        (tr!(settings_page_scene()), Pane::SceneTypography),
        (tr!(settings_page_synopsis()), Pane::SynopsisTypography),
        (tr!(settings_page_notes()), Pane::NotesTypography),
        (tr!(settings_page_distraction_free()), Pane::DistractionFree),
        (
            tr!(settings_page_distraction_free_themes()),
            Pane::DistractionFreeThemes,
        ),
    ] {
        for field in [
            tr!(settings_field_typeface()),
            tr!(settings_field_size()),
            tr!(settings_field_line_height()),
            tr!(settings_field_first_line_indent()),
        ] {
            let page = page.clone();
            idx.push((
                localized(move || format!("{} — {}", page.resolve_now(), field.resolve_now())),
                pane,
            ));
        }
    }
    // The parents, from the same spec the tree is built from — so a section
    // added there is searchable without a second list to keep in step.
    for root in spec {
        if let Root::Section(sec, branches) = root {
            idx.push((section_title(*sec, work_title), Pane::Section(*sec)));
            for branch in branches {
                match branch {
                    Branch::Group(group, _) => idx.push((group.label(), Pane::Group(*group))),
                    // ⚠ **A contributed page is the one leaf this list cannot
                    // hardcode**, because its label lives in the registry and
                    // not in this file — so it was the one page in the window
                    // the search could not reach. That mattered more than it
                    // sounds: the Extensions section is last, the tree mounts a
                    // window of rows rather than all of them, and on a fresh
                    // install its rows are below that window. Searching is how a
                    // writer gets to a page they cannot see, and for exactly one
                    // kind of page it did nothing.
                    //
                    // `Pane::label()` resolves through the registry per
                    // keystroke, like the tree row's own label, so a runtime
                    // locale switch reaches this too.
                    Branch::Page(page @ Pane::Extension(_)) => idx.push((page.label(), *page)),
                    // Every other leaf is already named above, most of them
                    // twice — once as a page and once per field that should
                    // find it. Adding them here would only make the suggestion
                    // list repeat itself.
                    Branch::Page(_) => {}
                }
            }
        }
    }
    idx
}

/// The "Search settings" field, offering suggestions across every page and
/// setting; selecting a suggestion jumps to (and highlights) its page.
pub(crate) fn search_field(spec: &[Root], work_title: &str, nav: Navigator) -> impl Widget {
    let index: Rc<Vec<(LocalizedString, Pane)>> = Rc::new(search_index(spec, work_title));

    let for_suggest = index.clone();
    let for_select = index.clone();

    SearchField::new(Signal::new(String::new()))
        .placeholder(tr!(settings_search()))
        .max_suggestions(8)
        .with_suggestions(move |prefix: &str| {
            let needle = prefix.trim().to_lowercase();
            if needle.is_empty() {
                return Vec::new();
            }
            let mut out = Vec::new();
            for (label, _) in for_suggest.iter() {
                let s = label.resolve_now();
                if s.to_lowercase().contains(&needle) && !out.contains(&s) {
                    out.push(s);
                }
            }
            out
        })
        .on_select(move |value: &str, _ctx| {
            if let Some((_, pane)) = for_select
                .iter()
                .find(|(label, _)| label.resolve_now() == value)
            {
                nav.go(*pane);
            }
        })
}
