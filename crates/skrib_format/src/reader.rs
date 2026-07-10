//! Dispatch a read by detected shape, then run the format migration chain.

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::bundle::{ProjectManifest, WorkBundle};
use super::migration::migrate_bundle;
use super::shape::{MANIFEST_NAME, SkribShape, detect_shape, folder_root};
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

/// Read **only** the `project.skrib` manifest from a bundle, without extracting
/// the whole archive.
///
/// The open-flow backup sniff and the retention scan classify potentially many
/// `.skrib` files; a full [`read_bundle`] (which extracts the entire zip to a
/// tempdir) would be far too costly for that. This reads just the one manifest
/// entry. Legacy SQLite has no manifest, so it errors (a legacy file is never a
/// backup). Note: no migration is run, so only fields present on disk are
/// populated — fine for classification (`kind`/`backup_of`/`unique_id`).
pub fn peek_manifest(path: &str) -> Result<ProjectManifest> {
    match detect_shape(path)? {
        SkribShape::ExplodedFolder => {
            let manifest_path = folder_root(path).join(MANIFEST_NAME);
            let text = std::fs::read_to_string(&manifest_path)
                .with_context(|| format!("reading {}", manifest_path.display()))?;
            ron::from_str(&text).with_context(|| format!("parsing {}", manifest_path.display()))
        }
        SkribShape::ZipFile => {
            let file = File::open(path).with_context(|| format!("opening '{path}'"))?;
            let mut archive =
                zip::ZipArchive::new(file).with_context(|| format!("reading zip '{path}'"))?;
            let mut entry = archive
                .by_name(MANIFEST_NAME)
                .with_context(|| format!("no {MANIFEST_NAME} entry in '{path}'"))?;
            let mut text = String::new();
            entry
                .read_to_string(&mut text)
                .with_context(|| format!("reading {MANIFEST_NAME} from '{path}'"))?;
            ron::from_str(&text).with_context(|| format!("parsing {MANIFEST_NAME} from '{path}'"))
        }
        SkribShape::LegacySqlite => {
            anyhow::bail!("'{path}' is a legacy SQLite file; it has no manifest")
        }
    }
}
