// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where a new binder item lands — the pure topological half of "create".
//!
//! [`skribisto_model::recommendations`] says *what* to create and *how* it relates
//! to the anchor ([`Relation`]); this module resolves that relation against the
//! binder's flat, ordered item list into a concrete `(insert_index, indent)`.
//!
//! It is shared, not duplicated: the outline's "＋ Create" and the container tabs'
//! stream "Add / Insert" both anchor on an item and must place it identically. The
//! stream is the reason this had to come out of `OutlineViewModel` — its old
//! chapter-only ancestor could get away with "right after the anchor, at the
//! anchor's own indent" because every row it showed was a same-indent `Scene`. A
//! Full Part / Full Book stream mixes part heads, chapter heads and scenes, and the
//! default recommendation for those anchors is [`Relation::Child`], so that
//! shortcut would put new chapters *outside* the part they were added to.
//!
//! Plain functions over plain data, so both view-models can unit-test the placement
//! without a backend.

use std::collections::HashMap;

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use skribisto_model::{Relation, SubRoleExt};

/// `{item_id -> (role, indent, sub_role)}` for one binder's items — what the walks below
/// need, fetched once by the caller.
///
/// Carries `role` (not just `indent`/`sub_role`) so a Go-command traversal
/// (`crate::view_models::binder_ops::go_targets`) can resolve each row's
/// `skribisto_model::GoKind` — Chapter identity spans two role encodings
/// (`Item/ChapterScene` and `Folder/ChapterScene`), so `sub_role` alone cannot answer it.
pub type ItemMeta = HashMap<u64, (BinderItemRole, i64, BinderItemSubRole)>;

/// First index after `order[pos]`'s whole subtree: the next row whose indent is
/// `<= base_indent`. A leaf (nothing deeper follows) returns `pos + 1`.
///
/// (Mirrors `binder_item_management::move_items_uc::subtree_end`, reimplemented here
/// because `bastyde_ui` doesn't depend on that use-case crate.)
pub fn subtree_end(order: &[u64], meta: &ItemMeta, pos: usize, base_indent: i64) -> usize {
    let mut j = pos + 1;
    while j < order.len()
        && meta
            .get(&order[j])
            .map(|(_role, ind, _sr)| *ind)
            .unwrap_or(base_indent)
            > base_indent
    {
        j += 1;
    }
    j
}

/// `(position, indent)` of the nearest ancestor of `order[pos]` that opens a chapter
/// or a book — the target of a [`Relation::ParentSibling`] insertion. `None` if the
/// anchor has no such enclosing opener.
pub fn enclosing_opener(order: &[u64], meta: &ItemMeta, pos: usize) -> Option<(usize, i64)> {
    let mut cur = pos;
    let mut cur_indent = meta.get(order.get(pos)?)?.1;
    while cur > 0 {
        cur -= 1;
        let (_role, ind, sr) = meta.get(&order[cur])?;
        if *ind < cur_indent {
            if sr.opens_chapter() || sr.opens_book() {
                return Some((cur, *ind));
            }
            cur_indent = *ind;
        }
    }
    None
}

/// `(insert_index, indent)` for a new item placed by `relation` relative to the item
/// at `order[pos]` (whose indent is `anchor_indent`).
///
/// - `Sibling` lands after the anchor's **entire subtree**, at the anchor's own
///   indent — so a sibling of a populated folder follows its children.
/// - `Child` appends **inside** a folder anchor (indent + 1), but before any direct
///   child that `closes_book()`, so a Book's trailing `BookEnd` stays last.
/// - `ParentSibling` walks up to the nearest ancestor that opens a chapter or book
///   and behaves as `Sibling` of it — "close what I'm inside and start the next one".
pub fn insertion_point_for_item(
    order: &[u64],
    meta: &ItemMeta,
    pos: usize,
    anchor_indent: i64,
    relation: Relation,
) -> (usize, i64) {
    match relation {
        Relation::Sibling => {
            let end = subtree_end(order, meta, pos, anchor_indent);
            (end, anchor_indent)
        }
        Relation::Child => {
            let end = subtree_end(order, meta, pos, anchor_indent);
            let child_indent = anchor_indent + 1;
            let before_close = ((pos + 1)..end).find(|&k| {
                meta.get(&order[k])
                    .is_some_and(|(_role, ind, sr)| *ind == child_indent && sr.closes_book())
            });
            (before_close.unwrap_or(end), child_indent)
        }
        Relation::ParentSibling => {
            let (apos, aind) = enclosing_opener(order, meta, pos).unwrap_or((pos, anchor_indent));
            let end = subtree_end(order, meta, apos, aind);
            (end, aind)
        }
    }
}

/// The Next/Previous target (if any) for each [`skribisto_model::GoKind`], resolved
/// once from `pos`'s position in `order` — the Go menu's traversal math.
///
/// Deliberately answers all **six** independently of the row at `pos`'s own kind —
/// "Next Chapter" from inside a Scene still has its own answer, per the Go menu's
/// settled "six static rows" design (each row asks its own question, not "the next
/// thing of whatever kind I'm currently in"). Every field is `None` when nothing of
/// that kind lies in that direction — **no wraparound**, matching Scrivener's own
/// Previous/Next behaviour, and this never looks outside `order` (the caller's own
/// binder), so it never jumps to another binder either.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GoTargets {
    pub next_scene: Option<u64>,
    pub prev_scene: Option<u64>,
    pub next_chapter: Option<u64>,
    pub prev_chapter: Option<u64>,
    pub next_note: Option<u64>,
    pub prev_note: Option<u64>,
}

impl GoTargets {
    /// The resolved target for one (kind, direction) pair — the single lookup both the
    /// Go-menu enablement mirrors and the actual jump share, so they can never disagree
    /// about whether a row that fires would have found anywhere to go.
    pub fn get(
        &self,
        kind: skribisto_model::GoKind,
        direction: skribisto_model::GoDirection,
    ) -> Option<u64> {
        use skribisto_model::{GoDirection::*, GoKind::*};
        match (kind, direction) {
            (Scene, Next) => self.next_scene,
            (Scene, Previous) => self.prev_scene,
            (Chapter, Next) => self.next_chapter,
            (Chapter, Previous) => self.prev_chapter,
            (Note, Next) => self.next_note,
            (Note, Previous) => self.prev_note,
        }
    }
}

/// Walk `order` from `pos`, forward for the `next_*` fields and backward for the
/// `prev_*` fields, filling in [`GoTargets`] as each kind's first match is found.
pub fn go_targets_in(order: &[u64], meta: &ItemMeta, pos: usize) -> GoTargets {
    use skribisto_model::GoKind;

    let kind_at = |id: u64| {
        meta.get(&id)
            .and_then(|(role, _indent, sub_role)| skribisto_model::go_kind_of(role, sub_role))
    };

    let mut out = GoTargets::default();
    for &id in &order[pos + 1..] {
        match kind_at(id) {
            Some(GoKind::Scene) if out.next_scene.is_none() => out.next_scene = Some(id),
            Some(GoKind::Chapter) if out.next_chapter.is_none() => out.next_chapter = Some(id),
            Some(GoKind::Note) if out.next_note.is_none() => out.next_note = Some(id),
            _ => {}
        }
        if out.next_scene.is_some() && out.next_chapter.is_some() && out.next_note.is_some() {
            break;
        }
    }
    for &id in order[..pos].iter().rev() {
        match kind_at(id) {
            Some(GoKind::Scene) if out.prev_scene.is_none() => out.prev_scene = Some(id),
            Some(GoKind::Chapter) if out.prev_chapter.is_none() => out.prev_chapter = Some(id),
            Some(GoKind::Note) if out.prev_note.is_none() => out.prev_note = Some(id),
            _ => {}
        }
        if out.prev_scene.is_some() && out.prev_chapter.is_some() && out.prev_note.is_some() {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::BinderItemRole::Item;
    use frontend::common::entities::BinderItemSubRole::*;

    /// `Book(0) [ ChapterScene(1) [ Scene(2), Scene(2) ], BookEnd(1) ]`
    ///
    /// Every row is `role = Item` — this fixture exercises the pure indent/sub_role
    /// walk, which does not care which role a row carries. `go_targets_in`'s own tests
    /// below use a separate, mixed-role fixture, since Chapter identity is exactly the
    /// one place role matters.
    fn fixture() -> (Vec<u64>, ItemMeta) {
        let rows: Vec<(u64, i64, BinderItemSubRole)> = vec![
            (1, 0, Book),
            (2, 1, ChapterScene),
            (3, 2, Scene),
            (4, 2, Scene),
            (5, 1, BookEnd),
        ];
        let order = rows.iter().map(|r| r.0).collect();
        let meta = rows
            .into_iter()
            .map(|(id, ind, sr)| (id, (Item, ind, sr)))
            .collect();
        (order, meta)
    }

    #[test]
    fn sibling_lands_after_the_whole_subtree() {
        let (order, meta) = fixture();
        // Sibling of the chapter (pos 1, indent 1) → after both its scenes (index 4).
        assert_eq!(
            insertion_point_for_item(&order, &meta, 1, 1, Relation::Sibling),
            (4, 1)
        );
    }

    #[test]
    fn child_nests_inside_and_keeps_book_end_last() {
        let (order, meta) = fixture();
        // Child of the chapter → indent 2, after its last scene.
        assert_eq!(
            insertion_point_for_item(&order, &meta, 1, 1, Relation::Child),
            (4, 2)
        );
        // Child of the book → indent 1, but *before* the trailing BookEnd.
        assert_eq!(
            insertion_point_for_item(&order, &meta, 0, 0, Relation::Child),
            (4, 1),
            "a book's BookEnd must stay last"
        );
    }

    #[test]
    fn parent_sibling_closes_the_enclosing_chapter() {
        let (order, meta) = fixture();
        // ParentSibling of a scene (pos 2, indent 2) → sibling of its chapter, i.e.
        // after the chapter's whole subtree, at the chapter's indent.
        assert_eq!(
            insertion_point_for_item(&order, &meta, 2, 2, Relation::ParentSibling),
            (4, 1)
        );
    }

    /// A leaf anchor with nothing nested under it: its subtree is just itself.
    #[test]
    fn leaf_sibling_lands_immediately_after() {
        let (order, meta) = fixture();
        assert_eq!(
            insertion_point_for_item(&order, &meta, 2, 2, Relation::Sibling),
            (3, 2)
        );
    }

    // ── `go_targets_in` (the Go menu's traversal) ───────────────────────────

    use frontend::common::entities::BinderItemRole::Folder;

    /// A mixed-role stream, deliberately crossing chapter/part/book boundaries and
    /// covering both chapter encodings:
    ///
    /// `id1 Folder/Book(0), id2 Item/ChapterScene(1) [flat chapter],
    /// id3 Item/Scene(2), id4 Item/Scene(2),
    /// id5 Folder/ChapterScene(1) [chapter folder], id6 Item/Scene(2) [its child],
    /// id7 Item/Note(1), id8 Item/BookEnd(0)`
    fn go_fixture() -> (Vec<u64>, ItemMeta) {
        let rows: Vec<(u64, BinderItemRole, i64, BinderItemSubRole)> = vec![
            (1, Folder, 0, Book),
            (2, Item, 1, ChapterScene),
            (3, Item, 2, Scene),
            (4, Item, 2, Scene),
            (5, Folder, 1, ChapterScene),
            (6, Item, 2, Scene),
            (7, Item, 1, Note),
            (8, Item, 0, BookEnd),
        ];
        let order = rows.iter().map(|r| r.0).collect();
        let meta = rows
            .into_iter()
            .map(|(id, role, ind, sr)| (id, (role, ind, sr)))
            .collect();
        (order, meta)
    }

    /// From the very first row (the Book), every Next answer is the first row of its
    /// kind found forward — crossing the chapter boundary at id2 to find id3's scene,
    /// and reaching all the way to id7 for the note. No Previous answer exists yet.
    #[test]
    fn from_the_first_row_every_next_target_is_the_first_of_its_kind() {
        let (order, meta) = go_fixture();
        let targets = go_targets_in(&order, &meta, 0);
        assert_eq!(targets.next_scene, Some(3));
        assert_eq!(targets.next_chapter, Some(2));
        assert_eq!(targets.next_note, Some(7));
        // `Option::None`, not bare `None`: this module's `use ...BinderItemSubRole::*`
        // glob-imports the `SubRole::None` variant, which shadows `Option::None`.
        assert_eq!(targets.prev_scene, Option::None);
        assert_eq!(targets.prev_chapter, Option::None);
        assert_eq!(targets.prev_note, Option::None);
    }

    /// From the middle (id4, the flat chapter's second scene): Previous answers look
    /// backward (id3 the sibling scene, id2 the flat chapter, no note yet), and Next
    /// answers cross into the *folder*-encoded chapter (id5) and its child (id6),
    /// proving both chapter encodings are found as `GoKind::Chapter` alike.
    #[test]
    fn from_the_middle_both_directions_resolve_independently() {
        let (order, meta) = go_fixture();
        let pos = order.iter().position(|&id| id == 4).unwrap();
        let targets = go_targets_in(&order, &meta, pos);
        assert_eq!(targets.prev_scene, Some(3));
        assert_eq!(targets.prev_chapter, Some(2));
        assert_eq!(
            targets.prev_note,
            Option::None,
            "no note lies before this position"
        );
        assert_eq!(
            targets.next_chapter,
            Some(5),
            "the folder-encoded chapter must be found exactly like the flat one"
        );
        assert_eq!(targets.next_scene, Some(6));
        assert_eq!(targets.next_note, Some(7));
    }

    /// From the last row (BookEnd): every Previous answer resolves, and — with
    /// **no wraparound** — every Next answer is `None`, matching Scrivener's own
    /// Previous/Next behaviour rather than cycling back to the top.
    #[test]
    fn from_the_last_row_next_never_wraps_around() {
        let (order, meta) = go_fixture();
        let pos = order.len() - 1;
        let targets = go_targets_in(&order, &meta, pos);
        assert_eq!(targets.next_scene, Option::None);
        assert_eq!(targets.next_chapter, Option::None);
        assert_eq!(targets.next_note, Option::None);
        assert_eq!(targets.prev_scene, Some(6));
        assert_eq!(targets.prev_chapter, Some(5));
        assert_eq!(targets.prev_note, Some(7));
    }

    /// `GoTargets::get` is the one lookup both the menu mirrors and the actual jump
    /// share — pinned so the (kind, direction) → field mapping cannot silently drift.
    #[test]
    fn go_targets_get_maps_every_kind_and_direction() {
        let (order, meta) = go_fixture();
        let targets = go_targets_in(&order, &meta, 0);
        use skribisto_model::{GoDirection::*, GoKind::*};
        assert_eq!(targets.get(Scene, Next), targets.next_scene);
        assert_eq!(targets.get(Scene, Previous), targets.prev_scene);
        assert_eq!(targets.get(Chapter, Next), targets.next_chapter);
        assert_eq!(targets.get(Chapter, Previous), targets.prev_chapter);
        assert_eq!(targets.get(Note, Next), targets.next_note);
        assert_eq!(targets.get(Note, Previous), targets.prev_note);
    }
}
