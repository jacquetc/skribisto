// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A container's own page states how long it is meant to be, how far along it is, and what
//! the targets set inside it add up to.
//!
//! Three facts, one of which is deliberately not a target. A chapter's own progress is
//! everything written beneath it measured against **its own** number; the second line is
//! the sum of its children's numbers, which this app never treats as the container's
//! target and never lets drift into one. Other tools do exactly that and have been fielding
//! the resulting complaint since 2007 — a folder's figure moving on its own because a scene
//! inside it was given one — so the line is worded as an observation and rendered as one:
//! no bar, no percentage, no colour band, nothing to click.
//!
//! Deferred to first paint like the outline card's word line, because the measurement walks
//! the container's whole subtree and a Book's own page would otherwise count the entire
//! manuscript synchronously every time its tab was rebuilt.

use std::cell::Cell;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, HStack, TextWidget, VStack};

use frontend::AppContext;
use frontend::common::entities::GoalUnit;
use skribisto_model::counting::CountingMethodSetting;

use crate::goals::{Measured, format_count, measure};
use crate::tabs::ContentTab;

/// The header block for a container's own page, or `None` for a row that should not carry
/// one.
///
/// Every container gets it, including one with no target set: the progress line then reads
/// as a plain length, which is the honest answer and the one that makes the Inspector's
/// empty target field discoverable.
pub fn container_goal_header(tab: &ContentTab) -> ContainerGoalHeader {
    ContainerGoalHeader {
        app_ctx: tab.app_ctx(),
        work_id: tab.ids().work_id.clone(),
        stack_id: tab.ids().stack_id.clone(),
        item_id: tab.item_id(),
        method: tab.counting_method().clone(),
        unit: tab.goal_unit().clone(),
        goal: Signal::new(0),
        measured: Signal::new(None),
        loaded: Cell::new(false),
        root: None,
    }
}

pub struct ContainerGoalHeader {
    app_ctx: Rc<AppContext>,
    work_id: Signal<Option<u64>>,
    /// The project's undo stack, so a distribution is one Ctrl+Z.
    stack_id: Signal<Option<u64>>,
    item_id: u64,
    method: Signal<CountingMethodSetting>,
    unit: Signal<GoalUnit>,
    /// This container's own target, refreshed with the measurement.
    goal: Signal<i64>,
    measured: Signal<Option<Measured>>,
    loaded: Cell<bool>,
    root: Option<WidgetId>,
}

impl ContainerGoalHeader {
    fn load(&self) {
        let Some(work_id) = self.work_id.get() else {
            return;
        };
        let unit = self.unit.get();
        // Through the single, like every other read of an item's scalars in this app.
        let probe = crate::singles::SingleBinderItem::new(self.app_ctx.clone());
        probe.set_id(Some(self.item_id));
        self.goal.set(probe.dto().map_or(0, |it| match unit {
            GoalUnit::Words => it.word_count_goal,
            GoalUnit::Characters => it.char_count_goal,
        }));
        self.measured.set(measure::measure(
            &self.app_ctx,
            work_id,
            self.item_id,
            self.method.get(),
            &unit,
        ));
    }
}

impl std::fmt::Debug for ContainerGoalHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContainerGoalHeader").finish()
    }
}

impl Widget for ContainerGoalHeader {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.measured.bind_to(sid, reg, BindingLevel::Rebuild);
        self.goal.bind_to(sid, reg, BindingLevel::Rebuild);
        self.unit.bind_to(sid, reg, BindingLevel::Rebuild);

        let Some(m) = self.measured.get() else {
            self.root = None;
            return Vec::new();
        };
        let unit = self.unit.get();
        let goal = self.goal.get();
        let written = m.subtree.by(&unit);

        let mut col = VStack::new().spacing(4.0).child(
            HStack::new()
                .spacing(8.0)
                .child(crate::widgets::goal_progress::line(written, goal, &unit))
                .child(distribute_button(self, goal)),
        );
        // The sum of what is inside — an observation, never a target. Shown only when
        // there is something to observe: a container whose children carry no targets has
        // nothing to say here, and a line reading "0 targets add up to 0" would be noise
        // on the majority of pages.
        if m.descendant_goal_items > 0 {
            // `count` selects the plural; `items` and `words` are what is printed.
            let count = m.descendant_goal_items as i64;
            let items = format_count(m.descendant_goal_items);
            let words = format_count(m.descendant_goal_total.max(0) as usize);
            let label = match unit {
                GoalUnit::Words => tr!(goal_subtree_total_words(
                    items = items,
                    words = words,
                    count = count
                )),
                GoalUnit::Characters => tr!(goal_subtree_total_characters(
                    items = items,
                    words = words,
                    count = count
                )),
            };
            col = col.child(crate::widgets::tip::RichTip::new(
                crate::tooltip_registry::GOAL_SUBTREE_TOTAL,
                TextWidget::new(label)
                    .style(TextStyleRole::Small)
                    .color(TextRole::Disabled)
                    .single_line(),
            ));
        }
        let id = ctx.add(col);
        self.root = Some(id);
        vec![id]
    }

    fn paint(&self, _bounds: Rect, _canvas: &mut Canvas, _ctx: &PaintContext) {
        if !self.loaded.replace(true) {
            self.load();
        }
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// "Distribute…", enabled only once there is something to share out.
///
/// Disabled rather than hidden when the container has no target: the action is what the
/// target is *for*, and a button that appears the moment a number is typed reads as the app
/// changing shape under the writer.
fn distribute_button(header: &ContainerGoalHeader, goal: i64) -> impl Widget + use<> {
    let app_ctx = header.app_ctx.clone();
    let work_id = header.work_id.clone();
    let stack_id = header.stack_id.clone();
    let item_id = header.item_id;
    let method = header.method.clone();
    let unit = header.unit.clone();
    Button::new(tr!(goal_distribute_action()))
        .variant(ButtonVariant::Plain)
        .enabled(goal > 0)
        .rich_tooltip(crate::tooltip_registry::GOAL_DISTRIBUTE)
        .on_activate_fn(move |ctx| {
            crate::goals::distribute_panel::present(
                ctx,
                crate::goals::distribute_panel::Request {
                    app_ctx: app_ctx.clone(),
                    work_id: work_id.get(),
                    item_id,
                    parent_goal: goal,
                    method: method.get(),
                    unit: unit.get(),
                    stack: stack_id.get(),
                },
            );
        })
}
