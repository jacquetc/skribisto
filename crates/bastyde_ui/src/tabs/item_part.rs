// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/Part` — a part heading in the flat stream: a part-title field above a
//! synopsis. Shares the [`heading`](super::shared::heading) form.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::heading(tab)
}
