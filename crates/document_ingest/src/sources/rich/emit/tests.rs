// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The emitter against the parser the editor reads prose with.
//!
//! Every assertion about what a paragraph reads back as goes through `text-document`
//! itself (`skrib_format::read_djot`, or a `TextDocument` where a block's format matters),
//! never through a reading of the Djot string, which would share the emitter's blind spots.

use proptest::prelude::*;
use skribisto_model::scene_break;
use text_document::{
    Alignment as TdAlignment, CharVerticalAlignment, FragmentContent, TextDocument,
};

use skrib_format::read_djot;

use super::*;
use crate::sources::rich::RichBlock;

/// Prove with the parser the editor uses.
fn prove(sources: &[Source<'_>], frame: Frame) -> anyhow::Result<Proven> {
    prove_with(sources, frame, &read_djot)
}

fn plain_run(text: &str) -> Run {
    Run::plain(text)
}

fn styled(text: &str, style: RunStyle) -> Run {
    Run::styled(text, style)
}

fn bold() -> RunStyle {
    RunStyle {
        bold: true,
        ..RunStyle::default()
    }
}

fn paragraph<'a>(runs: &'a [Run], props: BlockProps) -> Source<'a> {
    Source::Paragraph {
        kind: ParagraphKind::Body,
        runs,
        props,
    }
}

fn centred() -> BlockProps {
    BlockProps {
        alignment: Some(Alignment::Center),
        ..BlockProps::default()
    }
}

/// The document `text-document` builds from `djot`, as the editor would open it.
fn open(djot: &str) -> TextDocument {
    let doc = TextDocument::new();
    let parsed = doc.set_djot(djot).and_then(|op| {
        op.wait_timeout(std::time::Duration::from_secs(30))
            .expect("the text-document operation did not finish within 30 s")
    });
    assert!(parsed.is_ok(), "{djot:?} did not parse: {parsed:?}");
    doc
}

/// The characters of `text` at `range`.
fn slice(text: &str, range: Range<usize>) -> String {
    text.chars()
        .skip(range.start)
        .take(range.end - range.start)
        .collect()
}

// ── the defect ──────────────────────────────────────────────────────────────────────

/// The shape that used to break every comment after it: a double space and a tab early in
/// the run. The text is kept as written, and every later member is found exactly where the
/// proof says, which is what a comment offset is rebased through.
#[test]
fn a_double_space_and_a_tab_neither_change_the_text_nor_move_what_follows() {
    let first = [plain_run("One.  Two.\tThree.")];
    let second = [plain_run("\tIndented, with a tab.")];
    let third = [plain_run("She turned the corner.")];
    let sources = [
        paragraph(&first, BlockProps::default()),
        paragraph(&second, BlockProps::default()),
        paragraph(&third, BlockProps::default()),
    ];
    let proven = prove(&sources, Frame::Blocks).expect("prove");

    assert_eq!(
        proven.text,
        "One.  Two.\tThree.\nIndented, with a tab.\nShe turned the corner."
    );
    let reread = read_djot(&proven.djot).expect("parse").text;
    assert_eq!(reread, proven.text, "the stored Djot reads back as proved");
    assert!(proven.members.iter().all(|m| m.exact && !m.reported));

    // The leading tab of the second paragraph is not written, a paragraph's indent being
    // layout, and the map says so: its character 1 is the stored character 0 of that
    // paragraph.
    let second = &proven.members[1].segments[0];
    assert_eq!(second.lead, 1);
    assert_eq!(
        slice(
            &proven.text,
            second.map(1).expect("in range")..second.map(9).expect("in range")
        ),
        "Indented"
    );
    let third = &proven.members[2].segments[0];
    assert_eq!(
        slice(
            &proven.text,
            third.map(4).expect("in range")..third.map(10).expect("in range")
        ),
        "turned"
    );
}

/// What the parser used to rewrite on the first load, one of each kind, stored verbatim.
#[test]
fn text_the_parser_would_rewrite_is_stored_as_written() {
    let lines = [
        "He said \"no\" and 'yes'.",
        "Wait--no. Then---yes...",
        "At 10:30:45, std::vector and a :b: c.",
        "I. Introduction",
        "A. Smith said so.",
        "(c) third",
        "E=mc2 and x = 5",
        "- not a list",
        "# not a heading",
    ];
    let runs: Vec<[Run; 1]> = lines.iter().map(|l| [plain_run(l)]).collect();
    let sources: Vec<Source<'_>> = runs
        .iter()
        .map(|r| paragraph(r, BlockProps::default()))
        .collect();
    let proven = prove(&sources, Frame::Blocks).expect("prove");
    assert_eq!(proven.text, lines.join("\n"));
    assert_eq!(
        read_djot(&proven.djot).expect("parse").text,
        lines.join("\n")
    );
}

// ── paragraph formatting ────────────────────────────────────────────────────────────

/// Centred and flush-right paragraphs keep their alignment, and the editor reads it.
#[test]
fn centred_and_right_aligned_paragraphs_keep_their_alignment() {
    let a = [plain_run("A centred line.")];
    let b = [plain_run("A line set right.")];
    let c = [plain_run("An ordinary line.")];
    let right = BlockProps {
        alignment: Some(Alignment::Right),
        ..BlockProps::default()
    };
    let sources = [
        paragraph(&a, centred()),
        paragraph(&b, right),
        paragraph(&c, BlockProps::default()),
    ];
    let proven = prove(&sources, Frame::Blocks).expect("prove");
    let doc = open(&proven.djot);
    let alignments: Vec<Option<TdAlignment>> = doc
        .blocks()
        .iter()
        .map(|b| b.block_format().alignment)
        .collect();
    assert_eq!(
        alignments,
        vec![Some(TdAlignment::Center), Some(TdAlignment::Right), None],
        "from {:?}",
        proven.djot
    );
}

/// Left in left-to-right text is where a paragraph starts anyway, and justification is
/// the export style's decision: neither is written. In right-to-left text the edges swap.
#[test]
fn only_an_alignment_away_from_the_start_edge_is_written() {
    let props = |alignment, direction| BlockProps {
        alignment: Some(alignment),
        direction,
        page_break_before: false,
    };
    let rtl = Some(Direction::RightToLeft);
    for (props, expected) in [
        (props(Alignment::Left, None), None),
        (props(Alignment::Justify, None), None),
        (props(Alignment::Left, Some(Direction::LeftToRight)), None),
        (props(Alignment::Right, rtl), Some("{direction=rtl}")),
        (
            props(Alignment::Left, rtl),
            Some("{alignment=left direction=rtl}"),
        ),
        (
            props(Alignment::Center, rtl),
            Some("{alignment=center direction=rtl}"),
        ),
    ] {
        assert_eq!(
            attribute_line(ParagraphKind::Body, props, "Text.", Frame::Blocks).as_deref(),
            expected,
            "{props:?}"
        );
    }
}

/// A page break before a paragraph reaches the editor; a list item and an epigraph carry
/// no block attribute at all.
#[test]
fn a_page_break_is_kept_on_a_paragraph() {
    let a = [plain_run("Before.")];
    let b = [plain_run("On a new page.")];
    let page = BlockProps {
        page_break_before: true,
        ..BlockProps::default()
    };
    let sources = [paragraph(&a, BlockProps::default()), paragraph(&b, page)];
    let proven = prove(&sources, Frame::Blocks).expect("prove");
    let doc = open(&proven.djot);
    let breaks: Vec<Option<bool>> = doc
        .blocks()
        .iter()
        .map(|b| b.block_format().page_break_before)
        .collect();
    assert_eq!(breaks, vec![None, Some(true)], "from {:?}", proven.djot);

    let item = ParagraphKind::ListItem {
        ordered: false,
        depth: 0,
    };
    assert_eq!(attribute_line(item, page, "An item.", Frame::Blocks), None);
    assert_eq!(
        attribute_line(
            ParagraphKind::Epigraph,
            page,
            "A quotation.",
            Frame::Epigraph
        ),
        None
    );
}

/// The break vocabulary matches the bare line, so an attribute in front of one would hide
/// the break. Whatever the paragraph says about itself, a break line never carries one.
#[test]
fn a_scene_break_line_never_carries_an_attribute_line() {
    for glyph in ["* * *", "# # #", "***"] {
        let runs = [plain_run(glyph)];
        let props = BlockProps {
            alignment: Some(Alignment::Center),
            direction: Some(Direction::RightToLeft),
            page_break_before: true,
        };
        let proven = prove(&[paragraph(&runs, props)], Frame::Blocks).expect("prove");
        assert!(
            !proven.djot.contains('{'),
            "{glyph:?} was written as {:?}",
            proven.djot
        );
        assert!(
            scene_break::tier_of_djot_block(&proven.djot).is_some()
                || scene_break::tier_of_plain_line(glyph).is_none(),
            "{glyph:?} no longer reads as a break: {:?}",
            proven.djot
        );
    }
}

// ── character formatting ────────────────────────────────────────────────────────────

/// Raised and lowered characters keep their position.
#[test]
fn superscript_and_subscript_are_kept() {
    let sup = RunStyle {
        superscript: true,
        ..RunStyle::default()
    };
    let sub = RunStyle {
        subscript: true,
        ..RunStyle::default()
    };
    let runs = [
        plain_run("E=mc"),
        styled("2", sup),
        plain_run(" and H"),
        styled("2", sub),
        plain_run("O"),
    ];
    let proven = prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
    assert_eq!(proven.text, "E=mc2 and H2O");
    let doc = open(&proven.djot);
    let positions: Vec<(String, Option<CharVerticalAlignment>)> = doc.blocks()[0]
        .fragments()
        .into_iter()
        .filter_map(|f| match f {
            FragmentContent::Text { text, format, .. } => Some((text, format.vertical_alignment)),
            _ => None,
        })
        .filter(|(_, v)| v.is_some())
        .collect();
    assert_eq!(
        positions,
        vec![
            ("2".to_string(), Some(CharVerticalAlignment::SuperScript)),
            ("2".to_string(), Some(CharVerticalAlignment::SubScript)),
        ],
        "from {:?}",
        proven.djot
    );
}

/// A footnote reference is a node of its own: it never sits between a styled run's
/// delimiters, so it is read as a reference and not as text.
#[test]
fn a_footnote_reference_is_never_inside_delimiters() {
    let runs = [
        styled("See", bold()),
        Run {
            footnote: Some("srcfn-1".into()),
            ..Run::default()
        },
        styled(" here", bold()),
    ];
    let proven = prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
    assert_eq!(proven.djot, "{*See*}[^srcfn-1] {*here*}");
    assert_eq!(proven.text, "See\u{FFFC} here");
}

/// A link keeps its target byte for byte: the characters the parser would otherwise read
/// as the end of the destination, or keep as a stray backslash, are percent-encoded.
#[test]
fn a_link_target_reads_back_exactly() {
    for (url, stored) in [
        (
            r"https://example.com/Mac OS (Lion)\x<y>",
            "https://example.com/Mac%20OS%20%28Lion%29%5Cx%3Cy%3E",
        ),
        // A backtick would open a verbatim span that swallows the rest of the paragraph.
        (
            "https://example.com/`code`{a|b}^\"q\"",
            "https://example.com/%60code%60%7Ba%7Cb%7D%5E%22q%22",
        ),
    ] {
        let runs = [
            plain_run("Read "),
            Run::linked("the ", RunStyle::default(), url),
            Run::linked("notes", bold(), url),
            plain_run("."),
        ];
        let proven =
            prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
        assert_eq!(proven.text, "Read the notes.");
        assert!(
            proven.members[0].exact && !proven.members[0].reported,
            "{url}"
        );
        let reading = read_djot(&proven.djot).expect("parse");
        assert_eq!(
            reading.blocks[0].links,
            vec![stored.to_string()],
            "from {:?}",
            proven.djot
        );
    }
}

/// A picture keeps the size it is shown at, written the way `text-document` writes it back.
#[test]
fn a_picture_keeps_its_display_size() {
    let runs = [
        plain_run("See "),
        Run::sized_image("a map", "media/image 1.png", 320, 200),
        plain_run(" and "),
        Run::image("", "media/unsized.png"),
    ];
    let proven = prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
    assert_eq!(
        proven.djot,
        "See ![a map](media/image%201.png){width=320 height=200} and ![](media/unsized.png)"
    );
    assert_eq!(proven.text, "See \u{FFFC} and \u{FFFC}");
    let doc = open(&proven.djot);
    let sizes: Vec<(u32, u32)> = doc.blocks()[0]
        .fragments()
        .into_iter()
        .filter_map(|f| match f {
            FragmentContent::Image { width, height, .. } => Some((width, height)),
            _ => None,
        })
        .collect();
    assert_eq!(sizes, vec![(320, 200), (0, 0)]);
}

// ── blocks ──────────────────────────────────────────────────────────────────────────

/// How many cells each row of a stored pipe table is written with. Every `|` the emitter
/// writes inside a cell is behind a backslash, so a cell boundary is the only ` | ` there is.
fn written_row_lengths(djot: &str) -> Vec<usize> {
    djot.lines()
        .filter(|line| line.starts_with("| "))
        .map(|line| line.matches(" | ").count() + 1)
        .collect()
}

/// A table is proved cell by cell, like any paragraph, and written with the cells the file
/// holds: the parser completes the short first row itself, and the editor's first save
/// keeps every cell.
#[test]
fn a_table_is_proved_cell_by_cell() {
    let rows = vec![
        vec![vec![plain_run("a")], vec![plain_run("b | pipe")]],
        vec![
            vec![plain_run("c")],
            vec![styled("d", bold())],
            vec![plain_run("e")],
        ],
    ];
    let proven = prove(&[Source::Table { rows: &rows }], Frame::Blocks).expect("prove");
    assert!(proven.members[0].exact);
    assert_eq!(
        written_row_lengths(&proven.djot),
        vec![2, 3],
        "{:?}",
        proven.djot
    );
    let segments = &proven.members[0].segments;
    assert_eq!(segments.len(), 5, "one per real cell, none for the padding");
    // "a\nb | pipe\nc\nd\ne" is the table's own plain text.
    let cells: Vec<String> = segments
        .iter()
        .map(|s| slice(&proven.text, s.stored()))
        .collect();
    assert_eq!(cells, vec!["a", "b | pipe", "c", "d", "e"]);
    assert_eq!(segments[3].source_start, 13);

    let doc = open(&proven.djot);
    let resaved = doc.to_djot().expect("export");
    for cell in ["a", "b \\| pipe", "c", "*d*", "e"] {
        assert!(resaved.contains(cell), "{cell:?} lost on save: {resaved:?}");
    }
}

/// A table of short rows with one wide row is written as the file holds it, no row widened.
/// `text-document` sizes a table from its widest row since 1.12.3, so every real cell is
/// proved, the short rows completed by the parser, and every cell survives the editor's
/// first save. With 1.12.2, which sized a table from its first row, this emitter widened
/// that row to the widest, or the first save dropped every cell past it.
#[test]
fn a_ragged_table_is_written_as_the_file_holds_it() {
    let rows = vec![
        vec![vec![plain_run("a")]],
        vec![vec![plain_run("b")]],
        vec![vec![plain_run("c")]],
        vec![
            vec![plain_run("d")],
            vec![styled("e", bold())],
            vec![plain_run("f")],
            vec![plain_run("g")],
            vec![plain_run("h")],
        ],
    ];
    let proven = prove(&[Source::Table { rows: &rows }], Frame::Blocks).expect("prove");
    assert!(proven.members[0].exact, "{:?}", proven.djot);
    assert_eq!(
        written_row_lengths(&proven.djot),
        vec![1, 1, 1, 5],
        "{:?}",
        proven.djot
    );
    let cells: Vec<String> = proven.members[0]
        .segments
        .iter()
        .map(|s| slice(&proven.text, s.stored()))
        .collect();
    assert_eq!(cells, vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
    // "a\nb\nc\nd\ne\nf\ng\nh" is the table's own plain text.
    assert_eq!(proven.members[0].segments[4].source_start, 8);

    let resaved = open(&proven.djot).to_djot().expect("export");
    for cell in ["a", "b", "c", "d", "*e*", "f", "g", "h"] {
        assert!(
            resaved.contains(&format!("| {cell} |")),
            "{cell:?} lost on save: {resaved:?}"
        );
    }
}

/// Every shape of table is written with exactly the cells the file holds, row by row, and
/// its separator line sets off the first row as the file holds it: the emitter adds no cell
/// the file does not contain, whatever count the file asks for.
#[test]
fn a_table_is_written_with_the_cells_the_file_holds_and_no_more() {
    let cell = || vec![plain_run("x")];
    let shapes: Vec<Vec<usize>> = vec![
        vec![1],
        vec![3, 3, 3],
        vec![1, 5, 5, 5],
        vec![5, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 9],
        vec![20_000, 1, 1, 1],
    ];
    for lengths in shapes {
        let rows: Vec<Vec<Vec<Run>>> = lengths.iter().map(|&n| vec![cell(); n]).collect();
        let written = render_table(&rows, Fidelity::Formatted).djot;
        assert_eq!(written_row_lengths(&written), lengths);
        let separator = written.lines().nth(1).unwrap_or_default();
        assert_eq!(separator, format!("|{}", "---|".repeat(lengths[0])));
    }
}

/// One wide row over `narrow` rows of one cell, `doubled` of which hold a second cell, the
/// last narrow row left empty.
fn wide_over_narrow(wide: usize, narrow: usize, doubled: usize) -> Vec<Vec<Vec<Run>>> {
    let mut rows = vec![
        (0..wide)
            .map(|i| vec![plain_run(&format!("w{i}"))])
            .collect(),
    ];
    for i in 0..narrow {
        let mut row = vec![if i + 1 == narrow {
            Vec::new()
        } else {
            vec![plain_run(&format!("r{i}"))]
        }];
        if i < doubled {
            row.push(vec![plain_run(&format!("s{i}"))]);
        }
        rows.push(row);
    }
    rows
}

/// The parser completes a table's short rows only while that keeps the table within
/// 4,096 cells or sixteen times the cells it holds; past both it reads the cells that hold
/// anything as paragraphs. The proof expects each shape on its own side of each limit, so
/// the limits mirrored in `emit` cannot drift from the parser's without this failing: a
/// table expected in the wrong shape does not read back. Either way every real cell is
/// proved exactly, where its segment says, and kept by the editor's first save.
#[test]
fn a_table_is_read_as_a_grid_up_to_the_parsers_limit_and_as_paragraphs_past_it() {
    // (wide, narrow, doubled, read as a grid): 64 by 64 is 4,096 cells, and one row more
    // is past it with 128 cells of the table's own. 100 by 41 is 4,100 cells over 140 of
    // its own, the least a table past 4,096 can complete to with its own cells allowing
    // less: 4,097 and 4,098 cells take more cells of their own than that, and 4,099 is a
    // prime, so this pins the first limit from above as the first case does from below.
    // 20 by 250 is 5,000 cells, which 313 cells of its own allow and 312 do not.
    for (wide, narrow, doubled, grid) in [
        (64, 63, 0, true),
        (64, 64, 0, false),
        (100, 40, 0, false),
        (20, 249, 44, true),
        (20, 249, 43, false),
    ] {
        let rows = wide_over_narrow(wide, narrow, doubled);
        let expected = if grid {
            TableReading::Grid { width: wide }
        } else {
            TableReading::Paragraphs
        };
        assert_eq!(
            table_reading(rows.iter().map(Vec::len)),
            expected,
            "{wide}, {narrow}, {doubled}"
        );
        let proven = prove(&[Source::Table { rows: &rows }], Frame::Blocks).expect("prove");
        let member = &proven.members[0];
        assert!(
            member.exact && !member.reported,
            "{wide}, {narrow}, {doubled}: {:.200?}",
            proven.djot
        );
        assert_eq!(
            proven.text.contains(text_document::TABLE_ANCHOR),
            grid,
            "{wide}, {narrow}, {doubled}: read as a table"
        );
        let cells: Vec<String> = rows
            .iter()
            .flatten()
            .map(|runs| runs.iter().map(|r| r.text.as_str()).collect())
            .collect();
        let with_text: Vec<&str> = cells
            .iter()
            .map(String::as_str)
            .filter(|t| !t.is_empty())
            .collect();
        let stored: Vec<String> = member
            .segments
            .iter()
            .map(|s| slice(&proven.text, s.stored()))
            .collect();
        if grid {
            assert_eq!(stored, cells, "{wide}, {narrow}, {doubled}");
        } else {
            assert_eq!(stored, with_text, "{wide}, {narrow}, {doubled}");
        }

        let resaved = open(&proven.djot).to_djot().expect("export");
        let reread = read_djot(&resaved).expect("parse the save");
        let kept: Vec<&str> = reread
            .blocks
            .iter()
            .map(|b| b.text.as_str())
            .filter(|t| !t.is_empty())
            .collect();
        assert_eq!(
            kept, with_text,
            "{wide}, {narrow}, {doubled}: the first save"
        );
    }
}

/// Cells a word processor fills with more than words (a styled word, a link, a picture,
/// a note, blank space alone) are proved exactly on either side of the parser's limit: as
/// the cells of a grid, and as the paragraphs of the cells that hold anything, each where
/// its segment says, the link's destination and the picture kept. A cell of blank space
/// alone is an empty cell of the grid, and no paragraph at all past the limit.
#[test]
fn odd_cells_are_proved_as_a_grid_and_as_paragraphs() {
    let mut note = plain_run("");
    note.footnote = Some("n1".to_string());
    for (narrow, grid) in [(63, true), (64, false)] {
        let mut rows = wide_over_narrow(64, narrow, 0);
        rows[1] = vec![vec![styled("bold word", bold())]];
        rows[2] = vec![vec![Run::linked(
            "a link",
            RunStyle::default(),
            "https://example.com/a b",
        )]];
        rows[3] = vec![vec![Run::image("", "media/a.png")]];
        rows[4] = vec![vec![plain_run("before "), note.clone()]];
        rows[5] = vec![vec![plain_run("   ")]];
        let proven = prove(&[Source::Table { rows: &rows }], Frame::Blocks).expect("prove");
        let member = &proven.members[0];
        assert!(
            member.exact && !member.reported,
            "{narrow}: {:.300?}",
            proven.djot
        );
        assert_eq!(member.table_as_paragraphs, !grid);
        let stored: Vec<String> = member
            .segments
            .iter()
            .map(|s| slice(&proven.text, s.stored()))
            .collect();
        for cell in ["bold word", "a link", "\u{FFFC}", "before \u{FFFC}"] {
            assert!(
                stored.iter().any(|s| s == cell),
                "{narrow}: {cell:?} in {stored:?}"
            );
        }
        assert_eq!(
            stored.iter().any(String::is_empty),
            grid,
            "{narrow}: the blank cell is a cell of the grid, and no paragraph past it"
        );
        let reading = read_djot(&proven.djot).expect("parse");
        assert!(
            reading
                .blocks
                .iter()
                .any(|b| b.links == vec!["https://example.com/a%20b".to_string()]),
            "{narrow}: the link keeps its destination"
        );
    }
}

/// An epigraph run is one blockquote, however many paragraphs it holds: the compiler
/// marks every blockquote it meets as an epigraph.
#[test]
fn an_epigraph_run_is_one_blockquote() {
    let a = [plain_run("All happy families are alike.")];
    let b = [plain_run("Tolstoy")];
    let sources = [
        Source::Paragraph {
            kind: ParagraphKind::Epigraph,
            runs: &a,
            props: BlockProps::default(),
        },
        Source::Paragraph {
            kind: ParagraphKind::Epigraph,
            runs: &b,
            props: BlockProps {
                alignment: Some(Alignment::Right),
                ..BlockProps::default()
            },
        },
    ];
    let proven = prove(&sources, Frame::Epigraph).expect("prove");
    assert_eq!(
        proven.djot,
        "> All happy families are alike.\n>\n> {alignment=right}\n> Tolstoy"
    );
    assert_eq!(proven.text, "All happy families are alike.\nTolstoy");
    assert!(proven.members.iter().all(|m| m.exact));
}

/// Lists keep their markers as structure and their content as text, a marker-looking
/// start included.
#[test]
fn a_list_item_keeps_its_structure_and_its_words() {
    let a = [plain_run("1. not a sublist")];
    let b = [plain_run("deeper")];
    let sources = [
        Source::Paragraph {
            kind: ParagraphKind::ListItem {
                ordered: false,
                depth: 0,
            },
            runs: &a,
            props: centred(),
        },
        Source::Paragraph {
            kind: ParagraphKind::ListItem {
                ordered: true,
                depth: 1,
            },
            runs: &b,
            props: BlockProps::default(),
        },
    ];
    let proven = prove(&sources, Frame::Blocks).expect("prove");
    assert_eq!(proven.djot, "- 1\\. not a sublist\n\n  1. deeper");
    assert_eq!(proven.text, "1. not a sublist\ndeeper");
}

/// A list nested far past what any word processor writes is stored as prose the next load
/// accepts: every item deeper than [`MAX_LIST_LEVELS`] is written at that level, beside the
/// deepest items kept, and flagged so the importer can say so. Written as deep as its
/// source, the last item would sit 298 columns in, which the load refuses.
#[test]
fn a_list_nested_past_the_levels_kept_is_written_within_what_a_load_accepts() {
    let texts: Vec<String> = (0..150)
        .map(|level| format!("Words at level {level}."))
        .collect();
    let runs: Vec<[Run; 1]> = texts.iter().map(|t| [plain_run(t)]).collect();
    let sources: Vec<Source<'_>> = runs
        .iter()
        .enumerate()
        .map(|(level, runs)| Source::Paragraph {
            kind: ParagraphKind::ListItem {
                ordered: level % 2 == 1,
                depth: u8::try_from(level).expect("150 levels fit a u8"),
            },
            runs,
            props: BlockProps::default(),
        })
        .collect();
    let proven = prove(&sources, Frame::Blocks).expect("prove");

    assert!(
        skrib_format::djot_depth::check(&proven.djot).is_ok(),
        "a load must accept what is stored: {:?}",
        skrib_format::djot_depth::check(&proven.djot)
    );
    let deepest = proven
        .djot
        .lines()
        .map(|line| line.len() - line.trim_start().len())
        .max()
        .unwrap_or(0);
    assert_eq!(deepest, 2 * (MAX_LIST_LEVELS - 1), "{:?}", proven.djot);

    // Every word, in order, each item a list item of its own.
    assert_eq!(proven.text, texts.join("\n"));
    let read = read_djot(&proven.djot).expect("parse");
    assert_eq!(read.blocks.len(), texts.len());
    for (level, member) in proven.members.iter().enumerate() {
        assert!(member.exact && !member.reported, "level {level} reads back");
        assert_eq!(
            member.list_flattened,
            level >= MAX_LIST_LEVELS,
            "level {level} is flattened exactly when it lies past the levels kept"
        );
    }
    let doc = open(&proven.djot);
    let in_lists = doc
        .blocks()
        .iter()
        .filter(|block| block.list().is_some())
        .count();
    assert_eq!(in_lists, texts.len(), "every item is still a list item");
}

/// What the levels kept stand for, as `text-document` 1.12.3 behaves: a list pasted into
/// the editor is held to [`MAX_LIST_LEVELS`], its deeper items at the last of them and
/// still list items, while a list read from stored prose keeps every level it has, and the
/// load of a project refuses one only past `skrib_format::MAX_DJOT_DEPTH` lists deep. None
/// of them is read as literal text, which 1.12.3 was once expected to do past a ceiling.
#[test]
fn a_deep_list_is_held_to_the_levels_kept_when_pasted_and_refused_only_far_past_them() {
    let list = |levels: usize| -> String {
        (0..levels)
            .map(|level| format!("{}- level {level}", "  ".repeat(level)))
            .collect::<Vec<_>>()
            .join("\n\n")
    };
    let levels = |doc: &TextDocument| -> Vec<Option<u8>> {
        doc.blocks()
            .iter()
            .map(|block| block.list().map(|list| list.indent()))
            .collect()
    };
    let deepest = |doc: &TextDocument| levels(doc).into_iter().flatten().max();

    let pasted = TextDocument::new();
    pasted
        .cursor()
        .insert_djot(&list(40))
        .expect("the paste goes in");
    assert!(
        levels(&pasted).iter().all(Option::is_some),
        "every pasted item is a list item"
    );
    assert_eq!(
        deepest(&pasted).map(usize::from),
        Some(MAX_LIST_LEVELS - 1),
        "a paste is held to the levels kept"
    );

    let loaded = open(&list(40));
    assert_eq!(levels(&loaded).len(), 40);
    assert!(levels(&loaded).iter().all(Option::is_some));
    assert_eq!(deepest(&loaded), Some(39), "a load keeps every level");

    let ceiling = skrib_format::MAX_DJOT_DEPTH;
    assert_eq!(skrib_format::djot_depth::check(&list(ceiling)), Ok(()));
    assert!(
        skrib_format::djot_depth::check(&list(ceiling + 1)).is_err(),
        "one list past the ceiling is refused"
    );
}

/// A run longer than one parse is proved a slice at a time, and the slices join exactly
/// as the whole run reads.
#[test]
fn a_long_run_is_proved_in_slices_that_join_as_the_whole_reads() {
    let texts: Vec<String> = (0..(MEMBERS_PER_PARSE * 2 + 7))
        .map(|i| format!("Paragraph {i}: \"quoted\" -- at 10:{i}:00."))
        .collect();
    let runs: Vec<[Run; 1]> = texts.iter().map(|t| [plain_run(t)]).collect();
    let sources: Vec<Source<'_>> = runs
        .iter()
        .map(|r| paragraph(r, BlockProps::default()))
        .collect();
    let proven = prove(&sources, Frame::Blocks).expect("prove");
    let whole = read_djot(&proven.djot).expect("parse");
    assert_eq!(whole.text, proven.text);
    for (member, block) in proven.members.iter().zip(&whole.blocks) {
        assert_eq!(member.segments[0].stored_start, block.start);
    }
}

// ── the proof gates storage ─────────────────────────────────────────────────────────

/// A parser that misreads every block of any Djot holding `markup`, to exercise the
/// fallback.
fn misreading_markup(markup: &'static str) -> impl Fn(&str) -> anyhow::Result<DjotReading> {
    move |djot: &str| {
        let mut reading = read_djot(djot)?;
        if djot.contains(markup) {
            for block in &mut reading.blocks {
                block.text.push('!');
            }
        }
        Ok(reading)
    }
}

/// A parser that misreads every block whose text holds `word`, however it is written.
fn misreading_text(word: &'static str) -> impl Fn(&str) -> anyhow::Result<DjotReading> {
    move |djot: &str| {
        let mut reading = read_djot(djot)?;
        for block in &mut reading.blocks {
            if block.text.contains(word) {
                block.text.push('!');
            }
        }
        Ok(reading)
    }
}

/// A member whose formatted writing does not read back is written again, first with every
/// mark escaped, then as its words alone, and stored in the first writing that proves. The
/// member that fell to its words is reported; its neighbour is untouched.
#[test]
fn a_member_that_does_not_read_back_falls_back_until_one_writing_proves() {
    let a = [styled("Bold words.", bold())];
    let b = [plain_run("Plain words.")];
    let sources = [
        paragraph(&a, centred()),
        paragraph(&b, BlockProps::default()),
    ];
    // Every writing that keeps the bold fails; only the words alone pass.
    let proven = prove_with(&sources, Frame::Blocks, &misreading_markup("{*")).expect("prove");
    assert_eq!(proven.djot, "Bold words\\.\n\nPlain words.");
    assert!(proven.members[0].exact, "its words alone did read back");
    assert!(
        proven.members[0].reported,
        "and losing the formatting is said"
    );
    assert!(proven.members[1].exact && !proven.members[1].reported);
}

/// A member no writing can prove is still stored, in its plainest writing, but no offset
/// into it is trusted and it is reported.
#[test]
fn a_member_no_writing_proves_is_stored_plain_inexact_and_reported() {
    let a = [plain_run("A cursed paragraph.")];
    let b = [plain_run("A fine one.")];
    let sources = [
        paragraph(&a, BlockProps::default()),
        paragraph(&b, BlockProps::default()),
    ];
    let proven = prove_with(&sources, Frame::Blocks, &misreading_text("cursed")).expect("prove");
    assert!(!proven.members[0].exact);
    assert!(proven.members[0].reported);
    assert!(
        proven.djot.starts_with("A cursed paragraph\\."),
        "the plainest writing is what is stored: {:?}",
        proven.djot
    );
    assert!(!proven.members[1].reported);
}

/// A paragraph opening with a link whose text is code holding `]:`. The parser reads any
/// line opening `[…]:` as a reference definition, whatever sits between the brackets, and
/// nothing can be escaped inside a verbatim span, so the line start has to be guarded some
/// other way.
///
/// Which way depends on the `text-document` the build resolves. Up to 1.12.2 its line
/// guard has no such way: the proof catches the misreading, the words are stored plain, and
/// the lost link and code are reported. From 1.12.3 the guard opens such a line with an empty
/// attribute (`{}`), and the link and its code read back as written, with nothing to
/// report. The words read back either way, and a member is reported exactly when it lost
/// its formatting; that is what is pinned here, so the test holds on both sides of the
/// upgrade.
#[test]
fn a_linked_code_span_that_can_read_as_a_definition_keeps_its_words_either_way() {
    let code = RunStyle {
        code: true,
        ..RunStyle::default()
    };
    let runs = [Run::linked("]: x", code, "https://example.com/")];
    let proven = prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
    assert!(proven.members[0].exact, "the words read back");
    assert_eq!(proven.text, "]: x");

    if proven.members[0].reported {
        // Stored as its words alone: every punctuation mark escaped, no link, no code.
        assert_eq!(proven.djot, "\\]\\: x", "the plainest writing is stored");
    } else {
        // Nothing reported, so the link and the code must both have been kept.
        assert!(
            proven.djot.contains("`]: x`"),
            "the code is written as a verbatim span: {:?}",
            proven.djot
        );
        let doc = open(&proven.djot);
        let linked: Vec<(String, Option<String>)> = doc.blocks()[0]
            .fragments()
            .into_iter()
            .filter_map(|f| match f {
                FragmentContent::Text { text, format, .. } => Some((text, format.anchor_href)),
                _ => None,
            })
            .collect();
        assert_eq!(
            linked,
            vec![("]: x".to_string(), Some("https://example.com/".to_string()))],
            "the link reads back around its words: {:?}",
            proven.djot
        );
    }
}

/// Code is written as a verbatim span, its text untouched, fenced past its own backticks.
#[test]
fn a_code_run_is_a_verbatim_span() {
    let code = RunStyle {
        code: true,
        ..RunStyle::default()
    };
    let runs = [
        plain_run("Type "),
        styled("a `tick` *here*", code),
        plain_run(" then "),
        styled("`", RunStyle { bold: true, ..code }),
        plain_run("."),
    ];
    let proven = prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
    assert_eq!(proven.djot, "Type ``a `tick` *here*`` then {*`` ` ``*}.");
    assert_eq!(proven.text, "Type a `tick` *here* then `.");
    assert!(proven.members[0].exact && !proven.members[0].reported);
}

/// The parser's second quirk around verbatim spans: one followed at once by an escaped
/// backslash and a braced delimiter loses the backslash and reads the delimiter as text.
/// The proof catches it and the words are stored plain, reported.
#[test]
fn a_code_span_followed_by_an_escaped_backslash_is_stored_plain_and_reported() {
    let code = RunStyle {
        code: true,
        ..RunStyle::default()
    };
    let strike = RunStyle {
        strikethrough: true,
        ..RunStyle::default()
    };
    let runs = [styled("!", code), plain_run("\\"), styled("!", strike)];
    let proven = prove(&[paragraph(&runs, BlockProps::default())], Frame::Blocks).expect("prove");
    assert_eq!(proven.text, "!\\!");
    assert!(proven.members[0].exact, "the words read back");
    assert!(
        proven.members[0].reported,
        "the lost code and strikethrough are said"
    );
}

// ── the property ────────────────────────────────────────────────────────────────────

const ALPHABET: &[char] = &[
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', 'a', 'A', 'b', 'c', 'i', 'I',
    'v', 'x', 'X', '0', '1', '9', ' ', ' ', '\t', '\u{a0}', '\u{2014}', '\u{e9}',
];

fn text_of(len: Range<usize>) -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALPHABET), len)
        .prop_map(|chars| chars.into_iter().collect())
}

/// Every style but code. No scanner marks a run as code, and the parser has two known
/// quirks around verbatim spans, each pinned by a test of its own above.
fn run_style() -> impl Strategy<Value = RunStyle> {
    (any::<[bool; 4]>(), 0u8..4).prop_map(|([bold, italic, underline, strike], raised)| RunStyle {
        bold,
        italic,
        underline,
        strikethrough: strike,
        code: false,
        superscript: raised == 1,
        subscript: raised == 2,
    })
}

fn run() -> impl Strategy<Value = Run> {
    prop_oneof![
        8 => (text_of(1..8), run_style()).prop_map(|(text, style)| Run::styled(text, style)),
        2 => (text_of(1..6), run_style(), text_of(0..6)).prop_map(|(text, style, tail)| {
            Run::linked(text, style, format!("https://example.com/{tail}"))
        }),
        1 => Just(Run {
            footnote: Some("srcfn-7".into()),
            ..Run::default()
        }),
        1 => text_of(0..4).prop_map(|alt| Run::image(alt, "media/image 1.png")),
    ]
}

fn props() -> impl Strategy<Value = BlockProps> {
    (0u8..5, any::<bool>(), any::<bool>()).prop_map(|(align, rtl, page_break_before)| BlockProps {
        alignment: match align {
            1 => Some(Alignment::Left),
            2 => Some(Alignment::Center),
            3 => Some(Alignment::Right),
            4 => Some(Alignment::Justify),
            _ => None,
        },
        direction: rtl.then_some(Direction::RightToLeft),
        page_break_before,
    })
}

fn kind() -> impl Strategy<Value = ParagraphKind> {
    prop_oneof![
        4 => Just(ParagraphKind::Body),
        1 => Just(ParagraphKind::Quote),
        1 => (any::<bool>(), 0u8..3).prop_map(|(ordered, depth)| ParagraphKind::ListItem { ordered, depth }),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 400, ..ProptestConfig::default() })]

    /// Whatever a paragraph holds, the first writing proves: every member reads back
    /// exactly, none is reported, and every member's text is where its segment says.
    #[test]
    fn every_formatted_paragraph_reads_back_exactly(
        members in prop::collection::vec((kind(), prop::collection::vec(run(), 1..5), props()), 1..6)
    ) {
        let usable: Vec<&(ParagraphKind, Vec<Run>, BlockProps)> = members
            .iter()
            .filter(|(_, runs, _)| {
                let block = RichBlock::Paragraph {
                    kind: ParagraphKind::Body,
                    runs: runs.clone(),
                    props: BlockProps::default(),
                };
                !block.is_blank() && scene_break::tier_of_plain_line(block.plain_text().trim()).is_none()
            })
            .collect();
        prop_assume!(!usable.is_empty());
        let sources: Vec<Source<'_>> = usable
            .iter()
            .map(|(kind, runs, props)| Source::Paragraph { kind: *kind, runs, props: *props })
            .collect();

        let proven = prove(&sources, Frame::Blocks).expect("prove");
        let reading = read_djot(&proven.djot).expect("parse");
        prop_assert_eq!(&reading.text, &proven.text, "Djot {:?}", proven.djot);
        for (i, (member, (_, runs, _))) in proven.members.iter().zip(&usable).enumerate() {
            prop_assert!(member.exact && !member.reported, "member {} of {:?}", i, proven.djot);
            let block = RichBlock::Paragraph {
                kind: ParagraphKind::Body,
                runs: runs.clone(),
                props: BlockProps::default(),
            };
            let source = block.plain_text().replace(['\n', '\r'], " ");
            let kept = skrib_format::trim_djot_whitespace(&source).to_string();
            let segment = &member.segments[0];
            prop_assert_eq!(slice(&proven.text, segment.stored()), kept);
        }
    }
}

fn cell() -> impl Strategy<Value = Vec<Run>> {
    prop::collection::vec(
        prop_oneof![
            4 => (text_of(1..6), run_style()).prop_map(|(text, style)| Run::styled(text, style)),
            1 => (text_of(1..4), text_of(0..4)).prop_map(|(text, tail)| {
                Run::linked(text, RunStyle::default(), format!("https://example.com/{tail}"))
            }),
        ],
        0..3,
    )
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 150, ..ProptestConfig::default() })]

    /// A table of any shape, ragged rows included, proves cell by cell, and every real
    /// cell's text is where its segment says.
    #[test]
    fn every_table_reads_back_cell_by_cell(
        rows in prop::collection::vec(prop::collection::vec(cell(), 1..4), 1..4)
    ) {
        let table = RichBlock::Table { rows: rows.clone() };
        prop_assume!(!table.is_blank());
        let proven = prove(&[Source::Table { rows: &rows }], Frame::Blocks).expect("prove");
        prop_assert!(proven.members[0].exact && !proven.members[0].reported, "{:?}", proven.djot);
        let reading = read_djot(&proven.djot).expect("parse");
        prop_assert_eq!(&reading.text, &proven.text);
        let cells: Vec<String> = rows
            .iter()
            .flatten()
            .map(|runs| {
                let text: String = runs.iter().map(|r| r.text.replace(['\n', '\r'], " ")).collect();
                skrib_format::trim_djot_whitespace(&text).to_string()
            })
            .collect();
        let stored: Vec<String> = proven.members[0]
            .segments
            .iter()
            .map(|s| slice(&proven.text, s.stored()))
            .collect();
        prop_assert_eq!(stored, cells);
    }

    /// A table of short rows with one wide row anywhere in it proves cell by cell, written
    /// with the cells the file holds and no more, every real cell's text where its segment
    /// says, and every cell kept by the editor's first save. Under 1.12.2, which sized a
    /// table from its first row, the shape lost every cell past that row's width at the
    /// first save unless the first row was widened, which this emitter no longer does.
    #[test]
    fn a_table_of_short_rows_and_one_wide_row_reads_back_cell_by_cell(
        short in prop::collection::vec(prop::collection::vec(cell(), 1..3), 3..9),
        wide in prop::collection::vec(cell(), 6..14),
        at in any::<prop::sample::Index>(),
    ) {
        let mut rows = short;
        let at = at.index(rows.len() + 1);
        rows.insert(at, wide);
        let table = RichBlock::Table { rows: rows.clone() };
        prop_assume!(!table.is_blank());
        let proven = prove(&[Source::Table { rows: &rows }], Frame::Blocks).expect("prove");
        prop_assert!(proven.members[0].exact && !proven.members[0].reported, "{:?}", proven.djot);
        let reading = read_djot(&proven.djot).expect("parse");
        prop_assert_eq!(&reading.text, &proven.text);
        let lengths: Vec<usize> = rows.iter().map(Vec::len).collect();
        prop_assert_eq!(written_row_lengths(&proven.djot), lengths, "{:?}", proven.djot);
        let cells: Vec<String> = rows
            .iter()
            .flatten()
            .map(|runs| {
                let text: String = runs.iter().map(|r| r.text.replace(['\n', '\r'], " ")).collect();
                skrib_format::trim_djot_whitespace(&text).to_string()
            })
            .collect();
        let stored: Vec<String> = proven.members[0]
            .segments
            .iter()
            .map(|s| slice(&proven.text, s.stored()))
            .collect();
        prop_assert_eq!(&stored, &cells);
        // The editor's first save keeps every cell's text.
        let resaved = open(&proven.djot).to_djot().expect("export");
        let reread = read_djot(&resaved).expect("parse the save");
        let texts: Vec<&str> = reread
            .blocks
            .iter()
            .map(|b| b.text.as_str())
            .filter(|t| !t.is_empty())
            .collect();
        let wanted: Vec<&str> = cells.iter().map(String::as_str).filter(|t| !t.is_empty()).collect();
        prop_assert_eq!(texts, wanted, "{:?} saved as {:?}", proven.djot, resaved);
    }

    /// An epigraph run of any paragraphs is one blockquote that reads back exactly.
    #[test]
    fn every_epigraph_run_reads_back_as_one_quotation(
        members in prop::collection::vec((prop::collection::vec(run(), 1..4), props()), 1..4)
    ) {
        let usable: Vec<&(Vec<Run>, BlockProps)> = members
            .iter()
            .filter(|(runs, _)| {
                let block = RichBlock::body(runs.clone());
                !block.is_blank() && scene_break::tier_of_plain_line(block.plain_text().trim()).is_none()
            })
            .collect();
        prop_assume!(!usable.is_empty());
        let sources: Vec<Source<'_>> = usable
            .iter()
            .map(|(runs, props)| Source::Paragraph { kind: ParagraphKind::Epigraph, runs, props: *props })
            .collect();
        let proven = prove(&sources, Frame::Epigraph).expect("prove");
        prop_assert!(proven.members.iter().all(|m| m.exact && !m.reported), "{:?}", proven.djot);
        prop_assert!(!proven.djot.contains("\n\n"), "one blockquote: {:?}", proven.djot);
        let doc = open(&proven.djot);
        let frames: std::collections::HashSet<usize> = doc.blocks().iter().map(|b| b.frame().id()).collect();
        prop_assert_eq!(frames.len(), 1, "one frame: {:?}", proven.djot);
    }
}
