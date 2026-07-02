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
use bastyde::widgets::{
    Breadcrumb, BreadcrumbItem, Button, ButtonVariant, Center, Checkbox, ComboBox, Divider, Expand,
    FixedSize, FormLayout, GroupHeader, HStack, IconButton, IconWidget, LanguageSwitcher, MessageBox,
    MessageBoxButton, MessageBoxButtons, Padding, Panel, ScrollArea, SearchField, Slider, Spacer,
    StandardButton, StandardTreeItem, Switcher, TextScaleControl, TextWidget, ThemeSwitcher, Toggle,
    TreeView, VStack,
};

use crate::view_models::SettingsViewModel;
use crate::{
    EDITOR_WIDTH_DEFAULT, FONT_FAMILY_DEFAULT, HIGHLIGHT_SENTENCE_DEFAULT, LINE_HEIGHT_DEFAULT,
    SYNOPSIS_PANE_DEFAULT, TYPEWRITER_DEFAULT,
};

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

/// Manuscript typeface choices (data — proper names, never translated).
const FONT_FAMILIES: &[&str] = &[
    "Spectral",
    "Literata",
    "Georgia",
    "Iowan Old Style",
    "Inter",
    "System",
];

// ── Category tree model ──────────────────────────────────────────────────────

/// A selectable settings page (a tree *leaf* → its own pane). The discriminant
/// order is the `Switcher` child order, so `pane as usize` is the page index.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pane {
    Appearance = 0,
    MenusToolbars,
    Notifications,
    Manuscript,
    Goals,
    Corkboard,
    Dictionaries,
    Autosave,
    ExportFormats,
    Keymap,
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
            Pane::Manuscript => tr!(settings_page_manuscript()),
            Pane::Goals => tr!(settings_page_goals()),
            Pane::Corkboard => tr!(settings_page_corkboard()),
            Pane::Dictionaries => tr!(settings_page_dictionaries()),
            Pane::Autosave => tr!(settings_page_autosave()),
            Pane::ExportFormats => tr!(settings_page_export()),
            Pane::Keymap => tr!(settings_page_keymap()),
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
}

impl Sec {
    fn label(self) -> LocalizedString {
        match self {
            Sec::AppearanceBehaviour => tr!(settings_sec_appearance_behaviour()),
            Sec::Editor => tr!(settings_sec_editor()),
            Sec::Spelling => tr!(settings_sec_spelling()),
            Sec::BackupSync => tr!(settings_sec_backup()),
            Sec::CompileExport => tr!(settings_sec_compile()),
        }
    }

    fn icon_svg(self) -> &'static SvgIcon {
        match self {
            Sec::AppearanceBehaviour => res!("assets/icons/settings/appearance.svg"),
            Sec::Editor => res!("assets/icons/settings/editor.svg"),
            Sec::Spelling => res!("assets/icons/settings/spelling.svg"),
            Sec::BackupSync => res!("assets/icons/settings/backup.svg"),
            Sec::CompileExport => res!("assets/icons/settings/compile.svg"),
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
    let mut diffs = vec![
        theme.map(|t| t.is_dark()), // default = light
        scale.map(|s| (*s - TEXT_SCALE_DEFAULT).abs() > f32::EPSILON),
        vm.column_width().map(|w| (*w - EDITOR_WIDTH_DEFAULT).abs() > 0.01),
        vm.autosave().map(|a| *a),   // default = off
        vm.show_welcome().map(|s| !*s), // default = on
        vm.font_family().map(|f| f.as_str() != FONT_FAMILY_DEFAULT),
        vm.line_height().map(|h| (*h - LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        vm.synopsis_pane().map(|s| *s != SYNOPSIS_PANE_DEFAULT),
        vm.typewriter().map(|s| *s != TYPEWRITER_DEFAULT),
        vm.highlight_sentence().map(|s| *s != HIGHLIGHT_SENTENCE_DEFAULT),
    ];
    if let Some(loc) = locale {
        diffs.push(loc.map(|l| l.to_string() != "en-US"));
    }
    any_true(diffs)
}

// ── Small view helpers ───────────────────────────────────────────────────────

/// A left-column field label (dimmed, small) — matches the design's `--tx2`.
fn field_label(text: LocalizedString) -> TextWidget {
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
fn group(text: LocalizedString) -> GroupHeader {
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
            Expand::vertical().child(
                ScrollArea::new().child(Padding::symmetric(20.0, 24.0).child(content)),
            ),
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

pub struct SettingsPanel {
    /// The active page — a panel field so it survives rebuilds (and seeds the
    /// tree selection + the content `Switcher`). Defaults to Manuscript & Fonts.
    selected_pane: Signal<Pane>,
    root_child: Option<WidgetId>,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            selected_pane: Signal::new(Pane::Manuscript),
            root_child: None,
        }
    }

    /// Editor ▸ Manuscript & Fonts — the fully-specced design pane. Migrates the
    /// existing Text-width and theme settings; adds typeface / size / line height
    /// / synopsis / typewriter / highlight.
    fn manuscript_pane(
        ctx: &mut BuildContext,
        vm: &SettingsViewModel,
        scale: Signal<f32>,
    ) -> impl Widget {
        // Typeface: a `ComboBox<Option<String>>` bridged to the persisted
        // `Signal<String>` (which serialises cleanly). The effect mirrors
        // external changes (Reset / Cancel) back into the combo's selection.
        let persisted = vm.font_family();
        let selection: Signal<Option<String>> = Signal::new(Some(persisted.get()));
        {
            let selection = selection.clone();
            ctx.effect(&persisted, move |p: &String| {
                if selection.get().as_deref() != Some(p.as_str()) {
                    selection.set(Some(p.clone()));
                }
            });
        }
        let families: Vec<String> = FONT_FAMILIES.iter().map(|s| s.to_string()).collect();
        let write_back = persisted.clone();
        let typeface = FixedSize::new().width(240.0).child(
            ComboBox::from_items(families, selection, |f: &String| {
                let f = f.clone();
                localized(move || f.clone())
            })
            .placeholder(tr!(settings_field_typeface()))
            .on_select(move |f: &String, _ctx| write_back.set(f.clone())),
        );

        let text_scale = FixedSize::new()
            .width(300.0)
            .child(TextScaleControl::new(scale));

        let form = FormLayout::new()
            .label(tr!(settings_page_manuscript()))
            .label_gap(16.0)
            .row_spacing(14.0)
            // ── Manuscript font ──
            .full_width(group(tr!(settings_group_manuscript_font())))
            .line(field_label(tr!(settings_field_typeface())), typeface)
            .line(field_label(tr!(settings_field_size())), text_scale)
            .line(
                field_label(tr!(settings_field_line_height())),
                slider_field(vm.line_height(), 1.0, 2.4, 0.02, |v| format!("{v:.2}")),
            )
            // ── Writing column ──
            .full_width(group(tr!(settings_group_writing_column())))
            .line(
                field_label(tr!(settings_text_width())),
                slider_field(vm.column_width(), 400.0, 1200.0, 20.0, |v| {
                    format!("{} px", v.round() as i32)
                }),
            )
            .full_width(Checkbox::new(vm.synopsis_pane()).label(tr!(settings_synopsis_pane())))
            .full_width(Checkbox::new(vm.typewriter()).label(tr!(settings_typewriter())))
            .full_width(
                Checkbox::new(vm.highlight_sentence()).label(tr!(settings_highlight_sentence())),
            )
            // ── Theme ──
            .full_width(group(tr!(settings_group_theme())))
            .line(
                field_label(tr!(settings_field_editor_theme())),
                FixedSize::new()
                    .width(240.0)
                    .child(ThemeSwitcher::new().system(true)),
            );

        pane_frame(
            crumb(Some(tr!(settings_sec_editor())), tr!(settings_page_manuscript())),
            form,
        )
    }

    /// Appearance & Behaviour ▸ Appearance — migrates interface language +
    /// the "show welcome at startup" preference.
    fn appearance_pane(vm: &SettingsViewModel) -> impl Widget {
        let form = FormLayout::new()
            .label(tr!(settings_page_appearance()))
            .label_gap(16.0)
            .row_spacing(14.0)
            .full_width(group(tr!(settings_group_language())))
            .line(
                field_label(tr!(settings_field_language())),
                FixedSize::new().width(240.0).child(LanguageSwitcher::new()),
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
    fn autosave_pane(vm: &SettingsViewModel) -> impl Widget {
        let form = FormLayout::new()
            .label(tr!(settings_page_autosave()))
            .label_gap(16.0)
            .row_spacing(12.0)
            .full_width(group(tr!(settings_group_autosave())))
            .full_width(Toggle::new(vm.autosave()).label(tr!(settings_autosave())))
            .full_width(hint(tr!(settings_autosave_hint())));

        pane_frame(
            crumb(Some(tr!(settings_sec_backup())), tr!(settings_page_autosave())),
            form,
        )
    }

    /// The category tree (left rail). Builds the `TreeModel`, seeds selection to
    /// the active page, and wires selection → `selected_pane`. Returns the
    /// `TreeView`, the search's page→node map, and the model for lookups.
    fn build_tree(
        &self,
        ctx: &mut BuildContext,
    ) -> (impl Widget, KeyedSelectionModel<NodeId>, HashMap<Pane, NodeId>) {
        let model: TreeModel<Node> = TreeModel::new();
        let mut nodes: HashMap<Pane, NodeId> = HashMap::new();

        let ab = model.insert_root(0, Node::Section(Sec::AppearanceBehaviour));
        nodes.insert(Pane::Appearance, model.insert_child(ab, 0, Node::Page(Pane::Appearance)));
        nodes.insert(
            Pane::MenusToolbars,
            model.insert_child(ab, 1, Node::Page(Pane::MenusToolbars)),
        );
        nodes.insert(
            Pane::Notifications,
            model.insert_child(ab, 2, Node::Page(Pane::Notifications)),
        );

        let ed = model.insert_root(1, Node::Section(Sec::Editor));
        nodes.insert(Pane::Manuscript, model.insert_child(ed, 0, Node::Page(Pane::Manuscript)));
        nodes.insert(Pane::Goals, model.insert_child(ed, 1, Node::Page(Pane::Goals)));
        nodes.insert(Pane::Corkboard, model.insert_child(ed, 2, Node::Page(Pane::Corkboard)));

        let sp = model.insert_root(2, Node::Section(Sec::Spelling));
        nodes.insert(
            Pane::Dictionaries,
            model.insert_child(sp, 0, Node::Page(Pane::Dictionaries)),
        );

        let bk = model.insert_root(3, Node::Section(Sec::BackupSync));
        nodes.insert(Pane::Autosave, model.insert_child(bk, 0, Node::Page(Pane::Autosave)));

        let ce = model.insert_root(4, Node::Section(Sec::CompileExport));
        nodes.insert(
            Pane::ExportFormats,
            model.insert_child(ce, 0, Node::Page(Pane::ExportFormats)),
        );

        nodes.insert(Pane::Keymap, model.insert_root(5, Node::Page(Pane::Keymap)));

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
                if let Some(id) = set.iter().next().copied() {
                    if let Some(Some(pane)) = model.with_item(id, |n: &Node| n.pane()) {
                        selected_pane.set(pane);
                    }
                }
            });
        }

        // A row-body click selects (leaf → pane; section → no pane); the chevron
        // (wired via `on_toggle_rc`) expands/collapses a section. Proven config
        // (mirrors the framework's own tree examples) — reliable for both mouse
        // and synthetic input.
        let tree = TreeView::new_with_context(model, move |node: &Node, entry, selected, rowctx| {
            let mut row = StandardTreeItem::new(node.label())
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
        let index: Rc<Vec<(LocalizedString, Pane)>> = Rc::new(vec![
            (tr!(settings_page_appearance()), Pane::Appearance),
            (tr!(settings_page_menus()), Pane::MenusToolbars),
            (tr!(settings_page_notifications()), Pane::Notifications),
            (tr!(settings_page_manuscript()), Pane::Manuscript),
            (tr!(settings_page_goals()), Pane::Goals),
            (tr!(settings_page_corkboard()), Pane::Corkboard),
            (tr!(settings_page_dictionaries()), Pane::Dictionaries),
            (tr!(settings_page_autosave()), Pane::Autosave),
            (tr!(settings_page_export()), Pane::ExportFormats),
            (tr!(settings_page_keymap()), Pane::Keymap),
            (tr!(settings_field_typeface()), Pane::Manuscript),
            (tr!(settings_field_size()), Pane::Manuscript),
            (tr!(settings_field_line_height()), Pane::Manuscript),
            (tr!(settings_text_width()), Pane::Manuscript),
            (tr!(settings_synopsis_pane()), Pane::Manuscript),
            (tr!(settings_field_editor_theme()), Pane::Manuscript),
            (tr!(settings_field_language()), Pane::Appearance),
            (tr!(settings_show_welcome()), Pane::Appearance),
            (tr!(settings_autosave()), Pane::Autosave),
        ]);

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

        // ── Left rail: search + category tree ───────────────────────────────
        let (tree, selection, nodes) = self.build_tree(ctx);
        let search = self.search_field(selection, nodes);
        let left = VStack::new()
            .spacing(0.0)
            .child(Padding::symmetric(10.0, 10.0).child(search))
            .child(Expand::vertical().child(Padding::symmetric(2.0, 6.0).child(tree)));

        // ── Right pane: the per-page content behind the selection Switcher ──
        let ab = tr!(settings_sec_appearance_behaviour());
        let editor = tr!(settings_sec_editor());
        let content = Switcher::new(self.selected_pane.map(|p| p.index()))
            .child(Self::appearance_pane(&vm))
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
            .child(Self::manuscript_pane(ctx, &vm, scale.clone()))
            .child(empty_pane(
                Some(editor.clone()),
                tr!(settings_page_goals()),
                Sec::Editor.icon_svg(),
            ))
            .child(empty_pane(
                Some(editor),
                tr!(settings_page_corkboard()),
                Sec::Editor.icon_svg(),
            ))
            .child(empty_pane(
                Some(tr!(settings_sec_spelling())),
                tr!(settings_page_dictionaries()),
                Sec::Spelling.icon_svg(),
            ))
            .child(Self::autosave_pane(&vm))
            .child(empty_pane(
                Some(tr!(settings_sec_compile())),
                tr!(settings_page_export()),
                Sec::CompileExport.icon_svg(),
            ))
            .child(empty_pane(
                None,
                tr!(settings_page_keymap()),
                res!("assets/icons/settings/keymap.svg"),
            ));

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

        let root = bati!(ctx =>
            FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        Expand::horizontal { child: header }
                        Expand::horizontal { Divider }
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
        assert_eq!(Pane::Manuscript.index(), 3);
        assert_eq!(Pane::Goals.index(), 4);
        assert_eq!(Pane::Corkboard.index(), 5);
        assert_eq!(Pane::Dictionaries.index(), 6);
        assert_eq!(Pane::Autosave.index(), 7);
        assert_eq!(Pane::ExportFormats.index(), 8);
        assert_eq!(Pane::Keymap.index(), 9);
    }
}
