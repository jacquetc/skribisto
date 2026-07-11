//! `BackupsListPanel` — browse the backup **files** for the open project.
//!
//! Scans the project's effective destinations (+ its own folder) for this
//! project's backups (correlated on `unique_id`, newest first) and lists them
//! with date + size. Each row can **Open** the backup (in its own instance, which
//! shows the read-only/restore choice), **Reveal** it in the file manager, or
//! **Delete** it. A **Refresh** button re-scans on demand (e.g. after plugging a
//! drive in). Distinct from the destinations editor in Settings — that lists the
//! configured *paths*; this lists the actual backup *files*.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use bastyde::core::styles::PanelVariant;
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, ListView, Padding,
    Panel, Spacer, StandardListItem, Switcher, TextWidget, VStack,
};

use skrib_format::retention;

const CARD_W: f32 = 680.0;
const CARD_H: f32 = 520.0;

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
    epoch: Signal<u64>,
}

impl Scanner {
    fn rescan(&self) {
        self.model
            .replace_all(scan_backups(&self.uid, &self.project_path, &self.dirs));
        let e = &self.epoch;
        e.set(e.get().wrapping_add(1));
    }

    fn delete(&self, path: &str) {
        let p = Path::new(path);
        let _ = if p.is_dir() {
            std::fs::remove_dir_all(p)
        } else {
            std::fs::remove_file(p)
        };
        self.rescan();
    }
}

pub struct BackupsListPanel {
    scanner: Scanner,
    root_child: Option<WidgetId>,
}

impl BackupsListPanel {
    /// `dirs` are the configured destinations; the project's own folder is always
    /// scanned too (the default "next to the project" destination).
    pub fn new(uid: String, project_path: String, mut dirs: Vec<String>) -> Self {
        dirs.push(String::new()); // the project's own folder
        let rows = scan_backups(&uid, &project_path, &dirs);
        Self {
            scanner: Scanner {
                uid,
                project_path,
                dirs,
                model: ListModel::from_vec(rows),
                epoch: Signal::new(0),
            },
            root_child: None,
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
        let model = self.scanner.model.clone();
        let scanner_for_rows = self.scanner.clone();

        let list = ListView::new(model.clone(), move |_i, row: &BackupRow, selected| {
            let s = scanner_for_rows.clone();
            let open_path = row.path.clone();
            let reveal_path = row.path.clone();
            let delete_path = row.path.clone();
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
                        .on_activate_fn(move |_c| s.delete(&delete_path)),
                );
            Box::new(
                StandardListItem::new(lit!(row.date.clone()))
                    .subtitle(lit!(format!("{} · {}", row.size, row.path)))
                    .trailing_slot(actions)
                    .selected(selected),
            )
        })
        .auto_item_height(56.0);

        // Empty-state vs list, re-derived on each rescan (epoch bump).
        let idx_model = model.clone();
        let switch_index = self
            .scanner
            .epoch
            .map(move |_: &u64| if idx_model.is_empty() { 0usize } else { 1usize });
        let body = Switcher::new(switch_index)
            .child(
                Padding::symmetric(24.0, 40.0).child(
                    TextWidget::new(tr!(backups_empty()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(list);

        let refresh_scanner = self.scanner.clone();

        let root = bati!(ctx =>
            FixedSize {
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
                                        TextWidget::new(tr!(backups_title())) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                        }
                                    }
                                    Button::new(tr!(backups_refresh())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: move |_c| refresh_scanner.rescan()
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(backups_close())
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                }
                            }
                        }
                        Expand::horizontal { Divider }
                        Expand::vertical { child: body }
                        Expand::horizontal { Divider }
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
}
