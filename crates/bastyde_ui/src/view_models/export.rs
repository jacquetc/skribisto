// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ExportViewModel` — the Export feature's business logic.
//!
//! Single-instance live state, created once in `main.rs` and registered as app-state so the
//! title-bar menu + split-button (built outside `App`) and `App::build`'s wiring can all
//! reach the one instance. It owns:
//!
//! - the **focus-adaptive scope list** (`applicable`) that drives *both* the title-bar
//!   Export split-button and the File ▸ Export submenu — one source, two surfaces. `App`
//!   recomputes it whenever the focused editor item changes;
//! - the **panel state** (chosen scope + anchor, format, style, output path) the Export
//!   modal binds;
//! - the in-flight export **long operation** id, whose `Origin::LongOperation(...)` events
//!   `App::build` routes here to drive the progress / success / error toast.
//!
//! The **live preview** is rendered *client-side* by [`skribisto_compiler`] over a
//! `Gathered` tree assembled here from the frontend read commands — the exact same compile
//! path the backend `export_work` long op runs, so the preview and the committed file cannot
//! diverge (given the same store state). The panel is modal, so no prose edit can slip in
//! between opening it (which flushes the editors) and exporting.

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use bastyde::prelude::*; // EventContext, Signal, tr!, lit!
use bastyde::text_document::TextDocument;
use bastyde::widgets::{MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction};

use export_management::{ExportFormat, ExportScopeKind, ExportWorkDto};
use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, content_commands, export_management_commands,
    long_operation_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{Binder, BinderItem, Content, Work};
use frontend::common::event::Event;

use skrib_format::{BinderWithItems, Gathered, ItemWithContents};
use skribisto_compiler::{
    ExportFormat as CFormat, HeadingScheme, LineSpacing, Preset, RenderRequest, SceneBreak,
    builtin_presets, render_preview_document,
};
use skribisto_model::compile::{
    ItemMeta, ScopeKind, StreamLevel, enclosing_head, primary_scope, resolve_scope,
};

use super::long_op::{CapturedWork, event_id, parse_payload, payload_id};
use crate::app_ids::AppIds;
use crate::export::choose::ChooseModel;
use crate::toast_scope::ToastWorkExt;

/// Update-in-place key for the single toast an export drives (loading → progress →
/// success / cancelled / error) — folded through [`work_scoped_toast_id`] with
/// [`ExportViewModel::active_work_id`] at every use, never bare: see that
/// field's doc (and `long_op::CapturedWork`'s) for why a bare static id would
/// let a second Work's export silently collide with this one's still-in-flight
/// toast.
const EXPORT_TOAST_ID: &str = "export.work";

/// The output formats the panel offers — every one has a complete renderer in
/// `skribisto_compiler` (DOCX manuscript typography M5; EPUB 3 M6; PDF via Typst M7). PDF is
/// **only offered when the app was built with the `pdf` feature** — a build without it can't
/// produce a PDF and would only surface a build-config error, so the option is omitted entirely
/// rather than shown as a format that always fails.
#[cfg(feature = "pdf")]
const PANEL_FORMATS: [ExportFormat; 8] = [
    ExportFormat::Docx,
    ExportFormat::Pdf,
    ExportFormat::Epub,
    ExportFormat::Html,
    ExportFormat::Markdown,
    ExportFormat::Djot,
    ExportFormat::PlainText,
    ExportFormat::Latex,
];
#[cfg(not(feature = "pdf"))]
const PANEL_FORMATS: [ExportFormat; 7] = [
    ExportFormat::Docx,
    ExportFormat::Epub,
    ExportFormat::Html,
    ExportFormat::Markdown,
    ExportFormat::Djot,
    ExportFormat::PlainText,
    ExportFormat::Latex,
];

/// The conventional file extension (no dot) for a panel format.
fn extension_of(f: &ExportFormat) -> &'static str {
    match f {
        ExportFormat::Djot => "dj",
        ExportFormat::PlainText => "txt",
        ExportFormat::Markdown => "md",
        ExportFormat::Html => "html",
        ExportFormat::Latex => "tex",
        ExportFormat::Docx => "docx",
        ExportFormat::Epub => "epub",
        ExportFormat::Pdf => "pdf",
    }
}

/// The `skribisto_model` scope kind for a DTO scope kind.
fn to_scope_kind(s: &ExportScopeKind) -> ScopeKind {
    match s {
        ExportScopeKind::CurrentBook => ScopeKind::Book,
        ExportScopeKind::CurrentPart => ScopeKind::Part,
        ExportScopeKind::CurrentChapter => ScopeKind::Chapter,
        ExportScopeKind::CurrentScene => ScopeKind::Scene,
        ExportScopeKind::CurrentNote => ScopeKind::Note,
        ExportScopeKind::CurrentFolder => ScopeKind::Folder,
        ExportScopeKind::Custom => ScopeKind::Custom,
    }
}

/// The DTO scope kind for a resolved `skribisto_model` scope kind.
fn from_scope_kind(s: ScopeKind) -> ExportScopeKind {
    match s {
        ScopeKind::Book => ExportScopeKind::CurrentBook,
        ScopeKind::Part => ExportScopeKind::CurrentPart,
        ScopeKind::Chapter => ExportScopeKind::CurrentChapter,
        ScopeKind::Scene => ExportScopeKind::CurrentScene,
        ScopeKind::Note => ExportScopeKind::CurrentNote,
        ScopeKind::Folder => ExportScopeKind::CurrentFolder,
        ScopeKind::Custom => ExportScopeKind::Custom,
    }
}

/// The localized "Export …" label for a quick scope (split-button primary + menu item).
pub fn scope_label(scope: &ExportScopeKind) -> bastyde::i18n::LocalizedString {
    match scope {
        ExportScopeKind::CurrentBook => tr!(menu_export_book()),
        ExportScopeKind::CurrentPart => tr!(menu_export_part()),
        ExportScopeKind::CurrentChapter => tr!(menu_export_chapter()),
        ExportScopeKind::CurrentScene => tr!(menu_export_scene()),
        ExportScopeKind::CurrentNote => tr!(menu_export_note()),
        ExportScopeKind::CurrentFolder => tr!(menu_export_folder()),
        ExportScopeKind::Custom => tr!(menu_export_choose()),
    }
}

/// The localized name of a panel output format (for the format `SegmentedControl`).
pub fn format_label(f: &ExportFormat) -> bastyde::i18n::LocalizedString {
    match f {
        ExportFormat::Docx => tr!(export_format_docx()),
        ExportFormat::Html => tr!(export_format_html()),
        ExportFormat::Markdown => tr!(export_format_markdown()),
        ExportFormat::Djot => tr!(export_format_djot()),
        ExportFormat::PlainText => tr!(export_format_text()),
        ExportFormat::Latex => tr!(export_format_latex()),
        ExportFormat::Epub => tr!(export_format_epub()),
        ExportFormat::Pdf => tr!(export_format_pdf()),
    }
}

#[derive(Clone)]
pub struct ExportViewModel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// The focus-adaptive quick scopes for the current selection, primary facet first
    /// (drives the split-button order); empty when nothing exportable is focused.
    applicable: Signal<Vec<ExportScopeKind>>,
    /// The scope the panel is currently exporting.
    scope: Signal<ExportScopeKind>,
    /// The quick (non-`Custom`) scope this panel session may switch back to via the "What to
    /// export" segmented control — `Some` when a focusable anchor resolved one, else `None`
    /// (opened from Choose… with nothing focused, so only Custom is offered).
    quick_scope: Signal<Option<ExportScopeKind>>,
    /// The scope segmented control's selection: 0 = the quick scope, 1 = Custom selection.
    /// An effect maps a change here onto [`scope`] via [`apply_segment`].
    segment_index: Signal<usize>,
    /// The focused item the quick scope resolves from (the backend re-resolves the extent
    /// against its frozen snapshot from this anchor).
    anchor: Signal<Option<u64>>,
    /// Index into [`PANEL_FORMATS`] — the chosen output format (remembered across opens),
    /// bound by the format `RadioTileGroup`.
    format_index: Signal<usize>,
    /// The chosen style; `None` until first opened, then the previous choice.
    preset: Signal<Option<Preset>>,
    /// The destination file path.
    output_path: Signal<String>,
    /// The in-flight export op id (set on start, cleared on completion / cancel / failure).
    active: Signal<Option<String>>,
    /// This window's own Work, captured the instant [`Self::run_export`]
    /// starts the long operation — set alongside `active`, cleared alongside
    /// it. See `long_op::CapturedWork`'s doc for why every handler below must
    /// route and scope its toast on THIS field, never a live
    /// `self.ids.work_id.get()`.
    active_work_id: Signal<CapturedWork>,

    // ── Choose… (Custom scope) state ─────────────────────────────────────────
    /// Whether the Choose tree reveals items marked non-exportable (default off).
    show_non_exportable: Signal<bool>,
    /// The current Choose tree + check model (built lazily from the store on a Custom open).
    choose: Rc<RefCell<Option<ChooseModel>>>,
    /// The `show_non_exportable` value the current `choose` model was built for — so
    /// `ensure_choose` rebuilds only when the toggle actually flipped.
    choose_show: Rc<Cell<bool>>,
    /// Bumped on any check change; stable across rebuilds so the preview binding survives a
    /// "show non-exportable" toggle.
    custom_changed: Signal<u64>,
}

#[allow(dead_code)]
impl ExportViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            app_ctx,
            ids,
            applicable: Signal::new(Vec::new()),
            scope: Signal::new(ExportScopeKind::CurrentBook),
            quick_scope: Signal::new(None),
            segment_index: Signal::new(0),
            anchor: Signal::new(None),
            format_index: Signal::new(0),
            // Seed the first built-in style so the picker shows a real selection (and the
            // preview renders) from the first open, rather than an empty placeholder.
            preset: Signal::new(builtin_presets().into_iter().next()),
            output_path: Signal::new(String::new()),
            active: Signal::new(None),
            active_work_id: Signal::new(CapturedWork::none()),
            show_non_exportable: Signal::new(false),
            choose: Rc::new(RefCell::new(None)),
            choose_show: Rc::new(Cell::new(false)),
            custom_changed: Signal::new(0),
        }
    }

    // ── Focus-adaptive scope list (split-button + File ▸ Export) ─────────────

    /// The applicable quick scopes, primary facet first. Bound at `BindingLevel::Rebuild`
    /// by the split-button and per-item by the menu.
    pub fn applicable_signal(&self) -> Signal<Vec<ExportScopeKind>> {
        self.applicable.clone()
    }

    /// Recompute the applicable scopes for a newly-focused item. Called by `App`'s effect
    /// on the editors' active-item signal (and once on load). Reads the live binder tree —
    /// cheap, and only ever off a focus change.
    pub fn recompute_applicable(&self, active: Option<u64>) {
        let next = self.compute_applicable(active);
        if self.applicable.get() != next {
            self.applicable.set(next);
        }
    }

    fn compute_applicable(&self, active: Option<u64>) -> Vec<ExportScopeKind> {
        let mut out: Vec<ExportScopeKind> = Vec::new();
        // The focus-driven quick scopes (skipped entirely when nothing is focused).
        if let Some(active) = active
            && let Ok(g) = self.client_gather()
        {
            let metas = skribisto_compiler::item_metas(&g);
            if let Some(pos) = metas.iter().position(|m| m.id == active) {
                let item = &metas[pos];
                // The focused item's own facet — enabled only when it actually resolves.
                if let Some(primary) = primary_scope(&item.role, &item.sub_role)
                    && resolve_scope(&metas, pos, primary).is_some()
                {
                    out.push(from_scope_kind(primary));
                }
                // Then the enclosing structural containers, coarser outward.
                for (level, sk) in [
                    (StreamLevel::Chapter, ScopeKind::Chapter),
                    (StreamLevel::Part, ScopeKind::Part),
                    (StreamLevel::Book, ScopeKind::Book),
                ] {
                    let es = from_scope_kind(sk);
                    if !out.contains(&es)
                        && enclosing_head(&metas, pos, level).is_some()
                        && resolve_scope(&metas, pos, sk).is_some()
                    {
                        out.push(es);
                    }
                }
            }
        }
        // Choose… is always available once a project is open — it exports whatever the
        // checkbox tree selects, independent of focus.
        if self.ids.work_id.get().is_some() {
            out.push(ExportScopeKind::Custom);
        }
        out
    }

    // ── Panel preparation ────────────────────────────────────────────────────

    /// Set up the panel for a quick scope anchored at `anchor` (the focused item), defaulting
    /// the output path from the project's location + title + the current format's extension.
    /// The caller (`App`'s `export.scope` action) flushes the editors first, so the preview
    /// and the committed export both see current prose.
    pub fn prepare(&self, scope: ExportScopeKind, anchor: Option<u64>) {
        let is_custom = scope == ExportScopeKind::Custom;
        // The quick scope the "What to export" toggle can switch back to: the opening scope
        // when it isn't Custom, otherwise the focused item's own primary facet (if any), so a
        // panel opened via Choose… on a focused scene still offers "the current scene" beside
        // "Custom selection".
        let quick = if is_custom {
            anchor.and_then(|a| {
                self.compute_applicable(Some(a))
                    .into_iter()
                    .find(|s| *s != ExportScopeKind::Custom)
            })
        } else {
            Some(scope.clone())
        };
        self.quick_scope.set(quick);
        self.segment_index.set(if is_custom { 1 } else { 0 });
        self.scope.set(scope);
        self.anchor.set(anchor);
        self.output_path.set(self.default_output_path());
        // A fresh Choose session each open: drop any prior tree so `ensure_choose` rebuilds
        // from the current store (with the default seed).
        if is_custom {
            self.choose.replace(None);
            self.ensure_choose();
        }
    }

    /// The quick (non-`Custom`) scope this session may switch to, if any — the label for the
    /// first segment of the "What to export" control.
    pub fn quick_scope(&self) -> Option<ExportScopeKind> {
        self.quick_scope.get()
    }

    /// The scope segmented control's selection signal (0 = quick scope, 1 = Custom).
    pub fn segment_index(&self) -> Signal<usize> {
        self.segment_index.clone()
    }

    /// The active scope signal — the leading column, preview, and `can_export` bind this so
    /// flipping the segmented control re-renders them.
    pub fn scope_signal(&self) -> Signal<ExportScopeKind> {
        self.scope.clone()
    }

    /// Fold the segmented control's index onto the active scope. Called from a panel effect on
    /// `segment_index`: index 0 restores the quick scope (a no-op when none exists), index 1
    /// switches to Custom and lazily builds the checkbox tree.
    pub fn apply_segment(&self) {
        let next = if self.segment_index.get() == 0 {
            self.quick_scope.get()
        } else {
            Some(ExportScopeKind::Custom)
        };
        let Some(next) = next else { return };
        if self.scope.get() != next {
            self.scope.set(next.clone());
        }
        if next == ExportScopeKind::Custom {
            self.ensure_choose();
        }
    }

    // ── Choose… (Custom scope) ───────────────────────────────────────────────

    /// The reveal toggle bound by the panel's "Show non-exportable" checkbox.
    pub fn show_non_exportable(&self) -> Signal<bool> {
        self.show_non_exportable.clone()
    }

    /// Bumped on any check change — the preview binds this so it refreshes as the user checks.
    pub fn custom_changed(&self) -> Signal<u64> {
        self.custom_changed.clone()
    }

    /// Ensure the Choose tree is built for the current `show_non_exportable` value, rebuilding
    /// (and preserving the user's checks) only when it flipped or the tree is absent. Called
    /// from the panel's Choose pane build.
    pub fn ensure_choose(&self) {
        let want_show = self.show_non_exportable.get();
        let need = self.choose.borrow().is_none() || self.choose_show.get() != want_show;
        if !need {
            return;
        }
        let prev = self.choose.borrow().as_ref().map(|m| m.checked_item_ids());
        let Ok(g) = self.client_gather() else {
            self.choose.replace(None);
            return;
        };
        let model = ChooseModel::build(&g, want_show, self.custom_changed.clone());
        if let Some(prev_ids) = prev {
            model.apply_checked(&prev_ids);
        }
        self.choose_show.set(want_show);
        self.choose.replace(Some(model));
    }

    /// The current Choose model (a clone of the `Rc`-backed handles), if built.
    pub fn choose_model(&self) -> Option<ChooseModel> {
        self.choose.borrow().clone()
    }

    /// The checked item ids for the `Custom` scope's include set.
    pub fn checked_item_ids(&self) -> Vec<u64> {
        self.choose
            .borrow()
            .as_ref()
            .map(|m| m.checked_item_ids())
            .unwrap_or_default()
    }

    /// The number of checked items in the Choose tree — the Choose footer's "{n} selected"
    /// count. Read by a small reactive label that binds `custom_changed` to refresh it.
    pub fn checked_count(&self) -> usize {
        self.choose
            .borrow()
            .as_ref()
            .map(|m| m.checked_item_ids().len())
            .unwrap_or(0)
    }

    fn default_output_path(&self) -> String {
        let stem = crate::project_stem(&self.app_ctx, &self.ids);
        let ext = extension_of(&self.selected_format());
        let name = format!("{stem}.{ext}");
        match self.default_dir() {
            Some(dir) => dir.join(&name).to_string_lossy().into_owned(),
            None => name,
        }
    }

    /// The folder the project lives in (its *sibling*, so an export doesn't land inside a
    /// folder-shaped project). `None` when no project path is known.
    fn default_dir(&self) -> Option<PathBuf> {
        let cur = crate::current_project_path(&self.app_ctx, &self.ids)?;
        let p = Path::new(&cur);
        // A folder work's entry is `…/<Project>/project.skrib`; a zip work's is `…/X.skrib`.
        let base = if p.file_name().and_then(|n| n.to_str()) == Some("project.skrib") {
            p.parent()?
        } else {
            p
        };
        base.parent().map(Path::to_path_buf)
    }

    // ── Panel accessors (bound by the view) ──────────────────────────────────

    pub fn scope(&self) -> ExportScopeKind {
        self.scope.get()
    }
    /// The chosen format, bound by the panel's format `ComboBox`.
    /// The chosen format index, bound by the panel's format `RadioTileGroup`.
    pub fn format_index(&self) -> Signal<usize> {
        self.format_index.clone()
    }
    pub fn preset_signal(&self) -> Signal<Option<Preset>> {
        self.preset.clone()
    }
    pub fn output_path(&self) -> Signal<String> {
        self.output_path.clone()
    }
    pub fn set_output_path(&self, path: String) {
        self.output_path.set(path);
    }

    /// The styles the picker offers. Built-ins for now; M4 unions the user's styles in.
    pub fn presets(&self) -> Vec<Preset> {
        builtin_presets()
    }

    /// A short, read-only summary of the selected style's structural choices — the chips
    /// shown under the style picker (chapters · scene break · notes · spacing). Localized,
    /// so the panel binds `preset_signal` and rebuilds these on a style change.
    pub fn preset_chips(&self) -> Vec<bastyde::i18n::LocalizedString> {
        let p = self.selected_preset();
        vec![
            match p.chapter_heading {
                HeadingScheme::None => tr!(export_chip_chapters_none()),
                HeadingScheme::Numbered => tr!(export_chip_chapters_numbered()),
                HeadingScheme::TitleOnly => tr!(export_chip_chapters_title()),
                HeadingScheme::NumberAndTitle => tr!(export_chip_chapters_both()),
            },
            match &p.scene_break {
                SceneBreak::Glyph(g) => tr!(export_chip_scene_break_glyph(glyph = g.clone())),
                SceneBreak::BlankLine => tr!(export_chip_scene_break_blank()),
                SceneBreak::None => tr!(export_chip_scene_break_none()),
            },
            // The major tier only earns its own chip when it actually differs —
            // several regional styles (Japanese print, for one) render both the
            // same, and a duplicate chip would read as a mistake.
            if p.major_scene_break == p.scene_break {
                tr!(export_chip_major_break_same())
            } else {
                match &p.major_scene_break {
                    SceneBreak::Glyph(g) => tr!(export_chip_major_break_glyph(glyph = g.clone())),
                    SceneBreak::BlankLine => tr!(export_chip_major_break_blank()),
                    SceneBreak::None => tr!(export_chip_major_break_none()),
                }
            },
            if p.include_notes {
                tr!(export_chip_notes_included())
            } else {
                tr!(export_chip_notes_excluded())
            },
            match p.line_spacing {
                LineSpacing::Single => tr!(export_chip_spacing_single()),
                LineSpacing::OneAndHalf => tr!(export_chip_spacing_onehalf()),
                LineSpacing::Double => tr!(export_chip_spacing_double()),
            },
        ]
    }

    /// The formats the picker offers (one `RadioTile` each).
    pub fn panel_formats() -> &'static [ExportFormat] {
        &PANEL_FORMATS
    }

    fn selected_format(&self) -> ExportFormat {
        PANEL_FORMATS[self.format_index.get().min(PANEL_FORMATS.len() - 1)].clone()
    }

    /// The localized name of the currently-chosen output format — the preview header's
    /// "compiled · <format> · …" subtitle (rebuilt on a `format_index` change).
    pub fn current_format_label(&self) -> bastyde::i18n::LocalizedString {
        format_label(&self.selected_format())
    }

    /// The name of the currently-chosen style — the preview subtitle's trailing segment
    /// (data, so it stays `lit!`; rebuilt on a `preset_signal` change).
    pub fn current_preset_name(&self) -> String {
        self.selected_preset().name
    }
    fn selected_preset(&self) -> Preset {
        self.preset.get().unwrap_or_else(|| {
            builtin_presets()
                .into_iter()
                .next()
                .expect("a built-in style")
        })
    }

    /// The extension for the currently-chosen format — the panel keeps the output path's
    /// extension in step when the format changes.
    pub fn retarget_extension(&self) {
        let ext = extension_of(&self.selected_format());
        let current = self.output_path.get();
        if current.trim().is_empty() {
            self.output_path.set(self.default_output_path());
            return;
        }
        let p = Path::new(&current);
        let with_ext = p.with_extension(ext);
        self.output_path
            .set(with_ext.to_string_lossy().into_owned());
    }

    /// Whether "Export" may fire: a non-blank destination and something to export — a
    /// resolvable anchor for a quick scope, or at least one checked item for Choose…
    /// (reactive on the checkbox tree via `custom_changed`).
    pub fn can_export(&self) -> Signal<bool> {
        let path_ok = self.output_path.map(|p| !p.trim().is_empty());
        let choose = self.choose.clone();
        // Recompute when the checks change, the anchor changes, or the scope is switched via
        // the segmented control — so flipping quick ↔ Custom re-evaluates the Export button.
        let sel_ok = self.custom_changed.zip(&self.anchor).zip(&self.scope).map(
            move |((_, anchor), scope)| {
                if *scope == ExportScopeKind::Custom {
                    choose
                        .borrow()
                        .as_ref()
                        .map(|m| !m.checked_item_ids().is_empty())
                        .unwrap_or(false)
                } else {
                    anchor.is_some()
                }
            },
        );
        path_ok.and(&sel_ok)
    }

    /// An explicit single-item / Choose… selection keeps note items regardless of the
    /// preset's `include_notes` (mirrors the backend's rule).
    fn is_explicit(&self) -> bool {
        matches!(
            self.scope.get(),
            ExportScopeKind::CurrentScene | ExportScopeKind::CurrentNote | ExportScopeKind::Custom
        )
    }

    // ── Live preview ─────────────────────────────────────────────────────────

    /// The compiled document for the live preview — the exact `TextDocument` the chosen
    /// style + scope would render, shown read-only in the panel. `None` when nothing
    /// resolves (no work / empty scope) so the panel shows its empty state.
    pub fn preview_document(&self) -> Option<TextDocument> {
        let g = self.client_gather().ok()?;
        let metas = skribisto_compiler::item_metas(&g);
        let include = self.resolve_include(&metas)?;
        if include.is_empty() {
            return None;
        }
        let preset = self.selected_preset();
        let work_lang = g.work.dict_language.clone();
        let req = RenderRequest {
            gathered: &g,
            include: &include,
            preset: &preset,
            // The preview shows the assembled document; the *format* only matters at write
            // time, so any value works here.
            format: CFormat::Html,
            work_lang: skribisto_model::language::primary(&work_lang),
            explicit_selection: self.is_explicit(),
        };
        render_preview_document(&req).ok()
    }

    /// The ordered include ids for the current scope + anchor, resolved against the live
    /// tree exactly as the backend resolves them against its frozen one.
    fn resolve_include(&self, metas: &[ItemMeta]) -> Option<Vec<u64>> {
        // Choose… supplies its ids directly from the checkbox tree.
        if self.scope.get() == ExportScopeKind::Custom {
            let ids = self.checked_item_ids();
            return (!ids.is_empty()).then_some(ids);
        }
        let anchor = self.anchor.get()?;
        let pos = metas.iter().position(|m| m.id == anchor)?;
        resolve_scope(metas, pos, to_scope_kind(&self.scope.get()))
    }

    /// Read the open Work subtree into a `Gathered` from the frontend read commands,
    /// converting each DTO to its entity. The client analogue of the backend's frozen
    /// `gather` — best-effort (an empty store yields "no open work", and the preview then
    /// shows its empty state).
    ///
    /// Resolved through `self.ids.work_id` (the Phase-1 seam), not
    /// `get_all_work(ctx)`'s first entry — see `dto`'s identical resolution just
    /// below for the same reasoning.
    fn client_gather(&self) -> anyhow::Result<Gathered> {
        let ctx = &self.app_ctx;
        let work_id = self.ids.work_id.get().ok_or_else(|| anyhow::anyhow!("no open work"))?;
        let work_dto =
            work_commands::get_work(ctx, &work_id)?.ok_or_else(|| anyhow::anyhow!("no open work"))?;
        let work: Work = work_dto.into();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work.id, &WorkRelationshipField::Binders)?;
        let binder_dtos = binder_commands::get_binder_multi(ctx, &binder_ids)?;
        let mut binders = Vec::new();
        for binder_dto in binder_dtos.into_iter().flatten() {
            let binder: Binder = binder_dto.into();
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder.id,
                &BinderRelationshipField::BinderItems,
            )?;
            let item_dtos = binder_item_commands::get_binder_item_multi(ctx, &item_ids)?;
            let mut items = Vec::new();
            for item_dto in item_dtos.into_iter().flatten() {
                let item: BinderItem = item_dto.into();
                let content_ids = binder_item_commands::get_binder_item_relationship(
                    ctx,
                    &item.id,
                    &BinderItemRelationshipField::Contents,
                )?;
                let contents: Vec<Content> =
                    content_commands::get_content_multi(ctx, &content_ids)?
                        .into_iter()
                        .flatten()
                        .map(Content::from)
                        .collect();
                items.push(ItemWithContents { item, contents });
            }
            binders.push(BinderWithItems { binder, items });
        }
        Ok(Gathered {
            work,
            tags: Vec::new(),
            dict_words: Vec::new(),
            text_replacement_rules: Vec::new(),
            // Punctuation settings shape prose as it is typed; by export time the
            // substitutions are already in the text, so there is nothing to read.
            smart_punctuation: None,
            trash_infos: Vec::new(),
            // Export only needs the item stream for scope resolution, not the writing plan
            // or the progress history.
            paces: Vec::new(),
            progress_snapshots: Vec::new(),
            binders,
            work_info: None,
        })
    }

    // ── Export (a long operation) ─────────────────────────────────────────────

    fn dto(&self) -> Option<ExportWorkDto> {
        let work_id = self.ids.work_id.get()? as i64;
        let preset_json = serde_json::to_string(&self.selected_preset()).ok()?;
        // A quick scope carries its focused anchor (the backend re-resolves the extent);
        // Choose… carries the full checked set.
        let binder_item_ids: Vec<i64> = if self.scope.get() == ExportScopeKind::Custom {
            self.checked_item_ids()
                .into_iter()
                .map(|i| i as i64)
                .collect()
        } else {
            vec![self.anchor.get()? as i64]
        };
        Some(ExportWorkDto {
            work_id,
            output_path: self.output_path.get(),
            format: self.selected_format(),
            scope_kind: self.scope.get(),
            preset_json,
            binder_item_ids,
        })
    }

    /// "Export" — if the destination already exists, confirm overwrite first; otherwise
    /// export straight away.
    pub fn export(&self, ctx: &mut EventContext) {
        let target = self.output_path.get();
        if !target.trim().is_empty() && Path::new(&target).exists() {
            let vm = self.clone();
            let fname = Path::new(&target)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            MessageBox::warning(tr!(export_overwrite_title()))
                .text(tr!(export_overwrite_text(name = fname)))
                .buttons(MessageBoxButtons::OkCancel)
                .on_result(move |r, c| {
                    if r.button == StandardButton::Ok {
                        vm.run_export(c);
                    }
                })
                .present(ctx);
        } else {
            self.run_export(ctx);
        }
    }

    /// Start the export (a long operation). Returns immediately; the panel closes and a
    /// loading toast takes over, driven by the `Origin::LongOperation(...)` events routed to
    /// `on_long_op_*`.
    fn run_export(&self, ctx: &mut EventContext) {
        let Some(dto) = self.dto() else {
            self.show_error(ctx, "no project is open to export", self.ids.work_id.get());
            return;
        };
        match export_management_commands::export_work(&self.app_ctx, &dto) {
            Ok(op_id) => {
                self.active.set(Some(op_id));
                // Captured NOW — see `long_op::CapturedWork`'s doc. Every
                // later handler for THIS op routes on this snapshot, never a
                // live re-read of `self.ids.work_id`.
                self.active_work_id.set(CapturedWork::now(&self.ids));
                // Close the export panel. `dismiss_top_overlay` (not `dismiss_modal`) so the
                // overwrite-confirmation path — whose `on_result` context is anchored at the
                // tree root — still closes the panel; it is the topmost overlay in both paths.
                ctx.dismiss_top_overlay();
                ctx.show_toast(
                    self.progress_toast(0.0, "")
                        .target_work(self.active_work_id.get()),
                );
            }
            Err(e) => self.show_error(ctx, &format!("{e:#}"), self.ids.work_id.get()),
        }
    }

    fn progress_toast(&self, percent: f32, message: &str) -> Toast {
        let vm = self.clone();
        let body = if message.is_empty() {
            format!("{percent:.0}%")
        } else {
            format!("{percent:.0}% · {message}")
        };
        Toast::loading(tr!(export_progress_title()))
            .scoped_id(EXPORT_TOAST_ID, self.active_work_id.get())
            .body(lit!(body))
            .action(
                ToastAction::destructive(tr!(export_cancel()), move |c| vm.cancel(c))
                    .closes_toast(false),
            )
    }

    pub fn cancel(&self, _ctx: &mut EventContext) {
        if let Some(op_id) = self.active.get() {
            long_operation_commands::cancel_operation(&self.app_ctx, &op_id);
        }
    }

    // ── Long-operation event handlers (wired in `App::build`) ────────────────
    // Every LongOperation event is generic, so each handler first matches the payload's id
    // against the in-flight export — events for save / import / backup are ignored.

    pub fn on_long_op_progress(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(op_id.as_str()) {
            return;
        }
        let percent = payload
            .get("percentage")
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0) as f32;
        let message = payload
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("");
        ctx.show_toast(
            self.progress_toast(percent, message)
                .target_work(self.active_work_id.get()),
        );
    }

    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        if event_id(event) != Some(op_id.clone()) {
            return;
        }
        // Captured BEFORE clearing — see `Self::active_work_id`'s doc: this
        // completion belongs to the Work the export started for, not whatever
        // this window shows now.
        let work_id = self.active_work_id.get();
        self.active.set(None);
        self.active_work_id.set(CapturedWork::none());
        match export_management_commands::get_export_work_result(&self.app_ctx, &op_id) {
            Ok(Some(res)) => {
                let done = tr!(export_done(count = res.exported_count));
                ctx.show_toast(
                    Toast::success(done)
                        .scoped_id(EXPORT_TOAST_ID, work_id)
                        .body(lit!(res.output_path.clone()))
                        .auto_dismiss_after(Duration::from_secs(6))
                        .target_work(work_id),
                );
            }
            Ok(None) | Err(_) => {
                ctx.show_toast(
                    Toast::info(tr!(export_progress_title()))
                        .scoped_id(EXPORT_TOAST_ID, work_id)
                        .auto_dismiss_after(Duration::from_secs(4))
                        .target_work(work_id),
                );
            }
        }
    }

    pub fn on_long_op_cancelled(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        if event_id(event) != Some(op_id.clone()) {
            return;
        }
        // Captured BEFORE clearing — see `Self::active_work_id`'s doc.
        let work_id = self.active_work_id.get();
        self.active.set(None);
        self.active_work_id.set(CapturedWork::none());
        ctx.show_toast(
            Toast::info(tr!(export_cancelled()))
                .scoped_id(EXPORT_TOAST_ID, work_id)
                .auto_dismiss_after(Duration::from_secs(4))
                .target_work(work_id),
        );
    }

    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(op_id.as_str()) {
            return;
        }
        // Captured BEFORE clearing — see `Self::active_work_id`'s doc.
        let work_id = self.active_work_id.get();
        self.active.set(None);
        self.active_work_id.set(CapturedWork::none());
        let error = payload
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or_default()
            .to_string();
        self.show_error(ctx, &error, work_id);
    }

    /// `work_id` is the Work this error concerns: the export's own captured
    /// [`Self::active_work_id`] when it fails mid-flight, or (when `run_export`
    /// never even started — e.g. "no project is open to export") the window's
    /// current `self.ids.work_id`, since no operation ever claimed a Work to
    /// route on instead. Accepts either a plain `Option<u64>` or a
    /// `long_op::CapturedWork` — both callers below hand in whichever one they
    /// actually have.
    fn show_error(&self, ctx: &mut EventContext, message: &str, work_id: impl Into<Option<u64>>) {
        let work_id = work_id.into();
        let details = message.to_string();
        ctx.show_toast(
            Toast::error(tr!(export_error_title()))
                .scoped_id(EXPORT_TOAST_ID, work_id)
                .body(lit!(message.to_string()))
                .persistent()
                .target_work(work_id)
                .action(ToastAction::primary(
                    tr!(export_error_details()),
                    move |c| {
                        MessageBox::warning(tr!(export_error_title()))
                            .text(lit!(details.clone()))
                            .buttons(MessageBoxButtons::Ok)
                            .present(c);
                    },
                )),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::commands::{binder_item_commands, work_management_commands};
    use frontend::work_management::LoadWorkDto;

    fn fixture() -> String {
        format!(
            "{}/../../resources/test/skribisto_test_project.skrib",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    /// Load the real fixture project into a fresh store and build a view-model over it.
    fn loaded_vm() -> (ExportViewModel, Vec<u64>) {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        work_management_commands::load_work(&app_ctx, &LoadWorkDto { file_name: fixture() })
            .expect("load fixture");
        // A fresh, single-project store: the one `Work` the load just created.
        let work_id = frontend::commands::work_commands::get_all_work(&app_ctx)
            .expect("work")
            .first()
            .expect("one work")
            .id;
        ids.seed(&app_ctx, work_id);
        let item_ids: Vec<u64> = binder_item_commands::get_all_binder_item(&app_ctx)
            .expect("items")
            .into_iter()
            .map(|it| it.id)
            .collect();
        (ExportViewModel::new(app_ctx, ids), item_ids)
    }

    #[test]
    fn client_gather_reads_the_fixture_tree() {
        let (vm, _) = loaded_vm();
        let g = vm.client_gather().expect("gather");
        assert!(!g.binders.is_empty(), "the fixture has at least one binder");
        let total: usize = g.binders.iter().map(|b| b.items.len()).sum();
        assert!(total > 0, "the fixture has binder items");
    }

    #[test]
    fn focusing_an_item_offers_at_least_its_own_facet() {
        let (vm, items) = loaded_vm();
        // Some item in the fixture must yield a non-empty adaptive scope list.
        let any = items
            .iter()
            .any(|&id| !vm.compute_applicable(Some(id)).is_empty());
        assert!(
            any,
            "at least one focused item should offer a quick export scope"
        );
    }

    #[test]
    fn no_focus_offers_only_choose() {
        // With a project open but nothing focused, the quick scopes drop out but Choose…
        // (Custom) stays — it exports whatever the checkbox tree selects, focus or not.
        let (vm, _) = loaded_vm();
        assert_eq!(vm.compute_applicable(None), vec![ExportScopeKind::Custom]);
    }

    #[test]
    fn a_focused_item_offers_choose_last() {
        let (vm, items) = loaded_vm();
        // Whatever the focused item, Choose… is always the final entry.
        let with_scopes = items
            .iter()
            .find(|&&id| vm.compute_applicable(Some(id)).len() > 1);
        if let Some(&id) = with_scopes {
            let scopes = vm.compute_applicable(Some(id));
            assert_eq!(scopes.last(), Some(&ExportScopeKind::Custom));
        }
    }

    #[test]
    fn custom_scope_exports_the_checked_tree() {
        let (vm, _) = loaded_vm();
        vm.prepare(ExportScopeKind::Custom, None);
        // The default seed checks the prose rows, so the include set is non-empty and the
        // preview renders — without any anchor.
        assert!(
            !vm.checked_item_ids().is_empty(),
            "prose rows are checked by default"
        );
        assert!(
            vm.preview_document().is_some(),
            "Custom previews the checked selection"
        );
    }

    #[test]
    fn preview_renders_a_document_for_a_resolvable_scope() {
        let (vm, items) = loaded_vm();
        // Anchor on the first item that offers a scope, take that scope, and preview it.
        let anchored = items.iter().find_map(|&id| {
            let scopes = vm.compute_applicable(Some(id));
            scopes.first().cloned().map(|s| (id, s))
        });
        let (id, scope) = anchored.expect("a resolvable scope somewhere in the fixture");
        vm.prepare(scope, Some(id));
        assert!(
            vm.preview_document().is_some(),
            "a resolvable scope should preview a document"
        );
    }

    // ── Phase 3 — F4: the captured Work must survive a later in-place switch ──
    //
    // `ExportViewModel` is minted once per window and outlives any one export
    // (see `Self::active_work_id`'s doc). Before this fix, every `on_long_op_*`
    // handler re-read `self.ids.work_id.get()` live — so a window that started
    // an export, then switched to a different Work before it finished, would
    // route the completion toast to the NEW Work, and the Work that actually
    // ran the export would never hear about it. These tests pin the fix: the
    // Work an export started for is captured once (`run_export`, simulated
    // here since this crate has no `EventContext` test harness — see the
    // module's other tests) and never re-derived from the live `ids`.

    #[test]
    fn the_captured_work_id_survives_a_later_in_place_switch() {
        let (vm, _) = loaded_vm();
        let original_work_id = vm.ids.work_id.get();
        assert!(original_work_id.is_some(), "the fixture loads a real Work");

        // Simulate what `run_export` does the instant the long operation starts:
        // snapshot the window's current Work.
        vm.active.set(Some("fake-export-op".to_string()));
        vm.active_work_id.set(CapturedWork::now(&vm.ids));

        // An in-place project switch reseeds `ids.work_id` on the SAME `AppIds`
        // this long-lived view-model holds (`ProjectSwitchViewModel::request`
        // gates only on unsaved edits, never on an export in flight).
        let other_work_id = original_work_id.unwrap() + 1000;
        vm.ids.seed(&vm.app_ctx, other_work_id);

        assert_eq!(
            vm.active_work_id.get(),
            original_work_id,
            "the in-flight export's own Work must stay pinned to what `run_export` \
             captured, even after this window switches to a different Work"
        );
        assert_ne!(
            vm.active_work_id.get(),
            vm.ids.work_id.get(),
            "the captured Work must now differ from the window's live `ids.work_id` — \
             proving a handler reading `active_work_id` cannot silently be reading the \
             same live value `ids.work_id` would give it"
        );
    }

    #[test]
    fn export_toast_ids_for_two_works_never_collide() {
        // The other half of the fix (F1/F2's root cause, applied to export): even
        // with the right Work captured, a bare `EXPORT_TOAST_ID` shared by every
        // window would let a second Work's export find this one's still-live
        // toast entry (`ToastRegistry::enqueue` dedups on id alone) and silently
        // retarget/steal it.
        let id_a = crate::toast_scope::work_scoped_toast_id(EXPORT_TOAST_ID, Some(1));
        let id_b = crate::toast_scope::work_scoped_toast_id(EXPORT_TOAST_ID, Some(2));
        assert_ne!(id_a, id_b, "two different Works' export toasts must never collide");
    }

    /// The test above only proves `work_scoped_toast_id` itself is collision-free —
    /// it never touches `progress_toast`'s actual `.id(...)` call site, so reverting
    /// that call site back to a bare `EXPORT_TOAST_ID` would still leave it green.
    /// This one drives the real call site through a real `ToastRegistry`: two
    /// `ExportViewModel`s captured for two different Works each raise their export
    /// progress toast through a wired `Button` + a dispatched click (a real
    /// `EventContext`, not a direct fn call), then asserts both stay live —
    /// `ToastRegistry::enqueue`'s update-in-place merge would collapse them to ONE
    /// entry if the id were ever bare again.
    #[test]
    fn export_progress_toasts_for_two_works_both_stay_live_in_a_real_registry() {
        use bastyde::i18n::lit;
        use bastyde::widgets::{Button, ToastInstallOptions, ToastRegistry};

        let (vm_a, _) = loaded_vm();
        vm_a.active_work_id.set(CapturedWork::for_test(Some(1)));
        let (vm_b, _) = loaded_vm();
        vm_b.active_work_id.set(CapturedWork::for_test(Some(2)));

        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        let mut tree =
            crate::test_support::tree_with_toast_registry(&vm_a.app_ctx, &registry);

        let a = vm_a.clone();
        let b = vm_b.clone();
        let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| {
            ctx.show_toast(a.progress_toast(10.0, "").target_work(a.active_work_id.get()));
        }));
        let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| {
            ctx.show_toast(b.progress_toast(20.0, "").target_work(b.active_work_id.get()));
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));

        crate::test_support::click(&mut tree, btn_a);
        crate::test_support::click(&mut tree, btn_b);

        assert_eq!(
            registry.live_count(),
            2,
            "two different Works' export-progress toasts must both stay live — a bare \
             EXPORT_TOAST_ID would let Work B's enqueue find Work A's still-live entry \
             (ToastRegistry::enqueue dedups on id alone) and merge into it, leaving only 1"
        );
    }

    #[test]
    fn dto_carries_scope_anchor_and_style() {
        let (vm, items) = loaded_vm();
        let id = items[0];
        vm.prepare(ExportScopeKind::CurrentBook, Some(id));
        vm.set_output_path("/tmp/out.html".into());
        let dto = vm.dto().expect("dto");
        assert_eq!(dto.scope_kind, ExportScopeKind::CurrentBook);
        assert_eq!(dto.binder_item_ids, vec![id as i64]);
        assert_eq!(dto.output_path, "/tmp/out.html");
        assert!(!dto.preset_json.is_empty(), "the style travels as JSON");
    }
}
