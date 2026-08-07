// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One reload per burst of backend events, not one per event.
//!
//! ## The measurement this exists because of
//!
//! `import_management`'s `a_batched_import_fires_at_most_one_event_per_row`
//! measured what a bulk operation actually publishes: **exactly one
//! `BinderItem` event per created row** — 20 rows, 20 events; 200 rows, 200
//! events. `create_multi` batches the *store writes*, and `EventBuffer` holds
//! every event until commit so they arrive together rather than interleaved
//! with the work, but nothing merges them. There is no bulk event, and there
//! should not be: an event names one entity, which is what makes it useful to
//! everything else that listens.
//!
//! What that cost, before this module: three models subscribed to those origins
//! and called a **full reload** on each one, and a reload re-queries the whole
//! binder from the backend and rebuilds a tree from it. Importing 200 rows into
//! a 4,000-item project re-read and rebuilt that binder 200 times, in one burst,
//! while the writer watched. Trash, Empty Trash, restore, duplicate and
//! multi-select move all paid the same price through the same models — the
//! import is simply where it was finally measured.
//!
//! ## What it does instead
//!
//! An event marks the model stale and asks for a frame. The next frame reloads
//! it, once, however many events arrived. A burst of 200 collapses to one
//! reload; a single rename still reloads on the very next frame, which is the
//! same frame it would have repainted on anyway.
//!
//! Deliberately **not** a time-based throttle like
//! [`MentionIndex::rescan_throttled`](crate::view_models::MentionIndex). That
//! shape is right for an expensive *background* scan whose result can lag, and
//! wrong for the binder tree: a throttle either makes a rename take its interval
//! to appear, or fires immediately and coalesces nothing. A frame is the natural
//! quantum here, because a frame is exactly how often the result could be seen.
//!
//! ## Why `wake_at` and not just `frame_tick`
//!
//! Teksilo only pumps frames when something asks for one (see
//! [`crate::view_models::timers`]'s module doc). An effect registered on
//! `frame_tick` alone would run when a frame happened to be pumped for some
//! other reason — which, for a background import into a window nobody is
//! touching, could be never. Marking stale therefore also arms a wake for *now*,
//! the same one-shot handle the autosave and the save indicator use, so the loop
//! pumps exactly one frame and goes back to sleep.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use teksilo::prelude::*;

use frontend::common::event::{Event, Origin};

/// A shared "something changed; reload on the next frame" flag.
///
/// Cloneable and cheap: the event closures and the frame-tick effect all hold
/// the same cell.
#[derive(Clone, Default)]
pub(crate) struct ReloadCoalescer {
    stale: Rc<Cell<bool>>,
}

impl ReloadCoalescer {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Note that a reload is owed, and make sure a frame arrives to do it on.
    fn mark(&self, wake: &Rc<Cell<Option<Instant>>>) {
        self.stale.set(true);
        // Now, not "soon": this is a coalescer, not a debounce. The point is to
        // do the reload at the first opportunity and no more than once, never to
        // make the writer wait for it.
        wake.set(Some(Instant::now()));
    }

    /// Take the pending reload, if there is one.
    fn take(&self) -> bool {
        self.stale.replace(false)
    }
}

/// Reload `on_reload` when any of `origins` fires — **once per frame**, however
/// many of them arrive.
///
/// Call from a model's `wire`, in place of a `subscribe_event` loop that
/// reloads directly. See the module doc for what that loop cost.
///
/// The reload runs on a frame, so a caller that must observe the effect
/// immediately (a test, or a mutation whose result is read back in the same
/// call) should keep calling its own `reload()` directly — this governs the
/// *event-driven* path only, and leaves the imperative one alone.
/// Returns the coalescer so a test can drive the frame half for real — a
/// headless `WidgetTree` drops backend events (`test_support`'s `NullPoster`),
/// so marking by hand and pumping `frame_tick` is the only way to exercise the
/// effect that was actually registered rather than a re-implementation of it.
pub(crate) fn reload_on_events(
    ctx: &mut BuildContext,
    origins: impl IntoIterator<Item = Origin>,
    on_reload: impl Fn() + 'static,
) -> ReloadCoalescer {
    let coalescer = ReloadCoalescer::new();
    let wake = ctx.wake_at_handle();

    for origin in origins {
        let coalescer = coalescer.clone();
        let wake = wake.clone();
        ctx.subscribe_event(origin, move |_e: &Event| coalescer.mark(&wake));
    }

    let tick = ctx.frame_tick();
    {
        let coalescer = coalescer.clone();
        ctx.effect(&tick, move |_| {
            if coalescer.take() {
                on_reload();
            }
        });
    }
    coalescer
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole point: many marks, one reload.
    #[test]
    fn a_burst_of_events_owes_exactly_one_reload() {
        let coalescer = ReloadCoalescer::new();
        let wake: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));

        for _ in 0..200 {
            coalescer.mark(&wake);
        }

        assert!(coalescer.take(), "the frame owes a reload");
        assert!(
            !coalescer.take(),
            "and only one — a second frame must not reload again"
        );
    }

    /// A quiet frame must not reload, or the model would re-query the backend on
    /// every frame the app happens to pump for something else.
    #[test]
    fn a_frame_with_nothing_pending_does_not_reload() {
        assert!(!ReloadCoalescer::new().take());
    }

    /// Marking always arms a wake, because teksilo pumps no frames of its own —
    /// without this, a burst arriving while the window is idle would sit unread
    /// until something unrelated woke the loop.
    #[test]
    fn marking_asks_for_a_frame_to_reload_on() {
        let coalescer = ReloadCoalescer::new();
        let wake: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));
        assert!(wake.get().is_none());

        coalescer.mark(&wake);
        assert!(wake.get().is_some(), "a stale model must ask for a frame");
    }

    /// Staleness survives across bursts: an event arriving *after* a reload owes
    /// the next frame another one.
    #[test]
    fn an_event_after_a_reload_owes_another() {
        let coalescer = ReloadCoalescer::new();
        let wake: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));

        coalescer.mark(&wake);
        assert!(coalescer.take());

        coalescer.mark(&wake);
        assert!(coalescer.take(), "the second burst is not swallowed");
    }

    // ── the wiring, driven for real ─────────────────────────────────────────
    //
    // The three tests above pin the flag's logic; on their own they would still
    // pass if `reload_on_events` never registered the effect at all. These build
    // a real widget through a real `WidgetTree`, so what runs is the closure
    // that was actually handed to `ctx.effect`.

    use teksilo::core::widget_tree::WidgetTree;
    use std::rc::Rc as StdRc;

    /// A widget that wires one coalescer and counts its reloads.
    struct Probe {
        reloads: StdRc<Cell<usize>>,
        coalescer: StdRc<RefCell<Option<ReloadCoalescer>>>,
    }

    use std::cell::RefCell;

    impl std::fmt::Debug for Probe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Probe").finish()
        }
    }

    impl Widget for Probe {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let reloads = self.reloads.clone();
            let coalescer = reload_on_events(
                ctx,
                [Origin::DirectAccess(
                    frontend::common::event::DirectAccessEntity::BinderItem(
                        frontend::common::event::EntityEvent::Created,
                    ),
                )],
                move || reloads.set(reloads.get() + 1),
            );
            *self.coalescer.borrow_mut() = Some(coalescer);
            Vec::new()
        }

        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    fn probe() -> (WidgetTree, StdRc<Cell<usize>>, ReloadCoalescer) {
        let app_ctx = StdRc::new(frontend::AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let reloads = StdRc::new(Cell::new(0));
        let slot = StdRc::new(RefCell::new(None));
        tree.add_boxed(Box::new(Probe {
            reloads: reloads.clone(),
            coalescer: slot.clone(),
        }));
        let coalescer = slot.borrow().clone().expect("wire ran during build");
        (tree, reloads, coalescer)
    }

    /// Two hundred events, one reload. The measurement that started this
    /// (`import_management::a_batched_import_fires_at_most_one_event_per_row`)
    /// says a 200-row import publishes exactly 200 events; before this, each one
    /// re-queried the whole binder.
    #[test]
    fn two_hundred_events_reload_the_model_once() {
        let (tree, reloads, coalescer) = probe();
        let wake: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));

        for _ in 0..200 {
            coalescer.mark(&wake);
        }
        assert_eq!(reloads.get(), 0, "nothing reloads until a frame arrives");

        tree.frame_tick().set(16.0);
        assert_eq!(reloads.get(), 1, "the whole burst cost one reload");

        tree.frame_tick().set(16.0);
        assert_eq!(reloads.get(), 1, "a later quiet frame reloads nothing");
    }

    /// …and a single edit is not swallowed by the deferral. This is the failure
    /// that would matter most: a coalescer that never fires looks exactly like a
    /// binder tree that has stopped noticing renames.
    #[test]
    fn one_event_still_reloads_on_the_very_next_frame() {
        let (tree, reloads, coalescer) = probe();
        let wake: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));

        coalescer.mark(&wake);
        tree.frame_tick().set(16.0);
        assert_eq!(reloads.get(), 1);

        coalescer.mark(&wake);
        tree.frame_tick().set(16.0);
        assert_eq!(reloads.get(), 2, "the next edit reloads too");
    }
}
