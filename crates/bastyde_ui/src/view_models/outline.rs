// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `OutlineViewModel` — the binder/outline dock: its tree, its selection, and its
//! visibility. Visibility is *delegated* to the `DockingModel` (which is itself a
//! cloneable model handle); the view-model adds no visibility state of its own —
//! it only binds the dock id + side and speaks Skribisto vocabulary.
//!
//! Single-instance live state: owns the `DockingModel` and the tree model.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use bastyde::data::{DropPosition, KeyedSelectionModel, SelectionMode};
use bastyde::prelude::*; // EventContext, Signal, tr!
use bastyde::widgets::{DockOpenLocation, DockSide, DockWidgetId, DockingModel, InputDialog};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands,
    trash_management_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, UpdateBinderDto};

use frontend::binder_item_management::{DuplicateDto, MoveDto, MovePlace};
use frontend::trash_management::{TrashBinderDto, TrashBinderItemsDto};

use skribisto_model::{PromoteTarget, Recommendation, Relation, SubRoleExt};

use crate::app_ids::AppIds;
use crate::binder::placement;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, CommitMove, TreeFilters};
use crate::singles::{SingleBinder, SingleBinderItem};

use super::binder_ops::{self, update_item_dto};

#[derive(Clone)]
pub struct OutlineViewModel {
    app_ctx: Rc<AppContext>,
    model: BinderBinderItemsTreeModel,
    selection: KeyedSelectionModel<BinderTreeKey>,
    docking: DockingModel,
    dock_id: DockWidgetId,
    /// The app's id-only global state (root/work/work-info/undo-stack ids).
    /// Binder-tree mutations run on `ids.stack_id` so one Ctrl+Z history reverses
    /// them all; shared by clone with `EditorsViewModel` and the singles.
    ids: AppIds,
    /// Reactive read handles (Layer A) re-pointed to fetch the current state of
    /// the item/binder a mutation is about to update — replacing ad-hoc `get_*`.
    item_probe: SingleBinderItem,
    binder_probe: SingleBinder,
    /// The binder-switcher + search filter signals, shared (by clone) with the
    /// tree model, which observes them and re-sources itself on change.
    filters: TreeFilters,
}

// The visibility / reveal / action methods are the feature's public API
// (callable from menus, shortcuts, other view-models); wired incrementally, so
// not every one has a caller yet.
#[allow(dead_code)]
impl OutlineViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        model: BinderBinderItemsTreeModel,
        docking: DockingModel,
        dock_id: DockWidgetId,
        filters: TreeFilters,
    ) -> Self {
        let vm = Self {
            item_probe: SingleBinderItem::new(app_ctx.clone()),
            binder_probe: SingleBinder::new(app_ctx.clone()),
            app_ctx,
            model,
            // Multi: Ctrl/Shift-click, Shift+arrow and Ctrl+A extend the set;
            // the batch actions (trash/duplicate/indent) already act on the
            // whole selection and `reload`'s `prune_missing` handles a multi-set.
            selection: KeyedSelectionModel::new(SelectionMode::Multi),
            docking,
            dock_id,
            ids,
            filters,
        };
        vm.install_reorder();
        vm
    }

    /// The current `BinderItem` DTO, read through the reactive single (Layer A) —
    /// the source of truth when a tree mutation builds its update.
    fn item_dto(&self, id: u64) -> Option<frontend::direct_access::BinderItemDto> {
        self.item_probe.set_id(Some(id));
        self.item_probe.dto()
    }

    /// The current `Binder` DTO, read through the reactive single (Layer A).
    fn binder_dto(&self, id: u64) -> Option<frontend::direct_access::BinderDto> {
        self.binder_probe.set_id(Some(id));
        self.binder_probe.dto()
    }

    /// Build the outline view-model with its standard leading-dock presentation
    /// (280 px side + 48 px activity rail). The single place that knows the
    /// outline's dock geometry — so `App` and the title-bar menu share one
    /// handle without either re-stating layout constants.
    pub fn new_default(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        // The switcher/search filter signals — the single source of truth, shared
        // (by clone) into the tree model and read back by the header widgets.
        let filters = TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            match_counts: Signal::new((0, 0)),
            all_binders: Signal::new(false),
        };
        let model =
            BinderBinderItemsTreeModel::new(app_ctx.clone(), ids.work_id.clone(), filters.clone());
        let docking = DockingModel::new();
        docking.set_side_size(DockSide::Leading, 280.0);
        // A non-zero rail thickness switches the leading side to Rail
        // presentation (a `DockActivityBar` icon rail); the layout sizes the
        // rail from the `DockRail` config.
        docking.set_side_rail(DockSide::Leading, 48.0);
        // A *stable* id (not `fresh()`) so the per-work dock-layout restore can
        // match this dock across launches — see `crate::docks` module docs.
        Self::new(
            app_ctx,
            ids,
            model,
            docking,
            DockWidgetId::from_raw(crate::docks::OUTLINE_DOCK_ID),
            filters,
        )
    }

    /// Inject the model's drag-reorder closure. **Cycle-safety:** it captures
    /// only `app_ctx` + the `stack_id` signal and calls the free `apply_move`
    /// helper — NOT `self` (capturing the vm would form model → closure → vm →
    /// model, the `Rc` cycle the designer deliberately avoids).
    fn install_reorder(&self) {
        let app_ctx = self.app_ctx.clone();
        let stack_id = self.ids.stack_id.clone();
        // Capture the uid → id **map**, never the model: the slice owns this closure, so
        // capturing the model would capture the slice that owns it and leak the tree.
        let ids_by_uid = self.model.ids_by_uid();
        let commit: CommitMove = Rc::new(move |dragged, target, place| {
            let map = ids_by_uid.borrow();
            let (Some(&item_id), Some(&target_id)) = (map.get(&dragged), map.get(&target)) else {
                return false; // a row that left the tree between drag-start and drop
            };
            let target_is_binder = matches!(target, BinderTreeKey::Binder(_));
            drop(map);
            apply_move(
                &app_ctx,
                stack_id.get(),
                item_id,
                target_id,
                target_is_binder,
                place,
            )
            .is_ok()
        });
        self.model.set_reorder(commit);
    }

    // ── handles for wiring the widgets / layout ──
    /// The `TreeDataSource` to hand to `TreeView::from_source_keyed`.
    pub fn model(&self) -> BinderBinderItemsTreeModel {
        self.model.clone()
    }
    /// The live store id behind an **item** key — `None` for a binder key, or for a row
    /// that has left the tree. Every command site goes through here, because a durable
    /// key names a row rather than an id and the row may be gone.
    pub fn item_id_of(&self, key: BinderTreeKey) -> Option<u64> {
        self.model.item_id_of(&key)
    }

    /// The key addressing a live item id — for callers that arrive holding an id (an
    /// intent payload, a freshly created row) rather than a key.
    pub fn key_for_item(&self, item_id: u64) -> Option<BinderTreeKey> {
        self.model.key_for_item(item_id)
    }

    /// The key addressing a live binder id.
    pub fn key_for_binder(&self, binder_id: u64) -> Option<BinderTreeKey> {
        self.model.key_for_binder(binder_id)
    }

    pub fn selection(&self) -> KeyedSelectionModel<BinderTreeKey> {
        self.selection.clone()
    }
    pub fn selection_signal(&self) -> Signal<HashSet<BinderTreeKey>> {
        self.selection.selection_signal()
    }
    pub fn docking(&self) -> DockingModel {
        self.docking.clone()
    }
    pub fn dock_id(&self) -> DockWidgetId {
        self.dock_id
    }

    // ── binder switcher + search filters (shared with the tree model) ──
    /// Binder display scope — `None` = all binders, `Some(id)` = that binder.
    /// Bind the switcher label to it; the tree model re-sources on change.
    pub fn binder_filter_signal(&self) -> Signal<Option<u64>> {
        self.filters.binder.clone()
    }
    /// The live text-search query — hand a clone to `SearchField::new`.
    pub fn search_query_signal(&self) -> Signal<String> {
        self.filters.query.clone()
    }
    /// Search scope toggle — `false` = current binder, `true` = all binders.
    pub fn search_all_signal(&self) -> Signal<bool> {
        self.filters.all_binders.clone()
    }
    /// Switch the displayed binder (the tree model observes this and re-sources).
    pub fn set_binder_filter(&self, binder: Option<u64>) {
        self.filters.binder.set(binder);
    }
    /// The open Work id — for models that key on it (e.g. the binder list).
    pub fn work_id_signal(&self) -> Signal<Option<u64>> {
        self.ids.work_id.clone()
    }
    /// Clear the text search (e.g. on project load).
    pub fn clear_search(&self) {
        self.filters.query.set(String::new());
    }

    /// `(matched, total)` item rows under the active filter — see
    /// [`TreeFilters::match_counts`]. Bind it to show what the filter is doing.
    pub fn match_counts_signal(&self) -> Signal<(usize, usize)> {
        self.filters.match_counts.clone()
    }

    /// Whether a text filter is currently narrowing the tree.
    ///
    /// The one thing the dock cannot see for itself: the search field lives in a popover,
    /// so once dismissed there is nothing on screen to distinguish "filtered to nothing"
    /// from "this project is empty".
    pub fn is_filtering(&self) -> bool {
        !self.filters.query.get().trim().is_empty()
    }

    /// Resolve a tree key to its `(item_id, title)` (binder rows: `item_id` None).
    pub fn node_item(&self, key: BinderTreeKey) -> Option<(Option<u64>, String)> {
        self.model.node_of(&key)
    }

    /// The first selected **item** row's `(item_id, title)`, if the selection is
    /// an item (not a binder root). Backs the outline's "Open to the Side"
    /// keyboard shortcut.
    pub fn selected_item(&self) -> Option<(u64, String)> {
        let key = self.selection.selected_keys().first().copied()?;
        match self.node_item(key)? {
            (Some(item_id), title) => Some((item_id, title)),
            _ => None,
        }
    }

    // ── visibility: pure delegation to the DockingModel ──
    //
    // The outline lives on a *rail* side, so "visible" means the side panel is
    // shown (the rail itself persists) — i.e. side visibility, the same concept
    // the activity-rail's click-to-hide drives. We toggle/reflect side
    // visibility, NOT dock open/close (which would add/remove the rail icon).

    /// Show the outline panel.
    pub fn show(&self) {
        self.docking.set_side_visible(DockSide::Leading, true);
    }
    /// Hide the outline panel (the rail stays).
    pub fn hide(&self) {
        self.docking.set_side_visible(DockSide::Leading, false);
    }
    /// Flip the outline panel's visibility.
    pub fn toggle(&self) {
        let visible = self.docking.is_side_visible(DockSide::Leading);
        self.docking.set_side_visible(DockSide::Leading, !visible);
    }
    /// Reactive shown/hidden state — bind a menu check or toolbar toggle to this.
    /// Reflects the side's visibility (also flips when the rail hides the side).
    pub fn is_visible(&self) -> Signal<bool> {
        self.docking.side_visible_signal(DockSide::Leading)
    }

    /// Register the outline dock on its side (called once, at layout build).
    pub fn open_in_layout(&self) {
        self.docking
            .open_dock(self.dock_id, DockOpenLocation::side(DockSide::Leading));
    }

    /// Rebuild the tree from the backend (e.g. on project load), pruning a
    /// now-stale selection (a trashed item drops out of the tree).
    pub fn reload(&self) {
        self.model.reload();
        let model = self.model.clone();
        self.selection.prune_missing(move |k| model.contains(k));
    }

    /// Bring the outline forward AND select a row ("reveal in outline").
    ///
    /// Expands the row's ancestors first: selecting a row inside a collapsed
    /// parent moves an invisible cursor, which reads to the writer as nothing
    /// having happened at all.
    pub fn reveal_item(&self, key: BinderTreeKey) {
        self.show();
        self.select_in_place(key);
    }

    /// Expand a row's ancestors and select it, **without** bringing the outline
    /// forward — the half of [`reveal_item`] that a create wants.
    ///
    /// Creation is not always an outline gesture: the same `AppIntent::NewItem`
    /// path serves the Corkboard and Overview header buttons, and popping the
    /// outline dock open because the writer added a scene from the corkboard is
    /// an interruption they did not ask for. Expanding and selecting is enough —
    /// if the outline is on screen, the new row is visible in it; if it is not,
    /// nothing forces it into view.
    pub fn select_in_place(&self, key: BinderTreeKey) {
        self.model.expand_ancestors(&key);
        self.selection.select(key);
    }

    // ── undo stack (one per loaded Work) ──

    /// Create the per-`Work` undo stack. Call on `LoadWork`.
    pub fn init_stack(&self) {
        self.ids.open_stack(&self.app_ctx);
    }
    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }
    /// The shared undo-stack signal, handed to `EditorsViewModel` so editor
    /// write-back lands on the same Ctrl+Z history as tree edits.
    pub fn stack_id_signal(&self) -> Signal<Option<u64>> {
        self.ids.stack_id.clone()
    }
    /// The app's id-only global state (shared by clone), handed to
    /// `EditorsViewModel` so its tabs (esp. the manuscript streams) can reach
    /// `work_id`/`stack_id`.
    pub fn ids(&self) -> AppIds {
        self.ids.clone()
    }

    /// Subscribe the tree to backend structural events so it re-sources on any
    /// mutation, whoever caused it (a manuscript stream, import, etc.) — not
    /// just the outline's own commands. Call once from `App::build`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.model.wire(ctx);
    }

    // ── actions (each: backend command on the undo stack, then reload) ──

    /// The shared create tail: build the DTO and run the undoable create command,
    /// then reload and reveal what was just made.
    ///
    /// `title` is passed in already resolved rather than derived from `role` here:
    /// the caller knows the logical `CreateType` (Chapter, Scene, Note…), which
    /// `role` alone cannot recover — `Folder` covers books, parts, chapters and note
    /// folders alike. See [`create_labels::default_title`].
    ///
    /// Revealing is not cosmetic. Creating the *first* child of a container puts the
    /// new row under a parent that, having had no children, has never been expanded —
    /// so without this the write succeeds and the writer sees nothing happen.
    fn create_item_at(
        &self,
        binder: u64,
        index: usize,
        indent: i64,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
        title: String,
    ) {
        let dto = CreateBinderItemDto {
            title,
            role,
            sub_role,
            activated: true,
            is_exportable: true,
            indent,
            ..Default::default()
        };
        if let Ok(created) = binder_item_commands::create_binder_item(
            &self.app_ctx,
            self.stack(),
            &dto,
            binder,
            index as i32,
        ) {
            // Reload first: the row must exist in the tree before its ancestors can
            // be walked, and `expand_ancestors` resolves the chain through the model.
            //
            // `select_in_place`, not `reveal_item` — a create fired from the
            // Corkboard or Overview must not yank the outline dock open.
            self.reload();
            self.select_in_place(BinderTreeKey::Item(created.uid));
        }
    }

    // ── context-dependent "create" recommendations ──

    /// The ordered `Recommendation`s for the current selection's first key — what
    /// the header "Create" SplitButton offers (default first). Reads live data,
    /// so it must be recomputed whenever `selection_signal()` changes.
    pub fn recommendations_for_selection(&self) -> Vec<Recommendation> {
        self.recommendations_for_key(self.selection.selected_keys().first().copied())
    }

    /// The ordered `Recommendation`s for a specific anchor row (the per-item
    /// "Add ▸" submenu). A `Binder`/absent/stale anchor falls back to the
    /// top-level set (Book first). Applies the live BookEnd gating.
    pub fn recommendations_for_key(&self, anchor: Option<BinderTreeKey>) -> Vec<Recommendation> {
        match anchor {
            Some(key @ BinderTreeKey::Item(_)) => {
                match self
                    .model
                    .item_id_of(&key)
                    .and_then(|i| Some((i, self.item_dto(i)?)))
                {
                    Some((i, dto)) => {
                        let mut recs = skribisto_model::recommendations(&dto.role, &dto.sub_role);
                        self.gate_book_end(i, &mut recs);
                        recs
                    }
                    None => skribisto_model::recommendations_root(),
                }
            }
            _ => skribisto_model::recommendations_root(),
        }
    }

    /// Create the recommended item, relation-aware. `anchor = None` re-reads the
    /// current selection (header button); `Some(key)` anchors explicitly (context
    /// menu). The logical `CreateType` is resolved to a concrete `(role, sub_role)`
    /// via the project's chapter mode. Guarded by `validate_item`.
    pub fn add_recommended(&self, anchor: Option<BinderTreeKey>, rec: &Recommendation) {
        let anchor = anchor.or_else(|| self.selection.selected_keys().first().copied());
        let (role, sub_role) = rec.create_type.combo(self.chapter_mode());
        if skribisto_model::validate_item(&role, &sub_role, &[]).is_err() {
            return;
        }
        let Some((binder, index, indent)) = self.insertion_point_for(anchor, rec.relation) else {
            return;
        };
        // Resolve the localized default to owned data here — this call is the
        // chrome/data boundary (see `create_labels::default_title`).
        let title: String = crate::binder::create_labels::default_title(rec.create_type).into();
        self.create_item_at(binder, index, indent, role, sub_role, title);
    }

    // ── promote / demote (convert a binder item to its paired type) ──

    /// The paired promote target for a row, if any — `(role, sub_role)` of the
    /// type this item would become (see `skribisto_model::promote_target`).
    /// The backend context — for tests that assert on entities directly.
    #[cfg(test)]
    pub(crate) fn app_ctx(&self) -> &AppContext {
        &self.app_ctx
    }

    /// Every type `key` may be converted to, in menu order. Empty when it has none.
    pub fn promote_targets_of(&self, key: BinderTreeKey) -> Vec<PromoteTarget> {
        let Some(item_id) = self.model.item_id_of(&key) else {
            return Vec::new();
        };
        binder_ops::promote_targets_of(&self.app_ctx, item_id)
    }

    /// The content roles whose text `key` would **lose** by becoming `target` (empty
    /// rows never count). Non-empty means the conversion is refused: a chapter holding
    /// prose cannot become a Part, which has nowhere to put it.
    pub fn promote_content_loss(
        &self,
        key: BinderTreeKey,
        target: PromoteTarget,
    ) -> Vec<ContentRole> {
        let Some(item_id) = self.model.item_id_of(&key) else {
            return Vec::new();
        };
        binder_ops::promote_content_loss(&self.app_ctx, item_id, target)
    }

    /// The number of child items that block converting `key` into `target` — non-zero
    /// only when a container would become a leaf (a chapter folder → a flat chapter)
    /// while it still holds items. The caller shows a "move or trash them first" prompt.
    /// Located through the backend (`binder_ops`), **not** through this outline's tree.
    /// The tree only holds what the current binder scope and search leave visible, and an
    /// item it cannot see used to answer `0` — which reads as "nothing blocks this" and
    /// waved through exactly the conversion the guard exists to stop.
    pub fn demote_blocked_children(&self, key: BinderTreeKey, target: PromoteTarget) -> usize {
        let Some(item_id) = self.model.item_id_of(&key) else {
            return 0;
        };
        binder_ops::demote_blocked_children(&self.app_ctx, &self.ids, item_id, target)
    }

    /// Convert a binder item to `target` (undoable). The use case re-validates the
    /// target against the item's current type and refuses a conversion that would drop
    /// text, so this is safe to call even from a stale menu. The demote-empty guard is
    /// the caller's job (`demote_blocked_children`); this trusts it.
    pub fn promote(&self, key: BinderTreeKey, target: PromoteTarget) {
        let Some(item_id) = self.model.item_id_of(&key) else {
            return;
        };
        if binder_ops::promote(&self.app_ctx, &self.ids, item_id, target) {
            self.reload();
        }
    }

    /// The open project's chapter storage mode — how a `CreateType::Chapter` is
    /// encoded — read from the open Work's `chapter_mode` field (defaults to
    /// folder mode when no Work is open).
    fn chapter_mode(&self) -> skribisto_model::ChapterMode {
        binder_ops::chapter_mode(&self.app_ctx, &self.ids)
    }

    /// Begin a rename: present a modal `InputDialog`, applying `rename` on OK.
    pub fn begin_rename(&self, key: BinderTreeKey, ctx: &mut EventContext) {
        let current = self.model.node_of(&key).map(|(_, t)| t).unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |result, _ctx| {
                if let Some(name) = result
                    && !name.trim().is_empty()
                {
                    vm.rename(key, &name);
                }
            })
            .present(ctx);
    }

    /// Apply a rename to a binder (name) or item (title).
    pub fn rename(&self, key: BinderTreeKey, title: &str) {
        let ctx = &*self.app_ctx;
        match key {
            BinderTreeKey::Binder(_) => {
                let Some(b) = self.model.binder_of(&key) else {
                    return;
                };
                if let Some(binder) = self.binder_dto(b) {
                    let dto = UpdateBinderDto {
                        id: b,
                        created_at: binder.created_at,
                        updated_at: binder.updated_at,
                        // Durable identity: carried, never re-minted.
                        uid: binder.uid.clone(),
                        name: title.to_string(),
                        activated: binder.activated,
                    };
                    let _ = binder_commands::update_binder(ctx, self.stack(), &dto);
                }
            }
            BinderTreeKey::Item(_) => {
                let Some(i) = self.model.item_id_of(&key) else {
                    return;
                };
                // Through the single, so the title `Content` row follows the entity field.
                // A tree rename that wrote only `BinderItem.title` would leave the
                // manuscript compiling the old chapter title — the mirror image of the
                // editor-rename bug.
                let item = SingleBinderItem::new(self.app_ctx.clone());
                item.set_id(Some(i));
                let _ = item.set_title(title, self.stack());
            }
        }
        self.reload();
    }

    /// Rename the first selected row (the `binder.rename` command entry point).
    pub fn rename_selected(&self, ctx: &mut EventContext) {
        if let Some(key) = self.selection.selected_keys().first().copied() {
            self.begin_rename(key, ctx);
        }
    }

    /// Move the selected items to trash (the `binder.trash_selected` command).
    pub fn trash_selected(&self) {
        let sel = self.selection.selected_keys();
        self.trash_keys(&sel);
    }

    /// Move the given keys to trash (binders and items both supported). Used by
    /// the context menu (operates on the right-clicked row, not the selection).
    pub fn trash_keys(&self, sel: &[BinderTreeKey]) {
        let ctx = &*self.app_ctx;
        if sel.is_empty() {
            return;
        }
        // Read once, outside the per-key loops below — the open Work does
        // not change mid-selection, and every DTO built here shares it.
        let Some(work_id) = self.ids.work_id.get() else {
            return; // no project open
        };
        let stack = self.stack();
        let composite = sel.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        // Whole binders.
        for key in sel {
            if matches!(key, BinderTreeKey::Binder(_))
                && let Some(b) = self.model.binder_of(key)
            {
                let _ = trash_management_commands::trash_binder(
                    ctx,
                    stack,
                    &TrashBinderDto {
                        work_id,
                        binder_id: b as i64,
                    },
                );
            }
        }
        // Items, grouped by their origin binder.
        let mut by_binder: HashMap<u64, Vec<i64>> = HashMap::new();
        for key in sel {
            if let Some(i) = self.model.item_id_of(key)
                && let Some(b) = self.model.binder_of(key)
            {
                by_binder.entry(b).or_default().push(i as i64);
            }
        }
        for (binder, ids) in by_binder {
            let _ = trash_management_commands::trash_binder_items(
                ctx,
                stack,
                &TrashBinderItemsDto {
                    work_id,
                    binder_item_ids: ids,
                    origin_binder_id: binder as i64,
                },
            );
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.reload();
    }

    /// The item ids strictly *below* `item_id` in the binder tree (its subtree, excluding
    /// itself), in document order. Empty for a leaf — the Inspector uses that to decide
    /// whether to offer "Apply to children".
    pub fn subtree_descendants(&self, item_id: u64) -> Vec<u64> {
        let Some((binder, _, _)) = binder_ops::locate(&self.app_ctx, &self.ids, item_id) else {
            return Vec::new();
        };
        let (order, meta) = self.ordered_meta(binder);
        let Some(pos) = order.iter().position(|&x| x == item_id) else {
            return Vec::new();
        };
        let base = meta.get(&item_id).map(|(_role, i, _sr)| *i).unwrap_or(0);
        let end = placement::subtree_end(&order, &meta, pos, base);
        order[pos + 1..end].to_vec()
    }

    /// Set every descendant's `is_exportable` to `value` in **one** undo step (the outline's
    /// composite pattern). The item itself is not touched — the Inspector's own toggle owns
    /// that; this is the "apply to children" affordance beside it.
    pub fn apply_exportable_to_subtree(&self, item_id: u64, value: bool) {
        let descendants = self.subtree_descendants(item_id);
        if descendants.is_empty() {
            return;
        }
        let ctx = &*self.app_ctx;
        let stack = self.stack();
        let _ = undo_redo_commands::begin_composite(ctx, stack);
        for id in descendants {
            // A probe fixed to each descendant, reusing the tested full-DTO write.
            let probe = SingleBinderItem::new(self.app_ctx.clone());
            probe.set_id(Some(id));
            let _ = probe.set_exportable(value, stack);
        }
        undo_redo_commands::end_composite(ctx);
        self.reload();
    }

    /// Write `tags` onto every descendant's `dict_language` in **one** undo step — the
    /// explicit half of the language model.
    ///
    /// Nothing propagates a language implicitly (`skribisto_model::language::tags_in_binder`
    /// resolves an item's own tag, else the Work's — there is no container scope). This is
    /// how a writer says "this whole chapter is in Turkish" and means it: each descendant
    /// ends up carrying a real tag, so what the Inspector shows on an item is what that item
    /// is actually checked against.
    ///
    /// The item itself is not touched — the pill field beside this button owns that — and an
    /// **empty** `tags` is a legitimate value to push: it clears the descendants back to
    /// inheriting the Work's language, which is the only way to undo an over-broad apply
    /// without visiting each child.
    pub fn apply_dict_language_to_subtree(&self, item_id: u64, tags: &[String]) {
        let descendants = self.subtree_descendants(item_id);
        if descendants.is_empty() {
            return;
        }
        let ctx = &*self.app_ctx;
        let stack = self.stack();
        let _ = undo_redo_commands::begin_composite(ctx, stack);
        for id in descendants {
            // A probe fixed to each descendant, reusing the tested full-DTO write.
            let probe = SingleBinderItem::new(self.app_ctx.clone());
            probe.set_id(Some(id));
            let _ = probe.set_dict_language(tags, stack);
        }
        undo_redo_commands::end_composite(ctx);
        self.reload();
    }

    /// Create a new binder in the open Work, switch the switcher to it, and open
    /// the rename dialog so the user names it. Backs the popover's "New binder…".
    pub fn new_binder(&self, ctx: &mut EventContext) {
        let Some(work_id) = self.ids.work_id.get() else {
            return; // no project open
        };
        let dto = CreateBinderDto {
            name: "New Binder".to_string(),
            activated: true,
            ..Default::default()
        };
        // `-1` appends to the Work's ordered binder list; undoable on the stack.
        if let Ok(binder) =
            binder_commands::create_binder(&self.app_ctx, self.stack(), &dto, work_id, -1)
        {
            // Show the new binder (this re-sources the tree so its row exists),
            // then rename it in place — `begin_rename` reads the row's name.
            self.set_binder_filter(Some(binder.id));
            // The filter change re-sourced the tree, so the new binder now has a row —
            // and therefore a key.
            if let Some(key) = self.model.key_for_binder(binder.id) {
                self.begin_rename(key, ctx);
            }
        }
    }

    /// Trash a whole binder by id — the switcher context-menu action, reached via
    /// `AppIntent::TrashBinder` → the `binder.trash` global action. If it was the
    /// displayed binder, revert to "all binders" so the tree isn't left filtered
    /// to a now-trashed binder.
    pub fn trash_binder(&self, id: u64) {
        if let Some(key) = self.model.key_for_binder(id) {
            self.trash_keys(&[key]);
        }
        if self.filters.binder.get() == Some(id) {
            self.set_binder_filter(None);
        }
    }

    /// Duplicate the selected items (the `binder.duplicate` command).
    pub fn duplicate_selected(&self) {
        let sel = self.selection.selected_keys();
        self.duplicate_keys(&sel);
    }

    /// Duplicate the given keys' item subtrees (binder keys ignored). Used by the
    /// context menu (operates on the right-clicked row, not the selection).
    pub fn duplicate_keys(&self, keys: &[BinderTreeKey]) {
        // `item_id_of` returns `None` for a binder key and for a row that has left the
        // tree, so the filter covers both without a separate match.
        let item_ids: Vec<u64> = keys
            .iter()
            .filter_map(|k| self.model.item_id_of(k))
            .collect();
        if item_ids.is_empty() {
            return;
        }
        let _ = binder_item_management_commands::duplicate(
            &self.app_ctx,
            self.stack(),
            &DuplicateDto { item_ids },
        );
        self.reload();
    }

    /// Move one dragged item relative to a target (drag-drop and keyboard moves
    /// share the same `apply_move` helper).
    pub fn move_item(&self, dragged: BinderTreeKey, target: BinderTreeKey, place: DropPosition) {
        let Some(item_id) = self.model.item_id_of(&dragged) else {
            return; // only items move, and only ones still in the tree
        };
        let target_is_binder = matches!(target, BinderTreeKey::Binder(_));
        let target_id = if target_is_binder {
            self.model.binder_of(&target)
        } else {
            self.model.item_id_of(&target)
        };
        let Some(target_id) = target_id else {
            return;
        };
        if apply_move(
            &self.app_ctx,
            self.stack(),
            item_id,
            target_id,
            target_is_binder,
            place,
        )
        .is_ok()
        {
            self.reload();
        }
    }

    pub fn indent_selected(&self) {
        self.reindent(1);
    }
    pub fn outdent_selected(&self) {
        self.reindent(-1);
    }

    // ── private helpers ──

    /// `(binder, insert_index, indent)` for a new item, relative to `anchor`.
    fn insertion_point(&self, anchor: Option<BinderTreeKey>) -> Option<(u64, usize, i64)> {
        let ctx = &*self.app_ctx;
        match anchor {
            Some(key @ BinderTreeKey::Binder(_)) => Some((self.model.binder_of(&key)?, 0, 0)),
            Some(key @ BinderTreeKey::Item(_)) => {
                let i = self.model.item_id_of(&key)?;
                let binder = self.model.binder_of(&key)?;
                let order = binder_commands::get_binder_relationship(
                    ctx,
                    &binder,
                    &BinderRelationshipField::BinderItems,
                )
                .ok()?;
                let pos = order.iter().position(|&x| x == i)?;
                let it = self.item_dto(i)?;
                let indent = if it.role == BinderItemRole::Folder {
                    it.indent + 1 // first child of the folder
                } else {
                    it.indent // following sibling
                };
                Some((binder, pos + 1, indent))
            }
            None => Some((self.first_binder()?, 0, 0)),
        }
    }

    /// `(binder, insert_index, indent)` for a new item placed by `relation`
    /// relative to `anchor` — the relation-aware generalisation of
    /// [`insertion_point`](Self::insertion_point) (which it leaves untouched).
    ///
    /// - `Sibling` lands after the anchor's *entire subtree* at the anchor's own
    ///   indent (so a sibling of a populated folder follows its children).
    /// - `Child` appends inside a folder anchor (indent + 1), but before any direct
    ///   child that `closes_book()` (keeps a Book's `BookEnd` last).
    /// - `ParentSibling` walks up to the nearest ancestor that opens a chapter/book
    ///   and behaves as `Sibling` of it.
    fn insertion_point_for(
        &self,
        anchor: Option<BinderTreeKey>,
        relation: Relation,
    ) -> Option<(u64, usize, i64)> {
        match anchor {
            Some(key @ BinderTreeKey::Binder(_)) => {
                let b = self.model.binder_of(&key)?;
                match relation {
                    Relation::Child => Some((b, 0, 0)),
                    _ => Some((b, self.ordered_meta(b).0.len(), 0)),
                }
            }
            Some(key @ BinderTreeKey::Item(_)) => {
                let i = self.model.item_id_of(&key)?;
                let binder = self.model.binder_of(&key)?;
                let (order, meta) = self.ordered_meta(binder);
                let pos = order.iter().position(|&x| x == i)?;
                let (_, anchor_indent, _) = *meta.get(&i)?;
                // Shared with `StreamViewModel` — see `crate::binder::placement`.
                let (index, indent) = placement::insertion_point_for_item(
                    &order,
                    &meta,
                    pos,
                    anchor_indent,
                    relation,
                );
                Some((binder, index, indent))
            }
            None => {
                let b = self.first_binder()?;
                Some((b, self.ordered_meta(b).0.len(), 0))
            }
        }
    }

    /// The binder's ordered item ids plus `{id -> (role, indent, sub_role)}`, in one
    /// batch fetch — the data `insertion_point_for` / `gate_book_end` walk.
    fn ordered_meta(&self, binder: u64) -> (Vec<u64>, placement::ItemMeta) {
        let ctx = &*self.app_ctx;
        let order = binder_commands::get_binder_relationship(
            ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        let meta = binder_item_commands::get_binder_item_multi(ctx, &order)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|it| (it.id, (it.role, it.indent, it.sub_role)))
            .collect();
        (order, meta)
    }

    /// `(position, indent)` of the book enclosing `order[pos]` — the anchor itself
    /// if it opens a book, else the nearest book ancestor (walking *past* any
    /// intermediate chapters). `None` if the anchor is not inside a book.
    fn enclosing_book(
        order: &[u64],
        meta: &placement::ItemMeta,
        pos: usize,
    ) -> Option<(usize, i64)> {
        let (_role0, ind0, sr0) = meta.get(&order[pos])?;
        let mut cur_indent = *ind0;
        if sr0.opens_book() {
            return Some((pos, cur_indent));
        }
        let mut cur = pos;
        while cur > 0 {
            cur -= 1;
            let (_role, ind, sr) = meta.get(&order[cur])?;
            if *ind < cur_indent {
                cur_indent = *ind;
                if sr.opens_book() {
                    return Some((cur, *ind));
                }
            }
        }
        None
    }

    /// Drop any `closes_book()` recommendation (End of Book) when the anchor's
    /// enclosing book already contains one — a book has exactly one end. This is
    /// the live-data gating the pure `recommendations()` table can't do itself.
    fn gate_book_end(&self, anchor_item: u64, recs: &mut Vec<Recommendation>) {
        if !recs.iter().any(|r| r.create_type.closes_book()) {
            return;
        }
        let Some((binder, _, _)) = binder_ops::locate(&self.app_ctx, &self.ids, anchor_item) else {
            return;
        };
        let (order, meta) = self.ordered_meta(binder);
        let Some(pos) = order.iter().position(|&x| x == anchor_item) else {
            return;
        };
        let Some((book_pos, book_indent)) = Self::enclosing_book(&order, &meta, pos) else {
            return;
        };
        let end = placement::subtree_end(&order, &meta, book_pos, book_indent);
        let has_end = order[book_pos..end].iter().any(|id| {
            meta.get(id)
                .is_some_and(|(_role, _ind, sr)| sr.closes_book())
        });
        if has_end {
            recs.retain(|r| !r.create_type.closes_book());
        }
    }

    fn first_binder(&self) -> Option<u64> {
        let ctx = &*self.app_ctx;
        // ids-only global state: the open Work's id is known; no `get_all_work`.
        let work_id = self.ids.work_id.get()?;
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .ok()?
            .into_iter()
            .next()
    }

    fn reindent(&self, delta: i64) {
        let ctx = &*self.app_ctx;
        let items: Vec<u64> = self
            .selection
            .selected_keys()
            .iter()
            .filter_map(|k| self.model.item_id_of(k))
            .collect();
        if items.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = items.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        for i in items {
            let Some((binder, _, _)) = binder_ops::locate(ctx, &self.ids, i) else {
                continue;
            };
            let Ok(order) = binder_commands::get_binder_relationship(
                ctx,
                &binder,
                &BinderRelationshipField::BinderItems,
            ) else {
                continue;
            };
            let Some(pos) = order.iter().position(|&x| x == i) else {
                continue;
            };
            let Some(it) = self.item_dto(i) else {
                continue;
            };
            let new_indent = if delta > 0 {
                // Indent ≤ predecessor sibling's indent + 1 (can't indent the
                // first child of a level).
                let pred = if pos == 0 {
                    -1
                } else {
                    self.item_dto(order[pos - 1])
                        .map(|p| p.indent)
                        .unwrap_or(-1)
                };
                (it.indent + 1).min(pred + 1).max(0)
            } else {
                (it.indent - 1).max(0)
            };
            if new_indent != it.indent {
                let mut dto = update_item_dto(&it);
                dto.indent = new_indent;
                let _ = binder_item_commands::update_binder_item(ctx, stack, &dto);
            }
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.reload();
    }
}

/// Apply a single-item move through the backend (undoable). Free function so the
/// model's reorder closure can call it without capturing the view-model (which
/// would create an `Rc` cycle model → closure → vm → model).
pub(crate) fn apply_move(
    ctx: &AppContext,
    stack: Option<u64>,
    item_id: u64,
    target_id: u64,
    target_is_binder: bool,
    place: DropPosition,
) -> anyhow::Result<()> {
    let move_place = match place {
        DropPosition::Before => MovePlace::Before,
        DropPosition::After => MovePlace::After,
        DropPosition::Into => MovePlace::Into,
    };
    binder_item_management_commands::move_items(
        ctx,
        stack,
        &MoveDto {
            item_ids: vec![item_id],
            target_id: Some(target_id),
            target_is_binder,
            move_place,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "mocks")]
    use bastyde::data::TreeDataSource; // brings `visible_count` into scope

    #[test]
    fn outline_init_stack_creates_a_stack_id() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        assert!(outline.stack_id_signal().get().is_none());
        outline.init_stack();
        assert!(
            outline.stack_id_signal().get().is_some(),
            "init_stack opens the per-Work undo stack"
        );
    }

    #[test]
    fn outline_actions_on_empty_selection_are_noops() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        // No selection, no loaded Work — these must not panic and must do nothing.
        outline.trash_selected();
        outline.duplicate_selected();
        outline.indent_selected();
        outline.outdent_selected();
    }

    #[test]
    fn outline_show_hide_toggle_track_side_visibility() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let visible = outline.is_visible();

        // Sides start hidden.
        assert!(!visible.get());

        outline.show();
        assert!(visible.get(), "show() makes the side visible");

        outline.toggle();
        assert!(!visible.get(), "toggle() hides a visible side");

        outline.toggle();
        assert!(visible.get(), "toggle() re-shows a hidden side");

        outline.hide();
        assert!(!visible.get(), "hide() hides the side");
    }

    // The mock tree has content (2 binders, 13 items) so these assert the
    // switcher/search signals drive the model's re-source end-to-end.
    #[cfg(feature = "mocks")]
    #[test]
    fn set_binder_filter_scopes_the_tree() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let model = outline.model();
        assert_eq!(model.visible_count(), 15); // all binders
        outline.set_binder_filter(Some(1));
        assert_eq!(model.visible_count(), 12); // Manuscript = binder + 11 items
        outline.set_binder_filter(None);
        assert_eq!(model.visible_count(), 15);
    }

    #[cfg(feature = "mocks")]
    #[test]
    fn search_query_filters_the_tree() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let model = outline.model();
        outline.search_query_signal().set("dawn".to_string());
        assert_eq!(model.visible_count(), 3); // Manuscript > Book One > Scene at dawn
        outline.clear_search();
        assert_eq!(model.visible_count(), 15);
    }

    // ── relation-aware creation (real backend: seed a Work/Binder/items and
    // assert where `add_recommended` lands). Gated off `mocks`, whose tree model
    // ignores `work_id` and re-sources a static fixture instead. ──
    #[cfg(not(feature = "mocks"))]
    mod recommend {
        use frontend::commands::content_commands;
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

        use super::*;

        /// Seed an empty Work + Binder; return a VM wired to it (reloaded) and the
        /// binder id.
        pub(super) fn seed() -> (OutlineViewModel, u64) {
            let app_ctx = Rc::new(AppContext::new());
            let work = work_commands::create_orphan_work(
                &app_ctx,
                None,
                &frontend::direct_access::CreateWorkDto::default(),
            )
            .unwrap();
            let binder = binder_commands::create_binder(
                &app_ctx,
                None,
                &CreateBinderDto {
                    name: "B".into(),
                    activated: true,
                    ..Default::default()
                },
                work.id,
                0,
            )
            .unwrap();
            let ids = AppIds::default();
            ids.work_id.set(Some(work.id));
            let outline = OutlineViewModel::new_default(app_ctx, ids);
            outline.reload();
            (outline, binder.id)
        }

        /// Append an item to `binder` at `index` (sequential = append) and reload.
        /// The tree key for a seeded item.
        ///
        /// The seed writes straight to the backend, so the tree has to re-source before
        /// the row (and therefore its key) exists — production gets that re-source from
        /// the `BinderItem(Created)` event, which a headless test has no source for.
        /// The tree key for a seeded binder.
        pub(super) fn binder_key_of(outline: &OutlineViewModel, binder_id: u64) -> BinderTreeKey {
            outline.reload();
            outline
                .key_for_binder(binder_id)
                .expect("the seeded binder must have a row in the tree")
        }

        pub(super) fn key_of(outline: &OutlineViewModel, item_id: u64) -> BinderTreeKey {
            outline.reload();
            outline
                .key_for_item(item_id)
                .expect("the seeded item must have a row in the tree")
        }

        pub(super) fn seed_item(
            outline: &OutlineViewModel,
            binder: u64,
            role: BinderItemRole,
            sub_role: BinderItemSubRole,
            indent: i64,
            index: i32,
        ) -> u64 {
            let dto = CreateBinderItemDto {
                title: format!("{role:?}/{sub_role:?}"),
                role,
                sub_role,
                activated: true,
                is_exportable: true,
                indent,
                ..Default::default()
            };
            let id = binder_item_commands::create_binder_item(
                &outline.app_ctx,
                None,
                &dto,
                binder,
                index,
            )
            .unwrap()
            .id;
            outline.reload();
            id
        }

        fn order_of(outline: &OutlineViewModel, binder: u64) -> Vec<u64> {
            binder_commands::get_binder_relationship(
                &outline.app_ctx,
                &binder,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap()
        }

        use skribisto_model::CreateType;

        fn rec(create_type: CreateType, relation: Relation) -> Recommendation {
            Recommendation {
                create_type,
                relation,
            }
        }

        #[test]
        fn chapter_creates_a_folder_chapter_in_folder_mode() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            outline.add_recommended(
                Some(key_of(&outline, book)),
                &rec(CreateType::Chapter, Relation::Child),
            );
            let new = *order_of(&outline, binder).last().unwrap();
            let dto = outline.item_dto(new).unwrap();
            // Default (folder) mode → a Chapter is a Folder/ChapterScene.
            assert_eq!(dto.role, BinderItemRole::Folder);
            assert_eq!(dto.sub_role, BinderItemSubRole::ChapterScene);
        }

        #[test]
        fn child_appends_inside_a_book_before_its_book_end() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let ch1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                1,
                1,
            );
            let end = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::BookEnd,
                1,
                2,
            );

            outline.add_recommended(
                Some(key_of(&outline, book)),
                &rec(CreateType::Chapter, Relation::Child),
            );

            let order = order_of(&outline, binder);
            assert_eq!(order.len(), 4);
            let new = order[2];
            // New chapter lands after the existing chapter but *before* BookEnd.
            assert_eq!(order, vec![book, ch1, new, end]);
            // …and at the book's child indent.
            assert_eq!(outline.item_dto(new).unwrap().indent, 1);
        }

        #[test]
        fn sibling_lands_after_the_anchor_folders_whole_subtree() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                1,
            );
            let s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                2,
            );

            outline.add_recommended(
                Some(key_of(&outline, chapter)),
                &rec(CreateType::Chapter, Relation::Sibling),
            );

            let order = order_of(&outline, binder);
            let new = *order.last().unwrap();
            // After both scenes (not nested between the chapter and its children).
            assert_eq!(order, vec![chapter, s1, s2, new]);
            assert_eq!(outline.item_dto(new).unwrap().indent, 0);
        }

        #[test]
        fn parent_sibling_targets_the_enclosing_chapters_level() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                1,
                1,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                2,
                2,
            );
            let s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                2,
                3,
            );

            // Anchored on a deep scene: a new Chapter should start after the whole
            // enclosing chapter, at the chapter's own indent — not nested in it.
            outline.add_recommended(
                Some(key_of(&outline, s1)),
                &rec(CreateType::Chapter, Relation::ParentSibling),
            );

            let order = order_of(&outline, binder);
            let new = *order.last().unwrap();
            assert_eq!(order, vec![book, chapter, s1, s2, new]);
            assert_eq!(outline.item_dto(new).unwrap().indent, 1);
        }

        #[test]
        fn book_end_recommendation_is_gated_once_the_book_has_one() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let _end = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::BookEnd,
                1,
                1,
            );

            let recs = outline.recommendations_for_key(Some(key_of(&outline, book)));
            assert!(
                recs.iter().all(|r| r.create_type != CreateType::EndOfBook),
                "End of Book should be hidden when the book already has one"
            );
        }

        #[test]
        fn binder_and_empty_selection_use_the_root_recommendations() {
            let (outline, binder) = seed();
            let expected = skribisto_model::recommendations_root();
            assert_eq!(
                outline.recommendations_for_key(Some(binder_key_of(&outline, binder))),
                expected
            );
            assert_eq!(outline.recommendations_for_key(None), expected);
        }

        #[test]
        fn promote_flips_scene_to_note() {
            let (outline, binder) = seed();
            let scene = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                0,
                0,
            );
            outline.promote(key_of(&outline, scene), PromoteTarget::Note);
            let dto = outline.item_dto(scene).unwrap();
            assert_eq!(dto.role, BinderItemRole::Item);
            assert_eq!(dto.sub_role, BinderItemSubRole::Note);
        }

        #[test]
        fn promote_flips_chapterscene_to_chapter_folder() {
            let (outline, binder) = seed();
            let cs = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            outline.promote(key_of(&outline, cs), PromoteTarget::ChapterFolder);
            let dto = outline.item_dto(cs).unwrap();
            assert_eq!(dto.role, BinderItemRole::Folder);
            assert_eq!(dto.sub_role, BinderItemSubRole::ChapterScene);
        }

        #[test]
        fn demote_blocked_children_counts_the_subtree() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                1,
            );
            let _s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                2,
            );
            // The chapter folder holds two scenes, so collapsing it into a flat chapter
            // is blocked...
            assert_eq!(
                outline
                    .demote_blocked_children(key_of(&outline, chapter), PromoteTarget::FlatChapter),
                2
            );
            // ...but becoming another *folder* is not: nothing is being collapsed.
            assert_eq!(
                outline
                    .demote_blocked_children(key_of(&outline, chapter), PromoteTarget::PartFolder),
                0
            );
            // A leaf scene has no container to empty.
            assert_eq!(
                outline.demote_blocked_children(key_of(&outline, s1), PromoteTarget::Note),
                0
            );
        }

        /// A trashed child is not "in" the chapter any more — `activated = !trashed` and a
        /// trashed row keeps its slot and indent in the binder order, so counting the raw
        /// subtree span made the guard block a chapter the writer had already emptied. The
        /// prompt itself says "move or trash them", so trashing them all must unblock it.
        #[test]
        fn trashed_children_do_not_block_the_demote() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                1,
            );
            let s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                2,
            );
            let blocked = |o: &OutlineViewModel| {
                o.demote_blocked_children(key_of(o, chapter), PromoteTarget::FlatChapter)
            };
            assert_eq!(blocked(&outline), 2);

            // Trash one: the other still blocks, and the count is now honest about it.
            outline.trash_keys(&[key_of(&outline, s1)]);
            assert_eq!(blocked(&outline), 1);

            // Trash the last one: the chapter is empty as far as the writer is concerned,
            // so the conversion goes through.
            outline.trash_keys(&[key_of(&outline, s2)]);
            assert_eq!(blocked(&outline), 0);

            // And it really converts — the guard was the only thing standing in the way.
            outline.promote(key_of(&outline, chapter), PromoteTarget::FlatChapter);
            let dto = outline.item_dto(chapter).unwrap();
            assert_eq!(dto.role, BinderItemRole::Item);
        }

        /// The other half of letting trashed children through the demote guard: those rows
        /// keep their indent, so the chapter they were nested in is now a *leaf* sitting
        /// right above them. Restoring one in place would strand a live row nested under a
        /// leaf — silently, since nothing else re-checks that. `restore_items` must instead
        /// report it `orphaned` and leave it indexed, which is what makes the trash dock
        /// open its destination picker.
        #[test]
        fn restoring_into_a_demoted_chapter_is_reported_orphaned() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let scene = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                1,
            );
            outline.trash_keys(&[key_of(&outline, scene)]);

            let work_id = outline.ids.work_id.get().unwrap();
            let infos = || {
                work_commands::get_work_relationship(
                    &outline.app_ctx,
                    &work_id,
                    &WorkRelationshipField::TrashInfos,
                )
                .unwrap()
            };
            let restore = || {
                trash_management_commands::restore_items(
                    &outline.app_ctx,
                    None,
                    &frontend::trash_management::RestoreItemsDto {
                        work_id,
                        trash_info_ids: infos().iter().map(|&x| x as i64).collect(),
                    },
                )
                .unwrap()
            };

            // While the chapter is still a folder, the scene restores in place as before.
            let res = restore();
            assert!(
                !res.orphaned,
                "a live folder is a valid place to restore into"
            );
            assert_eq!(res.restored_count, 1);
            assert!(outline.item_dto(scene).unwrap().activated);
            assert!(infos().is_empty(), "a consumed TrashInfo is unlinked");

            // Now trash it again and collapse the chapter under it.
            outline.trash_keys(&[key_of(&outline, scene)]);
            outline.promote(key_of(&outline, chapter), PromoteTarget::FlatChapter);
            assert_eq!(
                outline.item_dto(chapter).unwrap().role,
                BinderItemRole::Item
            );

            let res = restore();
            assert!(
                res.orphaned,
                "the chapter is a leaf now — the scene has nowhere to be restored *into*"
            );
            assert_eq!(res.restored_count, 0);
            assert!(
                !outline.item_dto(scene).unwrap().activated,
                "it must stay trashed rather than come back nested under a leaf"
            );
            assert_eq!(
                infos().len(),
                1,
                "its TrashInfo stays indexed — that is what drives the destination picker"
            );
        }

        /// An item's name has two homes: `BinderItem.title`, which the outline tree and
        /// the tab show, and a title `Content` row, which compiles into the manuscript.
        /// **Renaming through either door must reach both.**
        ///
        /// Renaming a chapter in the tree used to write only the entity field, leaving the
        /// manuscript compiling the old title; renaming it in its editor wrote only the
        /// content row, leaving the tree and the tab showing the old name. Both now go
        /// through `SingleBinderItem::set_title`.
        #[test]
        fn a_rename_reaches_both_homes_of_the_title() {
            let (outline, binder) = seed();
            let ch = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );

            outline.rename(key_of(&outline, ch), "The Long Road");

            // The entity field the tree and the tab read...
            assert_eq!(outline.item_dto(ch).unwrap().title, "The Long Road");
            // ...and the content row the manuscript compiles.
            let content_ids = binder_item_commands::get_binder_item_relationship(
                outline.app_ctx(),
                &ch,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap();
            let chapter_title =
                content_commands::get_content_multi(outline.app_ctx(), &content_ids)
                    .unwrap()
                    .into_iter()
                    .flatten()
                    .find(|c| c.role == ContentRole::ChapterTitle)
                    .map(|c| c.data);
            assert_eq!(
                chapter_title.as_deref(),
                Some("The Long Road"),
                "a tree rename must reach the title the manuscript compiles"
            );
        }

        /// The other half of the rename story: `a_rename_reaches_both_homes_of_the_title`
        /// calls `rename` directly with a freshly-resolved key; this drives the **actual**
        /// context-menu wiring end to end — `begin_rename` (the `MenuItem`'s
        /// `on_activate_fn`) presenting the `InputDialog`, then its `on_result` closure
        /// firing *later* (after layout, after a `SetValue`/`Click` round trip) and
        /// resolving `key` again through the same live tree model.
        ///
        /// Guards the three things `BinderTreeKey`'s uid re-keying put at risk: the
        /// dialog actually appears (a modal request is queued), it's pre-filled with the
        /// row's current title, and the deferred `on_result` still finds the row and
        /// applies the edit rather than silently no-oping.
        #[test]
        fn the_rename_dialog_appears_and_a_submitted_title_actually_renames() {
            use bastyde::core::ModalContent;
            use bastyde::core::accessibility::widget_id_to_node_id;
            use bastyde::core::widget_id::WidgetId;
            use bastyde::core::widget_tree::WidgetTree;
            use bastyde::i18n::lit;
            use bastyde::widgets::Button;

            let (outline, binder) = seed();
            let item = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                0,
                0,
            );
            let key = key_of(&outline, item);

            // Mirrors `binder_context_menu`'s "Rename" `MenuItem` exactly:
            // `.on_activate_fn(move |ctx| rename.begin_rename(key, ctx))`.
            let vm = outline.clone();
            let mut tree = WidgetTree::new().with_theme(intui::light());
            let trigger = tree.add(
                Button::new(lit!("rename")).on_activate_fn(move |ctx| vm.begin_rename(key, ctx)),
            );
            tree.layout(SizeProposal::exact(420.0, 60.0));

            tree.dispatch_event(WidgetEvent::AccessAction {
                action: bastyde::core::accesskit::Action::Click,
                target: Some(trigger),
                target_node: widget_id_to_node_id(trigger),
                data: None,
            });

            assert!(
                tree.has_pending_modal_requests(),
                "the dialog must appear: begin_rename must queue a modal request"
            );
            let request = tree.drain_pending_modal_requests().pop().unwrap().request;
            let ModalContent::Deferred(builder) = request.content else {
                panic!("InputDialog must present as deferred content");
            };
            let content_id = builder(&mut tree);
            tree.layout(SizeProposal::exact(420.0, 180.0));

            // Collect descendants by type name — locale-independent, unlike hunting the
            // OK/Cancel buttons by their translated label.
            fn collect(tree: &WidgetTree, root: WidgetId, needle: &str, out: &mut Vec<WidgetId>) {
                if tree
                    .widget_type_name(root)
                    .is_some_and(|t| t.contains(needle))
                {
                    out.push(root);
                }
                for c in tree.children(root) {
                    collect(tree, c, needle, out);
                }
            }

            let mut fields = Vec::new();
            collect(&tree, content_id, "TextInputField", &mut fields);
            let field = *fields
                .first()
                .expect("the InputDialog must mount its text field");
            {
                let update = tree.sync_accessibility();
                let field_node = widget_id_to_node_id(field);
                let value = update
                    .nodes
                    .iter()
                    .find(|(id, _)| *id == field_node)
                    .and_then(|(_, n)| n.value());
                assert_eq!(
                    value,
                    Some("Item/Scene"),
                    "begin_rename must read the row's current title via node_of and \
                     pre-fill the dialog with it"
                );
            }

            // Overwrite the pre-filled title (proving the field is live), the
            // AT-driven equivalent of selecting all and typing.
            tree.dispatch_event(WidgetEvent::AccessAction {
                action: bastyde::core::accesskit::Action::SetValue,
                target: Some(field),
                target_node: widget_id_to_node_id(field),
                data: Some(bastyde::core::accesskit::ActionData::Value(
                    "Renamed Scene".into(),
                )),
            });

            // The field's edit is debounced onto its outer `Signal<String>` via a
            // frame-tick effect (`TextInputField`'s doc comment: "frames are
            // demand-driven"), so the OK button's `text_for_ok.get()` won't see it
            // until a frame actually ticks.
            tree.request_frame();
            tree.tick_animations(std::time::Duration::from_millis(16));
            tree.layout(SizeProposal::exact(420.0, 180.0));

            let mut buttons = Vec::new();
            collect(&tree, content_id, "Button", &mut buttons);
            // Footer order is Cancel, then OK (`InputDialogBody::build`) — the last
            // Button is OK.
            let ok = *buttons
                .last()
                .expect("the InputDialog must mount its OK/Cancel buttons");

            tree.dispatch_event(WidgetEvent::AccessAction {
                action: bastyde::core::accesskit::Action::Click,
                target: Some(ok),
                target_node: widget_id_to_node_id(ok),
                data: None,
            });

            assert_eq!(
                outline.item_dto(item).unwrap().title,
                "Renamed Scene",
                "the on_result closure must resolve the key later, through the live tree \
                 model, and actually apply the rename"
            );
        }

        /// The headline of this feature: outline in bare folders, then declare what each
        /// one is. A plain folder carries only a synopsis, which every folder type allows,
        /// so every one of these conversions is lossless.
        #[test]
        fn a_plain_folder_becomes_any_other_kind_of_folder() {
            for (target, want) in [
                (
                    PromoteTarget::ChapterFolder,
                    BinderItemSubRole::ChapterScene,
                ),
                (PromoteTarget::PartFolder, BinderItemSubRole::Part),
                (PromoteTarget::BookFolder, BinderItemSubRole::Book),
                (PromoteTarget::NoteFolder, BinderItemSubRole::Note),
            ] {
                let (outline, binder) = seed();
                let f = seed_item(
                    &outline,
                    binder,
                    BinderItemRole::Folder,
                    BinderItemSubRole::None,
                    0,
                    0,
                );
                assert!(
                    outline
                        .promote_targets_of(key_of(&outline, f))
                        .contains(&target),
                    "a plain folder must offer {target:?}"
                );
                outline.promote(key_of(&outline, f), target);
                let dto = outline.item_dto(f).unwrap();
                assert_eq!(dto.role, BinderItemRole::Folder);
                assert_eq!(dto.sub_role, want, "promoting to {target:?}");
            }
        }

        // ── the created row is named for its type, and is actually visible ──

        use bastyde::data::TreeDataSource;

        /// Every creatable type gets its own default title. The bug this pins:
        /// the title used to be derived from `role` alone, which cannot tell a
        /// chapter from a book from a note folder — they are all `Folder` — so
        /// six of the eight types came out as the same "New Folder".
        #[test]
        fn a_created_row_is_titled_for_its_type_not_generically() {
            let mut seen: Vec<String> = Vec::new();
            for create_type in [
                CreateType::Book,
                CreateType::Part,
                CreateType::Chapter,
                CreateType::Scene,
                CreateType::Note,
                CreateType::NoteFolder,
                CreateType::Folder,
            ] {
                let (outline, binder) = seed();
                outline.add_recommended(None, &rec(create_type, Relation::Child));
                let created = order_of(&outline, binder);
                assert_eq!(
                    created.len(),
                    1,
                    "{create_type:?} must create exactly one row"
                );
                let title = outline.item_dto(created[0]).unwrap().title;
                assert!(
                    !title.is_empty(),
                    "{create_type:?} must not be created untitled"
                );
                // The row carries this type's own default, resolved to data at
                // creation — not a title derived from `role`, which collapses six
                // distinct types onto one string.
                let want: String = crate::binder::create_labels::default_title(create_type).into();
                assert_eq!(
                    title, want,
                    "{create_type:?} must be titled from its own vocabulary"
                );
                assert_ne!(
                    title, "New Item",
                    "{create_type:?} still falls back to the old generic title"
                );
                seen.push(title);
            }
            // Distinct types must read distinctly — otherwise "adapt the name to the
            // type" is satisfied only in the letter.
            let mut unique = seen.clone();
            unique.sort();
            unique.dedup();
            assert_eq!(
                unique.len(),
                seen.len(),
                "each type needs its own title, got duplicates in {seen:?}"
            );
        }

        /// The reported bug: creating the **first** child of a container left the
        /// parent collapsed, so the new row was real, selected, and invisible.
        /// A container with no children has never been expanded — there was no
        /// twist to open — so nothing put it in the expanded set.
        #[test]
        fn creating_a_first_child_expands_the_parent_that_had_none() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let chapter_key = key_of(&outline, chapter);
            assert!(
                !outline.model.is_expanded(&chapter_key),
                "precondition: a childless container starts collapsed"
            );

            outline.add_recommended(Some(chapter_key), &rec(CreateType::Scene, Relation::Child));

            assert!(
                outline.model.is_expanded(&chapter_key),
                "the parent must be expanded so its first child is visible"
            );
            // …and the new row is what the writer is now pointed at.
            let scene = *order_of(&outline, binder)
                .iter()
                .find(|id| **id != chapter)
                .expect("the scene must exist");
            assert_eq!(
                outline.selection.selected_keys().first().copied(),
                Some(key_of(&outline, scene)),
                "the newly created row must be selected"
            );
        }

        /// Creating must not drag the outline dock open. The same create path
        /// serves the Corkboard and Overview header buttons, so forcing the dock
        /// forward on every create interrupts a writer who is working elsewhere.
        /// Expanding + selecting is the whole job; showing is `reveal_item`'s.
        #[test]
        fn creating_selects_without_forcing_the_outline_open() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            outline.hide();
            assert!(
                !outline.is_visible().get(),
                "precondition: the outline is hidden"
            );

            outline.add_recommended(
                Some(key_of(&outline, chapter)),
                &rec(CreateType::Scene, Relation::Child),
            );

            assert!(
                !outline.is_visible().get(),
                "a create must not pop the outline open"
            );
            assert!(
                outline.model.is_expanded(&key_of(&outline, chapter)),
                "…but the parent must still be expanded"
            );
        }

        /// Expansion must reach *every* ancestor, not just the immediate parent —
        /// a create can land several levels below anything currently open.
        #[test]
        fn revealing_opens_the_whole_ancestor_chain() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let part = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Part,
                1,
                1,
            );
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                2,
                2,
            );
            for k in [
                key_of(&outline, book),
                key_of(&outline, part),
                key_of(&outline, chapter),
            ] {
                outline.model.set_expanded(&k, false);
            }

            outline.add_recommended(
                Some(key_of(&outline, chapter)),
                &rec(CreateType::Scene, Relation::Child),
            );

            for (id, what) in [(book, "book"), (part, "part"), (chapter, "chapter")] {
                assert!(
                    outline.model.is_expanded(&key_of(&outline, id)),
                    "the {what} ancestor must be expanded too"
                );
            }
        }
    }

    // ── explicit language propagation ("Apply to children") ──
    //
    // Nothing inherits a language from a container any more (`skribisto_model::language`
    // resolves an item's own tag, else the Work's), so this button is the *only* way a
    // language reaches a subtree. These assert the write itself; `skribisto_model` owns the
    // resolution rule these values then feed.
    #[cfg(not(feature = "mocks"))]
    mod apply_language {
        use super::recommend::{seed, seed_item};
        use super::*;
        /// Space-separated in the tests, a list in storage — one parser, shared.
        /// Scoped to this module: it is the only one that uses it, and at the
        /// `tests` level it read as unused under `--features mocks`, where the
        /// module is compiled out.
        use skribisto_model::language::parse_legacy_list as tags;
        use std::collections::HashMap;

        /// Book > Chapter > Scene, plus an outsider after the chapter's subtree, so the
        /// blast radius is observable in both directions.
        fn seed_tree(outline: &OutlineViewModel, binder: u64) -> (u64, u64, u64, u64) {
            let book = seed_item(
                outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let chapter = seed_item(
                outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                1,
                1,
            );
            let scene = seed_item(
                outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                2,
                2,
            );
            // A sibling scene back at the Book's level — NOT part of the chapter's subtree.
            let outsider = seed_item(
                outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                3,
            );
            (book, chapter, scene, outsider)
        }

        fn lang_of(outline: &OutlineViewModel, id: u64) -> Vec<String> {
            outline
                .item_dto(id)
                .map(|d| d.dict_language)
                .unwrap_or_default()
        }

        #[test]
        fn writes_every_descendant_and_leaves_everything_else_alone() {
            let (outline, binder) = seed();
            let (book, chapter, scene, outsider) = seed_tree(&outline, binder);

            outline.apply_dict_language_to_subtree(chapter, &tags("tr-TR"));

            assert_eq!(
                lang_of(&outline, scene),
                tags("tr-TR"),
                "the descendant is written"
            );
            assert_eq!(
                lang_of(&outline, book),
                tags(""),
                "an ancestor is untouched"
            );
            assert_eq!(
                lang_of(&outline, outsider),
                tags(""),
                "a non-descendant is untouched"
            );
            assert_eq!(
                lang_of(&outline, chapter),
                tags(""),
                "the item itself is untouched — the pill field beside the button owns that"
            );
        }

        /// The whole point of the composite: an over-broad apply is one Ctrl+Z, not one per
        /// descendant.
        #[test]
        fn is_a_single_undo_step() {
            let (outline, binder) = seed();
            outline.init_stack();
            let (_book, chapter, scene, _outsider) = seed_tree(&outline, binder);
            let stack = outline.stack();

            outline.apply_dict_language_to_subtree(chapter, &tags("tr-TR"));
            assert_eq!(lang_of(&outline, scene), tags("tr-TR"));

            undo_redo_commands::undo(&outline.app_ctx, stack).unwrap();
            assert_eq!(
                lang_of(&outline, scene),
                tags(""),
                "one undo reverses the whole apply, not just the last descendant"
            );

            undo_redo_commands::redo(&outline.app_ctx, stack).unwrap();
            assert_eq!(
                lang_of(&outline, scene),
                tags("tr-TR"),
                "and redo puts it back"
            );
        }

        /// An empty list is a legitimate value to push: it resets the subtree to inheriting
        /// the Work's language — the only way to undo an over-broad apply after the fact
        /// without visiting every child by hand.
        #[test]
        fn an_empty_list_resets_descendants_to_the_work_language() {
            let (outline, binder) = seed();
            let (_book, chapter, scene, _outsider) = seed_tree(&outline, binder);
            outline.apply_dict_language_to_subtree(chapter, &tags("tr-TR"));
            assert_eq!(lang_of(&outline, scene), tags("tr-TR"));

            outline.apply_dict_language_to_subtree(chapter, &tags(""));
            assert_eq!(
                lang_of(&outline, scene),
                tags(""),
                "cleared back to inheriting"
            );

            // And the resolver then hands it the Work's language — the two halves meeting.
            let items = vec![frontend::common::entities::BinderItem {
                id: scene,
                dict_language: lang_of(&outline, scene),
                ..Default::default()
            }];
            let mut out = HashMap::new();
            skribisto_model::language::tags_in_binder(&tags("en-US"), &items, &mut out);
            assert_eq!(out[&scene], tags("en-US"));
        }

        /// A leaf has no subtree, so the button is never offered — and the call is inert if
        /// it somehow fires anyway.
        #[test]
        fn a_leaf_has_no_descendants_and_the_call_is_a_noop() {
            let (outline, binder) = seed();
            let (_book, _chapter, scene, _outsider) = seed_tree(&outline, binder);
            assert!(
                outline.subtree_descendants(scene).is_empty(),
                "the button's own gate"
            );
            outline.apply_dict_language_to_subtree(scene, &tags("tr-TR"));
            assert_eq!(
                lang_of(&outline, scene),
                tags(""),
                "an inert call writes nothing"
            );
        }
    }
}
