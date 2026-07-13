//! `SaveIndicator` — the status bar's save state, next to the binder toggle.
//!
//! A flat icon button showing whether the manuscript is on disk: a save glyph with
//! an **asterisk** while there are unsaved changes, with a **check** once the write
//! has landed, and a spinner while a genuinely slow write is running (see
//! [`SpinnerGate`] — a local save is far too fast to be worth a flicker). Clicking
//! saves.
//!
//! Deliberately *not* a toast: a save happens every few seconds, and a notification
//! that often is noise in the corner of the eye of someone trying to write. A
//! **failed** save is a toast, though (`App`'s long-operation `Failed` handler) — a
//! write that didn't happen is not a quiet fact — so the indicator has no failure
//! state to sit on.
//!
//! It fills a real hole: the only save feedback the app has ever had is the Save
//! affordance greying out — and under **autosave that affordance doesn't exist**
//! (the menu item and Ctrl+S are hidden), so there was no way at all to tell
//! whether the last paragraph was safe. The indicator stays visible under autosave
//! for exactly that reason, but disabled: the timer owns the saving, and a button
//! that looks live while doing nothing reads as a bug.
//!
//! Thin, per the house rules: every decision is the pure
//! [`crate::view_models::save_status`] table, and the write itself is
//! `EditorsViewModel::request_save` (which flushes the editors, coalesces against
//! any save already in flight, and reports failures).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use bastyde::core::BindingLevel;
use bastyde::core::color_prop::ColorProp;
use bastyde::prelude::*;
use bastyde::widgets::{FixedSize, IconButton, IconButtonSize, Spinner};

use crate::view_models::{EditorsViewModel, SaveStatus, SpinnerGate, save_clickable, save_status};

/// Keeps the status bar's other items from shifting as the glyph swaps between the
/// icon button and the spinner.
const SLOT: f32 = 28.0;
const SPINNER_SIZE: f32 = 14.0;

pub struct SaveIndicator {
    editors: EditorsViewModel,
    /// Derived in `App`: the work has edits not yet on disk (`dirty_seq >
    /// saved_seq`), so it stays true while a save that predates the latest edits is
    /// still running.
    unsaved: Signal<bool>,
    autosave: Signal<bool>,
    /// A read-only backup file is open: saving is inert, so the indicator hides
    /// (the banner already explains it, and offers Save As / Restore).
    backup_mode: Signal<bool>,
    /// A project is open at all.
    has_work: Signal<bool>,
    /// Hysteresis for the "saving…" spinner. **Owned by `App`**, not by this widget:
    /// `App::build` constructs a fresh `SaveIndicator` every time it runs, so a gate
    /// living here would have its delay/min-display timers reset by an unrelated App
    /// rebuild in the middle of a slow save — the spinner would vanish and have to
    /// re-earn its 200 ms.
    gate: Rc<RefCell<SpinnerGate>>,
    /// The gate's answer, as a signal, so revealing/hiding the spinner rebuilds us.
    /// Owned by `App` for the same reason.
    spinner_visible: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl SaveIndicator {
    pub fn new(
        editors: EditorsViewModel,
        unsaved: Signal<bool>,
        autosave: Signal<bool>,
        backup_mode: Signal<bool>,
        has_work: Signal<bool>,
        gate: Rc<RefCell<SpinnerGate>>,
        spinner_visible: Signal<bool>,
    ) -> Self {
        Self {
            editors,
            unsaved,
            autosave,
            backup_mode,
            has_work,
            gate,
            spinner_visible,
            root_child: None,
        }
    }

    /// Advance the spinner gate against the clock, arm the next wake-up if its state
    /// changes on its own, and publish its answer.
    fn poll_gate(gate: &Rc<RefCell<SpinnerGate>>, visible: &Signal<bool>, wake: &WakeAt) {
        let mut g = gate.borrow_mut();
        if let Some(at) = g.poll(Instant::now()) {
            wake.set(Some(at));
        }
        let now_visible = g.visible();
        drop(g);
        if visible.get() != now_visible {
            visible.set(now_visible);
        }
    }

    fn render(&self, ctx: &mut BuildContext, status: SaveStatus) -> Option<WidgetId> {
        match status {
            // No project, or a read-only backup: nothing to say, and no width taken.
            SaveStatus::Hidden => None,
            SaveStatus::Saving => Some(
                ctx.add(
                    FixedSize::new()
                        .width(SLOT)
                        .child(Spinner::new(SPINNER_SIZE).label(tr!(statusbar_saving()))),
                ),
            ),
            SaveStatus::Unsaved | SaveStatus::Saved => {
                let autosave = self.autosave.get();
                let unsaved = status == SaveStatus::Unsaved;
                let icon = if unsaved {
                    crate::editor_icons::save_unsaved()
                } else {
                    crate::editor_icons::save_saved()
                };
                // **Warning, not Error**, for unsaved: having unsaved work is the
                // normal state of a writing session, not a fault. The error role's
                // whole job is to be alarming, and a theme that does it well would
                // then flash an alarm on the first keystroke of every paragraph —
                // while the genuine failures it exists for now live in toasts. Amber
                // says "pending"; the shape (asterisk vs check) carries the meaning
                // regardless of colour.
                //
                // **Undimmed**, because this colour is a statement, not an affordance.
                // The button is disabled whenever there is nothing to write (a clean
                // project) or the timer owns the saving (autosave) — and a role-based
                // colour normally resolves to `text_disabled` in a disabled subtree,
                // which would grey out the check exactly when it is saying the thing
                // a writer most wants to know. `ColorProp::undimmed` is the framework
                // opt-out for that (added for this).
                let role = ColorProp::undimmed(if unsaved {
                    TextRole::Warning
                } else {
                    TextRole::Success
                });
                let tooltip = match (unsaved, autosave) {
                    (_, true) => tr!(statusbar_save_autosave()),
                    (true, false) => tr!(statusbar_save_unsaved()),
                    (false, false) => tr!(statusbar_save_saved()),
                };
                let editors = self.editors.clone();
                Some(
                    ctx.add(
                        FixedSize::new().width(SLOT).child(
                            IconButton::new(icon)
                                .size(IconButtonSize::Compact)
                                .icon_role(role)
                                .enabled(save_clickable(status, autosave))
                                .tooltip(tooltip)
                                .on_activate_fn(move |_| editors.save_to_disk()),
                        ),
                    ),
                )
            }
        }
    }
}

/// The shared one-shot deadline cell (`ctx.wake_at_handle()`).
type WakeAt = Rc<std::cell::Cell<Option<Instant>>>;

impl std::fmt::Debug for SaveIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaveIndicator").finish()
    }
}

impl Widget for SaveIndicator {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        let saving = self.editors.saving();
        for sig in [
            &self.unsaved,
            &self.autosave,
            &self.backup_mode,
            &self.has_work,
            &self.spinner_visible,
        ] {
            sig.bind_to(sid, reg, BindingLevel::Rebuild);
        }
        // `saving` itself is NOT bound: the spinner is gated (a fast save must not
        // rebuild the status bar at all). `spinner_visible` above is the gate's
        // answer, and that is what we rebuild on.

        // Drive the gate's delay/minimum off the frame clock, but keep the loop
        // asleep between the two instants that matter — the same `wake_at` idiom as
        // the autosave debounce in `App`. Effects are re-registered per build and
        // cleaned up on rebuild; the gate they drive is owned by `App` and outlives
        // both this widget's rebuilds and App's own, so no rebuild can reset a
        // slow save's timers mid-flight.
        let wake: WakeAt = ctx.wake_at_handle();
        {
            let gate = self.gate.clone();
            let visible = self.spinner_visible.clone();
            let wake = wake.clone();
            ctx.effect(&saving, move |on| {
                gate.borrow_mut().set_saving(*on, Instant::now());
                Self::poll_gate(&gate, &visible, &wake);
            });
        }
        {
            let gate = self.gate.clone();
            let visible = self.spinner_visible.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| Self::poll_gate(&gate, &visible, &wake));
        }

        let status = save_status(
            self.has_work.get(),
            self.backup_mode.get(),
            self.spinner_visible.get(),
            self.unsaved.get(),
        );
        self.root_child = self.render(ctx, status);
        self.root_child.into_iter().collect()
    }

    /// Zero-sized when hidden (no project / a backup window) — it takes no width of
    /// its own there. (It is still a child of the status bar's `HStack`, so that
    /// row's 8 dp spacing is still applied around it: hiding the glyph leaves an
    /// empty gap rather than closing it up entirely.)
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
