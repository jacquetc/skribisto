// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for a notes folder's Gallery header. Line-style, `stroke="currentColor"`, on the
//! 16 px grid the other toolbars use. `res!` needs a literal path per call site, so there is
//! one function per icon.

use teksilo::res;
use teksilo::widgets::IconWidget;

/// Notes without images: a tile holding only lines of text.
pub fn text_notes() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/gallery/text-notes.svg"))
}
