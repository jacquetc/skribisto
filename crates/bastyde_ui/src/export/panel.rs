// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Export modal — pick a scope + format + style + destination, watch a live preview, and
//! export.
//!
//! Presented as an in-tree modal (see the `export.scope` action in `app.rs`), mirroring the
//! Import panel's chrome (title strip + close). The body is a three-column layout:
//!
//! - **Leading** — **What to export** (scope segmented control + the Choose… checkbox tree in
//!   Custom mode). The tree owns the full column height so a real manuscript outline is usable.
//! - **Middle** — **Format**, **Style preset**, and **Destination**.
//! - **Trailing** — full-height live **preview**: the exact `TextDocument` the chosen style
//!   would compile, rebuilt when the style, scope, or Choose… checks change. Same compile path
//!   as the committed export.
//!
//! Cancel / Export sit in a full-width footer under all three columns. All logic lives on
//! [`ExportViewModel`]; this view is thin.

use bastyde::core::binding::BindingLevel;
use bastyde::core::styles::PanelVariant;
use bastyde::core::widget::WidgetPlacement;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::rich_text::RichTextEditor;
use bastyde::widgets::{
    Badge, Button, ButtonVariant, Center, ComboBox, Divider, Expand, FilePickerField,
    FilePickerKind, FixedSize, HStack, IconButton, Padding, Panel, RadioTile, RadioTileGroup,
    Segment, SegmentedControl, Spacer, TextWidget, TileLayout, Toggle, VStack, Wrap,
};

use export_management::{ExportFormat, ExportScopeKind};
use skribisto_compiler::Preset;

use crate::export::choose::ChooseTreeWidget;
use crate::view_models::{ExportViewModel, SettingsViewModel, format_label, scope_label};

const CARD_W: f32 = 1200.0;
const CARD_H: f32 = 760.0;
/// Leading column: scope control + Choose… tree (Custom mode).
const TREE_W: f32 = 340.0;
/// Middle column: format · style · destination.
const OPTIONS_W: f32 = 360.0;

pub struct ExportPanel {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl ExportPanel {
    /// Build the panel over the shared, app-state [`ExportViewModel`] (the same instance
    /// `App::build` wired the long-operation events to). The presenting action calls
    /// [`ExportViewModel::prepare`] first so the scope/anchor/default path are set.
    pub fn new(vm: ExportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

/// A small, dimmed section header (normal case — no full caps).
fn field_label(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// A labeled section: its header above the body widget.
fn section(header: LocalizedString, body: impl Widget + 'static) -> VStack {
    VStack::new()
        .spacing(9.0)
        .child(field_label(header))
        .child(body)
}

/// Fixed-width column that fills the proposed body height.
fn fixed_col_size(
    root: Option<WidgetId>,
    width: f32,
    proposal: SizeProposal,
    ctx: &LayoutContext,
) -> LayoutResponse {
    let child_h = root
        .and_then(|id| ctx.child_size(id, SizeProposal::with_width(width)))
        .map(|s| s.height)
        .unwrap_or(0.0);
    Size::new(width, proposal.height.unwrap_or(child_h)).into()
}

fn place_fill(bounds: Rect, children: &mut [WidgetPlacement]) {
    for child in children.iter_mut() {
        child.origin = bounds.origin();
        child.size = bounds.size();
    }
}

/// Centre a child horizontally at a capped column width while **filling the
/// proposed height** — the dual of [`crate::tabs::shared::editor::centered`]'s
/// `CenterColumnFlowing`.
///
/// Flowing centres measure the child width-only so intrinsic (`min_lines`)
/// editors grow with their document. A side-panel **preview** wants the
/// opposite: a **greedy** editor that consumes the pane height and scrolls
/// inside itself. That only works when the parent proposes an exact height;
/// this wrapper fills the slot and forwards both axes to the child.
struct CenterColumnFill {
    column_width: Signal<f32>,
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl CenterColumnFill {
    fn new(child: impl Widget + 'static, column_width: Signal<f32>) -> Self {
        Self {
            column_width,
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }

    fn col_w(&self, available: f32) -> f32 {
        self.column_width.get().min(available).max(0.0)
    }
}

impl std::fmt::Debug for CenterColumnFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CenterColumnFill").finish()
    }
}

impl Widget for CenterColumnFill {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.column_width.bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Relayout,
        );
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        // Fill the slot (or collapse to zero if the parent left an axis open —
        // the preview body always proposes exact bounds via Expand).
        proposal.resolve(0.0, 0.0).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let w = self.col_w(bounds.width);
        let dx = ((bounds.width - w) / 2.0).max(0.0);
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x + dx, bounds.y);
            child.size = Size::new(w, bounds.height);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The `.ext` suffix shown on a format tile.
fn format_ext(f: &ExportFormat) -> &'static str {
    match f {
        ExportFormat::Docx => ".docx",
        ExportFormat::Html => ".html",
        ExportFormat::Markdown => ".md",
        ExportFormat::Djot => ".dj",
        ExportFormat::PlainText => ".txt",
        ExportFormat::Latex => ".tex",
        ExportFormat::Epub => ".epub",
        ExportFormat::Pdf => ".pdf",
    }
}

/// The Format picker — a wrapping grid of format cards (title + extension). An `ExportPanel`
/// effect keeps the destination extension in step with the choice.
fn format_grid(vm: &ExportViewModel) -> RadioTileGroup {
    let mut grid = RadioTileGroup::new(vm.format_index())
        .layout(TileLayout::Grid {
            min_tile_width: 122.0,
        })
        .spacing(7.0)
        .line_spacing(7.0);
    for f in ExportViewModel::panel_formats() {
        grid = grid.tile(
            RadioTile::new()
                .title(format_label(f))
                .description(lit!(format_ext(f))),
        );
    }
    grid
}

/// The Destination save-file field, seeded with the current output path + format filter.
fn destination_field(vm: &ExportViewModel) -> FilePickerField {
    let default_name = std::path::Path::new(&vm.output_path().get())
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("export")
        .to_string();
    FilePickerField::new(vm.output_path())
        .kind(FilePickerKind::SaveFile)
        .default_file_name(default_name)
        .add_filter("Export", &[current_extension(vm)])
}

/// The extension of the currently-selected format (for the save dialog filter).
fn current_extension(vm: &ExportViewModel) -> &'static str {
    let name = vm.output_path().get();
    std::path::Path::new(&name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| match e {
            "docx" => "docx",
            "pdf" => "pdf",
            "epub" => "epub",
            "html" => "html",
            "md" => "md",
            "dj" => "dj",
            "txt" => "txt",
            "tex" => "tex",
            _ => "html",
        })
        .unwrap_or("html")
}

impl std::fmt::Debug for ExportPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportPanel").finish()
    }
}

impl Widget for ExportPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Keep the destination's extension in step with the chosen format.
        {
            let vm = self.vm.clone();
            ctx.effect(&self.vm.format_index(), move |_| vm.retarget_extension());
        }
        // Fold the "What to export" segmented control's selection onto the active scope.
        {
            let vm = self.vm.clone();
            ctx.effect(&self.vm.segment_index(), move |_| vm.apply_segment());
        }

        let selection = SelectionColumn::new(self.vm.clone());
        let options = OptionsColumn::new(self.vm.clone());
        let header = PreviewHeader::new(self.vm.clone());
        let preview = ExportPreviewBody::new(self.vm.clone());

        let export_vm = self.vm.clone();
        let export_can = self.vm.can_export();
        let footer = Padding::symmetric(12.0, 20.0).child(
            HStack::new()
                .spacing(9.0)
                .child(
                    Button::new(tr!(export_cancel()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(|ctx| ctx.dismiss_modal()),
                )
                .child(Spacer::new())
                .child(
                    Button::new(tr!(export_export()))
                        .variant(ButtonVariant::Filled)
                        .enabled(export_can)
                        .on_activate_fn(move |ctx| export_vm.export(ctx)),
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
                        // Title strip (full width) — wrapped in Expand::horizontal so the
                        // height-only FixedSize doesn't collapse the row to its min width.
                        Expand::horizontal {
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(tr!(export_title())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        IconButton::clear() {
                                            tooltip: tr!(export_close())
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
                                    }
                                }
                            }
                        }
                        Expand::horizontal { Divider }
                        // Body: tree | options | preview — each full height.
                        Expand::vertical {
                            HStack {
                                spacing: 0.0
                                child: selection
                                Divider::vertical
                                child: options
                                Divider::vertical
                                // Fill BOTH axes: width takes the trailing side, and the full
                                // row height flows down so the inner `Expand::vertical` (and the
                                // preview page) actually fill — `Expand::horizontal` alone would
                                // propose an unbounded height and the preview would collapse.
                                Expand {
                                    VStack {
                                        spacing: 0.0
                                        child: header
                                        Expand::horizontal { Divider }
                                        Expand::vertical { child: preview }
                                    }
                                }
                            }
                        }
                        Expand::horizontal { Divider }
                        Expand::horizontal { child: footer }
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// The live-preview header: a "Preview" label + a "compiled · <format> · <style>" subtitle +
/// a "Live preview" status, rebuilt when the format or style changes.
struct PreviewHeader {
    vm: ExportViewModel,
    child_id: Option<WidgetId>,
}

impl PreviewHeader {
    fn new(vm: ExportViewModel) -> Self {
        Self { vm, child_id: None }
    }
}

impl std::fmt::Debug for PreviewHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreviewHeader").finish()
    }
}

impl Widget for PreviewHeader {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.format_index().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.vm.preset_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let dot = || {
            TextWidget::new(lit!("·"))
                .style(TextStyleRole::Small)
                .color(TextRole::Disabled)
        };
        let subtitle = HStack::new()
            .spacing(5.0)
            .child(
                TextWidget::new(tr!(export_preview_compiled()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Disabled),
            )
            .child(dot())
            .child(
                TextWidget::new(self.vm.current_format_label())
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(dot())
            .child(
                TextWidget::new(lit!(self.vm.current_preset_name()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );

        let row = Padding::symmetric(10.0, 16.0).child(
            HStack::new()
                .spacing(10.0)
                .child(TextWidget::new(tr!(export_preview_label())).color(TextRole::Primary))
                .child(subtitle)
                .child(Spacer::new())
                .child(
                    TextWidget::new(tr!(export_preview_live()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Success),
                ),
        );
        let id = ctx.add(row);
        self.child_id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The live-preview body: rebuilds when the chosen style, scope, or Choose… selection changes,
/// hosting the compiled read-only document or an empty state.
struct ExportPreviewBody {
    vm: ExportViewModel,
    child_id: Option<WidgetId>,
}

impl ExportPreviewBody {
    fn new(vm: ExportViewModel) -> Self {
        Self { vm, child_id: None }
    }
}

impl std::fmt::Debug for ExportPreviewBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportPreviewBody").finish()
    }
}

impl Widget for ExportPreviewBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the style changes, when the scope is switched (quick ↔ Custom), or —
        // under Choose… — when the checkbox selection changes (`custom_changed`).
        self.vm.preset_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.vm.scope_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.vm.custom_changed().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let inner: Box<dyn Widget> = match self.vm.preview_document() {
            Some(doc) => {
                // Cap the editor's width like a scene column (Settings ▸ preview
                // width). Unlike the search-preview dock — which is a flowing
                // `min_lines` editor inside an outer `ScrollArea` — this pane is
                // a **side panel**: fill the available height and let the editor
                // scroll itself (greedy + Auto). That avoids the first-frame
                // chicken-and-egg where `layout_full` only runs in paint, so an
                // intrinsic editor measures as `min_lines(1)` until a click
                // forces another layout.
                //
                // `CenterColumnFill` (not `centered` / `CenterColumnFlowing`) is
                // load-bearing: flowing centres measure the child **width-only**,
                // so a greedy editor would fall through to the 100 px height
                // fallback again.
                let width = SettingsViewModel::new(ctx.settings()).preview_width();
                let editor = RichTextEditor::read_only(doc).content_padding_symmetric(8.0, 8.0);
                Box::new(Padding::symmetric(12.0, 8.0).child(CenterColumnFill::new(editor, width)))
            }
            None => Box::new(
                Center::new().child(
                    Padding::symmetric(24.0, 16.0).child(
                        TextWidget::new(tr!(export_preview_empty()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    ),
                ),
            ),
        };
        // Mirror the editor's `tab_backdrop` (a Content-surface `Panel` over an `Expand`) so
        // the preview fills the column's full height — a short compiled document still presents
        // a full-height page rather than collapsing to its few lines. Built inline (not via
        // `tab_backdrop`) because the body is a `Box<dyn Widget>` from the match above.
        let inner_id = ctx.add_boxed(inner);
        let backdrop = Panel::new()
            .background(SurfaceRole::Content)
            .corner_radius(0.0)
            .padding(0.0)
            .child(Expand::new().child_id(inner_id));
        self.child_id = Some(ctx.add(backdrop));
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.child_id.and_then(|id| ctx.child_size(id, proposal)) {
            Some(size) => size.into(),
            None => proposal.resolve(0.0, 0.0).into(),
        }
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// Leading column: scope segmented control and, under Custom, the Choose… tree filling the
/// leftover height. Fixed width; rebuilds when the scope switches.
struct SelectionColumn {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl SelectionColumn {
    fn new(vm: ExportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }

    /// The "What to export" section: the scope segmented control (present only when a quick
    /// scope is available beside Custom).
    fn what_section(&self) -> VStack {
        let mut col = VStack::new()
            .spacing(9.0)
            .child(field_label(tr!(export_section_what())));
        if let Some(quick) = self.vm.quick_scope() {
            col = col.child(
                SegmentedControl::new(self.vm.segment_index())
                    .segment(Segment::new(scope_label(&quick)))
                    .segment(Segment::new(tr!(export_custom_selection()))),
            );
        }
        col
    }
}

impl std::fmt::Debug for SelectionColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectionColumn").finish()
    }
}

impl Widget for SelectionColumn {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.scope_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let is_custom = self.vm.scope() == ExportScopeKind::Custom;

        // Under Custom the checkbox tree fills the leftover height; otherwise a trailing
        // Spacer top-aligns the compact scope control.
        let mut col = VStack::new().spacing(12.0).child(self.what_section());
        if is_custom {
            col = col.child(Expand::vertical().child(ChoosePane::new(self.vm.clone())));
        } else {
            col = col.child(Spacer::new());
        }

        let body = Padding::symmetric(16.0, 16.0).child(col);
        let root = bati!(ctx => Expand::vertical { child: body });
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        fixed_col_size(self.root_child, TREE_W, proposal, ctx)
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// Middle column: Format · Style preset · Destination. Fixed width; fills body height with a
/// trailing spacer so the sections stay top-aligned.
struct OptionsColumn {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl OptionsColumn {
    fn new(vm: ExportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }

    /// The "Style preset" section: the style picker (over `catalogue` = built-ins ∪ user
    /// styles) + a wrap of read-only summary chips.
    fn style_section(&self, catalogue: Vec<Preset>) -> VStack {
        let style = ComboBox::from_items(catalogue, self.vm.preset_signal(), |p: &Preset| {
            lit!(p.name.clone())
        });
        VStack::new()
            .spacing(9.0)
            .child(field_label(tr!(export_section_style())))
            .child(style)
            .child(ChipsRow::new(self.vm.clone()))
    }
}

impl std::fmt::Debug for OptionsColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionsColumn").finish()
    }
}

impl Widget for OptionsColumn {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The style picker offers built-ins ∪ the user's saved styles (Settings ▸ Export
        // Formats), read from app-state; it falls back to the panel VM's built-ins when the
        // styles view-model isn't registered (e.g. headless tests).
        let catalogue = ctx
            .app_state::<crate::view_models::ExportStylesViewModel>()
            .cloned()
            .map(|s| s.all_presets())
            .unwrap_or_else(|| self.vm.presets());

        let col = VStack::new()
            .spacing(18.0)
            .child(section(tr!(export_format_label()), format_grid(&self.vm)))
            .child(self.style_section(catalogue))
            .child(section(
                tr!(export_section_destination()),
                destination_field(&self.vm),
            ))
            .child(Spacer::new());

        let body = Padding::symmetric(16.0, 16.0).child(col);
        let root = bati!(ctx => Expand::vertical { child: body });
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        fixed_col_size(self.root_child, OPTIONS_W, proposal, ctx)
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// The Choose… tree box: a bordered, sunken panel holding the checkbox tree above a footer
/// bar ("{n} selected" + a "Show non-exportable" toggle). Rebuilds the tree (preserving
/// checks) when the toggle flips.
struct ChoosePane {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl ChoosePane {
    fn new(vm: ExportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for ChoosePane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChoosePane").finish()
    }
}

impl Widget for ChoosePane {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Flipping the reveal toggle rebuilds this pane, which rebuilds the tree with (or
        // without) the non-exportable rows — `ensure_choose` does the work + preserves checks.
        self.vm.show_non_exportable().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.vm.ensure_choose();

        // `TreeView` is already a virtualized, self-scrolling surface (same as
        // the outline dock). Do **not** wrap it in a `ScrollArea`: that measures
        // the tree with an open height, so `TreeView` falls back to ~200 px and
        // — with `widget_resizable` off — stays stuck at that height inside a
        // tall sunken frame. Hand the tree the Expand slot directly.
        let tree = ChooseTreeWidget::new(self.vm.choose_model());
        let show = self.vm.show_non_exportable();
        let footer_bar = Padding::symmetric(0.0, 12.0).child(
            HStack::new()
                .spacing(8.0)
                .child(SelectedCount::new(self.vm.clone()))
                .child(Spacer::new())
                .child(Toggle::new(show).label(tr!(export_show_non_exportable()))),
        );

        let root = bati!(ctx => Panel {
            variant: PanelVariant::Sunken
            corner_radius: 8.0
            padding: 0.0
            VStack {
                spacing: 0.0
                Expand::vertical { child: tree }
                Expand::horizontal { Divider }
                FixedSize {
                    height: 34.0
                    child: footer_bar
                }
            }
        });
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// The Choose footer's reactive "{n} selected" count — binds `custom_changed` so it refreshes
/// as the writer checks / unchecks rows.
struct SelectedCount {
    vm: ExportViewModel,
    child_id: Option<WidgetId>,
}

impl SelectedCount {
    fn new(vm: ExportViewModel) -> Self {
        Self { vm, child_id: None }
    }
}

impl std::fmt::Debug for SelectedCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectedCount").finish()
    }
}

impl Widget for SelectedCount {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.custom_changed().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let n = self.vm.checked_count() as i64;
        let id = ctx.add(
            TextWidget::new(tr!(export_selected_count(count = n)))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );
        self.child_id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The Style-preset summary chips — a wrap of read-only `Badge`s describing the selected
/// style's structural choices, rebuilt when the style changes.
struct ChipsRow {
    vm: ExportViewModel,
    child_id: Option<WidgetId>,
}

impl ChipsRow {
    fn new(vm: ExportViewModel) -> Self {
        Self { vm, child_id: None }
    }
}

impl std::fmt::Debug for ChipsRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChipsRow").finish()
    }
}

impl Widget for ChipsRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.preset_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let wrap = Wrap::new()
            .spacing(5.0)
            .line_spacing(5.0)
            .children(self.vm.preset_chips().into_iter().map(Badge::new));
        let id = ctx.add(wrap);
        self.child_id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        place_fill(bounds, children);
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use std::rc::Rc;

    /// The whole panel — title strip, three body columns, preview, and the footer — must
    /// build and lay out headlessly at the modal's card size.
    #[test]
    fn panel_builds_and_lays_out() {
        let vm = ExportViewModel::new(Rc::new(AppContext::new()), AppIds::new());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(ExportPanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }

    /// The Custom-scope path (segmented control + bordered tree box + options + footer) must
    /// also build and fill the card, even with no project (an empty Choose tree).
    #[test]
    fn panel_builds_in_custom_mode() {
        let vm = ExportViewModel::new(Rc::new(AppContext::new()), AppIds::new());
        vm.prepare(ExportScopeKind::Custom, None);
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(ExportPanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "custom-mode panel fills the card"
        );
    }
}
