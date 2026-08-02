// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Plain fullscreen for the project window (Increment 1 of distraction-free —
//! **not** distraction-free mode itself, which collapses chrome; this is only
//! "go fullscreen and come back", by shortcut (F11) and View menu.
//!
//! Per the house rules the decision is a pure function (unit-tested, no
//! `Signal`/`WindowState`); the view-model below is the thin reactive shell
//! that remembers what to restore and applies the decision to a real window.
//!
//! **Per-window, not shared.** [`FullscreenViewModel::new`] is minted fresh
//! for each project window in
//! `shell::windows::ProjectWindowFactory::window_config` — the same shape as
//! that factory's `scene_focused` field (see its doc): "was this window
//! maximized before it went fullscreen" is this window's own UI state, and a
//! process-wide singleton would let a second project window's toggle answer
//! with the first window's memory.

use bastyde::prelude::{Signal, WindowPlacement, WindowState};

/// Decide the placement to apply and the value to remember next, from the
/// window's **current** placement (read fresh, not a private "am I
/// fullscreen" bool) and whatever this window remembered from the last time
/// it entered fullscreen.
///
/// Reading `current` fresh is what makes leaving fullscreen *behind the
/// app's back* — the OS's own shortcut, the titlebar's own affordance — self-
/// correcting: `WindowState::placement()` round-trips real OS state (see its
/// module doc), so the next toggle observes the truth and treats it as a
/// fresh "enter fullscreen", discarding whatever stale memory this window
/// still held from before. `remembered: None` only ever falls back to
/// `WindowPlacement::default()` (`Floating`) — reachable only if `toggle` is
/// somehow invoked while already fullscreen with nothing remembered, which
/// the view-model's own construction never produces.
fn next_fullscreen_state(
    current: WindowPlacement,
    remembered: Option<WindowPlacement>,
) -> (WindowPlacement, Option<WindowPlacement>) {
    if current == WindowPlacement::Fullscreen {
        (remembered.unwrap_or_default(), None)
    } else {
        (WindowPlacement::Fullscreen, Some(current))
    }
}

/// Owns one project window's "what was I before fullscreen" memory.
#[derive(Clone)]
pub struct FullscreenViewModel {
    /// `Some(p)` while this window is fullscreen and `p` is what to restore;
    /// `None` otherwise. Read reactively by the View menu's checkmark
    /// (mirrored off `WindowState::placement()` directly, not this signal —
    /// this one only needs to survive between two `toggle` calls).
    remembered: Signal<Option<WindowPlacement>>,
}

impl FullscreenViewModel {
    pub fn new() -> Self {
        Self {
            remembered: Signal::new(None),
        }
    }

    /// Toggle `window` between fullscreen and whatever it was before —
    /// `window` is resolved by the caller from `EventContext::window()`, so
    /// this always acts on the window that fired the command, correct with
    /// several project windows open.
    pub fn toggle(&self, window: &WindowState) {
        let (new_placement, new_remembered) =
            next_fullscreen_state(window.placement().get(), self.remembered.get());
        self.remembered.set(new_remembered);
        window.placement().set(new_placement);
    }
}

impl Default for FullscreenViewModel {
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
    fn entering_fullscreen_remembers_floating() {
        let (placement, remembered) = next_fullscreen_state(WindowPlacement::Floating, None);
        assert_eq!(placement, WindowPlacement::Fullscreen);
        assert_eq!(remembered, Some(WindowPlacement::Floating));
    }

    #[test]
    fn leaving_fullscreen_restores_the_remembered_maximized_not_floating() {
        // A hardcoded `Floating` restore would get this wrong.
        let (placement, remembered) = next_fullscreen_state(
            WindowPlacement::Fullscreen,
            Some(WindowPlacement::Maximized),
        );
        assert_eq!(placement, WindowPlacement::Maximized);
        assert_eq!(remembered, None);
    }

    #[test]
    fn leaving_fullscreen_with_nothing_remembered_falls_back_to_floating() {
        let (placement, remembered) = next_fullscreen_state(WindowPlacement::Fullscreen, None);
        assert_eq!(placement, WindowPlacement::Floating);
        assert_eq!(remembered, None);
    }

    #[test]
    fn a_placement_change_behind_the_apps_back_is_observed_on_the_next_toggle() {
        // The user left fullscreen with KDE's own shortcut/titlebar, not
        // through `toggle` — the window is now Floating, but this window's
        // memory still holds a stale Maximized from before it last entered
        // fullscreen. Reading `current` fresh must treat this as a brand new
        // "enter fullscreen", overwriting the stale memory rather than acting
        // on it.
        let (placement, remembered) =
            next_fullscreen_state(WindowPlacement::Floating, Some(WindowPlacement::Maximized));
        assert_eq!(placement, WindowPlacement::Fullscreen);
        assert_eq!(remembered, Some(WindowPlacement::Floating));
    }

    // ── the view-model, against a real (headless) WindowState ─────────────

    #[test]
    fn toggle_writes_fullscreen_when_not_fullscreen() {
        let vm = FullscreenViewModel::new();
        let window = test_window(WindowPlacement::Floating);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
    }

    #[test]
    fn toggle_back_restores_maximized_not_floating() {
        let vm = FullscreenViewModel::new();
        let window = test_window(WindowPlacement::Maximized);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Maximized);
    }

    #[test]
    fn toggle_observes_a_placement_left_behind_the_apps_back() {
        let vm = FullscreenViewModel::new();
        let window = test_window(WindowPlacement::Maximized);
        vm.toggle(&window); // Maximized -> Fullscreen, remembers Maximized
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);

        // KDE's own shortcut restores the window without going through `toggle`.
        window.set_placement_from_os(WindowPlacement::Floating);

        // The next toggle must not blindly "restore" the stale Maximized
        // memory — the window isn't fullscreen anymore, so this is a fresh
        // entry.
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Fullscreen);
        vm.toggle(&window);
        assert_eq!(window.placement().get(), WindowPlacement::Floating);
    }

    #[test]
    fn per_window_state_is_not_shared_between_two_view_models() {
        // Two windows, each with its own view-model — mirrors
        // `ProjectWindowFactory::window_config` minting a fresh
        // `FullscreenViewModel` per window.
        let vm_a = FullscreenViewModel::new();
        let vm_b = FullscreenViewModel::new();
        let window_a = test_window(WindowPlacement::Maximized);
        let window_b = test_window(WindowPlacement::Floating);

        vm_a.toggle(&window_a); // a: Maximized -> Fullscreen, remembers Maximized
        vm_b.toggle(&window_b); // b: Floating -> Fullscreen, remembers Floating
        assert_eq!(window_a.placement().get(), WindowPlacement::Fullscreen);
        assert_eq!(window_b.placement().get(), WindowPlacement::Fullscreen);

        vm_a.toggle(&window_a); // a leaves fullscreen
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

        vm_b.toggle(&window_b); // b leaves fullscreen
        assert_eq!(
            window_b.placement().get(),
            WindowPlacement::Floating,
            "b must restore its OWN remembered placement, not a's"
        );
    }
}
