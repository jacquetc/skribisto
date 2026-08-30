// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The left rail: the category tree and the search field above it.
//!
//! Both read the same [`tree_spec`](super::nav::tree_spec) the rest of the
//! window does, and both hand back the pieces a [`Navigator`] is assembled from
//! — the tree model, the selection model and the page → node map. They are free
//! functions
//! rather than methods because the only thing they need from the panel is which
//! page is open, and taking that as an argument is what makes them reachable
//! from a test that never builds a `SettingsPanel`.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::data::{KeyedSelectionModel, NodeId, SelectionMode, TreeModel, TreeSliceHandle};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
// The one definition of "a match" in this app: the project-wide search, the
// editor's Find and this box all fold text the same way. See `matches` below.
use teksilo::text_document::matching::{FoldLocale, MatchOptions, find_all};
use teksilo::widgets::{SearchField, StandardTreeItem, TreeView};

use super::nav::{Branch, GroupKind, Navigator, Pane, Root, Sec, ancestors_of};
use super::section_title;

/// The parent rows the rail starts **open**.
///
/// Design-state expansion, and it is not cosmetic. The rail viewport is ~519 px,
/// so about 18 rows are above the fold; the fully expanded tree is 33, and the
/// "Work: `<title>`" header used to be row 23 — below the fold on *every* open,
/// whatever the writer did.
pub(crate) const DEFAULT_EXPANDED: &[Pane] = &[
    Pane::Section(Sec::AppearanceBehaviour),
    Pane::Section(Sec::Work),
];

/// The parent rows the rail starts **closed** — the other half of
/// [`DEFAULT_EXPANDED`].
///
/// **Editor and its nested Typography group start collapsed; Work stays open.**
/// The twelve Editor rows are what pushed Work under; and of the two, Editor is
/// the one that can afford to be closed. Its pages are searchable by name and
/// every one of them has another way in (the Document and View menus, the
/// distraction-free strip, the games dock's own "Writing game settings…"). The
/// Work section's pages — the tag palette, the status ladder, the template
/// library — have neither: they are the two largest editors in the window and
/// Settings is the only door to any of them.
///
/// This list is what [`super::DEFAULT_PANE`] is checked against: a landing page
/// under any row named here re-expands it on open and the 33-row rail is back,
/// which is the regression `the_landing_pane_has_no_collapsed_ancestor` exists
/// to catch.
pub(crate) const DEFAULT_COLLAPSED: &[Pane] = &[
    Pane::Section(Sec::Editor),
    Pane::Group(GroupKind::Typography),
    Pane::Section(Sec::Spelling),
    Pane::Section(Sec::BackupSync),
    Pane::Section(Sec::CompileExport),
];

/// The category tree (left rail). Walks `spec` into a `TreeModel`, seeds
/// selection to the active page, and wires selection → `selected_pane`.
/// Returns the `TreeView`, a handle onto its expand/collapse set, its selection
/// model, and the page → node map every other way of reaching a page goes
/// through ([`Navigator`]).
///
/// The expand handle is handed back rather than kept private because a jump
/// from a search suggestion or a parent's link has to open the rows its target
/// hides behind, and half the sections start collapsed. It is a handle onto the
/// `TreeView`'s *own* slice: a second `TreeSlice` over the same `TreeModel`
/// carries independent expand state, so it would fold a tree nobody renders.
/// See [`Navigator::go`].
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
    Option<TreeSliceHandle<Pane>>,
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
    // agree on open (`super::DEFAULT_PANE` unless a deep link named another).
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
        let mut row = StandardTreeItem::new(label.clone())
            // Every other row here is a `tr!()` label of known length, but the Work
            // section's is "Work: <the project's title>" — as long as the writer named
            // their book. See `binder::dock`.
            .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
            // …and an ellipsized label is otherwise unrecoverable. The rail is 262px
            // wide and the cut is not rare: French renders Appearance & Behavior as
            // "Apparence et comportement", which already truncates at the default
            // text scale, and there is nowhere else in the window that spells the row
            // out. The tooltip carries the same `LocalizedString` the row does, so it
            // follows a runtime locale switch with it.
            .tooltip(label)
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
    // **Rows measure themselves.** A rail row's natural height is 31.7px at
    // interface text scale 1.3 and 35.3 at 1.5 — and that scale runs from 80% to
    // 200%, edited from Appearance ▸ Interface text size, a pane *inside this very
    // window*. A uniform height pins every row at the constant regardless, so from
    // about 115% up every label clipped vertically while the writer watched. The
    // 28.0 is now the estimate for rows not yet realized. Same fix, same reason as
    // `binder::dock`.
    .auto_item_height(28.0);

    // Design-state expansion (see `DEFAULT_EXPANDED` / `DEFAULT_COLLAPSED`).
    //
    // A pane the spec left out (Work, with nothing open) has no node, so its line
    // is a no-op rather than a special case.
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
    for pane in DEFAULT_EXPANDED {
        expand(*pane);
    }
    for pane in DEFAULT_COLLAPSED {
        collapse(*pane);
    }
    // Reveal the page we opened at — a nicety for the default landing page, whose
    // one ancestor is expanded above anyway, and load-bearing for the `open_to_*`
    // deep links: Dictionaries sits under Spelling, Backup under Backup & Sync and
    // Games under Editor, all three of them collapsed above. Whatever pane the
    // panel opens at ends up expanded, visible and selected; the collapse above
    // only decides what the *rest* of the tree does. Idempotent with the
    // design-state expansion.
    for ancestor in ancestors_of(spec, selected_pane.get()) {
        expand(ancestor);
    }

    // The same expand set the chevrons and the loops above drive, handed out so
    // a jump that lands inside a folded branch can open it. `tree_slice()` is
    // `None` only on the `from_source` path; this rail is on the `TreeModel`
    // one, so it is `Some` here in practice.
    let reveal = tree.tree_slice().map(|slice| slice.handle());

    (tree, reveal, selection, nodes)
}

/// Everything the search field can find: a searchable label, and the page it
/// lives on.
///
/// Separate from the widget below so it can be asserted on. What is in this list
/// *is* what a writer can reach without hunting through the tree.
///
/// **Every leaf comes off `spec`, not off a hand-written list.** The hand-written
/// list reached 19 of 32 pages: the Margin marks, Paratext, Punctuation and
/// Spell-checking pages were missing, and so was every page of the open project's
/// own section bar one — typing "tags" or "statuses" found nothing, and those are
/// the two largest editors in the window. This file already diagnosed exactly that
/// failure for a *contributed* page and then guarded only that one case; the same
/// argument was true of thirteen built-in ones. Deriving the leaves from the spec
/// is what makes the guard hold for a page added later, and
/// `every_page_in_the_tree_is_searchable` below is what stops it rotting again.
///
/// What stays hand-written is the *field*-level rows — "Text width", "Typewriter
/// scrolling", "Your initials" — which name a setting rather than a page and
/// which no walk of the tree could produce.
///
/// Labels are `LocalizedString`s, resolved per keystroke (see [`search_rows`]),
/// so the whole index follows a runtime locale change.
pub(crate) fn search_index(spec: &[Root], work_title: &str) -> Vec<(LocalizedString, Pane)> {
    // ── Fields: a setting by the name the writer saw beside it, not by its page.
    let mut idx: Vec<(LocalizedString, Pane)> = vec![
        // Both User fields by name: someone hunting for this is far likelier to
        // type "initials" — the word they saw beside a comment — than "user".
        (tr!(settings_field_user_name()), Pane::User),
        (tr!(settings_field_user_initials()), Pane::User),
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
    // Typeface / Size / Line height / First-line indent / the two paragraph
    // spacings repeat on every typography page, so disambiguate each by page
    // ("Scene — Typeface"). All six, not four: the two spacings were left out, so
    // "paragraph spacing" found nothing on any of the five pages that set it.
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
            tr!(settings_field_paragraph_spacing_before()),
            tr!(settings_field_paragraph_spacing_after()),
        ] {
            let page = page.clone();
            idx.push((
                localized(move || format!("{} — {}", page.resolve_now(), field.resolve_now())),
                pane,
            ));
        }
    }
    // ── Pages: every row of the tree, off the same spec the tree is built from.
    //
    // Parents included — a section and the Typography group are pages like any
    // other now, and a search that could not reach them would be the one place in
    // the window still treating them as mere branches. A contributed page is
    // reached the same way, which is the only way it *can* be reached: its label
    // lives in the registry and not in this file.
    //
    // `Pane::label()` resolves through the registry per keystroke, like the tree
    // row's own label, so a runtime locale switch reaches these too.
    //
    // One behaviour follows from taking the leaves off the spec rather than a
    // list: with no project open the spec carries no Work section, so its ten
    // pages are not searchable either. That is the same condition the tree
    // applies, and it is the honest answer — the alternative was one of the ten
    // (Text replacements) being hardcoded and so answering a search by landing on
    // an empty "no project" placeholder while its nine siblings answered nothing.
    for root in spec {
        match root {
            Root::Page(page) => idx.push((page.label(), *page)),
            Root::Section(sec, branches) => {
                idx.push((section_title(*sec, work_title), Pane::Section(*sec)));
                for branch in branches {
                    match branch {
                        Branch::Page(page) => idx.push((page.label(), *page)),
                        Branch::Group(group, pages) => {
                            idx.push((group.label(), Pane::Group(*group)));
                            for page in pages {
                                idx.push((page.label(), *page));
                            }
                        }
                    }
                }
            }
        }
    }
    idx
}

/// Each page's parent title — what a colliding row is disambiguated *by*.
///
/// A root page (User, Keymap) has no parent and so no entry; a group's pages take
/// the group, not the section, because that is the narrower answer and the one the
/// typography rows in [`search_index`] already compose by hand.
fn parent_titles(spec: &[Root], work_title: &str) -> HashMap<Pane, LocalizedString> {
    let mut out: HashMap<Pane, LocalizedString> = HashMap::new();
    for root in spec {
        let Root::Section(sec, branches) = root else {
            continue;
        };
        let section = section_title(*sec, work_title);
        for branch in branches {
            match branch {
                Branch::Page(page) => {
                    out.insert(*page, section.clone());
                }
                Branch::Group(group, pages) => {
                    out.insert(Pane::Group(*group), section.clone());
                    for page in pages {
                        out.insert(*page, group.label());
                    }
                }
            }
        }
    }
    out
}

/// The index as the writer sees it *right now*: resolved, disambiguated, deduped.
///
/// Resolution happens here rather than in [`search_index`] for one reason: two
/// pages that read the same in one language may read differently in another, so
/// **which** rows need disambiguating is a question that can only be answered in
/// the current locale. Deciding it when the window was built would leave a page
/// unreachable for anyone who switched language with Settings open — and switching
/// language is done from a pane in this very window.
///
/// A text naming more than one page is prefixed with that page's parent
/// ("Work: Starforgers — Backups"), the convention the typography rows already
/// use. Anything still identical after that is dropped, keeping the first — which
/// is what would make a page unreachable, and what
/// `every_page_in_the_tree_is_searchable` exists to catch.
pub(crate) fn search_rows(
    index: &[(LocalizedString, Pane)],
    parents: &HashMap<Pane, LocalizedString>,
) -> Vec<(String, Pane)> {
    let resolved: Vec<(String, Pane)> = index
        .iter()
        .map(|(label, pane)| (label.resolve_now(), *pane))
        .collect();

    // A text is ambiguous when it names two *different* pages. Plain repetition is
    // not ambiguity: a page listed once by name and once by a field of its own
    // resolves to the same string twice and needs no parent.
    let mut owner: HashMap<&str, Pane> = HashMap::new();
    let mut ambiguous: HashSet<&str> = HashSet::new();
    for (text, pane) in &resolved {
        match owner.get(text.as_str()) {
            Some(first) if first != pane => {
                ambiguous.insert(text.as_str());
            }
            Some(_) => {}
            None => {
                owner.insert(text.as_str(), *pane);
            }
        }
    }

    let mut out: Vec<(String, Pane)> = Vec::with_capacity(resolved.len());
    for (text, pane) in &resolved {
        let display = match parents.get(pane) {
            Some(parent) if ambiguous.contains(text.as_str()) => {
                format!("{} — {}", parent.resolve_now(), text)
            }
            _ => text.clone(),
        };
        if !out.iter().any(|(seen, _)| *seen == display) {
            out.push((display, *pane));
        }
    }
    out
}

/// How a typed query is compared against a row.
///
/// Through `text_document`'s matcher — the same one the project-wide search and
/// the editor's Find use — rather than `to_lowercase()` + `contains`, which folds
/// no diacritics: in the app that ships this matcher, `etiquettes` did not find
/// `Étiquettes`, and no French writer types the accent into a search box. The fold
/// is locale-aware too, so a Turkish interface folds its dotted and dotless i the
/// way Turkish does.
fn matches(text: &str, needle: &str) -> bool {
    let options = MatchOptions {
        case_sensitive: false,
        diacritic_sensitive: false,
        whole_word: false,
        locale: teksilo::i18n::current_locale()
            .map(|sig| FoldLocale::from_tag(&sig.get().to_string()))
            .unwrap_or_default(),
    };
    !find_all(text, needle, &options).is_empty()
}

/// The "Search settings" field, offering suggestions across every page and
/// setting; selecting a suggestion jumps to (and highlights) its page.
///
/// Returns the field's own [`WidgetId`], which the panel hands back as its
/// initial-focus hint: opening Settings parked focus on the header's ✕ otherwise,
/// which is the one control in the window nobody opened it to reach.
pub(crate) fn search_field(
    ctx: &mut BuildContext,
    spec: &[Root],
    work_title: &str,
    nav: Navigator,
) -> WidgetId {
    let index: Rc<Vec<(LocalizedString, Pane)>> = Rc::new(search_index(spec, work_title));
    let parents: Rc<HashMap<Pane, LocalizedString>> = Rc::new(parent_titles(spec, work_title));

    let (suggest_index, suggest_parents) = (index.clone(), parents.clone());
    let (select_index, select_parents) = (index, parents);

    ctx.add(
        SearchField::new(Signal::new(String::new()))
            .placeholder(tr!(settings_search()))
            .max_suggestions(8)
            .with_suggestions(move |prefix: &str| {
                let needle = prefix.trim();
                if needle.is_empty() {
                    return Vec::new();
                }
                search_rows(&suggest_index, &suggest_parents)
                    .into_iter()
                    .filter(|(text, _)| matches(text, needle))
                    .map(|(text, _)| text)
                    .collect()
            })
            .on_select(move |value: &str, _ctx| {
                // Resolved the same way the suggestion was, so the string that came
                // back names exactly one page — including a disambiguated one.
                if let Some((_, pane)) = search_rows(&select_index, &select_parents)
                    .into_iter()
                    .find(|(text, _)| text == value)
                {
                    nav.go(pane);
                }
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::nav::{all_panes, tree_spec};

    /// The tree with everything switched on: a project open, one extension page.
    fn full_spec() -> Vec<Root> {
        tree_spec(true, &["t.extension.page"])
    }

    fn rows(spec: &[Root], work_title: &str) -> Vec<(String, Pane)> {
        search_rows(
            &search_index(spec, work_title),
            &parent_titles(spec, work_title),
        )
    }

    /// **Every page in the tree can be found by typing its name.**
    ///
    /// The index used to be a hand-written list, and it had drifted to 19 of the
    /// 32 leaves: Margin marks, Paratext, Punctuation and Spell-checking were
    /// missing, and so was every page of the open project's section except Text
    /// replacements — the tag palette, the status ladder and the template library
    /// among them, which are the largest editors in the window and which the tree
    /// does not scroll to. Searching is how a writer reaches a page they cannot
    /// see; for thirteen pages it did nothing.
    ///
    /// This is the guard that stops it happening again: it walks the same
    /// `all_panes` the tree is asserted against, so a page added to the spec and
    /// forgotten here fails rather than silently going missing.
    #[test]
    fn every_page_in_the_tree_is_searchable() {
        let spec = full_spec();
        let rows = rows(&spec, "Starforgers");
        for pane in all_panes(&spec) {
            assert!(
                rows.iter().any(|(_, p)| *p == pane),
                "'{}' has a row in the settings tree and no way to search for it",
                pane.id()
            );
        }
    }

    /// **No two pages answer to the same typed text.**
    ///
    /// The suggestion list dedupes by resolved text and the selection resolves by
    /// first match, so a collision does not read as a collision — it silently
    /// makes the *second* page unreachable. Two pairs collided: both punctuation
    /// pages resolved through one key, and Backup/Backups read the same in English
    /// and in French. Disambiguation by parent is the fix; this asserts the result.
    #[test]
    fn no_two_pages_answer_to_the_same_search_text() {
        let spec = full_spec();
        let mut by_text: HashMap<String, Pane> = HashMap::new();
        for (text, pane) in rows(&spec, "Starforgers") {
            if let Some(other) = by_text.insert(text.clone(), pane) {
                assert_eq!(
                    other,
                    pane,
                    "'{text}' names two different settings pages ('{}' and '{}'), so one of \
                     them cannot be reached by search",
                    other.id(),
                    pane.id()
                );
            }
        }
    }

    /// **The app-level and project-level twins are told apart on the row itself.**
    ///
    /// Not merely in the index: these four are tree rows one section apart, and
    /// their distinguishing descriptions render only on a parent's landing page.
    #[test]
    fn the_app_and_project_twins_have_distinct_labels() {
        for (app, project) in [
            (Pane::Punctuation, Pane::WorkPunctuation),
            (Pane::Backup, Pane::WorkBackup),
        ] {
            assert_ne!(
                app.label().resolve_now(),
                project.label().resolve_now(),
                "'{}' and '{}' render the same text in the rail, one section apart",
                app.id(),
                project.id()
            );
        }
    }

    /// **A collision is disambiguated by parent, and only a collision is.**
    ///
    /// The pair below is the one that shipped: "Backups" under Backup & Sync and
    /// "Backups" under the open project read identically in English *and* in
    /// French, one section apart. The same page named twice — once by its own
    /// label and once by a field on it — is not a collision and keeps its bare
    /// name, which is what stops every row growing a prefix.
    #[test]
    fn a_collision_is_disambiguated_by_parent() {
        let index = vec![
            (lit!("Backups".to_string()), Pane::Backup),
            (lit!("Backups".to_string()), Pane::WorkBackup),
            (lit!("Keymap".to_string()), Pane::Keymap),
            (lit!("Keymap".to_string()), Pane::Keymap),
        ];
        let mut parents: HashMap<Pane, LocalizedString> = HashMap::new();
        parents.insert(Pane::Backup, lit!("Backup & Sync".to_string()));
        parents.insert(Pane::WorkBackup, lit!("Work: Starforgers".to_string()));

        assert_eq!(
            search_rows(&index, &parents),
            vec![
                ("Backup & Sync — Backups".to_string(), Pane::Backup),
                ("Work: Starforgers — Backups".to_string(), Pane::WorkBackup),
                ("Keymap".to_string(), Pane::Keymap),
            ]
        );
    }

    /// **A query matches without its accents.**
    ///
    /// `to_lowercase()` + `contains` — what this used — folds case and nothing
    /// else, so `etiquettes` did not find `Étiquettes` in an app that ships a
    /// diacritic-folding matcher and uses it for every other search it offers.
    #[test]
    fn a_query_finds_an_accented_label_without_the_accents() {
        assert!(matches("Étiquettes", "etiquettes"));
        assert!(matches("Réglages de ponctuation", "REGLAGES"));
        assert!(!matches("Étiquettes", "statuts"));
    }

    /// **A page reached only through the extension registry is searchable too**,
    /// and by its registered label rather than its id.
    #[test]
    fn a_contributed_page_carries_its_registered_label() {
        let _h = crate::settings_ext::register_page(
            "test.settings.tree.search",
            crate::settings_ext::SettingsPage {
                id: "t.tree.search.page",
                label: Rc::new(|| lit!("Record & check-in".to_string())),
                build: Rc::new(|_| Box::new(teksilo::widgets::Spacer::new())),
            },
        )
        .expect("register");

        let spec = tree_spec(true, &["t.tree.search.page"]);
        let rows = rows(&spec, "Starforgers");
        assert!(
            rows.iter().any(
                |(text, pane)| *pane == Pane::Extension("t.tree.search.page")
                    && text == "Record & check-in"
            ),
            "a contributed page must be searchable by its registered label"
        );
    }
}
