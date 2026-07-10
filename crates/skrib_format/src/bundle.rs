//! On-disk DTOs for the `.skrib` bundle format (v1) plus the in-memory
//! [`WorkBundle`] that ties them together.
//!
//! These are deliberately **separate** from the `common::entities` structs:
//!  - datetimes are RFC3339 strings (human-readable, clean diffs) rather than
//!    the entities' `ts_milliseconds` integers;
//!  - title-role content is stored *inline* while prose-role content is a
//!    reference to a `.djot` file;
//!  - ids are stable `file_id`s (the entity `u64`s at save time), remapped to
//!    fresh store ids on load.
//!
//! RON serialises the typed `BinderItemRole`/`BinderItemSubRole`/`ContentRole`
//! enums as their plain variant names (`Item`, `Scene`, `SceneText`), so the
//! manifests stay readable and isomorphic to the domain model.

use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Current on-disk format version. Bump + add a `migration` step when the
/// schema changes.
///
/// v2 added `WorkFile.unique_id` (a stable project identity). It's
/// `#[serde(default)]`, so v1 bundles still deserialize (empty id), and the
/// load path mints a fresh id when it's empty — see `load_work_uc::materialize`.
pub const FORMAT_VERSION: u32 = 2;

/// The shape recorded in `project.skrib` (informational; the real shape is the
/// physical layout). Mirrors `common::entities::WorkShape`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeTag {
    Zip,
    Folder,
}

/// Is this bundle a regular project file, or a point-in-time **backup** copy?
///
/// The manifest — not the filename — is the authoritative answer to "is this a
/// backup". The `<stem>-<stamp>.skrib` naming stays a human/sort convention and a
/// fallback sniff for backups written before this field existed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BundleKind {
    #[default]
    Regular,
    Backup,
}

/// `project.skrib` — the manifest, and the commit point of every save.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub format_version: u32,
    pub shape: ShapeTag,
    pub work: WorkFile,
    /// Ordered binder `file_id`s (authoritative binder order).
    pub binder_order: Vec<u64>,
    /// Regular project vs. backup copy. Added post-v2; `#[serde(default)]` keeps
    /// every existing manifest (all `Regular`) readable — same additive pattern as
    /// `WorkFile.unique_id`/`chapter_flat`, so `FORMAT_VERSION` is not bumped.
    #[serde(default)]
    pub kind: BundleKind,
    /// For a backup: the original project's path at backup time (best-effort — a
    /// path can move/rename later; retention correlates on `WorkFile.unique_id`).
    #[serde(default)]
    pub backup_of: Option<String>,
    /// For a backup: its own creation timestamp (RFC3339), independent of the
    /// filename's `-YYYYMMDD-HHMMSS` suffix.
    #[serde(default)]
    pub backup_created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub author_name: String,
    pub dict_language: String,
    pub tag_ids: Vec<u64>,
    pub dict_word_ids: Vec<u64>,
    /// Stable project identity (UUID v4, or a preserved legacy id). Added in v2;
    /// `#[serde(default)]` keeps v1 bundles readable (empty → healed on load).
    #[serde(default)]
    pub unique_id: String,
    /// Per-project chapter storage: `true` = flat `Item/ChapterScene`, `false`
    /// (default) = `Folder/Chapter`. Added post-v2; `#[serde(default)]` keeps
    /// older bundles readable (missing → folder mode).
    #[serde(default)]
    pub chapter_flat: bool,
}

/// `tags.ron`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderTagFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub color: String,
    pub text_color: String,
}

/// `dictionary.ron`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictWordFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub word: String,
}

/// `trash.ron`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrashInfoFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub trashed_at: String,
    pub origin_binder_id: i64,
    pub trashed_binder: Option<u64>,
    pub trashed_binder_item: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub activated: bool,
    /// Ordered item `file_id`s (authoritative item order within the binder).
    pub item_order: Vec<u64>,
}

/// A title-role content row, stored inline in `items.ron`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineContent {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub activated: bool,
    pub role: ContentRole,
    pub text: String,
}

/// A prose-role content row: the text lives in a sibling `.djot` file at `path`
/// (relative to the bundle root).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProseRef {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub activated: bool,
    pub role: ContentRole,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderItemFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub sub_title: String,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub label: String,
    pub activated: bool,
    pub is_favorite: bool,
    pub is_printable: bool,
    pub indent: i64,
    pub word_count_goal: i64,
    pub char_count_goal: i64,
    pub dict_language: String,
    pub inline_contents: Vec<InlineContent>,
    pub prose_refs: Vec<ProseRef>,
    /// M2M self-references (cross-links), as `file_id`s.
    pub reference_ids: Vec<u64>,
    /// M2M tag `file_id`s.
    pub tag_ids: Vec<u64>,
}

/// The content of one `binders/NN-slug/items.ron`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemsFile {
    pub binder: BinderFile,
    pub items: Vec<BinderItemFile>,
}

/// The whole document, in memory. Not serialised as a single file — `writer`
/// splits it across `project.skrib`, `tags.ron`, … and the `.djot` blobs. The
/// `Serialize` derive is used only to compute a stable content fingerprint (see
/// `fingerprint`); the on-disk format is still the split layout, never this.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WorkBundle {
    pub manifest: ProjectManifest,
    pub tags: Vec<BinderTagFile>,
    pub dict_words: Vec<DictWordFile>,
    pub trash_infos: Vec<TrashInfoFile>,
    pub binders: Vec<BundledBinder>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BundledBinder {
    pub binder: BinderFile,
    pub items: Vec<BundledItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BundledItem {
    pub item: BinderItemFile,
    /// Prose content text keyed by content `file_id` (the `.djot` blobs).
    pub prose: BTreeMap<u64, String>,
}

/// Pre-fetched, ordered store data handed to [`super::from_entities`] at save
/// time (the use case reads the tree through its UoW and fills these).
pub struct BinderWithItems {
    pub binder: common::entities::Binder,
    pub items: Vec<ItemWithContents>,
}

pub struct ItemWithContents {
    pub item: common::entities::BinderItem,
    pub contents: Vec<common::entities::Content>,
}
