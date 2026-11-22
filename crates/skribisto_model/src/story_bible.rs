// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which binder items are story bible entries.
//!
//! An entry has no storage shape of its own: it is any item carrying at least one
//! **discoverable** tag (`BinderTag.discoverable`). That is deliberate. Tagging a note
//! "Characters" makes it an entry and removing the tag makes it a note again, with
//! nothing to migrate, and an entry may wear several discoverable tags at once.
//!
//! Being derived, the answer used to be derived separately wherever it was needed: the
//! mention scan, the note page, the Inspector and the container panes each filtered the
//! tags their own way. This module is the one place that says what an entry is, so the
//! icon a row shows and the fields its page offers cannot disagree about the same item.
//!
//! Pure functions over ids, like [`crate::mentions`]: no store, no entities, no UI.

use std::collections::HashSet;

use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::types::EntityId;

/// The ids of the discoverable tags among `tags`, given as `(id, discoverable)` pairs so
/// every caller can feed it from whatever row type it already holds (a `BinderTag`, a
/// UI `TagRow`, a DTO) without converting first.
pub fn discoverable_tag_ids(tags: impl IntoIterator<Item = (EntityId, bool)>) -> HashSet<EntityId> {
    tags.into_iter()
        .filter_map(|(id, discoverable)| discoverable.then_some(id))
        .collect()
}

/// True when an item carrying `item_tags` is a story bible entry: when any of its tags is
/// one of `discoverable`.
///
/// This is the membership the mention scan indexes. It does not look at the item's
/// `sub_role`: the scan finds a tagged scene's title in the prose as readily as a note's.
/// What a row *shows* for it is narrower, see [`shows_entry_glyph`].
pub fn is_entry(item_tags: &[EntityId], discoverable: &HashSet<EntityId>) -> bool {
    item_tags.iter().any(|id| discoverable.contains(id))
}

/// True when a row for this item should wear the story bible entry glyph instead of its
/// type's own.
///
/// Only a **note item**. A scene or a chapter that carries a discoverable tag is still
/// first a scene or a chapter to the writer, and its type glyph is what tells them where
/// they are in the manuscript, so it keeps it. A notes **folder** keeps the folder-note
/// glyph too: what the scan finds in the prose are the entries inside it, and a folder
/// that suddenly looked like one of its own children would make the tree harder to read.
pub fn shows_entry_glyph(role: &Role, sub_role: &SubRole, is_entry: bool) -> bool {
    is_entry && matches!((role, sub_role), (Role::Item, SubRole::Note))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_discoverable_tags_are_kept() {
        let ids = discoverable_tag_ids([(1, true), (2, false), (3, true)]);
        assert_eq!(ids, HashSet::from([1, 3]));
    }

    #[test]
    fn an_item_with_no_tag_is_never_an_entry() {
        assert!(!is_entry(&[], &HashSet::from([1, 2])));
    }

    #[test]
    fn an_item_is_an_entry_when_any_of_its_tags_is_discoverable() {
        let discoverable = HashSet::from([7]);
        assert!(is_entry(&[3, 7], &discoverable));
        assert!(!is_entry(&[3, 4], &discoverable));
    }

    #[test]
    fn only_a_note_item_wears_the_entry_glyph() {
        assert!(shows_entry_glyph(&Role::Item, &SubRole::Note, true));
        assert!(!shows_entry_glyph(&Role::Item, &SubRole::Note, false));
        assert!(!shows_entry_glyph(&Role::Folder, &SubRole::Note, true));
        assert!(!shows_entry_glyph(&Role::Item, &SubRole::Scene, true));
        assert!(!shows_entry_glyph(
            &Role::Folder,
            &SubRole::ChapterScene,
            true
        ));
    }
}
