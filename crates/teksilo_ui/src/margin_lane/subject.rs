// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Whose names the lane is marking** — the story-bible entry a reading is *about*.
//!
//! ## Why this is an arbiter and not a parameter
//!
//! The same problem [`super::query`] solves for search, and solved the same way, for
//! the same reason its own module doc gives: a provider is registered long before the
//! surfaces it draws on exist, and a lane on a stream row has no route back to the tab
//! that owns it. A provider closing over a view-model at registration time would
//! capture state before the application it belongs to is built.
//!
//! ## Why not the search query
//!
//! [`super::LaneQuery`] carries one string. A story-bible entry has a title *and*
//! aliases, and the whole point of the reading is that "Lizzy" and "Elizabeth Bennet"
//! are the same person. A single-needle query cannot say that, and asking the writer to
//! type an alternation would be asking them to do the index's job.
//!
//! ## The matcher is the app's own, and that is not an optimisation
//!
//! [`hits`] runs `skribisto_model::mentions`, the same scan the roster, the Inspector's
//! cast list and every backlink read from. It did not always: a first cut matched the
//! names byte for byte, and **an entry called "Élise Laroche" lit up nothing at all**
//! while her alias "Claire" lit up three times — because the app's matcher folds
//! diacritics and a byte comparison does not. The reading listed the row (the index found
//! her) and then marked nothing in it, which reads as the feature being broken rather
//! than as two matchers disagreeing.
//!
//! So there is one matcher. Case-sensitive, diacritic-**in**sensitive, whole word, names
//! under `MIN_NAME_LEN` ignored: those are `mention_options`' rules, they are deliberately
//! not preferences, and every surface that says where a name is has to obey the same ones
//! or the writer is told two different stories about their own book.
//!
//! ## Scope
//!
//! Set while a note's **In prose** segment is on screen, cleared when it goes away.
//! Nothing here is persisted: a project closed and reopened marks nothing until the
//! writer opens such a reading again, exactly as the find banner forgets its query.

use common::types::EntityId;
use skribisto_model::mentions::{self, DiscoverableEntity};
use teksilo::core::signal::Signal;
use teksilo::text_document::matching::FoldLocale;

/// The entry a reading is about, and the names it goes by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneSubject {
    /// The entry's matching surface: its title and every alias, in the shape the app's own
    /// mention matcher takes. `entity.id` is the note itself — a row declaring it as its
    /// point of view earns a mark even where the prose never writes the name, which is the
    /// case the whole reading exists for: a scene in deep third person may name nobody.
    ///
    /// The whole entity rather than a flattened list of names, because the matcher wants
    /// the title and the aliases apart (a title match outranks an alias one) and because
    /// ordering the names is *its* job — see the module note on why there is one matcher.
    pub entity: DiscoverableEntity,
    /// Which Work this belongs to, so a second window on a second project marks nothing
    /// rather than this project's names.
    pub work_uid: String,
}

impl LaneSubject {
    /// The note this reading is about.
    pub fn note_id(&self) -> EntityId {
        self.entity.id
    }
}

thread_local! {
    static ACTIVE: Signal<Option<LaneSubject>> = Signal::new(None);
}

/// The live subject, if a reading is open.
pub fn active_subject() -> Signal<Option<LaneSubject>> {
    ACTIVE.with(|s| s.clone())
}

/// Publish the subject, or withdraw it with `None`.
pub fn set_active_subject(subject: Option<LaneSubject>) {
    ACTIVE.with(|s| {
        let _ = s.set_if_changed(subject);
    });
}

/// Withdraw the subject **only if `note_id` is the one that published it**.
///
/// A reading closing must take its own marks off the lane and no one else's: the writer
/// may have opened a second note's reading in another window, and clearing
/// unconditionally would blank a strip nobody touched.
pub fn clear_subject_for(note_id: EntityId) {
    ACTIVE.with(|s| {
        if s.get().is_some_and(|a| a.note_id() == note_id) {
            let _ = s.set_if_changed(None);
        }
    });
}

/// Every span of `text` where `entity` is named, as `(start, end)` char offsets in
/// document order.
///
/// The app's own mention scan, not a second matcher — see the module note for the bug
/// that rule is written in. It is memoised on `(prose, alias table, locale)`, so a lane
/// and an underline layer asking about the same row in the same frame pay for one scan.
///
/// `FoldLocale::default()` rather than the row's own dictionary language, because that is
/// what [`crate::mentions::MentionIndex`] scans with: this must agree with the roster the
/// reading was built from, and a locale of its own would let the two disagree about a
/// Turkish dotless i.
pub fn hits(text: &str, entity: &DiscoverableEntity) -> Vec<(usize, usize)> {
    let table = [entity.clone()];
    let fingerprint = mentions::fingerprint_alias_table(&table);
    mentions::cached_mentions(text, &table, fingerprint, FoldLocale::default())
        .iter()
        .map(|m| (m.char_start, m.char_start + m.char_len))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The entry as the matcher takes it: a title and its aliases, kept apart.
    fn entry(title: &str, aliases: &[&str]) -> DiscoverableEntity {
        DiscoverableEntity {
            id: 7,
            title: title.to_string(),
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// **The bug this module's matcher rule is written in.**
    ///
    /// An entry called "Élise Laroche" with the aliases "Élise" and "Claire" lit up only
    /// Claire: the byte-exact matcher this replaced could not see through the accent, so
    /// the reading listed the row (the index folds diacritics and found her) and then
    /// marked nothing in it. Both spellings must find both spellings, in either direction,
    /// because a writer types one of them into the story bible and the other into the
    /// prose without ever noticing.
    #[test]
    fn an_accent_does_not_hide_a_name() {
        let accented = entry("Élise Laroche", &["Élise", "Claire"]);
        let plain = entry("Elise Laroche", &["Elise", "Claire"]);
        let prose_accented = "Élise entra. Claire la suivit.";
        let prose_plain = "Elise entra. Claire la suivit.";

        for (who, prose) in [
            (&accented, prose_accented),
            (&accented, prose_plain),
            (&plain, prose_accented),
            (&plain, prose_plain),
        ] {
            assert_eq!(
                hits(prose, who),
                vec![(0, 5), (13, 19)],
                "both names, whichever side carries the accent"
            );
        }
    }

    /// **A name inside a longer name is marked once, not twice.**
    ///
    /// "Elizabeth Bennet" contains "Elizabeth". Marking both would put two overlapping
    /// marks on one occurrence and count her twice in anything reading the list.
    #[test]
    fn a_longer_name_wins_the_overlap() {
        let who = entry("Elizabeth Bennet", &["Elizabeth"]);
        assert_eq!(hits("Elizabeth Bennet arrived.", &who), vec![(0, 16)]);
    }

    /// Each alias is found on its own.
    #[test]
    fn every_alias_is_found() {
        let who = entry("Elizabeth", &["Lizzy"]);
        assert_eq!(
            hits("Lizzy teased Elizabeth.", &who),
            vec![(0, 5), (13, 22)]
        );
    }

    /// **Whole words only.** A character called Ana must not light up every banana —
    /// the mention scanner's own rule, which this now simply *is* rather than imitates.
    #[test]
    fn a_name_inside_a_word_is_not_a_hit() {
        let who = entry("Ana", &[]);
        assert!(hits("a banana, and Ana's hat", &who).contains(&(14, 17)));
        assert_eq!(hits("a banana", &who), Vec::new());
    }

    /// An apostrophe is not a letter, so a possessive still matches.
    #[test]
    fn a_possessive_still_matches() {
        let who = entry("Ana", &[]);
        assert_eq!(hits("Ana's hat", &who), vec![(0, 3)]);
    }

    /// The subject is withdrawn only by whoever published it.
    #[test]
    fn only_the_publisher_withdraws_the_subject() {
        let mine = LaneSubject {
            entity: entry("Elizabeth", &[]),
            work_uid: "w".into(),
        };
        set_active_subject(Some(mine.clone()));
        clear_subject_for(9);
        assert_eq!(active_subject().get(), Some(mine));
        clear_subject_for(7);
        assert_eq!(active_subject().get(), None);
    }

    /// **A scene told through her eyes earns a stop even when it never writes her name.**
    ///
    /// The reading shows that row because the writer *declared* it, and deep third person
    /// very often names nobody at all. The provider marks such a row at its own first
    /// character; here we pin the half that decides it, which is that the prose genuinely
    /// yields nothing to mark.
    #[test]
    fn a_deep_point_of_view_scene_offers_the_prose_nothing_to_mark() {
        let who = entry("Elizabeth", &["Lizzy"]);
        assert_eq!(
            hits(
                "She stared out at the grey water long after the ferry had gone.",
                &who
            ),
            Vec::new()
        );
    }
}
