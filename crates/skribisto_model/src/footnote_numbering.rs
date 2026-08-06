// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which number each footnote prints — "this is note 12".
//!
//! The sibling of [`crate::numbering`], and it exists for the same reason and takes
//! the same shape: **the number is a fact about the manuscript, not about the
//! selection or the view.** Export chapter five on its own and its notes must be
//! numbered as the whole book numbers them, because that is the number the writer
//! sees in the editor and the number a reader would find in the finished book. A
//! per-export counter would have said 1, 2, 3 — and would have disagreed with the
//! badge in the editor at the same moment, for the same note.
//!
//! So this is one pass over the **whole** ordered stream, before anyone filters
//! anything, and both the exporter and the editor's live marker look their answer
//! up by `(item, label)`. There is no second pass to drift.
//!
//! Store-free and IO-free, like its siblings. It takes the same flat
//! [`ItemMeta`](crate::compile::ItemMeta)
//! slice its neighbours do, plus each row's prose — because where a note sits in
//! the book is decided by where its **reference** sits in the text, not by anything
//! stored on the note.
//!
//! # Finding the references
//!
//! By plain substring search for `[^label]`, deliberately, rather than by parsing
//! the Djot. Two reasons:
//!
//! * It keeps this module in the store-free, dependency-free family its siblings
//!   are in — no document model, no parser, no allocation per scene beyond the
//!   answer.
//! * The label is **minted by Skribisto**, never typed by the writer, so there is no
//!   ambiguity to resolve. `[^` and `]` bracket it on both sides, so `[^1]` cannot
//!   match inside `[^10]` — the trailing bracket is what makes the naive search
//!   exact.

use std::collections::HashMap;

use crate::SubRoleExt;
use crate::compile::ItemMeta;

/// Where a note's number restarts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FootnoteRestart {
    /// One run of numbers through the whole book. Common in non-fiction, where a
    /// reader may cite "note 214" and expect it to be findable.
    #[default]
    Continuous,
    /// Numbers restart at each chapter — common in fiction, and what keeps a long
    /// novel's markers from reaching four digits.
    PerChapter,
    /// Restart at each book, for a manuscript holding several.
    PerBook,
}

/// One footnote's place in the manuscript.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberedNote {
    /// What the marker prints.
    pub number: usize,
    /// Position in the whole manuscript, ignoring restarts — the order an endnote
    /// list is written in, and a stable sort key that survives a change of restart
    /// rule.
    pub ordinal: usize,
}

/// Which rows carry notes that count.
///
/// The same three filters [`crate::numbering`] applies to chapters, and for the same
/// reasons: a trashed row is not in the book, a non-exportable row is not in the
/// book, and both close the gap behind them rather than leaving a hole in the
/// sequence.
///
/// `exclude_from_numbering` is deliberately **not** among them. It means "this row's
/// own heading takes no chapter number" — a prologue — and says nothing about
/// whether the prose inside it may carry a note. A footnote in a prologue is still
/// a footnote in the book.
fn counts(item: &ItemMeta) -> bool {
    item.activated && item.is_exportable
}

/// Every reference in `prose`, in the order they are read, as `(byte_offset, label)`.
///
/// Public because the same walk answers "which notes does this scene reference, and
/// in what order" for the editor's dock, which needs it without wanting numbers.
pub fn references_in(prose: &str, labels: &[String]) -> Vec<(usize, String)> {
    let mut found: Vec<(usize, String)> = Vec::new();
    for label in labels {
        let needle = format!("[^{label}]");
        let mut from = 0usize;
        while let Some(rel) = prose[from..].find(&needle) {
            let at = from + rel;
            found.push((at, label.clone()));
            from = at + needle.len();
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    found
}

/// Number every footnote in the manuscript.
///
/// `prose_of` yields each item's prose rows — a scene has one, but a row can carry
/// several (a synopsis, a note body), and a reference in any of them is a reference
/// in the book. `labels` is every label the project knows, which is what lets the
/// search be exact rather than a scan for `[^…]`-shaped text.
///
/// Keyed by `(item_id, label)`: the same note referenced from two items would be two
/// markers, and the key has to tell them apart.
pub fn number_map<'a, F>(
    items: &[ItemMeta],
    labels: &[String],
    mut prose_of: F,
    restart: FootnoteRestart,
) -> HashMap<(u64, String), NumberedNote>
where
    F: FnMut(u64) -> Vec<&'a str>,
{
    let mut out: HashMap<(u64, String), NumberedNote> = HashMap::new();
    let mut counter = 0usize;
    let mut ordinal = 0usize;

    for item in items {
        // A structural row restarts the count before its own prose is walked, so a
        // note in a chapter's own opening paragraph is note 1 of that chapter.
        let restarts_here = match restart {
            FootnoteRestart::Continuous => false,
            FootnoteRestart::PerBook => item.sub_role.opens_book(),
            // A new book opens a new chapter too, so per-chapter restarts there as
            // well — otherwise book two's first chapter would continue book one's
            // run, which is the one arrangement nobody asks for.
            FootnoteRestart::PerChapter => {
                item.sub_role.opens_chapter() || item.sub_role.opens_book()
            }
        };
        if restarts_here {
            counter = 0;
        }

        if !counts(item) {
            continue;
        }
        for prose in prose_of(item.id) {
            for (_, label) in references_in(prose, labels) {
                let key = (item.id, label);
                // A label referenced twice in one row keeps one number: it is one
                // note, cited twice, exactly as it would be in print.
                if out.contains_key(&key) {
                    continue;
                }
                counter += 1;
                ordinal += 1;
                out.insert(
                    key,
                    NumberedNote {
                        number: counter,
                        ordinal,
                    },
                );
            }
        }
    }

    out
}

/// Notes the manuscript no longer references.
///
/// A note is orphaned when nothing in the prose names it — its reference was
/// deleted along with the sentence that carried it, or its annotated row was
/// purged. The note's *words* survive (they are the writer's, and the format keeps
/// them), but nothing in the book points at them any more.
///
/// Reported rather than repaired: putting the reference back would mean guessing
/// where the sentence went, and a footnote attached to the wrong sentence is worse
/// than one the writer is told about. This is the number an export preflight shows.
pub fn orphaned_labels<'a, F>(items: &[ItemMeta], labels: &[String], mut prose_of: F) -> Vec<String>
where
    F: FnMut(u64) -> Vec<&'a str>,
{
    let mut referenced: std::collections::HashSet<String> = std::collections::HashSet::new();
    for item in items {
        // Deliberately not filtered by `counts`: a reference in a trashed or
        // non-exportable row still means the writer has not lost track of the note.
        // Calling it orphaned would send them hunting for a reference that is right
        // there, in a chapter they chose to leave out.
        for prose in prose_of(item.id) {
            for (_, label) in references_in(prose, labels) {
                referenced.insert(label);
            }
        }
    }
    let mut out: Vec<String> = labels
        .iter()
        .filter(|l| !referenced.contains(*l))
        .cloned()
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::{BinderItemRole, BinderItemSubRole};

    fn meta(id: u64, sub_role: BinderItemSubRole) -> ItemMeta {
        ItemMeta {
            id,
            role: BinderItemRole::Item,
            sub_role,
            indent: 0,
            activated: true,
            is_exportable: true,
            exclude_from_numbering: false,
        }
    }

    #[test]
    fn references_come_back_in_reading_order() {
        let labels = vec!["b".to_string(), "a".to_string()];
        let found = references_in("one[^a] two[^b] three", &labels);
        assert_eq!(
            found.iter().map(|(_, l)| l.as_str()).collect::<Vec<_>>(),
            vec!["a", "b"],
            "order is where they sit, not the order labels were given"
        );
    }

    /// The bracket on both sides is what makes the naive search exact.
    #[test]
    fn a_short_label_does_not_match_inside_a_longer_one() {
        let labels = vec!["1".to_string(), "10".to_string()];
        let found = references_in("only[^10] here", &labels);
        assert_eq!(found.len(), 1, "matched {found:?}");
        assert_eq!(found[0].1, "10");
    }

    #[test]
    fn notes_are_numbered_in_manuscript_order() {
        let items = vec![
            meta(1, BinderItemSubRole::Scene),
            meta(2, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                1 => vec!["first[^a] and[^b]"],
                2 => vec!["second[^c]"],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        assert_eq!(map[&(1, "a".into())].number, 1);
        assert_eq!(map[&(1, "b".into())].number, 2);
        assert_eq!(map[&(2, "c".into())].number, 3);
    }

    /// **The regression class `ef2a98a0` fixed for chapters.** The number must not
    /// depend on what a given export happens to include — `number_map` never sees a
    /// selection, so exporting the second scene alone still calls its note 2.
    #[test]
    fn a_notes_number_does_not_depend_on_the_export_selection() {
        let items = vec![
            meta(1, BinderItemSubRole::Scene),
            meta(2, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                1 => vec!["first[^a]"],
                2 => vec!["second[^b]"],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        assert_eq!(
            map[&(2, "b".into())].number,
            2,
            "the second scene's note is note 2 of the book, whatever is exported"
        );
    }

    /// A trashed or excluded row's notes hold no number, and the sequence closes the
    /// gap behind them rather than leaving a hole.
    #[test]
    fn an_excluded_row_holds_no_numbers_and_leaves_no_gap() {
        let mut skipped = meta(2, BinderItemSubRole::Scene);
        skipped.is_exportable = false;
        let items = vec![
            meta(1, BinderItemSubRole::Scene),
            skipped,
            meta(3, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                1 => vec!["one[^a]"],
                2 => vec!["skipped[^b]"],
                3 => vec!["three[^c]"],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        assert!(!map.contains_key(&(2, "b".into())), "excluded row numbered");
        assert_eq!(map[&(3, "c".into())].number, 2, "the gap must close");
    }

    /// One note cited twice keeps one number.
    #[test]
    fn a_note_referenced_twice_keeps_one_number() {
        let items = vec![meta(1, BinderItemSubRole::Scene)];
        let labels = vec!["a".to_string()];
        let map = number_map(
            &items,
            &labels,
            |_| vec!["here[^a] and again[^a]"],
            FootnoteRestart::Continuous,
        );
        assert_eq!(map.len(), 1);
        assert_eq!(map[&(1, "a".into())].number, 1);
    }

    #[test]
    fn a_note_nothing_references_is_reported_orphaned() {
        let items = vec![meta(1, BinderItemSubRole::Scene)];
        let labels = vec!["kept".to_string(), "lost".to_string()];
        let orphans = orphaned_labels(&items, &labels, |_| vec!["still here[^kept]"]);
        assert_eq!(orphans, vec!["lost".to_string()]);
    }

    /// A reference in a chapter the writer excluded from export is still a
    /// reference. Calling that note orphaned would send them hunting for something
    /// that is exactly where they left it.
    #[test]
    fn a_reference_in_an_excluded_row_still_counts_as_referenced() {
        let mut excluded = meta(1, BinderItemSubRole::Scene);
        excluded.is_exportable = false;
        let items = vec![excluded];
        let labels = vec!["a".to_string()];
        let orphans = orphaned_labels(&items, &labels, |_| vec!["here[^a]"]);
        assert!(orphans.is_empty(), "reported {orphans:?}");
    }

    /// `ordinal` ignores restarts, so an endnote list stays in manuscript order even
    /// when the printed markers repeat.
    #[test]
    fn the_ordinal_survives_a_restart() {
        let items = vec![
            meta(1, BinderItemSubRole::Book),
            meta(2, BinderItemSubRole::Scene),
            meta(3, BinderItemSubRole::Book),
            meta(4, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                2 => vec!["one[^a]"],
                4 => vec!["two[^b]"],
                _ => vec![],
            },
            FootnoteRestart::PerBook,
        );
        assert_eq!(map[&(2, "a".into())].number, 1);
        assert_eq!(map[&(4, "b".into())].number, 1, "the second book restarts");
        assert_eq!(map[&(2, "a".into())].ordinal, 1);
        assert_eq!(map[&(4, "b".into())].ordinal, 2, "the ordinal does not");
    }
}
