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
    Button, ButtonVariant, Center, ComboBox, Divider, Expand, FilePickerField, FilePickerKind,
    FixedSize, FormLayout, HStack, IconButton, Padding, Panel, ScrollArea, SegmentedControl, Spacer,
    TextWidget, VStack,
};

use skribisto_compiler::Preset;

use crate::view_models::{ExportViewModel, SettingsViewModel, format_label, scope_label};

const CARD_W: f32 = 760.0;
const CARD_H: f32 = 640.0;

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

    fn field_label(text: LocalizedString) -> TextWidget {
        TextWidget::new(text).style(TextStyleRole::Small).color(TextRole::Secondary)
    }

    /// The four control rows: what · format · style · file.
    fn controls(&self) -> impl Widget + 'static {
        let vm = &self.vm;

        // Format picker — the segments track `format_index`; the `App`-side effect keeps the
        // path extension in step.
        let mut format = SegmentedControl::new(vm.format_index());
        for f in ExportViewModel::panel_formats() {
            format = format.segment(format_label(f));
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
            .add_filter("Export", &[Self::current_extension(vm)]);

        FormLayout::new()
            .label_gap(16.0)
            .row_spacing(14.0)
            .line(
                Self::field_label(tr!(export_scope_label())),
                TextWidget::new(scope_label(&vm.scope())).color(TextRole::Primary),
            )
            .line(Self::field_label(tr!(export_format_label())), format)
            .line(Self::field_label(tr!(export_style_label())), style)
            .full_width(Divider::new())
            .line(Self::field_label(tr!(export_path_label())), path)
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

        let controls = Padding::symmetric(20.0, 18.0).child(self.controls());
        let preview = ExportPreviewBody::new(self.vm.clone());

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
                        // Header — wrapped in Expand::horizontal so the height-only
                        // FixedSize doesn't collapse the row to its min width (which would
                        // squeeze the title + close button into the top-left corner).
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
                        // Controls
                        child: controls
                        Expand::horizontal { Divider }
                        // Preview label
                        Padding::new(10.0, 20.0, 4.0, 20.0) {
                            TextWidget::new(tr!(export_preview_label())) {
                                style: TextStyleRole::Small
                                color: TextRole::Secondary
                            }
                        }
                        // Preview body (fills the remaining space)
                        Expand::vertical {
                            child: preview
                        }
                        Expand::horizontal { Divider }
                        // Footer — wrapped so the Spacer can push the buttons to the right.
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
        // Rebuild when the style changes — that is exactly when the compiled document does.
        // (Scope + anchor are fixed for the panel's lifetime; format is a write-time knob.)
        self.vm.preset_signal().bind_to(
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
