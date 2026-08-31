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
    assert_eq!(djot, "He was _utterly_ lost, and *furious*.");
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
    assert_eq!(djot, "*bold*", "not *b**o**ld*");
}

#[test]
fn a_heading_is_a_boundary_not_prose() {
    let doc = assemble_doc(
        vec![
            RichBlock::Paragraph {
                kind: ParagraphKind::Heading { level: 2 },
                runs: vec![Run::plain("Chapter One")],
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

/// A table's own block positions cannot be trusted, so a comment inside one is
/// carried as a comment on the row rather than pointed confidently at nothing.
#[test]
fn a_comment_inside_a_table_becomes_a_whole_row_comment() {
    let doc = assemble_doc(
        vec![RichBlock::Table {
            rows: vec![vec![vec![Run::plain("a")], vec![Run::plain("b")]]],
        }],
        vec![annotation(0, 0, 1)],
    );
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Document);
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
    assert_eq!(doc.annotations[0].kind, AnnotationKind::Document);
    assert!(
        doc.diagnostics
            .iter()
            .any(|d| matches!(d, crate::ImportDiagnostic::CommentUnanchored { .. })),
        "and says so: {:?}",
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
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::ListItem {
                    ordered: false,
                    depth: 1,
                },
                runs: vec![Run::plain("deep")],
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::Quote,
                runs: vec![Run::plain("quoted")],
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
// A comment's own text goes through the same HTML→Djot pipeline as
// manuscript prose, so its emphasis survives instead of being flattened —
// see `body_to_djot`'s doc.

#[test]
fn a_comments_own_emphasis_survives_as_djot() {
    let doc = assemble_doc(
        vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
        vec![RichAnnotation {
            block_index: 0,
            start: 0,
            length: 0,
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
        "Is this *really* the _right_ word?"
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
    assert_eq!(doc.annotations[0].replies[0].body, "*Fixed.*");
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
    assert_eq!(body_to_djot(&[]).expect("convert"), "");
    assert_eq!(
        body_to_djot(&[vec![Run::plain("   ")]]).expect("convert"),
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
            },
            RichBlock::Paragraph {
                kind: ParagraphKind::Epigraph,
                runs: vec![Run::plain("— Tolstoy")],
            },
        ],
        Vec::new(),
    );

    let blocks: Vec<&SourceBlock> = doc.blocks.iter().collect();
    assert_eq!(blocks.len(), 1, "one run, one block: {blocks:?}");
    let SourceBlock::Epigraph { djot, .. } = blocks[0] else {
        panic!("expected an epigraph block, got {blocks:?}");
    };
    assert_eq!(
        djot.lines().filter(|l| l.starts_with('>')).count(),
        2,
        "both paragraphs must sit inside the same quotation: {djot:?}"
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
