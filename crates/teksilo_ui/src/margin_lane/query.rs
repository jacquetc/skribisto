// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **What the writer is currently looking for** — one answer, whoever asked.
//!
//! Skribisto has two searches. The per-editor find banner (Ctrl+F) searches the
//! document in front of the writer; Project search searches every document in the
//! Work. Both are legitimate sources of lane marks, and the writer uses whichever
//! is in front of them.
//!
//! ## Why an arbiter rather than reading the find banner
//!
//! Three reasons, and the first alone decides it.
//!
//! A `FindViewModel` lives on the `ContentTab` that owns it, built long after any
//! provider registers, and reachable only as "the focused pane's active tab". A
//! lane on a **stream row** or a search preview is not that tab, so there is
//! genuinely no route from a provider to the session — and a provider that closed
//! over one at registration time would capture state before the app it belongs to
//! exists, which is the failure this seam's own documentation warns about.
//!
//! Second, it is what "last one used wins" *is*. The writer's intent is a single
//! piece of state, and two sources racing to paint the same column would otherwise
//! need an arbiter invented somewhere anyway. Here it is one signal, written by
//! whoever ran a search most recently.
//!
//! Third, the matches are re-derived against the live document at draw time rather
//! than carried across an edit. A project-wide search result is a snapshot of a
//! document as it was when the search ran; drawing its offsets after the writer has
//! typed would put marks on text that has moved. `TextDocument::find_all` is the
//! same matcher the find banner's session uses, so re-running it cannot disagree
//! about what a match is, and it is always current by construction.

use common::types::EntityId;
use teksilo::core::signal::Signal;
use teksilo::text_document::FindOptions;

/// Which of the two searches published the live query.
///
/// Named rather than inferred, so a banner closing can withdraw **its own** query
/// without silently taking a project search's off the lane with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneQuerySource {
    /// One editor's find banner (Ctrl+F), on this item.
    Editor(EntityId),
    /// Project search, which is not standing in any one document.
    Project,
}

/// The query whose hits the lane marks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneQuery {
    /// What the writer typed. Empty means nothing is being searched for, which is
    /// why [`set_active_query`] takes an `Option` rather than leaving callers to
    /// decide whether the empty string counts.
    pub text: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
    /// Folding off. Carried because **project search has this switch** and the find
    /// banner does not: a lane that folded differently from the search that
    /// published to it would mark hits the results list does not count, and miss
    /// ones it does.
    pub diacritic_sensitive: bool,
    /// Who is asking.
    pub source: LaneQuerySource,
    /// The **zero-based ordinal** of the hit the writer is standing on, within the
    /// document the banner is on.
    ///
    /// `None` for a project-wide search, which deliberately has no current match in
    /// any one document: it found hits in forty scenes and the writer is standing on
    /// none of them. Marking one anyway would be inventing a position.
    pub current: Option<usize>,
}

impl LaneQuery {
    /// The matcher options this query means.
    ///
    /// Language tailoring is left at its default, as both searches leave it: it
    /// decides *how* to fold rather than whether to, and only Turkish and
    /// Azerbaijani change under it.
    pub fn options(&self) -> FindOptions {
        FindOptions {
            case_sensitive: self.case_sensitive,
            whole_word: self.whole_word,
            diacritic_sensitive: self.diacritic_sensitive,
            ..Default::default()
        }
    }

    /// Whether `item` is the one carrying the current match, and which ordinal.
    pub fn current_in(&self, item: EntityId) -> Option<usize> {
        match self.source {
            LaneQuerySource::Editor(id) if id == item => self.current,
            _ => None,
        }
    }
}

thread_local! {
    /// Thread-local, like every registry in this seam, and for the same reason: a
    /// `Signal` is not `Send`, and every writer and reader of this is on the UI
    /// thread by construction.
    static ACTIVE: Signal<Option<LaneQuery>> = Signal::new(None);
}

/// The live query, for a surface that wants to react to it.
pub fn active_query() -> Signal<Option<LaneQuery>> {
    ACTIVE.with(|s| s.clone())
}

/// Publish what the writer is looking for. `None` when a search closed.
///
/// **Last call wins**, which is the whole arbitration: the find banner publishes on
/// every keystroke in its field, Project search publishes when it runs, and
/// whichever the writer touched most recently is what the lane shows.
///
/// A no-op when nothing changed, so republishing the same query on every frame does
/// not invalidate anything downstream.
pub fn set_active_query(query: Option<LaneQuery>) {
    ACTIVE.with(|s| {
        let _ = s.set_if_changed(query);
    });
}

/// Withdraw the live query **only if `source` is the one that published it**.
///
/// A find banner closing must take its own marks off the lane. It must not take a
/// project search's off as well: the writer may have run one, then opened and
/// closed Ctrl+F without typing, and clearing unconditionally would make the
/// project's hits vanish from a strip nobody touched.
pub fn clear_active_query_from(source: LaneQuerySource) {
    ACTIVE.with(|s| {
        if s.get().is_some_and(|q| q.source == source) {
            let _ = s.set_if_changed(None);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear() {
        set_active_query(None);
    }

    #[test]
    fn the_last_source_to_publish_is_the_one_the_lane_shows() {
        clear();
        set_active_query(Some(LaneQuery {
            text: "ferry".into(),
            case_sensitive: false,
            whole_word: false,
            diacritic_sensitive: false,
            source: LaneQuerySource::Project,
            current: None,
        }));
        set_active_query(Some(LaneQuery {
            text: "lamp".into(),
            case_sensitive: true,
            whole_word: false,
            diacritic_sensitive: false,
            source: LaneQuerySource::Editor(7),
            current: Some(2),
        }));

        let q = active_query().get().expect("a query is active");
        assert_eq!(q.text, "lamp");
        assert!(q.case_sensitive);
        assert_eq!(q.current_in(7), Some(2));
        assert_eq!(
            q.current_in(8),
            None,
            "another item has no current match, even while the query is live"
        );
        clear();
    }

    /// Republishing an unchanged query must not bump the signal, or every frame of
    /// an open find banner invalidates the lane and the marks are rebuilt sixty
    /// times a second for a query nobody touched.
    #[test]
    fn republishing_the_same_query_changes_nothing() {
        clear();
        let q = Some(LaneQuery {
            text: "ferry".into(),
            case_sensitive: false,
            whole_word: false,
            diacritic_sensitive: false,
            source: LaneQuerySource::Project,
            current: None,
        });
        set_active_query(q.clone());
        let generation = active_query().generation();
        set_active_query(q);
        assert_eq!(active_query().generation(), generation);
        clear();
    }

    /// A banner closing withdraws its own query and nothing else's. The writer may
    /// have run a project search, then opened and closed Ctrl+F without typing, and
    /// clearing unconditionally would take the project's hits off a strip nobody
    /// touched.
    #[test]
    fn closing_one_search_does_not_withdraw_the_others_query() {
        clear();
        set_active_query(Some(LaneQuery {
            text: "ferry".into(),
            case_sensitive: false,
            whole_word: false,
            diacritic_sensitive: false,
            source: LaneQuerySource::Project,
            current: None,
        }));

        clear_active_query_from(LaneQuerySource::Editor(7));
        assert!(
            active_query().get().is_some_and(|q| q.text == "ferry"),
            "an editor banner closing must not clear a project search"
        );

        clear_active_query_from(LaneQuerySource::Project);
        assert!(
            active_query().get().is_none(),
            "but its own source may withdraw it"
        );
        clear();
    }

    /// The two toggles that travel are the two the writer sees; the rest stay at the
    /// find banner's own defaults so the lane and the banner cannot count differently.
    #[test]
    fn the_options_are_the_banners_options() {
        let q = LaneQuery {
            text: "strasse".into(),
            case_sensitive: false,
            whole_word: true,
            diacritic_sensitive: false,
            source: LaneQuerySource::Project,
            current: None,
        };
        let o = q.options();
        assert!(!o.case_sensitive);
        assert!(o.whole_word);
        assert!(
            !o.diacritic_sensitive,
            "folding stays on unless a search says otherwise"
        );
        assert!(!o.use_regex);
    }
}
