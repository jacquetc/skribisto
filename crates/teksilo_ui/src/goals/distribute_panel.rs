// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The confirm-with-preview modal behind "Distribute…".
//!
//! The preview is the point, not politeness. This action writes a target onto every
//! eligible child at once, and the one guarantee that makes it trustworthy — that the parts
//! add up to the parent exactly — is arithmetic the writer cannot check by eye across a
//! column of numbers. So the footer states the sum, beside the parent's own target, and
//! they are visibly the same figure before anything is written.
//!
//! Nothing is committed until OK. Cancel closes with no backend call at all: the plan is
//! computed entirely from data already in memory.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Expand, FixedSize, HStack, Padding, ScrollArea, Segment, SegmentSizing,
    SegmentedControl, Spacer, TextWidget, Toggle, VStack,
};

use frontend::AppContext;
use frontend::commands::undo_redo_commands;
use frontend::common::entities::GoalUnit;
use skribisto_model::counting::CountingMethodSetting;

use crate::goals::distribute::{Plan, Weighting, plan};
use crate::goals::{format_goal, measure};
use crate::singles::SingleBinderItem;

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 460.0;

/// Everything the modal needs, resolved by the caller while it still has the tab's handles.
#[derive(Clone)]
pub struct Request {
    pub app_ctx: Rc<AppContext>,
    pub work_id: Option<u64>,
    pub item_id: u64,
    /// The container's own target — what is being shared out.
    pub parent_goal: i64,
    pub method: CountingMethodSetting,
    pub unit: GoalUnit,
    /// The undo stack this project writes on, so the whole distribution is one Ctrl+Z.
    pub stack: Option<u64>,
}

pub fn present(ctx: &mut EventContext, req: Request) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(DistributePanel::new(req.clone())))
            .presentation(ModalPresentation::InTree)
            .title(tr!(distribute_title()))
            .size(CARD_W as u32, CARD_H as u32)
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct DistributePanel {
    req: Request,
    /// The eligible children, measured once when the modal opens. Re-measuring on every
    /// weighting flip would re-walk the subtree for a number that cannot have changed
    /// while a modal is up.
    children: Vec<measure::Child>,
    weighting: Signal<usize>,
    overwrite: Signal<bool>,
    root_child: Option<WidgetId>,
}

/// The weighting modes, in the order the control offers them.
const MODES: [Weighting; 3] = [Weighting::Length, Weighting::Rows, Weighting::Even];

impl DistributePanel {
    fn new(req: Request) -> Self {
        let children = match req.work_id {
            Some(work_id) => {
                measure::children(&req.app_ctx, work_id, req.item_id, req.method, &req.unit)
            }
            None => Vec::new(),
        };
        Self {
            req,
            children,
            weighting: Signal::new(0),
            overwrite: Signal::new(false),
            root_child: None,
        }
    }

    fn plan(&self) -> Plan {
        let mode = MODES
            .get(self.weighting.get())
            .copied()
            .unwrap_or(Weighting::Length);
        plan(
            self.req.parent_goal,
            &self.children,
            mode,
            self.overwrite.get(),
        )
    }
}

impl std::fmt::Debug for DistributePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DistributePanel").finish()
    }
}

impl Widget for DistributePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.weighting
            .bind_to(sid, reg, teksilo::core::BindingLevel::Rebuild);
        self.overwrite
            .bind_to(sid, reg, teksilo::core::BindingLevel::Rebuild);

        let p = self.plan();
        let mut col = VStack::new().spacing(12.0);

        if self.children.is_empty() {
            col = col.child(
                TextWidget::new(tr!(distribute_empty()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
        } else {
            col = col
                .child(
                    HStack::new()
                        .spacing(10.0)
                        .child(
                            TextWidget::new(tr!(distribute_weight()))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Secondary),
                        )
                        .child(
                            SegmentedControl::indexed(self.weighting.clone())
                                .sizing(SegmentSizing::Fit)
                                .segment(Segment::new(tr!(distribute_weight_length())))
                                .segment(Segment::new(tr!(distribute_weight_rows())))
                                .segment(Segment::new(tr!(distribute_weight_even()))),
                        ),
                )
                .child(Toggle::new(self.overwrite.clone()).label(tr!(distribute_overwrite())))
                .child(ScrollArea::new().child(rows_table(&p)))
                .child(footer_line(&p, self.req.parent_goal));
        }

        let committable = p.is_committable();
        let plan_for_ok = p.clone();
        // `self` cannot be captured by the button's closure, so the write is prepared
        // here: the panel's state is already resolved into `plan_for_ok`, and the
        // handles it needs are cheap clones.
        let req = self.req.clone();
        let actions = HStack::new()
            .spacing(8.0)
            .child(Spacer::new())
            .child(
                Button::new(tr!(distribute_cancel()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(|ctx| ctx.dismiss_modal()),
            )
            .child(
                Button::new(tr!(distribute_apply()))
                    .variant(ButtonVariant::Filled)
                    .enabled(committable)
                    .on_activate_fn(move |ctx| {
                        commit_plan(&req, &plan_for_ok);
                        ctx.dismiss_modal();
                    }),
            );

        let root = ctx.add(
            FixedSize::new().width(CARD_W).height(CARD_H).child(
                Padding::symmetric(16.0, 16.0).child(
                    VStack::new()
                        .spacing(12.0)
                        .child(Expand::vertical().child(col))
                        .child(actions),
                ),
            ),
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// Write the plan as **one** undo entry.
///
/// A composite around per-item writes rather than a new backend use case: the outline
/// already does exactly this for "apply exportable to children", through the same
/// `begin_composite`/`end_composite` pair, and a use case would mean regenerating the one
/// crate this workspace flags as its most dangerous regen target — for a bulk write of a
/// single scalar the writer has explicitly confirmed in a preview.
///
/// A free function rather than a method so the OK handler can call it without capturing
/// the panel.
fn commit_plan(req: &Request, p: &Plan) {
    let ctx = &*req.app_ctx;
    let stack = req.stack;
    let _ = undo_redo_commands::begin_composite(ctx, stack);
    for row in p.rows.iter().filter(|r| r.changes()) {
        let probe = SingleBinderItem::new(req.app_ctx.clone());
        probe.set_id(Some(row.id));
        let wrote = match req.unit {
            GoalUnit::Words => probe.set_word_count_goal(row.proposed, stack),
            GoalUnit::Characters => probe.set_char_count_goal(row.proposed, stack),
        };
        if let Err(e) = wrote {
            eprintln!("distribute: set target failed for {}: {e}", row.id);
        }
    }
    undo_redo_commands::end_composite(ctx);
}

/// Title · current · proposed, one row per eligible child.
fn rows_table(p: &Plan) -> impl Widget + use<> {
    let mut col = VStack::new().spacing(6.0).child(
        HStack::new()
            .spacing(12.0)
            .child(
                Expand::horizontal().child(
                    TextWidget::new(tr!(distribute_col_item()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                ),
            )
            .child(
                FixedSize::new().width(90.0).child(
                    TextWidget::new(tr!(distribute_col_current()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                ),
            )
            .child(
                FixedSize::new().width(90.0).child(
                    TextWidget::new(tr!(distribute_col_proposed()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                ),
            ),
    );
    for row in &p.rows {
        // A row the plan leaves alone is dimmed, so "what would change" is legible without
        // reading two columns of digits against each other.
        let role = if row.changes() {
            TextRole::Primary
        } else {
            TextRole::Disabled
        };
        col =
            col.child(
                HStack::new()
                    .spacing(12.0)
                    .child(
                        Expand::horizontal().child(
                            TextWidget::new(lit!(row.title.clone()))
                                .color(role)
                                .single_line(),
                        ),
                    )
                    .child(FixedSize::new().width(90.0).child(
                        TextWidget::new(lit!(dash_or(row.current))).color(TextRole::Secondary),
                    ))
                    .child(
                        FixedSize::new()
                            .width(90.0)
                            .child(TextWidget::new(lit!(dash_or(row.proposed))).color(role)),
                    ),
            );
    }
    col
}

/// The sum, beside the parent's own target — the guarantee, made visible.
fn footer_line(p: &Plan, parent_goal: i64) -> impl Widget + use<> {
    if let Some(over) = p.over_budget {
        // Reported, never clamped: shrinking the writer's own numbers to fit would be the
        // app overruling a decision it was not asked about.
        return TextWidget::new(tr!(distribute_over_budget(over = format_goal(over))))
            .style(TextStyleRole::Small)
            .color(TextRole::Error);
    }
    TextWidget::new(tr!(distribute_total(
        total = format_goal(p.total),
        goal = format_goal(parent_goal)
    )))
    .style(TextStyleRole::Small)
    .color(TextRole::Secondary)
}

fn dash_or(n: i64) -> String {
    if n > 0 {
        format_goal(n)
    } else {
        "—".to_string()
    }
}
