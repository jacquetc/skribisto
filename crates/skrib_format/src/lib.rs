// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `.skrib` document format.
//!
//! A Skribisto document is RON manifests + Djot prose, stored either as a single
//! **zip** `.skrib` (canonical, cross-platform) or an **exploded folder** (opt-in,
//! git-friendly) — interconvertible at any time. This crate is the single source
//! for reading and writing it; it is consumed by `work_management` (`load_work`
//! read, `save_work`/`save_as` write, `backup_now` copy) and by `import_management`
//! (the Plume importer builds a `WorkBundle` directly and writes it).
//!
//! It lives in its own crate — rather than inside `work_management` — so two
//! features can share it without a feature-to-feature dependency (a house rule),
//! and because it may depend on `skribisto_model` (which `common` cannot).
//!
//! Prose is Djot end-to-end: `Content.data` holds Djot, so `.djot` blobs are
//! written and read verbatim — no conversion at the file boundary. Title-role
//! content stays inline in `items.ron`. See `bundle` for the on-disk schema. The
//! `convert` module bridges legacy Qt rich-text HTML into Djot (used by the
//! legacy `.skrib` upgrader and the Plume importer).

#[cfg(test)]
mod asset_tests;
mod bundle;
pub mod convert;
mod errors;
mod fingerprint;
mod folder_io;
mod loaded;
mod mapping;
pub mod media;
mod migration;
mod reader;
pub mod retention;
mod shape;
/// Filesystem-safe name shaping. `pub` because the UI's template export needs the same
/// `slugify` the bundle writer uses — a name the writer typed must land on one safe path
/// segment on both paths, and two spellings of that rule would be two behaviours.
pub mod slug;
mod sniff;
#[cfg(test)]
mod tests;
/// One ordered, relationship-hydrated read of the open Work tree, shared by every use
/// case that snapshots it (`save_work` / `save_as` / `backup_now` / `export_work`).
pub mod tree_read;
/// The single authority on "can this build open this bundle" — the pre-parse version
/// gate and the content-derived read floor it judges. `pub` because
/// [`compute_min_read_version`](version_gate::compute_min_read_version) is the forcing
/// function a future format change has to reckon with, and hiding it would make that
/// contract invisible from outside the crate.
pub mod version_gate;
mod writer;
mod zip_io;

// On-disk bundle DTOs + the in-memory `WorkBundle`. Public so an external
// producer (the Plume importer) can build a bundle directly, exactly the way
// `from_entities` builds one from store entities.
pub use bundle::{
    BinderFile, BinderItemFile, BinderTagFile, BinderWithItems, BundleKind, BundledBinder,
    BundledItem, CommentFile, CommentReplyFile, CommentWithReplies, DictWordFile, FORMAT_VERSION,
    FootnoteFile, FootnoteWithContent, HolidayFile, InlineContent, ItemWithContents, ItemsFile,
    MilestoneFile, NoteTemplateFile, PaceFile, PaceWithChildren, ProgressSnapshotFile,
    ProjectManifest, ProseRef, ShapeTag, SmartPunctuationFile, TextReplacementRuleFile,
    TrashInfoFile, WorkBundle, WorkFile,
};
pub use convert::{html_to_djot, markdown_to_html};
pub use errors::SkribFormatError;
pub use fingerprint::content_fingerprint;
pub use loaded::{
    LoadedBinder, LoadedHoliday, LoadedItem, LoadedMilestone, LoadedPace, LoadedProgressSnapshot,
    LoadedTrash, LoadedWork,
};
pub use mapping::{bundle_to_loaded, from_entities, mark_as_backup};
pub use reader::{peek_manifest, read_bundle};
pub use shape::{SkribShape, detect_shape};
pub use slug::{
    binder_dir_name, nearest_titled_ancestor, prose_file_name, prose_kind, prose_relpath, short_id,
    slugify,
};
pub use sniff::{BackupSniff, sniff_backup, sniff_backup_filename};
pub use tree_read::{Gathered, TreeReader, gather};
pub use writer::{mark_existing_as_backup, verify_backup_at, write_bundle};

/// Generate a fresh, stable project identity string (UUID v4). Used to mint a
/// `Work.unique_id` for brand-new projects, to heal a load whose source carries
/// none, and by the Plume importer for the work it creates.
pub fn new_unique_id() -> String {
    // `Work.unique_id` is still a `string` field in the manifest, unlike the
    // row-level `uid`s which are typed `uuid`. Same generator, different
    // representation.
    common::uid::new_uid().to_string()
}

impl From<SkribShape> for Option<ShapeTag> {
    fn from(s: SkribShape) -> Self {
        match s {
            SkribShape::ZipFile => Some(ShapeTag::Zip),
            SkribShape::ExplodedFolder => Some(ShapeTag::Folder),
            SkribShape::LegacySqlite => None,
        }
    }
}
