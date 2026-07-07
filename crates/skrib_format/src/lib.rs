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

mod bundle;
pub mod convert;
mod folder_io;
mod loaded;
mod mapping;
mod migration;
mod reader;
mod shape;
mod slug;
#[cfg(test)]
mod tests;
mod writer;
mod zip_io;

// On-disk bundle DTOs + the in-memory `WorkBundle`. Public so an external
// producer (the Plume importer) can build a bundle directly, exactly the way
// `from_entities` builds one from store entities.
pub use bundle::{
    BinderFile, BinderItemFile, BinderTagFile, BinderWithItems, BundledBinder, BundledItem,
    DictWordFile, FORMAT_VERSION, InlineContent, ItemWithContents, ItemsFile, ProjectManifest,
    ProseRef, ShapeTag, TrashInfoFile, WorkBundle, WorkFile,
};
pub use convert::{html_to_djot, markdown_to_html};
pub use loaded::{LoadedBinder, LoadedItem, LoadedTrash, LoadedWork};
pub use mapping::{bundle_to_loaded, from_entities};
pub use reader::read_bundle;
pub use shape::{SkribShape, detect_shape};
pub use slug::{binder_dir_name, prose_file_name, prose_kind, prose_relpath, slugify};
pub use writer::write_bundle;

use anyhow::Result;
use std::path::Path;

/// Generate a fresh, stable project identity string (UUID v4). Used to mint a
/// `Work.unique_id` for brand-new projects, to heal a load whose source carries
/// none, and by the Plume importer for the work it creates.
pub fn new_unique_id() -> String {
    uuid::Uuid::new_v4().to_string()
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

/// Copy a bundle to `dst` as a single `.skrib` file (used by `backup_now`): a
/// zip or legacy file is copied verbatim; an exploded folder is packed into a
/// zip so a backup is always one portable file.
pub fn copy_bundle(src: &str, dst: &str) -> Result<()> {
    match shape::detect_shape(src)? {
        SkribShape::ZipFile | SkribShape::LegacySqlite => {
            if let Some(p) = Path::new(dst)
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
            {
                std::fs::create_dir_all(p)?;
            }
            std::fs::copy(src, dst)?;
            Ok(())
        }
        SkribShape::ExplodedFolder => zip_io::zip_dir(&shape::folder_root(src), Path::new(dst)),
    }
}
