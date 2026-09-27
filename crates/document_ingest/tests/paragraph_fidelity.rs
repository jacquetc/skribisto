// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a Word or OpenDocument paragraph brings with it, read from files built here.
//!
//! Every file is spelled out in this module, byte for byte, in the shapes Word and
//! LibreOffice write: a double space as it is typed, a tab as `<w:tab/>` or `<text:tab/>`,
//! alignment through a paragraph style, a page break as its own paragraph. None of these
//! would survive a trip through a writer that normalises its input, which is why the files
//! are not produced by one.
//!
//! What is checked is what the editor will read: every row's Djot goes through
//! `text-document`, the parser the editor opens prose with.

use std::io::Write;
use std::path::Path;

use common::entities::{ChapterMode, CommentAnchorKind};
use document_ingest::plan::ImportPlan;
use document_ingest::{ScannerRegistry, scan_and_plan};
use skribisto_model::CreateType;
use text_document::{
    Alignment, CharVerticalAlignment, FragmentContent, TextDirection, TextDocument,
};

const NS: &str = concat!(
    r#"xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" "#,
    r#"xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" "#,
    r#"xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" "#,
    r#"xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" "#,
    r#"xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture""#,
);

/// A `.docx` holding `body` as its document, with `styles` and `comments` as their parts.
fn docx(body: &str, styles: &str, comments: &str) -> Vec<u8> {
    let parts = [
        (
            "[Content_Types].xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/><Override PartName="/word/comments.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"/></Types>"#
                .to_string(),
        ),
        (
            "_rels/.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#
                .to_string(),
        ),
        (
            "word/_rels/document.xml.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" Target="comments.xml"/></Relationships>"#
                .to_string(),
        ),
        (
            "word/document.xml",
            format!(
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document {NS}><w:body>{body}</w:body></w:document>"#
            ),
        ),
        (
            "word/styles.xml",
            format!(
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles {NS}><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style>{styles}</w:styles>"#
            ),
        ),
        (
            "word/comments.xml",
            format!(
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:comments {NS}>{comments}</w:comments>"#
            ),
        ),
    ];
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, content) in parts {
        zip.start_file(name, options).expect("start a part");
        zip.write_all(content.as_bytes()).expect("write a part");
    }
    zip.finish().expect("finish the container").into_inner()
}

/// A comment on the range its id brackets.
fn docx_comment(id: usize, text: &str) -> String {
    format!(
        r#"<w:comment w:id="{id}" w:author="Editor" w:date="2026-01-02T03:04:05Z"><w:p><w:r><w:t>{text}</w:t></w:r></w:p></w:comment>"#
    )
}

/// A flat OpenDocument file: `automatic` styles and `body` paragraphs.
fn fodt(automatic: &str, body: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0"
  xmlns:dc="http://purl.org/dc/elements/1.1/"
  xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0"
  xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0"
  xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  office:mimetype="application/vnd.oasis.opendocument.text">
  <office:automatic-styles>{automatic}</office:automatic-styles>
  <office:body>
    <office:text>
{body}
    </office:text>
  </office:body>
</office:document>"#
    )
}

fn plan_of(name: &str, bytes: &[u8]) -> ImportPlan {
    let registry = ScannerRegistry::with_builtin_scanners();
    scan_and_plan(
        &registry,
        &[(Path::new(name).to_path_buf(), bytes.to_vec())],
        CreateType::Chapter,
        ChapterMode::Folder,
        0,
    )
}

/// The one row a heading-free file becomes, and what the editor reads its Djot as.
fn only_row(plan: &ImportPlan) -> (&document_ingest::PlannedRow, String) {
    assert_eq!(
        plan.rows.len(),
        1,
        "one file, no heading, one row: {plan:#?}"
    );
    let row = &plan.rows[0];
    let text = skrib_format::djot_plain_text(&row.djot)
        .expect("the stored Djot parses")
        .0;
    (row, text)
}

/// The characters of `text` a comment's anchor covers.
fn anchored(text: &str, start: usize, length: usize) -> String {
    text.chars().skip(start).take(length).collect()
}

fn open(djot: &str) -> TextDocument {
    let doc = TextDocument::new();
    let parsed = doc.set_djot(djot).and_then(|op| op.wait());
    assert!(parsed.is_ok(), "{djot:?} did not parse: {parsed:?}");
    doc
}

// ── comments after a double space and a tab ────────────────────────────────────────

/// The defect this module pins: a double space and a tab early in the file shifted every
/// later paragraph, and each comment there fell back to the whole document.
#[test]
fn docx_comments_after_a_double_space_and_a_tab_keep_their_words() {
    let body = r#"
<w:p><w:r><w:t xml:space="preserve">One.  Two.</w:t></w:r><w:r><w:tab/><w:t>Three.</w:t></w:r></w:p>
<w:p><w:r><w:tab/><w:t xml:space="preserve">Indented,  with a double space.</w:t></w:r></w:p>
<w:p><w:r><w:t xml:space="preserve">She </w:t></w:r><w:commentRangeStart w:id="0"/><w:r><w:t>turned</w:t></w:r><w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r><w:r><w:t xml:space="preserve"> the corner.</w:t></w:r></w:p>
<w:p><w:r><w:t xml:space="preserve">The fog had not lifted, and </w:t></w:r><w:commentRangeStart w:id="1"/><w:r><w:t>nobody</w:t></w:r><w:commentRangeEnd w:id="1"/><w:r><w:commentReference w:id="1"/></w:r><w:r><w:t xml:space="preserve"> said a word.</w:t></w:r></w:p>"#;
    let comments = docx_comment(0, "Which way?") + &docx_comment(1, "Really nobody?");
    let plan = plan_of("chapter.docx", &docx(body, "", &comments));
    let (row, text) = only_row(&plan);

    assert_eq!(
        text,
        "One.  Two. Three.\nIndented,  with a double space.\nShe turned the corner.\nThe fog had not lifted, and nobody said a word.",
        "the double spaces are kept, and the leading tab is the paragraph's indent"
    );
    let words: Vec<(CommentAnchorKind, bool, String)> = row
        .comments
        .iter()
        .map(|c| {
            (
                c.kind.clone(),
                c.orphaned,
                anchored(&text, c.anchor.start, c.anchor.length),
            )
        })
        .collect();
    assert_eq!(
        words,
        vec![
            (CommentAnchorKind::Range, false, "turned".to_string()),
            (CommentAnchorKind::Range, false, "nobody".to_string()),
        ]
    );
}

#[test]
fn odt_comments_after_a_double_space_and_a_tab_keep_their_words() {
    let body = r#"
      <text:p>One. <text:s/>Two.<text:tab/>Three.</text:p>
      <text:p><text:tab/>Indented, <text:s/>with a double space.</text:p>
      <text:p>She <office:annotation office:name="a0">
          <dc:creator>Editor</dc:creator>
          <text:p>Which way?</text:p>
        </office:annotation>turned<office:annotation-end office:name="a0"/> the corner.</text:p>
      <text:p>The fog had not lifted, and <office:annotation office:name="a1">
          <dc:creator>Editor</dc:creator>
          <text:p>Really nobody?</text:p>
        </office:annotation>nobody<office:annotation-end office:name="a1"/> said a word.</text:p>"#;
    let plan = plan_of("chapter.fodt", fodt("", body).as_bytes());
    let (row, text) = only_row(&plan);

    assert_eq!(
        text,
        "One.  Two. Three.\nIndented,  with a double space.\nShe turned the corner.\nThe fog had not lifted, and nobody said a word."
    );
    let words: Vec<(CommentAnchorKind, bool, String)> = row
        .comments
        .iter()
        .map(|c| {
            (
                c.kind.clone(),
                c.orphaned,
                anchored(&text, c.anchor.start, c.anchor.length),
            )
        })
        .collect();
    assert_eq!(
        words,
        vec![
            (CommentAnchorKind::Range, false, "turned".to_string()),
            (CommentAnchorKind::Range, false, "nobody".to_string()),
        ]
    );
    // The comment's own text follows the same white-space rule: the file's indentation
    // around it is layout, not words.
    assert_eq!(row.comments[0].body, "Which way?");
}

// ── paragraph formatting ────────────────────────────────────────────────────────────

/// Alignment from a paragraph's own properties and from its style, page breaks in both
/// of Word's spellings, right-to-left paragraphs, and raised and lowered characters.
#[test]
fn docx_paragraph_formatting_reaches_the_editor() {
    let styles = r#"
<w:style w:type="paragraph" w:styleId="Centred"><w:basedOn w:val="Normal"/><w:pPr><w:jc w:val="center"/></w:pPr></w:style>
<w:style w:type="paragraph" w:styleId="ChapterStart"><w:basedOn w:val="Normal"/><w:pPr><w:pageBreakBefore/></w:pPr></w:style>"#;
    let body = r#"
<w:p><w:pPr><w:pStyle w:val="Centred"/></w:pPr><w:r><w:t>Centred by its style.</w:t></w:r></w:p>
<w:p><w:pPr><w:jc w:val="right"/></w:pPr><w:r><w:t>Set right.</w:t></w:r></w:p>
<w:p><w:pPr><w:jc w:val="both"/></w:pPr><w:r><w:t>Justified, which the export style decides.</w:t></w:r></w:p>
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
<w:p><w:r><w:t>After a page break of its own.</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="ChapterStart"/></w:pPr><w:r><w:t>A style that starts a page.</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="ChapterStart"/><w:pageBreakBefore w:val="0"/></w:pPr><w:r><w:t>The same style, turned off here.</w:t></w:r></w:p>
<w:p><w:pPr><w:bidi/><w:jc w:val="left"/></w:pPr><w:r><w:t>שלום עולם</w:t></w:r></w:p>
<w:p><w:pPr><w:bidi/><w:jc w:val="right"/></w:pPr><w:r><w:t>סוף</w:t></w:r></w:p>
<w:p><w:r><w:t>E=mc</w:t></w:r><w:r><w:rPr><w:vertAlign w:val="superscript"/></w:rPr><w:t>2</w:t></w:r><w:r><w:t xml:space="preserve"> and H</w:t></w:r><w:r><w:rPr><w:vertAlign w:val="subscript"/></w:rPr><w:t>2</w:t></w:r><w:r><w:t>O</w:t></w:r></w:p>"#;
    let plan = plan_of("chapter.docx", &docx(body, styles, ""));
    let (row, text) = only_row(&plan);
    assert_eq!(
        text,
        "Centred by its style.\nSet right.\nJustified, which the export style decides.\nAfter a page break of its own.\nA style that starts a page.\nThe same style, turned off here.\nשלום עולם\nסוף\nE=mc2 and H2O"
    );
    check_formatting(&row.djot);
}

#[test]
fn odt_paragraph_formatting_reaches_the_editor() {
    let automatic = r#"
    <style:style style:name="P1" style:family="paragraph"><style:paragraph-properties fo:text-align="center"/></style:style>
    <style:style style:name="P2" style:family="paragraph"><style:paragraph-properties fo:text-align="end"/></style:style>
    <style:style style:name="P3" style:family="paragraph"><style:paragraph-properties fo:text-align="justify"/></style:style>
    <style:style style:name="P4" style:family="paragraph"><style:paragraph-properties fo:break-after="page"/></style:style>
    <style:style style:name="P5" style:family="paragraph"><style:paragraph-properties fo:break-before="page"/></style:style>
    <style:style style:name="P6" style:family="paragraph" style:parent-style-name="P5"><style:paragraph-properties fo:break-before="auto"/></style:style>
    <style:style style:name="P7" style:family="paragraph"><style:paragraph-properties style:writing-mode="rl-tb" fo:text-align="start"/></style:style>
    <style:style style:name="P8" style:family="paragraph"><style:paragraph-properties style:writing-mode="rl-tb" fo:text-align="end"/></style:style>
    <style:style style:name="T1" style:family="text"><style:text-properties style:text-position="super 58%"/></style:style>
    <style:style style:name="T2" style:family="text"><style:text-properties style:text-position="-33% 100%"/></style:style>"#;
    let body = r#"
      <text:p text:style-name="P1">Centred by its style.</text:p>
      <text:p text:style-name="P2">Set right.</text:p>
      <text:p text:style-name="P3">Justified, which the export style decides.</text:p>
      <text:p text:style-name="P4">The page ends after this.</text:p>
      <text:p>After a page break of its own.</text:p>
      <text:p text:style-name="P5">A style that starts a page.</text:p>
      <text:p text:style-name="P6">The same style, turned off here.</text:p>
      <text:p text:style-name="P7">שלום עולם</text:p>
      <text:p text:style-name="P8">סוף</text:p>
      <text:p>E=mc<text:span text:style-name="T1">2</text:span> and H<text:span text:style-name="T2">2</text:span>O</text:p>"#;
    let plan = plan_of("chapter.fodt", fodt(automatic, body).as_bytes());
    let (row, text) = only_row(&plan);
    assert_eq!(
        text,
        "Centred by its style.\nSet right.\nJustified, which the export style decides.\nThe page ends after this.\nAfter a page break of its own.\nA style that starts a page.\nThe same style, turned off here.\nשלום עולם\nסוף\nE=mc2 and H2O"
    );
    check_formatting(&row.djot);
}

/// What both formatting fixtures must read as in the editor, found by each paragraph's
/// opening words.
fn check_formatting(djot: &str) {
    let doc = open(djot);
    let blocks = doc.blocks();
    let block = |opening: &str| {
        blocks
            .iter()
            .find(|b| b.text().starts_with(opening))
            .unwrap_or_else(|| panic!("no paragraph opens {opening:?} in {djot:?}"))
            .block_format()
    };

    assert_eq!(
        block("Centred").alignment,
        Some(Alignment::Center),
        "{djot}"
    );
    assert_eq!(
        block("Set right").alignment,
        Some(Alignment::Right),
        "{djot}"
    );
    assert_eq!(block("Justified").alignment, None, "{djot}");
    assert_eq!(
        block("After a page").page_break_before,
        Some(true),
        "{djot}"
    );
    assert_eq!(
        block("A style that").page_break_before,
        Some(true),
        "{djot}"
    );
    assert_eq!(block("The same style").page_break_before, None, "{djot}");
    for other in ["Centred", "Set right", "Justified"] {
        assert_eq!(block(other).page_break_before, None, "{other}: {djot}");
    }

    let hello = block("שלום");
    assert_eq!(hello.direction, Some(TextDirection::RightToLeft), "{djot}");
    assert_eq!(
        hello.alignment, None,
        "the start edge of a right-to-left paragraph is not stated: {djot}"
    );
    let end = block("סוף");
    assert_eq!(end.direction, Some(TextDirection::RightToLeft), "{djot}");
    assert_eq!(end.alignment, Some(Alignment::Left), "{djot}");

    let raised: Vec<(String, Option<CharVerticalAlignment>)> = blocks
        .iter()
        .find(|b| b.text().starts_with("E=mc"))
        .expect("the formula paragraph")
        .fragments()
        .into_iter()
        .filter_map(|f| match f {
            FragmentContent::Text { text, format, .. } => Some((text, format.vertical_alignment)),
            _ => None,
        })
        .filter(|(_, v)| v.is_some())
        .collect();
    assert_eq!(
        raised,
        vec![
            ("2".to_string(), Some(CharVerticalAlignment::SuperScript)),
            ("2".to_string(), Some(CharVerticalAlignment::SubScript)),
        ],
        "{djot}"
    );
}

// ── text the parser would rewrite ──────────────────────────────────────────────────

/// Typed punctuation arrives as typed: the parser curls straight quotes, turns `--` and
/// `...` into a dash and an ellipsis, drops `:30:` as a symbol and reads `I. ` as a list.
const TYPED: [&str; 5] = [
    "He said \"hi\" and 'bye'.",
    "Wait--no. Then...",
    "At 10:30:45 exactly.",
    "I. Introduction",
    "std::vector and a :b: c",
];

#[test]
fn docx_typed_punctuation_reads_back_as_typed() {
    let body: String = TYPED
        .iter()
        .map(|line| {
            format!(
                r#"<w:p><w:r><w:t xml:space="preserve">{}</w:t></w:r></w:p>"#,
                line.replace('&', "&amp;").replace('"', "&quot;")
            )
        })
        .collect();
    let plan = plan_of("chapter.docx", &docx(&body, "", ""));
    let (_, text) = only_row(&plan);
    assert_eq!(text, TYPED.join("\n"));
}

#[test]
fn odt_typed_punctuation_reads_back_as_typed() {
    let body: String = TYPED
        .iter()
        .map(|line| format!("<text:p>{}</text:p>\n", line.replace('&', "&amp;")))
        .collect();
    let plan = plan_of("chapter.fodt", fodt("", &body).as_bytes());
    let (_, text) = only_row(&plan);
    assert_eq!(text, TYPED.join("\n"));
}

// ── ODF white space ─────────────────────────────────────────────────────────────────

/// Character data follows ODF's white-space rule: the indentation of a pretty-printed file
/// is not text, while `<text:s/>` and `<text:tab/>` are exactly what they say.
#[test]
fn odt_white_space_is_read_the_way_libreoffice_reads_it() {
    let body = r#"
      <text:p>
        A line
        wrapped   in the file,<text:s text:c="2"/>then two spaces.
      </text:p>"#;
    let plan = plan_of("chapter.fodt", fodt("", body).as_bytes());
    let (_, text) = only_row(&plan);
    assert_eq!(text, "A line wrapped in the file,  then two spaces.");
}

// ── pictures ────────────────────────────────────────────────────────────────────────

/// The display size of each picture of `djot`, in the editor's pixels.
fn picture_sizes(djot: &str) -> Vec<(u32, u32)> {
    open(djot)
        .blocks()
        .iter()
        .flat_map(|b| b.fragments())
        .filter_map(|f| match f {
            FragmentContent::Image { width, height, .. } => Some((width, height)),
            _ => None,
        })
        .collect()
}

/// A picture is not copied in yet, but the reference it leaves keeps the size it was
/// shown at: 952,500 by 476,250 English Metric Units, or 1.0417 by 0.5208 inches, is 100 by
/// 50 pixels at 96 to the inch.
#[test]
fn a_picture_reference_keeps_its_display_size() {
    let body = r#"
<w:p><w:r><w:t xml:space="preserve">A map: </w:t></w:r><w:r><w:drawing><wp:inline><wp:extent cx="952500" cy="476250"/><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic><pic:blipFill><a:blip r:embed="rId9"/></pic:blipFill><pic:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="952500" cy="476250"/></a:xfrm></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r></w:p>"#;
    let plan = plan_of("chapter.docx", &docx(body, "", ""));
    let (row, text) = only_row(&plan);
    assert_eq!(text, "A map: \u{FFFC}");
    assert_eq!(picture_sizes(&row.djot), vec![(100, 50)], "{}", row.djot);

    let body = r#"
      <text:p>A map: <draw:frame svg:width="1.0417in" svg:height="0.5208in"><draw:image xlink:href="Pictures/map.png"/></draw:frame></text:p>"#;
    let plan = plan_of("chapter.fodt", fodt("", body).as_bytes());
    let (row, text) = only_row(&plan);
    assert_eq!(text, "A map: \u{FFFC}");
    assert_eq!(picture_sizes(&row.djot), vec![(100, 50)], "{}", row.djot);
}

// ── a comment over two paragraphs ───────────────────────────────────────────────────

/// Word and LibreOffice both let a comment run from one paragraph into the next. It
/// comes over whole, the paragraph break included.
#[test]
fn a_comment_across_two_paragraphs_keeps_its_whole_extent() {
    let body = r#"
<w:p><w:r><w:t xml:space="preserve">She turned </w:t></w:r><w:commentRangeStart w:id="0"/><w:r><w:t>the corner.</w:t></w:r></w:p>
<w:p><w:r><w:t xml:space="preserve">The fog</w:t></w:r><w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r><w:r><w:t xml:space="preserve"> stayed.</w:t></w:r></w:p>"#;
    let plan = plan_of("chapter.docx", &docx(body, "", &docx_comment(0, "Both?")));
    let (row, text) = only_row(&plan);
    let c = &row.comments[0];
    assert_eq!(
        (c.kind.clone(), c.orphaned),
        (CommentAnchorKind::Range, false)
    );
    assert_eq!(
        anchored(&text, c.anchor.start, c.anchor.length),
        "the corner.\nThe fog"
    );

    let body = r#"
      <text:p>She turned <office:annotation office:name="a0"><dc:creator>Editor</dc:creator><text:p>Both?</text:p></office:annotation>the corner.</text:p>
      <text:p>The fog<office:annotation-end office:name="a0"/> stayed.</text:p>"#;
    let plan = plan_of("chapter.fodt", fodt("", body).as_bytes());
    let (row, text) = only_row(&plan);
    let c = &row.comments[0];
    assert_eq!(
        (c.kind.clone(), c.orphaned),
        (CommentAnchorKind::Range, false)
    );
    assert_eq!(
        anchored(&text, c.anchor.start, c.anchor.length),
        "the corner.\nThe fog"
    );
}

// ── tables ──────────────────────────────────────────────────────────────────────────

/// A table is proved cell by cell like any paragraph, so a comment inside a cell keeps
/// its words. The cell's text is its own: a comment's author, date and body sitting in
/// the cell are not part of it.
#[test]
fn a_comment_in_a_table_cell_keeps_its_words() {
    let body = r#"
<w:p><w:r><w:t>Intro.</w:t></w:r></w:p>
<w:tbl><w:tr><w:tc><w:p><w:r><w:t>a</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t xml:space="preserve">salt </w:t></w:r><w:commentRangeStart w:id="0"/><w:r><w:rPr><w:b/></w:rPr><w:t>bleached</w:t></w:r><w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r></w:p></w:tc></w:tr></w:tbl>"#;
    let plan = plan_of(
        "chapter.docx",
        &docx(body, "", &docx_comment(0, "Which salt?")),
    );
    let (row, text) = only_row(&plan);
    assert_eq!(text, "Intro.\n\u{FFFC}\na\nsalt bleached");
    assert!(
        row.djot.contains("{*bleached*}"),
        "the cell keeps its bold: {}",
        row.djot
    );
    let c = &row.comments[0];
    assert_eq!(
        (c.kind.clone(), c.orphaned),
        (CommentAnchorKind::Range, false)
    );
    assert_eq!(anchored(&text, c.anchor.start, c.anchor.length), "bleached");

    let body = r#"
      <text:p>Intro.</text:p>
      <table:table><table:table-row><table:table-cell><text:p>a</text:p></table:table-cell><table:table-cell><text:p>salt <office:annotation office:name="a0"><dc:creator>Editor</dc:creator><dc:date>2026-01-02T03:04:05</dc:date><text:p>Which salt?</text:p></office:annotation>bleached<office:annotation-end office:name="a0"/></text:p></table:table-cell></table:table-row></table:table>"#;
    let plan = plan_of("chapter.fodt", fodt("", body).as_bytes());
    let (row, text) = only_row(&plan);
    assert_eq!(text, "Intro.\n\u{FFFC}\na\nsalt bleached");
    let c = &row.comments[0];
    assert_eq!(
        (c.kind.clone(), c.orphaned),
        (CommentAnchorKind::Range, false)
    );
    assert_eq!(anchored(&text, c.anchor.start, c.anchor.length), "bleached");
    assert_eq!(c.body, "Which salt?");
}

// ── comments with no text of their own to go to ─────────────────────────────────────

/// Every diagnostic a plan raises about a comment, on the plan or on one of its rows.
fn comment_reports(plan: &ImportPlan) -> Vec<&document_ingest::ImportDiagnostic> {
    plan.diagnostics
        .iter()
        .chain(plan.rows.iter().flat_map(|r| r.diagnostics.iter()))
        .filter(|d| {
            matches!(
                d,
                document_ingest::ImportDiagnostic::CommentUnanchored { .. }
                    | document_ingest::ImportDiagnostic::CommentNotCarried { .. }
            )
        })
        .collect()
}

/// A file whose only paragraph is a commented space brings no text, so there is nowhere
/// to keep the comment. It is reported once, as not imported: never twice, and never with
/// the sentence that says it was kept.
#[test]
fn a_comment_in_a_file_with_no_text_is_reported_once_as_not_imported() {
    let body = r#"
<w:p><w:commentRangeStart w:id="0"/><w:r><w:t xml:space="preserve"> </w:t></w:r><w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r></w:p>"#;
    let docx_plan = plan_of("empty.docx", &docx(body, "", &docx_comment(0, "Lonely")));

    let body = r#"
      <text:p><office:annotation office:name="a0"><dc:creator>Editor</dc:creator><text:p>Lonely</text:p></office:annotation><text:s/><office:annotation-end office:name="a0"/></text:p>"#;
    let odt_plan = plan_of("empty.fodt", fodt("", body).as_bytes());

    for plan in [docx_plan, odt_plan] {
        assert!(plan.rows.is_empty(), "{:?}", plan.rows);
        assert!(
            matches!(
                comment_reports(&plan).as_slice(),
                [document_ingest::ImportDiagnostic::CommentNotCarried { quote, .. }]
                    if quote == "Lonely"
            ),
            "{:?}",
            comment_reports(&plan)
        );
    }
}

/// A comment on a blank line under a title that heads nothing but another heading: the
/// scanner moves it to the nearest paragraph, which is that title, and the title's row
/// stores no prose. It arrives on the first stored paragraph instead, and is reported once.
#[test]
fn a_comment_under_a_title_only_heading_arrives_on_stored_prose() {
    let styles = r#"<w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:pPr><w:outlineLvl w:val="0"/></w:pPr></w:style><w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/><w:pPr><w:outlineLvl w:val="1"/></w:pPr></w:style>"#;
    let body = r#"
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>The Book</w:t></w:r></w:p>
<w:p><w:commentRangeStart w:id="0"/><w:r><w:t xml:space="preserve"> </w:t></w:r><w:commentRangeEnd w:id="0"/><w:r><w:commentReference w:id="0"/></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>Chapter One</w:t></w:r></w:p>
<w:p><w:r><w:t>The ferry was late.</w:t></w:r></w:p>"#;
    let plan = plan_of(
        "titled.docx",
        &docx(body, styles, &docx_comment(0, "A stronger title?")),
    );
    let titles: Vec<&str> = plan.rows.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(titles, vec!["The Book", "Chapter One"]);
    assert!(
        plan.rows[0].comments.is_empty(),
        "{:?}",
        plan.rows[0].comments
    );
    let [c] = plan.rows[1].comments.as_slice() else {
        panic!("one comment on the chapter: {:?}", plan.rows[1].comments);
    };
    assert_eq!(
        (c.kind.clone(), c.orphaned, c.anchor.exact.as_str()),
        (CommentAnchorKind::Paragraph, false, "The ferry was late.")
    );
    assert!(
        matches!(
            comment_reports(&plan).as_slice(),
            [document_ingest::ImportDiagnostic::CommentUnanchored { .. }]
        ),
        "{:?}",
        comment_reports(&plan)
    );
}

// ── a line left to fill in ──────────────────────────────────────────────────────────

/// An underlined stretch of blank space, written the way Word and LibreOffice write a
/// line left to fill in, arrives as plain spaces and is reported.
#[test]
fn an_underlined_blank_is_reported_from_both_formats() {
    let body = r#"
<w:p><w:r><w:t>Name:</w:t></w:r><w:r><w:rPr><w:u w:val="single"/></w:rPr><w:t xml:space="preserve">      </w:t></w:r><w:r><w:t xml:space="preserve"> Date.</w:t></w:r></w:p>"#;
    let docx_plan = plan_of("form.docx", &docx(body, "", ""));

    let automatic = r#"<style:style style:name="T1" style:family="text"><style:text-properties style:text-underline-style="solid" style:text-underline-width="auto" style:text-underline-color="font-color"/></style:style>"#;
    let body = r#"
      <text:p>Name:<text:span text:style-name="T1"><text:s text:c="6"/></text:span> Date.</text:p>"#;
    let odt_plan = plan_of("form.fodt", fodt(automatic, body).as_bytes());

    for plan in [docx_plan, odt_plan] {
        let (row, text) = only_row(&plan);
        assert_eq!(text, "Name:       Date.");
        assert!(!row.djot.contains("{+"), "{}", row.djot);
        assert!(
            plan.diagnostics.iter().any(|d| matches!(
                d,
                document_ingest::ImportDiagnostic::StyledSpacesNotCarried { count: 1, .. }
            )),
            "{:?}",
            plan.diagnostics
        );
    }
}

/// A line left to fill in at the end of a paragraph, after a label, or on a line of its
/// own, does not arrive at all, since blank space at a paragraph's edge is not kept, styled
/// or not. It is reported with the stretches that arrive as plain spaces.
#[test]
fn a_line_left_to_fill_in_at_a_paragraphs_end_is_reported_from_both_formats() {
    let body = r#"
<w:p><w:r><w:t>Signature:</w:t></w:r><w:r><w:rPr><w:u w:val="single"/></w:rPr><w:t xml:space="preserve">                    </w:t></w:r></w:p>
<w:p><w:r><w:rPr><w:u w:val="single"/></w:rPr><w:tab/><w:tab/></w:r></w:p>
<w:p><w:r><w:t>Date.</w:t></w:r></w:p>"#;
    let docx_plan = plan_of("fill.docx", &docx(body, "", ""));

    let automatic = r#"<style:style style:name="T1" style:family="text"><style:text-properties style:text-underline-style="solid" style:text-underline-width="auto" style:text-underline-color="font-color"/></style:style>"#;
    let body = r#"
      <text:p>Signature:<text:span text:style-name="T1"><text:s text:c="20"/></text:span></text:p>
      <text:p><text:span text:style-name="T1"><text:tab/><text:tab/></text:span></text:p>
      <text:p>Date.</text:p>"#;
    let odt_plan = plan_of("fill.fodt", fodt(automatic, body).as_bytes());

    for plan in [docx_plan, odt_plan] {
        let (_, text) = only_row(&plan);
        assert_eq!(text, "Signature:\nDate.");
        assert!(
            plan.diagnostics.iter().any(|d| matches!(
                d,
                document_ingest::ImportDiagnostic::StyledSpacesNotCarried { count: 2, .. }
            )),
            "{:?}",
            plan.diagnostics
        );
    }
}

// ── quotations ──────────────────────────────────────────────────────────────────────

/// A paragraph in LibreOffice's `Quotations` style, directly or through a style built on
/// it, arrives as a quotation, as `odt.rs`' module documentation says.
#[test]
fn an_odt_quotations_paragraph_is_a_quotation() {
    let automatic = r#"<style:style style:name="P1" style:family="paragraph" style:parent-style-name="Quotations"/>"#;
    let body = r#"
      <text:p>Before.</text:p>
      <text:p text:style-name="Quotations">A quoted line.</text:p>
      <text:p text:style-name="P1">Another quoted line.</text:p>
      <text:p>After.</text:p>"#;
    let plan = plan_of("quoted.fodt", fodt(automatic, body).as_bytes());
    let (row, text) = only_row(&plan);
    assert_eq!(
        text,
        "Before.\nA quoted line.\nAnother quoted line.\nAfter."
    );
    assert!(
        row.djot.contains("> A quoted line.") && row.djot.contains("> Another quoted line."),
        "{}",
        row.djot
    );
}
