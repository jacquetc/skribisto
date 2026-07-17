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
use bastyde::res;
use bastyde::settings::{SettingsExt, TEXT_SCALE_KEY};
use bastyde::widgets::tooltip::TooltipContent;
use bastyde::widgets::{
    Breadcrumb, BreadcrumbItem, Button, ButtonVariant, Center, Checkbox, Divider, Expand,
    FixedSize, FontPicker, FormLayout, GroupHeader, HStack, IconButton, IconWidget,
    LanguageSwitcher, MessageBox, MessageBoxButton, MessageBoxButtons, Padding, Panel, RadioButton,
    RadioGroup, ScrollArea, SearchField, Slider, Spacer, StandardButton, StandardTreeItem, Switcher,
    TextScaleControl, TextWidget, ThemeSwitcher, Toggle, TreeView, VStack,
};

use crate::app_ids::AppIds;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::{BackupSettingsViewModel, EditorTypography, SettingsViewModel};
use crate::{
    EDITOR_WIDTH_DEFAULT, GOALS_SHOW_CHARACTERS_DEFAULT, HIGHLIGHT_SENTENCE_DEFAULT,
    NOTES_FIRST_LINE_INDENT_DEFAULT,
    NOTES_FONT_FAMILY_DEFAULT, NOTES_LINE_HEIGHT_DEFAULT, NOTES_PARA_SPACING_AFTER_DEFAULT,
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
            Pane::Backup => tr!(settings_page_backup()),
            Pane::WorkBackup => tr!(settings_page_work_backup()),
            Pane::WorkLanguage => tr!(settings_page_language()),
            Pane::WorkDictionary => tr!(settings_page_personal_dictionary()),
            Pane::Spellcheck => tr!(settings_page_spellcheck()),
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

/// A node in the category tree: a parent section or a leaf page.
#[derive(Clone, Copy)]
enum Node {
    Section(Sec),
    Page(Pane),
}

impl Node {
    fn label(self) -> LocalizedString {
        match self {
            Node::Section(s) => s.label(),
            Node::Page(p) => p.label(),
        }
    }

    /// Leading icon: every section, plus the top-level Keymap leaf (design shows
    /// no icons on the indented sub-pages).
    fn icon(self) -> Option<IconWidget> {
        let svg = match self {
            Node::Section(s) => s.icon_svg(),
            Node::Page(Pane::Keymap) => res!("assets/icons/settings/keymap.svg"),
            Node::Page(_) => return None,
        };
        Some(IconWidget::from_svg_icon(svg).icon_size(16.0))
    }

    /// The pane a leaf selects; `None` for a parent section (expands only).
    fn pane(self) -> Option<Pane> {
        match self {
            Node::Page(p) => Some(p),
            Node::Section(_) => None,
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
    let seed = fmt(value.get());
    let text = value.map(move |v| fmt(*v));
    let readout = TextWidget::new(LocalizedString::literal(seed))
        .text(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary);
    FixedSize::new().width(300.0).child(
        HStack::new()
            .spacing(12.0)
            .child(Expand::horizontal().child(Slider::new(value, min, max).step(step)))
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
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            selected_pane: Signal::new(Pane::SceneTypography),
            root_child: None,
        }
    }

    /// A `FontPicker` bound to a persisted typeface `Signal<String>` (bridged to
    /// the picker's `Option<String>` selection; the effect mirrors external
    /// changes — Reset — back into the picker). Self-populates from the shared
    /// typesetter's real font database, including the bundled writing serifs, so
    /// every offered name renders.
    fn font_picker(ctx: &mut BuildContext, persisted: Signal<String>) -> impl Widget {
        let selection: Signal<Option<String>> = Signal::new(Some(persisted.get()));
        {
            let selection = selection.clone();
            ctx.effect(&persisted, move |p: &String| {
                if selection.get().as_deref() != Some(p.as_str()) {
                    selection.set(Some(p.clone()));
                }
            });
        }
        let write_back = persisted.clone();
        FixedSize::new().width(240.0).child(
            FontPicker::new(selection)
                .placeholder(tr!(settings_field_typeface()))
                .on_select(move |f: &str, _ctx| write_back.set(f.to_string())),
        )
    }

    /// One per-editor-type typography page (Scene / Synopsis / Notes): Typeface /
    /// Size / Line height / First-line indent, bound to `typo`'s live signals.
    fn typography_pane(
        ctx: &mut BuildContext,
        page: LocalizedString,
        typo: &EditorTypography,
    ) -> impl Widget {
        let form = FormLayout::new()
            .label(page.clone())
            .label_gap(16.0)
            .row_spacing(14.0)
            .full_width(group(tr!(settings_group_typography())))
            .line(
                field_label(tr!(settings_field_typeface())),
                Self::font_picker(ctx, typo.font_family.clone()),
            )
            .line(
                field_label(tr!(settings_field_size())),
                slider_field(typo.size.clone(), 0.7, 1.6, 0.05, |v| {
                    format!("{:.0}%", v * 100.0)
                }),
            )
            .line(
                field_label(tr!(settings_field_line_height())),
                slider_field(typo.line_height.clone(), 1.0, 2.4, 0.02, |v| {
                    format!("{v:.2}")
                }),
            )
            .line(
                field_label(tr!(settings_field_first_line_indent())),
                slider_field(typo.first_line_indent.clone(), 0.0, 60.0, 2.0, |v| {
                    format!("{} px", v.round() as i32)
                }),
            )
            .line(
                field_label(tr!(settings_field_paragraph_spacing_before())),
                slider_field(typo.para_spacing_before.clone(), 0.0, 40.0, 2.0, |v| {
                    format!("{} px", v.round() as i32)
                }),
            )
            .line(
                field_label(tr!(settings_field_paragraph_spacing_after())),
                slider_field(typo.para_spacing_after.clone(), 0.0, 40.0, 2.0, |v| {
                    format!("{} px", v.round() as i32)
                }),
            );
        pane_frame(crumb(Some(tr!(settings_sec_editor())), page), form)
    }

    /// Editor ▸ Editor Behavior — the non-typographic writing settings: the
    /// centered-column width + the synopsis-pane / typewriter / highlight toggles.
    fn editor_behavior_pane(vm: &SettingsViewModel) -> impl Widget {
        let form = FormLayout::new()
            .label(tr!(settings_page_editor_behavior()))
            .label_gap(16.0)
            .row_spacing(14.0)
            .full_width(group(tr!(settings_group_writing_column())))
            .line(
                field_label(tr!(settings_text_width())),
                slider_field(vm.column_width(), 400.0, 1200.0, 20.0, |v| {
                    format!("{} px", v.round() as i32)
                }),
            )
            .line(
                field_label(tr!(settings_preview_width())),
                slider_field(vm.preview_width(), 400.0, 1200.0, 20.0, |v| {
                    format!("{} px", v.round() as i32)
                }),
            )
            .full_width(Checkbox::new(vm.synopsis_pane()).label(tr!(settings_synopsis_pane())))
            .full_width(Checkbox::new(vm.typewriter()).label(tr!(settings_typewriter())))
            .full_width(
                Checkbox::new(vm.highlight_sentence()).label(tr!(settings_highlight_sentence())),
            )
            .full_width(group(tr!(settings_group_container_views())))
            .full_width(
                Checkbox::new(vm.remember_view())
                    .label(tr!(settings_remember_view()))
                    .rich_tooltip_content(
                        TooltipContent::new(
                            "settings.remember_view",
                            tr!(settings_remember_view_tip()),
                        )
                        .with_more(tr!(settings_remember_view_tip_more())),
                    ),
            );

        pane_frame(
            crumb(
                Some(tr!(settings_sec_editor())),
                tr!(settings_page_editor_behavior()),
            ),
            form,
        )
    }

    /// Editor ▸ Goals — the word-**counting method** (a global USER preference that
    /// drives only the live status-bar count; the canonical progress snapshot always
    /// counts with `Auto`) and whether the status bar shows characters beside words.
    /// The 4-variant method is bridged to the `RadioGroup`'s `usize` selection with two
    /// guarded effects — the shape `work_structure_pane` uses for `ChapterMode`.
    fn goals_pane(ctx: &mut BuildContext, vm: &SettingsViewModel) -> impl Widget {
        let method = vm.counting_method();
        let index: Signal<usize> = Signal::new(method_to_index(method.get()));
        {
            let index = index.clone();
            ctx.effect(&method, move |m| {
                let i = method_to_index(*m);
                if index.get() != i {
                    index.set(i);
                }
            });
        }
        {
            let method = method.clone();
            ctx.effect(&index, move |i| {
                let m = index_to_method(*i);
                if method.get() != m {
                    method.set(m);
                }
            });
        }

        let form = FormLayout::new()
            .label(tr!(settings_page_goals()))
            .label_gap(16.0)
            .row_spacing(14.0)
            .full_width(group(tr!(settings_group_counting())))
            .full_width(
                RadioGroup::new()
                    .radio(RadioButton::new(0, index.clone()).label(tr!(settings_counting_auto())))
                    .radio(
                        RadioButton::new(1, index.clone())
                            .label(tr!(settings_counting_whitespace())),
                    )
                    .radio(
                        RadioButton::new(2, index.clone())
                            .label(tr!(settings_counting_unicode_words())),
                    )
                    .radio(
                        RadioButton::new(3, index.clone())
                            .label(tr!(settings_counting_cjk_hybrid())),
                    ),
            )
            .full_width(hint(tr!(settings_counting_hint())))
            .full_width(group(tr!(settings_group_goals_display())))
            .full_width(Toggle::new(vm.show_characters()).label(tr!(settings_show_characters())));

        pane_frame(
            crumb(Some(tr!(settings_sec_editor())), tr!(settings_page_goals())),
            form,
        )
    }

    /// Appearance & Behaviour ▸ Appearance — interface language, the app-wide
    /// **Theme** and **Interface text size** (both relocated here from the old
    /// Manuscript pane, where "Editor theme"/"Text size" were misnomers for
    /// app-wide controls), and the "show welcome at startup" preference.
    fn appearance_pane(vm: &SettingsViewModel, scale: Signal<f32>) -> impl Widget {
        let form = FormLayout::new()
            .label(tr!(settings_page_appearance()))
            .label_gap(16.0)
            .row_spacing(14.0)
            .full_width(group(tr!(settings_group_language())))
            .line(
                field_label(tr!(settings_field_language())),
                FixedSize::new().width(240.0).child(LanguageSwitcher::new()),
            )
            .full_width(group(tr!(settings_group_theme())))
            .line(
                field_label(tr!(settings_field_app_theme())),
                FixedSize::new()
                    .width(240.0)
                    .child(ThemeSwitcher::new().system(true)),
            )
            .line(
                field_label(tr!(settings_field_text_scale())),
                FixedSize::new()
                    .width(300.0)
                    .child(TextScaleControl::new(scale)),
            )
            .full_width(group(tr!(settings_group_startup())))
            .full_width(Checkbox::new(vm.show_welcome()).label(tr!(settings_show_welcome())));

        pane_frame(
            crumb(
                Some(tr!(settings_sec_appearance_behaviour())),
                tr!(settings_page_appearance()),
            ),
            form,
        )
    }

    /// Backup & Sync ▸ Autosave — migrates the autosave preference.
    /// Settings ▸ Spelling ▸ Spell-checking — the app-wide master switch, the same
    /// `SPELLCHECK_ENABLED_KEY` the title-bar toggle / View menu / F7 drive. A plain
    /// store-backed `Toggle`: writing the signal persists, and `App::build`'s effect turns it
    /// into `set_enabled` + a re-attach. No per-language controls here — those live on the
    /// Work's / an item's Language field (the hint says so).
    fn spellcheck_pane(vm: &SettingsViewModel) -> impl Widget {
        let form = FormLayout::new()
            .label(tr!(settings_page_spellcheck()))
            .label_gap(16.0)
            .row_spacing(12.0)
            .full_width(group(tr!(settings_group_spellcheck())))
            .full_width(
                Toggle::new(vm.spellcheck_enabled()).label(tr!(settings_spellcheck_enabled())),
            )
            .full_width(hint(tr!(settings_spellcheck_hint())));

        pane_frame(
            crumb(
                Some(tr!(settings_sec_spelling())),
                tr!(settings_page_spellcheck()),
            ),
            form,
        )
    }

    fn autosave_pane(vm: &SettingsViewModel) -> impl Widget {
        let form = FormLayout::new()
            .label(tr!(settings_page_autosave()))
            .label_gap(16.0)
            .row_spacing(12.0)
            .full_width(group(tr!(settings_group_autosave())))
            .full_width(Toggle::new(vm.autosave()).label(tr!(settings_autosave())))
            .full_width(hint(tr!(settings_autosave_hint())));

        pane_frame(
            crumb(
                Some(tr!(settings_sec_backup())),
                tr!(settings_page_autosave()),
            ),
            form,
        )
    }

    /// Work: `<name>` ▸ Structure — the per-project chapter storage mode, backed by
    /// the shared `SingleWork` (entity-backed, undoable via the Work's stack). The
    /// `Toggle` is bridged to `chapter_mode` (checked = flat) with two effects: one
    /// mirrors external changes (refresh/undo) into the toggle, the other writes +
    /// saves on a user toggle. Both guard on the current value to avoid a loop.
    fn work_structure_pane(
        ctx: &mut BuildContext,
        work: &SingleWork,
        stack: Signal<Option<u64>>,
        work_title: String,
    ) -> impl Widget {
        let mode = work.chapter_mode();
        let flat: Signal<bool> = Signal::new(matches!(mode.get(), ChapterMode::Flat));
        {
            let flat = flat.clone();
            ctx.effect(&mode, move |m| {
                let is_flat = matches!(m, ChapterMode::Flat);
                if flat.get() != is_flat {
                    flat.set(is_flat);
                }
            });
        }
        {
            let work = work.clone();
            let mode = mode.clone();
            ctx.effect(&flat, move |f| {
                let want = if *f {
                    ChapterMode::Flat
                } else {
                    ChapterMode::Folder
                };
                if mode.get() != want {
                    work.set_chapter_mode(want.clone());
                    work.save(stack.get());
                }
            });
        }

        let form = FormLayout::new()
            .label(tr!(settings_page_structure()))
            .label_gap(16.0)
            .row_spacing(12.0)
            .full_width(group(tr!(settings_group_chapters())))
            .full_width(Toggle::new(flat).label(tr!(settings_chapter_flat())))
            .full_width(hint(tr!(settings_chapter_flat_hint())));

        pane_frame(
            crumb(
                Some(lit!(format!(
                    "{}: {}",
                    tr!(settings_sec_work()).resolve_now(),
                    work_title
                ))),
                tr!(settings_page_structure()),
            ),
            form,
        )
    }

    /// Work: `<name>` ▸ Language — the project's default spell-check language(s), edited with
    /// the shared [`LanguagePillField`](crate::language_pill_field::LanguagePillField) over the
    /// live `SingleWork::dict_language` signal. Adding a language persists it and saves; a hint
    /// states the multi-language trade-off.
    fn work_language_pane(
        ctx: &mut BuildContext,
        work: &SingleWork,
        stack: Signal<Option<u64>>,
        work_title: String,
    ) -> impl Widget {
        // Build the whole form per branch so the pill field is added through FormLayout's own
        // `full_width(widget)` **deferred insertion** (which parents it to the FormLayout). The
        // earlier `ctx.add_boxed(field)` + `full_width_id` route parented the field to *this*
        // build context instead, orphaning it into an arena root — which the layout pass then
        // placed at the window origin (0,0) with the full window size, leaking a stray pill row
        // to the top-left that even survived closing Settings.
        let base = FormLayout::new()
            .label(tr!(settings_page_language()))
            .label_gap(16.0)
            .row_spacing(12.0)
            .full_width(group(tr!(settings_field_dict_language())));
        let form = match ctx.app_state::<crate::spellcheck::SpellcheckService>().cloned() {
            Some(spell) => {
                let value = work.dict_language();
                let set: crate::language_pill_field::SetLanguages = {
                    let work = work.clone();
                    Rc::new(move |new: String, _c| {
                        work.set_dict_language(new);
                        work.save(stack.get());
                    })
                };
                // The Work is the root of the inheritance chain — nothing to inherit from.
                base.full_width(crate::language_pill_field::LanguagePillField::new(
                    value, set, spell, None,
                ))
            }
            None => base.full_width(TextWidget::new(tr!(settings_field_dict_language()))),
        }
        .full_width(hint(tr!(dict_tradeoff_hint())));

        pane_frame(
            crumb(
                Some(lit!(format!(
                    "{}: {}",
                    tr!(settings_sec_work()).resolve_now(),
                    work_title
                ))),
                tr!(settings_page_language()),
            ),
            form,
        )
    }

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
        nodes.insert(
            Pane::SceneTypography,
            model.insert_child(ed, 0, Node::Page(Pane::SceneTypography)),
        );
        nodes.insert(
            Pane::SynopsisTypography,
            model.insert_child(ed, 1, Node::Page(Pane::SynopsisTypography)),
        );
        nodes.insert(
            Pane::NotesTypography,
            model.insert_child(ed, 2, Node::Page(Pane::NotesTypography)),
        );
        nodes.insert(
            Pane::EditorBehavior,
            model.insert_child(ed, 3, Node::Page(Pane::EditorBehavior)),
        );
        nodes.insert(
            Pane::Goals,
            model.insert_child(ed, 4, Node::Page(Pane::Goals)),
        );
        nodes.insert(
            Pane::Corkboard,
            model.insert_child(ed, 5, Node::Page(Pane::Corkboard)),
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
        // Structure (the chapter mode). Read through the shared `SingleWork`.
        let work_title = ctx
            .app_state::<SingleWork>()
            .filter(|w| w.id().is_some())
            .map(|w| w.title().get());
        let mut work_node: Option<NodeId> = None;
        if work_title.is_some() {
            let wk = model.insert_root(6, Node::Section(Sec::Work));
            nodes.insert(
                Pane::WorkStructure,
                model.insert_child(wk, 0, Node::Page(Pane::WorkStructure)),
            );
            nodes.insert(
                Pane::WorkLanguage,
                model.insert_child(wk, 1, Node::Page(Pane::WorkLanguage)),
            );
            nodes.insert(
                Pane::WorkBackup,
                model.insert_child(wk, 2, Node::Page(Pane::WorkBackup)),
            );
            nodes.insert(
                Pane::WorkDictionary,
                model.insert_child(wk, 3, Node::Page(Pane::WorkDictionary)),
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
        tree.expand(ab);
        tree.expand(ed);
        tree.collapse(sp);
        tree.collapse(bk);
        tree.collapse(ce);
        if let Some(wk) = work_node {
            tree.expand(wk);
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
            (tr!(settings_page_dictionaries()), Pane::Dictionaries),
            (tr!(settings_page_autosave()), Pane::Autosave),
            (tr!(settings_page_backup()), Pane::Backup),
            (tr!(settings_page_export()), Pane::ExportFormats),
            (tr!(settings_page_keymap()), Pane::Keymap),
            (tr!(settings_text_width()), Pane::EditorBehavior),
            (tr!(settings_synopsis_pane()), Pane::EditorBehavior),
            (tr!(settings_field_app_theme()), Pane::Appearance),
            (tr!(settings_field_text_scale()), Pane::Appearance),
            (tr!(settings_field_language()), Pane::Appearance),
            (tr!(settings_show_welcome()), Pane::Appearance),
            (tr!(settings_autosave()), Pane::Autosave),
        ];
        // Typeface / Size / Line height / First-line indent repeat on all three
        // typography pages, so disambiguate each by page ("Scene — Typeface") —
        // the index dedupes by resolved text, so bare labels would collide and
        // leave two of three pages unreachable.
        for (page, pane) in [
            (tr!(settings_page_scene()), Pane::SceneTypography),
            (tr!(settings_page_synopsis()), Pane::SynopsisTypography),
            (tr!(settings_page_notes()), Pane::NotesTypography),
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

        // The open project (shared handle) backs the "Work: `<name>` ▸ Structure"
        // page. When no project is open, the page is present in the Switcher but
        // its tree node isn't shown, so it renders an empty placeholder.
        let work = ctx.app_state::<SingleWork>().cloned();
        let stack = ctx
            .app_state::<AppIds>()
            .map(|i| i.stack_id.clone())
            .unwrap_or_else(|| Signal::new(None));
        let work_title = work.as_ref().map(|w| w.title().get()).unwrap_or_default();
        let structure_pane: Box<dyn Widget> = match &work {
            Some(w) => Box::new(Self::work_structure_pane(
                ctx,
                w,
                stack.clone(),
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_structure()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let language_pane: Box<dyn Widget> = match &work {
            Some(w) => Box::new(Self::work_language_pane(
                ctx,
                w,
                stack.clone(),
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_language()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        // Spelling ▸ Dictionaries — the management pane (Installed / Get more), wrapped in the
        // shared `pane_frame` like every other pane. Always available (dictionaries are a
        // machine-wide resource, independent of any open project).
        let dictionaries_pane: Box<dyn Widget> =
            match ctx.app_state::<crate::view_models::DictionariesViewModel>().cloned() {
                Some(vm) => Box::new(pane_frame(
                    crumb(Some(tr!(settings_sec_spelling())), tr!(settings_page_dictionaries())),
                    crate::settings_dictionaries::dictionaries_pane(ctx, &vm),
                )),
                None => Box::new(empty_pane(
                    Some(tr!(settings_sec_spelling())),
                    tr!(settings_page_dictionaries()),
                    Sec::Spelling.icon_svg(),
                )),
            };
        // Compile & Export ▸ Export Formats — the export-style manager (built-in + user styles,
        // duplicate-to-edit, JSON import/export), wrapped in `pane_frame` like every other pane.
        let export_styles_pane: Box<dyn Widget> =
            match ctx.app_state::<crate::view_models::ExportStylesViewModel>().cloned() {
                Some(vm) => Box::new(pane_frame(
                    crumb(Some(tr!(settings_sec_compile())), tr!(settings_page_export())),
                    crate::settings_export_styles::export_styles_pane(ctx, &vm),
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
                crate::settings_backup::general_pane(ctx, vm),
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
                let path = ctx
                    .app_state::<SingleWorkInfo>()
                    .and_then(|wi| wi.file_name().get())
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
                    crate::settings_backup::work_backup_pane(ctx, vm, uid, path, title),
                ))
            }
            _ => Box::new(empty_pane(
                None,
                tr!(settings_page_work_backup()),
                res!("assets/icons/binder/book.svg"),
            )),
        };

        // Work ▸ Personal dictionary — the per-project word-list manager, over the
        // shared `UserDictionaryViewModel`. Present in the Switcher regardless, an
        // empty placeholder when no project is open (same as the other Work panes).
        let dictionary_pane: Box<dyn Widget> = match (
            ctx.app_state::<crate::view_models::UserDictionaryViewModel>().cloned(),
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
                    crate::settings_user_dictionary::user_dictionary_pane(ctx, &vm),
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
        // Child order MUST equal the `Pane` discriminant order (guarded by
        // `pane_indices_match_switcher_order`).
        let ab = tr!(settings_sec_appearance_behaviour());
        let editor = tr!(settings_sec_editor());
        let typo = vm.editor_typography();
        let content = Switcher::new(self.selected_pane.map(|p| p.index()))
            .child(Self::appearance_pane(&vm, scale.clone()))
            .child(empty_pane(
                Some(ab.clone()),
                tr!(settings_page_menus()),
                Sec::AppearanceBehaviour.icon_svg(),
            ))
            .child(empty_pane(
                Some(ab),
                tr!(settings_page_notifications()),
                Sec::AppearanceBehaviour.icon_svg(),
            ))
            .child(Self::typography_pane(
                ctx,
                tr!(settings_page_scene()),
                &typo.scene,
            ))
            .child(Self::typography_pane(
                ctx,
                tr!(settings_page_synopsis()),
                &typo.synopsis,
            ))
            .child(Self::typography_pane(
                ctx,
                tr!(settings_page_notes()),
                &typo.notes,
            ))
            .child(Self::editor_behavior_pane(&vm))
            .child(Self::goals_pane(ctx, &vm))
            .child(empty_pane(
                Some(editor),
                tr!(settings_page_corkboard()),
                Sec::Editor.icon_svg(),
            ))
            .child(Self::spellcheck_pane(&vm))
            .child_boxed(dictionaries_pane)
            .child(Self::autosave_pane(&vm))
            .child_boxed(export_styles_pane)
            .child(empty_pane(
                None,
                tr!(settings_page_keymap()),
                res!("assets/icons/settings/keymap.svg"),
            ))
            .child_boxed(structure_pane)
            .child_boxed(backup_pane)
            .child_boxed(work_backup_pane)
            .child_boxed(language_pane)
            .child_boxed(dictionary_pane);

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

    /// Pane discriminants are the `Switcher` child order — guard the mapping so a
    /// reordering can't silently desync the tree selection from the shown pane.
    /// Also fixes the ten-page contract the search index + tree rely on.
    #[test]
    fn pane_indices_match_switcher_order() {
        assert_eq!(Pane::Appearance.index(), 0);
        assert_eq!(Pane::MenusToolbars.index(), 1);
        assert_eq!(Pane::Notifications.index(), 2);
        assert_eq!(Pane::SceneTypography.index(), 3);
        assert_eq!(Pane::SynopsisTypography.index(), 4);
        assert_eq!(Pane::NotesTypography.index(), 5);
        assert_eq!(Pane::EditorBehavior.index(), 6);
        assert_eq!(Pane::Goals.index(), 7);
        assert_eq!(Pane::Corkboard.index(), 8);
        assert_eq!(Pane::Dictionaries.index(), 9);
        assert_eq!(Pane::Autosave.index(), 10);
        assert_eq!(Pane::ExportFormats.index(), 11);
        assert_eq!(Pane::Keymap.index(), 12);
        assert_eq!(Pane::WorkStructure.index(), 13);
        assert_eq!(Pane::Backup.index(), 14);
        assert_eq!(Pane::WorkBackup.index(), 15);
        assert_eq!(Pane::WorkLanguage.index(), 16);
        assert_eq!(Pane::WorkDictionary.index(), 17);
        assert_eq!(Pane::Spellcheck.index(), 18);
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
