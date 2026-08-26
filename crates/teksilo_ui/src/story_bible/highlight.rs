// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Every name of the entry, marked in the prose it is read against.**
//!
//! The In prose reading answers one question — where does this character turn up — and
//! it answered it twice in the margin and not once in the text. The strip says which
//! rows and roughly where; this says which words. A reader scanning a scene for "the
//! one place Lizzy is actually named" was left to find it by eye.
//!
//! ## The same measurement, drawn twice
//!
//! The offsets come from [`crate::margin_lane::subject::hits`], which is also what the
//! lane's own provider marks. One function, deliberately: a strip and a text that
//! disagreed about where a name is would be worse than either alone, and "whole words,
//! longest name first" is a rule with enough edges (a possessive, an alias inside a full
//! name, a character called Ana in a book with bananas) that two implementations of it
//! would drift.
//!
//! ## A highlight, and why not the dotted underline this shipped with
//!
//! The first cut drew a dotted underline, reasoning that the three other layers on this
//! prose each own a channel: a background is the find banner's, a wavy underline the
//! spell checker's, a solid one is what a link looks like. The reasoning was sound and
//! the result was wrong — at a reading size, over a page of prose, a two-pixel dotted rule
//! under a word is not something anyone sees. The whole point is that a reader can find
//! the name at a glance, and a mark that has to be looked for fails at exactly that.
//!
//! So it is a **wash of colour behind the word**, which is what "highlighted" means to
//! everyone who has ever used a marker. The overlap with the find banner is real and
//! accepted: this reading is *about* one entry, and the two are told apart by hue — the
//! banner's accent against the provider's own palette slot — and by the fact that find's
//! marks come and go with a query while these are simply always there.
//!
//! The colour is the lane provider's own palette slot, read from the registered spec
//! rather than repeated here, so the wash behind a name and the dot beside it are the
//! same colour or the registration is wrong.
//!
//! ## Marked is not the same as reachable
//!
//! Marking every mention says *that* they are there. It does not get a reader to
//! them: a Book's reading is forty documents on one scroll, and "she is named twice in
//! chapter nine" is useless if finding those two means reading chapter nine. So the
//! marks are also a **walk** — [`SubjectWalk`] — with the page's own header counting
//! them and stepping through, the way the find banner steps through what was typed.
//!
//! It is a walk of its own rather than the find banner seeded with a name, and the
//! reason is the same one [`crate::margin_lane::subject`] gives for not reusing the
//! search query: a find needle is one string, and an entry has a title *and* aliases.
//! Typing an alternation would be asking the writer to do the index's job, and a banner
//! seeded with the title alone would silently walk past every "Lizzy" in the book.
//!
//! ## One switch, two surfaces
//!
//! The strip's own switch governs this as well — `editor.margin_lane.provider.story_bible`,
//! read through the registered spec so the key is never written twice. "Show me where this
//! entry is named" is one idea, and a second setting for the other half of it would be
//! more settings rather than more control. Off, the layer keeps its session and pushes an
//! empty set, so switching back re-derives on the next frame rather than rebuilding
//! everything the reading holds.
//!
//! ## Read-only, and it never edits
//!
//! A range session is a paint layer: it changes no characters, survives no save, and is
//! retired with the widget that made it. The prose on this page stays as editable as it
//! is anywhere else, and typing into it simply re-derives the marks on the next frame.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use teksilo::text_document::{
    DocumentEvent, HighlightFormat, RangeHighlight, SessionId, Subscription, TextDocument,
    UnderlineStyle,
};
use teksilo::tokens::{Color as ThemeColor, ColorTokens};

use common::types::EntityId;
use teksilo::core::signal::Signal;

use skribisto_model::mentions::DiscoverableEntity;

use crate::margin_lane::subject::hits;

/// The format every marked name is drawn in.
///
/// `DocColor` and the theme's `Color` are different types with different component
/// ranges (0..255 against 0..1), the same conversion the find banner's own
/// `highlight_of` makes.
pub fn subject_format(color: ThemeColor) -> HighlightFormat {
    HighlightFormat {
        // Light enough that the prose reads through it and heavy enough to find from
        // across a page. The colour it washes is already tuned to clear 3:1 against the
        // writing surface (`resolve_slot`), so a fifth of it is a legible tint on either
        // theme rather than a value that works in light and vanishes in dark.
        background_color: Some(doc_color(color.with_alpha(0.22))),
        ..Default::default()
    }
}

/// The theme's `Color` (components 0..1) as a document highlight colour (0..255) — the
/// same conversion the find banner's own `highlight_of` makes.
fn doc_color(c: ThemeColor) -> teksilo::text_document::Color {
    let to_u8 = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    teksilo::text_document::Color::rgba(to_u8(c.r()), to_u8(c.g()), to_u8(c.b()), to_u8(c.a()))
}

/// **The one mention the reader is standing on**, drawn to be findable on a page where
/// forty others are already washed in the same colour.
///
/// Two channels, not one: a much stronger wash *and* a solid underline. At a glance the
/// question is "which of these am I on", and one channel shifting a shade of the same hue
/// does not answer that from across a scene — the difference has to survive a reader who
/// cannot separate two tints of one colour, which is the rule the margin lane states for
/// its own marks.
///
/// A solid underline is a link's channel, which is why the *resting* mark does not use
/// one. Spending it on the single mention under a heavy wash is a different question, and
/// there it is the clearest thing available.
pub fn subject_current_format(color: ThemeColor) -> HighlightFormat {
    HighlightFormat {
        underline_style: Some(UnderlineStyle::SingleUnderline),
        underline_color: Some(doc_color(color)),
        background_color: Some(doc_color(color.with_alpha(0.55))),
        ..Default::default()
    }
}

/// The colour the lane marks this entry's names in, so the text under a name and the dot
/// beside it agree.
///
/// Read from the **registered** spec rather than from a constant repeated here: the slot
/// is the provider's own choice and moving it must move both. Falls back to the primary
/// text colour if the provider is not registered, which is the headless-test state and
/// draws a legible mark rather than nothing.
pub fn subject_color(colors: &ColorTokens) -> ThemeColor {
    crate::margin_lane::registered_for(crate::margin_lane::LaneSurface::Stream)
        .iter()
        .find(|spec| spec.id == crate::margin_lane::providers::STORY_BIBLE_PROVIDER_ID)
        .map(|spec| crate::margin_lane::resolve_slot(colors, spec.palette_slot))
        .unwrap_or(colors.text_primary)
}

/// Whether the writer wants these names marked at all.
///
/// The lane provider's own switch, since the strip and the text are two surfaces of one
/// finding — see the module note. Read through the registered spec rather than by
/// spelling the key out here, so the two can never address different settings.
///
/// `true` when the provider is not registered: that is the headless-test state, and a
/// test that had to install the whole provider registry to see a mark would be
/// testing the registry.
pub fn subject_enabled(store: &teksilo::settings::SettingsStore) -> bool {
    crate::margin_lane::registered_for(crate::margin_lane::LaneSurface::Stream)
        .iter()
        .find(|spec| spec.id == crate::margin_lane::providers::STORY_BIBLE_PROVIDER_ID)
        .is_none_or(|spec| store.signal(&spec.settings_key(), spec.default_on).get())
}

/// One document's highlight layer: the names of the entry this reading is about.
///
/// Mirrors `FindSession`'s lifecycle, which every other range-session holder in this
/// crate does too — a held [`Subscription`], a dirty flag drained by the caller once a
/// frame, and a [`Drop`] that retires the layer rather than leaving it on a document the
/// rest of the app shares.
pub struct SubjectHighlight {
    doc: TextDocument,
    session: SessionId,
    /// Set by the document subscription on any offset-moving edit; drained by
    /// [`refresh`](Self::refresh). `Arc` because `on_change` takes a `Send + Sync` closure.
    dirty: Arc<AtomicBool>,
    /// The entry the pushed ranges were derived from — `None` while the marks are
    /// switched off. Compared rather than assumed, so a rename or a new alias re-derives
    /// without the caller having to notice.
    subject: RefCell<Option<DiscoverableEntity>>,
    /// Where every name falls in this document, in order — `(start, length)` in
    /// characters. Kept apart from the pushed ranges below because the walk asks about
    /// positions while the paint layer asks about formats, and a step changes the second
    /// without touching the first.
    spans: RefCell<Vec<(usize, usize)>>,
    /// Which of those the reader is standing on, if any. `None` for every document but
    /// the one holding the walk's cursor — the same invariant a multi-document find keeps,
    /// and for the same reason: a reading with forty rows must not show forty current
    /// marks.
    current: std::cell::Cell<Option<usize>>,
    /// The last set pushed, so an unchanged recompute skips the repaint.
    last: RefCell<Vec<RangeHighlight>>,
    /// How a mention is drawn, and how the one the reader is standing on is drawn instead.
    format: HighlightFormat,
    current_format: HighlightFormat,
    /// Held so the subscription lives as long as this does (dropping it unsubscribes).
    _sub: Subscription,
}

impl std::fmt::Debug for SubjectHighlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubjectHighlight")
            .field("marks", &self.last.borrow().len())
            .finish()
    }
}

impl SubjectHighlight {
    /// Attach an empty layer to `doc`. Nothing is drawn until the first
    /// [`refresh`](Self::refresh).
    pub fn new(
        doc: &TextDocument,
        format: HighlightFormat,
        current_format: HighlightFormat,
    ) -> Self {
        let session = doc.add_range_session();
        let dirty = Arc::new(AtomicBool::new(false));
        let sub = {
            let dirty = dirty.clone();
            doc.on_change(move |event| {
                // Only edits that MOVE char offsets stale the marks. Never
                // `HighlightPaintChanged`, which this layer's own push emits — reacting
                // to it would loop. The same filter `FindSession` and `SpellSession` use.
                if matches!(
                    event,
                    DocumentEvent::ContentsChanged { .. }
                        | DocumentEvent::DocumentReset
                        | DocumentEvent::BlockCountChanged(_)
                        | DocumentEvent::FlowElementsInserted { .. }
                        | DocumentEvent::FlowElementsRemoved { .. }
                ) {
                    dirty.store(true, Ordering::Relaxed);
                }
            })
        };
        Self {
            doc: doc.clone(),
            session,
            dirty,
            subject: RefCell::new(None),
            spans: RefCell::new(Vec::new()),
            current: std::cell::Cell::new(None),
            last: RefCell::new(Vec::new()),
            format,
            current_format,
            _sub: sub,
        }
    }

    /// Re-derive the marks for `subject` if an edit or a rename has moved them, and push
    /// them. Returns whether the pushed set actually changed. `None` clears them, which is
    /// what the provider's switch turned off means.
    ///
    /// Cheap to call every frame: with no edit and the same entry it does nothing at all,
    /// which matters because this runs once per mapped row and a Book's reading can be
    /// forty of them.
    pub fn refresh(&self, subject: Option<&DiscoverableEntity>) -> bool {
        let renamed = self.subject.borrow().as_ref() != subject;
        if !self.dirty.swap(false, Ordering::Relaxed) && !renamed {
            return false;
        }
        if renamed {
            *self.subject.borrow_mut() = subject.cloned();
        }
        let spans: Vec<(usize, usize)> = match subject {
            Some(entity) => {
                let text = self.doc.to_plain_text().unwrap_or_default();
                hits(&text, entity)
                    .into_iter()
                    .map(|(start, end)| (start, end - start))
                    .collect()
            }
            None => Vec::new(),
        };
        // An edit can delete the very mention the reader was standing on. Clamped rather
        // than reset, the rule `FindSession::refresh_if_stale` states: an edit relocates
        // the marks, it does not throw away where the reader was among them.
        // A document that has lost all of them holds no current mention at all — the same
        // three-arm shape `set_current` below uses, so the two cannot answer differently.
        self.current.set(match self.current.get() {
            Some(_) if spans.is_empty() => None,
            Some(i) => Some(i.min(spans.len() - 1)),
            None => None,
        });
        *self.spans.borrow_mut() = spans;
        self.push()
    }

    /// Say which of this document's mentions the reader is standing on, or `None` for
    /// none of them — what every document but the walk's own is told.
    ///
    /// An index past the end clamps; any index at all on a document with no mentions is
    /// `None`. A caller stepping into a document backwards asks for "the last one" as a
    /// number only this can know, exactly as `FindSession::set_current` is asked.
    pub fn set_current(&self, index: Option<usize>) -> bool {
        let n = self.spans.borrow().len();
        self.current.set(match index {
            Some(_) if n == 0 => None,
            Some(i) => Some(i.min(n - 1)),
            None => None,
        });
        self.push()
    }

    /// Build the paint layer from the spans and the current index, and push it if it
    /// actually changed. The current mention is emitted **last**, so where two marks abut
    /// its format wins the registry's last-writer-per-field merge — the rule
    /// `FindSession::apply` records.
    fn push(&self) -> bool {
        let current = self.current.get();
        let spans = self.spans.borrow();
        let mut next: Vec<RangeHighlight> = Vec::with_capacity(spans.len());
        for (i, (start, length)) in spans.iter().enumerate() {
            if Some(i) == current {
                continue;
            }
            next.push(RangeHighlight {
                start: *start,
                length: *length,
                format: self.format.clone(),
            });
        }
        if let Some((start, length)) = current.and_then(|i| spans.get(i)) {
            next.push(RangeHighlight {
                start: *start,
                length: *length,
                format: self.current_format.clone(),
            });
        }
        drop(spans);
        if *self.last.borrow() == next {
            return false;
        }
        self.doc.set_session_ranges(self.session, next.clone());
        *self.last.borrow_mut() = next;
        true
    }

    /// How many names are marked right now.
    pub fn marks(&self) -> usize {
        self.spans.borrow().len()
    }

    /// Where one of them falls, as `(start, length)` in characters — what a walk hands to
    /// an editor to select and scroll to.
    pub fn span(&self, index: usize) -> Option<(usize, usize)> {
        self.spans.borrow().get(index).copied()
    }

    /// Which mention the reader is standing on in this document, if any.
    pub fn current(&self) -> Option<usize> {
        self.current.get()
    }
}

impl Drop for SubjectHighlight {
    /// Retire the layer. The `Subscription`'s own drop stops delivery but leaves the
    /// ranges on a document the rest of the app shares — a reading closed would otherwise
    /// leave its marks over every scene it had opened.
    fn drop(&mut self) {
        self.doc.remove_session(self.session);
    }
}

/// **Where the reader is in this entry's mentions**, across the rows of one reading.
///
/// The rows are separate documents on one scroll, so a step is two questions: which
/// document, and which mention inside it. The cursor is kept as that pair rather than as
/// a flat index, because the flat index of a mention moves whenever a row *above* it
/// gains or loses one — which happens on every keystroke somewhere else on the page.
///
/// It owns no layers. The reading builds and prunes those with the rows it shows; this
/// only ever borrows them, and a row it names that is no longer there is simply skipped.
pub struct SubjectWalk {
    layers: Rc<RefCell<HashMap<EntityId, SubjectHighlight>>>,
    /// The rows in the order the page mounts them.
    order: RefCell<Vec<EntityId>>,
    /// The row holding the mention the reader is standing on, and which of its mentions.
    at: std::cell::Cell<Option<(EntityId, usize)>>,
    /// Mentions on the whole page, and the 1-based ordinal of the current one (0 for
    /// none). Signals because the page's header reads them and a keystroke changes them
    /// without rebuilding anything.
    total: Signal<usize>,
    ordinal: Signal<usize>,
}

impl std::fmt::Debug for SubjectWalk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubjectWalk")
            .field("total", &self.total.get())
            .field("at", &self.at.get())
            .finish()
    }
}

impl SubjectWalk {
    pub fn new(layers: Rc<RefCell<HashMap<EntityId, SubjectHighlight>>>) -> Self {
        Self {
            layers,
            order: RefCell::new(Vec::new()),
            at: std::cell::Cell::new(None),
            total: Signal::new(0),
            ordinal: Signal::new(0),
        }
    }

    /// The rows the page is showing, in the order it shows them. Re-set on every build:
    /// a Book switch replaces the list wholesale, and a split adds to it.
    pub fn set_order(&self, order: Vec<EntityId>) {
        *self.order.borrow_mut() = order;
    }

    /// How many mentions the reading holds, and which one the reader is on.
    pub fn total_signal(&self) -> Signal<usize> {
        self.total.clone()
    }
    pub fn ordinal_signal(&self) -> Signal<usize> {
        self.ordinal.clone()
    }

    /// Re-derive every row's marks against `names` and republish the counters.
    ///
    /// Called once a frame. Each layer is a cheap no-op unless its document was edited or
    /// the entry renamed, so the cost of a quiet frame is one flag read per row.
    pub fn refresh(&self, subject: Option<&DiscoverableEntity>) {
        {
            let layers = self.layers.borrow();
            for layer in layers.values() {
                layer.refresh(subject);
            }
        }
        // Re-apply the cursor before publishing. This is the whole of the one-source-of-
        // truth rule: a layer clamps its own current index on an edit, and a layer rebuilt
        // for a new theme starts with none at all, so without this the header's ordinal
        // and the mark on the page drift apart and stay drifted until the next chevron.
        self.reapply();
        self.publish();
    }

    /// Push the walk's cursor onto the layers, clamping it into what its row still holds
    /// and dropping it when that row has left the reading or lost its mentions.
    ///
    /// **The one place that decides which mention is current.** Every layer is told on
    /// every refresh and every step, so no layer can hold a current mark the walk does
    /// not know about, and the walk cannot claim an ordinal nothing is drawing.
    fn reapply(&self) {
        let order = self.order.borrow();
        let layers = self.layers.borrow();
        let at = self.at.get().and_then(|(item, i)| {
            if !order.contains(&item) {
                return None;
            }
            let n = layers.get(&item)?.marks();
            (n > 0).then(|| (item, i.min(n - 1)))
        });
        for (id, layer) in layers.iter() {
            layer.set_current(at.filter(|(item, _)| item == id).map(|(_, i)| i));
        }
        drop(layers);
        drop(order);
        self.at.set(at);
    }

    /// Move to the next (or previous) mention, crossing into another row when this one
    /// runs out, and wrapping round the whole reading.
    ///
    /// Returns the row and where in it to look — `(item, start, length)` — or `None` when
    /// the reading holds no mentions at all. Every row but the one landed on is told it
    /// holds no current mention, which is what keeps one mark of forty looking current.
    pub fn step(&self, forward: bool) -> Option<(EntityId, usize, usize)> {
        let counted: Vec<(EntityId, usize)> = {
            let order = self.order.borrow();
            let layers = self.layers.borrow();
            order
                .iter()
                .filter_map(|id| layers.get(id).map(|l| (*id, l.marks())))
                .filter(|(_, n)| *n > 0)
                .collect()
        };
        if counted.is_empty() {
            self.at.set(None);
            self.publish();
            return None;
        }

        let here = self.at.get().and_then(|(item, i)| {
            counted
                .iter()
                .position(|(id, _)| *id == item)
                .map(|r| (r, i))
        });
        let (row, index) = match here {
            Some((row, i)) => {
                let n = counted[row].1;
                if forward && i + 1 < n {
                    (row, i + 1)
                } else if !forward && i > 0 {
                    (row, i - 1)
                } else if forward {
                    let next = (row + 1) % counted.len();
                    (next, 0)
                } else {
                    let prev = (row + counted.len() - 1) % counted.len();
                    (prev, counted[prev].1 - 1)
                }
            }
            // Nowhere yet — or standing in a row the page no longer shows. Entering
            // forwards means the reading's first mention and backwards its last, the
            // mirror `FindSession`'s own entry rule states.
            None if forward => (0, 0),
            None => (counted.len() - 1, counted[counted.len() - 1].1 - 1),
        };

        let item = counted[row].0;
        self.at.set(Some((item, index)));
        // Through the same door a refresh uses, so a step and a frame cannot put the
        // layers in two different states.
        self.reapply();
        self.publish();
        let (start, length) = self.layers.borrow().get(&item)?.span(index)?;
        Some((item, start, length))
    }

    /// Mirror the counters.
    ///
    /// The ordinal is read back off **the layers**, not off this walk's own remembered
    /// index: [`reapply`](Self::reapply) has just written the cursor onto them, so what
    /// they are drawing is the answer by definition. Deriving it from the remembered pair
    /// instead is how a header came to say "3 of 17" over a page with no current mark on
    /// it, and "17 mentions" over one that had.
    ///
    /// Call it only after `reapply`.
    fn publish(&self) {
        let order = self.order.borrow();
        let layers = self.layers.borrow();
        let mut total = 0usize;
        let mut ordinal = 0usize;
        for id in order.iter() {
            let Some(layer) = layers.get(id) else {
                continue;
            };
            if let Some(i) = layer.current() {
                ordinal = total + i + 1;
            }
            total += layer.marks();
        }
        drop(layers);
        drop(order);
        let _ = self.total.set_if_changed(total);
        let _ = self.ordinal.set_if_changed(ordinal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(text: &str) -> TextDocument {
        let d = TextDocument::new();
        d.set_plain_text(text).unwrap();
        d
    }

    /// An entry as the matcher takes it: the first name is the title, the rest aliases.
    fn who(v: &[&str]) -> DiscoverableEntity {
        DiscoverableEntity {
            id: 99,
            title: v[0].to_string(),
            aliases: v[1..].iter().map(|s| s.to_string()).collect(),
        }
    }

    const BLUE: ThemeColor = ThemeColor::new(0.0, 0.5, 1.0, 1.0);

    fn fmt() -> HighlightFormat {
        subject_format(BLUE)
    }

    fn here() -> HighlightFormat {
        subject_current_format(BLUE)
    }

    /// A layer over `text`, dressed so a test can tell a resting mark from the one the
    /// reader is standing on.
    fn layer_over(d: &TextDocument) -> SubjectHighlight {
        SubjectHighlight::new(d, fmt(), here())
    }

    /// The marks land on the characters the names occupy — every alias, and the whole of
    /// a full name rather than the surname inside it.
    #[test]
    fn every_name_is_marked_where_it_is_written() {
        let d = doc("Lizzy waited. Elizabeth Bennet did not.");
        let layer = layer_over(&d);
        assert!(layer.refresh(Some(&who(&["Elizabeth Bennet", "Lizzy"]))));

        let marked: Vec<(usize, usize)> = layer
            .last
            .borrow()
            .iter()
            .map(|r| (r.start, r.length))
            .collect();
        assert_eq!(marked, vec![(0, 5), (14, 16)]);
    }

    /// **An edit re-derives rather than carrying offsets across it.** Text inserted ahead
    /// of a name moves it, and a layer that kept the old offsets would mark the
    /// wrong characters — the failure every range session in this app is built to avoid.
    #[test]
    fn an_edit_moves_the_marks_onto_the_text_that_moved() {
        let d = doc("Lizzy waited.");
        let layer = layer_over(&d);
        let subject = who(&["Lizzy"]);
        assert!(layer.refresh(Some(&subject)));
        assert_eq!(layer.last.borrow()[0].start, 0);

        d.set_plain_text("That morning, Lizzy waited.").unwrap();
        assert!(layer.refresh(Some(&subject)), "the edit staled the marks");
        assert_eq!(layer.last.borrow()[0].start, 14);
    }

    /// A rename re-derives without an edit: nothing touched the prose, but the entry now
    /// goes by another name and the reading is about that one.
    #[test]
    fn a_renamed_entry_re_derives_without_an_edit() {
        let d = doc("Lizzy teased Marcus.");
        let layer = layer_over(&d);
        assert!(layer.refresh(Some(&who(&["Lizzy"]))));
        assert_eq!(layer.marks(), 1);

        assert!(layer.refresh(Some(&who(&["Lizzy", "Marcus"]))));
        assert_eq!(layer.marks(), 2, "the new alias is marked too");
    }

    /// Called every frame over every row of a reading, so an unchanged one must cost
    /// nothing and must not push a repaint.
    #[test]
    fn an_unchanged_frame_pushes_nothing() {
        let d = doc("Lizzy waited.");
        let layer = layer_over(&d);
        let subject = who(&["Lizzy"]);
        assert!(layer.refresh(Some(&subject)));
        assert!(!layer.refresh(Some(&subject)));
        assert!(!layer.refresh(Some(&subject)));
    }

    /// An entry with no names marks nothing rather than everything — the state a note
    /// with a blank title is in while the writer is still typing it.
    #[test]
    fn an_unnamed_entry_marks_nothing() {
        let d = doc("Lizzy waited.");
        let layer = layer_over(&d);
        assert!(!layer.refresh(None));
        assert_eq!(layer.marks(), 0);
    }

    // ── the walk ────────────────────────────────────────────────────────────

    /// A reading of `rows`, each `(item, prose)`, with a layer over every one.
    fn reading(rows: &[(EntityId, &str)]) -> (SubjectWalk, Vec<TextDocument>) {
        let layers: Rc<RefCell<HashMap<EntityId, SubjectHighlight>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let mut docs = Vec::new();
        for (id, text) in rows {
            let d = doc(text);
            layers
                .borrow_mut()
                .insert(*id, SubjectHighlight::new(&d, fmt(), here()));
            docs.push(d);
        }
        let walk = SubjectWalk::new(layers);
        walk.set_order(rows.iter().map(|(id, _)| *id).collect());
        (walk, docs)
    }

    /// How many documents claim to hold the mention the reader is standing on. The
    /// invariant the whole walk rests on: never more than one, on a page of forty.
    fn standing_on(walk: &SubjectWalk) -> usize {
        walk.layers
            .borrow()
            .values()
            .filter(|l| l.current().is_some())
            .count()
    }

    /// **A reading counts as one run.** "17 mentions" is a fact about the page, not about
    /// whichever row the reader happens to be looking at.
    #[test]
    fn the_counter_spans_every_row_of_the_reading() {
        let (walk, _d) = reading(&[
            (1, "Lizzy waited."),
            (2, "Nobody came."),
            (3, "Lizzy, and Lizzy again."),
        ]);
        walk.refresh(Some(&who(&["Lizzy"])));
        assert_eq!(walk.total_signal().get(), 3);
        assert_eq!(
            walk.ordinal_signal().get(),
            0,
            "nobody has stepped anywhere yet, so there is no ordinal to report"
        );
    }

    /// Next walks off the end of a row into the next one that has a mention — skipping
    /// the row with none — and wraps. Exactly one row is standing on a mention at every
    /// step, which is what goes wrong if a row is left behind with a current mark.
    #[test]
    fn stepping_crosses_rows_and_wraps() {
        let (walk, _d) = reading(&[
            (1, "Lizzy waited."),
            (2, "Nobody came."),
            (3, "Lizzy, and Lizzy again."),
        ]);
        walk.refresh(Some(&who(&["Lizzy"])));

        assert_eq!(walk.step(true), Some((1, 0, 5)), "the reading's first");
        assert_eq!(walk.ordinal_signal().get(), 1);
        assert_eq!(standing_on(&walk), 1);

        assert_eq!(
            walk.step(true),
            Some((3, 0, 5)),
            "row 2 has none to stop at"
        );
        assert_eq!(walk.ordinal_signal().get(), 2);

        assert_eq!(walk.step(true), Some((3, 11, 5)), "still inside row 3");
        assert_eq!(walk.ordinal_signal().get(), 3);

        assert_eq!(walk.step(true), Some((1, 0, 5)), "wrapped to the top");
        assert_eq!(walk.ordinal_signal().get(), 1);
        assert_eq!(standing_on(&walk), 1);
    }

    /// Backwards is the mirror: stepping out of a row's first mention arrives at the
    /// **last** of the row above, and entering the reading backwards starts at its end.
    #[test]
    fn stepping_back_enters_a_row_at_its_last_mention() {
        let (walk, _d) = reading(&[(1, "Lizzy, and Lizzy again."), (2, "One Lizzy.")]);
        walk.refresh(Some(&who(&["Lizzy"])));

        assert_eq!(walk.step(false), Some((2, 4, 5)), "the reading's last");
        assert_eq!(walk.ordinal_signal().get(), 3);

        assert_eq!(walk.step(false), Some((1, 11, 5)), "row 1's *last*");
        assert_eq!(walk.ordinal_signal().get(), 2);
        assert_eq!(standing_on(&walk), 1);
    }

    /// A reading with nothing written — an entry declared only as a point of view — has
    /// nowhere to step, and must say so rather than pretending to move.
    #[test]
    fn a_reading_with_no_mention_has_nowhere_to_step() {
        let (walk, _d) = reading(&[(1, "Nobody came."), (2, "Still nobody.")]);
        walk.refresh(Some(&who(&["Lizzy"])));
        assert_eq!(walk.total_signal().get(), 0);
        assert_eq!(walk.step(true), None);
        assert_eq!(walk.step(false), None);
        assert_eq!(walk.ordinal_signal().get(), 0);
    }

    /// **An edit under the reader's feet must not strand the cursor.** Deleting the
    /// mention being stood on leaves an ordinal that names nothing, so it is dropped and
    /// the next step re-enters from the top rather than reporting a position that is gone.
    #[test]
    fn an_edit_that_deletes_the_current_mention_drops_the_ordinal() {
        let (walk, docs) = reading(&[(1, "Lizzy waited."), (2, "Lizzy again.")]);
        let subject = who(&["Lizzy"]);
        walk.refresh(Some(&subject));
        assert_eq!(walk.step(true), Some((1, 0, 5)));
        assert_eq!(walk.ordinal_signal().get(), 1);

        docs[0].set_plain_text("Nobody waited.").unwrap();
        walk.refresh(Some(&subject));
        assert_eq!(walk.total_signal().get(), 1, "only row 2's is left");
        assert_eq!(
            walk.ordinal_signal().get(),
            0,
            "the mention that was current is gone, so no ordinal names it"
        );
        assert_eq!(
            walk.step(true),
            Some((2, 0, 5)),
            "and stepping starts again"
        );
    }

    /// A row the page has stopped showing — a Book switch, a scene moved out — takes the
    /// cursor with it rather than leaving a step that reaches into a document nobody can
    /// see.
    #[test]
    fn a_row_that_leaves_the_reading_takes_the_cursor_with_it() {
        let (walk, _d) = reading(&[(1, "Lizzy waited."), (2, "Lizzy again.")]);
        let subject = who(&["Lizzy"]);
        walk.refresh(Some(&subject));
        assert_eq!(walk.step(true), Some((1, 0, 5)));

        walk.set_order(vec![2]);
        walk.refresh(Some(&subject));
        assert_eq!(walk.total_signal().get(), 1, "only the row still shown");
        assert_eq!(walk.ordinal_signal().get(), 0);
        assert_eq!(walk.step(true), Some((2, 0, 5)));
    }

    /// **An edit that shrinks a row must not leave the mark and the counter disagreeing.**
    ///
    /// A layer clamps its own current index when the text loses mentions, so it keeps
    /// drawing one. If the walk decided the ordinal from its *own* remembered index
    /// instead of from what the layers are drawing, it would report "no current mention"
    /// over a page that visibly has one — and stay wrong for every quiet frame after,
    /// because a refresh with nothing to re-derive short-circuits.
    #[test]
    fn an_edit_that_shrinks_a_row_keeps_the_mark_and_the_counter_together() {
        let (walk, docs) = reading(&[(1, "Lizzy, Lizzy and Lizzy.")]);
        let subject = who(&["Lizzy"]);
        walk.refresh(Some(&subject));
        walk.step(true);
        walk.step(true);
        walk.step(true);
        assert_eq!(walk.ordinal_signal().get(), 3);

        // Two of the three are gone; the row still has one.
        docs[0].set_plain_text("Lizzy waited.").unwrap();
        walk.refresh(Some(&subject));

        assert_eq!(walk.total_signal().get(), 1);
        assert_eq!(
            standing_on(&walk),
            1,
            "the row is still drawing a current mention"
        );
        assert_eq!(
            walk.ordinal_signal().get(),
            1,
            "and the counter names the one it is drawing, not the one that is gone"
        );
    }

    /// **A layer rebuilt from scratch gets the cursor back.**
    ///
    /// A theme switch throws every layer away and builds new ones, which start with no
    /// current mention at all. Without the cursor being re-applied the header would go on
    /// reporting "3 of 17" over a page where nothing is marked as current — the same
    /// disagreement as above, in the other direction.
    #[test]
    fn a_layer_rebuilt_from_scratch_gets_the_cursor_back() {
        let (walk, docs) = reading(&[(1, "Lizzy, and Lizzy again.")]);
        let subject = who(&["Lizzy"]);
        walk.refresh(Some(&subject));
        walk.step(true);
        walk.step(true);
        assert_eq!(walk.ordinal_signal().get(), 2);

        // What a theme change does: the map is cleared and refilled with fresh layers.
        walk.layers.borrow_mut().clear();
        walk.layers
            .borrow_mut()
            .insert(1, SubjectHighlight::new(&docs[0], fmt(), here()));
        walk.refresh(Some(&subject));

        assert_eq!(standing_on(&walk), 1, "something is drawn as current again");
        assert_eq!(
            walk.ordinal_signal().get(),
            2,
            "and it is the mention the reader had stepped to"
        );
    }

    /// A settings store of its own, on a temp file it does not share.
    fn temp_store() -> teksilo::settings::SettingsStore {
        use std::sync::atomic::{AtomicU32, Ordering};
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_subject_highlight_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        teksilo::settings::SettingsStore::open(path).expect("open temp settings store")
    }

    /// **One switch, both surfaces.** The strip and the wash are two views of one
    /// finding, so the provider's own setting governs both — and it has to be *that*
    /// setting, read through the registered spec, not a second key spelled out here that
    /// would drift from it the first time either moved.
    #[test]
    fn the_lane_providers_switch_governs_the_marks_too() {
        let store = temp_store();
        assert!(
            subject_enabled(&store),
            "with no provider registered the marks are on, which is what a headless \
             build and a fresh install both are"
        );

        let _installed = crate::margin_lane::install_builtin_providers();
        assert!(subject_enabled(&store), "and on by default once registered");

        let key = crate::margin_lane::registered_for(crate::margin_lane::LaneSurface::Stream)
            .iter()
            .find(|s| s.id == crate::margin_lane::providers::STORY_BIBLE_PROVIDER_ID)
            .expect("the provider is registered")
            .settings_key();
        store.signal(&key, true).set(false);
        assert!(
            !subject_enabled(&store),
            "turning the strip off must take the marks with it"
        );
    }

    /// **The layer does not outlive the widget that made it.** A reading closed must
    /// leave no marks on documents the rest of the app is still showing.
    #[test]
    fn dropping_the_layer_takes_its_marks_off_the_document() {
        let d = doc("Lizzy waited.");
        {
            let layer = layer_over(&d);
            assert!(layer.refresh(Some(&who(&["Lizzy"]))));
            assert!(!spans(&d).is_empty());
        }
        assert!(
            spans(&d).is_empty(),
            "the reading's marks must not survive it"
        );
    }

    fn spans(doc: &TextDocument) -> Vec<teksilo::text_document::PaintHighlightSpan> {
        use teksilo::text_document::{FlowElementSnapshot, HighlightMask};
        match &doc.snapshot_flow_masked(&HighlightMask::all()).elements[0] {
            FlowElementSnapshot::Block(b) => b.paint_highlights.clone(),
            _ => panic!("block"),
        }
    }
}
