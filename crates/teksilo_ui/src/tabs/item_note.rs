// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/Note`: a story-bible entry's own tab, now a workspace rather than a single
//! prose editor. Elaborating an entry no longer depends on a dock being open beside it.
//!
//! Three segments, built by [`shared::item_note_segmented`] exactly the way a notes
//! folder's own tab is built by [`shared::folder_synopsis_with_overview`]:
//!
//! - **Note** ([`shared::segments::SEG_NOTE_OWN`]): the dual-pane writing editor this
//!   tab has always been, byte-identical. [`shared::item_note_segmented`] reuses this
//!   crate's own [`shared::prose`] body, split from its backdrop the same way a notes
//!   folder's own page is (see that function's doc for why one backdrop, not two).
//! - **Details** ([`shared::segments::SEG_NOTE_DETAILS`]): the story-bible fields
//!   ([`crate::tabs::note_details`]) at the tab's full width, so a writer can flesh out
//!   a character or a place with no Inspector dock open at all.
//! - **In prose** ([`shared::segments::SEG_NOTE_IN_PROSE`]): shown only when this note
//!   carries a tag flagged discoverable, reading the manuscript prose it has been
//!   declared present in ([`crate::tabs::note_in_prose`]).
//!
//! A plain note (a stray thought, a research clipping with no story-bible tag at all)
//! shows only the first two, and the third's chip grows onto the bar the moment a
//! discoverable tag is added, with no need to leave and reopen the tab: see
//! [`shared::item_note_segmented`]'s own doc for how the gate stays live.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::item_note_segmented(tab)
}
