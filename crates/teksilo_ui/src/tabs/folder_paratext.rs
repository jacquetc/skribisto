// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/Paratext` — somewhere to keep the paratexts so they do not clutter the binder.
//!
//! Organisational only. It emits nothing into the export, so its name never reaches the
//! book, and it carries no manuscript extent — so it gets the same two-segment body a
//! notes folder does: its own synopsis page, plus an Overview of what is inside it.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_synopsis_with_overview(tab)
}
