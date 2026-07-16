// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/BookEnd` — the book-end delimiter: a contentless marker, so it opens a
//! quiet [`placeholder`](super::shared::placeholder) rather than an editor.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::placeholder(tab)
}
