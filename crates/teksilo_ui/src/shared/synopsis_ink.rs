// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The colour a synopsis is written in: the manuscript's own, or the dimmed one that
//! marks it as a working note beside the text.
//!
//! Not a view-model (it owns no state): the vocabulary the Synopsis typography page
//! stores and the synopsis editors read, kept out of both for the reason
//! [`SynopsisPlacement`](super::SynopsisPlacement) is.
//!
//! Two values and no middle step. Each is a theme **role**, so the editor resolves it
//! against the theme in force at every paint and follows light, dark, high contrast and
//! the desktop's own colours with nothing to track here. A shade between the two would
//! have to be a colour mixed by the app, which teksilo has no role for, and which would
//! then have to follow every one of those theme changes by hand.

use serde::{Deserialize, Serialize};
use teksilo::prelude::TextRole;

/// How a synopsis is inked.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynopsisInk {
    /// The colour the manuscript is written in.
    Normal,
    /// The theme's secondary text colour: the synopsis reads as a note about the text
    /// rather than as the text. The default, because it is what every synopsis looked
    /// like before the choice existed.
    #[default]
    Dimmed,
}

impl SynopsisInk {
    /// Every choice, in the order the settings control lists them.
    pub fn all() -> [Self; 2] {
        [Self::Normal, Self::Dimmed]
    }

    /// The theme role an editor paints this ink with.
    ///
    /// `Normal` is [`TextRole::EditorFg`], the manuscript editor's own ink, and not
    /// `TextRole::Primary`: the two differ in the dark theme (`#BCBEC4` against
    /// `#DFE1E5`), and "normal" means "like the text above it".
    pub fn text_role(self) -> TextRole {
        match self {
            Self::Normal => TextRole::EditorFg,
            Self::Dimmed => TextRole::Secondary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_dimmed_so_an_existing_synopsis_looks_unchanged() {
        assert_eq!(SynopsisInk::default(), SynopsisInk::Dimmed);
        assert_eq!(SynopsisInk::default().text_role(), TextRole::Secondary);
    }

    #[test]
    fn normal_is_the_manuscript_ink_not_the_interface_one() {
        assert_eq!(SynopsisInk::Normal.text_role(), TextRole::EditorFg);
    }

    #[test]
    fn each_choice_is_listed_once() {
        let all = SynopsisInk::all();
        assert_eq!(all.len(), 2);
        assert_ne!(all[0], all[1]);
    }

    #[test]
    fn a_choice_round_trips_through_the_stored_form() {
        for ink in SynopsisInk::all() {
            let stored = toml::Value::try_from(ink).map_err(|e| e.to_string());
            let back: Result<SynopsisInk, String> =
                stored.and_then(|v| v.try_into().map_err(|e: toml::de::Error| e.to_string()));
            assert_eq!(back, Ok(ink));
        }
    }
}
