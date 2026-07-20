// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Wiring the Pace view-model into the pane's build.

#[allow(unused_imports)]
use super::*;

// ── PaceWire: wire the view-model + the two-way binding effects ──────────────

/// Zero-size child that, on build, wires the view-model (its event subscriptions)
/// and registers the effects syncing the local field mirrors with the view-model
/// - the one place in the pane's tree that has a `BuildContext`.
pub(super) struct PaceWire {
    vm: PaceViewModel,
    goal_local: Signal<i64>,
    end_local: Signal<Option<Date>>,
    active_local: Signal<bool>,
    has_pace: Signal<usize>,
}

impl PaceWire {
    pub(super) fn new(
        vm: PaceViewModel,
        goal_local: Signal<i64>,
        end_local: Signal<Option<Date>>,
        active_local: Signal<bool>,
        has_pace: Signal<usize>,
    ) -> Self {
        Self {
            vm,
            goal_local,
            end_local,
            active_local,
            has_pace,
        }
    }
}

impl std::fmt::Debug for PaceWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceWire").finish()
    }
}

impl Widget for PaceWire {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);

        // Switcher index mirrors "does a Pace exist yet".
        {
            let s = self.has_pace.clone();
            ctx.effect(&self.vm.pace_id(), move |id| {
                let v = id.is_some() as usize;
                if s.get() != v {
                    s.set(v);
                }
            });
        }
        // Goal: VM → local (external edits, e.g. the Inspector / Goals settings).
        // Local → VM is the SpinBox's `on_value_changed`, so no local effect here.
        {
            let l = self.goal_local.clone();
            ctx.effect(&self.vm.goal_words(), move |g| {
                let v = (*g).max(0);
                if l.get() != v {
                    l.set(v);
                }
            });
        }
        // Active: two-way.
        {
            let l = self.active_local.clone();
            ctx.effect(&self.vm.active(), move |a| {
                if l.get() != *a {
                    l.set(*a);
                }
            });
        }
        {
            let vm = self.vm.clone();
            ctx.effect(&self.active_local, move |a| vm.set_active(*a));
        }
        // Deadline: two-way (chrono ↔ jiff at the boundary).
        {
            let l = self.end_local.clone();
            ctx.effect(&self.vm.end(), move |e| {
                let jd = naive_to_jiff_opt(*e);
                if l.get() != jd {
                    l.set(jd);
                }
            });
        }
        {
            let vm = self.vm.clone();
            ctx.effect(&self.end_local, move |e| {
                if let Some(jd) = *e {
                    let end = jiff_to_naive(jd);
                    // Keep the existing start; a Pace with no start yet begins today.
                    let start = vm.start().get().unwrap_or_else(|| Utc::now().date_naive());
                    vm.set_dates(start, end);
                }
            });
        }
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}
