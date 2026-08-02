// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Document ▸ **Save as template…** — name the note you are in and keep its prose.
//!
//! A hand-built panel rather than a `MessageBox`: the name has to be validated **as it is
//! typed**, with the confirm button disabled until it is usable, and a `MessageBox` has no
//! field to validate. The wiring is the tag pane's add-row shape (a `TextInput`, a
//! `Signal<ValidationState>` pushed from an effect, a `Button::enabled` bound to a signal)
//! lifted into a modal.
//!
//! **A duplicate name is refused here, not merely warned** — the opposite of the settings
//! pane's inline rename. The difference is transient state: mid-rename two rows may
//! legitimately share a name for a keystroke, so refusing there would fight the writer.
//! Naming a *new* template has no such in-between, and the name is what the insert menu is
//! picked by, so two identical entries would be genuinely ambiguous.

use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, DialogContent, Expand, HStack, Spacer, TextInput, Toast, ValidationState,
};

use crate::app_ids::HasWorkId;
use crate::toast_scope::ToastWorkExt;
use crate::view_models::NoteTemplatesViewModel;

/// The modal's body: an explanation, the name field, and the two footer buttons.
pub struct SaveAsTemplatePanel {
    vm: NoteTemplatesViewModel,
    /// The note's prose, captured when the command fired — deliberately not re-read at
    /// confirm time, so what is saved is what the writer was looking at when they asked.
    body: String,
    root_child: Option<WidgetId>,
}

impl SaveAsTemplatePanel {
    pub fn new(vm: NoteTemplatesViewModel, body: String) -> Self {
        Self {
            vm,
            body,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for SaveAsTemplatePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaveAsTemplatePanel").finish()
    }
}

impl Widget for SaveAsTemplatePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let name = Signal::new(String::new());
        let validation = Signal::new(ValidationState::None);
        // Concrete, not derived: it has to fall on a *collision* as well as on emptiness,
        // and a `.map()` over `name` cannot see the view-model. Pushed from the same effect
        // that sets the validation state, so the two can never disagree.
        let can_save = Signal::new(false);

        {
            let vm = self.vm.clone();
            let validation = validation.clone();
            let can_save = can_save.clone();
            ctx.effect(&name, move |typed| {
                let trimmed = typed.trim();
                let (state, ok) = if trimmed.is_empty() {
                    // Not an error while the field is untouched — shouting at an empty
                    // field before anything is typed is noise. The disabled button is what
                    // says "not yet".
                    (ValidationState::None, false)
                } else if let Some(clash) = vm.duplicate_name(trimmed, None) {
                    (
                        ValidationState::Error(tr!(save_as_template_duplicate(name = clash))),
                        false,
                    )
                } else {
                    (ValidationState::None, true)
                };
                validation.set(state);
                can_save.set(ok);
            });
        }

        let confirm = {
            let vm = self.vm.clone();
            let name = name.clone();
            let body = self.body.clone();
            move |c: &mut EventContext| {
                let typed = name.get().trim().to_string();
                // Re-validated inside the view-model too: a race between the last keystroke
                // and this click must not be able to create a duplicate.
                match vm.save_as_template(&typed, &body) {
                    Ok(_) => {
                        c.show_toast(
                            Toast::info(tr!(save_as_template_saved(name = typed)))
                                .scoped_id("templates.saved", vm.work_id())
                                .target_work(vm.work_id()),
                        );
                        c.dismiss_top_overlay();
                    }
                    Err(e) => {
                        c.show_toast(
                            Toast::warning(lit!(format!("{e:#}")))
                                .scoped_id("templates.error", vm.work_id())
                                .target_work(vm.work_id()),
                        );
                    }
                }
            }
        };

        let field = {
            let confirm = confirm.clone();
            let can_save = can_save.clone();
            TextInput::new(name.clone())
                .placeholder(tr!(save_as_template_placeholder()))
                .validation(validation.clone())
                // Enter confirms, but only when the name is usable — otherwise Enter on a
                // colliding name would bypass the disabled button.
                .on_submit_fn(move |c| {
                    if can_save.get() {
                        confirm(c);
                    }
                })
        };

        let save_btn = {
            let confirm = confirm.clone();
            Button::new(tr!(save_as_template_confirm()))
                .variant(ButtonVariant::Filled)
                .enabled(can_save.clone())
                .on_activate_fn(move |c| confirm(c))
        };

        let footer = HStack::new()
            .spacing(8.0)
            .child(Expand::horizontal().child(Spacer::new()))
            .child(
                Button::new(tr!(save_as_template_cancel()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(|c| c.dismiss_top_overlay()),
            )
            .child(save_btn);

        let content = DialogContent::new()
            .title(tr!(save_as_template_title()))
            .supporting_text(tr!(save_as_template_explain()))
            .body(bastyde::widgets::VStack::new().spacing(12.0).child(field))
            .footer(footer);

        let root = ctx.add(content);
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}
