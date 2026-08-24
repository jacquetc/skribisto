// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Panic hygiene: this crate is at zero `unwrap()`/`expect()`/`panic!` outside
// tests, so the lint is switched on here to keep it that way — CI lints with
// `-D warnings`, which makes any new panic path a build failure. See the note
// in the workspace `Cargo.toml` for why this is per-crate and not workspace-wide.
#![warn(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! Pure primitives over a Binder's ordered `binder_items` relationship vec and
//! each item's `indent`.
//!
//! There is no parent/child graph in the model: hierarchy is encoded purely by
//! position + `indent` inside `Binder.binder_items`. A "subtree rooted at X" is
//! X plus every following item whose `indent` is strictly greater, up to (but
//! not including) the first item at or below X's indent.
//!
//! No unit-of-work access here — callers fetch the order/indent map themselves
//! and pass it in; these functions only compute. Shared by
//! `binder_item_management::move_items` and
//! `trash_management::restore_items_to` so the ordering math lives once.

use anyhow::{Result, anyhow};
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

/// Where a relocated block lands relative to an anchor. Mirrors
/// `binder_item_management::MovePlace` and `trash_management::DropPosition` —
/// each caller maps its own DTO enum into this one at the call boundary (Qleany
/// DTO enums can't be shared across crates).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropPlace {
    Before,
    After,
    Into,
}

/// The contiguous subtree rooted at `root`: `root` plus every following item in
/// `order` whose indent is strictly greater than `root`'s, stopping at the first
/// item at or below `root`'s indent (or end-of-list). `order` must be the
/// binder's full relationship-vec order (never raw entity-scan order). Returns
/// an empty vec if `root` is not in `order`.
pub fn subtree_of(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    root: EntityId,
) -> Vec<EntityId> {
    let Some(pos) = order.iter().position(|&x| x == root) else {
        return Vec::new();
    };
    let root_indent = *indent.get(&root).unwrap_or(&0);
    let mut out = vec![root];
    let mut j = pos + 1;
    while j < order.len() {
        if *indent.get(&order[j]).unwrap_or(&0) <= root_indent {
            break;
        }
        out.push(order[j]);
        j += 1;
    }
    out
}

/// The chain of ids that enclose the item at `order[pos]`, nearest first: the
/// closest earlier item one indent level up, then the one above that, and so on
/// to a root (indent 0) item. Empty for a root-level item itself.
///
/// Domain-blind on purpose, like every other function here: it answers "what
/// encloses this position" from `order`/`indent` alone. A caller that needs to
/// know whether any of those enclosing ids is itself some particular kind of row
/// (a `Folder/Book`, say) fetches that separately and filters this chain; teaching
/// this crate what a Book is would be exactly the kind of domain leak its own
/// module docs warn against.
pub fn ancestors_of(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    pos: usize,
) -> Vec<EntityId> {
    let mut chain = Vec::new();
    let Some(&start) = order.get(pos) else {
        return chain;
    };
    let mut floor = *indent.get(&start).unwrap_or(&0);
    let mut j = pos;
    while j > 0 && floor > 0 {
        j -= 1;
        let ind = *indent.get(&order[j]).unwrap_or(&0);
        if ind < floor {
            chain.push(order[j]);
            floor = ind;
        }
    }
    chain
}

/// First index `j > pos` whose item indent is `<= base_indent`, or `order.len()`
/// — i.e. the end (exclusive) of the subtree rooted at `pos`.
pub fn subtree_end(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    pos: usize,
    base_indent: i64,
) -> usize {
    let mut j = pos + 1;
    while j < order.len() {
        if *indent.get(&order[j]).unwrap_or(&0) <= base_indent {
            break;
        }
        j += 1;
    }
    j
}

/// Expand a requested id set to full contiguous subtrees, in `order`'s order,
/// deduplicating nested selections (a descendant already covered by an earlier
/// requested ancestor's subtree is not repeated).
pub fn expand_to_subtrees(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    requested: &HashSet<EntityId>,
) -> Vec<EntityId> {
    let mut full: Vec<EntityId> = Vec::new();
    let mut seen: HashSet<EntityId> = HashSet::new();
    let mut i = 0usize;
    while i < order.len() {
        let id = order[i];
        if requested.contains(&id) && !seen.contains(&id) {
            let root_indent = *indent.get(&id).unwrap_or(&0);
            let mut j = i;
            loop {
                let cur = order[j];
                full.push(cur);
                seen.insert(cur);
                j += 1;
                if j >= order.len() {
                    break;
                }
                if *indent.get(&order[j]).unwrap_or(&0) <= root_indent {
                    break;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
    full
}

/// Insert `block` into `base` immediately before `anchor` (or at the end when
/// `anchor` is `None`). `block` ids are assumed absent from `base`.
pub fn insert_block(
    base: &[EntityId],
    block: &[EntityId],
    anchor: Option<EntityId>,
) -> Vec<EntityId> {
    let mut out = Vec::with_capacity(base.len() + block.len());
    let mut inserted = false;
    for &id in base {
        if Some(id) == anchor {
            out.extend_from_slice(block);
            inserted = true;
        }
        out.push(id);
    }
    if !inserted {
        out.extend_from_slice(block);
    }
    out
}

/// Resolve a drop directly onto a binder (no anchor item): `Before`/`Into` land
/// at the top of the list, `After` at the bottom. `exclude` = ids being
/// relocated (skipped when scanning for the first surviving id to anchor
/// before). Returns the anchor id to insert before (`None` = append at end).
pub fn anchor_for_binder_target(
    dest_order: &[EntityId],
    place: DropPlace,
    exclude: &HashSet<EntityId>,
) -> Option<EntityId> {
    match place {
        DropPlace::After => None,
        DropPlace::Before | DropPlace::Into => {
            dest_order.iter().copied().find(|id| !exclude.contains(id))
        }
    }
}

/// Resolve a drop relative to an existing anchor item already present in
/// `dest_order`. Returns `(base_indent_for_moved_root, insertion_anchor)`.
/// `Into` a non-folder anchor falls back to `After` it (sibling indent).
/// `exclude` = ids being relocated (skipped when scanning for the insertion
/// anchor). Errors if `anchor_id` is not in `dest_order`.
pub fn resolve_item_target(
    dest_order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    anchor_id: EntityId,
    anchor_indent: i64,
    anchor_is_folder: bool,
    place: DropPlace,
    exclude: &HashSet<EntityId>,
) -> Result<(i64, Option<EntityId>)> {
    let anchor_pos = dest_order
        .iter()
        .position(|&x| x == anchor_id)
        .ok_or_else(|| {
            anyhow!("resolve_item_target: anchor {anchor_id} not in destination order")
        })?;
    let into_folder = place == DropPlace::Into && anchor_is_folder;
    // Into a leaf item is meaningless → fall back to After it.
    let effective = match place {
        DropPlace::Into if !into_folder => DropPlace::After,
        other => other,
    };
    let base_indent = if into_folder {
        anchor_indent + 1
    } else {
        anchor_indent
    };
    let idx = match effective {
        DropPlace::Before => anchor_pos,
        DropPlace::After | DropPlace::Into => {
            subtree_end(dest_order, indent, anchor_pos, anchor_indent)
        }
    };
    let insertion_anchor = dest_order[idx..]
        .iter()
        .copied()
        .find(|id| !exclude.contains(id));
    Ok((base_indent, insertion_anchor))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indent_map(pairs: &[(EntityId, i64)]) -> HashMap<EntityId, i64> {
        pairs.iter().copied().collect()
    }

    #[test]
    fn subtree_of_grabs_root_plus_deeper_run() {
        // 1(0) 2(1) 3(2) 4(1) 5(0)
        let order = vec![1, 2, 3, 4, 5];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 2), (4, 1), (5, 0)]);
        assert_eq!(subtree_of(&order, &indent, 1), vec![1, 2, 3, 4]);
        assert_eq!(subtree_of(&order, &indent, 2), vec![2, 3]);
        assert_eq!(subtree_of(&order, &indent, 3), vec![3]);
        assert_eq!(subtree_of(&order, &indent, 5), vec![5]);
    }

    #[test]
    fn ancestors_of_walks_up_one_level_at_a_time() {
        // 1(0) 2(1) 3(2) 4(1) 5(0)
        let order = vec![1, 2, 3, 4, 5];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 2), (4, 1), (5, 0)]);
        assert_eq!(
            ancestors_of(&order, &indent, 2),
            vec![2, 1],
            "3's ancestors: 2 then 1"
        );
        assert_eq!(ancestors_of(&order, &indent, 1), vec![1], "2's ancestor: 1");
        assert_eq!(
            ancestors_of(&order, &indent, 0),
            Vec::<EntityId>::new(),
            "1 is root-level"
        );
        assert_eq!(
            ancestors_of(&order, &indent, 3),
            vec![1],
            "4 is back at 1's level"
        );
        assert_eq!(
            ancestors_of(&order, &indent, 4),
            Vec::<EntityId>::new(),
            "5 is root-level"
        );
    }

    #[test]
    fn ancestors_of_out_of_range_pos_is_empty() {
        let order = vec![1, 2];
        let indent = indent_map(&[(1, 0), (2, 0)]);
        assert!(ancestors_of(&order, &indent, 99).is_empty());
    }

    #[test]
    fn subtree_of_missing_root_is_empty() {
        let order = vec![1, 2];
        let indent = indent_map(&[(1, 0), (2, 0)]);
        assert!(subtree_of(&order, &indent, 99).is_empty());
    }

    #[test]
    fn subtree_end_is_exclusive_end() {
        let order = vec![1, 2, 3, 4, 5];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 2), (4, 1), (5, 0)]);
        assert_eq!(subtree_end(&order, &indent, 0, 0), 4); // subtree of 1 = [0..4)
        assert_eq!(subtree_end(&order, &indent, 1, 1), 3); // subtree of 2 = [1..3)
        assert_eq!(subtree_end(&order, &indent, 4, 0), 5); // last item
    }

    #[test]
    fn expand_dedups_nested_selection() {
        let order = vec![1, 2, 3, 4, 5];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 2), (4, 1), (5, 0)]);
        // selecting both the folder (1) and a descendant (3) yields 1's whole subtree once
        let requested: HashSet<EntityId> = [1, 3].into_iter().collect();
        assert_eq!(
            expand_to_subtrees(&order, &indent, &requested),
            vec![1, 2, 3, 4]
        );
        // two disjoint roots keep order
        let requested: HashSet<EntityId> = [2, 5].into_iter().collect();
        assert_eq!(
            expand_to_subtrees(&order, &indent, &requested),
            vec![2, 3, 5]
        );
    }

    #[test]
    fn insert_block_before_anchor_or_append() {
        assert_eq!(
            insert_block(&[1, 2, 3], &[8, 9], Some(2)),
            vec![1, 8, 9, 2, 3]
        );
        assert_eq!(insert_block(&[1, 2, 3], &[8, 9], None), vec![1, 2, 3, 8, 9]);
        assert_eq!(
            insert_block(&[1, 2, 3], &[8, 9], Some(99)),
            vec![1, 2, 3, 8, 9]
        );
    }

    #[test]
    fn anchor_for_binder_target_top_vs_bottom() {
        let order = vec![1, 2, 3];
        let exclude: HashSet<EntityId> = HashSet::new();
        assert_eq!(
            anchor_for_binder_target(&order, DropPlace::After, &exclude),
            None
        );
        assert_eq!(
            anchor_for_binder_target(&order, DropPlace::Into, &exclude),
            Some(1)
        );
        assert_eq!(
            anchor_for_binder_target(&order, DropPlace::Before, &exclude),
            Some(1)
        );
        // top item excluded (being moved) → anchor before the next survivor
        let exclude: HashSet<EntityId> = [1].into_iter().collect();
        assert_eq!(
            anchor_for_binder_target(&order, DropPlace::Into, &exclude),
            Some(2)
        );
    }

    #[test]
    fn resolve_item_target_into_folder_indents_and_appends_after_subtree() {
        // 1(0 folder) 2(1) 3(1) 4(0)
        let order = vec![1, 2, 3, 4];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 1), (4, 0)]);
        let exclude: HashSet<EntityId> = HashSet::new();
        // Into folder 1 → base indent 1, anchor after 1's subtree (before 4)
        let (base, anchor) =
            resolve_item_target(&order, &indent, 1, 0, true, DropPlace::Into, &exclude).unwrap();
        assert_eq!(base, 1);
        assert_eq!(anchor, Some(4));
    }

    #[test]
    fn resolve_item_target_after_leaf_uses_sibling_indent() {
        let order = vec![1, 2, 3, 4];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 1), (4, 0)]);
        let exclude: HashSet<EntityId> = HashSet::new();
        // After leaf 2 (indent 1) → base indent 1, anchor before 3
        let (base, anchor) =
            resolve_item_target(&order, &indent, 2, 1, false, DropPlace::After, &exclude).unwrap();
        assert_eq!(base, 1);
        assert_eq!(anchor, Some(3));
    }

    #[test]
    fn resolve_item_target_into_leaf_falls_back_to_after() {
        let order = vec![1, 2, 3];
        let indent = indent_map(&[(1, 0), (2, 0), (3, 0)]);
        let exclude: HashSet<EntityId> = HashSet::new();
        // Into a non-folder leaf 2 → same as After 2 (sibling indent), anchor before 3
        let (base, anchor) =
            resolve_item_target(&order, &indent, 2, 0, false, DropPlace::Into, &exclude).unwrap();
        assert_eq!(base, 0);
        assert_eq!(anchor, Some(3));
    }

    #[test]
    fn resolve_item_target_before_anchor() {
        let order = vec![1, 2, 3];
        let indent = indent_map(&[(1, 0), (2, 0), (3, 0)]);
        let exclude: HashSet<EntityId> = HashSet::new();
        let (base, anchor) =
            resolve_item_target(&order, &indent, 2, 0, false, DropPlace::Before, &exclude).unwrap();
        assert_eq!(base, 0);
        assert_eq!(anchor, Some(2));
    }

    #[test]
    fn resolve_item_target_unknown_anchor_errors() {
        let order = vec![1, 2, 3];
        let indent = indent_map(&[(1, 0), (2, 0), (3, 0)]);
        let exclude: HashSet<EntityId> = HashSet::new();
        assert!(
            resolve_item_target(&order, &indent, 99, 0, false, DropPlace::After, &exclude).is_err()
        );
    }
}
