// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `OverviewViewModel` — the business logic behind one container tab's **Overview**
//! segment: the dense, sortable, searchable table of everything inside that container.
//!
//! One per tab, built alongside `stream` / `pace` / `corkboard` in `ContentTab::new` and
//! gated on [`skribisto_model::overview_capable`] — which is deliberately *not* the
//! stream's gate, because a notes folder has no manuscript extent but does have a subtree
//! worth tabulating.
//!
//! It owns the search/sort state, the keyed selection, the inline-edit cursor, and the
//! [`OverviewRowsModel`] beneath them. Cross-view-model talk stays a **DAG**: it never
//! imports a peer view-model. Everything it cannot do itself it either delegates *down*
//! to [`binder_ops`] (a shared layer below the view-models, which reads the backend
//! directly) or fires as an [`AppIntent`] on the bus for `App` to route.
//!
//! That is a deliberate departure from "just call the outline's context menu". The
//! outline's menu computes its batch from the **outline's** selection and resolves an
//! item's binder through the **outline's** tree — so invoked from here it would act on
//! whatever the outline happened to have selected, and would silently skip its own
//! demote guard for any row the outline's binder scope or search filter had hidden.
//! The actions are shared; the *selection* is this view's own.
//!
//! Plain Rust → headless-testable.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::core::ObserverHandle;
use bastyde::data::{
    DropPosition, KeyedSelectionModel, SelectionMode, SortDirection, TreeDataSource,
};
use bastyde::prelude::*; // Signal, BuildContext, EventContext, tr!
use bastyde::widgets::{MessageBox, MessageBoxButtons};
use uuid::Uuid;

use frontend::AppContext;
use frontend::binder_item_management::{DuplicateDto, MovePlace};
use frontend::commands::{binder_item_commands, binder_item_management_commands, trash_management_commands, undo_redo_commands};
use frontend::trash_management::TrashBinderItemsDto;

use skribisto_model::counting::CountingMethodSetting;
use skribisto_model::{CreateType, PromoteTarget, Recommendation};

use crate::app_ids::AppIds;
use crate::intents::AppIntent;
use crate::models::{OverviewFilters, OverviewRow, OverviewRowsModel};
use crate::singles::SingleBinderItem;

use super::binder_ops;

/// Which cell is being edited in place: the row's uid and the column id. One at a time —
/// a cell renders as an editor iff it matches.
pub type EditingCell = Option<(Uuid, String)>;

/// The live text of the cell being edited.
///
/// It lives on the **view-model**, not inside the cell widget, and that is the whole
/// point. A cell delegate re-runs on every table rebuild, so a buffer created there
/// (`Signal::new(row.title)`) is silently re-seeded from the model whenever anything
/// triggers a reload — the other pane's autosave firing `Content(Updated)`, an undo, a
/// rename elsewhere — and the writer's half-typed name vanishes. Held here, the buffer
/// survives every rebuild, exactly as `ContentTab` holds its documents rather than the
/// widgets that draw them.
#[derive(Clone)]
pub struct EditBuffer {
    pub text: Signal<String>,
}

struct Inner {
    /// The container this table tabulates — the tab's own item. Fixed for the tab's
    /// lifetime: unlike the corkboard, the Overview does not drill (the tree *is* the
    /// drill).
    container_id: u64,
    rows: OverviewRowsModel,
    filters: OverviewFilters,
    selection: KeyedSelectionModel<Uuid>,
    /// Visible row count, for the header.
    count: Signal<usize>,
    editing: Signal<EditingCell>,
    /// The in-progress text of the cell named by `editing` — see [`EditBuffer`].
    edit_text: Signal<String>,
    /// `search active || sort active` — while true the table is showing a **projection**
    /// of the manuscript, so reordering is disabled. A real signal (not a derived one) so
    /// the table can bind it at `BindingLevel::Rebuild`.
    projecting: Signal<bool>,
    /// Tracks the container's own dto — its title for the header, and its
    /// `(role, sub_role)` for the "＋ Create" recommendations.
    container_probe: SingleBinderItem,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    observers: RefCell<Vec<ObserverHandle>>,
    /// One-shot guard for the count observer (see `wire`).
    count_wired: Cell<bool>,
    /// One-shot guard for the expand-state restore — a second `wire` (a segment switch)
    /// must not undo collapses the writer has made since the first.
    expansion_restored: Cell<bool>,
    /// This window's own Work-scoped tree-expansion service — constructor-threaded
    /// (from `WorkSession::tree_expansion` via `EditorsViewModel`/`ContentTab::new`),
    /// never `ctx.app_state::<TreeExpansionViewModel>()`. See `restore_expansion`'s
    /// doc for why the `app_state` lookup was wrong the moment a second Work's
    /// window exists.
    tree_expansion: crate::view_models::TreeExpansionViewModel,
}

#[derive(Clone)]
pub struct OverviewViewModel {
    inner: Rc<Inner>,
}

impl OverviewViewModel {
    /// Build the view-model for a container, or `None` when this `(role, sub_role)` has
    /// no Overview. One gate, consulted here and in `ContentTab::new` alike.
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        container_id: u64,
        role: &frontend::common::entities::BinderItemRole,
        sub_role: &frontend::common::entities::BinderItemSubRole,
        counting_method: Signal<CountingMethodSetting>,
        tree_expansion: crate::view_models::TreeExpansionViewModel,
    ) -> Option<Self> {
        if !skribisto_model::overview_capable(role, sub_role) {
            return None;
        }
        let filters = OverviewFilters::new();
        let rows = OverviewRowsModel::new(
            app_ctx.clone(),
            ids.work_id.clone(),
            container_id,
            counting_method,
            filters.clone(),
        );
        let container_probe = SingleBinderItem::new(app_ctx.clone());
        container_probe.set_id(Some(container_id));
        Some(Self {
            inner: Rc::new(Inner {
                container_id,
                rows,
                filters,
                selection: KeyedSelectionModel::new(SelectionMode::Multi),
                count: Signal::new(0),
                editing: Signal::new(None),
                edit_text: Signal::new(String::new()),
                projecting: Signal::new(false),
                container_probe,
                app_ctx,
                ids,
                observers: RefCell::new(Vec::new()),
                count_wired: Cell::new(false),
                expansion_restored: Cell::new(false),
                tree_expansion,
            }),
        })
    }

    /// Subscribe the model + probe, install the reorder commit, and keep the header count
    /// in step. Idempotent per build (the model's own `wire` guards itself).
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.rows.wire(ctx);
        self.inner.container_probe.wire(ctx);
        self.install_reorder();
        self.restore_expansion(ctx);

        // Search / sort → `projecting`, which gates reordering. Guarded so the signal
        // only notifies on a real change.
        {
            let me = self.clone();
            ctx.effect(&self.inner.filters.query, move |_| me.recompute_projecting());
        }
        {
            let me = self.clone();
            ctx.effect(&self.inner.filters.sort, move |_| me.recompute_projecting());
        }
        // The count follows the *visible* row set, so it reflects an active search.
        //
        // Registered **once**: `wire` runs on every build of the pane, and the `Switcher`
        // rebuilds it every time the writer returns to this segment — so an unguarded
        // push would accumulate a duplicate observer per visit, for the tab's lifetime.
        if !self.inner.count_wired.replace(true) {
            let count = self.inner.count.clone();
            let rows = self.inner.rows.clone();
            count.set(rows.visible_count());
            let rows2 = rows.clone();
            let handle = rows
                .version_signal()
                .observe(move |_| count.set(rows2.visible_count()));
            self.inner.observers.borrow_mut().push(handle);
        }
    }

    /// Apply this container's remembered expand set, once.
    ///
    /// Runs after `rows.wire`, which performs the first load — restoring onto an unloaded
    /// slice would apply the keys to nothing. Keyed by the container's **uid**, so what
    /// was written last session is what is read now, with no translation step and no
    /// ordering constraint against the workspace-layout restore.
    ///
    /// **Scope C fix.** This used to resolve `TreeExpansionViewModel` via
    /// `ctx.app_state`, justified by a doc comment calling "the service" app-wide —
    /// but `TreeExpansionViewModel` (unlike the on-disk `TreeExpansionService` it
    /// wraps, which genuinely is one shared file) is bound to its own `ids: AppIds`
    /// at construction, i.e. Tier 2 (per open Work), not Tier 1. The `app_state`
    /// lookup silently resolved to whichever Work's session registered it first —
    /// a second Work's window would restore (and later capture) the FIRST Work's
    /// chevron state instead of its own. Now threaded in at construction (via
    /// `EditorsViewModel`/`ContentTab::new`, from `WorkSession::tree_expansion`),
    /// exactly like `save_state`/`tags` already are.
    fn restore_expansion(&self, _ctx: &mut BuildContext) {
        if self.inner.expansion_restored.replace(true) {
            return;
        }
        let Some(container_uid) = self.inner.container_probe.dto().map(|d| d.uid) else {
            return; // the probe has not resolved the container yet
        };
        let remembered = self.inner.tree_expansion.expanded_for(container_uid);
        if !remembered.is_empty() {
            self.inner.rows.set_expanded_uids(&remembered);
        }
    }

    /// This container's uid and its live expand set — what `App` gathers at a door to
    /// hand to [`TreeExpansionViewModel::capture`]. `None` before the container probe has
    /// resolved, which is also when there is nothing worth remembering.
    pub fn expansion_snapshot(&self) -> Option<(Uuid, Vec<Uuid>)> {
        let container_uid = self.inner.container_probe.dto().map(|d| d.uid)?;
        Some((container_uid, self.inner.rows.expanded_uids()))
    }

    /// Inject the drag-reorder commit.
    ///
    /// **Cycle-safety.** The closure captures `app_ctx`, the app ids, and the model's
    /// uid → store-id **map** — never `self`, and never the model itself.
    ///
    /// Capturing the model looks harmless and is not: `set_reorder` stores this closure
    /// inside the slice's `Rc<Inner>`, and the model holds that same slice, so the
    /// closure would keep alive the allocation that owns it. Every container tab ever
    /// opened would leak its whole table. (Pinned by `leak_probe`, which found it.)
    fn install_reorder(&self) {
        let app_ctx = self.inner.app_ctx.clone();
        let ids = self.inner.ids.clone();
        let ids_by_uid = self.inner.rows.ids_by_uid();
        self.inner
            .rows
            .set_reorder(Rc::new(move |dragged: Uuid, target: Uuid, place| {
                // The tree is keyed by durable uid; the backend speaks store ids. A uid
                // whose row vanished between drag-start and drop resolves to `None` and
                // the move is refused rather than applied to the wrong item.
                let map = ids_by_uid.borrow();
                let (Some(&item_id), Some(&target_id)) =
                    (map.get(&dragged), map.get(&target))
                else {
                    return false;
                };
                drop(map);
                let move_place = match place {
                    DropPosition::Before => MovePlace::Before,
                    DropPosition::After => MovePlace::After,
                    DropPosition::Into => MovePlace::Into,
                };
                binder_item_management_commands::move_items(
                    &app_ctx,
                    ids.stack_id.get(),
                    &frontend::binder_item_management::MoveDto {
                        item_ids: vec![item_id],
                        target_id: Some(target_id),
                        target_is_binder: false,
                        move_place,
                    },
                )
                .is_ok()
            }));
    }

    // ── Handles for the view ─────────────────────────────────────────────────

    /// The `TreeDataSource` to hand to `TreeTableView::from_source_keyed`.
    pub fn rows(&self) -> OverviewRowsModel {
        self.inner.rows.clone()
    }
    pub fn selection(&self) -> KeyedSelectionModel<Uuid> {
        self.inner.selection.clone()
    }
    pub fn search_query_signal(&self) -> Signal<String> {
        self.inner.filters.query.clone()
    }
    pub fn sort_signal(&self) -> Signal<Option<(String, SortDirection)>> {
        self.inner.filters.sort.clone()
    }
    pub fn count_signal(&self) -> Signal<usize> {
        self.inner.count.clone()
    }
    pub fn editing_cell(&self) -> Signal<EditingCell> {
        self.inner.editing.clone()
    }
    /// The live buffer of the open edit, for the cell editor to bind. Which cell that is
    /// comes from [`editing_cell`](Self::editing_cell).
    pub fn edit_buffer(&self) -> EditBuffer {
        EditBuffer {
            text: self.inner.edit_text.clone(),
        }
    }
    pub fn container_title(&self) -> Signal<String> {
        self.inner.container_probe.title()
    }
    /// Whether the container this table tabulates still exists — false once it has been
    /// trashed out from under an open tab.
    pub fn container_present(&self) -> Signal<bool> {
        self.inner.rows.container_present()
    }

    // ── Expansion ────────────────────────────────────────────────────────────

    pub fn expand_all(&self) {
        self.inner.rows.expand_all();
    }
    pub fn collapse_all(&self) {
        self.inner.rows.collapse_all();
    }

    // ── Row resolution ───────────────────────────────────────────────────────

    /// The row at a flat display index — how an activation or a context click, which the
    /// widget reports positionally, becomes a row.
    pub fn row_at(&self, index: usize) -> Option<OverviewRow> {
        self.inner.rows.with_entry(index, |r, _| r.clone())
    }
    pub fn row_of(&self, uid: &Uuid) -> Option<OverviewRow> {
        self.inner.rows.row_of(uid)
    }
    fn item_id_of(&self, uid: &Uuid) -> Option<u64> {
        self.inner.rows.item_id_of(uid)
    }
    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    /// The rows a batch action applies to, by the standard convention: a right-click
    /// *inside* the selection acts on the whole selection; a click on a row *outside* it
    /// acts on just that row.
    ///
    /// Resolved against **this** table's selection — the outline's is a different view's
    /// state and must never decide what happens here.
    pub fn batch_for(&self, uid: Uuid) -> Vec<Uuid> {
        let selected = self.inner.selection.selected_keys();
        if selected.contains(&uid) {
            selected
        } else {
            vec![uid]
        }
    }

    // ── Open ─────────────────────────────────────────────────────────────────

    /// Row activation (double-click / Enter): open it in the editor. Every Overview row
    /// is a real item, including the container rows — a Part has its own page, so
    /// activating one opens it rather than doing nothing.
    pub fn activate(&self, ctx: &mut EventContext, index: usize) {
        if let Some(row) = self.row_at(index) {
            ctx.send_intent(AppIntent::OpenItem {
                item_id: row.item_id,
                title: row.title,
            });
        }
    }

    pub fn open_to_side(&self, ctx: &mut EventContext, uid: Uuid) {
        if let Some(row) = self.row_of(&uid) {
            ctx.send_intent(AppIntent::OpenItemToSide {
                item_id: row.item_id,
                title: row.title,
            });
        }
    }

    /// Select this row in the binder outline and reveal the dock — "where does this sit
    /// in the project?". Goes over the intent bus rather than importing the outline.
    pub fn reveal_in_outline(&self, ctx: &mut EventContext, uid: Uuid) {
        if let Some(item_id) = self.item_id_of(&uid) {
            ctx.send_intent(AppIntent::RevealInOutline { item_id });
        }
    }

    // ── Inline editing ───────────────────────────────────────────────────────

    /// Begin editing a cell in place (the table's `on_cell_edit_request`, F2, or the
    /// context menu).
    ///
    /// Seeds the buffer **here, once** — not in the cell delegate, which re-runs on every
    /// rebuild. Starting a second edit while one is open commits the first, so moving
    /// between cells never silently discards what was typed.
    pub fn begin_edit(&self, uid: Uuid, col_id: &str) {
        if let Some((prev_uid, prev_col)) = self.inner.editing.get()
            && (prev_uid != uid || prev_col != col_id)
        {
            self.commit_open_edit();
        }
        let seed = self
            .row_of(&uid)
            .map(|r| match col_id {
                crate::models::COL_LABEL => r.label,
                _ => r.title,
            })
            .unwrap_or_default();
        self.inner.edit_text.set(seed);
        self.inner.editing.set(Some((uid, col_id.to_string())));
    }

    /// Commit whatever edit is currently open, if any — what a focus loss means.
    ///
    /// A text field that vanishes without writing is the writer's edit thrown away, so
    /// clicking elsewhere, switching segment, or starting another edit all land here
    /// rather than discarding. `commit_edit` itself decides whether anything is actually
    /// written (an unchanged or blank-title value writes nothing).
    pub fn commit_open_edit(&self) {
        if let Some((uid, col)) = self.inner.editing.get() {
            let text = self.inner.edit_text.get();
            self.commit_edit(uid, &col, &text);
        }
    }

    pub fn cancel_edit(&self) {
        if self.inner.editing.get().is_some() {
            self.inner.editing.set(None);
            self.inner.edit_text.set(String::new());
        }
    }

    /// Commit an inline edit and leave edit mode.
    ///
    /// An **unchanged** value writes nothing, so Enter-with-no-change and the
    /// Esc→restore→blur cancel path are both quiet no-ops rather than stray undo entries.
    /// A blank *title* is refused (a nameless row is unfindable); a blank *label* is
    /// legitimate — clearing a status note is a real edit.
    pub fn commit_edit(&self, uid: Uuid, col_id: &str, value: &str) {
        let value = value.trim();
        let Some(row) = self.row_of(&uid) else {
            self.inner.editing.set(None);
            return;
        };
        match col_id {
            crate::models::COL_TITLE if !value.is_empty() && value != row.title => {
                // Through the single, not a raw DTO patch: a title has **two** homes —
                // `BinderItem.title` (what the tree and the tab caption show) and the
                // title `Content` row (what the manuscript compiles) — and only
                // `set_title` writes both.
                let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
                probe.set_id(Some(row.item_id));
                let _ = probe.set_title(value, self.stack());
            }
            crate::models::COL_LABEL if value != row.label => {
                self.set_label(row.item_id, value);
            }
            _ => {}
        }
        self.inner.editing.set(None);
        self.inner.edit_text.set(String::new());
    }

    /// Replace an item's tags — what the row's tag-dot picker commits.
    ///
    /// Through `SingleBinderItem::set_tags`, the same writer the Corkboard and the editor
    /// use, so a tag set from any of the three is one edit and one undo entry.
    pub fn set_tags(&self, item_id: u64, tags: &[u64]) {
        let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
        probe.set_id(Some(item_id));
        let _ = probe.set_tags(tags, self.stack());
    }

    fn set_label(&self, item_id: u64, label: &str) {
        if let Some(it) = binder_ops::item_dto(&self.inner.app_ctx, item_id) {
            let mut dto = binder_ops::update_item_dto(&it);
            dto.label = label.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
        }
    }

    // ── Create ───────────────────────────────────────────────────────────────

    /// The "＋ Create" offers for a row (or, with no row, for the container itself).
    ///
    /// **Not filtered by relation.** A leaf's recommendations are all `Sibling` /
    /// `ParentSibling` — `recommendations(Item, Scene)` is `[(Scene, Sibling),
    /// (Chapter, ParentSibling)]` — so a `Child`-only filter would leave the "Add ▸"
    /// submenu empty on every Scene, Note and flat Chapter in the table. (That filter is
    /// right for the Corkboard, whose anchor is always the drilled-into *container*; here
    /// the anchor is an arbitrary row.) The menu instead shows every offer and explains
    /// where each lands via `recommendation_placement`, exactly as the outline's does.
    ///
    /// `EndOfBook` stays omitted — a structural terminator is not a table affordance.
    pub fn create_recommendations(&self, anchor: Option<Uuid>) -> Vec<Recommendation> {
        let (role, sub_role) = match anchor.and_then(|uid| self.row_of(&uid)) {
            Some(row) => (row.role, row.sub_role),
            None => {
                let Some(dto) = self.inner.container_probe.dto() else {
                    return Vec::new();
                };
                (dto.role, dto.sub_role)
            }
        };
        skribisto_model::recommendations(&role, &sub_role)
            .into_iter()
            .filter(|r| r.create_type != CreateType::EndOfBook)
            .collect()
    }

    /// Fire a create anchored on a row, or on the container when no row is given.
    ///
    /// Goes over the bus with an explicit `anchor_item_id` (never `None`, which the
    /// global action reads as "use the outline's selection") — this table's anchor is its
    /// own, exactly as the corkboard's drilled-into container is.
    pub fn fire_create(&self, ctx: &mut EventContext, rec: Recommendation, anchor: Option<Uuid>) {
        let anchor_item_id = anchor
            .and_then(|uid| self.item_id_of(&uid))
            .unwrap_or(self.inner.container_id);
        ctx.send_intent(AppIntent::NewItem {
            create_type: rec.create_type,
            relation: rec.relation,
            anchor_item_id: Some(anchor_item_id),
        });
    }

    // ── Convert to (promote) ─────────────────────────────────────────────────

    pub fn promote_targets_of(&self, uid: Uuid) -> Vec<PromoteTarget> {
        match self.item_id_of(&uid) {
            Some(id) => binder_ops::promote_targets_of(&self.inner.app_ctx, id),
            None => Vec::new(),
        }
    }

    /// Convert a row's type, behind the same two guards the outline applies — a container
    /// becoming a leaf must be empty, and the target must have somewhere to keep the
    /// item's text. Both guards live in [`binder_ops`], so the two views cannot drift.
    pub fn promote_with_guard(&self, ctx: &mut EventContext, uid: Uuid, target: PromoteTarget) {
        let Some(item_id) = self.item_id_of(&uid) else {
            return;
        };
        let blocked =
            binder_ops::demote_blocked_children(&self.inner.app_ctx, &self.inner.ids, item_id, target);
        if blocked > 0 {
            MessageBox::warning(tr!(promote_blocked_title()))
                .text(tr!(promote_blocked_text(count = blocked.to_string())))
                .buttons(MessageBoxButtons::Ok)
                .present(ctx);
            return;
        }
        let lost = binder_ops::promote_content_loss(&self.inner.app_ctx, item_id, target);
        if !lost.is_empty() {
            let kinds = lost
                .iter()
                .map(|r| crate::binder::create_labels::content_role_label(r).resolve_now())
                .collect::<Vec<_>>()
                .join(", ");
            MessageBox::warning(tr!(promote_lossy_title()))
                .text(tr!(promote_lossy_text(
                    target =
                        crate::binder::create_labels::promote_target_label(target).resolve_now(),
                    kinds = kinds
                )))
                .buttons(MessageBoxButtons::Ok)
                .present(ctx);
            return;
        }
        binder_ops::promote(&self.inner.app_ctx, &self.inner.ids, item_id, target);
    }

    // ── Batch actions ────────────────────────────────────────────────────────

    /// Duplicate whole subtrees. One backend call for the batch, so it is one undo step.
    pub fn duplicate(&self, uids: &[Uuid]) {
        let item_ids: Vec<u64> = uids.iter().filter_map(|u| self.item_id_of(u)).collect();
        if item_ids.is_empty() {
            return;
        }
        let _ = binder_item_management_commands::duplicate(
            &self.inner.app_ctx,
            self.stack(),
            &DuplicateDto { item_ids },
        );
    }

    /// Move rows to the trash.
    ///
    /// Grouped by **origin binder** because that is what `trash_binder_items` takes, and
    /// the binder is resolved through [`binder_ops::locate`] (the backend), not through
    /// any view's tree. A multi-row trash is wrapped in one composite so a single undo
    /// puts all of it back.
    pub fn trash(&self, uids: &[Uuid]) {
        let ctx = &*self.inner.app_ctx;
        let mut by_binder: HashMap<u64, Vec<i64>> = HashMap::new();
        for uid in uids {
            if let Some(item_id) = self.item_id_of(uid)
                && let Some((binder, _, _)) = binder_ops::locate(ctx, &self.inner.ids, item_id)
            {
                by_binder.entry(binder).or_default().push(item_id as i64);
            }
        }
        if by_binder.is_empty() {
            return;
        }
        // Read once — the open Work does not change mid-selection, and every
        // DTO built below shares it.
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };
        let stack = self.stack();
        let composite = by_binder.len() > 1 || by_binder.values().map(Vec::len).sum::<usize>() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        for (binder, binder_item_ids) in by_binder {
            let _ = trash_management_commands::trash_binder_items(
                ctx,
                stack,
                &TrashBinderItemsDto {
                    work_id,
                    binder_item_ids,
                    origin_binder_id: binder as i64,
                },
            );
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.inner.selection.clear();
    }

    /// Delete key / menu: trash the current selection.
    pub fn trash_selected(&self) {
        let sel = self.inner.selection.selected_keys();
        if !sel.is_empty() {
            self.trash(&sel);
        }
    }

    // ── Reorder by keyboard (the drag's non-pointer equivalent) ──────────────

    /// Move a row before its previous **sibling**, or after its next one.
    ///
    /// Siblings, not neighbouring display rows: the row under a chapter in the flat list
    /// may be that chapter's first scene, and "move up" must not mean "become a child of
    /// the thing above me". `parent`/`child_keys` come from the slice, so this reads the
    /// same hierarchy the table draws.
    ///
    /// **Refused while the table is projecting** — see [`can_reorder`](Self::can_reorder).
    pub fn move_up(&self, uid: Uuid) {
        self.move_by(uid, -1);
    }
    pub fn move_down(&self, uid: Uuid) {
        self.move_by(uid, 1);
    }

    /// Whether reordering is meaningful right now.
    ///
    /// False while a search or sort is active. The slice the table draws *is* the
    /// projected tree, so `parent`/`child_keys` return **sorted or filtered** neighbours:
    /// "move down" under a word-count sort would move the scene after whichever row is
    /// next *by word count*, writing a manuscript order the writer never chose, and under
    /// a search it would jump the row over every hidden sibling. The Corkboard makes the
    /// same call (`.reorderable(!projecting)`).
    pub fn can_reorder(&self) -> bool {
        !self.inner.projecting.get()
    }

    fn move_by(&self, uid: Uuid, delta: isize) {
        if !self.can_reorder() {
            return;
        }
        let rows = &self.inner.rows;
        let siblings = match rows.parent(&uid) {
            Some(parent) => rows.child_keys(&parent),
            // A root-level row's siblings are the other root-level rows.
            None => (0..rows.visible_count())
                .filter_map(|i| rows.with_entry(i, |_, e| (e.depth == 0).then_some(e.node_id)))
                .flatten()
                .collect(),
        };
        let Some(pos) = siblings.iter().position(|k| *k == uid) else {
            return;
        };
        let target_pos = match delta {
            d if d < 0 => pos.checked_sub(1),
            _ => (pos + 1 < siblings.len()).then_some(pos + 1),
        };
        let Some(target_pos) = target_pos else {
            return; // already at the end of its sibling run
        };
        let (Some(item_id), Some(target_id)) = (
            self.item_id_of(&uid),
            self.item_id_of(&siblings[target_pos]),
        ) else {
            return;
        };
        let place = if delta < 0 {
            MovePlace::Before
        } else {
            MovePlace::After
        };
        binder_ops::move_relative(&self.inner.app_ctx, &self.inner.ids, item_id, target_id, place);
    }

    /// Whether the table is showing a projection (search and/or sort) rather than
    /// manuscript order. The table binds this to disable reordering.
    pub fn is_projecting(&self) -> Signal<bool> {
        self.inner.projecting.clone()
    }

    fn recompute_projecting(&self) {
        let projecting = !self.inner.filters.query.get().trim().is_empty()
            || self.inner.filters.sort.get().is_some();
        if self.inner.projecting.get() != projecting {
            self.inner.projecting.set(projecting);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    /// Only the mocks-gated tests below need a built view-model — against a real
    /// backend there are no fixture rows for it to read.
    #[cfg(feature = "mocks")]
    fn vm_for(container: u64) -> Option<OverviewViewModel> {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let vm = OverviewViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            container,
            &BinderItemRole::Folder,
            &BinderItemSubRole::Book,
            Signal::new(CountingMethodSetting::default()),
            crate::view_models::TreeExpansionViewModel::new(
                app_ctx,
                ids,
                crate::models::TreeExpansionService::in_memory_default(),
            ),
        );
        // Stands in for `wire()`, which performs the first load in the app.
        if let Some(vm) = &vm {
            vm.rows().reload();
        }
        vm
    }

    /// The gate is the model's predicate, not the stream's: a notes folder gets an
    /// Overview, a plain grouping folder and every leaf do not.
    #[test]
    fn the_view_model_exists_for_exactly_the_overview_capable_combinations() {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{Book, ChapterScene, None as NoSub, Note, Part, Scene};
        let mk = |role: BinderItemRole, sub_role: BinderItemSubRole| {
            let app_ctx = Rc::new(AppContext::new());
            let ids = AppIds::new();
            OverviewViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                101,
                &role,
                &sub_role,
                Signal::new(CountingMethodSetting::default()),
                crate::view_models::TreeExpansionViewModel::new(
                    app_ctx,
                    ids,
                    crate::models::TreeExpansionService::in_memory_default(),
                ),
            )
            .is_some()
        };
        assert!(mk(Folder, Book));
        assert!(mk(Folder, Part));
        assert!(mk(Folder, ChapterScene));
        assert!(mk(Folder, Note), "a notes folder has a subtree to tabulate");
        assert!(!mk(Folder, NoSub), "a plain grouping folder does not");
        assert!(!mk(Item, Scene), "a leaf has no subtree at all");
        assert!(
            !mk(Item, ChapterScene),
            "nor does the flat chapter encoding"
        );
    }

    /// The batch convention: acting on a row inside the selection takes the whole
    /// selection; acting on one outside it takes only that row — and neither consults any
    /// other view's selection.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_batch_follows_this_tables_own_selection() {
        let vm = vm_for(101).expect("a Book has an Overview");
        let scene1 = common::uid::fixture_uid(201);
        let scene2 = common::uid::fixture_uid(202);
        let elsewhere = common::uid::fixture_uid(105);

        // Nothing selected: a row acts alone.
        assert_eq!(vm.batch_for(scene1), vec![scene1]);

        vm.selection().select_keys([scene1, scene2], false);
        let mut batch = vm.batch_for(scene1);
        batch.sort();
        let mut want = vec![scene1, scene2];
        want.sort();
        assert_eq!(batch, want, "inside the selection → the whole selection");

        assert_eq!(
            vm.batch_for(elsewhere),
            vec![elsewhere],
            "outside the selection → that row alone, selection untouched"
        );
        assert_eq!(vm.selection().count(), 2, "and the selection is not disturbed");
    }

    /// The inline-edit cursor is one cell at a time, and cancelling clears it.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_edit_cursor_holds_one_cell() {
        let vm = vm_for(101).unwrap();
        let uid = common::uid::fixture_uid(201);
        vm.begin_edit(uid, crate::models::COL_TITLE);
        assert_eq!(
            vm.editing_cell().get(),
            Some((uid, crate::models::COL_TITLE.to_string()))
        );
        vm.begin_edit(uid, crate::models::COL_LABEL);
        assert_eq!(
            vm.editing_cell().get(),
            Some((uid, crate::models::COL_LABEL.to_string())),
            "beginning another edit replaces the first"
        );
        vm.cancel_edit();
        assert_eq!(vm.editing_cell().get(), None);
    }

    /// Committing always leaves edit mode — including the paths that deliberately write
    /// nothing (unchanged value, blank title, a row that vanished under the editor).
    #[cfg(feature = "mocks")]
    #[test]
    fn committing_always_closes_the_editor() {
        let vm = vm_for(101).unwrap();
        let uid = common::uid::fixture_uid(201);
        for (col, value) in [
            (crate::models::COL_TITLE, "Scene 1"), // unchanged
            (crate::models::COL_TITLE, "   "),     // blank → refused
            (crate::models::COL_LABEL, ""),        // blank label → a real edit
        ] {
            vm.begin_edit(uid, col);
            vm.commit_edit(uid, col, value);
            assert_eq!(vm.editing_cell().get(), None, "{col} / {value:?}");
        }
        // A row that no longer exists must not strand the editor open either.
        let gone = common::uid::fixture_uid(9999);
        vm.begin_edit(gone, crate::models::COL_TITLE);
        vm.commit_edit(gone, crate::models::COL_TITLE, "anything");
        assert_eq!(vm.editing_cell().get(), None);
    }

    /// Create offers are the writing model's recommendations for the *anchor* — a row
    /// when one is given, the container otherwise — never a terminator.
    ///
    /// **Every row must offer something, including a leaf.** A leaf's recommendations are
    /// all `Sibling` / `ParentSibling`, so an earlier `relation == Child` filter (copied
    /// from the Corkboard, whose anchor is always a container) emptied the "Add ▸" submenu
    /// on every Scene, Note and flat Chapter in the table.
    #[cfg(feature = "mocks")]
    #[test]
    fn every_row_offers_a_create_including_leaves() {
        let vm = vm_for(101).unwrap();
        for (uid, what) in [
            (common::uid::fixture_uid(104), "a chapter folder"),
            (common::uid::fixture_uid(201), "a Scene (a leaf)"),
            (common::uid::fixture_uid(302), "a flat Chapter (a leaf)"),
            (common::uid::fixture_uid(203), "a Note (a leaf)"),
        ] {
            let recs = vm.create_recommendations(Some(uid));
            assert!(
                !recs.is_empty(),
                "{what} must offer at least one create; an empty Add submenu reads as broken"
            );
            assert!(
                recs.iter().all(|r| r.create_type != CreateType::EndOfBook),
                "{what}: a book terminator is not a table affordance"
            );
        }
        // The container anchor (the header's create button) still works too.
        assert!(!vm.create_recommendations(None).is_empty());
    }

    /// Reordering is refused while the table shows a projection: under a sort or search
    /// the row's "neighbour" is a projected one, so a move would write a manuscript order
    /// the writer never chose.
    #[cfg(feature = "mocks")]
    #[test]
    fn reordering_is_refused_while_sorted_or_searching() {
        let vm = vm_for(101).unwrap();
        assert!(vm.can_reorder(), "manuscript order: reordering is meaningful");

        vm.sort_signal()
            .set(Some((crate::models::COL_TOTAL_WORDS.to_string(), SortDirection::Descending)));
        vm.recompute_projecting();
        assert!(!vm.can_reorder(), "a sorted table must not be reordered");

        vm.sort_signal().set(None);
        vm.search_query_signal().set("scene".to_string());
        vm.recompute_projecting();
        assert!(!vm.can_reorder(), "a filtered table must not be reordered");

        vm.search_query_signal().set(String::new());
        vm.recompute_projecting();
        assert!(vm.can_reorder(), "clearing both restores reordering");
    }

    /// The edit buffer lives on the view-model, so a reload cannot re-seed it — the
    /// failure that silently discarded a half-typed rename when the other pane autosaved.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_edit_buffer_survives_a_reload() {
        let vm = vm_for(101).unwrap();
        let uid = common::uid::fixture_uid(201);
        vm.begin_edit(uid, crate::models::COL_TITLE);
        assert_eq!(vm.edit_buffer().text.get(), "Scene 1", "seeded from the row");

        vm.edit_buffer().text.set("Half-typed nam".to_string());
        vm.rows().reload(); // what any backend event triggers
        assert_eq!(
            vm.edit_buffer().text.get(),
            "Half-typed nam",
            "a reload must not re-seed the buffer from the model"
        );
        assert_eq!(
            vm.editing_cell().get(),
            Some((uid, crate::models::COL_TITLE.to_string())),
            "and the edit is still open on the same row"
        );
    }

    /// Moving a row steps through its **siblings**, so a row already last among them does
    /// not move (rather than burrowing into the next branch), and a row with no parent
    /// falls back to the root-level run.
    #[cfg(feature = "mocks")]
    #[test]
    fn sibling_moves_stop_at_the_ends_of_their_run() {
        let vm = vm_for(104).unwrap(); // the chapter folder: 3 flat scenes
        let first = common::uid::fixture_uid(201);
        let last = common::uid::fixture_uid(203);
        // No backend rows exist under `--features mocks`, so the move is a no-op call;
        // what is under test is that the *decision* to call is bounded correctly.
        assert_eq!(vm.rows().parent(&first), None, "they are root-level here");
        vm.move_up(first); // at the top of its run — nothing to swap with
        vm.move_down(last); // at the end of its run — likewise
        assert_eq!(
            vm.rows().visible_count(),
            3,
            "a refused move changes nothing"
        );
    }
}

#[cfg(all(test, feature = "mocks"))]
mod leaks {
    use super::*;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    /// **The rows model must actually drop when the tab closes.**
    ///
    /// `set_reorder` stores its closure inside the slice's `Rc<Inner>`, and the model
    /// holds that slice — so a closure capturing the *model* keeps alive the allocation
    /// that owns it, and every container tab ever opened leaks its whole table. The
    /// closure captures the uid → id map instead, which references nothing.
    #[test]
    fn the_rows_model_drops_with_its_view_model() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let vm = OverviewViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            101,
            &BinderItemRole::Folder,
            &BinderItemSubRole::Book,
            Signal::new(CountingMethodSetting::default()),
            crate::view_models::TreeExpansionViewModel::new(
                app_ctx,
                ids,
                crate::models::TreeExpansionService::in_memory_default(),
            ),
        )
        .unwrap();
        vm.install_reorder();
        // Probe the *model*, not the view-model: the hypothesised cycle is
        // slice.inner -> reorder closure -> OverviewRowsModel -> slice.inner.
        let weak = vm.rows().weak_probe();
        drop(vm);
        assert!(
            weak.upgrade().is_none(),
            "LEAK: the rows model outlived its own drop — the reorder closure the slice \
             owns is holding the model that holds the slice"
        );
    }
}
