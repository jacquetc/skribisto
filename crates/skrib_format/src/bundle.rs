// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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
///
/// v3 added `BinderFile.uid` / `BinderItemFile.uid` — the same idea one level
/// down, so a binder row can be referenced durably (expand state, bookmarks,
/// cross-links) instead of by a position that every load renumbers. Also
/// `#[serde(default)]`; `migration::step_v2_to_v3` mints the empties, which is
/// the first real step the migration chain has ever had to run.
///
/// v5 added the per-project **note templates** (`templates.ron` + the
/// `templates/*.djot` bodies). The data itself is purely additive — a missing
/// `templates.ron` reads back as an empty list — so by the rule the fields above
/// follow it would need no bump at all. It gets one anyway, and the reason is the
/// *other* direction: [`crate::zip_io::write_zip`] rebuilds the archive from a fresh
/// staging dir on every save, so an older build — whose `WorkBundle` has no
/// `note_templates` field — would silently drop every template the first time it saved
/// a project that had them. Permanently, with nothing to notice it by. The bump makes
/// [`migration::migrate_bundle`] refuse the file up front ("written by a newer
/// Skribisto") instead, turning silent data loss into a loud, recoverable error.
/// That is the whole value of the bump; `step_v4_to_v5` itself has nothing to do.
pub const FORMAT_VERSION: u32 = 5;

/// Read `dict_language` as a list, accepting the pre-v4 space-separated string.
///
/// A type change cannot be handled by [`migration`](crate::migration): that runs *after*
/// serde has parsed the bundle, and a v3 file would fail to parse before it ever got there.
/// So the tolerance lives in the deserializer, and the migration step only advances the
/// stamp — the same division `step_v1_to_v2` already uses for a field healed elsewhere.
///
/// This is not back-compatibility for its own sake: without it every project written before
/// this change becomes unopenable, and the list is the *only* record of which dictionaries a
/// writer chose.
fn tags_or_legacy_string<'de, D>(d: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt;

    struct TagsOrString;

    impl<'de> serde::de::Visitor<'de> for TagsOrString {
        type Value = Vec<String>;

        // A hand-written visitor rather than `#[serde(untagged)]`: untagged reports every
        // malformed value as "data did not match any variant of untagged enum", and the
        // exploded-folder shape is meant to be hand-edited and diffed. A typo there deserves
        // to say what was expected.
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a list of language tags, or (before format v4) one space-separated string")
        }

        fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
            Ok(skribisto_model::language::parse_legacy_list(s))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(tag) = seq.next_element::<String>()? {
                out.push(tag);
            }
            Ok(out)
        }
    }

    d.deserialize_any(TagsOrString)
}

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
    /// The writer's name, for the compiled title page and the exported metadata.
    /// Optional — an empty string means "not set" and is omitted downstream.
    ///
    /// `#[serde(default)]` like every other additive field here: without it a
    /// manifest written before this field existed fails to *deserialize*, which
    /// happens before [`migration`](crate::migration) ever runs, so no migration
    /// step could rescue it. It carries no version bump for the same reason
    /// `unique_id` and `chapter_flat` carry none — a purely additive optional
    /// field costs older readers nothing.
    #[serde(default)]
    pub author_name: String,
    #[serde(default, deserialize_with = "tags_or_legacy_string")]
    pub dict_language: Vec<String>,
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
    /// The custom text-replacement lexicon's ids for this Work. Additive, like
    /// `chapter_flat` — `#[serde(default)]` keeps older bundles readable (missing
    /// → no rules, exactly what a project written before this feature existed
    /// should mean).
    #[serde(default)]
    pub text_replacement_rule_ids: Vec<u64>,
    /// Per-project master switch for the custom text-replacement lexicon, off by
    /// default — a writer opts a specific project in explicitly. `#[serde(default)]`
    /// for the same reason as `text_replacement_rule_ids`.
    #[serde(default)]
    pub custom_replacement_rules_enabled: bool,
    /// The punctuation house style, nested rather than given a file of its own.
    ///
    /// `dictionary.ron` and `replacements.ron` are separate files because they
    /// hold *collections* that grow independently of the Work. This is a single
    /// row of seven scalars that exists exactly once per Work and is meaningless
    /// without it — the same shape as `chapter_flat` above, just wider. A
    /// `punctuation.ron` would buy file-level diff granularity nobody needs, at
    /// the cost of another read/write path and another id space.
    ///
    /// `None` means the bundle predates the feature, and is deliberately not
    /// collapsed to an all-false default here: only the loader can decide what
    /// absence should become, and it needs to be able to tell absence from a
    /// writer who switched everything off.
    #[serde(default)]
    pub smart_punctuation: Option<SmartPunctuationFile>,
}

/// The punctuation house style, nested inside [`WorkFile`].
///
/// No `file_id`, unlike every sibling `*File` type: those ids exist so other
/// records can reference the row and so the materialiser can remap it. Nothing
/// references this one — it is reached only through its owning `WorkFile` — so
/// an id would be a value to keep unique for no reader's benefit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmartPunctuationFile {
    pub created_at: String,
    pub updated_at: String,
    /// Whether this project overrides the app-level preference at all. False
    /// means "follow the application default", and the flags below are then
    /// inert — kept rather than cleared, so turning the override back on
    /// restores what the writer had configured.
    #[serde(default)]
    pub override_app_default: bool,
    #[serde(default)]
    pub dashes: bool,
    #[serde(default)]
    pub ellipsis: bool,
    #[serde(default)]
    pub quotes: bool,
    /// `LocaleDefault`, `CurlyDouble`, `Guillemets` or `LowHigh` — stored as a
    /// string so a future variant added by one build does not make the bundle
    /// unreadable to another; an unrecognised value falls back to the locale
    /// default on read.
    #[serde(default)]
    pub quote_style: String,
    #[serde(default)]
    pub pre_punctuation_spacing: bool,
    #[serde(default)]
    pub dialogue_marker: bool,
}

/// `tags.ron`
///
/// No text colour is stored: it is derived from `color` at render time via
/// `Color::best_contrast_text`, which always clears WCAG AA. Persisting it would be
/// redundant state every write path would have to keep in step.
///
/// **This revision is not purely additive** — it added `details`/`discoverable` *and*
/// dropped `text_color`, so unlike `WorkFile.unique_id`/`chapter_flat` the two directions
/// differ:
/// * *Reading older bundles still works*: the new fields default, and serde ignores the
///   now-unknown `text_color` (no `deny_unknown_fields` anywhere in this crate).
/// * *Older builds cannot read what we now write*: `text_color` is absent and they
///   require it. `FORMAT_VERSION` is deliberately left at 2 anyway, because the project
///   has no external users and no back-compat obligation — but be aware that the failure
///   mode is a raw serde "missing field" error rather than `migrate_bundle`'s friendly
///   "written by a newer Skribisto", which only triggers on a version *greater* than ours.
///   Bump `FORMAT_VERSION` if that error quality ever starts to matter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderTagFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub color: String,
    /// What the tag means, shown in its hover tooltip.
    #[serde(default)]
    pub details: String,
    /// Items carrying this tag are story-bible material for the mention index.
    #[serde(default)]
    pub discoverable: bool,
}

/// `dictionary.ron`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictWordFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub word: String,
}

/// `replacements.ron`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextReplacementRuleFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub trigger: String,
    pub replacement: String,
    pub enabled: bool,
}

/// One row of `templates.ron`.
///
/// The body is **not** inline: it lives in a sibling `templates/<file_id>-<slug>.djot`
/// blob at `path`, exactly as a scene's prose does. Two reasons, both about the
/// exploded-folder shape: a multi-paragraph Djot document inside a RON string is
/// unreadable in a diff, and a writer who wants to hand-author a template should be
/// able to drop a `.djot` file in beside the others rather than escape it into RON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteTemplateFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub starred: bool,
    /// Path to this template's Djot body, relative to the bundle root.
    pub path: String,
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

/// A skipped-days stretch inside a `PaceFile` (nested in `paces.ron`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HolidayFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub label: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

/// A per-Part/Chapter deadline inside a `PaceFile`. `target_item` is a weak reference
/// (item `file_id`), `None` when it no longer resolves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub label: String,
    pub target_item: Option<u64>,
    pub target_date: String,
    pub target_word_count: Option<i64>,
}

/// `paces.ron` — one per-Book writing plan, children nested inline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaceFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    /// The Book item this plan targets (weak reference, item `file_id`).
    pub book_item: Option<u64>,
    pub start_date: String,
    pub end_date: String,
    pub weekday_mask: i64,
    pub active: bool,
    pub holidays: Vec<HolidayFile>,
    pub milestones: Vec<MilestoneFile>,
}

/// `snapshots.ron` — one day's writing-progress totals. `book_item_ids` /
/// `book_word_counts` are index-paired parallel arrays (the per-Book breakdown); the item
/// ids are weak references remapped on load.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressSnapshotFile {
    pub file_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub day: String,
    pub total_word_count: i64,
    pub total_char_count: Option<i64>,
    pub book_item_ids: Vec<u64>,
    pub book_word_counts: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderFile {
    pub file_id: u64,
    /// Durable per-row identity (UUID v4), stable across every save→load
    /// cycle — unlike `file_id`, which is only the store id at save time and is
    /// re-minted on the next save. `#[serde(default)]` so pre-v3 bundles still
    /// deserialize (empty), and `migrate_bundle` fills them in.
    #[serde(default)]
    pub uid: uuid::Uuid,

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
    /// Durable per-row identity (UUID v4), stable across every save→load
    /// cycle — unlike `file_id`, which is only the store id at save time and is
    /// re-minted on the next save. `#[serde(default)]` so pre-v3 bundles still
    /// deserialize (empty), and `migrate_bundle` fills them in.
    #[serde(default)]
    pub uid: uuid::Uuid,

    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub sub_title: String,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub label: String,
    pub activated: bool,
    pub is_favorite: bool,
    pub is_exportable: bool,
    pub indent: i64,
    pub word_count_goal: i64,
    pub char_count_goal: i64,
    #[serde(default, deserialize_with = "tags_or_legacy_string")]
    pub dict_language: Vec<String>,
    /// Other names this item answers to in prose, matched alongside its title by the
    /// mention index. Purely additive, so an existing `items.ron` reads back with an
    /// empty vector (`parses_a_bundle_written_before_these_fields_existed` covers it).
    #[serde(default)]
    pub aliases: Vec<String>,
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
    pub text_replacement_rules: Vec<TextReplacementRuleFile>,
    pub note_templates: Vec<NoteTemplateFile>,
    /// Template body text keyed by template `file_id` (the `templates/*.djot` blobs) —
    /// the same split `BundledItem::prose` uses for scene text.
    pub note_template_bodies: BTreeMap<u64, String>,
    pub trash_infos: Vec<TrashInfoFile>,
    pub paces: Vec<PaceFile>,
    pub progress_snapshots: Vec<ProgressSnapshotFile>,
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

/// Pre-fetched Pace + its child Holiday/Milestone entities, handed to
/// [`super::from_entities`] at save time (mirrors [`BinderWithItems`]).
pub struct PaceWithChildren {
    pub pace: common::entities::Pace,
    pub holidays: Vec<common::entities::Holiday>,
    pub milestones: Vec<common::entities::Milestone>,
}
