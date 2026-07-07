//! Dispatch a read by detected shape, then run the format migration chain.

use anyhow::Result;
use std::path::Path;

use super::bundle::WorkBundle;
use super::migration::migrate_bundle;
use super::shape::{SkribShape, detect_shape, folder_root};
use super::{folder_io, zip_io};

/// Read a new-format bundle (zip or exploded folder). Legacy SQLite is handled
/// by the dedicated legacy reader, not here.
pub fn read_bundle(path: &str) -> Result<WorkBundle> {
    let mut bundle = match detect_shape(path)? {
        SkribShape::ExplodedFolder => folder_io::read_folder(&folder_root(path))?,
        SkribShape::ZipFile => zip_io::read_zip(Path::new(path))?,
        SkribShape::LegacySqlite => {
            anyhow::bail!("'{path}' is a legacy SQLite file; use the legacy reader")
        }
    };
    migrate_bundle(&mut bundle)?;
    Ok(bundle)
}
