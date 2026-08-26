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
//! ## Scope
//!
//! Set while a note's **In prose** segment is on screen, cleared when it goes away.
//! Nothing here is persisted: a project closed and reopened marks nothing until the
//! writer opens such a reading again, exactly as the find banner forgets its query.

use common::types::EntityId;
use teksilo::core::signal::Signal;

/// The entry a reading is about, and the names it goes by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneSubject {
    /// The note itself. A row declaring this id as its point of view earns a mark even
    /// where the prose never writes the name, which is the case the whole reading exists
    /// for: a scene in deep third person may name nobody at all.
    pub note_id: EntityId,
    /// Its title and every alias, longest first.
    ///
    /// Longest first so a document naming her "Elizabeth Bennet" is marked once for the
    /// full name rather than twice, overlapping, for the name and the surname inside it.
    pub names: Vec<String>,
    /// Which Work this belongs to, so a second window on a second project marks nothing
    /// rather than this project's names.
    pub work_uid: String,
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
        if s.get().is_some_and(|a| a.note_id == note_id) {
            let _ = s.set_if_changed(None);
        }
    });
}

/// Every offset in `text` where one of `names` appears, as whole words, longest name
/// first so an alias inside a longer name does not double-mark it.
///
/// Whole-word rather than substring, matching the mention scanner's own rule: a
/// character called Ana must not light up every "banana" in the book.
pub fn hits(text: &str, names: &[String]) -> Vec<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    let mut taken = vec![false; chars.len()];
    let mut out: Vec<(usize, usize)> = Vec::new();
    for name in names {
        let needle: Vec<char> = name.chars().collect();
        if needle.is_empty() || needle.len() > chars.len() {
            continue;
        }
        for start in 0..=(chars.len() - needle.len()) {
            let end = start + needle.len();
            if taken[start..end].iter().any(|t| *t) {
                continue;
            }
            if chars[start..end] != needle[..] {
                continue;
            }
            let before_ok = start == 0 || !chars[start - 1].is_alphanumeric();
            let after_ok = end == chars.len() || !chars[end].is_alphanumeric();
            if before_ok && after_ok {
                taken[start..end].iter_mut().for_each(|t| *t = true);
                out.push((start, end));
            }
        }
    }
    out.sort_unstable();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(v: &[&str]) -> Vec<String> {
        let mut n: Vec<String> = v.iter().map(|s| s.to_string()).collect();
        n.sort_by_key(|s| std::cmp::Reverse(s.chars().count()));
        n
    }

    /// **A name inside a longer name is marked once, not twice.**
    ///
    /// "Elizabeth Bennet" contains "Elizabeth". Marking both would put two overlapping
    /// marks on one occurrence and count her twice in anything reading the list.
    #[test]
    fn a_longer_name_wins_the_overlap() {
        let n = names(&["Elizabeth", "Elizabeth Bennet"]);
        assert_eq!(hits("Elizabeth Bennet arrived.", &n), vec![(0, 16)]);
    }

    /// Each alias is found on its own.
    #[test]
    fn every_alias_is_found() {
        let n = names(&["Elizabeth", "Lizzy"]);
        assert_eq!(hits("Lizzy teased Elizabeth.", &n), vec![(0, 5), (13, 22)]);
    }

    /// **Whole words only.** A character called Ana must not light up every banana,
    /// which is the mention scanner's own rule and the reason this is not a substring
    /// search.
    #[test]
    fn a_name_inside_a_word_is_not_a_hit() {
        let n = names(&["Ana"]);
        assert!(hits("a banana, and Ana's hat", &n).contains(&(14, 17)));
        assert_eq!(hits("a banana", &n), Vec::new());
    }

    /// An apostrophe is not a letter, so a possessive still matches.
    #[test]
    fn a_possessive_still_matches() {
        let n = names(&["Ana"]);
        assert_eq!(hits("Ana's hat", &n), vec![(0, 3)]);
    }

    /// The subject is withdrawn only by whoever published it.
    #[test]
    fn only_the_publisher_withdraws_the_subject() {
        let mine = LaneSubject {
            note_id: 7,
            names: vec!["Elizabeth".into()],
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
        let n = names(&["Elizabeth", "Lizzy"]);
        assert_eq!(
            hits(
                "She stared out at the grey water long after the ferry had gone.",
                &n
            ),
            Vec::new(),
            "nothing textual, which is exactly why the declaration has to earn its own mark"
        );
    }

    /// The subject is per Work: a second window on another project marks nothing.
    #[test]
    fn a_subject_carries_the_work_it_belongs_to() {
        let s = LaneSubject {
            note_id: 3,
            names: names(&["Elizabeth"]),
            work_uid: "work-a".into(),
        };
        set_active_subject(Some(s.clone()));
        assert_eq!(
            active_subject().get().map(|a| a.work_uid),
            Some("work-a".into())
        );
        clear_subject_for(3);
    }
}
