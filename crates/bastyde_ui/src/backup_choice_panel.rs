//! `BackupChoicePanel` — the modal shown when a **backup file** is opened.
//!
//! Two choices: **Open the backup** (the default — edit freely; changes can only
//! be kept with Save As; the original file is untouched) or **Restore this
//! project to this point** (overwrite the original, after a safety copy). The
//! window is already in backup mode on load, so "Open the backup" just dismisses.

use bastyde::core::styles::PanelVariant;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel, Spacer,
    TextWidget, VStack,
};

use crate::backup::BackupContext;
use crate::view_models::RestoreViewModel;

/// Restore button click: dismiss this modal, then start the restore flow.
fn on_restore(restore: &RestoreViewModel, ctx: &mut EventContext) {
    ctx.dismiss_modal();
    restore.begin(ctx);
}

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 320.0;

pub struct BackupChoicePanel {
    restore: RestoreViewModel,
    context: BackupContext,
    root_child: Option<WidgetId>,
}

impl BackupChoicePanel {
    pub fn new(restore: RestoreViewModel, context: BackupContext) -> Self {
        Self {
            restore,
            context,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for BackupChoicePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackupChoicePanel").finish()
    }
}

impl Widget for BackupChoicePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // A subtitle line naming the backup (date, if the manifest carried one).
        let subtitle = match &self.context.backup_created_at {
            Some(dt) => tr!(backup_choice_subtitle_dated(date = dt.clone())),
            None => tr!(backup_choice_subtitle()),
        };
        let restore = self.restore.clone();

        let root = bati!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        FixedSize {
                            height: 44.0
                            Padding::symmetric(8.0, 14.0) {
                                HStack {
                                    spacing: 8.0
                                    Expand::horizontal {
                                        TextWidget::new(tr!(backup_choice_title())) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(backup_choice_open())
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                }
                            }
                        }
                        Expand::horizontal {
                            Divider
                        }
                        Expand::vertical {
                            Padding::symmetric(24.0, 22.0) {
                                VStack {
                                    spacing: 12.0
                                    TextWidget::new(tr!(backup_choice_heading())) {
                                        style: TextStyleRole::BodyBold
                                    }
                                    TextWidget::new(subtitle) {
                                        style: TextStyleRole::Small
                                        color: TextRole::Secondary
                                    }
                                    TextWidget::new(tr!(backup_choice_body())) {
                                        color: TextRole::Secondary
                                    }
                                }
                            }
                        }
                        Expand::horizontal {
                            Divider
                        }
                        FixedSize {
                            height: 56.0
                            Padding::symmetric(10.0, 22.0) {
                                HStack {
                                    spacing: 9.0
                                    Button::new(tr!(backup_choice_restore())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: move |ctx| on_restore(&restore, ctx)
                                    }
                                    Spacer
                                    Button::new(tr!(backup_choice_open())) {
                                        variant: ButtonVariant::Filled
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
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

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use std::rc::Rc;

    #[test]
    fn choice_panel_builds_and_lays_out() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = crate::app_ids::AppIds::new();
        let single_work = crate::singles::SingleWork::new(app_ctx.clone());
        let restore = RestoreViewModel::new(
            app_ctx,
            ids,
            single_work,
            Signal::new(true),
            Signal::new(None),
        );
        let ctx = BackupContext {
            path: "/b/novel-20260101-120000.skrib".into(),
            backup_of: Some("/b/novel.skrib".into()),
            backup_created_at: Some("2026-01-01T12:00:00Z".into()),
            authoritative: true,
        };
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(BackupChoicePanel::new(restore, ctx)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }
}
