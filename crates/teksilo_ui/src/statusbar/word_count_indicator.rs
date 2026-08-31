// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WordCountIndicator` — the status bar's live word count of the focused item.
//!
//! A quiet, secondary-coloured count sitting after the save glyph, showing the words in
//! the scene the writer is editing. It appears only when a project is open and something
//! prose-bearing is focused (a container tab / an unopened item shows nothing and takes no
//! width). The count is one scene, cheap to recompute, so it tracks typing live — no
//! spinner-style hysteresis. Thin per the house rules: the decision is the pure
//! [`crate::statusbar::count_display`] table and the number comes from [`StatsModel`].

use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::TextWidget;

use frontend::AppContext;
use frontend::common::entities::GoalUnit;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

use crate::models::StatsModel;
use crate::singles::SingleBinderItem;
use crate::statusbar::{CountDisplay, GoalDisplay, count_display, goal_display};

pub struct WordCountIndicator {
    stats: StatsModel,
    /// A project is open at all (same test the save indicator uses).
    has_work: Signal<bool>,
    /// Show the character count beside the words (`goals.show_characters`).
    show_characters: Signal<bool>,
    /// The focused item, read here rather than handed in.
    ///
    /// The bar lives inside this widget — rather than beside it in the status bar — so
    /// that the distraction-free strip, which mounts this very widget unchanged, inherits
    /// it for free. Owning the probe keeps that true: a caller that only knows how to
    /// build a `WordCountIndicator` needs to know nothing about targets.
    ///
    /// A **one-shot** probe, deliberately unwired: `set_id` reloads synchronously, and
    /// `SingleBinderItem::wire`'s refresh would only write a signal nothing here reads.
    probe: SingleBinderItem,
    /// The focused item's target, and the only thing here bound at `Rebuild` on the
    /// target's behalf.
    ///
    /// **Not** `probe.dto_signal()`. Binding that and then calling `set_id` in the same
    /// `build` is a runaway rebuild loop: `set_id` refreshes, `refresh` calls
    /// `Signal::set`, and `set` has no equality check by design — so every build dirtied
    /// this widget and the status bar rebuilt on every frame for as long as a project was
    /// open. This signal is written with `set_if_changed` instead, which is exactly the
    /// guard that documents itself as the fix for a reactive write that might cycle.
    goal: Signal<i64>,
    /// Which unit the target is expressed in, so the readout beside the bar says the same
    /// thing the Inspector's does.
    unit: Signal<GoalUnit>,
    root_child: Option<WidgetId>,
}

impl WordCountIndicator {
    pub fn new(
        stats: StatsModel,
        has_work: Signal<bool>,
        show_characters: Signal<bool>,
        unit: Signal<GoalUnit>,
        app_ctx: Rc<AppContext>,
    ) -> Self {
        Self {
            stats,
            has_work,
            show_characters,
            probe: SingleBinderItem::new(app_ctx),
            goal: Signal::new(0),
            unit,
            root_child: None,
        }
    }

    fn render(
        &self,
        ctx: &mut BuildContext,
        display: CountDisplay,
        goal: GoalDisplay,
    ) -> Option<WidgetId> {
        let label = match display {
            CountDisplay::Hidden => return None,
            CountDisplay::Words(n) => tr!(statusbar_word_count(count = n as i64)),
            CountDisplay::WordsChars { words, chars } => {
                tr!(statusbar_word_char_count(
                    words = words as i64,
                    chars = chars as i64
                ))
            }
        };
        let text = TextWidget::new(label)
            .color(TextRole::Secondary)
            .single_line();
        let GoalDisplay::Progress { written, goal } = goal else {
            return Some(ctx.add(text));
        };
        // Bar first, then the count: the eye lands on the shape before the digits, and the
        // count keeps the position it has always had relative to the save glyph.
        Some(
            ctx.add(
                teksilo::widgets::HStack::new()
                    .spacing(6.0)
                    .child(crate::goals::readout::bar(written, goal))
                    .child(text)
                    .child(
                        TextWidget::new(crate::goals::readout::progress_label(
                            written,
                            goal,
                            &self.unit.get(),
                        ))
                        .color(TextRole::Disabled)
                        .single_line(),
                    ),
            ),
        )
    }
}

/// The focused item's target in the project's unit, or `0` when it has none.
fn target_of(probe: &SingleBinderItem, unit: &GoalUnit) -> i64 {
    probe.dto().map_or(0, |d| match unit {
        GoalUnit::Words => d.word_count_goal,
        GoalUnit::Characters => d.char_count_goal,
    })
}

impl std::fmt::Debug for WordCountIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WordCountIndicator").finish()
    }
}

impl Widget for WordCountIndicator {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        // Cloned rather than borrowed: the registry is `Rc`-backed and shares state, and
        // holding a borrow of `ctx` here would block the `subscribe_event` below.
        let reg = &ctx.binding_registry().clone();
        // Focus change → recount immediately; any edit → recount (cheap, one scene);
        // a counting-method or show-characters change also re-derives (the status bar
        // sits behind the Settings modal, so nothing else would rebuild it).
        self.has_work.bind_to(sid, reg, BindingLevel::Rebuild);
        self.stats
            .active_item()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.stats
            .edited_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.stats
            .method_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.show_characters
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.unit.bind_to(sid, reg, BindingLevel::Rebuild);
        self.goal.bind_to(sid, reg, BindingLevel::Rebuild);

        // A target set from the Inspector or the Overview has to move this bar without the
        // writer touching the editor, so the entity's own event is what drives it — read
        // the item again and publish the number, guarded so an unrelated `BinderItem`
        // update (a rename, a tag) settles instead of cycling.
        {
            let (probe, goal, unit) = (self.probe.clone(), self.goal.clone(), self.unit.clone());
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
                move |event: &Event| {
                    let Some(id) = probe.id() else { return };
                    if !event.ids.is_empty() && !event.ids.contains(&id) {
                        return;
                    }
                    probe.set_id(Some(id));
                    goal.set_if_changed(target_of(&probe, &unit.get()));
                },
            );
        }
        // Follow the focused item. One-shot: `set_id` reloads synchronously, so the value
        // read below is current the moment this returns.
        self.probe.set_id(self.stats.active_item().get());
        self.goal
            .set_if_changed(target_of(&self.probe, &self.unit.get()));

        let counts = self.stats.focused_counts();
        let focused = counts.map(|c| (c.words, c.chars_with_spaces));
        let display = count_display(self.has_work.get(), focused, self.show_characters.get());
        // Measured in whichever unit the project counts in — the same figure the
        // Inspector's readout shows for the same scene.
        let written = counts.map(|c| match self.unit.get() {
            GoalUnit::Words => c.words,
            GoalUnit::Characters => c.chars_with_spaces,
        });
        let goal = goal_display(self.has_work.get(), written, self.goal.get());
        self.root_child = self.render(ctx, display, goal);
        self.root_child.into_iter().collect()
    }

    /// Zero-sized when hidden (no project / a container tab), so it takes no width of
    /// its own there — the surrounding `HStack` spacing still leaves a small gap.
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
