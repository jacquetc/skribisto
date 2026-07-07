//! The Import Plume Creator modal — pick a `.plume`/`.plume_backup` project and a
//! destination, then convert it into a new `.skrib`.
//!
//! Presented as an in-tree modal (see the `work.import_plume` action in `app.rs`),
//! mirroring the New Work panel's chrome (title strip + close, full-width rule,
//! bottom action bar) and structure (a two-column [`FormLayout`]). All logic lives
//! on [`ImportPlumeViewModel`]; this view is thin — it binds the VM's signals and
//! forwards the footer buttons to its methods. Picking a source defaults the
//! destination folder + name (same folder, same base name → `.skrib`); a warning
//! notes that trashed items are not migrated.

use bastyde::core::styles::PanelVariant;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FilePickerField, FilePickerKind, FixedSize, FormLayout,
    HStack, IconButton, Padding, Panel, ScrollArea, Spacer, TextInput, TextWidget, VStack,
};

use crate::view_models::ImportPlumeViewModel;

const CARD_W: f32 = 600.0;
const CARD_H: f32 = 500.0;

pub struct ImportPlumePanel {
    vm: ImportPlumeViewModel,
    root_child: Option<WidgetId>,
}

impl ImportPlumePanel {
    /// Build the panel over the shared, app-state [`ImportPlumeViewModel`] (the
    /// same instance `App::build` wired the long-operation events to). The
    /// presenting action resets the form before showing it.
    pub fn new(vm: ImportPlumeViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }

    fn field_label(text: LocalizedString) -> TextWidget {
        TextWidget::new(text)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
    }

    fn hint(text: LocalizedString) -> TextWidget {
        TextWidget::new(text)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
    }

    /// The reactive "Will create `…/<name>.skrib`" preview.
    fn path_preview(&self) -> impl Widget + 'static {
        HStack::new()
            .spacing(7.0)
            .child(
                TextWidget::new(tr!(import_plume_will_create()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(
                TextWidget::new(lit!(""))
                    .text(self.vm.target_path())
                    .style(TextStyleRole::Small)
                    .color(TextRole::Accent),
            )
    }

    fn form(&self) -> impl Widget + 'static {
        let vm = &self.vm;
        // The source picker defaults the destination on pick.
        let picker_vm = self.vm.clone();

        FormLayout::new()
            .label(tr!(import_plume_title()))
            .label_gap(16.0)
            .row_spacing(18.0)
            // ── Source .plume (open-file picker, filtered) ────────────────
            .line(
                Self::field_label(tr!(import_plume_source())),
                VStack::new()
                    .spacing(6.0)
                    .child(
                        FilePickerField::new(vm.source())
                            .kind(FilePickerKind::OpenFile)
                            // A file-dialog filter label — a plain string like the
                            // other dialogs (not a localized key).
                            .add_filter("Plume Creator project", &["plume", "plume_backup"])
                            .validation(vm.source_validation())
                            .on_pick(move |res, _ctx| picker_vm.apply_source_defaults(res)),
                    )
                    .child(Self::hint(tr!(import_plume_source_hint()))),
            )
            .full_width(Divider::new())
            // ── Destination folder ────────────────────────────────────────
            .line(
                Self::field_label(tr!(import_plume_location())),
                FilePickerField::new(vm.location())
                    .kind(FilePickerKind::PickFolder)
                    .validation(vm.location_validation()),
            )
            // ── Output name + "Will create …" preview ─────────────────────
            .line(
                Self::field_label(tr!(import_plume_name())),
                VStack::new()
                    .spacing(6.0)
                    .child(
                        TextInput::new(vm.name())
                            .placeholder(tr!(import_plume_name_placeholder()))
                            .validation(vm.name_validation()),
                    )
                    .child(self.path_preview()),
            )
            // ── Trashed-items warning (persistent) ────────────────────────
            .full_width(
                TextWidget::new(tr!(import_plume_trash_warning()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Warning),
            )
    }
}

impl std::fmt::Debug for ImportPlumePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportPlumePanel").finish()
    }
}

impl Widget for ImportPlumePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let body = ScrollArea::new().child(Padding::symmetric(20.0, 22.0).child(self.form()));

        let import_vm = self.vm.clone();
        let import_can = self.vm.can_import();

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
                        Expand::horizontal {
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(tr!(import_plume_title())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        IconButton::clear() {
                                            tooltip: tr!(import_plume_close())
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
                                    }
                                }
                            }
                        }
                        Expand::horizontal {
                            Divider
                        }
                        Expand::vertical {
                            child: body
                        }
                        Expand::horizontal {
                            Divider
                        }
                        FixedSize {
                            height: 56.0
                            Padding::symmetric(10.0, 22.0) {
                                HStack {
                                    spacing: 9.0
                                    Spacer
                                    Button::new(tr!(import_plume_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                    Button::new(tr!(import_plume_import())) {
                                        variant: ButtonVariant::Filled
                                        enabled: import_can
                                        on_activate_fn: move |ctx| import_vm.import(ctx)
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

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use std::rc::Rc;

    /// The whole panel — header, the `FormLayout` body (two `FilePickerField`s,
    /// a `TextInput`, the preview, the warning), and the footer — must build and
    /// lay out headlessly without panicking, at the modal's card size.
    #[test]
    fn panel_builds_and_lays_out() {
        let vm = ImportPlumeViewModel::new(Rc::new(AppContext::new()));
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(ImportPlumePanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }
}
