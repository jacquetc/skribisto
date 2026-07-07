//! The neutral, entity-typed graph that a reader produces and the load use case
//! materialises into the store. Both the new-format reader (`bundle_to_loaded`)
//! and the legacy SQLite reader feed this, so there is a single, lossless
//! materialisation path.
//!
//! Ids on these entities are the source's **file ids** — the materialiser
//! remaps them to fresh store ids while preserving order and M2M links.

use common::entities::{Binder, BinderItem, BinderTag, Content, DictWord, Work};

pub struct LoadedWork {
    /// `work.id` = work file id; `binders`/`tags`/`dict_words` vecs are empty
    /// (order comes from the explicit vecs below).
    pub work: Work,
    pub tags: Vec<BinderTag>,
    pub dict_words: Vec<DictWord>,
    pub binders: Vec<LoadedBinder>,
    pub trash_infos: Vec<LoadedTrash>,
    /// (source file id, destination file id) cross-link pairs.
    pub references: Vec<(u64, u64)>,
    /// Absolute path recorded in `RecentWork` (the opened file/folder).
    pub absolute_path: String,
}

pub struct LoadedBinder {
    pub binder: Binder,
    pub items: Vec<LoadedItem>,
}

pub struct LoadedItem {
    /// `item.id` = item file id; relationship vecs on the entity are unused.
    pub item: BinderItem,
    pub contents: Vec<Content>,
    /// M2M tag file ids for this item.
    pub tag_ids: Vec<u64>,
}

pub struct LoadedTrash {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub trashed_at: chrono::DateTime<chrono::Utc>,
    /// Origin binder **file id** (0 = a whole trashed binder, no parent);
    /// remapped to the store id at materialise time.
    pub origin_binder_id: i64,
    /// Binder / item file ids of the trashed entity (one of them set).
    pub trashed_binder: Option<u64>,
    pub trashed_binder_item: Option<u64>,
}
