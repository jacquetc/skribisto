// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Rebase a comment's row-local anchor onto the compiled export document.
//!
//! A `Comment` belongs to one `Content` row, and its anchor — `quote_prefix` / `quote_exact` /
//! `quote_suffix` plus a block ordinal — is expressed against **that row's** plain text. DOCX
//! and ODF comments are positional: `w:commentRangeStart`/`End` and
//! `office:annotation`/`annotation-end` bracket runs in one flat document body, and neither
//! format has any notion of "scene 3, offset 412". So a comment cannot be written at all until
//! its range is known in the *compiled* document's coordinates. That conversion is this module.
//!
//! # Why it is not `row_start + range_start`
//!
//! Four reasons, in increasing order of nastiness:
//!
//! 1. The compiled document is not the concatenation of the rows' stored prose. `assemble`
//!    interleaves headings, a title page, epigraphs and scene-break glyphs, and omits rows
//!    outside the export scope entirely.
//! 2. `assemble` works in Djot bytes; an anchor works in plain-text characters. `*salt-bleached*`
//!    is 15 bytes of Djot and 13 characters of text.
//! 3. The preset changes the text. Suppress chapter headings and every offset below shifts; a
//!    scene break renders as `* * *` under one preset and a blank line under another.
//! 4. `range_start` is a **hint** by design. The quote is the durable anchor precisely because
//!    offsets do not survive re-rendering — trusting the integer here would reintroduce the very
//!    failure the quote model exists to prevent.
//!
//! # The method
//!
//! Do not compute — **re-resolve**. `assemble` records every `Content` whose prose reached the
//! document, in emission order. For each, find the window `[lo, hi)` its prose occupies in the
//! compiled plain text using a **monotone forward cursor**, then run
//! [`comment_anchor::resolve`] — the same three-tier matcher that re-anchors a comment on every
//! reopen and on every import — restricted to that window.
//!
//! The monotone cursor is what makes the result trustworthy rather than merely likely. A
//! sentence repeated verbatim in two scenes would match either one under a whole-document
//! search; because emission is document-ordered and the cursor never moves backward, the later
//! scene's comment cannot claim the earlier scene's text.
//!
//! Nothing here invents a matching rule. A comment that will not place comes back as an
//! [`Resolution::Orphan`] with the same typed reason the editor would show, so the caller can
//! warn about it rather than move it somewhere plausible — the failure this whole model exists
//! to avoid is "a comment that is mostly right", which is invisible until someone's note has
//! silently moved.

use std::collections::HashMap;

use common::entities::CommentOrphanReason;
use common::types::EntityId;
use skrib_format::djot_plain_text;
use skribisto_model::comment_anchor::{self, Anchor, Resolution};

use super::EmittedContent;

/// Where one `Content`'s prose landed in the compiled document, in character offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Window {
    lo: usize,
    hi: usize,
}

/// One comment, resolved against the compiled document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommentPlacement {
    /// The `Comment` row this is about.
    pub comment_id: EntityId,
    /// Its range in the compiled document's plain text, or why it has none.
    pub resolution: Resolution,
}

/// A comment as the caller holds it, reduced to what rebasing needs.
///
/// Deliberately not `common::entities::Comment`: this module has no business knowing about
/// authors, bodies, replies or resolved state, and taking the whole entity would let it grow
/// an opinion about them.
#[derive(Debug, Clone)]
pub(crate) struct CommentToPlace {
    pub comment_id: EntityId,
    /// The `Content` the comment is anchored to.
    pub content_id: EntityId,
    pub anchor: Anchor,
    /// Paragraph comments re-derive their extent from block boundaries rather than trusting a
    /// stored length, exactly as they do in the editor.
    pub is_paragraph: bool,
}

/// Find `needle` in `hay` at or after `from`, both as char slices.
fn find_from(hay: &[char], needle: &[char], from: usize) -> Option<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return None;
    }
    (from..=hay.len() - needle.len()).find(|&i| &hay[i..i + needle.len()] == needle)
}

/// One Content's own plain text, split into its blocks, each trimmed of separators.
fn blocks_of(text: &str, starts: &[usize]) -> Vec<Vec<char>> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    for (i, &s) in starts.iter().enumerate() {
        let s = s.min(chars.len());
        let end = starts.get(i + 1).copied().unwrap_or(chars.len()).min(chars.len());
        if end <= s {
            continue;
        }
        let block: Vec<char> = chars[s..end]
            .iter()
            .copied()
            .skip_while(|c| *c == '\n')
            .collect();
        if let Some(last) = block.iter().rposition(|c| *c != '\n') {
            out.push(block[..=last].to_vec());
        }
    }
    out
}

/// Locate one Content's prose in the compiled text, searching forward from `cursor`.
///
/// Block-wise on purpose. `push_prose` **transforms** text on the way in — a scene-break marker
/// becomes the preset's own rendering, or nothing at all — so a whole-row string comparison
/// fails on any scene containing a break.
///
/// # Two rules that look fussy and are load-bearing
///
/// **A match must land on a compiled block boundary.** A row's block is a block; finding its
/// text in the *middle* of some unrelated paragraph is a coincidence, not a location. Without
/// this, a short common block ("Yes.", "* * *") matches the first place those characters happen
/// to occur and drags the window somewhere else entirely.
///
/// **Extension stops at the first block that cannot be found.** The window then ends at the
/// last block located *contiguously*, and a later block that happens to match further on is
/// ignored. Skipping to it is what let a window swallow the rows in between: with the
/// `BlankLine` scene break — the default minor break of nearly every built-in preset — a marker
/// block renders to nothing, so the row's third block sits immediately after its first in the
/// compiled text, the window covered both, and a paragraph comment written over
/// paragraph-plus-marker resolved across the paragraph *after* it. A short window costs a
/// comment its placement, and it is reported; a long one silently brackets text the writer
/// never selected.
fn locate(compiled: &[char], compiled_starts: &[usize], own_djot: &str, cursor: usize) -> Option<Window> {
    let (own_text, own_starts) = djot_plain_text(own_djot).ok()?;
    let blocks = blocks_of(&own_text, &own_starts);

    // The first block that can be found at all opens the window — not necessarily block 0,
    // which may itself be a marker the preset rendered away.
    let mut it = blocks.iter();
    let (lo, mut hi) = loop {
        let b = it.next()?;
        if let Some(p) = find_on_block_boundary(compiled, compiled_starts, b, cursor) {
            break (p, p + b.len());
        }
    };

    for b in it {
        match find_on_block_boundary(compiled, compiled_starts, b, hi) {
            Some(p) => hi = p + b.len(),
            None => break,
        }
    }
    Some(Window { lo, hi })
}

/// [`find_from`], restricted to positions that begin a block of the compiled document.
fn find_on_block_boundary(
    hay: &[char],
    starts: &[usize],
    needle: &[char],
    from: usize,
) -> Option<usize> {
    let mut at = from;
    while let Some(p) = find_from(hay, needle, at) {
        if starts.binary_search(&p).is_ok() {
            return Some(p);
        }
        at = p + 1;
    }
    None
}

/// Re-resolve one anchor inside its window, returning compiled-document coordinates.
///
/// # Why the window is what keeps a paragraph comment honest
///
/// `Anchor::block_span` counts blocks of the row's **own** prose, and the compiled document
/// need not have the same ones — a scene-break marker renders to one block under a glyph
/// preset and to none under `BlankLine`. Counting `span` blocks forward in compiled
/// coordinates therefore indexes the wrong block as soon as the two diverge.
///
/// It is nonetheless safe to hand the span straight to [`comment_anchor::resolve`], because
/// its paragraph branch clamps the extent to the end of the **last block it was given** — and
/// the block list here is the window's, which [`locate`] bounds to the blocks of this row that
/// actually survived, contiguously. An over-long span can therefore reach the end of the row's
/// own prose and no further. It could once reach further, and did: the window itself used to
/// skip over a vanished block to a later match, so "the end of the window" was inside the
/// *next* paragraph. That is fixed in `locate`, and this function relies on it.
///
/// # A comment with no quote is orphaned, never pinned
///
/// A legacy `CommentAnchorKind::Document` row stores an empty quote and a zero-length range.
/// An empty needle matches the empty slice at offset 0, so without this guard such a row
/// resolves "successfully" to a zero-width range at the very start of its content and is
/// written into the exported file as a comment bracketing nothing. The UI already refuses to
/// seek those (`CommentRow::is_unplaced`); the exporter must refuse to place them.
fn rebase(
    compiled: &[char],
    compiled_starts: &[usize],
    win: Window,
    anchor: &Anchor,
    is_paragraph: bool,
) -> Resolution {
    if anchor.exact.trim().is_empty() {
        return Resolution::Orphan(CommentOrphanReason::TextNotFound);
    }

    let slice: String = compiled[win.lo..win.hi].iter().collect();
    // Block starts rebased into the window, and always beginning at 0: the window opens on a
    // block boundary by construction, but `compiled_starts` only lists it when the window
    // happens to start exactly at one, and `comment_anchor` needs a first block to count from.
    let mut local: Vec<usize> = compiled_starts
        .iter()
        .filter(|&&s| s > win.lo && s < win.hi)
        .map(|&s| s - win.lo)
        .collect();
    local.insert(0, 0);

    match comment_anchor::resolve(&slice, anchor, is_paragraph, &local) {
        Resolution::Anchored { start, length } => Resolution::Anchored {
            start: start + win.lo,
            length,
        },
        other => other,
    }
}

/// Rebase every comment onto the compiled document.
///
/// `compiled_text` and `compiled_starts` must be the **addressable** text and block starts of
/// the assembled document — the pair `skrib_format::djot_plain_text` returns, and the same
/// space an anchor was captured in. Pairing them with the human-readable export text instead is
/// the classic form of this bug: that string omits each table's `U+FFFC` anchor, and every
/// offset after a table lands two characters off in it.
///
/// Comments whose `content_id` is not in `emitted` are **absent from the result entirely**,
/// not orphaned. That distinction is the caller's whole warning story: a comment on a row
/// outside the export scope, or on a synopsis the preset omits, has not failed — it simply is
/// not in this document, and warning about it would cry wolf on every scoped export.
pub(crate) fn place_comments(
    compiled_text: &str,
    compiled_starts: &[usize],
    emitted: &[EmittedContent],
    comments: &[CommentToPlace],
) -> Vec<CommentPlacement> {
    let compiled: Vec<char> = compiled_text.chars().collect();

    // One window per emitted Content, walked in emission order so the cursor only moves
    // forward. Built once: a comment lookup must not re-scan, or two comments on the same row
    // could disagree about where that row is.
    let mut windows: HashMap<EntityId, Window> = HashMap::new();
    let mut cursor = 0usize;
    for e in emitted {
        if let Some(w) = locate(&compiled, compiled_starts, &e.djot, cursor) {
            // Past the END of the window just claimed, not its start. Emitted contents occupy
            // disjoint spans, so the next one begins after this one finishes — and advancing
            // only to `lo` lets the *next* content match this one's text all over again.
            // That is not hypothetical: a scene and its own synopsis frequently share a
            // sentence, and with `lo` both of their comments landed on the scene.
            cursor = w.hi;
            windows.insert(e.content_id, w);
        }
    }

    comments
        .iter()
        .filter_map(|c| {
            let win = windows.get(&c.content_id)?;
            Some(CommentPlacement {
                comment_id: c.comment_id,
                resolution: rebase(&compiled, compiled_starts, *win, &c.anchor, c.is_paragraph),
            })
        })
        .collect()
}

/// Whether a placement failed, and why — for the caller's export warning.
pub(crate) fn orphan_reason(p: &CommentPlacement) -> Option<CommentOrphanReason> {
    match &p.resolution {
        Resolution::Anchored { .. } => None,
        Resolution::Orphan(reason) => Some(reason.clone()),
    }
}

/// The stored `Comment`'s anchor, rebuilt as the matcher wants it.
///
/// `block_span` is the one field that is **not** stored: the editor re-derives it on every
/// load from the range against the live document's block boundaries, so there is nothing on
/// disk to read. It is re-derived here the same way, against the row's own prose — which is
/// what keeps a paragraph comment covering all of the paragraphs it covered when it was made,
/// rather than silently shrinking to the first one on the way out.
pub(crate) fn anchor_of(comment: &common::entities::Comment, row_djot: &str) -> Anchor {
    let start = comment.range_start as usize;
    let length = comment.range_length as usize;
    let block_span = match djot_plain_text(row_djot) {
        Ok((_, starts)) if length > 0 => {
            let first = comment_anchor::block_of(&starts, start);
            let last = comment_anchor::block_of(&starts, start + length - 1);
            last.saturating_sub(first) + 1
        }
        // No prose to measure against, or an empty range: one block, which is what a
        // freshly-made paragraph comment covers anyway.
        _ => 1,
    };
    Anchor {
        start,
        length,
        prefix: comment.quote_prefix.clone(),
        exact: comment.quote_exact.clone(),
        exact_truncated: comment.quote_exact_truncated,
        suffix: comment.quote_suffix.clone(),
        block_ordinal: comment.block_ordinal_hint as usize,
        block_span,
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::preset::builtin_presets;
    use common::entities::{Binder, BinderItem, BinderItemRole, BinderItemSubRole as SR, Work};
    use skrib_format::{BinderWithItems, ItemWithContents};

    fn c(id: u64, role: ContentRole, data: &str) -> Content {
        Content {
            id,
            activated: true,
            role,
            data: data.to_string(),
            ..Default::default()
        }
    }

    fn iwc(id: u64, sub_role: SR, contents: Vec<Content>) -> ItemWithContents {
        ItemWithContents {
            item: BinderItem {
                id,
                role: BinderItemRole::Item,
                sub_role,
                dict_language: language::parse_legacy_list("en"),
                is_exportable: true,
                activated: true,
                ..Default::default()
            },
            contents,
        }
    }

    fn gathered(items: Vec<ItemWithContents>) -> Gathered {
        Gathered {
            assets: Vec::new(),
            footnotes: Vec::new(),
            work: Work {
                id: 1,
                number_chapters: true,
                ..Default::default()
            },
            tags: vec![],
            dict_words: vec![],
            text_replacement_rules: vec![],
            note_templates: vec![],
            smart_punctuation: None,
            trash_infos: vec![],
            paces: vec![],
            progress_snapshots: vec![],
            comments: vec![],
            binders: vec![BinderWithItems {
                binder: Binder {
                    id: 10,
                    ..Default::default()
                },
                items,
            }],
            work_info: None,
        }
    }

    fn preset_of(id: &str) -> Preset {
        builtin_presets()
            .into_iter()
            .find(|p| p.id == id)
            .expect("preset")
    }

    fn req<'a>(g: &'a Gathered, include: &'a [u64], p: &'a Preset) -> RenderRequest<'a> {
        RenderRequest {
            media_dir: std::path::Path::new(""),
            gathered: g,
            include,
            preset: p,
            format: ExportFormat::Html,
            work_lang: "en",
            explicit_selection: false,
        }
    }

    /// Capture an anchor exactly as the editor does: against the row's own plain text.
    fn anchor_on(djot: &str, phrase: &str) -> Anchor {
        let (text, starts) = djot_plain_text(djot).expect("plain");
        let chars: Vec<char> = text.chars().collect();
        let needle: Vec<char> = phrase.chars().collect();
        let start = find_from(&chars, &needle, 0)
            .unwrap_or_else(|| panic!("fixture phrase {phrase:?} is not in {text:?}"));
        let ordinal = comment_anchor::block_of(&starts, start);
        comment_anchor::capture(&text, start, start + needle.len(), ordinal)
    }

    /// Assemble, then place — the whole pipeline, returning the compiled text alongside so a
    /// test can assert on the words a placement actually lands on.
    fn run(
        g: &Gathered,
        include: &[u64],
        p: &Preset,
        comments: &[CommentToPlace],
    ) -> (String, Vec<CommentPlacement>) {
        let built =
            assemble(&req(g, include, p), &|_| {}, &AtomicBool::new(false)).expect("assemble");
        // The ADDRESSABLE text, paired with the block starts that index it — never
        // `to_plain_text()`, which omits each table's U+FFFC anchor and would put every offset
        // after a table two characters out.
        let text = built.doc.to_addressable_text().expect("addressable");
        let starts: Vec<usize> = built
            .doc
            .blocks()
            .into_iter()
            .map(|b| b.position())
            .collect();
        let placed = place_comments(&text, &starts, &built.emitted, comments);
        (text, placed)
    }

    fn words_at(text: &str, p: &CommentPlacement) -> String {
        match p.resolution {
            Resolution::Anchored { start, length } => {
                text.chars().skip(start).take(length).collect()
            }
            Resolution::Orphan(ref r) => format!("<orphan: {r:?}>"),
        }
    }

    const S1: &str = "The wind rose over the hills and the salt-bleached door rattled all night.";
    const S2: &str = "She counted the lamps again. The harbour was quiet, and the boats were still.";
    const S3: &str = "The wind rose over the hills once more, but nobody was left to hear it.";

    fn three_scene_book() -> Gathered {
        gathered(vec![
            iwc(
                100,
                SR::BookBegin,
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, S1),
                ],
            ),
            iwc(102, SR::Scene, vec![c(4, ContentRole::SceneText, S2)]),
            iwc(
                103,
                SR::ChapterScene,
                vec![
                    c(5, ContentRole::ChapterTitle, "After"),
                    c(6, ContentRole::SceneText, S3),
                ],
            ),
        ])
    }

    fn to_place(comment_id: u64, content_id: u64, djot: &str, phrase: &str) -> CommentToPlace {
        CommentToPlace {
            comment_id,
            content_id,
            anchor: anchor_on(djot, phrase),
            is_paragraph: false,
        }
    }

    #[test]
    fn a_comment_in_each_scene_rebases_onto_the_right_words() {
        let g = three_scene_book();
        let p = preset_of("neutral");
        let include = [100u64, 101, 102, 103];
        let comments = vec![
            to_place(1, 3, S1, "salt-bleached door"),
            to_place(2, 4, S2, "the boats were still"),
            to_place(3, 6, S3, "nobody was left"),
        ];

        let (text, placed) = run(&g, &include, &p, &comments);
        assert_eq!(placed.len(), 3, "every comment's row is in the export");
        for (want, got) in ["salt-bleached door", "the boats were still", "nobody was left"]
            .iter()
            .zip(&placed)
        {
            assert_eq!(&words_at(&text, got), want, "placement {got:?}");
        }
    }

    /// The case that defeats a whole-document quote search: S1 and S3 open with the *same*
    /// sentence. Only the monotone window can tell them apart, and getting it wrong means one
    /// scene's note silently moves to another scene.
    #[test]
    fn a_sentence_repeated_in_two_scenes_lands_in_the_right_one() {
        let g = three_scene_book();
        let p = preset_of("neutral");
        let include = [100u64, 101, 102, 103];
        let comments = vec![
            to_place(1, 3, S1, "The wind rose over the hills"),
            to_place(2, 6, S3, "The wind rose over the hills"),
        ];

        let (_, placed) = run(&g, &include, &p, &comments);
        let starts: Vec<usize> = placed
            .iter()
            .map(|p| match p.resolution {
                Resolution::Anchored { start, .. } => start,
                ref o => panic!("did not anchor: {o:?}"),
            })
            .collect();
        assert!(
            starts[0] < starts[1],
            "the third scene's comment must land later in the compiled document than the \
             first's — got {starts:?}. Equal or reversed means the window failed to \
             disambiguate and a note has moved to another scene."
        );
    }

    /// `push_prose` rewrites a scene-break marker into the preset's own rendering, so a row's
    /// compiled text is not character-identical to its stored text. The window search has to
    /// tolerate that.
    #[test]
    fn a_comment_after_a_scene_break_still_rebases() {
        let with_break = "Before the storm broke.\n\n* * *\n\nAfter it had passed for good.";
        let g = gathered(vec![
            iwc(
                100,
                SR::BookBegin,
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, with_break),
                ],
            ),
        ]);
        let p = preset_of("neutral");
        let include = [100u64, 101];
        let comments = vec![to_place(1, 3, with_break, "After it had passed")];

        let (text, placed) = run(&g, &include, &p, &comments);
        assert_eq!(words_at(&text, &placed[0]), "After it had passed");
    }

    /// A comment on a row outside the export scope is **absent**, not orphaned. Warning about
    /// it would cry wolf on every "Export Chapter 5".
    #[test]
    fn a_comment_on_an_out_of_scope_row_is_absent_rather_than_orphaned() {
        let g = three_scene_book();
        let p = preset_of("neutral");
        // Chapter 3 (item 103, content 6) is excluded from the scope.
        let include = [100u64, 101, 102];
        let comments = vec![
            to_place(1, 3, S1, "salt-bleached door"),
            to_place(2, 6, S3, "nobody was left"),
        ];

        let (_, placed) = run(&g, &include, &p, &comments);
        assert_eq!(
            placed.len(),
            1,
            "only the in-scope comment is reported at all: {placed:?}"
        );
        assert_eq!(placed[0].comment_id, 1);
        assert!(orphan_reason(&placed[0]).is_none());
    }

    /// A comment whose quoted words are gone comes back as a typed orphan, never as a
    /// plausible-looking guess.
    #[test]
    fn a_comment_whose_text_is_gone_reports_a_typed_orphan() {
        let g = three_scene_book();
        let p = preset_of("neutral");
        let include = [100u64, 101, 102, 103];
        let mut c1 = to_place(1, 3, S1, "salt-bleached door");
        c1.anchor.exact = "a sentence that was deleted entirely".into();
        c1.anchor.prefix = String::new();
        c1.anchor.suffix = String::new();

        let (_, placed) = run(&g, &include, &p, &[c1]);
        assert_eq!(placed.len(), 1, "the row IS in scope, so it must be reported");
        assert_eq!(
            orphan_reason(&placed[0]),
            Some(CommentOrphanReason::TextNotFound),
            "got {:?}",
            placed[0].resolution
        );
    }

    /// A synopsis is its own window. Two remarks quoting the same sentence — one on the scene,
    /// one on the summary of it — must not be able to claim each other's position.
    #[test]
    fn a_synopsis_comment_is_placed_in_the_synopsis_not_the_scene() {
        let shared = "The lamp guttered in the hallway.";
        let g = gathered(vec![
            iwc(
                100,
                SR::BookBegin,
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, shared),
                    c(4, ContentRole::SynopsisText, shared),
                ],
            ),
        ]);
        let mut p = preset_of("neutral");
        p.include_synopses = true;
        let include = [100u64, 101];
        let comments = vec![
            to_place(1, 3, shared, "lamp guttered"),
            to_place(2, 4, shared, "lamp guttered"),
        ];

        let (_, placed) = run(&g, &include, &p, &comments);
        assert_eq!(placed.len(), 2);
        let (a, b) = (&placed[0].resolution, &placed[1].resolution);
        match (a, b) {
            (Resolution::Anchored { start: sa, .. }, Resolution::Anchored { start: sb, .. }) => {
                assert_ne!(
                    sa, sb,
                    "the scene's comment and the synopsis's must land at different offsets"
                )
            }
            _ => panic!("both should anchor: {a:?} {b:?}"),
        }
    }

    /// **Regression.** A paragraph comment covering a paragraph *and* the scene-break marker
    /// after it must never widen to the paragraph beyond.
    ///
    /// Under `SceneBreak::BlankLine` — the minor break nearly every built-in preset uses — the
    /// marker renders to no characters at all, so the row's third block sits immediately after
    /// its first in the compiled document. Deriving the extent from the stored `block_span`
    /// then counted one block too far and bracketed text the writer never selected, silently:
    /// no orphan, and `comments_written` reported it as a success.
    #[test]
    fn a_paragraph_comment_does_not_swallow_the_paragraph_after_a_vanished_scene_break() {
        let with_break = "Before the storm broke.\n\n\\* \\* \\*\n\nAfter it had passed for good.";
        let g = gathered(vec![
            iwc(
                100,
                SR::BookBegin,
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, with_break),
                ],
            ),
        ]);
        let p = preset_of("neutral");
        let include = [100u64, 101];

        // Captured as the editor captures a selection covering the first paragraph and the
        // marker's own block — but NOT the paragraph after it.
        let (row_text, row_starts) = djot_plain_text(with_break).expect("plain");
        let end = row_starts.get(2).copied().unwrap_or(row_text.chars().count());
        let mut anchor = comment_anchor::capture(&row_text, 0, end.saturating_sub(1), 0);
        anchor.block_span = 2;

        let comments = vec![CommentToPlace {
            comment_id: 1,
            content_id: 3,
            anchor,
            is_paragraph: true,
        }];

        let (text, placed) = run(&g, &include, &p, &comments);
        let got = words_at(&text, &placed[0]);
        assert!(
            !got.contains("After it had passed"),
            "the comment must not reach the paragraph after the break — got {got:?}"
        );
        assert!(
            got.starts_with("Before the storm broke."),
            "…and must still cover the paragraph it was made on — got {got:?}"
        );
    }

    /// **Regression.** A comment with no quote at all — the shape a legacy
    /// `CommentAnchorKind::Document` row stores — must orphan, not resolve to a zero-width
    /// range at offset 0.
    ///
    /// An empty needle matches the empty slice anywhere, so without an explicit guard this
    /// "succeeded" and was written into the exported file as a comment bracketing nothing,
    /// pinned to the start of its row.
    #[test]
    fn a_comment_with_no_quote_orphans_instead_of_pinning_itself_at_the_start() {
        let g = three_scene_book();
        let p = preset_of("neutral");
        let include = [100u64, 101, 102, 103];
        let comments = vec![CommentToPlace {
            comment_id: 1,
            content_id: 3,
            anchor: Anchor::default(),
            is_paragraph: false,
        }];

        let (_, placed) = run(&g, &include, &p, &comments);
        assert_eq!(placed.len(), 1, "the row is in scope, so it must be reported");
        assert_eq!(
            orphan_reason(&placed[0]),
            Some(CommentOrphanReason::TextNotFound),
            "got {:?}",
            placed[0].resolution
        );
    }

    /// **Regression.** A row's block must be located at a compiled *block boundary*, never
    /// mid-paragraph.
    ///
    /// A short, common block matched wherever those characters first occurred, which could drag
    /// the window into an unrelated row and take every later row's cursor with it.
    #[test]
    fn a_block_is_never_located_in_the_middle_of_another_paragraph() {
        // Scene 1 contains the words "Yes." inside a longer sentence; scene 2 IS "Yes.".
        let s1 = "He asked whether it was true. Yes. she said, and turned away.";
        let s2 = "Yes.";
        let g = gathered(vec![
            iwc(
                100,
                SR::BookBegin,
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(101, SR::Scene, vec![c(3, ContentRole::SceneText, s1)]),
            iwc(102, SR::Scene, vec![c(4, ContentRole::SceneText, s2)]),
        ]);
        let p = preset_of("neutral");
        let include = [100u64, 101, 102];
        let comments = vec![to_place(1, 4, s2, "Yes.")];

        let (text, placed) = run(&g, &include, &p, &comments);
        assert_eq!(placed.len(), 1);
        let start = match placed[0].resolution {
            Resolution::Anchored { start, .. } => start,
            ref o => panic!("did not anchor: {o:?}"),
        };
        let before: String = text.chars().take(start).collect();
        assert!(
            before.contains("turned away"),
            "scene 2's comment landed inside scene 1's sentence instead of on scene 2 — \
             text before it was {before:?}"
        );
    }

    /// With synopses switched off, a comment on one is out of scope — absent, not orphaned.
    #[test]
    fn a_synopsis_comment_is_absent_when_the_preset_omits_synopses() {
        let shared = "The lamp guttered in the hallway.";
        let g = gathered(vec![
            iwc(
                100,
                SR::BookBegin,
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, shared),
                    c(4, ContentRole::SynopsisText, shared),
                ],
            ),
        ]);
        let mut p = preset_of("neutral");
        p.include_synopses = false;
        let include = [100u64, 101];
        let comments = vec![to_place(2, 4, shared, "lamp guttered")];

        let (_, placed) = run(&g, &include, &p, &comments);
        assert!(
            placed.is_empty(),
            "a synopsis the preset omits is not in the document, so its comment has not \
             failed — it is simply not here: {placed:?}"
        );
    }
}
