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

use teksilo::data::{DropPosition, KeyedSelectionModel, SelectionMode};
use teksilo::prelude::*; // EventContext, Signal, tr!
use teksilo::widgets::{DockOpenLocation, DockSide, DockWidgetId, DockingModel, InputDialog};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands,
    trash_management_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto};

use frontend::binder_item_management::{
    DuplicateDto, MoveDto, MovePlace, SetDescendantsDictLanguageDto, SetDescendantsExportableDto,
};
use frontend::trash_management::{TrashBinderDto, TrashBinderItemsDto};

use skribisto_compiler::headings;
use skribisto_model::{PromoteTarget, Recommendation, Relation, SubRoleExt};

use crate::app_ids::AppIds;
use crate::binder::placement;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, CommitMove, TreeFilters};
use crate::shared::binder_ops::{self, update_item_dto};
use crate::singles::{SingleBinder, SingleBinderItem};

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
    /// forward — the half of [`Self::reveal_item`] that a create wants.
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
    /// folders alike. See [`crate::binder::create_labels::default_title`].
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

    /// Whether `key` takes part in the book's numbering — `None` when the row has no
    /// numbering to speak of (a scene, a note, a binder), which is also the gate for
    /// showing the affordance at all.
    pub fn numbering_state(&self, key: BinderTreeKey) -> Option<bool> {
        let item_id = self.model.item_id_of(&key)?;
        let it = binder_ops::item_dto(&self.app_ctx, item_id)?;
        skribisto_model::numbering::level_of(&it.sub_role)?;
        Some(!it.exclude_from_numbering)
    }

    /// Take `key` in or out of the book's numbering — the prologue lever, reachable from
    /// the row itself rather than only from the Inspector.
    ///
    /// An unnumbered row keeps everything else: its heading, its prose, its word count. It
    /// stops printing a number *and* stops consuming one, so the chapter after a prologue
    /// is chapter one. That is the half writers do not expect and the half that matters —
    /// hence the tooltip on both surfaces.
    pub fn set_numbered(&self, key: BinderTreeKey, on: bool) {
        let Some(item_id) = self.model.item_id_of(&key) else {
            return;
        };
        let probe = SingleBinderItem::new(self.app_ctx.clone());
        probe.set_id(Some(item_id));
        if let Err(e) = probe.set_excluded_from_numbering(!on, self.stack()) {
            eprintln!("outline: set numbered failed for item {item_id}: {e}");
        }
        self.reload();
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
    ///
    /// **An emptied title is accepted for an item, and rejected for a binder.** They are
    /// not the same kind of name. A chapter clearing its title is naming itself by its
    /// ordinal instead — the state Tidy chapter titles… produces wholesale, and the state
    /// the exporter has always rendered as "Chapter 3" — so refusing it would leave the
    /// writer able to reach it in bulk but not one row at a time. A binder has no ordinal
    /// and no generated name to fall back on; an empty one is simply a blank row in the
    /// switcher, so the old guard still applies there.
    pub fn begin_rename(&self, key: BinderTreeKey, ctx: &mut EventContext) {
        let current = self.model.node_of(&key).map(|(_, t)| t).unwrap_or_default();
        let vm = self.clone();
        let is_binder = matches!(key, BinderTreeKey::Binder(_));
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |result, _ctx| {
                if let Some(name) = result {
                    let name = name.trim();
                    if is_binder && name.is_empty() {
                        return;
                    }
                    vm.rename(key, name);
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
                    let mut dto = binder_ops::update_binder_dto(&binder);
                    dto.name = title.to_string();
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

    /// Set every descendant's `is_exportable` to `value` in **one** undo step. The item
    /// itself is not touched — the Inspector's own toggle owns that; this is the "apply to
    /// children" affordance beside it.
    ///
    /// One backend call, not a composite of per-item writes: the subtree walk *and* the
    /// undo step both belong to
    /// [`binder_item_management::set_descendants_exportable`](frontend::commands::binder_item_management_commands::set_descendants_exportable),
    /// so a binder screen that is not this outline gets the same gesture without
    /// re-deriving "beneath" from an ordered, indent-annotated view it may not hold.
    pub fn apply_exportable_to_subtree(&self, item_id: u64, value: bool) {
        let _ = binder_item_management_commands::set_descendants_exportable(
            &self.app_ctx,
            self.stack(),
            &SetDescendantsExportableDto {
                item_id,
                exportable: value,
            },
        );
        self.reload();
    }

    /// The chapters and parts whose *title* says nothing but their own number.
    ///
    /// Every project this app has ever created starts out this way: the new-project
    /// template writes "Chapter 1".."Chapter N" into each chapter's title, because until
    /// the ordinal became visible that was the only place a writer could see it. Now that
    /// the binder shows the number itself, those titles read as "3. Chapter 3" — the
    /// duplication has simply moved from the exported file into the outline.
    ///
    /// Detection is [`headings::is_redundant_number_title`], the *same* predicate the
    /// exporter uses to decide whether to append a title to a number, against the row's own
    /// resolved language. So a title this offers to clear is exactly a title the export was
    /// already discarding — clearing it changes what the writer sees, never what the book
    /// says. Roman numerals, spelled-out numbers and a title naming a *different* number
    /// are all deliberately excluded there, and so are excluded here.
    ///
    /// Returns `(item_id, title)` in stream order, for a preview the writer confirms.
    pub fn redundant_number_titles(&self) -> Vec<(u64, String)> {
        let ctx = &*self.app_ctx;
        let Some(work_id) = self.ids.work_id.get() else {
            return Vec::new();
        };
        let Ok(Some(work)) = work_commands::get_work(ctx, &work_id) else {
            return Vec::new();
        };
        // The whole Work, in stream order — the same walk the badge is numbered from.
        let mut items = Vec::new();
        for binder_id in
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default()
        {
            let ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let by_id: HashMap<u64, _> = binder_item_commands::get_binder_item_multi(ctx, &ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|it| (it.id, it))
                .collect();
            items.extend(ids.into_iter().filter_map(|id| by_id.get(&id).cloned()));
        }
        let numbers = crate::models::numbers_for_items(ctx, work_id, &items);
        let work_langs =
            skribisto_model::language::parse_legacy_list(&work.dict_language.join(" "));
        redundant_of(&items, &numbers, &work_langs)
    }

    /// Clear the titles named by [`Self::redundant_number_titles`], in **one** undo step.
    ///
    /// Clearing rather than rewriting: the number is not the title's to hold any more, and
    /// an empty title is a first-class state everywhere — the binder falls back to the
    /// badge plus the item's type, and the exporter's `NumberAndTitle` already renders an
    /// untitled chapter as its number alone. A writer who wanted "Chapter 3" printed gets
    /// exactly that, from the generator, wherever the export style asks for it.
    ///
    /// Takes explicit ids rather than re-deriving them, so the rows the writer saw in the
    /// preview are the rows that change even if something moved in between.
    pub fn clear_number_titles(&self, ids: &[u64]) {
        if ids.is_empty() {
            return;
        }
        let ctx = &*self.app_ctx;
        let stack = self.stack();
        let _ = undo_redo_commands::begin_composite(ctx, stack);
        for &id in ids {
            // `set_title` writes both homes — `BinderItem.title` and the title `Content`
            // row — so neither is left holding a number the other has dropped.
            let probe = SingleBinderItem::new(self.app_ctx.clone());
            probe.set_id(Some(id));
            if let Err(e) = probe.set_title("", stack) {
                // Keep going and close the composite: a partial clear is still one undo
                // step, and abandoning it half-way would leave the writer with a mixture
                // they cannot revert in one go. Reported rather than swallowed — the row
                // simply keeps its old title, and the next tidy will offer it again.
                eprintln!("outline: clearing the title of item {id} failed: {e}");
            }
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
    ///
    /// One backend call, not a composite of per-item writes — the same move as
    /// [`Self::apply_exportable_to_subtree`], and for the same reason: the subtree walk
    /// belongs beside the write, not above it.
    pub fn apply_dict_language_to_subtree(&self, item_id: u64, tags: &[String]) {
        let _ = binder_item_management_commands::set_descendants_dict_language(
            &self.app_ctx,
            self.stack(),
            &SetDescendantsDictLanguageDto {
                item_id,
                tags: tags.to_vec(),
            },
        );
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

/// The redundancy filter itself, split out from the backend reads so it can be tested.
///
/// A row qualifies only if it carries an ordinal at all (so scenes, notes and anything the
/// writer excluded are never candidates) and its title merely restates that ordinal, judged
/// in the language the row is actually written in — its own tag if it has one, else the
/// Work's, which is the resolution the exporter's `HeadingLanguage::Auto` performs per row.
/// Getting that wrong in either direction is what makes this worth isolating: an English
/// manuscript must not have "Chapitre 3" cleared, and a French one must.
fn redundant_of(
    items: &[frontend::direct_access::BinderItemDto],
    numbers: &HashMap<u64, skribisto_model::numbering::Numbered>,
    work_langs: &[String],
) -> Vec<(u64, String)> {
    let mut out = Vec::new();
    for it in items {
        let Some(n) = numbers.get(&it.id) else {
            continue;
        };
        let tags: &[String] = if it.dict_language.iter().any(|t| !t.is_empty()) {
            &it.dict_language
        } else {
            work_langs
        };
        let lang = skribisto_model::language::primary(tags);
        if !it.title.trim().is_empty()
            && headings::is_redundant_number_title(&it.title, lang, n.level, n.number())
        {
            out.push((it.id, it.title.clone()));
        }
    }
    out
}

#[cfg(test)]
mod tests;
