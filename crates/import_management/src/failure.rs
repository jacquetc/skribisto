// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How a project import's failure crosses to the interface, typed.
//!
//! A long operation reports failure as a string (`OperationStatus::Failed`, taken from
//! the error's `Display`), so whatever the UI is to act on has to survive as text.
//! `for_long_operation` finds the typed refusal inside the error, whichever crate
//! raised it (a converter's own `Refusal`, the staged output's, a depth, entity or
//! size refusal from `skrib_format`), and spells it as one tagged line of JSON;
//! [`ProjectImportFailure::from_failure_message`](crate::failure::ProjectImportFailure::from_failure_message)
//! reads it back. The interface words the
//! [`FailureReason`](crate::failure::FailureReason) in the writer's own language and
//! keeps the English chain,
//! [`ProjectImportFailure::details`](crate::failure::ProjectImportFailure::details),
//! behind **Details**.
//!
//! Shared by the Plume, Manuskript and Scrivener use cases, which all end in exactly this;
//! a module rather than a use case calling a use case. Hand-written, like `events`: the
//! failure channel is a plain `String` the generator knows nothing about.
//!
//! A Scrivener refusal crosses as the very row the analysis shows on the project card
//! ([`ScrivenerFinding`]), and a question as the question the wizard asks
//! ([`ScrivenerQuestion`]), so the interface words a refusal the same way before and
//! after the review. A project that changed since the review is its own reason, for the
//! interface to offer the review again.

use serde::{Deserialize, Serialize};
use skrib_format::ZipRefused;
use skrib_format::importing::{ImportCancelled, OutputRefused};

use crate::dtos::{ScrivenerFinding, ScrivenerQuestion};
use crate::scrivener::{self, ChoiceOutOfRange};

/// What a failed import message starts with, so a reader can tell the typed
/// spelling from a failure some other layer reported in its own words (a panic
/// caught by the operation manager, say).
const FAILURE_TAG: &str = "project-import-failed:";

/// A failed project import, as the interface receives it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectImportFailure {
    /// Why, for the interface to say in the writer's language.
    pub reason: FailureReason,
    /// The whole technical chain, in English, for **Details**.
    pub details: String,
}

/// Why a project was not imported.
///
/// Every value carries only what its sentence needs: a file named by its name, a
/// part of the project named as the project names it, a count, a limit. A
/// `reason` field is what the system or a parser said, for the interface to quote
/// inside a sentence of its own. A variant an importer alone raises is named after
/// it; the rest can come from any importer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailureReason {
    /// The writer cancelled the import. Nothing was written.
    Cancelled,
    /// A file is already at the target, and replacing it was not allowed.
    TargetExists { file: String },
    /// A folder is at the target (a project kept as a folder, most likely),
    /// which the one file an import writes cannot replace.
    TargetIsFolder { file: String },
    /// The new project could not be written in `folder` (the destination, or the
    /// temporary folder it is assembled in).
    NotWritten {
        file: String,
        folder: String,
        reason: String,
    },
    /// `folder` ran out of space (or out of the writer's quota) while the new
    /// project was being written there.
    DiskFull { file: String, folder: String },
    /// Nothing is at the chosen path.
    SourceMissing { file: String },
    /// A file or folder of the source project could not be read.
    Unreadable { file: String, reason: String },
    /// The source is an archive that could not be read.
    ArchiveDamaged { file: String, reason: String },
    /// The source archive lists more entries than a project may hold.
    ArchiveTooManyEntries { entries: u64, limit: u64 },
    /// One member of the source archive unpacks past what it may.
    ArchiveMemberTooLarge { part: String, limit_mib: u64 },
    /// The source archive unpacks past what it may, altogether.
    ArchiveTooLarge { limit_mib: u64 },
    /// A part of the project nests its XML past the ceiling.
    NestedTooDeep { part: String, depth: u64, line: u64 },
    /// A part of the project declares an XML entity, which expanded as often as
    /// the part names it could take more memory than the computer has.
    DeclaresEntities { part: String, line: u64 },
    /// A file of the project sits more folders deep than the ceiling.
    FoldersTooDeep { part: String, depth: u64 },
    /// A part of the project is not well-formed XML.
    Malformed { part: String, reason: String },
    /// A Plume Creator zip with no outline in it.
    PlumeNoOutline,
    /// An old-style Plume Creator project whose outline file is empty.
    PlumeEmptyOutline,
    /// A part that is not the Plume Creator document it should be.
    PlumeWrongDocument { part: String, tag: String },
    /// A part written by a newer Plume Creator than the importer knows.
    PlumeTooNew {
        part: String,
        version: String,
        supported: String,
    },
    /// A `.msk` that is neither an archive nor the file pointing at a folder.
    ManuskriptNotAProjectFile { file: String },
    /// A `.msk` pointing at a project folder that is not beside it.
    ManuskriptFolderMissing { stub: String, folder: String },
    /// Nothing that makes a Manuskript project.
    ManuskriptNotAProject,
    /// A Manuskript format newer than the importer knows.
    ManuskriptFormatTooNew { version: u64, supported: u64 },
    /// More files than a Manuskript project may hold.
    ManuskriptTooManyFiles { limit: u64 },
    /// Larger than a Manuskript project may be, in mebibytes.
    ManuskriptTooLarge { limit_mib: u64 },
    /// The Scrivener project cannot be imported: the refusal, keyed and typed as the
    /// analysis gives it.
    ScrivenerRefused { refusal: ScrivenerFinding },
    /// The Scrivener project asks a question the review did not answer (a manifest or a
    /// project appeared since): the review must be run again.
    ScrivenerQuestionFirst { question: ScrivenerQuestion },
    /// Questions the writer must accept before importing were not accepted.
    ScrivenerNotAccepted { questions: Vec<ScrivenerQuestion> },
    /// The Scrivener project changed since the review (its fingerprint moved): the review
    /// must be run again, and nothing was written.
    ScrivenerChangedSinceReview,
    /// A structure choice names a depth the review does not offer.
    ScrivenerDepthNotOffered { depth: i64 },
    /// A structure choice names a binder row that is not a root item the review offers.
    ScrivenerRootNotOffered { item: i64 },
    /// Something no refusal accounts for: `detail` is the technical chain.
    Internal { detail: String },
}

impl ProjectImportFailure {
    /// A failure the typed spelling does not cover, reported in `message`'s own
    /// words: an operation that panicked, or an import that could not start.
    pub fn internal(message: &str) -> Self {
        Self {
            reason: FailureReason::Internal {
                detail: message.to_string(),
            },
            details: message.to_string(),
        }
    }

    /// This failure as the one line a long operation carries.
    pub fn failure_message(&self) -> String {
        // A struct of strings, integers and unit variants: nothing in it can fail
        // to serialise, and `internal` would say so if it ever did.
        match serde_json::to_string(self) {
            Ok(json) => format!("{FAILURE_TAG}{json}"),
            Err(e) => format!("{}: {e}", self.details),
        }
    }

    /// Read a failure back from a long operation's message, or `None` when the
    /// message is not the typed spelling.
    pub fn from_failure_message(message: &str) -> Option<Self> {
        serde_json::from_str(message.strip_prefix(FAILURE_TAG)?).ok()
    }

    /// Read a failure back from a long operation's message, whatever spelled it:
    /// the typed spelling, or anything else as [`FailureReason::Internal`].
    pub fn from_any_message(message: &str) -> Self {
        Self::from_failure_message(message).unwrap_or_else(|| Self::internal(message))
    }
}

/// The message a failed import hands the `LongOperationManager`: the typed
/// refusal inside `error`, with the whole `{:#}` chain as its details.
///
/// The chain matters: the manager records only `e.to_string()`, which for a plain
/// `anyhow` error is the outermost context alone, losing the root cause.
pub(crate) fn for_long_operation(error: anyhow::Error) -> anyhow::Error {
    let failure = ProjectImportFailure {
        reason: reason_of(&error),
        details: format!("{error:#}"),
    };
    anyhow::anyhow!(failure.failure_message())
}

/// The typed refusal inside `error`, wherever in its chain it sits.
fn reason_of(error: &anyhow::Error) -> FailureReason {
    if let Some(refused) = skrib_format::xml_depth::too_deep(error) {
        return FailureReason::NestedTooDeep {
            part: refused.part.clone(),
            depth: refused.depth as u64,
            line: refused.line as u64,
        };
    }
    if let Some(refused) = skrib_format::xml_depth::declares_entities(error) {
        return FailureReason::DeclaresEntities {
            part: refused.part.clone(),
            line: refused.line as u64,
        };
    }
    if let Some(refused) = skrib_format::xml_depth::folders_too_deep(error) {
        return FailureReason::FoldersTooDeep {
            part: refused.part.clone(),
            depth: refused.depth as u64,
        };
    }
    if let Some(refused) = skrib_format::zip_guard::refused(error) {
        return archive_refused(refused);
    }
    for cause in error.chain() {
        if cause.downcast_ref::<ImportCancelled>().is_some() {
            return FailureReason::Cancelled;
        }
        if let Some(refused) = cause.downcast_ref::<OutputRefused>() {
            return output_refused(refused);
        }
        if let Some(refused) = cause.downcast_ref::<plume_import::Refusal>() {
            return plume_refused(refused);
        }
        if let Some(refused) = cause.downcast_ref::<manuskript_import::Refusal>() {
            return manuskript_refused(refused);
        }
        if let Some(refused) = cause.downcast_ref::<scrivener_import::ConvertError>() {
            return scrivener_refused(refused);
        }
        if let Some(out) = cause.downcast_ref::<ChoiceOutOfRange>() {
            return scrivener_choice_out_of_range(out);
        }
    }
    FailureReason::Internal {
        detail: format!("{error:#}"),
    }
}

fn archive_refused(refused: &ZipRefused) -> FailureReason {
    match refused {
        ZipRefused::TooManyEntries { entries, limit } => FailureReason::ArchiveTooManyEntries {
            entries: *entries as u64,
            limit: *limit as u64,
        },
        ZipRefused::MemberTooLarge { part, .. } => FailureReason::ArchiveMemberTooLarge {
            part: part.clone(),
            limit_mib: refused.limit_mib().unwrap_or(1),
        },
        ZipRefused::ArchiveTooLarge { .. } => FailureReason::ArchiveTooLarge {
            limit_mib: refused.limit_mib().unwrap_or(1),
        },
    }
}

fn output_refused(refused: &OutputRefused) -> FailureReason {
    match refused {
        OutputRefused::TargetExists { file } => FailureReason::TargetExists { file: file.clone() },
        OutputRefused::TargetIsFolder { file } => {
            FailureReason::TargetIsFolder { file: file.clone() }
        }
        OutputRefused::NotWritten {
            file,
            folder,
            disk_full: true,
            ..
        } => FailureReason::DiskFull {
            file: file.clone(),
            folder: folder.clone(),
        },
        OutputRefused::NotWritten {
            file,
            folder,
            disk_full: false,
            reason,
        } => FailureReason::NotWritten {
            file: file.clone(),
            folder: folder.clone(),
            reason: reason.clone(),
        },
    }
}

fn plume_refused(refused: &plume_import::Refusal) -> FailureReason {
    use plume_import::Refusal;
    match refused.clone() {
        Refusal::Unreadable { file, reason } => FailureReason::Unreadable { file, reason },
        Refusal::Damaged { file, reason } => FailureReason::ArchiveDamaged { file, reason },
        Refusal::NoOutline => FailureReason::PlumeNoOutline,
        Refusal::EmptyOutline => FailureReason::PlumeEmptyOutline,
        Refusal::WrongDocument { part, tag } => FailureReason::PlumeWrongDocument { part, tag },
        Refusal::TooNew {
            part,
            version,
            supported,
        } => FailureReason::PlumeTooNew {
            part,
            version,
            supported,
        },
        Refusal::Malformed { part, reason } => FailureReason::Malformed { part, reason },
    }
}

fn manuskript_refused(refused: &manuskript_import::Refusal) -> FailureReason {
    use manuskript_import::Refusal;
    match refused.clone() {
        Refusal::Missing { file } => FailureReason::SourceMissing { file },
        Refusal::Unreadable { file, reason } => FailureReason::Unreadable { file, reason },
        Refusal::Damaged { file, reason } => FailureReason::ArchiveDamaged { file, reason },
        Refusal::NotAProjectFile { file } => FailureReason::ManuskriptNotAProjectFile { file },
        Refusal::FolderMissing { stub, folder } => {
            FailureReason::ManuskriptFolderMissing { stub, folder }
        }
        Refusal::NotAProject => FailureReason::ManuskriptNotAProject,
        Refusal::FormatTooNew { version, supported } => FailureReason::ManuskriptFormatTooNew {
            version: u64::from(version),
            supported: u64::from(supported),
        },
        Refusal::TooManyFiles { limit } => FailureReason::ManuskriptTooManyFiles { limit },
        Refusal::TooLarge { limit_mib } => FailureReason::ManuskriptTooLarge { limit_mib },
    }
}

/// Why a Scrivener conversion wrote nothing.
fn scrivener_refused(refused: &scrivener_import::ConvertError) -> FailureReason {
    use scrivener_import::ConvertError;
    use scrivener_import::structure::{ChoiceError, PlanError};
    match refused {
        ConvertError::Refused(refusal) => FailureReason::ScrivenerRefused {
            refusal: scrivener::refusal(refusal),
        },
        ConvertError::Choose(question) => FailureReason::ScrivenerQuestionFirst {
            question: scrivener::question(question),
        },
        ConvertError::NotAccepted(questions) => FailureReason::ScrivenerNotAccepted {
            questions: questions.iter().map(scrivener::question).collect(),
        },
        ConvertError::ChangedSinceReview => FailureReason::ScrivenerChangedSinceReview,
        ConvertError::Plan(PlanError::Choice(ChoiceError::DepthOutOfRange { depth })) => {
            FailureReason::ScrivenerDepthNotOffered {
                depth: i64::from(*depth),
            }
        }
        ConvertError::Plan(PlanError::Choice(ChoiceError::RootNotOffered { item })) => {
            FailureReason::ScrivenerRootNotOffered {
                item: i64::from(*item),
            }
        }
        // Any other broken plan is the importer's defect, refused as the analysis refuses
        // one: under the same key, with the broken invariant behind Details.
        ConvertError::Plan(error) => FailureReason::ScrivenerRefused {
            refusal: scrivener::refusal(&scrivener_import::Refusal::StructureUnmappable {
                error: error.clone(),
            }),
        },
        ConvertError::Output(refused) => output_refused(refused),
        ConvertError::Cancelled => FailureReason::Cancelled,
    }
}

fn scrivener_choice_out_of_range(out: &ChoiceOutOfRange) -> FailureReason {
    match out {
        ChoiceOutOfRange::Depth(depth) => FailureReason::ScrivenerDepthNotOffered { depth: *depth },
        ChoiceOutOfRange::Item(item) => FailureReason::ScrivenerRootNotOffered { item: *item },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use skrib_format::xml_depth::XmlError;
    use skrib_format::{FoldersTooDeep, XmlDeclaresEntities, XmlTooDeep};

    /// The message a failed use case hands the manager for `error`, read back as
    /// the interface reads it.
    fn received(error: anyhow::Error) -> ProjectImportFailure {
        let message = for_long_operation(error).to_string();
        match ProjectImportFailure::from_failure_message(&message) {
            Some(failure) => failure,
            None => panic!("not the typed spelling: {message}"),
        }
    }

    #[test]
    fn a_depth_refusal_reaches_the_ui_as_the_typed_value() {
        let refused = XmlTooDeep {
            part: "world.opml".to_string(),
            depth: 257,
            line: 4,
        };
        let error = anyhow::Error::new(refused.clone()).context("reading the project");
        let failure = received(error);
        assert_eq!(
            failure.reason,
            FailureReason::NestedTooDeep {
                part: "world.opml".to_string(),
                depth: 257,
                line: 4,
            }
        );
        assert!(failure.details.contains("reading the project"));
    }

    #[test]
    fn an_entity_refusal_reaches_the_ui_as_the_typed_value() {
        let refused = XmlDeclaresEntities {
            part: "tree".to_string(),
            line: 2,
        };
        let error = anyhow::Error::new(XmlError::DeclaresEntities(refused))
            .context("reading the Plume outline (tree)");
        let failure = received(error);
        assert_eq!(
            failure.reason,
            FailureReason::DeclaresEntities {
                part: "tree".to_string(),
                line: 2,
            }
        );
        assert!(failure.details.contains("reading the Plume outline (tree)"));
    }

    #[test]
    fn a_folder_refusal_reaches_the_ui_as_the_typed_value() {
        let refused = FoldersTooDeep {
            part: format!("outline/{}x.md", "0-a/".repeat(300)),
            depth: 301,
        };
        let error = anyhow::Error::new(refused.clone()).context("opening the project");
        assert_eq!(
            received(error).reason,
            FailureReason::FoldersTooDeep {
                part: refused.part,
                depth: 301,
            }
        );
    }

    #[test]
    fn each_converter_s_refusal_reaches_the_ui_as_the_typed_value() {
        let plume = anyhow::Error::new(plume_import::Refusal::TooNew {
            part: "tree".to_string(),
            version: "0.9".to_string(),
            supported: "0.5".to_string(),
        })
        .context("reading the Plume outline (tree)");
        assert_eq!(
            received(plume).reason,
            FailureReason::PlumeTooNew {
                part: "tree".to_string(),
                version: "0.9".to_string(),
                supported: "0.5".to_string(),
            }
        );
        let manuskript = anyhow::Error::new(manuskript_import::Refusal::FolderMissing {
            stub: "Novel.msk".to_string(),
            folder: "Novel".to_string(),
        });
        assert_eq!(
            received(manuskript).reason,
            FailureReason::ManuskriptFolderMissing {
                stub: "Novel.msk".to_string(),
                folder: "Novel".to_string(),
            }
        );
    }

    /// Running out of space names the folder that ran out, and nothing else.
    #[test]
    fn a_full_disk_reaches_the_ui_naming_its_folder() {
        let error = anyhow::Error::new(OutputRefused::NotWritten {
            file: "novel.skrib".to_string(),
            folder: "/tmp".to_string(),
            disk_full: true,
            reason: "No space left on device (os error 28)".to_string(),
        });
        assert_eq!(
            received(error).reason,
            FailureReason::DiskFull {
                file: "novel.skrib".to_string(),
                folder: "/tmp".to_string(),
            }
        );
    }

    /// A folder at the target is its own reason: the writer has to pick another
    /// name, which "already exists" would not tell them.
    #[test]
    fn a_folder_at_the_target_reaches_the_ui_as_its_own_reason() {
        let error = anyhow::Error::new(OutputRefused::TargetIsFolder {
            file: "novel.skrib".to_string(),
        });
        assert_eq!(
            received(error).reason,
            FailureReason::TargetIsFolder {
                file: "novel.skrib".to_string()
            }
        );
    }

    #[test]
    fn a_size_refusal_of_the_archive_reaches_the_ui_in_mebibytes() {
        let error = anyhow::Error::new(ZipRefused::ArchiveTooLarge { limit: 512 << 20 })
            .context("reading the archive");
        assert_eq!(
            received(error).reason,
            FailureReason::ArchiveTooLarge { limit_mib: 512 }
        );
    }

    #[test]
    fn any_other_failure_keeps_its_whole_chain() {
        let error = anyhow::anyhow!("root cause").context("outer");
        let failure = received(error);
        match &failure.reason {
            FailureReason::Internal { detail } => {
                assert!(detail.contains("outer") && detail.contains("root cause"));
            }
            other => panic!("expected an internal failure, got {other:?}"),
        }
        assert!(failure.details.contains("root cause"));
    }

    /// A message nothing here spelled (a panic the operation manager caught) is
    /// still a failure the interface can word, with the message as its detail.
    #[test]
    fn a_message_in_other_words_is_read_as_internal() {
        let failure = ProjectImportFailure::from_any_message("operation panicked: boom");
        assert_eq!(
            failure.reason,
            FailureReason::Internal {
                detail: "operation panicked: boom".to_string()
            }
        );
        assert_eq!(
            ProjectImportFailure::from_failure_message("operation panicked: boom"),
            None
        );
    }

    /// Every reason survives the trip through the one string a failed long
    /// operation carries, unchanged.
    #[test]
    fn every_reason_survives_the_failure_channel() {
        for reason in every_reason() {
            let failure = ProjectImportFailure {
                reason,
                details: "a: b: c".to_string(),
            };
            let message = anyhow::anyhow!(failure.failure_message()).to_string();
            assert_eq!(
                ProjectImportFailure::from_failure_message(&message),
                Some(failure)
            );
        }
    }

    /// One of each reason, every variant present: [`slot`] is exhaustive, so a new
    /// variant fails to compile until it has one, and this list is checked against
    /// it below.
    fn every_reason() -> Vec<FailureReason> {
        let s = |t: &str| t.to_string();
        vec![
            FailureReason::Cancelled,
            FailureReason::TargetExists { file: s("a.skrib") },
            FailureReason::TargetIsFolder { file: s("a.skrib") },
            FailureReason::NotWritten {
                file: s("a.skrib"),
                folder: s("/b"),
                reason: s("denied"),
            },
            FailureReason::DiskFull {
                file: s("a.skrib"),
                folder: s("/tmp"),
            },
            FailureReason::SourceMissing { file: s("a.msk") },
            FailureReason::Unreadable {
                file: s("a"),
                reason: s("denied"),
            },
            FailureReason::ArchiveDamaged {
                file: s("a.plume"),
                reason: s("bad crc"),
            },
            FailureReason::ArchiveTooManyEntries {
                entries: 9,
                limit: 8,
            },
            FailureReason::ArchiveMemberTooLarge {
                part: s("tree"),
                limit_mib: 256,
            },
            FailureReason::ArchiveTooLarge { limit_mib: 512 },
            FailureReason::NestedTooDeep {
                part: s("tree"),
                depth: 257,
                line: 3,
            },
            FailureReason::DeclaresEntities {
                part: s("tree"),
                line: 2,
            },
            FailureReason::FoldersTooDeep {
                part: s("outline/a"),
                depth: 257,
            },
            FailureReason::Malformed {
                part: s("tree"),
                reason: s("unexpected end"),
            },
            FailureReason::PlumeNoOutline,
            FailureReason::PlumeEmptyOutline,
            FailureReason::PlumeWrongDocument {
                part: s("tree"),
                tag: s("html"),
            },
            FailureReason::PlumeTooNew {
                part: s("tree"),
                version: s("0.9"),
                supported: s("0.5"),
            },
            FailureReason::ManuskriptNotAProjectFile { file: s("a.msk") },
            FailureReason::ManuskriptFolderMissing {
                stub: s("a.msk"),
                folder: s("a"),
            },
            FailureReason::ManuskriptNotAProject,
            FailureReason::ManuskriptFormatTooNew {
                version: 2,
                supported: 1,
            },
            FailureReason::ManuskriptTooManyFiles { limit: 100_000 },
            FailureReason::ManuskriptTooLarge { limit_mib: 1024 },
            FailureReason::Internal { detail: s("boom") },
            FailureReason::ScrivenerRefused {
                refusal: scrivener::refusal(&scrivener_import::Refusal::ScrivenerOneForMac),
            },
            FailureReason::ScrivenerQuestionFirst {
                question: ScrivenerQuestion::NoTextInCopy { text_items: 12 },
            },
            FailureReason::ScrivenerNotAccepted {
                questions: vec![ScrivenerQuestion::NoTextInCopy { text_items: 12 }],
            },
            FailureReason::ScrivenerChangedSinceReview,
            FailureReason::ScrivenerDepthNotOffered { depth: 7 },
            FailureReason::ScrivenerRootNotOffered { item: 3 },
        ]
    }

    /// A distinct number per variant, by an exhaustive match.
    fn slot(reason: &FailureReason) -> usize {
        match reason {
            FailureReason::Cancelled => 0,
            FailureReason::TargetExists { .. } => 1,
            FailureReason::NotWritten { .. } => 2,
            FailureReason::DiskFull { .. } => 3,
            FailureReason::SourceMissing { .. } => 4,
            FailureReason::Unreadable { .. } => 5,
            FailureReason::ArchiveDamaged { .. } => 6,
            FailureReason::ArchiveTooManyEntries { .. } => 7,
            FailureReason::ArchiveMemberTooLarge { .. } => 8,
            FailureReason::ArchiveTooLarge { .. } => 9,
            FailureReason::NestedTooDeep { .. } => 10,
            FailureReason::FoldersTooDeep { .. } => 11,
            FailureReason::Malformed { .. } => 12,
            FailureReason::PlumeNoOutline => 13,
            FailureReason::PlumeEmptyOutline => 14,
            FailureReason::PlumeWrongDocument { .. } => 15,
            FailureReason::PlumeTooNew { .. } => 16,
            FailureReason::ManuskriptNotAProjectFile { .. } => 17,
            FailureReason::ManuskriptFolderMissing { .. } => 18,
            FailureReason::ManuskriptNotAProject => 19,
            FailureReason::ManuskriptFormatTooNew { .. } => 20,
            FailureReason::ManuskriptTooManyFiles { .. } => 21,
            FailureReason::ManuskriptTooLarge { .. } => 22,
            FailureReason::Internal { .. } => 23,
            FailureReason::DeclaresEntities { .. } => 24,
            FailureReason::TargetIsFolder { .. } => 25,
            FailureReason::ScrivenerRefused { .. } => 26,
            FailureReason::ScrivenerQuestionFirst { .. } => 27,
            FailureReason::ScrivenerNotAccepted { .. } => 28,
            FailureReason::ScrivenerChangedSinceReview => 29,
            FailureReason::ScrivenerDepthNotOffered { .. } => 30,
            FailureReason::ScrivenerRootNotOffered { .. } => 31,
        }
    }

    #[test]
    fn the_list_of_reasons_holds_every_variant() {
        let slots: std::collections::BTreeSet<usize> = every_reason().iter().map(slot).collect();
        assert_eq!(slots, (0..=31).collect());
        assert_eq!(every_reason().len(), slots.len());
    }

    /// Every way a Scrivener conversion can end without writing reaches the interface as
    /// its own typed reason, read back from the one string the manager keeps.
    #[test]
    fn every_scrivener_conversion_error_reaches_the_ui_typed() {
        use scrivener_import::structure::{ChoiceError, PlanError};
        use scrivener_import::{ConvertError, Question, Refusal};
        let convert = |e: ConvertError| received(anyhow::Error::new(e));
        assert_eq!(
            convert(ConvertError::ChangedSinceReview).reason,
            FailureReason::ScrivenerChangedSinceReview
        );
        assert_eq!(
            convert(ConvertError::Cancelled).reason,
            FailureReason::Cancelled
        );
        let refused = convert(ConvertError::Refused(Refusal::NewerFormatStamp {
            stamp: 24,
            newest: 23,
        }));
        let FailureReason::ScrivenerRefused {
            refusal: ScrivenerFinding::Reported { key, args, .. },
        } = refused.reason
        else {
            panic!("expected a Scrivener refusal, got {:?}", refused.reason);
        };
        assert_eq!(key, "scrivener-refusal-newer-format-stamp");
        assert_eq!(
            args,
            vec![
                crate::dtos::ScrivenerArg::Count {
                    name: "stamp".into(),
                    value: 24
                },
                crate::dtos::ScrivenerArg::Count {
                    name: "newest".into(),
                    value: 23
                },
            ]
        );
        assert_eq!(
            convert(ConvertError::Choose(Question::NoTextInCopy {
                text_items: 40
            }))
            .reason,
            FailureReason::ScrivenerQuestionFirst {
                question: ScrivenerQuestion::NoTextInCopy { text_items: 40 }
            }
        );
        assert_eq!(
            convert(ConvertError::NotAccepted(vec![Question::NoTextInCopy {
                text_items: 40
            }]))
            .reason,
            FailureReason::ScrivenerNotAccepted {
                questions: vec![ScrivenerQuestion::NoTextInCopy { text_items: 40 }]
            }
        );
        assert_eq!(
            convert(ConvertError::Plan(PlanError::Choice(
                ChoiceError::DepthOutOfRange { depth: 5 }
            )))
            .reason,
            FailureReason::ScrivenerDepthNotOffered { depth: 5 }
        );
        assert_eq!(
            convert(ConvertError::Plan(PlanError::Choice(
                ChoiceError::RootNotOffered { item: 9 }
            )))
            .reason,
            FailureReason::ScrivenerRootNotOffered { item: 9 }
        );
        // A broken plan is refused under the analysis's own key for it.
        let broken = convert(ConvertError::Plan(PlanError::FactsMismatch {
            items: 3,
            facts: 2,
        }));
        assert!(
            matches!(
                &broken.reason,
                FailureReason::ScrivenerRefused {
                    refusal: ScrivenerFinding::Reported { key, .. }
                } if key == "scrivener-refusal-structure-unmappable"
            ),
            "{:?}",
            broken.reason
        );
        assert_eq!(
            convert(ConvertError::Output(OutputRefused::TargetExists {
                file: "novel.skrib".into()
            }))
            .reason,
            FailureReason::TargetExists {
                file: "novel.skrib".into()
            }
        );
        // A choice out of every range is refused by name before anything is read.
        assert_eq!(
            received(anyhow::Error::new(ChoiceOutOfRange::Depth(-1))).reason,
            FailureReason::ScrivenerDepthNotOffered { depth: -1 }
        );
        assert_eq!(
            received(anyhow::Error::new(ChoiceOutOfRange::Item(-4))).reason,
            FailureReason::ScrivenerRootNotOffered { item: -4 }
        );
    }
}
