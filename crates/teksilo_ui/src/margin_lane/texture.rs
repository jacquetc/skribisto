// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The texture column: **one bar per paragraph**, its length the word count and
//! its filled part the share of it a reader hears as speech.
//!
//! A reader hears dialogue as a shape down the page, a ladder of speech against a
//! wall of narration, and that shape is the one thing about a scene's rhythm that
//! survives being shrunk to twenty-eight pixels. It is not a minimap: nothing here
//! pictures the text, and a bar says only how long a paragraph is and how much of
//! it is spoken.
//!
//! ## Why this is not a provider
//!
//! Every other source of lane content goes through
//! [`register_lane_provider`](super::register_lane_provider), and this deliberately
//! does not. The texture is **one physical column** — [`MarginLane::texture`] takes
//! exactly one `Vec<LaneBar>`, sized by one width — where the mark columns exist so
//! that several sources can share them without collision. A registry whose slot
//! admits exactly one occupant is not a registry; it is a field, and pretending
//! otherwise would put a row on the settings page with a swatch and a shape it
//! never draws. The writer's own model of it agrees: the settings page carries the
//! texture as its own switch under its own heading, not as one of the things the
//! lane *marks*.
//!
//! If a second texture source is ever wanted, adding an optional texture function
//! to [`LaneProviderSpec`](super::LaneProviderSpec) is exactly as cheap then as it
//! would be now, and would have a real second occupant to justify it.
//!
//! ## Measured per block, not per line
//!
//! The paragraph split [`prose_stats::measure`] performs works on a flattened
//! string and cannot see a block boundary, so a fenced code block — one live block
//! whose content legitimately contains blank lines — comes out as several
//! paragraphs that were never paragraphs. Here the document's own blocks *are* the
//! units, which is both more correct and the only way to get each paragraph's
//! character offset, which is what positions its bar.
//!
//! [`MarginLane::texture`]: crate::widgets::MarginLane::texture

use std::cell::Cell;

use crate::widgets::{LaneBar, LaneSpan};
use common::types::EntityId;
use skribisto_model::analysis::prose_stats::{self, DialogueMarkers, ParagraphStats};
use teksilo::text_document::{FlowElementSnapshot, TextDocument};

/// Shortest bar the texture will draw, in lane pixels.
///
/// Below this a bar is a line of aliasing rather than a reading, so consecutive
/// paragraphs are merged until one clears it. Two rather than three (the mark
/// minimum): a bar is a horizontal rule whose *length* carries the meaning, and it
/// stays legible at a height a square would not.
pub const MIN_BAR_HEIGHT: f32 = 2.0;

/// The gap left under every bar, in lane pixels.
///
/// **This is what makes the column a ladder rather than a slab.** Bars that meet
/// edge to edge fuse into one grey mass the moment two neighbours are a similar
/// length, and the whole point of the texture is the shape down the page — where
/// speech clusters and where it does not. One pixel is enough to separate them and
/// little enough that a bar keeps its length.
///
/// Taken off the bottom, never off the top, so a bar still begins exactly where its
/// paragraph does and stays in agreement with the marks beside it.
pub const BAR_GAP: f32 = 1.0;

/// One paragraph, measured and placed.
///
/// Public, and the reason is a **stream**. A bar's length is a fraction of the
/// longest paragraph, and "longest" has to mean the longest in the *book* rather
/// than the longest in each scene — otherwise a two-paragraph note draws the same
/// full-length bar a chapter does, and the column stops meaning anything down the
/// page. So the measurement and the scaling are separate steps: a caller collects
/// units from every row it maps, then scales them together.
///
/// Kept as its own type rather than going straight to [`LaneBar`] so the merge pass
/// has the word counts to weight by. A merged bar reporting the *mean* of its
/// members' dialogue shares would be wrong wherever the members differ in length,
/// which is exactly where merging happens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Paragraph {
    pub span: LaneSpan,
    pub stats: ParagraphStats,
}

/// The texture for one document.
///
/// `lane_height` is the strip's height in pixels, used only to decide how much
/// merging a paragraph needs to stay visible. Pass `0.0` before the lane has been
/// laid out and nothing is merged, which is right: there is no pixel budget to
/// measure against yet.
///
/// Empty when the document has no words, when nothing can be located yet, or when
/// dialogue is not measurable in this language — the last on purpose. An
/// unmeasurable language is not a language with no dialogue, and drawing every bar
/// with an empty fill would state a zero this application's own rules forbid.
pub fn bars(
    doc: &TextDocument,
    markers: DialogueMarkers,
    locate: &dyn Fn(usize, usize) -> Option<LaneSpan>,
    lane_height: f32,
) -> Vec<LaneBar> {
    // A scale of its own: one document is the whole manuscript here, so there is no
    // other row whose arrival could move it.
    bars_from(units(doc, markers, locate), lane_height, &Cell::new(0))
}

/// The measured paragraphs of one document, placed on the lane but **not yet
/// scaled**.
///
/// The half a stream calls per row before scaling them all together.
pub fn units(
    doc: &TextDocument,
    markers: DialogueMarkers,
    locate: &dyn Fn(usize, usize) -> Option<LaneSpan>,
) -> Vec<Paragraph> {
    if !markers.is_measurable() {
        return Vec::new();
    }

    // One lock, one walk. `blocks()` would hand back handles whose `text()` and
    // `position()` each re-lock the document and re-seek the rope, so a
    // thousand-block Book would pay two thousand lock acquisitions to learn what
    // this single call already computed on its way past.
    doc.snapshot_flow()
        .elements
        .iter()
        .filter_map(|element| {
            let FlowElementSnapshot::Block(block) = element else {
                return None;
            };
            let stats = prose_stats::measure_paragraph(&block.text, markers);
            if stats.words == 0 {
                return None;
            }
            // **The paragraph's own extent**, top of its first line to bottom of its
            // last — not two point positions. Locating the two ends separately gives
            // the *middle* of each line, so a paragraph occupying one line came out
            // as a zero-height bar and was then merged away for being too short to
            // see. A span asks the question the bar is actually about.
            Some(Paragraph {
                span: locate(block.position, block.position + block.length)?,
                stats,
            })
        })
        .collect()
}

/// Merge and scale measured paragraphs into the bars the lane draws.
///
/// `lane_height` is the strip's height in pixels, used only to decide how much
/// merging a paragraph needs to stay visible. Pass `0.0` before the lane has been
/// laid out and nothing is merged, which is right: there is no pixel budget to
/// measure against yet.
///
/// Empty when there are no words to scale against. Note what is *not* checked here:
/// whether dialogue is measurable at all. [`units`] answers that, and answers it
/// with nothing — an unmeasurable language is not a language with no dialogue, and
/// a column of empty bars would state a zero this application's own rules forbid.
pub fn bars_from(
    paragraphs: Vec<Paragraph>,
    lane_height: f32,
    scale: &Cell<usize>,
) -> Vec<LaneBar> {
    let merged = merge_below(paragraphs, lane_height);

    // The longest bar is measured **after** merging, not before: a merged bar's word
    // count is the sum of its members', so a longest taken from the unmerged list
    // lets a bar report an extent above 1.0 — a length past the end of the column it
    // is drawn in.
    let here = merged.iter().map(|p| p.stats.words).max().unwrap_or(0);

    // **And it only ever grows.** This was the bug a writer actually reported, and it
    // is not what it looked like: scrolling a Full Book made the whole texture
    // column redraw, bars a mile from the scene they had reached changing length.
    //
    // The cause is that `merged` holds only the rows that could be *placed* — a row
    // below the fold has no geometry, so it contributes nothing — and that set
    // changes with every scroll. Dividing by its longest paragraph meant dividing by
    // a number that wandered up and down as rows arrived and were unbuilt, and every
    // bar in the book moved with it. Measured: bars four fifths of a strip away from
    // the scene being read went from 17 px to 20 px on nothing but a scroll.
    //
    // A monotone maximum fixes it without giving up the thing the scale is *for*:
    // it is still the longest paragraph in this manuscript, still nothing to do with
    // any other book, and it simply stops being re-litigated by whichever rows
    // happen to be on screen. It settles after one pass through and never moves
    // again.
    //
    // The alternative — normalising each row against its own longest — is stable too
    // and says something false: a two-paragraph note would draw the same full-length
    // bar a chapter does, and the column would stop meaning anything down the page.
    if here > scale.get() {
        scale.set(here);
    }
    let longest = scale.get();
    if longest == 0 {
        return Vec::new();
    }

    // The gap, in fractions of the lane. Zero before the lane is laid out, which is
    // right: there is no pixel budget to take one from.
    let gap = if lane_height > 0.0 {
        BAR_GAP / lane_height
    } else {
        0.0
    };
    let floor = if lane_height > 0.0 {
        MIN_BAR_HEIGHT / lane_height
    } else {
        0.0
    };

    merged
        .into_iter()
        .map(|p| LaneBar {
            // Separated, and never below the height that makes one visible: a bar
            // shortened past the minimum in the name of a gap would be a gap.
            span: LaneSpan::new(
                p.span.start,
                (p.span.end - gap).max(p.span.start + floor).min(p.span.end),
            ),
            // Against the longest paragraph in *this* manuscript. Every measure this
            // application makes is relative to the manuscript it is measuring, and a
            // bar scaled to some absolute idea of a long paragraph would be a claim
            // about writing rather than a reading of this book.
            extent: p.stats.words as f32 / longest as f32,
            filled: p.stats.spoken as f32 / p.stats.words as f32,
        })
        .collect()
}

/// Merge consecutive paragraphs until each bar clears [`MIN_BAR_HEIGHT`].
///
/// Adaptive in pixels rather than by a fixed paragraph count: the same scene is
/// twelve bars in a tall pane and three in a short one, and a count chosen for one
/// is wrong for the other.
///
/// A merged bar's word count is the sum and its spoken count is the sum, so its
/// fill is the true share over the merged range. Averaging the members' shares
/// instead would over-weight a two-word paragraph of pure speech against the
/// four-hundred-word paragraph beside it.
fn merge_below(paragraphs: Vec<Paragraph>, lane_height: f32) -> Vec<Paragraph> {
    if lane_height <= 0.0 {
        return paragraphs;
    }
    let min_span = MIN_BAR_HEIGHT / lane_height;
    let mut out: Vec<Paragraph> = Vec::with_capacity(paragraphs.len());
    for p in paragraphs {
        match out.last_mut() {
            Some(last) if last.span.end - last.span.start < min_span => {
                last.span = LaneSpan::new(last.span.start, p.span.end);
                last.stats.words += p.stats.words;
                last.stats.spoken += p.stats.spoken;
            }
            _ => out.push(p),
        }
    }
    out
}

/// How this item marks speech, under the project's house style.
///
/// The same resolution the Analysis use case performs, for one item instead of a
/// whole binder: the item's own dictionary language when it has one, the Work's
/// otherwise, and the Work's quote-style override either way. Both halves matter —
/// a project set to guillemets under an `en-US` locale has `« »` inserted as the
/// writer types, and a texture that looked for `" "` would measure every paragraph
/// as zero spoken words while the editor produced speech in front of it.
pub fn markers_for_item(
    app_ctx: &std::rc::Rc<frontend::AppContext>,
    work_id: Option<EntityId>,
    item_id: EntityId,
) -> DialogueMarkers {
    use frontend::commands::{binder_item_commands, smart_punctuation_commands, work_commands};

    let work = work_id.and_then(|id| work_commands::get_work(app_ctx, &id).ok().flatten());

    let quote_style = work
        .as_ref()
        .and_then(|w| {
            smart_punctuation_commands::get_smart_punctuation(app_ctx, &w.smart_punctuation)
                .ok()
                .flatten()
        })
        .map(|sp| sp.quote_style)
        .unwrap_or_default();

    let item_tags = binder_item_commands::get_binder_item(app_ctx, &item_id)
        .ok()
        .flatten()
        .map(|item| item.dict_language)
        .unwrap_or_default();
    let work_tags = work.map(|w| w.dict_language).unwrap_or_default();

    let tags = if skribisto_model::language::has_tags(&item_tags) {
        item_tags
    } else {
        work_tags
    };
    prose_stats::markers_for(skribisto_model::language::primary(&tags), quote_style)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(start: f32, end: f32, words: usize, spoken: usize) -> Paragraph {
        Paragraph {
            span: LaneSpan::new(start, end),
            stats: ParagraphStats { words, spoken },
        }
    }

    /// The whole point of merging: a hundred paragraphs on a six-hundred-pixel lane
    /// are six pixels apart, and a bar under two pixels is aliasing rather than a
    /// reading.
    #[test]
    fn paragraphs_too_short_to_see_are_merged_until_they_clear_the_minimum() {
        // Ten paragraphs, each a tenth of the lane. On a 200 px strip that is 20 px
        // apiece, well clear of the 2 px minimum, so nothing merges.
        let ten: Vec<Paragraph> = (0..10)
            .map(|i| p(i as f32 * 0.1, (i + 1) as f32 * 0.1, 10, 0))
            .collect();
        assert_eq!(
            merge_below(ten.clone(), 200.0).len(),
            10,
            "20 px bars stand alone"
        );
        // On a 10 px strip the same bars are 1 px apiece, under the minimum, and each
        // has to take its neighbour with it.
        assert!(
            merge_below(ten, 10.0).len() < 10,
            "a strip too short for them must merge them"
        );
    }

    /// A merged bar's fill is the share over the merged range, not the mean of its
    /// members' shares. The two differ wherever the members differ in length, which
    /// is exactly where merging happens.
    #[test]
    fn a_merged_bar_reports_the_share_over_its_whole_range() {
        // Two words of pure speech, then four hundred of narration.
        let merged = merge_below(vec![p(0.0, 0.001, 2, 2), p(0.001, 0.9, 400, 0)], 100.0);
        assert_eq!(merged.len(), 1, "the sliver merged into its neighbour");
        let bar = merged[0];
        let share = bar.stats.spoken as f32 / bar.stats.words as f32;
        assert!(
            share < 0.02,
            "the sliver must not drag the share up to the mean of 0.5: got {share}"
        );
    }

    /// Before the lane is laid out there is no pixel budget to measure against, and
    /// merging on a guess would collapse a whole document into one bar that then
    /// never un-merged.
    #[test]
    fn nothing_is_merged_before_the_lane_has_a_height() {
        let three = vec![
            p(0.0, 0.001, 5, 0),
            p(0.001, 0.002, 5, 0),
            p(0.002, 1.0, 5, 0),
        ];
        assert_eq!(merge_below(three.clone(), 0.0).len(), 3);
    }
}
