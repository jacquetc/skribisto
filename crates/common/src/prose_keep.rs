// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Undoing a structural command keeps the prose written since it.
//!
//! A command that moves, duplicates, trashes or retypes rows records its undo as
//! two scoped snapshots, and undo or redo restores one of them. The restore
//! reverts every row in scope wholesale, `Content` included. But the prose in a
//! `Content` row is not on the project's undo stack: an open editor writes it
//! through [`UNTRACKED_STACK_ID`](crate::undo_redo::UNTRACKED_STACK_ID), because
//! the editor's own history owns the typing. So between a command and its undo,
//! the writer may have typed into a row the command never touched, and a bare
//! restore puts that row back as it was when the command ran. Undoing a binder
//! move erased the words typed into the moved chapter afterwards, and the open
//! editor was reloaded over them.
//!
//! The rule here: a `Content` row whose prose is the same in both snapshots was
//! not changed by the command, so its undo and its redo leave the prose as it
//! is now. A row the command *did* rewrite (a merge, a split, Replace All, a
//! document import) still goes back to the snapshot: that rewrite is what the
//! undo undoes.
//!
//! Use it around the restore, inside the same write transaction:
//!
//! ```ignore
//! let kept = uow.keep_prose(target, other)?;
//! uow.restore_binder(target)?;
//! uow.put_back_prose(kept)?;
//! ```
//!
//! where `target` is the snapshot being restored and `other` the command's
//! opposite end (the "after" snapshot when undoing, the "before" when redoing).

use crate::database::hashmap_store::{HashMapStoreSnapshot, read_or_recover, write_or_recover};
use crate::database::transactions::Transaction;
use crate::entities::Content;
use crate::event::{DirectAccessEntity, EntityEvent, Event, EventBuffer, Origin};
use crate::snapshot::EntityTreeSnapshot;
use crate::types::EntityId;
use anyhow::{Result, anyhow};
use std::cell::RefCell;

/// The prose of the rows a restore must not revert, read before the restore.
#[derive(Debug, Default)]
#[must_use = "a kept prose does nothing until it is put back after the restore"]
pub struct KeptProse {
    rows: Vec<Content>,
}

impl KeptProse {
    /// How many rows will be put back.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether nothing was written since the command, so the restore loses nothing.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

fn open(transaction: Option<&Transaction>) -> Result<&Transaction> {
    transaction.ok_or_else(|| anyhow!("prose_keep: no active transaction"))
}

fn snapshot_store(snap: &EntityTreeSnapshot) -> Result<&HashMapStoreSnapshot> {
    snap.store_snapshot
        .as_ref()
        .ok_or_else(|| anyhow!("prose_keep: snapshot holds no store"))?
        .downcast_ref::<HashMapStoreSnapshot>()
        .ok_or_else(|| anyhow!("prose_keep: snapshot type mismatch"))
}

/// Read, from the live store, the prose a restore to `target` must not revert:
/// every `Content` row whose prose the command left unchanged (`target` and
/// `other` agree on it) and that has been written since (it differs from
/// `target` now).
pub fn keep(
    transaction: Option<&Transaction>,
    target: &EntityTreeSnapshot,
    other: &EntityTreeSnapshot,
) -> Result<KeptProse> {
    let store = open(transaction)?.get_store();
    let target = &snapshot_store(target)?.contents;
    let other = &snapshot_store(other)?.contents;
    let live = read_or_recover(&store.contents);
    let rows = live
        .iter()
        .filter(|&(id, row)| match (target.get(id), other.get(id)) {
            (Some(t), Some(o)) => t.data == o.data && row.data != t.data,
            _ => false,
        })
        .map(|(_, row)| row.clone())
        .collect();
    Ok(KeptProse { rows })
}

/// Write the kept prose back over what the restore put there, and say so.
///
/// Only the prose moves (`data`, and `updated_at` with it); the row's role,
/// activation and links stay as the restore left them, since those are what the
/// command changed. A row the restore removed stays removed.
pub fn put_back(
    transaction: Option<&Transaction>,
    events: &RefCell<EventBuffer>,
    kept: KeptProse,
) -> Result<()> {
    let store = open(transaction)?.get_store();
    if kept.rows.is_empty() {
        return Ok(());
    }
    let mut written: Vec<EntityId> = Vec::new();
    {
        let mut live = write_or_recover(&store.contents);
        for row in kept.rows {
            if let Some(restored) = live.get(&row.id)
                && restored.data != row.data
            {
                let mut back = restored.clone();
                back.data = row.data;
                back.updated_at = row.updated_at;
                written.push(back.id);
                live.insert(back.id, back);
            }
        }
    }
    if !written.is_empty() {
        events.borrow_mut().push(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
            ids: written,
            data: None,
        });
    }
    Ok(())
}
