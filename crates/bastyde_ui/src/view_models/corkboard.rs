// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `CorkboardViewModel` — the business logic behind one container tab's Corkboard
//! segment. One per tab, built alongside `stream`/`pace` in `ContentTab::new` and
//! gated on the same [`StreamLevel::for_container`](crate::models::StreamLevel).
//!
//! It owns the drilled-into `current_container` (a folder card drills in *in
//! place*, deepening the breadcrumb — no new tab is opened, which sidesteps the
//! absence of an "open a tab at a segment" API), the free search/sort state, and
//! the [`CorkboardCardsModel`] + its [`SortFilterListModel`] projection. Cross-
//! view-model talk stays a DAG: it never imports a peer view-model — it drills in
//! place, or fires an [`AppIntent`] on the bus (open an item, create an item).
//!
//! Plain Rust → headless-testable.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::core::ObserverHandle;
use bastyde::data::{SelectionMode, SelectionModel, SortDirection, SortFilterListModel};
use bastyde::prelude::*; // Signal, BuildContext, EventContext
use bastyde::widgets::InputDialog;

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, MovePlace, SplitSceneDto};
use frontend::commands::{
    binder_item_commands, binder_item_management_commands, trash_management_commands,
};
use frontend::trash_management::TrashBinderItemsDto;

use skribisto_model::counting::CountingMethodSetting;
use skribisto_model::{CreateType, Recommendation, Relation};

use crate::app_ids::AppIds;
use crate::intents::AppIntent;
use crate::models::{CorkboardCard, CorkboardCardsModel, OpenDoc, OpenDocsStore};
use crate::singles::SingleBinderItem;
use crate::view_models::EditorTypography;

use super::binder_ops::{self, is_prose_bearing, opens_a_section, split_djot, update_item_dto};

/// The sort column ids bound into the projection (see [`CorkboardCardsModel::projection`]).
pub const SORT_TITLE: &str = "title";

struct Inner {
    /// The container currently shown (the tab's own container at the trail root,
    /// or a folder drilled into). Shared with the model, which re-queries on change.
    current_container: Signal<u64>,
    /// Ancestor path `(id, title)` for the breadcrumb, root-inclusive. The last
    /// entry is the current container; its title is kept fresh from `container_probe`.
    trail: Signal<Vec<(u64, String)>>,
    /// Live text filter → the projection's "text" column.
    query: Signal<String>,
    /// Active sort, or `None` for manuscript order.
    sort: Signal<Option<(String, SortDirection)>>,
    /// `!query.is_empty() || sort.is_some()` — the grid binds the projection (and
    /// disables reorder) while this is true. A real signal (not a derived one) so
    /// the grid wrapper can bind it at `BindingLevel::Rebuild`.
    projecting: Signal<bool>,
    /// Visible card count, for the header.
    count: Signal<usize>,

    // Threaded settings.
    nested: Signal<bool>,
    card_size: Signal<f32>,
    show_word_count: Signal<bool>,
    counting_method: Signal<CountingMethodSetting>,

    // Per-tab, transient (not persisted).
    selection: SelectionModel,
    scroll_y: Signal<f32>,
    /// The card whose title is being edited inline (`None` = nobody). One at a
    /// time; a card's title renders as an editable field iff its id matches.
    editing_item: Signal<Option<u64>>,
    /// The `GridView`'s widget id, published by the grid on build. The inline
    /// title editor returns focus here when it commits/cancels, so a screen
    /// reader (and keyboard nav) lands back on the card instead of the window.
    grid_id: Signal<Option<WidgetId>>,
    /// The shared open documents backing the visible cards' synopsis editors — one
    /// per card realized since the container was entered, keyed by item id. Every
    /// card renders its synopsis in a *live* editor over the **same** `OpenDoc` an
    /// editor tab uses (one source of truth: an edit on a card, in the expand modal,
    /// or in a tab is one edit, one undo, one save). Held here (a strong ref) so the
    /// refcount stays up while the container is shown, and flushed + released
    /// together when the container changes or the pane is torn down. Mirrors
    /// `StreamViewModel`'s per-row `row_handles`.
    open_synopses: RefCell<HashMap<u64, Rc<OpenDoc>>>,
    /// The app-wide open-document store (shared with the editor panes).
    docs: OpenDocsStore,
    /// Synopsis typography, so the card's synopsis editor renders like the scene
    /// editor.
    synopsis_typo: EditorTypography,

    cards: CorkboardCardsModel,
    projection: SortFilterListModel<CorkboardCard>,
    /// Tracks `current_container`'s dto for the current-crumb title + the "＋ Create"
    /// recommendations (which depend on the container's `(role, sub_role)`).
    container_probe: SingleBinderItem,

    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// Keeps model/probe change-observers alive for the model's lifetime.
    observers: RefCell<Vec<ObserverHandle>>,
}

#[derive(Clone)]
pub struct CorkboardViewModel {
    inner: Rc<Inner>,
}

impl CorkboardViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        docs: OpenDocsStore,
        container_id: u64,
        nested: Signal<bool>,
        card_size: Signal<f32>,
        show_word_count: Signal<bool>,
        counting_method: Signal<CountingMethodSetting>,
        synopsis_typo: EditorTypography,
    ) -> Self {
        let current_container = Signal::new(container_id);
        let cards = CorkboardCardsModel::new(
            app_ctx.clone(),
            ids.work_id.clone(),
            ids.stack_id.clone(),
            current_container.clone(),
            nested.clone(),
        );
        let projection = cards.projection();
        let container_probe = SingleBinderItem::new(app_ctx.clone());
        container_probe.set_id(Some(container_id));

        Self {
            inner: Rc::new(Inner {
                current_container,
                trail: Signal::new(vec![(container_id, String::new())]),
                query: Signal::new(String::new()),
                sort: Signal::new(None),
                projecting: Signal::new(false),
                count: Signal::new(0),
                nested,
                card_size,
                show_word_count,
                counting_method,
                selection: SelectionModel::new(SelectionMode::Multi),
                scroll_y: Signal::new(0.0),
                editing_item: Signal::new(None),
                grid_id: Signal::new(None),
                open_synopses: RefCell::new(HashMap::new()),
                docs,
                synopsis_typo,
                cards,
                projection,
                container_probe,
                app_ctx,
                ids,
                observers: RefCell::new(Vec::new()),
            }),
        }
    }

    /// Subscribe the model + probe and wire the search/sort → projection plumbing.
    /// Idempotent per build (mirrors the stream pane's `WireOnBuild`).
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.cards.wire(ctx);
        self.inner.container_probe.wire(ctx);

        // Search text → the projection's filter (its filters_signal is observe-only,
        // so push imperatively) + recompute `projecting`.
        {
            let me = self.clone();
            ctx.effect(&self.inner.query, move |q| {
                me.inner.projection.set_filter("text", q);
                me.recompute_projecting();
            });
        }
        // Sort selection → the projection's sort + recompute `projecting`.
        {
            let me = self.clone();
            ctx.effect(&self.inner.sort, move |s| {
                match s {
                    Some((col, dir)) => me.inner.projection.set_sort(Some(col), *dir),
                    None => me.inner.projection.clear_sort(),
                }
                me.recompute_projecting();
            });
        }
        // Keep the header count in sync with the (unfiltered) card set.
        {
            let count = self.inner.count.clone();
            let list = self.inner.cards.list();
            count.set(list.len());
            let list2 = list.clone();
            let handle = list.observe_changes(move |_| count.set(list2.len()));
            self.inner.observers.borrow_mut().push(handle);
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────────

    /// Drill into a folder card: deepen the breadcrumb and re-scope the grid, all
    /// in place (same tab, same segment). Only meaningful for a container card.
    pub fn drill_into(&self, folder_id: u64, title: String) {
        let mut trail = self.inner.trail.get();
        // Freeze the level we're leaving with its *resolved* title (the probe has
        // loaded it by now) so its ancestor crumb reads correctly — `get()` on the
        // probe's derived title signal is fine (only `observe`/`effect` isn't).
        if let Some(last) = trail.last_mut() {
            let live = self.inner.container_probe.title().get();
            if !live.is_empty() {
                last.1 = live;
            }
        }
        trail.push((folder_id, title));
        self.inner.trail.set(trail);
        self.enter(folder_id);
    }

    /// Jump to an ancestor crumb (index into the trail).
    pub fn go_to_crumb(&self, index: usize) {
        let mut trail = self.inner.trail.get();
        if index + 1 >= trail.len() {
            return; // already here
        }
        trail.truncate(index + 1);
        let id = trail[index].0;
        self.inner.trail.set(trail);
        self.enter(id);
    }

    fn enter(&self, container_id: u64) {
        // Leaving the level flushes + releases every synopsis doc held for the old
        // container's cards, so their edits persist before the grid re-scopes.
        self.release_all_synopses();
        self.inner.current_container.set(container_id);
        self.inner.container_probe.set_id(Some(container_id));
        self.inner.selection.clear();
        self.inner.query.set(String::new());
        self.inner.scroll_y.set(0.0);
    }

    /// Handle a tile activation (double-click / Enter): a folder in nested mode
    /// drills in; anything else opens the item in the editor via the intent bus.
    pub fn activate(&self, ctx: &mut EventContext, card: &CorkboardCard) {
        if card.is_container && self.inner.nested.get() {
            self.drill_into(card.item_id, card.title.clone());
        } else {
            ctx.send_intent(AppIntent::OpenItem {
                item_id: card.item_id,
                title: card.title.clone(),
            });
        }
    }

    // ── Create ──────────────────────────────────────────────────────────────

    /// The "＋ Create" offers for the current container: the child types the
    /// writing model recommends here. `EndOfBook` is intentionally omitted — a
    /// structural terminator is not something to add from a card board.
    pub fn create_recommendations(&self) -> Vec<Recommendation> {
        let Some(dto) = self.inner.container_probe.dto() else {
            return Vec::new();
        };
        skribisto_model::recommendations(&dto.role, &dto.sub_role)
            .into_iter()
            .filter(|r| r.relation == Relation::Child && r.create_type != CreateType::EndOfBook)
            .collect()
    }

    /// Fire a create anchored on the *current container* (not the outline selection).
    pub fn fire_create(&self, ctx: &mut EventContext, rec: Recommendation) {
        ctx.send_intent(AppIntent::NewItem {
            create_type: rec.create_type,
            relation: rec.relation,
            anchor_item_id: Some(self.inner.current_container.get()),
        });
    }

    // ── Open into the editor panes ────────────────────────────────────────────

    /// Middle-click / "open to the side": open an item card in the *other* editor
    /// pane. Only meaningful for a leaf item — a folder card has no editor.
    pub fn open_to_side(&self, ctx: &mut EventContext, card: &CorkboardCard) {
        if card.is_container {
            return;
        }
        ctx.send_intent(AppIntent::OpenItemToSide {
            item_id: card.item_id,
            title: card.title.clone(),
        });
    }

    // ── Inline rename ─────────────────────────────────────────────────────────

    /// The id of the card whose title is being edited inline (`None` = none).
    pub fn editing_item(&self) -> Signal<Option<u64>> {
        self.inner.editing_item.clone()
    }

    /// The grid's widget id (for returning focus after an inline rename).
    pub fn grid_id(&self) -> Signal<Option<WidgetId>> {
        self.inner.grid_id.clone()
    }
    /// The grid publishes its id here on build.
    pub fn set_grid_id(&self, id: WidgetId) {
        if self.inner.grid_id.get() != Some(id) {
            self.inner.grid_id.set(Some(id));
        }
    }

    /// Begin editing a card's title in place.
    pub fn begin_rename(&self, id: u64) {
        self.inner.editing_item.set(Some(id));
    }

    /// F2 / menu on the current selection: edit the first selected card's title.
    pub fn rename_selected(&self) {
        if let Some(id) = self.first_selected_item() {
            self.begin_rename(id);
        }
    }

    /// Cancel the inline edit without writing (Esc / focus loss with no commit).
    pub fn cancel_rename(&self) {
        if self.inner.editing_item.get().is_some() {
            self.inner.editing_item.set(None);
        }
    }

    /// Commit an inline rename: write `title` to the item (entity field **and** its
    /// title `Content` row — one title, two homes) and leave edit mode. A blank or
    /// *unchanged* title writes nothing — so Enter-with-no-change and the
    /// Esc→restore→blur cancel path are both quiet no-ops (no stray undo entry).
    pub fn rename(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        let title = title.trim();
        let current = binder_ops::item_dto(&self.inner.app_ctx, id).map(|d| d.title).unwrap_or_default();
        if !title.is_empty() && title != current {
            let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
            probe.set_id(Some(id));
            let _ = probe.set_title(title, self.stack());
        }
        self.inner.editing_item.set(None);
    }

    /// Persist a card's tag ids, from the dot row's picker.
    ///
    /// A transient probe, exactly as `commit_rename` above does: the corkboard keeps no
    /// per-card `SingleBinderItem` (only `container_probe`), and a write is rare enough that
    /// standing one up per card would cost more than it saves.
    pub fn set_card_tags(&self, id: u64, tags: &[u64]) {
        let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
        probe.set_id(Some(id));
        let _ = probe.set_tags(tags, self.stack());
    }

    // ── Shared synopsis documents (one per card, shared with the editor panes) ──

    /// The shared `OpenDoc` for a card's synopsis, opened once and reused while the
    /// container is shown. A realized tile calls this to render its synopsis in a
    /// live editor over the **same** document any editor tab of the item uses — so an
    /// edit on the card, in the expand modal, or in a tab is one edit. `None` if the
    /// item can't be read.
    pub fn synopsis_doc_for(&self, id: u64) -> Option<Rc<OpenDoc>> {
        if let Some(doc) = self.inner.open_synopses.borrow().get(&id) {
            return Some(doc.clone());
        }
        let doc = self.inner.docs.open(id)?;
        self.inner.open_synopses.borrow_mut().insert(id, doc.clone());
        Some(doc)
    }

    /// Flush + release every synopsis doc this corkboard holds. Called when the
    /// container changes (drill in/out) and when the pane is torn down (segment
    /// switch / tab close), so no card's edit is stranded. `OpenDocsStore::release`
    /// flushes on the **last** reference; a doc an editor tab still holds is only
    /// decremented (it owns the flush). Idempotent — a second call finds an empty map.
    pub fn release_all_synopses(&self) {
        let stack = self.stack();
        let ids: Vec<u64> = self
            .inner
            .open_synopses
            .borrow_mut()
            .drain()
            .map(|(id, _)| id)
            .collect();
        for id in ids {
            self.inner.docs.release(id, stack);
        }
    }

    /// Synopsis typography — so the card's editor matches the Full-Synopsis view.
    pub fn synopsis_typo(&self) -> EditorTypography {
        self.inner.synopsis_typo.clone()
    }
    /// Whether this card can be split — only a prose-bearing scene has two halves
    /// to cut. The backend enforces the same rule.
    pub fn can_split(&self, card: &CorkboardCard) -> bool {
        is_prose_bearing(&card.role, &card.sub_role)
    }

    /// Split the item at `caret` in its **synopsis**: the synopsis text before the
    /// caret stays on the source and the rest moves to a new scene; the prose stays
    /// whole on the source. Mirrors [`StreamViewModel::split_row`] for the synopsis
    /// flavour, over the same shared `OpenDoc`.
    pub fn split_synopsis(&self, ctx: &mut EventContext, id: u64, caret: usize) {
        let Some(doc) = self.synopsis_doc_for(id) else {
            return;
        };
        let stack = self.stack();
        let _ = doc.flush(stack);
        let Some(syn) = doc.synopsis.as_ref() else {
            return;
        };
        let Ok((before_synopsis, after_synopsis)) = split_djot(&syn.doc, caret) else {
            return;
        };
        let whole_prose = doc.main.as_ref().map(|m| m.djot()).unwrap_or_default();
        let _ = binder_item_management_commands::split_scene(
            &self.inner.app_ctx,
            stack,
            &SplitSceneDto {
                source_id: id,
                before_text: whole_prose,
                after_text: String::new(),
                before_synopsis,
                after_synopsis,
                new_title: tr!(new_scene_title()).resolve_now(),
            },
        );
        // The source now holds only the before-halves — reflect that in the shared
        // doc, pumping a frame so the queued document events drain.
        doc.reload();
        ctx.request_frame();
    }

    /// The first selected card's item id (selection is index-keyed against the raw
    /// card order), or `None` when nothing is selected.
    fn first_selected_item(&self) -> Option<u64> {
        let cards = self.ordered_cards();
        self.inner
            .selection
            .selected_indices()
            .into_iter()
            .min()
            .and_then(|i| cards.get(i).map(|c| c.item_id))
    }

    // ── Drag-out / foreign receive ───────────────────────────────────────────

    /// Move cards received from another corkboard into the current container.
    pub fn receive_cards(&self, items: &[CorkboardCard]) {
        let item_ids: Vec<u64> = items.iter().map(|c| c.item_id).collect();
        if item_ids.is_empty() {
            return;
        }
        use frontend::binder_item_management::{MoveDto, MovePlace};
        let _ = frontend::commands::binder_item_management_commands::move_items(
            &self.inner.app_ctx,
            self.inner.ids.stack_id.get(),
            &MoveDto {
                item_ids,
                target_id: Some(self.inner.current_container.get()),
                target_is_binder: false,
                move_place: MovePlace::Into,
            },
        );
    }

    // ── Per-card "More actions" (mirrors the Full-Synopsis row menu) ──────────
    //
    // Each action is an undoable backend call, delegating to the same commands
    // `StreamViewModel` uses. `begin_*` presents a modal for text entry; the
    // apply method does the call. Move / merge resolve the card's neighbour from
    // the *current container's* ordered cards, so they read as manuscript order.

    /// A snapshot of the container's cards in order (what the grid shows before
    /// any filter/sort projection) — the basis for move/merge neighbour math.
    fn ordered_cards(&self) -> Vec<CorkboardCard> {
        self.inner.cards.cards()
    }

    fn card_pos(&self, id: u64) -> Option<(Vec<CorkboardCard>, usize)> {
        let cards = self.ordered_cards();
        cards
            .iter()
            .position(|c| c.item_id == id)
            .map(|pos| (cards, pos))
    }

    /// This card can be merged into the one before it: both carry scene prose
    /// and this card does not open a structural section (merging it away would
    /// delete the boundary). The backend enforces the same rule; this hides the
    /// menu item where it would be rejected.
    pub fn can_merge_into_previous(&self, id: u64) -> bool {
        let Some((cards, pos)) = self.card_pos(id) else {
            return false;
        };
        if pos == 0 {
            return false;
        }
        let this = &cards[pos];
        let prev = &cards[pos - 1];
        is_prose_bearing(&this.role, &this.sub_role)
            && !opens_a_section(&this.sub_role)
            && is_prose_bearing(&prev.role, &prev.sub_role)
    }

    // -- dialog entry points --

    pub fn begin_set_label(&self, ctx: &mut EventContext, id: u64) {
        let current = binder_ops::item_dto(&self.inner.app_ctx, id).map(|d| d.label).unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_set_label()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(label) = r {
                    vm.set_label(ctx, id, label.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_insert_after(&self, ctx: &mut EventContext, id: u64) {
        let vm = self.clone();
        InputDialog::new(tr!(dialog_new_scene()))
            .placeholder(tr!(placeholder_scene_name()))
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.insert_after(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    // -- apply methods --

    pub fn set_label(&self, _ctx: &mut EventContext, id: u64, label: &str) {
        if let Some(it) = binder_ops::item_dto(&self.inner.app_ctx, id) {
            let mut dto = update_item_dto(&it);
            dto.label = label.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
        }
    }

    /// Create the model's default recommendation for this card, placed by the
    /// relation it recommends — the same `binder_placement` math the outline and
    /// stream use, so a new item lands identically everywhere.
    pub fn insert_after(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        let Some(card) = self.ordered_cards().into_iter().find(|c| c.item_id == id) else {
            return;
        };
        binder_ops::create_by_recommendation(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            &card.role,
            &card.sub_role,
            title,
        );
    }

    pub fn move_up(&self, _ctx: &mut EventContext, id: u64) {
        if let Some((cards, pos)) = self.card_pos(id)
            && pos > 0
        {
            binder_ops::move_relative(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            cards[pos - 1].item_id,
            MovePlace::Before,
        );
        }
    }

    pub fn move_down(&self, _ctx: &mut EventContext, id: u64) {
        if let Some((cards, pos)) = self.card_pos(id)
            && pos + 1 < cards.len()
        {
            binder_ops::move_relative(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            cards[pos + 1].item_id,
            MovePlace::After,
        );
        }
    }

    /// Merge this card into the previous one. The backend concatenates both the
    /// prose and the synopsis into the survivor. The cards are read-only viewers
    /// (never open editors here), so there is nothing to flush first — the merge
    /// reads current content straight from the store.
    pub fn merge_into_previous(&self, _ctx: &mut EventContext, id: u64) {
        let Some((cards, pos)) = self.card_pos(id) else {
            return;
        };
        if pos == 0 || !self.can_merge_into_previous(id) {
            return;
        }
        let prev_id = cards[pos - 1].item_id;
        let _ = binder_item_management_commands::merge_two_scenes(
            &self.inner.app_ctx,
            self.stack(),
            &MergeTwoScenesDto {
                target_id: prev_id,
                source_id: id,
            },
        );
    }

    pub fn trash(&self, _ctx: &mut EventContext, id: u64) {
        if let Some((binder, _order, _pos)) = binder_ops::locate(&self.inner.app_ctx, &self.inner.ids, id) {
            let _ = trash_management_commands::trash_binder_items(
                &self.inner.app_ctx,
                self.stack(),
                &TrashBinderItemsDto {
                    binder_item_ids: vec![id as i64],
                    origin_binder_id: binder as i64,
                },
            );
        }
    }

    // -- backend helpers --
    //
    // The binder round-trips this used to carry (`chapter_mode`, `item_dto`, `locate`,
    // `item_meta`, `create_by_recommendation`, `move_relative`) now live in
    // `view_models::binder_ops`, shared with the stream and the outline.

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    // ── Accessors for the view ───────────────────────────────────────────────

    pub fn cards_model(&self) -> CorkboardCardsModel {
        self.inner.cards.clone()
    }
    pub fn projection(&self) -> SortFilterListModel<CorkboardCard> {
        self.inner.projection.clone()
    }
    /// Snapshot of the current cards (for resolving an activation index → card).
    pub fn cards(&self) -> Vec<CorkboardCard> {
        self.inner.cards.cards()
    }
    pub fn is_projecting(&self) -> Signal<bool> {
        self.inner.projecting.clone()
    }
    pub fn selection(&self) -> SelectionModel {
        self.inner.selection.clone()
    }
    pub fn scroll_y(&self) -> Signal<f32> {
        self.inner.scroll_y.clone()
    }
    pub fn trail_signal(&self) -> Signal<Vec<(u64, String)>> {
        self.inner.trail.clone()
    }
    pub fn container_title(&self) -> Signal<String> {
        self.inner.container_probe.title()
    }
    pub fn search_query_signal(&self) -> Signal<String> {
        self.inner.query.clone()
    }
    pub fn sort_signal(&self) -> Signal<Option<(String, SortDirection)>> {
        self.inner.sort.clone()
    }
    pub fn count_signal(&self) -> Signal<usize> {
        self.inner.count.clone()
    }
    pub fn nested(&self) -> Signal<bool> {
        self.inner.nested.clone()
    }
    pub fn card_size(&self) -> Signal<f32> {
        self.inner.card_size.clone()
    }
    pub fn show_word_count(&self) -> Signal<bool> {
        self.inner.show_word_count.clone()
    }
    pub fn counting_method(&self) -> Signal<CountingMethodSetting> {
        self.inner.counting_method.clone()
    }
    /// The app context — the tile delegate needs it to build a per-card single.
    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.inner.app_ctx.clone()
    }
    /// The recommendations' anchor (current container), for the create button's title.
    pub fn current_container(&self) -> u64 {
        self.inner.current_container.get()
    }

    // ── internals ─────────────────────────────────────────────────────────────

    fn recompute_projecting(&self) {
        let projecting =
            !self.inner.query.get().trim().is_empty() || self.inner.sort.get().is_some();
        if self.inner.projecting.get() != projecting {
            self.inner.projecting.set(projecting);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typo() -> EditorTypography {
        EditorTypography {
            font_family: Signal::new(String::new()),
            size: Signal::new(16.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        }
    }

    fn vm() -> CorkboardViewModel {
        // An empty AppContext: the model/probe reads find nothing (graceful) — the
        // navigation logic under test manipulates only signals.
        let ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let docs = OpenDocsStore::new(ctx.clone());
        CorkboardViewModel::new(
            ctx,
            ids,
            docs,
            100,
            Signal::new(true),
            Signal::new(200.0),
            Signal::new(true),
            Signal::new(CountingMethodSetting::default()),
            typo(),
        )
    }

    #[test]
    fn starts_at_the_tab_container() {
        let vm = vm();
        assert_eq!(vm.current_container(), 100);
        let trail = vm.trail_signal().get();
        assert_eq!(trail.len(), 1);
        assert_eq!(trail[0].0, 100);
    }

    #[test]
    fn drill_in_deepens_the_trail_and_rescopes() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        assert_eq!(vm.current_container(), 200);
        let trail = vm.trail_signal().get();
        assert_eq!(trail.len(), 2);
        assert_eq!(trail[1], (200, "Part Two".to_string()));

        vm.drill_into(300, "Chapter Five".to_string());
        assert_eq!(vm.current_container(), 300);
        assert_eq!(vm.trail_signal().get().len(), 3);
    }

    #[test]
    fn crumb_navigation_truncates_back_to_an_ancestor() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        vm.drill_into(300, "Chapter Five".to_string());
        // Jump back to the root crumb.
        vm.go_to_crumb(0);
        assert_eq!(vm.current_container(), 100);
        assert_eq!(vm.trail_signal().get().len(), 1);
    }

    #[test]
    fn crumb_navigation_to_the_current_level_is_a_noop() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        vm.go_to_crumb(1); // already here
        assert_eq!(vm.current_container(), 200);
        assert_eq!(vm.trail_signal().get().len(), 2);
    }
}
