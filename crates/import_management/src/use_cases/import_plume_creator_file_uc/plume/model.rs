// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The intermediate, in-memory Plume model produced by the parsers and consumed
//! by the mapper.
//!
//! It is **version-neutral**: the parsers (`tree_parse`, `attend_parse`,
//! `info_parse`) normalize every historical Plume schema — the pre-0.3 loose-file
//! "old system", the `plume` vs `plume-tree` root tag, the pre-0.4
//! `char`/`item`/`place` attendance elements, the pre-0.5 `level`/`role`
//! attributes — into these shapes (mirroring Plume's own `FileUpdater`), so the
//! mapper only ever sees one shape.

/// A manuscript-tree node kind. Plume's `act` is always a container element (an
/// optional layer between book and chapter), so it has no leaf variant here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlumeKind {
    Book,
    Act,
    Chapter,
    Scene,
    /// A scene-break marker; carries no content.
    Separator,
}

/// One node of the Plume outline tree.
#[derive(Debug, Clone)]
pub struct PlumeNode {
    pub kind: PlumeKind,
    /// Plume `number` — the project-wide id keying `text/T{n}.html` /
    /// `text/S{n}.html` / `text/N{n}.html`. `None` for a separator that predates
    /// the 0.3→0.4 numbering step (such a node simply has no text files).
    pub number: Option<u32>,
    pub name: String,
    /// True for a node reparented under `<trash>` **or** carrying
    /// `isTrashed="yes"` (propagated to every descendant during parsing). The
    /// mapper skips these and counts them into `skipped_trashed`.
    pub is_trashed: bool,
    /// Plume `badge` free-text label (maps to `BinderItemFile.label`).
    pub badge: String,
    /// Attendance object `number`s this node references — the union of the
    /// `attend=` (present characters) and `pov=` (point-of-view) lists, with the
    /// `0` "none" sentinel and duplicates removed.
    pub attend: Vec<u32>,
    pub children: Vec<PlumeNode>,
}

/// The parsed manuscript tree: the top-level `<book>`s in document order, plus
/// any trashed subtrees (flagged `is_trashed`) so the mapper can count them.
#[derive(Debug, Clone)]
pub struct PlumeTree {
    /// The root's `projectName` attribute (vestigial in Plume, but a useful title
    /// fallback when the `info` file is missing).
    pub project_name: String,
    pub roots: Vec<PlumeNode>,
}

/// A character / place / item entry in the story bible.
#[derive(Debug, Clone)]
pub struct PlumeObj {
    /// Plume `number` — keys `attend/A{n}.html` and is the cross-link target.
    pub number: Option<u32>,
    pub name: String,
    /// Other names this entry answers to, split on Plume's `--` list separator (the same
    /// one the box catalogs use). These become `BinderItem.aliases`, which is what lets
    /// the mention index find "Kiri" in prose for an entry named "Elise".
    pub aliases: Vec<String>,
    pub quick_details: String,
    /// The classification-box labels already resolved against the root `--`
    /// catalogs (empty string where a box is absent, index 0, or unresolvable).
    pub box_labels: [String; 3],
    /// The spin-box value as written (e.g. an age), or empty when absent / `"0"`.
    pub spinbox: String,
}

/// A story-bible group (Characters / Items / Places).
#[derive(Debug, Clone)]
pub struct PlumeGroup {
    /// Plume `number` — keys the group's own `attend/A{n}.html` (v0.6+), or
    /// `None` for older files where groups had no document.
    pub number: Option<u32>,
    pub name: String,
    pub objs: Vec<PlumeObj>,
}

/// The parsed story bible.
#[derive(Debug, Clone)]
pub struct PlumeAttendance {
    /// The spin-box label (e.g. `"Age :"`), folded into each obj's synopsis.
    pub spinbox_label: String,
    pub groups: Vec<PlumeGroup>,
}

/// Project metadata from the `info` file's `<prj>` element.
#[derive(Debug, Clone, Default)]
pub struct PlumeInfo {
    pub title: String,
    /// Best-effort RFC3339 timestamps (parsed from `creationDate`/`lastModified`
    /// when they are ISO 8601; `None` when absent or in Plume's localized format).
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
