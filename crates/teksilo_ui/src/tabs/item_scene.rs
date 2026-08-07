// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/Scene` — the flat writing scene: the dual-pane synopsis + main-text
//! editor with no title. Shares the [`prose`](super::shared::prose) body with
//! Item/ChapterScene and Item/Note.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::prose(tab)
}
