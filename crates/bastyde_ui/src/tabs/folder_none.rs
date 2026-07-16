// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Folder/None` — a plain grouping folder: it carries only a synopsis, shown by
//! the shared [`folder_synopsis_only`](super::shared::folder_synopsis_only) body.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_synopsis_only(tab)
}
