// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What these primitives must be true of, for **every** tree and every legal drop.
//!
//! The unit tests in `lib.rs` draw six trees and check the answer on each. That is the right
//! way to say what a function means, and the wrong way to find out whether it holds: the
//! binder's hierarchy is not stored anywhere, it is *implied* by a flat list plus an indent
//! per row, so "still a tree" is an invariant over the whole list that no single example can
//! stand for. A relocated subtree that lands one indent too deep is not a crash and not a
//! failed assertion. It is a chapter that has quietly become a scene of the chapter above
//! it, in a file that opens and saves perfectly well.
//!
//! # Well-formed
//!
//! The binder model gives exactly three rules, and [`well_formed`] is them:
//!
//! * the first row sits at indent 0 — there is nothing for it to be a child of;
//! * no indent is negative;
//! * a row is at most one level deeper than the row above it. A jump of two would name a
//!   parent that does not exist.
//!
//! Everything else here is derived from those.
//!
//! # Why the whole move is replayed
//!
//! [`simulate_move`] performs, in order, exactly the steps
//! `binder_item_management::move_items` performs with these primitives: expand the selection
//! to whole subtrees, resolve the drop into a base indent and an insertion anchor, drop the
//! moved ids out of the source order, splice the block back in, then rebase each moved
//! subtree onto the base indent. The interesting invariant is a property of that *sequence*
//! and of nothing in it alone — each primitive can be individually correct while the
//! composition leaves a row parented to nothing — so testing them one at a time would not
//! reach it. That is not hypothetical: the first run of
//! `a_move_leaves_the_binder_well_formed` shrank to a two-subtree selection that the use case
//! shifted by one shared delta, landing rows at indent −2.
//!
//! The replay is deliberately a copy rather than a call into the use case: the use case
//! needs a store, a unit of work and an undo stack, none of which have anything to do with
//! the arithmetic. `binder_item_management/tests/move_properties.rs` drives the real thing
//! for the part a copy cannot check — that the use case still performs these steps, and in
//! this order.

use std::collections::{HashMap, HashSet};

use binder_ordering::{
    DropPlace, anchor_for_binder_target, expand_to_subtrees, insert_block, rebase_indents,
    resolve_item_target, subtree_end, subtree_of,
};
use common::types::EntityId;
use proptest::prelude::*;

/// A binder as this crate sees one: ordered rows, each with an indent.
type Rows = Vec<(EntityId, i64)>;

fn order_of(rows: &Rows) -> Vec<EntityId> {
    rows.iter().map(|(id, _)| *id).collect()
}

fn indent_of(rows: &Rows) -> HashMap<EntityId, i64> {
    rows.iter().copied().collect()
}

/// The three rules of the binder model, as one predicate.
fn well_formed(rows: &Rows) -> Result<(), String> {
    let mut prev: Option<i64> = None;
    for (i, (id, indent)) in rows.iter().enumerate() {
        if *indent < 0 {
            return Err(format!("row {i} (id {id}) has negative indent {indent}"));
        }
        match prev {
            None if *indent != 0 => {
                return Err(format!("first row (id {id}) is at indent {indent}, not 0"));
            }
            Some(p) if *indent > p + 1 => {
                return Err(format!(
                    "row {i} (id {id}) jumps from indent {p} to {indent}, naming a parent that \
                     does not exist"
                ));
            }
            _ => {}
        }
        prev = Some(*indent);
    }
    Ok(())
}

/// Well-formed binders of 1..=`max` rows, ids `1..=n` in order.
///
/// Generated as a walk rather than filtered into shape: each row picks a depth and is then
/// clamped to at most one deeper than the row above, so every draw is legal by construction
/// and proptest never wastes a case on a rejected one. The small depth range is what makes
/// the shapes interesting — a range of 0..3 over ten rows produces nesting, siblings,
/// several roots and abrupt returns to the top, which is where the boundaries are.
fn rows_strategy(max: usize) -> impl Strategy<Value = Rows> {
    prop::collection::vec(0i64..4, 1..=max).prop_map(|depths| {
        let mut rows: Rows = Vec::with_capacity(depths.len());
        let mut prev = -1i64;
        for (i, wanted) in depths.into_iter().enumerate() {
            let indent = wanted.min(prev + 1).max(0);
            rows.push((i as EntityId + 1, indent));
            prev = indent;
        }
        rows
    })
}

/// A binder, a non-empty selection of its rows, a target row outside that selection, and a
/// drop place — i.e. everything one call to `move_items` takes, already known to be legal.
///
/// The selection is drawn as a subset of the real ids so it is never empty and never names a
/// row that does not exist; the target is drawn from what the *expanded* selection leaves
/// over, because a drop onto one's own descendant is refused by the use case and so is not
/// part of the contract being checked here.
fn move_strategy() -> impl Strategy<Value = (Rows, HashSet<EntityId>, EntityId, bool, DropPlace)> {
    rows_strategy(12)
        .prop_flat_map(|rows| {
            let n = rows.len();
            (Just(rows), prop::collection::vec(any::<bool>(), n))
        })
        .prop_filter_map(
            "selection must leave a target outside its subtrees",
            |(rows, picks)| {
                let order = order_of(&rows);
                let indent = indent_of(&rows);
                let requested: HashSet<EntityId> = order
                    .iter()
                    .zip(picks)
                    .filter(|(_, pick)| *pick)
                    .map(|(id, _)| *id)
                    .collect();
                if requested.is_empty() {
                    return None;
                }
                let moved: HashSet<EntityId> = expand_to_subtrees(&order, &indent, &requested)
                    .into_iter()
                    .collect();
                let targets: Vec<EntityId> = order
                    .iter()
                    .copied()
                    .filter(|id| !moved.contains(id))
                    .collect();
                if targets.is_empty() {
                    return None;
                }
                Some((rows, requested, targets))
            },
        )
        .prop_flat_map(|(rows, requested, targets)| {
            let n = targets.len();
            (
                Just(rows),
                Just(requested),
                (0..n).prop_map(move |i| targets[i]),
                any::<bool>(),
                prop_oneof![
                    Just(DropPlace::Before),
                    Just(DropPlace::After),
                    Just(DropPlace::Into)
                ],
            )
        })
}

/// Replay `move_items`' arithmetic, step for step, inside one binder.
///
/// Returns `None` for the inputs the use case itself refuses, so a refusal is never mistaken
/// for a malformed result.
fn simulate_move(
    rows: &Rows,
    requested: &HashSet<EntityId>,
    target: EntityId,
    target_is_folder: bool,
    place: DropPlace,
) -> Option<Rows> {
    let order = order_of(rows);
    let indent = indent_of(rows);

    let full = expand_to_subtrees(&order, &indent, requested);
    if full.is_empty() {
        return None;
    }
    let move_set: HashSet<EntityId> = full.iter().copied().collect();
    if move_set.contains(&target) {
        return None; // "cannot move a subtree into itself"
    }
    let (base_indent, anchor) = resolve_item_target(
        &order,
        &indent,
        target,
        *indent.get(&target)?,
        target_is_folder,
        place,
        &move_set,
    )
    .ok()?;

    let filtered: Vec<EntityId> = order
        .iter()
        .copied()
        .filter(|id| !move_set.contains(id))
        .collect();
    let final_order = insert_block(&filtered, &full, anchor);

    let mut final_indent = indent.clone();
    for (id, depth) in rebase_indents(&order, &indent, requested, base_indent) {
        final_indent.insert(id, depth);
    }
    Some(
        final_order
            .into_iter()
            .map(|id| {
                let i = final_indent.get(&id).copied().unwrap_or(0);
                (id, i)
            })
            .collect(),
    )
}

proptest! {
    /// **The one that matters.** Relocating a subtree leaves a binder that still describes a
    /// tree.
    ///
    /// `move_items` applies its indent shift as a bare `it.indent += delta`, with no clamp
    /// and no check on the result, so nothing downstream of it would notice a row left at an
    /// impossible depth. Everything that reads the binder afterwards — the numbering pass,
    /// the exporter's scope resolution, the exploded folder's slug ancestry, the stream
    /// views — takes "a row is a child of the nearest shallower row above it" as given.
    #[test]
    fn a_move_leaves_the_binder_well_formed(
        (rows, requested, target, folder, place) in move_strategy()
    ) {
        let Some(after) = simulate_move(&rows, &requested, target, folder, place) else {
            return Ok(());
        };
        if let Err(why) = well_formed(&after) {
            prop_assert!(
                false,
                "moving {requested:?} {place:?} {target} (folder={folder})\n  before: {rows:?}\n  \
                 after:  {after:?}\n  {why}"
            );
        }
    }

    /// A move relocates rows; it never creates, drops or duplicates one.
    ///
    /// The cheapest possible statement of "my chapter is still there", and the one a writer
    /// would notice first. Worth its own property rather than folding it into the one above,
    /// because losing a row and misplacing a row are different bugs with different causes,
    /// and a combined assertion would report whichever it happened to check first.
    #[test]
    fn a_move_is_a_permutation(
        (rows, requested, target, folder, place) in move_strategy()
    ) {
        let Some(after) = simulate_move(&rows, &requested, target, folder, place) else {
            return Ok(());
        };
        let mut before_ids = order_of(&rows);
        let mut after_ids = order_of(&after);
        before_ids.sort_unstable();
        after_ids.sort_unstable();
        prop_assert_eq!(before_ids, after_ids, "row set changed\n  after: {:?}", after);
    }

    /// The moved block stays in one piece, in its own order.
    ///
    /// A subtree is only a subtree because it is contiguous: the model has no parent link, so
    /// a row separated from its former root by one surviving row has silently been reparented
    /// to whatever now precedes it.
    #[test]
    fn the_moved_block_stays_contiguous_and_ordered(
        (rows, requested, target, folder, place) in move_strategy()
    ) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        let full = expand_to_subtrees(&order, &indent, &requested);
        let Some(after) = simulate_move(&rows, &requested, target, folder, place) else {
            return Ok(());
        };
        let after_ids = order_of(&after);
        let Some(at) = after_ids.iter().position(|id| *id == full[0]) else {
            prop_assert!(false, "the moved root vanished");
            return Ok(());
        };
        prop_assert_eq!(
            &after_ids[at..at + full.len()],
            &full[..],
            "the block was broken up\n  after: {:?}",
            after_ids
        );
    }

    /// The block keeps its internal shape: every row's depth below the block's root is
    /// exactly what it was, whatever the block's own depth became.
    ///
    /// This is what "preserving its internal relative shape" in `move_items` means, stated so
    /// it can fail. A uniform `+= delta` is the correct implementation of it, and also the
    /// implementation that silently does the wrong thing if the delta is ever computed from
    /// the wrong row — the block's first row is its root only because `expand_to_subtrees`
    /// returns it first.
    #[test]
    fn a_move_preserves_the_blocks_internal_shape(
        (rows, requested, target, folder, place) in move_strategy()
    ) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        let full = expand_to_subtrees(&order, &indent, &requested);
        let Some(after) = simulate_move(&rows, &requested, target, folder, place) else {
            return Ok(());
        };
        let after_indent = indent_of(&after);
        // Measure each subtree against its own root. The roots are recovered the way the
        // expansion finds them — a requested row not already covered by an earlier one —
        // rather than by reading depths back out of the block, which cannot distinguish a
        // second subtree's root from a descendant of the first.
        let mut covered: HashSet<EntityId> = HashSet::new();
        for id in &order {
            if !requested.contains(id) || covered.contains(id) {
                continue;
            }
            let sub = subtree_of(&order, &indent, *id);
            let root_before = indent[id];
            let root_after = after_indent[id];
            for row in &sub {
                covered.insert(*row);
                prop_assert_eq!(
                    after_indent[row] - root_after,
                    indent[row] - root_before,
                    "row {} changed its depth within its subtree",
                    row
                );
            }
        }
        prop_assert_eq!(covered.len(), full.len(), "the expansion and the subtrees disagree");
    }

    /// Rows that were not moved keep the order they were in.
    ///
    /// A move is allowed to take rows out and put them back somewhere else. It is not allowed
    /// to disturb anything it was not asked about — which `insert_block` gets right only
    /// because it rebuilds the list in one pass.
    #[test]
    fn a_move_leaves_the_untouched_rows_in_their_order(
        (rows, requested, target, folder, place) in move_strategy()
    ) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        let moved: HashSet<EntityId> = expand_to_subtrees(&order, &indent, &requested)
            .into_iter()
            .collect();
        let Some(after) = simulate_move(&rows, &requested, target, folder, place) else {
            return Ok(());
        };
        let before_rest: Vec<EntityId> =
            order.iter().copied().filter(|id| !moved.contains(id)).collect();
        let after_rest: Vec<EntityId> = order_of(&after)
            .into_iter()
            .filter(|id| !moved.contains(id))
            .collect();
        prop_assert_eq!(before_rest, after_rest);
    }

    /// A subtree is the root plus a contiguous run of strictly deeper rows, and it stops at
    /// the first row that is not.
    ///
    /// The definition the whole crate rests on, checked against every row of every tree
    /// rather than the four roots `subtree_of_grabs_root_plus_deeper_run` picks out.
    #[test]
    fn a_subtree_is_the_contiguous_deeper_run(rows in rows_strategy(14)) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        for (pos, (root, root_indent)) in rows.iter().enumerate() {
            let sub = subtree_of(&order, &indent, *root);
            prop_assert_eq!(sub.first(), Some(root), "a subtree starts at its root");
            prop_assert_eq!(
                &sub[..],
                &order[pos..pos + sub.len()],
                "a subtree is a contiguous run of the order"
            );
            for id in &sub[1..] {
                prop_assert!(indent[id] > *root_indent, "row {} is not below the root", id);
            }
            if let Some(next) = order.get(pos + sub.len()) {
                prop_assert!(
                    indent[next] <= *root_indent,
                    "the run stopped early: row {next} is still below the root"
                );
            }
            // The two entry points must agree; `move_items` uses one and `subtree_of` the
            // other, so a disagreement would be a move that relocates a different set of
            // rows than the one the caller resolved.
            prop_assert_eq!(
                subtree_end(&order, &indent, pos, *root_indent),
                pos + sub.len(),
                "subtree_end and subtree_of disagree at row {}",
                root
            );
        }
    }

    /// Expanding a selection yields whole subtrees, in binder order, each row once.
    ///
    /// Three claims in one because they are one operation: it is a closure (every descendant
    /// of a chosen row comes too), a subsequence (binder order is preserved), and a set (a
    /// row chosen twice over — directly and as someone's descendant — appears once). The
    /// third is what `expand_dedups_nested_selection` checks on one tree; a duplicate id
    /// here becomes a row written twice by `update_binder_item_multi`.
    #[test]
    fn expanding_a_selection_is_an_ordered_subtree_closure(
        (rows, picks) in rows_strategy(12)
            .prop_flat_map(|r| { let n = r.len(); (Just(r), prop::collection::vec(any::<bool>(), n)) })
    ) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        let requested: HashSet<EntityId> = order
            .iter()
            .zip(picks)
            .filter(|(_, p)| *p)
            .map(|(id, _)| *id)
            .collect();
        let full = expand_to_subtrees(&order, &indent, &requested);

        let seen: HashSet<EntityId> = full.iter().copied().collect();
        prop_assert_eq!(seen.len(), full.len(), "a row was expanded twice: {:?}", full);

        for id in &requested {
            prop_assert!(seen.contains(id), "requested row {id} was dropped");
        }
        for id in &full {
            for descendant in subtree_of(&order, &indent, *id) {
                prop_assert!(
                    seen.contains(&descendant),
                    "row {descendant} is below {id} but was left behind"
                );
            }
        }

        let positions: Vec<usize> = full
            .iter()
            .filter_map(|id| order.iter().position(|o| o == id))
            .collect();
        prop_assert!(
            positions.windows(2).all(|w| w[0] < w[1]),
            "the expansion is out of binder order: {:?}",
            full
        );
    }

    /// Splicing a block in loses nothing and reorders nothing.
    ///
    /// `insert_block` is the one primitive that touches every row of the list, so it is the
    /// one with the most room to drop one. When an anchor is present the block goes
    /// immediately before it; when it is absent — which is how "drop at the very bottom" is
    /// spelled — the block goes last.
    #[test]
    fn splicing_a_block_preserves_both_sides(
        (base, block, anchor) in (
            prop::collection::vec(1u64..30, 0..10),
            prop::collection::vec(100u64..130, 0..6),
            prop::option::of(1u64..30),
        )
    ) {
        let base: Vec<EntityId> = { let mut b = base; b.sort_unstable(); b.dedup(); b };
        let block: Vec<EntityId> = { let mut b = block; b.sort_unstable(); b.dedup(); b };
        let out = insert_block(&base, &block, anchor);

        prop_assert_eq!(out.len(), base.len() + block.len(), "rows appeared or vanished");
        let kept: Vec<EntityId> = out.iter().copied().filter(|id| base.contains(id)).collect();
        prop_assert_eq!(kept, base.clone(), "the base order was disturbed");
        let spliced: Vec<EntityId> = out.iter().copied().filter(|id| block.contains(id)).collect();
        prop_assert_eq!(spliced, block.clone(), "the block order was disturbed");

        if block.is_empty() {
            return Ok(());
        }
        let at = out.iter().position(|id| *id == block[0]).unwrap_or(usize::MAX);
        prop_assert_eq!(&out[at..at + block.len()], &block[..], "the block was broken up");
        match anchor.filter(|a| base.contains(a)) {
            Some(a) => prop_assert_eq!(out.get(at + block.len()), Some(&a), "not placed before the anchor"),
            None => prop_assert_eq!(at + block.len(), out.len(), "no anchor means last"),
        }
    }

    /// Dropping onto a binder rather than onto a row lands at one end or the other, and never
    /// on a row that is itself on its way out.
    ///
    /// `Before` and `Into` mean the top, `After` means the bottom. The skip matters: the rows
    /// being relocated are still in the destination order at the moment the anchor is chosen,
    /// so anchoring on one of them would splice the block in front of itself.
    #[test]
    fn a_binder_drop_lands_at_an_end_and_never_on_a_departing_row(
        (order, picks, place) in (
            prop::collection::vec(1u64..20, 0..10),
            prop::collection::vec(any::<bool>(), 10),
            prop_oneof![Just(DropPlace::Before), Just(DropPlace::After), Just(DropPlace::Into)],
        )
    ) {
        let order: Vec<EntityId> = { let mut o = order; o.sort_unstable(); o.dedup(); o };
        let exclude: HashSet<EntityId> = order
            .iter()
            .zip(picks.iter().chain(std::iter::repeat(&false)))
            .filter(|(_, p)| **p)
            .map(|(id, _)| *id)
            .collect();
        let anchor = anchor_for_binder_target(&order, place, &exclude);

        if let Some(a) = anchor {
            prop_assert!(!exclude.contains(&a), "anchored on a row being moved");
            prop_assert!(order.contains(&a), "anchored on a row not in the binder");
        }
        match place {
            DropPlace::After => prop_assert_eq!(anchor, None, "After means the bottom"),
            DropPlace::Before | DropPlace::Into => {
                let first_survivor = order.iter().copied().find(|id| !exclude.contains(id));
                prop_assert_eq!(anchor, first_survivor, "Before/Into mean the top");
            }
        }
    }

    /// Resolving a drop onto a row answers with a reachable anchor and a non-negative depth.
    ///
    /// The two halves of the answer are used in different places — the anchor decides where
    /// the block is spliced, the base indent decides how deep it lands — so each needs to be
    /// usable on its own. `Into` a row that is not a folder is defined to mean `After` it,
    /// which is the rule that keeps a drop onto a scene from inventing a level.
    #[test]
    fn resolving_a_row_drop_answers_with_a_reachable_anchor(
        (rows, requested, target, folder, place) in move_strategy()
    ) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        let moved: HashSet<EntityId> = expand_to_subtrees(&order, &indent, &requested)
            .into_iter()
            .collect();
        if moved.contains(&target) {
            return Ok(());
        }
        let Ok((base, anchor)) = resolve_item_target(
            &order, &indent, target, indent[&target], folder, place, &moved,
        ) else {
            prop_assert!(false, "a drop onto a row that is present must resolve");
            return Ok(());
        };

        prop_assert!(base >= 0, "a negative base indent is not a depth");
        if let Some(a) = anchor {
            prop_assert!(order.contains(&a), "anchored outside the binder");
            prop_assert!(!moved.contains(&a), "anchored on a row being moved");
        }
        let expected_base = match place {
            DropPlace::Into if folder => indent[&target] + 1,
            _ => indent[&target],
        };
        prop_assert_eq!(base, expected_base, "the base indent does not follow the drop place");
    }

    /// A drop onto a row that is not in the binder is an error, not a guess.
    ///
    /// The only failure mode this function has, and the one place it must not fall back to
    /// something plausible: an anchor that cannot be found means the caller resolved the
    /// target against a different binder, and appending "somewhere sensible" would scatter a
    /// chapter into a project it does not belong to.
    #[test]
    fn a_drop_onto_an_absent_row_is_refused(
        (rows, place) in (
            rows_strategy(8),
            prop_oneof![Just(DropPlace::Before), Just(DropPlace::After), Just(DropPlace::Into)],
        )
    ) {
        let order = order_of(&rows);
        let indent = indent_of(&rows);
        let absent = order.iter().copied().max().unwrap_or(0) + 1;
        prop_assert!(
            resolve_item_target(&order, &indent, absent, 0, false, place, &HashSet::new()).is_err()
        );
    }
}
