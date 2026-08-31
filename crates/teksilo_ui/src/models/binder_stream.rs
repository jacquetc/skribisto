// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The flat, binder-major `(item_id, title)` stream of a Work — a **one-shot**
//! snapshot in the authoritative relationship order that a save writes and a load
//! reproduces. Read here (Layer A) so the raw entity reads don't live in the
//! view-model that consumes them (per the repo's "read through Layer A" rule).
//!
//! Unlike the reactive collection models, this is a plain query: the
//! workspace-layout capture/restore needs the *current* order at one moment (to
//! map an open tab to a stable ordinal, and back), not a live-updating handle.
//!
//! **One implementation, both feature sets.** This used to follow the collection
//! models' real/mock seam, with a `mocks` arm returning an empty stream on the
//! grounds that there is no backend under mocks and tab persistence is not
//! meaningful there. That premise is false: `mocks` is a feature of this crate
//! alone and never reaches `frontend`, so the commands below are the real ones in
//! either build. The seam was not making a mock build inert, it was making it
//! answer a question wrongly, and eight `workspace_layout` tests that seed real
//! items and expect them back failed under `--features mocks` because of it.
//! [`crate::models::binder_stream::ordered_all_items`] below makes the identical
//! relationship hops with no arm
//! at all, which is the shape to match. A build with no project open still gets
//! an empty stream, because a Work with no binders has no items, which is the
//! only case the old arm was really covering.

mod imp {
    use std::collections::HashMap;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};

    use super::BinderItemRef;
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;

    /// Every binder item of `work_id` as `(id, title)`, binder-major, in each
    /// binder's stored relationship order (trashed items included — they stay in
    /// place, so the ordinal space is stable). Empty on any backend hiccup.
    ///
    /// **Deliberately not a projection of [`super::ordered_all_items`]**, though it
    /// looks like one. It emits a row for *every* id the relationship names, falling
    /// back to a default when the dto does not come back; the base walk drops those.
    /// The difference only shows up when a fetch fails mid-stream, and then it decides
    /// whether the rows after it keep their positions — which is the one thing tab
    /// persistence reads this for.
    pub fn ordered_binder_items(ctx: &AppContext, work_id: u64) -> Vec<BinderItemRef> {
        let mut out = Vec::new();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            // `get_binder_item_multi` returns `Vec<Option<..>>` in db-key order (not
            // request order), so index by id and walk `item_ids` (the authoritative
            // relationship order) to build the stream.
            let by_id: HashMap<u64, (uuid::Uuid, String)> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, (it.uid, it.title)))
                    .collect();
            for id in item_ids {
                let (uid, title) = by_id.get(&id).cloned().unwrap_or_default();
                out.push(BinderItemRef { id, uid, title });
            }
        }
        out
    }
}

pub use imp::ordered_binder_items;

/// Every binder item of `work_id` paired with the binder holding it, binder-major, in
/// each binder's stored relationship order — the same order a save writes. **Trashed
/// rows included**; each caller decides.
///
/// The binder id travels alongside because `indent` nests items *within* a binder: it
/// is the only thing marking where one binder's indents stop meaning anything to the
/// next. A caller that does not care drops it.
///
/// This is the one traversal. Four call sites used to spell it out — same relationship
/// hops, same db-key reindexing, same comment about it — and they differed only in what
/// they kept: two filtered on `activated`, two did not, and *that* difference is real,
/// which is why they are projections below rather than one function with a flag. A
/// boolean parameter here would put the load-bearing decision at the call site, spelled
/// `true`.
pub fn ordered_all_items(
    ctx: &frontend::AppContext,
    work_id: u64,
) -> Vec<(u64, frontend::direct_access::BinderItemDto)> {
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::direct_access::BinderItemDto;
    use std::collections::HashMap;

    let mut out = Vec::new();
    let binder_ids =
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default();
    for binder_id in binder_ids {
        let item_ids = binder_commands::get_binder_relationship(
            ctx,
            &binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        // `get_binder_item_multi` answers in db-key order, not request order — index by
        // id and walk `item_ids`, which is the authoritative one.
        let by_id: HashMap<u64, BinderItemDto> =
            binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|it| (it.id, it))
                .collect();
        out.extend(
            item_ids
                .into_iter()
                .filter_map(|id| by_id.get(&id).cloned())
                .map(|it| (binder_id, it)),
        );
    }
    out
}

/// [`ordered_all_items`] less the trashed rows — what a *view* of the manuscript shows.
///
/// The corkboard and the overview both want this. Numbering and tab persistence want the
/// unfiltered walk instead, because a trashed row stays in place and the ordinal space
/// has to stay stable across it.
pub fn ordered_flat_items(
    ctx: &frontend::AppContext,
    work_id: u64,
) -> Vec<(u64, frontend::direct_access::BinderItemDto)> {
    ordered_all_items(ctx, work_id)
        .into_iter()
        .filter(|(_, it)| it.activated)
        .collect()
}

/// One item of the work's flat, binder-major stream: its live store id, its **durable**
/// uid, and its title.
///
/// The uid is what anything persisted keys by — a store id is a position in an ephemeral
/// `HashMap` that `load_work` re-mints on every open — while the id is what commands take
/// and the title is what a restored tab is captioned with before its content loads.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinderItemRef {
    pub id: u64,
    pub uid: uuid::Uuid,
    pub title: String,
}
