// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How many tabs an editor pane keeps open.
//!
//! Not a view-model (it owns no state): the vocabulary the setting stores, the settings
//! page offers and `EditorsViewModel` enforces, kept out of both for the same reason
//! [`SynopsisPlacement`](super::SynopsisPlacement) is.
//!
//! The limit is **per pane** and counts only tabs that are not pinned: pinning is the
//! writer saying "keep this one", and a limit that could close a pinned tab, or that a
//! row of pinned tabs could fill, would make the two features fight. It is applied when
//! a tab is opened, never when the setting changes or a saved desk is restored, so
//! choosing a smaller limit closes nothing until the next open.

use serde::{Deserialize, Serialize};

/// How many unpinned tabs a pane keeps.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabLimit {
    /// One: each item opened replaces the tab the writer was on.
    One,
    /// The five most recently used.
    Five,
    /// Every tab stays until the writer closes it. The default, because it is how every
    /// existing installation already behaves.
    #[default]
    All,
}

impl TabLimit {
    /// Every limit, in the order the settings control lists them.
    pub fn all() -> [Self; 3] {
        [Self::One, Self::Five, Self::All]
    }

    /// How many unpinned tabs a pane may hold, or `None` for no limit.
    pub fn cap(self) -> Option<usize> {
        match self {
            Self::One => Some(1),
            Self::Five => Some(5),
            Self::All => None,
        }
    }

    /// This limit's slot in the settings control. Paired with
    /// [`from_index`](Self::from_index), which is how the enum crosses to the
    /// control's `Signal<usize>`.
    pub fn to_index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Five => 1,
            Self::All => 2,
        }
    }

    /// The limit at a control slot. Out of range reads as the default rather than
    /// panicking, so a stray index can never start closing tabs.
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::One,
            1 => Self::Five,
            _ => Self::All,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_keeps_every_tab_so_existing_installations_are_unchanged() {
        assert_eq!(TabLimit::default(), TabLimit::All);
        assert_eq!(TabLimit::default().cap(), None);
    }

    #[test]
    fn the_caps_are_one_and_five() {
        assert_eq!(TabLimit::One.cap(), Some(1));
        assert_eq!(TabLimit::Five.cap(), Some(5));
    }

    #[test]
    fn every_limit_round_trips_through_its_control_index() {
        for (slot, limit) in TabLimit::all().into_iter().enumerate() {
            assert_eq!(limit.to_index(), slot);
            assert_eq!(TabLimit::from_index(slot), limit);
        }
    }

    #[test]
    fn an_out_of_range_index_closes_nothing() {
        assert_eq!(TabLimit::from_index(99), TabLimit::All);
    }
}
