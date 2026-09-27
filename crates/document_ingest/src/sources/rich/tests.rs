// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

fn bold() -> RunStyle {
    RunStyle {
        bold: true,
        ..Default::default()
    }
}

fn assemble_doc(blocks: Vec<RichBlock>, annotations: Vec<RichAnnotation>) -> SourceDocument {
    let mut out = SourceDocument::new("fixture", "fixture.docx");
    assemble(
        &RichDocument {
            blocks,
            annotations,
            row_marks: Vec::new(),
            footnotes: Vec::new(),
        },
        &mut out,
    )
    .expect("assemble");
    out
}

fn annotation(block_index: usize, start: usize, length: usize) -> RichAnnotation {
    RichAnnotation {
        block_index,
        start,
        length,
        end: None,
        uid: None,
        uid_tag: None,
        author: "Editor".into(),
        author_initials: String::new(),
        created: None,
        paragraphs: vec![vec![Run::plain("Is this the right word?")]],
        resolved: false,
        replies: Vec::new(),
    }
}

/// The words an annotation points at, taken from the block it names rather than from
/// what the annotation says about itself.
fn quoted(doc: &SourceDocument, index: usize) -> String {
    let annotation = &doc.annotations[index];
    let text = doc.blocks[annotation.block_index].plain_text();
    text.chars()
        .skip(annotation.anchor.start)
        .take(annotation.anchor.length)
        .collect()
}

#[test]
fn styled_runs_become_djot_with_the_right_delimiters() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![
            Run::plain("He was "),
            Run::styled(
                "utterly",
                RunStyle {
                    italic: true,
                    ..Default::default()
                },
            ),
            Run::plain(" lost, and "),
            Run::styled("furious", bold()),
            Run::plain("."),
        ])],
        Vec::new(),
    );
    let SourceBlock::Prose { djot, text } = &doc.blocks[0] else {
        panic!("expected prose, got {:?}", doc.blocks);
    };
    // Braced delimiters always: they read the same inside a word and when nested.
    assert_eq!(djot, "He was {_utterly_} lost, and {*furious*}.");
    assert_eq!(
        text, "He was utterly lost, and furious.",
        "the plain text is the space an anchor is measured in"
    );
}

/// Word splits one styled word across several identically-formatted runs. The
/// conversion must not show the seams.
#[test]
fn a_word_split_across_three_identical_runs_comes_out_once() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![
            Run::styled("b", bold()),
            Run::styled("ol", bold()),
            Run::styled("d", bold()),
        ])],
        Vec::new(),
    );
    let SourceBlock::Prose { djot, .. } = &doc.blocks[0] else {
        panic!("expected prose");
    };
    assert_eq!(djot, "{*bold*}", "one pair of delimiters, not three");
}

#[test]
fn a_heading_is_a_boundary_not_prose() {
    let doc = assemble_doc(
        vec![
            RichBlock::Paragraph {
                kind: ParagraphKind::Heading { level: 2 },
                runs: vec![Run::plain("Chapter One")],
                props: BlockProps::default(),
            },
            RichBlock::body(vec![Run::plain("Prose.")]),
        ],
        Vec::new(),
    );
    assert!(matches!(
        doc.blocks.as_slice(),
        [
            SourceBlock::Heading { level: 2, .. },
            SourceBlock::Prose { .. }
        ]
    ));
}

/// Content beats style, the same rule the Markdown scanner applies to a raw
/// span — so re-importing Skribisto's own export does not invent a chapter.
#[test]
fn a_paragraph_that_is_only_a_break_glyph_is_a_break_whatever_it_was_styled() {
    for (kind, glyph) in [
        (ParagraphKind::Body, "* * *"),
        (ParagraphKind::Heading { level: 1 }, "# # #"),
        (ParagraphKind::Body, "⁂"),
    ] {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("Before.")]),
                RichBlock::Paragraph {
                    kind,
                    runs: vec![Run::plain(glyph)],
                    props: BlockProps::default(),
                },
                RichBlock::body(vec![Run::plain("After.")]),
            ],
            Vec::new(),
        );
        assert!(
            doc.blocks
                .iter()
                .any(|b| matches!(b, SourceBlock::SceneBreak { .. })),
            "{glyph:?} styled {kind:?} was not read as a break: {:?}",
            doc.blocks
        );
        assert!(
            !doc.blocks
                .iter()
                .any(|b| matches!(b, SourceBlock::Heading { .. })),
            "{glyph:?} must not also be a heading"
        );
    }
}

#[test]
fn consecutive_paragraphs_become_one_prose_block() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("First.")]),
            RichBlock::body(vec![Run::plain("Second.")]),
            RichBlock::body(vec![Run::plain("Third.")]),
        ],
        Vec::new(),
    );
    assert_eq!(doc.blocks.len(), 1);
    let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
        panic!("expected prose");
    };
    assert_eq!(text, "First.\nSecond.\nThird.");
}

/// The arithmetic the whole comment feature rests on: block *n* of a run starts
/// where the plain text says it does.
#[test]
fn an_annotation_on_a_later_paragraph_is_offset_by_the_ones_before_it() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("First.")]),
            RichBlock::body(vec![Run::plain("She turned the corner.")]),
        ],
        vec![annotation(1, 4, 6)], // "turned"
    );
    assert_eq!(doc.annotations.len(), 1);
    let a = &doc.annotations[0];
    assert_eq!(a.kind, AnnotationKind::Range);
    assert_eq!(a.block_index, 0, "one prose block holds both paragraphs");
}

#[test]
fn a_comment_with_no_range_is_a_paragraph_comment() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
        vec![annotation(0, 0, 0)],
    );
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Paragraph);
}

/// A table is proved cell by cell like any paragraph, so a comment inside one keeps its
/// words, measured in the addressable text the table's anchor character is part of.
#[test]
fn a_comment_inside_a_table_keeps_its_words() {
    let doc = assemble_doc(
        vec![RichBlock::Table {
            rows: vec![vec![vec![Run::plain("salt")], vec![Run::plain("bleached")]]],
        }],
        // "bleached": the table's own plain text is "salt\nbleached".
        vec![annotation(0, 5, 8)],
    );
    let a = &doc.annotations[0];
    assert_eq!(a.kind, AnnotationKind::Range);
    assert_eq!(a.anchor.exact, "bleached");
    assert_eq!(quoted(&doc, 0), "bleached");
}

#[test]
fn a_table_is_a_prose_block_of_its_own() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("Intro.")]),
            RichBlock::Table {
                rows: vec![vec![vec![Run::plain("a")], vec![Run::plain("b")]]],
            },
            RichBlock::body(vec![Run::plain("Outro.")]),
        ],
        Vec::new(),
    );
    assert_eq!(
        doc.blocks.len(),
        3,
        "the table must not share a block with the prose around it: {:?}",
        doc.blocks
    );
}

#[test]
fn a_blank_paragraph_produces_nothing_and_does_not_shift_what_follows() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("First.")]),
            RichBlock::body(vec![Run::plain("   ")]),
            RichBlock::body(vec![Run::plain("Second.")]),
        ],
        vec![annotation(2, 0, 6)], // "Second"
    );
    let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
        panic!("expected prose");
    };
    assert_eq!(text, "First.\nSecond.");
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Range);
}

/// A comment on a paragraph that produced nothing lands on the nearest paragraph, the one
/// before it, as a paragraph comment: never as a comment on the whole document, which
/// neither format can express and the comment panel cannot open.
#[test]
fn a_comment_whose_paragraph_produced_nothing_is_reported_not_dropped() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("Kept.")]),
            RichBlock::body(vec![Run::plain("  ")]),
        ],
        vec![annotation(1, 0, 0)],
    );
    assert_eq!(doc.annotations.len(), 1, "the comment survives");
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Paragraph);
    assert_eq!(doc.annotations[0].anchor.exact, "Kept.");
    assert!(
        doc.annotations[0].unanchored,
        "and is flagged for the planner to report"
    );
    assert!(
        !doc.diagnostics
            .iter()
            .any(|d| matches!(d, crate::ImportDiagnostic::CommentUnanchored { .. })),
        "once, by the planner, which knows where it lands: {:?}",
        doc.diagnostics
    );
}

#[test]
fn lists_and_quotes_convert_and_keep_one_line_each() {
    let doc = assemble_doc(
        vec![
            RichBlock::Paragraph {
                kind: ParagraphKind::ListItem {
                    ordered: false,
                    depth: 0,
                },
                runs: vec![Run::plain("one")],
                props: BlockProps::default(),
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::ListItem {
                    ordered: false,
                    depth: 1,
                },
                runs: vec![Run::plain("deep")],
                props: BlockProps::default(),
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::Quote,
                runs: vec![Run::plain("quoted")],
                props: BlockProps::default(),
            },
        ],
        Vec::new(),
    );
    let SourceBlock::Prose { djot, text } = &doc.blocks[0] else {
        panic!("expected prose");
    };
    assert!(djot.contains("- one"), "got {djot:?}");
    assert!(djot.contains("> quoted"), "got {djot:?}");
    assert_eq!(text, "one\ndeep\nquoted");
}

/// The characters that would otherwise become markup.
#[test]
fn markup_characters_in_the_prose_survive_as_themselves() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain(
            "Tom & Jerry <3 — \"quoted\" and 5 > 3",
        )])],
        Vec::new(),
    );
    let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
        panic!("expected prose");
    };
    assert_eq!(text, "Tom & Jerry <3 — \"quoted\" and 5 > 3");
}

/// A stray newline inside a run would survive into the plain text without
/// producing a block, and every later offset in the run would be wrong.
#[test]
fn a_stray_newline_inside_a_run_does_not_desynchronise_the_offsets() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("First\nline")]),
            RichBlock::body(vec![Run::plain("Second.")]),
        ],
        vec![annotation(1, 0, 6)],
    );
    let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
        panic!("expected prose");
    };
    assert_eq!(text, "First line\nSecond.");
    assert_eq!(
        doc.annotations[0].kind,
        AnnotationKind::Range,
        "the offset check must still have passed"
    );
}

#[test]
fn a_body_preview_is_one_short_line() {
    assert_eq!(preview("  two\n  lines  "), "two lines");
    assert_eq!(preview(&"x".repeat(80)).chars().count(), 60);
}

// ── Rich comment bodies (M-S4) ──────────────────────────────────────────
//
// A comment's own text is written and proved by the same emitter as manuscript
// prose, so its emphasis survives instead of being flattened. See
// `Assembly::body_to_djot`'s doc.

#[test]
fn a_comments_own_emphasis_survives_as_djot() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
        vec![RichAnnotation {
            block_index: 0,
            start: 0,
            length: 0,
            end: None,
            uid: None,
            uid_tag: None,
            author: "Editor".into(),
            author_initials: String::new(),
            created: None,
            paragraphs: vec![vec![
                Run::plain("Is this "),
                Run::styled("really", bold()),
                Run::plain(" the "),
                Run::styled(
                    "right",
                    RunStyle {
                        italic: true,
                        ..Default::default()
                    },
                ),
                Run::plain(" word?"),
            ]],
            resolved: false,
            replies: Vec::new(),
        }],
    );
    assert_eq!(
        doc.annotations[0].body,
        "Is this {*really*} the {_right_} word?"
    );
}

/// A reply carries the same richness as the opening comment — the card
/// treats every turn alike, and so must the importer.
#[test]
fn a_replys_own_emphasis_survives_as_djot_too() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
        vec![RichAnnotation {
            block_index: 0,
            start: 0,
            length: 0,
            end: None,
            uid: None,
            uid_tag: None,
            author: "Editor".into(),
            author_initials: String::new(),
            created: None,
            paragraphs: vec![vec![Run::plain("Opening remark.")]],
            resolved: false,
            replies: vec![RichReply {
                uid: None,
                author: "Writer".into(),
                author_initials: String::new(),
                created: None,
                paragraphs: vec![vec![Run::styled("Fixed.", bold())]],
            }],
        }],
    );
    assert_eq!(doc.annotations[0].replies.len(), 1);
    assert_eq!(doc.annotations[0].replies[0].body, "{*Fixed.*}");
}

/// A comment written as more than one paragraph — the LibreOffice
/// convention for "Enter" inside a comment box — keeps both paragraphs
/// rather than being glued into one run-on sentence.
#[test]
fn a_multi_paragraph_comment_keeps_both_paragraphs() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
        vec![RichAnnotation {
            block_index: 0,
            start: 0,
            length: 0,
            end: None,
            uid: None,
            uid_tag: None,
            author: "Editor".into(),
            author_initials: String::new(),
            created: None,
            paragraphs: vec![
                vec![Run::plain("First thought.")],
                vec![Run::plain("Second thought.")],
            ],
            resolved: false,
            replies: Vec::new(),
        }],
    );
    let body = &doc.annotations[0].body;
    assert!(body.contains("First thought."), "got {body:?}");
    assert!(body.contains("Second thought."), "got {body:?}");
    assert_ne!(
        body, "First thought.Second thought.",
        "the paragraph break must survive, not glue the two sentences together"
    );
}

/// A paragraph that is entirely whitespace — an editor who pressed Enter
/// twice without typing anything — must not turn into a bare, meaningless
/// Djot paragraph marker, the same rule `RichBlock::is_blank` applies to
/// manuscript prose.
#[test]
fn a_blank_paragraph_in_a_comment_contributes_nothing() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
        vec![RichAnnotation {
            block_index: 0,
            start: 0,
            length: 0,
            end: None,
            uid: None,
            uid_tag: None,
            author: "Editor".into(),
            author_initials: String::new(),
            created: None,
            paragraphs: vec![vec![Run::plain("Only thought.")], vec![Run::plain("   ")]],
            resolved: false,
            replies: Vec::new(),
        }],
    );
    assert_eq!(doc.annotations[0].body, "Only thought.");
}

/// An annotation with no paragraphs at all (defensive — neither scanner
/// produces this) converts to the empty string rather than erroring.
#[test]
fn an_annotation_with_no_paragraphs_converts_to_an_empty_body() {
    let mut assembly = Assembly {
        placement: Vec::new(),
        not_verbatim: 0,
        styled_blanks: 0,
        lists_flattened: 0,
        read: &skrib_format::read_djot,
    };
    assert_eq!(assembly.body_to_djot(&[]).expect("convert"), "");
    assert_eq!(
        assembly
            .body_to_djot(&[vec![Run::plain("   ")]])
            .expect("convert"),
        ""
    );
}

// ── the reply-citation stripper ─────────────────────────────────────────────────────

/// What LibreOffice actually writes, in the locales it writes it in.
#[test]
fn a_word_processors_own_citation_line_is_recognised() {
    for line in [
        "Répondre à  (10/08/2026, 09:27): \"en sorte qu\"",
        "Reply to Editor (08/10/2026, 09:27): \u{201C}the passage\u{201D}",
        "Antwort an Lektor (10.08.2026, 09:27): \"die Stelle\"",
        "Ответить (2026-08-10, 09:27): \u{00AB}текст\u{00BB}",
    ] {
        assert!(is_reply_citation(line), "not recognised: {line}");
    }
}

/// The editor's own words, which merely share the punctuation.
///
/// Every one of these was eaten by the shape test before it required a clock time as well
/// as a date: the reply arrived in the writer's thread with its first paragraph missing,
/// on both formats, whichever application wrote the file.
#[test]
fn a_real_reply_that_merely_looks_like_one_is_kept() {
    for line in [
        "My note from our call (10/14): \"cut this scene entirely\"",
        "See the style guide (p. 231): \"never open on weather\"",
        "As we agreed (rev. 3.2): \"this chapter stays\"",
        "She said it herself (twice): \"I am not going\"",
        "Compare chapters 4-5 with 11-12: \"the same beat\"",
        // These three survived the first tightening — "contains a date-ish pair and a
        // time-ish pair" is satisfied by a time range, a scripture reference and a
        // chapter-and-verse span. What rules them out is that a real stamp holds nothing
        // but digits and separators.
        "Confirm the schedule (9:15-10:30): \"keep as is\"",
        "As discussed (John 3:16, Matt 5:3-12): \"consider these verses\"",
        "Check the timeline (ch. 3:16-5:22): \"way too fast\"",
    ] {
        assert!(!is_reply_citation(line), "wrongly eaten: {line}");
    }
}

/// The timestamp test on its own, since it is what carries the whole judgement.
#[test]
fn a_bare_stamp_is_told_from_anything_a_person_would_write() {
    for stamp in [
        "10/08/2026, 09:27",
        "2026-08-10, 09:27",
        "10.08.2026, 09.27",
        "08/10/2026, 9:27 AM",
        " 10/08/2026 , 09:27:31 ",
    ] {
        assert!(is_timestamp(stamp), "not a stamp: {stamp:?}");
    }
    for not in [
        "9:15-10:30",
        "John 3:16, Matt 5:3-12",
        "ch. 3:16-5:22",
        "2026, 9",
        "p. 231",
        "",
        "10/08/2026",
        "see fig. 3, table 4",
    ] {
        assert!(!is_timestamp(not), "wrongly a stamp: {not:?}");
    }
}

/// A citation with nothing after it is a reply whose content the editor deleted. Dropping
/// the only paragraph would leave an empty reply rather than an odd one.
#[test]
fn a_reply_that_is_only_a_citation_keeps_it() {
    let only = vec![vec![Run::plain(
        "Répondre à  (10/08/2026, 09:27): \"en sorte qu\"",
    )]];
    assert_eq!(without_reply_citation(&only).len(), 1);
}

#[test]
fn a_citation_ahead_of_real_words_is_dropped_and_the_words_are_not() {
    let reply = vec![
        vec![Run::plain(
            "Répondre à  (10/08/2026, 09:27): \"en sorte qu\"",
        )],
        vec![Run::plain("My reply")],
    ];
    let kept = without_reply_citation(&reply);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0][0].text, "My reply");
}

// ── attaching marks to annotations ──────────────────────────────────────────────────

fn comment_mark(block: usize, start: usize, length: usize, tag: &str) -> CommentMark {
    CommentMark {
        uid_tag: tag.into(),
        block,
        start,
        length,
    }
}

/// The function's whole documented purpose — position, not name and not order — and until
/// now nothing tested it with more than one comment in play. Gutting the `(block, start)`
/// check to "the first unclaimed annotation" left every round-trip test green.
#[test]
fn each_mark_reaches_the_annotation_at_its_own_position() {
    // Deliberately out of order relative to the annotations, which is the case an
    // order-based match would get wrong.
    let mut annotations = vec![
        annotation(0, 5, 4),
        annotation(0, 40, 6),
        annotation(2, 5, 4),
    ];
    let marks = [
        comment_mark(2, 5, 4, "tag-third"),
        comment_mark(0, 40, 6, "tag-second"),
        comment_mark(0, 5, 4, "tag-first"),
    ];
    attach_comment_marks(&mut annotations, &marks);

    assert_eq!(annotations[0].uid_tag.as_deref(), Some("tag-first"));
    assert_eq!(annotations[1].uid_tag.as_deref(), Some("tag-second"));
    assert_eq!(annotations[2].uid_tag.as_deref(), Some("tag-third"));
}

/// A mark whose comment the editor deleted is simply unused, and an annotation the editor
/// wrote themselves keeps no tag — which is what makes it import as a new comment.
#[test]
fn a_mark_without_an_annotation_and_an_annotation_without_a_mark_both_survive() {
    let mut annotations = vec![annotation(0, 5, 4), annotation(0, 90, 3)];
    let marks = [
        comment_mark(0, 90, 3, "tag-ours"),
        comment_mark(1, 12, 5, "tag-for-a-deleted-comment"),
    ];
    attach_comment_marks(&mut annotations, &marks);

    assert_eq!(annotations[0].uid_tag, None, "the editor's own remark");
    assert_eq!(annotations[1].uid_tag.as_deref(), Some("tag-ours"));
}

/// A paragraph comment stores no range of its own and takes the mark's, so the extent the
/// editor's application maintained is what comes home.
#[test]
fn a_paragraph_comment_adopts_its_marks_extent_but_a_ranged_one_keeps_its_own() {
    let mut annotations = vec![annotation(0, 5, 0), annotation(1, 5, 4)];
    let marks = [
        comment_mark(0, 5, 30, "tag-para"),
        comment_mark(1, 5, 99, "tag-range"),
    ];
    attach_comment_marks(&mut annotations, &marks);

    assert_eq!(annotations[0].length, 30, "a paragraph comment adopts it");
    assert_eq!(annotations[1].length, 4, "a ranged one keeps what it had");
}

// ── Style vocabulary ────────────────────────────────────────────────────

/// Both writers' own names, and the ODF escape for the one that contains a space.
#[test]
fn our_own_style_names_are_recognised_however_they_are_spelled() {
    for name in [
        "Epigraph",
        "EpigraphAttribution",
        "Epigraph Attribution",
        "Epigraph_20_Attribution",
        "epigraph-attribution",
    ] {
        assert_eq!(
            styled_as(name),
            Some(StyledAs::Epigraph),
            "{name} names an epigraph"
        );
    }
    assert_eq!(styled_as("Quote"), Some(StyledAs::Quote));
}

/// The styles a writer gets from the block-quote button in the application they
/// actually wrote the manuscript in. Reading them is the same move as reading
/// `style:default-outline-level` off a novel template's chapter style: the document
/// stating what a paragraph is, in its own producer's vocabulary.
#[test]
fn the_host_applications_own_quote_styles_are_recognised() {
    for name in ["Quotations", "IntenseQuote", "Intense Quote", "BlockText"] {
        assert_eq!(styled_as(name), Some(StyledAs::Quote), "{name} is a quote");
    }
}

/// Everything else is body text. A style whose name merely *contains* one of the
/// words is not a match: folding is over the whole name, not a substring search, so a
/// writer's own "Quotebox Caption" stays the caption it is.
#[test]
fn an_unrelated_style_claims_nothing() {
    for name in [
        "Standard",
        "Heading1",
        "Normal",
        "Quotebox Caption",
        "Epigraphy",
        "",
    ] {
        assert_eq!(styled_as(name), None, "{name:?} must claim nothing");
    }
}

/// An epigraph run becomes **one** block holding one blockquote, however many
/// paragraphs it had.
///
/// One quotation per blockquote is what `render::mark_epigraph` assumes on the way
/// back out — it marks every blockquote it finds — so a two-paragraph epigraph split
/// into two quotations here would export as two epigraphs on the next trip, and four
/// on the one after that.
#[test]
fn an_epigraph_run_becomes_one_block_holding_one_quotation() {
    let doc = assemble_doc(
        vec![
            RichBlock::Paragraph {
                kind: ParagraphKind::Epigraph,
                runs: vec![Run::plain("All happy families are alike.")],
                props: BlockProps::default(),
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::Epigraph,
                runs: vec![Run::plain("— Tolstoy")],
                props: BlockProps::default(),
            },
        ],
        Vec::new(),
    );

    let blocks: Vec<&SourceBlock> = doc.blocks.iter().collect();
    assert_eq!(blocks.len(), 1, "one run, one block: {blocks:?}");
    let SourceBlock::Epigraph { djot, .. } = blocks[0] else {
        panic!("expected an epigraph block, got {blocks:?}");
    };
    // One blockquote: the empty quoted line keeps the attribution inside it, where a
    // blank line would open a second quotation.
    assert_eq!(
        djot, "> All happy families are alike.\n>\n> — Tolstoy",
        "both paragraphs must sit inside the same quotation"
    );
}

/// An epigraph splits the prose around it rather than being lifted out of the middle
/// of it, so `out.blocks` stays in document order — which is the entire basis on
/// which `plan` decides, from adjacency, which heading the quotation belongs to.
#[test]
fn an_epigraph_keeps_its_place_in_document_order() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("Before.")]),
            RichBlock::Paragraph {
                kind: ParagraphKind::Epigraph,
                runs: vec![Run::plain("The quotation.")],
                props: BlockProps::default(),
            },
            RichBlock::body(vec![Run::plain("After.")]),
        ],
        Vec::new(),
    );

    let shape: Vec<&str> = doc
        .blocks
        .iter()
        .map(|b| match b {
            SourceBlock::Prose { text, .. } => text.as_str(),
            SourceBlock::Epigraph { .. } => "<epigraph>",
            _ => "<other>",
        })
        .collect();
    assert_eq!(shape, vec!["Before.", "<epigraph>", "After."]);
}

// ── The defect this module was rewritten for ────────────────────────────────────────

/// A double space and a tab early in a run used to shift every later paragraph's offset,
/// and each of their comments fell back to the whole document. Written directly, the
/// whitespace is kept and every comment keeps its words.
#[test]
fn comments_after_a_double_space_or_a_tab_keep_their_words() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("One.  Two sentences, two spaces.")]),
            // A tab the scanner turned into a space, opening the paragraph.
            RichBlock::body(vec![Run::plain(" Indented by a tab.")]),
            RichBlock::body(vec![Run::plain("She turned the corner.")]),
            RichBlock::body(vec![
                Run::plain("The street was "),
                Run::styled("gone", bold()),
                Run::plain("."),
            ]),
        ],
        vec![
            annotation(1, 1, 8),  // "Indented"
            annotation(2, 4, 6),  // "turned"
            annotation(3, 15, 4), // "gone"
        ],
    );
    let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
        panic!("expected prose, got {:?}", doc.blocks);
    };
    assert_eq!(
        text,
        "One.  Two sentences, two spaces.\nIndented by a tab.\nShe turned the corner.\nThe street was gone."
    );
    for (index, words) in ["Indented", "turned", "gone"].iter().enumerate() {
        assert_eq!(doc.annotations[index].kind, AnnotationKind::Range);
        assert_eq!(quoted(&doc, index), *words);
    }
}

/// Whatever the reason a comment cannot keep its words, it lands on a paragraph: never on
/// the whole document.
#[test]
fn no_comment_is_ever_a_comment_on_the_whole_document() {
    let doc = assemble_doc(
        vec![
            RichBlock::Paragraph {
                kind: ParagraphKind::Heading { level: 1 },
                runs: vec![Run::plain("Chapter")],
                props: BlockProps::default(),
            },
            RichBlock::body(vec![Run::plain("   ")]),
            RichBlock::body(vec![Run::plain("Prose.")]),
            RichBlock::body(vec![Run::plain("* * *")]),
        ],
        vec![
            annotation(0, 0, 3),          // on the heading
            annotation(1, 0, 0),          // on a blank paragraph
            annotation(3, 0, 5),          // on the break
            annotation(usize::MAX, 0, 0), // nowhere the scanner could say
        ],
    );
    assert_eq!(doc.annotations.len(), 4);
    for a in &doc.annotations {
        assert_ne!(a.kind, AnnotationKind::Document, "{a:?}");
    }
    // The blank paragraph's comment went to the paragraph before it that produced a block.
    assert_eq!(doc.annotations[1].block_index, 0, "the heading before it");
    // The one with no position went to the last paragraph.
    assert_eq!(doc.annotations[3].anchor.exact, "* * *");
}

/// Centring and a page break reach the stored prose; a centred line that reads as a scene
/// break stays a break.
#[test]
fn paragraph_formatting_reaches_the_prose() {
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("Before.")]),
            RichBlock::Paragraph {
                kind: ParagraphKind::Body,
                runs: vec![Run::plain("A centred line on a new page.")],
                props: BlockProps {
                    alignment: Some(Alignment::Center),
                    direction: None,
                    page_break_before: true,
                },
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::Body,
                runs: vec![Run::plain("* * *")],
                props: BlockProps {
                    alignment: Some(Alignment::Center),
                    ..BlockProps::default()
                },
            },
        ],
        Vec::new(),
    );
    let SourceBlock::Prose { djot, text } = &doc.blocks[0] else {
        panic!("expected prose, got {:?}", doc.blocks);
    };
    assert_eq!(
        djot,
        "Before.\n\n{alignment=center page_break_before=true}\nA centred line on a new page."
    );
    assert_eq!(text, "Before.\nA centred line on a new page.");
    assert!(matches!(doc.blocks[1], SourceBlock::SceneBreak { .. }));
}

/// A paragraph whose writing does not read back is stored as its words alone, and the
/// writer is told; a comment on it keeps to its paragraph rather than to unproved words.
#[test]
fn a_paragraph_that_fails_its_proof_is_stored_plain_and_reported() {
    let misread = |djot: &str| -> anyhow::Result<skrib_format::DjotReading> {
        let mut reading = skrib_format::read_djot(djot)?;
        for block in &mut reading.blocks {
            if block.text.contains("cursed") {
                block.text.push('!');
            }
        }
        Ok(reading)
    };
    let rich = RichDocument {
        blocks: vec![
            RichBlock::body(vec![Run::plain("A fine paragraph.")]),
            RichBlock::body(vec![
                Run::plain("A "),
                Run::styled("cursed", bold()),
                Run::plain(" one."),
            ]),
        ],
        annotations: vec![annotation(1, 2, 6)],
        row_marks: Vec::new(),
        footnotes: Vec::new(),
    };
    let mut out = SourceDocument::new("fixture", "fixture.docx");
    assemble_with(&rich, &mut out, &misread).expect("assemble");

    let SourceBlock::Prose { djot, .. } = &out.blocks[0] else {
        panic!("expected prose, got {:?}", out.blocks);
    };
    assert_eq!(djot, "A fine paragraph.\n\nA cursed one\\.");
    assert!(
        out.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::ProseNotVerbatim { count: 1, .. })),
        "{:?}",
        out.diagnostics
    );
    assert_eq!(out.annotations[0].kind, AnnotationKind::Paragraph);
    assert!(out.annotations[0].unanchored, "{:?}", out.annotations[0]);
}

/// `Block Quote`, the style Scrivener's compiler writes into its Word and OpenDocument
/// output, is a quotation like the host applications' own.
#[test]
fn a_block_quote_style_is_a_quotation() {
    for name in ["Block Quote", "BlockQuote", "Block_20_Quote"] {
        assert_eq!(styled_as(name), Some(StyledAs::Quote), "{name}");
    }
}

/// A range an editor laid across two paragraphs keeps its whole extent, the paragraph
/// break included, when both ends land in the same stored block.
#[test]
fn a_range_across_paragraphs_keeps_its_whole_extent() {
    let mut spanning = annotation(1, 4, 18); // "turned the corner." to the end of block 1
    spanning.end = Some(AnnotationEnd {
        block_index: 2,
        offset: 7, // "The fog"
    });
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("First.")]),
            RichBlock::body(vec![Run::plain("She turned the corner.")]),
            RichBlock::body(vec![Run::plain("The fog stayed.")]),
        ],
        vec![spanning],
    );
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Range);
    assert_eq!(quoted(&doc, 0), "turned the corner.\nThe fog");
}

/// When something that is not prose lies between the two ends, the range stops at the
/// end of the paragraph it started in, rather than pointing somewhere unproved.
#[test]
fn a_range_across_a_heading_stops_at_the_end_of_its_first_paragraph() {
    let mut spanning = annotation(0, 4, 18);
    spanning.end = Some(AnnotationEnd {
        block_index: 2,
        offset: 7,
    });
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("She turned the corner.")]),
            RichBlock::Paragraph {
                kind: ParagraphKind::Heading { level: 1 },
                runs: vec![Run::plain("Chapter Two")],
                props: BlockProps::default(),
            },
            RichBlock::body(vec![Run::plain("The fog stayed.")]),
        ],
        vec![spanning],
    );
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Range);
    assert_eq!(quoted(&doc, 0), "turned the corner.");
}

/// A stretch of blank space that is underlined or struck through, a line left to fill in,
/// is written as plain spaces and reported, once per stretch: the editor would drop the
/// line the first time it saved the paragraph. The struck one ends its paragraph, so it is
/// not written at all, and is reported the same. Bold spaces show nothing and are not
/// counted, nor is a styled word whose edge spaces go outside its delimiters.
#[test]
fn an_underlined_blank_arrives_as_plain_spaces_and_is_reported() {
    let underline = RunStyle {
        underline: true,
        ..Default::default()
    };
    let struck = RunStyle {
        strikethrough: true,
        ..Default::default()
    };
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![
                Run::plain("Name:"),
                Run::styled("      ", underline),
                Run::plain(" Date:"),
                Run::styled("    ", struck),
            ]),
            RichBlock::body(vec![
                Run::plain("A"),
                Run::styled("   ", bold()),
                Run::styled("word ", underline),
                Run::plain("here."),
            ]),
        ],
        Vec::new(),
    );
    let SourceBlock::Prose { djot, text } = &doc.blocks[0] else {
        panic!("expected prose, got {:?}", doc.blocks);
    };
    assert_eq!(djot, "Name:       Date:\n\nA   {+word+} here.");
    assert_eq!(text, "Name:       Date:\nA   word here.");
    let reported: Vec<&ImportDiagnostic> = doc
        .diagnostics
        .iter()
        .filter(|d| matches!(d, ImportDiagnostic::StyledSpacesNotCarried { .. }))
        .collect();
    assert!(
        matches!(
            reported.as_slice(),
            [ImportDiagnostic::StyledSpacesNotCarried { count: 2, .. }]
        ),
        "{reported:?}"
    );
}

/// Blank space at a paragraph's start or end is not kept, styled or not, so a line left to
/// fill in after a label, or on a line of its own, arrives not at all. It is counted with
/// the stretches that arrive as plain spaces all the same: the writer is told either way.
#[test]
fn an_underlined_blank_at_a_paragraphs_edge_or_on_its_own_is_reported_too() {
    let underline = RunStyle {
        underline: true,
        ..Default::default()
    };
    let struck = RunStyle {
        strikethrough: true,
        ..Default::default()
    };
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![
                Run::plain("Signature:"),
                Run::styled("                    ", underline),
            ]),
            RichBlock::body(vec![Run::styled("\t\t", underline)]),
            RichBlock::body(vec![Run::styled("    ", struck), Run::plain("Date.")]),
        ],
        Vec::new(),
    );
    let [SourceBlock::Prose { djot, text }] = doc.blocks.as_slice() else {
        panic!("one prose block: {:?}", doc.blocks);
    };
    assert_eq!(djot, "Signature:\n\nDate.");
    assert_eq!(text, "Signature:\nDate.");
    let reported: Vec<&ImportDiagnostic> = doc
        .diagnostics
        .iter()
        .filter(|d| matches!(d, ImportDiagnostic::StyledSpacesNotCarried { .. }))
        .collect();
    assert!(
        matches!(
            reported.as_slice(),
            [ImportDiagnostic::StyledSpacesNotCarried { count: 3, .. }]
        ),
        "{reported:?}"
    );
}

/// The same in a table cell and in a comment's own text, which go through the same
/// emitter, down to a table or a paragraph of the comment that holds nothing else.
#[test]
fn an_underlined_blank_in_a_cell_or_a_comment_is_reported_too() {
    let underline = RunStyle {
        underline: true,
        ..Default::default()
    };
    let mut remark = annotation(0, 0, 4);
    remark.paragraphs = vec![
        vec![Run::plain("Sign here:"), Run::styled("     ", underline)],
        vec![Run::styled("      ", underline)],
    ];
    let doc = assemble_doc(
        vec![
            RichBlock::body(vec![Run::plain("Text.")]),
            RichBlock::Table {
                rows: vec![vec![
                    vec![Run::plain("Name")],
                    vec![Run::styled("        ", underline)],
                ]],
            },
            RichBlock::Table {
                rows: vec![vec![vec![Run::styled("        ", underline)]]],
            },
        ],
        vec![remark],
    );
    assert!(
        doc.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::StyledSpacesNotCarried { count: 4, .. })),
        "{:?}",
        doc.diagnostics
    );
}
