// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive model for the **Trash** panel: a tree of trashed *roots* — one row
//! per `TrashInfo` (a whole trashed binder, or a trashed item + its subtree) —
//! each disclosing a **read-only** preview of its cascaded descendants.
//!
//! Like [`BinderBinderItemsTreeModel`](super::BinderBinderItemsTreeModel) it is a
//! thin facade over a Bastyde [`TreeDataSlice`]; the only real/mock difference is
//! the row source ([`rows::load`]). Unlike the outline it is **read-only**: no
//! drag policy, no reorder hook, and roots stay collapsed by default (a scannable
//! summary you expand to see a root's cascade).
//!
//! The source of truth is the **index** (`Work.trash_infos`), not
//! `get_all_trash_info()` — `restore_items`/`restore_items_to` unlink a consumed
//! entry from the index but leave the orphan `TrashInfo` entity in the store, so
//! only the index reflects what is actually in the trash.

use std::cell::Cell;
use std::rc::Rc;

use bastyde::data::{
    DragEligibility, DropCommit, DropQuery, DropResponse, FlatEntry, TreeDataSlice, TreeDataSource,
};
use bastyde::prelude::{BuildContext, Signal};
use frontend::AppContext;
use frontend::common::entities::BinderItemSubRole;
use frontend::common::event::{
    DirectAccessEntity, EntityEvent, Event, Origin, TrashManagementEvent,
};

/// Stable per-row identity. A root is keyed by its **TrashInfo** id (the handle
/// every action needs); a cascade row by its **BinderItem** id (globally unique).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrashTreeKey {
    Root(u64),
    Descendant(u64),
}

/// What a root row represents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TrashRootKind {
    /// A whole trashed binder.
    Binder,
    /// A trashed item + its subtree.
    #[default]
    Item,
}

/// One node in the trash tree.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrashNode {
    pub title: String,
    /// Subtitle: the trashed-at stamp on a root, the item's own note on a
    /// descendant.
    pub label: String,
    /// `true` on a top-level `TrashInfo` row.
    pub is_root: bool,
    /// Meaningful only when `is_root`.
    pub root_kind: TrashRootKind,
    /// Icon selector (default on whole-binder roots, which use the binder glyph).
    pub sub_role: BinderItemSubRole,
    /// The `BinderItem` to open on activation — `Some` for descendants and for
    /// item roots; `None` for whole-binder roots (nothing single to open).
    pub item_id: Option<u64>,
    /// The owning `TrashInfo` id — `Some` on root rows (the delegate reconstructs
    /// a root's [`TrashTreeKey`] from this).
    pub trash_info_id: Option<u64>,
}

#[derive(Clone)]
pub struct TrashTreeModel {
    slice: TreeDataSlice<TrashTreeKey, TrashNode>,
    /// One-shot guard so `wire` subscribes only once.
    subscribed: Rc<Cell<bool>>,
}

impl TrashTreeModel {
    pub fn new(ctx: Rc<AppContext>, work_id: Signal<Option<u64>>) -> Self {
        let slice = TreeDataSlice::new();
        // Roots stay collapsed by default: a scannable summary, expand to preview.
        {
            let ctx = ctx.clone();
            let work_id = work_id.clone();
            slice.set_source(move || rows::load(&ctx, &work_id));
        }
        slice.reload();
        Self {
            slice,
            subscribed: Rc::new(Cell::new(false)),
        }
    }

    /// Subscribe once to every backend change that can add/remove/rename a trash
    /// entry. The coarse `TrashManagement` events cover the forward path; the
    /// granular `DirectAccess` events are **required** because the restore/empty
    /// *undo* paths publish only granular Created/Updated/Removed events.
    pub fn wire(&self, ctx: &mut BuildContext) {
        if self.subscribed.replace(true) {
            return;
        }
        use DirectAccessEntity::{Binder, BinderItem, TrashInfo};
        use EntityEvent::{Created, Removed, Updated};
        let origins = [
            Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
            Origin::TrashManagement(TrashManagementEvent::TrashBinder),
            Origin::TrashManagement(TrashManagementEvent::RestoreItems),
            Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
            Origin::TrashManagement(TrashManagementEvent::RestoreItemsTo),
            Origin::TrashManagement(TrashManagementEvent::DeleteTrashEntries),
            Origin::DirectAccess(TrashInfo(Created)),
            Origin::DirectAccess(TrashInfo(Updated)),
            Origin::DirectAccess(TrashInfo(Removed)),
            Origin::DirectAccess(BinderItem(Created)),
            Origin::DirectAccess(BinderItem(Updated)),
            Origin::DirectAccess(BinderItem(Removed)),
            Origin::DirectAccess(Binder(Created)),
            Origin::DirectAccess(Binder(Updated)),
            Origin::DirectAccess(Binder(Removed)),
        ];
        for origin in origins {
            let me = self.clone();
            ctx.subscribe_event(origin, move |_e: &Event| me.reload());
        }
    }

    pub fn reload(&self) {
        self.slice.reload();
    }

    pub fn contains(&self, key: &TrashTreeKey) -> bool {
        self.slice.contains_key(key)
    }

    pub fn node_title(&self, key: TrashTreeKey) -> Option<String> {
        self.slice.with_key(&key, |n| n.title.clone())
    }

    /// The `BinderItem` to open for a row (item roots + descendants); `None` for
    /// whole-binder roots.
    pub fn item_id_of(&self, key: TrashTreeKey) -> Option<u64> {
        self.slice.with_key(&key, |n| n.item_id).flatten()
    }

    /// Title used when opening the row's editor tab.
    pub fn title_of(&self, key: TrashTreeKey) -> Option<String> {
        self.slice.with_key(&key, |n| n.title.clone())
    }

    /// The kind of a root row (`None` if the key isn't a root).
    pub fn kind_of(&self, trash_info_id: u64) -> Option<TrashRootKind> {
        self.slice
            .with_key(&TrashTreeKey::Root(trash_info_id), |n| {
                if n.is_root { Some(n.root_kind) } else { None }
            })
            .flatten()
    }

    pub fn is_descendant(&self, key: TrashTreeKey) -> bool {
        matches!(key, TrashTreeKey::Descendant(_))
    }

    /// Count of top-level trash entries (excludes cascade rows). Drives the
    /// "Empty Trash…" button's enabled state and the empty-state placeholder.
    pub fn visible_root_count(&self) -> usize {
        let mut n = 0usize;
        for i in 0..self.slice.visible_count() {
            if self
                .slice
                .with_entry(i, |node, _| node.is_root)
                .unwrap_or(false)
            {
                n += 1;
            }
        }
        n
    }

    /// Reactive "does the trash hold anything" — recomputed on every slice bump.
    pub fn has_entries_signal(&self) -> Signal<bool> {
        let me = self.clone();
        self.slice
            .version_signal()
            .map(move |_| me.visible_root_count() > 0)
    }
}

/// Straight delegation onto the backing slice. Read-only: `drag`/`can_accept`/
/// `accept_drop` fall through to the slice defaults (no drag policy set), so the
/// tree is inert for drag-and-drop.
impl TreeDataSource for TrashTreeModel {
    type Item = TrashNode;
    type Key = TrashTreeKey;

    fn visible_count(&self) -> usize {
        self.slice.visible_count()
    }
    fn with_entry<R>(
        &self,
        flat_index: usize,
        f: impl FnOnce(&Self::Item, &FlatEntry<Self::Key>) -> R,
    ) -> Option<R> {
        self.slice.with_entry(flat_index, f)
    }
    fn key_at(&self, flat_index: usize) -> Option<TrashTreeKey> {
        self.slice.key_at(flat_index)
    }
    fn flat_index_of(&self, key: &TrashTreeKey) -> Option<usize> {
        self.slice.flat_index_of(key)
    }
    fn parent(&self, key: &TrashTreeKey) -> Option<TrashTreeKey> {
        self.slice.parent_of(key)
    }
    fn child_keys(&self, key: &TrashTreeKey) -> Vec<TrashTreeKey> {
        self.slice.child_keys_of(key)
    }
    fn version_signal(&self) -> Signal<u64> {
        self.slice.version_signal()
    }
    fn first_changed_index(&self) -> Option<usize> {
        self.slice.first_changed_index()
    }
    fn contains_key(&self, key: &TrashTreeKey) -> bool {
        self.slice.contains_key(key)
    }
    fn is_expanded(&self, key: &TrashTreeKey) -> bool {
        self.slice.is_expanded(key)
    }
    fn set_expanded(&self, key: &TrashTreeKey, expanded: bool) {
        self.slice.set_expanded(key, expanded);
    }
    fn drag(&self, key: &TrashTreeKey) -> DragEligibility {
        self.slice.drag(key)
    }
    fn can_accept(&self, query: &DropQuery<'_, TrashTreeKey>) -> DropResponse {
        self.slice.can_accept(query)
    }
    fn accept_drop(&self, commit: DropCommit<'_, TrashTreeKey>) -> bool {
        self.slice.accept_drop(commit)
    }
}

// ── The row-source seam: the only real/mock difference ──────────────────────

#[cfg(not(feature = "mocks"))]
mod rows {
    use std::collections::HashMap;

    use bastyde::data::TreeRow;
    use bastyde::prelude::Signal;

    use frontend::AppContext;
    use frontend::commands::{
        binder_commands, binder_item_commands, trash_info_commands, work_commands,
    };
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::types::EntityId;
    use frontend::direct_access::TrashInfoDto;

    use super::{TrashNode, TrashRootKind, TrashTreeKey};

    fn when(dto: &TrashInfoDto) -> String {
        dto.trashed_at.format("%Y-%m-%d %H:%M").to_string()
    }

    /// Find the binder whose order contains `item_id` (trashed items stay in
    /// place). Fast path: `origin_binder_id`; fall back to a scan of the Work's
    /// binders. Returns `(binder order, indent map for that order)`.
    fn resolve_item_binder(
        ctx: &AppContext,
        binder_ids: &[EntityId],
        origin: EntityId,
        item_id: EntityId,
    ) -> Option<(Vec<EntityId>, HashMap<EntityId, i64>)> {
        let candidates = std::iter::once(origin).chain(binder_ids.iter().copied());
        let mut tried = std::collections::HashSet::new();
        for b in candidates {
            if !tried.insert(b) {
                continue;
            }
            let order = binder_commands::get_binder_relationship(
                ctx,
                &b,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            if order.contains(&item_id) {
                let mut indent = HashMap::new();
                for it in binder_item_commands::get_binder_item_multi(ctx, &order)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                {
                    indent.insert(it.id, it.indent);
                }
                return Some((order, indent));
            }
        }
        None
    }

    pub fn load(
        ctx: &AppContext,
        work_id: &Signal<Option<u64>>,
    ) -> Vec<TreeRow<TrashTreeKey, TrashNode>> {
        let mut rows: Vec<TreeRow<TrashTreeKey, TrashNode>> = Vec::new();
        let Some(work_id) = work_id.get() else {
            return rows;
        };

        // The index (not get_all_trash_info — that returns unlinked orphans).
        let indexed =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::TrashInfos)
                .unwrap_or_default();
        let mut infos: Vec<TrashInfoDto> = trash_info_commands::get_trash_info_multi(ctx, &indexed)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .collect();
        // Newest first.
        infos.sort_by_key(|i| std::cmp::Reverse(i.trashed_at));

        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();

        for info in infos {
            if let Some(binder_id) = info.trashed_binder {
                let Ok(Some(binder)) = binder_commands::get_binder(ctx, &binder_id) else {
                    continue;
                };
                rows.push(TreeRow::new(
                    TrashTreeKey::Root(info.id),
                    TrashNode {
                        title: binder.name,
                        label: when(&info),
                        is_root: true,
                        root_kind: TrashRootKind::Binder,
                        item_id: None,
                        trash_info_id: Some(info.id),
                        ..Default::default()
                    },
                    0,
                ));
                let item_ids = binder_commands::get_binder_relationship(
                    ctx,
                    &binder_id,
                    &BinderRelationshipField::BinderItems,
                )
                .unwrap_or_default();
                for it in binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                {
                    rows.push(TreeRow::new(
                        TrashTreeKey::Descendant(it.id),
                        TrashNode {
                            title: it.title,
                            label: it.label,
                            is_root: false,
                            sub_role: it.sub_role,
                            item_id: Some(it.id),
                            ..Default::default()
                        },
                        (it.indent.max(0) as usize) + 1,
                    ));
                }
            } else if let Some(item_id) = info.trashed_binder_item {
                let Ok(Some(root_item)) = binder_item_commands::get_binder_item(ctx, &item_id)
                else {
                    continue;
                };
                let root_indent = root_item.indent;
                rows.push(TreeRow::new(
                    TrashTreeKey::Root(info.id),
                    TrashNode {
                        title: root_item.title.clone(),
                        label: when(&info),
                        is_root: true,
                        root_kind: TrashRootKind::Item,
                        sub_role: root_item.sub_role,
                        item_id: Some(item_id),
                        trash_info_id: Some(info.id),
                    },
                    0,
                ));
                // Read-only cascade preview: the contiguous subtree in the item's
                // binder order (all deactivated along with the root).
                if let Some((order, indent)) = resolve_item_binder(
                    ctx,
                    &binder_ids,
                    info.origin_binder_id.max(0) as u64,
                    item_id,
                ) {
                    let subtree = binder_ordering::subtree_of(&order, &indent, item_id);
                    if subtree.len() > 1 {
                        let descendants: Vec<EntityId> = subtree[1..].to_vec();
                        for it in binder_item_commands::get_binder_item_multi(ctx, &descendants)
                            .unwrap_or_default()
                            .into_iter()
                            .flatten()
                        {
                            let depth = (it.indent - root_indent).max(1) as usize;
                            rows.push(TreeRow::new(
                                TrashTreeKey::Descendant(it.id),
                                TrashNode {
                                    title: it.title,
                                    label: it.label,
                                    is_root: false,
                                    sub_role: it.sub_role,
                                    item_id: Some(it.id),
                                    ..Default::default()
                                },
                                depth,
                            ));
                        }
                    }
                }
            }
            // else: a stale TrashInfo (neither relationship) — render nothing.
        }
        rows
    }
}

/// Static mock tree (no backend): two roots — a whole trashed binder with two
/// cascade items, and a trashed folder-item with a two-row cascade.
#[cfg(feature = "mocks")]
mod rows {
    use bastyde::data::TreeRow;
    use bastyde::prelude::Signal;

    use frontend::AppContext;
    use frontend::common::entities::BinderItemSubRole;

    use super::{TrashNode, TrashRootKind, TrashTreeKey};

    fn desc(
        id: u64,
        title: &str,
        sub_role: BinderItemSubRole,
        depth: usize,
    ) -> TreeRow<TrashTreeKey, TrashNode> {
        TreeRow::new(
            TrashTreeKey::Descendant(id),
            TrashNode {
                title: title.to_string(),
                label: String::new(),
                is_root: false,
                sub_role,
                item_id: Some(id),
                ..Default::default()
            },
            depth,
        )
    }

    pub fn load(
        _ctx: &AppContext,
        _work_id: &Signal<Option<u64>>,
    ) -> Vec<TreeRow<TrashTreeKey, TrashNode>> {
        use BinderItemSubRole::{ChapterScene, Note, Scene};
        vec![
            // Root 1: a whole trashed binder + 2 items.
            TreeRow::new(
                TrashTreeKey::Root(9001),
                TrashNode {
                    title: "Old drafts".into(),
                    label: "2026-07-18 09:12".into(),
                    is_root: true,
                    root_kind: TrashRootKind::Binder,
                    item_id: None,
                    trash_info_id: Some(9001),
                    ..Default::default()
                },
                0,
            ),
            desc(8101, "Discarded scene", Scene, 1),
            desc(8102, "Stray note", Note, 1),
            // Root 2: a trashed chapter folder + 2 scenes.
            TreeRow::new(
                TrashTreeKey::Root(9002),
                TrashNode {
                    title: "Chapter 7".into(),
                    label: "2026-07-18 10:45".into(),
                    is_root: true,
                    root_kind: TrashRootKind::Item,
                    sub_role: ChapterScene,
                    item_id: Some(8200),
                    trash_info_id: Some(9002),
                },
                0,
            ),
            desc(8201, "The ambush", Scene, 1),
            desc(8202, "Aftermath", Scene, 1),
        ]
    }
}

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use frontend::AppContext;

    fn model() -> TrashTreeModel {
        TrashTreeModel::new(Rc::new(AppContext::new()), Signal::new(Some(1)))
    }

    #[test]
    fn roots_collapsed_by_default() {
        let m = model();
        // Two roots, cascades hidden until expanded.
        assert_eq!(m.visible_count(), 2);
        assert_eq!(m.visible_root_count(), 2);
    }

    #[test]
    fn expanding_a_root_reveals_its_cascade() {
        let m = model();
        m.set_expanded(&TrashTreeKey::Root(9001), true);
        // Root1 + its 2 items + Root2 (still collapsed).
        assert_eq!(m.visible_count(), 4);
        m.set_expanded(&TrashTreeKey::Root(9002), true);
        assert_eq!(m.visible_count(), 6);
    }

    #[test]
    fn kind_and_item_id_resolve() {
        let m = model();
        assert_eq!(m.kind_of(9001), Some(TrashRootKind::Binder));
        assert_eq!(m.kind_of(9002), Some(TrashRootKind::Item));
        // Whole-binder root opens nothing; item root opens its item.
        assert_eq!(m.item_id_of(TrashTreeKey::Root(9001)), None);
        assert_eq!(m.item_id_of(TrashTreeKey::Root(9002)), Some(8200));
        assert_eq!(m.item_id_of(TrashTreeKey::Descendant(8201)), Some(8201));
    }

    #[test]
    fn has_entries_reflects_root_count() {
        let m = model();
        assert!(m.has_entries_signal().get());
    }
}
