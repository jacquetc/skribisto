// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The compile spine: fold the flat, ordered `(role, sub_role)` item stream into the
//! *selection* an exporter renders.
//!
//! Pure and IO-free — no `text-document`, no backend. These are the same structural
//! predicates the UI's Full Chapter / Part / Book stream already uses
//! (`StreamLevel` / `is_boundary` / `is_row` / `row_indices`, lifted here so both the
//! stream view and the exporter share one definition), plus the two things export needs
//! that the stream view does not: a **backward** scope-resolution walk
//! ([`enclosing_head`](crate::compile::enclosing_head)) and the "Current Book / Chapter / Scene / Note / Folder" scope
//! resolver ([`resolve_scope`](crate::compile::resolve_scope)).
//!
//! # Scope, not content
//!
//! [`resolve_scope`](crate::compile::resolve_scope) returns an ordered list of **item ids**. It is deliberately
//! preset-unaware: it does not know whether notes or synopses are wanted, or how a
//! heading is worded. It answers only "which items are in this scope", filtered to
//! `activated && is_exportable` for items *swept into* a multi-item scope (an explicitly
//! anchored single Scene/Note overrides that — the user pointed at it). The compiler
//! downstream decides what each item actually emits.

use common::entities::{BinderItemRole, BinderItemSubRole};

use crate::{SearchFacet, SubRoleExt, search_facet_of};

/// Which kind of container a stream is showing — it selects the boundary rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamLevel {
    Chapter,
    Part,
    Book,
}

impl StreamLevel {
    /// The level for a container head, or `None` if this `(role, sub_role)` has no
    /// stream. **The single place that decision is made.**
    ///
    /// Only *folder* containers get a stream. A flat `Item/ChapterScene` keeps its
    /// dual-pane writing editor; its scenes still stream inside the enclosing Full Part /
    /// Full Book, where it appears as a chapter-heading row carrying its own prose.
    pub fn for_container(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> Option<Self> {
        if *role != BinderItemRole::Folder {
            return None;
        }
        match sub_role {
            BinderItemSubRole::ChapterScene => Some(Self::Chapter),
            BinderItemSubRole::Part => Some(Self::Part),
            BinderItemSubRole::Book => Some(Self::Book),
            _ => None,
        }
    }
}

/// Does `sr` close a container at `level`? (Forward extent rule, for [`row_indices`].)
pub fn is_boundary(level: StreamLevel, sr: &BinderItemSubRole) -> bool {
    match level {
        // A chapter ends at the next chapter, part or book.
        StreamLevel::Chapter => {
            sr.opens_chapter() || sr.opens_part() || sr.opens_book() || sr.closes_book()
        }
        // A part ends at the next part or book — chapters live *inside* it.
        StreamLevel::Part => sr.opens_part() || sr.opens_book() || sr.closes_book(),
        // A book ends only at the next book, or at its explicit end marker.
        StreamLevel::Book => sr.opens_book() || sr.closes_book(),
    }
}

/// Is `sr` worth a row in a stream *view*? Scene-bearing items are the prose; chapter and
/// part heads are the structure headings that make a Full Book read as a manuscript.
///
/// This is the *view* predicate. Export scope building ([`resolve_scope`]) does **not**
/// use it — a folder of notes has no `is_row` item yet must still export — it includes
/// every `activated && is_exportable` item in extent and lets the compiler skip the
/// content-less ones.
pub fn is_row(sr: &BinderItemSubRole) -> bool {
    sr.carries_scene() || sr.opens_chapter() || sr.opens_part()
}

/// Indices (into `sub_roles`) of the rows belonging to the container head at `head`:
/// every row-worthy item forward from `head + 1` until `level`'s boundary (or the end of
/// the stream). The head itself is never a row — it is the pane header. `indent` / folder
/// nesting is deliberately not consulted.
pub fn row_indices(sub_roles: &[BinderItemSubRole], head: usize, level: StreamLevel) -> Vec<usize> {
    let mut out = Vec::new();
    for (i, sr) in sub_roles.iter().enumerate().skip(head + 1) {
        if is_boundary(level, sr) {
            break;
        }
        if is_row(sr) {
            out.push(i);
        }
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Export scope resolution
// ─────────────────────────────────────────────────────────────────────────────

/// One item in the flat, ordered stream, carrying the fields the scope algorithms need.
///
/// Built by the caller from either the frozen `Gathered` tree (backend export) or the
/// live binder query (the UI's adaptive Export split-button).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemMeta {
    pub id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    /// Binder-tree indent (0 = top level). Only the [`ScopeKind::Folder`] subtree walk
    /// consults it — book structure otherwise ignores nesting.
    pub indent: i32,
    /// `activated == !trashed`. A trashed item never exports.
    pub activated: bool,
    /// The user's per-item export toggle (defaults on). Filters items *swept into* a
    /// multi-item scope; an explicitly anchored single item overrides it.
    pub is_exportable: bool,
    /// The user's per-item numbering opt-out (defaults **off**, i.e. numbered) — the
    /// prologue lever. Read only by [`crate::numbering`]; scope resolution ignores it,
    /// because "does this row print a numeral" has nothing to do with "is this row in
    /// the export".
    ///
    /// Stated as the *exception* rather than as `numbered: bool` on purpose: `false` is
    /// the legacy state every existing project must load with, and `false` is what
    /// `bool::default()` — and therefore a bare `#[serde(default)]` — already gives.
    /// The positive spelling would need a custom `default_true` on every deserializer it
    /// touches, and forgetting one silently un-numbers a manuscript.
    pub exclude_from_numbering: bool,
}

/// What an export "Current …" action targets. Chosen from the focused item's facet by
/// [`primary_scope`]; `Custom` is the Choose… dialog, whose ids come straight from the
/// checkbox tree (never through [`resolve_scope`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    Book,
    Part,
    Chapter,
    Scene,
    Note,
    /// One paratext item on its own — a preface, an afterword. Like `Scene` and `Note` it
    /// is a single explicitly-pointed-at row, not a swept extent: a paratext has no
    /// children and no structural reach.
    Paratext,
    Folder,
    Custom,
}

/// The primary quick-export scope for a focused item — what the split-button's default
/// action names ("Export Scene" / "Export Note" / "Export Chapter" / …). `None` only for
/// `(role, sub_role)` pairs with no facet at all (there are none in the matrix).
pub fn primary_scope(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> Option<ScopeKind> {
    Some(match search_facet_of(role, sub_role)? {
        SearchFacet::Book => ScopeKind::Book,
        SearchFacet::Part => ScopeKind::Part,
        SearchFacet::Chapter => ScopeKind::Chapter,
        SearchFacet::Scene => ScopeKind::Scene,
        SearchFacet::Note => ScopeKind::Note,
        SearchFacet::Paratext => ScopeKind::Paratext,
        SearchFacet::Folder => ScopeKind::Folder,
    })
}

/// The head that opens the enclosing container at `level` for the item at `pos` (or `pos`
/// itself if it opens that level). `None` when a boundary *coarser than* `level` is hit
/// first walking backward — the item lives in a larger container but not in one of
/// `level`, so the corresponding quick scope is disabled rather than silently wrong.
///
/// Example: a scene directly under a Part (no chapter) has no enclosing *chapter*, so
/// "Export Chapter" is unavailable on it.
pub fn enclosing_head(items: &[ItemMeta], pos: usize, level: StreamLevel) -> Option<usize> {
    if pos >= items.len() {
        return None;
    }
    let opens = |sr: &BinderItemSubRole| match level {
        StreamLevel::Book => sr.opens_book(),
        StreamLevel::Part => sr.opens_part(),
        StreamLevel::Chapter => sr.opens_chapter(),
    };
    // Crossing one of these backward means the enclosing container of `level` does not
    // contain `pos`.
    let coarser_boundary = |sr: &BinderItemSubRole| match level {
        StreamLevel::Book => sr.closes_book(),
        StreamLevel::Part => sr.opens_book() || sr.closes_book(),
        StreamLevel::Chapter => sr.opens_part() || sr.opens_book() || sr.closes_book(),
    };

    if opens(&items[pos].sub_role) {
        return Some(pos);
    }
    for i in (0..pos).rev() {
        let sr = &items[i].sub_role;
        if opens(sr) {
            return Some(i);
        }
        if coarser_boundary(sr) {
            return None;
        }
    }
    None
}

/// The ordered item ids to include for a quick scope anchored at `focused`. `None` ⇒ the
/// scope does not apply (its menu entry is disabled / hidden).
///
/// - **Book / Part / Chapter** → the enclosing container's head plus every
///   `activated && is_exportable` item until that level's boundary. Not `is_row`-filtered:
///   notes swept in are included here and dropped by the compiler unless the preset keeps
///   them.
/// - **Scene / Note** → the single focused item, *iff* its sub_role matches exactly
///   (Scene never matches `ChapterScene`). Not `is_exportable`-filtered: an explicit pick
///   overrides the toggle.
/// - **Folder** → the folder's binder subtree (contiguous deeper-`indent` run).
/// - **Custom** → always `None`; Choose… supplies its ids directly.
pub fn resolve_scope(items: &[ItemMeta], focused: usize, scope: ScopeKind) -> Option<Vec<u64>> {
    if focused >= items.len() {
        return None;
    }
    match scope {
        ScopeKind::Custom => None,
        ScopeKind::Book => scope_extent(items, focused, StreamLevel::Book),
        ScopeKind::Part => scope_extent(items, focused, StreamLevel::Part),
        ScopeKind::Chapter => scope_extent(items, focused, StreamLevel::Chapter),
        ScopeKind::Scene => single(items, focused, BinderItemSubRole::Scene),
        ScopeKind::Note => single(items, focused, BinderItemSubRole::Note),
        ScopeKind::Paratext => single(items, focused, BinderItemSubRole::Paratext),
        ScopeKind::Folder => folder_subtree(items, focused),
    }
}

/// A single explicitly-anchored item — activated (never a trashed item) but *not*
/// `is_exportable`-gated: pointing the split-button at an item is an explicit override.
fn single(items: &[ItemMeta], focused: usize, want: BinderItemSubRole) -> Option<Vec<u64>> {
    let it = &items[focused];
    (it.sub_role == want && it.activated).then(|| vec![it.id])
}

fn scope_extent(items: &[ItemMeta], focused: usize, level: StreamLevel) -> Option<Vec<u64>> {
    let head = enclosing_head(items, focused, level)?;
    let mut ids = Vec::new();
    push_swept(&mut ids, &items[head]);
    for it in items.iter().skip(head + 1) {
        if is_boundary(level, &it.sub_role) {
            break;
        }
        push_swept(&mut ids, it);
    }
    (!ids.is_empty()).then_some(ids)
}

/// A generic (organizational) folder exports its **binder subtree**: the contiguous run
/// of following items whose `indent` exceeds the folder's. Folders are organizational, so
/// this is a tree subtree, not a `sub_role` boundary.
fn folder_subtree(items: &[ItemMeta], focused: usize) -> Option<Vec<u64>> {
    if items[focused].role != BinderItemRole::Folder {
        return None;
    }
    let base = items[focused].indent;
    let mut ids = Vec::new();
    push_swept(&mut ids, &items[focused]);
    for it in items.iter().skip(focused + 1) {
        if it.indent <= base {
            break;
        }
        push_swept(&mut ids, it);
    }
    (!ids.is_empty()).then_some(ids)
}

/// Include an item swept into a multi-item scope: only if live and exportable.
fn push_swept(ids: &mut Vec<u64>, it: &ItemMeta) {
    if it.activated && it.is_exportable {
        ids.push(it.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole as SR;

    /// A tiny stream builder. Each entry: `(id, role, sub_role, indent)`; all activated +
    /// exportable unless overridden with [`off`].
    fn meta(id: u64, role: BinderItemRole, sub_role: SR, indent: i32) -> ItemMeta {
        ItemMeta {
            id,
            role,
            sub_role,
            indent,
            activated: true,
            is_exportable: true,
            exclude_from_numbering: false,
        }
    }
    fn off(mut m: ItemMeta) -> ItemMeta {
        m.is_exportable = false;
        m
    }

    /// A flat novel: Book markers around two flat chapters, each with two scenes.
    fn flat_book() -> Vec<ItemMeta> {
        vec![
            meta(1, Item, SR::BookBegin, 0),    // 0: opens book
            meta(2, Item, SR::ChapterScene, 0), // 1: chapter 1 head
            meta(3, Item, SR::Scene, 1),        // 2
            meta(4, Item, SR::Scene, 1),        // 3
            meta(5, Item, SR::ChapterScene, 0), // 4: chapter 2 head
            meta(6, Item, SR::Scene, 1),        // 5
            meta(7, Item, SR::BookEnd, 0),      // 6: closes book
        ]
    }

    #[test]
    fn enclosing_book_chapter_and_none() {
        let s = flat_book();
        // A scene resolves up to its chapter and its book.
        assert_eq!(enclosing_head(&s, 2, StreamLevel::Chapter), Some(1));
        assert_eq!(enclosing_head(&s, 2, StreamLevel::Book), Some(0));
        // No parts in this book → no enclosing part.
        assert_eq!(enclosing_head(&s, 2, StreamLevel::Part), None);
        // The chapter head resolves to itself.
        assert_eq!(enclosing_head(&s, 4, StreamLevel::Chapter), Some(4));
    }

    #[test]
    fn current_chapter_is_head_plus_its_scenes() {
        let s = flat_book();
        // Anchored on scene id 3 (index 2): chapter 1 = head 2 + scenes 3,4.
        assert_eq!(
            resolve_scope(&s, 2, ScopeKind::Chapter),
            Some(vec![2, 3, 4])
        );
        // Anchored on chapter 2 head (index 4): head 5 + scene 6.
        assert_eq!(resolve_scope(&s, 4, ScopeKind::Chapter), Some(vec![5, 6]));
    }

    #[test]
    fn current_book_is_everything_up_to_the_end_marker() {
        // BookBegin (the title source) through the last scene; the content-less BookEnd
        // marker is the boundary and is excluded.
        let s = flat_book();
        assert_eq!(
            resolve_scope(&s, 3, ScopeKind::Book),
            Some(vec![1, 2, 3, 4, 5, 6])
        );
    }

    #[test]
    fn current_scene_is_exactly_one_and_never_a_chapter_scene() {
        let s = flat_book();
        assert_eq!(resolve_scope(&s, 2, ScopeKind::Scene), Some(vec![3]));
        // The chapter head carries a scene but is a ChapterScene, so "Current Scene" is off.
        assert_eq!(resolve_scope(&s, 1, ScopeKind::Scene), None);
    }

    #[test]
    fn non_exportable_scenes_are_dropped_from_a_swept_scope_but_not_an_explicit_pick() {
        let mut s = flat_book();
        s[3] = off(s[3].clone()); // scene id 4 marked non-exportable
        // Swept into "Current Chapter": dropped.
        assert_eq!(resolve_scope(&s, 2, ScopeKind::Chapter), Some(vec![2, 3]));
        // But explicitly "Export Scene" on it: included (explicit override).
        assert_eq!(resolve_scope(&s, 3, ScopeKind::Scene), Some(vec![4]));
    }

    #[test]
    fn folder_scope_is_the_indent_subtree() {
        // A notes folder with two notes, then a sibling scene outside it.
        let s = vec![
            meta(1, Folder, SR::Note, 0), // 0: notes folder
            meta(2, Item, SR::Note, 1),   // 1: note in it
            meta(3, Item, SR::Note, 1),   // 2: note in it
            meta(4, Item, SR::Scene, 0),  // 3: sibling, not in the folder
        ];
        // Folder facet → subtree = folder head + its two notes, not the sibling.
        assert_eq!(resolve_scope(&s, 0, ScopeKind::Folder), Some(vec![1, 2, 3]));
    }

    #[test]
    fn primary_scope_follows_the_facet() {
        assert_eq!(primary_scope(&Item, &SR::Scene), Some(ScopeKind::Scene));
        assert_eq!(primary_scope(&Item, &SR::Note), Some(ScopeKind::Note));
        assert_eq!(
            primary_scope(&Item, &SR::ChapterScene),
            Some(ScopeKind::Chapter)
        );
        assert_eq!(primary_scope(&Folder, &SR::Book), Some(ScopeKind::Book));
        assert_eq!(primary_scope(&Folder, &SR::None), Some(ScopeKind::Folder));
    }

    #[test]
    fn custom_never_resolves_here() {
        let s = flat_book();
        assert_eq!(resolve_scope(&s, 2, ScopeKind::Custom), None);
    }
}
