//! The dictionary **licence** modal — shown to read a licence (from a "View licence" button)
//! and to accept one before downloading (the licence gate, §7 of the plan).
//!
//! The full licence text is a **bundled** asset (never fetched), so the modal always works
//! offline and the exact text the user accepts is a permanent, immutable record. `MessageBox`
//! is deliberately not used — licence texts run to a full page and need scrolling, which a
//! message box does not provide; this is the `BackupsListPanel`-shaped scrolling modal card.

use std::rc::Rc;

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::PanelVariant;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel,
    ScrollArea, Spacer, TextWidget, VStack,
};

use crate::dictionary_registry;

const CARD_W: f32 = 660.0;
const CARD_H: f32 = 560.0;

/// Whether the modal is a read-only view or the accept-before-download gate.
#[derive(Clone)]
enum Mode {
    /// "View licence" — a single Close button.
    View,
    /// Accept-before-download — Cancel + "Accept & Download"; the callback runs on Accept.
    Accept(Rc<dyn Fn(&mut EventContext)>),
}

/// Present the licence for `id` read-only ("View licence").
pub fn present_license_view(ctx: &mut EventContext, id: &str) {
    present(ctx, id, Mode::View);
}

/// Present the licence for `id` with an **Accept & Download** action. `on_accept` runs when the
/// user accepts (it records acceptance and starts the download).
pub fn present_license_accept(
    ctx: &mut EventContext,
    id: &str,
    on_accept: impl Fn(&mut EventContext) + 'static,
) {
    present(ctx, id, Mode::Accept(Rc::new(on_accept)));
}

fn present(ctx: &mut EventContext, id: &str, mode: Mode) {
    let title = dictionary_registry::by_id(id)
        .map(|e| e.display_name.clone())
        .unwrap_or_else(|| id.to_string());
    let id = id.to_string();
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(LicensePanel::new(id.clone(), mode.clone())))
            .presentation(ModalPresentation::InTree)
            .title(title)
            .size(CARD_W as u32, CARD_H as u32)
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct LicensePanel {
    id: String,
    mode: Mode,
    root_child: Option<WidgetId>,
}

impl LicensePanel {
    fn new(id: String, mode: Mode) -> Self {
        Self {
            id,
            mode,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for LicensePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LicensePanel").field("id", &self.id).finish()
    }
}

impl Widget for LicensePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let entry = dictionary_registry::by_id(&self.id);
        let display = entry
            .map(|e| e.display_name.clone())
            .unwrap_or_else(|| self.id.clone());
        let license_name = entry
            .map(|e| e.license_name.clone())
            .unwrap_or_default();
        let text = entry
            .and_then(|e| dictionary_registry::license_text(&e.license_asset))
            .unwrap_or("This dictionary's licence text is unavailable.");

        // The scrollable licence body — a plain, wrapping, read-only text block.
        let body = ScrollArea::new()
            .child(Padding::symmetric(16.0, 18.0).child(
                TextWidget::new(lit!(text.to_string())).style(TextStyleRole::Small),
            ));

        // The footer buttons depend on the mode. Both arms are an `HStack` so the `child:`
        // slot below sees one concrete widget type.
        let footer = match &self.mode {
            Mode::View => HStack::new().spacing(8.0).child(Spacer::new()).child(
                Button::new(tr!(dict_license_close()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(|ctx| ctx.dismiss_modal()),
            ),
            Mode::Accept(cb) => {
                let cb = cb.clone();
                HStack::new()
                    .spacing(8.0)
                    .child(Spacer::new())
                    .child(
                        Button::new(tr!(dict_license_cancel()))
                            .variant(ButtonVariant::Plain)
                            .on_activate_fn(|ctx| ctx.dismiss_modal()),
                    )
                    .child(
                        Button::new(tr!(dict_license_accept()))
                            .variant(ButtonVariant::Filled)
                            // Dismiss first, then run accept+download so the toast that
                            // replaces this modal isn't immediately torn down with it.
                            .on_activate_fn(move |ctx| {
                                ctx.dismiss_modal();
                                cb(ctx);
                            }),
                    )
            }
        };

        let root = bati!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    // Header: dictionary name + licence name, and a close button.
                    Expand::horizontal {
                        FixedSize {
                            height: 52.0
                            Padding::symmetric(8.0, 16.0) {
                                HStack {
                                    spacing: 8.0
                                    Expand::horizontal {
                                        VStack {
                                            spacing: 2.0
                                            TextWidget::new(lit!(display)) {
                                                style: TextStyleRole::BodyBold
                                            }
                                            TextWidget::new(lit!(license_name)) {
                                                style: TextStyleRole::Tiny
                                                color: TextRole::Secondary
                                            }
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(dict_license_close())
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                }
                            }
                        }
                    }
                    Expand::horizontal { Divider }
                    Expand::vertical { child: body }
                    Expand::horizontal { Divider }
                    Expand::horizontal {
                        FixedSize {
                            height: 56.0
                            Padding::symmetric(12.0, 20.0) { child: footer }
                        }
                    }
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
}
