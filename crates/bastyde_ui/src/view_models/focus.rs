// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Distraction-free mode (Increment 2 of distraction-free — chrome collapse,
//! not the plain fullscreen from Increment 1, though it reuses the same
//! `WindowPlacement` mechanism to also go fullscreen when it turns on).
//!
//! **Per-window, not shared** — the same shape as [`crate::view_models::FullscreenViewModel`]
//! (see its module doc for the full rationale): [`FocusViewModel::new`] is
//! minted fresh for each project window in
//! `shell::windows::ProjectWindowFactory::window_config`, so one window's
//! distraction-free toggle never answers with — or clobbers — a sibling
//! window's chrome/placement memory.
//!
//! **Independent of `FullscreenViewModel`.** Shift+F11 (this mode) and plain
//! F11 (Increment 1) are two separate per-window toggles that both end up
//! writing the same window's `WindowPlacement` — deliberately not sharing one
//! "remembered placement" signal, so pressing one never corrupts the other's
//! restore target. Each toggle only ever restores what *it itself* last
//! remembered.
//!
//! **Reset on Close-Work/Load-Work, exactly like `AppIds`.** The process
//! outlives a single project (a project window is replaced in place, or
//! closed and a fresh Launcher window opens — see `main.rs`'s module doc), so
//! a `FocusViewModel` that isn't explicitly re-seeded would carry a stale
//! "mode was on" into the next project this window shows. [`Self::reset`] is
//! wired into this window's own `LoadWork`/`NewWork`/`CloseWork` subscribers
//! in `App::build`, the same guarded pattern `search`'s
//! `restore_for_project`/`clear_preview` already uses. It deliberately does
//! **not** touch the window's placement — those subscribers run from a plain
//! `ctx.subscribe_event`, which hands back only the `Event`, never an
//! `EventContext`/`WindowState` to act on; forgetting the mode's own
//! bookkeeping is enough to stop it leaking into the next project, and the
//! window's real OS placement is left for the user's own next F11/Escape.

use bastyde::prelude::{Signal, WindowPlacement, WindowState};

/// Pure decision: given whether the mode is being entered or left, the
/// window's **current** placement (read fresh — see
/// `next_fullscreen_state`'s doc for why that self-corrects a placement
/// change left behind the app's back), and whatever this window remembered
/// from its last entry, decide the placement to *apply* (`None` = leave the
/// window's placement alone) and the memory to keep for the next toggle.
///
/// Entering while **already** fullscreen (e.g. the plain F11 toggle got
/// there first) remembers `None` rather than the current `Fullscreen` value:
/// leaving distraction-free mode later must not un-fullscreen a window this
/// mode never itself fullscreened — that fullscreen belongs to the other
/// toggle, and only *it* should ever undo it.
fn next_focus_placement(
    entering: bool,
    current: WindowPlacement,
    remembered: Option<WindowPlacement>,
) -> (Option<WindowPlacement>, Option<WindowPlacement>) {
    if entering {
        if current == WindowPlacement::Fullscreen {
            (None, None)
        } else {
            (Some(WindowPlacement::Fullscreen), Some(current))
        }
    } else {
        match remembered {
            Some(p) => (Some(p), None),
            None => (None, None),
        }
    }
}

/// Owns one project window's distraction-free state: whether it is active
/// (drives the chrome/dock `VisibleWhen` gates in `shell::windows` and
/// `App::build`) and what placement to restore on exit.
#[derive(Clone)]
pub struct FocusViewModel {
    active: Signal<bool>,
    remembered: Signal<Option<WindowPlacement>>,
}

impl FocusViewModel {
    pub fn new() -> Self {
        Self {
            active: Signal::new(false),
            remembered: Signal::new(None),
        }
    }

    /// Reactive "is this window in distraction-free mode" flag — bound (not
    /// observed) by the chrome-collapse gates and the View-menu checkmark.
    pub fn active_signal(&self) -> Signal<bool> {
        self.active.clone()
    }

    /// Toggle distraction-free mode on `window` — resolved by the caller from
    /// `EventContext::window()`, so this always acts on the window that fired
    /// the command, correct with several project windows open.
    pub fn toggle(&self, window: &WindowState) {
        let entering = !self.active.get();
        let (placement, remembered) =
            next_focus_placement(entering, window.placement().get(), self.remembered.get());
        if let Some(p) = placement {
            window.placement().set(p);
        }
        self.remembered.set(remembered);
        self.active.set(entering);
    }

    /// Leave the mode unconditionally — the Escape / strip Exit-button path.
    /// Unlike [`Self::toggle`], a second call while already inactive is a
    /// harmless no-op: neither Escape nor the Exit button must ever be able
    /// to toggle the mode back *on*.
    pub fn exit(&self, window: &WindowState) {
        if self.active.get() {
            self.toggle(window);
        }
    }

    /// Forget this window's distraction-free state — Close-Work/Load-Work
    /// (see this module's doc for why, and why it cannot also restore the
    /// window's placement). Idempotent.
    pub fn reset(&self) {
        self.active.set(false);
        self.remembered.set(None);
    }
}

impl Default for FocusViewModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::{BastydeWindowId, WindowStateInit};

    fn test_window(placement: WindowPlacement) -> WindowState {
        WindowState::new(WindowStateInit {
            id: BastydeWindowId::new(1),
            string_id: None,
            placement,
            title: "Test".to_string(),
            size: (800, 600),
            position: (0, 0),
            focused: false,
            resizable: true,
            always_on_top: false,
        })
    }

    // ── the pure decision ────────────────────────────────────────────────

    #[test]
    fn entering_remembers_the_prior_placement_and_goes_fullscreen() {
        let (placement, remembered) = next_focus_placement(true, WindowPlacement::Maximized, None);
        assert_eq!(placement, Some(WindowPlacement::Fullscreen));
        assert_eq!(remembered, Some(WindowPlacement::Maximized));
    }

    #[test]
    fn entering_while_already_fullscreen_leaves_placement_alone_and_remembers_nothing() {
        // The plain F11 toggle (Increment 1) got here first. Distraction-free
        // must not treat "already fullscreen" as something *it* needs to undo
        // later.
        let (placement, remembered) = next_focus_placement(true, WindowPlacement::Fullscreen, None);
        assert_eq!(placement, None);
        assert_eq!(remembered, None);
    }

    #[test]
    fn leaving_restores_the_remembered_placement() {
        let (placement, remembered) = next_focus_placement(
            false,
            WindowPlacement::Fullscreen,
            Some(WindowPlacement::Maximized),
        );
        assert_eq!(placement, Some(WindowPlacement::Maximized));
        assert_eq!(remembered, None);
    }

    #[test]
    fn leaving_with_nothing_remembered_leaves_placement_alone() {
        // Reachable when this mode entered while already fullscreen (see the
        // test above) and is now leaving — the fullscreen it didn't cause is
        // not this mode's to undo.
        let (placement, remembered) =
            next_focus_placement(false, WindowPlacement::Fullscreen, None);
        assert_eq!(placement, None);
        assert_eq!(remembered, None);
    }

    // ── the view-model, against a real (headless) WindowState ─────────────

    #[test]
    fn toggle_enters_fullscreen_and_marks_active() {
        let vm = FocusViewModel::new();
        let window = test_window(WindowPlacement::Floating);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
        assert!(vm.active_signal().get());
    }

    #[test]
    fn toggle_back_restores_maximized_and_marks_inactive() {
        let vm = FocusViewModel::new();
        let window = test_window(WindowPlacement::Maximized);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Maximized);
        assert!(!vm.active_signal().get());
    }

    #[test]
    fn exit_while_inactive_is_a_no_op() {
        let vm = FocusViewModel::new();
        let window = test_window(WindowPlacement::Maximized);
        vm.exit(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Maximized);
        assert!(!vm.active_signal().get());
    }

    #[test]
    fn exit_while_active_restores_and_never_toggles_back_on() {
        let vm = FocusViewModel::new();
        let window = test_window(WindowPlacement::Maximized);
        vm.toggle(&window); // enter
        vm.exit(&window); // leave
        assert_eq!(window.placement().get(), WindowPlacement::Maximized);
        assert!(!vm.active_signal().get());
        // A second exit call must stay a no-op, not re-toggle into the mode.
        vm.exit(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Maximized);
        assert!(!vm.active_signal().get());
    }

    #[test]
    fn reset_forgets_state_without_touching_the_window() {
        let vm = FocusViewModel::new();
        let window = test_window(WindowPlacement::Maximized);
        vm.toggle(&window); // enter: fullscreen, remembers Maximized
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);

        vm.reset();
        assert!(!vm.active_signal().get());
        // The window itself is left exactly as it was — reset cannot reach a
        // `WindowState` (see this module's doc).
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);

        // Now inactive per its own bookkeeping: the *next* toggle treats this
        // as a fresh entry rather than trying to "leave" using stale memory.
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
    }

    #[test]
    fn per_window_state_is_not_shared_between_two_view_models() {
        let vm_a = FocusViewModel::new();
        let vm_b = FocusViewModel::new();
        let window_a = test_window(WindowPlacement::Maximized);
        let window_b = test_window(WindowPlacement::Floating);

        vm_a.toggle(&window_a);
        vm_b.toggle(&window_b);
        assert_eq!(window_a.placement().get(), WindowPlacement::Fullscreen);
        assert_eq!(window_b.placement().get(), WindowPlacement::Fullscreen);

        vm_a.toggle(&window_a);
        assert_eq!(
            window_a.placement().get(),
            WindowPlacement::Maximized,
            "a must restore its OWN remembered placement"
        );
        assert_eq!(
            window_b.placement().get(),
            WindowPlacement::Fullscreen,
            "b must be untouched by a's toggle"
        );
    }
}
