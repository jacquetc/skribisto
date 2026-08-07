// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/BookBegin` — the book-start marker: the book title + subtitle + the
//! book's synopsis (symmetric with the `Folder/Book` container). Shares the
//! [`heading`](super::shared::heading) form.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::heading(tab)
}
