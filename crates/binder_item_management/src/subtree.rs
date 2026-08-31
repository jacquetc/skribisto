// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resolving "everything beneath this item", once, for the use cases that
//! write to a subtree.
//!
//! Hand-written, not generated. It exists because no use case may call another
//! (see the architecture note in `CLAUDE.md`): `set_descendants_exportable` and
//! `set_descendants_dict_language` are the same walk over two different
//! fields, so the walk lives here and each of them calls in.
//!
//! The walk is the whole reason those use cases exist at all. A binder holds no
//! parent/child graph -- it is a flat ordered list where depth is an `indent`
//! integer -- so "beneath X" means *X plus the following rows at a strictly
//! greater indent*, and computing it needs the binder's ordered item list and
//! every item's indent. A caller holding an item id and nothing else cannot do
//! it, which is why the gesture used to exist in exactly one view-model.

use anyhow::Result;
use common::types::EntityId;
use std::collections::HashMap;

/// The three reads a subtree walk needs.
///
/// A trait rather than three closures because each use case has its own
/// generated unit-of-work trait — they share no supertype, so this is the seam
/// that lets one walk serve all of them. The per-use-case impl is three
/// forwarding calls; the walk below is what is actually being shared.
pub(crate) trait BinderWalk {
    /// The binder that lists `item`, or `None` if no binder does.
    fn owning_binder(&self, item: EntityId) -> Result<Option<EntityId>>;
    /// `binder`'s items in document order — the relationship vec, never a raw
    /// entity scan, which carries no order at all.
    fn ordered_items(&self, binder: EntityId) -> Result<Vec<EntityId>>;
    /// `{id -> indent}` for `ids`. Ids with no row are simply absent.
    fn indents(&self, ids: &[EntityId]) -> Result<HashMap<EntityId, i64>>;
}

/// The ids strictly *below* `item_id` in its binder, in document order.
///
/// Empty for a leaf, and empty for an id no binder holds. Neither is an error:
/// "apply this to my children" addressed to something with no children is a
/// complete instruction that happens to require no writes, and answering it
/// with a failure would make the caller distinguish two cases it has no reason
/// to care about.
///
/// **Trashed rows are included.** `activated` gates what the manuscript
/// compiles, not what a flag may be written onto, and a row restored later
/// should carry whatever its neighbours were given while it was away.
/// Generic over `W`, and `?Sized`, so the caller can hand it its unit of work
/// directly: each use case implements [`BinderWalk`] *for its own trait object*
/// (`impl BinderWalk for dyn FooUnitOfWorkTrait`), and Rust will not coerce
/// `&dyn FooUnitOfWorkTrait` to `&dyn BinderWalk` — unsizing between unrelated
/// trait objects is not a coercion it performs. Resolving `W` to the concrete
/// `dyn FooUnitOfWorkTrait` sidesteps that entirely.
pub(crate) fn descendants_of<W: BinderWalk + ?Sized>(
    walk: &W,
    item_id: EntityId,
) -> Result<Vec<EntityId>> {
    let Some(binder) = walk.owning_binder(item_id)? else {
        return Ok(Vec::new());
    };
    let order = walk.ordered_items(binder)?;
    let indent = walk.indents(&order)?;

    // `subtree_of` yields the root first. The root is never ours: these are the
    // "apply to children" gestures, which sit *beside* an item's own control
    // and would swallow it if they included it.
    let mut subtree = binder_ordering::subtree_of(&order, &indent, item_id);
    if subtree.is_empty() {
        return Ok(Vec::new());
    }
    subtree.remove(0);
    Ok(subtree)
}
