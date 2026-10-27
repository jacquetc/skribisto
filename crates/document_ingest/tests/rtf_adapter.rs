// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! RTF read by `rtf_reader` and mapped by `sources::rtf::to_rich`: what reaches the rich
//! model, what is reported, and that the Djot the shared emitter writes from it reads back
//! as the text the RTF held, through the parser and through an editor's save.
//!
//! Every fixture is built here from synthetic English text. `@u` in a fixture stands for
//! the backslash control word starting with `u`: a backslash, a `u` and four hexadecimal
//! digits in a source file are too easily taken for a Unicode escape by the tools that
//! handle it.

#![cfg(feature = "rtf")]

use document_ingest::pictures::{PictureRefusal, PlacedPicture};
use document_ingest::sources::rich::{
    self, Alignment, BlockProps, Direction, ParagraphKind, RichBlock, RichDocument, Run,
};
use document_ingest::sources::rtf::{
    NotCarried, RtfConversion, RtfOptions, SourcedPicture, to_rich,
};
use document_ingest::{ImportDiagnostic, SourceBlock, SourceDocument};
use proptest::prelude::*;
use std::sync::mpsc;
use std::time::{Duration, Instant};

const ORIGIN: &str = "manuscript.rtf";

fn rtf(template: &str) -> Vec<u8> {
    let mut out = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '@' && chars.peek() == Some(&'u') {
            out.push('\\');
        } else {
            out.push(c);
        }
    }
    out.into_bytes()
}

fn convert(template: &str) -> RtfConversion {
    let doc = rtf_reader::parse(&rtf(template)).expect("the fixture is RTF");
    to_rich(&doc, &RtfOptions::new(ORIGIN))
}

fn paragraphs(document: &RichDocument) -> Vec<(ParagraphKind, String, BlockProps)> {
    document
        .blocks
        .iter()
        .filter_map(|block| match block {
            RichBlock::Paragraph { kind, props, .. } => Some((*kind, block.plain_text(), *props)),
            RichBlock::Table { .. } => None,
        })
        .collect()
}

fn texts(document: &RichDocument) -> Vec<String> {
    paragraphs(document)
        .into_iter()
        .map(|(_, t, _)| t)
        .collect()
}

fn runs(document: &RichDocument, index: usize) -> &[Run] {
    match &document.blocks[index] {
        RichBlock::Paragraph { runs, .. } => runs,
        other => panic!("block {index} is {other:?}"),
    }
}

// ── paragraphs ───────────────────────────────────────────────────────────────

#[test]
fn alignment_direction_and_page_breaks_reach_the_paragraph() {
    let conversion = convert(
        r"{\rtf1\pard\qc * * *\par\pard\qr Right.\par\pard\qnatural Natural.\par\pard\rtlpar\qc Centred right to left.\par\pard Before the break.\page After the break.\par\pard\pagebb New page.}",
    );
    let got = paragraphs(&conversion.document);
    let props: Vec<BlockProps> = got.iter().map(|(_, _, p)| *p).collect();
    assert_eq!(props[0].alignment, Some(Alignment::Center));
    assert_eq!(props[1].alignment, Some(Alignment::Right));
    assert_eq!(props[2].alignment, None, "natural is the start edge");
    assert_eq!(props[3].direction, Some(Direction::RightToLeft));
    assert_eq!(
        texts(&conversion.document),
        [
            "* * *",
            "Right.",
            "Natural.",
            "Centred right to left.",
            "Before the break.",
            "After the break.",
            "New page."
        ]
    );
    assert!(!props[4].page_break_before);
    assert!(props[5].page_break_before);
    assert!(props[6].page_break_before);
}

#[test]
fn a_page_break_at_the_end_of_a_paragraph_starts_the_next_page() {
    let conversion = convert(r"{\rtf1 End of part one.\page\par Part two.}");
    let got = paragraphs(&conversion.document);
    let starts_page: Vec<(String, bool)> = got
        .iter()
        .filter(|(_, text, _)| !text.is_empty())
        .map(|(_, text, props)| (text.clone(), props.page_break_before))
        .collect();
    assert_eq!(
        starts_page,
        [
            ("End of part one.".to_string(), false),
            ("Part two.".to_string(), true)
        ]
    );
}

/// A line break (`\line`, or a line separator in the text) stays in its paragraph, as a
/// line separator. A column break and a paragraph separator end the paragraph, and the
/// rest keeps its kind.
#[test]
fn a_line_break_stays_in_its_paragraph() {
    let conversion = convert(
        r"{\rtf1\pard\qc First line\line second line\par Soft @uc0@u8232 return.\par Column\column next.\par Hard@uc0@u8233 stop.\par}",
    );
    let got = paragraphs(&conversion.document);
    assert_eq!(
        got.iter().map(|(_, t, _)| t.as_str()).collect::<Vec<_>>(),
        [
            "First line\u{2028}second line",
            "Soft \u{2028}return.",
            "Column",
            "next.",
            "Hard",
            "stop."
        ]
    );
    assert_eq!(got[0].2.alignment, Some(Alignment::Center));
}

#[test]
fn headings_come_from_outline_levels_and_heading_styles() {
    let conversion = convert(
        r"{\rtf1{\stylesheet{\s0 Normal;}{\s1\outlinelevel0 Title Style;}{\s2 heading 2;}{\s3\sbasedon1 Derived;}{\s4 Quote;}{\s5 Epigraph;}}\pard\outlinelevel1 By level\par\pard\s1 By style level\par\pard\s2 By style name\par\pard\s3 Based on\par\pard\s4 A quotation.\par\pard\s5 An epigraph.\par\pard\s0 Body.}",
    );
    let kinds: Vec<ParagraphKind> = paragraphs(&conversion.document)
        .into_iter()
        .map(|(k, _, _)| k)
        .collect();
    assert_eq!(
        kinds,
        [
            ParagraphKind::Heading { level: 2 },
            ParagraphKind::Heading { level: 1 },
            ParagraphKind::Heading { level: 2 },
            ParagraphKind::Heading { level: 1 },
            ParagraphKind::Quote,
            ParagraphKind::Epigraph,
            ParagraphKind::Body,
        ]
    );
}

#[test]
fn list_items_are_ordered_or_not_and_nested() {
    let conversion = convert(
        r"{\rtf1{\*\listtable{\list{\listlevel\levelnfc0}{\listlevel\levelnfc23}{\listlevel\levelnfc255}\listid1}{\list{\listlevel{\*\levelmarker \{disc\}}}\listid2}}{\*\listoverridetable{\listoverride\listid1\ls1}{\listoverride\listid2\ls2}}\pard\ls1\ilvl0{\listtext 1.\tab}One\par\pard\ls1\ilvl1{\listtext \'95\tab}Point\par\pard\ls1\ilvl2 Indented only\par\pard\ls2\ilvl0 Disc\par\pard\ls9{\listtext iv.\tab}Roman label\par\pard{\*\pn\pnlvlblt}{\pntext \'b7\tab}Old bullet\par\pard Body}",
    );
    let got = paragraphs(&conversion.document);
    let kinds: Vec<ParagraphKind> = got.iter().map(|(k, _, _)| *k).collect();
    assert_eq!(
        kinds,
        [
            ParagraphKind::ListItem {
                ordered: true,
                depth: 0
            },
            ParagraphKind::ListItem {
                ordered: false,
                depth: 1
            },
            ParagraphKind::Body,
            ParagraphKind::ListItem {
                ordered: false,
                depth: 0
            },
            ParagraphKind::ListItem {
                ordered: true,
                depth: 0
            },
            ParagraphKind::ListItem {
                ordered: false,
                depth: 0
            },
            ParagraphKind::Body,
        ]
    );
    // The label the file stored is not text: a Djot list numbers itself.
    assert_eq!(got[0].1, "One");
}

// ── runs ─────────────────────────────────────────────────────────────────────

#[test]
fn character_styles_are_carried_and_a_family_is_never_code() {
    let conversion = convert(
        r"{\rtf1{\fonttbl{\f0\fmodern Courier New;}}\f0 plain {\b bold} {\i italic} {\ul under} {\strike struck} x{\super 2} H{\sub 2}O}",
    );
    let styled: Vec<(String, rich::RunStyle)> = runs(&conversion.document, 0)
        .iter()
        .map(|r| (r.text.clone(), r.style))
        .collect();
    let style_of = |text: &str| {
        styled
            .iter()
            .find(|(t, _)| t == text)
            .map(|(_, s)| *s)
            .unwrap_or_else(|| panic!("no run {text:?} in {styled:?}"))
    };
    assert!(style_of("bold").bold);
    assert!(style_of("italic").italic);
    assert!(style_of("under").underline);
    assert!(style_of("struck").strikethrough);
    assert!(style_of("2").superscript || style_of("2").subscript);
    assert!(styled.iter().all(|(_, s)| !s.code));
    assert!(conversion.not_carried.is_empty());
}

#[test]
fn web_links_are_carried_and_other_links_keep_only_their_text() {
    let conversion = convert(
        r#"{\rtf1 Read {\field{\*\fldinst HYPERLINK "https://example.org/a(b)"}{\fldrslt the essay}}, write {\field{\*\fldinst {HYPERLINK "mailto:writer@example.org"}}{\fldrslt to me}}, see {\field{\*\fldinst HYPERLINK "scrivlnk://D3796A92"}{\fldrslt chapter two}} and {\field{\*\fldinst HYPERLINK \\l "notes"}{\fldrslt the notes}}.}"#,
    );
    let linked: Vec<(String, Option<String>)> = runs(&conversion.document, 0)
        .iter()
        .map(|r| (r.text.clone(), r.link.clone()))
        .collect();
    let link_of = |text: &str| {
        linked
            .iter()
            .find(|(t, _)| t == text)
            .and_then(|(_, l)| l.clone())
    };
    assert_eq!(
        link_of("the essay").as_deref(),
        Some("https://example.org/a(b)")
    );
    assert_eq!(
        link_of("to me").as_deref(),
        Some("mailto:writer@example.org")
    );
    assert_eq!(link_of("chapter two"), None);
    assert_eq!(link_of("the notes"), None);
    assert_eq!(conversion.not_carried.links_kept_as_text, 2);
    assert_eq!(
        texts(&conversion.document),
        ["Read the essay, write to me, see chapter two and the notes."]
    );
}

#[test]
fn a_field_other_than_a_link_is_reported_with_its_last_result_kept() {
    let conversion = convert(
        r"{\rtf1 Page {\field{\*\fldinst PAGE}{\fldrslt 12}} of {\field{\*\fldinst NUMPAGES}}.}",
    );
    assert_eq!(texts(&conversion.document), ["Page 12 of ."]);
    assert!(
        conversion
            .diagnostics
            .contains(&ImportDiagnostic::FieldFlattened {
                path: ORIGIN.into(),
                count: 2
            })
    );
}

// ── notes and comments ───────────────────────────────────────────────────────

#[test]
fn footnotes_and_endnotes_become_footnotes() {
    let conversion = convert(
        r"{\rtf1 The ferry was late.{\super\chftn}{\footnote\pard\plain{\super\chftn} It always is.\par In November.} Again.{\footnote\ftnalt At the end.}{\footnote \par}\par}",
    );
    let document = &conversion.document;
    assert_eq!(
        texts(document),
        ["The ferry was late.\u{FFFC} Again.\u{FFFC}"]
    );
    let labels: Vec<&str> = runs(document, 0)
        .iter()
        .filter_map(|r| r.footnote.as_deref())
        .collect();
    assert_eq!(labels, ["srcfn-1", "srcfn-2"]);
    assert_eq!(document.footnotes.len(), 2);
    let body: Vec<String> = document.footnotes[0]
        .paragraphs
        .iter()
        .map(|p| p.iter().map(|r| r.text.as_str()).collect())
        .collect();
    assert_eq!(body, [" It always is.", "In November."]);
    assert_eq!(conversion.not_carried.endnotes, 1);
    assert!(
        conversion
            .diagnostics
            .contains(&ImportDiagnostic::FootnoteNotCarried {
                path: ORIGIN.into(),
                count: 1
            })
    );
}

/// A line break inside a note or a comment starts a new paragraph of it, as it does in a
/// Word or an OpenDocument one: the same two-line note comes in as the same two paragraphs
/// whichever program wrote it. Manuscript prose keeps its line breaks.
#[test]
fn a_line_break_in_a_note_or_a_comment_starts_a_paragraph_of_it() {
    let conversion = convert(
        r"{\rtf1 Verse\line on two lines.{\footnote First line.\line {\b Bold @uc0@u8232 next} line.\line\line}{\*\annotation{\*\atnref 1}Which\line one?}\par}",
    );
    let document = &conversion.document;
    assert_eq!(texts(document), ["Verse\u{2028}on two lines.\u{FFFC}"]);
    let lines = |paragraphs: &[Vec<Run>]| -> Vec<String> {
        paragraphs
            .iter()
            .map(|p| p.iter().map(|r| r.text.as_str()).collect())
            .collect()
    };
    assert_eq!(document.footnotes.len(), 1);
    let body = &document.footnotes[0].paragraphs;
    assert_eq!(lines(body), ["First line.", "Bold ", "next line."]);
    assert!(
        body[1][0].style.bold,
        "the run cut by the break keeps its style"
    );
    assert!(body[2][0].style.bold);
    assert!(!body[2][1].style.bold);
    assert_eq!(document.annotations.len(), 1);
    assert_eq!(
        lines(&document.annotations[0].paragraphs),
        ["Which", "one?"]
    );
}

#[test]
fn comments_keep_their_range_their_author_and_their_replies() {
    let conversion = convert(
        r"{\rtf1 An example of {\*\atrfstart 1}a paradigm{\*\atrfend 1} here.{\*\atnid JD}{\*\atnauthor John Doe}\chatn{\*\annotation{\*\atnref 1}{\*\atndate 1180187342}Which one?}{\*\atnid AW}\chatn{\*\annotation{\*\atnref 2}{\*\atnparent 1}The first.}\par Second{\*\atrfstart 3} paragraph\par spans{\*\atrfend 3}.{\*\atnauthor Ed}\chatn{\*\annotation{\*\atnref 3}Long range.}\par Point{\*\atnauthor Ed}\chatn{\*\annotation On this paragraph.}.}",
    );
    let document = &conversion.document;
    assert_eq!(
        texts(document),
        [
            "An example of a paradigm here.",
            "Second paragraph",
            "spans.",
            "Point."
        ]
    );
    assert_eq!(document.annotations.len(), 3);
    let first = &document.annotations[0];
    assert_eq!((first.block_index, first.start, first.length), (0, 14, 10));
    assert_eq!(first.author, "John Doe");
    assert_eq!(first.author_initials, "JD");
    assert_eq!(first.replies.len(), 1);
    assert_eq!(first.replies[0].author, "AW");
    let spanning = &document.annotations[1];
    assert_eq!((spanning.block_index, spanning.start), (1, 6));
    assert_eq!(spanning.length, "Second paragraph".len() - 6);
    let end = spanning.end.expect("ends in a later block");
    assert_eq!((end.block_index, end.offset), (2, 5));
    let point = &document.annotations[2];
    assert_eq!((point.block_index, point.start, point.length), (3, 5, 0));
    assert_eq!(conversion.not_carried.comment_dates, 1);
}

// ── tables ───────────────────────────────────────────────────────────────────

#[test]
fn a_line_break_in_a_table_cell_is_a_space() {
    let conversion = convert(
        r"{\rtf1 Before.\par\trowd\cellx3000\pard\intbl roses\line violets\cell\row\trowd\pard\intbl red@uc0@u8232 blue\cell\row\pard After.}",
    );
    let document = &conversion.document;
    let RichBlock::Table { rows } = &document.blocks[1] else {
        panic!("expected a table, got {:?}", document.blocks[1]);
    };
    let cells: Vec<String> = rows
        .iter()
        .map(|row| row[0].iter().map(|r| r.text.as_str()).collect())
        .collect();
    assert_eq!(cells, ["roses violets", "red blue"]);
}

#[test]
fn tables_join_a_cells_paragraphs_and_flatten_a_nested_table() {
    let conversion = convert(
        r"{\rtf1 Before.\par\trowd\cellx3000\cellx6000\pard\intbl Name\cell\pard\intbl First line\par\pard\intbl second line\cell\row\trowd\pard\intbl Inner:\par\pard\intbl\itap2 deep\nestcell{\*\nesttableprops\nestrow}\pard\intbl\cell\pard\intbl last\cell\row\pard After.}",
    );
    let document = &conversion.document;
    let RichBlock::Table { rows } = &document.blocks[1] else {
        panic!("expected a table, got {:?}", document.blocks[1]);
    };
    let cells: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.iter().map(|r| r.text.as_str()).collect())
                .collect()
        })
        .collect();
    assert_eq!(
        cells,
        [
            vec!["Name".to_string(), "First line second line".to_string()],
            vec!["Inner: deep".to_string(), "last".to_string()]
        ]
    );
    assert_eq!(conversion.not_carried.joined_cells, 2);
    assert_eq!(conversion.not_carried.nested_tables, 1);
    assert_eq!(texts(document), ["Before.", "After."]);
}

// ── pictures ─────────────────────────────────────────────────────────────────

/// A 1×1 PNG, written as RTF writes a picture's data: in hexadecimal.
const PNG_HEX: &str = "89504e470d0a1a0a0000000d4948445200000001000000010802000000907753de0000000c49444154789c63f8ffff3f0005fe02fe0def46b80000000049454e44ae426082";

#[test]
fn a_picture_is_brought_in_and_one_that_reads_as_none_is_left_out_with_why() {
    let template = [
        r"{\rtf1 See {\*\shppict{\pict{\*\nisusfilename harbour}\picw1\pich1\picwgoal3000\pichgoal1500\pngblip ",
        PNG_HEX,
        r"}}{\nonshppict{\pict\wmetafile8 0102}} and {\pict\jpegblip\picw10\pich20 ffd8}.}",
    ]
    .concat();
    let conversion = convert(&template);
    let images: Vec<rich::RunImage> = runs(&conversion.document, 0)
        .iter()
        .filter_map(|r| r.image.clone())
        .collect();
    assert_eq!(images.len(), 1, "the two bytes named JPEG are no picture");
    assert!(images[0].src.starts_with("assets/") && images[0].src.ends_with(".png"));
    assert_eq!(images[0].name, "harbour");
    assert_eq!(
        (images[0].width, images[0].height),
        (200, 100),
        "shown at its goal size: fifteen twips to a pixel"
    );
    assert_eq!(conversion.pictures.len(), 1, "handed back for the project");
    assert_eq!(conversion.pictures[0].relative_path(), images[0].src);
    let reported: Vec<&ImportDiagnostic> = conversion
        .diagnostics
        .iter()
        .filter(|d| matches!(d, ImportDiagnostic::ImageNotIngested { .. }))
        .collect();
    assert_eq!(reported.len(), 1, "{reported:?}");

    // A caller that keeps pictures itself names them.
    let doc = rtf_reader::parse(&rtf(&template)).expect("RTF");
    let stored = |id: usize, _: &rtf_reader::Picture| {
        Ok(SourcedPicture {
            placed: PlacedPicture {
                src: format!("assets/picture-{id}.png"),
                width: 7,
                height: 9,
            },
            first_frame_only: false,
        })
    };
    let options = RtfOptions {
        picture_source: Some(&stored),
        ..RtfOptions::new(ORIGIN)
    };
    let stored = to_rich(&doc, &options);
    let images: Vec<(String, u32, u32)> = runs(&stored.document, 0)
        .iter()
        .filter_map(|r| r.image.as_ref().map(|i| (i.src.clone(), i.width, i.height)))
        .collect();
    assert_eq!(
        images,
        [
            ("assets/picture-0.png".to_string(), 200, 100),
            ("assets/picture-1.png".to_string(), 10, 20),
        ]
    );
    assert!(stored.diagnostics.is_empty(), "{:?}", stored.diagnostics);
    assert!(stored.pictures.is_empty());
}

#[test]
fn a_picture_past_the_readers_limit_is_left_out_and_the_text_kept() {
    let mut template = r"{\rtf1 Before {\pict\pngblip ".to_string();
    template.push_str(&"00".repeat(300));
    template.push_str(r"} after.}");
    let options = rtf_reader::Options {
        limits: rtf_reader::Limits {
            max_picture_bytes: 256,
            ..rtf_reader::Limits::default()
        },
        ..rtf_reader::Options::default()
    };
    let doc = rtf_reader::parse_with(template.as_bytes(), &options).expect("RTF");
    let conversion = to_rich(&doc, &RtfOptions::new(ORIGIN));
    assert_eq!(texts(&conversion.document), ["Before  after."]);
    assert_eq!(
        conversion.diagnostics,
        [ImportDiagnostic::ImageNotIngested {
            path: ORIGIN.into(),
            target: "1".into(),
            reason: PictureRefusal::TooLarge,
        }]
    );
}

/// A note's picture stays out, as it does from a Word document: a note keeps its words.
#[test]
fn a_picture_in_a_footnote_is_left_out_and_its_words_kept() {
    let template = [
        r"{\rtf1 Text{\footnote\pard Note {\pict\pngblip ",
        PNG_HEX,
        r"} words.}.}",
    ]
    .concat();
    let conversion = convert(&template);
    assert_eq!(conversion.document.footnotes.len(), 1);
    assert!(conversion.pictures.is_empty());
    assert!(
        conversion.diagnostics.iter().any(|d| matches!(
            d,
            ImportDiagnostic::ImageNotIngested {
                reason: PictureRefusal::InNote,
                ..
            }
        )),
        "{:?}",
        conversion.diagnostics
    );
}

#[test]
fn headings_stay_in_the_prose_and_code_follows_the_callers_fonts_when_asked() {
    let template = r"{\rtf1{\fonttbl{\f0 Serif;}{\f7 Mono;}}{\stylesheet{\s1 heading 2;}}\pard\s1 A section\par\pard Plain and {\f7 code} here.\par}";
    let doc = rtf_reader::parse(&rtf(template)).expect("RTF");
    let options = RtfOptions {
        headings_in_prose: true,
        code_fonts: &[7],
        ..RtfOptions::new(ORIGIN)
    };
    let conversion = to_rich(&doc, &options);
    let kinds: Vec<ParagraphKind> = paragraphs(&conversion.document)
        .into_iter()
        .map(|(kind, _, _)| kind)
        .collect();
    assert_eq!(
        kinds,
        [
            ParagraphKind::InlineHeading { level: 2 },
            ParagraphKind::Body
        ]
    );
    let code: Vec<&str> = runs(&conversion.document, 1)
        .iter()
        .filter(|r| r.style.code)
        .map(|r| r.text.as_str())
        .collect();
    assert_eq!(code, ["code"]);
}

#[test]
fn footnote_labels_come_from_the_caller_when_it_mints_them() {
    let counter = std::cell::Cell::new(0u32);
    let mint = || {
        counter.set(counter.get() + 1);
        format!("fn{}", counter.get() + 10)
    };
    let doc = rtf_reader::parse(&rtf(
        r"{\rtf1 One{\footnote First.} two{\footnote Second.}.\par}",
    ))
    .expect("RTF");
    let options = RtfOptions {
        footnote_labels: Some(&mint),
        ..RtfOptions::new(ORIGIN)
    };
    let conversion = to_rich(&doc, &options);
    let labels: Vec<&str> = conversion
        .document
        .footnotes
        .iter()
        .map(|f| f.label.as_str())
        .collect();
    assert_eq!(labels, ["fn11", "fn12"]);
}

// ── what the model has no place for ──────────────────────────────────────────

#[test]
fn what_is_not_carried_is_counted() {
    let conversion = convert(
        r"{\rtf1{\colortbl;\red0\green0\blue0;\red255\green255\blue255;\red255\green0\blue0;}{\header Running head}{\shp{\*\shpinst{\shptxt boxed}}}Seen{\v  hidden} {\cf1 black} {\cf3 red} {\cb2 white} {\highlight3 marked} {\scaps small} {\caps loud} a\uc1@u65532?b \chpgn}",
    );
    let not_carried = conversion.not_carried;
    assert_eq!(
        not_carried,
        NotCarried {
            hidden_characters: 7,
            highlighted_runs: 1,
            coloured_runs: 1,
            small_caps_runs: 1,
            all_caps_runs: 1,
            object_replacement_characters: 1,
            headers_and_footers: 1,
            computed_characters: 1,
            ..NotCarried::default()
        }
    );
    assert_eq!(
        texts(&conversion.document),
        ["Seen black red white marked small loud ab "]
    );
    assert!(
        conversion
            .diagnostics
            .contains(&ImportDiagnostic::TextBoxDropped {
                path: ORIGIN.into(),
                count: 1
            })
    );
    assert!(
        conversion
            .diagnostics
            .contains(&ImportDiagnostic::EmbeddedObjectDropped {
                path: ORIGIN.into(),
                count: 1
            })
    );
}

// ── through the emitter and back ─────────────────────────────────────────────

/// The Djot `assemble` stores for `rtf`, and each prose block's text as the parser read it.
fn assembled(template: &str) -> SourceDocument {
    let conversion = convert(template);
    let mut out = SourceDocument::new("manuscript", ORIGIN);
    rich::assemble(&conversion.document, &mut out).expect("assemble");
    out
}

/// `djot` loaded the way the editor loads a row, then saved and loaded again the way the
/// editor's first save does: the addressable text both times, the text every stored offset
/// is measured in (a table's anchor and a note's reference each one U+FFFC).
fn reload(djot: &str) -> (String, String) {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(djot).expect("the stored Djot loads");
    let loaded = doc.to_addressable_text().expect("addressable text");
    let saved = doc.to_djot().expect("the editor saves");
    let again = text_document::TextDocument::new();
    again.set_djot_sync(&saved).expect("the saved Djot loads");
    (
        loaded,
        again.to_addressable_text().expect("addressable text"),
    )
}

fn prose(document: &SourceDocument) -> Vec<(String, String)> {
    document
        .blocks
        .iter()
        .filter_map(|block| match block {
            SourceBlock::Prose { djot, text } => Some((djot.clone(), text.clone())),
            _ => None,
        })
        .collect()
}

#[test]
fn a_manuscript_reads_back_through_the_emitter_and_an_editors_save() {
    // Both Scrivener writers' shapes, French spacing, the strings Djot would rewrite if
    // they were not escaped, and a table.
    let windows = r#"{\rtf1\ansi\ansicpg1252\uc1\deff0{\fonttbl{\f0\fmodern\fcharset0\fprq2 Cambria;}}
\pard\plain {\f0\fs24\b0\i0 It was 10:30:45 -- or so... "Late", she said.}
\par\plain {\f0\fs24\b1\i0 Bold}{\f0\fs24\b0\i1  and italic}{\f0\fs24  with a\tab tab and  two spaces.}
\par\plain {\f0\fs24 Il dit\~: \loch\af0\hich\af0\dbch\af0\uc1@u171\'ab\~bonjour\~@u187\'bb\~!}
\par\plain {\f0\fs24 I. Not a list, *not* bold, [not] a link, \{braces\} and \\backslash.}
\par\plain {\f0\fs24 See {\field{\*\fldinst HYPERLINK "https://example.org/x"}{\fldrslt the site}}.}}"#;
    let document = assembled(windows);
    assert!(
        !document
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::ProseNotVerbatim { .. })),
        "{:?}",
        document.diagnostics
    );
    let want = [
        "It was 10:30:45 -- or so... \"Late\", she said.",
        "Bold and italic with a\ttab and  two spaces.",
        "Il dit\u{A0}: \u{AB}\u{A0}bonjour\u{A0}\u{BB}\u{A0}!",
        "I. Not a list, *not* bold, [not] a link, {braces} and \\backslash.",
        "See the site.",
    ]
    .join("\n");
    let blocks = prose(&document);
    assert_eq!(blocks.len(), 1);
    let (djot, text) = &blocks[0];
    assert_eq!(text, &want);
    let (loaded, saved) = reload(djot);
    assert_eq!(loaded, want);
    assert_eq!(saved, want, "the editor's first save changes nothing");
}

#[test]
fn a_cocoa_document_with_a_table_and_a_note_reads_back() {
    let cocoa = "{\\rtf1\\ansi\\ansicpg1252\\cocoartf2822\n{\\fonttbl\\f0\\fswiss\\fcharset0 Helvetica;}\n\\pard\\qc \\f0 Chapter One\\\n\\pard Once upon a time\\uc0@u8217 s end.{\\footnote A note.}\\\n\\itap1\\trowd\\cellx4000\\cellx8000\n\\pard\\intbl\\itap1 Left\\cell \\pard\\intbl\\itap1 Right\\cell \\lastrow\\row\n\\pard The end.}";
    let document = assembled(cocoa);
    let blocks = prose(&document);
    let texts: Vec<&str> = blocks.iter().map(|(_, t)| t.as_str()).collect();
    assert_eq!(texts.len(), 3, "{texts:?}");
    assert!(texts[0].starts_with("Chapter One\nOnce upon a time\u{2019}s end."));
    assert_eq!(document.footnotes.len(), 1);
    for (djot, text) in &blocks {
        let (loaded, saved) = reload(djot);
        assert_eq!(&loaded, text);
        assert_eq!(saved, loaded);
    }
}

/// One paragraph's text for the property below: letters, the characters Djot gives a
/// meaning to, the strings its parser used to rewrite, tabs, double spaces, French spacing.
fn words() -> impl Strategy<Value = String> {
    let pieces = prop::sample::select(vec![
        "the",
        "ferry",
        " ",
        "  ",
        "\t",
        ".",
        ",",
        ":",
        "::",
        "10:30:45",
        "--",
        "---",
        "...",
        "\"",
        "'",
        "*",
        "_",
        "[",
        "]",
        "(",
        ")",
        "{",
        "}",
        "\\",
        "#",
        ">",
        "-",
        "+",
        "=",
        "~",
        "^",
        "!",
        "`",
        "|",
        "$",
        "%",
        "&",
        "<",
        "1.",
        "a)",
        "I.",
        "iv.",
        "\u{A0}",
        "\u{202F}",
        "\u{E9}",
        "\u{2019}",
        "\u{2014}",
        "\u{3C6}",
        "\u{1F600}",
    ]);
    proptest::collection::vec(pieces, 1..14).prop_map(|p| p.concat())
}

/// `text` as RTF, every character escaped the way Scrivener for Windows writes it.
fn windows_rtf(paragraphs: &[(String, bool, bool)]) -> String {
    let mut out = String::from(
        "{\\rtf1\\ansi\\ansicpg1252\\uc1\\deff0{\\fonttbl{\\f0\\fmodern Cambria;}}\n\\pard\\plain ",
    );
    for (index, (text, bold, italic)) in paragraphs.iter().enumerate() {
        if index > 0 {
            out.push_str("\n\\par\\plain ");
        }
        out.push_str(&format!(
            "{{\\f0\\b{}\\i{} ",
            u8::from(*bold),
            u8::from(*italic)
        ));
        for c in text.chars() {
            match c {
                '\\' | '{' | '}' => {
                    out.push('\\');
                    out.push(c);
                }
                '\t' => out.push_str("\\tab "),
                ' '..='~' => out.push(c),
                _ => {
                    let mut units = [0u16; 2];
                    for unit in c.encode_utf16(&mut units).iter() {
                        out.push_str(&format!("@u{}?", *unit as i16));
                    }
                }
            }
        }
        out.push('}');
    }
    out.push('}');
    out
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 32, ..ProptestConfig::default() })]

    /// Whatever the words, the stored Djot reads back as the words, once loaded and once
    /// more after an editor saves it.
    #[test]
    fn styled_paragraphs_read_back_through_djot(
        paragraphs in proptest::collection::vec((words(), any::<bool>(), any::<bool>()), 1..5)
    ) {
        // A paragraph that reads as a scene break becomes one: another test's subject.
        prop_assume!(paragraphs.iter().all(|(text, _, _)| {
            skribisto_model::scene_break::tier_of_plain_line(text.trim()).is_none()
        }));
        let document = assembled(&windows_rtf(&paragraphs));
        // A paragraph of nothing but white space is no paragraph (`rich` drops it, a
        // no-break space included); the emitter trims only the white space Djot would.
        let want: Vec<String> = paragraphs
            .iter()
            .filter(|(text, _, _)| !text.trim().is_empty())
            .map(|(text, _, _)| text.trim_matches(|c: char| c.is_ascii_whitespace()).to_string())
            .collect();
        let blocks = prose(&document);
        if want.is_empty() {
            prop_assert!(blocks.is_empty());
            return Ok(());
        }
        prop_assert_eq!(blocks.len(), 1);
        let want = want.join("\n");
        let (djot, text) = &blocks[0];
        prop_assert_eq!(text, &want);
        let (loaded, saved) = reload(djot);
        prop_assert_eq!(&loaded, &want);
        prop_assert_eq!(&saved, &want);
    }
}

// ── what the reader counted and the mapping must not lose ───────────────────

#[test]
fn text_after_a_stray_closing_brace_is_counted_not_lost_without_a_word() {
    // One closing brace too many ends the document; the reader reads nothing after it.
    let conversion = convert(r"{\rtf1 The ferry was late.\par}} The harbour was not.\par}");
    assert_eq!(texts(&conversion.document), ["The ferry was late."]);
    // The stray brace, the sentence, its paragraph mark and the last brace.
    assert_eq!(conversion.not_carried.after_document, 4);
}

#[test]
fn a_group_the_reader_does_not_know_is_counted() {
    let conversion = convert(r"{\rtf1 Before{\*\madeup a table of its own} after.}");
    assert_eq!(texts(&conversion.document), ["Before after."]);
    assert_eq!(conversion.not_carried.unread_destinations, 1);
}

#[test]
fn a_pdf_written_into_the_text_is_reported_where_it_stands() {
    // Scrivener 2 for macOS writes a PDF dropped into a document whole into its RTF.
    let conversion = convert(
        r"{\rtf1 See {\*\scrivenerpdf {{\*\pdffilename Harbour survey} 255044462d312e33}} here.}",
    );
    let images: Vec<String> = runs(&conversion.document, 0)
        .iter()
        .filter_map(|r| r.image.as_ref().map(|i| i.src.clone()))
        .collect();
    assert!(images.is_empty(), "no picture the editor could not show");
    assert_eq!(texts(&conversion.document), ["See  here."]);
    assert_eq!(
        conversion.diagnostics,
        [ImportDiagnostic::ImageNotIngested {
            path: ORIGIN.into(),
            target: "Harbour survey".into(),
            reason: PictureRefusal::Unsupported { format: "PDF" },
        }]
    );
    assert!(
        conversion.not_carried.is_empty(),
        "{:?}",
        conversion.not_carried
    );
}

#[test]
fn a_comment_starting_in_a_blank_paragraph_of_a_cell_starts_where_the_text_does() {
    // The range opens after white space in a paragraph of its own, which the cell does not
    // keep: the comment covers the word, not its last letter.
    let conversion = convert(
        r"{\rtf1\trowd\pard\intbl    {\*\atrfstart 1}\par\pard\intbl word{\*\atrfend 1}\cell\row\pard After.{\*\annotation{\*\atnref 1}Note.}}",
    );
    let document = &conversion.document;
    let RichBlock::Table { rows } = &document.blocks[0] else {
        panic!("expected a table, got {:?}", document.blocks[0]);
    };
    let cell: String = rows[0][0].iter().map(|r| r.text.as_str()).collect();
    assert_eq!(cell, "word");
    let comment = &document.annotations[0];
    assert_eq!(
        (comment.block_index, comment.start, comment.length),
        (0, 0, 4)
    );
}

#[test]
fn replies_thread_under_the_comment_their_chain_opens_with() {
    // Three comments in a chain, two answering each other in a loop, one answering a comment
    // the file does not hold.
    let conversion = convert(
        r"{\rtf1 Text.{\*\annotation{\*\atnref 1}First.}{\*\annotation{\*\atnref 2}{\*\atnparent 1}Second.}{\*\annotation{\*\atnref 3}{\*\atnparent 2}Third.}{\*\annotation{\*\atnref 4}{\*\atnparent 5}Loop one.}{\*\annotation{\*\atnref 5}{\*\atnparent 4}Loop two.}{\*\annotation{\*\atnref 6}{\*\atnparent 9}Dangling.}}",
    );
    let text = |paragraphs: &[Vec<Run>]| -> String {
        paragraphs
            .iter()
            .flat_map(|p| p.iter().map(|r| r.text.clone()))
            .collect()
    };
    let threads: Vec<(String, Vec<String>)> = conversion
        .document
        .annotations
        .iter()
        .map(|a| {
            (
                text(&a.paragraphs),
                a.replies.iter().map(|r| text(&r.paragraphs)).collect(),
            )
        })
        .collect();
    assert_eq!(
        threads,
        [
            (
                "First.".to_string(),
                vec!["Second.".to_string(), "Third.".to_string()]
            ),
            ("Loop one.".to_string(), vec![]),
            ("Loop two.".to_string(), vec![]),
            ("Dangling.".to_string(), vec![]),
        ]
    );
    assert!(
        conversion
            .diagnostics
            .contains(&ImportDiagnostic::CommentRepliesFlattened {
                path: ORIGIN.into(),
                count: 3
            })
    );
}

// ── what a mapping costs ─────────────────────────────────────────────────────

/// How long mapping `doc` took, on a thread of its own, or `None` when it had not finished
/// after `budget`. A mapping that runs over is left to finish on its thread: there is no way
/// to stop it from outside, and it ends with the test binary.
fn mapped_within(doc: rtf_reader::Document, budget: Duration) -> Option<Duration> {
    let (sender, receiver) = mpsc::channel();
    let started = Instant::now();
    std::thread::Builder::new()
        .spawn(move || {
            let conversion = to_rich(&doc, &RtfOptions::new(ORIGIN));
            // The receiver is gone only once the test has already failed.
            let _ = sender.send(conversion.document.blocks.len());
        })
        .ok()?;
    receiver
        .recv_timeout(budget)
        .ok()
        .map(|_| started.elapsed())
}

/// Each shape made the mapping walk a whole table of the document once per paragraph or per
/// comment: minutes for a few megabytes that reading takes a fraction of a second over.
/// Mapping must cost about what reading does.
#[test]
fn mapping_costs_about_what_reading_does() {
    let styles = {
        let mut rtf = String::from(r"{\rtf1{\stylesheet");
        for number in 0..60_000 {
            rtf.push_str(&format!(r"{{\s{number} Style;}}"));
        }
        rtf.push_str(r"}\s999999 ");
        rtf.push_str(&r"Text.\par ".repeat(60_000));
        rtf.push('}');
        rtf
    };
    let lists = {
        let mut rtf = String::from(r"{\rtf1{\*\listoverridetable");
        for number in 0..60_000 {
            rtf.push_str(&format!(r"{{\listoverride\listid{number}\ls{number}}}"));
        }
        rtf.push_str(r"}\ls999999 ");
        rtf.push_str(&r"Item.\par ".repeat(60_000));
        rtf.push('}');
        rtf
    };
    let replies = {
        let mut rtf = String::from(r"{\rtf1 Text.{\*\annotation{\*\atnref 0}First.}");
        for number in 1..20_000 {
            rtf.push_str(&format!(
                r"{{\*\annotation{{\*\atnref {number}}}{{\*\atnparent {}}}Reply.}}",
                number - 1
            ));
        }
        rtf.push('}');
        rtf
    };
    let mut slow = Vec::new();
    for (shape, rtf) in [("styles", styles), ("lists", lists), ("replies", replies)] {
        let started = Instant::now();
        let doc = rtf_reader::parse(rtf.as_bytes()).expect("the shape is RTF");
        let read = started.elapsed();
        let budget = read * 10 + Duration::from_millis(500);
        if mapped_within(doc, budget).is_none() {
            slow.push(format!(
                "{shape}: read in {read:?}, not mapped within {budget:?}"
            ));
        }
    }
    assert!(slow.is_empty(), "{slow:#?}");
}
