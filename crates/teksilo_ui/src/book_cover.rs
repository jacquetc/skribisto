// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The book cover feature: each Book carries its own cover, chosen and shown on the Book's
//! own page.
//!
//! A cover belongs to a **Book**, not to the project. A Work holding a trilogy holds three
//! covers, and a Book exported alone opens with its own (see `skribisto_compiler`, which
//! reads the same link). That is why the cover is set from the Book's page and not from a
//! Work-wide menu: the page is the one place that is unambiguously about one Book.
//!
//! Which rows may carry a cover is [`skribisto_model::cover_capable`]: the two encodings
//! of a Book opening, `Folder/Book` and the legacy flat `Item/BookBegin`.

mod book_cover_vm;
mod cover_block;

pub use book_cover_vm::{BookCoverViewModel, CoverError, CoverState};
pub use cover_block::CoverBlock;

use crate::tabs::ContentTab;

/// The cover block for a tab's own page, or `None` when the tab's row is not a Book.
pub fn cover_block_for(tab: &ContentTab) -> Option<CoverBlock> {
    skribisto_model::cover_capable(tab.role(), tab.sub_role()).then(|| {
        CoverBlock::new(BookCoverViewModel::new(
            tab.app_ctx(),
            tab.ids().clone(),
            tab.docs(),
            tab.item_id(),
        ))
    })
}
