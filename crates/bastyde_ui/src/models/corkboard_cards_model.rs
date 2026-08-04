// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive, ordered list of the **cards** a corkboard shows for one container —
//! the data behind the Corkboard segment of a Book / Part / Chapter-folder tab.
//!
//! Two scopes, driven by the view's `nested` toggle:
//! - **Nested** — the container's *direct children* (the `indent == base + 1`
//!   window over the binder's flat item stream). A folder child is a card the
//!   writer can drill into; a leaf child opens in the editor.
//! - **Flat** — every descendant of the container (leaves *and* containers — a
//!   Part / Chapter / folder carries a synopsis too), depth-first in reading order:
//!   the whole subtree flattened to one card list.
//!
//! Built the same "framework way" the Outline tree is (`bastyde::data`): the
//! model owns a [`ListModel<CorkboardCard>`] synced to the Qleany backend via the
//! framework's keyed [`reconcile_by_key`](bastyde::data::ListModel::reconcile_by_key)
//! (insert / remove / move / in-place update in one diff — no hand-rolled
//! reconcile), and exposes a [`SortFilterListModel`] projection that gives the
//! corkboard a **live search filter and column sort for free**. It implements
//! [`ListDataSource`] itself, so a `GridView::from_source` renders it and a
//! drag-reorder commits straight to the backend `move_items` use case.
//!
//! Only the **cheap** fields are baked into a card (title, label, type, and a
//! folder's direct-child count — all from one `get_binder_item_multi`). The
//! expensive per-card data (synopsis excerpt, leaf word count) is resolved lazily
//! per *visible* tile by [`SingleCorkboardCard`](crate::singles::SingleCorkboardCard),
//! so a long container's board stays as cheap as one viewport.

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

/// One card. Carries only the fields that are free to compute in bulk — the
/// title/label also power the free text filter, and `(role, sub_role)` drive the
/// tile chrome (icon, badge) and the drill-vs-open decision.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CorkboardCard {
    pub item_id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub title: String,
    /// The user-written note under the title (`BinderItem.label`) — the "status"
    /// pill, and part of the searchable text.
    pub label: String,
    /// `role == Folder` — a card the writer can drill into (nested mode).
    pub is_container: bool,
    /// Direct child count, for a container card's footer ("N items"). `0` for leaves.
    pub child_count: usize,
    /// The item's tag ids, for the dot row. Baked rather than probed per card: this model
    /// already refetches on `BinderItem(Updated)` (unlike the stream's), so the ids stay
    /// live for free and a card needs no `SingleBinderItem` of its own.
    pub tags: Vec<u64>,
}

/// Whether a card matches the corkboard's text filter — a case-insensitive
/// substring of the title or the status label. The single predicate both the
/// real and the mock projection register, so the filter behaves identically and
/// is unit-testable without a backend.
pub(crate) fn card_text_matches(card: &CorkboardCard, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return true;
    }
    card.title.to_lowercase().contains(&q) || card.label.to_lowercase().contains(&q)
}

#[cfg(test)]
mod filter_tests {
    use super::*;
    use bastyde::data::{ListDataSource, ListModel, SortFilterListModel};

    fn card(id: u64, title: &str) -> CorkboardCard {
        CorkboardCard {
            item_id: id,
            title: title.to_string(),
            ..Default::default()
        }
    }

    /// The projection's search filter (the "free" corkboard search) narrows the
    /// visible cards by title, and restores them when cleared.
    #[test]
    fn text_filter_narrows_and_restores() {
        let model = ListModel::from_vec(vec![
            card(1, "Into the Dark"),
            card(2, "The light returns"),
            card(3, "First night"),
        ]);
        let proj = SortFilterListModel::from_source(model).with_predicate("text", |q| {
            let q = q.to_string();
            Box::new(move |c: &CorkboardCard| card_text_matches(c, &q))
        });
        assert_eq!(proj.len(), 3);
        proj.set_filter("text", "light");
        assert_eq!(proj.len(), 1, "only 'The light returns' matches 'light'");
        proj.set_filter("text", "night");
        assert_eq!(proj.len(), 1, "only 'First night' matches 'night'");
        proj.set_filter("text", "");
        assert_eq!(proj.len(), 3, "clearing the filter restores every card");
    }

    #[test]
    fn empty_query_matches_all() {
        assert!(card_text_matches(&card(1, "Scene"), ""));
        assert!(card_text_matches(&card(1, "Scene"), "   "));
    }
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    use bastyde::core::ObserverHandle;
    use bastyde::data::{
        DragEligibility, DragSource, DropPosition, DropQuery, DropResponse, ListDataSource,
        ListModel, SortFilterListModel,
    };
    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::binder_item_management::{MoveDto, MovePlace};
    use frontend::commands::{
        binder_commands, binder_item_commands, binder_item_management_commands, work_commands,
    };
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::BinderItemRole;
    use frontend::common::event::{
        BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin,
        TrashManagementEvent, WorkManagementEvent,
    };
    use frontend::direct_access::BinderItemDto;
    use skribisto_model::SubRoleExt;

    use super::CorkboardCard;

    struct Inner {
        model: ListModel<CorkboardCard>,
        ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        stack_id: Signal<Option<u64>>,
        /// The container currently shown — changes as the writer drills in/out.
        container_id: Signal<u64>,
        /// `true` = direct children (drillable); `false` = all leaf descendants.
        nested: Signal<bool>,
        /// Invoked with the ids that left the board on each refresh, so the owner
        /// can release their shared synopsis documents. See [`CorkboardCardsModel::wire`].
        #[allow(clippy::type_complexity)]
        on_removed: RefCell<Option<Box<dyn Fn(&[u64])>>>,
    }

    #[derive(Clone)]
    pub struct CorkboardCardsModel {
        inner: Rc<Inner>,
    }

    impl CorkboardCardsModel {
        pub fn new(
            ctx: Rc<AppContext>,
            work_id: Signal<Option<u64>>,
            stack_id: Signal<Option<u64>>,
            container_id: Signal<u64>,
            nested: Signal<bool>,
        ) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::new(),
                    ctx,
                    work_id,
                    stack_id,
                    container_id,
                    nested,
                    on_removed: RefCell::new(None),
                }),
            }
        }

        /// The raw list, for GridView's `from_source` in natural (unfiltered) order.
        pub fn list(&self) -> ListModel<CorkboardCard> {
            self.inner.model.clone()
        }

        /// The filter+sort projection over this model — GridView binds it when a
        /// search or sort is active. Reorder is inert through the projection (a
        /// filtered/sorted view can't be dragged), which is the intended UX.
        pub fn projection(&self) -> SortFilterListModel<CorkboardCard> {
            SortFilterListModel::from_source(self.clone())
                // The corkboard's live search field pushes into the "text" column.
                .with_predicate("text", |q| {
                    let q = q.to_string();
                    Box::new(move |c: &CorkboardCard| super::card_text_matches(c, &q))
                })
                // Sort by title (case-insensitive). Manuscript order = no sort.
                .with_comparator("title", |a: &CorkboardCard, b| {
                    a.title.to_lowercase().cmp(&b.title.to_lowercase())
                })
        }

        /// Current ordered cards — a synchronous snapshot for the view-model.
        pub fn cards(&self) -> Vec<CorkboardCard> {
            let m = &self.inner.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |c| c.clone()))
                .collect()
        }

        /// Subscribe once (per build) to the structural + rename events that can
        /// change the card set, plus the scope signals, then do the initial fill.
        ///
        /// `on_removed` is invoked (from this and every later refresh) with the ids
        /// that left the board, so the owner can release their shared synopsis
        /// documents — a card can vanish by merge, trash, promote, undo or a sibling
        /// window's edit, and every one of those paths must drop the reference.
        ///
        /// **`on_removed` must not capture its owner strongly.** It is stored for the
        /// model's lifetime and the model is owned by that owner — an `Rc` capture
        /// would close a cycle, the owner's `Drop` would never run, and every synopsis
        /// document the board ever opened would leak. [`CorkboardViewModel::wire`]
        /// passes a `Weak`-capturing closure.
        pub fn wire(&self, ctx: &mut BuildContext, on_removed: impl Fn(&[u64]) + 'static) {
            *self.inner.on_removed.borrow_mut() = Some(Box::new(on_removed));
            use DirectAccessEntity::BinderItem;
            use EntityEvent::{Created, Removed, Updated};
            let origins = [
                // Rename/label edits (Updated) matter here — titles/labels are baked.
                Origin::DirectAccess(BinderItem(Created)),
                Origin::DirectAccess(BinderItem(Updated)),
                Origin::DirectAccess(BinderItem(Removed)),
                Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
                Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
                Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
                Origin::BinderItemManagement(BinderItemManagementEvent::SplitScene),
                Origin::BinderItemManagement(BinderItemManagementEvent::Promote),
                // Every trash transition, not a subset: `RestoreItemsTo` is the trash
                // dock's own restore path and `TrashBinder`/`DeleteTrashEntries` also
                // move items in or out of `activated` — missing any of them leaves an
                // open board showing a card set the store no longer agrees with.
                Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
                Origin::TrashManagement(TrashManagementEvent::TrashBinder),
                Origin::TrashManagement(TrashManagementEvent::RestoreItems),
                Origin::TrashManagement(TrashManagementEvent::RestoreItemsTo),
                Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
                Origin::TrashManagement(TrashManagementEvent::DeleteTrashEntries),
            ];
            for origin in origins {
                let me = self.clone();
                ctx.subscribe_event(origin, move |_event: &Event| me.refresh());
            }
            // Project (re)load — guarded (loose form): `refresh` always re-derives
            // from this model's own `work_id`, so a sibling Work's Load/New would
            // only cost a harmless, still-correct re-derive; guarded anyway so
            // opening a second Work doesn't force a wasted rebuild of every other
            // open window's corkboard.
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                let work_id = self.inner.work_id.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    let mine = work_id.get();
                    if mine.is_none() || event.ids.first() == mine.as_ref() {
                        me.refresh();
                    }
                });
            }
            // Drilling in/out and toggling nested/flat re-scope the query.
            {
                let me = self.clone();
                ctx.effect(&self.inner.container_id, move |_| me.refresh());
            }
            {
                let me = self.clone();
                ctx.effect(&self.inner.nested, move |_| me.refresh());
            }
            self.refresh();
        }

        fn refresh(&self) {
            let cards = query(
                &self.inner.ctx,
                self.inner.work_id.get(),
                self.inner.container_id.get(),
                self.inner.nested.get(),
            );
            let before: Vec<u64> = {
                let m = &self.inner.model;
                (0..m.len())
                    .filter_map(|i| m.with_item(i, |c| c.item_id))
                    .collect()
            };
            let after: HashSet<u64> = cards.iter().map(|c| c.item_id).collect();
            // Framework keyed diff: only actually-changed rows emit a change, so a
            // GridView tile is rebuilt only where a card changed.
            self.inner.model.reconcile_by_key(cards, |c| c.item_id);

            let removed: Vec<u64> = before
                .into_iter()
                .filter(|id| !after.contains(id))
                .collect();
            if !removed.is_empty() {
                // Take the callback out of the RefCell before invoking it: it calls
                // back into the owner, which may re-enter this model.
                let cb = self.inner.on_removed.borrow_mut().take();
                if let Some(cb) = cb {
                    cb(&removed);
                    *self.inner.on_removed.borrow_mut() = Some(cb);
                }
            }
        }

        fn stack(&self) -> Option<u64> {
            self.inner.stack_id.get()
        }

        /// Read one field off the card with the given id (drag gating).
        fn with_card<R>(&self, key: u64, f: impl Fn(&CorkboardCard) -> R) -> Option<R> {
            let m = &self.inner.model;
            (0..m.len())
                .find_map(|i| m.with_item(i, |c| (c.item_id == key).then(|| f(c))))
                .flatten()
        }
    }

    impl ListDataSource for CorkboardCardsModel {
        type Item = CorkboardCard;
        // The item id IS the key — stable across reorders, so a GridView drag
        // carries item ids (not positions) and reorder maps straight to `move_items`.
        type Key = u64;

        fn len(&self) -> usize {
            self.inner.model.len()
        }

        fn with_item<R>(&self, index: usize, f: impl FnOnce(&CorkboardCard) -> R) -> Option<R> {
            self.inner.model.with_item(index, f)
        }

        fn key_at(&self, index: usize) -> Option<u64> {
            self.inner.model.with_item(index, |c| c.item_id)
        }

        fn index_of(&self, key: &u64) -> Option<usize> {
            (0..self.inner.model.len())
                .find(|&i| self.inner.model.with_item(i, |c| c.item_id) == Some(*key))
        }

        fn observe_changes(
            &self,
            f: impl Fn(&bastyde::data::DataChange) + 'static,
        ) -> ObserverHandle {
            self.inner.model.observe_changes(f)
        }

        fn drag(&self, key: &u64) -> DragEligibility {
            // A book terminator must stay last — never draggable.
            if self
                .with_card(*key, |c| c.sub_role.closes_book())
                .unwrap_or(false)
            {
                DragEligibility::NoDrag
            } else {
                DragEligibility::CanDrag
            }
        }

        fn can_accept(&self, query: &DropQuery<'_, u64>) -> DropResponse {
            match &query.source {
                // Intra-corkboard reorder onto a different card.
                DragSource::SameView { key } if *key != query.target => DropResponse::Accept,
                // Self-drop, or a foreign payload (handled by GridView's
                // `accept_foreign_rows` path, not here).
                _ => DropResponse::Reject,
            }
        }

        /// The same- or multi-select reorder commit: one `move_items` call for the
        /// whole dragged block, then re-derive from the backend. The model is never
        /// mutated locally, so the `MoveItems` event refresh is an idempotent no-op.
        fn reorder_within(&self, sources: &[u64], target: &u64, position: DropPosition) -> bool {
            let item_ids: Vec<u64> = sources.iter().copied().filter(|k| k != target).collect();
            if item_ids.is_empty() {
                return false;
            }
            let move_place = match position {
                DropPosition::Before => MovePlace::Before,
                // A flat card grid has no "Into"; treat it as After.
                DropPosition::After | DropPosition::Into => MovePlace::After,
            };
            let ok = binder_item_management_commands::move_items(
                &self.inner.ctx,
                self.stack(),
                &MoveDto {
                    item_ids,
                    target_id: Some(*target),
                    target_is_binder: false,
                    move_place,
                },
            )
            .is_ok();
            if ok {
                self.refresh();
            }
            ok
        }
    }

    /// The container's ordered cards for the given scope.
    fn query(
        ctx: &AppContext,
        work_id: Option<u64>,
        container_id: u64,
        nested: bool,
    ) -> Vec<CorkboardCard> {
        let Some(work_id) = work_id else {
            return Vec::new();
        };
        let flat = flat_items(ctx, work_id);
        cards_for(&flat, container_id, nested)
    }

    /// Every activated binder item of `work_id`, binder-major, in each binder's
    /// stored relationship order — the same order a save writes. `indent` nests them.
    fn flat_items(ctx: &AppContext, work_id: u64) -> Vec<BinderItemDto> {
        let mut out = Vec::new();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            // `get_binder_item_multi` returns db-key order, so index by id and walk
            // `item_ids` (the authoritative relationship order).
            let by_id: HashMap<u64, BinderItemDto> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, it))
                    .collect();
            for id in item_ids {
                if let Some(it) = by_id.get(&id)
                    && it.activated
                {
                    out.push(it.clone());
                }
            }
        }
        out
    }

    /// Cards for `container_id`'s scope, out of the flat item stream.
    fn cards_for(flat: &[BinderItemDto], container_id: u64, nested: bool) -> Vec<CorkboardCard> {
        let Some(pos) = flat.iter().position(|it| it.id == container_id) else {
            return Vec::new();
        };
        let base = flat[pos].indent;
        // The container's subtree: the contiguous run after it whose indent stays
        // deeper than the container's own.
        let after = &flat[pos + 1..];
        let end = after
            .iter()
            .position(|it| it.indent <= base)
            .unwrap_or(after.len());
        let subtree = &after[..end];

        let mut out = Vec::new();
        if nested {
            for (i, it) in subtree.iter().enumerate() {
                if it.indent != base + 1 {
                    continue; // grandchildren belong to their own (folder) corkboard
                }
                let is_container = matches!(it.role, BinderItemRole::Folder);
                let child_count = if is_container {
                    count_direct_children(&subtree[i + 1..], it.indent)
                } else {
                    0
                };
                out.push(card(it, is_container, child_count));
            }
        } else {
            // Flat: every descendant in the whole subtree, in reading order — leaves
            // *and* containers (a Part / Chapter / folder carries its own synopsis too,
            // so flattening must not drop them). A container card is not drillable here
            // (flat has no drill); activating it opens its own editor.
            for (i, it) in subtree.iter().enumerate() {
                let is_container = matches!(it.role, BinderItemRole::Folder);
                let child_count = if is_container {
                    count_direct_children(&subtree[i + 1..], it.indent)
                } else {
                    0
                };
                out.push(card(it, is_container, child_count));
            }
        }
        out
    }

    /// Direct children of a folder = items right after it at exactly one deeper
    /// indent, until the subtree closes.
    fn count_direct_children(after: &[BinderItemDto], parent_indent: i64) -> usize {
        let mut n = 0;
        for it in after {
            if it.indent <= parent_indent {
                break;
            }
            if it.indent == parent_indent + 1 {
                n += 1;
            }
        }
        n
    }

    fn card(it: &BinderItemDto, is_container: bool, child_count: usize) -> CorkboardCard {
        CorkboardCard {
            item_id: it.id,
            role: it.role.clone(),
            sub_role: it.sub_role.clone(),
            title: it.title.clone(),
            label: it.label.clone(),
            is_container,
            child_count,
            tags: it.tags.clone(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use frontend::common::entities::BinderItemSubRole;

        fn item(
            id: u64,
            indent: i64,
            role: BinderItemRole,
            sub_role: BinderItemSubRole,
        ) -> BinderItemDto {
            BinderItemDto {
                id,
                indent,
                role,
                sub_role,
                title: format!("item {id}"),
                activated: true,
                ..Default::default()
            }
        }

        /// A book with: Part(1) › Chapter-folder(2) › Scene(3), Scene(4); then a
        /// flat Chapter(5); then a sibling Part(6) outside.
        fn fixture() -> Vec<BinderItemDto> {
            use BinderItemRole::{Folder, Item};
            use BinderItemSubRole::{ChapterScene, Part, Scene};
            vec![
                item(100, 0, Folder, BinderItemSubRole::Book),
                item(1, 1, Folder, Part),
                item(2, 2, Folder, ChapterScene),
                item(3, 3, Item, Scene),
                item(4, 3, Item, Scene),
                item(5, 2, Item, ChapterScene),
                item(6, 1, Folder, Part),
            ]
        }

        #[test]
        fn nested_shows_direct_children_with_child_counts() {
            let flat = fixture();
            // The Book's direct children: Part(1) and Part(6).
            let cards = cards_for(&flat, 100, true);
            let ids: Vec<u64> = cards.iter().map(|c| c.item_id).collect();
            assert_eq!(ids, vec![1, 6]);

            // Part(1)'s direct children: the chapter-folder(2) and the flat chapter(5).
            let cards = cards_for(&flat, 1, true);
            let ids: Vec<u64> = cards.iter().map(|c| c.item_id).collect();
            assert_eq!(ids, vec![2, 5]);
            let folder = &cards[0];
            assert!(folder.is_container);
            assert_eq!(
                folder.child_count, 2,
                "chapter-folder(2) holds scenes 3 and 4"
            );
            assert!(!cards[1].is_container, "flat chapter is a leaf card");
        }

        #[test]
        fn flat_shows_all_descendants_including_containers() {
            let flat = fixture();
            // Part(1) flattened: every descendant in reading order — the chapter-folder(2)
            // (a container, kept because it has its own synopsis), its scenes 3 and 4,
            // and the flat chapter 5.
            let cards = cards_for(&flat, 1, false);
            let ids: Vec<u64> = cards.iter().map(|c| c.item_id).collect();
            assert_eq!(ids, vec![2, 3, 4, 5]);
            let folder = &cards[0];
            assert!(
                folder.is_container,
                "the chapter-folder is kept in flat mode"
            );
            assert_eq!(folder.child_count, 2, "and still reports its scene count");
            assert!(
                cards[1..].iter().all(|c| !c.is_container),
                "3, 4, 5 are leaves"
            );
        }

        #[test]
        fn unknown_container_yields_no_cards() {
            assert!(cards_for(&fixture(), 999, true).is_empty());
        }

        #[test]
        fn empty_container_yields_no_cards() {
            // Part(6) has no children (it's last, indent 1, nothing deeper follows).
            assert!(cards_for(&fixture(), 6, true).is_empty());
            assert!(cards_for(&fixture(), 6, false).is_empty());
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;

    use bastyde::core::ObserverHandle;
    use bastyde::data::{ListDataSource, ListModel, SortFilterListModel};
    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    use super::CorkboardCard;

    /// A small fixture matching the mock binder in `singles::SingleBinderItem`:
    /// a Part (301) holds a chapter-folder (104) and a flat chapter (302).
    fn mock_cards(container_id: u64, nested: bool) -> Vec<CorkboardCard> {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{ChapterScene, Note, Scene};
        // Tag ids point at the mock palette in `WorkTagsListModel`: 1 = status/draft,
        // 4 = needs research, 5 = character (discoverable). Given so the mock corkboard
        // actually renders a dot row, including the discoverable ring and an overflow cell.
        let card =
            |item_id, role, sub_role, title: &str, is_container, child_count, tags: &[u64]| {
                CorkboardCard {
                    item_id,
                    role,
                    sub_role,
                    title: title.to_string(),
                    label: String::new(),
                    is_container,
                    child_count,
                    tags: tags.to_vec(),
                }
            };
        match (container_id, nested) {
            // The chapter-folder's own scenes.
            (104, _) => vec![
                card(201, Item, Scene, "Into the Dark", false, 0, &[1, 5]),
                card(202, Item, Scene, "The light returns", false, 0, &[2]),
                card(203, Item, Note, "First night", false, 0, &[]),
            ],
            // A part, nested: its chapters.
            (301, true) => vec![
                card(
                    104,
                    Folder,
                    ChapterScene,
                    "The Keeper",
                    true,
                    3,
                    &[1, 4, 5, 6, 2, 3],
                ),
                card(302, Item, ChapterScene, "The ferry", false, 0, &[4]),
            ],
            // A part, flat: every descendant — the chapter-folder (kept for its own
            // synopsis) plus every leaf under it.
            (301, false) => vec![
                card(
                    104,
                    Folder,
                    ChapterScene,
                    "The Keeper",
                    true,
                    3,
                    &[1, 4, 5, 6, 2, 3],
                ),
                card(201, Item, Scene, "Into the Dark", false, 0, &[1, 5]),
                card(202, Item, Scene, "The light returns", false, 0, &[2]),
                card(203, Item, Note, "First night", false, 0, &[]),
                card(302, Item, ChapterScene, "The ferry", false, 0, &[4]),
            ],
            _ => Vec::new(),
        }
    }

    #[derive(Clone)]
    pub struct CorkboardCardsModel {
        model: ListModel<CorkboardCard>,
        container_id: Signal<u64>,
        nested: Signal<bool>,
        /// Shared across clones, exactly as the real model's `Rc<Inner>` field is —
        /// see the real [`CorkboardCardsModel::wire`] for the contract.
        #[allow(clippy::type_complexity)]
        on_removed: Rc<RefCell<Option<Box<dyn Fn(&[u64])>>>>,
    }

    impl CorkboardCardsModel {
        pub fn new(
            _ctx: Rc<AppContext>,
            _work_id: Signal<Option<u64>>,
            _stack_id: Signal<Option<u64>>,
            container_id: Signal<u64>,
            nested: Signal<bool>,
        ) -> Self {
            let model = ListModel::from_vec(mock_cards(container_id.get(), nested.get()));
            Self {
                model,
                container_id,
                nested,
                on_removed: Rc::new(RefCell::new(None)),
            }
        }

        pub fn list(&self) -> ListModel<CorkboardCard> {
            self.model.clone()
        }

        pub fn projection(&self) -> SortFilterListModel<CorkboardCard> {
            SortFilterListModel::from_source(self.clone())
                .with_predicate("text", |q| {
                    let q = q.to_string();
                    Box::new(move |c: &CorkboardCard| super::card_text_matches(c, &q))
                })
                .with_comparator("title", |a: &CorkboardCard, b| {
                    a.title.to_lowercase().cmp(&b.title.to_lowercase())
                })
        }

        pub fn cards(&self) -> Vec<CorkboardCard> {
            let m = &self.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |c| c.clone()))
                .collect()
        }

        pub fn wire(&self, ctx: &mut BuildContext, on_removed: impl Fn(&[u64]) + 'static) {
            *self.on_removed.borrow_mut() = Some(Box::new(on_removed));
            // Re-fill the fixture when the scope changes; no backend to subscribe.
            let me = self.clone();
            ctx.effect(&self.container_id, move |_| me.refill());
            let me = self.clone();
            ctx.effect(&self.nested, move |_| me.refill());
        }

        fn refill(&self) {
            let next = mock_cards(self.container_id.get(), self.nested.get());
            let before: Vec<u64> = {
                let m = &self.model;
                (0..m.len())
                    .filter_map(|i| m.with_item(i, |c| c.item_id))
                    .collect()
            };
            let after: HashSet<u64> = next.iter().map(|c| c.item_id).collect();
            self.model.reconcile_by_key(next, |c| c.item_id);

            let removed: Vec<u64> = before
                .into_iter()
                .filter(|id| !after.contains(id))
                .collect();
            if !removed.is_empty() {
                let cb = self.on_removed.borrow_mut().take();
                if let Some(cb) = cb {
                    cb(&removed);
                    *self.on_removed.borrow_mut() = Some(cb);
                }
            }
        }
    }

    impl ListDataSource for CorkboardCardsModel {
        type Item = CorkboardCard;
        type Key = u64;

        fn len(&self) -> usize {
            self.model.len()
        }

        fn with_item<R>(&self, index: usize, f: impl FnOnce(&CorkboardCard) -> R) -> Option<R> {
            self.model.with_item(index, f)
        }

        fn key_at(&self, index: usize) -> Option<u64> {
            self.model.with_item(index, |c| c.item_id)
        }

        fn observe_changes(
            &self,
            f: impl Fn(&bastyde::data::DataChange) + 'static,
        ) -> ObserverHandle {
            self.model.observe_changes(f)
        }
    }
}

pub use imp::CorkboardCardsModel;
