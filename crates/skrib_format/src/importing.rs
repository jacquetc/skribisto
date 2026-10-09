// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What every whole-project importer does the same way: write the new `.skrib` so that
//! nothing half-written is ever left behind, and store the words its caller gave it.
//!
//! The Plume Creator and Manuskript converters (and any later one) each read a foreign
//! project their own way, then end in exactly the same steps. They live here, below
//! every converter, because a converter may depend on this crate and on nothing the
//! others own.
//!
//! Nothing a converter says is a sentence: its progress is a phase code
//! (`import.opening`, `import.outline`, `import.converting`, `import.writing`,
//! `import.done`) or the title of the entry it is converting, which the interface words
//! or shows as data, and what it tells the writer afterwards is an
//! [`ImportNotice`](crate::import_notice::ImportNotice).
//!
//! # The output is staged
//!
//! [`StagedOutput`](crate::importing::StagedOutput) writes the bundle to
//! `<target>.importing` and moves it over the target only once it is whole and the
//! import was not cancelled, so an existing project at the target survives a failed or
//! cancelled import untouched. Every other way out of an import, a refusal, a
//! cancellation, an error while writing, a failed rename, a panic, drops the guard, and
//! the drop removes the staged file: an import leaves either the finished project or
//! nothing at all.
//!
//! A failure to write says which folder refused. Assembling a zip happens in the
//! system's temporary folder before the zip is written beside the target, and the first
//! is often a small memory-backed file system: "the disk is full" naming the wrong
//! folder would send the writer to free space where none was needed.
//!
//! # The words an import writes are the caller's
//!
//! A title the importer makes up (a note holding a container's own notes, a project with
//! no name at all) is written in the writer's language, never in English: the interface
//! resolves the words when the import starts and the converter stores them as they are.
//! A word naming something (`Notes on {title}`) arrives as a pattern holding
//! [`TITLE_SLOT`](crate::importing::TITLE_SLOT) where the name goes, and
//! [`fill_title`](crate::importing::fill_title) puts it there.

use std::fmt;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use crate::bundle::WorkBundle;

/// Where a caller's pattern puts the name it is about: `"\u{E000} (notes)"`. A
/// private-use character, which no word of any interface holds, passed to the interface's
/// message as the value of its argument.
pub const TITLE_SLOT: char = '\u{E000}';

/// `pattern` with `value` where its [`TITLE_SLOT`] is, in one pass, so a value holding the
/// slot's character is never filled into. A pattern without the slot has the value put
/// before it, an empty one is the value alone; the result is trimmed, so a nameless value
/// does not leave the pattern's space behind.
pub fn fill_title(pattern: &str, value: &str) -> String {
    let filled = if pattern.contains(TITLE_SLOT) {
        pattern.replace(TITLE_SLOT, value)
    } else if pattern.trim().is_empty() {
        value.to_string()
    } else {
        format!("{value} {pattern}")
    };
    filled.trim().to_string()
}

/// An import stopped because the writer cancelled it. Nothing was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportCancelled;

impl fmt::Display for ImportCancelled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the import was cancelled")
    }
}

impl std::error::Error for ImportCancelled {}

/// Why an import could not put its new project in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputRefused {
    /// A file is already at the target, and the import was not allowed to
    /// replace it. `file` is the target's file name.
    TargetExists { file: String },
    /// A folder is at the target: a project kept as a folder, most likely. The
    /// import writes one file, and no platform renames a file over a folder, so
    /// it is refused whether or not replacing was allowed. `file` is the
    /// target's name.
    TargetIsFolder { file: String },
    /// Writing the new project failed.
    NotWritten {
        /// The target's file name.
        file: String,
        /// The folder that refused the write: the target's own, or the system's
        /// temporary folder the project is assembled in first. A full path,
        /// because it is the one thing the writer has to act on.
        folder: String,
        /// Whether that folder ran out of space (or out of the writer's quota).
        disk_full: bool,
        /// The system's own words for what went wrong.
        reason: String,
    },
}

impl fmt::Display for OutputRefused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputRefused::TargetExists { file } => write!(
                f,
                "'{file}' already exists (choose another name or allow overwrite)"
            ),
            OutputRefused::TargetIsFolder { file } => write!(
                f,
                "'{file}' is a folder, which an import cannot replace (choose another name)"
            ),
            OutputRefused::NotWritten {
                file,
                folder,
                disk_full: true,
                reason,
            } => write!(
                f,
                "'{file}' could not be written: no space is left in '{folder}' ({reason})"
            ),
            OutputRefused::NotWritten {
                file,
                folder,
                disk_full: false,
                reason,
            } => write!(f, "'{file}' could not be written in '{folder}': {reason}"),
        }
    }
}

impl std::error::Error for OutputRefused {}

/// The `.skrib` an import is writing, staged beside its target until it is whole.
///
/// See the module doc. Created before the source is even opened, so the one
/// question that needs no reading, whether the target may be replaced, is
/// answered first; and so a leftover `<target>.importing` from an import that was
/// killed outright is swept up by the next import to the same file.
#[must_use = "dropping the guard removes the staged file"]
pub struct StagedOutput {
    target: PathBuf,
    staged: PathBuf,
    committed: bool,
}

impl StagedOutput {
    /// Stage an import to `target`, refusing an existing target unless
    /// `overwrite` allows replacing it, and a folder at the target always.
    pub fn begin(target: &str, overwrite: bool) -> Result<Self, OutputRefused> {
        let target_path = PathBuf::from(target);
        // Asked here, before the source is opened: [`Self::commit`]'s rename
        // would fail on a folder only once the whole project had been read and
        // converted.
        if target_path.is_dir() {
            return Err(OutputRefused::TargetIsFolder {
                file: file_name(&target_path),
            });
        }
        if !overwrite && target_path.exists() {
            return Err(OutputRefused::TargetExists {
                file: file_name(&target_path),
            });
        }
        Ok(Self {
            staged: PathBuf::from(format!("{target}.importing")),
            target: target_path,
            committed: false,
        })
    }

    /// The file the bundle is written to before it is moved into place.
    pub fn staged_path(&self) -> &Path {
        &self.staged
    }

    /// Write `bundle`, as a `.skrib` zip, to the staged file.
    ///
    /// The same steps as `write_bundle` for a zip, taken one by one so a failure
    /// names the folder it happened in: the bundle is assembled in the system's
    /// temporary folder, then zipped into a temporary file beside the target, then
    /// renamed onto the staged path, durably.
    pub fn write(&self, bundle: &WorkBundle) -> Result<(), OutputRefused> {
        let assembly = std::env::temp_dir();
        let staging = tempfile::tempdir().map_err(|e| self.refused(&assembly, &e.into()))?;
        crate::folder_io::write_folder(staging.path(), bundle)
            .map_err(|e| self.refused(&assembly, &e))?;

        let beside = self
            .staged
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."))
            .to_path_buf();
        let mut zipped =
            NamedTempFile::new_in(&beside).map_err(|e| self.refused(&beside, &e.into()))?;
        crate::zip_io::zip_dir(staging.path(), zipped.as_file_mut())
            .map_err(|e| self.refused(&beside, &e))?;
        crate::writer::persist_durably(zipped, &self.staged).map_err(|e| self.refused(&beside, &e))
    }

    /// Move the staged file over the target: the import is done.
    pub fn commit(mut self) -> Result<(), OutputRefused> {
        match std::fs::rename(&self.staged, &self.target) {
            Ok(()) => {
                self.committed = true;
                Ok(())
            }
            Err(e) => {
                let folder = self
                    .target
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .unwrap_or(Path::new("."))
                    .to_path_buf();
                Err(self.refused(&folder, &e.into()))
            }
        }
    }

    /// A write that failed in `folder` for `error`.
    fn refused(&self, folder: &Path, error: &anyhow::Error) -> OutputRefused {
        OutputRefused::NotWritten {
            file: file_name(&self.target),
            folder: folder.display().to_string(),
            disk_full: out_of_space(error),
            reason: format!("{error:#}"),
        }
    }
}

impl Drop for StagedOutput {
    fn drop(&mut self) {
        if !self.committed {
            // Nothing to report to: this runs on the way out of an import that
            // has already failed, or was cancelled, and the file may never have
            // been created at all.
            let _ = std::fs::remove_file(&self.staged);
        }
    }
}

/// Whether `error` is, anywhere in its chain, the file system running out of
/// space or out of the writer's quota.
fn out_of_space(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause.downcast_ref::<std::io::Error>().is_some_and(|io| {
            matches!(
                io.kind(),
                std::io::ErrorKind::StorageFull | std::io::ErrorKind::QuotaExceeded
            )
        })
    })
}

/// `path`'s own name, or the whole path when it has none.
fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bundle() -> WorkBundle {
        crate::tests::build_bundle(crate::ShapeTag::Zip)
    }

    #[test]
    fn a_title_goes_where_the_pattern_says() {
        let notes = format!("Notes de {TITLE_SLOT}");
        assert_eq!(fill_title(&notes, "Chapitre 1"), "Notes de Chapitre 1");
        assert_eq!(
            fill_title(&format!("{TITLE_SLOT} (notes)"), "Scene"),
            "Scene (notes)"
        );
        // A name with nothing in it leaves no stray space.
        assert_eq!(fill_title(&format!("{TITLE_SLOT} (notes)"), ""), "(notes)");
        // A pattern with no slot has the name before it; an empty one is the name.
        assert_eq!(fill_title("(notes)", "Scene"), "Scene (notes)");
        assert_eq!(fill_title("", "Scene"), "Scene");
        // One pass: a name holding the slot's character is never filled into.
        let odd = format!("A{TITLE_SLOT}B");
        assert_eq!(fill_title(&notes, &odd), format!("Notes de {odd}"));
    }

    #[test]
    fn an_existing_target_is_refused_unless_it_may_be_replaced() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"PK").expect("target");
        let target = target.to_string_lossy().into_owned();
        assert_eq!(
            StagedOutput::begin(&target, false).err(),
            Some(OutputRefused::TargetExists {
                file: "novel.skrib".to_string()
            })
        );
        assert!(StagedOutput::begin(&target, true).is_ok());
    }

    /// A folder at the target (a project kept as a folder, say) cannot be
    /// replaced by the zip an import writes: the final rename fails on every
    /// platform, after the whole project has been read and converted. So it is
    /// refused before any of that, whatever the overwrite answer, and left as it
    /// was.
    #[test]
    fn a_folder_at_the_target_is_refused_before_anything_is_read() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("novel.skrib");
        std::fs::create_dir(&target).expect("a folder project");
        std::fs::write(target.join("project.skrib"), b"(manifest)").expect("its manifest");
        let target_s = target.to_string_lossy().into_owned();
        for overwrite in [false, true] {
            assert_eq!(
                StagedOutput::begin(&target_s, overwrite).err(),
                Some(OutputRefused::TargetIsFolder {
                    file: "novel.skrib".to_string()
                }),
                "overwrite {overwrite}: a folder is not a file an import may replace"
            );
        }
        assert!(target.join("project.skrib").is_file(), "left as it was");
    }

    #[test]
    fn a_committed_output_replaces_the_target_and_leaves_nothing_beside_it() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"old").expect("target");
        let output = StagedOutput::begin(&target.to_string_lossy(), true).expect("may replace it");
        output.write(&bundle()).expect("written");
        assert!(output.staged_path().is_file());
        output.commit().expect("committed");
        assert_ne!(std::fs::read(&target).expect("target"), b"old");
        assert_eq!(
            std::fs::read_dir(dir.path()).expect("dir").count(),
            1,
            "only the project itself"
        );
    }

    /// Dropped before it was committed, on any path out of an import: the staged
    /// file goes, and a project already at the target is untouched.
    #[test]
    fn an_uncommitted_output_leaves_nothing_behind() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"old").expect("target");
        {
            let output =
                StagedOutput::begin(&target.to_string_lossy(), true).expect("may replace it");
            output.write(&bundle()).expect("written");
            assert!(output.staged_path().is_file());
        }
        assert_eq!(std::fs::read(&target).expect("target"), b"old");
        assert_eq!(std::fs::read_dir(dir.path()).expect("dir").count(), 1);
    }

    /// A rename that fails (a folder with something in it appeared at the target
    /// while the import ran) reports the target's folder, and still leaves no
    /// staged file behind.
    #[test]
    fn a_failed_commit_names_the_folder_and_leaves_nothing_behind() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("novel.skrib");
        let output = StagedOutput::begin(&target.to_string_lossy(), true).expect("allowed");
        output.write(&bundle()).expect("written");
        std::fs::create_dir(&target).expect("a folder in the way");
        std::fs::write(target.join("keep"), b"x").expect("its content");
        let staged = output.staged_path().to_path_buf();
        match output.commit() {
            Err(OutputRefused::NotWritten {
                file,
                folder,
                disk_full,
                ..
            }) => {
                assert_eq!(file, "novel.skrib");
                assert_eq!(folder, dir.path().display().to_string());
                assert!(!disk_full);
            }
            other => panic!("expected a refusal, got {other:?}"),
        }
        assert!(!staged.exists(), "the staged file went with the guard");
        assert!(target.join("keep").is_file(), "what was there is untouched");
    }

    /// A folder that cannot be written in is named in the refusal.
    #[test]
    fn a_write_into_a_missing_folder_names_that_folder() {
        let dir = tempfile::tempdir().expect("tempdir");
        let folder = dir.path().join("gone");
        let target = folder.join("novel.skrib");
        let output = StagedOutput::begin(&target.to_string_lossy(), false).expect("allowed");
        match output.write(&bundle()) {
            Err(OutputRefused::NotWritten { folder: named, .. }) => {
                assert_eq!(named, folder.display().to_string());
            }
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[test]
    fn running_out_of_space_is_recognised_anywhere_in_the_chain() {
        let full = anyhow::Error::new(std::io::Error::from(std::io::ErrorKind::StorageFull))
            .context("zipping")
            .context("writing");
        assert!(out_of_space(&full));
        let quota = anyhow::Error::new(std::io::Error::from(std::io::ErrorKind::QuotaExceeded));
        assert!(out_of_space(&quota));
        let denied = anyhow::Error::new(std::io::Error::from(std::io::ErrorKind::PermissionDenied));
        assert!(!out_of_space(&denied));
    }
}
