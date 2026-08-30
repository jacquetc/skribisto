// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Compile & Export ▸ Paratext structures.
//!
//! The catalogue of front/back-matter structures a new project can start from: the four
//! shipped traditions, read-only, plus the writer's own.
//!
//! ## Why a code editor and not a form
//!
//! A preset is a name and two lists of page titles. A form over that would be a list
//! editor with add/remove/reorder buttons on each side — more chrome than content, and it
//! would still round-trip through the same TOML. The file *is* the interface, so the pane
//! edits it as text in a modal [`CodeEditor`], the way the Corkboard opens its synopsis
//! editor.
//!
//! It also means a writer can paste in a structure someone sent them, which is how a
//! tradition we do not ship gets used.
//!
//! ## Saving validates
//!
//! The modal refuses TOML that does not parse and shows the error under the editor,
//! keeping what was typed. The loader *skips* a broken preset — right on load, where the
//! alternative is a New Work dialog that will not open — but that makes a silent failure
//! here: a preset gone from the picker with nothing to explain it. Catching it at the one
//! moment the writer is looking at the text is the point.
//!
//! A preset that broke some other way (a hand edit, a bad merge) is still listed, with its
//! error, so there is somewhere to go and fix it.

use teksilo::core::binding::BindingLevel;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::tokens::{BorderRole, SurfaceRole};
use teksilo::widgets::{
    Button, ButtonVariant, CodeEditor, Divider, FixedSize, GroupHeader, HStack, MaxSize, Padding,
    Panel, ScrollArea, Spacer, TextWidget, VStack,
};

use crate::export::{ParatextPresetsViewModel, PresetRow};
use crate::models::{NEW_PRESET_TEMPLATE, ParatextPreset};

/// Room for a preset without scrolling: the metadata block plus two lists.
const EDITOR_W: f32 = 620.0;
const EDITOR_H: f32 = 420.0;

pub fn paratext_pane(ctx: &mut BuildContext, vm: &ParatextPresetsViewModel) -> impl Widget {
    // Rebuild the pane on every mutation. The list is a handful of rows read straight out
    // of a file that is edited whole, so re-deriving it is simpler and cheaper than
    // keeping a reactive model in step with it.
    vm.changed_signal()
        .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

    let mut list = VStack::new().spacing(0.0);
    for (i, row) in vm.rows().into_iter().enumerate() {
        if i > 0 {
            list = list.child(Divider::new());
        }
        list = list.child(preset_row(vm, row));
    }

    let new_vm = vm.clone();
    VStack::new()
        .spacing(12.0)
        .child(
            TextWidget::new(tr!(settings_paratext_intro()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            GroupHeader::new(tr!(settings_paratext_structures()))
                .style(TextStyleRole::SmallBold)
                .color(TextRole::Secondary),
        )
        .child(
            Panel::new()
                .background(SurfaceRole::Content)
                .border_color(BorderRole::Default)
                .border_width(1.0)
                .corner_radius(8.0)
                .padding(0.0)
                .child(list),
        )
        .child(
            HStack::new().child(
                Button::new(tr!(settings_paratext_new()))
                    .variant(ButtonVariant::Tinted)
                    .on_activate_fn(move |c| {
                        open_editor(c, &new_vm, None, NEW_PRESET_TEMPLATE.to_string())
                    }),
            ),
        )
}

/// One row: what the preset calls itself, how big it is, and what can be done to it.
fn preset_row(vm: &ParatextPresetsViewModel, row: PresetRow) -> impl Widget {
    let broken = row.error.is_some();
    let name: LocalizedString = if broken {
        // A broken preset has no name to show — saying so beats a blank row the writer
        // cannot identify.
        tr!(settings_paratext_broken())
    } else {
        lit!(row.name())
    };
    let subtitle: LocalizedString = match (&row.error, &row.preset) {
        (Some(e), _) => lit!(e.clone()),
        (None, Some(p)) => lit!(summary(p)),
        (None, None) => lit!(String::new()),
    };

    let mut actions = HStack::new().spacing(8.0);
    match row.user_index {
        Some(index) => {
            let edit_vm = vm.clone();
            let source = row.source.clone();
            let del_vm = vm.clone();
            actions = actions
                .child(
                    Button::new(tr!(settings_paratext_edit()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(move |c| {
                            open_editor(c, &edit_vm, Some(index), source.clone())
                        }),
                )
                .child(
                    Button::new(tr!(settings_paratext_delete()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(move |c| {
                            if let Err(e) = del_vm.remove(index) {
                                c.show_toast(Toast::error(lit!(e)));
                            }
                        }),
                );
        }
        None => {
            // A shipped preset is read-only, so making one your own means starting from
            // its text rather than retyping it.
            let dup_vm = vm.clone();
            let id = row
                .preset
                .as_ref()
                .map(|p| p.id.clone())
                .unwrap_or_default();
            actions = actions.child(
                Button::new(tr!(settings_paratext_duplicate()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |c| {
                        let source = dup_vm.bundled_source(&id).unwrap_or_default();
                        open_editor(c, &dup_vm, None, source);
                    }),
            );
        }
    }

    Padding::symmetric(12.0, 10.0).child(
        HStack::new()
            .spacing(12.0)
            .child(
                VStack::new()
                    .spacing(2.0)
                    .child(TextWidget::new(name))
                    .child(TextWidget::new(subtitle).style(TextStyleRole::Small).color(
                        if broken {
                            TextRole::Error
                        } else {
                            TextRole::Secondary
                        },
                    )),
            )
            .child(Spacer::new())
            .child(actions),
    )
}

/// "6 · 3" — front and back counts, enough to tell two traditions apart at a glance
/// without listing every page.
fn summary(p: &ParatextPreset) -> String {
    format!("{} · {}", p.front.len(), p.back.len())
}

/// Open the editor. `index` is `Some` when overwriting a user preset in place and `None`
/// when adding one — which is what makes *new* and *duplicate* the same path.
fn open_editor(
    ctx: &mut EventContext,
    vm: &ParatextPresetsViewModel,
    index: Option<usize>,
    source: String,
) {
    let vm = vm.clone();
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(PresetEditorModal {
                vm: vm.clone(),
                index,
                source: source.clone(),
                doc: None,
                error: Signal::new(String::new()),
                root: None,
            })
        })
        .presentation(ModalPresentation::InTree)
        // Explicit only: an outside click on an editor holding unsaved text would throw
        // it away, which is the one thing this surface must not do.
        .close_behavior(ModalCloseBehavior::Manual),
    );
}

struct PresetEditorModal {
    vm: ParatextPresetsViewModel,
    index: Option<usize>,
    source: String,
    /// Created once, on first build — a fresh document per rebuild would throw away
    /// whatever the writer had typed the moment a parse error re-rendered the surface.
    doc: Option<TextDocument>,
    error: Signal<String>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for PresetEditorModal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PresetEditorModal").finish()
    }
}

impl Widget for PresetEditorModal {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Re-render when a save is refused, so the error appears under the editor.
        self.error
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let doc = self.doc.get_or_insert_with(|| {
            let doc = TextDocument::new();
            // Plain text, not djot: this is TOML, and `set_djot` would parse it as markup.
            let _ = doc.set_plain_text(&self.source);
            doc
        });

        let error = self.error.get();
        let save_vm = self.vm.clone();
        let save_doc = doc.clone();
        let save_error = self.error.clone();
        let index = self.index;

        let root = ctx.add(
            Panel::new()
                .variant(PanelVariant::Raised)
                .corner_radius(10.0)
                .padding(16.0)
                .child(
                    VStack::new()
                        .spacing(10.0)
                        // Capped at the editor's own width. This modal is presented
                        // `InTree`, outside the pane column that bounds every other
                        // wrapping paragraph in this window, so its `VStack`'s cross
                        // axis is `max(EDITOR_W, the hint's natural one-line width)` —
                        // and a 190-character hint measured unbounded is ~700 px, a
                        // modal wider than the editor it explains and plausibly wider
                        // than the card behind it.
                        .child(
                            MaxSize::width(EDITOR_W).child(
                                TextWidget::new(tr!(settings_paratext_editor_hint()))
                                    .style(TextStyleRole::Small)
                                    .color(TextRole::Secondary),
                            ),
                        )
                        // A bounded box, so the greedy editor learns a height to fill and
                        // scrolls inside it rather than overflowing the modal.
                        .child(
                            FixedSize::new().width(EDITOR_W).height(EDITOR_H).child(
                                ScrollArea::new().child(
                                    CodeEditor::new(doc.clone())
                                        .gutter(true)
                                        .current_line_highlight(true),
                                ),
                            ),
                        )
                        .child(
                            TextWidget::new(lit!(error))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Error),
                        )
                        .child(
                            HStack::new()
                                .spacing(8.0)
                                .child(Spacer::new())
                                .child(
                                    Button::new(tr!(settings_cancel()))
                                        .variant(ButtonVariant::Plain)
                                        .on_activate_fn(|c| c.dismiss_top_overlay()),
                                )
                                .child(
                                    Button::new(tr!(settings_paratext_save()))
                                        .variant(ButtonVariant::Filled)
                                        .on_activate_fn(move |c| {
                                            let text = save_doc.to_plain_text().unwrap_or_default();
                                            let saved = match index {
                                                Some(i) => save_vm.replace(i, &text),
                                                None => save_vm.add(&text),
                                            };
                                            match saved {
                                                Ok(()) => c.dismiss_top_overlay(),
                                                // Kept open with the reason attached:
                                                // closing on a refused save would throw
                                                // away what they wrote.
                                                Err(e) => save_error.set(e),
                                            }
                                        }),
                                ),
                        ),
                ),
        );
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _p: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // Fill our own bounds: the default place would leave the greedy editor at its
        // measured fallback height inside the FixedSize box, overflowing the modal.
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
