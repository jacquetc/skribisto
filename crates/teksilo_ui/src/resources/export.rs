// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Copies of resources, written to a folder the writer picked: what the Work's
//! Resources page exports.
//!
//! A stored file and a linked file are copied under their own names; when two would land
//! on one name, or a file of that name is already there, the later one is numbered
//! ("Map (2).pdf"), so nothing is ever written over. Web links are not files: they are
//! listed, one per line with their titles, in one `links.txt` beside the copies. Never
//! as `.url` or `.desktop` files, which a file manager would run when opened.
//!
//! A file that is not where the project expects it is left out and named in the report,
//! as is one that could not be copied, with the reason.
//!
//! Not a view-model: plain file work, testable without a window.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::models::ResourceRow;

/// The file the web links are listed in.
pub const LINKS_FILE: &str = "links.txt";

/// What an export wrote, and what it could not.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExportReport {
    /// The copies, in the order of the rows.
    pub copied: Vec<PathBuf>,
    /// The list of web links, when there was one to write.
    pub links_file: Option<PathBuf>,
    /// How many web links it lists.
    pub links: usize,
    /// The files that are not where the project expects them, by name.
    pub missing: Vec<String>,
    /// The files that could not be copied, by name, with the reason.
    pub failed: Vec<(String, String)>,
}

impl ExportReport {
    /// How many things the folder received: copies, and links in the list.
    pub fn written(&self) -> usize {
        self.copied.len() + self.links
    }
}

/// A name for `wanted` in `dir` that neither `taken` nor a file already in `dir` holds:
/// `wanted` itself, else numbered before its extension from 2 up. The name is recorded
/// in `taken`, compared without case, because two names that differ only by case are
/// one file on most of the systems a writer uses.
pub fn free_name(dir: &Path, wanted: &str, taken: &mut HashSet<String>) -> String {
    let wanted = safe_name(wanted);
    let (stem, ext) = match wanted.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => {
            (stem.to_string(), format!(".{ext}"))
        }
        _ => (wanted.clone(), String::new()),
    };
    let mut n = 1usize;
    loop {
        let candidate = if n == 1 {
            wanted.clone()
        } else {
            format!("{stem} ({n}){ext}")
        };
        let key = candidate.to_lowercase();
        if !taken.contains(&key) && !dir.join(&candidate).exists() {
            taken.insert(key);
            return candidate;
        }
        n += 1;
    }
}

/// `name` as one file name: its last component only, never empty.
fn safe_name(name: &str) -> String {
    let base = Path::new(name)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let base = base.trim();
    if base.is_empty() || base == "." || base == ".." {
        "resource".to_string()
    } else {
        base.to_string()
    }
}

/// The web links of `rows` as `links.txt` lists them: one per line, its title and its
/// address, or the address alone when the title says nothing more.
pub fn links_text(rows: &[ResourceRow]) -> String {
    let mut out = String::new();
    for row in rows.iter().filter(|r| r.is_web_link()) {
        let title = row.title.trim();
        if title.is_empty() || title == row.location.trim() {
            out.push_str(row.location.trim());
        } else {
            out.push_str(title);
            out.push_str(": ");
            out.push_str(row.location.trim());
        }
        out.push('\n');
    }
    out
}

/// Copy the files of `rows` into `dir` and list their web links there. See the module
/// docs.
pub fn export_copies(rows: &[ResourceRow], dir: &Path) -> ExportReport {
    let mut report = ExportReport::default();
    let mut taken = HashSet::new();
    for row in rows.iter().filter(|r| !r.is_web_link()) {
        let name = if row.file_name.trim().is_empty() {
            row.title.clone()
        } else {
            row.file_name.clone()
        };
        let Some(source) = row.path.as_deref().filter(|p| !row.missing && p.is_file()) else {
            report.missing.push(name);
            continue;
        };
        let target = dir.join(free_name(dir, &name, &mut taken));
        match std::fs::copy(source, &target) {
            Ok(_) => report.copied.push(target),
            Err(e) => report.failed.push((name, e.to_string())),
        }
    }
    let links = links_text(rows);
    if !links.is_empty() {
        let target = dir.join(free_name(dir, LINKS_FILE, &mut taken));
        match std::fs::write(&target, links.as_bytes()) {
            Ok(()) => {
                report.links = rows.iter().filter(|r| r.is_web_link()).count();
                report.links_file = Some(target);
            }
            Err(e) => report.failed.push((LINKS_FILE.to_string(), e.to_string())),
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ResourceKind;

    fn file(dir: &Path, id: u64, name: &str, bytes: &[u8]) -> ResourceRow {
        let path = dir.join(format!("src-{id}")).join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("source folder");
        }
        std::fs::write(&path, bytes).expect("source file");
        ResourceRow {
            id,
            title: name.to_string(),
            kind: ResourceKind::StoredFile,
            file_name: name.to_string(),
            path: Some(path),
            ..Default::default()
        }
    }

    fn link(id: u64, title: &str, url: &str) -> ResourceRow {
        ResourceRow {
            id,
            title: title.to_string(),
            kind: ResourceKind::WebLink,
            location: url.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn two_files_of_one_name_are_both_copied_the_second_numbered() {
        let src = tempfile::tempdir().expect("tmp");
        let out = tempfile::tempdir().expect("tmp");
        let rows = [
            file(src.path(), 1, "Map.pdf", b"first"),
            ResourceRow {
                kind: ResourceKind::LinkedFile,
                ..file(src.path(), 2, "Map.pdf", b"second")
            },
            file(src.path(), 3, "map.PDF", b"third"),
        ];
        let report = export_copies(&rows, out.path());
        let names: Vec<String> = report
            .copied
            .iter()
            .map(|p| {
                p.file_name()
                    .expect("a name")
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        assert_eq!(names, ["Map.pdf", "Map (2).pdf", "map (3).PDF"]);
        assert_eq!(
            std::fs::read(out.path().join("Map.pdf")).expect("read"),
            b"first"
        );
        assert_eq!(
            std::fs::read(out.path().join("Map (2).pdf")).expect("read"),
            b"second"
        );
        assert!(report.links_file.is_none(), "no link, no list");
    }

    #[test]
    fn a_file_already_in_the_folder_is_never_written_over() {
        let src = tempfile::tempdir().expect("tmp");
        let out = tempfile::tempdir().expect("tmp");
        std::fs::write(out.path().join("Notes.txt"), b"the writer's own").expect("write");
        std::fs::write(out.path().join(LINKS_FILE), b"older list").expect("write");
        let rows = [
            file(src.path(), 1, "Notes.txt", b"exported"),
            link(2, "Harbour records", "https://example.org/harbour"),
        ];
        let report = export_copies(&rows, out.path());
        assert_eq!(
            std::fs::read(out.path().join("Notes.txt")).expect("read"),
            b"the writer's own"
        );
        assert_eq!(
            std::fs::read(out.path().join("Notes (2).txt")).expect("read"),
            b"exported"
        );
        assert_eq!(
            report.links_file.as_deref(),
            Some(out.path().join("links (2).txt").as_path())
        );
    }

    #[test]
    fn web_links_are_listed_in_one_text_file_never_as_shortcuts() {
        let out = tempfile::tempdir().expect("tmp");
        let rows = [
            link(1, "Harbour records", "https://example.org/harbour"),
            link(2, "", "https://example.org/tides"),
            link(3, "https://example.org/gulls", "https://example.org/gulls"),
        ];
        let report = export_copies(&rows, out.path());
        assert!(report.copied.is_empty());
        assert_eq!(report.links, 3);
        assert_eq!(report.written(), 3);
        let text = std::fs::read_to_string(out.path().join(LINKS_FILE)).expect("links.txt");
        assert_eq!(
            text,
            "Harbour records: https://example.org/harbour\n\
             https://example.org/tides\n\
             https://example.org/gulls\n"
        );
        let names: Vec<String> = std::fs::read_dir(out.path())
            .expect("list")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, [LINKS_FILE], "no .url, no .desktop: {names:?}");
    }

    #[test]
    fn a_missing_file_is_named_and_left_out() {
        let out = tempfile::tempdir().expect("tmp");
        let gone = ResourceRow {
            id: 1,
            title: "Tide tables".into(),
            kind: ResourceKind::LinkedFile,
            file_name: "Tide tables.pdf".into(),
            path: Some(PathBuf::from("/nowhere/Tide tables.pdf")),
            missing: true,
            ..Default::default()
        };
        let report = export_copies(&[gone], out.path());
        assert!(report.copied.is_empty());
        assert_eq!(report.missing, ["Tide tables.pdf"]);
        assert_eq!(report.written(), 0);
    }

    #[test]
    fn a_name_is_one_file_name_and_never_empty() {
        let out = tempfile::tempdir().expect("tmp");
        let mut taken = HashSet::new();
        assert_eq!(
            free_name(out.path(), "../../etc/passwd", &mut taken),
            "passwd"
        );
        assert_eq!(free_name(out.path(), "", &mut taken), "resource");
        assert_eq!(free_name(out.path(), "..", &mut taken), "resource (2)");
        assert_eq!(free_name(out.path(), "README", &mut taken), "README");
        assert_eq!(free_name(out.path(), "readme", &mut taken), "readme (2)");
        assert_eq!(free_name(out.path(), ".hidden", &mut taken), ".hidden");
    }
}
