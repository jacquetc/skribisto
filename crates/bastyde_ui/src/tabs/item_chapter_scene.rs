// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/ChapterScene` — a flat chapter that owns its scene prose directly: the
//! dual-pane writing editor with a chapter-title field on top. Shares the
//! [`prose`](super::shared::prose) body with the plain scene; the title field is
//! present because the constraint matrix grants ChapterScene a `ChapterTitle`.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::prose(tab)
}
