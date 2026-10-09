// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The reader's contract, stated over every input rather than over fixtures:
//!
//! * it never panics and always ends, on any bytes at all and on RTF-shaped noise;
//! * it never passes its limits, its work is bounded by the input's length, and the heap its
//!   model holds is bounded by what its reading accounted;
//! * what three real writers write, it reads back, text and formatting;
//! * spellings RTF says are the same read the same.
//!
//! `prop_assert_eq!` is given its values as arguments, never as inline captures in the
//! message, which the macro does not support.

mod common;
mod writers;

use std::sync::mpsc;
use std::time::{Duration, Instant};

use proptest::prelude::*;
use rtf_reader::{
    Diagnostic, Document, Inline, Limits, Options, Refusal, VerticalPosition, paragraphs, parse,
    parse_with,
};
use writers::Style;

/// Limits low enough for generated input to reach every refusal.
fn small_limits() -> Options {
    Options {
        limits: Limits {
            max_depth: 48,
            max_document_bytes: 64 << 10,
            max_binary_bytes: 256,
            max_picture_bytes: 256,
        },
        ..Options::default()
    }
}

/// What every reading must satisfy, whatever it read.
fn check_bounds(input: &[u8], result: &Result<Document, Refusal>, limits: &Limits) {
    if let Ok(doc) = result {
        assert!(
            doc.stats.tokens <= input.len(),
            "{} tokens from {} bytes",
            doc.stats.tokens,
            input.len()
        );
        assert!(doc.stats.max_depth <= limits.max_depth);
        assert!(doc.stats.document_bytes <= limits.max_document_bytes);
        assert!(
            doc.pictures
                .iter()
                .all(|p| p.data.len() <= limits.max_picture_bytes)
        );
        assert!(doc.stats.picture_bytes <= input.len());
        if let Err(message) = common::heap_within_accounting(doc) {
            panic!("{message}");
        }
    }
}

// ── any input ────────────────────────────────────────────────────────────────

proptest! {
    #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]

    #[test]
    fn arbitrary_bytes_never_panic(bytes in proptest::collection::vec(any::<u8>(), 0..2048)) {
        let result = parse(&bytes);
        check_bounds(&bytes, &result, &Limits::default());
        let mut prefixed = common::rtf(r"{\rtf1 ");
        prefixed.extend_from_slice(&bytes);
        let result = parse_with(&prefixed, &small_limits());
        check_bounds(&prefixed, &result, &small_limits().limits);
    }
}

/// One piece of RTF-shaped noise.
fn piece() -> impl Strategy<Value = String> {
    let words = prop::sample::select(vec![
        "par",
        "pard",
        "plain",
        "b",
        "b0",
        "i",
        "ul",
        "ulnone",
        "super",
        "sub",
        "up3",
        "dn3",
        "f0",
        "f2",
        "fs24",
        "cf1",
        "cb2",
        "v",
        "tab",
        "line",
        "page",
        "cell",
        "nestcell",
        "row",
        "nestrow",
        "intbl",
        "itap2",
        "itap40",
        "ls1",
        "ilvl3",
        "qc",
        "rtlpar",
        "s1",
        "outlinelevel0",
        "chftn",
        "chatn",
        "sect",
        "ansicpg932",
        "ansicpg1251",
        "mac",
        "fcharset128",
        "fcharset2",
        "cpg1250",
        "deff0",
        "zzunknown12",
    ]);
    let destinations = prop::sample::select(vec![
        "fonttbl",
        "colortbl",
        "stylesheet",
        "info",
        "title",
        "field",
        "fldinst",
        "fldrslt",
        "pict",
        "shppict",
        "nonshppict",
        "footnote",
        "annotation",
        "atnid",
        "atrfstart",
        "listtable",
        "list",
        "listlevel",
        "listoverridetable",
        "listoverride",
        "listtext",
        "pn",
        "upr",
        "ud",
        "shp",
        "shpinst",
        "sp",
        "sn",
        "sv",
        "object",
        "result",
        "nesttableprops",
        "header",
        "NeXTGraphic",
        "nisusfilename",
        "levelmarker",
        "madeup",
    ]);
    prop_oneof![
        4 => Just("{".to_string()),
        4 => Just("}".to_string()),
        2 => Just("{\\*".to_string()),
        4 => words.prop_map(|w| format!("\\{w} ")),
        3 => destinations.prop_map(|w| format!("{{\\{w} ")),
        2 => any::<u8>().prop_map(|b| format!("\\'{b:02x}")),
        2 => any::<i32>().prop_map(|n| format!("\\u{n}?")),
        1 => (0u32..5).prop_map(|n| format!("\\uc{n} ")),
        1 => (0i64..5000).prop_map(|n| format!("\\bin{n} ab")),
        1 => Just("\\\n".to_string()),
        1 => Just("89504e47".to_string()),
        4 => "[a-zA-Z ;.\\\\{}]{0,12}",
    ]
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]

    #[test]
    fn rtf_shaped_noise_never_panics(pieces in proptest::collection::vec(piece(), 0..120)) {
        let mut input = String::from("{\\rtf1\\ansi ");
        for piece in &pieces {
            input.push_str(piece);
        }
        let bytes = input.into_bytes();
        let result = parse(&bytes);
        check_bounds(&bytes, &result, &Limits::default());
        let result = parse_with(&bytes, &small_limits());
        check_bounds(&bytes, &result, &small_limits().limits);
    }
}

// ── writers read back ────────────────────────────────────────────────────────

fn style() -> impl Strategy<Value = Style> {
    (
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        0u8..3,
    )
        .prop_map(|(bold, italic, underline, strike, vertical)| Style {
            bold,
            italic,
            underline,
            strike,
            vertical,
        })
}

/// Text that stays text through any RTF writer: letters, the characters RTF reserves, a
/// tab, code page 1252's own characters, and characters outside it, astral ones included.
fn text() -> impl Strategy<Value = String> {
    let chars = prop::sample::select(vec![
        'a',
        'b',
        'z',
        'A',
        'Q',
        '0',
        '9',
        ' ',
        ' ',
        '.',
        ',',
        ';',
        '?',
        '\\',
        '{',
        '}',
        '\t',
        '*',
        '_',
        '\'',
        '"',
        '\u{A0}',
        '\u{E9}',
        '\u{2019}',
        '\u{201C}',
        '\u{2014}',
        '\u{20AC}',
        '\u{3C6}',
        '\u{416}',
        '\u{5D0}',
        '\u{4E2D}',
        '\u{202F}',
        '\u{2028}',
        '\u{FFFC}',
        '\u{1F600}',
        '\u{10348}',
        '\u{301}',
    ]);
    proptest::collection::vec(chars, 1..24).prop_map(|c| c.into_iter().collect())
}

fn model() -> impl Strategy<Value = Vec<writers::Paragraph>> {
    let run = (text(), style());
    let paragraph = proptest::collection::vec(run, 1..5);
    proptest::collection::vec(paragraph, 1..6)
}

/// Each paragraph as its characters and their styles, whatever the run boundaries.
fn read_back(doc: &Document) -> Vec<Vec<(char, Style)>> {
    paragraphs(&doc.body)
        .map(|paragraph| {
            let mut out = Vec::new();
            for inline in &paragraph.inlines {
                if let Inline::Text(run) = inline {
                    let style = Style {
                        bold: run.props.bold,
                        italic: run.props.italic,
                        underline: run.props.underline,
                        strike: run.props.strike,
                        vertical: match run.props.vertical {
                            VerticalPosition::Baseline => 0,
                            VerticalPosition::Superscript => 1,
                            VerticalPosition::Subscript => 2,
                        },
                    };
                    out.extend(run.text.chars().map(|c| (c, style)));
                }
            }
            out
        })
        .collect()
}

fn expected(model: &[writers::Paragraph]) -> Vec<Vec<(char, Style)>> {
    model
        .iter()
        .map(|paragraph| {
            paragraph
                .iter()
                .flat_map(|(text, style)| text.chars().map(move |c| (c, *style)))
                .collect()
        })
        .collect()
}

fn read(bytes: &[u8]) -> Result<Vec<Vec<(char, Style)>>, TestCaseError> {
    let doc = parse(bytes).map_err(|refusal| TestCaseError::fail(refusal.to_string()))?;
    prop_assert!(doc.diagnostics.is_empty(), "{:?}", doc.diagnostics);
    Ok(read_back(&doc))
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 128, ..ProptestConfig::default() })]

    #[test]
    fn what_the_cocoa_writer_writes_reads_back(model in model()) {
        let got = read(&writers::cocoa(&model))?;
        prop_assert_eq!(got, expected(&model));
    }

    #[test]
    fn what_the_windows_writer_writes_reads_back(model in model()) {
        let got = read(&writers::windows(&model))?;
        prop_assert_eq!(got, expected(&model));
    }

    #[test]
    fn what_a_word_processor_writes_reads_back(model in model(), wrap in 0usize..40) {
        let got = read(&writers::word_processor(&model, wrap))?;
        prop_assert_eq!(got, expected(&model));
    }
}

// ── the same thing spelled differently ───────────────────────────────────────

/// The body's text, paragraphs joined by a line feed.
fn body_text(bytes: &[u8]) -> String {
    match parse(bytes) {
        Ok(doc) => common::body(&doc).join("\n"),
        Err(refusal) => format!("refused: {refusal}"),
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 256, ..ProptestConfig::default() })]

    /// A code page escape, a Unicode escape with a fallback, and a Unicode escape without
    /// one are the same character.
    #[test]
    fn three_spellings_of_one_character(before in "[a-z ]{0,8}", after in "[a-z ]{0,8}") {
        let hex = common::rtf(&format!("{{\\rtf1\\ansi {before}\\'e9{after}}}"));
        let fallback = common::rtf(&format!("{{\\rtf1\\ansi\\uc1 {before}@u233?{after}}}"));
        let bare = common::rtf(&format!("{{\\rtf1\\ansi {before}@uc0@u233 {after}}}"));
        let want = body_text(&hex);
        prop_assert_eq!(body_text(&fallback), want.clone());
        prop_assert_eq!(body_text(&bare), want);
    }

    /// Empty groups and unknown control words between runs change nothing.
    #[test]
    fn empty_groups_and_unknown_words_change_nothing(
        model in model(),
        noise in proptest::collection::vec(prop::sample::select(vec!["{}", "\\zzfoo123 ", "{\\zzbar}", "{\\*\\zzbaz x}"]), 1..8),
    ) {
        let plain = writers::word_processor(&model, 0);
        let text = String::from_utf8_lossy(&plain).into_owned();
        // Insert the noise at run boundaries, where the Word writer closes a group.
        let mut noisy = String::new();
        let mut inserted = 0usize;
        for (index, piece) in text.split("}{").enumerate() {
            if index > 0 {
                noisy.push('}');
                noisy.push_str(noise[inserted % noise.len()]);
                inserted += 1;
                noisy.push('{');
            }
            noisy.push_str(piece);
        }
        let got = parse(noisy.as_bytes()).map(|d| read_back(&d)).map_err(|r| TestCaseError::fail(r.to_string()))?;
        prop_assert_eq!(got, expected(&model));
    }

    /// A run split into two groups with the same formatting reads as the same text and
    /// formatting, in two runs.
    #[test]
    fn a_run_split_across_groups_reads_the_same(left in text(), right in text(), style in style()) {
        let whole = vec![vec![(format!("{left}{right}"), style)]];
        let split = vec![vec![(left, style), (right, style)]];
        let a = read(&writers::word_processor(&whole, 0))?;
        let b = read(&writers::word_processor(&split, 0))?;
        prop_assert_eq!(a, b);
    }
}

// ── limits ───────────────────────────────────────────────────────────────────

/// Run `f` on a thread with a small stack: a reader that recursed on the input's nesting
/// would overflow it and abort the process, which no test can catch.
fn on_small_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    match std::thread::Builder::new()
        .stack_size(256 << 10)
        .spawn(f)
        .map(|handle| handle.join())
    {
        Ok(Ok(value)) => value,
        _ => panic!("the reading thread failed"),
    }
}

#[test]
fn a_hundred_thousand_nested_groups_are_refused_without_recursing() {
    let result = on_small_stack(|| {
        let mut input = b"{\\rtf1 ".to_vec();
        input.extend(std::iter::repeat_n(b'{', 100_000));
        parse(&input)
    });
    assert_eq!(result.err(), Some(Refusal::NestedTooDeep { limit: 1_024 }));
}

#[test]
fn nesting_inside_a_skipped_group_counts_too() {
    let result = on_small_stack(|| {
        let mut input = b"{\\rtf1 {\\*\\madeup ".to_vec();
        input.extend(std::iter::repeat_n(b'{', 100_000));
        parse(&input)
    });
    assert!(matches!(result, Err(Refusal::NestedTooDeep { .. })));
}

#[test]
fn nesting_below_the_limit_is_read() {
    let result = on_small_stack(|| {
        let mut input = b"{\\rtf1 ".to_vec();
        input.extend(std::iter::repeat_n(b'{', 1_000));
        input.extend_from_slice(b"deep");
        input.extend(std::iter::repeat_n(b'}', 1_001));
        parse(&input)
    });
    let doc = match result {
        Ok(doc) => doc,
        Err(refusal) => panic!("{refusal}"),
    };
    assert_eq!(common::body(&doc), ["deep"]);
}

#[test]
fn a_paragraph_bomb_is_refused_within_its_limit() {
    let options = Options {
        limits: Limits {
            max_document_bytes: 1 << 20,
            ..Limits::default()
        },
        ..Options::default()
    };
    // Two bytes per paragraph, macOS's own mark: the shape that amplifies most.
    let mut input = b"{\\rtf1 ".to_vec();
    for _ in 0..200_000 {
        input.extend_from_slice(b"\\\n");
    }
    input.push(b'}');
    assert_eq!(
        parse_with(&input, &options).err(),
        Some(Refusal::DocumentTooLarge { limit: 1 << 20 })
    );
}

/// A paragraph deep in nested tables, then one in the main text: each pair opens and closes
/// every table level, which allocates tens of kilobytes from about twenty bytes of input.
/// Counted one block per paragraph, 7.8 MB of this was read whole into a 6.7 GB model under
/// the default limits.
#[test]
fn table_levels_a_paragraph_opens_are_accounted_as_the_memory_they_take() {
    let mut input = b"{\\rtf1 ".to_vec();
    for _ in 0..2_000 {
        input.extend_from_slice(b"\\itap32 x\\par\\itap0 y\\par ");
    }
    input.push(b'}');
    let doc = match parse(&input) {
        Ok(doc) => doc,
        Err(refusal) => panic!("{refusal}"),
    };
    if let Err(message) = common::heap_within_accounting(&doc) {
        panic!("{message}");
    }

    // The same shape a hundred times longer passes the default limit, and is refused.
    let mut input = b"{\\rtf1 ".to_vec();
    for _ in 0..200_000 {
        input.extend_from_slice(b"\\itap32 x\\par\\itap0 y\\par ");
    }
    input.push(b'}');
    assert_eq!(
        parse(&input).err(),
        Some(Refusal::DocumentTooLarge { limit: 128 << 20 })
    );
}

/// How long reading `input` took, on a thread of its own, or `None` when it had not
/// finished after `budget`. A reading that runs over is left to finish on its thread: there
/// is no way to stop it from outside, and it ends with the test binary.
fn read_within(input: Vec<u8>, budget: Duration) -> Option<Duration> {
    let (sender, receiver) = mpsc::channel();
    let started = Instant::now();
    std::thread::Builder::new()
        .spawn(move || {
            let result = parse(&input);
            // The receiver is gone only once the test has already failed.
            let _ = sender.send(result.is_ok());
        })
        .ok()?;
    receiver
        .recv_timeout(budget)
        .ok()
        .map(|_| started.elapsed())
}

/// Every skipped destination the reading counts by name used to walk every count kept so
/// far, and a file stating a different unsupported code page for each of its bytes keeps
/// one count per page, up to 65,536: 7 MB of such input took minutes. Reading the two
/// halves together must cost about what reading each alone does.
#[test]
fn counting_a_skipped_destination_does_not_walk_every_other_count() {
    let pages: String = (0..60_000u32)
        .map(|page| format!("\\ansicpg{} \\'80", 2_000 + page))
        .collect();
    let destinations = "{\\*\\madeup}".repeat(20_000);
    let document = |body: &str| format!("{{\\rtf1\\ansi {body}}}").into_bytes();
    let patience = Duration::from_secs(120);
    let alone = [document(&pages), document(&destinations)]
        .into_iter()
        .map(|input| read_within(input, patience))
        .sum::<Option<Duration>>();
    let Some(alone) = alone else {
        panic!("reading either half alone took more than {patience:?}");
    };
    let budget = alone * 10 + Duration::from_millis(500);
    let together = read_within(document(&format!("{pages}{destinations}")), budget);
    assert!(
        together.is_some(),
        "the halves read in {alone:?} alone, and together not within {budget:?}"
    );
}

#[test]
fn a_real_length_of_blank_paragraphs_is_read() {
    let mut input = b"{\\rtf1 ".to_vec();
    for _ in 0..10_000 {
        input.extend_from_slice(b"\\\n");
    }
    input.extend_from_slice(b"end}");
    let doc = match parse(&input) {
        Ok(doc) => doc,
        Err(refusal) => panic!("{refusal}"),
    };
    assert_eq!(paragraphs(&doc.body).count(), 10_001);
}

/// The text of every paragraph of `doc`, joined.
fn all_text(doc: &Document) -> String {
    paragraphs(&doc.body)
        .flat_map(|p| p.runs())
        .map(|r| r.text.as_str())
        .collect()
}

/// An importer must not lose a chapter's words over one photograph: a picture whose `\bin`
/// declares more than the limit is dropped, the text around it kept.
#[test]
fn a_binary_length_past_the_limit_drops_the_picture_and_keeps_the_text() {
    let mut input = b"{\\rtf1 Before {\\pict\\pngblip\\bin300 ".to_vec();
    input.extend(std::iter::repeat_n(0xABu8, 300));
    input.extend_from_slice(b"} after.}");
    let doc = match parse_with(&input, &small_limits()) {
        Ok(doc) => doc,
        Err(refusal) => panic!("the text is read, not refused: {refusal}"),
    };
    assert_eq!(doc.pictures.len(), 1);
    assert!(doc.pictures[0].oversized);
    assert!(doc.pictures[0].data.is_empty());
    assert_eq!(doc.diagnostics.count(&Diagnostic::PictureTooLarge), 1);
    assert_eq!(all_text(&doc), "Before  after.");
}

#[test]
fn a_picture_past_the_limit_is_dropped_and_the_text_around_it_kept() {
    let options = small_limits();
    let mut input = b"{\\rtf1 Before {\\pict\\pngblip ".to_vec();
    input.extend(std::iter::repeat_n(b'a', 2_000));
    input.extend_from_slice(b"} after {\\pict\\pngblip 0102} end.}");
    let doc = match parse_with(&input, &options) {
        Ok(doc) => doc,
        Err(refusal) => panic!("the text is read, not refused: {refusal}"),
    };
    assert_eq!(doc.pictures.len(), 2);
    assert!(doc.pictures[0].oversized);
    assert!(doc.pictures[0].data.is_empty());
    assert!(
        !doc.pictures[1].oversized,
        "a later picture is read as usual"
    );
    assert_eq!(doc.pictures[1].data, [1, 2]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::PictureTooLarge), 1);
    assert_eq!(doc.stats.picture_bytes, 2);
    assert_eq!(all_text(&doc), "Before  after  end.");
}

// ── the mutation sweep ───────────────────────────────────────────────────────

/// Every prefix cut and a few hundred mutations of each fixture, from a fixed seed: none may
/// panic or pass a limit. Deterministic, unlike the properties above, so a failure here
/// replays exactly.
#[test]
fn prefixes_and_mutations_of_the_fixtures_never_panic() {
    let model = vec![
        vec![
            ("The ferry {was} late\\.".to_string(), Style::default()),
            (
                "caf\u{E9} \u{1F600}\t\u{2019}".to_string(),
                Style {
                    bold: true,
                    vertical: 1,
                    ..Style::default()
                },
            ),
        ],
        vec![("Second paragraph.".to_string(), Style::default())],
    ];
    let mut fixtures = vec![
        writers::cocoa(&model),
        writers::windows(&model),
        writers::word_processor(&model, 7),
        common::rtf(
            r#"{\rtf1{\fonttbl{\f0\fcharset128 Sans;}}{\colortbl;\red1\green2\blue3;}{\stylesheet{\s1 heading 1;}}{\*\listtable{\list{\listlevel\levelnfc23}\listid1}}{\*\listoverridetable{\listoverride\listid1\ls1}}\pard\ls1{\listtext\'95\tab}A{\field{\*\fldinst HYPERLINK "https://example.org"}{\fldrslt link}}{\footnote note\par two}{\*\atnid X}\chatn{\*\annotation{\*\atnref 1}c}{\*\shppict{\pict\pngblip 8950}}\trowd\pard\intbl a\cell b\cell\row\pard\intbl\itap2 n\nestcell{\*\nesttableprops\nestrow}\pard\intbl\cell\row{\upr{x}{\*\ud{y}}}@u-10179?@u-8704?\'81\'cb}"#,
        ),
    ];
    fixtures.push(b"{\\rtf1 {\\pict\\bin4 {}\\x}}\r\n\0".to_vec());

    let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut random = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let significant = b"{}\\'*0123456789-u ";
    let options = small_limits();
    let mut runs = 0usize;
    for fixture in &fixtures {
        let mut cases: Vec<Vec<u8>> = (0..fixture.len())
            .map(|cut| fixture[..cut].to_vec())
            .collect();
        for _ in 0..200 {
            let mut mutated = fixture.clone();
            for _ in 0..(1 + random() % 4) {
                if mutated.is_empty() {
                    break;
                }
                let at = (random() as usize) % mutated.len();
                match random() % 5 {
                    0 => mutated[at] = significant[(random() as usize) % significant.len()],
                    1 => {
                        mutated.remove(at);
                    }
                    2 => {
                        let deep = vec![b'{'; 2_000];
                        mutated.splice(at..at, deep);
                    }
                    3 => {
                        let noise = common::rtf(r"@u-99999999999@uc99\bin999 ");
                        mutated.splice(at..at, noise);
                    }
                    _ => mutated[at] = (random() & 0xFF) as u8,
                }
            }
            cases.push(mutated);
        }
        for case in cases {
            runs += 1;
            let result = parse(&case);
            check_bounds(&case, &result, &Limits::default());
            let result = parse_with(&case, &options);
            check_bounds(&case, &result, &options.limits);
        }
    }
    assert_eq!(runs, fixtures.iter().map(|f| f.len() + 200).sum::<usize>());
}
