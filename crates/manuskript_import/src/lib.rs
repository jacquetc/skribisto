// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Manuskript → newest-version `.skrib` importer.
//!
//! One-directional, version-neutral, and a **pure file→file transform**: it reads
//! a Manuskript project — either format generation, the modern folder or the
//! zipped `.msk` — and writes a `.skrib` at the newest format version. The entity
//! store is never touched; the UI loads the result afterward through the existing
//! `load_work`.
//!
//! The source project is only ever **read**. Nothing here writes, moves or deletes
//! anything in it.

pub mod map;
pub mod mmd;
pub mod model;
pub mod model_xml;
pub mod outline_xml;
pub mod prose;
pub mod refs;
pub mod source;
pub mod v0;
pub mod v1;
pub mod version;
pub mod xml;

use crate::model::Project;
use crate::source::ManuskriptSource;
use crate::version::FormatVersion;

/// Read an opened project with whichever reader its format calls for.
///
/// Both readers normalise into the same [`Project`], so nothing downstream knows
/// or asks which generation it came from.
pub fn read_project(src: &ManuskriptSource) -> Project {
    match src.format {
        FormatVersion::V0 => v0::read(src),
        FormatVersion::V1 => v1::read(src),
    }
}

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result, bail};
use skrib_format::{SkribShape, write_bundle};

/// What the import produced, for the UI's summary.
pub struct ImportSummary {
    pub output_path: String,
    /// Rows written, across both binders.
    pub imported_items: u64,
    /// Earlier versions carried into the project's history.
    pub imported_revisions: u64,
    /// Everything the writer should know: what was not understood, what had
    /// nowhere to go, and which copy of the project was read.
    pub warnings: Vec<String>,
}

/// Convert the Manuskript project at `source_path` into a `.skrib` at
/// `output_path`, reporting progress and honouring cancellation.
///
/// `source_path` may be a zipped `.msk`, the one-byte `.msk` beside a project
/// folder, or the folder itself.
///
/// The `.skrib` is written to a sibling temp file and renamed into place only
/// after the last cancel check, so an aborted or failed run leaves an existing
/// target untouched. The Manuskript project is never written to.
pub fn import_with_progress(
    source_path: &str,
    output_path: &str,
    overwrite: bool,
    names: &map::Names,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Result<ImportSummary> {
    if !overwrite && Path::new(output_path).exists() {
        bail!("'{output_path}' already exists (choose another name or allow overwrite)");
    }

    report(2.0, "Opening the Manuskript project…");
    let src = ManuskriptSource::open(source_path)?;
    let container = src.container;
    let newest = src.newest_modified;
    bail_if_cancelled(cancel)?;

    report(12.0, "Reading the outline…");
    let project = read_project(&src);
    bail_if_cancelled(cancel)?;

    report(20.0, "Converting chapters and scenes…");
    let mut mapped = map::build_bundle(&project, names, report, cancel);
    bail_if_cancelled(cancel)?;

    // Which copy was read, and how recent it is. A project that has been through
    // both storage modes can have two, and only one of them is current.
    mapped.warnings.insert(
        0,
        match newest {
            Some(at) => format!(
                "Read the {} copy of this project, last changed {}.",
                container.label(),
                at.format("%Y-%m-%d")
            ),
            None => format!("Read the {} copy of this project.", container.label()),
        },
    );

    report(92.0, "Writing the .skrib…");
    let tmp_path = format!("{output_path}.importing");
    write_bundle(&tmp_path, SkribShape::ZipFile, &mapped.bundle)
        .with_context(|| format!("writing '{output_path}'"))?;
    if cancel.load(Ordering::Relaxed) {
        let _ = std::fs::remove_file(&tmp_path);
        bail!("import cancelled");
    }
    std::fs::rename(&tmp_path, output_path)
        .with_context(|| format!("finalising '{output_path}'"))?;

    report(100.0, "Done");
    Ok(ImportSummary {
        output_path: output_path.to_string(),
        imported_items: mapped.imported_items,
        imported_revisions: mapped.imported_revisions,
        warnings: mapped.warnings,
    })
}

/// Abort before anything is written if the cancel token has been set.
fn bail_if_cancelled(cancel: &AtomicBool) -> Result<()> {
    if cancel.load(Ordering::Relaxed) {
        bail!("import cancelled");
    }
    Ok(())
}
