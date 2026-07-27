// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Add dictionary** modal — install a Hunspell dictionary from local `.aff`/`.dic` files.
//!
//! Same chrome and shape as [`crate::panels::import_plume`]: a title strip, a two-column
//! [`FormLayout`] body, and a bottom action bar. All logic lives on [`AddDictionaryViewModel`];
//! this view binds its signals and forwards the buttons. Presented from the Installed tab of the
//! Dictionaries settings pane via [`present_add_dictionary`].

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::PanelVariant;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FilePickerField, FilePickerKind, FixedSize, FormLayout,
    HStack, IconButton, Padding, Panel, ScrollArea, Spacer, TextInput, TextWidget, VStack,
};

use crate::view_models::{AddDictionaryViewModel, DictionariesViewModel};

const CARD_W: f32 = 580.0;
const CARD_H: f32 = 500.0;

/// Present the Add-dictionary modal over the current window, form freshly reset.
pub fn present_add_dictionary(ctx: &mut EventContext, dicts: DictionariesViewModel) {
    let vm = AddDictionaryViewModel::new(dicts);
    vm.reset_form();
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(AddDictionaryPanel::new(vm.clone())))
            .presentation(ModalPresentation::InTree)
            .title(tr!(dict_add_title()))
            .size(CARD_W as u32, CARD_H as u32)
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

pub struct AddDictionaryPanel {
    vm: AddDictionaryViewModel,
    root_child: Option<WidgetId>,
    /// The first form field, captured during `build` and handed to the modal
    /// pipeline by [`Widget::initial_focus_hint`] — see the note there.
    first_field: std::cell::Cell<Option<WidgetId>>,
}

impl AddDictionaryPanel {
    pub fn new(vm: AddDictionaryViewModel) -> Self {
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

    fn form(&self) -> impl Widget + 'static {
        let vm = &self.vm;
        let picker_vm = self.vm.clone();

        FormLayout::new()
            .label(tr!(dict_add_title()))
            .label_gap(16.0)
            .row_spacing(18.0)
            // ── Name ──────────────────────────────────────────────────────
            .line(
                Self::field_label(tr!(dict_add_name())),
                TextInput::new(vm.name())
                    .placeholder(tr!(dict_add_name_placeholder()))
                    .validation(vm.name_validation()),
            )
            // ── Language code + what it's for ─────────────────────────────
            .line(
                Self::field_label(tr!(dict_add_code())),
                VStack::new()
                    .spacing(6.0)
                    .child(
                        TextInput::new(vm.code())
                            .placeholder(tr!(dict_add_code_placeholder()))
                            .validation(vm.code_validation()),
                    )
                    .child(Self::hint(tr!(dict_add_code_hint()))),
            )
            .full_width(Divider::new())
            // ── .aff (open-file picker; defaults the other fields on pick) ─
            .line(
                Self::field_label(tr!(dict_add_aff())),
                FilePickerField::new(vm.aff())
                    .kind(FilePickerKind::OpenFile)
                    .add_filter("Hunspell affix (.aff)", &["aff"])
                    .validation(vm.aff_validation())
                    .on_pick(move |res, _ctx| picker_vm.apply_aff_pick(res)),
            )
            // ── .dic (open-file picker) ───────────────────────────────────
            .line(
                Self::field_label(tr!(dict_add_dic())),
                FilePickerField::new(vm.dic())
                    .kind(FilePickerKind::OpenFile)
                    .add_filter("Hunspell dictionary (.dic)", &["dic"])
                    .validation(vm.dic_validation()),
            )
    }
}

impl std::fmt::Debug for AddDictionaryPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddDictionaryPanel").finish()
    }
}

impl Widget for AddDictionaryPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Build the form first so its first focusable descendant can be captured
        // for `initial_focus_hint` (see the note there).
        let form_id = ctx.add(self.form());
        self.first_field
            .set(ctx.first_focusable_descendant(form_id));

        let body = ScrollArea::new().child(Padding::symmetric(20.0, 22.0).child_id(form_id));

        let add_vm = self.vm.clone();
        let can_add = self.vm.can_add();

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
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(tr!(dict_add_title())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        IconButton::clear() {
                                            tooltip: tr!(dict_add_close())
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
                                    }
                                }
                            }
                        }
                        Expand::horizontal { Divider }
                        Expand::vertical { child: body }
                        Expand::horizontal { Divider }
                        FixedSize {
                            height: 56.0
                            Padding::symmetric(10.0, 22.0) {
                                HStack {
                                    spacing: 9.0
                                    Spacer
                                    Button::new(tr!(dict_add_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                    Button::new(tr!(dict_add_submit())) {
                                        variant: ButtonVariant::Filled
                                        enabled: can_add
                                        on_activate_fn: move |ctx| add_vm.add(ctx)
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
    /// panel, and this panel draws its own header chrome (title strip + close X)
    /// *above* the form — so the fallback picked the close button: the dialog
    /// opened focused on "dismiss me" and swallowed whatever the user typed first.
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
    use crate::models::{DictionarySettingsService, InstalledDictionariesModel};
    use bastyde::core::widget_tree::WidgetTree;

    /// The panel builds and lays out headlessly at its card size (the FormLayout body — two
    /// `TextInput`s, two `FilePickerField`s — plus header and footer).
    #[test]
    fn panel_builds_and_lays_out() {
        let settings = DictionarySettingsService::in_memory_default();
        let dicts =
            DictionariesViewModel::new(settings.clone(), InstalledDictionariesModel::new(settings));
        let vm = AddDictionaryViewModel::new(dicts);
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(AddDictionaryPanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }
}
