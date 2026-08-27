// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which Book a fresh story-bible entry should be pre-filed under: a **guess**
//! offered as an editable default, never a write. See [`crate::story_bible::modal`]
//! for where it lands (a visible, changeable chip row) and `common::entities::BinderItem`'s
//! own `books` field doc for what the value means once the writer confirms it.
//!
//! Two different backward walks, and it matters which one a caller reaches for:
//!
//! - [`book_containing`] answers "which Book does this **scene** physically sit
//!   inside", by binder position, the same question
//!   `structure_drift::measure::book_containing` answers inside the commercial
//!   edition, reimplemented here rather than depended on: nothing in this crate may
//!   ever name that one (the one-way rule), and the walk itself is three relationship
//!   hops, cheap enough that duplicating it costs far less than inventing a seam to
//!   avoid duplicating it.
//! - Filing a fresh entry created *from* an existing note copies that note's own
//!   already-declared `books`, no walk at all; see [`crate::story_bible::modal`]'s
//!   own construction sites, which is the highest-confidence of the pre-sets and
//!   needs nothing from this module.
//!
//! **This is a proximity guess, not a fact.** A scene sits inside exactly one Book
//! by construction (the flat stream is a state machine over Book/Chapter/Scene
//! markers), so for a scene the guess is exact. Off a scene (the top of the
//! binder, a paratext, a row before any Book) it is `None`, and the caller leaves
//! the pre-set empty rather than inventing a "the first Book" default:
//! `structure_drift::measure::first_book`'s own doc names exactly this shortcut as
//! a trap, kept alive with zero callers precisely as a warning against reaching for
//! it the moment enumerating Books gets inconvenient.

use frontend::AppContext;
use frontend::commands::binder_item_commands;
use frontend::common::entities::BinderItemSubRole;

use crate::models::binder_stream::ordered_flat_items;

/// The Book containing the item at `item_id`, as a store id: the nearest
/// `Folder/Book` at or before `item_id` in the Work's flat, binder-major stream,
/// **within `item_id`'s own binder** and not across a book's own end marker.
///
/// `None` when `item_id` names no live row, when it sits before any Book (front
/// matter, a stray note at the top of the binder), when it sits in a binder that
/// holds no Book of its own (a Notes or Research binder: the stream is concatenated
/// binder-major with nothing between one binder and the next), or when a `BookEnd`
/// closed the last book before it. Both guards are the ones commit e300338b ("a book
/// stops at its own binder's edge") gave every other walk of this stream: without
/// them a note in the Notes binder is guessed into the manuscript's last Book, and
/// only the shipped templates' trailing `BookEnd` ever hid it.
///
/// `Folder/Book` only, deliberately, and not every row `SubRoleExt::opens_book()`
/// would admit: this pre-set feeds the same picker
/// `crate::docks::inspector::live_books` fills, which is scoped to the modern
/// encoding, and a guess it cannot render is a filing the writer can neither see nor
/// undo. A legacy `Item/BookBegin` therefore opens nothing here either.
///
/// Trashed rows are already excluded by [`ordered_flat_items`], so a merely-trashed
/// Book is never offered as a guess, the same rule that picker applies.
pub fn book_containing(ctx: &AppContext, work_id: u64, item_id: u64) -> Option<u64> {
    let items = ordered_flat_items(ctx, work_id);
    let position = items.iter().position(|(_, it)| it.id == item_id)?;
    let binder = items[position].0;
    for (index, (row_binder, it)) in items[..=position].iter().enumerate().rev() {
        if *row_binder != binder {
            break;
        }
        if it.sub_role == BinderItemSubRole::Book {
            return Some(it.id);
        }
        // The end marker itself is the last row *of* its book, so it only closes the
        // walk for the rows that come after it.
        if index != position && it.sub_role == BinderItemSubRole::BookEnd {
            break;
        }
    }
    None
}

/// The item's own `books` field, read fresh from the store.
///
/// Used when a fresh entry is filed *from* an existing note (a folder's "apply to
/// children" cousin, at creation time rather than by hand): the source is itself
/// unscoped by binder position, so copying its declared value once is the honest
/// pre-set, not a positional guess. A one-shot snapshot, never a live binding: see
/// [`crate::story_bible::modal`]'s own doc for why the copy must never track the
/// source afterward.
pub fn declared_books(ctx: &AppContext, item_id: u64) -> Vec<u64> {
    binder_item_commands::get_binder_item(ctx, &item_id)
        .ok()
        .flatten()
        .map(|it| it.books)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{CreateBinderItemDto, CreateWorkDto};
    use std::rc::Rc;

    /// A Work with two Binders: a manuscript holding a Note before any Book, a Book
    /// with a Chapter and a Scene inside it, the Book's own `BookEnd` and a paratext
    /// row after it; and a Notes binder holding one note of its own. Those are the
    /// shapes `book_containing` has to tell apart, including the two the walk used to
    /// run straight through.
    struct Fixture {
        ctx: Rc<AppContext>,
        work_id: u64,
        note_before_any_book: u64,
        book: u64,
        scene: u64,
        book_end: u64,
        row_after_book_end: u64,
        note_in_notes_binder: u64,
    }

    fn seed() -> Fixture {
        let ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &frontend::direct_access::CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("create binder");
        let notes_binder = binder_commands::create_binder(
            &ctx,
            None,
            &frontend::direct_access::CreateBinderDto {
                name: "Notes".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            1,
        )
        .expect("create binder");

        // One index counter per binder: `create_binder_item`'s index is relative to the
        // binder it names, never to the concatenated stream.
        let mut next_manuscript_index = 0i32;
        let mut next_notes_index = 0i32;
        let create = |binder_id: u64,
                      index: &mut i32,
                      role: BinderItemRole,
                      sub_role: BinderItemSubRole,
                      indent: i64| {
            let created = binder_item_commands::create_binder_item(
                &ctx,
                None,
                &CreateBinderItemDto {
                    title: "row".into(),
                    role,
                    sub_role,
                    activated: true,
                    is_exportable: true,
                    indent,
                    ..Default::default()
                },
                binder_id,
                *index,
            )
            .expect("create item");
            *index += 1;
            created.id
        };

        let note_before_any_book = create(
            binder.id,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Note,
            0,
        );
        let book = create(
            binder.id,
            &mut next_manuscript_index,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
        );
        let _chapter = create(
            binder.id,
            &mut next_manuscript_index,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            1,
        );
        let scene = create(
            binder.id,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            2,
        );
        let book_end = create(
            binder.id,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::BookEnd,
            1,
        );
        let row_after_book_end = create(
            binder.id,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Note,
            0,
        );
        let note_in_notes_binder = create(
            notes_binder.id,
            &mut next_notes_index,
            BinderItemRole::Item,
            BinderItemSubRole::Note,
            0,
        );

        Fixture {
            ctx,
            work_id: work.id,
            note_before_any_book,
            book,
            scene,
            book_end,
            row_after_book_end,
            note_in_notes_binder,
        }
    }

    #[test]
    fn a_scene_resolves_to_its_own_book() {
        let f = seed();
        assert_eq!(book_containing(&f.ctx, f.work_id, f.scene), Some(f.book));
    }

    #[test]
    fn the_book_itself_resolves_to_itself() {
        let f = seed();
        assert_eq!(book_containing(&f.ctx, f.work_id, f.book), Some(f.book));
    }

    #[test]
    fn a_row_before_any_book_resolves_to_none() {
        let f = seed();
        assert_eq!(
            book_containing(&f.ctx, f.work_id, f.note_before_any_book),
            None
        );
    }

    #[test]
    fn an_unknown_item_resolves_to_none() {
        let f = seed();
        assert_eq!(book_containing(&f.ctx, f.work_id, 999_999), None);
    }

    /// The stream is concatenated binder-major with nothing between one binder and the
    /// next, so a backwards walk with no binder guard runs out of the Notes binder and
    /// into the manuscript's last Book.
    #[test]
    fn a_note_in_another_binder_resolves_to_none() {
        let f = seed();
        assert_eq!(
            book_containing(&f.ctx, f.work_id, f.note_in_notes_binder),
            None
        );
    }

    #[test]
    fn a_row_past_the_book_end_resolves_to_none() {
        let f = seed();
        assert_eq!(
            book_containing(&f.ctx, f.work_id, f.row_after_book_end),
            None
        );
    }

    /// The end marker is the last row *of* its book, not the first row outside it.
    #[test]
    fn the_book_end_itself_resolves_to_its_own_book() {
        let f = seed();
        assert_eq!(book_containing(&f.ctx, f.work_id, f.book_end), Some(f.book));
    }
}
