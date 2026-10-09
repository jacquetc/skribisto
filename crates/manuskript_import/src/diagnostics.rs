// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Why a Manuskript project was not imported, as values rather than sentences.
//!
//! The converter never words anything a writer reads. It refuses a project with a
//! [`Refusal`], and records everything it had to drop, guess at or read around as
//! [`ImportNotice`](skrib_format::import_notice::ImportNotice)s; the interface says
//! either in the writer's own language. A refusal carries the data its sentence needs
//! and nothing more: a limit, a file inside the project named as the project names
//! it, never a path into the writer's home folder (a file outside the project is
//! named by its file name alone).
//!
//! Where a reason comes from somewhere else (the system refusing a file, a parser
//! refusing a document), it is carried as those words, `reason`, for the interface
//! to quote inside a sentence of its own.
//!
//! A refusal's `Display` is English for the logs and for the technical details
//! behind a failure's **Details**; nothing shows it as the reason itself.

use std::fmt;

/// Why a Manuskript project was not imported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// Nothing is at the chosen path.
    Missing { file: String },
    /// A file or folder of the project could not be read. `reason` is the
    /// system's own words.
    Unreadable { file: String, reason: String },
    /// A `.msk` archive that could not be read. `reason` is the archive reader's
    /// own words.
    Damaged { file: String, reason: String },
    /// A `.msk` that is neither an archive nor the small file that points at the
    /// project folder beside it.
    NotAProjectFile { file: String },
    /// A `.msk` that points at a project folder, `folder`, which is not beside it.
    FolderMissing { stub: String, folder: String },
    /// Nothing that makes a Manuskript project: no marker, no outline.
    NotAProject,
    /// A format newer than this importer knows.
    FormatTooNew { version: u32, supported: u32 },
    /// More files than a project may hold.
    TooManyFiles { limit: u64 },
    /// Larger than a project may be, in mebibytes.
    TooLarge { limit_mib: u64 },
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::Missing { file } => write!(f, "'{file}' does not exist"),
            Refusal::Unreadable { file, reason } => write!(f, "reading '{file}': {reason}"),
            Refusal::Damaged { file, reason } => {
                write!(f, "'{file}' could not be read as an archive: {reason}")
            }
            Refusal::NotAProjectFile { file } => {
                write!(
                    f,
                    "'{file}' is neither a Manuskript archive nor a project file"
                )
            }
            Refusal::FolderMissing { stub, folder } => write!(
                f,
                "'{stub}' is a pointer to the project folder beside it, and '{folder}' is \
                 missing. A Manuskript project in folder mode is both, and they have to travel \
                 together."
            ),
            Refusal::NotAProject => f.write_str(
                "this does not look like a Manuskript project: no MANUSKRIPT marker, no \
                 outline/ folder and no outline.xml",
            ),
            Refusal::FormatTooNew { version, supported } => write!(
                f,
                "this Manuskript project's format is version {version}, newer than this \
                 importer supports ({supported})"
            ),
            Refusal::TooManyFiles { limit } => {
                write!(f, "this project holds more than {limit} files")
            }
            Refusal::TooLarge { limit_mib } => write!(
                f,
                "this project is larger than the {limit_mib} MiB this importer will read"
            ),
        }
    }
}

impl std::error::Error for Refusal {}

/// The name a refusal or a notice gives a path outside the project: its file
/// name, or the path as it is when it has none.
pub(crate) fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// `error`, a failure to read `path`, as a refusal naming it.
pub(crate) fn unreadable(path: &std::path::Path, error: &std::io::Error) -> anyhow::Error {
    anyhow::Error::new(Refusal::Unreadable {
        file: file_name(path),
        reason: error.to_string(),
    })
}
