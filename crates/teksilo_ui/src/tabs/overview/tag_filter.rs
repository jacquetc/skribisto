// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The tag filter chip row above the table: the filter half of a promise the
//! Tags column's own doc comment already made (`columns::tags_column`: "finding
//! tagged rows is a filter question, not a sort one"). The column itself stays
//! exactly as it was: display-only, unsortable. This is the surface that answers
//! the filter question instead.
//!
//! **OR, not AND.** Checking two chips shows every row carrying *either* tag:
//! see [`OverviewFilters::tag_filter`]'s own doc for why that reading, not "every
//! checked tag on the same row", is the one a chip row visually promises.
//!
//! `TagsViewModel` is read from `app_state` here, not taken as a constructor
//! parameter, the same choice [`crate::tags::tag_chip::TagDotsRow`] makes and for
//! the same reason: this is a plain composition function with no `BuildContext`
//! of its own to thread a view-model down through, and the palette is a
//! documented, one-instance-per-window singleton.

#[allow(unused_imports)]
use super::*;

use teksilo::widgets::{Button, Wrap};

use crate::tags::TagsViewModel;

/// `None` when no palette exists to filter by at all: the row costs a
/// childless project nothing, the same "no chrome for a question nobody can
/// ask yet" discipline the Books column and the Story bible place's own Books
/// chip row both follow.
pub(super) fn tag_filter_row(vm: &OverviewViewModel) -> Box<dyn Widget> {
    Box::new(TagFilterChips {
        selected: vm.tag_filter_signal(),
        root: None,
    })
}

struct TagFilterChips {
    selected: Signal<Vec<u64>>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for TagFilterChips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagFilterChips").finish()
    }
}

impl Widget for TagFilterChips {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let Some(tags_vm) = ctx.app_state::<TagsViewModel>().cloned() else {
            self.root = None;
            return Vec::new();
        };
        tags_vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.selected
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let palette = tags_vm.rows();
        if palette.is_empty() {
            self.root = None;
            return Vec::new();
        }
        let current = self.selected.get();

        let mut row = Wrap::new().spacing(6.0).line_spacing(6.0);
        for t in palette {
            let id = t.id;
            let on = current.contains(&id);
            let selected = self.selected.clone();
            row = row.child(
                Button::new(lit!(t.name.clone()))
                    .variant(if on {
                        ButtonVariant::Tinted
                    } else {
                        ButtonVariant::Plain
                    })
                    .on_activate_fn(move |_c| {
                        let mut next = selected.get();
                        if on {
                            next.retain(|x| *x != id);
                        } else {
                            next.push(id);
                        }
                        selected.set(next);
                    }),
            );
        }
        let id = ctx.add(row);
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
