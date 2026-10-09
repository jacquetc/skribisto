// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Why a Plume Creator project was not imported, as values rather than
//! sentences.
//!
//! The converter never words anything a writer reads. It refuses a project with
//! a [`Refusal`], and records what it had to drop or guess at as
//! [`ImportNotice`](skrib_format::import_notice::ImportNotice)s; the interface
//! says either in the writer's own language. A refusal carries the data its
//! sentence needs and nothing more: a name the project itself gave, a file
//! name, never a path to the writer's home folder.
//!
//! A refusal's `Display` is English for the logs and for the technical details
//! behind a failure's **Details**; nothing shows it as the reason itself.

use std::fmt;

/// Why a Plume Creator project was not imported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// A file of the project could not be read. `file` is its name; `reason`
    /// the system's own words.
    Unreadable { file: String, reason: String },
    /// The chosen file is a zip archive that could not be read. `reason` is the
    /// archive reader's own words.
    Damaged { file: String, reason: String },
    /// A zip archive with no outline in it: not a Plume Creator project.
    NoOutline,
    /// An old-style project whose outline file is empty.
    EmptyOutline,
    /// A part that is not the Plume document it should be: its outermost
    /// element is `tag`. `part` is Plume's own name for it (`tree`,
    /// `attendance`, `info`).
    WrongDocument { part: String, tag: String },
    /// A part written by a newer Plume Creator than this importer knows.
    TooNew {
        part: String,
        version: String,
        supported: String,
    },
    /// A part that is not well-formed XML. `reason` is the parser's own words.
    Malformed { part: String, reason: String },
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::Unreadable { file, reason } => write!(f, "reading '{file}': {reason}"),
            Refusal::Damaged { file, reason } => {
                write!(
                    f,
                    "'{file}' is a zip archive that could not be read: {reason}"
                )
            }
            Refusal::NoOutline => f.write_str(
                "this file is a zip but has no `tree` member: not a Plume Creator project",
            ),
            Refusal::EmptyOutline => {
                f.write_str("the Plume tree file is empty: not a Plume Creator project")
            }
            Refusal::WrongDocument { part, tag } => {
                write!(f, "<{tag}> is not a Plume Creator {part} document")
            }
            Refusal::TooNew {
                part,
                version,
                supported,
            } => write!(
                f,
                "this Plume project's {part} is version {version}, newer than this importer \
                 supports ({supported})"
            ),
            Refusal::Malformed { part, reason } => {
                write!(f, "the Plume {part} is not well-formed XML: {reason}")
            }
        }
    }
}

impl std::error::Error for Refusal {}

/// The name a refusal or a notice gives `path`: its file name, or the path as it
/// is when it has none.
pub(crate) fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// `error`, a failure to read `path`, as a refusal naming the file.
pub(crate) fn unreadable(path: &std::path::Path, error: &std::io::Error) -> anyhow::Error {
    anyhow::Error::new(Refusal::Unreadable {
        file: file_name(path),
        reason: error.to_string(),
    })
}
