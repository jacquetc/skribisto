// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The escaper's contract, checked against the parser the editor reads prose with.
//!
//! Everything here asks the same question: once written, does the pinned parser read the
//! Djot back as exactly the text (and the styles) that went in? The oracle is
//! [`djot_plain_text`], the parse every anchor and every editor tab uses, never a model
//! of Djot's grammar written here, because a model would share the escaper's blind spots.
//!
//! The properties compose styled runs rather than escaping one bare line, since the
//! losses that reach real documents sit where runs meet: a `-` ending one run and a `-`
//! opening the next, an `=` in a paragraph that is one italic run, a marker followed by a
//! tab. The alphabet carries every ASCII punctuation mark, the letters that can open an
//! ordered-list marker, digits, spaces, tabs, a no-break space and a few characters a
//! manuscript holds.

use proptest::prelude::*;
use text_document::{CharVerticalAlignment, FragmentContent, TextDocument};

use super::{
    DjotInlineStyle, EscapeContext, djot_plain_text, escape_djot_text, extend_inline_escapes,
    guard_djot_line_start, is_djot_whitespace, neutralise_block_start, plain_text_to_djot_verbatim,
    push_djot_run, trim_djot_whitespace,
};

const ALPHABET: &[char] = &[
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', 'a', 'A', 'b', 'B', 'c', 'C',
    'd', 'i', 'I', 'l', 'm', 'M', 'v', 'V', 'x', 'X', '0', '1', '9', ' ', ' ', '\t', '\u{a0}',
    '\u{2014}', '\u{2019}', '\u{e9}',
];

fn text_of(len: std::ops::Range<usize>) -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALPHABET), len)
        .prop_map(|chars| chars.into_iter().collect())
}

fn style() -> impl Strategy<Value = DjotInlineStyle> {
    (any::<[bool; 4]>(), 0u8..4).prop_map(|([bold, italic, underline, strike], raised)| {
        DjotInlineStyle {
            bold,
            italic,
            underline,
            strikethrough: strike,
            superscript: raised == 1,
            subscript: raised == 2,
        }
    })
}

fn runs() -> impl Strategy<Value = Vec<(String, DjotInlineStyle)>> {
    prop::collection::vec((text_of(1..7), style()), 1..6)
}

/// What one character is expected to read back as, beyond the character itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Styled {
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    superscript: bool,
    subscript: bool,
}

impl From<DjotInlineStyle> for Styled {
    fn from(s: DjotInlineStyle) -> Self {
        Self {
            bold: s.bold,
            italic: s.italic,
            underline: s.underline,
            strikethrough: s.strikethrough,
            superscript: s.superscript,
            subscript: s.subscript && !s.superscript,
        }
    }
}

/// One paragraph written the way the document importer writes it: the paragraph's edges
/// trimmed, each run through [`push_djot_run`] with its neighbours, the line guarded.
///
/// Returns the Djot, the text a reader must get back, and the style each of its
/// characters must carry. Edge whitespace of a run is written outside its delimiters, so
/// it is expected plain; a run that is only whitespace is expected plain too.
fn write_paragraph(runs: &[(String, DjotInlineStyle)]) -> (String, String, Vec<Styled>) {
    let full: String = runs.iter().map(|(t, _)| t.as_str()).collect();
    let start = full.len() - full.trim_start_matches(is_djot_whitespace).len();
    let end = full.trim_end_matches(is_djot_whitespace).len().max(start);
    let plain = Styled::from(DjotInlineStyle::default());

    let mut djot = String::new();
    let mut styles = Vec::new();
    let mut pos = 0;
    for (text, style) in runs {
        let (run_start, run_end) = (pos, pos + text.len());
        pos = run_end;
        let (s, e) = (run_start.max(start), run_end.min(end));
        if s >= e {
            continue;
        }
        let piece = &full[s..e];
        push_djot_run(&mut djot, piece, *style, &full[start..s], &full[e..end]);

        let core = trim_djot_whitespace(piece);
        let lead = piece.len() - piece.trim_start_matches(is_djot_whitespace).len();
        for (offset, _) in piece.char_indices() {
            let in_core = !core.is_empty() && offset >= lead && offset < lead + core.len();
            styles.push(if in_core { Styled::from(*style) } else { plain });
        }
    }
    (
        guard_djot_line_start(&djot),
        full[start..end].to_string(),
        styles,
    )
}

/// The style of every character of the first block, as the editor's model reads it.
fn styles_read_back(djot: &str) -> Vec<Styled> {
    let doc = TextDocument::new();
    let parsed = doc.set_djot(djot).and_then(|op| {
        op.wait_timeout(std::time::Duration::from_secs(30))
            .expect("the text-document operation did not finish within 30 s")
    });
    assert!(parsed.is_ok(), "{djot:?} did not parse: {parsed:?}");
    let mut out = Vec::new();
    if let Some(block) = doc.blocks().first() {
        for fragment in block.fragments() {
            if let FragmentContent::Text {
                text,
                format,
                offset,
                ..
            } = fragment
            {
                let styled = Styled {
                    bold: format.font_bold == Some(true),
                    italic: format.font_italic == Some(true),
                    underline: format.font_underline == Some(true),
                    strikethrough: format.font_strikeout == Some(true),
                    superscript: format.vertical_alignment
                        == Some(CharVerticalAlignment::SuperScript),
                    subscript: format.vertical_alignment == Some(CharVerticalAlignment::SubScript),
                };
                for k in 0..text.chars().count() {
                    let at = offset + k;
                    if out.len() <= at {
                        out.resize(at + 1, Styled::from(DjotInlineStyle::default()));
                    }
                    out[at] = styled;
                }
            }
        }
    }
    out
}

fn read_back(djot: &str) -> (String, usize) {
    let (text, starts) = djot_plain_text(djot).expect("the escaper's output must parse");
    (text, starts.len())
}

/// What [`plain_text_to_djot_verbatim`] promises to read back as.
fn verbatim_image(text: &str) -> String {
    text.split(['\n', '\r'])
        .map(trim_djot_whitespace)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every ASCII punctuation mark behind a backslash: what an escaper that already covers
/// every rule here would write, at the most.
fn escape_everything(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        if c.is_ascii_punctuation() {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 1500, ..ProptestConfig::default() })]

    /// The contract the document importer stores prose on: a paragraph of styled runs
    /// reads back with every character, and every character's style, exactly as written.
    #[test]
    fn a_paragraph_of_styled_runs_reads_back_verbatim(runs in runs()) {
        let (djot, expected, styles) = write_paragraph(&runs);
        prop_assume!(!expected.is_empty());

        let (text, blocks) = read_back(&djot);
        prop_assert_eq!(&text, &expected, "Djot {:?} for runs {:?}", djot, runs);
        prop_assert_eq!(blocks, 1, "one paragraph, not {} blocks: {:?}", blocks, djot);

        let read = styles_read_back(&djot);
        let chars: Vec<char> = expected.chars().collect();
        for (i, want) in styles.iter().enumerate() {
            let got = read.get(i).copied().unwrap_or(Styled::from(DjotInlineStyle::default()));
            prop_assert_eq!(
                got, *want,
                "character {} ({:?}) of {:?} from Djot {:?}", i, chars[i], expected, djot
            );
        }
    }

    /// Plain text of any shape comes back line for line.
    #[test]
    fn plain_text_reads_back_one_paragraph_per_line(
        lines in prop::collection::vec(text_of(0..12), 1..5),
        ending in prop::sample::select(vec!["\n", "\r\n", "\r", "\n\n"]),
    ) {
        let text = lines.join(ending);
        let djot = plain_text_to_djot_verbatim(&text);
        let expected = verbatim_image(&text);

        let (read, blocks) = read_back(&djot);
        prop_assert_eq!(&read, &expected, "Djot {:?} for {:?}", djot, text);
        prop_assert_eq!(blocks, expected.lines().count(), "Djot {:?}", djot);
    }

    /// Once text-document escapes a character itself, the extra rules add nothing: fed an
    /// escaper that already escapes every punctuation mark, the output is its input.
    #[test]
    fn the_extra_inline_rules_leave_an_escaped_character_alone(
        text in text_of(1..16),
        before in text_of(0..4),
        after in text_of(0..4),
        braced in any::<bool>(),
    ) {
        let escaped = escape_everything(&text);
        let context = EscapeContext { before: &before, after: &after, braced };
        prop_assert_eq!(extend_inline_escapes(&escaped, &text, context), escaped);
    }

    /// A guarded line is left alone by a second guard, which is also what the guard does
    /// to a line the upstream guard already neutralised.
    #[test]
    fn guarding_a_guarded_line_changes_nothing(text in text_of(1..16)) {
        let once = guard_djot_line_start(&text);
        prop_assert_eq!(neutralise_block_start(&once), once.clone());
        prop_assert_eq!(guard_djot_line_start(&once), once);
    }
}

/// The strings the pinned parser rewrites on the first load, one of each kind.
#[test]
fn every_named_loss_reads_back_verbatim() {
    for line in [
        "10:30:45",
        "Meet at 9:20pm-10:00pm.",
        "std::vector",
        "a :b: c",
        "http://example.com:8080:",
        "::before",
        "x:y:",
        "don't",
        "\"Quoted\"",
        "5'10\"",
        "He said--no.",
        "Then---yes.",
        "Wait...",
        "...and then",
        "----",
        "- - -",
        "* * *",
        "***",
        "-- signed",
        "I. Introduction",
        "IV. Fourth part",
        "iv.\tFourth part",
        "A. Smith said so.",
        "A.\tSmith",
        "A.",
        "e. e. cummings",
        "x. marks the spot",
        "mix. of roman letters",
        "B) plan",
        "(c) third",
        "(1)",
        "1. Numbered",
        "12) also",
        "::: warning",
        ":::",
        ": colon",
        "# not a heading",
        "> not a quote",
        "+ plus",
        "| pipe |",
        "[^1]: not a footnote",
        "[label]: not a link",
        "```",
        "~~~",
        "{=x=}",
        "E=mc2",
        "a\\ b",
        "C:\\Users\\me",
        "snake_case_name",
        "email@example.com",
    ] {
        let djot = plain_text_to_djot_verbatim(line);
        let (text, blocks) = read_back(&djot);
        assert_eq!(text, line, "{line:?} was written as {djot:?}");
        assert_eq!(blocks, 1, "{line:?} was written as {djot:?}");
    }
}

/// A marker after leading spaces or a tab is still a marker to the parser, which skips
/// the indentation before it looks. Trimmed or not, the words survive.
#[test]
fn a_marker_after_leading_whitespace_is_neutralised() {
    for (line, expected) in [
        ("  - item", "- item"),
        ("\t1. step", "1. step"),
        (" # heading", "# heading"),
        ("  > quote", "> quote"),
        (" ---", "---"),
        ("  * * *", "* * *"),
        ("\tIV. part", "IV. part"),
    ] {
        let guarded = guard_djot_line_start(&escape_djot_text(line, EscapeContext::default()));
        assert_eq!(
            read_back(&guarded).0,
            expected,
            "{line:?} became {guarded:?}"
        );
    }
}

/// Text that means nothing to Djot is written as it came, so an ordinary synopsis or
/// comment body is stored byte for byte.
#[test]
fn ordinary_text_is_left_byte_identical() {
    for line in [
        "Just an ordinary remark.",
        "Two sentences. Both ordinary!",
        "Price: $5, 50% off",
        "Mr. Smith and Mrs. Jones",
        "a - b",
        "1 + 1 = 2",
    ] {
        assert_eq!(plain_text_to_djot_verbatim(line), line);
    }
}

/// `x-` in one run and `-y` in the next, joined by nothing, are `--` to the parser. The
/// escaper sees across the boundary because it is told what surrounds each run.
#[test]
fn a_dash_or_an_ellipsis_split_across_two_runs_is_escaped() {
    let plain = DjotInlineStyle::default();
    for pieces in [
        ["x-", "-y"],
        ["Wait.", ".."],
        ["Wait..", "."],
        ["10:3", "0:45"],
        ["std:", ":vector"],
    ] {
        let full = pieces.concat();
        let mut djot = String::new();
        push_djot_run(&mut djot, pieces[0], plain, "", pieces[1]);
        push_djot_run(&mut djot, pieces[1], plain, pieces[0], "");
        let djot = guard_djot_line_start(&djot);
        assert_eq!(read_back(&djot).0, full, "{pieces:?} became {djot:?}");
    }
}

/// A paragraph that is one italic run shaped like `key=value` would otherwise be read as
/// a block attribute line, and the paragraph would vanish.
#[test]
fn an_equals_sign_inside_braces_keeps_its_paragraph() {
    let italic = DjotInlineStyle {
        italic: true,
        ..DjotInlineStyle::default()
    };
    for text in ["E=mc2", "x=5", "a=b c=d", "="] {
        let mut djot = String::new();
        push_djot_run(&mut djot, text, italic, "", "");
        let djot = guard_djot_line_start(&djot);
        assert_eq!(
            read_back(&djot),
            (text.to_string(), 1),
            "{text:?} became {djot:?}"
        );
    }
    assert_eq!(
        escape_djot_text("1 + 1 = 2", EscapeContext::default()),
        "1 + 1 = 2",
        "outside braces an equals sign is left alone"
    );
}

/// A footnote reference that opens a paragraph, followed by a colon, is a footnote
/// definition to the parser: the paragraph would become the note's text.
#[test]
fn a_reference_opening_a_line_before_a_colon_is_not_a_definition() {
    let guarded = guard_djot_line_start("[^fn1]: see above");
    assert_eq!(guarded, "[^fn1]\\: see above");
    let (text, blocks) = read_back(&guarded);
    assert_eq!(blocks, 1);
    assert!(
        text.ends_with(": see above"),
        "the words after the reference stay in the paragraph: {text:?}"
    );
}

/// The content of a list item starts a line of its own inside the item, so it is guarded
/// like one: `- 1. x` would otherwise nest a numbered list and lose its `1.`.
#[test]
fn a_list_items_content_start_is_guarded_like_a_line() {
    for content in ["1. x", "A. Smith", "- dash", "iv.\tpart"] {
        let djot = format!(
            "- {}",
            guard_djot_line_start(&escape_djot_text(content, EscapeContext::default()))
        );
        assert_eq!(read_back(&djot), (content.to_string(), 1), "{djot:?}");
    }
}

/// Styled whitespace at a run's edge is written outside the delimiters, and a run that is
/// only whitespace carries no delimiters at all.
#[test]
fn a_runs_edge_whitespace_is_written_outside_its_delimiters() {
    let bold = DjotInlineStyle {
        bold: true,
        ..DjotInlineStyle::default()
    };
    let mut djot = String::new();
    push_djot_run(&mut djot, " word\t", bold, "a", "b");
    assert_eq!(djot, " {*word*}\t");

    let mut djot = String::new();
    push_djot_run(&mut djot, "  ", bold, "a", "b");
    assert_eq!(djot, "  ");
}

/// An escaper whose output does not spell the text escape by escape is not trusted: every
/// punctuation mark is escaped instead, which still reads back exactly.
#[test]
fn an_unrecognised_upstream_escape_falls_back_to_escaping_every_mark() {
    let text = "a-b 'c' 1:2";
    for unrecognised in ["a-b", "\\a-b 'c' 1:2", "a-b 'c' 1:2 extra"] {
        let out = extend_inline_escapes(unrecognised, text, EscapeContext::default());
        assert_eq!(out, escape_everything(text));
        assert_eq!(read_back(&out).0, text);
    }
}

/// The rules are not applied to what is not there: characters outside the piece are only
/// read, never written.
#[test]
fn the_context_is_read_but_never_written() {
    let context = EscapeContext {
        before: "ab-",
        after: "-cd",
        braced: false,
    };
    assert_eq!(escape_djot_text("x", context), "x");
    assert_eq!(escape_djot_text("-", context), "\\-");
}

/// Characters that open a block, or could be counted as nesting, where a line starts.
const NESTING: &[char] = &[
    '>', '>', '>', ':', ':', ' ', ' ', '\t', '\u{c}', '\r', '\u{a0}', '\u{3000}', '-', '*', '+',
    '[', '^', ']', '1', '.', 'x',
];

proptest! {
    #![proptest_config(ProptestConfig { cases: 400, ..ProptestConfig::default() })]

    /// Plain text is stored as prose a load always accepts, however it opens its lines:
    /// the verbatim writer is the fallback every importer uses when markup nests too
    /// deep, so it must never be refused itself.
    #[test]
    fn plain_text_is_always_within_the_depth_a_load_accepts(
        lines in prop::collection::vec(
            prop::collection::vec(prop::sample::select(NESTING), 0..400)
                .prop_map(|chars| chars.into_iter().collect::<String>()),
            1..4,
        ),
    ) {
        let text = lines.join("\n");
        let djot = plain_text_to_djot_verbatim(&text);
        prop_assert!(
            crate::djot_depth::check(&djot).is_ok(),
            "{:?} for {:?}",
            crate::djot_depth::check(&djot),
            text
        );
    }
}
