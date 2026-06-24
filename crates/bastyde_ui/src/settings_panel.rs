//! Settings modal: language (en/fr) and theme (light/dark), on a raised Panel.
//!
//! Presented as an in-tree modal (see `app.rs`). Choices apply live
//! (`set_locale` / `set_theme`) and persist via the `SettingsStore`; they are
//! re-applied at startup in `main`.

use bastyde::core::styles::PanelVariant;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::settings::SettingsExt;
use bastyde::widgets::{Button, Divider, HStack, MinSize, Panel, Slider, TextWidget, VStack};

use crate::{DARK_KEY, EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY, LOCALE_KEY};

pub struct SettingsPanel {
    root_child: Option<WidgetId>,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self { root_child: None }
    }
}

impl std::fmt::Debug for SettingsPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsPanel").finish()
    }
}

impl Widget for SettingsPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Persisted, shared width of the centered editor column (bound to the
        // slider below; every open editor reads the same signal and resizes live).
        let column_width = ctx.settings().signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT);

        // The raised Panel is the modal's card background. Its centered placement
        // comes from `layout_response` reporting a fixed compact size (below) —
        // the in-tree modal centers on the content's measured size.
        let root = bati!(ctx => Panel {
                variant: PanelVariant::Raised
                corner_radius: 8.0
                padding: 16.0
                VStack {
                    spacing: 10.0
                    TextWidget::new(tr!(language()))
                    HStack {
                        spacing: 8.0
                        Button::new(tr!(english())) {
                            on_activate_fn: |ctx| {
                                ctx.set_locale("en-US");
                                ctx.settings()
                                    .signal(LOCALE_KEY, "en-US".to_string())
                                    .set("en-US".to_string());
                            }
                        }
                        Button::new(tr!(french())) {
                            on_activate_fn: |ctx| {
                                ctx.set_locale("fr-FR");
                                ctx.settings()
                                    .signal(LOCALE_KEY, "en-US".to_string())
                                    .set("fr-FR".to_string());
                            }
                        }
                    }
                    Divider
                    TextWidget::new(tr!(theme()))
                    HStack {
                        spacing: 8.0
                        Button::new(tr!(light())) {
                            on_activate_fn: |ctx| {
                                ctx.set_theme(intui::light());
                                ctx.settings().signal(DARK_KEY, false).set(false);
                            }
                        }
                        Button::new(tr!(dark())) {
                            on_activate_fn: |ctx| {
                                ctx.set_theme(intui::dark());
                                ctx.settings().signal(DARK_KEY, false).set(true);
                            }
                        }
                    }
                    Divider
                    TextWidget::new(lit!("Text width"))
                    MinSize::width(360.0) {
                        Slider::new(column_width, 400.0, 1200.0) {
                            step: 20.0
                        }
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        // Report a fixed compact card size. The in-tree modal measures content
        // with an *unspecified* proposal and centers on the result, so we must
        // return a bounded size here rather than delegating to the (greedy)
        // Panel — otherwise it spans the window and reads as "not centered".
        Size::new(proposal.width.unwrap_or(480.0), proposal.height.unwrap_or(360.0)).into()
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
