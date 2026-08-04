// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Overview** pane — a container's whole subtree as a dense, sortable table.
//!
//! One segment of a Book / Part / Chapter-folder / Note-folder tab. A header (count ·
//! search · expand/collapse · ＋ New) sits over a virtualized
//! [`TreeTableView`](bastyde::widgets::TreeTableView) whose rows
//! come straight from the [`OverviewRowsModel`](crate::models::OverviewRowsModel) — a
//! `TreeDataSlice` keyed by durable uid, no `TreeModel` mirror. The view where you see the
//! shape of the book rather than the words of it — what is where, how long each piece is,
//! and what is still a stub.
//!
//! **Search reveals, it does not rearrange.** A match keeps its ancestors so it stays in
//! context, and the reveal is an override: clearing the box restores exactly the collapse
//! state the writer had. **Sort is per sibling group** — a book whose scenes were globally
//! sorted by word count would no longer be a book.

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    ButtonVariant, Center, Expand, HStack, IconButton, MenuItem, MenuList, Padding, Panel,
    SearchField, TextWidget, VStack,
};

use crate::binder::create_labels::{
    recommendation_label, recommendation_placement, recommendation_tooltip_key,
};
use crate::models::OverviewRow;
use crate::view_models::OverviewViewModel;

mod columns;
mod header;
mod table;
mod wire;

use columns::*;
use header::*;
use table::*;
use wire::*;

/// The pane: a wiring child, the header, and the table filling the rest.
pub fn overview_pane(tab: &super::ContentTab) -> Box<dyn Widget> {
    let Some(vm) = tab.overview().cloned() else {
        // Non-Overview tabs never reach here (the segment only exists for them), but the
        // `SegmentedControl` ↔ `Switcher` contract is positional, so the child must
        // exist regardless.
        return Box::new(VStack::new());
    };
    Box::new(
        VStack::new()
            .spacing(0.0)
            .child(WireOverview { vm: vm.clone() })
            .child(overview_header(&vm))
            .child(Expand::new().child(OverviewTable { vm, root: None })),
    )
}
