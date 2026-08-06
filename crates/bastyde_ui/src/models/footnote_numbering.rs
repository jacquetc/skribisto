// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading the open Work's **footnote** numbering: where each note's reference sits
//! in the manuscript, and therefore what its marker prints.
//!
//! The sibling of [`crate::models::numbering`], and it divides the work the same
//! way. The counting itself is
//! [`skribisto_model::footnote_numbering::number_map`] — the one function the
//! exporter also calls — and this module only *feeds* it. That split is the whole
//! point of the feature: a note's number is a fact about the manuscript, so the
//! badge a writer sees while typing and the number printed in the finished book
//! have to fall out of the same pass over the same stream. Two passes drift, and
//! the drift is invisible until somebody reads the exported PDF.
//!
//! # Reading in the exporter's order, deliberately
//!
//! [`read_work`] walks binders in `Work.binders` order, items in each binder's
//! `BinderItems` relationship order, and an item's prose rows in its `Contents`
//! relationship order — because that is exactly what `skrib_format::tree_read`
//! hands the compiler, and the numbers only agree if the two walks agree. A
//! `get_content_multi` returns rows in db-key order rather than request order, so
//! the ids are re-indexed and the relationship order walked; taking the multi-get's
//! order instead would number a scene's synopsis before its prose on one run and
//! after it on the next, with nothing to show for it but a badge that moved.
//!
//! # The live overlay
//!
//! [`read_work`] prefers an **open document's** current text to the `Content` row
//! behind it. A reference inserted a second ago is not in the store yet — it lands
//! there on the next flush — so numbering from the store alone would leave the
//! marker the writer just made unnumbered until autosave caught up, then renumber
//! the chapter under them when it did.
//!
//! # What the editor cannot know
//!
//! The restart rule ([`FootnoteRestart`]) belongs to an **export preset**, and a
//! project may hold several with different rules. The editor has to show one
//! number, so it shows [`FootnoteRestart::Continuous`] — the manuscript's own
//! end-to-end count, which is also [`NotePlacement::ordinal`]'s order. A preset
//! that restarts per chapter renumbers at export; that is a typesetting decision
//! made in the export dialog, and the badge does not guess which preset a writer
//! will reach for.

use std::collections::HashMap;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, content_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use skribisto_model::compile::ItemMeta;
use skribisto_model::footnote_numbering::{self, FootnoteRestart};

use crate::models::OpenDocsStore;

/// One manuscript row as the footnote pass reads it.
#[derive(Clone, Debug)]
pub struct ItemProse {
    pub meta: ItemMeta,
    pub title: String,
    /// `(content_id, prose)` for every activated row that carries a reference at
    /// all. Reference-free prose is dropped by [`read_work`] rather than passed on:
    /// [`footnote_numbering::references_in`] scans the text once **per label**, so
    /// handing it the 99% of scenes with no notes in them is the only cost this
    /// pass has, and dropping them cannot change an answer.
    pub contents: Vec<(u64, String)>,
}

/// Where one note lives, and what its marker prints.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotePlacement {
    pub item_id: u64,
    pub item_title: String,
    pub content_id: u64,
    /// What the marker prints — `None` for a note whose row is **not in the book**
    /// (trashed, or not exportable). Such a row is still openable and editable, so
    /// its references are still shown; they simply have no number, because the
    /// sequence they would belong to closes the gap behind them. See
    /// [`marker_for`] for what is drawn instead.
    pub number: Option<usize>,
    /// Position in the whole manuscript, counting rows that carry no number. The
    /// dock's sort key: it is defined for every placed note, and it does not move
    /// when a chapter is excluded from an export.
    pub ordinal: usize,
}

/// Every note's place in the manuscript, plus the ones nothing points at.
#[derive(Clone, Debug, Default)]
pub struct FootnotePlaces {
    pub placed: HashMap<String, NotePlacement>,
    /// Labels no prose in the project references, sorted. Reported, never repaired
    /// — see [`footnote_numbering::orphaned_labels`].
    pub orphans: Vec<String>,
}

/// What a reference draws for a note that has no number.
///
/// A bullet rather than the label, and rather than nothing. The label
/// (`fn7`) is machinery the writer never typed and should never read; an empty
/// marker is indistinguishable from a rendering fault. The bullet says "a note is
/// attached here" without inventing a number that the book will never print —
/// which is the honest thing to say about a row that is not in the book.
pub const UNNUMBERED_MARKER: &str = "\u{2022}";

/// What a reference draws, given the number its note carries.
///
/// The one definition of that mapping: the dock's chip and the map pushed into
/// every open document both go through here, so a note cannot read as `12` in one
/// and as a bullet in the other.
pub fn marker_for(number: Option<usize>) -> String {
    match number {
        Some(n) => n.to_string(),
        None => UNNUMBERED_MARKER.to_string(),
    }
}

/// Read the whole Work in the exporter's own order (see the module docs).
///
/// `docs` supplies the live overlay; pass `None` where there are no open documents
/// to prefer (a headless read, or a caller that only wants what is on disk).
/// Empty on any backend hiccup — a numbering pass that cannot read the tree must
/// not invent an order.
pub fn read_work(ctx: &AppContext, work_id: u64, docs: Option<&OpenDocsStore>) -> Vec<ItemProse> {
    let mut out = Vec::new();
    let binder_ids =
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default();
    for binder_id in binder_ids {
        let item_ids = binder_commands::get_binder_relationship(
            ctx,
            &binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        let by_id: HashMap<u64, frontend::direct_access::BinderItemDto> =
            binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|it| (it.id, it))
                .collect();
        for id in item_ids {
            let Some(it) = by_id.get(&id) else {
                continue;
            };
            out.push(ItemProse {
                meta: crate::models::item_meta_of(it),
                title: it.title.clone(),
                contents: prose_of_item(ctx, id, docs),
            });
        }
    }
    out
}

/// One item's reference-carrying prose rows, in `Contents` relationship order,
/// with any open document's live text preferred over the stored row.
fn prose_of_item(
    ctx: &AppContext,
    item_id: u64,
    docs: Option<&OpenDocsStore>,
) -> Vec<(u64, String)> {
    let content_ids = binder_item_commands::get_binder_item_relationship(
        ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap_or_default();
    // Re-indexed by id, then walked in relationship order — `get_content_multi`
    // answers in db-key order (see the module docs on why that matters here).
    let stored: HashMap<u64, frontend::direct_access::ContentDto> =
        content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|c| (c.id, c))
            .collect();
    let live = docs.and_then(|d| d.peek(item_id)).map(|d| live_prose(&d));
    let mut out = Vec::new();
    for cid in content_ids {
        let Some(c) = stored.get(&cid) else {
            continue;
        };
        if !c.activated {
            continue;
        }
        let data = live
            .as_ref()
            .and_then(|m| m.get(&cid).cloned())
            .unwrap_or_else(|| c.data.clone());
        if data.contains("[^") {
            out.push((cid, data));
        }
    }
    out
}

/// An open document's current prose, keyed by the `Content` row behind each field.
///
/// Serialising back to Djot is what makes the reference searchable at all: the
/// live document holds it as an inline object, and `[^label]` is how it reads on
/// the way out. Cheap enough at this cadence — the caller only re-reads when a
/// reference has actually appeared or vanished, never per keystroke.
fn live_prose(doc: &crate::models::OpenDoc) -> HashMap<u64, String> {
    let mut out = HashMap::new();
    for field in [&doc.main, &doc.synopsis, &doc.epigraph]
        .into_iter()
        .flatten()
    {
        if let (Some(id), Ok(djot)) = (field.content_id(), field.doc.to_djot()) {
            out.insert(id, djot);
        }
    }
    out
}

/// Number every note and place it, from a read of the whole manuscript.
///
/// Pure: the reading happened in [`read_work`], so this half is unit-testable
/// without a backend — which is where the behaviour worth pinning lives.
pub fn places(rows: &[ItemProse], labels: &[String]) -> FootnotePlaces {
    let metas: Vec<ItemMeta> = rows.iter().map(|r| r.meta.clone()).collect();
    let prose_by_item: HashMap<u64, Vec<&str>> = rows
        .iter()
        .map(|r| {
            (
                r.meta.id,
                r.contents.iter().map(|(_, d)| d.as_str()).collect(),
            )
        })
        .collect();

    let numbered = footnote_numbering::number_map(
        &metas,
        labels,
        |id| prose_by_item.get(&id).cloned().unwrap_or_default(),
        // See the module docs: the editor shows the manuscript's own count.
        FootnoteRestart::Continuous,
    );

    // A second walk in the same order, for the two things `number_map` does not
    // answer: which `Content` row a note's reference sits in (the dock navigates
    // there), and a position for the notes it declines to number.
    let mut placed: HashMap<String, NotePlacement> = HashMap::new();
    let mut ordinal = 0usize;
    for row in rows {
        for (content_id, data) in &row.contents {
            for (_, label) in footnote_numbering::references_in(data, labels) {
                if placed.contains_key(&label) {
                    // One note cited twice keeps its first home, exactly as it
                    // keeps its first number.
                    continue;
                }
                ordinal += 1;
                placed.insert(
                    label.clone(),
                    NotePlacement {
                        item_id: row.meta.id,
                        item_title: row.title.clone(),
                        content_id: *content_id,
                        number: numbered.get(&(row.meta.id, label)).map(|n| n.number),
                        ordinal,
                    },
                );
            }
        }
    }

    let orphans = footnote_numbering::orphaned_labels(&metas, labels, |id| {
        prose_by_item.get(&id).cloned().unwrap_or_default()
    });

    FootnotePlaces { placed, orphans }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    fn row(id: u64, title: &str, contents: &[(u64, &str)]) -> ItemProse {
        ItemProse {
            meta: ItemMeta {
                id,
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                indent: 0,
                activated: true,
                is_exportable: true,
                exclude_from_numbering: false,
            },
            title: title.to_string(),
            contents: contents
                .iter()
                .map(|(cid, d)| (*cid, d.to_string()))
                .collect(),
        }
    }

    fn labels(ls: &[&str]) -> Vec<String> {
        ls.iter().map(|s| s.to_string()).collect()
    }

    /// The badge is the number the book prints, in manuscript order.
    #[test]
    fn notes_are_numbered_in_manuscript_order() {
        let rows = vec![
            row(1, "One", &[(11, "opening[^a] and[^b]")]),
            row(2, "Two", &[(21, "later[^c]")]),
        ];
        let p = places(&rows, &labels(&["a", "b", "c"]));
        assert_eq!(p.placed["a"].number, Some(1));
        assert_eq!(p.placed["b"].number, Some(2));
        assert_eq!(p.placed["c"].number, Some(3));
        assert!(p.orphans.is_empty());
    }

    /// The dock navigates to the row *and the `Content` row* the reference sits
    /// in — a scene's synopsis is a different document from its prose, and landing
    /// in the wrong one puts the caret nowhere near the marker.
    #[test]
    fn a_placement_names_the_content_row_its_reference_sits_in() {
        let rows = vec![row(7, "Scene", &[(70, "prose[^a]"), (71, "synopsis[^b]")])];
        let p = places(&rows, &labels(&["a", "b"]));
        assert_eq!(p.placed["a"].content_id, 70);
        assert_eq!(p.placed["b"].content_id, 71);
        assert_eq!(p.placed["b"].item_id, 7);
        assert_eq!(p.placed["b"].item_title, "Scene");
    }

    /// A row that is not in the book still shows its notes — it is openable and
    /// editable — but they carry no number, because the sequence closes the gap
    /// behind them. They keep an ordinal, so the dock can still order them.
    #[test]
    fn a_note_outside_the_book_is_placed_but_unnumbered() {
        let mut excluded = row(2, "Cut", &[(21, "dropped[^b]")]);
        excluded.meta.is_exportable = false;
        let rows = vec![
            row(1, "Kept", &[(11, "kept[^a]")]),
            excluded,
            row(3, "After", &[(31, "after[^c]")]),
        ];
        let p = places(&rows, &labels(&["a", "b", "c"]));
        assert_eq!(p.placed["b"].number, None, "not in the book, so no number");
        assert_eq!(marker_for(p.placed["b"].number), UNNUMBERED_MARKER);
        assert_eq!(p.placed["c"].number, Some(2), "the gap closes");
        assert_eq!(
            p.placed["b"].ordinal, 2,
            "but it keeps its place in the dock"
        );
        assert!(
            p.orphans.is_empty(),
            "a reference in an excluded row is still a reference"
        );
    }

    /// One note cited twice keeps one number and one home.
    #[test]
    fn a_note_cited_twice_keeps_its_first_home() {
        let rows = vec![
            row(1, "One", &[(11, "here[^a] and again[^a]")]),
            row(2, "Two", &[(21, "third time[^a]")]),
        ];
        let p = places(&rows, &labels(&["a"]));
        assert_eq!(p.placed.len(), 1);
        assert_eq!(p.placed["a"].item_id, 1);
        assert_eq!(p.placed["a"].content_id, 11);
        assert_eq!(p.placed["a"].number, Some(1));
    }

    /// A note nothing points at is reported, and has no placement to navigate to.
    #[test]
    fn a_note_nothing_references_is_orphaned_and_unplaced() {
        let rows = vec![row(1, "One", &[(11, "still here[^kept]")])];
        let p = places(&rows, &labels(&["kept", "lost"]));
        assert_eq!(p.orphans, vec!["lost".to_string()]);
        assert!(!p.placed.contains_key("lost"));
        assert!(p.placed.contains_key("kept"));
    }

    /// The bracket on both sides is what keeps `fn1` out of `fn10` — the property
    /// the minting relies on, pinned here too because the dock's whole ordering
    /// rests on it.
    #[test]
    fn a_short_label_does_not_match_inside_a_longer_one() {
        let rows = vec![row(1, "One", &[(11, "only[^fn10] here")])];
        let p = places(&rows, &labels(&["fn1", "fn10"]));
        assert!(p.placed.contains_key("fn10"));
        assert_eq!(p.orphans, vec!["fn1".to_string()]);
    }
}
