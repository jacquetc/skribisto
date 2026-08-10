// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The *capture* half of the table-anchor story: a selection offset from the editor
//! widget, applied to the document string a comment snapshot holds, must slice back to
//! the words the writer selected.
//!
//! `comments::binding::snapshot` used to pair `to_plain_text()` (the export, no table
//! anchor) with widget selection offsets (the document's own cursor space, anchor
//! counted). On a row holding a table, a comment made on prose after the table captured
//! its quote from the wrong offset — `"salt-bleached"` was stored as `"lt-bleached d"`.
//! Export rebasing was immune, because resolve searches by quote text; the capture was
//! not.
//!
//! `snapshot` now reads `to_addressable_text()`, the string that actually shares the
//! widget's offset space. These tests pin that pairing, table and no table.

use text_document::{FindOptions, TextDocument};

/// The document's own offset for `needle` — the space a widget selection reports in.
fn document_offset(doc: &TextDocument, needle: &str) -> usize {
    doc.find_all(needle, &FindOptions::default())
        .expect("find")
        .first()
        .expect("the needle must be in the document")
        .position
}

/// What `comments::binding::snapshot` hands the anchor capture, sliced at the widget's
/// own selection offset, must be the selected words — with a table earlier in the row.
#[test]
fn a_selection_offset_after_a_table_indexes_the_snapshot_text() {
    let djot = "intro\n\n| a | b |\n| - | - |\n| c | d |\n\nthe salt-bleached door";
    let doc = TextDocument::new();
    doc.set_djot(djot).expect("set").wait().expect("wait");

    // What the editor widget would report for a selection of "salt-bleached".
    let selection_start = document_offset(&doc, "salt-bleached");

    // What the fixed `snapshot()` holds: the addressable text, anchors counted.
    let snapshot = doc.to_addressable_text().expect("addressable");
    let chars: Vec<char> = snapshot.chars().collect();

    let captured: String = chars
        .iter()
        .skip(selection_start)
        .take("salt-bleached".chars().count())
        .collect();

    assert_eq!(
        captured, "salt-bleached",
        "the quote captured at the widget's own selection offset must be the selected \
         words.\n  djot            = {djot:?}\n  snapshot text   = {snapshot:?}\n  \
         selection start = {selection_start}\n  captured        = {captured:?}\n\
         A mismatch here means every comment made after a table in the same row stores a \
         quote sliced from the wrong place."
    );
}

/// The same question without a table, to prove the fixture is otherwise sound — if this
/// fails too, the test is measuring its own mistake rather than the product's.
#[test]
fn a_selection_offset_with_no_table_indexes_the_snapshot_text() {
    let djot = "intro\n\nthe salt-bleached door";
    let doc = TextDocument::new();
    doc.set_djot(djot).expect("set").wait().expect("wait");

    let selection_start = document_offset(&doc, "salt-bleached");
    let snapshot = doc.to_addressable_text().expect("addressable");
    let chars: Vec<char> = snapshot.chars().collect();
    let captured: String = chars
        .iter()
        .skip(selection_start)
        .take("salt-bleached".chars().count())
        .collect();

    assert_eq!(
        captured, "salt-bleached",
        "control: with no table the export and the addressable text coincide"
    );
}
