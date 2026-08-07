// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Export modal — pick a scope + format + style + destination, and export.
//!
//! Presented as an in-tree modal (see the `export.scope` action in `app.rs`), mirroring the
//! Import panel's chrome (title strip + close). The body is a two-column layout:
//!
//! - **Leading** — **What to export** (scope segmented control + the Choose… checkbox tree in
//!   Custom mode). The tree owns the full column height so a real manuscript outline is usable.
//! - **Trailing** — **Format**, **Style preset**, and **Destination**.
//!
//! Cancel / Export sit in a full-width footer under both columns. All logic lives on
//! [`ExportViewModel`]; this view is thin.
//!
//! There used to be a third column carrying a live preview of the compiled document. It is
//! gone. It could only ever show the *assembled text* — it compiled through one fixed format
//! regardless of the one chosen — so the things a writer opens this modal to check (page
//! breaks, the title page, DOCX styling, PDF pagination) were exactly the things it could not
//! show. Whatever it did show, the real file shows better. The export toast now offers to
//! open that file, or the folder holding it, which is what "let me look at the result"
//! always meant.

use bastyde::core::binding::BindingLevel;
use bastyde::core::styles::PanelVariant;
use bastyde::core::widget::WidgetPlacement;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::{
    Badge, Button, ButtonVariant, ComboBox, Divider, Expand, FilePickerField, FilePickerKind,
    FixedSize, HStack, IconButton, Padding, Panel, RadioTile, RadioTileGroup, Segment,
    SegmentedControl, Spacer, TextWidget, TileLayout, Toggle, VStack, Wrap,
};

use export_management::{ExportFormat, ExportScopeKind};
use skribisto_compiler::Preset;

use crate::export::choose::ChooseTreeWidget;
use crate::view_models::{ExportViewModel, format_label, scope_label};

/// Two columns now, not three — sized for what is left after the preview column went,
/// rather than keeping a third of the card empty.
const CARD_W: f32 = 900.0;
/// Two heights, because the two modes genuinely hold different amounts. A quick scope is
/// a segmented control and three fields; Custom adds a whole manuscript outline, and the
/// tree is only usable with room to be a tree in. One height for both meant either a
/// cramped outline or a half-empty dialog, and with the preview column gone there is
/// nothing to fill the slack with.
const CARD_H_QUICK: f32 = 520.0;
const CARD_H_CUSTOM: f32 = 760.0;
/// The taller of the two — what the headless layout tests propose.
const CARD_H: f32 = CARD_H_CUSTOM;
/// Leading column: scope control + Choose… tree (Custom mode).
const TREE_W: f32 = 340.0;
/// Trailing column: format · style · destination. Its *minimum*; it grows into whatever
/// the tree leaves.
const OPTIONS_W: f32 = 360.0;

pub struct ExportPanel {
    vm: ExportViewModel,
    /// Driven by the mode effect below rather than derived inline: a mapped signal is
    /// read-only and lazy, and `FixedSize` wants a plain one it can observe.
    card_height: Signal<f32>,
    root_child: Option<WidgetId>,
}

impl ExportPanel {
    /// Build the panel over the shared, app-state [`ExportViewModel`] (the same instance
    /// `App::build` wired the long-operation events to). The presenting action calls
    /// [`ExportViewModel::prepare`] first so the scope/anchor/default path are set.
    pub fn new(vm: ExportViewModel) -> Self {
        // Seeded from the scope the presenting action already set, so the card opens at the
        // right height rather than snapping to it on the first frame.
        let custom = vm.scope_signal().get() == ExportScopeKind::Custom;
        Self {
            vm,
            card_height: Signal::new(if custom { CARD_H_CUSTOM } else { CARD_H_QUICK }),
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
///
/// `ctx` only to open the dialog where the writer last exported
/// (`models::picker_starts_in`) — a `FilePickerField` builds its own dialog on Browse,
/// so the directory has to be supplied here rather than at the click.
fn destination_field(ctx: &BuildContext, vm: &ExportViewModel) -> FilePickerField {
    let default_name = std::path::Path::new(&vm.output_path().get())
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("export")
        .to_string();
    crate::models::picker_starts_in(
        ctx,
        crate::models::FolderPurpose::Export,
        FilePickerField::new(vm.output_path())
            .kind(FilePickerKind::SaveFile)
            .default_file_name(default_name)
            .add_filter("Export", &[current_extension(vm)])
            .on_pick(|res, ctx| {
                crate::models::remember_pick(ctx, crate::models::FolderPurpose::Export, res)
            }),
    )
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
        // Fold the "What to export" segmented control's selection onto the active scope,
        // and resize the card to the mode's own content.
        {
            let vm = self.vm.clone();
            let height = self.card_height.clone();
            ctx.effect(&self.vm.segment_index(), move |i| {
                vm.apply_segment();
                height.set(if *i == 1 { CARD_H_CUSTOM } else { CARD_H_QUICK });
            });
        }

        let selection = SelectionColumn::new(self.vm.clone());
        let options = OptionsColumn::new(self.vm.clone());

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
                height: self.card_height.clone()
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
                        Expand::horizontal {
                            Divider
                        }
                        // Body: tree | options, both full height.
                        Expand::vertical {
                            HStack {
                                spacing: 0.0
                                child: selection
                                Divider::vertical
                                // Takes the rest of the width; the options column measures at
                                // its own fixed width, so the surplus is space around it.
                                Expand {
                                    child: options
                                }
                            }
                        }
                        Expand::horizontal {
                            Divider
                        }
                        Expand::horizontal {
                            child: footer
                        }
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
        let root = bati!(ctx => Expand::vertical {
            child: body
        });
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
                destination_field(ctx, &self.vm),
            ))
            .child(Spacer::new());

        let body = Padding::symmetric(16.0, 16.0).child(col);
        let root = bati!(ctx => Expand::vertical {
            child: body
        });
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Takes whatever the row leaves after the tree, never less than its natural width.
        // With the preview column gone there is no third claimant for the surplus, and the
        // format grid and the destination field both read better wide than they did
        // squeezed into a fixed 360.
        let width = proposal.width.unwrap_or(OPTIONS_W).max(OPTIONS_W);
        fixed_col_size(self.root_child, width, proposal, ctx)
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
                Expand::vertical {
                    child: tree
                }
                Expand::horizontal {
                    Divider
                }
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

    /// The whole panel — title strip, both body columns, and the footer — must
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

    /// A quick scope opens shorter than Custom. The two modes hold different amounts —
    /// Custom adds a whole outline — and with the preview column gone there is nothing to
    /// pad the difference out with, so one height would leave one of them wrong.
    #[test]
    fn the_card_is_shorter_without_the_outline() {
        let quick = ExportViewModel::new(Rc::new(AppContext::new()), AppIds::new());
        assert_eq!(ExportPanel::new(quick).card_height.get(), CARD_H_QUICK);

        let custom = ExportViewModel::new(Rc::new(AppContext::new()), AppIds::new());
        custom.prepare(ExportScopeKind::Custom, None);
        assert_eq!(ExportPanel::new(custom).card_height.get(), CARD_H_CUSTOM);
        const { assert!(CARD_H_QUICK < CARD_H_CUSTOM) };
    }

    /// The options column takes the width the tree leaves rather than staying pinned at its
    /// old fixed 360 — otherwise removing the preview column would just have left a third
    /// of the card blank. (Its `.max(OPTIONS_W)` floor is not asserted here: a `WidgetTree`
    /// clips its root to the proposal, so a narrower proposal measures the harness rather
    /// than the widget.)
    #[test]
    fn the_options_column_fills_the_width_it_is_given() {
        let vm = ExportViewModel::new(Rc::new(AppContext::new()), AppIds::new());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(OptionsColumn::new(vm)));
        let offered = CARD_W - TREE_W;
        tree.layout(SizeProposal::exact(offered, CARD_H));
        assert_eq!(tree.bounds(id).width, offered);
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
