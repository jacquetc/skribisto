//! The `.skrib` document format (v1).
//!
//! A Skribisto document is RON manifests + Djot prose, stored either as a single
//! **zip** `.skrib` (canonical, cross-platform) or an **exploded folder** (opt-in,
//! git-friendly) — interconvertible at any time. This module is the single source
//! for reading and writing it; it is consumed by `load_work` (read), `save_work`
//! (write), `migrate_to_skrib_file` / `migrate_to_skrib_folder` (write a shape),
//! and `backup_now` (copy).
//!
//! Prose is Djot end-to-end: `Content.data` holds Djot, so `.djot` blobs are
//! written and read verbatim — no conversion at the file boundary. Title-role
//! content stays inline in `items.ron`. See `bundle` for the on-disk schema.

mod bundle;
mod folder_io;
mod loaded;
mod mapping;
mod migration;
mod reader;
mod shape;
mod slug;
mod writer;
mod zip_io;
#[cfg(test)]
mod tests;

pub use bundle::{BinderWithItems, ItemWithContents, ShapeTag};
#[cfg(test)]
pub use bundle::WorkBundle;
pub use loaded::{LoadedBinder, LoadedItem, LoadedTrash, LoadedWork};
pub use mapping::{bundle_to_loaded, from_entities};
pub use reader::read_bundle;
pub use shape::{SkribShape, detect_shape};
pub use writer::write_bundle;

use anyhow::Result;
use std::path::Path;

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
            if let Some(p) = Path::new(dst).parent().filter(|p| !p.as_os_str().is_empty()) {
                std::fs::create_dir_all(p)?;
            }
            std::fs::copy(src, dst)?;
            Ok(())
        }
        SkribShape::ExplodedFolder => zip_io::zip_dir(&shape::folder_root(src), Path::new(dst)),
    }
}
