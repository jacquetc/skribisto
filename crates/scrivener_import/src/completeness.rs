// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which document files the project says exist and the disk does not have.
//!
//! An empty document has no file at all, so a missing file on its own says
//! nothing. What does is Scrivener's search index (`Files/search.indexes`, or
//! `Mobile/search.indexes` for an iOS project): it holds each document's plain
//! text, notes and synopsis, so a file that is missing while the index remembers
//! words for it is writing that did not arrive. Those words are kept, as plain
//! text flagged as possibly an older version: the index can lag behind the files,
//! and one real index still held the template text its writer had replaced.
//!
//! `docs.checksum` is not trusted for this: Scrivener 3 for Mac leaves 58 % of its
//! entries stale, and entries outlive the files Scrivener itself deletes (an
//! emptied notes pane). It is used only when there is no readable index, and for
//! one thing it does say reliably: a file that is empty on disk although it was
//! recorded with content (a digest other than the empty file's) was cut short by a
//! sync. Nothing is ever hashed.

use std::collections::{BTreeSet, HashMap};

use crate::index::ReadError;
use crate::layout::{Layout, Tree, uuid_name};
use crate::model::{BinderRow, ItemKey, TextRole};
use crate::outcome::{Notice, ReadProblem};
use crate::xml::{self, child, children, text};

/// The largest search index read. Real ones run to a few megabytes.
const MAX_INDEX_BYTES: u64 = 256 << 20;
/// The largest checksum list read.
const MAX_CHECKSUM_BYTES: u64 = 16 << 20;
/// SHA-1 of nothing: the digest of a file that is empty on purpose.
const EMPTY_SHA1: &str = "da39a3ee5e6b4b0d3255bfef95601890afd80709";

/// What the check found.
#[derive(Debug, Default)]
pub(crate) struct Found {
    pub(crate) notices: Vec<Notice>,
    /// Fields salvaged from the index.
    pub(crate) salvaged: u32,
    /// Members read, for the consistency set.
    pub(crate) used: BTreeSet<String>,
    /// Whether a readable index was there.
    pub(crate) index_read: bool,
    /// The index or the checksum list changed under the read.
    pub(crate) changed: bool,
}

/// Look for missing document files, salvaging what the index remembers.
pub(crate) fn check(tree: &Tree<'_, '_>, rows: &mut [BinderRow], layout: Layout) -> Found {
    let mut found = Found::default();
    let index_path = match layout {
        Layout::Folders("Mobile/Data") => "Mobile/search.indexes",
        _ => "Files/search.indexes",
    };
    let checksum_path = match layout {
        Layout::Folders("Files/Data") => Some("Files/Data/docs.checksum"),
        Layout::Flat => Some("Files/Docs/docs.checksum"),
        Layout::Folders(_) => None,
    };
    let index_rel = tree.lookup(index_path);
    let mut index_ok = false;
    if let Some(rel) = &index_rel {
        match tree.read(rel, MAX_INDEX_BYTES) {
            Ok(bytes) => {
                found.used.insert(rel.clone());
                match salvage_from_index(rel, &bytes, rows, &mut found) {
                    Ok(()) => index_ok = true,
                    Err(problem) => found.notices.push(Notice::MetadataUnreadable {
                        member: rel.clone(),
                        problem,
                    }),
                }
            }
            Err(ReadError::Changed) => found.changed = true,
            Err(e) => {
                if let Some(problem) = e.problem() {
                    found.notices.push(Notice::MetadataUnreadable {
                        member: rel.clone(),
                        problem,
                    });
                }
            }
        }
    }
    found.index_read = index_ok;
    if let Some(path) = checksum_path
        && let Some(rel) = tree.lookup(path)
        && let Ok(bytes) = tree.read(&rel, MAX_CHECKSUM_BYTES)
    {
        found.used.insert(rel.clone());
        let listing = parse_checksum(&bytes);
        truncated_files(rows, layout, &listing, &mut found);
        if !index_ok {
            listed_but_missing(rows, layout, &listing, &mut found);
        }
    }
    found
}

/// The names the index may file an item under: its UUID, its number.
fn index_names(row: &BinderRow) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(ItemKey::Uuid(uuid)) = row.key {
        names.push(uuid_name(&uuid));
    }
    if let Some(number) = row.number {
        names.push(number.to_string());
    }
    names
}

fn salvage_from_index(
    rel: &str,
    bytes: &[u8],
    rows: &mut [BinderRow],
    found: &mut Found,
) -> Result<(), ReadProblem> {
    let text_of = xml::text_of(bytes).map_err(|p| p.problem())?;
    let doc = xml::parse(rel, text_of).map_err(|p| p.problem())?;
    let root = doc.root_element();
    // An index without its document list holds nothing to salvage from.
    let Some(documents) = child(root, "Documents") else {
        return Ok(());
    };
    let mut by_id: HashMap<String, roxmltree::Node<'_, '_>> = HashMap::new();
    for document in children(documents, "Document") {
        if let Some(id) = document.attribute("ID") {
            by_id.insert(id.trim().to_uppercase(), document);
        }
    }
    for row in rows.iter_mut() {
        let Some(key) = row.key.filter(|_| row.owns_key) else {
            continue;
        };
        let Some(document) = index_names(row)
            .iter()
            .find_map(|name| by_id.get(name).copied())
        else {
            continue;
        };
        let words = |field: &str| -> Option<String> {
            child(document, field)
                .and_then(text)
                .filter(|t| !t.trim().is_empty())
        };
        if row.kind.holds_text()
            && row.payload.text.is_none()
            && let Some(words) = words("Text")
        {
            row.payload.salvaged.text = Some(words);
            found.salvaged += 1;
            found.notices.push(Notice::ContentMissing {
                item: key,
                role: TextRole::Text,
                salvaged: true,
            });
        }
        if row.payload.notes.is_none()
            && let Some(words) = words("Notes")
        {
            row.payload.salvaged.notes = Some(words);
            found.salvaged += 1;
            found.notices.push(Notice::ContentMissing {
                item: key,
                role: TextRole::Notes,
                salvaged: true,
            });
        }
        if row.payload.synopsis.is_none()
            && let Some(words) = words("Synopsis")
        {
            row.payload.salvaged.synopsis = Some(words);
            found.salvaged += 1;
            found.notices.push(Notice::ContentMissing {
                item: key,
                role: TextRole::Synopsis,
                salvaged: true,
            });
        }
    }
    Ok(())
}

/// One checksum line: the file it names (lower-cased) and its digest.
struct Listed {
    name: String,
    digest: String,
}

fn parse_checksum(bytes: &[u8]) -> Vec<Listed> {
    String::from_utf8_lossy(bytes)
        .lines()
        .filter_map(|line| {
            let (name, digest) = line.trim().rsplit_once('=')?;
            let digest = digest.trim().to_lowercase();
            (digest.len() == 40 && digest.bytes().all(|b| b.is_ascii_hexdigit())).then(|| Listed {
                name: name.trim().to_lowercase(),
                digest,
            })
        })
        .collect()
}

/// The checksum name of each of an item's text files, and the role it holds.
fn checksum_names(row: &BinderRow, layout: Layout) -> Vec<(String, TextRole)> {
    match (layout, row.key) {
        (Layout::Folders(_), Some(ItemKey::Uuid(uuid))) => {
            let folder = uuid_name(&uuid).to_lowercase();
            vec![
                (format!("{folder}/content.rtf"), TextRole::Text),
                (format!("{folder}/notes.rtf"), TextRole::Notes),
                (format!("{folder}/synopsis.txt"), TextRole::Synopsis),
            ]
        }
        (Layout::Flat, _) => match row.number {
            Some(n) => vec![
                (format!("{n}.rtf"), TextRole::Text),
                (format!("{n}_notes.rtf"), TextRole::Notes),
                (format!("{n}_synopsis.txt"), TextRole::Synopsis),
            ],
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn payload_of(row: &BinderRow, role: TextRole) -> Option<&crate::model::MemberRef> {
    match role {
        TextRole::Text => row.payload.text.as_ref(),
        TextRole::Notes => row.payload.notes.as_ref(),
        TextRole::Synopsis => row.payload.synopsis.as_ref(),
    }
}

/// Without a readable index: a text file the checksum lists that is absent.
fn listed_but_missing(rows: &[BinderRow], layout: Layout, listing: &[Listed], found: &mut Found) {
    let listed: BTreeSet<&str> = listing.iter().map(|l| l.name.as_str()).collect();
    for row in rows {
        let Some(key) = row.key.filter(|_| row.owns_key) else {
            continue;
        };
        for (name, role) in checksum_names(row, layout) {
            if role == TextRole::Text && !row.kind.holds_text() {
                continue;
            }
            if payload_of(row, role).is_none() && listed.contains(name.as_str()) {
                found.notices.push(Notice::ContentMissing {
                    item: key,
                    role,
                    salvaged: false,
                });
            }
        }
    }
}

/// A text file present, empty, and recorded with content.
fn truncated_files(rows: &[BinderRow], layout: Layout, listing: &[Listed], found: &mut Found) {
    let recorded: HashMap<&str, &str> = listing
        .iter()
        .map(|l| (l.name.as_str(), l.digest.as_str()))
        .collect();
    for row in rows {
        let Some(key) = row.key.filter(|_| row.owns_key) else {
            continue;
        };
        for (name, role) in checksum_names(row, layout) {
            let Some(member) = payload_of(row, role) else {
                continue;
            };
            if member.size != 0 {
                continue;
            }
            if let Some(digest) = recorded.get(name.as_str())
                && *digest != EMPTY_SHA1
            {
                found
                    .notices
                    .push(Notice::ContentTruncated { item: key, role });
            }
        }
    }
}
