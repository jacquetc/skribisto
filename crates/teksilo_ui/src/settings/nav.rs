// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The category tree: what pages exist, how they nest, and how to reach one.
//!
//! [`tree_spec`] is the single declaration of order, nesting and membership.
//! Everything else in this module reads it — the `TreeView`'s own model, the
//! "reveal the page we opened at" expansion, each parent page's list of what is
//! under it — so a page cannot be in the tree and missing from its parent's
//! list, which is exactly what happened while three readers each held their own
//! copy of the shape.
//!
//! Split out of `settings.rs` because it is the settings window's *model* and
//! the rest of that file is its view: this half is a pure function of `has_work`
//! and the registered extension pages, testable without building a widget.

use std::collections::HashMap;
use std::rc::Rc;

use teksilo::canvas::svg::SvgIcon;
use teksilo::data::{KeyedSelectionModel, NodeId};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::IconWidget;

// ── Category tree model ──────────────────────────────────────────────────────

/// A selectable settings page — a tree *leaf*, or a *parent's own* landing page.
///
/// Identity is [`Pane::id`], a stable string — **not** the discriminant. The
/// discriminant used to be the page's `Switcher` slot, which made "append, never
/// insert" a real rule with a real cost: getting it wrong shifted every later page's
/// content by one, silently, and it shipped that way once. The `Switcher` index is now
/// derived from the pane list itself (see `build`), so declaration order in the enum
/// carries no meaning at all and a variant may be inserted anywhere.
///
/// The string id is what an extension-contributed page would carry, since it cannot be
/// given a variant here; giving the built-ins one now means the two kinds are already
/// addressed the same way.
///
/// A parent is a variant here rather than a separate concept so that it *is* a page
/// everywhere — the tree, the `Switcher`, the search index and the selection signal
/// all take it without a second code path, exactly as [`Pane::Extension`] does.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Pane {
    /// A top-level section's own page: its title and a link to each page under
    /// it (a nested group counts as one entry, linking to its own page).
    Section(Sec),
    /// A nested group's own page — same shape as a section's, one level in.
    Group(GroupKind),
    /// Who is using this installation — the name and initials comments are
    /// signed with. A root page of its own, like [`Pane::Keymap`]: it is one
    /// page's worth of content, so a section wrapping it would be a section with
    /// a single child, and it is about the *person*, which no existing section
    /// is. Deliberately **not** a "Skribisto" parent over the whole tree the way
    /// LibreOffice has one — that shape exists to separate app-wide settings from
    /// per-module ones (Writer, Calc, …), and this app has no modules.
    User,
    Appearance,
    MenusToolbars,
    Notifications,
    SceneTypography,
    SynopsisTypography,
    NotesTypography,
    EditorBehavior,
    Goals,
    /// Editor ▸ Writing games — the self-imposed drafting constraints
    /// ("Always forward"). Its own page rather than a corner of Editor
    /// Behavior: the switch at the top of it is per-session state, not a
    /// setting, which is a distinction that needs room to be explained.
    Games,
    Corkboard,
    Dictionaries,
    Autosave,
    ExportFormats,
    /// Compile & Export ▸ Paratext structures — the front/back matter catalogue.
    Paratext,
    Keymap,
    /// Per-project "Work: `<name>` ▸ Structure" — chapter mode (folder vs flat).
    WorkStructure,
    /// Per-project "Work: `<name>` ▸ Punctuation" — the smart-punctuation house style.
    WorkPunctuation,
    /// Application-level "Editor ▸ Punctuation" — the tier every project follows
    /// unless it takes an override of its own.
    Punctuation,
    /// General backup ("Copies de secours") policy (under Backup & Sync).
    Backup,
    /// Per-project backup override (under the open Work's section).
    WorkBackup,
    /// Per-project spell-check language(s) (under the open Work's section).
    WorkLanguage,
    /// Per-project personal dictionary (under the open Work's section).
    WorkDictionary,
    /// The app-wide master spell-check switch (under Spelling, above Dictionaries).
    Spellcheck,
    /// Per-project tag palette (under the open Work's section).
    WorkTags,
    /// Per-project author name (under the open Work's section).
    WorkAuthor,
    /// Per-project custom replacement lexicon (under the open Work's section).
    WorkTextReplacements,
    /// Distraction-free mode's own typography bundle (nested under Editor ▸
    /// Typography, alongside Scene/Synopsis/Notes/Corkboard).
    DistractionFree,
    /// The distraction-free **theme library** (built-ins + the writer's own).
    DistractionFreeThemes,
    /// Per-project note templates (under the open Work's section, beside Tags).
    WorkTemplates,
    /// A page an extension contributed, carrying its registered id.
    ///
    /// A variant rather than a separate parallel concept, so a contributed page
    /// *is* a page everywhere: the tree, the switcher, the search index and the
    /// selection signal all take it without a second code path. `&'static str`
    /// keeps `Pane` `Copy`, which the `HashMap<Pane, NodeId>` and `Signal<Pane>`
    /// below both rely on — and an extension's id is a literal anyway.
    Extension(&'static str),
}

impl Pane {
    /// Stable identity, not shown to the writer.
    ///
    /// Deliberately not derived from the discriminant: the point is that reordering or
    /// inserting a variant must not change what a page *is*. An extension page would
    /// namespace its own (`"ext.style"`).
    // A page's stable identity, and the vocabulary an extension-contributed page
    // carries (`Pane::Extension`). Only the uniqueness test reads it today — the
    // window addresses panes by value — but it is API, not scaffolding, so it is
    // annotated rather than gated behind `cfg(test)`.
    #[allow(dead_code)]
    pub(crate) fn id(self) -> &'static str {
        match self {
            // Prefixed, because a parent and one of its pages may well share a
            // word: Backup & Sync already holds a page called Backups, and
            // "backup" can only mean one of them.
            Pane::Section(s) => s.id(),
            Pane::Group(g) => g.id(),
            Pane::User => "user",
            Pane::Appearance => "appearance",
            Pane::MenusToolbars => "menus-toolbars",
            Pane::Notifications => "notifications",
            Pane::SceneTypography => "scene-typography",
            Pane::SynopsisTypography => "synopsis-typography",
            Pane::NotesTypography => "notes-typography",
            Pane::EditorBehavior => "editor-behavior",
            Pane::Goals => "goals",
            Pane::Games => "games",
            Pane::Corkboard => "corkboard",
            Pane::Dictionaries => "dictionaries",
            Pane::Autosave => "autosave",
            Pane::ExportFormats => "export-formats",
            Pane::Paratext => "paratext",
            Pane::Keymap => "keymap",
            Pane::WorkStructure => "work-structure",
            Pane::WorkPunctuation => "work-punctuation",
            Pane::Punctuation => "punctuation",
            Pane::Backup => "backup",
            Pane::WorkBackup => "work-backup",
            Pane::WorkLanguage => "work-language",
            Pane::WorkDictionary => "work-dictionary",
            Pane::Spellcheck => "spellcheck",
            Pane::WorkTags => "work-tags",
            Pane::WorkAuthor => "work-author",
            Pane::WorkTextReplacements => "work-text-replacements",
            Pane::DistractionFree => "distraction-free",
            Pane::DistractionFreeThemes => "distraction-free-themes",
            Pane::WorkTemplates => "work-templates",
            // Already namespaced by `settings_ext::register_page`, which refuses
            // an id without a dot.
            Pane::Extension(id) => id,
        }
    }

    pub(crate) fn label(self) -> LocalizedString {
        match self {
            // The Work section's row and page read "Work: `<title>`"; the title
            // is not the enum's to know, so the two places that show it compose
            // it themselves (`build_tree`'s row closure, `section_title`).
            Pane::Section(s) => s.label(),
            Pane::Group(g) => g.label(),
            Pane::User => tr!(settings_page_user()),
            Pane::Appearance => tr!(settings_page_appearance()),
            Pane::MenusToolbars => tr!(settings_page_menus()),
            Pane::Notifications => tr!(settings_page_notifications()),
            Pane::SceneTypography => tr!(settings_page_scene()),
            Pane::SynopsisTypography => tr!(settings_page_synopsis()),
            Pane::NotesTypography => tr!(settings_page_notes()),
            Pane::EditorBehavior => tr!(settings_page_editor_behavior()),
            Pane::Goals => tr!(settings_page_goals()),
            Pane::Games => tr!(settings_page_games()),
            Pane::Corkboard => tr!(settings_page_corkboard()),
            Pane::Dictionaries => tr!(settings_page_dictionaries()),
            Pane::Autosave => tr!(settings_page_autosave()),
            Pane::ExportFormats => tr!(settings_page_export()),
            Pane::Paratext => tr!(settings_page_paratext()),
            Pane::Keymap => tr!(settings_page_keymap()),
            Pane::WorkStructure => tr!(settings_page_structure()),
            Pane::WorkPunctuation => tr!(settings_page_punctuation()),
            Pane::Punctuation => tr!(settings_page_punctuation()),
            Pane::Backup => tr!(settings_page_backup()),
            Pane::WorkBackup => tr!(settings_page_work_backup()),
            Pane::WorkLanguage => tr!(settings_page_language()),
            Pane::WorkDictionary => tr!(settings_page_personal_dictionary()),
            Pane::WorkTags => tr!(settings_page_tags()),
            Pane::WorkTemplates => tr!(settings_page_templates()),
            // Resolved through the registry rather than stored, so a runtime
            // locale switch reaches a contributed page's label too. A page that
            // has since been unregistered falls back to its id — visible, and
            // better than an empty row.
            Pane::Extension(id) => crate::settings_ext::registered_pages()
                .iter()
                .find(|p| p.id == id)
                .map(|p| (p.label)())
                .unwrap_or_else(|| lit!(id.to_string())),
            Pane::WorkAuthor => tr!(settings_page_author()),
            Pane::WorkTextReplacements => tr!(settings_page_text_replacements()),
            Pane::Spellcheck => tr!(settings_page_spellcheck()),
            Pane::DistractionFree => tr!(settings_page_distraction_free()),
            Pane::DistractionFreeThemes => tr!(settings_page_distraction_free_themes()),
        }
    }

    /// The one line that says what is on this page — the gloss under its link on
    /// its parent's page, and (for a parent) the lead under its own title.
    ///
    /// `None` only for an extension's page: [`crate::settings_ext::SettingsPage`]
    /// carries a label and no description, and giving it one would break every
    /// downstream construction of that struct for a line the app cannot write
    /// itself. A link with nothing under it is the honest rendering.
    pub(crate) fn description(self) -> Option<LocalizedString> {
        Some(match self {
            Pane::Section(s) => s.description(),
            Pane::Group(g) => g.description(),
            Pane::User => tr!(settings_desc_user()),
            Pane::Appearance => tr!(settings_desc_appearance()),
            Pane::MenusToolbars => tr!(settings_desc_menus()),
            Pane::Notifications => tr!(settings_desc_notifications()),
            Pane::SceneTypography => tr!(settings_desc_scene()),
            Pane::SynopsisTypography => tr!(settings_desc_synopsis()),
            Pane::NotesTypography => tr!(settings_desc_notes()),
            Pane::EditorBehavior => tr!(settings_desc_editor_behavior()),
            Pane::Goals => tr!(settings_desc_goals()),
            Pane::Games => tr!(settings_desc_games()),
            Pane::Corkboard => tr!(settings_desc_corkboard()),
            Pane::Dictionaries => tr!(settings_desc_dictionaries()),
            Pane::Autosave => tr!(settings_desc_autosave()),
            Pane::ExportFormats => tr!(settings_desc_export()),
            Pane::Paratext => tr!(settings_desc_paratext()),
            Pane::Keymap => tr!(settings_desc_keymap()),
            Pane::WorkStructure => tr!(settings_desc_structure()),
            Pane::WorkPunctuation => tr!(settings_desc_work_punctuation()),
            Pane::Punctuation => tr!(settings_desc_punctuation()),
            Pane::Backup => tr!(settings_desc_backup()),
            Pane::WorkBackup => tr!(settings_desc_work_backup()),
            Pane::WorkLanguage => tr!(settings_desc_language()),
            Pane::WorkDictionary => tr!(settings_desc_personal_dictionary()),
            Pane::WorkTags => tr!(settings_desc_tags()),
            Pane::WorkTemplates => tr!(settings_desc_templates()),
            Pane::WorkAuthor => tr!(settings_desc_author()),
            Pane::WorkTextReplacements => tr!(settings_desc_text_replacements()),
            Pane::Spellcheck => tr!(settings_desc_spellcheck()),
            Pane::DistractionFree => tr!(settings_desc_distraction_free()),
            Pane::DistractionFreeThemes => tr!(settings_desc_distraction_free_themes()),
            Pane::Extension(_) => return None,
        })
    }

    /// Leading icon in the tree: every section, plus the two top-level leaves
    /// (design shows no icons on the indented sub-pages, groups included).
    pub(crate) fn icon(self) -> Option<IconWidget> {
        let svg = match self {
            Pane::Section(s) => s.icon_svg(),
            Pane::User => res!("assets/icons/settings/user.svg"),
            Pane::Keymap => res!("assets/icons/settings/keymap.svg"),
            _ => return None,
        };
        Some(IconWidget::from_svg_icon(svg).icon_size(16.0))
    }
}

/// A top-level category — a tree *branch*, and (since it has children to list) a
/// page of its own.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Sec {
    AppearanceBehaviour,
    Editor,
    Spelling,
    BackupSync,
    CompileExport,
    /// The open project. Its displayed label is "Work: `<title>`" (the title is
    /// filled in at tree-build time — the enum stays data-free / `Copy`).
    Work,
    /// Pages contributed through `settings_ext`. Present only when something is
    /// registered, and last, so an install with no extensions is unchanged.
    ///
    /// One fixed section rather than a parent each registration names: letting a
    /// registration address the app's own tree would make that tree's shape a
    /// compatibility promise, and rearranging Settings is ordinary work.
    Extensions,
}

impl Sec {
    /// Stable identity of the section's own page — `section-`-prefixed, see
    /// [`Pane::id`].
    // A page's stable identity, and the vocabulary an extension-contributed page
    // carries (`Pane::Extension`). Only the uniqueness test reads it today — the
    // window addresses panes by value — but it is API, not scaffolding, so it is
    // annotated rather than gated behind `cfg(test)`.
    #[allow(dead_code)]
    pub(crate) fn id(self) -> &'static str {
        match self {
            Sec::AppearanceBehaviour => "section-appearance-behaviour",
            Sec::Editor => "section-editor",
            Sec::Spelling => "section-spelling",
            Sec::BackupSync => "section-backup-sync",
            Sec::CompileExport => "section-compile-export",
            Sec::Work => "section-work",
            Sec::Extensions => "section-extensions",
        }
    }

    pub(crate) fn label(self) -> LocalizedString {
        match self {
            Sec::AppearanceBehaviour => tr!(settings_sec_appearance_behaviour()),
            Sec::Editor => tr!(settings_sec_editor()),
            Sec::Spelling => tr!(settings_sec_spelling()),
            Sec::BackupSync => tr!(settings_sec_backup()),
            Sec::CompileExport => tr!(settings_sec_compile()),
            Sec::Work => tr!(settings_sec_work()),
            Sec::Extensions => tr!(settings_sec_extensions()),
        }
    }

    /// The lead line under the section's title on its own page.
    pub(crate) fn description(self) -> LocalizedString {
        match self {
            Sec::AppearanceBehaviour => tr!(settings_desc_sec_appearance_behaviour()),
            Sec::Editor => tr!(settings_desc_sec_editor()),
            Sec::Spelling => tr!(settings_desc_sec_spelling()),
            Sec::BackupSync => tr!(settings_desc_sec_backup()),
            Sec::CompileExport => tr!(settings_desc_sec_compile()),
            Sec::Work => tr!(settings_desc_sec_work()),
            Sec::Extensions => tr!(settings_desc_sec_extensions()),
        }
    }

    pub(crate) fn icon_svg(self) -> &'static SvgIcon {
        match self {
            Sec::AppearanceBehaviour => res!("assets/icons/settings/appearance.svg"),
            Sec::Editor => res!("assets/icons/settings/editor.svg"),
            Sec::Spelling => res!("assets/icons/settings/spelling.svg"),
            Sec::BackupSync => res!("assets/icons/settings/backup.svg"),
            Sec::CompileExport => res!("assets/icons/settings/compile.svg"),
            Sec::Work => res!("assets/icons/binder/book.svg"),
            // No icon of its own to ship; the section reads by its label like the
            // nested groups do.
            Sec::Extensions => res!("assets/icons/settings/appearance.svg"),
        }
    }
}

/// A nested grouping node *inside* a section — one level deeper than [`Sec`],
/// for a cluster of pages that would otherwise crowd their section's flat
/// list. Currently only Editor ▸ Typography: Scene / Synopsis / Notes /
/// Corkboard / Distraction-free (the shared Typeface/Size/Line height/
/// First-line indent shape) plus the Distraction-free theme library. It has a
/// page of its own, like a section, and no icon (design shows icons only on
/// top-level sections plus the Keymap leaf — a nested group is indented like any
/// other sub-page).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum GroupKind {
    Typography,
}

impl GroupKind {
    /// Stable identity of the group's own page — `group-`-prefixed, see
    /// [`Pane::id`].
    // A page's stable identity, and the vocabulary an extension-contributed page
    // carries (`Pane::Extension`). Only the uniqueness test reads it today — the
    // window addresses panes by value — but it is API, not scaffolding, so it is
    // annotated rather than gated behind `cfg(test)`.
    #[allow(dead_code)]
    pub(crate) fn id(self) -> &'static str {
        match self {
            GroupKind::Typography => "group-typography",
        }
    }

    pub(crate) fn label(self) -> LocalizedString {
        match self {
            GroupKind::Typography => tr!(settings_group_typography()),
        }
    }

    /// The lead line under the group's title on its own page.
    pub(crate) fn description(self) -> LocalizedString {
        match self {
            GroupKind::Typography => tr!(settings_desc_group_typography()),
        }
    }
}

// ── The category tree, as data ───────────────────────────────────────────────

/// One entry under a section: a page, or a nested group with pages of its own.
#[derive(Clone)]
pub(crate) enum Branch {
    Page(Pane),
    Group(GroupKind, Vec<Pane>),
}

/// One top-level row of the tree: a section with its branches, or a page that is
/// a root in its own right (Keymap).
#[derive(Clone)]
pub(crate) enum Root {
    Section(Sec, Vec<Branch>),
    Page(Pane),
}

/// **The** shape of the category tree — order, nesting and membership, in one place.
///
/// Three readers derive everything they need from this: the `TreeView`'s model
/// ([`super::tree::build_tree`]), the "reveal the page we opened at" expansion
/// ([`ancestors_of`]), and each parent's own page, which lists what is under it
/// ([`children_of`]). They used to hold that knowledge separately — a `section_of`
/// match written out by hand beside the insert calls — which is how a page ends up
/// in the tree but missing from its parent's list, or reachable but never revealed.
///
/// `has_work` adds the open project's section; `extension_pages` are the ids
/// `settings_ext` has registered, in registration order (empty ⇒ no Extensions
/// section at all, so an install with no extensions gets exactly the tree it had).
pub(crate) fn tree_spec(has_work: bool, extension_pages: &[&'static str]) -> Vec<Root> {
    let mut roots = vec![
        // First, and a root in its own right. It is the one thing a new install
        // wants filled in once and then never again, and every section below it
        // is about the app's behaviour rather than about the person using it.
        Root::Page(Pane::User),
        Root::Section(
            Sec::AppearanceBehaviour,
            vec![
                Branch::Page(Pane::Appearance),
                Branch::Page(Pane::MenusToolbars),
                Branch::Page(Pane::Notifications),
            ],
        ),
        Root::Section(
            Sec::Editor,
            vec![
                // The five typography-shaped pages (Scene / Synopsis / Notes /
                // Corkboard / Distraction-free — all four fields plus
                // font/size/line-height/indent), and the Distraction-free theme
                // library, live under their own nested group rather than as six
                // more flat siblings in an already-crowded Editor list.
                Branch::Group(
                    GroupKind::Typography,
                    vec![
                        Pane::SceneTypography,
                        Pane::SynopsisTypography,
                        Pane::NotesTypography,
                        Pane::Corkboard,
                        Pane::DistractionFree,
                        Pane::DistractionFreeThemes,
                    ],
                ),
                Branch::Page(Pane::EditorBehavior),
                // Beside Editor Behavior: the other set of switches that change
                // what happens as the writer types, rather than how the page looks.
                Branch::Page(Pane::Punctuation),
                Branch::Page(Pane::Goals),
                // Beside Goals: both are about what the writer is asking of
                // themselves while drafting, rather than about how the page looks.
                Branch::Page(Pane::Games),
            ],
        ),
        Root::Section(
            Sec::Spelling,
            vec![
                Branch::Page(Pane::Spellcheck),
                Branch::Page(Pane::Dictionaries),
            ],
        ),
        Root::Section(
            Sec::BackupSync,
            vec![Branch::Page(Pane::Autosave), Branch::Page(Pane::Backup)],
        ),
        Root::Section(
            Sec::CompileExport,
            vec![
                Branch::Page(Pane::ExportFormats),
                Branch::Page(Pane::Paratext),
            ],
        ),
        Root::Page(Pane::Keymap),
    ];

    // The open project's own section (multi-project-ready): shown only when a Work
    // is open, labelled "Work: `<title>`" by whoever renders it.
    if has_work {
        roots.push(Root::Section(
            Sec::Work,
            vec![
                // Author leads the section: it is the one field about the *book*
                // rather than about how the app handles it.
                Branch::Page(Pane::WorkAuthor),
                Branch::Page(Pane::WorkStructure),
                Branch::Page(Pane::WorkLanguage),
                Branch::Page(Pane::WorkBackup),
                Branch::Page(Pane::WorkDictionary),
                Branch::Page(Pane::WorkTags),
                // Beside the tag palette: the other per-project catalogue the
                // writer curates and that travels inside the `.skrib`.
                Branch::Page(Pane::WorkTemplates),
                // Beside the personal dictionary and the tag palette: the third
                // per-project vocabulary the writer curates.
                Branch::Page(Pane::WorkTextReplacements),
                // Next to the lexicon: the other thing that rewrites prose as it
                // is typed, and the other one that travels inside the `.skrib`.
                Branch::Page(Pane::WorkPunctuation),
            ],
        ));
    }

    // Anything an extension registered, last and only when there is something.
    if !extension_pages.is_empty() {
        roots.push(Root::Section(
            Sec::Extensions,
            extension_pages
                .iter()
                .map(|id| Branch::Page(Pane::Extension(id)))
                .collect(),
        ));
    }

    roots
}

/// Every page linked from `parent`'s own page, in tree order.
///
/// A section lists its pages, a nested group counted as **one** entry (linking to
/// the group's own page rather than flattening six pages into its parent's list);
/// a group lists its pages. Anything that is not a parent has no children.
pub(crate) fn children_of(spec: &[Root], parent: Pane) -> Vec<Pane> {
    for root in spec {
        let Root::Section(sec, branches) = root else {
            continue;
        };
        if parent == Pane::Section(*sec) {
            return branches
                .iter()
                .map(|b| match b {
                    Branch::Page(p) => *p,
                    Branch::Group(g, _) => Pane::Group(*g),
                })
                .collect();
        }
        for branch in branches {
            if let Branch::Group(g, pages) = branch
                && parent == Pane::Group(*g)
            {
                return pages.clone();
            }
        }
    }
    Vec::new()
}

/// The rows that must be expanded for `pane`'s own row to be visible — its
/// section, then its group if it sits in one. Empty for a root.
pub(crate) fn ancestors_of(spec: &[Root], pane: Pane) -> Vec<Pane> {
    for root in spec {
        let Root::Section(sec, branches) = root else {
            continue;
        };
        for branch in branches {
            match branch {
                Branch::Page(p) if *p == pane => return vec![Pane::Section(*sec)],
                Branch::Group(g, pages) => {
                    if pane == Pane::Group(*g) {
                        return vec![Pane::Section(*sec)];
                    }
                    if pages.contains(&pane) {
                        return vec![Pane::Section(*sec), Pane::Group(*g)];
                    }
                }
                Branch::Page(_) => {}
            }
        }
    }
    Vec::new()
}

/// Every pane the tree places, parents included — the `Switcher` must carry a
/// body for each, and nothing may appear twice.
// Only the uniqueness and coverage tests enumerate the tree; the window builds
// it from tree_spec directly.
#[cfg(test)]
pub(crate) fn all_panes(spec: &[Root]) -> Vec<Pane> {
    let mut out = Vec::new();
    for root in spec {
        match root {
            Root::Page(p) => out.push(*p),
            Root::Section(sec, branches) => {
                out.push(Pane::Section(*sec));
                for branch in branches {
                    match branch {
                        Branch::Page(p) => out.push(*p),
                        Branch::Group(g, pages) => {
                            out.push(Pane::Group(*g));
                            out.extend(pages.iter().copied());
                        }
                    }
                }
            }
        }
    }
    out
}

/// Jumping to a page from somewhere other than its own tree row: a search
/// suggestion, or a link on a parent's page.
///
/// Both have to do the same two things — move the tree's highlight and switch the
/// right-hand pane — and a per-project page resolves to no tree node at all when
/// nothing is open, in which case the pane still switches, landing on that page's
/// "no project" placeholder. Correct either way.
#[derive(Clone)]
pub(crate) struct Navigator {
    pub(crate) selection: KeyedSelectionModel<NodeId>,
    pub(crate) nodes: Rc<HashMap<Pane, NodeId>>,
    pub(crate) selected_pane: Signal<Pane>,
}

impl Navigator {
    pub(crate) fn go(&self, pane: Pane) {
        if let Some(id) = self.nodes.get(&pane) {
            self.selection.select(*id);
        }
        self.selected_pane.set(pane);
    }
}
