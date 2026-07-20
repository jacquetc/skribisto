// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/Note` — a notes group. The matrix gives it only a synopsis, but it does hold a
//! subtree, so it gets the two-segment
//! [`folder_synopsis_with_overview`](super::shared::folder_synopsis_with_overview) body:
//! its own page, and an Overview of what is inside it.
//!
//! Not the five-segment container body: a notes folder has no manuscript extent, so the
//! three stream views and the Corkboard would all be empty by construction.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_synopsis_with_overview(tab)
}
