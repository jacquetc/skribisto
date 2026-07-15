//! The Export modal — pick a style + format + destination for a focus-adaptive quick scope,
//! watch a live preview, and export.
//!
//! Presented as an in-tree modal (see the `export.scope` action in `app.rs`), mirroring the
//! Import panel's chrome (title strip + close, full-width rule, bottom action bar). All logic
//! lives on [`ExportViewModel`]; this view is thin — it binds the VM's signals and forwards
//! the footer buttons to its methods. The **preview** shows the exact `TextDocument` the
//! chosen style would compile (headings + scene breaks + prose), rebuilt when the style
//! changes; it is the same compile path the committed export runs, so the two cannot diverge.

use bastyde::core::binding::BindingLevel;
use bastyde::core::styles::PanelVariant;
use bastyde::core::widget::WidgetPlacement;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Button, ButtonVariant, Center, Checkbox, ComboBox, Divider, Expand, FilePickerField,
    FilePickerKind, FixedSize, FormLayout, HStack, IconButton, Padding, Panel, RadioTile,
    RadioTileGroup, ScrollArea, Spacer, TextWidget, TileLayout, VStack,
};

use export_management::{ExportFormat, ExportScopeKind};
use skribisto_compiler::Preset;

use crate::export_choose::ChooseTreeWidget;
use crate::view_models::{ExportViewModel, SettingsViewModel, format_label, scope_label};

const CARD_W: f32 = 940.0;
const CARD_H: f32 = 660.0;
/// The fixed width of the leading column (scope + controls); the preview fills the rest.
const LEADING_W: f32 = 430.0;

pub struct ExportPanel {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl ExportPanel {
    /// Build the panel over the shared, app-state [`ExportViewModel`] (the same instance
    /// `App::build` wired the long-operation events to). The presenting action calls
    /// [`ExportViewModel::prepare`] first so the scope/anchor/default path are set.
    pub fn new(vm: ExportViewModel) -> Self {
        Self { vm, root_child: None }
    }
}

fn field_label(text: LocalizedString) -> TextWidget {
    TextWidget::new(text).style(TextStyleRole::Small).color(TextRole::Secondary)
}

/// The format · style · file control rows (shared by both scope modes).
fn controls_form(vm: &ExportViewModel) -> impl Widget + 'static {
    // Format picker — a compact vertical radio-tile list (all formats visible at once, the
    // New Work panel's shape). An effect keeps the path extension in step with the choice.
    let mut format = RadioTileGroup::new(vm.format_index())
        .layout(TileLayout::Vertical)
        .row_height(34.0)
        .line_spacing(2.0);
    for f in ExportViewModel::panel_formats() {
        let ext = match f {
            ExportFormat::Docx => ".docx",
            ExportFormat::Html => ".html",
            ExportFormat::Markdown => ".md",
            ExportFormat::Djot => ".dj",
            ExportFormat::PlainText => ".txt",
            ExportFormat::Latex => ".tex",
            ExportFormat::Epub => ".epub",
            ExportFormat::Pdf => ".pdf",
        };
        format = format.tile(RadioTile::new().title(format_label(f)).trailing(lit!(ext)));
    }

    // Style picker — built-in styles for now (M4 unions the user's in).
    let style = ComboBox::from_items(vm.presets(), vm.preset_signal(), |p: &Preset| {
        lit!(p.name.clone())
    });

    // Destination — a save-file picker bound to the output path.
    let default_name = std::path::Path::new(&vm.output_path().get())
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("export")
        .to_string();
    let path = FilePickerField::new(vm.output_path())
        .kind(FilePickerKind::SaveFile)
        .default_file_name(default_name)
        .add_filter("Export", &[current_extension(vm)]);

    FormLayout::new()
        .label_gap(14.0)
        .row_spacing(14.0)
        .line(field_label(tr!(export_format_label())), format)
        .line(field_label(tr!(export_style_label())), style)
        .full_width(Divider::new())
        .line(field_label(tr!(export_path_label())), path)
}

/// The extension of the currently-selected format (for the save dialog filter).
fn current_extension(vm: &ExportViewModel) -> &'static str {
    let name = vm.output_path().get();
    std::path::Path::new(&name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| match e {
            "docx" => "docx",
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

        let leading = LeadingColumn::new(self.vm.clone());
        let preview = ExportPreviewBody::new(self.vm.clone());
        let vdivider = Divider::vertical();

        let export_vm = self.vm.clone();
        let export_can = self.vm.can_export();

        let root = bati!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        // Header (full width) — wrapped in Expand::horizontal so the
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
                        // Body: a fixed-width leading column (scope + controls) and the
                        // preview filling the trailing side, full height.
                        Expand::vertical {
                            HStack {
                                spacing: 0.0
                                child: leading
                                child: vdivider
                                Expand::horizontal {
                                    VStack {
                                        spacing: 0.0
                                        Padding::new(10.0, 20.0, 6.0, 20.0) {
                                            TextWidget::new(tr!(export_preview_label())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        Expand::horizontal { Divider }
                                        Expand::vertical { child: preview }
                                    }
                                }
                            }
                        }
                        Expand::horizontal { Divider }
                        // Footer (full width) — wrapped so the Spacer pushes the buttons right.
                        Expand::horizontal {
                            FixedSize {
                                height: 56.0
                                Padding::symmetric(10.0, 22.0) {
                                    HStack {
                                        spacing: 9.0
                                        Spacer
                                        Button::new(tr!(export_cancel())) {
                                            variant: ButtonVariant::Plain
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
                                        Button::new(tr!(export_export())) {
                                            variant: ButtonVariant::Filled
                                            enabled: export_can
                                            on_activate_fn: move |ctx| export_vm.export(ctx)
                                        }
                                    }
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
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// The live-preview body: rebuilds when the chosen style changes, hosting the compiled
/// read-only document or an empty state.
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
        // Rebuild when the style changes, or — under Choose… — when the checkbox selection
        // changes (`custom_changed`). (Scope + anchor are fixed for the panel's lifetime;
        // format is a write-time knob.)
        self.vm.preset_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.vm.custom_changed().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let child: Box<dyn Widget> = match self.vm.preview_document() {
            Some(doc) => {
                let width = SettingsViewModel::new(ctx.settings()).preview_width();
                let editor = RichTextEditor::read_only(doc)
                    .content_padding_symmetric(8.0, 8.0)
                    .v_scroll_policy(ScrollPolicy::AlwaysOff);
                Box::new(
                    ScrollArea::new().child(
                        Padding::symmetric(12.0, 8.0)
                            .child(crate::tabs::shared::editor::centered(editor, &width)),
                    ),
                )
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
        self.child_id = Some(ctx.add_boxed(child));
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
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The fixed-width leading column: the scope selector (a "What" line for a quick scope, or
/// the Choose… checkbox tree) above the format/style/path controls. Reports a fixed width and
/// fills the body height, so the preview column takes the rest of the width and full height.
struct LeadingColumn {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl LeadingColumn {
    fn new(vm: ExportViewModel) -> Self {
        Self { vm, root_child: None }
    }
}

impl std::fmt::Debug for LeadingColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeadingColumn").finish()
    }
}

impl Widget for LeadingColumn {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let controls = Padding::symmetric(20.0, 16.0).child(controls_form(&self.vm));
        let root = if self.vm.scope() == ExportScopeKind::Custom {
            // Choose…: the checkbox tree fills the column; the controls sit below it.
            let choose = ChoosePane::new(self.vm.clone());
            bati!(ctx => VStack {
                spacing: 0.0
                Expand::vertical { child: choose }
                Expand::horizontal { Divider }
                child: controls
            })
        } else {
            // A quick scope: a small "What: <scope>" line above the controls.
            let what = Padding::new(14.0, 20.0, 12.0, 20.0).child(
                HStack::new()
                    .spacing(14.0)
                    .child(field_label(tr!(export_scope_label())))
                    .child(TextWidget::new(scope_label(&self.vm.scope())).color(TextRole::Primary)),
            );
            bati!(ctx => VStack {
                spacing: 0.0
                child: what
                Expand::horizontal { Divider }
                child: controls
            })
        };
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Fixed width; fill the body height (else the child's natural height).
        let child_h = self
            .root_child
            .and_then(|id| ctx.child_size(id, SizeProposal::with_width(LEADING_W)))
            .map(|s| s.height)
            .unwrap_or(0.0);
        Size::new(LEADING_W, proposal.height.unwrap_or(child_h)).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// The Choose… section: a "Show non-exportable" reveal toggle over the checkbox tree, which
/// fills the leading column's vertical space. Rebuilds the tree (preserving checks) when the
/// toggle flips.
struct ChoosePane {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl ChoosePane {
    fn new(vm: ExportViewModel) -> Self {
        Self { vm, root_child: None }
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

        let tree = ScrollArea::new().child(ChooseTreeWidget::new(self.vm.choose_model()));
        let show = self.vm.show_non_exportable();

        let root = bati!(ctx => Padding::new(10.0, 20.0, 8.0, 20.0) {
            VStack {
                spacing: 8.0
                Checkbox::new(show) {
                    label: tr!(export_show_non_exportable())
                }
                Expand::vertical { child: tree }
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
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use crate::app_ids::AppIds;
    use frontend::AppContext;
    use std::rc::Rc;

    /// The whole panel — header, the four-row `FormLayout`, the preview body, and the footer
    /// — must build and lay out headlessly without panicking, at the modal's card size.
    #[test]
    fn panel_builds_and_lays_out() {
        let vm = ExportViewModel::new(Rc::new(AppContext::new()), AppIds::new());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(ExportPanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!((b.width, b.height), (CARD_W, CARD_H), "panel fills the card");
    }
}
