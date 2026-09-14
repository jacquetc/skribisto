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

/// One pass over `order`, yielding every row of every requested subtree in binder order,
/// each flagged with whether it is the row that *opened* its subtree.
///
/// The single definition of "which rows move, and which of them is a root", so
/// [`expand_to_subtrees`] and [`rebase_indents`] cannot come to different answers. They must
/// not: one decides what is relocated and the other how deep each row lands, and a
/// disagreement is a row moved to a depth computed for a different row.
///
/// A root cannot be recovered from the result afterwards. Two disjoint subtrees can each be
/// deeper than the last, so "not deeper than the root currently open" recognises the second
/// root inside one subtree and misses it across two — which is exactly the mistake that
/// makes a selection of a chapter plus a far-away scene collapse into one block.
fn walk_subtrees(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    requested: &HashSet<EntityId>,
) -> Vec<(EntityId, bool)> {
    let mut full: Vec<(EntityId, bool)> = Vec::new();
    let mut seen: HashSet<EntityId> = HashSet::new();
    let mut i = 0usize;
    while i < order.len() {
        let id = order[i];
        if requested.contains(&id) && !seen.contains(&id) {
            let root_indent = *indent.get(&id).unwrap_or(&0);
            let mut j = i;
            loop {
                let cur = order[j];
                full.push((cur, j == i));
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

/// Expand a requested id set to full contiguous subtrees, in `order`'s order,
/// deduplicating nested selections (a descendant already covered by an earlier
/// requested ancestor's subtree is not repeated).
pub fn expand_to_subtrees(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    requested: &HashSet<EntityId>,
) -> Vec<EntityId> {
    walk_subtrees(order, indent, requested)
        .into_iter()
        .map(|(id, _)| id)
        .collect()
}

/// The indent each expanded row takes once the block is rebased so that its subtree roots sit
/// at `base_indent`.
///
/// **One rebase per subtree, never one delta for the whole block.** A selection is not a
/// subtree: a writer can pick a chapter and a scene from inside a different chapter, and the
/// expansion hands both back as one ordered list. Shifting that list by a single delta taken
/// from its first row moves every later subtree by an amount computed from somebody else's
/// depth — which produced rows at indent −2, a binder whose first row was not at the top
/// level, and chapters that had quietly become each other's scenes. Nothing contradicts it,
/// because the binder stores no parent link; the file is valid and describes a different book.
///
/// So each subtree is rebased on its own: its root lands at `base_indent`, and every row under
/// it keeps exactly the depth it had below that root. Dropping a chapter and a far-away scene
/// onto one folder makes both children of it, which is what the gesture says, and preserves
/// the accident of their former relative depths in neither.
///
/// `trash_management::restore_items_to` has always worked this way, one `root_old_indent` per
/// planned subtree. This is the same rule, for the caller that had a single one.
pub fn rebase_indents(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    requested: &HashSet<EntityId>,
    base_indent: i64,
) -> HashMap<EntityId, i64> {
    let mut out = HashMap::new();
    let mut root_indent = 0i64;
    for (id, is_root) in walk_subtrees(order, indent, requested) {
        let own = *indent.get(&id).unwrap_or(&0);
        if is_root {
            root_indent = own;
        }
        out.insert(id, base_indent + (own - root_indent));
    }
    out
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
    fn rebase_puts_every_selected_subtree_at_the_base_indent() {
        // 1(0) 2(0) 3(1) 4(2) 5(2) 6(1) 7(2) — the shape a property run shrank to.
        let order = vec![1, 2, 3, 4, 5, 6, 7];
        let indent = indent_map(&[(1, 0), (2, 0), (3, 1), (4, 2), (5, 2), (6, 1), (7, 2)]);
        // Two disjoint subtrees, at indent 0 and indent 2, dropped at the top level.
        let requested: HashSet<EntityId> = [1, 5].into_iter().collect();
        let out = rebase_indents(&order, &indent, &requested, 0);
        assert_eq!(out.get(&1), Some(&0));
        assert_eq!(
            out.get(&5),
            Some(&0),
            "the second subtree's root is rebased on its own, not carried along by the first"
        );
    }

    #[test]
    fn rebase_keeps_each_subtree_internal_shape() {
        // 1(0) 2(1) 3(2) 4(0) — selecting 1 takes 2 and 3 with it.
        let order = vec![1, 2, 3, 4];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 2), (4, 0)]);
        let requested: HashSet<EntityId> = [1].into_iter().collect();
        let out = rebase_indents(&order, &indent, &requested, 2);
        assert_eq!(out.get(&1), Some(&2));
        assert_eq!(out.get(&2), Some(&3));
        assert_eq!(out.get(&3), Some(&4));
        assert_eq!(out.get(&4), None, "an unselected row is not rebased");
    }

    #[test]
    fn rebase_never_returns_a_negative_indent_for_a_deeper_selection() {
        // The original fault: a delta taken from the first subtree's root (indent 2) applied
        // to a later root at indent 0 gave −2.
        let order = vec![1, 2, 3, 4];
        let indent = indent_map(&[(1, 0), (2, 1), (3, 2), (4, 0)]);
        let requested: HashSet<EntityId> = [3, 4].into_iter().collect();
        let out = rebase_indents(&order, &indent, &requested, 0);
        assert_eq!(out.get(&3), Some(&0));
        assert_eq!(out.get(&4), Some(&0));
        assert!(out.values().all(|d| *d >= 0));
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
