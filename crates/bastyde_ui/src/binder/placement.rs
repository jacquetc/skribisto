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

use frontend::common::entities::BinderItemSubRole;
use skribisto_model::{Relation, SubRoleExt};

/// `{item_id -> (indent, sub_role)}` for one binder's items — what the walks below
/// need, fetched once by the caller.
pub type ItemMeta = HashMap<u64, (i64, BinderItemSubRole)>;

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
            .map(|(ind, _)| *ind)
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
    let mut cur_indent = meta.get(order.get(pos)?)?.0;
    while cur > 0 {
        cur -= 1;
        let (ind, sr) = meta.get(&order[cur])?;
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
                    .is_some_and(|(ind, sr)| *ind == child_indent && sr.closes_book())
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

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::BinderItemSubRole::*;

    /// `Book(0) [ ChapterScene(1) [ Scene(2), Scene(2) ], BookEnd(1) ]`
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
            .map(|(id, ind, sr)| (id, (ind, sr)))
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
}
