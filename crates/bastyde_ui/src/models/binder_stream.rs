// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The flat, binder-major `(item_id, title)` stream of a Work — a **one-shot**
//! snapshot in the authoritative relationship order that a save writes and a load
//! reproduces. Read here (Layer A) so the raw entity reads don't live in the
//! view-model that consumes them (per the repo's "read through Layer A" rule).
//!
//! Unlike the reactive collection models, this is a plain query: the
//! workspace-layout capture/restore needs the *current* order at one moment (to
//! map an open tab to a stable ordinal, and back), not a live-updating handle. It
//! follows the models' real/mock seam so a mock build compiles and stays inert.

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::collections::HashMap;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;

    /// Every binder item of `work_id` as `(id, title)`, binder-major, in each
    /// binder's stored relationship order (trashed items included — they stay in
    /// place, so the ordinal space is stable). Empty on any backend hiccup.
    pub fn ordered_binder_items(ctx: &AppContext, work_id: u64) -> Vec<(u64, String)> {
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
            let title_of: HashMap<u64, String> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, it.title))
                    .collect();
            for id in item_ids {
                out.push((id, title_of.get(&id).cloned().unwrap_or_default()));
            }
        }
        out
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use frontend::AppContext;

    /// No real backend under mocks — tab persistence isn't meaningful there, so the
    /// stream is empty (restore becomes a no-op).
    pub fn ordered_binder_items(_ctx: &AppContext, _work_id: u64) -> Vec<(u64, String)> {
        Vec::new()
    }
}

pub use imp::ordered_binder_items;
