// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the writer picked, and which project folder it means.
//!
//! A writer can pick a `.scriv` folder (a package on macOS, a plain folder
//! elsewhere), the folder around it, a folder inside it, the `.scrivx` inside it
//! (the only file Scrivener for Windows shows in an Open dialog), an archive, or a
//! `.scrivtemplate`. Files are recognised by content, not by extension.
//!
//! # Strong and weak roots
//!
//! A folder is a project (a *strong* root) when it holds a `.scrivx` at its top
//! and a `Files` or `Mobile` folder beside it, or a Scrivener 1 `binder.scrivproj`.
//! A `.scrivx` with neither beside it is a *weak* root: a published manifest, or a
//! folder of loose manifests, which is not a project and does not stop the search
//! (a first version stopped there and hid 23 real projects below one). A strong
//! root is never searched inside: a stale `Files/Workflow.scrivx` inherited from
//! a template must never be taken for a project.
//!
//! # A bounded search
//!
//! Below a picked folder the search lists folders two levels down (three inside
//! an archive: its own wrapping folder plus two), never descends into a project,
//! and gives up past [`SEARCH_LIMIT`] folders listed. It lists only; nothing is
//! read, and the full index is built of the chosen project alone. The limit counts
//! folders, not the files in them, so an unrelated folder of a hundred thousand
//! photographs beside a novel costs one listing and cannot refuse the novel; a pick
//! of a whole home folder, whose second level holds thousands of folders, is
//! refused and the writer asked to pick the project itself.

use std::path::{Path, PathBuf};

use crate::disk::{Disk, EntryKind};
use crate::index::{Index, join};
use crate::names::{is_internal_folder, is_os_litter};
use crate::outcome::BackupSign;

/// The most folders the search below a picked folder lists before giving up.
pub(crate) const SEARCH_LIMIT: u32 = 2_000;
/// The most entries all those listings may hold together: a bound on memory,
/// far above any folder a writer keeps projects in.
const SEARCH_ENTRY_LIMIT: usize = 1_000_000;
/// How many levels below a picked folder the search looks.
pub(crate) const FOLDER_SEARCH_DEPTH: usize = 2;
/// How many levels inside an archive: its own wrapping folder plus two.
pub(crate) const ARCHIVE_SEARCH_DEPTH: usize = 3;
/// How far above a picked internal folder its project is looked for.
pub(crate) const ENCLOSING_LEVELS: u32 = 4;

/// What a picked file holds, from its first bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sniffed {
    Zip,
    Template,
    Manifest,
    BinaryPlist,
    Other,
}

/// Recognise a picked file by content.
pub(crate) fn sniff(head: &[u8]) -> Sniffed {
    if head.starts_with(b"PK\x03\x04") || head.starts_with(b"PK\x05\x06") {
        return Sniffed::Zip;
    }
    let trimmed = head.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(head);
    if trimmed.starts_with(b"bplist00") {
        return Sniffed::BinaryPlist;
    }
    let text = String::from_utf8_lossy(trimmed);
    if text.contains("<ScrivenerTemplate") {
        Sniffed::Template
    } else if text.contains("<ScrivenerProject") {
        Sniffed::Manifest
    } else {
        Sniffed::Other
    }
}

/// How strongly a folder's top level says "a project is here".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Strength {
    None,
    /// A `.scrivx` without `Files` or `Mobile` beside it.
    Weak,
    /// A `.scrivx` with `Files` or `Mobile`, or a Scrivener 1 binder.
    Strong,
}

/// A folder's top level, as listed: names and whether each is a folder.
pub(crate) struct Listing {
    pub(crate) entries: Vec<(String, EntryKind)>,
}

impl Listing {
    pub(crate) fn manifests(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .entries
            .iter()
            .filter(|(name, kind)| {
                *kind == EntryKind::File
                    && name.to_lowercase().ends_with(".scrivx")
                    && !name.starts_with("._")
            })
            .map(|(name, _)| name.clone())
            .collect();
        names.sort();
        names
    }

    pub(crate) fn has_file(&self, wanted: &str) -> bool {
        self.entries
            .iter()
            .any(|(name, kind)| *kind == EntryKind::File && name == wanted)
    }

    pub(crate) fn has_folder(&self, wanted: &str) -> bool {
        self.entries
            .iter()
            .any(|(name, kind)| *kind == EntryKind::Dir && name.eq_ignore_ascii_case(wanted))
    }

    pub(crate) fn strength(&self) -> Strength {
        if self.has_file("binder.scrivproj") {
            return Strength::Strong;
        }
        if self.manifests().is_empty() {
            return Strength::None;
        }
        if self.has_folder("Files") || self.has_folder("Mobile") {
            Strength::Strong
        } else {
            Strength::Weak
        }
    }
}

/// The most entries one listing made to find a project may hold.
const LISTING_LIMIT: usize = SEARCH_ENTRY_LIMIT;

/// List a folder on disk, at most [`LISTING_LIMIT`] entries and one more.
pub(crate) fn list_disk(disk: &dyn Disk, dir: &Path) -> std::io::Result<Listing> {
    Ok(Listing {
        entries: disk
            .list(dir, LISTING_LIMIT)?
            .into_iter()
            .map(|(name, meta)| (name.to_string_lossy().into_owned(), meta.kind))
            .collect(),
    })
}

/// List a folder of an index.
pub(crate) fn list_index(index: &Index<'_>, key: &str) -> Listing {
    Listing {
        entries: index
            .children(key)
            .into_iter()
            .filter_map(|name| {
                let kind = index.entry(&join(key, &name))?.kind;
                Some((name, kind))
            })
            .collect(),
    }
}

/// What a search below a pick found.
#[derive(Debug, Default)]
pub(crate) struct Found {
    /// Project folders, relative to the pick, `/`-separated, sorted.
    pub(crate) strong: Vec<String>,
    /// Folders holding only a manifest.
    pub(crate) weak: Vec<String>,
    /// The search gave up past [`SEARCH_LIMIT`] folders.
    pub(crate) too_wide: bool,
}

fn skip_folder(name: &str) -> bool {
    name == ".git" || name == "__MACOSX" || name.starts_with('.') || is_os_litter(name)
}

/// Search a picked folder on disk, `depth` levels down.
pub(crate) fn search_disk(disk: &dyn Disk, pick: &Path, depth: usize) -> Found {
    let mut found = Found::default();
    let mut folders: u32 = 0;
    let mut entries: usize = 0;
    // Breadth first, so a nearer project is found before the search gives up.
    let mut frontier: std::collections::VecDeque<(String, PathBuf, usize)> =
        std::collections::VecDeque::from([(String::new(), pick.to_path_buf(), 0)]);
    while let Some((rel, dir, level)) = frontier.pop_front() {
        folders += 1;
        if folders > SEARCH_LIMIT {
            found.too_wide = true;
            return found;
        }
        let Ok(listing) = list_disk(disk, &dir) else {
            continue;
        };
        entries = entries.saturating_add(listing.entries.len());
        if entries > SEARCH_ENTRY_LIMIT {
            found.too_wide = true;
            return found;
        }
        match listing.strength() {
            Strength::Strong => {
                found.strong.push(rel);
                continue;
            }
            Strength::Weak => found.weak.push(rel.clone()),
            Strength::None => {}
        }
        if level >= depth {
            continue;
        }
        let mut dirs: Vec<&String> = listing
            .entries
            .iter()
            .filter(|(name, kind)| *kind == EntryKind::Dir && !skip_folder(name))
            .map(|(name, _)| name)
            .collect();
        dirs.sort();
        for name in dirs {
            frontier.push_back((join(&rel, name), dir.join(name), level + 1));
        }
    }
    found.strong.sort();
    found.weak.sort();
    found
}

/// Search an archive's index, `depth` levels down.
pub(crate) fn search_index(index: &Index<'_>, depth: usize) -> Found {
    let mut found = Found::default();
    let mut frontier: std::collections::VecDeque<(String, usize)> =
        std::collections::VecDeque::from([(String::new(), 0)]);
    while let Some((key, level)) = frontier.pop_front() {
        let listing = list_index(index, &key);
        match listing.strength() {
            Strength::Strong => {
                found.strong.push(key);
                continue;
            }
            Strength::Weak => found.weak.push(key.clone()),
            Strength::None => {}
        }
        if level >= depth {
            continue;
        }
        for (name, kind) in &listing.entries {
            if *kind == EntryKind::Dir && !skip_folder(name) {
                frontier.push_back((join(&key, name), level + 1));
            }
        }
    }
    found.strong.sort();
    found.weak.sort();
    found
}

/// The project folder around a pick that sits inside one (`X.scriv/Files`,
/// `X.scriv/Files/Data/<UUID>`), and how many levels up it is.
pub(crate) fn enclosing_project(disk: &dyn Disk, pick: &Path) -> Option<(PathBuf, u32)> {
    let mut current = pick.to_path_buf();
    for level in 1..=ENCLOSING_LEVELS {
        let name = current.file_name()?.to_string_lossy().into_owned();
        if !is_internal_folder(&name) && !(level == 1 && is_scrivener_internal_file(&name)) {
            return None;
        }
        let parent = current.parent()?.to_path_buf();
        let listing = list_disk(disk, &parent).ok()?;
        if listing.strength() == Strength::Strong {
            return Some((parent, level));
        }
        current = parent;
    }
    None
}

/// A file only ever found inside a project's own folders.
fn is_scrivener_internal_file(name: &str) -> bool {
    matches!(
        name,
        "binder.autosave"
            | "binder.backup"
            | "search.indexes"
            | "version.txt"
            | "user.lock"
            | "writing.history"
    )
}

/// Whether the path to a pick marks it as one of Scrivener's automatic backups.
pub(crate) fn backup_sign(pick: &Path) -> Option<BackupSign> {
    let mut sign = None;
    for component in pick.components() {
        let part = component.as_os_str().to_string_lossy();
        if part.eq_ignore_ascii_case("Backups") {
            sign = Some(BackupSign::BackupsFolder);
        }
        if is_date_stamped(&part) && sign.is_none() {
            sign = Some(BackupSign::DateStamped);
        }
    }
    let name = pick
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    if let Some(stem) = name.strip_suffix(".zip")
        && let Some(at) = stem.rfind(".bak")
        && stem[at + 4..].bytes().all(|b| b.is_ascii_digit())
    {
        return Some(BackupSign::BakArchive);
    }
    sign
}

/// A name stamped `[YYYY_MM_DD_HH_MM_SS]`, as Scrivener's timestamped backups
/// are.
fn is_date_stamped(name: &str) -> bool {
    name.char_indices().any(|(i, c)| {
        c == '[' && {
            let rest = &name.as_bytes()[i + 1..];
            rest.len() >= 20
                && rest[19] == b']'
                && rest[..19].iter().enumerate().all(|(j, b)| match j {
                    4 | 7 | 10 | 13 | 16 => *b == b'_',
                    _ => b.is_ascii_digit(),
                })
        }
    })
}

/// When `folder` is named `<X> Backup` (`.scriv` or not), the names its
/// upgraded sibling may have: `<X>.scriv` and `<X>`.
pub(crate) fn upgraded_sibling_names(folder: &str) -> Option<Vec<String>> {
    let (stem, ext) = match folder.len().checked_sub(6) {
        Some(at) if folder.is_char_boundary(at) && folder[at..].eq_ignore_ascii_case(".scriv") => {
            (&folder[..at], &folder[at..])
        }
        _ => (folder, ""),
    };
    let base = stem.len().checked_sub(" Backup".len()).and_then(|at| {
        (stem.is_char_boundary(at) && stem[at..].eq_ignore_ascii_case(" Backup"))
            .then(|| &stem[..at])
    })?;
    if base.is_empty() {
        return None;
    }
    let mut names = vec![format!("{base}{ext}")];
    if ext.is_empty() {
        names.push(format!("{base}.scriv"));
    } else {
        names.push(base.to_string());
    }
    Some(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_are_recognised_by_content() {
        assert_eq!(sniff(b"PK\x03\x04rest"), Sniffed::Zip);
        assert_eq!(
            sniff(b"\xEF\xBB\xBF<?xml?><ScrivenerProject Version=\"2.0\">"),
            Sniffed::Manifest
        );
        assert_eq!(
            sniff(b"<?xml?>\n<ScrivenerTemplate Version=\"1.0\">"),
            Sniffed::Template
        );
        assert_eq!(sniff(b"bplist00..."), Sniffed::BinaryPlist);
        assert_eq!(sniff(b"hello"), Sniffed::Other);
    }

    #[test]
    fn backup_signs_are_read_off_the_path() {
        assert_eq!(
            backup_sign(Path::new(
                "/Users/a/Library/Application Support/Scrivener/Backups/Novel.bak3.zip"
            )),
            Some(BackupSign::BakArchive)
        );
        assert_eq!(
            backup_sign(Path::new("/x/Backups/Novel.scriv")),
            Some(BackupSign::BackupsFolder)
        );
        assert_eq!(
            backup_sign(Path::new("/x/Novel [2025_06_01_05_14_50]/Novel.scriv")),
            Some(BackupSign::DateStamped)
        );
        assert_eq!(backup_sign(Path::new("/x/Writing/Novel.scriv")), None);
    }

    #[test]
    fn a_pre_upgrade_copy_names_its_sibling() {
        assert_eq!(
            upgraded_sibling_names("Novel Backup.scriv"),
            Some(vec!["Novel.scriv".to_string(), "Novel".to_string()])
        );
        assert_eq!(
            upgraded_sibling_names("Novel backup"),
            Some(vec!["Novel".to_string(), "Novel.scriv".to_string()])
        );
        assert_eq!(upgraded_sibling_names("Novel.scriv"), None);
        assert_eq!(upgraded_sibling_names(" Backup.scriv"), None);
    }
}
