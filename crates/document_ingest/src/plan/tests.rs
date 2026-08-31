// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::structure::infer_rules;
use skribisto_model::scene_break::SceneBreakTier;

fn doc(origin: &str, blocks: Vec<SourceBlock>) -> SourceDocument {
    let mut d = SourceDocument::new("fixture", origin);
    d.blocks = blocks;
    d
}

fn heading(level: u8, text: &str) -> SourceBlock {
    SourceBlock::Heading {
        level,
        text: text.into(),
    }
}

fn prose(s: &str) -> SourceBlock {
    SourceBlock::prose(s, s)
}

#[test]
fn a_chapter_with_breaks_is_one_row_carrying_them_inline() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            prose("First."),
            SourceBlock::SceneBreak {
                tier: SceneBreakTier::Minor,
            },
            prose("Second."),
            SourceBlock::SceneBreak {
                tier: SceneBreakTier::Minor,
            },
            prose("Third."),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 1, "breaks must not create rows");
    let row = &plan.rows[0];
    assert_eq!(row.scene_breaks, 2);
    assert!(
        row.djot
            .contains(scene_break::canonical_djot(SceneBreakTier::Minor))
    );
    assert_eq!(row.word_count, 3, "markers are furniture, not words");
}

/// A book split across files shares one heading convention, so the ladder
/// has to span them. Per document, this `## Chapter Two` — the only heading
/// in its file — became a top-level row: type Chapter (the rules are global)
/// at indent 0, i.e. a chapter sitting *outside* the book its sibling is in.
#[test]
fn the_heading_ladder_spans_every_document_in_one_import() {
    let first = doc(
        "01.md",
        vec![heading(1, "Book"), heading(2, "Chapter One"), prose("x")],
    );
    let second = doc("02.md", vec![heading(2, "Chapter Two"), prose("y")]);

    let rules = infer_rules(&[1, 2], CreateType::Book);
    let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

    let shape: Vec<(i64, &str)> = plan
        .rows
        .iter()
        .map(|r| (r.indent, r.title.as_str()))
        .collect();
    assert_eq!(
        shape,
        vec![(0, "Book"), (1, "Chapter One"), (1, "Chapter Two")],
        "both chapters belong to the book"
    );
}

/// …and a later file that opens at the *top* level still starts over, which
/// is what makes the ordinary one-file-per-chapter export work.
#[test]
fn a_later_document_reopening_the_top_level_returns_to_the_top() {
    let first = doc(
        "01.md",
        vec![heading(1, "One"), heading(2, "A scene"), prose("x")],
    );
    let second = doc("02.md", vec![heading(1, "Two"), prose("y")]);

    let rules = infer_rules(&[1, 2], CreateType::Chapter);
    let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

    let shape: Vec<(i64, &str)> = plan
        .rows
        .iter()
        .map(|r| (r.indent, r.title.as_str()))
        .collect();
    assert_eq!(shape, vec![(0, "One"), (1, "A scene"), (0, "Two")]);
}

/// Word counting moved out of the per-document loop; it must still be right
/// for every row of a multi-document import, not only the last one's.
#[test]
fn every_document_gets_its_words_counted() {
    let first = doc("01.md", vec![heading(1, "One"), prose("one two three")]);
    let second = doc("02.md", vec![heading(1, "Two"), prose("four five")]);

    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

    let counts: Vec<usize> = plan.rows.iter().map(|r| r.word_count).collect();
    assert_eq!(counts, vec![3, 2]);
}

#[test]
fn nested_headings_become_indents() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Book"),
            heading(2, "Chapter One"),
            prose("x"),
            heading(2, "Chapter Two"),
            prose("y"),
        ],
    );
    let rules = infer_rules(&[1, 2], CreateType::Book);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    let shape: Vec<(i64, &str)> = plan
        .rows
        .iter()
        .map(|r| (r.indent, r.title.as_str()))
        .collect();
    assert_eq!(
        shape,
        vec![(0, "Book"), (1, "Chapter One"), (1, "Chapter Two")]
    );
}

/// A skipped level nests one step and says so, rather than growing the
/// phantom folders other importers are documented to produce.
#[test]
fn a_skipped_heading_level_nests_one_step_and_is_reported() {
    let d = doc(
        "a.md",
        vec![heading(1, "Book"), heading(4, "Deep"), prose("x")],
    );
    let rules = infer_rules(&[1, 4], CreateType::Book);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows[1].indent, 1);
    assert!(
        plan.rows[1]
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::HeadingLevelJump { .. }))
    );
}

#[test]
fn prose_before_the_first_heading_gets_its_own_row() {
    let d = doc(
        "a.md",
        vec![prose("Preamble."), heading(1, "Chapter One"), prose("x")],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 2);
    assert_eq!(plan.rows[0].djot, "Preamble.");
    assert_eq!(plan.rows[1].title, "Chapter One");
}

#[test]
fn an_ordinal_comes_off_the_title_and_is_kept() {
    let d = doc("a.md", vec![heading(1, "Chapter 3: The Storm"), prose("x")]);
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows[0].title, "The Storm");
    assert_eq!(plan.rows[0].stripped_ordinal.as_deref(), Some("chapter 3"));
}

#[test]
fn sixty_headingless_files_become_sixty_rows() {
    let docs: Vec<SourceDocument> = (0..60)
        .map(|i| doc(&format!("{i}.md"), vec![prose("Scene prose.")]))
        .collect();
    let rules = infer_rules(&[], CreateType::Scene);
    let plan = build_plan(&docs, &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 60);
    assert!(plan.rows.iter().all(|r| r.indent == 0));
}

#[test]
fn base_indent_offsets_the_whole_import() {
    let d = doc(
        "a.md",
        vec![heading(1, "Chapter"), heading(2, "Scene"), prose("x")],
    );
    let rules = infer_rules(&[1, 2], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 3);

    assert_eq!(plan.rows[0].indent, 3);
    assert_eq!(plan.rows[1].indent, 4);
}

#[test]
fn a_repeated_title_is_flagged_because_that_is_what_a_double_import_looks_like() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Later"),
            prose("x"),
            heading(1, "Later"),
            prose("y"),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert!(
        plan.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::DuplicateTitle { occurrences: 2, .. }))
    );
}

#[test]
fn prose_landing_on_a_type_that_cannot_hold_it_is_flagged_not_dropped() {
    let d = doc(
        "a.md",
        vec![heading(1, "A Book"), prose("Prose in a Book.")],
    );
    // Built by hand: `infer_rules` now keeps the deepest level on something
    // prose-bearing, so inference no longer produces this pairing. The writer
    // still can, by retyping a row to Book in the review tree, and it must be
    // caught there rather than by `content_allowed` silently dropping the
    // text at the next save.
    let rules = LevelRules::from_table(std::collections::BTreeMap::from([(1, CreateType::Book)]));
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows[0].create_type, CreateType::Book);
    assert!(
        plan.rows[0]
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::IllegalCombination { .. })),
        "a Book holds no prose, and save-time would drop it silently"
    );
}

// ── Comments no longer mint `CommentAnchorKind::Document` ──────────────
//
// Neither DOCX nor ODF has a "comment on the whole document" concept, and
// the comment UI never wires one up either — see the module doc. Both sites
// that used to mint it are covered here.

fn annotation(block_index: usize, kind: AnnotationKind, body: &str) -> SourceAnnotation {
    SourceAnnotation {
        block_index,
        kind,
        anchor: Anchor::default(),
        uid: None,
        uid_tag: None,
        author: "Editor".into(),
        author_initials: String::new(),
        created: None,
        body: body.into(),
        resolved: false,
        replies: Vec::new(),
    }
}

/// A comment landing on a heading block — what `sources::rich` produces for
/// any comment on a heading (`place.exact` is always `false` there) — must
/// become a `Paragraph` comment on the row's first *real* block, not
/// `CommentAnchorKind::Document`. It must also be a genuine, resolved anchor
/// (a real captured quote), not merely relabelled and left pointing nowhere.
#[test]
fn a_comment_on_a_heading_lands_as_a_paragraph_comment_on_the_rows_first_block() {
    let mut d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            prose("First paragraph."),
            prose("Second paragraph."),
        ],
    );
    // Mirrors `sources::rich::assemble`'s own shape for a heading comment:
    // `AnnotationKind::Document`, block 0 (the heading), no usable anchor.
    d.annotations = vec![annotation(0, AnnotationKind::Document, "Nice opening.")];

    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 1);
    let row = &plan.rows[0];
    assert_eq!(row.comments.len(), 1, "the comment must survive");
    let c = &row.comments[0];
    assert_eq!(
        c.kind,
        CommentAnchorKind::Paragraph,
        "must not be CommentAnchorKind::Document any more"
    );
    assert!(
        !c.orphaned,
        "the row has real prose to fall back to, so this must resolve, not orphan"
    );
    assert_eq!(
        c.anchor.block_ordinal, 0,
        "pinned to the row's first block, not the heading (which has no block at all)"
    );
    assert!(
        !c.anchor.exact.is_empty(),
        "a real quote must be captured from the first block, not left empty"
    );
    assert!(
        c.anchor.exact.contains("First paragraph"),
        "the captured quote must actually be the first block's text, got {:?}",
        c.anchor.exact
    );
}

/// A heading comment on a row with **no** prose at all (nothing ever follows
/// the heading) has nothing to fall back to. It must still not be
/// `CommentAnchorKind::Document` — it becomes a `Paragraph` comment that
/// honestly reports itself orphaned, exactly as a paragraph comment whose
/// wording vanished entirely would.
#[test]
fn a_comment_on_a_heading_with_no_following_prose_becomes_an_orphaned_paragraph_comment() {
    let mut d = doc("a.md", vec![heading(1, "Chapter One")]);
    d.annotations = vec![annotation(0, AnnotationKind::Document, "Nice title.")];

    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 1, "the heading still becomes a row");
    let c = &plan.rows[0].comments[0];
    assert_eq!(c.kind, CommentAnchorKind::Paragraph);
    assert!(
        c.orphaned,
        "nothing in the row's Djot to point at, so it must say so rather than \
             silently claim block 0 of an empty document"
    );
}

/// `pin_to_first_block` is `anchor_comments`' fallback for a row whose Djot
/// failed to parse — unit-tested directly (rather than through `build_plan`)
/// because a genuine Djot parse failure is not something a fixture string can
/// manufacture: `skrib_format::djot_plain_text` wraps a lenient parser that
/// recovers from anything rather than rejecting it, so `Err` in practice is
/// reserved for background-task plumbing, not content. See `anchor_comments`'
/// doc for why this fallback is a `Paragraph` anchor rather than
/// `CommentAnchorKind::Document`.
#[test]
fn the_parse_failure_fallback_pins_every_comment_to_block_zero_with_an_empty_quote() {
    let mut comments = vec![
        planned_comment(
            &annotation(0, AnnotationKind::Range, "A range comment."),
            &prose("Some prose."),
            0,
        ),
        planned_comment(
            &annotation(0, AnnotationKind::Document, "A document comment."),
            &prose("Some prose."),
            0,
        ),
    ];

    pin_to_first_block(&mut comments);

    for c in &comments {
        assert_eq!(
            c.kind,
            CommentAnchorKind::Paragraph,
            "must not be CommentAnchorKind::Document"
        );
        assert_eq!(c.anchor.block_ordinal, 0);
        assert_eq!(c.anchor.block_span, 1);
        assert_eq!(c.anchor.start, 0);
        assert_eq!(c.anchor.length, 0);
        assert!(
            c.anchor.exact.is_empty(),
            "the quote must be empty, not guessed"
        );
        assert!(
            !c.orphaned,
            "not resolved yet — that is the live editor's job on open"
        );
    }
}

/// The bug this exists to fix: a formatted comment body must not show its
/// Djot markup in a diagnostic — an editor's `*right*` reads to the writer as
/// `right` with two stray asterisks, not as the bold word it actually is.
#[test]
fn a_body_preview_strips_djot_markup_before_truncating() {
    assert_eq!(
        body_preview("Is this the *right* word?"),
        "Is this the right word?",
        "the emphasis markers must not survive into the preview"
    );
    assert_eq!(
        body_preview("*Bold* and _italic_ and {-struck-} text."),
        "Bold and italic and struck text.",
        "every marker family must be stripped, not only emphasis"
    );
}

/// The truncation itself still has to work on the *stripped* text, or a body
/// that is short in Djot but long once its escaping backslashes are dropped
/// (or the reverse) would be measured against the wrong length.
#[test]
fn a_body_preview_truncates_the_stripped_text_not_the_raw_djot() {
    let preview = body_preview(&"*x* ".repeat(40));
    assert!(
        preview.chars().count() <= 60,
        "preview ran past 60 chars: {} ({})",
        preview.chars().count(),
        preview
    );
    assert!(
        !preview.contains('*'),
        "markup leaked into the preview: {preview:?}"
    );
}

// ── Epigraphs ───────────────────────────────────────────────────────────
//
// The scanners decide *whether* a paragraph is an epigraph (from its named style);
// these decide *whose* it is. Both editorial placements are real and neither is
// recorded in the file, so the rule is read off adjacency alone — never off the
// export preset, which the importer has no way to know and a foreign file never had.

fn epi(s: &str) -> SourceBlock {
    SourceBlock::Epigraph {
        djot: s.into(),
        text: s.trim_start_matches("> ").into(),
    }
}

fn only_epigraph(plan: &ImportPlan) -> Vec<(&str, &str)> {
    plan.rows
        .iter()
        .filter(|r| !r.epigraph.trim().is_empty())
        .map(|r| (r.title.as_str(), r.epigraph.as_str()))
        .collect()
}

/// The documented convention, and the compiler's default placement.
#[test]
fn an_epigraph_under_a_heading_belongs_to_that_heading() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            epi("> Every winter asks twice."),
            prose("The city held its breath."),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(
        only_epigraph(&plan),
        vec![("Chapter One", "> Every winter asks twice.")]
    );
    assert_eq!(
        plan.rows[0].djot, "The city held its breath.",
        "the epigraph must stay out of the manuscript"
    );
}

/// `EpigraphPlacement::BeforeHeading`. The case a reader keyed on "the row currently
/// collecting prose" gets wrong — and gets wrong *silently*, by filing Chapter Two's
/// quotation at the end of Chapter One's manuscript.
#[test]
fn an_epigraph_above_a_heading_belongs_to_the_heading_below_it() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            prose("Rain all week."),
            epi("> Every winter asks twice."),
            heading(1, "Chapter Two"),
            prose("The city held its breath."),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(
        only_epigraph(&plan),
        vec![("Chapter Two", "> Every winter asks twice.")]
    );
    assert_eq!(
        plan.rows[0].djot, "Rain all week.",
        "Chapter One's prose must not have absorbed the next chapter's epigraph"
    );
}

/// A quotation with prose on both sides is a quotation, not an epigraph. Moving it to
/// a row's `EpigraphText` would take it out of the paragraph it was written into.
#[test]
fn a_quotation_in_the_middle_of_a_scene_stays_in_the_prose() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            prose("She read the letter."),
            epi("> I shall not be home before the thaw."),
            prose("Then she folded it away."),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert!(
        only_epigraph(&plan).is_empty(),
        "nothing here heads a chapter"
    );
    assert!(
        plan.rows[0].djot.contains("I shall not be home"),
        "the quotation belongs in the scene it was written into: {:?}",
        plan.rows[0].djot
    );
    assert!(
        plan.rows[0].diagnostics.is_empty(),
        "a mid-scene quotation is ordinary and must not warn: {:?}",
        plan.rows[0].diagnostics
    );
}

/// The matrix gives a Scene `[SceneText, SynopsisText]` and no `EpigraphText` — a
/// scene has no head to set a quotation at. The quotation is kept, at the top of the
/// prose where it reads correctly, and the writer is told: here they *did* mean an
/// epigraph, and silently making it the scene's opening paragraph is the failure this
/// whole change exists to stop.
#[test]
fn an_epigraph_on_a_type_that_cannot_hold_one_falls_back_to_prose_and_says_so() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            prose("The chapter's own words."),
            heading(2, "Scene A"),
            epi("> Every winter asks twice."),
            prose("Opening words."),
        ],
    );
    let rules = infer_rules(&[1, 2], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    let scene = plan
        .rows
        .iter()
        .find(|r| r.title == "Scene A")
        .expect("the scene row");
    assert_eq!(
        scene.create_type,
        CreateType::Scene,
        "the fixture only means anything if this really is a Scene"
    );
    assert!(only_epigraph(&plan).is_empty(), "a Scene heads no epigraph");
    assert!(
        scene.djot.starts_with("> Every winter asks twice."),
        "the quotation must open the text it was heading: {:?}",
        scene.djot
    );
    assert!(
        scene.diagnostics.iter().any(|d| matches!(
            d,
            ImportDiagnostic::EpigraphNotCarried { kind, .. } if *kind == CreateType::Scene
        )),
        "the writer must be told: {:?}",
        scene.diagnostics
    );
}

/// A Book cannot hold one but the chapter under it can, so the epigraph falls through
/// to the row below rather than being demoted to prose. This is why the rule asks
/// whether each candidate *can carry* an epigraph instead of simply preferring the
/// heading above.
#[test]
fn an_epigraph_under_a_book_falls_through_to_the_chapter_below_it() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "A Book"),
            epi("> Every winter asks twice."),
            heading(2, "Chapter One"),
            prose("Opening words."),
        ],
    );
    let rules = infer_rules(&[1, 2], CreateType::Book);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(
        only_epigraph(&plan),
        vec![("Chapter One", "> Every winter asks twice.")]
    );
}

/// Between a Part and a Chapter, both of which may hold one, the reading is genuinely
/// ambiguous: the convention says it heads the Part above, the other placement says it
/// heads the Chapter below. The convention wins and the writer is told, rather than the
/// tie being resolved out of sight.
#[test]
fn an_epigraph_between_two_types_that_can_both_hold_one_says_so() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Part One"),
            epi("> Every winter asks twice."),
            heading(2, "Chapter One"),
            prose("Opening words."),
        ],
    );
    let rules = infer_rules(&[1, 2], CreateType::Part);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(
        only_epigraph(&plan),
        vec![("Part One", "> Every winter asks twice.")],
        "the documented convention wins the tie"
    );
    assert!(
        plan.rows[0].diagnostics.iter().any(|d| matches!(
            d,
            ImportDiagnostic::EpigraphPlacementAmbiguous { below, .. } if below == "Chapter One"
        )),
        "and the other reading is named: {:?}",
        plan.rows[0].diagnostics
    );
}

/// Two quotations at one chapter's head are two blockquotes on one field, not a
/// silent replacement of the first by the second. `render::mark_epigraph` marks each
/// blockquote separately on the way back out, so both survive a further round trip.
#[test]
fn two_epigraphs_on_one_row_are_kept_as_two_quotations() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            epi("> The first."),
            epi("> The second."),
            prose("Opening words."),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(
        only_epigraph(&plan),
        vec![("Chapter One", "> The first.\n\n> The second.")]
    );
}

/// An epigraph's words are not the manuscript's — the whole reason it is a `Content`
/// of its own. `word_count` drives the review panel's "how much am I importing", and a
/// quotation counted there is a quotation counted into every pace goal downstream.
#[test]
fn an_epigraph_is_not_counted_in_the_rows_word_count() {
    let d = doc(
        "a.md",
        vec![
            heading(1, "Chapter One"),
            epi("> One two three four five."),
            prose("Six seven."),
        ],
    );
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(
        plan.rows[0].word_count, 2,
        "only the row's own prose counts"
    );
}

/// A note goes to the row whose prose cites it, and only that row.
#[test]
fn a_footnote_lands_on_the_row_that_cites_it() {
    let mut d = doc(
        "a.docx",
        vec![
            heading(1, "Chapter One"),
            prose("The ferry was late.[^srcfn-1]"),
            heading(1, "Chapter Two"),
            prose("The harbour was quiet."),
        ],
    );
    d.footnotes = vec![crate::block::SourceFootnote {
        label: "srcfn-1".into(),
        body: "It always is.".into(),
    }];
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 2);
    assert_eq!(
        plan.rows[0].footnotes,
        vec![PlannedFootnote {
            label: "srcfn-1".into(),
            body: "It always is.".into(),
        }]
    );
    assert!(
        plan.rows[1].footnotes.is_empty(),
        "a row that cites nothing carries nothing"
    );
    assert!(
        !plan
            .diagnostics
            .iter()
            .any(|d| d.key() == "footnote-not-carried"),
        "nothing was lost; got {:?}",
        plan.diagnostics
    );
}

/// Two files may safely mint the same placeholder — the pairing is per document.
///
/// This is the whole reason `SourceFootnote::label` promises uniqueness *within one
/// document* and no further: a scanner cannot know what the file beside it numbered
/// its notes, and `w:id` restarts at 1 in every `.docx` ever written.
#[test]
fn two_files_using_the_same_placeholder_do_not_take_each_others_notes() {
    let mut first = doc("a.docx", vec![heading(1, "One"), prose("Alpha.[^srcfn-1]")]);
    first.footnotes = vec![crate::block::SourceFootnote {
        label: "srcfn-1".into(),
        body: "The first note.".into(),
    }];
    let mut second = doc("b.docx", vec![heading(1, "Two"), prose("Beta.[^srcfn-1]")]);
    second.footnotes = vec![crate::block::SourceFootnote {
        label: "srcfn-1".into(),
        body: "The second note.".into(),
    }];

    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

    assert_eq!(plan.rows.len(), 2);
    assert_eq!(plan.rows[0].footnotes[0].body, "The first note.");
    assert_eq!(plan.rows[1].footnotes[0].body, "The second note.");
}

/// A note nobody cites is reported rather than attached to whatever row is nearest.
///
/// The real case is a footnote on a chapter *title*: the heading becomes a
/// `BinderItem.title`, a plain string with no `Content` for a `Footnote` to annotate,
/// so the reference never reaches any row's prose.
#[test]
fn a_note_no_row_cites_is_reported_not_guessed_at() {
    let mut d = doc(
        "a.docx",
        vec![heading(1, "Chapter One"), prose("The ferry was late.")],
    );
    d.footnotes = vec![crate::block::SourceFootnote {
        label: "srcfn-1".into(),
        body: "Orphaned.".into(),
    }];
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert!(plan.rows.iter().all(|r| r.footnotes.is_empty()));
    assert!(
        plan.diagnostics
            .iter()
            .any(|d| d.key() == "footnote-not-carried"),
        "got {:?}",
        plan.diagnostics
    );
}

/// A placeholder shown as an example is prose, not a citation.
///
/// `references_in` reads just enough of Djot's own grammar to know that a code span
/// is inert — the same rule the editor and the exporter apply, so a "notes to self"
/// document explaining the syntax cannot claim a note.
#[test]
fn a_placeholder_inside_a_code_span_does_not_cite_anything() {
    let mut d = doc(
        "a.docx",
        vec![
            heading(1, "Style notes"),
            prose("Write `[^srcfn-1]` for a note."),
        ],
    );
    d.footnotes = vec![crate::block::SourceFootnote {
        label: "srcfn-1".into(),
        body: "Not this one.".into(),
    }];
    let rules = infer_rules(&[1], CreateType::Chapter);
    let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

    assert!(plan.rows.iter().all(|r| r.footnotes.is_empty()));
}
