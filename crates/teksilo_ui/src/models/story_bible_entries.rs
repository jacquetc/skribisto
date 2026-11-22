// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a row source needs to say which of its rows are story bible entries.
//!
//! The rule is `skribisto_model::story_bible`'s. This is the store half: one Work's
//! discoverable tag ids, read once per load rather than once per row, and the events
//! after which a source's answer may be stale.
//!
//! Entry status changes without any item changing: turning "find in prose" on for a tag
//! makes every note wearing it an entry. So a source that carries `is_entry` on its rows
//! has to reload on tag events as well as item ones, which [`TAG_ORIGINS`] lists.

use std::collections::HashSet;

use frontend::AppContext;
use frontend::commands::{binder_tag_commands, work_commands};
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Origin, TagManagementEvent};

/// The ids of `work_id`'s discoverable tags.
///
/// Read through `Work.tags`, never `get_all_binder_tag`: with a second project open, the
/// whole store holds both projects' tags, and the same reason
/// [`crate::models::WorkTagsListModel`] gives applies here. An unreadable store reads as
/// "no discoverable tag", so a row shows its plain glyph rather than claiming to be an
/// entry it cannot prove it is.
pub fn discoverable_tag_ids_for_work(ctx: &AppContext, work_id: u64) -> HashSet<u64> {
    let tag_ids = work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Tags)
        .unwrap_or_default();
    skribisto_model::story_bible::discoverable_tag_ids(
        binder_tag_commands::get_binder_tag_multi(ctx, &tag_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|t| (t.id, t.discoverable)),
    )
}

/// The events after which an item's entry status may have changed with no item event:
/// a tag created, edited (its "find in prose" switch above all) or deleted, and a tag
/// import, which creates through its own unit of work and publishes one feature event.
pub const TAG_ORIGINS: [Origin; 4] = [
    Origin::DirectAccess(DirectAccessEntity::BinderTag(EntityEvent::Created)),
    Origin::DirectAccess(DirectAccessEntity::BinderTag(EntityEvent::Updated)),
    Origin::DirectAccess(DirectAccessEntity::BinderTag(EntityEvent::Removed)),
    Origin::TagManagement(TagManagementEvent::ImportTags),
];
