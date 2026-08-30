// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Import Manuskript modal — pick a Manuskript project and a destination,
//! then convert it into a new `.skrib`.
//!
//! Presented as an in-tree modal (see the `work.import_manuskript` action in
//! `app::commands::file`), mirroring the Plume panel's chrome (title strip +
//! close, full-width rule, bottom action bar) and structure (a two-column
//! [`FormLayout`]). All logic lives on [`ImportManuskriptViewModel`]; this view is
//! thin.
//!
//! # Why the source row is not a `FilePickerField`
//!
//! A Manuskript project is a `.msk` **or** a folder, and `FilePickerField` opens
//! one kind of dialog. So the source row is a plain path field with **two** Browse
//! buttons beside it, one per shape, both writing into the same signal. The
//! alternative was a combined picker kind in the framework, which would be a
//! change to Teksilo for one caller.

use teksilo::core::styles::PanelVariant;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FilePickerField, FilePickerKind, FixedSize, FormLayout,
    HStack, IconButton, Padding, Panel, ScrollArea, Spacer, TextInput, TextWidget, VStack,
};

use crate::import_manuskript::ImportManuskriptViewModel;

const CARD_W: f32 = 600.0;
const CARD_H: f32 = 500.0;

/// Present the Import Manuskript modal over whichever window asked for it.
///
/// **Two windows fire this, and each registers its own action for it** — the
/// project window's Work ▸ Import from ▸ Manuskript and the Launcher's Create
/// from ▸ Manuskript. They must, because each `WidgetTree` owns its own
/// `global_actions` and the Launcher never builds an `App`; what they must *not*
/// do is grow two copies of what the command does, which is why the body lives
/// here.
///
/// The view-model is the shared, app-state one — the same instance the long
/// operation's events are routed to — reset first, so a previous session's paths
/// do not linger in the form. An import needs no open project (it writes a
/// brand-new `.skrib` and touches no store entity), which is exactly why the
/// Launcher can offer it at all.
pub(crate) fn present_import_manuskript(ctx: &mut EventContext) {
    let Some(vm) = ctx.app_state::<ImportManuskriptViewModel>().cloned() else {
        return;
    };
    vm.reset_form();
    ctx.present_modal(
        teksilo::core::modal::ModalRequest::deferred(move |t| {
            t.add(ImportManuskriptPanel::new(vm))
        })
        .presentation(teksilo::core::modal::ModalPresentation::InTree)
        .title("Import Manuskript project")
        .close_behavior(teksilo::core::modal::ModalCloseBehavior::EscapeOrClickOutside)
        .size(600, 500),
    );
}

pub struct ImportManuskriptPanel {
    vm: ImportManuskriptViewModel,
    root_child: Option<WidgetId>,
    /// The first form field, captured during `build` and handed to the modal
    /// pipeline by [`Widget::initial_focus_hint`] — see the note there.
    first_field: std::cell::Cell<Option<WidgetId>>,
}

impl ImportManuskriptPanel {
    pub fn new(vm: ImportManuskriptViewModel) -> Self {
        Self {
            vm,
            root_child: None,
            first_field: std::cell::Cell::new(None),
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
                TextWidget::new(tr!(import_manuskript_will_create()))
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

    /// The path field plus its two Browse buttons.
    ///
    /// Both write into the same signal and both default the destination, so the
    /// two shapes of a Manuskript project are one field to the writer. Each
    /// request is built inside its own click, where the `EventContext` that knows
    /// the remembered directory exists.
    fn source_row(&self) -> impl Widget + 'static {
        let file_vm = self.vm.clone();
        let folder_vm = self.vm.clone();

        HStack::new()
            .spacing(8.0)
            .child(
                Expand::horizontal().child(
                    TextInput::new(self.vm.source()).validation(self.vm.source_validation()),
                ),
            )
            .child(
                Button::new(tr!(import_manuskript_source_file()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |ctx| {
                        let vm = file_vm.clone();
                        let request = crate::models::dialog_start_in(
                            ctx,
                            crate::models::FolderPurpose::ImportManuskript,
                            FileDialogRequest::pick_file()
                                .title("Choose a Manuskript project")
                                // A file-dialog filter label — a plain string,
                                // like the other dialogs, not a localized key.
                                .add_filter("Manuskript project", &["msk"]),
                        );
                        let _ = ctx.pick_file(request, move |res, c| {
                            crate::models::remember_pick(
                                c,
                                crate::models::FolderPurpose::ImportManuskript,
                                &res,
                            );
                            vm.apply_source_defaults(&res);
                        });
                    }),
            )
            .child(
                Button::new(tr!(import_manuskript_source_folder()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |ctx| {
                        let vm = folder_vm.clone();
                        let request = crate::models::dialog_start_in(
                            ctx,
                            crate::models::FolderPurpose::ImportManuskript,
                            FileDialogRequest::pick_folder()
                                .title("Choose a Manuskript project folder"),
                        );
                        let _ = ctx.pick_folder(request, move |res, c| {
                            crate::models::remember_pick(
                                c,
                                crate::models::FolderPurpose::ImportManuskript,
                                &res,
                            );
                            vm.apply_source_defaults(&res);
                        });
                    }),
            )
    }

    fn form(&self) -> impl Widget + 'static {
        let vm = &self.vm;
        FormLayout::new()
            .label(tr!(import_manuskript_title()))
            .label_gap(16.0)
            .row_spacing(18.0)
            // ── The project: a .msk of either kind, or the folder itself ──
            .line(
                Self::field_label(tr!(import_manuskript_source())),
                VStack::new()
                    .spacing(6.0)
                    .child(self.source_row())
                    .child(Self::hint(tr!(import_manuskript_source_hint()))),
            )
            .full_width(Divider::new())
            // ── Destination folder ────────────────────────────────────────
            .line(
                Self::field_label(tr!(import_manuskript_location())),
                FilePickerField::new(vm.location())
                    .kind(FilePickerKind::PickFolder)
                    .validation(vm.location_validation()),
            )
            // ── Output name + "Will create …" preview ─────────────────────
            .line(
                Self::field_label(tr!(import_manuskript_name())),
                VStack::new()
                    .spacing(6.0)
                    .child(
                        TextInput::new(vm.name())
                            .placeholder(tr!(import_manuskript_name_placeholder()))
                            .validation(vm.name_validation()),
                    )
                    .child(self.path_preview()),
            )
    }
}

impl std::fmt::Debug for ImportManuskriptPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportManuskriptPanel").finish()
    }
}

impl Widget for ImportManuskriptPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Build the form first so its first focusable descendant can be captured
        // for `initial_focus_hint` (see the note there).
        let form = self.form();
        let form_id = ctx.add(form);
        self.first_field
            .set(ctx.first_focusable_descendant(form_id));

        let body = ScrollArea::new().child(Padding::symmetric(20.0, 22.0).child_id(form_id));

        let import_vm = self.vm.clone();
        let import_can = self.vm.can_import();

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
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(tr!(import_manuskript_title())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        IconButton::clear() {
                                            tooltip: tr!(import_manuskript_close())
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
                                    Button::new(tr!(import_manuskript_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                    Button::new(tr!(import_manuskript_import())) {
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

    /// Open with the first form field focused, so the dialog is typeable the
    /// moment it appears.
    ///
    /// The modal pipeline's fallback is `first_focusable_descendant` of the whole
    /// panel, and this panel draws its own header chrome above the form — so the
    /// fallback would pick the close button, and the dialog would open focused on
    /// "dismiss me" and swallow whatever was typed first.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.first_field.get()
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
    use frontend::AppContext;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;

    /// The whole panel — header, the `FormLayout` body (the two-button source
    /// row, the destination picker, the name field and its preview), and the
    /// footer — must build and lay out headlessly without panicking, at the
    /// modal's card size.
    #[test]
    fn panel_builds_and_lays_out() {
        let vm = ImportManuskriptViewModel::new(Rc::new(AppContext::new()));
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(ImportManuskriptPanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }
}
