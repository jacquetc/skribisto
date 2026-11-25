// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A list style's `text:level` is the file's to write, and ODF levels run from 1 to 10.
//!
//! Taken as written, `text:level="0"` indexed the level before the first and panicked, and
//! `text:level="1000000000000"` asked for a table of a trillion levels, which ends the
//! process rather than the import. A level outside 1 to 10 is ignored; the levels the
//! style does describe still say whether a list is numbered.

use document_ingest::block::SourceBlock;
use document_ingest::scanner::ScannerRegistry;

fn scan(fodt: &str) -> document_ingest::block::SourceDocument {
    ScannerRegistry::with_builtin_scanners()
        .scan_bytes(std::path::Path::new("lists.fodt"), fodt.as_bytes())
}

/// A flat ODF document holding one list per `(style name, level-style elements)`, each
/// list holding one item whose text is the style's name.
fn document(styles: &[(&str, &str)]) -> String {
    let list_styles: String = styles
        .iter()
        .map(|(name, levels)| {
            format!(r#"<text:list-style style:name="{name}">{levels}</text:list-style>"#)
        })
        .collect();
    let lists: String = styles
        .iter()
        .map(|(name, _)| {
            format!(
                r#"<text:list text:style-name="{name}"><text:list-item><text:p>Item {name}</text:p></text:list-item></text:list>"#
            )
        })
        .collect();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  office:mimetype="application/vnd.oasis.opendocument.text">
  <office:automatic-styles>{list_styles}</office:automatic-styles>
  <office:body><office:text>{lists}</office:text></office:body>
</office:document>"#
    )
}

/// The Djot of every prose block, joined.
fn djot(doc: &document_ingest::block::SourceDocument) -> String {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The line of `djot` holding the item named `name`.
fn item_line<'a>(djot: &'a str, name: &str) -> &'a str {
    let wanted = format!("Item {name}");
    djot.lines()
        .find(|line| line.trim_end().ends_with(&wanted))
        .unwrap_or_else(|| panic!("no item {name} in {djot:?}"))
}

const NUMBER: &str = r#"<text:list-level-style-number text:level="1" style:num-format="1"/>"#;
const BULLET: &str = r#"<text:list-level-style-bullet text:level="1" text:bullet-char="•"/>"#;

#[test]
fn a_list_level_of_zero_is_ignored() {
    let zero = r#"<text:list-level-style-number text:level="0" style:num-format="1"/>"#;
    let doc = scan(&document(&[
        ("Numbered", NUMBER),
        ("Zero", &format!("{zero}{BULLET}")),
        ("OnlyZero", zero),
    ]));
    let djot = djot(&doc);
    assert!(item_line(&djot, "Numbered").starts_with("1."), "{djot}");
    // Level 1 still says bullet: the `0` changed nothing.
    assert!(item_line(&djot, "Zero").starts_with('-'), "{djot}");
    // A style describing no real level is a plain list.
    assert!(item_line(&djot, "OnlyZero").starts_with('-'), "{djot}");
}

#[test]
fn a_huge_list_level_is_ignored() {
    let huge = r#"<text:list-level-style-number text:level="1000000000000" style:num-format="1"/>"#;
    let eleven = r#"<text:list-level-style-number text:level="11" style:num-format="1"/>"#;
    let doc = scan(&document(&[
        ("Huge", &format!("{BULLET}{huge}")),
        ("Eleven", &format!("{eleven}{BULLET}")),
        ("Numbered", NUMBER),
    ]));
    let djot = djot(&doc);
    assert!(item_line(&djot, "Huge").starts_with('-'), "{djot}");
    assert!(item_line(&djot, "Eleven").starts_with('-'), "{djot}");
    assert!(item_line(&djot, "Numbered").starts_with("1."), "{djot}");
}
