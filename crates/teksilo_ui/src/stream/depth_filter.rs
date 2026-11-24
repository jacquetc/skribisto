// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Which levels of a stream are shown**: everything, or a Full Book or Full Part read
//! as its outline, its scenes (or its chapters and scenes) hidden.
//!
//! Named by kind of row, never by a number of levels. A book is rarely a strict
//! outline: a prologue chapter can stand beside the parts, an interlude scene can sit
//! directly in a part. Counted in levels, that interlude is as deep as the part's
//! chapters, so "two levels" would show it and hide the chapters' scenes, and "hide the
//! scenes" could not be said at all. By kind, it can, whatever the book's shape, and the
//! banner can say in the writer's words what is out of sight.
//!
//! A filter only ever takes levels away from the bottom: a scene is never shown without
//! its chapter, so what stays is a true outline of what is there (see
//! `skribisto_model::compile::row_depths`, whose tests prove the depths of the rows that
//! stay do not move).

use skribisto_model::compile::{RowRank, StreamLevel};
use teksilo::prelude::*;

/// The levels a stream shows. Per tab, and remembered with it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DepthFilter {
    /// Every row. The default, and the only choice a Full Chapter has.
    #[default]
    Everything,
    /// Parts and chapters; the scenes are hidden.
    DownToChapters,
    /// Parts alone; the chapters and the scenes are hidden.
    PartsOnly,
}

impl DepthFilter {
    /// The deepest kind of row this filter shows.
    pub fn limit(self) -> RowRank {
        match self {
            Self::Everything => RowRank::Scene,
            Self::DownToChapters => RowRank::Chapter,
            Self::PartsOnly => RowRank::Part,
        }
    }

    /// The choices a container of this level offers, the first being no filter.
    ///
    /// A Book has parts, chapters and scenes; a Part has chapters and scenes and can
    /// never hold a part (one ends it); a Chapter holds scenes alone, so a filter there
    /// could only empty the page and none is offered.
    pub fn offered(level: StreamLevel) -> &'static [Self] {
        match level {
            StreamLevel::Book => &[Self::Everything, Self::DownToChapters, Self::PartsOnly],
            StreamLevel::Part => &[Self::Everything, Self::DownToChapters],
            StreamLevel::Chapter => &[],
        }
    }

    /// This filter, if a container of `level` offers it, else no filter at all.
    ///
    /// A remembered choice can outlive the container it was made on: a Book converted to
    /// a Part keeps its tab, and "Parts only" on a Part would show nothing. Never hiding
    /// more than the writer chose for this container is the safe reading.
    pub fn within(self, level: StreamLevel) -> Self {
        if Self::offered(level).contains(&self) {
            self
        } else {
            Self::Everything
        }
    }

    /// The stable string a tab is remembered with. `None` for no filter, which is what a
    /// tab that never chose one has written down too.
    pub fn id(self) -> Option<&'static str> {
        match self {
            Self::Everything => None,
            Self::DownToChapters => Some("chapters"),
            Self::PartsOnly => Some("parts"),
        }
    }

    /// The filter a remembered string names. Anything else, a newer build's choice
    /// included, reads as no filter: showing everything never hides a row the writer
    /// did not choose to hide.
    pub fn from_id(id: Option<&str>) -> Self {
        match id {
            Some("chapters") => Self::DownToChapters,
            Some("parts") => Self::PartsOnly,
            _ => Self::Everything,
        }
    }

    /// The choice as the control names it.
    pub fn label(self) -> LocalizedString {
        match self {
            Self::Everything => tr!(stream_show_everything()),
            Self::DownToChapters => tr!(stream_show_down_to_chapters()),
            Self::PartsOnly => tr!(stream_show_parts_only()),
        }
    }

    /// What the banner says is out of sight, or `None` when nothing is.
    pub fn hidden(self) -> Option<LocalizedString> {
        match self {
            Self::Everything => None,
            Self::DownToChapters => Some(tr!(stream_hidden_scenes())),
            Self::PartsOnly => Some(tr!(stream_hidden_chapters_and_scenes())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each choice hides from the bottom: what it shows is every kind down to its limit.
    #[test]
    fn each_filter_shows_every_level_down_to_its_limit() {
        assert_eq!(DepthFilter::Everything.limit(), RowRank::Scene);
        assert_eq!(DepthFilter::DownToChapters.limit(), RowRank::Chapter);
        assert_eq!(DepthFilter::PartsOnly.limit(), RowRank::Part);
    }

    /// A Book offers all three, a Part two, a Chapter none; no filter always comes first.
    #[test]
    fn a_container_offers_the_levels_it_can_hold() {
        assert_eq!(DepthFilter::offered(StreamLevel::Book).len(), 3);
        assert_eq!(
            DepthFilter::offered(StreamLevel::Part),
            &[DepthFilter::Everything, DepthFilter::DownToChapters]
        );
        assert!(DepthFilter::offered(StreamLevel::Chapter).is_empty());
        for level in [StreamLevel::Book, StreamLevel::Part] {
            assert_eq!(DepthFilter::offered(level)[0], DepthFilter::Everything);
        }
    }

    /// A choice the container does not offer reads as no filter, never as another one.
    #[test]
    fn a_choice_the_container_cannot_hold_shows_everything() {
        assert_eq!(
            DepthFilter::PartsOnly.within(StreamLevel::Part),
            DepthFilter::Everything
        );
        assert_eq!(
            DepthFilter::DownToChapters.within(StreamLevel::Chapter),
            DepthFilter::Everything
        );
        assert_eq!(
            DepthFilter::DownToChapters.within(StreamLevel::Part),
            DepthFilter::DownToChapters
        );
    }

    /// The remembered strings round-trip, and nothing else hides a row.
    #[test]
    fn a_remembered_filter_round_trips_and_anything_else_shows_everything() {
        for filter in [
            DepthFilter::Everything,
            DepthFilter::DownToChapters,
            DepthFilter::PartsOnly,
        ] {
            assert_eq!(DepthFilter::from_id(filter.id()), filter);
        }
        assert_eq!(
            DepthFilter::from_id(Some("scenes")),
            DepthFilter::Everything
        );
        assert_eq!(DepthFilter::from_id(Some("")), DepthFilter::Everything);
        assert_eq!(DepthFilter::from_id(None), DepthFilter::Everything);
    }

    /// Only a filter that hides something has something for the banner to say.
    #[test]
    fn only_a_filter_that_hides_something_has_a_banner() {
        assert!(DepthFilter::Everything.hidden().is_none());
        assert!(DepthFilter::DownToChapters.hidden().is_some());
        assert!(DepthFilter::PartsOnly.hidden().is_some());
    }
}
