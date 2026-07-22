// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The neutral, entity-typed graph that a reader produces and the load use case
//! materialises into the store. Both the new-format reader (`bundle_to_loaded`)
//! and the legacy SQLite reader feed this, so there is a single, lossless
//! materialisation path.
//!
//! Ids on these entities are the source's **file ids** — the materialiser
//! remaps them to fresh store ids while preserving order and M2M links.

use common::entities::{
    Binder, BinderItem, BinderTag, Content, DictWord, SmartPunctuation, TextReplacementRule, Work,
};

pub struct LoadedWork {
    /// `work.id` = work file id; `binders`/`tags`/`dict_words` vecs are empty
    /// (order comes from the explicit vecs below).
    pub work: Work,
    pub tags: Vec<BinderTag>,
    pub dict_words: Vec<DictWord>,
    pub text_replacement_rules: Vec<TextReplacementRule>,
    /// The punctuation house style, or `None` for a bundle written before the
    /// setting existed.
    ///
    /// `Option` rather than a defaulted value on purpose: the materialiser has
    /// to create the row either way (a one-to-one child cannot be absent), and
    /// this is the only place that can still tell "the writer chose all-off"
    /// apart from "this file predates the feature". Collapsing them here would
    /// throw that distinction away before anyone could act on it.
    pub smart_punctuation: Option<SmartPunctuation>,
    pub binders: Vec<LoadedBinder>,
    pub trash_infos: Vec<LoadedTrash>,
    pub paces: Vec<LoadedPace>,
    pub progress_snapshots: Vec<LoadedProgressSnapshot>,
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

pub struct LoadedPace {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Book item **file id** (remapped at materialise time); `None` if it no longer resolves.
    pub book_item: Option<u64>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub weekday_mask: i64,
    pub active: bool,
    pub holidays: Vec<LoadedHoliday>,
    pub milestones: Vec<LoadedMilestone>,
}

pub struct LoadedHoliday {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct LoadedMilestone {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    /// Target Part/Chapter item **file id** (remapped at materialise time).
    pub target_item: Option<u64>,
    pub target_date: chrono::DateTime<chrono::Utc>,
    pub target_word_count: Option<i64>,
}

pub struct LoadedProgressSnapshot {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub day: chrono::DateTime<chrono::Utc>,
    pub total_word_count: i64,
    pub total_char_count: Option<i64>,
    /// Per-Book breakdown: item **file ids** (remapped at materialise time), index-paired
    /// with `book_word_counts`.
    pub book_item_ids: Vec<u64>,
    pub book_word_counts: Vec<i64>,
}
