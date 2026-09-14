// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The contracts in this crate that are statements about every input.
//!
//! Everything here is pure, store-free and IO-free, which is what makes these testable at
//! all — and also what makes them easy to under-test. A table of examples proves a function
//! does the right thing on the rows someone thought of. These are the claims the modules'
//! own doc comments already make in prose, written so they can fail:
//!
//! * a comment that has not moved still points where it pointed, and one that has cannot
//!   point outside the text (`comment_anchor`);
//! * a chapter's number is a fact about the manuscript, so it does not change when an export
//!   filters rows out (`numbering`);
//! * a resolved export scope is a real, contiguous part of the stream (`compile`);
//! * neither text engine can be made to panic, or to claim more characters than the writer
//!   typed (`replacement`, `typography::engine`);
//! * a scene break the writer typed is the scene break that is recognised (`scene_break`);
//! * case folding is idempotent and never loses a Turkish letter (`casing`).
//!
//! # Offsets are characters
//!
//! Every offset in `comment_anchor` is a document-absolute **character** offset, not a byte
//! offset. The tests build their haystacks out of multi-byte text on purpose: an
//! implementation that slipped into byte space passes every ASCII example and corrupts the
//! first paragraph containing an accent.

use proptest::prelude::*;

use crate::comment_anchor::{self, Resolution};
use crate::compile::{ItemMeta, ScopeKind, StreamLevel, resolve_scope};
use crate::numbering::{NumberingRules, level_of, number_map};
use crate::replacement::{TextReplacementEngine, TextReplacementRuleRow};
use crate::scene_break::{self, SceneBreakTier};
use crate::typography::engine::{SmartPunctuationFlags, TypographyEngine};
use common::entities::{BinderItemRole, BinderItemSubRole};

// ───────────────────────────────────────────────────────────────────────────────
// Shared generators
// ───────────────────────────────────────────────────────────────────────────────

/// Prose built from a small alphabet on purpose.
///
/// A narrow alphabet is what makes repeated phrases likely, and a repeated phrase is the
/// whole difficulty of re-anchoring: it is what the prefix and suffix exist to disambiguate
/// and what `Ambiguous` exists to refuse. Generating from a wide alphabet would produce text
/// in which every quote is unique, and the interesting half of the module would never run.
///
/// The non-ASCII characters are there so a byte/character mix-up cannot pass: `é` is two
/// bytes, `日` three, and the object replacement character is what an inline image is.
fn text_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "a", "b", " ", "the ", "cat ", "\n", "\u{e9}", "\u{65e5}", "\u{fffc}", ". ",
        ]),
        0..40,
    )
    .prop_map(|parts| parts.concat())
}

/// Prose together with a character range inside it.
///
/// Drawn as one strategy rather than two so the range is always valid for the text it goes
/// with. Generating them independently and clamping afterwards would collapse most cases onto
/// the end of the string, which is the one place a range is least interesting.
fn text_and_range() -> impl Strategy<Value = (String, usize, usize)> {
    text_strategy().prop_flat_map(|text| {
        let n = text.chars().count();
        (Just(text), 0..=n).prop_flat_map(move |(text, start)| (Just(text), Just(start), start..=n))
    })
}

/// Where every block of `text` starts, in characters — blocks being separated by `\n`, which
/// is how `djot_to_plain_text` joins them and therefore what the caller hands over.
fn block_starts_of(text: &str) -> Vec<usize> {
    let mut out = vec![0usize];
    for (i, ch) in text.chars().enumerate() {
        if ch == '\n' {
            out.push(i + 1);
        }
    }
    out
}

// ───────────────────────────────────────────────────────────────────────────────
// comment_anchor
// ───────────────────────────────────────────────────────────────────────────────

proptest! {
    /// A comment on unedited prose still points exactly where it pointed.
    ///
    /// The common case, and the one that has to be exact rather than merely close: the
    /// stored offset is right, so tier 1 answers and neither the quote search nor the block
    /// fallback is consulted. A failure here means a comment moved while the writer was not
    /// editing, which is the thing this design exists to make impossible.
    #[test]
    fn an_unedited_comment_resolves_to_itself(
        (text, start, end) in text_and_range()
    ) {
        let blocks = block_starts_of(&text);
        let anchor = comment_anchor::capture(&text, start, end, comment_anchor::block_of(&blocks, start));
        let resolved = comment_anchor::resolve(&text, &anchor, false, &blocks);
        prop_assert_eq!(
            resolved,
            Resolution::Anchored { start, length: end - start },
            "an unedited comment moved: text {:?} range {}..{}",
            text,
            start,
            end
        );
    }

    /// Capturing clamps rather than panicking, and reports the range it actually took.
    ///
    /// `capture` is called from the editor with offsets that came from a selection, and a
    /// selection can be stale by the time the comment is created. The doc comment says the
    /// offsets are clamped; this is that sentence as a test, over ranges that are inverted,
    /// past the end, or both.
    #[test]
    fn capturing_clamps_instead_of_panicking(
        text in text_strategy(),
        start in 0usize..200,
        end in 0usize..200,
        ordinal in 0usize..8,
    ) {
        let n = text.chars().count();
        let anchor = comment_anchor::capture(&text, start, end, ordinal);
        prop_assert!(anchor.start <= n, "start {} past the end of {} chars", anchor.start, n);
        prop_assert!(
            anchor.start + anchor.length <= n,
            "the captured range runs past the end of the text"
        );
        let clamped_start = start.min(n);
        prop_assert_eq!(anchor.start, clamped_start);
        prop_assert_eq!(anchor.start + anchor.length, end.clamp(clamped_start, n));
    }

    /// However the prose was rewritten, a resolved comment points inside the text or says it
    /// is an orphan.
    ///
    /// The safety net under the whole feature. Every consumer slices the document with what
    /// `resolve` returns — the margin card, the export, the highlight — so an offset past the
    /// end is not a wrong answer, it is a panic in whichever of them reaches it first. The
    /// anchor here is deliberately captured against *different* text from the one it is
    /// resolved against, which is the situation after an edit outside the app.
    #[test]
    fn a_resolved_comment_never_points_outside_the_text(
        (original, start, end) in text_and_range(),
        rewritten in text_strategy(),
        is_paragraph in any::<bool>(),
    ) {
        let blocks = block_starts_of(&original);
        let anchor = comment_anchor::capture(&original, start, end, comment_anchor::block_of(&blocks, start));

        let new_blocks = block_starts_of(&rewritten);
        let n = rewritten.chars().count();
        if let Resolution::Anchored { start, length } =
            comment_anchor::resolve(&rewritten, &anchor, is_paragraph, &new_blocks)
        {
            prop_assert!(start <= n, "start {start} past the end of {n} chars");
            prop_assert!(
                start + length <= n,
                "range {start}..{} runs past the end of {n} chars",
                start + length
            );
        }
    }

    /// A shifted range stays a range, and stays inside the edited text.
    ///
    /// `shift_range` runs on every `ContentsChanged`, including the ones undo and redo emit,
    /// so it is the hottest arithmetic in the feature and the one with no UI to notice it
    /// going wrong. Both endpoints have deliberately *different* sticky rules, which is
    /// exactly the sort of asymmetry that produces an inverted range on some edit nobody
    /// tried.
    #[test]
    fn a_shifted_range_stays_a_range(
        len in 0usize..80,
        start in 0usize..80,
        extra in 0usize..80,
        position in 0usize..80,
        removed in 0usize..80,
        added in 0usize..80,
    ) {
        // Constrain the inputs to what a real `ContentsChanged` can carry: a range inside the
        // old text, and an edit that removes only characters that were there.
        let start = start.min(len);
        let end = (start + extra).min(len);
        let position = position.min(len);
        let removed = removed.min(len - position);
        let new_len = len - removed + added;

        let (new_start, new_end) = comment_anchor::shift_range(start, end, position, removed, added);

        prop_assert!(new_start <= new_end, "the range inverted: {new_start}..{new_end}");
        prop_assert!(
            new_end <= new_len,
            "the range runs past the edited text: {new_start}..{new_end} of {new_len}"
        );
    }

    /// An edit strictly after a comment leaves it exactly where it was.
    ///
    /// The single most common edit there is — the writer is typing further down the page —
    /// and the one a reader would assume without checking. Stated separately from the bounds
    /// property above because "did not crash" and "did not move" are different promises.
    ///
    /// *Strictly* after: an edit exactly at an endpoint is the sticky-rule case below, not
    /// this one.
    #[test]
    fn an_edit_after_a_comment_does_not_move_it(
        (start, end, position) in (1usize..80).prop_flat_map(|len| {
            // Built inward from the text length so every draw is a legal edit: an edit
            // position strictly after a comment that is itself inside the text. Drawing the
            // four independently and filtering would throw most cases away.
            (1..=len).prop_flat_map(|position| {
                (0..position).prop_flat_map(move |end| (0..=end, Just(end), Just(position)))
            })
        }),
        added in 0usize..40,
        removed in 0usize..10,
    ) {
        let (new_start, new_end) = comment_anchor::shift_range(start, end, position, removed, added);
        prop_assert_eq!(
            (new_start, new_end),
            (start, end),
            "an edit at {} moved the comment at {}..{}",
            position,
            start,
            end
        );
    }

    /// Text typed at either edge of a comment lands **outside** it.
    ///
    /// The rule the module is built around, and the reason `TextCursor` cannot do this for
    /// us: its `adjust_cursors` applies one left-sticky rule to both endpoints, under which an
    /// insertion at a comment's start leaves `start` put while pushing `end` along, so the
    /// comment silently swallows the new text. Here the two endpoints stick to opposite sides
    /// — an insertion at `start` slides the whole comment down, an insertion at `end` leaves
    /// it entirely alone — and the comment covers the same characters either way.
    ///
    /// Only for a non-empty range. When `start == end` the two rules point in opposite
    /// directions on the same offset and the tie is broken in favour of a valid range, which
    /// is a different claim and belongs with the bounds property rather than here.
    #[test]
    fn text_typed_at_either_edge_lands_outside_the_comment(
        len in 1usize..80,
        start in 0usize..80,
        extra in 1usize..40,
        added in 1usize..40,
    ) {
        let start = start.min(len.saturating_sub(1));
        let end = (start + extra).min(len);
        prop_assume!(start < end);

        // At the start: right-sticky, so the comment slides and the new text is above it.
        prop_assert_eq!(
            comment_anchor::shift_range(start, end, start, 0, added),
            (start + added, end + added),
            "an insertion at the start did not slide the comment"
        );

        // At the end: left-sticky, so the comment stays and the new text is below it.
        prop_assert_eq!(
            comment_anchor::shift_range(start, end, end, 0, added),
            (start, end),
            "an insertion at the end moved the comment"
        );
    }

    /// An insertion entirely before a comment moves it by exactly what was inserted.
    ///
    /// The other half of the common case: text added above the comment slides it down, and
    /// slides it by the right amount. An off-by-one here is a comment that drifts one
    /// character per edit, which looks like nothing until it has drifted out of its sentence.
    #[test]
    fn an_insertion_before_a_comment_slides_it_by_that_much(
        len in 1usize..80,
        start in 1usize..80,
        extra in 0usize..40,
        added in 0usize..40,
    ) {
        let start = start.min(len).max(1);
        let end = (start + extra).min(len);
        let position = start - 1; // strictly before, so neither sticky rule is in play
        let (new_start, new_end) = comment_anchor::shift_range(start, end, position, 0, added);
        prop_assert_eq!((new_start, new_end), (start + added, end + added));
    }

    /// Which block an offset is in, and what that block spans, agree with each other.
    ///
    /// `block_of` is the paragraph comment's fallback and `block_extent` is what it then
    /// covers, so a disagreement between them is a paragraph comment bracketing the wrong
    /// paragraph. The end deliberately excludes the block separator: including it would draw
    /// the bracket one line below the paragraph it marks.
    #[test]
    fn a_block_contains_the_offsets_that_resolve_to_it(text in text_strategy()) {
        let blocks = block_starts_of(&text);
        let n = text.chars().count();
        for offset in 0..=n {
            let block = comment_anchor::block_of(&blocks, offset);
            prop_assert!(block < blocks.len(), "block {block} does not exist");
            let (s, e) = comment_anchor::block_extent(&blocks, n, block, block);
            prop_assert!(s <= e, "block {block} has an inverted extent {s}..{e}");
            prop_assert!(e <= n, "block {block} runs past the text");
            prop_assert!(
                blocks[block] <= offset,
                "offset {offset} was placed in block {block}, which starts later"
            );
            if let Some(next) = blocks.get(block + 1) {
                prop_assert!(
                    offset < *next,
                    "offset {offset} was placed in block {block}, but the next one starts at \
                     {next}"
                );
            }
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
// numbering and compile
// ───────────────────────────────────────────────────────────────────────────────

/// The sub-roles that matter to numbering and scope resolution, with a role that is legal
/// for each. Drawn from the constraint matrix rather than invented: a pair outside it cannot
/// occur, so generating one would only produce failures nobody can act on.
fn sub_role_strategy() -> impl Strategy<Value = (BinderItemRole, BinderItemSubRole)> {
    prop::sample::select(vec![
        (BinderItemRole::Item, BinderItemSubRole::BookBegin),
        (BinderItemRole::Item, BinderItemSubRole::BookEnd),
        (BinderItemRole::Item, BinderItemSubRole::Part),
        (BinderItemRole::Item, BinderItemSubRole::ChapterScene),
        (BinderItemRole::Item, BinderItemSubRole::Scene),
        (BinderItemRole::Item, BinderItemSubRole::Note),
        (BinderItemRole::Item, BinderItemSubRole::Paratext),
        (BinderItemRole::Folder, BinderItemSubRole::Book),
        (BinderItemRole::Folder, BinderItemSubRole::Part),
        (BinderItemRole::Folder, BinderItemSubRole::ChapterScene),
        (BinderItemRole::Folder, BinderItemSubRole::Note),
        (BinderItemRole::Folder, BinderItemSubRole::None),
    ])
}

fn items_strategy() -> impl Strategy<Value = Vec<ItemMeta>> {
    prop::collection::vec(
        (
            sub_role_strategy(),
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
            0i32..3,
        ),
        1..=16,
    )
    .prop_map(|rows| {
        rows.into_iter()
            .enumerate()
            .map(
                |(i, ((role, sub_role), activated, is_exportable, excluded, indent))| ItemMeta {
                    id: i as u64 + 1,
                    binder_id: 1,
                    role,
                    sub_role,
                    indent,
                    activated,
                    is_exportable,
                    exclude_from_numbering: excluded,
                },
            )
            .collect()
    })
}

proptest! {
    /// A row's number does not depend on what an export happened to select.
    ///
    /// The module exists because this was once false. A running counter inside the render
    /// loop only ever saw the rows one export had already been filtered down to, so marking a
    /// chapter non-exportable renumbered every later chapter in "Export Book" while "Export
    /// Chapter 7", which replayed the unfiltered stream, disagreed — same manuscript, same
    /// chapter, two numbers. `number_map` never sees a selection, and this is that sentence
    /// made checkable: dropping rows the map already refuses to number cannot change any
    /// number it gave.
    #[test]
    fn numbers_do_not_depend_on_what_an_export_selects(
        items in items_strategy(),
        part_resets_chapter in any::<bool>(),
    ) {
        let rules = NumberingRules { part_resets_chapter };
        let full = number_map(&items, rules);

        // Rows the map declined to number are exactly the rows a numbering pass must be able
        // to ignore. Removing them must leave every surviving row's number alone.
        let pruned: Vec<ItemMeta> = items
            .iter()
            .filter(|i| full.contains_key(&i.id) || level_of(&i.sub_role).is_none())
            .cloned()
            .collect();
        let after = number_map(&pruned, rules);

        for (id, numbered) in &full {
            prop_assert_eq!(
                after.get(id),
                Some(numbered),
                "row {} was renumbered by dropping rows it does not depend on",
                id
            );
        }
    }

    /// Numbers run upward through the stream, one level at a time.
    ///
    /// Two claims that together say the counters are counters: a level's ordinal never goes
    /// backwards as the stream advances except where a wider level restarts it, and a
    /// numbered row's own ordinal is at least one. Zero would print "Chapter 0".
    #[test]
    fn numbers_ascend_and_start_at_one(
        items in items_strategy(),
        part_resets_chapter in any::<bool>(),
    ) {
        let rules = NumberingRules { part_resets_chapter };
        let map = number_map(&items, rules);

        let mut last_book = 0usize;
        for item in &items {
            let Some(n) = map.get(&item.id) else { continue };
            prop_assert!(n.number() >= 1, "row {} carries ordinal {}", item.id, n.number());
            prop_assert!(n.book >= last_book, "the book counter went backwards");
            last_book = n.book;
            if n.level == StreamLevel::Book {
                prop_assert_eq!(n.number(), n.book);
            }
        }
    }

    /// A numbering-excluded row holds no number and does not consume one.
    ///
    /// The prologue lever, and the difference between it and the export toggle. A prologue is
    /// in the book, printed and word-counted, and uncounted — so the chapter after it is
    /// chapter one. A row that held no number but still bumped the counter would renumber the
    /// whole manuscript by one and look entirely plausible doing it.
    #[test]
    fn an_excluded_row_neither_holds_nor_consumes_a_number(
        items in items_strategy(),
        part_resets_chapter in any::<bool>(),
    ) {
        let rules = NumberingRules { part_resets_chapter };
        let map = number_map(&items, rules);
        for item in &items {
            if item.exclude_from_numbering {
                prop_assert!(
                    !map.contains_key(&item.id),
                    "excluded row {} was numbered anyway",
                    item.id
                );
            }
        }

        // Dropping the excluded rows entirely must leave every other number unchanged, which
        // is what "does not consume one" means.
        let without: Vec<ItemMeta> = items
            .iter()
            .filter(|i| !i.exclude_from_numbering)
            .cloned()
            .collect();
        let after = number_map(&without, rules);
        for (id, numbered) in &map {
            prop_assert_eq!(
                after.get(id),
                Some(numbered),
                "row {} changed number when an excluded row was removed",
                id
            );
        }
    }

    /// A resolved scope names real rows of the stream, each once, in stream order.
    ///
    /// `resolve_scope` answers with ids, and everything downstream treats that answer as a
    /// window onto the manuscript. A duplicate id exports a scene twice; an id from outside
    /// the stream is a lookup that fails at render time, well past the point where anything
    /// can be done about it.
    #[test]
    fn a_resolved_scope_is_a_real_ordered_part_of_the_stream(
        items in items_strategy(),
        focused in 0usize..16,
        scope in prop_oneof![
            Just(ScopeKind::Book),
            Just(ScopeKind::Part),
            Just(ScopeKind::Chapter),
            Just(ScopeKind::Scene),
            Just(ScopeKind::Note),
            Just(ScopeKind::Paratext),
            Just(ScopeKind::Folder),
        ],
    ) {
        let focused = focused % items.len();
        let Some(ids) = resolve_scope(&items, focused, scope) else {
            return Ok(());
        };

        let mut seen = std::collections::HashSet::new();
        let mut last_pos: Option<usize> = None;
        for id in &ids {
            let Some(pos) = items.iter().position(|i| i.id == *id) else {
                prop_assert!(false, "scope named row {id}, which is not in the stream");
                return Ok(());
            };
            prop_assert!(seen.insert(*id), "scope named row {id} twice");
            if let Some(prev) = last_pos {
                prop_assert!(prev < pos, "scope is out of stream order at row {id}");
            }
            last_pos = Some(pos);
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
// The two live text engines
// ───────────────────────────────────────────────────────────────────────────────

proptest! {
    /// The lexicon matcher never panics, and never claims more than the writer typed.
    ///
    /// It runs on the frame tick against a window cut out of live prose, in every script the
    /// app supports, so the input is genuinely arbitrary text and the only useful statement
    /// about it is a total one. `trigger_chars` is what the caller then *deletes* from the
    /// document, which is why a value larger than the window is not a wrong suggestion but a
    /// deletion of text the rule had nothing to do with.
    #[test]
    fn the_lexicon_matcher_is_total(
        window in r"[\PC]{0,24}",
        triggers in prop::collection::vec(r"[\PC]{0,6}", 0..4),
        locale in prop::sample::select(vec!["en-US", "fr-FR", "tr-TR", "az", "ja-JP"]),
    ) {
        let rows: Vec<TextReplacementRuleRow> = triggers
            .iter()
            .enumerate()
            .map(|(i, t)| TextReplacementRuleRow {
                id: i as u64 + 1,
                trigger: t.clone(),
                replacement: format!("<{t}>"),
                enabled: true,
            })
            .collect();
        let engine = TextReplacementEngine::from_rules_for_locale(&rows, locale);

        if let Some(fired) = engine.check(&window) {
            let chars: Vec<char> = window.chars().collect();
            prop_assert!(
                fired.trigger_chars < chars.len(),
                "a rule claimed {} of {} characters — the delimiter is not part of the trigger",
                fired.trigger_chars,
                chars.len()
            );
            let typed_end = chars.len() - 1;
            let typed: String = chars[typed_end - fired.trigger_chars..typed_end].iter().collect();
            prop_assert_eq!(
                fired.typed.as_str(),
                typed.as_str(),
                "the reported trigger is not the text it sits on"
            );
        }
    }

    /// Smart punctuation never panics, and never claims more than the writer typed.
    ///
    /// The same charter as the lexicon matcher, for the engine that converts quotes, dashes
    /// and spacing as the writer types. Both run inside the frame tick over live prose; a
    /// panic there takes the window down mid-sentence.
    #[test]
    fn smart_punctuation_is_total(
        before in r"[\PC]{0,24}",
        locale in prop::sample::select(vec!["en-US", "fr-FR", "de-DE", "ar", "ja-JP", "es-ES"]),
    ) {
        let engine = TypographyEngine::new(locale, SmartPunctuationFlags::default());
        let n = before.chars().count();
        if let Some(fired) = engine.check(&before) {
            prop_assert!(
                fired.replace_chars <= n,
                "a rule claimed {} of {n} characters",
                fired.replace_chars
            );
            prop_assert!(
                fired.typed.chars().count() <= n,
                "a rule reported more removed text than the window holds"
            );
        }
        // The paragraph-scoped half takes a whole block rather than a window; it must be just
        // as total.
        let _ = engine.check_paragraph(&before);
    }
}

// ───────────────────────────────────────────────────────────────────────────────
// scene_break and casing
// ───────────────────────────────────────────────────────────────────────────────

proptest! {
    /// The mark the app writes is the mark the app recognises.
    ///
    /// Two entry points read the same vocabulary because consumers hold different bytes:
    /// prose is stored as Djot, where a typed `* * *` persists escaped, while a live editor
    /// buffer has been through the parser and reads it bare. A writer and a reader that
    /// disagreed would produce a break that is invisible to the word count, or one that
    /// survives an export it should not.
    #[test]
    fn a_canonical_scene_break_is_recognised_by_both_readers(
        tier in prop_oneof![Just(SceneBreakTier::Minor), Just(SceneBreakTier::Major)]
    ) {
        prop_assert_eq!(
            scene_break::tier_of_djot_block(scene_break::canonical_djot(tier)),
            Some(tier),
            "the Djot form of {:?} is not recognised as one",
            tier
        );
        prop_assert_eq!(
            scene_break::tier_of_plain_line(scene_break::canonical_plain(tier)),
            Some(tier),
            "the plain form of {:?} is not recognised as one",
            tier
        );
    }

    /// Stripping markers removes markers and nothing else.
    ///
    /// The strip feeds the word count, including the persisted pace history where an error is
    /// permanent. Over-stripping silently lowers a writer's recorded output; under-stripping
    /// counts furniture as prose. Checked by stripping twice: whatever the first pass leaves
    /// must contain no marker for the second to find.
    #[test]
    fn stripping_markers_is_idempotent(
        blocks in prop::collection::vec(
            prop_oneof![
                Just("\\* \\* \\*".to_string()),
                Just("\\# # #".to_string()),
                Just("Ordinary prose.".to_string()),
                Just(String::new()),
                Just("A line with * a star in it.".to_string()),
            ],
            0..8,
        )
    ) {
        let djot = blocks.join("\n\n");
        let once = scene_break::strip_markers_djot(&djot).into_owned();
        let twice = scene_break::strip_markers_djot(&once).into_owned();
        prop_assert_eq!(&twice, &once, "a second strip removed more");
        for block in once.split("\n\n") {
            prop_assert!(
                scene_break::tier_of_djot_block(block).is_none(),
                "a marker survived the strip: {block:?}"
            );
        }
    }

    /// Folding a string for comparison is idempotent, and the Turkish tailoring is applied
    /// only where it belongs.
    ///
    /// `fold_key` is the key a lexicon rule is looked up by, so a fold that differed between
    /// the moment a rule was stored and the moment it is matched is a rule that never fires.
    /// The dotted-I pair is the reason the module exists: under the default mappings `İ`
    /// lowercases to two code points, so a case-insensitive comparison against `i` fails even
    /// though a Turkish reader considers them the same letter.
    #[test]
    fn folding_is_idempotent(
        s in r"[\PC]{0,20}",
        locale in prop::sample::select(vec!["en-US", "tr-TR", "az-Latn-AZ", "trv", "fr-FR"]),
    ) {
        let once = crate::casing::fold_key(&s, locale);
        let twice = crate::casing::fold_key(&once, locale);
        prop_assert_eq!(&twice, &once, "folding twice differs from folding once");
    }

    /// In Turkish and Azeri the two I's stay distinct letters; everywhere else they do not.
    ///
    /// Stated over the locale tag rather than over the text, because the tag is where this
    /// has gone wrong before: `trv` (Taroko) is not Turkish, and a prefix test catches it.
    #[test]
    fn the_dotted_i_tailoring_follows_the_language_subtag(
        locale in prop::sample::select(vec!["tr", "tr-TR", "az", "az-Latn-AZ", "trv", "en", "en-US"])
    ) {
        let turkish = matches!(locale, "tr" | "tr-TR" | "az" | "az-Latn-AZ");
        prop_assert_eq!(crate::casing::uses_dotted_i(locale), turkish);
        if turkish {
            prop_assert_eq!(crate::casing::to_uppercase("i", locale), "\u{130}");
            prop_assert_eq!(crate::casing::to_lowercase("I", locale), "\u{131}");
        } else {
            prop_assert_eq!(crate::casing::to_uppercase("i", locale), "I");
            prop_assert_eq!(crate::casing::to_lowercase("I", locale), "i");
        }
    }
}
