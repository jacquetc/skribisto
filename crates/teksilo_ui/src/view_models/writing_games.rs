// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Writing games — self-imposed drafting constraints the writer opts into.
//!
//! One game today: **Always forward** (« Droit devant »), which disables every
//! way of taking prose *back* from the keyboard, so a first draft can only grow.
//! It is the app's answer to the "forbid erasing" writing-game mode the roadmap
//! has carried for a while, and what other tools call *Hemingway mode*.
//!
//! ## Why the activation is deliberately not persisted
//!
//! [`always_forward`](WritingGamesViewModel::always_forward) is **session state**
//! — a plain `Signal` that starts `false` on every launch and dies with the
//! `WorkSession` that holds it. That is a decision, not an omission:
//!
//! * A game is something the writer *chooses to play right now*. Finding
//!   Backspace dead a week later, with no memory of having agreed to it, reads
//!   as a broken keyboard rather than a rule the writer set for themselves — the
//!   single most-reported complaint about every tool that ships this feature.
//! * It follows the closest precedent in this codebase: distraction-free mode
//!   (`FocusViewModel`) is also a per-session `Signal`, deliberately reset on
//!   Close-Work/Load-Work rather than restored.
//! * A per-`Work` *field* would push a writing-discipline preference into the
//!   `.skrib` file, where it would sync to other machines and outlive the sprint
//!   that motivated it; a sibling entity would additionally inherit the
//!   dirty-tracking trap that `mutation_origins()` documents.
//!
//! The two **sub-options** below it (which surfaces the game applies to) are the
//! opposite kind of thing — a stable preference about how the writer likes the
//! game to behave — so those *are* ordinary persisted settings, read here as the
//! app-global signals they already are.
//!
//! ## Tier 2, and why not Tier 3
//!
//! Lives on [`WorkSession`](crate::sessions::WorkSession): one instance per open
//! `Work`, shared by every window on it. A per-window flag would let two windows
//! on the same project disagree about whether Backspace works *in the same
//! document*, which reads as a bug rather than as a mode. A second,
//! simultaneously-open project gets its own instance, so playing the game in one
//! manuscript never freezes another.

use teksilo::prelude::Signal;
use teksilo::widgets::rich_text::CommandFilter;

use crate::view_models::EditorKind;

/// The **app-global**, persisted half: which surfaces "Always forward" covers.
///
/// A named type rather than two loose `Signal<bool>` parameters, because the
/// distinction it draws is the one thing this feature must not get wrong. These
/// two signals are genuinely process-wide (they are settings), so they are read
/// from `SettingsViewModel` and shared by every open project; the activation
/// they are paired with in [`WritingGamesViewModel::new`] is per-`Work` and
/// comes from that project's own `WorkSession`. Keeping the two halves in
/// separate types is what stops a future refactor from sharing one activation
/// across projects — which would freeze every open manuscript at once.
#[derive(Clone)]
pub struct WritingGameOptions {
    /// Persisted `games.always_forward.prose`.
    pub in_prose: Signal<bool>,
    /// Persisted `games.always_forward.synopsis`.
    pub in_synopsis: Signal<bool>,
}

impl WritingGameOptions {
    pub fn new(in_prose: Signal<bool>, in_synopsis: Signal<bool>) -> Self {
        Self {
            in_prose,
            in_synopsis,
        }
    }

    /// The shipped defaults, detached from any settings store — for tests and
    /// for surfaces built with no app around them.
    pub fn detached() -> Self {
        Self {
            in_prose: Signal::new(FORWARD_PROSE_DEFAULT),
            in_synopsis: Signal::new(FORWARD_SYNOPSIS_DEFAULT),
        }
    }
}

/// The writing-games state for one open `Work`.
///
/// Cloning shares — every field is a `Signal`, which is `Rc`-backed — so the
/// dock toggle, the settings pane, the status bar and every mounted editor all
/// read and write the same state.
#[derive(Clone)]
pub struct WritingGamesViewModel {
    /// Is **Always forward** being played right now? Session-only; see the
    /// module doc.
    always_forward: Signal<bool>,
    /// Persisted: does the game cover manuscript prose? (Default `true` — prose
    /// is what the game is *for*.)
    forward_in_prose: Signal<bool>,
    /// Persisted: does the game cover synopses? (Default `false` — a synopsis is
    /// a working note about the prose rather than the draft itself, and it is
    /// where a writer jots the fix they are not allowed to make.)
    forward_in_synopsis: Signal<bool>,
}

impl WritingGamesViewModel {
    /// Pair one `Work`'s activation with the app-global options.
    ///
    /// `always_forward` comes from that project's
    /// [`WorkSession`](crate::sessions::WorkSession) and `options` from
    /// `SettingsViewModel`, so this is assembled wherever both are in hand — the
    /// same shape as `TypewriterSettings` and `CaretHighlightSettings`, and for
    /// the same reason: a settings-backed signal can only be resolved where
    /// `ctx.settings()` exists, which is inside the app.
    pub fn new(always_forward: Signal<bool>, options: WritingGameOptions) -> Self {
        Self {
            always_forward,
            forward_in_prose: options.in_prose,
            forward_in_synopsis: options.in_synopsis,
        }
    }

    /// A standalone instance for tests and for surfaces built with no app around
    /// them: the game off, prose covered, synopsis not — the shipped defaults.
    pub fn detached() -> Self {
        Self::new(Signal::new(false), WritingGameOptions::detached())
    }

    /// Whether **Always forward** is being played. Bind a toggle straight to it.
    pub fn always_forward(&self) -> Signal<bool> {
        self.always_forward.clone()
    }

    /// Does the game cover manuscript prose? (Persisted preference.)
    pub fn forward_in_prose(&self) -> Signal<bool> {
        self.forward_in_prose.clone()
    }

    /// Does the game cover synopses? (Persisted preference.)
    pub fn forward_in_synopsis(&self) -> Signal<bool> {
        self.forward_in_synopsis.clone()
    }

    /// Start or stop playing.
    pub fn set_always_forward(&self, on: bool) {
        if self.always_forward.get() != on {
            self.always_forward.set(on);
        }
    }

    /// Flip the game — what the dock's toggle and any future command fire.
    pub fn toggle_always_forward(&self) {
        self.always_forward.set(!self.always_forward.get());
    }

    /// Does the game currently bind on an editor of this kind?
    ///
    /// Both halves are live signals, so this is the question every mounted
    /// editor re-asks whenever either changes.
    pub fn covers(&self, kind: EditorKind) -> bool {
        if !self.always_forward.get() {
            return false;
        }
        match kind {
            EditorKind::Prose => self.forward_in_prose.get(),
            EditorKind::Synopsis => self.forward_in_synopsis.get(),
        }
    }

    /// The command filter an editor of this kind must run under right now.
    ///
    /// The whole point of routing through teksilo's filter rather than
    /// intercepting keys here: the framework already knows which of its commands
    /// take text away, including the ones an application would forget (a
    /// same-editor drag-move, a context-menu Cut, an assistive-technology
    /// replace).
    pub fn filter_for(&self, kind: EditorKind) -> CommandFilter {
        if self.covers(kind) {
            CommandFilter::ForwardOnly
        } else {
            CommandFilter::All
        }
    }

    /// Is the game on but bound to nothing? A state the settings pane warns
    /// about rather than silently preventing — the writer may be mid-way through
    /// choosing which surfaces to cover.
    pub fn is_inert(&self) -> bool {
        self.always_forward.get() && !self.forward_in_prose.get() && !self.forward_in_synopsis.get()
    }
}

/// Default for [`WritingGamesViewModel::forward_in_prose`] — on. Prose is the
/// draft the game exists to protect from its own author.
pub const FORWARD_PROSE_DEFAULT: bool = true;
/// Default for [`WritingGamesViewModel::forward_in_synopsis`] — off. The
/// synopsis is where a writer notes the fix they have just forbidden themselves
/// from making, so freezing it too would close the one escape the game leaves
/// open.
pub const FORWARD_SYNOPSIS_DEFAULT: bool = false;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_game_starts_off_and_covers_nothing() {
        let vm = WritingGamesViewModel::detached();
        assert!(!vm.always_forward().get());
        assert_eq!(vm.filter_for(EditorKind::Prose), CommandFilter::All);
        assert_eq!(vm.filter_for(EditorKind::Synopsis), CommandFilter::All);
    }

    #[test]
    fn playing_binds_prose_but_not_the_synopsis_by_default() {
        let vm = WritingGamesViewModel::detached();
        vm.set_always_forward(true);
        assert_eq!(
            vm.filter_for(EditorKind::Prose),
            CommandFilter::ForwardOnly,
            "prose is what the game is for"
        );
        assert_eq!(
            vm.filter_for(EditorKind::Synopsis),
            CommandFilter::All,
            "the synopsis stays editable — it is where the forbidden fix gets noted"
        );
    }

    #[test]
    fn each_surface_follows_its_own_option() {
        let vm = WritingGamesViewModel::detached();
        vm.set_always_forward(true);
        vm.forward_in_prose().set(false);
        vm.forward_in_synopsis().set(true);
        assert_eq!(vm.filter_for(EditorKind::Prose), CommandFilter::All);
        assert_eq!(
            vm.filter_for(EditorKind::Synopsis),
            CommandFilter::ForwardOnly
        );
    }

    #[test]
    fn the_options_bind_nothing_while_the_game_is_off() {
        let vm = WritingGamesViewModel::detached();
        vm.forward_in_prose().set(true);
        vm.forward_in_synopsis().set(true);
        assert!(!vm.covers(EditorKind::Prose));
        assert!(!vm.covers(EditorKind::Synopsis));
    }

    #[test]
    fn a_game_bound_to_no_surface_reports_itself_inert() {
        let vm = WritingGamesViewModel::detached();
        assert!(!vm.is_inert(), "off is not inert, it is off");
        vm.set_always_forward(true);
        assert!(!vm.is_inert());
        vm.forward_in_prose().set(false);
        assert!(vm.is_inert(), "on, but bound to nothing");
    }

    #[test]
    fn toggling_flips_and_is_idempotent_when_set() {
        let vm = WritingGamesViewModel::detached();
        vm.toggle_always_forward();
        assert!(vm.always_forward().get());
        vm.set_always_forward(true);
        assert!(vm.always_forward().get());
        vm.toggle_always_forward();
        assert!(!vm.always_forward().get());
    }

    /// Two windows on one `Work` share one instance, so a clone is the same
    /// state — the reason this is Tier 2 rather than per window.
    /// The activation must be per-`Work`: two projects built from the SAME
    /// options must not share whether the game is being played.
    #[test]
    fn two_projects_share_the_options_but_not_the_activation() {
        let options = WritingGameOptions::detached();
        // Two projects: two session flags, one shared set of options.
        let first = WritingGamesViewModel::new(Signal::new(false), options.clone());
        let second = WritingGamesViewModel::new(Signal::new(false), options);

        first.set_always_forward(true);
        assert!(
            !second.always_forward().get(),
            "playing in one manuscript must not freeze another open one"
        );

        // The persisted options, by contrast, are one shared preference.
        first.forward_in_synopsis().set(true);
        assert!(
            second.forward_in_synopsis().get(),
            "which surfaces the game covers is an app setting, shared by every project"
        );
    }

    #[test]
    fn a_clone_shares_the_same_state() {
        let vm = WritingGamesViewModel::detached();
        let second_window = vm.clone();
        second_window.set_always_forward(true);
        assert!(
            vm.always_forward().get(),
            "both windows must agree about whether Backspace works in the same document"
        );
    }
}
