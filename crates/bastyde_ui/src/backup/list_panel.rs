// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupsListPanel` — browse the backup **files** for the open project.
//!
//! Scans the project's effective destinations (+ its own folder) for this
//! project's backups (correlated on `unique_id`, newest first) and lists them
//! with date + size. Each row can **Open** the backup (in its own instance, which
//! shows the read-only/restore choice), **Reveal** it in the file manager, or
//! **Delete** it (behind a confirmation — T1-4). A **Refresh** button re-scans on
//! demand (e.g. after plugging a drive in). Distinct from the destinations editor
//! in Settings — that lists the configured *paths*; this lists the actual backup
//! *files*.
//!
//! **T2-3 — off the UI thread.** The scan (`retention::scan_destination` — a
//! `read_dir` + one zip-manifest peek per candidate) and `human_size`'s recursive
//! folder-bundle walk can be slow on a USB stick or network share, so every scan
//! (initial load, the Refresh button, and the rescan after a delete) runs via the
//! main-thread async executor's `spawn_blocking`, with a loading state shown
//! meanwhile — never inline in `build()` or an event handler.

use bastyde::core::styles::PanelVariant;
use bastyde::prelude::*;

use crate::view_models::{BackupRow, BackupsListViewModel};
use bastyde::prelude::{EllipsisMode, TextOverflow};
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, ListView, MessageBox,
    MessageBoxButtons, Padding, Panel, Spacer, StandardButton, StandardListItem, Switcher,
    TextWidget, VStack,
};

const CARD_W: f32 = 680.0;
const CARD_H: f32 = 520.0;

pub struct BackupsListPanel {
    vm: BackupsListViewModel,
    root_child: Option<WidgetId>,
    /// One-shot guard: the initial background scan is kicked off only on the
    /// very first `build()` call.
    scan_kicked: bool,
}

impl BackupsListPanel {
    /// `dirs` are the configured destinations; the project's own folder is always
    /// scanned too (the default "next to the project" destination). The scan
    /// itself is deferred to the first `build()` (T2-3) — construction does no
    /// filesystem I/O. `work_id` is the caller's own window's open Work (for
    /// toast routing — see `BackupsListViewModel::new`'s doc).
    pub fn new(
        uid: String,
        project_path: String,
        mut dirs: Vec<String>,
        work_id: Option<u64>,
    ) -> Self {
        dirs.push(String::new()); // the project's own folder
        Self {
            vm: BackupsListViewModel::new(uid, project_path, dirs, work_id),
            root_child: None,
            scan_kicked: false,
        }
    }
}

impl std::fmt::Debug for BackupsListPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackupsListPanel").finish()
    }
}

impl Widget for BackupsListPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .set_async_runtime(ctx.app_state::<AsyncRuntimeHandle>().cloned());
        if !self.scan_kicked {
            self.scan_kicked = true;
            self.vm.reload();
        }

        let model = self.vm.rows();
        let vm_for_rows = self.vm.clone();

        let list = ListView::new(model.clone(), move |_i, row: &BackupRow, selected| {
            let vm = vm_for_rows.clone();
            let vm_open = vm_for_rows.clone();
            let open_path = row.path.clone();
            let reveal_path = row.path.clone();
            let delete_path = row.path.clone();
            let delete_name = BackupsListViewModel::display_name(&row.path);
            let actions = HStack::new()
                .spacing(6.0)
                .child(
                    Button::new(tr!(backups_open()))
                        .variant(ButtonVariant::Filled)
                        .on_activate_fn(move |c| vm_open.open(c, &open_path)),
                )
                .child(
                    Button::new(tr!(backups_reveal()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(move |_c| BackupsListViewModel::reveal(&reveal_path)),
                )
                .child(
                    IconButton::clear()
                        .tooltip(tr!(backups_delete()))
                        .on_activate_fn(move |ctx| {
                            let vm = vm.clone();
                            let path = delete_path.clone();
                            let name = delete_name.clone();
                            MessageBox::question(tr!(backups_delete_confirm_title()))
                                .text(tr!(backups_delete_confirm_text(name = name)))
                                .buttons(MessageBoxButtons::OkCancel)
                                .on_result(move |r, ctx2| {
                                    if r.button == StandardButton::Ok {
                                        vm.delete(ctx2, &path);
                                    }
                                })
                                .present(ctx);
                        }),
                );
            Box::new(
                StandardListItem::new(lit!(row.date.clone()))
                    .subtitle(lit!(format!("{} · {}", row.size, row.path)))
                    // A backup path is long and a destination can sit anywhere,
                    // so the subtitle must truncate rather than claim its full
                    // intrinsic width — a wrapping subtitle over-constrains the
                    // row and pushes Open/Reveal/Delete past the card's edge
                    // (Delete ended up outside it entirely). Middle elision
                    // keeps both the root and the file name legible.
                    .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Middle))
                    .trailing_slot(actions)
                    .selected(selected),
            )
        })
        .auto_item_height(56.0);

        // Loading / empty-state / list, re-derived whenever either the loading
        // flag or the list content changes.
        let idx_model = model.clone();
        let switch_index = self
            .vm
            .loading()
            .zip(&self.vm.epoch())
            .map(move |(loading, _epoch)| {
                if *loading {
                    0usize
                } else if idx_model.is_empty() {
                    1usize
                } else {
                    2usize
                }
            });
        let body = Switcher::new(switch_index)
            .child(
                Padding::symmetric(24.0, 40.0).child(
                    TextWidget::new(tr!(backups_loading()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(
                Padding::symmetric(24.0, 40.0).child(
                    TextWidget::new(tr!(backups_empty()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(list);

        let refresh_vm = self.vm.clone();

        let root = bati!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        // `FixedSize` ignores the parent's proposal and reports its
                        // child's *intrinsic* width, so a bare height-only bar is
                        // placed at ~its content width and left-aligned — which
                        // starves the title's `Expand` and leaves the footer's
                        // `Spacer` nothing to push against. `Expand::horizontal`
                        // (fill mode) places the bar across the whole card, exactly
                        // as the Settings card does with its own header strip.
                        Expand::horizontal {
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(tr!(backups_title())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        Button::new(tr!(backups_refresh())) {
                                            variant: ButtonVariant::Plain
                                            on_activate_fn: move |_c| refresh_vm.reload()
                                        }
                                        IconButton::clear() {
                                            tooltip: tr!(backups_close())
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
                        Expand::horizontal {
                            FixedSize {
                                height: 52.0
                                Padding::symmetric(10.0, 22.0) {
                                    HStack {
                                        Spacer
                                        Button::new(tr!(backups_close())) {
                                            variant: ButtonVariant::Filled
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
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

/// The file's basename, shown as the `{ $name }` placeholder in the delete
/// confirmation (e.g. `novel-20260101-120000.skrib`).
#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    fn descendants(tree: &WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
        for child in tree.children(id) {
            out.push(child);
            descendants(tree, child, out);
        }
    }

    /// The header (title · Refresh · ✕) and footer (Close) bars must span the
    /// whole card. `FixedSize` ignores its parent's proposal and reports its
    /// child's *intrinsic* width, so a height-only bar dropped straight into the
    /// VStack is placed at ~its content width: the title's `Expand` collapsed to
    /// nothing (the word wrapped one letter per line) and the footer's `Spacer`
    /// had no room to push Close to the right. Wrapping each bar in
    /// `Expand::horizontal` is what makes them fill the card.
    #[test]
    fn the_header_and_footer_bars_span_the_card() {
        let dir = tempfile::tempdir().unwrap();
        let mut tree = WidgetTree::new();
        let id = tree.add(BackupsListPanel::new(
            "uid".into(),
            dir.path()
                .join("novel.skrib")
                .to_string_lossy()
                .into_owned(),
            vec![dir.path().to_string_lossy().into_owned()],
            Some(1),
        ));
        // Lay the panel out at the size it reports (CARD_W x CARD_H) — that is
        // what the modal overlay gives it.
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));

        let card = tree.bounds(id);
        assert!(
            (card.width - CARD_W).abs() < 0.5,
            "card should be {CARD_W} wide, got {}",
            card.width
        );

        let mut ids = Vec::new();
        descendants(&tree, id, &mut ids);

        let bar_spans_card = |height: f32| {
            ids.iter().any(|d| {
                let b = tree.bounds(*d);
                (b.height - height).abs() < 0.5 && (b.width - CARD_W).abs() < 0.5
            })
        };
        assert!(
            bar_spans_card(44.0),
            "the 44px header bar must span the full card width"
        );
        assert!(
            bar_spans_card(52.0),
            "the 52px footer bar must span the full card width"
        );

        // And nothing may stick out past the card's edge (the row actions used
        // to: the Delete button landed ~30px outside it).
        for d in &ids {
            let b = tree.bounds(*d);
            assert!(
                b.right() <= card.right() + 0.5,
                "a descendant overflows the card: right={} vs card right={}",
                b.right(),
                card.right()
            );
        }
    }
}
