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
use crate::view_models::WritingSessionViewModel;

pub struct FocusStrip {
    stats: StatsModel,
    session_vm: WritingSessionViewModel,
    has_work: Signal<bool>,
    show_characters: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl FocusStrip {
    pub fn new(
        stats: StatsModel,
        session_vm: WritingSessionViewModel,
        has_work: Signal<bool>,
        show_characters: Signal<bool>,
    ) -> Self {
        Self {
            stats,
            session_vm,
            has_work,
            show_characters,
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
        // Fires the same named intent the Shift+F11 shortcut and the View menu
        // entry use (`app/commands/view.rs`'s `view.focus_mode` action) —
        // this button is always shown while the mode is active, so `toggle`
        // is always a clean exit here, never a re-entry.
        let bar = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new()
                .spacing(8.0)
                .child(word_count)
                .child(session_item)
                .child(Spacer::new())
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
                )
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
