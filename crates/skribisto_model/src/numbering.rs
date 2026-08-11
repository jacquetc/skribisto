// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which ordinal each structural row carries — "this is chapter 3 of book 1".
//!
//! **The number is a fact about the manuscript, not about the selection or the view.**
//! That sentence used to live only in a doc comment on the exporter's scoped-export seed,
//! while the exporter's main loop computed the number a second time, incrementally, over
//! the rows a given export had *already filtered down to*. The two disagreed: mark a
//! chapter non-exportable and "Export Book" renumbered every later chapter, while
//! "Export Chapter 7" — which replayed the unfiltered stream — did not. Same manuscript,
//! same chapter, two numbers.
//!
//! So numbering is one pass over the **whole** ordered stream, here, before anyone
//! filters anything, and both the exporter and the binder's live badge look their answer
//! up by item id. There is no second pass to drift.
//!
//! Store-free and IO-free, like its siblings `analysis` / `mentions` / `counting`: it
//! takes the same flat `ItemMeta` slice [`crate::compile::resolve_scope`] does, which
//! both callers already build ( `skribisto_compiler::item_metas` from the frozen
//! `Gathered` tree, `teksilo_ui`'s `live_item_metas` from the live binder query).
//!
//! # What counts
//!
//! Three filters, and the difference between them is the whole design:
//!
//! * `!activated` — trashed. Not in the book, holds no number.
//! * `!is_exportable` — the writer's persistent per-item "leave this out of the exported
//!   book" toggle. Not in the book either, so it holds no number and the chapters after
//!   it close the gap.
//! * `exclude_from_numbering` — in the book, printed, word-counted, but uncounted: the
//!   prologue. It holds no number *and does not consume one*, so the chapter after a
//!   prologue is chapter 1.
//!
//! Deliberately **not** a filter: the export's own selection. The Choose… dialog's
//! checkboxes and the "Current Chapter" scope pick which rows a given file contains;
//! they must not change what number those rows carry. That is exactly the mistake this
//! module exists to make impossible, so `number_map` never sees the selection at all.

use std::collections::HashMap;

use common::entities::BinderItemSubRole;

use crate::SubRoleExt;
use crate::compile::{ItemMeta, StreamLevel};

/// Manuscript-level numbering policy. Sourced from the `Work` entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NumberingRules {
    /// Whether a new Part restarts chapter numbering at one.
    ///
    /// Defaults **off**, which is the trade convention: chapters run continuously across
    /// the parts of one book, so "Part Two" opens on Chapter Eleven. Chicago's survey of
    /// published fiction finds no per-part restart, and an independent implementation
    /// elsewhere resets scenes at a part and chapters only at a new book — the same
    /// rule. It is a setting rather than a constant because the in-world "Book Two,
    /// Chapter One" framing is real, and because other tools expose the same choice.
    pub part_resets_chapter: bool,
}

/// The ordinal a structural row carries, at every level above it.
///
/// All three are always present because a chapter heading may want its part or book —
/// "Chapter 3" and "II.3" are the same fact formatted differently, and the formatter,
/// not this module, decides which parts of it print.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Numbered {
    /// Which level this row *opens*. The row's own number is `number_at(level)`.
    pub level: StreamLevel,
    pub book: usize,
    pub part: usize,
    pub chapter: usize,
}

impl Numbered {
    /// This row's own ordinal — the one a heading prints.
    pub fn number(&self) -> usize {
        self.number_at(self.level)
    }

    /// The ordinal at an arbitrary enclosing level, for a hierarchical format.
    pub fn number_at(&self, level: StreamLevel) -> usize {
        match level {
            StreamLevel::Book => self.book,
            StreamLevel::Part => self.part,
            StreamLevel::Chapter => self.chapter,
        }
    }
}

/// Which level a `sub_role` opens, or `None` for a row that carries no ordinal.
///
/// Reuses [`SubRoleExt`], the same transition alphabet the compile spine runs on, so a
/// chapter is numbered identically in both encodings (`Item/ChapterScene` and
/// `Folder/ChapterScene`) — `opens_chapter()` is `sub_role`-only and `role`-agnostic.
///
/// Scenes, notes and paratexts return `None`: none of them opens a structural level, so
/// none of them is ever numbered or ever consumes a number.
pub fn level_of(sub_role: &BinderItemSubRole) -> Option<StreamLevel> {
    if sub_role.opens_book() {
        Some(StreamLevel::Book)
    } else if sub_role.opens_part() {
        Some(StreamLevel::Part)
    } else if sub_role.opens_chapter() {
        Some(StreamLevel::Chapter)
    } else {
        None
    }
}

/// Whether this row takes part in numbering at all — see the module docs for why these
/// three and not the export selection.
fn counts(item: &ItemMeta) -> bool {
    item.activated && item.is_exportable && !item.exclude_from_numbering
}

/// The running counters, and the reset rule.
#[derive(Default, Clone, Copy)]
struct Counters {
    book: usize,
    part: usize,
    chapter: usize,
}

impl Counters {
    fn bump(&mut self, level: StreamLevel, rules: NumberingRules) {
        match level {
            // A new book restarts everything beneath it: book two opens with Part One and
            // Chapter One, not with part four and chapter twenty-three.
            StreamLevel::Book => {
                self.book += 1;
                self.part = 0;
                self.chapter = 0;
            }
            StreamLevel::Part => {
                self.part += 1;
                if rules.part_resets_chapter {
                    self.chapter = 0;
                }
            }
            StreamLevel::Chapter => self.chapter += 1,
        }
    }
}

/// Number every structural row of the whole manuscript, keyed by item id.
///
/// `items` must be the **entire** ordered stream — every binder, in binder order, exactly
/// as a save writes it and a load reproduces it. Handing this a scope-filtered slice is
/// the one way to misuse it, and it is what the old two-pass design did by accident.
///
/// A row absent from the returned map holds no number: it is trashed, non-exportable,
/// numbering-excluded, or simply not a structural row (a scene, a note, a paratext).
pub fn number_map(items: &[ItemMeta], rules: NumberingRules) -> HashMap<u64, Numbered> {
    let mut counters = Counters::default();
    let mut out = HashMap::new();
    for item in items {
        let Some(level) = level_of(&item.sub_role) else {
            continue;
        };
        if !counts(item) {
            continue;
        }
        counters.bump(level, rules);
        out.insert(
            item.id,
            Numbered {
                level,
                book: counters.book,
                part: counters.part,
                chapter: counters.chapter,
            },
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole as SR;
    use common::entities::BinderItemRole;

    fn meta(id: u64, role: BinderItemRole, sub_role: SR) -> ItemMeta {
        ItemMeta {
            id,
            role,
            sub_role,
            indent: 0,
            activated: true,
            is_exportable: true,
            exclude_from_numbering: false,
        }
    }

    /// Book, three chapters, scenes between them.
    fn book_of_three() -> Vec<ItemMeta> {
        vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::ChapterScene),
            meta(3, Item, SR::Scene),
            meta(4, Item, SR::ChapterScene),
            meta(5, Item, SR::Scene),
            meta(6, Item, SR::ChapterScene),
            meta(7, Item, SR::BookEnd),
        ]
    }

    fn chapter_of(map: &HashMap<u64, Numbered>, id: u64) -> Option<usize> {
        map.get(&id).map(|n| n.number())
    }

    #[test]
    fn chapters_number_from_one_and_scenes_hold_no_number() {
        let map = number_map(&book_of_three(), NumberingRules::default());
        assert_eq!(chapter_of(&map, 2), Some(1));
        assert_eq!(chapter_of(&map, 4), Some(2));
        assert_eq!(chapter_of(&map, 6), Some(3));
        // Scenes and the book-end marker are not structural openers.
        assert_eq!(map.get(&3), None);
        assert_eq!(map.get(&5), None);
        assert_eq!(map.get(&7), None);
        // The book itself is numbered, at its own level.
        assert_eq!(chapter_of(&map, 1), Some(1));
        assert_eq!(map[&1].level, StreamLevel::Book);
    }

    /// Both chapter encodings are the same thing to the counter — `opens_chapter()` reads
    /// `sub_role` only, so flipping `Work.chapter_mode` cannot renumber a manuscript.
    #[test]
    fn folder_and_flat_chapters_number_identically() {
        let flat = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::ChapterScene),
            meta(3, Item, SR::ChapterScene),
        ];
        let folders = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Folder, SR::ChapterScene),
            meta(3, Folder, SR::ChapterScene),
        ];
        let a = number_map(&flat, NumberingRules::default());
        let b = number_map(&folders, NumberingRules::default());
        assert_eq!(chapter_of(&a, 3), Some(2));
        assert_eq!(chapter_of(&a, 3), chapter_of(&b, 3));
    }

    /// The headline case: a prologue must not take "Chapter 1" from the real first
    /// chapter, and must not print a number of its own.
    #[test]
    fn an_excluded_chapter_neither_holds_nor_consumes_a_number() {
        let mut items = book_of_three();
        items[1].exclude_from_numbering = true; // id 2 is the "prologue"
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(map.get(&2), None, "a prologue prints no number");
        assert_eq!(
            chapter_of(&map, 4),
            Some(1),
            "the chapter after a prologue is chapter one"
        );
        assert_eq!(chapter_of(&map, 6), Some(2));
    }

    /// A new book restarts parts and chapters beneath it.
    #[test]
    fn a_new_book_restarts_parts_and_chapters() {
        let items = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::Part),
            meta(3, Item, SR::ChapterScene),
            meta(4, Item, SR::ChapterScene),
            meta(5, Item, SR::BookEnd),
            meta(6, Item, SR::BookBegin),
            meta(7, Item, SR::Part),
            meta(8, Item, SR::ChapterScene),
        ];
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(chapter_of(&map, 4), Some(2));
        assert_eq!(map[&6].book, 2);
        assert_eq!(chapter_of(&map, 7), Some(1), "part one of book two");
        assert_eq!(chapter_of(&map, 8), Some(1), "chapter one of book two");
    }

    /// The default: chapters run continuously across the parts of one book.
    #[test]
    fn a_part_does_not_reset_chapters_by_default() {
        let items = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::Part),
            meta(3, Item, SR::ChapterScene),
            meta(4, Item, SR::ChapterScene),
            meta(5, Item, SR::Part),
            meta(6, Item, SR::ChapterScene),
        ];
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(
            chapter_of(&map, 6),
            Some(3),
            "part two opens on chapter three"
        );
        assert_eq!(chapter_of(&map, 5), Some(2), "…which is part two");
    }

    /// …and the opt-in restarts them, without disturbing the part counter.
    #[test]
    fn a_part_resets_chapters_when_the_work_asks() {
        let items = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::Part),
            meta(3, Item, SR::ChapterScene),
            meta(4, Item, SR::ChapterScene),
            meta(5, Item, SR::Part),
            meta(6, Item, SR::ChapterScene),
        ];
        let rules = NumberingRules {
            part_resets_chapter: true,
        };
        let map = number_map(&items, rules);
        assert_eq!(chapter_of(&map, 4), Some(2));
        assert_eq!(
            chapter_of(&map, 6),
            Some(1),
            "part two opens on chapter one"
        );
        assert_eq!(
            chapter_of(&map, 5),
            Some(2),
            "the part counter still runs on"
        );
    }

    /// A trashed row is not in the book, so it neither holds nor consumes a number.
    #[test]
    fn a_trashed_chapter_is_not_numbered() {
        let mut items = book_of_three();
        items[3].activated = false; // id 4
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(map.get(&4), None);
        assert_eq!(chapter_of(&map, 6), Some(2));
    }

    /// Same for a chapter the writer has taken out of the exported book — and this is the
    /// half the old two-pass design got wrong in one direction and right in the other.
    #[test]
    fn a_non_exportable_chapter_is_not_numbered_and_leaves_no_gap() {
        let mut items = book_of_three();
        items[3].is_exportable = false; // id 4
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(map.get(&4), None);
        assert_eq!(
            chapter_of(&map, 6),
            Some(2),
            "numbering closes the gap rather than skipping a numeral"
        );
    }

    /// The regression this module exists for: the answer for one chapter must not depend
    /// on which slice of the manuscript an export happens to contain. Numbering the whole
    /// stream and numbering it again after the caller has filtered must agree — which is
    /// only true because the caller *cannot* filter first: it looks up by id.
    #[test]
    fn a_chapters_number_does_not_depend_on_the_export_selection() {
        let mut items = book_of_three();
        items[3].is_exportable = false;
        let full = number_map(&items, NumberingRules::default());

        // What a scoped export sees: only the last chapter's id survives selection. The
        // map is still built from the whole manuscript, so the answer is unchanged.
        let scoped = number_map(&items, NumberingRules::default());
        assert_eq!(chapter_of(&full, 6), chapter_of(&scoped, 6));

        // And had the map been built from the selection — the old bug — it would have
        // said "chapter 1" instead. Pin that this is genuinely a different answer, so the
        // test fails loudly if someone reintroduces a filtered pass.
        let only_last: Vec<ItemMeta> = items.iter().filter(|i| i.id == 6).cloned().collect();
        let wrong = number_map(&only_last, NumberingRules::default());
        assert_eq!(chapter_of(&wrong, 6), Some(1));
        assert_ne!(chapter_of(&full, 6), chapter_of(&wrong, 6));
    }

    /// An excluded *part* stops consuming part numbers without touching the chapters
    /// running through it.
    #[test]
    fn an_excluded_part_leaves_chapter_numbering_alone() {
        let mut items = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::Part),
            meta(3, Item, SR::ChapterScene),
            meta(4, Item, SR::Part),
            meta(5, Item, SR::ChapterScene),
        ];
        items[3].exclude_from_numbering = true; // the second Part
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(map.get(&4), None);
        assert_eq!(chapter_of(&map, 5), Some(2), "chapters run on regardless");
    }

    /// Numbering runs binder-major over the whole Work, so a chapter parked in a Notes or
    /// Research binder *does* take part. Pinned because it is surprising, and because the
    /// fix if it is ever unwanted belongs in what the caller passes, not in here.
    #[test]
    fn every_binder_shares_one_numbering_stream() {
        let items = vec![
            meta(1, Item, SR::BookBegin),
            meta(2, Item, SR::ChapterScene),
            // …a second binder's rows, concatenated by the caller:
            meta(3, Item, SR::ChapterScene),
        ];
        let map = number_map(&items, NumberingRules::default());
        assert_eq!(chapter_of(&map, 3), Some(2));
    }

    /// An empty manuscript is not an error.
    #[test]
    fn an_empty_stream_numbers_nothing() {
        assert!(number_map(&[], NumberingRules::default()).is_empty());
    }
}
