// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A text from RTF to stored Djot: the dialect, the mapping and the assembly together.

use std::cell::{Cell, RefCell};

use document_ingest::pictures::DocumentPictures;
use document_ingest::sources::rtf::SourcedPicture;
use document_ingest::{SourceBlock, SourceDocument};

use super::*;

/// The pictures of a test project: whatever the texts hold, and one linked item.
struct TestPictures {
    store: RefCell<DocumentPictures>,
    linked: Vec<u8>,
}

impl TestPictures {
    fn new() -> Self {
        TestPictures {
            store: RefCell::new(DocumentPictures::new()),
            linked: scrivener_fixtures::TINY_PNG.to_vec(),
        }
    }
}

impl Pictures for TestPictures {
    fn embedded(
        &self,
        key: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<SourcedPicture, PictureRefusal> {
        let brought = self
            .store
            .borrow_mut()
            .bring(key, name, || Ok(bytes.to_vec()));
        brought.result.map(|placed| SourcedPicture {
            placed,
            first_frame_only: false,
        })
    }

    fn linked(&self, uuid: &str) -> Result<SourcedPicture, PictureRefusal> {
        let bytes = self.linked.clone();
        let brought = self
            .store
            .borrow_mut()
            .bring(uuid, "linked.png", move || Ok(bytes));
        brought.result.map(|placed| SourcedPicture {
            placed,
            first_frame_only: false,
        })
    }
}

fn stream<'a>(splits: &'a [u32]) -> Stream<'a> {
    Stream {
        at: Place::Text { item: 7 },
        key: "Files/Data/X/content.rtf",
        styles: styles::StreamStyles::default(),
        sidecar: None,
        title: None,
        footnote_marker: None,
        scene_breaks: dialect::SceneBreaks::default(),
        splits,
        compiled: true,
        struck_compiled_away: false,
    }
}

fn convert(rtf: &str, splits: &[u32]) -> Converted {
    let pictures = TestPictures::new();
    let counter = Cell::new(0u32);
    let mint = || {
        counter.set(counter.get() + 1);
        format!("fn{}", counter.get())
    };
    rtf_stream(rtf.as_bytes(), &stream(splits), &pictures, &mint)
}

/// The Djot of a part, its blocks joined as a row joins them.
fn djot(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } | SourceBlock::Epigraph { djot, .. } => {
                Some(djot.clone())
            }
            SourceBlock::SceneBreak { tier } => {
                Some(skribisto_model::scene_break::canonical_djot(*tier).to_string())
            }
            SourceBlock::Heading { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn plain(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .map(|b| b.plain_text().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn a_text_with_a_footnote_and_a_comment_reaches_the_rich_model() {
    let out = convert(
        r"{\rtf1 The ferry was late\{\\Scrv_fn=It always was.\\end_Scrv_fn\} again. The keeper \{\\Scrv_annot \\text=Check this.\\end_Scrv_annot\} counted.\par}",
        &[],
    );
    assert_eq!(out.parts.len(), 1);
    let part = &out.parts[0];
    assert!(djot(part).contains("[^fn1]"), "{}", djot(part));
    assert_eq!(part.footnotes.len(), 1);
    assert_eq!(part.footnotes[0].label, "fn1");
    assert_eq!(part.annotations.len(), 1);
    assert!(
        out.notices
            .iter()
            .any(|n| matches!(n, ProseNotice::AnnotationsBecameComments { count: 1, .. }))
    );
}

/// A document holding nothing but an annotation keeps it as its text (X-03).
#[test]
fn a_document_holding_only_an_annotation_keeps_it_as_prose() {
    let out = convert(
        r"{\rtf1 \{\\Scrv_annot \\color=\{\\R=1\\G=0\\B=0\} \\text=All footnotes will be inserted here.\\end_Scrv_annot\}\par}",
        &[],
    );
    assert_eq!(plain(&out.parts[0]), "All footnotes will be inserted here.");
    assert!(out.parts[0].annotations.is_empty());
    assert!(
        out.notices
            .iter()
            .any(|n| matches!(n, ProseNotice::AnnotationOnlyDocumentKept { .. }))
    );
}

/// A text split at its chapter headings: each part what follows its heading, the heading
/// itself the part's title and not its text.
#[test]
fn a_split_text_hands_each_part_what_follows_its_heading() {
    // Blocks: 0 lead, 1 heading, 2 text, 3 heading, 4 text.
    let out = convert(
        r"{\rtf1 A word first.\par CHAPTER I\par It began\{\\Scrv_fn=A note.\\end_Scrv_fn\}.\par CHAPTER II\par It went on.\par}",
        &[1, 3],
    );
    assert_eq!(out.parts.len(), 3);
    assert_eq!(plain(&out.parts[0]), "A word first.");
    assert_eq!(plain(&out.parts[1]), "It began\u{FFFC}.");
    assert_eq!(
        out.parts[1].footnotes.len(),
        1,
        "the note goes with its part"
    );
    assert_eq!(plain(&out.parts[2]), "It went on.");
    assert!(out.parts[2].footnotes.is_empty());
}

#[test]
fn an_embedded_picture_is_brought_in_and_a_linked_one_too() {
    let hex: String = scrivener_fixtures::TINY_PNG
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let rtf = [
        r"{\rtf1 {\*\shppict{\pict{\*\nisusfilename map}\picw1\pich1\picwgoal300\pichgoal300\pngblip ",
        hex.as_str(),
        r"}} and \{$SCRImageLink[w:40;h:20]=$PROJECT://682DB6B7-3F22-4ED6-8E67-DF568697A79F.png\}.\par}",
    ]
    .concat();
    let out = convert(&rtf, &[]);
    let text = djot(&out.parts[0]);
    assert_eq!(
        text.matches("](assets/").count(),
        2,
        "both pictures are named by their place in the project: {text}"
    );
    assert!(text.contains("{width=20 height=20}"), "{text}");
    assert!(text.contains("{width=40 height=20}"), "{text}");
}

/// An oversized picture costs only itself: the reader drops its bytes and the text around
/// it arrives.
#[test]
fn text_around_a_picture_too_large_to_keep_arrives() {
    let mut rtf = String::from(r"{\rtf1 Before the picture {\pict\pngblip\bin");
    let big = 70usize << 20;
    rtf.push_str(&big.to_string());
    rtf.push(' ');
    let mut bytes = rtf.into_bytes();
    bytes.extend(std::iter::repeat_n(0u8, big));
    bytes.extend_from_slice(br"} and after it.\par}");
    let pictures = TestPictures::new();
    let counter = Cell::new(0u32);
    let mint = || {
        counter.set(counter.get() + 1);
        format!("fn{}", counter.get())
    };
    let out = rtf_stream(&bytes, &stream(&[]), &pictures, &mint);
    assert_eq!(plain(&out.parts[0]), "Before the picture  and after it.");
    assert!(
        out.notices.iter().any(|n| matches!(
            n,
            ProseNotice::Document { key, .. } if key == "image-not-ingested-too-large"
        )),
        "{:?}",
        out.notices
    );
}

/// Chinese in the GBK code page Scrivener for Mac writes for a Chinese project, and a
/// Hebrew paragraph marked right to left (A8).
#[test]
fn chinese_and_right_to_left_text_arrive_as_written() {
    // 概念 in GBK: B8 C5 C4 EE.
    let chinese = r"{\rtf1\ansi\ansicpg936{\fonttbl\f0\fnil\fcharset134 PingFangSC-Regular;}\f0 \'b8\'c5\'c4\'ee\par}";
    let out = convert(chinese, &[]);
    assert_eq!(plain(&out.parts[0]), "\u{6982}\u{5FF5}");

    // Hebrew שלום as Unicode escapes, built at run time, in a right-to-left paragraph.
    let shalom: String = [0x5E9u32, 0x5DC, 0x5D5, 0x5DD]
        .iter()
        .map(|c| format!("{}u{c}?", '\\'))
        .collect();
    let hebrew = format!(r"{{\rtf1\ansi\uc1\rtlpar {shalom} \par}}");
    let out = convert(&hebrew, &[]);
    assert_eq!(plain(&out.parts[0]), "\u{5E9}\u{5DC}\u{5D5}\u{5DD}");
    assert!(
        djot(&out.parts[0]).contains("direction=rtl"),
        "{}",
        djot(&out.parts[0])
    );
}

/// A text that is not RTF is read as plain text, and said so.
#[test]
fn plain_text_named_rtf_is_read_as_plain_text() {
    let out = convert("Just words.\nTwo lines.", &[]);
    assert_eq!(plain(&out.parts[0]), "Just words.\nTwo lines.");
    assert!(
        out.notices
            .iter()
            .any(|n| matches!(n, ProseNotice::ReadAsPlainText { .. }))
    );
}

#[test]
fn a_heading_style_stays_inside_the_text() {
    let out = convert(
        r"{\rtf1 <$Scr_H::2>A section<!$Scr_H::2>\par Its text.\par}",
        &[],
    );
    let text = djot(&out.parts[0]);
    assert!(text.starts_with("## A section"), "{text}");
    assert!(
        !out.parts[0]
            .blocks
            .iter()
            .any(|b| matches!(b, SourceBlock::Heading { .. })),
        "never a row of its own"
    );
}

/// A chapter heading that became its part's title holds no picture: one it showed is named
/// as left out, as the document importer names a heading's picture, never dropped unsaid.
#[test]
fn a_picture_in_a_heading_that_became_a_title_is_reported() {
    let hex: String = scrivener_fixtures::TINY_PNG
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let rtf = [
        r"{\rtf1 A word first.\par CHAPTER I {\pict{\*\nisusfilename crest}\picw1\pich1\pngblip ",
        hex.as_str(),
        r"}\par It began.\par}",
    ]
    .concat();
    let out = convert(&rtf, &[1]);
    assert_eq!(plain(&out.parts[1]), "It began.");
    assert!(
        out.notices.iter().any(|n| matches!(
            n,
            ProseNotice::Document { key, detail, .. }
                if key == "image-not-ingested-heading" && detail == "crest"
        )),
        "{:?}",
        out.notices
    );
}

/// Two chapter headings back to back: the first chapter holds no text, so a comment on its
/// heading goes to the first paragraph after it, in the next chapter, rather than into a
/// part that stores nothing.
#[test]
fn a_comment_on_a_heading_with_no_text_after_it_goes_to_the_next_words() {
    let side = sidecar::Sidecar {
        entries: vec![sidecar::Entry {
            id: Some("AAAAAAAA-0000-0000-0000-000000000001".to_string()),
            footnote: false,
            author: None,
            coloured: false,
            body: r"{\rtf1 Rename this chapter.}".to_string(),
            range: None,
        }],
        links: Vec::new(),
    };
    let rtf = r#"{\rtf1 {\field{\*\fldinst{HYPERLINK "scrivcmt://AAAAAAAA-0000-0000-0000-000000000001"}}{\fldrslt CHAPTER I}}\par CHAPTER II\par It began.\par}"#;
    let pictures = TestPictures::new();
    let counter = Cell::new(0u32);
    let mint = || {
        counter.set(counter.get() + 1);
        format!("fn{}", counter.get())
    };
    let mut stream = stream(&[0, 1]);
    stream.sidecar = Some(&side);
    let out = rtf_stream(rtf.as_bytes(), &stream, &pictures, &mint);
    assert_eq!(out.parts.len(), 3);
    assert!(out.parts[1].blocks.is_empty());
    assert!(out.parts[1].annotations.is_empty(), "{:?}", out.parts[1]);
    assert_eq!(plain(&out.parts[2]), "It began.");
    assert_eq!(out.parts[2].annotations.len(), 1, "{:?}", out.parts[2]);
    assert_eq!(out.parts[2].annotations[0].block_index, 0);
}

/// What the RTF holds and the text cannot carry is said, never dropped unsaid: words after
/// a stray closing brace (every reader stops there), a header's words, characters the file
/// wrote in no encoding it names.
#[test]
fn what_a_text_cannot_carry_is_reported() {
    let out = convert(
        r"{\rtf1 Hello there.\par} And the rest of the chapter.\par",
        &[],
    );
    assert_eq!(plain(&out.parts[0]), "Hello there.");
    assert!(
        out.notices.iter().any(|n| matches!(
            n,
            ProseNotice::LeftOut {
                what: LeftOut::AfterDocument,
                ..
            }
        )),
        "{:?}",
        out.notices
    );
    let out = convert(r"{\rtf1 {\header Running head}Hello.\par}", &[]);
    assert!(
        out.notices.iter().any(|n| matches!(
            n,
            ProseNotice::LeftOut {
                what: LeftOut::HeaderOrFooter,
                count: 1,
                ..
            }
        )),
        "{:?}",
        out.notices
    );
    let out = convert(r"{\rtf1\uc1 Half \u-10179? a pair.\par}", &[]);
    assert!(
        out.notices
            .iter()
            .any(|n| matches!(n, ProseNotice::UndecodableCharacters { count: 1, .. })),
        "{:?}",
        out.notices
    );
}

// ── the round trip: writer text that reads as Djot ──────────────────────────────────

/// `text` as RTF text: its braces and backslashes escaped, a tab as `\tab`, anything past
/// ASCII as `\uN` (a pair of them past the first plane).
fn rtf_escape(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\\' => out.push_str(r"\\"),
            '{' => out.push_str(r"\{"),
            '}' => out.push_str(r"\}"),
            '\t' => out.push_str(r"\tab "),
            c if c.is_ascii() && !c.is_ascii_control() => out.push(c),
            c => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    out.push_str(&format!(r"\u{}?", *unit as i16));
                }
            }
        }
    }
    out
}

/// What the editor reads a Djot text as, and what it reads once the editor saved it.
fn through_the_editor(djot: &str) -> (Option<String>, Option<String>) {
    let doc = text_document::TextDocument::new();
    let read = doc
        .set_djot_sync(djot)
        .ok()
        .and_then(|_| doc.to_addressable_text().ok());
    let saved = doc.to_djot().ok().and_then(|saved| {
        let again = text_document::TextDocument::new();
        again.set_djot_sync(&saved).ok()?;
        again.to_addressable_text().ok()
    });
    (read, saved)
}

/// What a paragraph of `text` reads as once stored: its edge spaces left out (Djot keeps
/// none), and a line of Skribisto's own break vocabulary written as the break it is.
fn stored_as(text: &str) -> String {
    let text = skrib_format::trim_djot_whitespace(text);
    match skribisto_model::scene_break::tier_of_plain_line(text.trim()) {
        Some(tier) => skribisto_model::scene_break::canonical_plain(tier).to_string(),
        None => text.to_string(),
    }
}

/// Writer text holding what Djot would read as markup comes back as written, through the
/// editor's read and through its save.
#[test]
fn writer_text_that_reads_as_djot_comes_back_as_written() {
    let samples = [
        "* * *",
        "***",
        "- item",
        "1. one",
        "a) x",
        "# heading",
        "> quote",
        "| a | b |",
        "::: div",
        "```",
        "[^1]: note",
        "[link](url)",
        "{#id}",
        "_under_",
        "*bold*",
        "`code`",
        r"back\slash",
        "x -- y --- z ... w",
        "10:30:45",
        "'quote' \"dq\"",
        ":smile:",
        "![img](x.png)",
        "<$n>",
        "<https://x.org>",
        "+ plus",
        "~ tilde",
        "I. Roman",
        "iv) roman",
        "(a) paren",
        "A. Letter",
        "= equal",
        "{=x=}",
        "$math$",
        "\u{a0}nbsp edge\u{a0}",
        "tab\tinside",
        "  lead",
        "^sup^",
        "~sub~",
        "{-del-}",
        "{+ins+}",
        "--",
        "...",
        "a\\",
        "\\",
        "[",
        "]",
        "![",
        "*",
        "_",
        "#",
        ">",
        "`",
        "|",
        ":::",
        "- ",
        "1.",
        "\u{2163}. roman numeral",
        "\u{65E5}\u{672C} * x",
        "emoji \u{1F600} *x*",
    ];
    for sample in samples {
        let rtf = format!(r"{{\rtf1\uc1 {}\par}}", rtf_escape(sample));
        let out = convert(&rtf, &[]);
        let stored = djot(&out.parts[0]);
        let (read, saved) = through_the_editor(&stored);
        assert_eq!(
            read.as_deref(),
            Some(stored_as(sample).as_str()),
            "{sample:?} as {stored:?}"
        );
        assert_eq!(
            saved, read,
            "{sample:?}: the editor's save changed {stored:?}"
        );
        assert!(
            !out.notices.iter().any(|n| n.key() == "prose-not-verbatim"),
            "{sample:?}: {:?}",
            out.notices
        );
    }
}

/// A text whose styles make its paragraphs a code block, a heading and a code span.
fn styled(rtf: &str) -> Converted {
    let mut table = styles::StyleTable::default();
    for (id, name, kind, heading) in [
        (
            "C0DE0000-0000-0000-0000-000000000000",
            "Code Block",
            styles::StyleType::Paragraph,
            None,
        ),
        (
            "4EAD0000-0000-0000-0000-000000000000",
            "Heading 2",
            styles::StyleType::Both,
            Some(2),
        ),
        (
            "C5A00000-0000-0000-0000-000000000000",
            "Code Span",
            styles::StyleType::Character,
            None,
        ),
    ] {
        table.insert(
            id,
            styles::StyleDef {
                name: name.to_string(),
                kind,
                heading,
                compile: None,
            },
        );
    }
    let list: Vec<String> = [
        "C0DE0000-0000-0000-0000-000000000000",
        "4EAD0000-0000-0000-0000-000000000000",
        "C5A00000-0000-0000-0000-000000000000",
    ]
    .map(str::to_string)
    .to_vec();
    let pictures = TestPictures::new();
    let counter = Cell::new(0u32);
    let mint = || {
        counter.set(counter.get() + 1);
        format!("fn{}", counter.get())
    };
    let mut stream = stream(&[]);
    stream.styles = styles::StreamStyles {
        list: Some(&list),
        table: Some(&table),
    };
    rtf_stream(rtf.as_bytes(), &stream, &pictures, &mint)
}

/// Text in a style the project's compile format deletes, or passes through as raw markup,
/// stays prose: it is the writer's, and nothing in Skribisto can leave it out. The export
/// will print it, so it is said, by style, with what it covered (D5.8).
#[test]
fn a_style_the_compile_leaves_out_is_kept_and_said() {
    let mut table = styles::StyleTable::default();
    for (id, name, kind) in [
        (
            "DE1E0000-0000-0000-0000-000000000000",
            "Private note",
            styles::StyleType::Paragraph,
        ),
        (
            "4A500000-0000-0000-0000-000000000000",
            "raw-tex",
            styles::StyleType::Character,
        ),
        (
            "B10C0000-0000-0000-0000-000000000000",
            "Block Quote",
            styles::StyleType::Paragraph,
        ),
    ] {
        table.insert(
            id,
            styles::StyleDef {
                name: name.to_string(),
                kind,
                heading: None,
                compile: None,
            },
        );
    }
    table.mark_compiled(
        &["private note".to_string(), "Block Quote".to_string()],
        &["RAW-TEX".to_string()],
    );
    let list: Vec<String> = [
        "DE1E0000-0000-0000-0000-000000000000",
        "4A500000-0000-0000-0000-000000000000",
        "B10C0000-0000-0000-0000-000000000000",
    ]
    .map(str::to_string)
    .to_vec();
    let pictures = TestPictures::new();
    let mint = || "fn1".to_string();
    let mut stream = stream(&[]);
    stream.styles = styles::StreamStyles {
        list: Some(&list),
        table: Some(&table),
    };
    let rtf = r"{\rtf1 <$Scr_Ps::0>Check the dates.<!$Scr_Ps::0>\par <$Scr_Ps::0>And the names.<!$Scr_Ps::0>\par Some <$Scr_Cs::1>\\emph<!$Scr_Cs::1> here.\par <$Scr_Ps::2>A quotation.<!$Scr_Ps::2>\par}";
    let out = rtf_stream(rtf.as_bytes(), &stream, &pictures, &mint);
    let text: String = out.parts[0]
        .blocks
        .iter()
        .map(SourceBlock::plain_text)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(text.contains("Check the dates."), "{text}");
    assert!(text.contains(r"\emph"), "{text}");
    let compiled: Vec<(String, CompileTreatment, u32)> = out
        .notices
        .iter()
        .filter_map(|n| match n {
            ProseNotice::CompiledAwayStylePrinted {
                name,
                treatment,
                count,
                ..
            } => Some((name.clone(), *treatment, *count)),
            _ => None,
        })
        .collect();
    assert_eq!(
        compiled,
        [
            ("Block Quote".to_string(), CompileTreatment::Deleted, 1),
            ("Private note".to_string(), CompileTreatment::Deleted, 2),
            ("raw-tex".to_string(), CompileTreatment::RawMarkup, 5),
        ],
        "a paragraph style counts paragraphs, a character style characters, whatever \
         the style means here"
    );
    let keys: Vec<&str> = out.notices.iter().map(ProseNotice::key).collect();
    assert!(keys.contains(&"scrivener-prose-compile-deleted-style-printed"));
    assert!(keys.contains(&"scrivener-prose-compile-raw-style-printed"));
}

/// Struck-through text stays struck through; where the project's compile removed it from
/// the book, the writer is told Skribisto's export will print it (D5.21).
#[test]
fn struck_text_is_kept_and_said_when_the_compile_removed_it() {
    let rtf = r"{\rtf1 Kept {\strike cut words} here, {\strike  } and a note{\*\footnote {\strike x}}.\par}";
    let pictures = TestPictures::new();
    let mint = || "fn1".to_string();
    let mut stream = stream(&[]);
    let quiet = rtf_stream(rtf.as_bytes(), &stream, &pictures, &mint);
    assert!(
        !quiet
            .notices
            .iter()
            .any(|n| matches!(n, ProseNotice::StruckTextCompiledAway { .. })),
        "a compile that keeps struck text has nothing to say"
    );
    stream.struck_compiled_away = true;
    let out = rtf_stream(rtf.as_bytes(), &stream, &pictures, &mint);
    let djot: String = out.parts[0]
        .blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect();
    assert!(djot.contains("{-cut words-}"), "{djot}");
    let struck: Vec<u32> = out
        .notices
        .iter()
        .filter_map(|n| match n {
            ProseNotice::StruckTextCompiledAway { count, .. } => Some(*count),
            _ => None,
        })
        .collect();
    assert_eq!(
        struck,
        [9],
        "the words' letters and the footnote's, never the struck spaces"
    );
    // Text no compile prints (a note, a text left out of the book): nothing to say.
    stream.compiled = false;
    let quiet = rtf_stream(rtf.as_bytes(), &stream, &pictures, &mint);
    assert!(
        !quiet
            .notices
            .iter()
            .any(|n| matches!(n, ProseNotice::StruckTextCompiledAway { .. }))
    );
}

/// A code block's line, a heading kept in the prose and a code span come back as written,
/// through the editor's read and its save. A line of code is code even when it reads as a
/// scene break.
#[test]
fn code_and_headings_come_back_as_written_through_the_editor() {
    let samples = [
        "``` fence",
        "`",
        "``x``",
        "{",
        "}",
        "a {b} c",
        "\\",
        "* * *",
        "#",
        "***",
        "# not heading",
        "- list",
        "tab\there",
        "x\u{a0}",
        ":::",
        "~~~",
        "a`",
        "`a",
        "<$n>",
        "[^1]",
        "\u{65E5}\u{672C}",
    ];
    for sample in samples {
        for (label, open, close) in [
            ("code", r"<$Scr_Ps::0>", r"<!$Scr_Ps::0>"),
            ("heading", r"<$Scr_Ps::1>", r"<!$Scr_Ps::1>"),
            ("span", r"<$Scr_Cs::2>", r"<!$Scr_Cs::2>"),
        ] {
            let rtf = format!(
                r"{{\rtf1\uc1 Before it.\par {open}{}{close}\par After it.\par}}",
                rtf_escape(sample)
            );
            let out = styled(&rtf);
            let stored = djot(&out.parts[0]);
            let (read, saved) = through_the_editor(&stored);
            let middle = if label == "code" {
                skrib_format::trim_djot_whitespace(sample).to_string()
            } else {
                stored_as(sample)
            };
            assert_eq!(
                read.as_deref(),
                Some(format!("Before it.\n{middle}\nAfter it.").as_str()),
                "{label} {sample:?} as {stored:?}"
            );
            assert_eq!(
                saved, read,
                "{label} {sample:?}: the save changed {stored:?}"
            );
        }
    }
}

mod round_trip {
    use super::*;
    use proptest::prelude::*;

    /// Text made mostly of what Djot reads as markup, with letters, spaces and a few
    /// characters past ASCII between.
    fn piece() -> impl Strategy<Value = String> {
        proptest::collection::vec(
            prop_oneof![
                3 => proptest::sample::select(vec![
                    '*', '_', '[', ']', '^', '{', '}', '#', '>', '`', '|', '-', ':', '\\', '!',
                    '~', '+', '=', '<', '$', '.', '(', ')', '\'', '"', '@', '%', '&', '/', '?',
                    ';', ',',
                ]),
                3 => proptest::sample::select(vec!['a', 'b', 'I', 'v', 'x', 'Z', '1', '0', '9']),
                2 => Just(' '),
                1 => proptest::sample::select(vec![
                    '\t', '\u{a0}', '\u{202f}', '\u{e9}', '\u{65E5}', '\u{2014}', '\u{2026}',
                    '\u{1F600}', '\u{2011}', '\u{ad}',
                ]),
            ],
            0..14,
        )
        .prop_map(|chars| chars.into_iter().collect())
    }

    /// A paragraph: a few runs, each plain, bold, italic or underlined.
    fn paragraph() -> impl Strategy<Value = Vec<(String, u8)>> {
        proptest::collection::vec((piece(), 0u8..4), 1..4)
    }

    fn rtf_of(paragraphs: &[Vec<(String, u8)>]) -> String {
        let mut rtf = String::from(r"{\rtf1\uc1 ");
        for p in paragraphs {
            for (text, style) in p {
                let body = rtf_escape(text);
                match style {
                    1 => rtf.push_str(&format!(r"{{\b {body}}}")),
                    2 => rtf.push_str(&format!(r"{{\i {body}}}")),
                    3 => rtf.push_str(&format!(r"{{\ul {body}}}")),
                    _ => rtf.push_str(&format!("{{{body}}}")),
                }
            }
            rtf.push_str(r"\par ");
        }
        rtf.push('}');
        rtf
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 256, ..ProptestConfig::default() })]

        /// Any writer text, formatted or not, is stored as Djot the editor reads back as
        /// that text, and still reads so once the editor has saved it. A paragraph of
        /// blank space alone is a blank line, which the import reports and does not keep.
        #[test]
        fn writer_text_survives_the_import_and_the_editor(
            paragraphs in proptest::collection::vec(paragraph(), 1..5)
        ) {
            let rtf = rtf_of(&paragraphs);
            let out = convert(&rtf, &[]);
            let stored = djot(&out.parts[0]);
            let expected: Vec<String> = paragraphs
                .iter()
                .map(|p| p.iter().map(|(t, _)| t.as_str()).collect::<String>())
                .filter(|t| !t.chars().all(char::is_whitespace))
                .map(|t| stored_as(&t))
                .collect();
            let (read, saved) = through_the_editor(&stored);
            prop_assert_eq!(read.clone(), Some(expected.join("\n")), "{:?} from {:?}", stored, rtf);
            prop_assert_eq!(saved, read, "the editor's save changed {:?}", stored);
        }
    }
}

/// Every finding of the document importer's, one of each and every picture's reason, for
/// the test below: [`every_diagnostic_is_listed`] keeps the list whole.
fn every_document_diagnostic() -> Vec<ImportDiagnostic> {
    use ImportDiagnostic as D;
    let path = || "Draft › Scene".to_string();
    let mut every = vec![
        D::FileUnreadable {
            path: path(),
            reason: "denied".into(),
        },
        D::NestedTooDeep {
            path: path(),
            part: "content".into(),
            limit: 256,
        },
        D::ArchiveTooLarge {
            path: path(),
            part: "content".into(),
        },
        D::LossyDecode {
            path: path(),
            replacements: 3,
        },
        D::DecodedFromBom {
            path: path(),
            encoding: "UTF-16LE",
        },
        D::EmptyFile { path: path() },
        D::NoHeadings { path: path() },
        D::UnsupportedFormat {
            path: path(),
            extension: "odt".into(),
        },
        D::FrontMatterNotFlat {
            path: path(),
            key: "tags".into(),
        },
        D::FootnotesDegraded {
            path: path(),
            count: 2,
        },
        D::FootnoteNotCarried {
            path: path(),
            count: 1,
        },
        D::RawHtmlDropped {
            path: path(),
            count: 1,
        },
        D::NestedBreakDropped {
            path: path(),
            count: 4,
        },
        D::PictureFirstFrameOnly {
            path: path(),
            target: "spin.gif".into(),
        },
        D::DuplicateTitle {
            title: "Later".into(),
            occurrences: 2,
        },
        D::HeadingLevelJump {
            title: "Deep".into(),
            from: 1,
            to: 4,
        },
        D::IllegalCombination {
            title: "A part".into(),
            kind: skribisto_model::CreateType::Part,
        },
        D::TrackedChangesFlattened {
            path: path(),
            count: 2,
            authors: vec!["Ada Rees".into(), "Marc Dubois".into()],
        },
        D::TextBoxDropped {
            path: path(),
            count: 1,
        },
        D::EmbeddedObjectDropped {
            path: path(),
            count: 2,
        },
        D::FieldFlattened {
            path: path(),
            count: 9,
        },
        D::UnknownStyleLevel {
            path: path(),
            style: "HeadingChapter".into(),
        },
        D::CommentUnanchored {
            path: path(),
            quote: "Is this the right word?".into(),
        },
        D::CommentNotCarried {
            path: path(),
            quote: "Nothing to keep me on.".into(),
        },
        D::CommentRepliesFlattened {
            path: path(),
            count: 3,
        },
        D::ProseNotVerbatim {
            path: path(),
            count: 2,
        },
        D::StyledSpacesNotCarried {
            path: path(),
            count: 3,
        },
        D::ListNestingFlattened {
            path: path(),
            count: 4,
            limit: 16,
        },
        D::TableReadAsParagraphs {
            path: path(),
            count: 1,
        },
        D::SpacesShortened {
            path: path(),
            count: 2,
            limit: 1_000,
        },
        D::EpigraphNotCarried {
            title: "A scene".into(),
            kind: skribisto_model::CreateType::Scene,
        },
        D::EpigraphPlacementAmbiguous {
            above: "Part One".into(),
            below: "Chapter One".into(),
        },
    ];
    for reason in [
        PictureRefusal::Unsupported { format: "EMF" },
        PictureRefusal::Remote,
        PictureRefusal::Outside,
        PictureRefusal::Missing,
        PictureRefusal::TooLarge,
        PictureRefusal::Unreadable,
        PictureRefusal::InHeading,
        PictureRefusal::InNote,
        PictureRefusal::InHtml,
        PictureRefusal::DocumentFull,
    ] {
        every.push(D::ImageNotIngested {
            path: path(),
            target: "cover.png".into(),
            reason,
        });
    }
    every
}

/// Which of the document importer's findings `diagnostic` is, by an exhaustive match: a new
/// kind fails to compile here until [`every_document_diagnostic`] lists it.
fn diagnostic_slot(diagnostic: &ImportDiagnostic) -> usize {
    use ImportDiagnostic as D;
    match diagnostic {
        D::FileUnreadable { .. } => 0,
        D::NestedTooDeep { .. } => 1,
        D::ArchiveTooLarge { .. } => 2,
        D::LossyDecode { .. } => 3,
        D::DecodedFromBom { .. } => 4,
        D::EmptyFile { .. } => 5,
        D::NoHeadings { .. } => 6,
        D::UnsupportedFormat { .. } => 7,
        D::FrontMatterNotFlat { .. } => 8,
        D::FootnotesDegraded { .. } => 9,
        D::FootnoteNotCarried { .. } => 10,
        D::RawHtmlDropped { .. } => 11,
        D::NestedBreakDropped { .. } => 12,
        D::ImageNotIngested { .. } => 13,
        D::PictureFirstFrameOnly { .. } => 14,
        D::DuplicateTitle { .. } => 15,
        D::HeadingLevelJump { .. } => 16,
        D::IllegalCombination { .. } => 17,
        D::TrackedChangesFlattened { .. } => 18,
        D::TextBoxDropped { .. } => 19,
        D::EmbeddedObjectDropped { .. } => 20,
        D::FieldFlattened { .. } => 21,
        D::UnknownStyleLevel { .. } => 22,
        D::CommentUnanchored { .. } => 23,
        D::CommentNotCarried { .. } => 24,
        D::CommentRepliesFlattened { .. } => 25,
        D::ProseNotVerbatim { .. } => 26,
        D::StyledSpacesNotCarried { .. } => 27,
        D::ListNestingFlattened { .. } => 28,
        D::TableReadAsParagraphs { .. } => 29,
        D::SpacesShortened { .. } => 30,
        D::EpigraphNotCarried { .. } => 31,
        D::EpigraphPlacementAmbiguous { .. } => 32,
    }
}

#[test]
fn every_diagnostic_is_listed() {
    let slots: std::collections::BTreeSet<usize> = every_document_diagnostic()
        .iter()
        .map(diagnostic_slot)
        .collect();
    assert_eq!(slots, (0..=32).collect());
}

/// A finding of the document importer's carries the one string and the one number the
/// document importer's own row carries for it, under its key, so the sentence written for
/// that row reads it alike: the level a list was flattened to, the longest run of spaces
/// kept, the ceiling a part nested past, the two levels of a heading's jump, the heading an
/// epigraph did not go to.
#[test]
fn a_document_finding_carries_what_the_document_importer_s_row_carries() {
    let at = Place::Text { item: 1 };
    for diagnostic in every_document_diagnostic() {
        let ProseNotice::Document {
            key,
            count,
            detail,
            warning,
            ..
        } = document_notice(at, &diagnostic)
        else {
            panic!("{diagnostic:?} is not a document finding");
        };
        assert_eq!(key, diagnostic.key());
        assert_eq!(
            warning,
            diagnostic.severity() != DiagnosticSeverity::Info,
            "{key}"
        );
        let (row_detail, row_count) = diagnostic.detail_and_count();
        assert_eq!((detail, i64::from(count)), (row_detail, row_count), "{key}");
    }
    // The ones the review's sentences cannot do without, spelled out.
    let carried = |diagnostic: ImportDiagnostic| match document_notice(at, &diagnostic) {
        ProseNotice::Document { count, detail, .. } => (count, detail),
        other => panic!("{other:?}"),
    };
    assert_eq!(
        carried(ImportDiagnostic::ListNestingFlattened {
            path: String::new(),
            count: 4,
            limit: 16,
        }),
        (4, "16".to_string())
    );
    assert_eq!(
        carried(ImportDiagnostic::HeadingLevelJump {
            title: "Deep".into(),
            from: 1,
            to: 4,
        }),
        (4, "1".to_string())
    );
    assert_eq!(
        carried(ImportDiagnostic::EpigraphPlacementAmbiguous {
            above: "Part One".into(),
            below: "Chapter One".into(),
        }),
        (0, "Chapter One".to_string())
    );
}
