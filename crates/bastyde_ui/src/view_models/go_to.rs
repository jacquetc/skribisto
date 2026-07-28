// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `GoToViewModel` — the "jump to any item" popup's state.
//!
//! The Go menu answers "next/previous Scene, Chapter or Note" (see
//! `app/commands/go.rs`), which is the right shape for drafting straight through
//! and useless for "take me to Chapter 19". This is the other half: a search
//! field over the binder tree, Enter to jump.
//!
//! **Its own tree model, not the outline dock's.** `BinderBinderItemsTreeModel`
//! already carries a live text filter in [`TreeFilters::query`], which is exactly
//! the projection this needs — but the outline dock's instance is driven by the
//! dock's own search field. Sharing it would mean typing in this popup silently
//! re-filtered the binder behind it (and vice versa), so this mints a second
//! model over the same `work_id` with its own filter signals. The rows are
//! re-sourced from the store, so the two stay consistent without being coupled.
//!
//! `TreeFilters::all_binders` is pinned **true** here, unlike the dock's: the
//! popup is a "find me anything in this project" affordance, and a match hiding
//! because it lives in a binder you are not currently scoped to would read as a
//! bug rather than a scope.
//!
//! **Per-window**, minted where [`GoAvailability`](super::GoAvailability) is
//! (`shell/windows.rs`), for the reason its module doc gives: a process-wide
//! instance would let one window's popup drive another window's editor.
//!
//! Activation goes through [`EditorsViewModel::open_or_focus`] — the same door
//! the binder tree, the Go commands and the recents list already use — so a jump
//! from here is indistinguishable downstream from any other navigation
//! (autosave, dirty tracking, `StatsModel::active_item` all see it).

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::data::{KeyedSelectionModel, SelectionMode};
use bastyde::prelude::*;
use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, TreeFilters, TreeNode};

/// What the popup does when a row is chosen. Set once by `App::build`, which is
/// the only place that has an `EditorsViewModel` — the view-model itself must
/// not import a peer (see the crate's cross-view-model DAG rule), so `App`
/// injects the edge rather than this reaching for it.
type OpenFn = Rc<dyn Fn(u64, &str)>;

#[derive(Clone)]
pub struct GoToViewModel {
    model: BinderBinderItemsTreeModel,
    selection: KeyedSelectionModel<BinderTreeKey>,
    filters: TreeFilters,
    /// Whether the popup is currently showing. Owned here rather than by the
    /// button so the `go.to` command can open it from a menu or shortcut without
    /// reaching into a widget.
    open: Signal<bool>,
    open_fn: Rc<RefCell<Option<OpenFn>>>,
}

impl GoToViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        let filters = TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            // Always project-wide — see this module's doc.
            all_binders: Signal::new(true),
        };
        let model =
            BinderBinderItemsTreeModel::new(app_ctx, ids.work_id.clone(), filters.clone());
        Self {
            model,
            // Single: this picks one destination. `Multi` would let arrow keys
            // accumulate a selection that Enter could only half-honour.
            selection: KeyedSelectionModel::new(SelectionMode::Single),
            filters,
            open: Signal::new(false),
            open_fn: Rc::new(RefCell::new(None)),
        }
    }

    /// Subscribe the underlying tree model to backend events. Must be called
    /// once from a `BuildContext`, exactly like the outline model's own `wire`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.model.wire(ctx);
    }

    /// Inject the "open this item" edge. `App::build` supplies
    /// `EditorsViewModel::open_or_focus`; tests supply a recorder.
    pub fn set_open_fn(&self, f: OpenFn) {
        *self.open_fn.borrow_mut() = Some(f);
    }

    pub fn model(&self) -> BinderBinderItemsTreeModel {
        self.model.clone()
    }

    pub fn selection(&self) -> KeyedSelectionModel<BinderTreeKey> {
        self.selection.clone()
    }

    /// The live query. Bound straight to the popup's `SearchField`; the tree
    /// model observes it and re-sources, which is what narrows the tree.
    pub fn query(&self) -> Signal<String> {
        self.filters.query.clone()
    }

    pub fn open_signal(&self) -> Signal<bool> {
        self.open.clone()
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    /// Open the popup on a clean slate.
    ///
    /// The query is cleared on every open rather than remembered: the previous
    /// jump's search is almost never the next one's, and a popup that opens
    /// pre-filtered to something you cannot see the cause of reads as an empty
    /// binder. The selection goes with it, so Enter can never fire at a row left
    /// over from last time.
    pub fn show(&self) {
        self.filters.query.set(String::new());
        self.selection.clear();
        self.open.set(true);
    }

    pub fn hide(&self) {
        self.open.set(false);
    }

    pub fn toggle(&self) {
        if self.open.get() {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Jump to `node`, if it names something openable, and close the popup.
    ///
    /// Binder rows (`item_id == None`) are **not** destinations — a binder is a
    /// container, not a document, and the editor has nothing to show for one.
    /// Choosing one is a quiet no-op that leaves the popup open, so the writer
    /// can pick again rather than having it close on them having done nothing.
    /// Returns whether the jump happened.
    pub fn activate(&self, node: &TreeNode) -> bool {
        let Some(item_id) = node.item_id else {
            return false;
        };
        let Some(open) = self.open_fn.borrow().clone() else {
            return false;
        };
        open(item_id, &node.title);
        self.hide();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn vm() -> GoToViewModel {
        GoToViewModel::new(Rc::new(AppContext::new()), AppIds::new())
    }

    fn item(title: &str, id: u64) -> TreeNode {
        TreeNode {
            title: title.to_string(),
            kind: "item".to_string(),
            item_id: Some(id),
            ..Default::default()
        }
    }

    fn binder_row(title: &str) -> TreeNode {
        TreeNode {
            title: title.to_string(),
            kind: "binder".to_string(),
            item_id: None,
            binder_id: Some(7),
            ..Default::default()
        }
    }

    #[test]
    fn the_popup_starts_closed_and_toggles() {
        let vm = vm();
        assert!(!vm.is_open());
        vm.toggle();
        assert!(vm.is_open());
        vm.toggle();
        assert!(!vm.is_open());
    }

    /// Opening always starts from a clean slate — a popup that reopened
    /// pre-filtered would look like a binder that had lost most of its rows.
    #[test]
    fn showing_clears_the_previous_query() {
        let vm = vm();
        vm.query().set("votanian".into());
        vm.show();
        assert_eq!(vm.query().get(), "", "the stale query must not survive");
    }

    #[test]
    fn activating_an_item_opens_it_and_closes_the_popup() {
        let vm = vm();
        let opened: Rc<Cell<Option<u64>>> = Rc::new(Cell::new(None));
        let sink = opened.clone();
        vm.set_open_fn(Rc::new(move |id, _title| sink.set(Some(id))));
        vm.show();

        assert!(vm.activate(&item("Chapter 19", 42)));
        assert_eq!(opened.get(), Some(42));
        assert!(!vm.is_open(), "a successful jump closes the popup");
    }

    /// A binder row is a container, not a destination. Choosing one must not
    /// close the popup — the writer has not gone anywhere yet.
    #[test]
    fn activating_a_binder_row_is_a_no_op_that_leaves_the_popup_open() {
        let vm = vm();
        let opened: Rc<Cell<Option<u64>>> = Rc::new(Cell::new(None));
        let sink = opened.clone();
        vm.set_open_fn(Rc::new(move |id, _title| sink.set(Some(id))));
        vm.show();

        assert!(!vm.activate(&binder_row("Writings")));
        assert_eq!(opened.get(), None, "nothing opened");
        assert!(vm.is_open(), "the popup stays up so another row can be picked");
    }

    /// With no `open_fn` injected (a window whose `App` has not built yet) an
    /// activation is inert rather than a panic — and must not close the popup,
    /// which would look like a jump that silently went nowhere.
    #[test]
    fn activating_without_an_open_fn_is_inert() {
        let vm = vm();
        vm.show();
        assert!(!vm.activate(&item("Chapter 1", 1)));
        assert!(vm.is_open());
    }

    /// The popup searches the whole project. A match hiding because it lives in
    /// a binder the outline dock is not scoped to would read as a missing item.
    #[test]
    fn the_search_is_always_project_wide() {
        let vm = vm();
        assert!(
            vm.filters.all_binders.get(),
            "all_binders is pinned on, unlike the outline dock's"
        );
    }
}
