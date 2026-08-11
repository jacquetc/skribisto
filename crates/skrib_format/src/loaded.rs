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
    Asset, Binder, BinderItem, BinderTag, CommentAnchorKind, CommentOrphanReason, Content,
    DictWord, NoteTemplate, SmartPunctuation, TextReplacementRule, Work,
};

pub struct LoadedWork {
    /// `work.id` = work file id; `binders`/`tags`/`dict_words` vecs are empty
    /// (order comes from the explicit vecs below).
    pub work: Work,
    pub tags: Vec<BinderTag>,
    pub dict_words: Vec<DictWord>,
    pub text_replacement_rules: Vec<TextReplacementRule>,
    /// Per-project note templates, in the order the writer arranged them. Each carries
    /// its Djot `body` inline here — the manifest/blob split is an on-disk concern that
    /// `bundle_to_loaded` has already reassembled by this point.
    pub note_templates: Vec<NoteTemplate>,
    /// Image metadata rows. The bytes are *not* here: they live in the
    /// project's media directory and in `assets/<hash>.<ext>` inside the
    /// bundle, which is what keeps a project's photographs out of the store
    /// and off every snapshot, undo and fingerprint path.
    pub assets: Vec<Asset>,
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
    /// Comment threads, each naming the **content file id** it annotates — or
    /// `None` for one that arrived from the bundle-root orphanage, whose anchored
    /// Content is already gone.
    pub comments: Vec<LoadedComment>,
    pub footnotes: Vec<LoadedFootnote>,
    /// (source file id, destination file id) cross-link pairs.
    pub references: Vec<(u64, u64)>,
    /// (scene file id, story-bible item file id) point-of-view pairs.
    pub point_of_view: Vec<(u64, u64)>,
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
    /// Which kind of waypoint this is. Carried from disk rather than re-derived from
    /// `target_item`, because that reference can fail to remap: an `Item` milestone whose
    /// chapter was deleted must stay an `Item` milestone with a missing target, not turn
    /// into a book-cumulative one holding a number nobody typed.
    pub kind: common::entities::MilestoneKind,
}

pub struct LoadedComment {
    /// Durable identity, carried from disk so anything outside the core entity
    /// tree can still name this row after a reload.
    pub uid: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Annotated Content **file id** (remapped at materialise time). `None` means
    /// the comment came from the orphanage and has no live anchor.
    pub content: Option<u64>,
    pub kind: CommentAnchorKind,
    pub author_name: String,
    /// The author's own initials, as an editor's word processor records them. Empty
    /// when the source carried none — never derived, so a writer's chosen form is not
    /// overwritten by a guess.
    pub author_initials: String,
    /// Djot.
    pub body: String,
    pub resolved: bool,
    pub orphaned: bool,
    pub orphan_reason: CommentOrphanReason,
    pub range_start: u64,
    pub range_length: u64,
    pub quote_prefix: String,
    pub quote_exact: String,
    pub quote_exact_truncated: bool,
    pub quote_suffix: String,
    pub block_ordinal_hint: u64,
    pub replies: Vec<LoadedCommentReply>,
}

/// One footnote as read from a bundle, before its ids are minted.
pub struct LoadedFootnote {
    /// Durable identity, carried from disk so anything outside the core entity
    /// tree can still name this row after a reload.
    pub uid: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Annotated Content **file id** (remapped at materialise time). `None` means
    /// the note came from the orphanage and its reference is gone from the prose.
    pub content: Option<u64>,
    pub label: String,
    pub body: String,
}

pub struct LoadedCommentReply {
    /// Durable identity, on the same terms as [`LoadedComment::uid`] — a reply has to be
    /// recognisable across an editorial round trip in its own right, or an answer
    /// inserted mid-conversation re-imports every later reply as a duplicate.
    pub uid: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author_name: String,
    /// The replier's own initials. Empty when the source carried none.
    pub author_initials: String,
    /// Djot.
    pub body: String,
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
