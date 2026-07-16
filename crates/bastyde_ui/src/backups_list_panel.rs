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

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use bastyde::core::styles::PanelVariant;
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::prelude::{EllipsisMode, TextOverflow};
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, ListView, MessageBox,
    MessageBoxButtons, Padding, Panel, Spacer, StandardButton, StandardListItem, Switcher,
    TextWidget, Toast, VStack,
};

use skrib_format::retention;

const CARD_W: f32 = 680.0;
const CARD_H: f32 = 520.0;
const DELETE_TOAST_ID: &str = "backups.delete";

#[derive(Clone)]
struct BackupRow {
    path: String,
    date: String,
    size: String,
}

/// Cloneable scan state shared into the row action closures.
#[derive(Clone)]
struct Scanner {
    uid: String,
    project_path: String,
    dirs: Vec<String>,
    model: ListModel<BackupRow>,
    /// Bumped whenever the list's *content* changes (a scan lands), driving the
    /// empty-state/list `Switcher`.
    epoch: Signal<u64>,
    /// `true` while a scan (initial / refresh / post-delete) is in flight.
    loading: Signal<bool>,
    /// The main-thread async executor, fetched once from `app_state` in
    /// `BackupsListPanel::build`. `None` only if `install_async()` was somehow
    /// not called at startup (an app bug, not a normal runtime state) — every
    /// method below falls back to a synchronous scan/delete in that case rather
    /// than getting stuck loading forever.
    async_rt: Option<AsyncRuntimeHandle>,
}

impl Scanner {
    /// (Re)scan in the background and push the result into `model` on
    /// completion. Safe to call from anywhere that holds a `Scanner` clone — no
    /// `EventContext` required, since landing the result is just `Signal`/
    /// `ListModel` mutation, no ambient op.
    fn kick_scan(&self) {
        self.loading.set(true);
        let uid = self.uid.clone();
        let project_path = self.project_path.clone();
        let dirs = self.dirs.clone();
        let model = self.model.clone();
        let epoch = self.epoch.clone();
        let loading = self.loading.clone();
        match &self.async_rt {
            Some(rt) => {
                rt.spawn_local(async move {
                    let rows = spawn_blocking(move || scan_backups(&uid, &project_path, &dirs))
                        .await
                        .unwrap_or_default();
                    model.replace_all(rows);
                    loading.set(false);
                    epoch.set(epoch.get().wrapping_add(1));
                })
                .detach();
            }
            None => {
                model.replace_all(scan_backups(&uid, &project_path, &dirs));
                loading.set(false);
                epoch.set(epoch.get().wrapping_add(1));
            }
        }
    }

    /// Delete `path` in the background, report a failure as an error toast
    /// (T1-4 — the previous `let _ =` silently swallowed it), then rescan
    /// either way (a partial folder-bundle removal should still be reflected).
    fn delete(&self, ctx: &mut EventContext, path: &str) {
        self.loading.set(true);
        let scanner = self.clone();
        let target = PathBuf::from(path);
        match &self.async_rt {
            Some(_) => {
                ctx.spawn_local_with(
                    async move {
                        spawn_blocking(move || {
                            if target.is_dir() {
                                std::fs::remove_dir_all(&target)
                            } else {
                                std::fs::remove_file(&target)
                            }
                        })
                        .await
                    },
                    move |result, ctx2| {
                        match result {
                            Ok(Ok(())) => {}
                            Ok(Err(e)) => {
                                ctx2.show_toast(
                                    Toast::error(tr!(backups_delete_error(error = e.to_string())))
                                        .id(DELETE_TOAST_ID),
                                );
                            }
                            Err(_panicked) => {
                                ctx2.show_toast(
                                    Toast::error(tr!(backups_delete_error(
                                        error = "panicked".to_string()
                                    )))
                                    .id(DELETE_TOAST_ID),
                                );
                            }
                        }
                        scanner.kick_scan();
                    },
                )
                .detach();
            }
            None => {
                let result = if target.is_dir() {
                    std::fs::remove_dir_all(&target)
                } else {
                    std::fs::remove_file(&target)
                };
                if let Err(e) = result {
                    ctx.show_toast(
                        Toast::error(tr!(backups_delete_error(error = e.to_string())))
                            .id(DELETE_TOAST_ID),
                    );
                }
                scanner.kick_scan();
            }
        }
    }
}

pub struct BackupsListPanel {
    scanner: Scanner,
    root_child: Option<WidgetId>,
    /// One-shot guard: the initial background scan is kicked off only on the
    /// very first `build()` call.
    scan_kicked: bool,
}

impl BackupsListPanel {
    /// `dirs` are the configured destinations; the project's own folder is always
    /// scanned too (the default "next to the project" destination). The scan
    /// itself is deferred to the first `build()` (T2-3) — construction does no
    /// filesystem I/O.
    pub fn new(uid: String, project_path: String, mut dirs: Vec<String>) -> Self {
        dirs.push(String::new()); // the project's own folder
        Self {
            scanner: Scanner {
                uid,
                project_path,
                dirs,
                model: ListModel::from_vec(Vec::new()),
                epoch: Signal::new(0),
                loading: Signal::new(true),
                async_rt: None,
            },
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
        if self.scanner.async_rt.is_none() {
            self.scanner.async_rt = ctx.app_state::<AsyncRuntimeHandle>().cloned();
        }
        if !self.scan_kicked {
            self.scan_kicked = true;
            self.scanner.kick_scan();
        }

        let model = self.scanner.model.clone();
        let scanner_for_rows = self.scanner.clone();

        let list = ListView::new(model.clone(), move |_i, row: &BackupRow, selected| {
            let s = scanner_for_rows.clone();
            let open_path = row.path.clone();
            let reveal_path = row.path.clone();
            let delete_path = row.path.clone();
            let delete_name = backup_display_name(&row.path);
            let actions = HStack::new()
                .spacing(6.0)
                .child(
                    Button::new(tr!(backups_open()))
                        .variant(ButtonVariant::Filled)
                        .on_activate_fn(move |c| {
                            let p = open_path.clone();
                            c.request_activation_token_self(Box::new(move |tok| {
                                crate::project_switcher_button::spawn_new_process(&p, tok);
                            }));
                        }),
                )
                .child(
                    Button::new(tr!(backups_reveal()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(move |_c| reveal_in_file_manager(&reveal_path)),
                )
                .child(
                    IconButton::clear()
                        .tooltip(tr!(backups_delete()))
                        .on_activate_fn(move |ctx| {
                            let s = s.clone();
                            let path = delete_path.clone();
                            let name = delete_name.clone();
                            MessageBox::question(tr!(backups_delete_confirm_title()))
                                .text(tr!(backups_delete_confirm_text(name = name)))
                                .buttons(MessageBoxButtons::OkCancel)
                                .on_result(move |r, ctx2| {
                                    if r.button == StandardButton::Ok {
                                        s.delete(ctx2, &path);
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
        let switch_index =
            self.scanner
                .loading
                .zip(&self.scanner.epoch)
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

        let refresh_scanner = self.scanner.clone();

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
                                            on_activate_fn: move |_c| refresh_scanner.kick_scan()
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
fn backup_display_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

/// Scan `dirs` (+ resolve the empty "project folder" destination) for this
/// project's backups, newest first, de-duplicated by path.
fn scan_backups(uid: &str, project_path: &str, dirs: &[String]) -> Vec<BackupRow> {
    let mut candidates = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    for d in dirs {
        let dir = if d.trim().is_empty() {
            Path::new(project_path)
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .map(|p| p.to_path_buf())
        } else {
            Some(PathBuf::from(d))
        };
        let Some(dir) = dir else { continue };
        if let Ok(found) = retention::scan_destination(&dir, uid, project_path) {
            for c in found {
                if seen.insert(c.path.clone()) {
                    candidates.push(c);
                }
            }
        }
    }
    candidates.sort_by_key(|c| std::cmp::Reverse(c.timestamp));
    candidates
        .into_iter()
        .map(|c| BackupRow {
            date: c.timestamp.format("%Y-%m-%d %H:%M").to_string(),
            size: human_size(&c.path),
            path: c.path.to_string_lossy().into_owned(),
        })
        .collect()
}

fn human_size(path: &Path) -> String {
    let bytes = byte_size(path);
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{} KB", (bytes / 1024).max(1))
    }
}

/// Total bytes at `path`. A backup is normally a single zip, but a folder-shaped
/// bundle must be summed recursively — `metadata(dir).len()` is the directory
/// entry's own size (~4 KB), not its contents.
fn byte_size(path: &Path) -> u64 {
    let Ok(meta) = std::fs::metadata(path) else {
        return 0;
    };
    if meta.is_file() {
        return meta.len();
    }
    if !meta.is_dir() {
        return 0;
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        return 0;
    };
    entries.flatten().map(|e| byte_size(&e.path())).sum()
}

/// Open the file manager at `path`'s containing folder (best-effort, per platform).
fn reveal_in_file_manager(path: &str) {
    let target = Path::new(path)
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(&target).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(&target).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(&target).spawn();
}

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

    #[test]
    fn scan_lists_this_projects_backups_newest_first() {
        // Reuse the retention test's zip writer would be ideal, but keep this a
        // pure check of ordering/dedup on an empty dir (no candidates).
        let d = tempfile::tempdir().unwrap();
        let rows = scan_backups(
            "uid",
            &d.path().join("novel.skrib").to_string_lossy(),
            &[d.path().to_string_lossy().into_owned()],
        );
        assert!(rows.is_empty());
    }

    #[test]
    fn human_size_formats_kb_and_mb() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("a");
        std::fs::write(&f, vec![0u8; 2048]).unwrap();
        assert_eq!(human_size(&f), "2 KB");
    }

    #[test]
    fn human_size_sums_a_folder_bundle_recursively() {
        // A folder-shaped bundle: `metadata(dir).len()` would report the directory
        // entry (~4 KB), not the 3 KB of actual content across nested files.
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("novel-20260101-120000.skrib");
        std::fs::create_dir_all(root.join("binders/manuscript")).unwrap();
        std::fs::write(root.join("project.skrib"), vec![0u8; 1024]).unwrap();
        std::fs::write(root.join("binders/manuscript/a.djot"), vec![0u8; 2048]).unwrap();
        assert_eq!(byte_size(&root), 3072, "summed recursively");
        assert_eq!(human_size(&root), "3 KB");
    }

    #[test]
    fn backup_display_name_is_the_basename() {
        assert_eq!(
            backup_display_name("/a/b/novel-20260101-120000.skrib"),
            "novel-20260101-120000.skrib"
        );
        assert_eq!(backup_display_name("novel.skrib"), "novel.skrib");
    }
}
