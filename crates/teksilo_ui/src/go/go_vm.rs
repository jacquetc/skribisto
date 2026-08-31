// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Live "is there a target" mirror for the six Go-menu rows (Next/Previous ×
//! Scene/Chapter/Note).
//!
//! **Minted where [`scene_focused`](crate::shell::windows) is** — `shell/windows.rs`'s window
//! chrome builds the Go menu before `App` (and therefore `EditorsViewModel`) exists, so
//! this has to be created there, threaded through `App::new`/`EditorsViewModel::new`,
//! and written by [`EditorsViewModel`](crate::editors::EditorsViewModel) once it exists — the
//! same *per-window* UI-state shape `scene_focused`'s own doc comment names, and for the
//! same reason: a process-wide instance would let a second project window's Go menu
//! grey out (or light up) off the *wrong* window's focused item.
//!
//! Six independent booleans, not one "can go" flag: each Go-menu row asks its own
//! question ("is there a next Chapter", regardless of what kind the focused item
//! itself is), so a single shared flag could not answer them separately.

use teksilo::prelude::*;

/// Six live booleans — one per Go-menu row — cloned wherever the row needs to bind its
/// `MenuEntry::enabled(..)`, and written by [`EditorsViewModel::sync_active_item`]
/// (via `set`) whenever the focused item changes.
///
/// [`EditorsViewModel::sync_active_item`]: crate::editors::EditorsViewModel::sync_active_item
#[derive(Clone)]
pub struct GoAvailability {
    next_scene: Signal<bool>,
    prev_scene: Signal<bool>,
    next_chapter: Signal<bool>,
    prev_chapter: Signal<bool>,
    next_note: Signal<bool>,
    prev_note: Signal<bool>,
}

impl GoAvailability {
    pub fn new() -> Self {
        Self {
            next_scene: Signal::new(false),
            prev_scene: Signal::new(false),
            next_chapter: Signal::new(false),
            prev_chapter: Signal::new(false),
            next_note: Signal::new(false),
            prev_note: Signal::new(false),
        }
    }

    /// The live signal for one Go-menu row — bind straight to `MenuEntry::enabled(..)`.
    /// Never `.visible()`: a disabled row still teaches the writer the feature exists
    /// and what it is called, and still reaches the a11y tree — the same rule the
    /// Format menu's own scene-break rows already follow.
    pub fn signal(
        &self,
        kind: skribisto_model::GoKind,
        direction: skribisto_model::GoDirection,
    ) -> Signal<bool> {
        use skribisto_model::{GoDirection::*, GoKind::*};
        match (kind, direction) {
            (Scene, Next) => self.next_scene.clone(),
            (Scene, Previous) => self.prev_scene.clone(),
            (Chapter, Next) => self.next_chapter.clone(),
            (Chapter, Previous) => self.prev_chapter.clone(),
            (Note, Next) => self.next_note.clone(),
            (Note, Previous) => self.prev_note.clone(),
        }
    }

    /// Write one row's mirror, skipping the write when the value has not changed.
    pub(crate) fn set(
        &self,
        kind: skribisto_model::GoKind,
        direction: skribisto_model::GoDirection,
        value: bool,
    ) {
        let signal = self.signal(kind, direction);
        if signal.get() != value {
            signal.set(value);
        }
    }
}

impl Default for GoAvailability {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use skribisto_model::{GoDirection::*, GoKind::*};

    #[test]
    fn every_row_starts_disabled() {
        let go = GoAvailability::new();
        for kind in [Scene, Chapter, Note] {
            for direction in [Next, Previous] {
                assert!(!go.signal(kind, direction).get());
            }
        }
    }

    #[test]
    fn set_writes_only_the_addressed_row() {
        let go = GoAvailability::new();
        go.set(Scene, Next, true);
        assert!(go.signal(Scene, Next).get());
        // Every other row is untouched.
        assert!(!go.signal(Scene, Previous).get());
        assert!(!go.signal(Chapter, Next).get());
        assert!(!go.signal(Chapter, Previous).get());
        assert!(!go.signal(Note, Next).get());
        assert!(!go.signal(Note, Previous).get());
    }

    /// Two clones share the same underlying signals — the whole point of threading one
    /// instance through `shell/windows.rs` and `EditorsViewModel` rather than two.
    #[test]
    fn clones_share_the_same_live_signals() {
        let go = GoAvailability::new();
        let clone = go.clone();
        clone.set(Chapter, Previous, true);
        assert!(go.signal(Chapter, Previous).get());
    }
}
