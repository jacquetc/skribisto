// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status filter chip row above the table, beside the tag one.
//!
//! Two things make this row different from its neighbour, and both come from statuses
//! being an *ordered, single-valued* axis rather than a set:
//!
//! * **The chips are in ladder order**, not alphabetical. A filter row for a ladder that
//!   listed "Draft" above "Final" alphabetically would fight the thing it is filtering.
//! * **"No status" is a chip.** Absence is the zeroth member of the vocabulary, not a null
//!   — which is exactly what makes *"show me everything I have not triaged yet"* a question
//!   the writer can ask. A tag row has no equivalent, because "untagged" is not a tag.
//!
//! **Flat until it is doing something**, on the tag row's reasoning and for the same
//! defect: `Ghost` while off, `Filled` while on. The old `Plain`/`Tinted` pair painted
//! identically under IntUI, which maps `Tinted` to `Plain` — the row offered five chips
//! and no way to see which were on.
//!
//! Still **OR, not AND**, like the tag row: checking Draft and Needs work shows rows at
//! either. On a single-valued axis an AND reading would be empty by construction, which is
//! the strongest possible reason not to offer it.
//!
//! The ladder is **threaded from the tab**, never `ctx.app_state` — see
//! [`super::tag_filter`]'s own doc for the bug that lookup causes here: `app_state`
//! resolves to whichever window's session registered one last, which on the ordinary
//! launcher-first path is a throwaway session on a never-seeded `AppIds`, so the row would
//! silently never appear for the whole session however many rungs the project has.

#[allow(unused_imports)]
use super::*;

use teksilo::widgets::{Button, IconLocation, Wrap};

use crate::statuses::StatusesViewModel;

/// The sentinel the filter uses for "no status".
///
/// `0` is safe as a marker precisely because it is not a legal `EntityId`: the store never
/// mints one, which is the same reasoning `load_work_uc` relies on when it skips a
/// relationship read for an id of `0`.
pub const UNSET: u64 = 0;

/// Like the tag row, a project with no ladder mounts no child and resolves to zero height,
/// **its own padding included** — which is why the padding is inside this widget rather
/// than wrapped around it by the pane.
pub(super) fn status_filter_row(
    vm: &OverviewViewModel,
    statuses: StatusesViewModel,
) -> Box<dyn Widget> {
    Box::new(StatusFilterChips {
        selected: vm.status_filter_signal(),
        statuses,
        root: None,
    })
}

struct StatusFilterChips {
    selected: Signal<Vec<u64>>,
    statuses: StatusesViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for StatusFilterChips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusFilterChips").finish()
    }
}

impl Widget for StatusFilterChips {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.statuses.wire(ctx);
        self.statuses.revision().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.selected
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let ladder = self.statuses.ladder();
        if ladder.is_empty() {
            self.root = None;
            return Vec::new();
        }
        let current = self.selected.get();

        // One toggle, shared by every chip including the sentinel.
        let chip = |label: LocalizedString,
                    id: u64,
                    icon: Option<teksilo::widgets::IconWidget>,
                    selected: Signal<Vec<u64>>| {
            let on = current.contains(&id);
            let mut b = Button::new(label);
            if let Some(icon) = icon {
                b = b.icon(icon, IconLocation::Leading);
            }
            b.variant(if on {
                ButtonVariant::Filled
            } else {
                ButtonVariant::Ghost
            })
            .on_activate_fn(move |_c| {
                let mut next = selected.get();
                if on {
                    next.retain(|x| *x != id);
                } else {
                    next.push(id);
                }
                selected.set(next);
            })
            // Last: these wrap the `Button`. A filter chip is a two-state control, not a
            // command, and a screen reader announcing it as a button would say nothing
            // about which rungs are actually filtering the table.
            .access_role(teksilo::core::accesskit::Role::CheckBox)
            .access_customize(move |b| b.set_toggled(on))
        };

        let mut row = Wrap::new().spacing(6.0).line_spacing(6.0);
        // The sentinel leads, because it is where the ladder starts: unset is the least
        // finished thing there is, and the row reads left-to-right as progress.
        row = row.child(chip(tr!(status_none()), UNSET, None, self.selected.clone()));
        for rung in ladder {
            row = row.child(chip(
                lit!(rung.name.clone()),
                rung.id,
                Some(crate::statuses::status_glyph(&rung.category)),
                self.selected.clone(),
            ));
        }

        let id = ctx.add(Padding::symmetric(0.0, 4.0).child(row));
        self.root = Some(id);
        vec![id]
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
