// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Plume Creator (`.plume`) → newest-version `.skrib` importer.
//!
//! One-directional, version-neutral, and a **pure file→file transform**: it reads
//! any Plume project — any historical schema version, the modern single-file zip
//! **or** the pre-0.3 old-system directory — and writes a `.skrib` zip at the
//! newest format version. The entity store is never touched; the UI loads the
//! result afterward via the existing `load_work`.

mod attend_parse;
mod dict_parse;
mod info_parse;
mod map;
mod model;
mod source;
mod tree_parse;
mod version;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result, bail};
use skrib_format::{SkribShape, write_bundle};

use model::{PlumeAttendance, PlumeInfo};
use source::PlumeSource;

/// What the import produced, for the UI's post-import summary.
pub struct ImportSummary {
    pub output_path: String,
    /// Total binder items written (both binders, including synthetic helpers).
    pub imported_items: u64,
    /// Meaningful (non-separator) Plume nodes that were trashed and skipped.
    pub skipped_trashed: u64,
    pub warnings: Vec<String>,
}

/// Convert the Plume project at `source_path` into a `.skrib` zip at `output_path`.
///
/// Progress-free convenience wrapper used by the fixture tests; production code
/// (the long-operation use case) calls [`import_with_progress`] directly.
#[cfg(test)]
pub fn import(
    source_path: &str,
    output_path: &str,
    overwrite: bool,
    manuscript_binder_name: &str,
    story_bible_binder_name: &str,
) -> Result<ImportSummary> {
    import_with_progress(
        source_path,
        output_path,
        overwrite,
        manuscript_binder_name,
        story_bible_binder_name,
        &|_, _| {},
        &AtomicBool::new(false),
    )
}

/// Convert the Plume project at `source_path` into a `.skrib` zip at
/// `output_path`, reporting progress and honouring cancellation.
///
/// `report(percent, label)` drives the UI's progress toast; `cancel` is polled
/// at every phase boundary and once per mapped node. Cancellation leaves nothing
/// on disk: the `.skrib` is written to a sibling temp file and atomically renamed
/// into place only after the final cancel check, so an existing `output_path`
/// (overwrite) survives an aborted or failed import untouched.
pub fn import_with_progress(
    source_path: &str,
    output_path: &str,
    overwrite: bool,
    manuscript_binder_name: &str,
    story_bible_binder_name: &str,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Result<ImportSummary> {
    if !overwrite && Path::new(output_path).exists() {
        bail!("'{output_path}' already exists (choose another name or allow overwrite)");
    }

    report(2.0, "Opening the Plume project…");
    let src = PlumeSource::open(source_path)?;
    bail_if_cancelled(cancel)?;

    report(12.0, "Reading the outline…");
    let tree = tree_parse::parse(&src.tree_xml).context("reading the Plume outline (tree)")?;
    let attendance = match &src.attendance_xml {
        Some(xml) => {
            attend_parse::parse(xml).context("reading the Plume story bible (attendance)")?
        }
        None => PlumeAttendance {
            spinbox_label: String::new(),
            groups: Vec::new(),
        },
    };
    // `info` is non-critical metadata (title + dates); a malformed one just falls
    // back to the tree's project name.
    let info = match &src.info_xml {
        Some(xml) => info_parse::parse(xml).unwrap_or_default(),
        None => PlumeInfo::default(),
    };
    let dict_words = src
        .dict
        .as_deref()
        .map(dict_parse::parse)
        .unwrap_or_default();
    bail_if_cancelled(cancel)?;

    report(20.0, "Converting chapters and scenes…");
    let mapped = map::build_bundle(
        &tree,
        &attendance,
        &info,
        &dict_words,
        &src,
        manuscript_binder_name,
        story_bible_binder_name,
        report,
        cancel,
    );
    bail_if_cancelled(cancel)?;

    // Write to a sibling temp file, then atomically rename into place — so an
    // existing target (overwrite) is only replaced once the write fully succeeds
    // and no late cancel arrived.
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
        skipped_trashed: mapped.skipped_trashed,
        warnings: mapped.warnings,
    })
}

/// Abort the import if the cancel token has been set, before any file is written.
fn bail_if_cancelled(cancel: &AtomicBool) -> Result<()> {
    if cancel.load(Ordering::Relaxed) {
        bail!("import cancelled");
    }
    Ok(())
}

#[cfg(test)]
mod tests;
