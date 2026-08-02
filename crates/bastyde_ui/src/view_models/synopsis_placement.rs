// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where the synopsis sits relative to the manuscript in the dual-pane editor.
//!
//! Not a view-model (it owns no state): the vocabulary the setting stores and the
//! editor layout reads, kept out of the settings pane for the same reason
//! [`HighlightScope`](super::HighlightScope) and
//! [`TypewriterAnchor`](super::TypewriterAnchor) are — both the pane and the pane
//! *bodies* read it, so it belongs to neither.
//!
//! Placement is a preference, not a guarantee. Side needs room for two columns, and
//! the editor can be split down to a 320px pane; below
//! `synopsis width + PROSE_MIN_WIDTH` the layout falls back to Top rather than
//! honouring the setting into a prose column too narrow to write in. See
//! `tabs::shared::editor::WidthProbe`.

use serde::{Deserialize, Serialize};

/// Where a scene's synopsis is drawn relative to its prose.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynopsisPlacement {
    /// A compact box above the manuscript, scrolling with it as one page. The
    /// default: it costs a few lines of height and nothing else, and it is what
    /// every existing project already looks like.
    #[default]
    Top,
    /// A full-height column beside the manuscript, on the window's own chrome
    /// colour, with a draggable divider between them. Lets the synopsis be read
    /// against the prose it describes, at the cost of a permanent slice of width.
    Side,
}

impl SynopsisPlacement {
    /// Every placement, in the order the settings control lists them.
    pub fn all() -> [Self; 2] {
        [Self::Top, Self::Side]
    }

    /// This placement's slot in that control. Paired with
    /// [`from_index`](Self::from_index), which is how the enum crosses to the
    /// control's `Signal<usize>`.
    pub fn to_index(self) -> usize {
        match self {
            Self::Top => 0,
            Self::Side => 1,
        }
    }

    /// The placement at a control slot. Out of range reads as the default rather
    /// than panicking — a stored index can outlive the list it was written against.
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Side,
            _ => Self::Top,
        }
    }

    /// Whether this placement wants the synopsis beside the manuscript. Whether it
    /// actually *gets* it is a layout question — see the module docs.
    pub fn is_side(self) -> bool {
        matches!(self, Self::Side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_top_so_existing_projects_look_unchanged() {
        assert_eq!(SynopsisPlacement::default(), SynopsisPlacement::Top);
        assert!(!SynopsisPlacement::default().is_side());
    }

    #[test]
    fn every_placement_round_trips_through_its_control_index() {
        for (slot, placement) in SynopsisPlacement::all().into_iter().enumerate() {
            assert_eq!(placement.to_index(), slot);
            assert_eq!(SynopsisPlacement::from_index(slot), placement);
        }
    }

    #[test]
    fn an_out_of_range_index_degrades_to_the_default() {
        assert_eq!(SynopsisPlacement::from_index(99), SynopsisPlacement::Top);
    }
}
