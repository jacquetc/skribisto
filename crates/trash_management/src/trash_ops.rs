// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What "trashing" actually means, written once for the three use cases that do it.
//!
//! Hand-written, not generated — the same arrangement as `work_management::work_io`.
//! It exists because no use case may call another: `trash_binder`,
//! `trash_binder_items` and `trash_selection` all soft-delete rows and file
//! `TrashInfo` entries under a Work, and the rules for doing that correctly are
//! subtle enough that three copies of them would drift.
//!
//! The rules, in one place:
//!
//! * **`activated = !trashed`.** A trashed row keeps its position in the binder;
//!   only the flag moves. The trash bin is an *index* over the tree, not a
//!   destination things are moved to.
//! * **One `TrashInfo` per requested root**, never per row in the cascade —
//!   restoring is per-root, so a subtree that went in as one entry comes back as
//!   one entry.
//! * **The `Work` must be open, and must own what is being trashed.** Both are
//!   checked, separately, because they fail differently: an unopened `work_id`
//!   is a caller passing a stale id, while a binder belonging to *another* open
//!   Work is the multi-Work bug the checks exist for — it would trash Work B's
//!   rows while filing the `TrashInfo` under Work A's index, leaving Work B with
//!   vanished items and no trash-bin trace of them.
//!
//! ## Why three traits rather than one
//!
//! Each use case's unit of work carries only the `#[macros::uow_action]`s its own
//! logic needs, so they genuinely differ: `trash_binder_items` never touches a
//! `Binder` row, and only `trash_selection` resolves items back to their owning
//! binder. A single wide trait would force every unit of work to grow actions it
//! does not use. The helpers below therefore take the narrowest trait each needs,
//! and a use case implements only the ones it calls.
//!
//! The helpers are generic over `S: … + ?Sized` so a caller can pass its unit of
//! work directly: each use case implements these for *its own trait object*, and
//! Rust will not coerce `&dyn FooUnitOfWorkTrait` to `&dyn TrashIndex` — unsizing
//! between unrelated trait objects is not a coercion it performs.

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{Binder, BinderItem, TrashInfo};
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

/// Reading and writing the Work-level trash index.
pub(crate) trait TrashIndex {
    /// The ids of every currently open `Work`.
    fn open_works(&self) -> Result<Vec<EntityId>>;
    /// `work`'s own binders.
    fn work_binders(&self, work: EntityId) -> Result<Vec<EntityId>>;
    /// `work.trash_infos`, in order.
    fn trash_index(&self, work: EntityId) -> Result<Vec<EntityId>>;
    fn set_trash_index(&self, work: EntityId, ids: &[EntityId]) -> Result<()>;
    fn new_trash_info(&self, info: &TrashInfo) -> Result<EntityId>;
    fn link_trash_info(
        &self,
        info: EntityId,
        field: &TrashInfoRelationshipField,
        right: &[EntityId],
    ) -> Result<()>;
    fn drop_trash_infos(&self, ids: &[EntityId]) -> Result<()>;
}

/// Reading and writing `BinderItem` rows.
pub(crate) trait ItemStore {
    /// `binder`'s items in document order — the relationship vec, which is the
    /// only thing that carries order.
    fn binder_items(&self, binder: EntityId) -> Result<Vec<EntityId>>;
    fn items(&self, ids: &[EntityId]) -> Result<Vec<BinderItem>>;
    fn save_items(&self, items: &[BinderItem]) -> Result<()>;
}

/// Reading and writing a `Binder` row itself.
pub(crate) trait BinderStore {
    fn binder(&self, id: EntityId) -> Result<Option<Binder>>;
    fn save_binder(&self, binder: &Binder) -> Result<()>;
}

/// Resolve `requested` to an **open** Work, or fail.
///
/// `get_work_relationship` does not validate that its id names a real, open
/// Work, so every caller must come through here first.
pub(crate) fn resolve_work<S: TrashIndex + ?Sized>(
    store: &S,
    requested: EntityId,
) -> Result<EntityId> {
    store
        .open_works()?
        .into_iter()
        .find(|&id| id == requested)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
}

/// Fail unless every id in `binders` is one of `work`'s own binders.
///
/// `Work.binders` is a strong one-to-many, so this forward lookup is exact.
/// Without it a caller pairing Work B's binder with Work A's `work_id` would
/// trash Work B's rows while the new `TrashInfo` landed under Work A's index —
/// no undo record on Work B's side, and nothing in its bin to restore.
pub(crate) fn assert_work_owns<S: TrashIndex + ?Sized>(
    store: &S,
    work: EntityId,
    binders: &[EntityId],
) -> Result<()> {
    if binders.is_empty() {
        return Ok(());
    }
    let owned: HashSet<EntityId> = store.work_binders(work)?.into_iter().collect();
    if let Some(stray) = binders.iter().find(|b| !owned.contains(b)) {
        return Err(anyhow!("binder {stray} does not belong to work {work}"));
    }
    Ok(())
}

/// Load `ids`, set `activated`, write them back. A no-op for an empty list.
pub(crate) fn set_activated<S: ItemStore + ?Sized>(
    store: &S,
    ids: &[EntityId],
    value: bool,
) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut items = store.items(ids)?;
    if items.is_empty() {
        return Ok(());
    }
    for item in &mut items {
        item.activated = value;
    }
    store.save_items(&items)
}

/// Set `activated` on a whole `Binder` row. Silently does nothing if it is gone
/// — a revert must not fail over a row that cannot come back.
pub(crate) fn set_binder_activated<S: BinderStore + ?Sized>(
    store: &S,
    id: EntityId,
    value: bool,
) -> Result<()> {
    if let Some(mut binder) = store.binder(id)? {
        binder.activated = value;
        store.save_binder(&binder)?;
    }
    Ok(())
}

/// File one `TrashInfo` per root, appending them to `work`'s index.
///
/// `origin_of` gives each root the binder it should be restored into — a whole
/// binder has no parent binder and passes `0`. Returns the created ids so the
/// inverse can remove exactly these.
pub(crate) fn file_trash_infos<S: TrashIndex + ?Sized>(
    store: &S,
    work: EntityId,
    roots: &[EntityId],
    field: &TrashInfoRelationshipField,
    origin_of: &dyn Fn(EntityId) -> i64,
    at: DateTime<Utc>,
) -> Result<Vec<EntityId>> {
    if roots.is_empty() {
        return Ok(Vec::new());
    }
    let mut index = store.trash_index(work)?;
    let mut created = Vec::with_capacity(roots.len());
    for &root in roots {
        let info = store.new_trash_info(&TrashInfo {
            created_at: at,
            updated_at: at,
            trashed_at: at,
            origin_binder_id: origin_of(root),
            ..Default::default()
        })?;
        store.link_trash_info(info, field, &[root])?;
        index.push(info);
        created.push(info);
    }
    store.set_trash_index(work, &index)?;
    Ok(created)
}

/// Remove `created` from `work`'s index and delete the rows — the inverse of
/// [`file_trash_infos`].
pub(crate) fn unfile_trash_infos<S: TrashIndex + ?Sized>(
    store: &S,
    work: EntityId,
    created: &[EntityId],
) -> Result<()> {
    if created.is_empty() {
        return Ok(());
    }
    let drop: HashSet<EntityId> = created.iter().copied().collect();
    let remaining: Vec<EntityId> = store
        .trash_index(work)?
        .into_iter()
        .filter(|id| !drop.contains(id))
        .collect();
    store.set_trash_index(work, &remaining)?;
    store.drop_trash_infos(created)
}

/// Compute the requested *roots* (items not nested under another requested
/// item) and the full *cascade* (each root plus its contiguous subtree), in
/// binder order.
///
/// Pure. A binder has no parent/child graph — depth is the `indent` integer —
/// so a subtree is a run of following rows at a strictly greater indent, and
/// nesting has to be detected rather than looked up.
pub(crate) fn roots_and_cascade(
    order: &[EntityId],
    indent: &HashMap<EntityId, i64>,
    requested: &HashSet<EntityId>,
) -> (Vec<EntityId>, Vec<EntityId>) {
    let mut roots = Vec::new();
    let mut cascade = Vec::new();
    let mut covered: HashSet<EntityId> = HashSet::new();
    let mut i = 0usize;
    while i < order.len() {
        let id = order[i];
        if requested.contains(&id) && !covered.contains(&id) {
            roots.push(id);
            let root_indent = *indent.get(&id).unwrap_or(&0);
            let mut j = i;
            loop {
                cascade.push(order[j]);
                covered.insert(order[j]);
                j += 1;
                if j >= order.len() || *indent.get(&order[j]).unwrap_or(&0) <= root_indent {
                    break;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
    (roots, cascade)
}

/// `{id -> indent}` for a binder's items, the input [`roots_and_cascade`] needs
/// alongside the order.
pub(crate) fn indents<S: ItemStore + ?Sized>(
    store: &S,
    ids: &[EntityId],
) -> Result<HashMap<EntityId, i64>> {
    Ok(store
        .items(ids)?
        .into_iter()
        .map(|it| (it.id, it.indent))
        .collect())
}
