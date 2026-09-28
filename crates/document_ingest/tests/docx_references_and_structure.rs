// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a Word file's relationships, notes, tables and empty lines bring, read from files
//! built here in the shapes Word and LibreOffice write.
//!
//! Each fixture is spelled out part by part: a link as `<w:hyperlink r:id>` with its
//! address in the part's relationships, a point comment as the bare
//! `<w:commentReference>` LibreOffice writes, a ranged one as Word's
//! `commentRangeStart`/`commentRangeEnd` pair, footnotes and endnotes numbered from 1 in
//! two parts of their own, a no-break hyphen as `<w:noBreakHyphen/>`, a symbol as `<w:sym>`.
//! A writer that normalised its input would not keep these shapes, which is why none is
//! produced by one.
//! The ODF fixtures are flat `.fodt`, spelled as LibreOffice writes the same constructs.

use std::io::Write;
use std::path::Path;

use common::entities::{ChapterMode, CommentAnchorKind};
use document_ingest::plan::{ImportPlan, PlannedRow};
use document_ingest::{ImportDiagnostic, ScannerRegistry, scan_and_plan};
use skribisto_model::CreateType;

const NS: &str = concat!(
    r#"xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" "#,
    r#"xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" "#,
    r#"xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" "#,
    r#"xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" "#,
    r#"xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" "#,
    r#"xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" "#,
    r#"xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" "#,
    r#"xmlns:v="urn:schemas-microsoft-com:vml" "#,
    r#"xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math""#,
);

const RELATIONSHIPS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// The parts of one `.docx`, built into a package by [`Word::bytes`].
#[derive(Default)]
struct Word {
    body: String,
    /// `<w:comment>` elements.
    comments: Vec<String>,
    /// `<Relationship>` elements of the comments part, `word/_rels/comments.xml.rels`.
    comment_relationships: String,
    /// `<w:footnote>` elements after Word's two separators, numbered from 1.
    footnotes: Vec<String>,
    /// `<w:endnote>` elements after Word's two separators, numbered from 1.
    endnotes: Vec<String>,
    /// `<Relationship>` elements of the main part beyond its styles, comments and notes.
    relationships: String,
    /// Other members, such as a picture.
    media: Vec<(&'static str, &'static [u8])>,
    /// The smallest package there is: `[Content_Types].xml`, `_rels/.rels` and
    /// `word/document.xml`, nothing else, as many scripts write one.
    minimal: bool,
}

impl Word {
    fn with_body(body: impl Into<String>) -> Self {
        Word {
            body: body.into(),
            ..Word::default()
        }
    }

    fn bytes(&self) -> Vec<u8> {
        let mut parts: Vec<(String, Vec<u8>)> = vec![
            (
                "[Content_Types].xml".into(),
                concat!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
                    r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">"#,
                    r#"<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>"#,
                    r#"<Default Extension="xml" ContentType="application/xml"/>"#,
                    r#"<Default Extension="png" ContentType="image/png"/>"#,
                    r#"<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>"#,
                    r#"</Types>"#
                )
                .into(),
            ),
            (
                "_rels/.rels".into(),
                format!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{RELATIONSHIPS}"><Relationship Id="rId1" Type="{TYPE}/officeDocument" Target="word/document.xml"/></Relationships>"#
                )
                .into_bytes(),
            ),
            (
                "word/document.xml".into(),
                format!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document {NS}><w:body>{}<w:sectPr/></w:body></w:document>"#,
                    self.body
                )
                .into_bytes(),
            ),
        ];
        if !self.minimal {
            let mut relationships = format!(
                r#"<Relationship Id="rId1" Type="{TYPE}/styles" Target="styles.xml"/><Relationship Id="rId2" Type="{TYPE}/comments" Target="comments.xml"/><Relationship Id="rId3" Type="{TYPE}/footnotes" Target="footnotes.xml"/><Relationship Id="rId4" Type="{TYPE}/endnotes" Target="endnotes.xml"/>"#
            );
            relationships.push_str(&self.relationships);
            parts.push((
                "word/_rels/document.xml.rels".into(),
                format!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{RELATIONSHIPS}">{relationships}</Relationships>"#
                )
                .into_bytes(),
            ));
            parts.push((
                "word/styles.xml".into(),
                format!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:styles {NS}><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style><w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:pPr><w:outlineLvl w:val="0"/></w:pPr></w:style><w:style w:type="character" w:styleId="Hyperlink"><w:name w:val="Hyperlink"/><w:rPr><w:u w:val="single"/></w:rPr></w:style><w:style w:type="character" w:styleId="FootnoteReference"><w:name w:val="footnote reference"/><w:rPr><w:vertAlign w:val="superscript"/></w:rPr></w:style></w:styles>"#
                )
                .into_bytes(),
            ));
            parts.push((
                "word/comments.xml".into(),
                format!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:comments {NS}>{}</w:comments>"#,
                    self.comments.concat()
                )
                .into_bytes(),
            ));
            if !self.comment_relationships.is_empty() {
                parts.push((
                    "word/_rels/comments.xml.rels".into(),
                    format!(
                        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="{RELATIONSHIPS}">{}</Relationships>"#,
                        self.comment_relationships
                    )
                    .into_bytes(),
                ));
            }
            for (part, element, notes) in [
                ("word/footnotes.xml", "footnote", &self.footnotes),
                ("word/endnotes.xml", "endnote", &self.endnotes),
            ] {
                let mut xml = format!(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:{element}s {NS}><w:{element} w:type="separator" w:id="-1"><w:p><w:r><w:separator/></w:r></w:p></w:{element}><w:{element} w:type="continuationSeparator" w:id="0"><w:p><w:r><w:continuationSeparator/></w:r></w:p></w:{element}>"#
                );
                for (index, note) in notes.iter().enumerate() {
                    xml.push_str(&format!(
                        r#"<w:{element} w:id="{}"><w:p><w:r><w:rPr><w:vertAlign w:val="superscript"/></w:rPr><w:{element}Ref/></w:r><w:r><w:t xml:space="preserve"> {note}</w:t></w:r></w:p></w:{element}>"#,
                        index + 1
                    ));
                }
                xml.push_str(&format!("</w:{element}s>"));
                parts.push((part.into(), xml.into_bytes()));
            }
            for (name, bytes) in &self.media {
                parts.push(((*name).into(), bytes.to_vec()));
            }
        }
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        for (name, content) in parts {
            zip.start_file(name.as_str(), options)
                .expect("start a part");
            zip.write_all(&content).expect("write a part");
        }
        zip.finish().expect("finish the container").into_inner()
    }
}

/// A body paragraph holding `text` in one run.
fn p(text: &str) -> String {
    format!(r#"<w:p><w:r><w:t xml:space="preserve">{text}</w:t></w:r></w:p>"#)
}

/// A paragraph in Word's first heading style.
fn heading(text: &str) -> String {
    format!(r#"<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>{text}</w:t></w:r></w:p>"#)
}

/// A comment as Word writes one: its reference mark, then its text.
fn comment(id: usize, body: &str) -> String {
    format!(
        r#"<w:comment w:id="{id}" w:author="Editor" w:date="2026-03-04T05:06:07Z" w:initials="E"><w:p><w:r><w:annotationRef/></w:r><w:r><w:t xml:space="preserve">{body}</w:t></w:r></w:p></w:comment>"#
    )
}

/// A footnote or endnote reference in a run of its own, as Word writes it.
fn note_reference(kind: &str, id: usize) -> String {
    format!(
        r#"<w:r><w:rPr><w:rStyle w:val="FootnoteReference"/></w:rPr><w:{kind}Reference w:id="{id}"/></w:r>"#
    )
}

/// A one-by-one table cell holding `content`.
fn cell(content: &str) -> String {
    format!(r#"<w:tc><w:tcPr><w:tcW w:w="3000" w:type="dxa"/></w:tcPr>{content}</w:tc>"#)
}

/// A one-row table of `cells`.
fn table(cells: &[String]) -> String {
    format!(
        r#"<w:tbl><w:tblPr><w:tblW w:w="0" w:type="auto"/></w:tblPr><w:tblGrid>{}</w:tblGrid><w:tr>{}</w:tr></w:tbl>"#,
        r#"<w:gridCol w:w="3000"/>"#.repeat(cells.len()),
        cells.concat()
    )
}

/// A flat OpenDocument file whose text is `body`.
fn fodt(body: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  xmlns:dc="http://purl.org/dc/elements/1.1/"
  office:mimetype="application/vnd.oasis.opendocument.text">
  <office:styles>
    <style:style style:name="Chapter_20_title" style:display-name="Chapter title"
                 style:family="paragraph" style:default-outline-level="1"/>
  </office:styles>
  <office:body>
    <office:text>
{body}
    </office:text>
  </office:body>
</office:document>"#
    )
}

fn plan_of(name: &str, bytes: &[u8]) -> ImportPlan {
    scan_and_plan(
        &ScannerRegistry::with_builtin_scanners(),
        &[(Path::new(name).to_path_buf(), bytes.to_vec())],
        CreateType::Chapter,
        ChapterMode::Folder,
        0,
    )
}

/// What the editor reads a row's prose as.
fn text_of(row: &PlannedRow) -> String {
    skrib_format::djot_plain_text(&row.djot)
        .expect("the stored Djot parses")
        .0
}

/// The row titled `title`.
fn row<'p>(plan: &'p ImportPlan, title: &str) -> &'p PlannedRow {
    plan.rows
        .iter()
        .find(|row| row.title == title)
        .unwrap_or_else(|| panic!("a row titled {title:?}: {plan:#?}"))
}

/// The only row of a file with no heading.
fn only_row(plan: &ImportPlan) -> &PlannedRow {
    assert_eq!(plan.rows.len(), 1, "one row: {plan:#?}");
    &plan.rows[0]
}

/// The words a comment of `row` points at, as the editor will read them.
fn anchored_words(row: &PlannedRow, index: usize) -> String {
    let comment = &row.comments[index];
    text_of(row)
        .chars()
        .skip(comment.anchor.start)
        .take(comment.anchor.length)
        .collect()
}

fn unanchored_in(row: &PlannedRow) -> usize {
    row.diagnostics
        .iter()
        .filter(|d| matches!(d, ImportDiagnostic::CommentUnanchored { .. }))
        .count()
}

// ── links ───────────────────────────────────────────────────────────────────────────

/// Word writes an external link as `<w:hyperlink r:id>`, its address in the relationships
/// of the part it sits in. A link inside a comment names a relationship of the comments
/// part, whose ids are its own: here its `rId1` is the document's style sheet.
#[test]
fn a_word_link_keeps_its_address_in_the_text_and_in_a_comment() {
    let mut word = Word::with_body(concat!(
        r#"<w:p><w:r><w:t xml:space="preserve">Visit </w:t></w:r>"#,
        r#"<w:hyperlink r:id="rId20" w:history="1"><w:r><w:rPr><w:rStyle w:val="Hyperlink"/></w:rPr><w:t>the example site</w:t></w:r></w:hyperlink>"#,
        r#"<w:r><w:t xml:space="preserve"> today, </w:t></w:r><w:commentRangeStart w:id="0"/><w:r><w:t>again</w:t></w:r><w:commentRangeEnd w:id="0"/>"#,
        r#"<w:r><w:rPr><w:rStyle w:val="CommentReference"/></w:rPr><w:commentReference w:id="0"/></w:r><w:r><w:t>.</w:t></w:r></w:p>"#,
    ));
    word.relationships = format!(
        r#"<Relationship Id="rId20" Type="{TYPE}/hyperlink" Target="https://example.org/page" TargetMode="External"/>"#
    );
    word.comments = vec![concat!(
        r#"<w:comment w:id="0" w:author="Editor" w:date="2026-03-04T05:06:07Z" w:initials="E">"#,
        r#"<w:p><w:r><w:annotationRef/></w:r><w:r><w:t xml:space="preserve">See </w:t></w:r>"#,
        r#"<w:hyperlink r:id="rId1" w:history="1"><w:r><w:t>the style guide</w:t></w:r></w:hyperlink>"#,
        r#"<w:r><w:t>.</w:t></w:r></w:p></w:comment>"#
    )
    .to_string()];
    word.comment_relationships = format!(
        r#"<Relationship Id="rId1" Type="{TYPE}/hyperlink" Target="https://example.org/guide" TargetMode="External"/>"#
    );

    let plan = plan_of("links.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(text_of(row), "Visit the example site today, again.");
    assert!(
        row.djot.contains("](https://example.org/page)"),
        "the link keeps its address: {}",
        row.djot
    );
    assert_eq!(row.comments.len(), 1, "{:#?}", row.comments);
    assert_eq!(anchored_words(row, 0), "again");
    assert_eq!(
        row.comments[0].body, "See [the style guide](https://example.org/guide).",
        "a link in a comment keeps the address its own part gives it"
    );
}

// ── comments made where the file holds no text ──────────────────────────────────────

/// LibreOffice writes a comment on a point as a bare `w:commentReference`. On an empty
/// line at the end of a chapter, it went to the last paragraph of the whole book, in
/// another chapter; it belongs to the passage before it.
#[test]
fn a_point_comment_on_an_empty_line_goes_to_the_paragraph_before_it() {
    let mut word = Word::with_body(
        heading("Chapter One")
            + &p("The passage the editor meant.")
            + r#"<w:p><w:r><w:commentReference w:id="0"/></w:r></w:p>"#
            + &heading("Chapter Two")
            + &p("Chapter two opens.")
            + &p("The last paragraph of the book."),
    );
    word.comments = vec![comment(0, "About the passage above.")];

    let plan = plan_of("empty-line.docx", &word.bytes());
    let one = row(&plan, "Chapter One");
    assert_eq!(one.comments.len(), 1, "{plan:#?}");
    assert_eq!(anchored_words(one, 0), "The passage the editor meant.");
    assert_eq!(unanchored_in(one), 1, "and the wizard says it moved");
    assert!(row(&plan, "Chapter Two").comments.is_empty());
}

/// Word writes a comment made on an empty line as a range with nothing in it. It landed,
/// silently, on the paragraph after it.
#[test]
fn a_ranged_comment_on_an_empty_line_goes_to_the_paragraph_before_it() {
    let mut word = Word::with_body(
        p("Before the gap.")
            + r#"<w:p><w:commentRangeStart w:id="1"/><w:commentRangeEnd w:id="1"/><w:r><w:commentReference w:id="1"/></w:r></w:p>"#
            + &p("After the gap.")
            // A range opened on an empty line and closed on the words after it covers those
            // words, exactly: it is not moved.
            + r#"<w:p><w:commentRangeStart w:id="2"/></w:p>"#
            + r#"<w:p><w:r><w:t>Covered</w:t></w:r><w:commentRangeEnd w:id="2"/><w:r><w:commentReference w:id="2"/></w:r><w:r><w:t xml:space="preserve"> words.</w:t></w:r></w:p>"#,
    );
    word.comments = vec![comment(1, "On the gap."), comment(2, "On the words.")];

    let plan = plan_of("gap.docx", &word.bytes());
    let row = only_row(&plan);
    let placed: Vec<(String, CommentAnchorKind, String)> = (0..row.comments.len())
        .map(|i| {
            (
                row.comments[i].body.clone(),
                row.comments[i].kind.clone(),
                anchored_words(row, i),
            )
        })
        .collect();
    assert_eq!(
        placed,
        vec![
            (
                "On the gap.".to_string(),
                CommentAnchorKind::Paragraph,
                "Before the gap.".to_string()
            ),
            (
                "On the words.".to_string(),
                CommentAnchorKind::Range,
                "Covered".to_string()
            ),
        ]
    );
    assert_eq!(unanchored_in(row), 1, "only the moved one is reported");
}

/// The same empty line in LibreOffice's own format.
#[test]
fn an_odt_comment_on_an_empty_line_goes_to_the_paragraph_before_it() {
    let body = r#"
      <text:h text:outline-level="1">Chapter One</text:h>
      <text:p>The passage the editor meant.</text:p>
      <text:p><office:annotation><dc:creator>Editor</dc:creator><text:p>About the passage above.</text:p></office:annotation></text:p>
      <text:h text:outline-level="1">Chapter Two</text:h>
      <text:p>Chapter two opens.</text:p>"#;
    let plan = plan_of("empty-line.fodt", fodt(body).as_bytes());
    let one = row(&plan, "Chapter One");
    assert_eq!(one.comments.len(), 1, "{plan:#?}");
    assert_eq!(anchored_words(one, 0), "The passage the editor meant.");
    assert_eq!(unanchored_in(one), 1);
    assert!(row(&plan, "Chapter Two").comments.is_empty());
}

/// A point comment inside a table cell, and one in a content control's paragraph, keep
/// their words. The raw pass read neither, and both went to the last paragraph of the
/// book.
#[test]
fn a_point_comment_in_a_table_cell_or_a_content_control_keeps_its_words() {
    let mut word = Word::with_body(
        heading("Chapter One")
            + &p("Before the table.")
            + &table(&[
                cell(&p("Left cell")),
                cell(
                    r#"<w:p><w:r><w:t>Cell words</w:t></w:r><w:r><w:commentReference w:id="0"/></w:r></w:p>"#,
                ),
            ])
            + r#"<w:sdt><w:sdtPr><w:alias w:val="Box"/></w:sdtPr><w:sdtContent><w:p><w:r><w:t>Inside a control.</w:t></w:r><w:r><w:commentReference w:id="1"/></w:r></w:p></w:sdtContent></w:sdt>"#
            + &heading("Chapter Two")
            + &p("Last paragraph of the book."),
    );
    word.comments = vec![
        comment(0, "About the cell."),
        comment(1, "About the control."),
    ];

    let plan = plan_of("cell.docx", &word.bytes());
    let one = row(&plan, "Chapter One");
    let bodies: Vec<(String, String)> = (0..one.comments.len())
        .map(|i| (one.comments[i].body.clone(), anchored_words(one, i)))
        .collect();
    assert_eq!(
        bodies,
        vec![
            ("About the cell.".to_string(), "Cell words".to_string()),
            (
                "About the control.".to_string(),
                "Inside a control.".to_string()
            ),
        ]
    );
    assert_eq!(unanchored_in(one), 0, "{:?}", one.diagnostics);
    assert!(row(&plan, "Chapter Two").comments.is_empty());
}

/// A point comment after a line break belongs to the line it follows, the second one.
#[test]
fn a_point_comment_after_a_line_break_stays_on_its_line() {
    let mut word = Word::with_body(concat!(
        r#"<w:p><w:r><w:t>Line A</w:t><w:br/><w:t>Line B</w:t></w:r>"#,
        r#"<w:r><w:commentReference w:id="5"/></w:r></w:p>"#,
    ));
    word.comments = vec![comment(5, "On line B.")];
    let plan = plan_of("lines.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(text_of(row), "Line A\nLine B");
    assert_eq!(anchored_words(row, 0), "Line B");
    assert_eq!(unanchored_in(row), 0);
}

/// A comment on an empty line of a paragraph that has words on another stays in that
/// paragraph, and nothing is reported: at the start of the line after it when it opens the
/// paragraph, and at the end of the line before it otherwise. LibreOffice writes a comment
/// made at the top of a page before the paragraph's leading break, and Word an empty range
/// before its page break. Both went to the paragraph before, which may close another scene,
/// and were reported as moved.
#[test]
fn a_comment_on_an_empty_line_of_a_paragraph_with_words_stays_in_it() {
    let mut word = Word::with_body(
        heading("Chapter One")
            + &p("The chapter before.")
            + &heading("Chapter Two")
            // As LibreOffice writes it.
            + r#"<w:p><w:r><w:rPr></w:rPr><w:commentReference w:id="0"/></w:r><w:r><w:rPr></w:rPr><w:br/><w:t>After a leading break.</w:t></w:r></w:p>"#
            // As Word writes it.
            + r#"<w:p><w:commentRangeStart w:id="1"/><w:commentRangeEnd w:id="1"/><w:r><w:commentReference w:id="1"/></w:r><w:r><w:br w:type="page"/></w:r><w:r><w:t>New page text.</w:t></w:r></w:p>"#
            // An empty line between two others, and one ending the paragraph.
            + r#"<w:p><w:r><w:t>Line A</w:t><w:br/></w:r><w:r><w:commentReference w:id="2"/></w:r><w:r><w:br/><w:t>Line B</w:t></w:r></w:p>"#
            + r#"<w:p><w:r><w:t>Last line.</w:t><w:br/></w:r><w:commentRangeStart w:id="3"/><w:commentRangeEnd w:id="3"/><w:r><w:commentReference w:id="3"/></w:r></w:p>"#,
    );
    word.comments = vec![
        comment(0, "At the top of the page."),
        comment(1, "An empty range at the top."),
        comment(2, "On the empty line."),
        comment(3, "After the last line."),
    ];

    let plan = plan_of("leading-break.docx", &word.bytes());
    assert!(row(&plan, "Chapter One").comments.is_empty(), "{plan:#?}");
    let two = row(&plan, "Chapter Two");
    let placed: Vec<(String, String)> = (0..two.comments.len())
        .map(|i| (two.comments[i].body.clone(), anchored_words(two, i)))
        .collect();
    assert_eq!(
        placed,
        vec![
            (
                "At the top of the page.".to_string(),
                "After a leading break.".to_string()
            ),
            (
                "An empty range at the top.".to_string(),
                "New page text.".to_string()
            ),
            ("On the empty line.".to_string(), "Line A".to_string()),
            ("After the last line.".to_string(), "Last line.".to_string()),
        ]
    );
    assert_eq!(unanchored_in(two), 0, "{:?}", two.diagnostics);
}

/// The same lines in LibreOffice's own format.
#[test]
fn an_odt_comment_on_an_empty_line_of_a_paragraph_with_words_stays_in_it() {
    let body = r#"
      <text:h text:outline-level="1">Chapter One</text:h>
      <text:p>The chapter before.</text:p>
      <text:h text:outline-level="1">Chapter Two</text:h>
      <text:p><office:annotation><dc:creator>Editor</dc:creator><text:p>At the top of the page.</text:p></office:annotation><text:line-break/>After a leading break.</text:p>
      <text:p><office:annotation office:name="__Annotation__7_1"><dc:creator>Editor</dc:creator><text:p>An empty range at the top.</text:p></office:annotation><office:annotation-end office:name="__Annotation__7_1"/><text:line-break/>New page text.</text:p>
      <text:p>Line A<text:line-break/><office:annotation><dc:creator>Editor</dc:creator><text:p>On the empty line.</text:p></office:annotation><text:line-break/>Line B</text:p>"#;
    let plan = plan_of("leading-break.fodt", fodt(body).as_bytes());
    assert!(row(&plan, "Chapter One").comments.is_empty(), "{plan:#?}");
    let two = row(&plan, "Chapter Two");
    let placed: Vec<(String, String)> = (0..two.comments.len())
        .map(|i| (two.comments[i].body.clone(), anchored_words(two, i)))
        .collect();
    assert_eq!(
        placed,
        vec![
            (
                "At the top of the page.".to_string(),
                "After a leading break.".to_string()
            ),
            (
                "An empty range at the top.".to_string(),
                "New page text.".to_string()
            ),
            ("On the empty line.".to_string(), "Line A".to_string()),
        ]
    );
    assert_eq!(unanchored_in(two), 0, "{:?}", two.diagnostics);
}

// ── notes ───────────────────────────────────────────────────────────────────────────

/// Word numbers footnotes and endnotes separately, both from 1. The endnote was looked up
/// among the footnotes and cited footnote 1's text; its own was lost.
#[test]
fn an_endnote_keeps_its_own_text_beside_a_footnote_of_the_same_number() {
    let mut word = Word::with_body(format!(
        r#"<w:p><w:r><w:t>A claim</w:t></w:r>{}<w:r><w:t xml:space="preserve"> and another</w:t></w:r>{}<w:r><w:t>.</w:t></w:r></w:p>"#,
        note_reference("footnote", 1),
        note_reference("endnote", 1),
    ));
    word.footnotes = vec!["The footnote.".into()];
    word.endnotes = vec!["The endnote.".into()];

    let plan = plan_of("notes.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(text_of(row), "A claim\u{FFFC} and another\u{FFFC}.");
    let notes: Vec<&str> = row.footnotes.iter().map(|f| f.body.as_str()).collect();
    assert_eq!(notes, vec!["The footnote.", "The endnote."]);
    assert!(
        !plan
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FootnoteNotCarried { .. })),
        "{:?}",
        plan.diagnostics
    );

    // A document with endnotes alone lost every one of them.
    let mut word = Word::with_body(format!(
        r#"<w:p><w:r><w:t>Only an endnote here</w:t></w:r>{}<w:r><w:t>.</w:t></w:r></w:p>"#,
        note_reference("endnote", 1)
    ));
    word.endnotes = vec!["Its own text.".into()];
    let plan = plan_of("endnotes.docx", &word.bytes());
    let row = only_row(&plan);
    let notes: Vec<&str> = row.footnotes.iter().map(|f| f.body.as_str()).collect();
    assert_eq!(notes, vec!["Its own text."]);
    assert_eq!(text_of(row), "Only an endnote here\u{FFFC}.");
}

/// A picture is one character to the prose, and to the pass that places a note after it.
/// That pass counted none, so a note after a picture landed a character early, before the
/// full stop. The picture's source is the part it names, not a relationship id.
#[test]
fn a_note_after_a_picture_lands_where_it_was_cited() {
    const PNG: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x44, 0x41, 0x54, 0x78, 0xDA, 0x63, 0x64,
        0x60, 0xF8, 0x5F, 0x0F, 0x00, 0x02, 0x87, 0x01, 0x80, 0xEB, 0x47, 0xBA, 0x92, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let picture = concat!(
        r#"<w:r><w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0"><wp:extent cx="952500" cy="952500"/>"#,
        r#"<wp:docPr id="1" name="Picture 1"/><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">"#,
        r#"<pic:pic><pic:nvPicPr><pic:cNvPr id="0" name="image1.png"/><pic:cNvPicPr/></pic:nvPicPr>"#,
        r#"<pic:blipFill><a:blip r:embed="rId20"/><a:stretch><a:fillRect/></a:stretch></pic:blipFill>"#,
        r#"<pic:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="952500" cy="952500"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></pic:spPr>"#,
        r#"</pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r>"#,
    );
    let mut word = Word::with_body(format!(
        r#"<w:p><w:r><w:t xml:space="preserve">See </w:t></w:r>{picture}<w:r><w:t xml:space="preserve"> the figure.</w:t></w:r>{}<w:r><w:t xml:space="preserve"> Then more.</w:t></w:r></w:p>"#,
        note_reference("footnote", 1)
    ));
    word.footnotes = vec!["Figure source.".into()];
    word.relationships =
        format!(r#"<Relationship Id="rId20" Type="{TYPE}/image" Target="media/image1.png"/>"#);
    word.media = vec![("word/media/image1.png", PNG)];

    let plan = plan_of("figure.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(text_of(row), "See \u{FFFC} the figure.\u{FFFC} Then more.");
    assert!(
        row.djot.contains("![](word/media/image1.png)"),
        "the picture names its part: {}",
        row.djot
    );
    assert!(
        plan.diagnostics.iter().any(|d| matches!(
            d,
            ImportDiagnostic::ImageNotIngested { target, .. } if target == "word/media/image1.png"
        )),
        "{:?}",
        plan.diagnostics
    );
}

/// A paragraph's own tab stops are `<w:tab>` elements too, inside its `<w:pPr>`, and a text
/// box's words sit inside its drawing: neither is text the prose holds. The pass that
/// places a note counted both, and the note landed that many characters late.
#[test]
fn tab_stops_and_a_text_box_do_not_move_a_note() {
    let text_box = concat!(
        r#"<w:r><mc:AlternateContent><mc:Choice Requires="wps"><w:drawing><wp:inline><wp:extent cx="914400" cy="457200"/><wp:docPr id="2" name="Text Box 2"/>"#,
        r#"<a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp>"#,
        r#"<wps:txbx><w:txbxContent><w:p><w:r><w:t>Boxed words</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr/>"#,
        r#"</wps:wsp></a:graphicData></a:graphic></wp:inline></w:drawing></mc:Choice>"#,
        r#"<mc:Fallback><w:pict><v:shape><v:textbox><w:txbxContent><w:p><w:r><w:t>Boxed words</w:t></w:r></w:p></w:txbxContent></v:textbox></v:shape></w:pict></mc:Fallback>"#,
        r#"</mc:AlternateContent></w:r>"#,
    );
    // A picture set inside a text box is the box's, not the paragraph's: the box is left
    // out whole, the picture with it, and counts for nothing either.
    let boxed_picture = concat!(
        r#"<w:r><w:drawing><wp:inline><wp:extent cx="914400" cy="457200"/><wp:docPr id="3" name="Text Box 3"/>"#,
        r#"<a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp>"#,
        r#"<wps:txbx><w:txbxContent><w:p><w:r><w:drawing><wp:inline><wp:extent cx="952500" cy="952500"/><wp:docPr id="4" name="Picture 4"/>"#,
        r#"<a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic>"#,
        r#"<pic:nvPicPr><pic:cNvPr id="0" name="image1.png"/><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:embed="rId20"/></pic:blipFill><pic:spPr/>"#,
        r#"</pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr/>"#,
        r#"</wps:wsp></a:graphicData></a:graphic></wp:inline></w:drawing></w:r>"#,
    );
    let mut word = Word::with_body(format!(
        concat!(
            r#"<w:p><w:pPr><w:tabs><w:tab w:val="left" w:pos="1440"/><w:tab w:val="left" w:pos="2880"/></w:tabs></w:pPr>"#,
            r#"<w:r><w:t>Tab stops set</w:t></w:r>{}<w:r><w:t xml:space="preserve"> here.</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:t xml:space="preserve">A box </w:t></w:r>{}<w:r><w:t>then words</w:t></w:r>{}<w:r><w:t>.</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:t xml:space="preserve">A framed picture </w:t></w:r>{}<w:r><w:t>then words</w:t></w:r>{}<w:r><w:t>.</w:t></w:r></w:p>"#,
        ),
        note_reference("footnote", 1),
        text_box,
        note_reference("footnote", 2),
        boxed_picture,
        note_reference("footnote", 3),
    ));
    word.footnotes = vec!["First.".into(), "Second.".into(), "Third.".into()];
    word.relationships =
        format!(r#"<Relationship Id="rId20" Type="{TYPE}/image" Target="media/image1.png"/>"#);

    let plan = plan_of("stops.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(
        text_of(row),
        "Tab stops set\u{FFFC} here.\nA box then words\u{FFFC}.\nA framed picture then words\u{FFFC}."
    );
}

/// A producer's choice between two spellings of the same runs (`mc:AlternateContent`) set
/// between runs rather than inside one: `docx-rs` reads the runs of both the choice and the
/// fallback there, and only inside a run does it skip the fallback. The pass that places a
/// note skipped the fallback everywhere, and a note after it landed that many characters
/// early, inside the word before it.
#[test]
fn a_note_after_a_choice_between_runs_lands_where_it_was_cited() {
    let mut word = Word::with_body(format!(
        concat!(
            r#"<w:p><w:r><w:t xml:space="preserve">Alpha </w:t></w:r>"#,
            r#"<mc:AlternateContent><mc:Choice Requires="w14"><w:r><w:t>X</w:t></w:r></mc:Choice>"#,
            r#"<mc:Fallback><w:r><w:t>X</w:t></w:r></mc:Fallback></mc:AlternateContent>"#,
            r#"<w:r><w:t xml:space="preserve"> beta gamma</w:t></w:r>{}<w:r><w:t xml:space="preserve"> delta.</w:t></w:r></w:p>"#,
        ),
        note_reference("footnote", 1),
    ));
    word.footnotes = vec!["The note.".into()];

    let plan = plan_of("choice.docx", &word.bytes());
    let row = only_row(&plan);
    let text = text_of(row);
    assert!(
        text.ends_with(" beta gamma\u{FFFC} delta."),
        "the note follows the word it was cited after: {text:?}"
    );
    assert_eq!(row.footnotes.len(), 1, "{:#?}", row.footnotes);
}

/// An equation (Office Math, `m:oMath`) is not text a manuscript holds: it arrives as
/// nothing, and the wizard counts it with the embedded objects, as it counts a formula in
/// an OpenDocument file. It vanished without a word, and a note cited after it moved to
/// the end of its paragraph: the pass that places a note counted the equation's
/// characters, which the prose does not hold.
#[test]
fn an_equation_is_reported_and_leaves_a_note_where_it_was_cited() {
    let inline = r#"<m:oMath><m:r><w:rPr><w:rFonts w:ascii="Cambria Math" w:hAnsi="Cambria Math"/></w:rPr><m:t>x=1</m:t></m:r></m:oMath>"#;
    let display = concat!(
        r#"<w:p><m:oMathPara><m:oMathParaPr><m:jc m:val="centerGroup"/></m:oMathParaPr><m:oMath>"#,
        r#"<m:r><m:t>E=m</m:t></m:r><m:sSup><m:e><m:r><m:t>c</m:t></m:r></m:e><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSup>"#,
        r#"</m:oMath></m:oMathPara></w:p>"#,
    );
    // Removed as a tracked change: somebody's deleted work, neither text nor reported.
    let deleted = concat!(
        r#"<w:p><w:r><w:t xml:space="preserve">Kept </w:t></w:r><w:del w:id="9" w:author="Editor">"#,
        r#"<m:oMath><m:r><w:rPr><w:del w:id="10" w:author="Editor"/></w:rPr><m:t>y=2</m:t></m:r></m:oMath></w:del>"#,
        r#"<w:r><w:t>words</w:t></w:r>{}<w:r><w:t>.</w:t></w:r></w:p>"#,
    );
    let mut word = Word::with_body(format!(
        concat!(
            r#"<w:p><w:r><w:t xml:space="preserve">Solve </w:t></w:r>{}"#,
            r#"<w:r><w:t xml:space="preserve"> now</w:t></w:r>{}<w:r><w:t xml:space="preserve">, then rest.</w:t></w:r></w:p>"#,
            r#"{}{}"#,
        ),
        inline,
        note_reference("footnote", 1),
        display,
        deleted.replace("{}", &note_reference("footnote", 2)),
    ));
    word.footnotes = vec!["First.".into(), "Second.".into()];

    let plan = plan_of("equations.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(
        text_of(row),
        "Solve  now\u{FFFC}, then rest.\nKept words\u{FFFC}."
    );
    let objects: Vec<usize> = plan
        .diagnostics
        .iter()
        .filter_map(|d| match d {
            ImportDiagnostic::EmbeddedObjectDropped { count, .. } => Some(*count),
            _ => None,
        })
        .collect();
    assert_eq!(objects, vec![2], "{:?}", plan.diagnostics);
}

/// A note alone on the empty stretch a line break leaves, after the paragraph's last line
/// or before its first, stays in its paragraph: at the end of the line before it, or at the
/// start of the line after it when nothing comes before. It was dropped and reported,
/// although the same document read from OpenDocument carried it.
#[test]
fn a_note_alone_beside_a_line_break_stays_in_its_paragraph() {
    let mut word = Word::with_body(format!(
        concat!(
            // As LibreOffice writes a note after a line break: the break in a run of its
            // own, then the reference.
            r#"<w:p><w:r><w:t>Verse line one.</w:t></w:r><w:r><w:br/></w:r>{}</w:p>"#,
            // As Word writes a note before one: the reference, then the break and the words
            // in one run.
            r#"<w:p>{}<w:r><w:br/><w:t>Text after the break.</w:t></w:r></w:p>"#,
            // Two breaks with the note between them: the line before it holds it.
            r#"<w:p><w:r><w:t>Before.</w:t><w:br/></w:r>{}<w:r><w:br/><w:t>After.</w:t></w:r></w:p>"#,
        ),
        note_reference("footnote", 1),
        note_reference("footnote", 2),
        note_reference("footnote", 3),
    ));
    word.footnotes = vec![
        "Alone after the break.".into(),
        "Before the break.".into(),
        "Between two breaks.".into(),
    ];

    let plan = plan_of("breaks.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(
        text_of(row),
        "Verse line one.\u{FFFC}\n\u{FFFC}Text after the break.\nBefore.\u{FFFC}\nAfter."
    );
    let notes: Vec<&str> = row.footnotes.iter().map(|f| f.body.as_str()).collect();
    assert_eq!(
        notes,
        vec![
            "Alone after the break.",
            "Before the break.",
            "Between two breaks."
        ]
    );
    assert!(
        !plan
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FootnoteNotCarried { .. })),
        "{:?}",
        plan.diagnostics
    );
}

/// Two notes with no line of their own beside a line break keep the order they were
/// cited in. Moved to the end of the line before, the second went first: the first note
/// was measured past the reference of the second, already in place.
#[test]
fn two_notes_beside_a_line_break_keep_their_order() {
    let mut word = Word::with_body(format!(
        concat!(
            // As LibreOffice writes two references after a line break ending a line.
            r#"<w:p><w:r><w:t>Verse line one.</w:t></w:r><w:r><w:br/></w:r>{}{}</w:p>"#,
            // As Word writes a note on each of two empty lines.
            r#"<w:p><w:r><w:t>Verse line two.</w:t><w:br/></w:r>{}<w:r><w:br/></w:r>{}</w:p>"#,
            // One note at the end of the line, the next after the break that follows it.
            r#"<w:p><w:r><w:t>Verse line three.</w:t></w:r>{}<w:r><w:br/></w:r>{}</w:p>"#,
            // Two references before a line break opening the paragraph.
            r#"<w:p>{}{}<w:r><w:br/><w:t>Verse line four.</w:t></w:r></w:p>"#,
        ),
        note_reference("footnote", 1),
        note_reference("footnote", 2),
        note_reference("footnote", 3),
        note_reference("footnote", 4),
        note_reference("footnote", 5),
        note_reference("footnote", 6),
        note_reference("footnote", 7),
        note_reference("footnote", 8),
    ));
    word.footnotes = (1..=8).map(|n| format!("Note {n}.")).collect();

    let plan = plan_of("verse.docx", &word.bytes());
    let row = only_row(&plan);
    for line in [
        "Verse line one.[^srcfn-1][^srcfn-2]",
        "Verse line two.[^srcfn-3][^srcfn-4]",
        "Verse line three.[^srcfn-5][^srcfn-6]",
        "[^srcfn-7][^srcfn-8]Verse line four.",
    ] {
        assert!(
            row.djot.lines().any(|l| l == line),
            "{line:?} in {}",
            row.djot
        );
    }
    let notes: Vec<&str> = row.footnotes.iter().map(|f| f.body.as_str()).collect();
    let expected: Vec<String> = (1..=8).map(|n| format!("Note {n}.")).collect();
    assert_eq!(notes, expected);
    assert!(
        !plan
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FootnoteNotCarried { .. })),
        "{:?}",
        plan.diagnostics
    );
}

/// A note whose paragraph holds nothing else has no line to stand beside, and is reported
/// rather than moved into another paragraph.
#[test]
fn a_note_alone_in_its_paragraph_is_reported() {
    let mut word = Word::with_body(
        p("The passage.")
            + &format!(
                r#"<w:p>{}<w:r><w:br/></w:r></w:p>"#,
                note_reference("footnote", 1)
            )
            + &p("After."),
    );
    word.footnotes = vec!["Nowhere to stand.".into()];
    let plan = plan_of("alone.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(text_of(row), "The passage.\nAfter.");
    assert!(
        plan.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FootnoteNotCarried { count: 1, .. })),
        "{:?}",
        plan.diagnostics
    );
}

/// Word's horizontal rule is an empty paragraph with a border under it, which arrives as a
/// scene break. A comment made on it, as a bare reference or as Word's empty range, stays on
/// that break, in its chapter. It went to the last paragraph of the whole book. A note cited
/// from a rule has no words to stand beside, and is reported rather than lost unsaid.
#[test]
fn a_comment_on_a_horizontal_rule_stays_on_the_break() {
    let rule = |content: &str| {
        format!(
            r#"<w:p><w:pPr><w:pBdr><w:bottom w:val="single" w:sz="6" w:space="1" w:color="auto"/></w:pBdr></w:pPr>{content}</w:p>"#
        )
    };
    let mut word = Word::with_body(
        heading("Chapter One")
            + &p("The passage.")
            + &rule(r#"<w:r><w:commentReference w:id="0"/></w:r>"#)
            + &p("Between the rules.")
            + &rule(
                r#"<w:commentRangeStart w:id="1"/><w:commentRangeEnd w:id="1"/><w:r><w:rPr><w:rStyle w:val="CommentReference"/></w:rPr><w:commentReference w:id="1"/></w:r>"#,
            )
            + &p("After the rules.")
            + &rule(&note_reference("footnote", 1))
            + &heading("Chapter Two")
            + &p("Last paragraph of the book."),
    );
    word.comments = vec![
        comment(0, "On the first rule."),
        comment(1, "On the second rule."),
    ];
    word.footnotes = vec!["On a rule.".into()];

    let plan = plan_of("rules.docx", &word.bytes());
    let one = row(&plan, "Chapter One");
    assert_eq!(
        text_of(one),
        "The passage.\n* * *\nBetween the rules.\n* * *\nAfter the rules.\n* * *"
    );
    assert!(
        plan.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FootnoteNotCarried { count: 1, .. })),
        "{:?}",
        plan.diagnostics
    );
    let placed: Vec<(String, usize)> = one
        .comments
        .iter()
        .map(|c| (c.body.clone(), c.anchor.start))
        .collect();
    assert_eq!(
        placed,
        vec![
            ("On the first rule.".to_string(), "The passage.\n".len()),
            (
                "On the second rule.".to_string(),
                "The passage.\n* * *\nBetween the rules.\n".len()
            ),
        ],
        "each on its own break"
    );
    for index in 0..2 {
        assert_eq!(anchored_words(one, index), "* * *");
    }
    assert_eq!(unanchored_in(one), 0, "{:?}", one.diagnostics);
    assert!(row(&plan, "Chapter Two").comments.is_empty());
}

/// A note cited inside a table cell arrives cited from that cell. The pass that finds
/// references read no table, so it was lost without a word.
#[test]
fn a_note_in_a_table_cell_is_carried() {
    let mut word = Word::with_body(
        p("Before.")
            + &table(&[
                cell(&p("Plain cell")),
                cell(&format!(
                    r#"<w:p><w:r><w:t>Cell with a note</w:t></w:r>{}</w:p>"#,
                    note_reference("footnote", 1)
                )),
            ])
            + &p("After."),
    );
    word.footnotes = vec!["From the table.".into()];
    let plan = plan_of("cell-note.docx", &word.bytes());
    let row = only_row(&plan);
    assert!(
        row.djot.contains("Cell with a note[^srcfn-1]"),
        "{}",
        row.djot
    );
    let notes: Vec<&str> = row.footnotes.iter().map(|f| f.body.as_str()).collect();
    assert_eq!(notes, vec!["From the table."]);
}

// ── text docx-rs has no reading for ─────────────────────────────────────────────────

/// Word writes a no-break hyphen and a soft hyphen as elements of their own. They were
/// dropped, joining the words either side. In a tracked deletion they are removed text,
/// like the rest of it; in a comment or a note they are that text's own.
#[test]
fn no_break_and_soft_hyphens_stay_in_their_words() {
    let mut word = Word::with_body(format!(
        concat!(
            r#"<w:p><w:r><w:t>She was twenty</w:t><w:noBreakHyphen/><w:t>one, from Aix</w:t></w:r>"#,
            r#"<w:r><w:noBreakHyphen/></w:r><w:r><w:t>en</w:t></w:r><w:r><w:noBreakHyphen/></w:r>"#,
            r#"<w:r><w:t>Provence, a hyphen</w:t><w:softHyphen/><w:t>ation</w:t></w:r>{}"#,
            r#"<w:commentRangeStart w:id="0"/><w:r><w:t xml:space="preserve"> twenty</w:t><w:noBreakHyphen/><w:t>two</w:t></w:r>"#,
            r#"<w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r><w:r><w:t>.</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:t>Kept</w:t></w:r><w:del w:id="9" w:author="Editor" w:date="2026-03-04T05:06:07Z">"#,
            r#"<w:r><w:delText>-cut</w:delText><w:noBreakHyphen/><w:delText>off</w:delText></w:r></w:del><w:r><w:t>.</w:t></w:r></w:p>"#,
        ),
        note_reference("footnote", 1),
    ));
    word.comments = vec![concat!(
        r#"<w:comment w:id="0" w:author="Editor" w:date="2026-03-04T05:06:07Z" w:initials="E">"#,
        r#"<w:p><w:r><w:t>Check twenty</w:t><w:noBreakHyphen/><w:t>two.</w:t></w:r></w:p></w:comment>"#
    )
    .to_string()];
    word.footnotes = vec!["See pages 4</w:t><w:noBreakHyphen/><w:t>5.".into()];

    let plan = plan_of("hyphens.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(
        text_of(row),
        "She was twenty\u{2011}one, from Aix\u{2011}en\u{2011}Provence, a hyphen\u{AD}ation\u{FFFC} twenty\u{2011}two.\nKept.",
        "and the note after them lands where it was cited"
    );
    assert_eq!(anchored_words(row, 0), " twenty\u{2011}two");
    assert_eq!(row.comments[0].body, "Check twenty\u{2011}two.");
    assert_eq!(row.footnotes[0].body, "See pages 4\u{2011}5.");
}

/// Word's Insert ▸ Symbol writes a symbol from the Symbol font as `<w:sym>`, which arrived
/// as nothing at all: "α" was lost from the prose, from a comment and from a note, and the
/// note cited after it landed a character early. It arrives as the character it shows. A
/// symbol from a font of pictures arrives as the character the file names.
#[test]
fn a_symbol_arrives_as_the_character_it_shows() {
    let mut word = Word::with_body(format!(
        concat!(
            r#"<w:p><w:r><w:t xml:space="preserve">Angle </w:t><w:sym w:font="Symbol" w:char="F061"/>"#,
            r#"<w:t xml:space="preserve"> is </w:t></w:r><w:r><w:rPr><w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/></w:rPr>"#,
            r#"<w:sym w:font="Symbol" w:char="F0B3"/></w:r><w:r><w:t xml:space="preserve"> 90, </w:t></w:r>"#,
            r#"<w:commentRangeStart w:id="0"/><w:r><w:t xml:space="preserve">then </w:t><w:sym w:font="Wingdings" w:char="F0E0"/></w:r>"#,
            r#"<w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r>{}<w:r><w:t>.</w:t></w:r></w:p>"#,
        ),
        note_reference("footnote", 1),
    ));
    word.comments = vec![concat!(
        r#"<w:comment w:id="0" w:author="Editor" w:date="2026-03-04T05:06:07Z" w:initials="E">"#,
        r#"<w:p><w:r><w:t xml:space="preserve">Use </w:t><w:sym w:font="Symbol" w:char="F062"/><w:t>.</w:t></w:r></w:p></w:comment>"#
    )
    .to_string()];
    word.footnotes =
        vec![r#"Where </w:t><w:sym w:font="Symbol" w:char="F070"/><w:t> is pi."#.into()];

    let plan = plan_of("symbols.docx", &word.bytes());
    let row = only_row(&plan);
    assert_eq!(
        text_of(row),
        "Angle \u{3B1} is \u{2265} 90, then \u{F0E0}\u{FFFC}.",
        "and the note after them lands where it was cited"
    );
    assert_eq!(anchored_words(row, 0), "then \u{F0E0}");
    assert_eq!(row.comments[0].body, "Use \u{3B2}.");
    assert_eq!(row.footnotes[0].body, "Where \u{3C0} is pi.");
}

// ── tables and content controls ────────────────────────────────────────────────────

/// A table nested in a cell, a table inside a content control, and a content control
/// inside a cell: all three were dropped, without a word. Their text arrives, a nested
/// table's rows after the row holding it, as the ODT scanner reads the same table.
#[test]
fn text_in_nested_tables_and_content_controls_arrives() {
    let nested = table(&[cell(&p("NESTED TEXT"))]);
    let word = Word::with_body(
        p("Before.")
            + &table(&[cell(&p("Outer A")), cell(&(nested + &p("")))])
            + r#"<w:sdt><w:sdtPr><w:alias w:val="Box"/></w:sdtPr><w:sdtContent>"#
            + &table(&[cell(&p("TABLE IN CONTROL"))])
            + "</w:sdtContent></w:sdt>"
            + &table(&[cell(&format!(
                r#"<w:sdt><w:sdtPr/><w:sdtContent>{}</w:sdtContent></w:sdt>"#,
                p("CONTROL IN CELL")
            ))])
            + &p("After."),
    );
    let plan = plan_of("nested.docx", &word.bytes());
    let row = only_row(&plan);
    let text = text_of(row);
    for words in [
        "NESTED TEXT",
        "TABLE IN CONTROL",
        "CONTROL IN CELL",
        "Before.",
        "After.",
    ] {
        assert!(text.contains(words), "{words:?} in {text:?}");
    }
    // Each row is written with the cells it holds; the parser completes the nested row's
    // second cell itself.
    assert!(
        row.djot
            .contains("| Outer A |  |\n|---|---|\n| NESTED TEXT |\n"),
        "the nested table's row follows the row holding it: {}",
        row.djot
    );
}

// ── what the package itself holds ──────────────────────────────────────────────────

/// A main part with no relationships is a valid package, the smallest there is, and
/// LibreOffice reads it. It was refused as unreadable.
#[test]
fn a_package_with_no_document_relationships_is_read() {
    let word = Word {
        body: p("Hello from a minimal package."),
        minimal: true,
        ..Word::default()
    };
    let plan = plan_of("minimal.docx", &word.bytes());
    assert!(
        !plan
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FileUnreadable { .. })),
        "{:?}",
        plan.diagnostics
    );
    assert_eq!(text_of(only_row(&plan)), "Hello from a minimal package.");
}

// ── OpenDocument headings in a list ─────────────────────────────────────────────────

/// LibreOffice writes a heading numbered through a list inside the list, as it saves a
/// Word file whose headings carry numbering of their own. Read as list items, the
/// chapters were never made; the same file as `.docx` made them.
#[test]
fn an_odt_heading_numbered_through_a_list_is_still_a_heading() {
    let body = r#"
      <text:list text:style-name="WWNum1">
        <text:list-item>
          <text:h text:outline-level="1">First chapter</text:h>
        </text:list-item>
      </text:list>
      <text:p>Body of the first chapter.</text:p>
      <text:list text:style-name="WWNum1" text:continue-numbering="true">
        <text:list-item>
          <text:p text:style-name="Chapter_20_title">Second chapter</text:p>
        </text:list-item>
      </text:list>
      <text:p>Body of the second chapter.</text:p>
      <text:list>
        <text:list-item><text:p>An ordinary item.</text:p></text:list-item>
      </text:list>"#;
    let plan = plan_of("numbered.fodt", fodt(body).as_bytes());
    let titles: Vec<&str> = plan.rows.iter().map(|row| row.title.as_str()).collect();
    assert_eq!(titles, vec!["First chapter", "Second chapter"], "{plan:#?}");
    assert_eq!(text_of(&plan.rows[0]), "Body of the first chapter.");
    assert_eq!(
        text_of(&plan.rows[1]),
        "Body of the second chapter.\nAn ordinary item.",
        "an ordinary paragraph in a list is still a list item"
    );
    assert!(
        plan.rows[1].djot.contains("- An ordinary item."),
        "{}",
        plan.rows[1].djot
    );
}
