// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Documents nested exactly to the XML ceiling, and one level past it.
//!
//! `roxmltree` recurses once per element and `docx-rs` once per table nested in a
//! table cell, and a stack overflow aborts the process rather than unwinding. Every
//! document here is generated in the test and scanned on a thread with the 2 MiB
//! stack a long operation gets, in the debug build the test suite runs in, which
//! is where the margin is thinnest.
//!
//! At the ceiling the document is read, through every recursive walk its scanner
//! has, which is the measurement behind `skrib_format::MAX_XML_DEPTH`. One level
//! past it the file is refused with `ImportDiagnostic::NestedTooDeep`, naming the
//! part, and the other files of the batch are unaffected.

use std::io::Write;
use std::path::Path;

use document_ingest::{ImportDiagnostic, ScannerRegistry, SourceBlock, SourceDocument};
use skrib_format::MAX_XML_DEPTH;

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 * 1024 * 1024;

/// Scan `bytes` as the file `name` on a thread with a long operation's stack. An
/// overflow would abort the test binary, not fail the test, which is exactly the
/// failure this file exists to rule out.
fn scan_on_a_long_operation_stack(name: &'static str, bytes: Vec<u8>) -> SourceDocument {
    std::thread::Builder::new()
        .stack_size(LONG_OPERATION_STACK)
        .spawn(move || ScannerRegistry::with_builtin_scanners().scan_bytes(Path::new(name), &bytes))
        .expect("spawn the scan thread")
        .join()
        .expect("the scan must not unwind")
}

fn zip(members: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut out);
    let options = zip::write::SimpleFileOptions::default();
    for (name, bytes) in members {
        writer.start_file(*name, options).expect("member");
        writer.write_all(bytes).expect("member bytes");
    }
    writer.finish().expect("finish");
    out.into_inner()
}

fn prose(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .map(SourceBlock::plain_text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Read, with nothing in it refused or unreadable. `shape` names the case.
fn assert_read(shape: &str, doc: &SourceDocument, words: &str) {
    for diagnostic in &doc.diagnostics {
        assert!(
            !matches!(
                diagnostic,
                ImportDiagnostic::NestedTooDeep { .. } | ImportDiagnostic::FileUnreadable { .. }
            ),
            "{shape}: at the ceiling the document must be read, got {diagnostic}"
        );
    }
    assert!(
        prose(doc).contains(words),
        "{shape}: the words must arrive: {:?}",
        prose(doc)
    );
}

/// Refused as nested too deep, naming `part`, and nothing else read. `shape`
/// names the case.
fn assert_refused(shape: &str, doc: &SourceDocument, part: &str) {
    match doc.diagnostics.as_slice() {
        [
            ImportDiagnostic::NestedTooDeep {
                part: refused,
                limit,
                ..
            },
        ] => {
            assert_eq!(refused, part, "{shape}");
            assert_eq!(*limit, MAX_XML_DEPTH, "{shape}");
        }
        other => panic!("{shape}: expected one nested-too-deep refusal of {part}, got {other:?}"),
    }
    assert!(
        doc.blocks.is_empty(),
        "{shape}: a refused file contributes nothing"
    );
}

// ---------------------------------------------------------------------------
// OpenDocument
// ---------------------------------------------------------------------------

const OFFICE: &str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
const TEXT: &str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";

/// A flat `.fodt` whose `<office:text>` holds `body`. `<office:document>` is the
/// first level, so `body`'s own elements start at the fourth.
fn fodt(body: &str) -> Vec<u8> {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><office:document xmlns:office=\"{OFFICE}\" \
         xmlns:text=\"{TEXT}\" office:version=\"1.3\"><office:body><office:text>{body}\
         </office:text></office:body></office:document>"
    )
    .into_bytes()
}

/// The same body as a zipped `.odt`'s `content.xml`.
fn content_xml(body: &str) -> Vec<u8> {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><office:document-content \
         xmlns:office=\"{OFFICE}\" xmlns:text=\"{TEXT}\" office:version=\"1.3\"><office:body>\
         <office:text>{body}</office:text></office:body></office:document-content>"
    )
    .into_bytes()
}

/// `levels` sections inside one another, a paragraph at the bottom: the deepest
/// element sits at `levels + 4`.
fn sections(levels: usize) -> String {
    format!(
        "{}<text:p>Words at the bottom.</text:p>{}",
        "<text:section text:name=\"s\">".repeat(levels),
        "</text:section>".repeat(levels)
    )
}

/// One paragraph holding `levels` spans inside one another: the deepest sits at
/// `levels + 4`.
fn spans(levels: usize) -> String {
    format!(
        "<text:p>{}Words at the bottom.{}</text:p>",
        "<text:span>".repeat(levels),
        "</text:span>".repeat(levels)
    )
}

/// `levels` lists inside one another's items, a paragraph in the innermost item
/// holding its words in `spans` spans: the paragraph sits at `2 * levels + 4`, and
/// each span one level under it.
fn lists(levels: usize, spans: usize) -> String {
    format!(
        "{}<text:p>{}Words at the bottom.{}</text:p>{}",
        "<text:list><text:list-item>".repeat(levels),
        "<text:span>".repeat(spans),
        "</text:span>".repeat(spans),
        "</text:list-item></text:list>".repeat(levels)
    )
}

/// A commented paragraph whose comment holds `levels` spans inside one another:
/// the deepest sits at `levels + 6`.
fn comment_spans(levels: usize) -> String {
    format!(
        "<text:p>Words at the bottom.<office:annotation><text:p>{}A note.{}</text:p>\
         </office:annotation></text:p>",
        "<text:span>".repeat(levels),
        "</text:span>".repeat(levels)
    )
}

/// A paragraph with a footnote whose own paragraph holds `levels` spans inside one
/// another: the note sits at 5, its body at 6, the body's paragraph at 7, and the
/// deepest span at `levels + 7`.
fn footnote_spans(levels: usize) -> String {
    format!(
        "<text:p>Words at the bottom.<text:note text:id=\"ftn1\" text:note-class=\"footnote\">\
         <text:note-citation>1</text:note-citation><text:note-body><text:p>{}A footnote.{}\
         </text:p></text:note-body></text:note></text:p>",
        "<text:span>".repeat(levels),
        "</text:span>".repeat(levels)
    )
}

/// The words a comment or a footnote carried, which never reach the prose.
fn margin_text(doc: &SourceDocument) -> String {
    doc.annotations
        .iter()
        .map(|annotation| annotation.body.as_str())
        .chain(doc.footnotes.iter().map(|footnote| footnote.body.as_str()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Each of the recursive walks in the ODT scanner, at the ceiling: sections
/// (`walk_container`), lists (`walk_list`), spans (`inline`), and the spans of a
/// comment's and of a footnote's own text (`annotation_inline`, reached two ways).
///
/// The words each shape keeps at its deepest level are the ones checked: a walk
/// stopping one level short would lose exactly those. For the comment and the
/// footnote they are not prose at all but the note's own text.
#[test]
fn an_odt_nested_to_the_ceiling_is_read_through_every_walk_from_a_long_operation_stack() {
    let in_the_prose = [
        ("sections", sections(MAX_XML_DEPTH - 4)),
        ("spans", spans(MAX_XML_DEPTH - 4)),
        ("lists", lists((MAX_XML_DEPTH - 4) / 2, 0)),
    ];
    for (shape, body) in in_the_prose {
        let doc = scan_on_a_long_operation_stack("hostile.fodt", fodt(&body));
        assert_read(shape, &doc, "Words at the bottom.");
    }

    let in_the_margin = [
        ("comment spans", comment_spans(MAX_XML_DEPTH - 6), "A note."),
        (
            "footnote spans",
            footnote_spans(MAX_XML_DEPTH - 7),
            "A footnote.",
        ),
    ];
    for (shape, body, deepest) in in_the_margin {
        let doc = scan_on_a_long_operation_stack("hostile.fodt", fodt(&body));
        assert_read(shape, &doc, "Words at the bottom.");
        assert!(
            margin_text(&doc).contains(deepest),
            "{shape}: the note's words at the ceiling must arrive: {:?}",
            margin_text(&doc)
        );
    }
}

/// The lists at the ceiling nest far past the levels imported prose keeps. The row they
/// make is stored as prose the next load of the project accepts, the item arrives as the
/// list item it was, at the deepest level kept, and the writer is told.
#[test]
fn an_odt_list_nested_to_the_ceiling_is_stored_as_prose_a_load_accepts() {
    let levels = (MAX_XML_DEPTH - 4) / 2;
    let doc = scan_on_a_long_operation_stack("hostile.fodt", fodt(&lists(levels, 0)));
    let djot: Vec<&str> = doc
        .blocks
        .iter()
        .filter_map(|block| match block {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect();
    let [djot] = djot.as_slice() else {
        panic!("one prose block, got {:?}", doc.blocks);
    };
    assert!(
        skrib_format::djot_depth::check(djot).is_ok(),
        "the load must accept what is stored: {djot:?}"
    );
    let kept = document_ingest::sources::rich::MAX_LIST_LEVELS;
    let deepest = format!("{}- Words at the bottom", "  ".repeat(kept - 1));
    assert!(
        djot.starts_with(&deepest),
        "the item is written at the deepest level kept: {djot:?}"
    );
    assert_eq!(prose(&doc), "Words at the bottom.");
    assert!(
        doc.diagnostics.iter().any(|d| matches!(
            d,
            ImportDiagnostic::ListNestingFlattened { count: 1, limit, .. } if *limit == kept
        )),
        "the flattening is reported: {:?}",
        doc.diagnostics
    );
}

#[test]
fn an_odt_one_level_past_the_ceiling_is_refused_by_name_whatever_the_shape() {
    let past_the_ceiling = [
        ("sections", sections(MAX_XML_DEPTH - 3)),
        ("spans", spans(MAX_XML_DEPTH - 3)),
        ("lists", lists((MAX_XML_DEPTH - 4) / 2, 1)),
        ("comment spans", comment_spans(MAX_XML_DEPTH - 5)),
        ("footnote spans", footnote_spans(MAX_XML_DEPTH - 6)),
    ];
    for (shape, body) in past_the_ceiling {
        let doc = scan_on_a_long_operation_stack("hostile.fodt", fodt(&body));
        assert_refused(shape, &doc, "hostile.fodt");
    }
}

#[test]
fn a_zipped_odt_names_the_member_it_refused() {
    let odt = zip(&[
        ("mimetype", b"application/vnd.oasis.opendocument.text"),
        ("content.xml", &content_xml(&sections(MAX_XML_DEPTH - 3))),
    ]);
    let doc = scan_on_a_long_operation_stack("hostile.odt", odt);
    assert_refused("zipped", &doc, "content.xml");
}

/// `meta.xml` only ever supplies a title, and one that does not parse costs just
/// the title. One nested past the ceiling refuses the file like any other part.
#[test]
fn a_meta_part_past_the_ceiling_refuses_the_file() {
    let meta = format!(
        "<office:document-meta xmlns:office=\"{OFFICE}\">{}{}</office:document-meta>",
        "<office:meta>".repeat(MAX_XML_DEPTH),
        "</office:meta>".repeat(MAX_XML_DEPTH)
    );
    let odt = zip(&[
        ("content.xml", &content_xml(&sections(1))),
        ("meta.xml", meta.as_bytes()),
    ]);
    let doc = scan_on_a_long_operation_stack("hostile.odt", odt);
    assert_refused("meta", &doc, "meta.xml");
}

// ---------------------------------------------------------------------------
// Word
// ---------------------------------------------------------------------------

const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// A `.docx` whose main part is `main`, holding `body`, plus `extra` members and
/// document relationships.
fn docx(main: &str, body: &str, relationships: &str, extra: &[(&str, &[u8])]) -> Vec<u8> {
    package(
        main,
        document_part("", body).as_bytes(),
        relationships,
        extra,
    )
}

/// A main document part holding `body`, with `preamble` between the XML
/// declaration and the root element.
fn document_part(preamble: &str, body: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{preamble}<w:document xmlns:w=\"{W}\"><w:body>\
         <w:p><w:r><w:t>Chapter One</w:t></w:r></w:p>{body}</w:body></w:document>"
    )
}

/// A `.docx` whose main part is `main`, holding exactly `document`, plus `extra`
/// members and document relationships.
fn package(main: &str, document: &[u8], relationships: &str, extra: &[(&str, &[u8])]) -> Vec<u8> {
    let content_types = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Types \
        xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default \
        Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
        <Default Extension=\"xml\" ContentType=\"application/xml\"/></Types>";
    let package = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships \
         xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship \
         Id=\"rId1\" Type=\"{REL}/officeDocument\" Target=\"{main}\"/></Relationships>"
    );
    let document_rels = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships \
         xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">{relationships}\
         </Relationships>"
    );
    let stem = Path::new(main)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("document");
    let rels_name = format!("word/_rels/{stem}.xml.rels");
    let mut members: Vec<(&str, &[u8])> = vec![
        ("[Content_Types].xml", content_types.as_bytes()),
        ("_rels/.rels", package.as_bytes()),
        (main, document),
        (rels_name.as_str(), document_rels.as_bytes()),
    ];
    members.extend_from_slice(extra);
    zip(&members)
}

/// Tables nested in table cells, `docx-rs`'s one recursion, down to a run whose
/// bold mark sits `extra_links` levels further than the ceiling allows plus one.
///
/// `<w:document>` is 1, `<w:body>` 2, and 83 tables take three levels each, so
/// the innermost cell is at 251 and its paragraph at 252. One hyperlink (253), the
/// run (254), its properties (255) and the empty `<w:b/>` (256) reach the ceiling
/// exactly; each extra hyperlink is one level past it.
fn nested_tables(extra_links: usize) -> String {
    let tables = 83;
    let links = 1 + extra_links;
    format!(
        "{}<w:p>{}<w:r><w:rPr><w:b/></w:rPr><w:t>Words at the bottom.</w:t></w:r>{}</w:p>{}",
        "<w:tbl><w:tr><w:tc>".repeat(tables),
        "<w:hyperlink w:anchor=\"a\">".repeat(links),
        "</w:hyperlink>".repeat(links),
        "</w:tc></w:tr></w:tbl>".repeat(tables)
    )
}

#[test]
fn a_docx_nested_to_the_ceiling_is_read_from_a_long_operation_stack() {
    let doc = scan_on_a_long_operation_stack(
        "hostile.docx",
        docx("word/document.xml", &nested_tables(0), "", &[]),
    );
    // The typed walk reads a table's own cells, not a table nested in one, so the
    // words at the bottom are not prose here; the heading above the table is.
    assert_read("tables", &doc, "Chapter One");
}

#[test]
fn a_docx_one_level_past_the_ceiling_is_refused_by_name() {
    let doc = scan_on_a_long_operation_stack(
        "hostile.docx",
        docx("word/document.xml", &nested_tables(1), "", &[]),
    );
    assert_refused("tables", &doc, "word/document.xml");
}

/// `docx-rs` reads with `quick-xml`, which ends a DOCTYPE where `roxmltree` does
/// not: it skips an unknown `<…>` inside the internal subset to its first `>`,
/// reads an entity's value quote to quote, and takes a lowercase `<!doctype`.
/// Measured only `roxmltree`'s way, either preamble below swallowed the rest of
/// the part into a comment, so the body behind it, however deep, was never
/// counted and went straight to `docx-rs`.
#[test]
fn a_doctype_cannot_hide_the_nesting_behind_it() {
    let preambles = [
        (
            "markup holding a comment",
            "<!DOCTYPE w:document [<x <!-- >]>",
        ),
        (
            "a lowercase doctype",
            "<!doctype w:document [<!ENTITY e \"> <!-- \">]>",
        ),
    ];
    for (shape, preamble) in preambles {
        let past = document_part(preamble, &nested_tables(1));
        let doc = scan_on_a_long_operation_stack(
            "hostile.docx",
            package("word/document.xml", past.as_bytes(), "", &[]),
        );
        assert_refused(shape, &doc, "word/document.xml");

        // And the check is not merely refusing the preamble: at the ceiling the
        // same file is read.
        let at = document_part(preamble, &nested_tables(0));
        let doc = scan_on_a_long_operation_stack(
            "hostile.docx",
            package("word/document.xml", at.as_bytes(), "", &[]),
        );
        assert_read(shape, &doc, "Chapter One");
    }
}

/// An end tag holding a quoted `>` is an error to both parsers, and they part
/// ways at it. `roxmltree` refuses the document there; the check, which follows
/// it, ends the tag at that `>` and so reads the rest of it as the start of a
/// comment that never closes, counting nothing after it. `quick-xml` reads the tag
/// quote to quote, reports the mismatch as ill-formed and goes on, and a content
/// control's reader catches the table's error and keeps reading the same stream:
/// the tables after it are parsed, so they have to be counted.
#[test]
fn an_error_docx_rs_reads_past_cannot_hide_the_nesting_behind_it() {
    let body = format!(
        "<w:sdt><w:sdtContent><w:tbl></w:x a='>' <!-- >{}</w:sdtContent></w:sdt>",
        nested_tables(0)
    );
    let doc =
        scan_on_a_long_operation_stack("hostile.docx", docx("word/document.xml", &body, "", &[]));
    assert_refused("error in a content control", &doc, "word/document.xml");
}

/// `quick-xml` recognises a UTF-16 byte-order mark and then parses the bytes after
/// it as they are, so a part opening with one and continuing in plain ASCII is
/// read by `docx-rs` like any other, as deep as its tables go. Decoded as the
/// UTF-16 its mark claims, the same bytes hold no markup at all.
///
/// The root carries no attribute and the tables come before any text: `docx-rs`
/// decodes those two as UTF-16 and stops at the first it cannot, and the part is
/// built to reach the tables first, as a hostile one would.
#[test]
fn a_byte_order_mark_cannot_hide_the_nesting_behind_it() {
    let mut past = vec![0xFF, 0xFE];
    past.extend_from_slice(
        format!(
            "<?xml version=\"1.0\"?><w:document><w:body>{}</w:body></w:document>",
            nested_tables(1)
        )
        .as_bytes(),
    );
    let doc = scan_on_a_long_operation_stack(
        "hostile.docx",
        package("word/document.xml", &past, "", &[]),
    );
    assert_refused("byte-order mark", &doc, "word/document.xml");
}

/// A part genuinely written in UTF-16 looks, to a parser reading it byte by byte,
/// like one start tag after another, every name holding NUL bytes. `docx-rs`
/// recognises none of them and reads nothing from it; refusing it as nested past
/// the ceiling would tell the writer something untrue.
#[test]
fn a_part_in_utf16_is_not_taken_for_one_nested_past_the_ceiling() {
    let paragraphs = "<w:p><w:r><w:t>A line.</w:t></w:r></w:p>".repeat(MAX_XML_DEPTH);
    let mut utf16 = vec![0xFF, 0xFE];
    for unit in document_part("", &paragraphs).encode_utf16() {
        utf16.extend_from_slice(&unit.to_le_bytes());
    }
    let doc =
        scan_on_a_long_operation_stack("wide.docx", package("word/document.xml", &utf16, "", &[]));
    assert!(
        !doc.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::NestedTooDeep { .. })),
        "a wide part is not a deep one: {:?}",
        doc.diagnostics
    );
}

/// `docx-rs` finds the main part through `_rels/.rels`, whatever it is called, and
/// parses it as XML. A check that went by the `.xml` extension would wave this one
/// straight through to the parser.
#[test]
fn a_main_part_not_named_xml_is_checked_all_the_same() {
    let doc = scan_on_a_long_operation_stack(
        "hostile.docx",
        docx("word/main.bin", &nested_tables(1), "", &[]),
    );
    assert_refused("main part", &doc, "word/main.bin");
}

/// A header is found through the document's own relationships, whatever it is
/// called, and `docx-rs` parses it with the same recursive readers as the body.
#[test]
fn a_header_part_past_the_ceiling_is_refused_by_name() {
    // A header's root sits where the document's `<w:body>` does, one level
    // shallower than the body's content, so it takes one more link to pass.
    let header = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><w:hdr xmlns:w=\"{W}\">{}</w:hdr>",
        nested_tables(2)
    );
    let relationship =
        format!("<Relationship Id=\"rId9\" Type=\"{REL}/header\" Target=\"page-top.bin\"/>");
    let doc = scan_on_a_long_operation_stack(
        "hostile.docx",
        docx(
            "word/document.xml",
            "",
            &relationship,
            &[("word/page-top.bin", header.as_bytes())],
        ),
    );
    assert_refused("header", &doc, "word/page-top.bin");
}

/// A picture is bytes, and bytes read as XML have a depth: every `<` followed by a
/// letter looks like a start tag, so a large enough image always "nests" past the
/// ceiling. The check reads only the parts that are parsed, and an image is not.
#[test]
fn an_image_is_not_read_as_xml() {
    // A deterministic stand-in for compressed image data: every byte value, many
    // times over, so `<` and letters and `>` all occur, and never a `</`.
    let mut picture = Vec::with_capacity(512 * 1024);
    let mut state: u32 = 0x2545_f491;
    while picture.len() < 512 * 1024 {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        picture.push((state >> 24) as u8);
    }
    assert!(
        skrib_format::xml_depth::check("image1.png", &picture).is_err(),
        "the stand-in must be one that a check of every part would refuse"
    );
    let relationship =
        format!("<Relationship Id=\"rId7\" Type=\"{REL}/image\" Target=\"media/image1.png\"/>");
    let doc = scan_on_a_long_operation_stack(
        "illustrated.docx",
        docx(
            "word/document.xml",
            "",
            &relationship,
            &[("word/media/image1.png", &picture)],
        ),
    );
    assert_read("image", &doc, "Chapter One");
}
