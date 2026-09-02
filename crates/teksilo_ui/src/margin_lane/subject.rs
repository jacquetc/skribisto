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
    /// **The rest of the Work's discoverable names**, as
    /// [`crate::mentions::MentionIndex::discoverable_table`] holds them.
    ///
    /// Carried because overlap resolution is a decision over the *whole* table, not over
    /// one entry: "Grace Kelly" and "Grace" are two characters, and a scan that is shown
    /// only "Grace" credits her with every "Grace Kelly" in the book. The index already
    /// refuses that, so a reading scanning alone marked names the roster beside it does
    /// not count, and the two surfaces disagreed about the same sentence.
    ///
    /// Empty is legitimate: the index is empty until its first scan lands, and
    /// [`hits`] still matches the subject's own names against the prose.
    pub table: Vec<DiscoverableEntity>,
    /// Which Work this belongs to, so a second window on a second project marks nothing
    /// rather than this project's names.
    pub work_uid: String,
    /// **Which reading published it**: the surface token that reading hands its own
    /// editors and its own lane ([`super::LaneScope`]).
    ///
    /// The note's id is not enough to tell two publishers apart. `Work ▸ New Window` puts
    /// a second window on the same project with its own tab set, so the *same* note's
    /// **In prose** page can be open twice at once; keyed by note, one of them leaving its
    /// segment withdrew the other's still-visible marks. A scope is minted per surface, so
    /// it separates them.
    pub publisher: super::LaneScope,
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

/// Withdraw the subject **only if `publisher` is the reading that published it**.
///
/// A reading leaving the screen must take its own marks off the lane and no one else's:
/// the writer may have another reading open in a second window, and clearing
/// unconditionally would blank a strip nobody touched. By the publishing *surface* and not
/// by the note, because the same note can be read in two windows at once and a note-keyed
/// test cannot tell those two apart. See [`LaneSubject::publisher`].
pub fn clear_subject_for(publisher: super::LaneScope) {
    ACTIVE.with(|s| {
        if s.get().is_some_and(|a| a.publisher == publisher) {
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
pub fn hits(
    text: &str,
    entity: &DiscoverableEntity,
    table: &[DiscoverableEntity],
) -> Vec<(usize, usize)> {
    // **Scanned against the whole table, then filtered.** `mentions::resolve_overlaps`
    // drops a name that falls inside a longer one, and it can only do that for names it
    // was shown: handed `[entity]` alone it kept "Grace" inside every "Grace Kelly",
    // marking a character the index deliberately does not count there. So the scan sees
    // the Work's names and this keeps the subject's own hits out of the result.
    //
    // The subject's own entry comes from the reading rather than from the table: the
    // writer may have typed an alias a moment ago and the index's last scan is behind it,
    // and the reading is about the entry as it is now.
    let mut scanned: Vec<DiscoverableEntity> = table
        .iter()
        .filter(|e| e.id != entity.id)
        .cloned()
        .collect();
    scanned.push(entity.clone());
    let fingerprint = mentions::fingerprint_alias_table(&scanned);
    mentions::cached_mentions(text, &scanned, fingerprint, FoldLocale::default())
        .iter()
        .filter(|m| m.entity_id == entity.id)
        .map(|m| (m.char_start, m.char_start + m.char_len))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The entry as the matcher takes it: a title and its aliases, kept apart.
    fn entry(title: &str, aliases: &[&str]) -> DiscoverableEntity {
        entry_id(7, title, aliases)
    }

    /// The same, for a Work that holds more than one entry.
    fn entry_id(id: u64, title: &str, aliases: &[&str]) -> DiscoverableEntity {
        DiscoverableEntity {
            id,
            title: title.to_string(),
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// A reading of `who` with no other entry in the Work: the state the index is in
    /// before its first scan lands, and what every case below but the last is about.
    fn hits_alone(text: &str, who: &DiscoverableEntity) -> Vec<(usize, usize)> {
        hits(text, who, &[])
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
                hits_alone(prose, who),
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
        assert_eq!(hits_alone("Elizabeth Bennet arrived.", &who), vec![(0, 16)]);
    }

    /// Each alias is found on its own.
    #[test]
    fn every_alias_is_found() {
        let who = entry("Elizabeth", &["Lizzy"]);
        assert_eq!(
            hits_alone("Lizzy teased Elizabeth.", &who),
            vec![(0, 5), (13, 22)]
        );
    }

    /// **Whole words only.** A character called Ana must not light up every banana —
    /// the mention scanner's own rule, which this now simply *is* rather than imitates.
    #[test]
    fn a_name_inside_a_word_is_not_a_hit() {
        let who = entry("Ana", &[]);
        assert!(hits_alone("a banana, and Ana's hat", &who).contains(&(14, 17)));
        assert_eq!(hits_alone("a banana", &who), Vec::new());
    }

    /// An apostrophe is not a letter, so a possessive still matches.
    #[test]
    fn a_possessive_still_matches() {
        let who = entry("Ana", &[]);
        assert_eq!(hits_alone("Ana's hat", &who), vec![(0, 3)]);
    }

    /// **A shorter name inside another entry's longer one is not this entry's hit.**
    ///
    /// Two characters, "Grace Kelly" and "Grace". `mentions::resolve_overlaps` is a
    /// decision over the whole table (the longer name claims the span), and a scan handed
    /// only the shorter entry has no longer name to lose to. That is what this used to do:
    /// the roster reported no appearance of Grace in "Grace Kelly stepped out", while the
    /// reading beside it washed the word and its header counted one, on the same sentence.
    #[test]
    fn a_longer_entry_takes_the_span_from_a_shorter_one() {
        let kelly = entry_id(1, "Grace Kelly", &[]);
        let grace = entry_id(2, "Grace", &[]);
        let table = vec![kelly.clone(), grace.clone()];
        let prose = "Grace Kelly stepped out.";

        assert_eq!(
            hits(prose, &grace, &table),
            Vec::new(),
            "the shorter name is inside the longer one and belongs to nobody here"
        );
        assert_eq!(
            hits(prose, &kelly, &table),
            vec![(0, 11)],
            "and the longer one keeps its own span"
        );
    }

    /// The rest of the Work is scanned and then dropped: another entry's name is never
    /// marked in a reading about this one.
    #[test]
    fn another_entrys_name_is_not_this_readings_hit() {
        let grace = entry_id(2, "Grace", &[]);
        let table = vec![entry_id(1, "Elizabeth", &[]), grace.clone()];
        assert_eq!(
            hits("Elizabeth found Grace.", &grace, &table),
            vec![(16, 21)]
        );
    }

    /// **The reading's own names beat the index's copy of them.**
    ///
    /// An alias typed a moment ago is not in the last scan's table. The entry the reading
    /// carries is the one that matches, so a fresh alias is marked at once rather than
    /// after the next scan lands.
    #[test]
    fn a_freshly_typed_alias_matches_before_the_index_has_it() {
        let stale = entry_id(7, "Elizabeth", &[]);
        let live = entry_id(7, "Elizabeth", &["Lizzy"]);
        assert_eq!(
            hits("Lizzy waited.", &live, &[stale]),
            vec![(0, 5)],
            "the reading's entry, not the table's stale copy of it"
        );
    }

    /// The subject is withdrawn only by whoever published it.
    ///
    /// **By surface and not by note**, which is the case the second half pins: `Work ▸ New
    /// Window` can put the same note's reading on screen twice, and the one that leaves
    /// must not blank the one that stayed.
    #[test]
    fn only_the_publisher_withdraws_the_subject() {
        let (mine, theirs) = (
            super::super::LaneScope::fresh(),
            super::super::LaneScope::fresh(),
        );
        let subject = LaneSubject {
            entity: entry("Elizabeth", &[]),
            table: Vec::new(),
            work_uid: "w".into(),
            publisher: mine,
        };
        set_active_subject(Some(subject.clone()));
        clear_subject_for(theirs);
        assert_eq!(
            active_subject().get(),
            Some(subject),
            "another reading of the *same* note, in another window, may not withdraw this one"
        );
        clear_subject_for(mine);
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
            hits_alone(
                "She stared out at the grey water long after the ferry had gone.",
                &who
            ),
            Vec::new()
        );
    }
}
