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
use chrono::{DateTime, FixedOffset, Utc};
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
                writer_day(at)
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

/// The calendar day of `at` as the writer's own clock names it, for a notice
/// they will read.
///
/// Every moment this crate handles is UTC (a file's modification time), and a
/// notice that printed the UTC day told a writer in Tokyo that their project was
/// last changed the day before they changed it.
pub(crate) fn writer_day(at: DateTime<Utc>) -> String {
    at.with_timezone(&writer_offset(at))
        .format("%Y-%m-%d")
        .to_string()
}

/// The writer's distance from UTC at the instant `at`.
///
/// The machine's zone, read the way the app reads it for every other moment it
/// shows: `TZ` first, then the system setting. Taken at `at` rather than now,
/// because a zone with daylight saving sits at one distance from UTC in winter
/// and another in summer, and a file changed in January is dated by January's.
#[cfg(not(test))]
fn writer_offset(at: DateTime<Utc>) -> FixedOffset {
    chrono::TimeZone::offset_from_utc_datetime(&chrono::Local, &at.naive_utc())
}

/// In this crate's own tests the writer is on UTC unless a test says otherwise
/// with [`override_writer_offset`], so no test depends on the zone of the
/// machine that runs it.
#[cfg(test)]
fn writer_offset(_at: DateTime<Utc>) -> FixedOffset {
    WRITER_OFFSET
        .with(std::cell::Cell::get)
        .unwrap_or_else(|| chrono::Offset::fix(&Utc))
}

#[cfg(test)]
thread_local! {
    /// The offset [`writer_offset`] answers with on this thread, when a test set
    /// one. Per thread, so one test's zone never reaches another's.
    static WRITER_OFFSET: std::cell::Cell<Option<FixedOffset>> =
        const { std::cell::Cell::new(None) };
}

/// Puts back the writer's offset a test replaced, when it goes out of scope.
#[cfg(test)]
#[must_use = "the override ends when the guard is dropped"]
pub(crate) struct WriterOffsetGuard {
    previous: Option<FixedOffset>,
}

#[cfg(test)]
impl Drop for WriterOffsetGuard {
    fn drop(&mut self) {
        WRITER_OFFSET.with(|o| o.set(self.previous));
    }
}

/// Put the writer `hours` east of UTC (west when negative) on this thread,
/// until the guard drops.
#[cfg(test)]
pub(crate) fn override_writer_offset(hours: i32) -> WriterOffsetGuard {
    let offset = match FixedOffset::east_opt(hours * 3600) {
        Some(o) => o,
        None => panic!("{hours} hours is not an offset"),
    };
    WriterOffsetGuard {
        previous: WRITER_OFFSET.with(|o| o.replace(Some(offset))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    fn names() -> map::Names {
        map::Names {
            manuscript_binder: "Manuscript".into(),
            story_bible_binder: "Story bible".into(),
            characters_group: "Characters".into(),
            world_group: "World".into(),
            plots_group: "Plots".into(),
            project_info_note: "Project information".into(),
            summary_note: "Summary".into(),
            importance: ["Minor".into(), "Secondary".into(), "Main".into()],
        }
    }

    /// A folder project whose every file was last changed at `at`.
    fn folder_project_changed_at(root: &Path, at: SystemTime) {
        let members = [
            ("MANUSKRIPT", "1"),
            ("infos.txt", "Title:          A Novel\n"),
            (
                "outline/0-Opening.md",
                "title:          Opening\nID:             2\ntype:           md\n\n\nProse.",
            ),
        ];
        for (name, content) in members {
            let path = root.join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("fixture dir");
            }
            std::fs::write(&path, content).expect("fixture file");
            std::fs::File::options()
                .write(true)
                .open(&path)
                .and_then(|f| f.set_modified(at))
                .expect("set mtime");
        }
    }

    /// Import `root` and return the first notice: the one naming the copy read.
    fn first_notice(root: &Path, out: &Path) -> String {
        let summary = import_with_progress(
            &root.to_string_lossy(),
            &out.to_string_lossy(),
            true,
            &names(),
            &|_, _| {},
            &AtomicBool::new(false),
        );
        match summary {
            Ok(s) => s.warnings.first().cloned().unwrap_or_default(),
            Err(e) => panic!("importing the fixture: {e:#}"),
        }
    }

    /// **The defect.** A project last changed at 08:30 on the 4th in Tokyo was
    /// changed at 23:30 on the 3rd in UTC, and the import summary said the 3rd.
    /// Read back from the notice the import itself writes, so it fails whether
    /// the day is formatted on UTC's clock here or at the notice.
    #[test]
    fn the_copy_read_is_dated_on_the_writers_calendar() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("A Novel");
        // 2026-03-03T23:30:00Z.
        folder_project_changed_at(
            &root,
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_772_580_600),
        );
        let out = dir.path().join("imported.skrib");

        assert_eq!(
            first_notice(&root, &out),
            "Read the folder copy of this project, last changed 2026-03-03.",
            "on UTC's clock it is still the 3rd",
        );
        let _tokyo = override_writer_offset(9);
        assert_eq!(
            first_notice(&root, &out),
            "Read the folder copy of this project, last changed 2026-03-04.",
        );
    }
}
