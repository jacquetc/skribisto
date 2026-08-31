// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dispatch a read by detected shape, then run the format migration chain.

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::bundle::{ProjectManifest, WorkBundle};
use super::errors::SkribFormatError;
use super::migration::migrate_bundle;
use super::shape::{MANIFEST_NAME, SkribShape, detect_shape, folder_root};
use super::version_gate;
use super::{folder_io, zip_io};

/// Read a new-format bundle (zip or exploded folder). Legacy SQLite is handled
/// by the dedicated legacy reader, not here.
///
/// The version gate runs **first**, on the manifest alone. That ordering is the whole
/// point: a bundle from a future format usually fails at `ron::from_str` on `items.ron`
/// — a raw `Unexpected variant named "…"` several frames deep — long before any
/// version number is compared, and for the zip shape only *after* the entire archive
/// has been extracted to a tempdir. Gating up front replaces both with one specific,
/// actionable [`SkribFormatError::TooNew`], at the cost of reading one small file twice.
pub fn read_bundle(path: &str) -> Result<WorkBundle, SkribFormatError> {
    let shape = detect_shape(path).map_err(SkribFormatError::Unreadable)?;

    let mut bundle = match shape {
        SkribShape::ExplodedFolder => {
            version_gate::check_version_gate(path, shape)?;
            folder_io::read_folder(&folder_root(path)).map_err(SkribFormatError::Unreadable)?
        }
        SkribShape::ZipFile => {
            version_gate::check_version_gate(path, shape)?;
            zip_io::read_zip(Path::new(path)).map_err(SkribFormatError::Unreadable)?
        }
        SkribShape::LegacySqlite => {
            return Err(SkribFormatError::Unreadable(anyhow::anyhow!(
                "'{path}' is a legacy SQLite file; use the legacy reader"
            )));
        }
    };

    migrate_bundle(&mut bundle).map_err(SkribFormatError::Unreadable)?;
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
