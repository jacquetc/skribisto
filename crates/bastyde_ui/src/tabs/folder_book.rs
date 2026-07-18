// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/Book` — the manuscript root container: its parts, chapters and scenes.
//!
//! Shares the [`folder_segmented`](super::shared::folder_segmented) body with the
//! Chapter and Part containers: **Synopsis** (book title + subtitle + synopsis), **Full
//! Book** (every part heading, chapter heading and scene, as one continuous
//! manuscript), and **Full Synopsis** (the same rows as an editable outline of the whole
//! book).

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    // Only the Book gets the "Pace" segment (the manuscript-wide writing plan).
    shared::folder_segmented(
        tab,
        tr!(segment_book()),
        tr!(full_book()),
        Some((tr!(segment_pace()), super::pace::pace_pane(tab))),
    )
}
