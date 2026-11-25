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
//! A row is also created by the editor, on its first flush, for a role the item
//! had no row of yet (a scratchpad, an epigraph, a synopsis nobody had typed).
//! Such a row is in neither snapshot, so the restore removes it with everything
//! else it did not see, and the words typed into it were lost. A row the command
//! itself created is in its "after" snapshot, so a row in neither is one written
//! since: it goes back after the restore, onto the same item, as long as that
//! item still exists. When the restore gave the item a row of that role again
//! (the command made one: a merge folded the other scene's scratchpad into it),
//! the text typed since is joined onto the end of that row instead, as the merge
//! itself joins two texts ([`join_text`](crate::prose_keep::join_text)): one row per role,
//! and nothing dropped.
//!
//! That join must not trap the text either. The row the command made is in one
//! snapshot only, so the next undo removes it, and with it the words joined onto it,
//! which were then in no snapshot at all. So a row the restore is about to remove,
//! that still begins with what the command wrote there, gives back what follows:
//! that text was typed since, and goes back onto the same item under the same id.
//! The redo after it joins that text onto the row it brings back, so the two keep
//! taking turns without losing a word. A row the writer rewrote inside what the
//! command wrote is that command's rewrite, and goes back to the snapshot like any
//! other.
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
    /// Rows created since the command, each with the item that owned it: in neither
    /// snapshot, so the restore may remove them.
    created: Vec<(EntityId, Content)>,
}

impl KeptProse {
    /// How many rows will be put back.
    pub fn len(&self) -> usize {
        self.rows.len() + self.created.len()
    }

    /// Whether nothing was written since the command, so the restore loses nothing.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.created.is_empty()
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
/// `target` now), and every row created since (in neither snapshot) with the
/// item that owns it.
///
/// And the text typed since onto a row in one snapshot only, which a bare restore
/// would drop with the row or reset under it:
///
/// - in `other` only (the restore removes it): what follows the text it holds in
///   `other`, put back as a row created since, under the same id;
/// - in `target` only and live (it was given back that way by the restore before
///   this one): its text, joined after the text `target` brings back.
pub fn keep(
    transaction: Option<&Transaction>,
    target: &EntityTreeSnapshot,
    other: &EntityTreeSnapshot,
) -> Result<KeptProse> {
    let store = open(transaction)?.get_store();
    let target = &snapshot_store(target)?.contents;
    let other = &snapshot_store(other)?.contents;
    let live = read_or_recover(&store.contents);
    let owners = read_or_recover(&store.jn_content_from_binder_item_contents);
    let mut rows = Vec::new();
    let mut created = Vec::new();
    for (id, row) in live.iter() {
        match (target.get(id), other.get(id)) {
            (Some(t), Some(o)) => {
                if t.data == o.data && row.data != t.data {
                    rows.push(row.clone());
                }
            }
            (None, None) => {
                if let Some(owner) = owners.sources_of(id).first() {
                    created.push((*owner, row.clone()));
                }
            }
            (None, Some(o)) => {
                if let Some(owner) = owners.sources_of(id).first()
                    && let Some(since) = typed_after(&o.data, &row.data)
                {
                    let mut back = row.clone();
                    back.data = since;
                    created.push((*owner, back));
                }
            }
            (Some(t), None) => {
                if row.data != t.data && !row.data.trim().is_empty() {
                    let mut back = row.clone();
                    back.data = join_text(&t.data, &row.data);
                    rows.push(back);
                }
            }
        }
    }
    Ok(KeptProse { rows, created })
}

/// Write the kept prose back over what the restore put there, and say so.
///
/// Only the prose moves (`data`, and `updated_at` with it); the row's role,
/// activation and links stay as the restore left them, since those are what the
/// command changed. A row the restore removed stays removed, except a row created
/// since the command: that one goes back, under its own id, at the end of its
/// item's contents, when the item is still there. If the item holds a row of its
/// role again, its text is joined onto the end of that row instead.
pub fn put_back(
    transaction: Option<&Transaction>,
    events: &RefCell<EventBuffer>,
    kept: KeptProse,
) -> Result<()> {
    let store = open(transaction)?.get_store();
    if kept.is_empty() {
        return Ok(());
    }
    put_back_created(store, events, kept.created);
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

/// Put back the rows created since the command that the restore removed.
///
/// A row whose item holds a row of its role again after the restore is not put back
/// beside it, which would make two rows of one role: its text is joined onto the end
/// of that row ([`join_text`]), which is then reported updated. A merge's redo is the
/// case: undo took the merged scratchpad away, the writer started a new one on the
/// surviving scene, and the redo brings the merged one back.
fn put_back_created(
    store: &crate::database::hashmap_store::HashMapStore,
    events: &RefCell<EventBuffer>,
    created: Vec<(EntityId, Content)>,
) {
    if created.is_empty() {
        return;
    }
    let mut recreated: Vec<EntityId> = Vec::new();
    let mut joined: Vec<EntityId> = Vec::new();
    let mut owners_touched: Vec<EntityId> = Vec::new();
    {
        let items = read_or_recover(&store.binder_items);
        let mut live = write_or_recover(&store.contents);
        let mut owners = write_or_recover(&store.jn_content_from_binder_item_contents);
        for (owner, row) in created {
            if live.contains_key(&row.id) || !items.contains_key(&owner) {
                continue; // the restore kept it, or its item is gone with the undo
            }
            if !owners.sources_of(&row.id).is_empty() {
                continue; // somehow already linked elsewhere: leave the restore's word
            }
            let mut list = owners.get(&owner).cloned().unwrap_or_default();
            let taken = list
                .iter()
                .copied()
                .find(|cid| live.get(cid).is_some_and(|c| c.role == row.role));
            if let Some(existing) = taken {
                // One row per role: the text typed since goes onto the end of the row
                // the restore brought back, never into a second row behind it.
                if row.data.trim().is_empty() {
                    continue;
                }
                if let Some(current) = live.get(&existing) {
                    let mut back = current.clone();
                    back.data = join_text(&current.data, &row.data);
                    back.updated_at = row.updated_at;
                    live.insert(existing, back);
                    if !joined.contains(&existing) {
                        joined.push(existing);
                    }
                }
                continue;
            }
            list.push(row.id);
            owners.insert(owner, list);
            recreated.push(row.id);
            if !owners_touched.contains(&owner) {
                owners_touched.push(owner);
            }
            live.insert(row.id, row);
        }
    }
    let mut buffer = events.borrow_mut();
    if !joined.is_empty() {
        buffer.push(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
            ids: joined,
            data: None,
        });
    }
    if recreated.is_empty() {
        return;
    }
    buffer.push(Event {
        origin: Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Created)),
        ids: recreated,
        data: None,
    });
    buffer.push(Event {
        origin: Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
        ids: owners_touched,
        data: None,
    });
}

/// What `now` holds after `made`, when `now` is `made` with text joined onto its end
/// as [`join_text`] joins it: the text typed onto a row since a command wrote `made`
/// into it. `None` when nothing was added, or when `now` no longer begins with
/// `made` (the writer changed what the command wrote, so the two cannot be told
/// apart).
fn typed_after(made: &str, now: &str) -> Option<String> {
    let line_break = |c: char| matches!(c, '\n' | '\r');
    let since = if made.trim().is_empty() {
        now
    } else {
        now.strip_prefix(made.trim_end_matches(line_break))?
            .strip_prefix("\n\n")?
            .trim_start_matches(line_break)
    };
    (!since.trim().is_empty()).then(|| since.to_string())
}

/// Append `b` onto `a` with a blank-line separator (paragraph break in Djot): how a merge
/// joins two scenes' texts, and how a text typed since a command joins the row the
/// command's redo brings back.
///
/// Only the line breaks at the join are trimmed, since the separator replaces them. A merge
/// cuts no paragraph, so every paragraph keeps the blanks it opens and ends with, as a split
/// keeps a paragraph's indentation (`teksilo_ui`'s `split_djot`): the ones the editor writes
/// behind an empty `{}` never touch the join, and a no-break or ideographic space it writes
/// bare there is text. `str::trim` knew none of that and took those last two, so merging a
/// scene whose first paragraph opened with an ideographic-space indent lost the indent.
pub fn join_text(a: &str, b: &str) -> String {
    let line_break = |c: char| matches!(c, '\n' | '\r');
    let b = b.trim_start_matches(line_break);
    if a.trim().is_empty() {
        b.to_string()
    } else {
        format!("{}\n\n{b}", a.trim_end_matches(line_break))
    }
}
