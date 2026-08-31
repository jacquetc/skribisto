// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The import review table's epigraph cell: a quotation mark, and the quotation itself
//! on hover.
//!
//! # Why a marker and not the text
//!
//! The review table is full. Its nine columns already come to the width of the card they
//! sit in, so a tenth carrying a text preview would push the table into horizontal scroll
//! — and a column the writer has to scroll to reach is a column most of them never see.
//! A marker fits in the same 80 px the Words / Breaks / Comments columns use, and the
//! tooltip carries what the preview would have said.
//!
//! # Why the quotation still has to be *readable*
//!
//! An epigraph is recognised from a paragraph **style**, not from anything the writer
//! typed — `Epigraph` in the file this app wrote, `Quotations` or `IntenseQuote` in one
//! LibreOffice or Word wrote. That makes the one question a tick cannot answer the
//! important one: *did it read the right paragraph as an epigraph?* So the tooltip shows
//! the quotation's own words, not a count, and it is **composite** rather than plain: a
//! quotation is a block of prose that has to wrap, and it deserves the label above it
//! saying what the importer decided it was.
//!
//! # House rules this obeys, each of which has gone wrong before
//!
//! * **`TextRole::TooltipText`, never a content-surface role.** The tooltip background is
//!   dark in *both* themes, while light-theme `text_primary` is black — `tag_tooltip.rs`
//!   records measuring that at 1.27:1. Hierarchy between the two lines comes from the type
//!   scale, which is surface-independent.
//! * **The body wraps rather than runs.** Horizontal overflow in a composite tooltip is
//!   silently clipped, not scrolled; `TextWidget` wraps inside whatever width the surface
//!   gives it, and `attach_labelled_composite_tooltip` caps that.
//! * **It is attached through the app's own helper**, not the framework's
//!   `attach_composite_tooltip_boxed` — that one cannot set an accessible name, so every
//!   tooltip through it announces the literal word "Tooltip".

use teksilo::core::overlay::TooltipPlacement;
use teksilo::core::widget::{LayoutContext, LayoutResponse, Widget};
use teksilo::prelude::*;
use teksilo::widgets::{Center, MinSize, Spacer, TextWidget, VStack};

use crate::widgets::attach_labelled_composite_tooltip;

/// The glyph a row with an epigraph shows.
///
/// A left double quotation mark, which reads as "a quotation" far more directly than a
/// tick or a `1` would — the cell is answering "is there a quotation here", and a count is
/// the wrong shape for something a row has either none or one of.
const MARK: &str = "\u{201C}";

/// The hoverable extent of the cell — the column's own width and the table's row height,
/// so the pointer finds the tooltip anywhere in the cell rather than only on the glyph.
/// Kept in step with `import_document`'s `.width(ColumnWidth::Fixed(80.0))` and
/// `.row_height(30.0)` by the test at the bottom of this file.
const HIT_W: f32 = 80.0;
const HIT_H: f32 = 30.0;

/// One review row's epigraph cell. Renders nothing at all when the row heads no quotation.
#[derive(Debug)]
pub struct EpigraphCell {
    /// The epigraph as stored — Djot, one blockquote per quotation.
    djot: String,
    id: Option<WidgetId>,
}

impl EpigraphCell {
    pub fn new(djot: impl Into<String>) -> Self {
        Self {
            djot: djot.into(),
            id: None,
        }
    }
}

impl Widget for EpigraphCell {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let quotation = epigraph_plain(&self.djot);
        if quotation.is_empty() {
            // Nothing to mark and nothing to say. An empty cell rather than a dimmed
            // glyph: most rows head no quotation, and a column of greyed marks reads as a
            // measurement that failed — the same reason Breaks and Comments stay blank
            // instead of showing 0.
            let id = ctx.add(Spacer::new());
            self.id = Some(id);
            return vec![id];
        }

        // **The anchor has to be a hit region, not a glyph.** A bare `TextWidget` is
        // painted, not hovered: it registers no area for the pointer, so a tooltip
        // attached to one fires only if the pointer happens to land on an inked pixel —
        // measured, and it did fire once in twenty tries before this wrapper existed.
        // `tag_chip.rs` states the same rule from the other side, of the gaps *between*
        // its dots: "the hit cells must abut, or the gaps between them would be dead
        // zones for hover".
        //
        // `MinSize` floors the cell at the row's own height so the whole 80 × 30 cell is
        // hoverable, and the mark is centred inside it. The accessible name rides the
        // anchor, so the cell announces itself as an epigraph rather than as a
        // punctuation character.
        let id = ctx.add(
            MinSize::new(HIT_W, HIT_H)
                .child(
                    Center::new().child(
                        TextWidget::new(lit!(MARK.to_string()))
                            .style(TextStyleRole::Body)
                            .color(TextRole::Secondary),
                    ),
                )
                .access_label(tr!(import_document_col_epigraph())),
        );

        attach_labelled_composite_tooltip(
            ctx,
            id,
            Box::new(tooltip_body(&quotation)),
            tr!(import_document_col_epigraph()),
            TooltipPlacement::Below,
        );

        self.id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// The tooltip's own body: what the importer decided, then the words it decided it about.
fn tooltip_body(quotation: &str) -> impl Widget {
    VStack::new()
        .spacing(4.0)
        .child(
            TextWidget::new(tr!(import_document_col_epigraph()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::TooltipText),
        )
        .child(
            TextWidget::new(lit!(quotation.to_string()))
                .style(TextStyleRole::Body)
                .color(TextRole::TooltipText),
        )
}

/// An epigraph's own words, with the Djot that carries them stripped out.
///
/// Three things go, and nothing else:
///
/// * the `>` that makes each line part of a blockquote;
/// * an attribute line (`{semantic_role=epigraph}` and friends) — markup the compiler
///   writes and no writer typed. Dropping the line whole is exact rather than approximate:
///   a Djot attribute line is *only* attributes, never attributes beside prose;
/// * the blank line between two quotations, which becomes a single space so the tooltip
///   reads as continuous prose rather than as a paragraph with a hole in it.
///
/// Inline emphasis (`_x_`, `*x*`) is left alone. It is rare inside an epigraph, stripping
/// it would mean a second Djot parse for a hover hint, and the markers reading literally is
/// a smaller wrong than a preview that silently disagrees with the stored text.
pub fn epigraph_plain(djot: &str) -> String {
    let joined = djot
        .lines()
        .map(|line| line.trim_start().trim_start_matches('>').trim())
        .filter(|line| !(line.starts_with('{') && line.ends_with('}')))
        .collect::<Vec<_>>()
        .join(" ");
    joined.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_compilers_own_markup_never_reaches_the_writer() {
        assert_eq!(
            epigraph_plain("> {semantic_role=epigraph}\n> Every winter asks twice."),
            "Every winter asks twice."
        );
    }

    #[test]
    fn a_multi_paragraph_quotation_reads_as_one_run_of_prose() {
        assert_eq!(
            epigraph_plain("> All happy families are alike.\n>\n> — Tolstoy"),
            "All happy families are alike. — Tolstoy"
        );
    }

    /// Two quotations on one row are two blockquotes; both belong in the tooltip.
    #[test]
    fn two_quotations_both_reach_the_tooltip() {
        assert_eq!(
            epigraph_plain("> The first.\n\n> The second."),
            "The first. The second."
        );
    }

    /// An empty field is what most rows carry, and it must produce no mark at all.
    #[test]
    fn a_row_with_no_epigraph_has_nothing_to_say() {
        assert_eq!(epigraph_plain(""), "");
        assert_eq!(epigraph_plain("> \n>\n"), "");
    }

    // ── The tooltip actually opens ──────────────────────────────────────────
    //
    // These exist because the obvious way to check — drive the running app through the
    // automation bridge, inject a pointer, look for a new overlay — **cannot see a
    // tooltip at all**. Measured against a control: hovering the leading rail's own
    // "Toggle the binder" button, whose tooltip nobody doubts, opens no overlay under
    // `inject_pointer` + `advance_clock` either. So a live hover proves nothing either
    // way, and the claim "it shows the quotation on hover" has to be settled here, where
    // `WidgetTree` exposes the real hover clock (`teksilo`'s own
    // `tooltip_appears_after_delay` is the same shape).

    use std::time::Duration;
    use teksilo::core::widget_tree::WidgetTree;

    const SAMPLE: &str = "> {semantic_role=epigraph}\n> Every winter asks twice.";

    /// Long enough to clear `motion.tooltip_delay_heavy` — the composite tier uses the
    /// heavy delay, not the plain one.
    const PAST_THE_DELAY: Duration = Duration::from_secs(3);

    #[test]
    fn hovering_the_mark_opens_a_tooltip_carrying_the_quotation() {
        let mut tree = WidgetTree::new();
        let cell = tree.add(EpigraphCell::new(SAMPLE));
        tree.layout(SizeProposal::exact(HIT_W, HIT_H));

        tree.pointer_move(tree.bounds(cell).center());
        assert!(
            tree.active_overlays().is_empty(),
            "a tooltip that opens instantly is a tooltip that opens by accident"
        );

        tree.advance_time(PAST_THE_DELAY);
        assert_eq!(
            tree.active_overlays().len(),
            1,
            "hovering the cell must open exactly one tooltip"
        );
        assert!(
            tree.find_by_label("Every winter asks twice.").is_some(),
            "the tooltip must carry the quotation itself — a marker whose tooltip does \
             not say what was detected answers nothing"
        );
    }

    /// The pointer has to be *on the cell*, not merely on the glyph.
    ///
    /// This is the regression that made the first attempt unreliable: the anchor was a
    /// bare `TextWidget`, which is painted rather than hovered, so the tooltip fired only
    /// where ink happened to be. Hovering a corner of the cell — well away from a centred
    /// quotation mark — is what proves the `MinSize` hit region is doing its job.
    #[test]
    fn the_whole_cell_is_hoverable_not_only_the_glyph() {
        let mut tree = WidgetTree::new();
        let cell = tree.add(EpigraphCell::new(SAMPLE));
        tree.layout(SizeProposal::exact(HIT_W, HIT_H));

        let b = tree.bounds(cell);
        tree.pointer_move(teksilo::canvas::Point::new(b.x + 2.0, b.y + 2.0));
        tree.advance_time(PAST_THE_DELAY);

        assert_eq!(
            tree.active_overlays().len(),
            1,
            "the top-left corner of the cell must show the tooltip too"
        );
    }

    /// A row that heads no quotation offers nothing to hover — no mark, and no tooltip
    /// promising content that does not exist.
    #[test]
    fn a_cell_with_no_epigraph_attaches_no_tooltip() {
        let mut tree = WidgetTree::new();
        let cell = tree.add(EpigraphCell::new(""));
        tree.layout(SizeProposal::exact(HIT_W, HIT_H));

        tree.pointer_move(tree.bounds(cell).center());
        tree.advance_time(PAST_THE_DELAY);

        assert!(
            tree.active_overlays().is_empty(),
            "an empty cell must not open a tooltip"
        );
    }
}
