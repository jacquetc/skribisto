// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/Paratext` — somewhere to keep the paratexts so they do not clutter the binder.
//!
//! Organisational only. It emits nothing into the export, so its name never reaches the
//! book, and it carries no manuscript extent — so it gets a two-segment body: its own
//! synopsis page, plus an Overview of what is inside it.
//!
//! Not the Story bible grid a notes folder also gets from the same function. That one
//! reads the container's `Item/Note` descendants, and this folder holds `Item/Paratext`
//! rows, so it would be empty by construction: see `shared::folder_synopsis_with_overview`.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_synopsis_with_overview(tab)
}
