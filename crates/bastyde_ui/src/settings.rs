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
//! Categories that don't yet carry settings render an **empty placeholder**.
//! **Instant-apply** (the macOS / GNOME convention): every change takes effect and
//! persists immediately, so there is no Apply/Cancel/OK staged model. The footer
//! carries only *Reset to defaults* (left — enabled only while something differs
//! from the factory defaults, and guarded by a confirmation since there is no undo)
//! and *Done* (right — closes the window). The header ✕ closes it too.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use bastyde::canvas::svg::SvgIcon;
use bastyde::core::styles::{PanelVariant, Theme};
use bastyde::data::{KeyedSelectionModel, NodeId, SelectionMode, TreeModel};
use bastyde::i18n::{LocalizedString, current_locale, localized};
use bastyde::prelude::*;

mod panes;
use bastyde::res;
use bastyde::settings::{SettingsExt, TEXT_SCALE_KEY};
use bastyde::widgets::{
    Breadcrumb, BreadcrumbItem, Button, ButtonVariant, Center, Divider, Expand, FixedSize,
    FontPicker, FormLayout, GroupHeader, HStack, IconButton, IconWidget, LanguageSwitcher,
    MessageBox, MessageBoxButton, MessageBoxButtons, Padding, Panel, RadioButton, RadioGroup,
    ScrollArea, SearchField, Slider, Spacer, StandardButton, StandardTreeItem, Switcher,
    TextScaleControl, TextWidget, ThemeSwitcher, Toggle, TreeView, VStack,
};

use crate::sessions::WorkSession;
use crate::view_models::{
    BackupSettingsViewModel, EditorTypography, SettingsViewModel, WorkSettingsViewModel,
};
use crate::{
    DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT, DISTRACTION_FREE_FONT_FAMILY_DEFAULT,
    DISTRACTION_FREE_LINE_HEIGHT_DEFAULT, DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT,
    DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT, DISTRACTION_FREE_SIZE_DEFAULT,
    DISTRACTION_FREE_GO_DEFAULT, DISTRACTION_FREE_GO_TO_DEFAULT, DISTRACTION_FREE_SESSION_DEFAULT,
    DISTRACTION_FREE_TAB_BAR_DEFAULT, DISTRACTION_FREE_WIDTH_DEFAULT,
    DISTRACTION_FREE_WORD_COUNT_DEFAULT, EDITOR_WIDTH_DEFAULT, GOALS_SHOW_CHARACTERS_DEFAULT,
    HIGHLIGHT_SENTENCE_DEFAULT, NOTES_FIRST_LINE_INDENT_DEFAULT, NOTES_FONT_FAMILY_DEFAULT,
    NOTES_LINE_HEIGHT_DEFAULT, NOTES_PARA_SPACING_AFTER_DEFAULT,
    NOTES_PARA_SPACING_BEFORE_DEFAULT, NOTES_SIZE_DEFAULT, SCENE_FIRST_LINE_INDENT_DEFAULT,
    SCENE_FONT_FAMILY_DEFAULT, SCENE_LINE_HEIGHT_DEFAULT, SCENE_PARA_SPACING_AFTER_DEFAULT,
    SCENE_PARA_SPACING_BEFORE_DEFAULT, SCENE_SIZE_DEFAULT, SYNOPSIS_FIRST_LINE_INDENT_DEFAULT,
    SYNOPSIS_FONT_FAMILY_DEFAULT, SYNOPSIS_LINE_HEIGHT_DEFAULT, SYNOPSIS_PANE_DEFAULT,
    SYNOPSIS_PARA_SPACING_AFTER_DEFAULT, SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT,
    SYNOPSIS_SIZE_DEFAULT, TYPEWRITER_DEFAULT,
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

/// A selectable settings page (a tree *leaf* → its own pane). The discriminant
/// order is the `Switcher` child order, so `pane as usize` is the page index.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pane {
    Appearance = 0,
    MenusToolbars,
    Notifications,
    SceneTypography,
    SynopsisTypography,
    NotesTypography,
    EditorBehavior,
    Goals,
    Corkboard,
    Dictionaries,
    Autosave,
    ExportFormats,
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
    /// Per-project personal dictionary (under the open Work's section). Appended last so
    /// the earlier discriminants — and the `Switcher` order they index — stay put.
    WorkDictionary,
    /// The app-wide master spell-check switch (under Spelling, above Dictionaries in the
    /// tree). Appended last, same rule: a new discriminant must never renumber the ones the
    /// `Switcher` already indexes.
    Spellcheck,
    /// Per-project tag palette (under the open Work's section). Appended last for the same
    /// reason — its tree position is chosen in `build_tree`, not by this number.
    WorkTags,
    /// Per-project author name (under the open Work's section). Appended last, same rule —
    /// it is shown *first* in the tree, but that is `build_tree`'s business, not this
    /// number's: renumbering here would re-point every later `Switcher` slot.
    WorkAuthor,
    /// Per-project custom replacement lexicon (under the open Work's section). Appended
    /// last, same rule as every one above it.
    WorkTextReplacements,
    /// Distraction-free mode's own typography bundle (nested under Editor ▸
    /// Typography, alongside Scene/Synopsis/Notes/Corkboard). Appended last, same
    /// rule as every one above it — its tree position (inside the new
    /// `GroupKind::Typography` node) is `build_tree`'s business, not this number's.
    DistractionFree,
}

impl Pane {
    fn index(self) -> usize {
        self as usize
    }

    fn label(self) -> LocalizedString {
        match self {
            Pane::Appearance => tr!(settings_page_appearance()),
            Pane::MenusToolbars => tr!(settings_page_menus()),
            Pane::Notifications => tr!(settings_page_notifications()),
            Pane::SceneTypography => tr!(settings_page_scene()),
            Pane::SynopsisTypography => tr!(settings_page_synopsis()),
            Pane::NotesTypography => tr!(settings_page_notes()),
            Pane::EditorBehavior => tr!(settings_page_editor_behavior()),
            Pane::Goals => tr!(settings_page_goals()),
            Pane::Corkboard => tr!(settings_page_corkboard()),
            Pane::Dictionaries => tr!(settings_page_dictionaries()),
            Pane::Autosave => tr!(settings_page_autosave()),
            Pane::ExportFormats => tr!(settings_page_export()),
            Pane::Keymap => tr!(settings_page_keymap()),
            Pane::WorkStructure => tr!(settings_page_structure()),
            Pane::WorkPunctuation => tr!(settings_page_punctuation()),
            Pane::Punctuation => tr!(settings_page_punctuation()),
            Pane::Backup => tr!(settings_page_backup()),
            Pane::WorkBackup => tr!(settings_page_work_backup()),
            Pane::WorkLanguage => tr!(settings_page_language()),
            Pane::WorkDictionary => tr!(settings_page_personal_dictionary()),
            Pane::WorkTags => tr!(settings_page_tags()),
            Pane::WorkAuthor => tr!(settings_page_author()),
            Pane::WorkTextReplacements => tr!(settings_page_text_replacements()),
            Pane::Spellcheck => tr!(settings_page_spellcheck()),
            Pane::DistractionFree => tr!(settings_page_distraction_free()),
        }
    }
}

/// A parent category (a tree *branch* → expands to its pages; no pane of its own).
#[derive(Clone, Copy)]
enum Sec {
    AppearanceBehaviour,
    Editor,
    Spelling,
    BackupSync,
    CompileExport,
    /// The open project. Its displayed label is "Work: `<title>`" (the title is
    /// filled in at tree-build time — the enum stays data-free / `Copy`).
    Work,
}

impl Sec {
    fn label(self) -> LocalizedString {
        match self {
            Sec::AppearanceBehaviour => tr!(settings_sec_appearance_behaviour()),
            Sec::Editor => tr!(settings_sec_editor()),
            Sec::Spelling => tr!(settings_sec_spelling()),
            Sec::BackupSync => tr!(settings_sec_backup()),
            Sec::CompileExport => tr!(settings_sec_compile()),
            Sec::Work => tr!(settings_sec_work()),
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
        }
    }
}

/// A nested grouping node *inside* a section — one level deeper than [`Sec`],
/// for a cluster of pages that would otherwise crowd their section's flat
/// list. Currently only Editor ▸ Typography (Scene / Synopsis / Notes /
/// Corkboard / Distraction-free — five pages that all start with the same
/// Typeface/Size/Line height/First-line indent shape). No pane of its own
/// (expands only, like [`Sec`]) and no icon (design shows icons only on
/// top-level sections plus the Keymap leaf — a nested group is indented like
/// any other sub-page).
#[derive(Clone, Copy)]
enum GroupKind {
    Typography,
}

impl GroupKind {
    fn label(self) -> LocalizedString {
        match self {
            GroupKind::Typography => tr!(settings_group_typography()),
        }
    }
}

/// A node in the category tree: a parent section, a nested group, or a leaf page.
#[derive(Clone, Copy)]
enum Node {
    Section(Sec),
    Group(GroupKind),
    Page(Pane),
}

impl Node {
    fn label(self) -> LocalizedString {
        match self {
            Node::Section(s) => s.label(),
            Node::Group(g) => g.label(),
            Node::Page(p) => p.label(),
        }
    }

    /// Leading icon: every section, plus the top-level Keymap leaf (design shows
    /// no icons on the indented sub-pages, which includes the nested `Group`s).
    fn icon(self) -> Option<IconWidget> {
        let svg = match self {
            Node::Section(s) => s.icon_svg(),
            Node::Page(Pane::Keymap) => res!("assets/icons/settings/keymap.svg"),
            Node::Group(_) | Node::Page(_) => return None,
        };
        Some(IconWidget::from_svg_icon(svg).icon_size(16.0))
    }

    /// The pane a leaf selects; `None` for a parent section or nested group
    /// (expands only).
    fn pane(self) -> Option<Pane> {
        match self {
            Node::Page(p) => Some(p),
            Node::Section(_) | Node::Group(_) => None,
        }
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
    locale: &Option<Signal<bastyde::i18n::LanguageIdentifier>>,
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
        vm.distraction_free_tab_bar()
            .map(|s| *s != DISTRACTION_FREE_TAB_BAR_DEFAULT),
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
        vm.highlight_sentence()
            .map(|s| *s != HIGHLIGHT_SENTENCE_DEFAULT),
        // ── Goals & word count ──
        vm.counting_method()
            .map(|m| *m != CountingMethodSetting::default()),
        vm.show_characters()
            .map(|s| *s != GOALS_SHOW_CHARACTERS_DEFAULT),
    ];
    if let Some(loc) = locale {
        // Compare against a once-parsed default rather than allocating a String
        // per change (clippy::cmp_owned).
        let default_locale: bastyde::i18n::LanguageIdentifier =
            "en-US".parse().expect("valid default locale");
        diffs.push(loc.map(move |l| *l != default_locale));
    }
    any_true(diffs)
}

// ── Small view helpers ───────────────────────────────────────────────────────

/// A left-column field label (dimmed, small) — matches the design's `--tx2`.
/// `pub(crate)` so the backup panes (`settings_backup`) share the exact same
/// row-label style as every built-in pane.
pub(crate) fn field_label(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// A dimmed sub-field hint line.
fn hint(text: LocalizedString) -> TextWidget {
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
fn slider_field(
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
    /// The Tier-2 bundle for the Work the OPENING window shows — never
    /// resolved via `ctx.app_state::<SingleWork/SingleWorkInfo/AppIds/
    /// TagsViewModel/UserDictionaryViewModel>()` any more (Phase 3 fix).
    ///
    /// **Why this was a bug**: `ctx.app_state::<T>()` is one process-wide slot
    /// per type (confirmed against `bastyde-app`'s `app_state_registry`, a
    /// single `HashMap<TypeId, Box<dyn Any>>` for the whole process), seeded
    /// once from the FIRST window's session in `main.rs`'s bootstrap and never
    /// updated thereafter. With a second Work open in a second window,
    /// `SettingsPanel::build()` used to silently read/write the FIRST Work's
    /// structure, language, author, backup override, tags and personal
    /// dictionary — regardless of which window's Settings the user actually
    /// opened. Every call site now threads the opening window's own
    /// `WorkSession` (available at each of the three construction sites:
    /// `app::commands::file`'s `app.settings` action via `CommandDeps::session`,
    /// and `app.rs`'s two toast call sites via `self.session`), the same
    /// pattern `SaveAsViewModel`/`BackupRestoreViewModel` already use.
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

    /// Open straight to Backup & Sync ▸ Backup — the target of the
    /// "no backups configured" nudge toast.
    pub fn open_to_backup(session: WorkSession) -> Self {
        Self::opening_at(Pane::Backup, session)
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
    /// The category tree (left rail). Builds the `TreeModel`, seeds selection to
    /// the active page, and wires selection → `selected_pane`. Returns the
    /// `TreeView`, the search's page→node map, and the model for lookups.
    fn build_tree(
        &self,
        ctx: &mut BuildContext,
    ) -> (
        impl Widget,
        KeyedSelectionModel<NodeId>,
        HashMap<Pane, NodeId>,
    ) {
        let model: TreeModel<Node> = TreeModel::new();
        let mut nodes: HashMap<Pane, NodeId> = HashMap::new();

        let ab = model.insert_root(0, Node::Section(Sec::AppearanceBehaviour));
        nodes.insert(
            Pane::Appearance,
            model.insert_child(ab, 0, Node::Page(Pane::Appearance)),
        );
        nodes.insert(
            Pane::MenusToolbars,
            model.insert_child(ab, 1, Node::Page(Pane::MenusToolbars)),
        );
        nodes.insert(
            Pane::Notifications,
            model.insert_child(ab, 2, Node::Page(Pane::Notifications)),
        );

        let ed = model.insert_root(1, Node::Section(Sec::Editor));
        // The five typography-shaped pages (Scene / Synopsis / Notes / Corkboard /
        // Distraction-free — all four fields + font/size/line-height/indent) live
        // under their own nested "Typography" group rather than as five more flat
        // siblings in an already-crowded Editor list.
        let typo_group = model.insert_child(ed, 0, Node::Group(GroupKind::Typography));
        nodes.insert(
            Pane::SceneTypography,
            model.insert_child(typo_group, 0, Node::Page(Pane::SceneTypography)),
        );
        nodes.insert(
            Pane::SynopsisTypography,
            model.insert_child(typo_group, 1, Node::Page(Pane::SynopsisTypography)),
        );
        nodes.insert(
            Pane::NotesTypography,
            model.insert_child(typo_group, 2, Node::Page(Pane::NotesTypography)),
        );
        nodes.insert(
            Pane::Corkboard,
            model.insert_child(typo_group, 3, Node::Page(Pane::Corkboard)),
        );
        nodes.insert(
            Pane::DistractionFree,
            model.insert_child(typo_group, 4, Node::Page(Pane::DistractionFree)),
        );
        nodes.insert(
            Pane::EditorBehavior,
            model.insert_child(ed, 1, Node::Page(Pane::EditorBehavior)),
        );
        // Beside Editor Behavior: the other set of switches that change what
        // happens as the writer types, rather than how the page looks.
        nodes.insert(
            Pane::Punctuation,
            model.insert_child(ed, 2, Node::Page(Pane::Punctuation)),
        );
        nodes.insert(
            Pane::Goals,
            model.insert_child(ed, 3, Node::Page(Pane::Goals)),
        );

        let sp = model.insert_root(2, Node::Section(Sec::Spelling));
        nodes.insert(
            Pane::Spellcheck,
            model.insert_child(sp, 0, Node::Page(Pane::Spellcheck)),
        );
        nodes.insert(
            Pane::Dictionaries,
            model.insert_child(sp, 1, Node::Page(Pane::Dictionaries)),
        );

        let bk = model.insert_root(3, Node::Section(Sec::BackupSync));
        nodes.insert(
            Pane::Autosave,
            model.insert_child(bk, 0, Node::Page(Pane::Autosave)),
        );
        nodes.insert(
            Pane::Backup,
            model.insert_child(bk, 1, Node::Page(Pane::Backup)),
        );

        let ce = model.insert_root(4, Node::Section(Sec::CompileExport));
        nodes.insert(
            Pane::ExportFormats,
            model.insert_child(ce, 0, Node::Page(Pane::ExportFormats)),
        );

        nodes.insert(Pane::Keymap, model.insert_root(5, Node::Page(Pane::Keymap)));

        // The open project's own section (multi-project-ready): shown only when a
        // Work is open, labelled "Work: `<title>`" — for now its single page is
        // Structure (the chapter mode). Read through THIS window's own
        // `WorkSession::single_work` (never `ctx.app_state`, see this struct's
        // `session` field doc).
        let work_title = Some(&self.session.single_work)
            .filter(|w| w.id().is_some())
            .map(|w| w.title().get());
        let mut work_node: Option<NodeId> = None;
        if work_title.is_some() {
            let wk = model.insert_root(6, Node::Section(Sec::Work));
            // Author leads the section: it is the one field about the *book* rather
            // than about how the app handles it.
            nodes.insert(
                Pane::WorkAuthor,
                model.insert_child(wk, 0, Node::Page(Pane::WorkAuthor)),
            );
            nodes.insert(
                Pane::WorkStructure,
                model.insert_child(wk, 1, Node::Page(Pane::WorkStructure)),
            );
            nodes.insert(
                Pane::WorkLanguage,
                model.insert_child(wk, 2, Node::Page(Pane::WorkLanguage)),
            );
            nodes.insert(
                Pane::WorkBackup,
                model.insert_child(wk, 3, Node::Page(Pane::WorkBackup)),
            );
            nodes.insert(
                Pane::WorkDictionary,
                model.insert_child(wk, 4, Node::Page(Pane::WorkDictionary)),
            );
            nodes.insert(
                Pane::WorkTags,
                model.insert_child(wk, 5, Node::Page(Pane::WorkTags)),
            );
            // Beside the personal dictionary and the tag palette: the third per-project
            // vocabulary the writer curates.
            nodes.insert(
                Pane::WorkTextReplacements,
                model.insert_child(wk, 6, Node::Page(Pane::WorkTextReplacements)),
            );
            // Next to the lexicon: the other thing that rewrites prose as it is
            // typed, and the other one that travels inside the `.skrib`.
            nodes.insert(
                Pane::WorkPunctuation,
                model.insert_child(wk, 7, Node::Page(Pane::WorkPunctuation)),
            );
            work_node = Some(wk);
        }
        // The dynamic section label needs the title inside the row closure.
        let work_title_row = work_title.unwrap_or_default();

        // Single-selection, seeded to the active page so the pane + highlight
        // agree on open (Manuscript & Fonts by default).
        let selection = KeyedSelectionModel::<NodeId>::new(SelectionMode::Single);
        if let Some(id) = nodes.get(&self.selected_pane.get()) {
            selection.select(*id);
        }

        // Selection → active page (leaves only; a section click just expands).
        {
            let selected_pane = self.selected_pane.clone();
            let model = model.clone();
            let sig = selection.selection_signal();
            ctx.effect(&sig, move |set: &HashSet<NodeId>| {
                if let Some(id) = set.iter().next().copied()
                    && let Some(Some(pane)) = model.with_item(id, |n: &Node| n.pane())
                {
                    selected_pane.set(pane);
                }
            });
        }

        // A row-body click selects (leaf → pane; section → no pane); the chevron
        // (wired via `on_toggle_rc`) expands/collapses a section. Proven config
        // (mirrors the framework's own tree examples) — reliable for both mouse
        // and synthetic input.
        let tree =
            TreeView::new_with_context(model, move |node: &Node, entry, selected, rowctx| {
                // The Work section's label is dynamic ("Work: `<title>`"); every
                // other node uses its static label.
                let label = match node {
                    Node::Section(Sec::Work) => lit!(format!(
                        "{}: {}",
                        tr!(settings_sec_work()).resolve_now(),
                        work_title_row
                    )),
                    other => other.label(),
                };
                let mut row = StandardTreeItem::new(label)
                    .from_entry(entry)
                    .selected(selected)
                    .on_toggle_rc(rowctx.toggle_callback());
                if let Some(icon) = node.icon() {
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
        // Editor section.
        tree.expand(ab);
        tree.expand(ed);
        tree.expand(typo_group);
        tree.collapse(sp);
        tree.collapse(bk);
        tree.collapse(ce);
        if let Some(wk) = work_node {
            tree.expand(wk);
        }
        // Reveal the section owning the page we opened at, so a deep-link open
        // (e.g. the toast that jumps straight to Dictionaries, under the
        // otherwise-collapsed Spelling section) shows its highlighted tree node.
        // Idempotent with the design-state expansion above.
        let section_of = |p: Pane| match p {
            Pane::Appearance | Pane::MenusToolbars | Pane::Notifications => Some(ab),
            // The nested Typography group — `ed` (its parent, always expanded
            // above) already guarantees these are reachable; only the group
            // itself needs revealing.
            Pane::SceneTypography
            | Pane::SynopsisTypography
            | Pane::NotesTypography
            | Pane::Corkboard
            | Pane::DistractionFree => Some(typo_group),
            Pane::EditorBehavior | Pane::Punctuation | Pane::Goals => Some(ed),
            Pane::Spellcheck | Pane::Dictionaries => Some(sp),
            Pane::Autosave | Pane::Backup => Some(bk),
            Pane::ExportFormats => Some(ce),
            Pane::WorkStructure
            | Pane::WorkPunctuation
            | Pane::WorkLanguage
            | Pane::WorkBackup
            | Pane::WorkDictionary
            | Pane::WorkTags
            | Pane::WorkAuthor
            | Pane::WorkTextReplacements => work_node,
            Pane::Keymap => None,
        };
        if let Some(sec) = section_of(self.selected_pane.get()) {
            tree.expand(sec);
        }

        (tree, selection, nodes)
    }

    /// The "Search settings" field, offering suggestions across every page and
    /// setting; selecting a suggestion jumps to (and highlights) its page.
    fn search_field(
        &self,
        selection: KeyedSelectionModel<NodeId>,
        nodes: HashMap<Pane, NodeId>,
    ) -> impl Widget {
        // (searchable label, the page it lives on). Resolved per-keystroke so it
        // follows a locale change.
        let mut idx: Vec<(LocalizedString, Pane)> = vec![
            (tr!(settings_page_appearance()), Pane::Appearance),
            (tr!(settings_page_menus()), Pane::MenusToolbars),
            (tr!(settings_page_notifications()), Pane::Notifications),
            (tr!(settings_page_scene()), Pane::SceneTypography),
            (tr!(settings_page_synopsis()), Pane::SynopsisTypography),
            (tr!(settings_page_notes()), Pane::NotesTypography),
            (tr!(settings_page_editor_behavior()), Pane::EditorBehavior),
            (tr!(settings_page_goals()), Pane::Goals),
            (tr!(settings_page_corkboard()), Pane::Corkboard),
            (tr!(settings_page_distraction_free()), Pane::DistractionFree),
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
            (tr!(settings_synopsis_pane()), Pane::EditorBehavior),
            // The distraction-free chrome toggles live on the Editor Behavior
            // page, not the Distraction-free typography page — searching for
            // "tabs" has to land where the checkbox actually is.
            (
                tr!(settings_distraction_free_tab_bar()),
                Pane::EditorBehavior,
            ),
            (
                tr!(settings_distraction_free_word_count()),
                Pane::EditorBehavior,
            ),
            (
                tr!(settings_distraction_free_session()),
                Pane::EditorBehavior,
            ),
            (tr!(settings_distraction_free_go()), Pane::EditorBehavior),
            (
                tr!(settings_distraction_free_go_to()),
                Pane::EditorBehavior,
            ),
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
        let index: Rc<Vec<(LocalizedString, Pane)>> = Rc::new(idx);

        let for_suggest = index.clone();
        let for_select = index.clone();
        let selected_pane = self.selected_pane.clone();

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
                    if let Some(id) = nodes.get(pane) {
                        selection.select(*id);
                    }
                    selected_pane.set(*pane);
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

        // Work ▸ Text replacements — the per-project custom lexicon, over THIS
        // WINDOW's own `WorkSession::text_replacements` (never `ctx.app_state`),
        // same reasoning as `tags_pane`/`dictionary_pane`/`punctuation` above.
        let text_replacements_pane: Box<dyn Widget> = match (
            Some(self.session.text_replacements.clone()),
            &work,
        ) {
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
                    crate::settings::panes::text_replacements::text_replacements_pane(ctx, &rvm),
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
        let dictionary_pane: Box<dyn Widget> = match (
            Some(self.session.user_dictionary.clone()),
            &work,
        ) {
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
        let (tree, selection, nodes) = self.build_tree(ctx);
        let search = self.search_field(selection, nodes);
        let left = VStack::new()
            .spacing(0.0)
            .child(Padding::symmetric(10.0, 10.0).child(search))
            .child(Expand::vertical().child(Padding::symmetric(2.0, 6.0).child(tree)));

        // ── Right pane: the per-page content behind the selection Switcher ──
        // The Switcher is indexed by `Pane::index()`, so its child at position i must
        // be pane i's body. Rather than trust a hand-kept `.child()` chain — where a
        // pane inserted mid-list silently shifts every later pane's content by one (a
        // real bug: the app-wide Spellcheck pane, its discriminant appended last but
        // its child slotted in the middle, made every Work pane show its neighbour) —
        // each child is tagged with the `Pane` it serves and the order is asserted.
        let ab = tr!(settings_sec_appearance_behaviour());
        let typo = vm.editor_typography();
        let panes: Vec<(Pane, Box<dyn Widget>)> = vec![
            (
                Pane::Appearance,
                Box::new(panes::appearance::appearance_pane(&vm, scale.clone())),
            ),
            (
                Pane::MenusToolbars,
                Box::new(empty_pane(
                    Some(ab.clone()),
                    tr!(settings_page_menus()),
                    Sec::AppearanceBehaviour.icon_svg(),
                )),
            ),
            (
                Pane::Notifications,
                Box::new(empty_pane(
                    Some(ab),
                    tr!(settings_page_notifications()),
                    Sec::AppearanceBehaviour.icon_svg(),
                )),
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
                Box::new(panes::editor_behavior::editor_behavior_pane(&vm)),
            ),
            (Pane::Goals, Box::new(panes::goals::goals_pane(ctx, &vm))),
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
            (
                Pane::Keymap,
                Box::new(empty_pane(
                    None,
                    tr!(settings_page_keymap()),
                    res!("assets/icons/settings/keymap.svg"),
                )),
            ),
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
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_distraction_free()),
                    &typo.distraction_free,
                )),
            ),
        ];
        if let Some((slot, (pane, _))) =
            panes.iter().enumerate().find(|(i, (p, _))| p.index() != *i)
        {
            // A pane inserted at the wrong slot shifts every later pane's content by one — the
            // failure that shipped once already. Catch it the instant the panel builds.
            debug_assert!(
                false,
                "settings Switcher child {slot} serves a pane whose discriminant is {}, \
                 not {slot} — insert it at its discriminant position",
                pane.index()
            );
        }
        let content = panes.into_iter().fold(
            Switcher::new(self.selected_pane.map(|p| p.index())),
            |sw, (_, body)| sw.child_boxed(body),
        );

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

        let root = bati!(ctx => FixedSize {
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
    // from outside `bastyde-core`), so the full panel is verified live via
    // `scripts/automation_settings.py` rather than headlessly — the same boundary
    // the settings-dependent `WelcomePanel` sits on.

    /// The `Switcher` is indexed by `Pane::index()`, so its child at slot i must be pane i's
    /// body. `build()` tags each child with the `Pane` it serves and a `debug_assert` trips if
    /// any lands at the wrong slot — the failure that shipped when the Spellcheck pane's child
    /// was inserted mid-chain while its discriminant went last, shifting ten Work/Spelling panes
    /// onto their neighbour's content.
    ///
    /// This test guards the half of the contract reachable headlessly: the discriminants form a
    /// gap-free `0..N` from `Appearance`, so `index()` is a valid dense `Switcher` slot for every
    /// pane. (The child↔pane pairing needs a built panel, which depends on `SettingsStore`
    /// app-state a bare `WidgetTree` can't provide — hence the build-time `debug_assert` and the
    /// live `run-app` check.)
    #[test]
    fn pane_discriminants_are_a_gap_free_range() {
        // Every variant, in declaration order — a new pane must be appended here too.
        let all = [
            Pane::Appearance,
            Pane::MenusToolbars,
            Pane::Notifications,
            Pane::SceneTypography,
            Pane::SynopsisTypography,
            Pane::NotesTypography,
            Pane::EditorBehavior,
            Pane::Goals,
            Pane::Corkboard,
            Pane::Dictionaries,
            Pane::Autosave,
            Pane::ExportFormats,
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
        ];
        for (i, pane) in all.iter().enumerate() {
            assert_eq!(
                pane.index(),
                i,
                "{} is not at slot {i}",
                pane.label().resolve_now()
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
}
