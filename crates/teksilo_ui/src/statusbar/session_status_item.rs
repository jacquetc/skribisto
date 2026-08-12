// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SessionStatusItem` — the status bar's writing-session control.
//!
//! A configure gear + a play/pause toggle; once running, a red→green gauge (when a word
//! goal is set) and a compact "N words · M:SS" readout. Right-click resets. The session is
//! ephemeral — the state lives in [`WritingSessionViewModel`], never the store — so this is
//! a thin reactive shell: it binds the view-model's signals, drives its clock off the frame
//! tick (a `wake_at` keeps the loop asleep between seconds), and feeds edits/focus changes
//! into its word tracker. Shown only when a project is open.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use teksilo::core::BindingLevel;
use teksilo::core::color_prop::ColorProp;
use teksilo::prelude::*;
use teksilo::widgets::{
    FixedSize, HStack, IconButton, IconButtonSize, MenuItem, MenuList, Padding, PopoverIconButton,
    ProgressBar, Spacer, SpinBox, TextWidget, VStack,
};

use crate::icons::session;
use crate::writing_session::{
    WritingSessionViewModel, format_mmss, gauge_role, remaining, words_progress,
};

/// Shared one-shot deadline cell (`ctx.wake_at_handle()`).
type WakeAt = Rc<Cell<Option<Instant>>>;

const GAUGE_WIDTH: f32 = 54.0;
const SPIN_WIDTH: f32 = 140.0;

pub struct SessionStatusItem {
    vm: WritingSessionViewModel,
    has_work: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl SessionStatusItem {
    pub fn new(vm: WritingSessionViewModel, has_work: Signal<bool>) -> Self {
        Self {
            vm,
            has_work,
            root_child: None,
        }
    }

    /// The compact "N words · M:SS\[ left\]" readout while running.
    fn readout(&self) -> teksilo::i18n::LocalizedString {
        let words = self.vm.session_words_signal().get();
        let elapsed = self.vm.elapsed_signal().get();
        match self.vm.time_target() {
            Some(t) => {
                let left = remaining(elapsed, Some(t)).unwrap_or_default();
                tr!(session_readout_timed(
                    words = words,
                    time = format_mmss(left)
                ))
            }
            None => tr!(session_readout(words = words, time = format_mmss(elapsed))),
        }
    }

    fn render(&self, ctx: &mut BuildContext) -> Option<WidgetId> {
        // A writing session needs a project (to count words in) — hidden otherwise.
        if !self.has_work.get() {
            return None;
        }
        let vm = self.vm.clone();
        let running = vm.running().get();

        let mut row = HStack::new().spacing(6.0);

        // Configure gear → popover with the two targets.
        row = row.child(
            PopoverIconButton::new(
                IconButton::new(session::gear())
                    .size(IconButtonSize::Compact)
                    .tooltip(tr!(session_configure())),
            )
            .show_disclosure_caret(false)
            .content(configure_form(&vm)),
        );

        // Play/pause. The icon is chosen from `running` (bound at Rebuild) rather than via
        // `toggle_with_icon` — that would flip the `running` signal itself on click and
        // fight `on_activate_fn`'s full start/pause logic (a double-toggle).
        let vm_toggle = vm.clone();
        let icon = if running {
            session::pause()
        } else {
            session::play()
        };
        row = row.child(
            IconButton::new(icon)
                .size(IconButtonSize::Compact)
                .tooltip(tr!(session_toggle()))
                .on_activate_fn(move |_| vm_toggle.toggle()),
        );

        // Running: the gauge (only with a word goal) + the readout.
        if running {
            if let Some(target) = vm.word_target_opt() {
                let progress =
                    words_progress(vm.session_words_signal().get(), Some(target)).unwrap_or(0.0);
                row = row.child(
                    FixedSize::new().width(GAUGE_WIDTH).child(
                        ProgressBar::new(progress)
                            .thickness(4.0)
                            .fill_color(ColorProp::from(gauge_role(progress)))
                            .track_color(SurfaceRole::Sunken),
                    ),
                );
            }
            row = row.child(
                TextWidget::new(self.readout())
                    .color(TextRole::Secondary)
                    .single_line(),
            );
        }

        // Right-click anywhere on the session item → Reset.
        let vm_menu = vm.clone();
        Some(ctx.add(row.context_menu(move |_pos, _ctx| {
            Some(Box::new(reset_menu(vm_menu.clone())) as Box<dyn Widget>)
        })))
    }
}

/// The gear popover: word goal + time limit, both persisted; `0` reads as "none".
fn configure_form(vm: &WritingSessionViewModel) -> impl Widget {
    let goal_row = HStack::new()
        .spacing(8.0)
        .child(TextWidget::new(tr!(session_word_goal())))
        .child(Spacer::new())
        .child(
            FixedSize::new().width(SPIN_WIDTH).child(
                SpinBox::new(vm.word_target(), 0i64, 100_000)
                    .special_value_text(tr!(session_no_goal())),
            ),
        );
    let time_row = HStack::new()
        .spacing(8.0)
        .child(TextWidget::new(tr!(session_time_limit())))
        .child(Spacer::new())
        .child(
            FixedSize::new().width(SPIN_WIDTH).child(
                SpinBox::new(vm.time_target_min(), 0i64, 600)
                    .special_value_text(tr!(session_no_limit())),
            ),
        );
    Padding::uniform(12.0).child(
        VStack::new()
            .spacing(10.0)
            .child(TextWidget::new(tr!(session_configure_title())).style(TextStyleRole::BodyBold))
            .child(goal_row)
            .child(time_row),
    )
}

fn reset_menu(vm: WritingSessionViewModel) -> MenuList {
    MenuList::new().item(MenuItem::new(tr!(session_reset())).on_activate_fn(move |_| vm.reset()))
}

impl std::fmt::Debug for SessionStatusItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionStatusItem").finish()
    }
}

impl Widget for SessionStatusItem {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        // Structural: show/hide + the gauge's presence (word target) rebuild the item.
        self.has_work.bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.running().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .word_target()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .time_target_min()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .session_words_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .elapsed_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        // Feed edits + focus changes into the word tracker (a no-op while paused).
        {
            let vm = self.vm.clone();
            let edited = self.vm.stats().edited_signal();
            ctx.effect(&edited, move |_| vm.recompute_words());
        }
        {
            let vm = self.vm.clone();
            let active = self.vm.stats().active_item();
            ctx.effect(&active, move |_| vm.recompute_words());
        }

        // Drive the clock off the frame tick, sleeping between seconds via `wake_at` (the
        // same idiom the save spinner uses). Re-armed on each running-state change so the
        // first tick fires immediately on play.
        let wake: WakeAt = ctx.wake_at_handle();
        {
            let vm = self.vm.clone();
            let tick = ctx.frame_tick();
            let wake = wake.clone();
            ctx.effect(&tick, move |_| {
                if let Some(at) = vm.poll(Instant::now()) {
                    wake.set(Some(at));
                }
            });
        }
        {
            let vm = self.vm.clone();
            ctx.effect(&self.vm.running(), move |_| {
                if let Some(at) = vm.poll(Instant::now()) {
                    wake.set(Some(at));
                }
            });
        }

        self.root_child = self.render(ctx);
        self.root_child.into_iter().collect()
    }

    /// Zero-sized when hidden (no project), taking no width of its own.
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
