// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FocusStrip` — distraction-free mode's always-visible control strip
//! (Increment 2): word count + writing session + Go Next/Previous (Increment 4)
//! + Exit, replacing the normal status bar's content while the mode is active.
//!
//! **Always visible, never hover-reveal** — this app's documented EN 301 549
//! / RGAA accessibility posture rules out a hover-only strip: both Scrivener
//! and FocusWriter have filed bugs where a hover-only control panel cannot be
//! brought back once dismissed. `App::build` gates this strip's *presence*
//! (shown only while [`crate::view_models::FocusViewModel`] is active) with
//! `VisibleWhen`, never with hover.
//!
//! `WordCountIndicator` and `SessionStatusItem` drop in unchanged — both
//! already take only a view-model + `Signal<bool>`, so this simply builds a
//! second instance of each over the *same* live `StatsModel`/
//! `WritingSessionViewModel` the normal status bar uses, rather than
//! duplicating their logic.
//!
//! **Every item here is optional except Exit** ([`FocusStripChrome`], bound to
//! the Settings ▸ Editor ▸ Editor Behavior ▸ Distraction-free checkboxes). Exit
//! is not a setting and must not become one: it is this strip's documented way
//! out, and the mode's other exits are a keystroke the focused editor may
//! legitimately swallow (Escape) and one the writer has to remember
//! (Shift+F11). A settings combination that can leave someone with no visible
//! way out is the failure mode this whole strip exists to avoid — see the
//! always-visible note above.
//!
//! **Go reuse, not reimplementation** — the Previous/Next icon buttons fire the
//! exact same `go.prev`/`go.next` named actions the Go menu's generic pair and
//! the Alt+Up/Alt+Down shortcut already drive (`app/commands/go.rs`), the same
//! way the Exit button above fires `view.focus_mode` rather than duplicating
//! `FocusViewModel::toggle`. Always enabled: like the shortcut, a target-less
//! press is a quiet no-op (`app/commands/go.rs`'s own doc), so there is no
//! separate "can I go" signal to bind here.

use bastyde::prelude::*;
use bastyde::widgets::{Button, ButtonVariant, HStack, IconButton, Spacer, StatusBar};

use crate::models::StatsModel;
use crate::statusbar::session_status_item::SessionStatusItem;
use crate::statusbar::word_count_indicator::WordCountIndicator;
use crate::tabs::shared::editor::VisibleWhen;
use crate::view_models::{SettingsViewModel, WritingSessionViewModel};

/// Which of the strip's optional items are shown, one live setting signal each.
///
/// Exit has no field here on purpose — see this module's doc. The tab strip's
/// own toggle is *not* here either: it gates a `TabWidget` up in `App::build`,
/// not a child of this strip.
#[derive(Clone)]
pub struct FocusStripChrome {
    pub word_count: Signal<bool>,
    pub session: Signal<bool>,
    pub go: Signal<bool>,
}

impl FocusStripChrome {
    /// Read the three live setting signals off the settings view-model.
    pub fn from_settings(settings: &SettingsViewModel) -> Self {
        Self {
            word_count: settings.distraction_free_word_count(),
            session: settings.distraction_free_session(),
            go: settings.distraction_free_go(),
        }
    }

    /// Detached signals with every item shown — for headless tests that only
    /// care about the strip's other behaviour.
    #[cfg(test)]
    pub fn all_shown() -> Self {
        Self {
            word_count: Signal::new(true),
            session: Signal::new(true),
            go: Signal::new(true),
        }
    }
}

pub struct FocusStrip {
    stats: StatsModel,
    session_vm: WritingSessionViewModel,
    has_work: Signal<bool>,
    show_characters: Signal<bool>,
    chrome: FocusStripChrome,
    root_child: Option<WidgetId>,
}

impl FocusStrip {
    pub fn new(
        stats: StatsModel,
        session_vm: WritingSessionViewModel,
        has_work: Signal<bool>,
        show_characters: Signal<bool>,
        chrome: FocusStripChrome,
    ) -> Self {
        Self {
            stats,
            session_vm,
            has_work,
            show_characters,
            chrome,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for FocusStrip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusStrip").finish()
    }
}

impl Widget for FocusStrip {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let word_count = WordCountIndicator::new(
            self.stats.clone(),
            self.has_work.clone(),
            self.show_characters.clone(),
        );
        let session_item = SessionStatusItem::new(self.session_vm.clone(), self.has_work.clone());
        // Each optional item is gated with `VisibleWhen` — the same dormant-
        // not-torn-down gate `App::build` wraps this whole strip in — so a
        // settings flip takes effect live, without the mode being re-entered.
        // The Go pair shares one gate: half a navigation control is worse than
        // none (see `DISTRACTION_FREE_GO_KEY`'s doc).
        let bar = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new()
                .spacing(8.0)
                .child(VisibleWhen::new(self.chrome.word_count.clone(), word_count))
                .child(VisibleWhen::new(self.chrome.session.clone(), session_item))
                .child(Spacer::new())
                .child(VisibleWhen::new(
                    self.chrome.go.clone(),
                    HStack::new()
                        .spacing(8.0)
                        .child(
                            IconButton::new(crate::icons::go::prev_icon())
                                .toolbar()
                                .tooltip(tr!(statusbar_focus_go_prev()))
                                .on_activate_fn(|ctx| ctx.send_intent(Intent::new("go.prev"))),
                        )
                        .child(
                            IconButton::new(crate::icons::go::next_icon())
                                .toolbar()
                                .tooltip(tr!(statusbar_focus_go_next()))
                                .on_activate_fn(|ctx| ctx.send_intent(Intent::new("go.next"))),
                        ),
                ))
                // Never gated: this strip's documented way out of the mode.
                // Fires the same named intent the Shift+F11 shortcut and the
                // View menu entry use (`app/commands/view.rs`'s
                // `view.focus_mode` action) — this button is only ever built
                // while the mode is active, so `toggle` is always a clean exit
                // here, never a re-entry.
                .child(
                    Button::new(tr!(statusbar_focus_exit()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(|ctx| ctx.send_intent(Intent::new("view.focus_mode"))),
                ),
        );
        self.root_child = Some(ctx.add(bar));
        self.root_child.into_iter().collect()
    }

    /// Zero-sized when hidden (the enclosing `VisibleWhen` gate never even
    /// builds this while the mode is inactive), matching every other
    /// status-bar item's own dormancy contract.
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OpenDocsStore, StatsModel};
    use bastyde::core::accesskit::Role;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use skribisto_model::counting::CountingMethodSetting;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn temp_store() -> bastyde::settings::SettingsStore {
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_focus_strip_test_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        bastyde::settings::SettingsStore::open(path).expect("open temp settings store")
    }

    /// Mount a strip with `chrome` and return its live tree plus the accessible
    /// names the accessibility layer exposes. `tree_with_events` (not a bare
    /// `WidgetTree`): `SessionStatusItem` subscribes to backend events, which
    /// panics with no event source registered.
    fn mount(chrome: FocusStripChrome) -> (WidgetTree, FocusStripChrome) {
        let ctx = Rc::new(AppContext::new());
        let store = temp_store();
        let stats = StatsModel::new(
            OpenDocsStore::new(ctx.clone()),
            Signal::new(None),
            Signal::new(CountingMethodSetting::default()),
        );
        let session_vm = WritingSessionViewModel::new(stats.clone(), &store);
        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add(FocusStrip::new(
            stats,
            session_vm,
            Signal::new(true),
            Signal::new(false),
            chrome.clone(),
        ));
        tree.layout(SizeProposal::exact(900.0, 40.0));
        (tree, chrome)
    }

    fn button_names(tree: &mut WidgetTree) -> Vec<String> {
        tree.sync_accessibility()
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Button)
            .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
            .collect()
    }

    /// The invariant the whole strip rests on: **Exit survives every
    /// combination of the settings**. Distraction-free mode's other two exits
    /// are a keystroke the focused editor may legitimately swallow (Escape)
    /// and one the writer has to remember (Shift+F11), so a settings choice
    /// that could take this button away would be a way to get stranded in
    /// fullscreen with no visible way out — the exact failure both Scrivener
    /// and FocusWriter have filed bugs for.
    #[test]
    fn exit_survives_every_combination_of_the_chrome_settings() {
        let exit = tr!(statusbar_focus_exit()).resolve_now();
        for wc in [false, true] {
            for session in [false, true] {
                for go in [false, true] {
                    let (mut tree, _) = mount(FocusStripChrome {
                        word_count: Signal::new(wc),
                        session: Signal::new(session),
                        go: Signal::new(go),
                    });
                    assert!(
                        button_names(&mut tree).contains(&exit),
                        "Exit must be present with word_count={wc}, session={session}, go={go}"
                    );
                }
            }
        }
    }

    /// The Go pair is one affordance: both arrows go, or neither does. A strip
    /// offering only Previous would be a worse answer than offering neither.
    #[test]
    fn the_go_pair_appears_and_disappears_together() {
        let prev = tr!(statusbar_focus_go_prev()).resolve_now();
        let next = tr!(statusbar_focus_go_next()).resolve_now();

        let (mut on, _) = mount(FocusStripChrome::all_shown());
        let names = button_names(&mut on);
        assert!(names.contains(&prev) && names.contains(&next), "both shown");

        let (mut off, _) = mount(FocusStripChrome {
            go: Signal::new(false),
            ..FocusStripChrome::all_shown()
        });
        let names = button_names(&mut off);
        assert!(
            !names.contains(&prev) && !names.contains(&next),
            "neither shown"
        );
    }

    /// Flipping a setting on an ALREADY-MOUNTED strip re-gates it. The
    /// per-combination tests above each mount a fresh strip, so they would pass
    /// even if the gate were read once at build time — but the writer's path is
    /// to be *in* the mode, open settings, untick a box and expect the strip to
    /// change under them.
    #[test]
    fn flipping_a_setting_re_gates_a_mounted_strip() {
        let prev = tr!(statusbar_focus_go_prev()).resolve_now();
        let (mut tree, chrome) = mount(FocusStripChrome::all_shown());
        assert!(button_names(&mut tree).contains(&prev), "starts shown");

        chrome.go.set(false);
        tree.layout(SizeProposal::exact(900.0, 40.0));
        assert!(
            !button_names(&mut tree).contains(&prev),
            "unticking Go must hide the arrows without re-entering the mode"
        );

        chrome.go.set(true);
        tree.layout(SizeProposal::exact(900.0, 40.0));
        assert!(
            button_names(&mut tree).contains(&prev),
            "and ticking it must bring them back"
        );
    }
}
