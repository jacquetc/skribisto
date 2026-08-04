// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Everything one editor needs in order to create and repaint comments on the
//! document it is showing, in a single handle.
//!
//! An editor widget knows its `TextDocument` and its selection, but nothing about
//! which `Content` row that document came from, nor how to reach the comment
//! store. A `CommentBinding` closes that gap: it is minted by the [`OpenDoc`](crate::models::OpenDoc) that
//! already owns both facts, and handed down to `writing_column` exactly the way
//! the spell session is. That keeps the editor free of any dependency on a
//! view-model, and keeps "which Content is this" answered by the one place that
//! genuinely knows — a `BinderItem` owns up to three Content rows, so an editor
//! guessing would be a coin flip between body and synopsis.

use std::rc::Rc;

use bastyde::prelude::Signal;
use bastyde::text_document::TextDocument;

use crate::comments::anchor;
use crate::comments::session::{CommentHighlightSession, LiveAnchor};
use crate::view_models::CommentsViewModel;

/// A deeper shade of the wash, for a span two threads both cover.
///
/// Derived rather than a fourth palette entry: "stacked" must always read as
/// *more of the same colour*, and an independent value would eventually be set to
/// an unrelated one.
fn deepen(c: bastyde::prelude::Color) -> bastyde::prelude::Color {
    c.darken(0.18)
}

/// Bridge the theme's colour type to text-document's.
///
/// Two distinct `Color` types meet here: the widget/canvas one the theme speaks,
/// and text-document's own, which `HighlightFormat` takes. Converting in one place
/// is what keeps the wash and the margin marks the same colour.
fn to_doc_color(c: bastyde::prelude::Color) -> bastyde::text_document::Color {
    let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    bastyde::text_document::Color::rgb(q(c.r()), q(c.g()), q(c.b()))
}

/// One editor's door to the comment feature.
#[derive(Clone)]
pub struct CommentBinding {
    vm: CommentsViewModel,
    doc: TextDocument,
    session: Rc<CommentHighlightSession>,
    /// The `Content` row this editor's document reads and writes.
    content_id: u64,
    /// The gutter this editor's **page** reserves, whatever this document holds.
    ///
    /// Zero for a tab, whose single margin decides for itself. A stream sets it
    /// for every row at once — see
    /// [`ColumnWithMargin::reserve`](crate::comments::pane::ColumnWithMargin::reserve)
    /// for why the reservation cannot be per-row. It rides on the binding rather
    /// than on a `writing_column` parameter because the binding is already *this
    /// editor's door to the comment feature*, and an editor with no comments needs
    /// no door at all.
    gutter: Signal<f32>,
}

impl CommentBinding {
    pub fn new(
        vm: CommentsViewModel,
        doc: TextDocument,
        session: Rc<CommentHighlightSession>,
        content_id: u64,
    ) -> Self {
        Self {
            vm,
            doc,
            session,
            content_id,
            gutter: Signal::new(0.0),
        }
    }

    /// Reserve a page-level gutter for this editor — see [`Self::gutter`].
    pub fn with_gutter(mut self, gutter: Signal<f32>) -> Self {
        self.gutter = gutter;
        self
    }

    pub fn gutter(&self) -> Signal<f32> {
        self.gutter.clone()
    }

    /// Whether this editor would show any card at all.
    ///
    /// The one predicate the margin's `build`, its marks and the page-level gutter
    /// all have to agree on — a thread that is resolved, collapsed to nothing, or
    /// hidden by Tools ▸ Comments produces no card, and a gutter reserved for it
    /// would be an empty column the writer cannot get rid of.
    pub fn has_live_cards(&self) -> bool {
        if !self.vm.is_visible() {
            return false;
        }
        // The row lookup is not redundant with the anchor filter: `CommentMargin::build`
        // skips any anchor whose id does not resolve against `rows_for_content`, so an
        // anchor the session still lists but the model no longer backs mounts no card.
        // Answering `true` for one of those would reserve the full column and then put
        // nothing in it — the empty gutter this predicate exists to prevent. Both sides
        // must ask the same question, so ask it the same way.
        let rows = self.vm.model().rows_for_content(self.content_id);
        self.live()
            .iter()
            .any(|a| !a.resolved && a.end > a.start && rows.iter().any(|r| r.id == a.comment_id))
    }

    pub fn view_model(&self) -> CommentsViewModel {
        self.vm.clone()
    }

    pub fn content_id(&self) -> u64 {
        self.content_id
    }

    /// The document's plain text and every block's start offset, both in the
    /// document-absolute **character** space the anchor engine speaks.
    ///
    /// Read fresh on each use rather than cached: the caller is about to anchor
    /// against it, and a snapshot taken a keystroke ago would anchor to text that
    /// is no longer there.
    fn snapshot(&self) -> (String, Vec<usize>) {
        let text = self.doc.to_plain_text().unwrap_or_default();
        let starts: Vec<usize> = self
            .doc
            .blocks()
            .into_iter()
            .map(|b| b.position())
            .collect();
        (text, starts)
    }

    /// Comment on `[start, end)`. No-op on an empty selection — there would be
    /// nothing to anchor to, and the thread would be born orphaned.
    pub fn add_range(&self, start: usize, end: usize) -> Option<u64> {
        if end <= start {
            return None;
        }
        let (text, _) = self.snapshot();
        let id = self
            .vm
            .add_range_comment(self.content_id, &text, start, end, "", None)?;
        self.push_live();
        Some(id)
    }

    /// Comment on the paragraph(s) the selection touches — one block for a bare
    /// caret, every covered block for a real selection.
    pub fn add_paragraph(&self, sel_start: usize, sel_end: usize) -> Option<u64> {
        let (text, starts) = self.snapshot();
        let id = self.vm.add_paragraph_comment(
            self.content_id,
            &text,
            &starts,
            sel_start,
            sel_end,
            "",
            None,
        )?;
        self.push_live();
        Some(id)
    }

    /// Re-derive every live anchor for this document from the persisted quotes and
    /// hand them to the highlight session.
    ///
    /// Called when the document is opened and whenever the comment set changes.
    /// Tier 1 of the re-anchor (the stored offset still holds) is the overwhelming
    /// common case, so this is cheap in the normal path.
    pub fn push_live(&self) {
        self.refresh_wash();
        // Tell the view-model which document the margin is showing, so a card's
        // "delete all comments here" is scoped to this document rather than to
        // the whole project.
        //
        // Deliberately *not* part of [`Self::refresh_wash`]: a stream mounts one
        // margin per row, and a refresh that claimed this slot would leave it
        // holding whichever row happened to rebuild last.
        self.vm.set_margin_content(self.content_id);
    }

    /// Re-derive the anchors and repaint the wash, without claiming the margin.
    ///
    /// The half of [`Self::push_live`] that has to run whenever the comment set
    /// *changes shape*, not merely when one is created. The wash is painted from
    /// [`CommentHighlightSession`]'s own anchor list, and nothing else refreshes
    /// it — so a deleted thread's ochre stayed on the text, and a resolved one's
    /// too, since `repaint`'s `!resolved` filter reads a flag on that same stale
    /// list. The cards vanished on cue, which is what made it look like a paint
    /// bug rather than a stale-model one: `build` drops a card whose id no longer
    /// resolves against the *model*, while `repaint` needs no such lookup.
    ///
    /// Called from [`CommentMargin::build`](crate::comments::margin::CommentMargin),
    /// which already rebuilds on exactly the two signals that matter — the
    /// structure version and Tools ▸ Comments. Refreshing there rather than at each
    /// mutation site is what keeps the next one added from re-introducing this.
    pub fn refresh_wash(&self) {
        // One palette for the wash, the marks and the cards. Pushed here rather
        // than at construction because the theme can change under a live document,
        // and this runs on every set that matters.
        let p = self.vm.palette_signal().get();
        self.session
            .set_colors(to_doc_color(p.wash), to_doc_color(deepen(p.wash)));

        // Inherit Tools ▸ Comments. Applied here as well as by the toggle's own walk
        // over the open documents, because a document opened *while* comments are
        // hidden has no other way to learn it — and would come up painting.
        self.session.set_active(self.vm.is_visible());

        let (text, starts) = self.snapshot();
        let live = self.vm.reanchor(self.content_id, &text, &starts, None);
        self.session.set_anchors(live);
    }

    /// Drain a frame's worth of edits, then flag anything whose text vanished.
    ///
    /// Returning the ids rather than swallowing them keeps the orphan contract
    /// honest: an anchor that collapsed while typing becomes a *visible* orphan on
    /// the spot, instead of quietly reappearing as one at the next load.
    pub fn tick(&self) {
        let orphaned = self.session.tick();
        if !orphaned.is_empty() {
            self.vm.mark_orphaned(&orphaned, None);
        }
    }

    /// The live anchors currently tracked, for hit-testing a click.
    pub fn live(&self) -> Vec<LiveAnchor> {
        self.session.anchors()
    }

    /// The annotation spans this document's editor should expose to AccessKit.
    ///
    /// Built from the *live* anchors, so an orphaned thread contributes nothing —
    /// it has no range to announce, and a screen reader pointed at a dead offset
    /// would be worse than silence. Resolved threads are likewise omitted: they
    /// are settled, and the docks remain their home.
    ///
    /// The summary is what a reader speaks. It leads with the author and body
    /// because that is the answer to "what does this say"; the reply count comes
    /// last, as context rather than as the headline.
    pub fn annotation_spans(&self) -> Vec<bastyde::widgets::rich_text::TextAnnotationSpan> {
        let rows = self.vm.model().rows_for_content(self.content_id);
        self.session
            .anchors()
            .into_iter()
            .filter(|a| !a.resolved && a.end > a.start)
            .filter_map(|a| {
                let row = rows.iter().find(|r| r.id == a.comment_id)?;
                if row.orphaned {
                    return None;
                }
                let mut summary = if row.author_name.is_empty() {
                    row.body.clone()
                } else {
                    format!("{}: {}", row.author_name, row.body)
                };
                if row.reply_count() > 0 {
                    summary.push_str(&format!(" ({} replies)", row.reply_count()));
                }
                Some(bastyde::widgets::rich_text::TextAnnotationSpan {
                    start: a.start,
                    end: a.end,
                    // The comment's own id, not its index: a stable synthetic
                    // NodeId is what keeps a screen reader's cursor inside the
                    // thread across an unrelated edit.
                    group_id: row.id,
                    summary,
                })
            })
            .collect()
    }

    /// The shared palette every mark and card for this document is painted with.
    pub fn palette(&self) -> bastyde::prelude::Signal<crate::view_models::CommentPalette> {
        self.vm.palette_signal()
    }

    /// A one-shot seek parked for this document, if any — consumed on attach.
    pub fn take_seek(&self) -> Option<(usize, usize)> {
        self.vm.take_seek(self.content_id)
    }

    /// Which block a document offset falls in — the paragraph a caret is "in".
    pub fn block_of(&self, offset: usize) -> usize {
        let (_, starts) = self.snapshot();
        anchor::block_of(&starts, offset)
    }
}
