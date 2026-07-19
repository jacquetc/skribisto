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
//! Lives in `common` so every crate can mint one without taking a dependency on
//! the `.skrib` format crate — `skribisto_model`, the management features, and
//! the UI all create rows.

/// A fresh durable identity (UUID v4).
///
/// **Mint one per newly created row, never copy an existing one.** Two rows
/// sharing a uid are indistinguishable to anything keyed by it, which is the
/// failure this type exists to prevent — so a duplicate, a split, or an import
/// mints, and only a load carries an existing value through.
pub fn new_uid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// A uid read from disk, or a fresh one when the source supplied none.
///
/// The single place an empty identity can be caught, whatever route a load
/// took: a pre-v3 `.skrib`, a legacy SQLite project, or a bundle whose
/// migration was skipped.
pub fn heal_uid(from_disk: &str) -> String {
    if from_disk.is_empty() {
        new_uid()
    } else {
        from_disk.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minted_uids_are_distinct() {
        let a = new_uid();
        let b = new_uid();
        assert_ne!(a, b, "every mint must be unique");
        assert!(!a.is_empty());
    }

    #[test]
    fn healing_preserves_an_existing_uid_and_mints_only_for_an_empty_one() {
        assert_eq!(heal_uid("keep-me"), "keep-me");
        assert!(!heal_uid("").is_empty());
    }
}
