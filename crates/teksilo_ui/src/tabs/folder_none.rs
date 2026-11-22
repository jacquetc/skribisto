// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/None` — a plain grouping folder: its synopsis, and the Resources filed in it,
//! on the two-segment
//! [`folder_synopsis_with_resources`](super::shared::folder_synopsis_with_resources) body.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_synopsis_with_resources(tab)
}
