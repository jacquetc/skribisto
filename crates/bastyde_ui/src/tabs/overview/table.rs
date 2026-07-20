// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `TreeTableView` itself.

#[allow(unused_imports)]
use super::*;

use bastyde::data::{SortDirection, TreeDataSource};
use bastyde::widgets::{DragTransferMode, TreeTableView};

use crate::models::COL_TITLE;

/// The table wrapper.
///
/// Built over `from_source_keyed`, so the rows come **straight from the
/// `TreeDataSlice`** — no `TreeModel` mirror — and the selection is keyed by durable uid.
/// That is what lets a selection (and the expand state) survive a full re-source, which
/// every backend event triggers: a `TreeModel` mirror reassigns its `NodeId`s on rebuild
/// and cannot promise it.
///
/// Sorting is pushed **down into the source** (the view-model rebuilds the slice's
/// `TreeRowFilter`) rather than layered over it, so it stays per-sibling and the
/// hierarchy is preserved. The widget's own sort signal is therefore an *input* here, not
/// the mechanism.
pub(super) struct OverviewTable {
    pub(super) vm: OverviewViewModel,
    pub(super) root: Option<WidgetId>,
}

impl std::fmt::Debug for OverviewTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverviewTable").finish()
    }
}

impl Widget for OverviewTable {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild the cells when the inline-edit cursor moves: `is_editing` is read per
        // cell at build time, so the editor appears (and disappears) on a rebuild.
        self.vm.editing_cell().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // Rebuild when search/sort toggles: `reorderable` is a build-time flag, and a
        // projected table must not be draggable (see below).
        self.vm.is_projecting().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let vm = self.vm.clone();
        let table = TreeTableView::from_source_keyed(self.vm.rows(), self.vm.selection())
            .columns(overview_columns(&vm))
            .tree_column(COL_TITLE)
            .auto_row_height(26.0)
            .alternating_rows(true)
            .a11y_label(tr!(overview_table_label()));

        // Sort: the header writes the widget's signal; we mirror it into the view-model,
        // which reshapes the source. Guarded both ways so the two cannot ping-pong.
        {
            let target = self.vm.sort_signal();
            ctx.effect(table.sort_signal(), move |s| {
                if target.get() != *s {
                    target.set(s.clone());
                }
            });
        }
        // ...and seed the widget from any sort the view-model already holds (a rebuild
        // must not silently drop it).
        {
            let current: Option<(String, SortDirection)> = self.vm.sort_signal().get();
            match current {
                Some((col, dir)) => table.set_sort(Some(&col), dir),
                None => table.clear_sort(),
            }
        }
        // Seed the **editing cell** the same way, and for the same reason.
        //
        // `CellContext::is_editing` — the only thing the cell delegates consult to swap
        // in an editor — is computed from the *widget's* own `editing_cell`, addressed by
        // (row index, display column). The view-model addresses the edit by (uid, column
        // id), because a durable key is what survives the re-sources this table does
        // constantly. Nothing bridged the two, so "Rename" set the view-model's intent and
        // no cell ever noticed: the menu item did nothing at all.
        //
        // It has to be re-seeded on **every** build, not once: this widget rebuilds
        // whenever the edit cursor moves, and each rebuild constructs a brand-new
        // `TreeTableView` whose `editing_cell` starts at `None`. That is exactly why the
        // sort seed above exists too.
        if let Some((uid, col_id)) = self.vm.editing_cell().get()
            && let Some(row) = self.vm.rows().flat_index_of(&uid)
        {
            table.begin_edit(row, &col_id);
        }

        let activate_vm = self.vm.clone();
        let edit_vm = self.vm.clone();
        let keys_vm = self.vm.clone();
        let empty_vm = self.vm.clone();

        let table = table
            // Double-click / Enter opens the row in the editor. Single-click would fight
            // the inline editors and the twist — an outliner is a place you *work in*,
            // not only a launcher (which is why the outline dock, whose whole job is
            // launching, uses single-click and this does not).
            .on_row_activate(move |idx, ctx| activate_vm.activate(ctx, idx))
            .on_cell_edit_request(move |idx, col_id, _ctx| {
                // The request arrives positionally; resolve it to the row's durable uid
                // immediately, so the edit stays attached to the *row* even if a
                // concurrent reload shifts the indices under it.
                if let Some(uid) = edit_vm.rows().key_at(idx) {
                    edit_vm.begin_edit(uid, col_id);
                }
            })
            // Type-ahead jumps by title. The delegate is handed the row itself, so there
            // is nothing to resolve and the closure captures nothing.
            .type_ahead_label(|row: &OverviewRow| row.title.clone())
            // Reorder by drag, exactly as in the outline tree: the source owns the
            // commit, so a drop is one `move_items` call with undo.
            //
            // **Off while projecting.** The slice being drawn *is* the sorted/filtered
            // tree, so a drop would resolve its neighbour in projected order and write a
            // manuscript order the writer never chose. Same call the Corkboard makes.
            .reorderable(!self.vm.is_projecting().get())
            // Rows can also be dragged OUT onto an editor pane, which opens them.
            // `Copy` leaves the row in place, so one drag can mean either.
            .exportable(DragTransferMode::Copy)
            .empty_view(move || {
                Box::new(OverviewEmpty {
                    vm: empty_vm.clone(),
                    root: None,
                })
            })
            // Delete trashes the selection. Attached last: this is a `WidgetBuilder`
            // hook, so no table-specific call may follow.
            //
            // **F2 is deliberately not handled here.** The table's own key handler already
            // implements it (`EditTrigger::F2OrTypeOrDoubleClick` is the default): it sets
            // the widget's `editing_cell` and fires `on_cell_edit_request`, which is wired
            // above into `begin_edit`. A second F2 handler would not override that —
            // bastyde fires the external and the widget's own handler *both*, with no
            // short-circuit on `Handled` — it would only run a second, different action
            // (rename the first *selected* row rather than the focused cell) on top.
            .on_key(move |ev, _ctx| match ev {
                WidgetEvent::KeyDown {
                    key: Key::Delete, ..
                } => {
                    keys_vm.trash_selected();
                    EventResponse::Handled
                }
                _ => EventResponse::Ignored,
            });
        // The row context menu is attached **per cell** (see `columns::with_row_menu`),
        // not to the table: a menu on the table would only know where the pointer was,
        // and would have to hit-test its own rows to find out what was clicked. A cell
        // already knows its row.

        let id = ctx.add(table);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
