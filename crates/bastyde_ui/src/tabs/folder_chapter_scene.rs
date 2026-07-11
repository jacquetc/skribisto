//! `Folder/ChapterScene` — a chapter whose extent is its child scenes (the container
//! encoding of a chapter; the flat one is `Item/ChapterScene`, and promote/demote
//! converts between them losslessly).
//!
//! Shares the [`folder_segmented`](super::shared::folder_segmented) body with the Part
//! and Book containers: **Synopsis**, **Full Chapter** (a Scrivenings-style continuous
//! manuscript of every scene in the chapter, above the chapter's own prose), and **Full
//! Synopsis** (the same rows as an editable outline).

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_segmented(tab, tr!(full_chapter()))
}
