// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Settings modal — Skribisto's full preferences window.
//!
//! Presented as an in-tree modal (see the `app.settings` action in `app.rs`). A
//! two-pane layout mirroring the design (IntelliJ Int UI vocabulary): a left
//! **category [`TreeView`]** (with a [`SearchField`] on top) and a right
//! **settings pane** that switches with the tree selection, breadcrumb header +
//! action footer. The header/footer chrome matches the Welcome and New Work
//! panels.
//!
//! All state lives on [`SettingsViewModel`] (persisted signals) plus the
//! framework's drop-in [`ThemeSwitcher`] / [`LanguageSwitcher`] / [`TextScaleControl`]
//! (theme / interface language / text scale — each applies live and persists).
//! The panes bind those signals directly; this view is thin.
//!
//! Categories that don't yet carry settings (today only Menus & Toolbars) render
//! an **empty placeholder**. Keymap embeds Teksilo's `ShortcutSettings`;
//! Notifications embeds the toast archive `NotificationLog`.
//!
//! A **parent** — a section, or the nested Typography group — is a page in its own
//! right: its title, what it is for, and a link to each thing under it
//! (`panes::overview`). Selecting one used to switch nothing at all, so the right
//! pane went on showing whichever leaf was open last and the window disagreed with
//! its own tree.
//! **Instant-apply** (the macOS / GNOME convention): every change takes effect and
//! persists immediately, so there is no Apply/Cancel/OK staged model. The footer
//! carries only *Reset to defaults* (left — enabled only while something differs
//! from the factory defaults, and guarded by a confirmation since there is no undo)
//! and *Done* (right — closes the window). The header ✕ closes it too.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::canvas::svg::SvgIcon;
use teksilo::core::styles::{PanelVariant, Theme};
use teksilo::data::{KeyedSelectionModel, NodeId, SelectionMode, TreeModel};
use teksilo::i18n::{LocalizedString, current_locale, localized};
use teksilo::prelude::*;

pub(crate) mod panes;
use teksilo::res;
use teksilo::settings::{SettingsExt, TEXT_SCALE_KEY};
use teksilo::widgets::{
    Breadcrumb, BreadcrumbItem, Button, ButtonVariant, Center, Divider, Expand, FixedSize,
    FontPicker, FormLayout, GroupHeader, HStack, IconButton, IconWidget, LanguageSwitcher,
    MessageBox, MessageBoxButton, MessageBoxButtons, Padding, Panel, RadioButton, RadioGroup,
    ScrollArea, SearchField, Slider, Spacer, StandardButton, StandardTreeItem, Switcher,
    TextScaleControl, TextWidget, ThemeSwitcher, Toggle, TreeView, VStack,
};

use crate::sessions::WorkSession;
use crate::view_models::{
    BackupSettingsViewModel, EditorTypography, HighlightScope, SettingsViewModel, TypewriterAnchor,
    WorkSettingsViewModel,
};
use crate::{
    DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT, DISTRACTION_FREE_FONT_FAMILY_DEFAULT,
    DISTRACTION_FREE_GO_DEFAULT, DISTRACTION_FREE_GO_TO_DEFAULT,
    DISTRACTION_FREE_LINE_HEIGHT_DEFAULT, DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT,
    DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT, DISTRACTION_FREE_SESSION_DEFAULT,
    DISTRACTION_FREE_SIZE_DEFAULT, DISTRACTION_FREE_TITLE_DEFAULT, DISTRACTION_FREE_WIDTH_DEFAULT,
    DISTRACTION_FREE_WORD_COUNT_DEFAULT, EDITOR_WIDTH_DEFAULT, GOALS_SHOW_CHARACTERS_DEFAULT,
    NOTES_FIRST_LINE_INDENT_DEFAULT, NOTES_FONT_FAMILY_DEFAULT, NOTES_LINE_HEIGHT_DEFAULT,
    NOTES_PARA_SPACING_AFTER_DEFAULT, NOTES_PARA_SPACING_BEFORE_DEFAULT, NOTES_SIZE_DEFAULT,
    SCENE_FIRST_LINE_INDENT_DEFAULT, SCENE_FONT_FAMILY_DEFAULT, SCENE_LINE_HEIGHT_DEFAULT,
    SCENE_PARA_SPACING_AFTER_DEFAULT, SCENE_PARA_SPACING_BEFORE_DEFAULT, SCENE_SIZE_DEFAULT,
    SYNOPSIS_FIRST_LINE_INDENT_DEFAULT, SYNOPSIS_FONT_FAMILY_DEFAULT, SYNOPSIS_LINE_HEIGHT_DEFAULT,
    SYNOPSIS_PANE_DEFAULT, SYNOPSIS_PARA_SPACING_AFTER_DEFAULT,
    SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT, SYNOPSIS_SIZE_DEFAULT, TYPEWRITER_DEFAULT,
};
use skribisto_model::ChapterMode;
use skribisto_model::counting::CountingMethodSetting;

/// Card dimensions (a compact two-pane preferences window).
const CARD_W: f32 = 920.0;
const CARD_H: f32 = 620.0;
const HEADER_H: f32 = 44.0;
const FOOTER_H: f32 = 56.0;
const TREE_W: f32 = 262.0;
/// Body height = card − header − the 1 px rule beneath it.
const BODY_H: f32 = CARD_H - HEADER_H - 1.0;
/// The default text-scale factor (framework `TEXT_SCALE_KEY` baseline).
const TEXT_SCALE_DEFAULT: f32 = 1.0;

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
enum Pane {
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
    fn id(self) -> &'static str {
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

    fn label(self) -> LocalizedString {
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
    fn description(self) -> Option<LocalizedString> {
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
    fn icon(self) -> Option<IconWidget> {
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
enum Sec {
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
    fn id(self) -> &'static str {
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

    fn label(self) -> LocalizedString {
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
    fn description(self) -> LocalizedString {
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

    fn icon_svg(self) -> &'static SvgIcon {
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
enum GroupKind {
    Typography,
}

impl GroupKind {
    /// Stable identity of the group's own page — `group-`-prefixed, see
    /// [`Pane::id`].
    fn id(self) -> &'static str {
        match self {
            GroupKind::Typography => "group-typography",
        }
    }

    fn label(self) -> LocalizedString {
        match self {
            GroupKind::Typography => tr!(settings_group_typography()),
        }
    }

    /// The lead line under the group's title on its own page.
    fn description(self) -> LocalizedString {
        match self {
            GroupKind::Typography => tr!(settings_desc_group_typography()),
        }
    }
}

// ── The category tree, as data ───────────────────────────────────────────────

/// One entry under a section: a page, or a nested group with pages of its own.
#[derive(Clone)]
enum Branch {
    Page(Pane),
    Group(GroupKind, Vec<Pane>),
}

/// One top-level row of the tree: a section with its branches, or a page that is
/// a root in its own right (Keymap).
#[derive(Clone)]
enum Root {
    Section(Sec, Vec<Branch>),
    Page(Pane),
}

/// **The** shape of the category tree — order, nesting and membership, in one place.
///
/// Three readers derive everything they need from this: the `TreeView`'s model
/// ([`SettingsPanel::build_tree`]), the "reveal the page we opened at" expansion
/// ([`ancestors_of`]), and each parent's own page, which lists what is under it
/// ([`children_of`]). They used to hold that knowledge separately — a `section_of`
/// match written out by hand beside the insert calls — which is how a page ends up
/// in the tree but missing from its parent's list, or reachable but never revealed.
///
/// `has_work` adds the open project's section; `extension_pages` are the ids
/// `settings_ext` has registered, in registration order (empty ⇒ no Extensions
/// section at all, so an install with no extensions gets exactly the tree it had).
fn tree_spec(has_work: bool, extension_pages: &[&'static str]) -> Vec<Root> {
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
fn children_of(spec: &[Root], parent: Pane) -> Vec<Pane> {
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
fn ancestors_of(spec: &[Root], pane: Pane) -> Vec<Pane> {
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
fn all_panes(spec: &[Root]) -> Vec<Pane> {
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
struct Navigator {
    selection: KeyedSelectionModel<NodeId>,
    nodes: Rc<HashMap<Pane, NodeId>>,
    selected_pane: Signal<Pane>,
}

impl Navigator {
    fn go(&self, pane: Pane) {
        if let Some(id) = self.nodes.get(&pane) {
            self.selection.select(*id);
        }
        self.selected_pane.set(pane);
    }
}

// ── Reset-to-defaults enable state ───────────────────────────────────────────

/// `true` if any signal in `sigs` is currently `true` — a reactive OR-fold.
fn any_true(sigs: Vec<Signal<bool>>) -> Signal<bool> {
    let mut it = sigs.into_iter();
    match it.next() {
        None => Signal::new(false),
        Some(first) => it.fold(first, |acc, s| acc.or(&s)),
    }
}

/// Reactive "the live settings differ from the factory defaults" — drives the
/// Reset-to-defaults button's enabled state.
fn build_not_defaults(
    theme: &Signal<Theme>,
    locale: &Option<Signal<teksilo::i18n::LanguageIdentifier>>,
    scale: &Signal<f32>,
    vm: &SettingsViewModel,
) -> Signal<bool> {
    let typo = vm.editor_typography();
    let mut diffs = vec![
        theme.map(|t| t.is_dark()), // default = light
        scale.map(|s| (*s - TEXT_SCALE_DEFAULT).abs() > f32::EPSILON),
        vm.column_width()
            .map(|w| (*w - EDITOR_WIDTH_DEFAULT).abs() > 0.01),
        vm.autosave().map(|a| *a),      // default = off
        vm.show_welcome().map(|s| !*s), // default = on
        // ── Scene typography ──
        typo.scene
            .font_family
            .map(|f| f.as_str() != SCENE_FONT_FAMILY_DEFAULT),
        typo.scene
            .size
            .map(|s| (*s - SCENE_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.scene
            .line_height
            .map(|h| (*h - SCENE_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.scene
            .first_line_indent
            .map(|i| (*i - SCENE_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.scene
            .para_spacing_before
            .map(|v| (*v - SCENE_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.scene
            .para_spacing_after
            .map(|v| (*v - SCENE_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        // ── Synopsis typography ──
        typo.synopsis
            .font_family
            .map(|f| f.as_str() != SYNOPSIS_FONT_FAMILY_DEFAULT),
        typo.synopsis
            .size
            .map(|s| (*s - SYNOPSIS_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.synopsis
            .line_height
            .map(|h| (*h - SYNOPSIS_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.synopsis
            .first_line_indent
            .map(|i| (*i - SYNOPSIS_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.synopsis
            .para_spacing_before
            .map(|v| (*v - SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.synopsis
            .para_spacing_after
            .map(|v| (*v - SYNOPSIS_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        // ── Notes typography ──
        typo.notes
            .font_family
            .map(|f| f.as_str() != NOTES_FONT_FAMILY_DEFAULT),
        typo.notes
            .size
            .map(|s| (*s - NOTES_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.notes
            .line_height
            .map(|h| (*h - NOTES_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.notes
            .first_line_indent
            .map(|i| (*i - NOTES_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.notes
            .para_spacing_before
            .map(|v| (*v - NOTES_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.notes
            .para_spacing_after
            .map(|v| (*v - NOTES_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        // ── Distraction-free typography ──
        typo.distraction_free
            .font_family
            .map(|f| f.as_str() != DISTRACTION_FREE_FONT_FAMILY_DEFAULT),
        typo.distraction_free
            .size
            .map(|s| (*s - DISTRACTION_FREE_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.distraction_free
            .line_height
            .map(|h| (*h - DISTRACTION_FREE_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.distraction_free
            .first_line_indent
            .map(|i| (*i - DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.distraction_free
            .para_spacing_before
            .map(|v| (*v - DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.distraction_free
            .para_spacing_after
            .map(|v| (*v - DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        vm.distraction_free_width()
            .map(|w| (*w - DISTRACTION_FREE_WIDTH_DEFAULT).abs() > 0.01),
        vm.distraction_free_title()
            .map(|s| *s != DISTRACTION_FREE_TITLE_DEFAULT),
        vm.distraction_free_word_count()
            .map(|s| *s != DISTRACTION_FREE_WORD_COUNT_DEFAULT),
        vm.distraction_free_session()
            .map(|s| *s != DISTRACTION_FREE_SESSION_DEFAULT),
        vm.distraction_free_go()
            .map(|s| *s != DISTRACTION_FREE_GO_DEFAULT),
        vm.distraction_free_go_to()
            .map(|s| *s != DISTRACTION_FREE_GO_TO_DEFAULT),
        // ── Editor behaviour ──
        vm.synopsis_pane().map(|s| *s != SYNOPSIS_PANE_DEFAULT),
        vm.typewriter().map(|s| *s != TYPEWRITER_DEFAULT),
        vm.typewriter_anchor()
            .map(|a| *a != Some(TypewriterAnchor::default())),
        vm.highlight_scope()
            .map(|s| *s != HighlightScope::default()),
        // ── Goals & word count ──
        vm.counting_method()
            .map(|m| *m != CountingMethodSetting::default()),
        vm.show_characters()
            .map(|s| *s != GOALS_SHOW_CHARACTERS_DEFAULT),
    ];
    if let Some(loc) = locale {
        // Compare against a once-parsed default rather than allocating a String
        // per change (clippy::cmp_owned).
        let default_locale: teksilo::i18n::LanguageIdentifier =
            "en-US".parse().expect("valid default locale");
        diffs.push(loc.map(move |l| *l != default_locale));
    }
    any_true(diffs)
}

// ── Small view helpers ───────────────────────────────────────────────────────

/// A left-column field label (dimmed, small) — matches the design's `--tx2`.
/// `pub(crate)` so `panes::backup` shares the exact same row-label style as
/// every built-in pane.
pub(crate) fn field_label(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// A dimmed sub-field hint line. `pub(crate)` so `panes::export_styles` says which formats
/// read the round-trip switches in the same voice every other hint in the window uses.
pub(crate) fn hint(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// A lowercase-titled section header + trailing rule (design's group headers).
/// `pub(crate)` so the backup panes reuse the identical group-header treatment.
pub(crate) fn group(text: LocalizedString) -> GroupHeader {
    GroupHeader::new(text)
        .style(TextStyleRole::SmallBold)
        .color(TextRole::Secondary)
}

/// A slider with a live value readout on its right, in a fixed 300 px cell.
pub(crate) fn slider_field(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
) -> impl Widget + 'static {
    slider_field_inner(value, min, max, step, fmt, None)
}

/// Like [`slider_field`], with a plain tooltip on the slider (for short
/// explanations that used to be body-copy hints under the control).
fn slider_field_tipped(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
    tip: LocalizedString,
) -> impl Widget + 'static {
    slider_field_inner(value, min, max, step, fmt, Some(tip))
}

fn slider_field_inner(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
    tip: Option<LocalizedString>,
) -> impl Widget + 'static {
    let seed = fmt(value.get());
    let text = value.map(move |v| fmt(*v));
    let readout = TextWidget::new(LocalizedString::literal(seed))
        .text(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary);
    let mut slider = Slider::new(value, min, max).step(step);
    if let Some(tip) = tip {
        slider = slider.tooltip(tip);
    }
    FixedSize::new().width(300.0).child(
        HStack::new()
            .spacing(12.0)
            .child(Expand::horizontal().child(slider))
            .child(FixedSize::new().width(52.0).child(readout)),
    )
}

/// The breadcrumb trail shown at the top of a pane (`Parent › Current`).
fn crumb(parent: Option<LocalizedString>, current: LocalizedString) -> Breadcrumb {
    let mut b = Breadcrumb::new();
    if let Some(p) = parent {
        b = b.item(BreadcrumbItem::new(p));
    }
    b.item(BreadcrumbItem::current(current))
}

/// The right-pane frame: breadcrumb header · rule · scrollable content.
fn pane_frame(breadcrumb: impl Widget + 'static, content: impl Widget + 'static) -> impl Widget {
    VStack::new()
        .spacing(0.0)
        .child(
            FixedSize::new()
                .height(46.0)
                .child(Padding::symmetric(13.0, 22.0).child(breadcrumb)),
        )
        .child(Expand::horizontal().child(Divider::new()))
        .child(
            Expand::vertical()
                .child(ScrollArea::new().child(Padding::symmetric(20.0, 24.0).child(content))),
        )
}

/// The centered placeholder body for a not-yet-implemented category.
fn empty_content(icon: &'static SvgIcon) -> impl Widget {
    Center::new().child(
        VStack::new()
            .spacing(10.0)
            .child(
                IconWidget::from_svg_icon(icon)
                    .icon_size(40.0)
                    .color(TextRole::Disabled),
            )
            .child(
                TextWidget::new(tr!(settings_empty_title()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Secondary),
            )
            .child(hint(tr!(settings_empty_hint()))),
    )
}

/// A section's displayed title — static, except the open project's, which reads
/// "Work: `<title>`".
///
/// Composed per resolve rather than once, so a runtime locale switch reaches the
/// "Work" half of it (the project's own title is data and stays as typed).
fn section_title(sec: Sec, work_title: &str) -> LocalizedString {
    match sec {
        Sec::Work => {
            let title = work_title.to_string();
            localized(move || format!("{}: {}", tr!(settings_sec_work()).resolve_now(), title))
        }
        other => other.label(),
    }
}

/// A full empty pane (breadcrumb + placeholder).
fn empty_pane(
    parent: Option<LocalizedString>,
    current: LocalizedString,
    icon: &'static SvgIcon,
) -> impl Widget {
    pane_frame(crumb(parent, current), empty_content(icon))
}

/// `CountingMethodSetting` ⟷ the `RadioGroup`'s `usize` selection. The order here
/// is the radio order in `goals_pane`; keep the two in step.
fn method_to_index(m: CountingMethodSetting) -> usize {
    match m {
        CountingMethodSetting::Auto => 0,
        CountingMethodSetting::Whitespace => 1,
        CountingMethodSetting::UnicodeWords => 2,
        CountingMethodSetting::CjkHybrid => 3,
    }
}

fn index_to_method(i: usize) -> CountingMethodSetting {
    match i {
        1 => CountingMethodSetting::Whitespace,
        2 => CountingMethodSetting::UnicodeWords,
        3 => CountingMethodSetting::CjkHybrid,
        _ => CountingMethodSetting::Auto,
    }
}

pub struct SettingsPanel {
    /// The active page — a panel field so it survives rebuilds (and seeds the
    /// tree selection + the content `Switcher`). Defaults to Editor ▸ Scene.
    selected_pane: Signal<Pane>,
    root_child: Option<WidgetId>,
    /// The Tier-2 bundle for the Work the OPENING window shows.
    ///
    /// Never resolve the equivalent state via `ctx.app_state::<SingleWork/
    /// SingleWorkInfo/AppIds/TagsViewModel/UserDictionaryViewModel>()` — that
    /// slot is one per process, seeded from the first window's session, so a
    /// second open Work would silently read/write the wrong project's
    /// settings. Same pattern as `SaveAsViewModel`/`BackupRestoreViewModel`.
    session: WorkSession,
}

impl SettingsPanel {
    pub fn new(session: WorkSession) -> Self {
        Self::opening_at(Pane::SceneTypography, session)
    }

    /// Open straight to Spelling ▸ Dictionaries — the target of the "install the
    /// missing dictionaries" toast (`offer_missing_dictionaries`). The tree seeds
    /// its selection to that page and expands the owning section on open.
    pub fn open_to_dictionaries(session: WorkSession) -> Self {
        Self::opening_at(Pane::Dictionaries, session)
    }

    /// Open straight to Editor ▸ Writing games — the target of the games dock's
    /// own "Writing game settings…" button, which promises that page by name.
    pub fn open_to_games(session: WorkSession) -> Self {
        Self::opening_at(Pane::Games, session)
    }

    /// Open straight to Backup & Sync ▸ Backup — the target of the
    /// "no backups configured" nudge toast.
    pub fn open_to_backup(session: WorkSession) -> Self {
        Self::opening_at(Pane::Backup, session)
    }

    /// Open straight to Settings ▸ User — the target of the "your comments are
    /// unsigned" toast (`crate::app::warn_unsigned_comments`), which promises
    /// that page by name.
    pub fn open_to_user(session: WorkSession) -> Self {
        Self::opening_at(Pane::User, session)
    }

    fn opening_at(pane: Pane, session: WorkSession) -> Self {
        Self {
            selected_pane: Signal::new(pane),
            root_child: None,
            session,
        }
    }

    // The per-page bodies live in `settings_panel::panes` — one module per page.
    // This impl keeps only the shell: the category tree, the search field and the footer.
    /// The category tree (left rail). Walks `spec` into a `TreeModel`, seeds
    /// selection to the active page, and wires selection → `selected_pane`.
    /// Returns the `TreeView`, its selection model, and the page → node map every
    /// other way of reaching a page goes through ([`Navigator`]).
    ///
    /// `work_title` fills the Work section's row label; it is ignored when the
    /// spec carries no Work section.
    fn build_tree(
        &self,
        ctx: &mut BuildContext,
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
        if let Some(id) = nodes.get(&self.selected_pane.get()) {
            selection.select(*id);
        }

        // Selection → active page. Every row is a page now, parents included:
        // selecting a section used to switch nothing, so the right-hand pane went
        // on showing whichever leaf was open last.
        {
            let selected_pane = self.selected_pane.clone();
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
        let tree =
            TreeView::new_with_context(model, move |pane: &Pane, entry, selected, rowctx| {
                // The Work section's label is dynamic ("Work: `<title>`"); every
                // other row uses its static label.
                let label = match pane {
                    Pane::Section(Sec::Work) => section_title(Sec::Work, &work_title),
                    other => other.label(),
                };
                let mut row = StandardTreeItem::new(label)
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
        for ancestor in ancestors_of(spec, self.selected_pane.get()) {
            expand(ancestor);
        }

        (tree, selection, nodes)
    }

    /// The "Search settings" field, offering suggestions across every page and
    /// setting; selecting a suggestion jumps to (and highlights) its page.
    ///
    /// `spec` contributes the parents: a section and the Typography group are
    /// pages like any other now, and a search that could not reach them would be
    /// the one place in the window still treating them as mere branches.
    fn search_field(&self, spec: &[Root], work_title: &str, nav: Navigator) -> impl Widget {
        // (searchable label, the page it lives on). Resolved per-keystroke so it
        // follows a locale change.
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
                    if let Branch::Group(group, _) = branch {
                        idx.push((group.label(), Pane::Group(*group)));
                    }
                }
            }
        }
        let index: Rc<Vec<(LocalizedString, Pane)>> = Rc::new(idx);

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
}

impl std::fmt::Debug for SettingsPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsPanel").finish()
    }
}

impl Widget for SettingsPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = SettingsViewModel::new(ctx.settings());
        // Which surfaces a game covers is an app setting; whether it is being
        // played is this project's session state. Paired here, exactly as
        // `App::build` pairs them for the editors.
        let games = crate::view_models::WritingGamesViewModel::new(
            self.session.always_forward.clone(),
            crate::view_models::WritingGameOptions::new(
                vm.games_forward_prose(),
                vm.games_forward_synopsis(),
            ),
        );
        let scale = ctx.settings().signal_for(&TEXT_SCALE_KEY);
        let theme_sig = ctx.theme_signal().clone();
        let locale_sig = current_locale();

        // Instant-apply: settings take effect + persist the moment they change, so
        // there is no Apply/Cancel/OK. The footer's Reset-to-defaults is enabled
        // only while something differs from the factory defaults.
        let not_defaults = build_not_defaults(&theme_sig, &locale_sig, &scale, &vm);

        // The OPENING WINDOW's own Work (never `ctx.app_state`, see this struct's
        // `session` field doc) backs the "Work: `<name>` ▸ Structure" page. When
        // no project is open yet, the page is present in the Switcher but its
        // tree node isn't shown, so it renders an empty placeholder — same
        // `w.id().is_some()` gate as before, just sourced from this window's own
        // session instead of a process-wide `app_state` slot.
        let work = Some(self.session.single_work.clone());
        let stack = self.session.ids.stack_id.clone();
        let work_title = work.as_ref().map(|w| w.title().get()).unwrap_or_default();

        // The shape of the whole window, resolved once: the tree walks it, the
        // parents' pages read their children off it, and the search index takes
        // its parents from it. Both variables it depends on are snapshots taken
        // as this window is built — a Work opened later gets its section when the
        // Settings window is next opened, and the extension registry is a
        // snapshot by design (see `settings_ext`).
        let extension_ids: Vec<&'static str> = crate::settings_ext::registered_pages()
            .iter()
            .map(|p| p.id)
            .collect();
        let spec = tree_spec(self.session.single_work.id().is_some(), &extension_ids);
        // The two Work pages edit the *entity*, not the settings store, so they go through
        // their own view-model rather than calling `SingleWork::set_*` + `save` from a pane.
        // THIS WINDOW's own session (never `ctx.app_state`), same reasoning as `work` above.
        let punctuation = Some(self.session.smart_punctuation.clone());
        let work_vm = work
            .as_ref()
            .zip(punctuation.as_ref())
            .map(|(w, p)| WorkSettingsViewModel::new(w.clone(), p.clone(), stack.clone()));
        let structure_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_structure::work_structure_pane(
                ctx,
                vm,
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_structure()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let punctuation_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_punctuation::work_punctuation_pane(
                ctx,
                vm,
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_punctuation()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let language_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_language::work_language_pane(
                ctx,
                vm,
                work_title.clone(),
                self.session.open_docs.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_language()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let author_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_author::work_author_pane(
                ctx,
                vm,
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_author()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        // Spelling ▸ Dictionaries — the management pane (Installed / Get more), wrapped in the
        // shared `pane_frame` like every other pane. Always available (dictionaries are a
        // machine-wide resource, independent of any open project).
        let dictionaries_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::DictionariesViewModel>()
            .cloned()
        {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_spelling())),
                    tr!(settings_page_dictionaries()),
                ),
                crate::settings::panes::dictionaries::dictionaries_pane(ctx, &vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_spelling())),
                tr!(settings_page_dictionaries()),
                Sec::Spelling.icon_svg(),
            )),
        };
        // Compile & Export ▸ Export Formats — the export-style manager (built-in + user styles,
        // duplicate-to-edit, JSON import/export), wrapped in `pane_frame` like every other pane.
        let export_styles_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::ExportStylesViewModel>()
            .cloned()
        {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_compile())),
                    tr!(settings_page_export()),
                ),
                crate::settings::panes::export_styles::export_styles_pane(ctx, &vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_compile())),
                tr!(settings_page_export()),
                Sec::CompileExport.icon_svg(),
            )),
        };

        // Compile & Export ▸ Paratext structures — the front/back-matter catalogue New
        // Work starts a project from. App-level like export styles, and resolved the same
        // way, so one instance backs both this pane and the New Work picker.
        let paratext_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::ParatextPresetsViewModel>()
            .cloned()
        {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_compile())),
                    tr!(settings_page_paratext()),
                ),
                crate::settings::panes::paratext::paratext_pane(ctx, &vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_compile())),
                tr!(settings_page_paratext()),
                Sec::CompileExport.icon_svg(),
            )),
        };

        // Editor ▸ Distraction-free themes — the theme library, same shape and
        // same app_state resolution as Export Formats just above.
        let df_themes_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::DistractionFreeThemesViewModel>()
            .cloned()
        {
            Some(themes_vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_editor())),
                    tr!(settings_page_distraction_free_themes()),
                ),
                crate::settings::panes::distraction_free_themes::distraction_free_themes_pane(
                    ctx,
                    &themes_vm,
                    vm.distraction_free_theme(),
                ),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_editor())),
                tr!(settings_page_distraction_free_themes()),
                Sec::Editor.icon_svg(),
            )),
        };

        // ── Backup ("Copies de secours") panes ──
        // Wrapped in the shared `pane_frame` (breadcrumb · rule · scrollable,
        // padded body) exactly like every built-in pane, so they match the rest
        // and their tall forms scroll instead of overflowing.
        let backup_vm = ctx.app_state::<BackupSettingsViewModel>().cloned();
        let backup_pane: Box<dyn Widget> = match &backup_vm {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_backup())),
                    tr!(settings_page_backup()),
                ),
                crate::settings::panes::backup::general_pane(ctx, vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_backup())),
                tr!(settings_page_backup()),
                Sec::BackupSync.icon_svg(),
            )),
        };
        let work_backup_pane: Box<dyn Widget> = match (&backup_vm, &work) {
            (Some(vm), Some(w)) if w.id().is_some() => {
                let uid = w.unique_id().get();
                let path = self
                    .session
                    .single_work_info
                    .file_name()
                    .get()
                    .unwrap_or_default();
                let title = w.title().get();
                Box::new(pane_frame(
                    crumb(
                        Some(lit!(format!(
                            "{}: {}",
                            tr!(settings_sec_work()).resolve_now(),
                            title
                        ))),
                        tr!(settings_page_work_backup()),
                    ),
                    crate::settings::panes::backup::work_backup_pane(
                        ctx,
                        vm,
                        uid,
                        path,
                        title,
                        // F3 — `ids.work_id` is the authoritative "current
                        // Work" id, never `single_work.id()` (`w` here):
                        // this feeds `BackupsListPanel`/`BackupsListViewModel`
                        // for the identical delete-failure toast routing as
                        // `app.rs`'s `backups.show` handler (F2), so it must
                        // read the same source.
                        self.session.ids.work_id.get(),
                    ),
                ))
            }
            _ => Box::new(empty_pane(
                None,
                tr!(settings_page_work_backup()),
                res!("assets/icons/binder/book.svg"),
            )),
        };

        // Work ▸ Tags — the per-project palette manager, over THIS WINDOW's own
        // `WorkSession::tags` (never `ctx.app_state::<TagsViewModel>()`, which
        // would resolve to whichever Work's session registered it first).
        // Present in the Switcher regardless, an empty placeholder when no
        // project is open (same as the other Work panes).
        let tags_pane: Box<dyn Widget> = match (Some(self.session.tags.clone()), &work) {
            (Some(tvm), Some(w)) if w.id().is_some() => {
                let title = w.title().get();
                Box::new(pane_frame(
                    crumb(
                        Some(lit!(format!(
                            "{}: {}",
                            tr!(settings_sec_work()).resolve_now(),
                            title
                        ))),
                        tr!(settings_page_tags()),
                    ),
                    crate::settings::panes::work_tags::work_tags_pane(ctx, &tvm),
                ))
            }
            _ => Box::new(empty_pane(
                None,
                tr!(settings_page_tags()),
                res!("assets/icons/binder/book.svg"),
            )),
        };

        // Work ▸ Templates — the per-project note-template catalogue, over THIS WINDOW's
        // own `WorkSession::note_templates`, same reasoning as `tags_pane` above.
        let templates_pane: Box<dyn Widget> =
            match (Some(self.session.note_templates.clone()), &work) {
                (Some(nvm), Some(w)) if w.id().is_some() => {
                    let title = w.title().get();
                    Box::new(pane_frame(
                        crumb(
                            Some(lit!(format!(
                                "{}: {}",
                                tr!(settings_sec_work()).resolve_now(),
                                title
                            ))),
                            tr!(settings_page_templates()),
                        ),
                        crate::settings::panes::work_templates::work_templates_pane(ctx, &nvm),
                    ))
                }
                _ => Box::new(empty_pane(
                    None,
                    tr!(settings_page_templates()),
                    res!("assets/icons/binder/book.svg"),
                )),
            };

        // Work ▸ Text replacements — the per-project custom lexicon, over THIS
        // WINDOW's own `WorkSession::text_replacements` (never `ctx.app_state`),
        // same reasoning as `tags_pane`/`dictionary_pane`/`punctuation` above.
        let text_replacements_pane: Box<dyn Widget> =
            match (Some(self.session.text_replacements.clone()), &work) {
                (Some(rvm), Some(w)) if w.id().is_some() => {
                    let title = w.title().get();
                    Box::new(pane_frame(
                        crumb(
                            Some(lit!(format!(
                                "{}: {}",
                                tr!(settings_sec_work()).resolve_now(),
                                title
                            ))),
                            tr!(settings_page_text_replacements()),
                        ),
                        crate::settings::panes::text_replacements::text_replacements_pane(
                            ctx, &rvm,
                        ),
                    ))
                }
                _ => Box::new(empty_pane(
                    None,
                    tr!(settings_page_text_replacements()),
                    res!("assets/icons/binder/book.svg"),
                )),
            };

        // Work ▸ Personal dictionary — the per-project word-list manager, over
        // THIS WINDOW's own `WorkSession::user_dictionary` (never
        // `ctx.app_state::<UserDictionaryViewModel>()`, same reasoning as
        // `tags_pane` above). Present in the Switcher regardless, an empty
        // placeholder when no project is open (same as the other Work panes).
        let dictionary_pane: Box<dyn Widget> =
            match (Some(self.session.user_dictionary.clone()), &work) {
                (Some(vm), Some(w)) if w.id().is_some() => {
                    let title = w.title().get();
                    Box::new(pane_frame(
                        crumb(
                            Some(lit!(format!(
                                "{}: {}",
                                tr!(settings_sec_work()).resolve_now(),
                                title
                            ))),
                            tr!(settings_page_personal_dictionary()),
                        ),
                        crate::settings::panes::user_dictionary::user_dictionary_pane(ctx, &vm),
                    ))
                }
                _ => Box::new(empty_pane(
                    None,
                    tr!(settings_page_personal_dictionary()),
                    res!("assets/icons/binder/book.svg"),
                )),
            };

        // ── Left rail: search + category tree ───────────────────────────────
        let (tree, selection, nodes) = self.build_tree(ctx, &spec, work_title.clone());
        // Every way of reaching a page that isn't clicking its own row goes
        // through this: the search suggestions, and the links on each parent's
        // page. It must be built from the tree's own selection model and node
        // map, or a jump would switch the pane while leaving the highlight behind.
        let nav = Navigator {
            selection,
            nodes: Rc::new(nodes),
            selected_pane: self.selected_pane.clone(),
        };
        let search = self.search_field(&spec, &work_title, nav.clone());
        let left = VStack::new()
            .spacing(0.0)
            .child(Padding::symmetric(10.0, 10.0).child(search))
            .child(Expand::vertical().child(Padding::symmetric(2.0, 6.0).child(tree)));

        // ── Right pane: the per-page content behind the selection Switcher ──
        // The Switcher index is looked up in this very list (see below), so a page
        // may be added anywhere in it; each child is tagged with the `Pane` it
        // serves rather than trusted from a hand-kept `.child()` chain.
        let typo = vm.editor_typography();
        let panes: Vec<(Pane, Box<dyn Widget>)> = vec![
            (Pane::User, Box::new(panes::user::user_pane(&vm))),
            (
                Pane::Appearance,
                Box::new(panes::appearance::appearance_pane(&vm, scale.clone())),
            ),
            (
                Pane::MenusToolbars,
                Box::new(empty_pane(
                    Some(tr!(settings_sec_appearance_behaviour())),
                    tr!(settings_page_menus()),
                    Sec::AppearanceBehaviour.icon_svg(),
                )),
            ),
            (
                Pane::Notifications,
                panes::notifications::notifications_pane(ctx),
            ),
            (
                Pane::SceneTypography,
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_scene()),
                    &typo.scene,
                )),
            ),
            (
                Pane::SynopsisTypography,
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_synopsis()),
                    &typo.synopsis,
                )),
            ),
            (
                Pane::NotesTypography,
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_notes()),
                    &typo.notes,
                )),
            ),
            (
                Pane::EditorBehavior,
                Box::new(panes::editor_behavior::editor_behavior_pane(ctx, &vm)),
            ),
            (Pane::Goals, Box::new(panes::goals::goals_pane(ctx, &vm))),
            (
                Pane::Games,
                // The activation comes from THIS panel's own `WorkSession` — the
                // project the opening window shows — never `ctx.app_state`, which
                // is one slot per process and would let the pane start (or stop) a
                // game in whichever project happened to open first.
                Box::new(panes::games::games_pane(ctx, &games)),
            ),
            (
                Pane::Corkboard,
                Box::new(panes::corkboard::corkboard_pane(ctx, &vm)),
            ),
            (Pane::Dictionaries, dictionaries_pane),
            (
                Pane::Autosave,
                Box::new(panes::autosave::autosave_pane(&vm)),
            ),
            (Pane::ExportFormats, export_styles_pane),
            (Pane::Paratext, paratext_pane),
            (Pane::Keymap, Box::new(panes::keymap::keymap_pane(ctx))),
            (Pane::WorkStructure, structure_pane),
            (Pane::WorkPunctuation, punctuation_pane),
            (
                Pane::Punctuation,
                Box::new(panes::punctuation::punctuation_pane(ctx, &vm)),
            ),
            (Pane::Backup, backup_pane),
            (Pane::WorkBackup, work_backup_pane),
            (Pane::WorkLanguage, language_pane),
            (Pane::WorkDictionary, dictionary_pane),
            (
                Pane::Spellcheck,
                Box::new(panes::spellcheck::spellcheck_pane(&vm)),
            ),
            (Pane::WorkTags, tags_pane),
            (Pane::WorkAuthor, author_pane),
            (Pane::WorkTextReplacements, text_replacements_pane),
            (
                Pane::DistractionFree,
                Box::new(panes::distraction_free::distraction_free_pane(
                    ctx,
                    &vm,
                    &typo.distraction_free,
                )),
            ),
            (Pane::DistractionFreeThemes, df_themes_pane),
            (Pane::WorkTemplates, templates_pane),
        ];
        // Anything an extension registered, appended in the same order
        // `build_tree` inserted its nodes — the `Switcher` index is derived from
        // this vec below, so the two cannot disagree however the list grows.
        //
        // Built here, with the real `BuildContext`, so a contributed page reaches
        // `ctx.settings()` and binds its own keys exactly as a built-in pane does.
        let mut panes = panes;
        for page in crate::settings_ext::registered_pages() {
            let body = crate::settings_ext::build_page(page.id, ctx)
                .unwrap_or_else(|| Box::new(teksilo::widgets::Spacer::new()));
            panes.push((Pane::Extension(page.id), body));
        }
        // Every parent's own page, off the same spec the tree was built from — so
        // a section added there arrives with its page already written, and a page
        // moved between sections is listed by its new parent without a second
        // edit. A parent the spec left out (the Work section with nothing open)
        // has no page here either, and nothing can select it.
        for root in &spec {
            let Root::Section(sec, branches) = root else {
                continue;
            };
            let parent = Pane::Section(*sec);
            panes.push((
                parent,
                Box::new(panes::overview::overview_pane(
                    parent,
                    section_title(*sec, &work_title),
                    None,
                    children_of(&spec, parent),
                    nav.clone(),
                )),
            ));
            for branch in branches {
                let Branch::Group(group, _) = branch else {
                    continue;
                };
                let parent = Pane::Group(*group);
                panes.push((
                    parent,
                    Box::new(panes::overview::overview_pane(
                        parent,
                        group.label(),
                        // One level in, so its trail names the section it sits in.
                        Some(section_title(*sec, &work_title)),
                        children_of(&spec, parent),
                        nav.clone(),
                    )),
                ));
            }
        }
        // The list IS the order. Deriving the `Switcher` index by looking the selected
        // pane up in this very vec is what makes the pairing true rather than merely
        // asserted: previously the index was `pane as usize` and a page added at the
        // wrong slot shifted every later page's content by one — silently, and it
        // shipped that way once. A `debug_assert!` caught it in debug builds only.
        // Nothing to catch now; the two cannot disagree.
        //
        // An unknown pane resolves to slot 0, matching `Switcher`'s own out-of-range
        // behaviour: showing the first page beats showing a blank panel.
        let order: Vec<Pane> = panes.iter().map(|(p, _)| *p).collect();
        let index = self
            .selected_pane
            .map(move |sel| order.iter().position(|p| p == sel).unwrap_or(0));
        let content = panes
            .into_iter()
            .fold(Switcher::new(index), |sw, (_, body)| sw.child_boxed(body));

        let footer = self.footer(vm, scale, not_defaults);
        let right = VStack::new()
            .spacing(0.0)
            .child(Expand::vertical().child(content))
            .child(Expand::horizontal().child(Divider::new()))
            .child(FixedSize::new().height(FOOTER_H).child(footer));

        // ── Header strip (title + close), mirroring the Welcome panel ───────
        let header = FixedSize::new().height(HEADER_H).child(
            Padding::symmetric(8.0, 14.0).child(
                HStack::new()
                    .spacing(8.0)
                    .child(
                        Expand::horizontal().child(
                            TextWidget::new(tr!(settings_title()))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Secondary),
                        ),
                    )
                    .child(
                        IconButton::clear()
                            .tooltip(tr!(settings_close()))
                            .on_activate_fn(|ctx| ctx.dismiss_modal()),
                    ),
            ),
        );

        let root = teksu!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        Expand::horizontal {
                            child: header
                        }
                        Expand::horizontal {
                            Divider
                        }
                        HStack {
                            spacing: 0.0
                            FixedSize {
                                width: TREE_W
                                height: BODY_H
                                child: left
                            }
                            FixedSize {
                                height: BODY_H
                                Divider::vertical()
                            }
                            Expand::horizontal {
                                FixedSize {
                                    height: BODY_H
                                    child: right
                                }
                            }
                        }
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Delegate to the fixed-size root so the modal host sizes/centres the
        // card (delegating to the greedy inner Panel would fill the window).
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

impl SettingsPanel {
    /// The action bar: Reset to defaults · (spacer) · Done.
    ///
    /// Instant-apply — every setting already took effect and persisted as it was
    /// changed, so there is no Apply/Cancel/OK. *Reset to defaults* live-applies
    /// factory values (enabled only while something differs from them) behind a
    /// confirmation, since there is no undo; *Done* closes the window.
    fn footer(
        &self,
        vm: SettingsViewModel,
        scale: Signal<f32>,
        not_defaults: Signal<bool>,
    ) -> impl Widget {
        // Reset to defaults — confirm first (no undo), then live-apply the
        // factory values. Disabled while already at defaults.
        let reset_vm = vm.clone();
        let reset_scale = scale.clone();
        let reset = Button::new(tr!(settings_reset()))
            .variant(ButtonVariant::Plain)
            .enabled(not_defaults)
            .on_activate_fn(move |ctx| {
                let reset_vm = reset_vm.clone();
                let reset_scale = reset_scale.clone();
                MessageBox::warning(tr!(settings_reset_confirm_title()))
                    .text(tr!(settings_reset_confirm_body()))
                    .buttons(MessageBoxButtons::Custom(vec![
                        MessageBoxButton::standard(StandardButton::RestoreDefaults),
                        MessageBoxButton::standard(StandardButton::Cancel),
                    ]))
                    .default_button(StandardButton::Cancel)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |res, ctx| {
                        if res.button == StandardButton::RestoreDefaults {
                            ctx.set_theme(intui::light());
                            ctx.set_locale("en-US");
                            reset_scale.set(TEXT_SCALE_DEFAULT);
                            reset_vm.reset_editor_defaults();
                        }
                    })
                    .present(ctx);
            });

        // Done — everything already applied + persisted; just close the window.
        let done = Button::new(tr!(settings_done()))
            .variant(ButtonVariant::Filled)
            .on_activate_fn(|ctx| ctx.dismiss_modal());

        Padding::symmetric(10.0, 22.0).child(
            HStack::new()
                .spacing(9.0)
                .child(reset)
                .child(Spacer::new())
                .child(done),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
