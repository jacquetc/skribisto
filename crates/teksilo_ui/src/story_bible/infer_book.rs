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
/// `Folder/Book` at or before `item_id` in the Work's flat, binder-major stream.
///
/// `None` when `item_id` names no live row, or when it sits before any Book (front
/// matter, a stray note at the top of the binder). Trashed rows are already excluded
/// by [`ordered_flat_items`], so a merely-trashed Book is never offered as a guess,
/// the same rule the Inspector's own `live_books` enumerator applies to the picker
/// this pre-set feeds into.
pub fn book_containing(ctx: &AppContext, work_id: u64, item_id: u64) -> Option<u64> {
    let items = ordered_flat_items(ctx, work_id);
    let position = items.iter().position(|(_, it)| it.id == item_id)?;
    items[..=position]
        .iter()
        .rev()
        .find(|(_, it)| it.sub_role == BinderItemSubRole::Book)
        .map(|(_, it)| it.id)
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

    /// A Work with one Binder, a Book, a Chapter inside it, a Scene inside that, and
    /// a Note sitting before any Book: the shapes `book_containing` has to tell
    /// apart.
    struct Fixture {
        ctx: Rc<AppContext>,
        work_id: u64,
        note_before_any_book: u64,
        book: u64,
        scene: u64,
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
            -1,
        )
        .expect("create binder");

        let mut next_index = 0i32;
        let mut create = |role: BinderItemRole, sub_role: BinderItemSubRole, indent: i64| {
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
                binder.id,
                next_index,
            )
            .expect("create item");
            next_index += 1;
            created.id
        };

        let note_before_any_book = create(BinderItemRole::Item, BinderItemSubRole::Note, 0);
        let book = create(BinderItemRole::Folder, BinderItemSubRole::Book, 0);
        let _chapter = create(BinderItemRole::Folder, BinderItemSubRole::ChapterScene, 1);
        let scene = create(BinderItemRole::Item, BinderItemSubRole::Scene, 2);

        Fixture {
            ctx,
            work_id: work.id,
            note_before_any_book,
            book,
            scene,
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
}
