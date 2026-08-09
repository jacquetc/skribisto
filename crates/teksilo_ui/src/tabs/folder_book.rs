// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/Book` — the manuscript root container: its parts, chapters and scenes.
//!
//! Shares the [`folder_segmented`](super::shared::folder_segmented) body with the
//! Chapter and Part containers: **Book** (title + subtitle + synopsis), **Full Book**
//! (every part heading, chapter heading and scene, as one continuous manuscript), and
//! **Full Synopsis** (the same rows as an editable outline of the whole book) — plus the
//! Book-only **Pace** and **Analysis** segments below.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    // Only the Book gets these two: "Pace" (the manuscript-wide writing plan) and
    // "Analysis" (measurements over the whole book). Both are book-scale questions, and
    // both sit before Corkboard and Overview — adding either shifts those two indices,
    // which is what `tabs::tests::the_overview_segment_mounts_a_table` pins.
    shared::folder_segmented(
        tab,
        tr!(segment_book()),
        tr!(full_book()),
        vec![
            (
                shared::segments::SEG_PACE,
                tr!(segment_pace()),
                super::pace::pace_pane(tab),
            ),
            (
                shared::segments::SEG_ANALYSIS,
                tr!(analysis_segment()),
                super::analysis::analysis_pane(tab),
            ),
        ],
    )
}
