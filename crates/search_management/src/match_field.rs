// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The one mapping between a `Content` row's [`ContentRole`] and the [`MatchField`] a
//! search result reports for it.
//!
//! Search and Replace All sit at opposite ends of the same round trip: `run_search`
//! records *which field* matched, and `replace_in_project` uses that field to find its way
//! back to the row it must rewrite. If the two disagree by even one role, Replace All
//! resolves a hit to the wrong `Content` row — and because it re-derives the occurrence
//! count inside whatever row it landed on, the usual outcome is a silent `skipped_stale`
//! with the writer's text left untouched. The nastier outcome is the counts coinciding, in
//! which case it rewrites a document the writer never searched.
//!
//! They used to be two `match` arms in two files, kept in step by a comment. That is what
//! this module exists to make impossible: one function each way, and a test that walks
//! every `ContentRole` proving they compose to the identity.
//!
//! The mapping is deliberately **not** the same question as which *toggle* a role obeys.
//! An epigraph is searched when the writer ticks "body text" — quoted matter is part of
//! the manuscript — but it reports [`MatchField::Epigraph`], because a chapter can carry
//! `SceneText` and `EpigraphText` at once and the resolution above has to tell them apart.

use common::entities::{ContentRole, MatchField};

/// The field a hit in this content role reports, or `None` for a role that search never
/// looks inside (the title roles, which are matched on the item, not on a `Content` row).
///
/// Only ever answers for `Content` rows. Titles and labels are matched on the
/// `BinderItem`, and comments on their own entities; those passes record their fields
/// directly and never come through here.
pub fn field_of_role(role: &ContentRole) -> Option<MatchField> {
    match role {
        // A paratext is prose the author wrote, so it answers to the body toggle. Unlike
        // the epigraph it needs no field of its own: an `Item/Paratext` carries only
        // `ParatextText`, so there is no second row on the same item for Replace All to
        // resolve to by mistake.
        ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText => {
            Some(MatchField::Body)
        }
        ContentRole::SynopsisText => Some(MatchField::Synopsis),
        ContentRole::EpigraphText => Some(MatchField::Epigraph),
        // Its own field, never `Body` or `Synopsis`: a scene carries `SceneText`,
        // `SynopsisText` and `ScratchpadText` at once, and a hit in the scratchpad must
        // resolve to the scratchpad and nothing else. Which toggle searches it is
        // `run_search`'s business, as for the others.
        ContentRole::ScratchpadText => Some(MatchField::Scratchpad),
        // Titles live on the `BinderItem`, so a title hit is `MatchField::Title` recorded
        // against the item — never against one of these mirror rows.
        ContentRole::BookTitle
        | ContentRole::BookSubtitle
        | ContentRole::PartTitle
        | ContentRole::ChapterTitle => None,
    }
}

/// Whether a `Content` row of this role is the row a result on `field` refers to — the
/// inverse of [`field_of_role`], and the predicate Replace All resolves rows with.
pub fn role_matches_field(field: &MatchField, role: &ContentRole) -> bool {
    field_of_role(role).as_ref() == Some(field)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `ContentRole`. The exhaustive match in [`role_index`] stops compiling when a
    /// variant is added, which is the reminder to list it here too.
    const ALL_ROLES: [ContentRole; 10] = [
        ContentRole::SceneText,
        ContentRole::NoteText,
        ContentRole::SynopsisText,
        ContentRole::BookTitle,
        ContentRole::BookSubtitle,
        ContentRole::PartTitle,
        ContentRole::ChapterTitle,
        ContentRole::EpigraphText,
        ContentRole::ParatextText,
        ContentRole::ScratchpadText,
    ];

    fn role_index(role: &ContentRole) -> usize {
        match role {
            ContentRole::SceneText => 0,
            ContentRole::NoteText => 1,
            ContentRole::SynopsisText => 2,
            ContentRole::BookTitle => 3,
            ContentRole::BookSubtitle => 4,
            ContentRole::PartTitle => 5,
            ContentRole::ChapterTitle => 6,
            ContentRole::EpigraphText => 7,
            ContentRole::ParatextText => 8,
            ContentRole::ScratchpadText => 9,
        }
    }

    /// Every `MatchField`, kept complete the same way through [`field_index`].
    const ALL_FIELDS: [MatchField; 10] = [
        MatchField::Body,
        MatchField::Title,
        MatchField::Synopsis,
        MatchField::Label,
        MatchField::Epigraph,
        MatchField::Comment,
        MatchField::CommentReply,
        MatchField::Footnote,
        MatchField::Resource,
        MatchField::Scratchpad,
    ];

    fn field_index(field: &MatchField) -> usize {
        match field {
            MatchField::Body => 0,
            MatchField::Title => 1,
            MatchField::Synopsis => 2,
            MatchField::Label => 3,
            MatchField::Epigraph => 4,
            MatchField::Comment => 5,
            MatchField::CommentReply => 6,
            MatchField::Footnote => 7,
            MatchField::Resource => 8,
            MatchField::Scratchpad => 9,
        }
    }

    /// The two lists above are complete and in step with their matches: each variant sits
    /// at the index its match gives it, so a variant added to the match but not to the
    /// list fails here.
    #[test]
    fn the_role_and_field_lists_cover_every_variant() {
        for (i, role) in ALL_ROLES.iter().enumerate() {
            assert_eq!(role_index(role), i, "{role:?} is out of place");
        }
        for (i, field) in ALL_FIELDS.iter().enumerate() {
            assert_eq!(field_index(field), i, "{field:?} is out of place");
        }
    }

    /// Every role that search can match must be resolvable back to itself, and to no
    /// other field. This is the property whose absence let an epigraph hit rewrite a
    /// chapter's scene prose.
    #[test]
    fn the_mapping_round_trips_for_every_content_role() {
        for role in &ALL_ROLES {
            let own = field_of_role(role);
            for field in &ALL_FIELDS {
                assert_eq!(
                    role_matches_field(field, role),
                    own.as_ref() == Some(field),
                    "{role:?} reports {own:?} but {field:?} disagrees about resolving to it"
                );
            }
        }
    }

    /// The specific collision the split exists to prevent: on a chapter carrying both,
    /// scene prose and an epigraph must not answer to the same field.
    #[test]
    fn a_chapters_prose_and_its_epigraph_are_distinguishable() {
        let prose = field_of_role(&ContentRole::SceneText).unwrap();
        let epigraph = field_of_role(&ContentRole::EpigraphText).unwrap();
        assert_ne!(
            prose, epigraph,
            "a chapter may hold both rows at once; one field for both would resolve a \
             hit in either to whichever row came first"
        );
        assert!(!role_matches_field(&prose, &ContentRole::EpigraphText));
        assert!(!role_matches_field(&epigraph, &ContentRole::SceneText));
    }

    /// A scene carries its prose, its synopsis and its scratchpad at once. A scratchpad
    /// hit reports its own field, and neither the body nor the synopsis field ever
    /// resolves to the scratchpad row, so Replace All cannot rewrite one for the other.
    #[test]
    fn a_scratchpad_is_its_own_field_never_body_or_synopsis() {
        assert_eq!(
            field_of_role(&ContentRole::ScratchpadText),
            Some(MatchField::Scratchpad)
        );
        assert!(!role_matches_field(
            &MatchField::Body,
            &ContentRole::ScratchpadText
        ));
        assert!(!role_matches_field(
            &MatchField::Synopsis,
            &ContentRole::ScratchpadText
        ));
        assert!(!role_matches_field(
            &MatchField::Scratchpad,
            &ContentRole::SceneText
        ));
        assert!(!role_matches_field(
            &MatchField::Scratchpad,
            &ContentRole::NoteText
        ));
    }
}
