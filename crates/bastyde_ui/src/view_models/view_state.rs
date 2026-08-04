// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Per-document **view state**: where the caret sits and how far the page is
//! scrolled.
//!
//! This is *view* state, not document state — two split panes showing the same
//! item have one `TextDocument` but two carets — so it hangs off [`crate::tabs::ContentTab`]
//! (the per-pane view), never off the shared `OpenDoc` beside the spell and
//! replacement sessions.
//!
//! Two consumers, which is why it lives in its own file rather than inside
//! `tabs.rs`:
//!
//! * the distraction-free surface, which mounts a second editor over the same
//!   `OpenDoc` and has to hand the caret across on the way in and back on the
//!   way out;
//! * `workspace.toml`, which persists it per tab so reopening a project puts
//!   the writer back where they stopped.
//!
//! **Caret and scroll come from two different places**, which is the one thing
//! about this that is not obvious. A writing page's editors are intrinsic-height
//! with their own scroll bars suppressed (`writing_column` sets
//! `ScrollPolicy::AlwaysOff`); the thing that actually scrolls is the page's
//! outer `ScrollArea` in `panes::writing_page_scroll`. So `editor.scroll_y()` on
//! the prose editor is permanently 0 and reading it would silently persist
//! nothing. The caret comes from the editor handle, the scroll from the page.
//! [`ViewStatePorts`] is where the two meet.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::Signal;
use bastyde::widgets::rich_text::EditorHandle;

/// Where the writer was in one document: the caret's character offset, and the
/// page's vertical scroll offset in logical pixels.
///
/// `caret` is a character index, so it survives a typography change (a different
/// face or size moves pixels, not offsets). `scroll` is in pixels and does not —
/// see [`ViewStatePorts::apply`] on why it is clamped rather than trusted.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ViewState {
    pub caret: usize,
    pub scroll: f32,
}

/// The page `ScrollArea`'s two live signals: its offset, and the maximum the
/// offset may take (`content_height − viewport_height`, 0 while the content has
/// not been laid out yet).
#[derive(Clone)]
struct PageScroll {
    offset: Signal<f32>,
    max: Signal<f32>,
}

/// The live wiring a mounted pane publishes so its view state can be read back
/// and written to.
///
/// Both slots are re-attached on **every** build, exactly as
/// `ContentTab::synopsis_handle` and the find banner's prose handle are: a tab
/// rebuild mints a fresh editor and a fresh `ScrollArea`, so a stored handle
/// would address the one the writer *used* to be typing in. Both are `None`
/// until the pane first builds, and a tab whose combination has no main prose
/// field (a heading, a placeholder) never fills the editor slot at all.
#[derive(Default)]
pub struct ViewStatePorts {
    editor: RefCell<Option<EditorHandle>>,
    page_scroll: RefCell<Option<PageScroll>>,
}

impl ViewStatePorts {
    /// Publish the main prose editor's handle. Called from `writing_column`.
    pub fn attach_editor(&self, handle: EditorHandle) {
        *self.editor.borrow_mut() = Some(handle);
    }

    /// Publish the page `ScrollArea`'s offset and maximum. Called from
    /// `panes::writing_page_scroll`, the one door every writing surface's scroll
    /// goes through.
    pub fn attach_page_scroll(&self, offset: Signal<f32>, max: Signal<f32>) {
        *self.page_scroll.borrow_mut() = Some(PageScroll { offset, max });
    }

    /// The main prose editor's handle, if this pane currently has one built.
    pub fn editor(&self) -> Option<EditorHandle> {
        self.editor.borrow().clone()
    }

    /// The page's maximum scroll offset — what a deferred restore waits on. A
    /// scroll written before the content has been laid out is clamped to 0 by
    /// `ScrollArea` (`clamp_and_set_scroll`), so restoring one means waiting for
    /// this to become non-zero rather than writing it at build time.
    pub fn max_scroll(&self) -> Option<Signal<f32>> {
        self.page_scroll.borrow().as_ref().map(|p| p.max.clone())
    }

    /// Read the live caret and scroll off the mounted pane.
    ///
    /// `fallback` supplies either half this pane cannot answer — a tab that has
    /// never been built, or a combination with no prose editor — so a capture
    /// taken before the writer ever looked at a tab returns what was seeded
    /// rather than zeroing it. Without that, opening a project and saving it
    /// without visiting a restored tab would erase that tab's remembered
    /// position.
    pub fn capture(&self, fallback: ViewState) -> ViewState {
        ViewState {
            caret: self
                .editor
                .borrow()
                .as_ref()
                .map(|h| h.cursor_position())
                .unwrap_or(fallback.caret),
            scroll: self
                .page_scroll
                .borrow()
                .as_ref()
                .map(|p| p.offset.get())
                .unwrap_or(fallback.scroll),
        }
    }

    /// Push `state` onto the mounted pane: collapse the selection to the caret,
    /// and scroll the page.
    ///
    /// `max_caret` is the document's character count; the caret is clamped to it
    /// because a document can have been edited in another window (or another
    /// pane) between capture and restore, and a stale offset past the end would
    /// otherwise land somewhere arbitrary.
    ///
    /// The scroll is clamped to the page's current maximum for the same reason
    /// *and* a structural one: while the content is still unlaid-out that maximum
    /// is 0, so an unclamped write would be silently reset by `ScrollArea` and
    /// the caller would have no way to tell it had been dropped. Callers
    /// restoring into a freshly-built pane should wait on [`Self::max_scroll`]
    /// instead of applying immediately.
    /// Scroll the page only, clamped to its current maximum.
    ///
    /// Split out from [`Self::apply`] for the deferred restore: a scroll written
    /// before the content has laid out is clamped to 0 and lost, so the caller
    /// waits on [`Self::max_scroll`] and then applies just this half — the caret
    /// went in at build time and must not be re-applied over a writer who has
    /// since moved it.
    pub fn apply_scroll(&self, scroll: f32) {
        if let Some(page) = self.page_scroll.borrow().as_ref() {
            page.offset.set(scroll.clamp(0.0, page.max.get()));
        }
    }

    pub fn apply(&self, state: ViewState, max_caret: usize) {
        if let Some(handle) = self.editor.borrow().as_ref() {
            let caret = state.caret.min(max_caret);
            handle.select_range(caret, caret);
        }
        if let Some(page) = self.page_scroll.borrow().as_ref() {
            page.offset.set(state.scroll.clamp(0.0, page.max.get()));
        }
    }
}

/// What a pane is handed so it can publish its view state and seed itself from a
/// remembered one.
///
/// `initial` is read **once**, as the pane builds; it is not a live signal,
/// because a caret that moved under the writer whenever a sibling wrote to a
/// settings file would be worse than useless.
#[derive(Clone)]
pub struct ViewStateBinding {
    pub initial: ViewState,
    pub ports: Rc<ViewStatePorts>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_falls_back_for_a_pane_that_never_built() {
        // The workspace capture runs on every successful save, including for
        // tabs the writer restored but never clicked into. Those have no editor
        // and no scroll area, and must report what they were seeded with — not
        // zero, which would quietly erase the remembered position.
        let ports = ViewStatePorts::default();
        let seeded = ViewState {
            caret: 412,
            scroll: 96.0,
        };
        assert_eq!(ports.capture(seeded), seeded);
    }

    #[test]
    fn apply_is_a_no_op_on_an_unbuilt_pane() {
        // Seeding a tab before its pane exists is the normal restore path; it
        // must not panic on the empty slots.
        let ports = ViewStatePorts::default();
        ports.apply(
            ViewState {
                caret: 10,
                scroll: 20.0,
            },
            100,
        );
        assert!(ports.editor().is_none());
    }

    #[test]
    fn a_default_view_state_is_the_top_of_the_document() {
        let s = ViewState::default();
        assert_eq!(s.caret, 0);
        assert_eq!(s.scroll, 0.0);
    }

    // The scroll half is unit-tested here against hand-made signals rather than
    // against a mounted pane. A headless tree's text backend reports the prose
    // editor's `min_lines` height rather than a faithful one, so a mounted page
    // never has anywhere to scroll and an integration assertion would pass
    // vacuously. What a mounted pane *is* worth asserting — that both ports get
    // attached at all — is covered by
    // `tabs::tests::a_mounted_prose_pane_publishes_both_view_state_ports`.

    fn ports_with_scroll(max: f32) -> (ViewStatePorts, Signal<f32>) {
        let ports = ViewStatePorts::default();
        let offset = Signal::new(0.0);
        ports.attach_page_scroll(offset.clone(), Signal::new(max));
        (ports, offset)
    }

    #[test]
    fn scroll_round_trips_through_the_page_port() {
        let (ports, _offset) = ports_with_scroll(500.0);
        ports.apply(
            ViewState {
                caret: 0,
                scroll: 120.0,
            },
            0,
        );
        assert_eq!(ports.capture(ViewState::default()).scroll, 120.0);
    }

    #[test]
    fn a_scroll_past_the_end_of_the_page_is_clamped() {
        // A shorter document than the one the offset was captured against —
        // text deleted in another window, or a wider window reflowing the prose
        // into fewer lines. `ScrollArea` would clamp this itself; doing it here
        // too keeps `capture` immediately after `apply` honest.
        let (ports, _offset) = ports_with_scroll(80.0);
        ports.apply(
            ViewState {
                caret: 0,
                scroll: 400.0,
            },
            0,
        );
        assert_eq!(ports.capture(ViewState::default()).scroll, 80.0);
    }

    #[test]
    fn a_scroll_written_before_layout_is_clamped_to_zero_not_kept() {
        // The trap this whole port exists to make visible: until the page has
        // been laid out its maximum is 0, so an offset written at build time is
        // dropped. A restore has to wait on `max_scroll` instead — see
        // `ViewStatePorts::apply`.
        let (ports, _offset) = ports_with_scroll(0.0);
        ports.apply(
            ViewState {
                caret: 0,
                scroll: 250.0,
            },
            0,
        );
        assert_eq!(ports.capture(ViewState::default()).scroll, 0.0);
    }
}
