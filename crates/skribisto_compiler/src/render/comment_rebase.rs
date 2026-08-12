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
pub(crate) struct Window {
    /// First character of this Content's prose in the compiled text — also where a round-trip
    /// row mark is anchored, so the returning file says which `BinderItem` this passage was.
    pub lo: usize,
    pub hi: usize,
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
        let end = starts
            .get(i + 1)
            .copied()
            .unwrap_or(chars.len())
            .min(chars.len());
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
fn locate(
    compiled: &[char],
    compiled_starts: &[usize],
    own_djot: &str,
    cursor: usize,
) -> Option<Window> {
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
#[cfg(test)]
pub(crate) fn place_comments(
    compiled_text: &str,
    compiled_starts: &[usize],
    emitted: &[EmittedContent],
    comments: &[CommentToPlace],
) -> Vec<CommentPlacement> {
    let windows = locate_windows(compiled_text, compiled_starts, emitted);
    place_comments_in(compiled_text, compiled_starts, &windows, comments)
}

/// Where each emitted `Content`'s prose landed in the compiled document.
///
/// Split out from [`place_comments`] because the export needs the same answer twice and must
/// not compute it twice: a comment is rebased into its row's window, and a **round-trip row
/// mark** is anchored at that window's start. Two independent scans could disagree — the search
/// is heuristic and cursor-dependent — and a mark that disagreed with its own row's comments
/// would put the two in different places in the same file.
///
/// Walked in emission order so the cursor only moves forward.
pub(crate) fn locate_windows(
    compiled_text: &str,
    compiled_starts: &[usize],
    emitted: &[EmittedContent],
) -> HashMap<EntityId, Window> {
    let compiled: Vec<char> = compiled_text.chars().collect();
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
    windows
}

/// Rebase every comment into the window its own `Content` occupies.
pub(crate) fn place_comments_in(
    compiled_text: &str,
    compiled_starts: &[usize],
    windows: &HashMap<EntityId, Window>,
    comments: &[CommentToPlace],
) -> Vec<CommentPlacement> {
    let compiled: Vec<char> = compiled_text.chars().collect();
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
mod tests;
