// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two small dialogs of the Resources dock: Add a web link (its address and a title)
//! and Rename and describe (a title and a description).
//!
//! One two-field modal serves both, on the shape of the Link dialog
//! ([`crate::format::link_panel`]): `InputDialog` holds one string, and each of these
//! needs two. What each field means, and what committing does, is handed in; the panel
//! only lays the fields out and wires the buttons.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, FormLayout, HStack, ModalContainer, Spacer, TextInput, VStack,
};

use crate::models::ResourceRow;
use crate::resources::ResourcesViewModel;
use crate::shared::text::field_label;

const CARD_W: u32 = 460;
const CARD_H: u32 = 200;

/// What committing the two fields does. `true` closes the dialog; `false` leaves it up,
/// for a value the commit refused and said why.
type CommitFn = Rc<dyn Fn(&mut EventContext, &str, &str) -> bool>;

/// Ask for a web address and its title, and attach it to the item in front of the
/// writer.
pub fn present_add_link(vm: &ResourcesViewModel, ctx: &mut EventContext, url: &str) {
    let vm = vm.clone();
    let commit: CommitFn = Rc::new(move |c, url, title| vm.add_web_link(c, url, title));
    present(
        ctx,
        Spec {
            title: tr!(resources_link_dialog_title()),
            first_label: tr!(resources_link_dialog_address()),
            first_placeholder: tr!(resources_link_dialog_address_placeholder()),
            first: url.to_string(),
            first_is_address: true,
            second_label: tr!(resources_link_dialog_name()),
            second_placeholder: tr!(resources_link_dialog_name_placeholder()),
            second: String::new(),
            confirm: tr!(resources_link_dialog_add()),
            commit,
        },
    );
}

/// Rename `row` and describe it.
pub fn present_edit(vm: &ResourcesViewModel, ctx: &mut EventContext, row: &ResourceRow) {
    let vm = vm.clone();
    let id = row.id;
    let commit: CommitFn = Rc::new(move |c, title, description| {
        vm.edit_text(c, id, title, description);
        true
    });
    present(
        ctx,
        Spec {
            title: tr!(resources_edit_dialog_title()),
            first_label: tr!(resources_edit_dialog_name()),
            first_placeholder: lit!(row.display_name().to_string()),
            first: row.title.clone(),
            first_is_address: false,
            second_label: tr!(resources_edit_dialog_description()),
            second_placeholder: tr!(resources_edit_dialog_description_placeholder()),
            second: row.description.clone(),
            confirm: tr!(resources_edit_dialog_save()),
            commit,
        },
    );
}

struct Spec {
    title: LocalizedString,
    first_label: LocalizedString,
    first_placeholder: LocalizedString,
    first: String,
    /// The first field holds an address, read left to right in every interface.
    first_is_address: bool,
    second_label: LocalizedString,
    second_placeholder: LocalizedString,
    second: String,
    confirm: LocalizedString,
    commit: CommitFn,
}

fn present(ctx: &mut EventContext, spec: Spec) {
    let title = spec.title.clone();
    let spec = Rc::new(spec);
    ctx.present_modal(
        ModalRequest::deferred({
            let title = title.clone();
            move |t| t.add(ModalContainer::new(TwoFields::new(spec.clone())).title(title.clone()))
        })
        .presentation(ModalPresentation::Auto)
        .title(title)
        .size(CARD_W, CARD_H)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct TwoFields {
    spec: Rc<Spec>,
    first: Signal<String>,
    second: Signal<String>,
    root: Option<WidgetId>,
}

impl TwoFields {
    fn new(spec: Rc<Spec>) -> Self {
        Self {
            first: Signal::new(spec.first.clone()),
            second: Signal::new(spec.second.clone()),
            spec,
            root: None,
        }
    }
}

impl std::fmt::Debug for TwoFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwoFields").finish()
    }
}

impl Widget for TwoFields {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // No rebuild on typing, for the reason the Link dialog gives: the only thing
        // that reacts is the confirm button's enablement, which takes a derived signal.
        let spec = &self.spec;
        let first_field = if spec.first_is_address {
            crate::shared::address_input::address_input(self.first.clone())
        } else {
            TextInput::new(self.first.clone())
        }
        .placeholder(spec.first_placeholder.clone());
        let form = FormLayout::new()
            .label_gap(12.0)
            .row_spacing(12.0)
            .line(field_label(spec.first_label.clone()), first_field)
            .line(
                field_label(spec.second_label.clone()),
                TextInput::new(self.second.clone()).placeholder(spec.second_placeholder.clone()),
            );

        // An address is required; a title may be left blank (the dock then shows what
        // the backend derives: the site's name, or the file's).
        let committable = if spec.first_is_address {
            self.first.map(|s| !s.trim().is_empty())
        } else {
            self.first.map(|_| true)
        };
        let (first, second, commit) =
            (self.first.clone(), self.second.clone(), spec.commit.clone());
        let confirm = spec.confirm.clone();
        let actions = teksu!(
            HStack {
                spacing: 8.0
                Spacer
                Button::new(tr!(resources_dialog_cancel())) {
                    variant: ButtonVariant::Plain
                    on_activate_fn: |c| c.dismiss_modal()
                }
                Button::new(confirm) {
                    variant: ButtonVariant::Filled
                    enabled: committable
                    on_activate_fn: move |c| {
                        if commit(c, &first.get(), &second.get()) {
                            c.dismiss_modal();
                            c.request_frame();
                        }
                    }
                }
            }
        );
        let root = ctx.add(teksu!(
            VStack {
                spacing: 16.0
                child: form
                child: actions
            }
        ));
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
