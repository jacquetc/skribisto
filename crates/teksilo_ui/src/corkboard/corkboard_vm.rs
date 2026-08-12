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

use teksilo::core::ObserverHandle;
use teksilo::data::{
    ListDataSource, SelectionMode, SelectionModel, SortDirection, SortFilterListModel,
};
use teksilo::prelude::*; // Signal, BuildContext, EventContext
use teksilo::widgets::InputDialog;

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, MovePlace, SplitSceneDto};
use frontend::commands::{
    binder_item_commands, binder_item_management_commands, trash_management_commands,
    undo_redo_commands,
};
use frontend::trash_management::TrashBinderItemsDto;

use skribisto_model::counting::CountingMethodSetting;
use skribisto_model::{CreateType, Recommendation, Relation};

use crate::app_ids::AppIds;
use crate::intents::AppIntent;
use crate::models::{CorkboardCard, CorkboardCardsModel, OpenDoc, OpenDocsStore};
use crate::singles::SingleBinderItem;
use crate::view_models::{EditorTypography, FormatViewModel};

use crate::shared::binder_ops::{
    self, is_prose_bearing, opens_a_section, split_djot, update_item_dto,
};

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
    show_card_numbers: Signal<bool>,
    /// The expanded-synopsis editor's own font-size scale.
    modal_size: Signal<f32>,
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
    caret_band: crate::view_models::CaretBand,
    /// The writing games this project is playing — a card's synopsis editor is
    /// a synopsis surface like any other, so it follows the same option.
    writing_games: crate::view_models::WritingGamesViewModel,
    /// This window's Format surfaces — card synopsis editors register with it.
    format: FormatViewModel,

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
        show_card_numbers: Signal<bool>,
        modal_size: Signal<f32>,
        counting_method: Signal<CountingMethodSetting>,
        synopsis_typo: EditorTypography,
        caret_band: crate::view_models::CaretBand,
        writing_games: crate::view_models::WritingGamesViewModel,
        format: FormatViewModel,
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
                show_card_numbers,
                modal_size,
                counting_method,
                selection: SelectionModel::new(SelectionMode::Multi),
                scroll_y: Signal::new(0.0),
                editing_item: Signal::new(None),
                grid_id: Signal::new(None),
                open_synopses: RefCell::new(HashMap::new()),
                docs,
                synopsis_typo,
                caret_band,
                writing_games,
                format,
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
        // Release the shared synopsis doc of any card that leaves the board — merged
        // away, trashed, promoted out of scope, undone, or removed by another window.
        // `Weak`, never `Rc`: the model is owned by this `Inner`, so a strong capture
        // would close a cycle and leak every synopsis the board ever opened (see
        // `CorkboardCardsModel::wire`).
        let weak = Rc::downgrade(&self.inner);
        self.inner.cards.wire(ctx, move |removed: &[u64]| {
            let Some(inner) = weak.upgrade() else {
                return;
            };
            let stack = inner.ids.stack_id.get();
            // Drop the handles *before* releasing, so the map's borrow is not held
            // across a `release` that flushes (and can re-enter the store).
            let gone: Vec<u64> = {
                let mut held = inner.open_synopses.borrow_mut();
                removed
                    .iter()
                    .filter(|id| held.remove(id).is_some())
                    .copied()
                    .collect()
            };
            for id in gone {
                inner.docs.release(id, stack);
            }
        });
        self.inner.container_probe.wire(ctx);

        // Search text → the projection's filter (its filters_signal is observe-only,
        // so push imperatively) + recompute `projecting`.
        {
            let me = self.clone();
            ctx.effect(&self.inner.query, move |q| {
                me.drop_selection();
                me.inner.projection.set_filter("text", q);
                me.recompute_projecting();
            });
        }
        // Sort selection → the projection's sort + recompute `projecting`.
        {
            let me = self.clone();
            ctx.effect(&self.inner.sort, move |s| {
                me.drop_selection();
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

    // ── Persisted board navigation ───────────────────────────────────────────

    /// The drilled-into trail as store ids, root-first and root-inclusive — what
    /// the workspace layout persists (translated to durable uids by its caller,
    /// since an `EntityId` is re-minted on every `load_work`).
    ///
    /// Returns empty when the board sits at its tab's own container, so nothing is
    /// written for the overwhelmingly common case.
    pub fn trail_ids(&self) -> Vec<u64> {
        let trail = self.inner.trail.get();
        if trail.len() <= 1 {
            return Vec::new();
        }
        trail.into_iter().map(|(id, _title)| id).collect()
    }

    /// Restore a persisted trail (store ids, root-first) and filter text.
    ///
    /// The trail is trusted only as far as it is still *true*. The caller has
    /// already dropped uids that no longer name a live item; this additionally
    /// re-walks the chain and truncates at the first crumb that is no longer inside
    /// its predecessor. Existence is not containment: between two sessions the
    /// writer can move a chapter out of the Part it sat under, and every uid would
    /// still resolve while the chain has stopped being an ancestor path. Restoring
    /// it unchecked would seat the board on a container that is not under this
    /// tab's own, and offer crumbs that navigate somewhere they never came from.
    ///
    /// A trail that truncates to one entry leaves the board at its own container.
    pub fn restore_trail(&self, ids: &[u64], titles: &[String], query: &str) {
        if !query.is_empty() {
            self.inner.query.set(query.to_string());
        }
        if ids.len() <= 1 {
            return;
        }
        // The root crumb must still be this tab's own container — the trail is
        // always rooted there, and a mismatch means the persisted row belongs to
        // some other container entirely.
        let root = self.inner.trail.get().first().map(|(id, _)| *id);
        if root != ids.first().copied() {
            return;
        }
        let mut trail: Vec<(u64, String)> = vec![(ids[0], titles[0].clone())];
        for (id, title) in ids[1..].iter().copied().zip(titles[1..].iter().cloned()) {
            let parent = trail.last().map(|(p, _)| *p).unwrap_or(id);
            if !binder_ops::subtree_contains(&self.inner.app_ctx, &self.inner.ids, parent, id) {
                break; // this crumb left its parent's subtree — stop here
            }
            trail.push((id, title));
        }
        if trail.len() <= 1 {
            return; // nothing survived past the root
        }
        let deepest = trail[trail.len() - 1].0;
        self.inner.trail.set(trail);
        // Not `enter`: that clears the query, and the writer's filter is being
        // restored alongside the trail here.
        self.inner.current_container.set(deepest);
        self.inner.container_probe.set_id(Some(deepest));
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

    /// Select this card in the binder outline and reveal the dock — "where does this
    /// sit in the project?", the question a drilled-into board makes easy to lose.
    /// Goes over the intent bus rather than importing the outline, exactly as
    /// [`OverviewViewModel::reveal_in_outline`](crate::overview::OverviewViewModel) does.
    pub fn reveal_in_outline(&self, ctx: &mut EventContext, id: u64) {
        ctx.send_intent(AppIntent::RevealInOutline { item_id: id });
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
        let current = binder_ops::item_dto(&self.inner.app_ctx, id)
            .map(|d| d.title)
            .unwrap_or_default();
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
        self.inner
            .open_synopses
            .borrow_mut()
            .insert(id, doc.clone());
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

    /// The **expanded** synopsis editor's typography: the card's, with its own
    /// font-size scale substituted.
    ///
    /// Everything else — face, line height, indents, paragraph spacing — stays
    /// shared, because the expanded editor is the same prose in a roomier box, not
    /// a different surface. Only the size differs, and only because the card's is
    /// chosen to be scannable in a tile while this one is chosen to be written in.
    pub fn modal_typo(&self) -> EditorTypography {
        EditorTypography {
            size: self.inner.modal_size.clone(),
            ..self.inner.synopsis_typo.clone()
        }
    }

    /// The ambient caret band for card synopsis editors.
    pub fn caret_band(&self) -> crate::view_models::CaretBand {
        self.inner.caret_band.clone()
    }

    /// This window's Format surfaces for card synopsis editors.
    pub fn format(&self) -> FormatViewModel {
        self.inner.format.clone()
    }

    /// The writing games this project is playing, for card synopsis editors.
    pub fn writing_games(&self) -> crate::view_models::WritingGamesViewModel {
        self.inner.writing_games.clone()
    }
    /// Whether this card can be split — only a prose-bearing scene has two halves
    /// to cut. The backend enforces the same rule.
    pub fn can_split(&self, card: &CorkboardCard) -> bool {
        is_prose_bearing(&card.role, &card.sub_role)
    }

    /// Split the item at `caret` in its **synopsis**: the synopsis text before the
    /// caret stays on the source and the rest moves to a new scene; the prose stays
    /// whole on the source. Mirrors [`crate::stream::StreamViewModel::split_row`] for the synopsis
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

    // ── Selection ────────────────────────────────────────────────────────────

    /// The cards the grid is *currently showing*, in the order it shows them —
    /// the projection while a search/sort is active, the raw model otherwise.
    ///
    /// Every selection lookup must go through this, not [`Self::ordered_cards`]:
    /// `SelectionModel` is **positional**, and `CorkboardGrid` binds the projection
    /// whenever [`Self::is_projecting`] is true, so an index means a position in the
    /// *filtered* list. Resolving it against the raw list silently acts on a
    /// different card — the further down the board, the further off.
    fn visible_cards(&self) -> Vec<CorkboardCard> {
        if self.inner.projecting.get() {
            let p = &self.inner.projection;
            (0..p.len())
                .filter_map(|i| p.with_item(i, |c| c.clone()))
                .collect()
        } else {
            self.ordered_cards()
        }
    }

    /// The selected cards' item ids, in board order. Empty when nothing is selected.
    pub fn selected_item_ids(&self) -> Vec<u64> {
        let cards = self.visible_cards();
        self.inner
            .selection
            .selected_indices()
            .into_iter()
            .filter_map(|i| cards.get(i).map(|c| c.item_id))
            .collect()
    }

    /// The first selected card's item id, or `None` when nothing is selected.
    fn first_selected_item(&self) -> Option<u64> {
        let cards = self.visible_cards();
        self.inner
            .selection
            .selected_indices()
            .into_iter()
            .min()
            .and_then(|i| cards.get(i).map(|c| c.item_id))
    }

    /// What a per-card action should act on: the whole selection when `id` is part
    /// of it, otherwise just `id`.
    ///
    /// The convention every multi-select surface in the app shares (see
    /// [`OverviewViewModel::batch_for`](crate::overview::OverviewViewModel)): opening a card's
    /// menu inside a selection acts on all of it, opening one outside acts on that
    /// card alone — and neither *changes* the selection, which would destroy the
    /// menu's own anchor.
    pub fn batch_for(&self, id: u64) -> Vec<u64> {
        let selected = self.selected_item_ids();
        if selected.contains(&id) {
            selected
        } else {
            vec![id]
        }
    }

    /// How many cards a per-card action would apply to — for menu labels that say
    /// "Delete 4 cards" rather than a bare "Delete".
    pub fn batch_len(&self, id: u64) -> usize {
        self.batch_for(id).len()
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

    /// Set the status label on `id`'s batch (see [`Self::batch_for`]).
    ///
    /// The field is seeded from the anchor card's current label; with a mixed
    /// selection that is the value the writer clicked on, which is the only one
    /// they can be said to have chosen.
    pub fn begin_set_label(&self, ctx: &mut EventContext, id: u64) {
        let targets = self.batch_for(id);
        let current = binder_ops::item_dto(&self.inner.app_ctx, id)
            .map(|d| d.label)
            .unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_set_label()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(label) = r {
                    vm.set_label_many(ctx, &targets, label.trim());
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

    pub fn set_label(&self, ctx: &mut EventContext, id: u64, label: &str) {
        self.set_label_many(ctx, &[id], label);
    }

    /// Write `label` to every id, as **one** undo entry when there is more than one
    /// — a batch the writer performed as a single gesture must undo as one too.
    pub fn set_label_many(&self, _ctx: &mut EventContext, ids: &[u64], label: &str) {
        if ids.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = ids.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(&self.inner.app_ctx, stack);
        }
        for id in ids {
            if let Some(it) = binder_ops::item_dto(&self.inner.app_ctx, *id) {
                let mut dto = update_item_dto(&it);
                dto.label = label.to_string();
                let _ = binder_item_commands::update_binder_item(&self.inner.app_ctx, stack, &dto);
            }
        }
        if composite {
            undo_redo_commands::end_composite(&self.inner.app_ctx);
        }
    }

    /// Duplicate `ids` (one backend call — the use case already takes a list, and
    /// pushes a single undo entry for the whole set).
    ///
    /// Silently does nothing when `ids` is empty, matching
    /// [`OutlineViewModel::duplicate_keys`](crate::binder::OutlineViewModel) — the established
    /// convention for this operation rather than a new failure channel.
    pub fn duplicate_many(&self, ids: &[u64]) {
        if ids.is_empty() {
            return;
        }
        use frontend::binder_item_management::DuplicateDto;
        let _ = binder_item_management_commands::duplicate(
            &self.inner.app_ctx,
            self.stack(),
            &DuplicateDto {
                item_ids: ids.to_vec(),
            },
        );
    }

    /// Move `ids` **into** `target` (a container), as one backend call — the
    /// drop-onto-a-container-card gesture. `true` when anything actually moved.
    pub fn move_many_into(&self, ids: &[u64], target: u64) -> bool {
        self.move_many_to(ids, target, false, MovePlace::Into) > 0
    }

    /// Move `ids` to `target`, as one backend call and one undo entry.
    ///
    /// Refuses a move that would put a container inside itself or inside its own
    /// subtree: containment here is positional (a subtree is the item plus every
    /// following item of greater indent), so such a move does **not** fail loudly —
    /// it writes an ordering the binder can never represent. A binder target is
    /// never self-containing, so the guard only applies to item targets.
    /// Returns how many items actually moved — `0` when the move was refused, so a
    /// caller's "N cards moved" can report what happened rather than what was asked
    /// (a self-target is filtered out of `ids` before the call).
    pub fn move_many_to(
        &self,
        ids: &[u64],
        target: u64,
        target_is_binder: bool,
        move_place: MovePlace,
    ) -> usize {
        let ids: Vec<u64> = ids.iter().copied().filter(|id| *id != target).collect();
        if ids.is_empty() || (!target_is_binder && !self.can_move_into(&ids, target)) {
            return 0;
        }
        let moved = ids.len();
        use frontend::binder_item_management::MoveDto;
        let ok = binder_item_management_commands::move_items(
            &self.inner.app_ctx,
            self.stack(),
            &MoveDto {
                item_ids: ids,
                target_id: Some(target),
                target_is_binder,
                move_place,
            },
        )
        .is_ok();
        if ok { moved } else { 0 }
    }

    /// Whether every id may legally move into `target`: `target` must not be one of
    /// them, nor lie inside any of their subtrees.
    pub fn can_move_into(&self, ids: &[u64], target: u64) -> bool {
        if ids.contains(&target) {
            return false;
        }
        !ids.iter().any(|id| {
            binder_ops::subtree_contains(&self.inner.app_ctx, &self.inner.ids, *id, target)
        })
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
    /// prose and the synopsis into the survivor, reading them **from the store** —
    /// so both cards' shared `OpenDoc`s must be flushed first, exactly as
    /// [`StreamViewModel::merge_into_previous`](crate::stream::StreamViewModel) does.
    ///
    /// A card's synopsis is a *live* editor over the same `OpenDoc` an editor tab
    /// uses (see [`Self::synopsis_doc_for`]), so a synopsis typed on the board and
    /// not yet autosaved is real, unflushed state — merging without flushing it
    /// silently drops the writer's words. Afterwards the survivor has absorbed the
    /// source's prose *and* synopsis, so it is reloaded and a frame pumped to drain
    /// the queued document events into any editor already on screen; skipping that
    /// leaves a stale buffer whose next flush would overwrite the merged text.
    pub fn merge_into_previous(&self, ctx: &mut EventContext, id: u64) {
        let Some((cards, pos)) = self.card_pos(id) else {
            return;
        };
        if pos == 0 || !self.can_merge_into_previous(id) {
            return;
        }
        let prev_id = cards[pos - 1].item_id;
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };

        // Flush both sides before the merge reads the store. `synopsis_doc_for`
        // returns the already-open doc when the card is realized, and opens it
        // otherwise — either way the flush covers unsaved card edits.
        let stack = self.stack();
        let prev_doc = self.synopsis_doc_for(prev_id);
        let cur_doc = self.synopsis_doc_for(id);
        if let Some(d) = prev_doc.as_ref() {
            let _ = d.flush(stack);
        }
        if let Some(d) = cur_doc.as_ref() {
            let _ = d.flush(stack);
        }

        let merged = binder_item_management_commands::merge_two_scenes(
            &self.inner.app_ctx,
            stack,
            &MergeTwoScenesDto {
                work_id,
                target_id: prev_id,
                source_id: id,
            },
        )
        .is_ok();
        if !merged {
            return;
        }

        // The survivor absorbed both roles — reflect that in the shared doc so the
        // card (and any editor tab on it) shows the merged text instead of a stale
        // buffer. `set_djot` only queues a document event, hence the frame pump.
        // The *source*'s doc is released by the model's `on_removed` callback when
        // the merge event lands and the card leaves the board (see `wire`).
        if let Some(d) = prev_doc.as_ref() {
            d.reload();
        }
        ctx.request_frame();
    }

    pub fn trash(&self, ctx: &mut EventContext, id: u64) {
        self.trash_many(ctx, &[id]);
    }

    /// Send every id to the trash, as **one** undo entry.
    ///
    /// `trash_binder_items` is scoped to a single origin binder, so the ids are
    /// grouped by the binder that owns each — a `Work` may hold several, and one
    /// flat call with a mixed list would file items under the wrong origin and
    /// restore them to the wrong place. More than one call is wrapped in a
    /// composite so Ctrl+Z takes the whole gesture back at once.
    ///
    /// Deliberately **no undo toast**: that pattern belongs to the irreversible
    /// trash operations (Empty Trash / Delete Forever), and trashing is an ordinary
    /// undoable move whose safety net is the undo stack — see
    /// [`TrashViewModel::run_with_undo_toast`](crate::trash::TrashViewModel).
    pub fn trash_many(&self, _ctx: &mut EventContext, ids: &[u64]) {
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };
        let mut by_binder: HashMap<u64, Vec<i64>> = HashMap::new();
        for id in ids {
            if let Some((binder, _order, _pos)) =
                binder_ops::locate(&self.inner.app_ctx, &self.inner.ids, *id)
            {
                by_binder.entry(binder).or_default().push(*id as i64);
            }
        }
        if by_binder.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = by_binder.len() > 1 || by_binder.values().map(Vec::len).sum::<usize>() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(&self.inner.app_ctx, stack);
        }
        for (binder, binder_item_ids) in by_binder {
            let _ = trash_management_commands::trash_binder_items(
                &self.inner.app_ctx,
                stack,
                &TrashBinderItemsDto {
                    work_id,
                    binder_item_ids,
                    origin_binder_id: binder as i64,
                },
            );
        }
        if composite {
            undo_redo_commands::end_composite(&self.inner.app_ctx);
        }
        // The board's own selection is positional; the cards it pointed at are gone.
        self.inner.selection.clear();
    }

    /// Delete-key entry point: trash whatever is selected. A no-op with an empty
    /// selection, so the key is inert rather than surprising.
    pub fn trash_selected(&self, ctx: &mut EventContext) {
        let ids = self.selected_item_ids();
        if !ids.is_empty() {
            self.trash_many(ctx, &ids);
        }
    }

    // -- backend helpers --
    //
    // The binder round-trips (`chapter_mode`, `item_dto`, `locate`, `item_meta`,
    // `create_by_recommendation`, `move_relative`) live in `shared::binder_ops`,
    // shared with the stream and the outline.

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
    /// Number the cards in board order (1-based).
    pub fn show_card_numbers(&self) -> Signal<bool> {
        self.inner.show_card_numbers.clone()
    }
    pub fn counting_method(&self) -> Signal<CountingMethodSetting> {
        self.inner.counting_method.clone()
    }
    /// The app context — the tile delegate needs it to build a per-card single.
    /// This board's `Work` id — for the destination picker's own tree model and for
    /// scoping its toasts to this Work's window.
    pub fn work_id(&self) -> Signal<Option<u64>> {
        self.inner.ids.work_id.clone()
    }

    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.inner.app_ctx.clone()
    }
    /// The recommendations' anchor (current container), for the create button's title.
    pub fn current_container(&self) -> u64 {
        self.inner.current_container.get()
    }

    // ── internals ─────────────────────────────────────────────────────────────

    /// Drop the selection because the list it indexes into is about to be
    /// re-derived.
    ///
    /// `SelectionModel` is **positional**, and every filter keystroke, sort change
    /// and raw↔projection swap rebuilds the bound list under it. The framework's
    /// own `Reset` handling cannot save us here: `set_filter`/`set_sort` notify
    /// synchronously, to whoever is subscribed *at that instant* — which is still
    /// the grid bound to the **old** source, since `projecting` has not flipped
    /// yet — and there is no catch-up delivery to the grid that subscribes after
    /// the rebuild. So the indices survive into a differently-ordered list and
    /// quietly come to mean different cards.
    ///
    /// That is only cosmetic for the highlight; it is not cosmetic for
    /// [`Self::selected_item_ids`], which every bulk action resolves through —
    /// Delete/Backspace, "Delete N cards", Duplicate, Move to…, Set label. Clearing
    /// is the honest option: a positional selection has no meaning across a
    /// re-derivation, and silently acting on whatever now sits at those positions
    /// is the failure this exists to remove.
    fn drop_selection(&self) {
        if !self.inner.selection.selected_indices().is_empty() {
            self.inner.selection.clear();
        }
    }

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
            Signal::new(false),
            Signal::new(1.0),
            Signal::new(CountingMethodSetting::default()),
            typo(),
            crate::view_models::CaretBand::off(),
            crate::view_models::WritingGamesViewModel::detached(),
            FormatViewModel::detached(),
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

    // ── Persisted navigation (P2.2) ──────────────────────────────────────────

    /// A board at its own container persists nothing — the common case must not
    /// write a trail to `workspace.toml`.
    #[test]
    fn an_undrilled_board_has_no_trail_to_persist() {
        let vm = vm();
        assert!(vm.trail_ids().is_empty());
    }

    #[test]
    fn a_drilled_board_persists_the_whole_trail_root_first() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        vm.drill_into(300, "Chapter Five".to_string());
        assert_eq!(
            vm.trail_ids(),
            vec![100, 200, 300],
            "root-inclusive and root-first — the breadcrumb, exactly"
        );
    }

    /// A restored trail is re-walked for containment, and truncates at the first
    /// crumb that is no longer inside its predecessor.
    ///
    /// These view-model tests run against an empty `AppContext`, where no crumb can
    /// be *shown* to be inside another — so the conservative branch is what is
    /// exercised here: the chain collapses to its root and the board stays put. That
    /// is the intended behaviour for an unverifiable chain (a real restore runs after
    /// `load_work`, with the binder live). The query still comes back either way,
    /// which is the part a writer would otherwise have to retype.
    #[test]
    fn an_unverifiable_trail_collapses_to_its_root_but_keeps_the_query() {
        let vm = vm();
        vm.restore_trail(
            &[100, 200, 300],
            &["Book".into(), "Part Two".into(), "Chapter Five".into()],
            "ferry",
        );
        assert_eq!(
            vm.current_container(),
            100,
            "no containment could be proven, so nothing past the root is trusted"
        );
        assert_eq!(vm.trail_signal().get().len(), 1);
        assert_eq!(vm.search_query_signal().get(), "ferry");
    }

    /// A persisted trail whose root is not this tab's own container is ignored
    /// outright — it describes some other container's board.
    #[test]
    fn a_trail_rooted_elsewhere_is_ignored() {
        let vm = vm();
        vm.restore_trail(&[999, 200], &["Elsewhere".into(), "Part Two".into()], "");
        assert_eq!(vm.current_container(), 100);
        assert_eq!(vm.trail_signal().get().len(), 1);
    }

    /// A single-entry (or empty) trail means "never drilled" — restoring it must
    /// leave the board at its own container rather than rewriting the breadcrumb.
    #[test]
    fn restoring_a_degenerate_trail_leaves_the_board_alone() {
        let vm = vm();
        vm.restore_trail(&[100], &["Book".into()], "");
        assert_eq!(vm.current_container(), 100);
        assert_eq!(vm.trail_signal().get().len(), 1);

        vm.restore_trail(&[], &[], "");
        assert_eq!(vm.current_container(), 100);
        assert_eq!(vm.trail_signal().get().len(), 1);
    }

    /// Restoring only a query (no trail) still filters — a board can be left
    /// filtered without ever having been drilled.
    #[test]
    fn restoring_only_a_query_still_filters() {
        let vm = vm();
        vm.restore_trail(&[], &[], "ferry");
        assert_eq!(vm.search_query_signal().get(), "ferry");
        assert_eq!(vm.current_container(), 100, "and does not navigate");
    }

    // ── Batch targeting (P1.1) ───────────────────────────────────────────────

    /// With nothing selected, a card's menu acts on that card alone.
    #[test]
    fn batch_for_an_unselected_card_is_just_that_card() {
        let vm = vm();
        assert_eq!(vm.batch_for(42), vec![42]);
        assert_eq!(vm.batch_len(42), 1);
    }

    /// `can_move_into` refuses the degenerate self-move without needing a store —
    /// the guard that keeps a drop onto a container card from eating itself.
    #[test]
    fn a_container_can_never_move_into_itself() {
        let vm = vm();
        assert!(
            !vm.can_move_into(&[7, 8], 7),
            "the target is one of the moved"
        );
        assert!(
            vm.can_move_into(&[7, 8], 9),
            "an unrelated target is allowed (no store here, so no subtree to consult)"
        );
    }

    /// `move_many_into` is a no-op for an empty set and for a self-move, and never
    /// reaches the backend for either.
    #[test]
    fn move_many_into_rejects_empty_and_self_moves() {
        let vm = vm();
        assert!(!vm.move_many_into(&[], 9));
        assert!(
            !vm.move_many_into(&[7], 7),
            "a card dropped on itself filters down to an empty set"
        );
    }

    /// Changing the filter or the sort **drops the selection**.
    ///
    /// `SelectionModel` is positional and the bound list is re-derived underneath
    /// it, so surviving indices would come to mean different cards — and every bulk
    /// action (Delete/Backspace, "Delete N cards", Duplicate, Move to…, Set label)
    /// resolves through those indices. This is the guard that stops a filter
    /// keystroke from silently re-aiming a destructive action.
    #[test]
    fn changing_the_filter_or_sort_drops_the_selection() {
        for (label, mutate) in [
            (
                "query",
                Box::new(|vm: &CorkboardViewModel| vm.search_query_signal().set("ferry".into()))
                    as Box<dyn Fn(&CorkboardViewModel)>,
            ),
            (
                "sort",
                Box::new(|vm: &CorkboardViewModel| {
                    vm.sort_signal()
                        .set(Some((SORT_TITLE.to_string(), SortDirection::Ascending)))
                }),
            ),
        ] {
            let vm = vm();
            let mut tree = crate::test_support::tree_with_events(&vm.app_ctx());
            let id = tree.add_boxed(Box::new(crate::tabs::corkboard::WireCorkboard {
                vm: vm.clone(),
            }));
            tree.layout(teksilo::prelude::SizeProposal::exact(800.0, 600.0));
            let _ = id;

            vm.selection().select(1);
            vm.selection().toggle(2);
            assert!(
                !vm.selection().selected_indices().is_empty(),
                "{label}: precondition — something is selected"
            );

            mutate(&vm);
            tree.layout(teksilo::prelude::SizeProposal::exact(800.0, 600.0));

            assert!(
                vm.selection().selected_indices().is_empty(),
                "{label}: the selection must not survive a re-derivation of the list"
            );
        }
    }
}
