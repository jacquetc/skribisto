// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! [`OpenWork`](crate::open_work::OpenWork): one open project in this process, for state that lives exactly
//! as long as the project is open.
//!
//! ## Why not `Work.unique_id`
//!
//! `unique_id` names a project **wherever it lives**: Save As keeps it, a copy
//! made in a file manager keeps it, and a backup carries it by design (backup
//! retention, version history, the Timeline and the remembered desk all depend on
//! that). So two open Works can carry one `unique_id`: the moved project beside the
//! file Save As left behind, or a project beside one of its backups opened from the
//! Backups list.
//!
//! A map held in memory and keyed by `unique_id` then has **one slot for two
//! projects**. Opening the second copy loads its state over the first's, which
//! the first then saves as its own; closing either empties the slot both were
//! using. Every one of those failures is silent.
//!
//! An `OpenWork` cannot collide that way: it names one Work row in one store, and
//! the store never hands that row's id to another Work (its ids only ever count
//! up, undo included).
//!
//! ## Which key to use
//!
//! - **Anything held in memory for an open project** (an extension's slot, the
//!   "changed off-thread" generation, the arrival tally): `OpenWork`.
//! - **Anything written to disk or sent elsewhere** (a settings file, a record
//!   kept between sessions, a request to a service): `unique_id`. An `OpenWork` is
//!   meaningless outside the process, which is why it implements neither
//!   `Serialize` nor `Display`.
//!
//! ## The store half
//!
//! A Work's `EntityId` alone is not enough: every `DbContext` counts from 1, and a
//! test binary builds many of them side by side while the registries that hold
//! these keys are process-wide. So the key pairs the Work id with a serial number
//! for the store it lives in, issued here the first time a store is seen.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crate::database::db_context::DbContext;
use crate::database::hashmap_store::HashMapStore;
use crate::types::EntityId;

/// One open Work in this process. See the module docs for when to use it and
/// when to use `Work.unique_id` instead.
///
/// `Copy`, and cheap to compare and hash, so it can key any map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OpenWork {
    store: u64,
    work: EntityId,
}

impl OpenWork {
    /// The key for the Work `work_id` in `db`'s store.
    ///
    /// Two `DbContext`s that share a store (every clone of one does) give the
    /// same key for the same Work.
    pub fn of(db: &DbContext, work_id: EntityId) -> Self {
        Self {
            store: store_serial(db.get_store()),
            work: work_id,
        }
    }

    /// A key naming no Work at all, different from every other key this process
    /// has made or will make.
    ///
    /// For testing a per-project store in isolation, where there is no
    /// `DbContext` to ask. Nothing real ever carries one, so a store keyed by it
    /// cannot collide with a project, nor with a key another test made.
    pub fn detached() -> Self {
        Self {
            store: NEXT_SERIAL.fetch_add(1, Ordering::Relaxed),
            work: 0,
        }
    }

    /// The Work's id in its store, for reading the row back.
    pub fn work_id(self) -> EntityId {
        self.work
    }
}

/// Carry an [`OpenWork`] inside an event payload that crosses a channel within
/// this process, with `#[serde(with = "common::open_work::in_process")]`.
///
/// `OpenWork` does not implement `Serialize` so that it cannot end up in a file
/// by accident: written to disk and read back by another process, it would name a
/// Work that store never held, or worse, one it holds by coincidence. A payload
/// that never leaves the process is the one place it belongs on the wire, and a
/// field has to opt in here by name to put it there.
///
/// Written as `[store, work]`.
pub mod in_process {
    use super::OpenWork;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(work: &OpenWork, serializer: S) -> Result<S::Ok, S::Error> {
        [work.store, work.work].serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<OpenWork, D::Error> {
        let [store, work] = <[u64; 2]>::deserialize(deserializer)?;
        Ok(OpenWork { store, work })
    }
}

static NEXT_SERIAL: AtomicU64 = AtomicU64::new(1);

/// The stores seen so far, each with its serial.
///
/// Held by `Weak` so a dropped store is not kept alive. A `Weak` still keeps the
/// allocation itself, so no new store can be built at the address of one listed
/// here: an address match is therefore always the same store. Dropped stores are
/// swept when a new one is listed, after which their address may be reused, by a
/// store that will then get a serial of its own.
static STORES: Mutex<Vec<(Weak<HashMapStore>, u64)>> = Mutex::new(Vec::new());

fn store_serial(store: &Arc<HashMapStore>) -> u64 {
    // Poison-safe, like every other lock in this crate.
    let mut stores = STORES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, serial)) = stores
        .iter()
        .find(|(listed, _)| std::ptr::eq(listed.as_ptr(), Arc::as_ptr(store)))
    {
        return *serial;
    }
    stores.retain(|(listed, _)| listed.strong_count() > 0);
    let serial = NEXT_SERIAL.fetch_add(1, Ordering::Relaxed);
    stores.push((Arc::downgrade(store), serial));
    serial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_work_in_one_store_has_one_key() {
        let db = DbContext::new().unwrap();
        let clone = db.clone();
        assert_eq!(OpenWork::of(&db, 3), OpenWork::of(&clone, 3));
        assert_eq!(OpenWork::of(&db, 3).work_id(), 3);
    }

    #[test]
    fn two_works_in_one_store_have_two_keys() {
        let db = DbContext::new().unwrap();
        assert_ne!(OpenWork::of(&db, 1), OpenWork::of(&db, 2));
    }

    /// The reason the key has a store half: every store numbers its Works from
    /// 1, and the registries holding these keys are process-wide.
    #[test]
    fn the_same_work_id_in_two_stores_has_two_keys() {
        let a = DbContext::new().unwrap();
        let b = DbContext::new().unwrap();
        assert_ne!(OpenWork::of(&a, 1), OpenWork::of(&b, 1));
    }

    /// A store dropped and another built, possibly at the same address, must not
    /// inherit the first one's keys.
    #[test]
    fn a_store_built_after_another_is_dropped_has_keys_of_its_own() {
        let first = OpenWork::of(&DbContext::new().unwrap(), 1);
        let second = OpenWork::of(&DbContext::new().unwrap(), 1);
        assert_ne!(first, second);
    }

    #[test]
    fn a_key_survives_an_in_process_payload() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct Payload {
            #[serde(with = "in_process")]
            work: OpenWork,
        }
        let db = DbContext::new().unwrap();
        let sent = Payload {
            work: OpenWork::of(&db, 4),
        };
        let text = serde_json::to_string(&sent).unwrap();
        assert_eq!(serde_json::from_str::<Payload>(&text).unwrap(), sent);
    }

    #[test]
    fn detached_keys_are_all_different_and_name_no_real_work() {
        let db = DbContext::new().unwrap();
        let real = OpenWork::of(&db, 0);
        let a = OpenWork::detached();
        let b = OpenWork::detached();
        assert_ne!(a, b);
        assert_ne!(a, real);
        assert_ne!(b, real);
    }
}
