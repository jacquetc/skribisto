// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The manuscript as it stands **now**, reduced to one digest per row — the
//! "now" side of the Timeline dock's comparison.
//!
//! A recorded version knows each row's `uid`, its position and the bytes of its
//! prose. To say what has changed *since* one, the live project has to be
//! reducible to the same three things. That is all this does.
//!
//! ## One definition of "this row's text", used on both sides
//!
//! [`digest_of`] is deliberately the only place the rule lives: a row's digest is
//! blake3 over its prose roles in a fixed order, each prefixed by the role's own
//! slug. Both sides call it — the live side with `Content` rows, the recorded side
//! with blobs read out of a bundle — so a difference in the digest is a difference
//! in the text and never a difference in how the two sides happened to concatenate.
//!
//! The role prefix is load-bearing: without it, moving a paragraph from a
//! synopsis into the scene body would leave the concatenation identical and the
//! row would read as unchanged.
//!
//! Read here (Layer A) rather than in the view-model that consumes it, per the
//! repo's read-through-Layer-A rule, and following the same real/mock seam as
//! [`crate::models::binder_stream`] — a mocks build has no backend, so it reports
//! an empty manuscript and every comparison honestly finds nothing.

/// One live row, reduced to what a comparison against a recorded version needs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LiveRow {
    pub uid: uuid::Uuid,
    pub title: String,
    /// Position in the binder-major order — what "moved" is measured against.
    pub order: usize,
    /// See [`digest_of`]. Empty when the row carries no prose at all.
    pub digest: String,
}

/// The digest of one row's prose.
///
/// `roles` is `(role slug, text)`; order does not matter, because this sorts.
/// A row with no prose digests to the empty string rather than to the hash of
/// nothing, so "has no text" and "has text that happens to hash to X" stay
/// distinguishable.
pub fn digest_of(roles: &[(String, String)]) -> String {
    if roles.iter().all(|(_, text)| text.is_empty()) {
        return String::new();
    }
    let mut sorted: Vec<&(String, String)> = roles.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut hasher = blake3::Hasher::new();
    for (slug, text) in sorted {
        hasher.update(slug.as_bytes());
        hasher.update(b"\0");
        hasher.update(text.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().to_hex().to_string()
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::collections::HashMap;

    use frontend::AppContext;
    use frontend::commands::{
        binder_commands, binder_item_commands, content_commands, work_commands,
    };
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;

    use super::{LiveRow, digest_of};
    use crate::models::NameContext;
    use common::entities::ContentRole;

    /// Every live row of `work_id`, binder-major, digested.
    ///
    /// Trashed rows are included and keep their place, exactly as
    /// `binder_stream::ordered_binder_items` does: an item in the trash still
    /// exists, and dropping it here would report it as *deleted since* — which is
    /// a different and more alarming claim than the truth.
    pub fn live_manuscript(ctx: &AppContext, work_id: u64) -> Vec<LiveRow> {
        let mut out: Vec<LiveRow> = Vec::new();
        // An untitled chapter is not nameless. Since numbering stopped writing
        // "Chapter 7" into titles, a structural row's `title` is usually empty
        // and every other surface derives the name instead — so a change list
        // built from the raw field is a column of blanks, which is what it was.
        let names = NameContext::read(ctx, work_id);
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
            let by_id: HashMap<u64, (uuid::Uuid, String)> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, (it.uid, it.title)))
                    .collect();
            for id in item_ids {
                let (uid, title) = by_id.get(&id).cloned().unwrap_or_default();
                let title = if title.trim().is_empty() {
                    names
                        .item(id)
                        .and_then(|it| names.generated_name(it))
                        .unwrap_or(title)
                } else {
                    title
                };
                let order = out.len();
                out.push(LiveRow {
                    uid,
                    title,
                    order,
                    digest: digest_for_item(ctx, id),
                });
            }
        }
        out
    }

    /// One item's prose, digested. Only the roles that are prose — a title is a
    /// name, and the recorded side stores no blob for it, so including it here
    /// would make every row differ from every version forever.
    fn digest_for_item(ctx: &AppContext, item_id: u64) -> String {
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        let roles: Vec<(String, String)> = content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter_map(|c| {
                skrib_format::slug::prose_kind(&c.role).map(|k| (k.to_string(), c.data))
            })
            .collect();
        digest_of(&roles)
    }

    /// One live row's text for **one** content role, as it stands now.
    ///
    /// The digest above answers "did this row change"; this answers "into what",
    /// which is what a comparison the writer can read needs. Deliberately one
    /// role and one row: the whole manuscript's prose is read on the UI thread
    /// every time the band's selection moves, and carrying all of it so that the
    /// rare opened row can be compared would pay for the manuscript to see the
    /// scene.
    ///
    /// `None` when the row is gone, or holds no text in that role — both of
    /// which are ordinary, and neither of which is an empty document.
    pub fn live_prose(
        ctx: &AppContext,
        work_id: u64,
        uid: uuid::Uuid,
        role: &ContentRole,
    ) -> Option<String> {
        let item_id = crate::models::ordered_binder_items(ctx, work_id)
            .into_iter()
            .find(|r| r.uid == uid)?
            .id;
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .find(|c| &c.role == role)
            .map(|c| c.data)
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use frontend::AppContext;

    use super::LiveRow;
    use common::entities::ContentRole;

    /// No real backend under mocks. An empty manuscript makes every comparison
    /// find nothing, which is the honest answer for a build with no project.
    pub fn live_manuscript(_ctx: &AppContext, _work_id: u64) -> Vec<LiveRow> {
        Vec::new()
    }

    /// Same reasoning: no backend, so no live text to compare against, and the
    /// caller falls back to showing the recorded text on its own.
    pub fn live_prose(
        _ctx: &AppContext,
        _work_id: u64,
        _uid: uuid::Uuid,
        _role: &ContentRole,
    ) -> Option<String> {
        None
    }
}

pub use imp::{live_manuscript, live_prose};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_with_no_prose_digests_to_nothing_rather_than_to_a_hash() {
        assert!(digest_of(&[]).is_empty());
        assert!(digest_of(&[("scene".into(), String::new())]).is_empty());
    }

    #[test]
    fn the_order_the_roles_arrive_in_does_not_change_the_digest() {
        let a = digest_of(&[
            ("scene".into(), "the lamp".into()),
            ("synopsis".into(), "a room".into()),
        ]);
        let b = digest_of(&[
            ("synopsis".into(), "a room".into()),
            ("scene".into(), "the lamp".into()),
        ]);
        assert_eq!(a, b, "two sides that sort differently must still agree");
    }

    /// The reason each role is prefixed by its own name.
    #[test]
    fn moving_text_between_two_roles_is_not_reported_as_unchanged() {
        let before = digest_of(&[
            ("scene".into(), "the lamp went out".into()),
            ("synopsis".into(), String::new()),
        ]);
        let after = digest_of(&[
            ("scene".into(), String::new()),
            ("synopsis".into(), "the lamp went out".into()),
        ]);
        assert_ne!(
            before, after,
            "the same words in a different field is a change, not a coincidence",
        );
    }

    #[test]
    fn the_same_text_digests_the_same_way_twice() {
        let roles = vec![(
            "scene".to_string(),
            "She waited by the harbour.".to_string(),
        )];
        assert_eq!(digest_of(&roles), digest_of(&roles));
    }
}
