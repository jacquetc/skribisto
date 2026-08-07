// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where a footnote's static `content` anchor should point after `split_scene` or
//! `merge_two_scenes` moves its prose to a different `Content` row.
//!
//! `Footnote.content` (`common::entities::Footnote`) is set **once, at creation**, and
//! nothing before this module ever wrote to one that already had a value. That is fine
//! for a note's *placement* — which item shows its badge, which number it prints —
//! because `skribisto_model::footnote_numbering` re-derives placement on every pass by
//! scanning live prose; it never trusts the stored pointer, by design (see that
//! module's own doc). But `run_search`/`count_words` (in `search_management` /
//! `progress_management`) are not placement — they attribute a search hit or a word
//! count to *an item*, and for that they read `Footnote.content` directly, because
//! walking every scene's prose just to find one footnote's owner on every keystroke
//! would defeat the point of an anchor. Nothing kept that anchor honest across a
//! structural edit until now.
//!
//! `split_scene` and `merge_two_scenes` are the two `binder_item_management` use cases
//! that actually relocate prose between `Content` rows — split moves the after-caret
//! half into a brand-new row, merge appends the source row's text onto the target's —
//! so they are the two places able to notice "the reference this footnote's anchor
//! pointed at just changed rows" and correct it. Fixed at the source of the drift, per
//! the house rule that no use case may call another: this module is the shared logic
//! both call instead.
//!
//! Pure and store-free, like `skribisto_model::footnote_numbering` (whose
//! [`references_in`] it reuses rather than re-deriving `[^label]` matching by hand —
//! that stays the one place that knows how a citation is written, so this cannot drift
//! from the placement logic it exists to keep honest).

use common::types::EntityId;
use skribisto_model::footnote_numbering::references_in;

/// One footnote's identity as far as reanchoring cares: its own id, its current
/// `content` anchor, and its label (the `[^label]` text a citation is written as).
pub(crate) type FootnoteAnchor = (EntityId, Option<EntityId>, String);

/// Which footnotes anchored to `old_content_id` must move to `moved_content_id`,
/// given the text that stayed on the source row (`stayed`) and the text that was cut
/// into a new/different row (`moved`).
///
/// A footnote reparents only when its label's citation is found in `moved` and **not**
/// in `stayed` — the ordinary case, where a split at the caret cuts each reference
/// cleanly to one side. A label cited from *both* halves (one note referenced twice,
/// now split apart) is left where it is: which half now "owns" it is genuinely
/// ambiguous, its citation still resolves either way (`number_map`'s key is
/// `(item, label)`, not `(content, label)`), and guessing wrong would misfile it worse
/// than leaving it exactly where it was already correct.
///
/// Returns `(footnote_id, new_content_id)` pairs to apply — empty when nothing moved,
/// including when `moved_content_id` is `None` (the moved half was empty, so it never
/// got a row of its own to reparent onto).
pub(crate) fn reanchor_on_split(
    footnotes: &[FootnoteAnchor],
    old_content_id: EntityId,
    stayed: &str,
    moved: &str,
    moved_content_id: Option<EntityId>,
) -> Vec<(EntityId, EntityId)> {
    let Some(new_id) = moved_content_id else {
        return Vec::new();
    };
    footnotes
        .iter()
        .filter(|(_, content, _)| *content == Some(old_content_id))
        .filter(|(_, _, label)| {
            let needle = std::slice::from_ref(label);
            !references_in(moved, needle).is_empty() && references_in(stayed, needle).is_empty()
        })
        .map(|(id, _, _)| (*id, new_id))
        .collect()
}

/// Every footnote anchored to `old_content_id` reparents to `new_content_id`.
///
/// Used by `merge_two_scenes`, where the *whole* role's text moves — appended onto the
/// target row verbatim, or, if the target had none, becomes its brand-new row — so
/// there is no "stayed" half to disambiguate against the way a split has one: whatever
/// referenced the old (about-to-be-trashed) row now sits, in full, on the new one.
pub(crate) fn reanchor_on_merge(
    footnotes: &[FootnoteAnchor],
    old_content_id: EntityId,
    new_content_id: EntityId,
) -> Vec<(EntityId, EntityId)> {
    footnotes
        .iter()
        .filter(|(_, content, _)| *content == Some(old_content_id))
        .map(|(id, _, _)| (*id, new_content_id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fp(id: EntityId, content: Option<EntityId>, label: &str) -> FootnoteAnchor {
        (id, content, label.to_string())
    }

    /// **The split_scene failure scenario from the finding**: a reference that moved
    /// wholesale into the after-caret half reparents to the new row.
    #[test]
    fn a_reference_that_moved_to_the_after_half_reparents_there() {
        let footnotes = vec![fp(1, Some(500), "1")];
        let out = reanchor_on_split(
            &footnotes,
            500,
            "The letter arrived at dawn. She read it twice.",
            "The letter arrived at dawn[^1]. She read it twice.",
            Some(200),
        );
        assert_eq!(out, vec![(1, 200)]);
    }

    /// A reference that stayed on the source's own half is not touched — it is
    /// already correctly anchored.
    #[test]
    fn a_reference_that_stayed_is_left_alone() {
        let footnotes = vec![fp(1, Some(500), "1")];
        let out = reanchor_on_split(
            &footnotes,
            500,
            "The letter[^1] arrived at dawn.",
            "She read it twice.",
            Some(200),
        );
        assert!(out.is_empty());
    }

    /// A footnote anchored to a DIFFERENT content row than the one being split is
    /// never touched, however its label reads.
    #[test]
    fn a_footnote_anchored_elsewhere_is_untouched() {
        let footnotes = vec![fp(1, Some(999), "1")];
        let out = reanchor_on_split(&footnotes, 500, "before[^1]", "after[^1]", Some(200));
        assert!(out.is_empty());
    }

    /// The moved half was empty text — it never got a `Content` row, so there is
    /// nothing to reparent onto even if (implausibly) a label were found in it.
    #[test]
    fn no_new_row_means_nothing_reparents() {
        let footnotes = vec![fp(1, Some(500), "1")];
        let out = reanchor_on_split(&footnotes, 500, "before[^1] stays", "", None);
        assert!(out.is_empty());
    }

    /// A label cited from both halves is ambiguous and is left exactly where it was.
    #[test]
    fn a_label_cited_from_both_halves_is_left_alone() {
        let footnotes = vec![fp(1, Some(500), "1")];
        let out = reanchor_on_split(
            &footnotes,
            500,
            "First mention[^1].",
            "Second mention[^1].",
            Some(200),
        );
        assert!(out.is_empty());
    }

    /// **The merge_two_scenes failure scenario from the finding**: the source's whole
    /// role text is folded into the target's row, so every footnote anchored to the
    /// source's row for that role reparents onto the target's — whether the target's
    /// row already existed (reused id) or was created fresh by the merge.
    #[test]
    fn merging_reparents_every_footnote_on_the_source_row() {
        let footnotes = vec![
            fp(1, Some(500), "a"),
            fp(2, Some(500), "b"),
            fp(3, Some(999), "c"),
        ];
        let out = reanchor_on_merge(&footnotes, 500, 100);
        assert_eq!(
            out,
            vec![(1, 100), (2, 100)],
            "only the source row's own notes move"
        );
    }
}
