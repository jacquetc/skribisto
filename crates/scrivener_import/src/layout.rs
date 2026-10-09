// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where each item's files live, and what every other file in the project is.
//!
//! # Where the files are
//!
//! Formats 1.0 and 1.5 keep an item's files flat in `Files/Docs`, named by its
//! number (`12.rtf`, `12_notes.rtf`, `12_synopsis.txt`, `12.comments`, `12.links`,
//! `12.pdf`, `12_icImage.png`). Format 2.0 gives each item a folder named by its
//! UUID (`Files/Data/<UUID>/content.rtf`, `notes.rtf`, `synopsis.txt`,
//! `content.comments`, `content.styles`, `content.pdf`, `card-image.png`); an iOS
//! project never opened on a desktop keeps the same folders under `Mobile/Data`.
//! Which layout an item's files are looked for in follows the identifiers the
//! binder actually carries, not the version it claims: a format 1.5 item carries a
//! UUID too, and its files are still under its number.
//!
//! An empty document has no file at all, so a missing text is normal and says
//! nothing. What the files are *expected* to hold is `completeness`'s question.
//!
//! # Everything else
//!
//! Every member of the project is then classified against the grammar of files
//! Scrivener writes. What falls outside it is one of: litter an operating system
//! or a version control tool left; files of a Scrivener 1 project left beside a
//! later one; a copy a sync service made; a document file no binder item lists;
//! an item's own file under a second spelling (`abc…/` beside `ABC…/`, which only
//! a disk telling case apart holds); or something unrecognised, which is counted
//! and never imported. A text file the model does not carry, whatever put it
//! there, comes across as a recovered text rather than vanishing while the
//! importer knows it exists.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use uuid::Uuid;

use crate::disk::EntryKind;
use crate::index::{Entry, Index, ReadError, join};
use crate::model::{
    BinderRow, ConflictScheme, Counts, IdScheme, ItemKey, MemberRef, RecoveredOrigin,
    RecoveredText, TextRole,
};
use crate::names::{
    conflict_original, is_item_number, is_litter, is_uuid, placeholder_target, split_ext,
};

/// A view of the index from one project folder inside it.
pub(crate) struct Tree<'i, 'd> {
    pub(crate) index: &'i Index<'d>,
    /// The project folder's key in the index (`""` when the index is the
    /// project).
    pub(crate) root: String,
}

impl<'i, 'd> Tree<'i, 'd> {
    pub(crate) fn key(&self, rel: &str) -> String {
        if rel.is_empty() {
            self.root.clone()
        } else {
            join(&self.root, rel)
        }
    }

    /// A key of the index, relative to the project folder.
    pub(crate) fn rel<'k>(&self, key: &'k str) -> &'k str {
        if self.root.is_empty() {
            key
        } else {
            key.strip_prefix(self.root.as_str())
                .and_then(|rest| rest.strip_prefix('/'))
                .unwrap_or(key)
        }
    }

    /// The member present for a wanted path, relative to the project folder.
    pub(crate) fn lookup(&self, rel: &str) -> Option<String> {
        self.index
            .lookup(&self.key(rel))
            .map(|key| self.rel(&key).to_string())
    }

    pub(crate) fn entry(&self, rel: &str) -> Option<&'i Entry> {
        self.index.entry(&self.key(rel))
    }

    pub(crate) fn is_dir(&self, rel: &str) -> bool {
        self.entry(rel).is_some_and(|e| e.kind == EntryKind::Dir)
    }

    /// The names directly inside the folder `rel`, sorted.
    pub(crate) fn children(&self, rel: &str) -> Vec<String> {
        self.index.children(&self.key(rel))
    }

    /// Every member below the project folder, relative, sorted.
    pub(crate) fn members(&self) -> Vec<String> {
        self.index
            .keys_under(&self.root)
            .into_iter()
            .map(|key| self.rel(&key).to_string())
            .collect()
    }

    /// A reference to the regular file at `rel`.
    pub(crate) fn member_ref(&self, rel: &str) -> Option<MemberRef> {
        let entry = self.entry(rel)?;
        (entry.kind == EntryKind::File).then(|| MemberRef {
            path: rel.to_string(),
            size: entry.size,
            modified_ns: entry.modified_ns,
        })
    }

    pub(crate) fn read(&self, rel: &str, limit: u64) -> Result<Vec<u8>, ReadError> {
        self.index.read(&self.key(rel), limit)
    }

    pub(crate) fn is_placeholder(&self, rel: &str) -> bool {
        self.index.is_placeholder(&self.key(rel))
    }

    /// The folder on the way to `rel` that is a link, if any: what lies behind
    /// it is never read.
    pub(crate) fn link_on_the_way(&self, rel: &str) -> Option<String> {
        let parts: Vec<&str> = rel.split('/').collect();
        (1..parts.len()).find_map(|end| {
            let prefix = parts[..end].join("/");
            let present = self.lookup(&prefix)?;
            (self.entry(&present)?.kind == EntryKind::Symlink).then_some(present)
        })
    }

    pub(crate) fn unreadable_folder_of(&self, rel: &str) -> Option<String> {
        self.index
            .unreadable_folder_of(&self.key(rel))
            .map(|key| self.rel(key).to_string())
    }
}

/// Where the files the import needs could not be reached.
#[derive(Debug, Default)]
pub(crate) struct Blocked {
    pub(crate) links: Vec<String>,
    pub(crate) placeholders: Vec<String>,
    pub(crate) unreadable: Vec<String>,
}

/// What resolving the payload found, beside the payload itself.
#[derive(Debug, Default)]
pub(crate) struct Resolution {
    /// Every member the model now refers to, relative.
    pub(crate) used: BTreeSet<String>,
    /// Needed members that cannot be read.
    pub(crate) blocked: Blocked,
    /// Media items whose file is missing, by key.
    pub(crate) missing_media: Vec<ItemKey>,
    /// Media placeholders, not needed now.
    pub(crate) media_placeholders: u32,
}

/// Which folder an item's files live in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Layout {
    /// `<base>/<UUID>/…`: `Files/Data`, or `Mobile/Data` for an iOS stub.
    Folders(&'static str),
    /// `Files/Docs/<ID>…`.
    Flat,
}

impl Layout {
    pub(crate) fn for_scheme(scheme: IdScheme, stub: bool) -> Self {
        match (scheme, stub) {
            (_, true) => Layout::Folders("Mobile/Data"),
            (IdScheme::Uuid, false) => Layout::Folders("Files/Data"),
            (IdScheme::Id | IdScheme::IdAndUuid, false) => Layout::Flat,
        }
    }
}

/// A UUID the way Scrivener names its folders.
pub(crate) fn uuid_name(uuid: &Uuid) -> String {
    uuid.hyphenated().to_string().to_uppercase()
}

/// Fill each row's payload from where its layout keeps it.
pub(crate) fn resolve_payloads(
    tree: &Tree<'_, '_>,
    rows: &mut [BinderRow],
    layout: Layout,
) -> Resolution {
    let mut res = Resolution::default();
    // The folder listing of Files/Docs once, not once per item.
    let flat_names: HashMap<String, String> = if layout == Layout::Flat {
        match tree.lookup("Files/Docs") {
            Some(docs) => tree
                .children(&docs)
                .into_iter()
                .map(|name| (name.to_lowercase(), join(&docs, &name)))
                .collect(),
            None => HashMap::new(),
        }
    } else {
        HashMap::new()
    };
    for row in rows.iter_mut() {
        if !row.owns_key {
            continue;
        }
        let key = match row.key {
            Some(key) => key,
            None => continue,
        };
        // Where this item's files would be, by name.
        let want = |name: String| -> Option<String> {
            match layout {
                Layout::Folders(base) => {
                    let ItemKey::Uuid(uuid) = key else {
                        return None;
                    };
                    Some(format!("{base}/{}/{name}", uuid_name(&uuid)))
                }
                Layout::Flat => {
                    let number = row.number?;
                    Some(format!("Files/Docs/{number}{name}"))
                }
            }
        };
        // The names each role has in each layout.
        let (text, notes, synopsis, comments, links, text_styles, notes_styles) = match layout {
            Layout::Folders(_) => (
                want("content.rtf".into()),
                want("notes.rtf".into()),
                want("synopsis.txt".into()),
                want("content.comments".into()),
                None,
                want("content.styles".into()),
                want("notes.styles".into()),
            ),
            Layout::Flat => (
                want(".rtf".into()),
                want("_notes.rtf".into()),
                want("_synopsis.txt".into()),
                want(".comments".into()),
                want(".links".into()),
                None,
                None,
            ),
        };
        let found =
            |wanted: Option<String>, needed: bool, res: &mut Resolution| -> Option<MemberRef> {
                let wanted = wanted?;
                let rel = match layout {
                    Layout::Flat => {
                        let name = wanted.rsplit('/').next().unwrap_or_default().to_lowercase();
                        flat_names.get(&name).map(|key| tree.rel(key).to_string())
                    }
                    Layout::Folders(_) => tree.lookup(&wanted),
                };
                let Some(rel) = rel else {
                    if needed {
                        if tree.is_placeholder(&wanted) {
                            res.blocked.placeholders.push(wanted.clone());
                        } else if let Some(folder) = tree.unreadable_folder_of(&wanted) {
                            res.blocked.unreadable.push(folder);
                        } else if let Some(link) = tree.link_on_the_way(&wanted) {
                            // The item's folder is a link: its writing may be
                            // behind it, and a link is never followed.
                            res.blocked.links.push(link);
                        }
                    }
                    return None;
                };
                let entry = tree.entry(&rel)?;
                if entry.kind == EntryKind::Other {
                    // A FIFO, device or socket where a document should be: never
                    // opened (a FIFO would block the read for ever), and never
                    // taken for an empty document either.
                    if needed {
                        res.blocked.unreadable.push(rel.clone());
                    }
                    return None;
                }
                if entry.kind == EntryKind::Symlink {
                    if needed {
                        res.blocked.links.push(rel.clone());
                    }
                    res.used.insert(rel);
                    return None;
                }
                if tree.is_placeholder(&rel) {
                    if needed {
                        res.blocked.placeholders.push(rel.clone());
                    }
                    res.used.insert(rel);
                    return None;
                }
                let member = tree.member_ref(&rel);
                if member.is_some() {
                    res.used.insert(rel);
                }
                member
            };
        // Every kind, pictures and research files included: a Windows 1.8
        // project in the corpus keeps 625 words in a picture item's own `<ID>.rtf`,
        // and the file is that item's whatever its kind says.
        row.payload.text = found(text, true, &mut res);
        row.payload.notes = found(notes, true, &mut res);
        row.payload.synopsis = found(synopsis, true, &mut res);
        row.payload.comments = found(comments, true, &mut res);
        row.payload.links = found(links, true, &mut res);
        row.payload.text_styles = found(text_styles, true, &mut res);
        row.payload.notes_styles = found(notes_styles, true, &mut res);
        if row.kind.is_media() {
            let wanted = row.meta.file_extension.clone().map(|ext| match layout {
                Layout::Folders(_) => format!("content.{ext}"),
                Layout::Flat => format!(".{ext}"),
            });
            let wanted_path = wanted.and_then(&want);
            // Pictures and research files are read later, if at all: a
            // placeholder or a link among them does not block the import.
            let file = found(wanted_path.clone(), false, &mut res);
            if file.is_none() {
                match wanted_path {
                    Some(path) if tree.is_placeholder(&path) => res.media_placeholders += 1,
                    _ => res.missing_media.push(key),
                }
            }
            row.payload.file = file;
        }
        let card = match (layout, row.meta.card_image_extension.clone()) {
            (Layout::Folders(_), Some(ext)) => want(format!("card-image.{ext}")),
            (Layout::Flat, Some(ext)) => want(format!("_icImage.{ext}")),
            (Layout::Folders(_), None) => want(String::new()).and_then(|folder| {
                // No extension recorded: take the one card image the folder holds.
                let folder = folder.trim_end_matches('/').to_string();
                let dir = tree.lookup(&folder)?;
                let names: Vec<String> = tree
                    .children(&dir)
                    .into_iter()
                    .filter(|n| n.to_lowercase().starts_with("card-image."))
                    .collect();
                match names.as_slice() {
                    [only] => Some(join(&dir, only)),
                    _ => None,
                }
            }),
            (Layout::Flat, None) => None,
        };
        row.payload.card_image = found(card, false, &mut res);
    }
    res
}

/// What the member grammar found outside the model.
#[derive(Debug, Default)]
pub(crate) struct Debris {
    pub(crate) recovered: Vec<RecoveredText>,
    pub(crate) conflict_copies: u32,
    pub(crate) orphan_copies: u32,
    pub(crate) unreferenced_texts: u32,
    /// Empty document files no item owns: counted, never carried.
    pub(crate) empty_texts: u32,
    /// Pictures, research files and sidecars no item owns.
    pub(crate) unreferenced_other: u32,
    pub(crate) sync_debris: u32,
    pub(crate) legacy_v1: u32,
    pub(crate) unrecognised: Vec<String>,
    pub(crate) links: Vec<String>,
    pub(crate) specials: Vec<String>,
    pub(crate) placeholders: Vec<String>,
    /// Members of the grammar's recovered and payload sets that the import
    /// needs (recovered texts): they join the consistency set.
    pub(crate) used: BTreeSet<String>,
}

/// The file names Scrivener writes directly in `Files`.
const FILES_TOP: &[&str] = &[
    "version.txt",
    "binder.autosave",
    "binder.backup",
    "binder.mob",
    "search.indexes",
    "writing.history",
    "styles.xml",
    "user.lock",
];

/// What a document folder's member is, in format 2.0.
fn folder_member_role(name: &str) -> Option<FolderRole> {
    let lower = name.to_lowercase();
    Some(match lower.as_str() {
        "content.rtf" => FolderRole::Text(TextRole::Text),
        "notes.rtf" => FolderRole::Text(TextRole::Notes),
        "synopsis.txt" => FolderRole::Text(TextRole::Synopsis),
        "content.comments" | "content.styles" | "notes.styles" => FolderRole::Sidecar,
        _ if lower.starts_with("card-image.") && ext_ok(&lower["card-image.".len()..]) => {
            FolderRole::Media
        }
        _ if lower.starts_with("content.") && ext_ok(&lower["content.".len()..]) => {
            FolderRole::Media
        }
        _ => return None,
    })
}

/// What a `Files/Docs` member is, in formats 1.0 and 1.5, and the number it
/// names.
fn flat_member_role(name: &str) -> Option<(u32, FolderRole)> {
    let digits = name.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 || !is_item_number(&name[..digits]) {
        return None;
    }
    let number: u32 = name[..digits].parse().ok()?;
    let rest = name[digits..].to_lowercase();
    let role = match rest.as_str() {
        ".rtf" => FolderRole::Text(TextRole::Text),
        "_notes.rtf" => FolderRole::Text(TextRole::Notes),
        "_synopsis.txt" => FolderRole::Text(TextRole::Synopsis),
        ".links" | ".comments" => FolderRole::Sidecar,
        _ if rest.starts_with("_icimage.") && ext_ok(&rest["_icimage.".len()..]) => {
            FolderRole::Media
        }
        _ if rest.starts_with('.') && ext_ok(&rest[1..]) => FolderRole::Media,
        _ => return None,
    };
    Some((number, role))
}

fn ext_ok(ext: &str) -> bool {
    crate::names::is_extension(ext)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FolderRole {
    Text(TextRole),
    Sidecar,
    Media,
}

/// The shape of a snapshot file's name: `2019-10-21-23-16-18-0500.rtf`.
pub(crate) fn is_snapshot_file(name: &str) -> bool {
    let b = name.as_bytes();
    b.len() == 28
        && name.ends_with(".rtf")
        && b[..24].iter().enumerate().all(|(i, c)| match i {
            4 | 7 | 10 | 13 | 16 => *c == b'-',
            19 => *c == b'-' || *c == b'+',
            _ => c.is_ascii_digit(),
        })
}

/// Files of a Scrivener 1 project, at a project folder's top level.
fn is_mac1_top(name: &str) -> bool {
    matches!(
        name,
        "binder.scrivproj"
            | "info.plist"
            | "BinderStrings.xml"
            | "ui.xml"
            | "ExportSettings.xml"
            | "PageLayout.xml"
            | "scriptSettings.xml"
    ) || {
        let digits = name.bytes().take_while(u8::is_ascii_digit).count();
        digits > 0 && {
            let rest = &name[digits..];
            rest == "_synopsis.txt"
                || rest == ".links"
                || (rest.starts_with('.') && ext_ok(&rest[1..]))
        }
    }
}

/// A Scrivener 1 `.rtfd` package folder at the top level.
fn is_mac1_package(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower == "scratchpad.rtfd"
        || lower.strip_suffix(".rtfd").is_some_and(|stem| {
            let stem = stem.strip_suffix("_notes").unwrap_or(stem);
            is_item_number(stem)
        })
}

/// The inputs the grammar needs about the binder.
pub(crate) struct Referenced<'a> {
    /// UUIDs whose folder an item owns (upper-case names).
    pub(crate) folders: &'a BTreeSet<String>,
    /// Numbers whose flat files an item owns.
    pub(crate) numbers: &'a BTreeSet<u32>,
    /// The item that owns a number, for tying a copy to its item.
    pub(crate) number_keys: &'a HashMap<u32, ItemKey>,
    /// Whether this is an iOS stub, whose `Mobile/Data` holds the project.
    pub(crate) stub: bool,
}

/// Classify every member of the project that the model does not already use.
pub(crate) fn classify(
    tree: &Tree<'_, '_>,
    used: &BTreeSet<String>,
    refs: &Referenced<'_>,
) -> Debris {
    let mut debris = Debris::default();
    let members = tree.members();
    let present: BTreeSet<String> = members.iter().map(|m| m.to_lowercase()).collect();
    for rel in &members {
        let Some(entry) = tree.entry(rel) else {
            continue;
        };
        if entry.kind == EntryKind::Dir || used.contains(rel) {
            continue;
        }
        let parts: Vec<&str> = rel.split('/').collect();
        let base = parts.last().copied().unwrap_or_default();
        match entry.kind {
            EntryKind::Symlink => {
                debris.links.push(rel.clone());
                continue;
            }
            EntryKind::Other => {
                debris.specials.push(rel.clone());
                continue;
            }
            EntryKind::File | EntryKind::Dir => {}
        }
        if entry.placeholder {
            debris.placeholders.push(rel.clone());
            continue;
        }
        if is_litter(base) || placeholder_target(base).is_some() {
            continue;
        }
        let top = parts[0].to_lowercase();
        // Scrivener's own files that the model does not carry.
        if parts.len() == 1 && base.to_lowercase().ends_with(".scrivx") {
            continue;
        }
        if matches!(top.as_str(), "settings" | "quicklook" | "icons") && parts.len() > 1 {
            continue;
        }
        if parts.len() == 1 && is_mac1_top(base) {
            debris.legacy_v1 += 1;
            continue;
        }
        if parts.len() > 1 && is_mac1_package(parts[0]) {
            debris.legacy_v1 += 1;
            continue;
        }
        if parts.len() > 1 && base.to_lowercase().ends_with(".scrivx") {
            // A stale manifest inside the project (a template's leftover).
            continue;
        }
        if is_grammar(&parts, refs.stub) {
            // A payload-shaped file no item owns, or Scrivener's own bookkeeping.
            match payload_shape_raw(&parts, refs) {
                Some((FolderRole::Text(text_role), names, false)) => recover(
                    &mut debris,
                    tree,
                    rel,
                    text_role,
                    RecoveredOrigin::Unreferenced,
                    names,
                ),
                Some((FolderRole::Text(text_role), names, true)) => {
                    // Named like a file of an item that owns it, and not the one
                    // read for that item: the same name spelled otherwise (case,
                    // Unicode form, leading zeros), which only a disk telling the
                    // spellings apart holds. A second version of that document.
                    debris.conflict_copies += 1;
                    recover(
                        &mut debris,
                        tree,
                        rel,
                        text_role,
                        RecoveredOrigin::SyncConflict(ConflictScheme::Spelling),
                        names,
                    );
                }
                Some((FolderRole::Media | FolderRole::Sidecar, _, false)) => {
                    debris.unreferenced_other += 1;
                }
                Some((FolderRole::Media | FolderRole::Sidecar, _, true)) | None => {}
            }
            continue;
        }
        // A copy a sync service made: it counts only when its original is
        // there, or when the original would be a document.
        if let Some((scheme, original)) = conflict_original(base) {
            let mut original_parts = parts.clone();
            let last = original_parts.len() - 1;
            original_parts[last] = original.as_str();
            let original_rel = original_parts.join("/");
            let original_present = present.contains(&original_rel.to_lowercase());
            if let Some((FolderRole::Text(role), names)) = payload_shape_any(&original_parts, refs)
            {
                let origin = if original_present {
                    debris.conflict_copies += 1;
                    RecoveredOrigin::SyncConflict(scheme)
                } else {
                    debris.orphan_copies += 1;
                    RecoveredOrigin::OrphanCopy(scheme)
                };
                recover(&mut debris, tree, rel, role, origin, names);
                continue;
            }
            if original_present || is_grammar(&original_parts, refs.stub) {
                debris.sync_debris += 1;
                continue;
            }
        }
        // A copy named after a document file by a scheme not listed above
        // (OneDrive's `content-MACHINE.rtf`): inside an item's folder, a text file
        // whose name starts with a document's stem.
        if let Some((role, names)) = stem_copy(&parts, refs) {
            debris.conflict_copies += 1;
            recover(
                &mut debris,
                tree,
                rel,
                role,
                RecoveredOrigin::SyncConflict(ConflictScheme::Stem),
                names,
            );
            continue;
        }
        debris.unrecognised.push(rel.clone());
    }
    debris
}

fn recover(
    debris: &mut Debris,
    tree: &Tree<'_, '_>,
    rel: &str,
    role: TextRole,
    origin: RecoveredOrigin,
    names: Option<ItemKey>,
) {
    let Some(member) = tree.member_ref(rel) else {
        return;
    };
    if member.size == 0 {
        // An empty file holds nothing to recover.
        debris.empty_texts += 1;
        return;
    }
    if matches!(origin, RecoveredOrigin::Unreferenced) {
        debris.unreferenced_texts += 1;
    }
    debris.used.insert(rel.to_string());
    debris.recovered.push(RecoveredText {
        origin,
        role,
        member,
        names,
    });
}

/// Whether `parts` is a file Scrivener writes, in any generation's layout.
fn is_grammar(parts: &[&str], stub: bool) -> bool {
    let lower: Vec<String> = parts.iter().map(|p| p.to_lowercase()).collect();
    match (lower.len(), lower.first().map(String::as_str)) {
        (2, Some("files")) => FILES_TOP.contains(&lower[1].as_str()),
        (3, Some("files")) if lower[1] == "data" && lower[2] == "docs.checksum" => true,
        (3, Some("files")) if lower[1] == "docs" && lower[2] == "docs.checksum" => true,
        (3, Some("files")) if lower[1] == "docs" => flat_member_role(parts[2]).is_some(),
        (4, Some("files")) if lower[1] == "data" => {
            is_uuid(parts[2]) && folder_member_role(parts[3]).is_some()
        }
        (3, Some("files")) if lower[1] == "projectnotes" => {
            lower[2] == "index.xml"
                || lower[2]
                    .strip_prefix("note-")
                    .and_then(|r| r.strip_suffix(".rtf"))
                    .is_some_and(is_item_number)
        }
        (3, Some("snapshots")) => {
            lower[1].ends_with(".snapshots")
                && (lower[2] == "index.xml"
                    || lower[2] == "snapshot.indexes"
                    || is_snapshot_file(parts[2]))
        }
        (2, Some("mobile")) => {
            matches!(
                lower[1].as_str(),
                "binder.mob" | "writing.history" | "search.indexes" | "structure.cache"
            )
        }
        (3, Some("mobile")) if lower[1] == "data" => lower[2].ends_with(".checksums"),
        (4, Some("mobile")) if lower[1] == "data" => {
            // A desktop project's pending iOS edits are the mobile question's,
            // never recovered texts; an iOS stub's are its payload.
            let _ = stub;
            is_uuid(parts[2]) && folder_member_role(parts[3]).is_some()
        }
        _ => false,
    }
}

/// When `parts` is a document file in one of the payload layouts, its role and
/// the key its name gives, whether or not an item owns it.
fn payload_shape_any(
    parts: &[&str],
    refs: &Referenced<'_>,
) -> Option<(FolderRole, Option<ItemKey>)> {
    payload_shape_raw(parts, refs).map(|(role, names, _)| (role, names))
}

/// When `parts` is a document file in one of the payload layouts, its role, the
/// key its name gives, and whether an item owns that key's files.
fn payload_shape_raw(
    parts: &[&str],
    refs: &Referenced<'_>,
) -> Option<(FolderRole, Option<ItemKey>, bool)> {
    let lower: Vec<String> = parts.iter().map(|p| p.to_lowercase()).collect();
    match lower.len() {
        4 if (lower[0] == "files" || (lower[0] == "mobile" && refs.stub)) && lower[1] == "data" => {
            if !is_uuid(parts[2]) {
                return None;
            }
            let role = folder_member_role(parts[3])?;
            let uuid = Uuid::parse_str(parts[2]).ok();
            let owned = refs.folders.contains(&parts[2].to_uppercase())
                && (lower[0] == "mobile") == refs.stub;
            Some((role, uuid.map(ItemKey::Uuid), owned))
        }
        3 if lower[0] == "files" && lower[1] == "docs" => {
            let (number, role) = flat_member_role(parts[2])?;
            let owned = refs.numbers.contains(&number);
            let names = refs
                .number_keys
                .get(&number)
                .copied()
                .or(Some(ItemKey::Number(number)));
            Some((role, names, owned))
        }
        _ => None,
    }
}

/// A text file in a document folder named after a document's stem, and the
/// role and key it copies.
fn stem_copy(parts: &[&str], refs: &Referenced<'_>) -> Option<(TextRole, Option<ItemKey>)> {
    let lower: Vec<String> = parts.iter().map(|p| p.to_lowercase()).collect();
    let (stem, ext) = split_ext(&lower[lower.len() - 1]);
    if ext != ".rtf" && ext != ".txt" {
        return None;
    }
    match lower.len() {
        4 if lower[0] == "files" && lower[1] == "data" && is_uuid(parts[2]) => {
            let role = [
                ("content", TextRole::Text),
                ("notes", TextRole::Notes),
                ("synopsis", TextRole::Synopsis),
            ]
            .into_iter()
            .find(|(s, _)| stem.starts_with(s) && stem.len() > s.len())
            .map(|(_, role)| role)?;
            Some((role, Uuid::parse_str(parts[2]).ok().map(ItemKey::Uuid)))
        }
        3 if lower[0] == "files" && lower[1] == "docs" => {
            let digits = stem.bytes().take_while(u8::is_ascii_digit).count();
            if digits == 0 || !is_item_number(&stem[..digits]) {
                return None;
            }
            let number: u32 = stem[..digits].parse().ok()?;
            let rest = &stem[digits..];
            let role = if rest.starts_with("_notes") && rest.len() > "_notes".len() {
                TextRole::Notes
            } else if rest.starts_with("_synopsis") && rest.len() > "_synopsis".len() {
                TextRole::Synopsis
            } else if !rest.is_empty() && !rest.starts_with('_') {
                TextRole::Text
            } else {
                return None;
            };
            Some((
                role,
                refs.number_keys
                    .get(&number)
                    .copied()
                    .or(Some(ItemKey::Number(number))),
            ))
        }
        _ => None,
    }
}

/// Snapshots per item, and snapshot folders naming no item.
pub(crate) fn count_snapshots(tree: &Tree<'_, '_>, rows: &mut [BinderRow], counts: &mut Counts) {
    let Some(folder) = tree.lookup("Snapshots") else {
        return;
    };
    let mut by_name: BTreeMap<String, u32> = BTreeMap::new();
    for name in tree.children(&folder) {
        let Some(stem) = name
            .strip_suffix(".snapshots")
            .or_else(|| name.strip_suffix(".SNAPSHOTS"))
        else {
            continue;
        };
        let dir = join(&folder, &name);
        if !tree.is_dir(&dir) {
            continue;
        }
        let taken = tree
            .children(&dir)
            .iter()
            .filter(|n| is_snapshot_file(n))
            .count() as u32;
        by_name.insert(stem.to_uppercase(), taken);
    }
    let mut claimed = BTreeSet::new();
    for row in rows.iter_mut() {
        if !row.owns_key {
            continue;
        }
        let mut names = Vec::new();
        if let Some(ItemKey::Uuid(uuid)) = row.key {
            names.push(uuid_name(&uuid));
        }
        if let Some(number) = row.number {
            names.push(number.to_string());
        }
        for name in names {
            if let Some(taken) = by_name.get(&name) {
                row.snapshots += taken;
                claimed.insert(name);
            }
        }
        if row.snapshots > 0 {
            counts.items_with_snapshots += 1;
            counts.snapshots += row.snapshots;
        }
    }
    for (name, taken) in &by_name {
        if !claimed.contains(name) {
            counts.orphan_snapshot_folders += 1;
            counts.orphan_snapshots = counts.orphan_snapshots.saturating_add(*taken);
        }
    }
}

/// Whether a folder holds at least one document file of an item: the test that
/// tells a copy whose documents never arrived from a project that has some.
pub(crate) fn has_payload(tree: &Tree<'_, '_>) -> bool {
    if let Some(docs) = tree.lookup("Files/Docs")
        && tree.children(&docs).iter().any(|name| {
            tree.entry(&join(&docs, name))
                .is_some_and(|e| e.kind == EntryKind::File)
        })
    {
        return true;
    }
    if let Some(data) = tree.lookup("Files/Data") {
        for name in tree.children(&data) {
            let dir = join(&data, &name);
            if is_uuid(&name) && tree.is_dir(&dir) && !tree.children(&dir).is_empty() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_file_names_follow_their_date() {
        assert!(is_snapshot_file("2019-10-21-23-16-18-0500.rtf"));
        assert!(is_snapshot_file("2006-08-23-19-33-44+0100.rtf"));
        assert!(!is_snapshot_file("index.xml"));
        assert!(!is_snapshot_file("2019-10-21-23-16-18.rtf"));
    }

    #[test]
    fn flat_document_names_carry_their_number_and_role() {
        assert_eq!(
            flat_member_role("12.rtf"),
            Some((12, FolderRole::Text(TextRole::Text)))
        );
        assert_eq!(
            flat_member_role("12_notes.rtf"),
            Some((12, FolderRole::Text(TextRole::Notes)))
        );
        assert_eq!(
            flat_member_role("12_synopsis.txt"),
            Some((12, FolderRole::Text(TextRole::Synopsis)))
        );
        assert_eq!(
            flat_member_role("12.links"),
            Some((12, FolderRole::Sidecar))
        );
        assert_eq!(flat_member_role("12.pdf"), Some((12, FolderRole::Media)));
        assert_eq!(
            flat_member_role("12_icImage.png"),
            Some((12, FolderRole::Media))
        );
        assert_eq!(flat_member_role("12_2.rtf"), None);
        assert_eq!(flat_member_role("docs.checksum"), None);
    }
}
