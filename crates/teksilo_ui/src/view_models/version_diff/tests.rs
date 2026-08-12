// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

fn plain(djot: &str) -> String {
    djot_to_plain_text(djot, &DjotImportOptions::default())
}

fn no_elision(_: usize) -> String {
    String::from("…")
}

fn rendered(before: &str, after: &str) -> Rendered {
    render(&diff_djot(before, after), None, &no_elision)
}

// ── escaping ───────────────────────────────────────────────────────────────

/// The property the whole renderer rests on: escaped prose reparses to itself.
///
/// Asserted against the real parser rather than by reading the escape table,
/// because the table is only correct relative to what the parser does with it.
#[test]
fn every_djot_special_character_survives_the_round_trip() {
    let adversarial = [
        "a *star* and an _underscore_",
        "brackets [like this] and (parens)",
        "braces {+ not an insert +} and {-not a delete-}",
        "back\\slash and `code` and ~tilde~ and ^caret^",
        "a pipe | in a sentence",
        "less < than and a #hash mid-line",
        "an ellipsis... and an em--dash and a \"quote\" and an 'apostrophe'",
        "# not a heading",
        "> not a quote",
        "- not a list",
        "1. not an ordered list",
        "42) also not one",
        ": not a definition",
        "+ not a bullet",
        "*[]{}~^|<\\`()_",
    ];
    for text in adversarial {
        let block = render_block(&DiffBlock {
            kind: BlockKind::Equal,
            runs: vec![Run {
                op: RunOp::Equal,
                text: text.to_string(),
            }],
        });
        assert_eq!(
            plain(&block),
            text,
            "escaping did not round-trip; emitted djot was {block:?}",
        );
    }
}

/// A wholly inserted paragraph renders as a line that *starts* with `{`, which
/// is where Djot puts a block attribute. It must still be prose.
#[test]
fn a_block_that_opens_with_an_insert_mark_is_not_read_as_an_attribute() {
    let r = rendered("", "A wholly new paragraph.");
    assert!(
        r.djot.starts_with("{+"),
        "the case under test did not arise: {}",
        r.djot,
    );
    assert_eq!(plain(&r.djot), "A wholly new paragraph.");
}

/// …and the same when the mark is a deletion, which is the other lone-brace
/// line this renderer can produce.
#[test]
fn a_block_that_opens_with_a_delete_mark_is_not_read_as_an_attribute() {
    let r = rendered("The paragraph that was removed.", "");
    assert!(r.djot.starts_with("{-"), "unexpected: {}", r.djot);
    assert_eq!(plain(&r.djot), "The paragraph that was removed.");
}

/// The marks have to actually reach the document as underline and strikeout,
/// or the pane is showing braces to a reader.
#[test]
fn insertions_and_deletions_reach_the_document_as_marks_not_as_braces() {
    let r = rendered("the lamp went out", "the lamp guttered");
    let text = plain(&r.djot);
    assert!(text.contains("went out"), "the old words must be shown");
    assert!(text.contains("guttered"), "the new words must be shown");
    assert!(
        !text.contains('{') && !text.contains('+'),
        "the marks leaked into the text as literal characters: {text}",
    );
}

// ── the word pass ──────────────────────────────────────────────────────────

#[test]
fn a_rewritten_sentence_produces_word_hunks_not_character_hunks() {
    let d = diff_djot("the lamp went out", "the lamp went dark");
    assert_eq!(d.blocks.len(), 1);
    assert_eq!(d.blocks[0].kind, BlockKind::Changed);
    let deleted: Vec<&str> = d.blocks[0]
        .runs
        .iter()
        .filter(|r| r.op == RunOp::Delete)
        .map(|r| r.text.as_str())
        .collect();
    assert_eq!(
        deleted,
        vec!["out"],
        "a whole word must change, not the letters it shares",
    );
}

/// The reason `absorb_islands` exists.
#[test]
fn coincidental_matches_do_not_shred_two_unrelated_sentences() {
    let d = diff_djot("she opened the gate", "he shut a door");
    let runs = &d.blocks[0].runs;
    let changes = runs.iter().filter(|r| r.op != RunOp::Equal).count();
    assert!(
        changes <= 2,
        "the sentence was shredded into {changes} fragments: {runs:?}",
    );
}

/// A real edit in the middle of a paragraph keeps its unchanged surroundings —
/// island absorption must not swallow genuine agreement.
#[test]
fn a_long_unchanged_stretch_between_two_edits_stays_unchanged() {
    let d = diff_djot(
        "the lamp burned bright and the room went dark",
        "the candle burned bright and the room went pale",
    );
    let runs = &d.blocks[0].runs;
    let equal: String = runs
        .iter()
        .filter(|r| r.op == RunOp::Equal)
        .map(|r| r.text.as_str())
        .collect();
    assert!(
        equal.contains("burned bright and the room went"),
        "the untouched middle was absorbed: {runs:?}",
    );
}

#[test]
fn two_unrelated_paragraphs_are_a_replacement_not_an_interleave() {
    let d = diff_djot(
        "She waited by the harbour until the tide turned.",
        "Rain fell on the roof of the disused signal box.",
    );
    let kinds: Vec<BlockKind> = d.blocks.iter().map(|b| b.kind).collect();
    assert_eq!(
        kinds,
        vec![BlockKind::Delete, BlockKind::Insert],
        "below the pair gate this must read as replaced, not edited",
    );
}

// ── the block pass ─────────────────────────────────────────────────────────

#[test]
fn an_unchanged_document_produces_no_changes() {
    let text = "One paragraph.\n\nAnd another.";
    let d = diff_djot(text, text);
    assert!(d.is_empty());
    assert_eq!(d.magnitude(), 0.0);
    assert!(!d.summary.formatting_only);
    assert!(rendered(text, text).change_offsets.is_empty());
}

#[test]
fn a_reordered_paragraph_is_reported_as_moved_not_as_destroyed_and_rewritten() {
    let before = "The first paragraph here.\n\nThe second paragraph here.\n\nThe third one here.";
    let after = "The second paragraph here.\n\nThe third one here.\n\nThe first paragraph here.";
    let d = diff_djot(before, after);
    assert_eq!(d.summary.blocks_moved, 1, "blocks: {:?}", d.blocks);
    assert!(
        !d.blocks.iter().any(|b| b.kind == BlockKind::Delete),
        "a moved paragraph must not also read as deleted: {:?}",
        d.blocks,
    );
    assert_eq!(
        d.summary.words_added, 0,
        "moving text adds no words, and saying it did would overstate the edit",
    );
}

#[test]
fn a_short_repeated_line_is_not_mistaken_for_a_move() {
    // Two scene breaks are not one scene break that travelled.
    let d = diff_djot(
        "* * *\n\nAlpha beta gamma delta.",
        "Alpha beta gamma delta.\n\n* * *",
    );
    assert_eq!(
        d.summary.blocks_moved, 0,
        "a three-character line is below the move threshold",
    );
}

#[test]
fn a_first_version_reads_as_wholly_added() {
    let d = diff_djot("", "A scene written from nothing.");
    assert_eq!(d.blocks.len(), 1);
    assert_eq!(d.blocks[0].kind, BlockKind::Insert);
    assert_eq!(d.magnitude(), 1.0);
}

#[test]
fn markup_only_changes_are_named_rather_than_shown_as_an_empty_diff() {
    let d = diff_djot("the *lamp* went out", "the _lamp_ went out");
    assert!(d.is_empty(), "no word moved");
    assert!(
        d.summary.formatting_only,
        "a version that only re-italicised must say so, not show nothing",
    );
}

// ── magnitude ──────────────────────────────────────────────────────────────

#[test]
fn magnitude_scales_with_how_much_actually_moved() {
    let base = "the lamp went out and the room fell dark";
    let small = diff_djot(base, "the lamp went out and the room fell silent").magnitude();
    let large = diff_djot(base, "she closed the door and the room fell dark").magnitude();
    assert!(small > 0.0, "one changed word must register");
    assert!(
        small < large,
        "one word ({small}) must read as less than four ({large})"
    );
    assert!(small < 0.3, "a one-word edit must not look like a rewrite");
}

/// The short-text case `analysis::repetition` cannot measure at all.
#[test]
fn magnitude_survives_the_short_text_that_shingles_cannot_measure() {
    let m = diff_djot("A quiet opening.", "A loud opening.").magnitude();
    assert!(m > 0.0 && m < 1.0, "short text must still read, got {m}");
}

// ── the summary ────────────────────────────────────────────────────────────

#[test]
fn the_summary_counts_words_and_names_where_the_change_is() {
    let d = diff_djot(
        "They met again at the garden gate, saying nothing.",
        "They met again at the garden gate, saying everything at once.",
    );
    assert!(d.summary.words_added >= 1);
    let anchor = d
        .summary
        .anchor
        .expect("a mid-paragraph change has a landmark");
    assert!(
        anchor.contains("garden gate") || anchor.contains("saying"),
        "the landmark must be the text beside the change, got {anchor:?}",
    );
}

#[test]
fn a_change_that_opens_a_paragraph_is_anchored_on_what_follows_it() {
    let d = diff_djot(
        "Rain fell all evening on the quiet street.",
        "Snow fell all evening on the quiet street.",
    );
    let anchor = d
        .summary
        .anchor
        .expect("an opening change still has a landmark");
    assert!(!anchor.is_empty());
}

// ── rendering and offsets ──────────────────────────────────────────────────

/// The offsets are what jump-to-next-change moves a cursor to, so they are
/// asserted against the document's own text rather than against the markup.
#[test]
fn every_change_offset_lands_on_the_first_character_of_its_block() {
    let before = "Alpha one two three.\n\nBeta four five six.\n\nGamma seven eight nine.";
    let after = "Alpha one two three.\n\nBeta four five SIX.\n\nGamma seven eight nine.";
    let d = diff_djot(before, after);
    let r = render(&d, None, &no_elision);
    let text: Vec<char> = plain(&r.djot).chars().collect();
    assert_eq!(r.change_offsets.len(), 1, "one block changed");
    let at = r.change_offsets[0];
    let got: String = text[at..(at + 4).min(text.len())].iter().collect();
    assert_eq!(got, "Beta", "the offset pointed at {got:?}");
}

#[test]
fn offsets_stay_correct_when_the_first_block_is_the_changed_one() {
    let d = diff_djot(
        "Alpha one two three.\n\nBeta.",
        "Alpha one two four.\n\nBeta.",
    );
    let r = render(&d, None, &no_elision);
    assert_eq!(r.change_offsets, vec![0]);
}

#[test]
fn the_unchanged_middle_collapses_and_says_how_much_it_hid() {
    let mut before = vec!["The paragraph that changed, before.".to_string()];
    for i in 0..8 {
        before.push(format!("Untouched paragraph number {i} carries on."));
    }
    let mut after = before.clone();
    after[0] = "The paragraph that changed, after.".to_string();
    let d = diff_djot(&before.join("\n\n"), &after.join("\n\n"));

    let full = render(&d, None, &no_elision);
    let collapsed = render(&d, Some(CollapseRule::default()), &|n| {
        format!("[{n} hidden]")
    });
    assert!(
        collapsed.djot.len() < full.djot.len(),
        "collapsing did not shorten anything",
    );
    // Eight unchanged blocks follow the change; one is kept as context on the
    // side that borders it, and there is no change after them to keep context
    // for — so seven are hidden.
    assert!(
        plain(&collapsed.djot).contains("[7 hidden]"),
        "the placeholder must say how many: {}",
        plain(&collapsed.djot),
    );
    // Both readings of the edited paragraph survive: the struck-out old words
    // and the underlined new ones sit adjacent, which is what track changes
    // looks like once the marks are applied.
    let text = plain(&collapsed.djot);
    assert!(
        text.contains("The paragraph that changed, before.after."),
        "the change itself must survive collapsing: {text}",
    );
}

#[test]
fn collapsing_keeps_context_on_the_side_that_borders_a_change() {
    let mut blocks = vec!["Changed line here, yes.".to_string()];
    for i in 0..6 {
        blocks.push(format!("Steady paragraph {i} unchanged throughout."));
    }
    let mut after = blocks.clone();
    after[0] = "Changed line here, no.".to_string();
    let d = diff_djot(&blocks.join("\n\n"), &after.join("\n\n"));
    let r = render(&d, Some(CollapseRule::default()), &|n| format!("[{n}]"));
    let text = plain(&r.djot);
    assert!(
        text.contains("Steady paragraph 0"),
        "one unchanged block of context must remain: {text}",
    );
    assert!(
        !text.contains("Steady paragraph 3"),
        "the middle must be hidden: {text}",
    );
}

/// Collapsing must not move a change offset — it is a cursor target.
#[test]
fn change_offsets_are_recomputed_for_the_collapsed_rendering() {
    let mut before: Vec<String> = (0..8)
        .map(|i| format!("Untouched paragraph number {i} carries on."))
        .collect();
    before.push("The last paragraph, before.".to_string());
    let mut after = before.clone();
    let last = after.len() - 1;
    after[last] = "The last paragraph, after.".to_string();
    let d = diff_djot(&before.join("\n\n"), &after.join("\n\n"));
    let r = render(&d, Some(CollapseRule::default()), &|n| format!("[{n}]"));
    let text: Vec<char> = plain(&r.djot).chars().collect();
    let at = r.change_offsets[0];
    let got: String = text[at..(at + 8).min(text.len())].iter().collect();
    assert_eq!(got, "The last", "the offset pointed at {got:?}");
}

/// The end-to-end escaping property, and the strongest statement this module
/// can make: comparing a document against itself reproduces its text exactly.
///
/// Stated against the *document's* reading of the source rather than against
/// the source, because that is what the pane shows — `*lamp*` is the word
/// `lamp` in italics, and a diff that printed the asterisks would be lying
/// about what the writer typed.
#[test]
fn comparing_a_document_against_itself_reproduces_its_text_exactly() {
    let sources = [
        "She wrote *lamp* twice, then stopped.",
        // Escaped in the source, so the document really does hold these
        // characters as prose — the case the renderer has to survive.
        "Braces \\{here\\} and brackets \\[twice\\] and a pipe \\| too.",
        "1\\. Not an ordered list.\n\n\\# Not a heading.\n\n\\> Not a quote.",
        "A back\\\\slash, a \\*star\\*, a \\`backtick\\` and a \\~tilde\\~.",
        "> A real blockquote.\n\n# A real heading\n\n- a real list item",
        "An ellipsis... an em---dash, a \"quote\" and an 'apostrophe'.",
    ];
    for src in sources {
        let r = rendered(src, src);
        assert_eq!(
            plain(&r.djot),
            plain(src),
            "the pane would have shown different text than the document holds",
        );
    }
}

/// …and the same when something *did* change around the awkward characters.
#[test]
fn markup_characters_survive_on_both_sides_of_a_real_change() {
    let before = "She wrote \\{here\\} on the door.";
    let after = "She wrote \\{there\\} on the door.";
    let text = plain(&rendered(before, after).djot);
    assert!(
        text.contains("{here}"),
        "the removed text was mangled: {text}"
    );
    assert!(
        text.contains("{there}"),
        "the added text was mangled: {text}"
    );
}
