// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/Text` — an inert text marker carrying no editable content: opens the
//! shared [`placeholder`](super::shared::placeholder).

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::placeholder(tab)
}
