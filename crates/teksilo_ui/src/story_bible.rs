// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The story-bible creation pipeline: two doors, and they no longer look alike.
//!
//! - **＋ Create** (`CreateType::StoryBibleEntry`) creates its row immediately, exactly
//!   like every sibling type in that vocabulary, then opens [`modal`]'s *configuration*
//!   step on the row that now exists: a name, tags, aliases, books, a template and a
//!   small body, gathered in local state and committed as one undo step when **Create**
//!   is pressed. Nothing lands before that, because the row is already there.
//!
//! - **"Add as note"**, on any text selection in any prose editor, opens no modal at
//!   all. It is a submenu of this project's tags, and picking one files the note: that
//!   one tag settles where the note goes (`BinderTag.creates_in`) and what it starts
//!   from (`BinderTag.note_template`), which is every question the modal used to ask
//!   except the name, and the name is the selection. See [`capture`] for the menu's
//!   ordering and [`capture_flow`] for what picking a tag does.
//!
//! It used to be one modal reached three ways. The third way ("+ New entry" on the Story
//! bible place) was never built, and the second stopped needing a dialog once a tag could
//! answer for itself.
//!
//! `create` holds the pure transaction (create the item, its content, and, for the
//! configuration path, its tags/aliases/books, as one composite); `infer_book` holds the
//! two ways a fresh entry's `books` pre-set is guessed, never written until the writer
//! confirms it; `modal` is the widget the first door opens.

pub mod capture;
pub mod capture_flow;
pub mod create;
pub mod highlight;
pub mod infer_book;
pub mod modal;

/// Whether an item wearing `tag_ids` is a story bible entry, read from the store.
///
/// For a surface that holds an item's tags but not the project's palette: the editor
/// tab icon, and the note page's "In prose" segment. A note carries a handful of tags, so
/// this is one small batched read over just those ids, never a scan of the whole palette.
/// A tag id the store no longer holds (deleted between the read that produced `tag_ids`
/// and this one) is simply not discoverable. The rule itself is
/// [`skribisto_model::story_bible::is_entry`].
pub fn tags_make_an_entry(ctx: &frontend::AppContext, tag_ids: &[u64]) -> bool {
    if tag_ids.is_empty() {
        return false;
    }
    let discoverable = skribisto_model::story_bible::discoverable_tag_ids(
        frontend::commands::binder_tag_commands::get_binder_tag_multi(ctx, tag_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|t| (t.id, t.discoverable)),
    );
    skribisto_model::story_bible::is_entry(tag_ids, &discoverable)
}
