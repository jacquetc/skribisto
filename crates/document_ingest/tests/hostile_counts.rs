// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Documents that ask for more than they hold.
//!
//! A count in a file is the file's to choose. Read as it stands, a few bytes of markup
//! can ask the importer for gigabytes, and an allocation that size ends the process: it
//! is an abort or a panic on the scan's thread, never an import that fails and says so.
//! Every count the ODT and DOCX scanners turn into text or cells is held to a limit here,
//! with the counts they read and do not honour pinned beside them.
//!
//! Each document is scanned on a thread with the 2 MiB stack a long operation gets, as
//! the importer scans it.

use std::io::Write;
use std::path::Path;

use document_ingest::{ImportDiagnostic, ScannerRegistry, SourceBlock, SourceDocument};

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 * 1024 * 1024;

/// The longest run of spaces an ODT `<text:s>` is read as (`odt::MAX_SPACE_RUN`).
const MAX_SPACE_RUN: usize = 1_000;

/// Scan `bytes` as the file `name` on a thread with a long operation's stack.
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

/// Every stored piece of prose, as Djot, joined.
fn djot(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .filter_map(|block| match block {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn prose(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .map(SourceBlock::plain_text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Nothing refused or unreadable.
fn assert_read(shape: &str, doc: &SourceDocument) {
    for diagnostic in &doc.diagnostics {
        assert!(
            !matches!(
                diagnostic,
                ImportDiagnostic::NestedTooDeep { .. } | ImportDiagnostic::FileUnreadable { .. }
            ),
            "{shape}: the document must be read, got {diagnostic}"
        );
    }
}

/// The longest run of spaces in `text`.
fn longest_space_run(text: &str) -> usize {
    text.split(|c| c != ' ').map(str::len).max().unwrap_or(0)
}

// ---------------------------------------------------------------------------
// OpenDocument
// ---------------------------------------------------------------------------

const OFFICE: &str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
const TEXT: &str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
const TABLE: &str = "urn:oasis:names:tc:opendocument:xmlns:table:1.0";
const DC: &str = "http://purl.org/dc/elements/1.1/";

/// A flat `.fodt` whose `<office:text>` holds `body`.
fn fodt(body: &str) -> Vec<u8> {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><office:document xmlns:office=\"{OFFICE}\" \
         xmlns:text=\"{TEXT}\" xmlns:table=\"{TABLE}\" xmlns:dc=\"{DC}\" office:version=\"1.3\">\
         <office:body><office:text>{body}</office:text></office:body></office:document>"
    )
    .into_bytes()
}

/// The counts a `<text:s>` can ask for, from four billion to more than a `usize` holds.
const HUGE_COUNTS: [&str; 4] = [
    "4000000000",
    "18446744073709551615",
    "18446744073709551616",
    "99999999999999999999999999999999",
];

/// A run of spaces asking for four billion, or for more than a `usize` holds, in the prose,
/// in a comment and in a footnote: each arrives a thousand spaces long, between the words
/// it separated, and the writer is told how many runs were cut. Read as written, the first
/// asked for four gigabytes of spaces, and the others overflowed the allocation, each
/// ending the scan's thread rather than the import.
#[test]
fn an_odt_run_of_spaces_asking_for_billions_arrives_a_thousand_long() {
    for count in HUGE_COUNTS {
        let body = format!(
            "<text:p>Before<text:s text:c=\"{count}\"/>after.</text:p>\
             <text:p>A remark.<office:annotation><dc:creator>Ed</dc:creator><text:p>Note\
             <text:s text:c=\"{count}\"/>end.</text:p></office:annotation></text:p>\
             <text:p>Cited.<text:note text:id=\"ftn1\" text:note-class=\"footnote\">\
             <text:note-citation>1</text:note-citation><text:note-body><text:p>Body\
             <text:s text:c=\"{count}\"/>tail.</text:p></text:note-body></text:note></text:p>"
        );
        let doc = scan_on_a_long_operation_stack("spaced.fodt", fodt(&body));
        assert_read(count, &doc);

        let text = prose(&doc);
        assert!(
            text.contains("Before") && text.contains("after."),
            "{count}: {text:.80?}"
        );
        assert_eq!(
            longest_space_run(&text),
            MAX_SPACE_RUN,
            "{count}: the prose's run"
        );
        let [comment] = doc.annotations.as_slice() else {
            panic!("{count}: one comment arrives, got {:?}", doc.annotations);
        };
        assert!(comment.body.contains("Note") && comment.body.contains("end."));
        assert_eq!(
            longest_space_run(&comment.body),
            MAX_SPACE_RUN,
            "{count}: the comment's run"
        );
        let [note] = doc.footnotes.as_slice() else {
            panic!("{count}: one footnote arrives, got {:?}", doc.footnotes);
        };
        assert!(note.body.contains("Body") && note.body.contains("tail."));
        assert_eq!(
            longest_space_run(&note.body),
            MAX_SPACE_RUN,
            "{count}: the note's run"
        );

        let shortened: Vec<&ImportDiagnostic> = doc
            .diagnostics
            .iter()
            .filter(|d| matches!(d, ImportDiagnostic::SpacesShortened { .. }))
            .collect();
        assert!(
            matches!(
                shortened.as_slice(),
                [ImportDiagnostic::SpacesShortened {
                    count: 3,
                    limit: MAX_SPACE_RUN,
                    ..
                }]
            ),
            "{count}: the three cut runs are reported once, together: {:?}",
            doc.diagnostics
        );
    }
}

/// A run at the limit, or of any ordinary length, is read as written and reported as
/// nothing, and a count that is no number is one space, as ODF has it.
#[test]
fn an_odt_run_of_spaces_within_the_limit_is_read_as_written() {
    for (count, spaces) in [
        ("1", 1),
        ("7", 7),
        (" 12 ", 12),
        ("1000", MAX_SPACE_RUN),
        ("", 1),
        ("many", 1),
    ] {
        let body = format!("<text:p>Before<text:s text:c=\"{count}\"/>after.</text:p>");
        let doc = scan_on_a_long_operation_stack("spaced.fodt", fodt(&body));
        assert_read(count, &doc);
        assert_eq!(
            prose(&doc),
            format!("Before{}after.", " ".repeat(spaces)),
            "{count:?}"
        );
        assert!(
            !doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::SpacesShortened { .. })),
            "{count:?}: nothing was cut"
        );
    }
}

/// How many cells a pipe table's rows hold, row by row, from its stored Djot.
fn table_shape(djot: &str) -> Vec<usize> {
    djot.lines()
        .filter(|line| line.starts_with('|') && !line.starts_with("|-"))
        .map(|line| line.matches(" |").count())
        .collect()
}

/// A table whose first row is `wide` cells and whose `narrow` other rows are one cell
/// each: squared, it would be `wide` times `narrow` cells, most of them empty.
fn odt_wide_then_narrow(wide: usize, narrow: usize) -> Vec<u8> {
    let first: String = (0..wide)
        .map(|i| format!("<table:table-cell><text:p>h{i}</text:p></table:table-cell>"))
        .collect();
    let rest: String = (0..narrow)
        .map(|i| {
            format!(
                "<table:table-row><table:table-cell><text:p>r{i}</text:p></table:table-cell>\
                 </table:table-row>"
            )
        })
        .collect();
    fodt(&format!(
        "<text:p>Before the table.</text:p><table:table table:name=\"T\">\
         <table:table-row>{first}</table:table-row>{rest}</table:table>"
    ))
}

/// Ten thousand one-cell rows under a first row of ten thousand cells: squared, a hundred
/// million cells, nearly all of them made up, and an allocation the process dies of. The
/// table arrives with every cell the file holds, its first row as wide as the widest, and
/// no more than twice the cells of the file.
#[test]
fn an_odt_table_squared_past_twice_its_cells_arrives_with_its_own_cells() {
    let (wide, narrow) = (10_000, 10_000);
    let doc = scan_on_a_long_operation_stack("wide.fodt", odt_wide_then_narrow(wide, narrow));
    assert_read("wide table", &doc);
    let text = prose(&doc);
    for word in ["Before the table.", "h0", "h9999", "r0", "r9999"] {
        assert!(text.contains(word), "{word} arrives");
    }
    let shape = table_shape(&djot(&doc));
    assert_eq!(shape.len(), 1 + narrow, "every row arrives");
    assert_eq!(shape[0], wide, "the first row is the widest");
    assert!(
        shape[1..].iter().all(|&cells| cells == 1),
        "the other rows keep their one cell"
    );
    let held = wide + narrow;
    assert!(shape.iter().sum::<usize>() <= 2 * held);
}

/// The counts the ODT scanner reads and does not honour: a row repeated a million times,
/// a cell repeated or spanning a million columns or rows. Each cell arrives once, as
/// written, and nothing is multiplied. Pinned so that honouring one of them later has to
/// bound it as `MAX_SPACE_RUN` bounds `text:c`.
#[test]
fn an_odt_table_counts_the_scanner_does_not_honour_multiply_nothing() {
    let million = "1048576";
    let body = format!(
        "<table:table table:name=\"T\">\
         <table:table-row table:number-rows-repeated=\"{million}\">\
         <table:table-cell table:number-columns-repeated=\"{million}\"><text:p>repeated</text:p>\
         </table:table-cell>\
         <table:table-cell table:number-columns-spanned=\"{million}\" \
         table:number-rows-spanned=\"{million}\"><text:p>spanned</text:p></table:table-cell>\
         <table:covered-table-cell table:number-columns-repeated=\"{million}\"/>\
         </table:table-row></table:table>"
    );
    let doc = scan_on_a_long_operation_stack("repeated.fodt", fodt(&body));
    assert_read("repeated", &doc);
    assert_eq!(
        table_shape(&djot(&doc)),
        vec![2],
        "one row of the two cells written"
    );
    let text = prose(&doc);
    assert_eq!(text.matches("repeated").count(), 1, "{text:.80?}");
    assert_eq!(text.matches("spanned").count(), 1, "{text:.80?}");
}

// ---------------------------------------------------------------------------
// Word
// ---------------------------------------------------------------------------

const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// A `.docx` whose `word/document.xml` body is `body`.
fn docx(body: &str) -> Vec<u8> {
    let content_types = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Types \
        xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default \
        Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
        <Default Extension=\"xml\" ContentType=\"application/xml\"/></Types>";
    let package = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships \
         xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship \
         Id=\"rId1\" Type=\"{REL}/officeDocument\" Target=\"word/document.xml\"/></Relationships>"
    );
    let document = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><w:document xmlns:w=\"{W}\"><w:body>\
         {body}</w:body></w:document>"
    );
    let document_rels = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships \
        xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"></Relationships>";
    zip(&[
        ("[Content_Types].xml", content_types.as_bytes()),
        ("_rels/.rels", package.as_bytes()),
        ("word/document.xml", document.as_bytes()),
        ("word/_rels/document.xml.rels", document_rels.as_bytes()),
    ])
}

fn docx_cell(text: &str, properties: &str) -> String {
    format!("<w:tc>{properties}<w:p><w:r><w:t>{text}</w:t></w:r></w:p></w:tc>")
}

/// The same table in Word's markup: a first row of ten thousand cells over ten thousand
/// rows of one. `w:gridSpan` says nothing here, as Word itself writes such a table only
/// from merged cells; the file's cells are what the scanner reads.
#[test]
fn a_docx_table_squared_past_twice_its_cells_arrives_with_its_own_cells() {
    let (wide, narrow) = (10_000, 10_000);
    let first: String = (0..wide).map(|i| docx_cell(&format!("h{i}"), "")).collect();
    let rest: String = (0..narrow)
        .map(|i| format!("<w:tr>{}</w:tr>", docx_cell(&format!("r{i}"), "")))
        .collect();
    let body = format!(
        "<w:p><w:r><w:t>Before the table.</w:t></w:r></w:p><w:tbl><w:tr>{first}</w:tr>{rest}\
         </w:tbl>"
    );
    let doc = scan_on_a_long_operation_stack("wide.docx", docx(&body));
    assert_read("wide table", &doc);
    let text = prose(&doc);
    for word in ["Before the table.", "h0", "h9999", "r0", "r9999"] {
        assert!(text.contains(word), "{word} arrives");
    }
    let shape = table_shape(&djot(&doc));
    assert_eq!(shape.len(), 1 + narrow, "every row arrives");
    assert_eq!(shape[0], wide, "the first row is the widest");
    assert!(
        shape[1..].iter().all(|&cells| cells == 1),
        "the other rows keep their one cell"
    );
    assert!(shape.iter().sum::<usize>() <= 2 * (wide + narrow));
}

/// Word's counts that could multiply a table, a span of a million columns, a vertical
/// merge, a million grid columns skipped before and after a row: the scanner reads the
/// cells the file holds and none of these, so each cell arrives once and nothing is
/// multiplied. Pinned beside the ODT ones, for the same reason.
#[test]
fn docx_table_counts_the_scanner_does_not_honour_multiply_nothing() {
    let million = "1048576";
    let row = format!(
        "<w:tr><w:trPr><w:gridBefore w:val=\"{million}\"/><w:gridAfter w:val=\"{million}\"/>\
         </w:trPr>{}{}</w:tr>",
        docx_cell(
            "spanned",
            &format!("<w:tcPr><w:gridSpan w:val=\"{million}\"/></w:tcPr>")
        ),
        docx_cell("merged", "<w:tcPr><w:vMerge w:val=\"restart\"/></w:tcPr>"),
    );
    let body = format!("<w:tbl><w:tblGrid><w:gridCol w:w=\"100\"/></w:tblGrid>{row}</w:tbl>");
    let doc = scan_on_a_long_operation_stack("spanned.docx", docx(&body));
    assert_read("spanned", &doc);
    assert_eq!(
        table_shape(&djot(&doc)),
        vec![2],
        "one row of the two cells written"
    );
    let text = prose(&doc);
    assert_eq!(text.matches("spanned").count(), 1, "{text:.80?}");
    assert_eq!(text.matches("merged").count(), 1, "{text:.80?}");
}
