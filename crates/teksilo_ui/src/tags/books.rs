// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Book filing: which Book or Books a writer has declared a story-bible note
//! belongs to.
//!
//! A **declaration**, never a measurement: `BinderItem.books` states what the
//! writer says, independent of anything a scan or a subtree walk finds. It
//! never touches, and never replaces, `book_containing`/`first_book` (which
//! Book a *scene* physically sits inside by binder position; those stay
//! positional, computed fresh every time) or the mention index (where a name
//! actually appears in prose). Only a note or note folder carries it: a
//! scene's Book is already derivable from where it sits in the binder, so
//! only rows outside that flow need to say so by hand.
//!
//! **Empty means not yet filed, never "belongs to every Book."** The four
//! reasons live on the field's own doc comment
//! (`common::entities::BinderItem::books`); the short version is that every
//! sibling relationship on this struct (tags, point of view, references)
//! already reads an empty list as none, and there is no second bit an
//! additive field could ever grow to tell "untouched" apart from "applies
//! everywhere."
//!
//! Reuses [`super::cast_add::CastAddPopover`] exactly as [`super::pov`] does,
//! over a different candidate table: the picker widget only ever needed
//! id/title pairs and a pin closure, never `DiscoverableEntity` itself, so
//! the same widget serves a `Folder/Book` candidate list with no changes.
//! What *is* new is the candidate table: a Book is not a story-bible entry
//! and never appears in [`skribisto_model::mentions::DiscoverableEntity`]'s
//! table, so candidates come from the Work's own live `Folder/Book` items
//! instead (see `docks::inspector::live_books`).

use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, HStack, IconButton, PopoverButton, TextWidget};

use super::cast_add::{CastAddPopover, CastCandidate};
use super::mention_list::PinReference;

/// Remove `target` from the focused item's Book filing.
pub type ClearBook = Rc<dyn Fn(u64, &mut EventContext)>;

/// One declared Book, as shown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BookChip {
    pub id: u64,
    pub title: String,
}

/// Resolve the ids stored on an item against the live `Folder/Book` table.
///
/// An id with no matching entry is dropped rather than rendered as a blank
/// chip. It means one of three things: the Book was trashed, the Book was
/// deleted, or the id never resolved to a `Folder/Book` at all.
///
/// The two removal cases are handled in different places, which is worth
/// stating because only one of them is a real prune. A deleted Book is
/// swept out of every list naming it by the generated
/// `reconcile_backref_binder_item_books`, so its id is gone from the store.
/// A merely trashed Book keeps its id on the item, and is filtered right
/// here instead, by the same `activated` rule the enumerator itself applies.
/// Restoring it therefore brings the chip back.
///
/// Matches the unresolved-target filtering [`super::pov::pov_chips`] already
/// applies to point of view.
pub fn book_chips(table: &[CastCandidate], ids: &[u64]) -> Vec<BookChip> {
    ids.iter()
        .filter_map(|id| {
            table.iter().find(|e| e.id == *id).map(|e| BookChip {
                id: e.id,
                title: e.title.clone(),
            })
        })
        .collect()
}

/// The Books an item is filed under, each removable.
pub fn book_chip_row(chips: Vec<BookChip>, clear: ClearBook) -> impl Widget {
    let mut row = HStack::new().spacing(4.0);
    for chip in chips {
        let clear = clear.clone();
        let id = chip.id;
        row = row
            .child(TextWidget::new(lit!(chip.title.clone())).style(TextStyleRole::Tiny))
            .child(
                IconButton::clear()
                    .embedded()
                    .tooltip(tr!(books_remove(name = chip.title.clone())))
                    .on_activate_fn(move |c| clear(id, c)),
            );
    }
    row
}

/// Trigger button that opens the Book-filing picker.
///
/// `already` is the item's own current `books`, so a Book already declared is
/// hidden from the popover rather than offered a second time.
pub fn book_add_button(
    candidates: Vec<CastCandidate>,
    already: Vec<u64>,
    owner_id: u64,
    set: PinReference,
) -> impl Widget {
    PopoverButton::new(Button::new(tr!(books_add())).variant(ButtonVariant::Plain))
        .content(CastAddPopover::new(candidates, already, owner_id, set))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(id: u64, title: &str) -> CastCandidate {
        CastCandidate {
            id,
            title: title.to_string(),
        }
    }

    #[test]
    fn chips_resolve_ids_against_the_live_book_table() {
        let table = vec![entity(1, "Book One"), entity(2, "Book Two")];
        let chips = book_chips(&table, &[2]);
        assert_eq!(chips.len(), 1);
        assert_eq!(chips[0].title, "Book Two");
    }

    /// A trashed, deleted, or otherwise unresolved Book id must vanish rather
    /// than render as a blank chip: the same unresolved-target rule the
    /// point-of-view chip row applies.
    #[test]
    fn an_unresolvable_id_is_dropped_not_rendered_blank() {
        let table = vec![entity(1, "Book One")];
        assert!(book_chips(&table, &[99]).is_empty());
        assert_eq!(book_chips(&table, &[1, 99]).len(), 1);
    }

    /// Filing under two Books at once is a legal, deliberate state (a note
    /// spanning a series-wide fact is exactly this shape), and the row must
    /// render both.
    #[test]
    fn two_books_both_render() {
        let table = vec![entity(1, "Book One"), entity(2, "Book Two")];
        assert_eq!(book_chips(&table, &[1, 2]).len(), 2);
    }

    #[test]
    fn no_filing_is_an_empty_row_not_an_error() {
        assert!(book_chips(&[entity(1, "Book One")], &[]).is_empty());
    }

    /// Order follows the stored ids, so the row is stable across rebuilds
    /// rather than reshuffling on every repaint.
    #[test]
    fn chip_order_follows_the_stored_ids() {
        let table = vec![entity(1, "Book One"), entity(2, "Book Two")];
        let titles: Vec<String> = book_chips(&table, &[2, 1])
            .into_iter()
            .map(|c| c.title)
            .collect();
        assert_eq!(titles, ["Book Two", "Book One"]);
    }
}
