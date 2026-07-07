//! Dispatch a write to the right physical shape.

use anyhow::Result;
use std::path::Path;

use super::bundle::WorkBundle;
use super::shape::{SkribShape, folder_root};
use super::{folder_io, zip_io};

/// Write `bundle` to `path` in `shape`, atomically. The legacy SQLite shape is
/// never written (it is read-only legacy input).
pub fn write_bundle(path: &str, shape: SkribShape, bundle: &WorkBundle) -> Result<()> {
    match shape {
        SkribShape::ExplodedFolder => folder_io::write_folder(&folder_root(path), bundle),
        SkribShape::ZipFile => zip_io::write_zip(Path::new(path), bundle),
        SkribShape::LegacySqlite => {
            anyhow::bail!("the legacy SQLite format is read-only; save as a .skrib zip or folder")
        }
    }
}
