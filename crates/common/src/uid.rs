// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Durable entity identity.
//!
//! Store ids (`EntityId`) are positions in an ephemeral `HashMap` that
//! `load_work` re-mints on every load, and a `.skrib`'s `file_id`s are only
//! those store ids at save time — so neither survives a save→load cycle. A
//! `uid` does: it is written to disk, read back unchanged, and is what anything
//! outliving a session (remembered expand state, bookmarks, cross-links) must
//! key on.
//!
//! Typed as [`uuid::Uuid`] rather than a `String`, so a non-uuid cannot be
//! stored and the "absent" case has a canonical value — [`Uuid::nil`] — instead
//! of an empty-string sentinel that every reader has to remember to check.
//!
//! Lives in `common` so every crate can mint one without depending on the
//! `.skrib` format crate; `skribisto_model`, the management features and the UI
//! all create rows.

use uuid::Uuid;

/// A fresh durable identity.
///
/// **Mint one per newly created row, never copy an existing one.** Two rows
/// sharing a uid are indistinguishable to anything keyed by it, which is the
/// failure this exists to prevent — so a duplicate, a split or an import mints,
/// and only a load carries an existing value through.
pub fn new_uid() -> Uuid {
    Uuid::new_v4()
}

/// A uid read from disk, or a fresh one when the source supplied none.
///
/// The single place an absent identity can be caught, whatever route a load
/// took: a pre-v3 `.skrib`, a legacy SQLite project, or a bundle whose
/// migration was skipped.
pub fn heal_uid(from_disk: Uuid) -> Uuid {
    if from_disk.is_nil() {
        new_uid()
    } else {
        from_disk
    }
}

/// A deterministic uid for test and mock fixtures.
///
/// Fixtures must NOT call [`new_uid`]: a mock row is fabricated on demand, so a
/// fresh random value per call would make the same row change identity between
/// refreshes — worse than leaving it absent, since anything keyed by uid would
/// treat every refresh as a new row. Distinct `n` give distinct uids, so a
/// fixture can satisfy the uniqueness precondition without randomness.
pub fn fixture_uid(n: u64) -> Uuid {
    Uuid::from_u128(n as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minted_uids_are_distinct_and_not_nil() {
        let a = new_uid();
        let b = new_uid();
        assert_ne!(a, b, "every mint must be unique");
        assert!(!a.is_nil(), "a minted uid must never read as absent");
    }

    #[test]
    fn healing_preserves_an_existing_uid_and_mints_only_for_a_nil_one() {
        let existing = new_uid();
        assert_eq!(heal_uid(existing), existing, "an existing uid is kept");
        assert!(!heal_uid(Uuid::nil()).is_nil(), "a nil uid is replaced");
    }

    #[test]
    fn fixture_uids_are_deterministic_and_distinct() {
        assert_eq!(fixture_uid(7), fixture_uid(7), "same n, same uid");
        assert_ne!(fixture_uid(7), fixture_uid(8), "distinct n, distinct uid");
        assert!(!fixture_uid(1).is_nil());
    }
}
