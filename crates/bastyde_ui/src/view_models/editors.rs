//! `EditorsViewModel` — the split editor: two panes of open tabs (a primary and a
//! secondary/side pane) over the shared [`OpenDocsStore`](crate::models::OpenDocsStore).
//!
//! Single-instance live state: owns the two panes' `ListModel`s + selection
//! signals, the split state, and the horizontal `SplitterModel`; `App` creates
//! exactly one and shares it by clone. All document ownership + write-back lives
//! in the store, so opening the same item in both panes yields two tabs over one
//! live document.

use std::rc::Rc;

use bastyde::data::ListModel;
use bastyde::prelude::*; // Signal, tr!, lit!
use bastyde::widgets::{Orientation, PaneDescriptor, SplitterModel, TabHandle, TabId, TabInfo};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::work_management::SaveWorkDto;

use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;
use crate::tabs::ContentTab;
use crate::view_models::EditorTypographySet;

/// Which editor pane. `Primary` is always present; `Secondary` is the side pane,
/// revealed by the split view.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Primary,
    Secondary,
}

/// One editor pane: its dynamic tab model + selection.
#[derive(Clone)]
struct Pane {
    tabs: ListModel<TabHandle>,
    selected: Signal<Option<TabId>>,
}

impl Pane {
    fn new() -> Self {
        Self {
            tabs: ListModel::from_vec(Vec::new()),
            selected: Signal::new(None),
        }
    }
}

/// Minimum width of a pane, so the splitter can't crush an editor to nothing.
const PANE_MIN_WIDTH: f32 = 320.0;

#[derive(Clone)]
pub struct EditorsViewModel {
    app_ctx: Rc<AppContext>,
    primary: Pane,
    secondary: Pane,
    /// `true` when the side pane is shown. Drives the split button visual, the
    /// splitter's pane-1 visibility, and the primary drop-target's side zone.
    split_active: Signal<bool>,
    /// The horizontal splitter behind the two panes; pane 1 starts hidden.
    splitter: SplitterModel,
    /// Which pane's selection feeds `active_item` (the binder's open-item marker).
    focused_side: Signal<Side>,
    /// The `BinderItem` of the focused pane's active tab — the "open document".
    active_item: Signal<Option<u64>>,
    column_width: Signal<f32>,
    show_synopsis: Signal<bool>,
    typography: EditorTypographySet,
    /// Id-only global state (work + undo-stack ids); write-back lands on
    /// `ids.stack_id` so it shares the tree edits' Ctrl+Z history.
    ids: AppIds,
    /// The shared holder of open documents (app-state clone), refcounted per item.
    docs: OpenDocsStore,
}

impl EditorsViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        column_width: Signal<f32>,
        show_synopsis: Signal<bool>,
        typography: EditorTypographySet,
        ids: AppIds,
        docs: OpenDocsStore,
    ) -> Self {
        // Two equal panes; the side pane starts hidden (no divider) until split.
        // The Splitter sums *every* pane's `min_size` into its own intrinsic
        // minimum regardless of visibility, so the hidden side pane starts at
        // min_size 0 (raised to PANE_MIN_WIDTH only while shown, in `set_split`) —
        // otherwise it would inflate the editor area's minimum width when unsplit.
        let splitter = SplitterModel::from_panes(
            vec![
                PaneDescriptor::new().stretch(1.0).min_size(PANE_MIN_WIDTH),
                PaneDescriptor::new().stretch(1.0).min_size(0.0).visible(false),
            ],
            Orientation::Horizontal,
        );
        Self {
            app_ctx,
            primary: Pane::new(),
            secondary: Pane::new(),
            split_active: Signal::new(false),
            splitter,
            focused_side: Signal::new(Side::Primary),
            active_item: Signal::new(None),
            column_width,
            show_synopsis,
            typography,
            ids,
            docs,
        }
    }

    // ── View handles ────────────────────────────────────────────────────────

    /// The "an edit happened" signal — bind the debounced autosave to it.
    pub fn edited_signal(&self) -> Signal<u64> {
        self.docs.edited_any()
    }

    /// The dynamic-tab model for a pane's `TabWidget::dynamic_model`.
    pub fn tabs(&self, side: Side) -> ListModel<TabHandle> {
        self.pane(side).tabs.clone()
    }

    /// The selection signal for a pane's `TabWidget::new`.
    pub fn selected(&self, side: Side) -> Signal<Option<TabId>> {
        self.pane(side).selected.clone()
    }

    /// Whether the side pane is shown (drives the split button + side drop zone).
    pub fn split_active(&self) -> Signal<bool> {
        self.split_active.clone()
    }

    /// The splitter behind the two panes (hand to `Splitter::new`).
    pub fn splitter(&self) -> SplitterModel {
        self.splitter.clone()
    }

    /// The currently-open item id (focused pane's active tab). Bind a binder row's
    /// "open document" accent to this.
    pub fn active_item(&self) -> Signal<Option<u64>> {
        self.active_item.clone()
    }

    // ── Focus / active item ─────────────────────────────────────────────────

    /// Mark `side` as the focused pane and refresh the open-item marker. Called
    /// from `App`'s per-pane selection effects.
    pub fn set_focused(&self, side: Side) {
        self.focused_side.set(side);
        self.sync_active_item();
    }

    /// Recompute `active_item` from the focused pane's selected tab.
    pub fn sync_active_item(&self) {
        let side = self.focused_side.get();
        let active = self
            .pane(side)
            .selected
            .get()
            .and_then(|tab| self.item_of_tab(side, tab));
        if self.active_item.get() != active {
            self.active_item.set(active);
        }
    }

    // ── Open ────────────────────────────────────────────────────────────────

    /// Open (or focus) `item_id`'s tab in `side`. The view is chosen per
    /// `(role, sub_role)`; the document is shared through the store, so the same
    /// item can be open once in each pane over one live document.
    pub fn open_in(&self, side: Side, item_id: u64, title: &str) {
        if let Some(tid) = self.find_open(side, item_id) {
            self.pane(side).selected.set(Some(tid));
            self.set_focused(side);
            return;
        }
        let Some(doc) = self.docs.open(item_id) else {
            return;
        };
        let sub_role = doc.sub_role.clone();
        let tab = ContentTab::new(
            doc,
            self.column_width.clone(),
            self.show_synopsis.clone(),
            self.typography.clone(),
        );
        let tab_title = if title.is_empty() {
            tr!(untitled())
        } else {
            lit!(title.to_string())
        };
        let id = TabId::fresh();
        self.pane(side).tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new()
                .title(tab_title)
                .closable(true)
                .icon(move || crate::binder_icons::sub_role_icon(&sub_role)),
            tab,
        ));
        self.pane(side).selected.set(Some(id));
        self.set_focused(side);
    }

    /// Open (or focus) `item_id` in the primary pane — the default click / command
    /// path (back-compat with the activation callback + `editor.open_item`).
    pub fn open_or_focus(&self, item_id: u64, title: &str) {
        self.open_in(Side::Primary, item_id, title);
    }

    /// Open (or focus) `item_id` in the side pane, revealing the split first.
    pub fn open_to_side(&self, item_id: u64, title: &str) {
        self.set_split(true);
        self.open_in(Side::Secondary, item_id, title);
    }

    // ── Split ───────────────────────────────────────────────────────────────

    /// Show or collapse the side pane. Collapsing **closes** the side pane's tabs
    /// (flushing each) and hides the pane.
    pub fn set_split(&self, active: bool) {
        if active {
            if !self.split_active.get() {
                // Raise the side pane's min width only while shown (the Splitter
                // sums hidden panes' min_size into its own minimum otherwise).
                self.splitter.set_min_size(1, PANE_MIN_WIDTH);
                self.splitter.set_pane_visible(1, true);
                self.split_active.set(true);
            }
            return;
        }
        // Collapse: flush + close every side tab in one pass, then hide + shrink
        // the side pane.
        self.drain_pane(Side::Secondary);
        self.splitter.set_min_size(1, 0.0);
        self.splitter.set_pane_visible(1, false);
        self.split_active.set(false);
        self.set_focused(Side::Primary);
    }

    /// Flush + close every tab in `side` in one shot: release each document, clear
    /// the model, and reset the pane's selection with a single selection-effect
    /// fire — instead of reselecting (and re-scanning the whole store to flush)
    /// once per closed tab.
    fn drain_pane(&self, side: Side) {
        let stack = self.ids.stack_id.get();
        let pane = self.pane(side);
        let items: Vec<u64> = (0..pane.tabs.len())
            .filter_map(|i| {
                pane.tabs
                    .with_item(i, |h| {
                        h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id())
                    })
                    .flatten()
            })
            .collect();
        pane.tabs.clear();
        for id in items {
            self.docs.release(id, stack); // flushes + evicts on the last reference
        }
        pane.selected.set(None);
    }

    /// Toggle the split (the primary pane's split button).
    pub fn toggle_split(&self) {
        self.set_split(!self.split_active.get());
    }

    /// Collapse the split (the side pane's close-split button).
    pub fn close_split(&self) {
        self.set_split(false);
    }

    // ── Close / migrate ─────────────────────────────────────────────────────

    /// A pane's `TabWidget::on_close` hook: flush the tab, remove it, release its
    /// document. Closing the **last** side tab auto-collapses the split.
    pub fn close_in(&self, side: Side, tab_id: TabId) {
        self.close_tab(side, tab_id, true);
    }

    /// Flush + remove `tab_id` from `side`, reselect within the pane, and release
    /// its document (evicting on the last reference). With `auto_collapse`, an
    /// emptied side pane collapses the split.
    ///
    /// The tab is matched (and removed) by id **regardless of payload type** — a
    /// tab is always removable, so a stray non-`ContentTab` tab can never wedge a
    /// caller (e.g. a collapse loop); flush + document-release happen only for an
    /// editor tab.
    fn close_tab(&self, side: Side, tab_id: TabId, auto_collapse: bool) {
        let stack = self.ids.stack_id.get();
        let pane = self.pane(side);
        let mut idx = None;
        let mut item = None;
        for i in 0..pane.tabs.len() {
            // `Some(item_opt)` when the id matches (item_opt = the editor item id,
            // flushed here, or `None` for a non-editor tab); `None` otherwise.
            let matched = pane
                .tabs
                .with_item(i, |h| {
                    (h.id == tab_id).then(|| {
                        h.payload.downcast_ref::<ContentTab>().map(|t| {
                            let _ = t.flush(stack);
                            t.item_id()
                        })
                    })
                })
                .flatten();
            if let Some(item_opt) = matched {
                idx = Some(i);
                item = item_opt;
                break;
            }
        }
        let Some(idx) = idx else {
            return;
        };
        pane.tabs.remove(idx);
        if pane.selected.get() == Some(tab_id) {
            let next = (0..pane.tabs.len()).find_map(|i| pane.tabs.with_item(i, |h| h.id));
            pane.selected.set(next);
        }
        if let Some(item_id) = item {
            self.docs.release(item_id, stack);
        }
        if auto_collapse && side == Side::Secondary && self.secondary.tabs.is_empty() {
            self.set_split(false);
        }
        self.sync_active_item();
    }

    /// A pane's `TabWidget::on_transfer_out` hook: the tab is migrating to the
    /// other pane, so remove it here (replacing the framework's default removal)
    /// but do **not** release its document — the moved tab keeps its reference. If
    /// this empties the side pane, collapse the split (the same invariant
    /// `close_in` enforces, but for the drag-out path).
    pub fn transfer_out(&self, side: Side, tab_id: TabId) {
        let pane = self.pane(side);
        let mut pos = None;
        for i in 0..pane.tabs.len() {
            if pane.tabs.with_item(i, |h| h.id == tab_id) == Some(true) {
                pos = Some(i);
                break;
            }
        }
        if let Some(p) = pos {
            pane.tabs.remove(p);
        }
        if pane.selected.get() == Some(tab_id) {
            let next = (0..pane.tabs.len()).find_map(|i| pane.tabs.with_item(i, |h| h.id));
            pane.selected.set(next);
        }
        if side == Side::Secondary && self.secondary.tabs.is_empty() {
            self.set_split(false);
        }
    }

    /// A pane's `TabWidget::on_tab_received` hook (cross-pane migration): if the
    /// target pane already shows this item, dedup — drop the incoming (releasing
    /// its now-redundant document reference) and focus the existing tab; otherwise
    /// insert the migrated handle (which keeps its document reference).
    pub fn receive_tab(&self, side: Side, handle: TabHandle) {
        let item_id = handle
            .payload
            .downcast_ref::<ContentTab>()
            .map(|t| t.item_id());
        if let Some(item_id) = item_id {
            if let Some(existing) = self.find_open(side, item_id) {
                self.docs.release(item_id, self.ids.stack_id.get());
                self.pane(side).selected.set(Some(existing));
                self.set_focused(side);
                return;
            }
        }
        let id = handle.id;
        self.pane(side).tabs.push(handle);
        self.pane(side).selected.set(Some(id));
        self.set_focused(side);
    }

    // ── Flush / save / reset ────────────────────────────────────────────────

    /// Persist every open document's edits back to its `Content` rows (changed
    /// fields only), through the per-Work undo stack. Each shared doc flushed once.
    pub fn flush_all(&self) {
        self.docs.flush_all(self.ids.stack_id.get());
    }

    /// Flush all editors to the store, then write the project to disk
    /// (`save_work`, a long operation).
    pub fn save_to_disk(&self) {
        self.flush_all();
        let _ = work_management_commands::save_work(
            &self.app_ctx,
            &SaveWorkDto {
                file_name: String::new(),
                overwrite: true,
            },
        );
    }

    /// Close every tab in both panes and reset the split (e.g. on project load).
    /// Does not flush — the outgoing work is saved/discarded by the close flow.
    pub fn close_all(&self) {
        self.primary.tabs.clear();
        self.secondary.tabs.clear();
        // Drop the documents *before* clearing selection: `selected.set(None)`
        // synchronously re-fires App's per-pane effect (which calls `flush_all`),
        // so emptying the store first keeps that a cheap no-op and honors this
        // method's no-flush contract.
        self.docs.clear();
        self.primary.selected.set(None);
        self.secondary.selected.set(None);
        self.splitter.set_min_size(1, 0.0);
        self.splitter.set_pane_visible(1, false);
        self.split_active.set(false);
        self.focused_side.set(Side::Primary);
        self.active_item.set(None);
    }

    // ── Internals ───────────────────────────────────────────────────────────

    fn pane(&self, side: Side) -> &Pane {
        match side {
            Side::Primary => &self.primary,
            Side::Secondary => &self.secondary,
        }
    }

    /// `Some(tab id)` if an editor for `item_id` is open in `side`.
    fn find_open(&self, side: Side, item_id: u64) -> Option<TabId> {
        let pane = self.pane(side);
        for i in 0..pane.tabs.len() {
            let hit = pane.tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<ContentTab>()
                    .filter(|t| t.item_id() == item_id)
                    .map(|_| h.id)
            });
            if let Some(Some(tid)) = hit {
                return Some(tid);
            }
        }
        None
    }

    /// The `BinderItem` id behind a tab in `side`, if it's an editor tab.
    fn item_of_tab(&self, side: Side, tab: TabId) -> Option<u64> {
        let pane = self.pane(side);
        for i in 0..pane.tabs.len() {
            let hit = pane.tabs.with_item(i, |h| {
                if h.id == tab {
                    h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id())
                } else {
                    None
                }
            });
            if let Some(Some(id)) = hit {
                return Some(id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tabs;
    use crate::view_models::EditorTypography;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    fn test_typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
        }
    }

    fn editors() -> EditorsViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let docs = OpenDocsStore::new(app_ctx.clone(), ids.clone());
        EditorsViewModel::new(
            app_ctx,
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            ids,
            docs,
        )
    }

    /// Push a tab directly into `side` (bypassing the backend / store) so tab
    /// management can be tested without a loaded project.
    fn push_tab(vm: &EditorsViewModel, side: Side, item_id: u64) -> TabId {
        let id = TabId::fresh();
        let tab = tabs::tab_for(
            &vm.app_ctx,
            item_id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            vm.column_width.clone(),
            vm.show_synopsis.clone(),
            vm.typography.clone(),
            &vm.ids,
        );
        vm.pane(side)
            .tabs
            .push(TabHandle::dynamic(id, "editor", TabInfo::new().closable(true), tab));
        id
    }

    #[test]
    fn open_or_focus_dedupes_within_the_primary_pane() {
        let vm = editors();
        let id = push_tab(&vm, Side::Primary, 42);
        assert_eq!(vm.tabs(Side::Primary).len(), 1);
        vm.open_or_focus(42, "Scene"); // already open → focuses, no backend hit
        assert_eq!(vm.tabs(Side::Primary).len(), 1);
        assert_eq!(vm.selected(Side::Primary).get(), Some(id));
    }

    #[test]
    fn set_split_toggles_visibility_and_focus() {
        let vm = editors();
        assert!(!vm.split_active().get());
        vm.set_split(true);
        assert!(vm.split_active().get());
        assert!(vm.splitter().is_pane_visible(1));
        // Collapsing with a side tab open closes it and hides the pane.
        push_tab(&vm, Side::Secondary, 7);
        assert_eq!(vm.tabs(Side::Secondary).len(), 1);
        vm.set_split(false);
        assert!(!vm.split_active().get());
        assert!(!vm.splitter().is_pane_visible(1));
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
    }

    #[test]
    fn closing_last_side_tab_auto_collapses() {
        let vm = editors();
        vm.set_split(true);
        let id = push_tab(&vm, Side::Secondary, 9);
        vm.close_in(Side::Secondary, id);
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
        assert!(!vm.split_active().get(), "emptying the side pane collapses the split");
    }

    #[test]
    fn transfer_out_of_last_side_tab_collapses_the_split() {
        let vm = editors();
        vm.set_split(true);
        let id = push_tab(&vm, Side::Secondary, 3);
        // Simulate the framework's on_transfer_out (the tab dragged to the other
        // pane): it must empty the side pane AND collapse the split, like a close.
        vm.transfer_out(Side::Secondary, id);
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
        assert!(
            !vm.split_active().get(),
            "dragging out the last side tab collapses the split"
        );
    }

    #[test]
    fn receive_tab_dedupes_against_the_target_pane() {
        let vm = editors();
        // The same item is open in both panes (two tabs, one item).
        push_tab(&vm, Side::Primary, 5);
        let existing = push_tab(&vm, Side::Secondary, 5);
        // Migrating the primary's tab into the side pane (which already has it)
        // must not create a duplicate; it focuses the existing side tab.
        let migrating = TabHandle::dynamic(
            TabId::fresh(),
            "editor",
            TabInfo::new().closable(true),
            tabs::tab_for(
                &vm.app_ctx,
                5,
                &BinderItemRole::Item,
                &BinderItemSubRole::Scene,
                &[],
                vm.column_width.clone(),
                vm.show_synopsis.clone(),
                vm.typography.clone(),
                &vm.ids,
            ),
        );
        vm.receive_tab(Side::Secondary, migrating);
        assert_eq!(vm.tabs(Side::Secondary).len(), 1, "no duplicate in the side pane");
        assert_eq!(vm.selected(Side::Secondary).get(), Some(existing));
    }

    #[test]
    fn close_all_empties_both_panes_and_resets_split() {
        let vm = editors();
        push_tab(&vm, Side::Primary, 1);
        vm.set_split(true);
        push_tab(&vm, Side::Secondary, 2);
        vm.close_all();
        assert_eq!(vm.tabs(Side::Primary).len(), 0);
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
        assert!(!vm.split_active().get());
        assert_eq!(vm.active_item().get(), None);
    }
}
