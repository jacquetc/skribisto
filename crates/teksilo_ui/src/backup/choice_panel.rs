// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupChoicePanel` — the modal shown when a **backup file** is opened.
//!
//! Three choices: **Open the backup** (the default — edit freely; changes can
//! only be kept with Save As; the original file is untouched), **Restore this
//! project to this point** (overwrite the original, after a safety copy), or —
//! only when the sniff was a non-authoritative filename guess (T2-5) — **No,
//! open it normally**, which clears backup mode entirely. The window is already
//! in backup mode on load, so "Open the backup" just dismisses.
//!
//! **T2-5 — the non-authoritative escape hatch.** `skrib_format::sniff_backup`
//! only *guesses* "this is a backup" from the filename when the manifest is
//! unreadable (a legacy SQLite `.skrib`) — flagged `authoritative: false`. A
//! legacy project that merely *looks* like a backup (e.g. it happens to be
//! named like one) must never be trapped in read-only backup mode with no way
//! out, so this third button is shown only in that case; it clears
//! `backup_mode`/`backup_context` (the same signals the successful-restore path
//! clears) and dismisses. When the sniff *is* authoritative (a real marker in
//! the manifest), today's two-choice behaviour is unchanged.

use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel, Spacer,
    TextWidget, VStack,
};

use crate::backup::BackupContext;
use crate::view_models::BackupRestoreViewModel;

/// Restore button click: dismiss this modal, then start the restore flow.
fn on_restore(restore: &BackupRestoreViewModel, ctx: &mut EventContext) {
    ctx.dismiss_modal();
    restore.begin(ctx);
}

/// Clear backup mode entirely — split out from [`on_open_normally`] so the
/// signal-mutation logic is unit-testable without an `EventContext` (this
/// codebase has no `EventContext` test harness — see `backup_scheduler.rs` /
/// `restore.rs` for the same constraint).
fn clear_backup_mode(backup_mode: &Signal<bool>, backup_context: &Signal<Option<BackupContext>>) {
    backup_mode.set(false);
    backup_context.set(None);
}

/// "No, open it normally" click (T2-5, non-authoritative sniff only): clear
/// backup mode and dismiss — the project opens as a regular, writable project.
fn on_open_normally(
    backup_mode: &Signal<bool>,
    backup_context: &Signal<Option<BackupContext>>,
    ctx: &mut EventContext,
) {
    clear_backup_mode(backup_mode, backup_context);
    ctx.dismiss_modal();
}

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 320.0;

pub struct BackupChoicePanel {
    restore: BackupRestoreViewModel,
    context: BackupContext,
    backup_mode: Signal<bool>,
    backup_context: Signal<Option<BackupContext>>,
    root_child: Option<WidgetId>,
}

impl BackupChoicePanel {
    pub fn new(
        restore: BackupRestoreViewModel,
        context: BackupContext,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<BackupContext>>,
    ) -> Self {
        Self {
            restore,
            context,
            backup_mode,
            backup_context,
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
        // T2-5: only a non-authoritative filename guess gets the escape hatch —
        // an authoritative manifest marker keeps today's two-choice behaviour.
        let show_open_normally = !self.context.authoritative;
        let backup_mode = self.backup_mode.clone();
        let backup_context = self.backup_context.clone();

        let root = teksu!(ctx => FixedSize {
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
                                        tooltip: tr!(backups_close())
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
                                    if show_open_normally {
                                        Button::new(tr!(backup_choice_not_a_backup())) {
                                            variant: ButtonVariant::Plain
                                            on_activate_fn: move |ctx| {
                                                on_open_normally(&backup_mode, &backup_context, ctx)
                                            }
                                        }
                                    }
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
    use teksilo::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use std::rc::Rc;

    fn make_restore(app_ctx: Rc<AppContext>) -> BackupRestoreViewModel {
        let ids = crate::app_ids::AppIds::new();
        let single_work = crate::singles::SingleWork::new(app_ctx.clone());
        BackupRestoreViewModel::new(
            app_ctx,
            ids,
            single_work,
            Signal::new(true),
            Signal::new(None),
        )
    }

    #[test]
    fn choice_panel_builds_and_lays_out_authoritative() {
        let app_ctx = Rc::new(AppContext::new());
        let restore = make_restore(app_ctx);
        let ctx = BackupContext {
            path: "/b/novel-20260101-120000.skrib".into(),
            backup_of: Some("/b/novel.skrib".into()),
            backup_created_at: Some("2026-01-01T12:00:00Z".into()),
            authoritative: true,
        };
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(BackupChoicePanel::new(
            restore,
            ctx,
            Signal::new(true),
            Signal::new(None),
        )));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }

    #[test]
    fn choice_panel_builds_and_lays_out_non_authoritative() {
        // T2-5: the extra "No, open it normally" button must not break layout
        // when the sniff was only a non-authoritative filename guess.
        let app_ctx = Rc::new(AppContext::new());
        let restore = make_restore(app_ctx);
        let ctx = BackupContext {
            path: "/b/novel-20260101-120000.skrib".into(),
            backup_of: Some("/b/novel.skrib".into()),
            backup_created_at: None,
            authoritative: false,
        };
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(BackupChoicePanel::new(
            restore,
            ctx,
            Signal::new(true),
            Signal::new(None),
        )));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card even with the extra escape-hatch button"
        );
    }

    #[test]
    fn clear_backup_mode_clears_both_signals() {
        let backup_mode = Signal::new(true);
        let backup_context = Signal::new(Some(BackupContext {
            path: "/b/novel-20260101-120000.skrib".into(),
            backup_of: Some("/b/novel.skrib".into()),
            backup_created_at: None,
            authoritative: false,
        }));
        clear_backup_mode(&backup_mode, &backup_context);
        assert!(!backup_mode.get());
        assert!(backup_context.get().is_none());
    }
}
