// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/Part` — a part container: its chapters and their scenes.
//!
//! Shares the [`folder_segmented`](super::shared::folder_segmented) body with the
//! Chapter and Book containers: **Part**, **Full Part** (every chapter heading and
//! every scene in the part, as one continuous manuscript), and **Full Synopsis** (the
//! same rows as an editable outline).

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_segmented(tab, tr!(segment_part()), tr!(full_part()), vec![])
}
