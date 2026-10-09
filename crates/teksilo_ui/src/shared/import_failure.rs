// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a project import's error toast says when its long operation fails.
//!
//! Not a view-model: one function the shared import job
//! (`shared::project_import`) calls for every importer, so no two importers can
//! word the same failure two ways.
//!
//! A long operation reports its failure as a string. The backend spells a
//! refused project as a typed `ProjectImportFailure` (see `import_management`'s
//! `failure` module), which is read back here and worded in the writer's
//! language: a reason in the body, the English technical chain behind
//! **Details**. A failure the backend did not type (an operation that panicked,
//! an import that could not start) arrives as `FailureReason::Internal` and is
//! still a translated sentence, with the system's own words quoted inside it.
//! Nothing reaches the writer as a bare "import failed".
//!
//! An importer words the reasons that are its own (a Plume Creator project with
//! no outline, a Manuskript `.msk` without its folder, and its own wording of a
//! project nested too deep or declaring an XML entity); everything any importer
//! can meet is worded here.

use frontend::import_management::failure::{FailureReason, ProjectImportFailure};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// An importer's own sentence for a failure it words itself, or `None` for the
/// shared wording.
pub(crate) type OwnWords = fn(&FailureReason) -> Option<LocalizedString>;

/// What the error toast says for `failure`, and what its **Details** shows.
///
/// `own` is the importer's sentence for the reasons that are its own; every
/// other reason is worded here.
pub(crate) fn failure_text(
    failure: &ProjectImportFailure,
    own: OwnWords,
) -> (LocalizedString, String) {
    let body = own(&failure.reason).unwrap_or_else(|| shared_words(failure));
    (body, failure.details.clone())
}

/// The shared sentence for `failure`'s reason.
fn shared_words(failure: &ProjectImportFailure) -> LocalizedString {
    let count = |n: u64| i64::try_from(n).unwrap_or(i64::MAX);
    match &failure.reason {
        FailureReason::Cancelled => tr!(project_import_failed_cancelled()),
        FailureReason::TargetExists { file } => {
            tr!(project_import_failed_target_exists(file = file.clone()))
        }
        FailureReason::TargetIsFolder { file } => {
            tr!(project_import_failed_target_folder(file = file.clone()))
        }
        FailureReason::NotWritten {
            file,
            folder,
            reason,
        } => tr!(project_import_failed_not_written(
            file = file.clone(),
            folder = folder.clone(),
            reason = reason.clone()
        )),
        FailureReason::DiskFull { file, folder } => tr!(project_import_failed_disk_full(
            file = file.clone(),
            folder = folder.clone()
        )),
        FailureReason::SourceMissing { file } => {
            tr!(project_import_failed_source_missing(file = file.clone()))
        }
        FailureReason::Unreadable { file, reason } => tr!(project_import_failed_unreadable(
            file = file.clone(),
            reason = reason.clone()
        )),
        FailureReason::ArchiveDamaged { file, reason } => {
            tr!(project_import_failed_archive_damaged(
                file = file.clone(),
                reason = reason.clone()
            ))
        }
        FailureReason::ArchiveTooManyEntries { entries, limit } => {
            tr!(project_import_failed_archive_entries(
                entries = count(*entries),
                limit = count(*limit)
            ))
        }
        FailureReason::ArchiveMemberTooLarge { part, limit_mib } => {
            tr!(project_import_failed_archive_member(
                part = part.clone(),
                limit = count(*limit_mib)
            ))
        }
        FailureReason::ArchiveTooLarge { limit_mib } => {
            tr!(project_import_failed_archive_size(
                limit = count(*limit_mib)
            ))
        }
        FailureReason::NestedTooDeep { part, .. } => tr!(project_import_failed_nested(
            part = part.clone(),
            limit = skrib_format::MAX_XML_DEPTH as i64
        )),
        FailureReason::DeclaresEntities { part, .. } => {
            tr!(project_import_failed_entities(part = part.clone()))
        }
        FailureReason::FoldersTooDeep { .. } => tr!(project_import_failed_folders(
            limit = skrib_format::MAX_XML_DEPTH as i64
        )),
        FailureReason::Malformed { part, reason } => tr!(project_import_failed_malformed(
            part = part.clone(),
            reason = reason.clone()
        )),
        // An importer's own reasons, reaching the shared wording only if an
        // importer ever met another's: worded as what they are underneath, a
        // project that could not be read, with the technical words quoted.
        FailureReason::PlumeNoOutline
        | FailureReason::PlumeEmptyOutline
        | FailureReason::PlumeWrongDocument { .. }
        | FailureReason::PlumeTooNew { .. }
        | FailureReason::ManuskriptNotAProjectFile { .. }
        | FailureReason::ManuskriptFolderMissing { .. }
        | FailureReason::ManuskriptNotAProject
        | FailureReason::ManuskriptFormatTooNew { .. }
        | FailureReason::ManuskriptTooManyFiles { .. }
        | FailureReason::ManuskriptTooLarge { .. }
        // A choice the review never offers: the defect the sentence below is for.
        | FailureReason::ScrivenerDepthNotOffered { .. }
        | FailureReason::ScrivenerRootNotOffered { .. } => tr!(project_import_failed_internal(
            detail = failure.details.clone()
        )),
        // What the Scrivener importer answers rather than meets: until its wizard words
        // each refusal itself, what is true of all of them, and what to do next.
        FailureReason::ScrivenerRefused { .. } => tr!(project_import_failed_scrivener_refused()),
        FailureReason::ScrivenerQuestionFirst { .. } => {
            tr!(project_import_failed_scrivener_question())
        }
        FailureReason::ScrivenerNotAccepted { .. } => {
            tr!(project_import_failed_scrivener_not_accepted())
        }
        FailureReason::ScrivenerChangedSinceReview => {
            tr!(project_import_failed_scrivener_changed())
        }
        FailureReason::Internal { detail } => {
            tr!(project_import_failed_internal(detail = detail.clone()))
        }
    }
}

/// One of each failure reason, every variant present: [`reason_slot`] is
/// exhaustive, so a new variant fails to compile until it has one, and the test
/// below checks this list against it. For the tests of every surface that words
/// a failure.
#[cfg(test)]
pub(crate) fn every_reason() -> Vec<FailureReason> {
    let s = |t: &str| t.to_string();
    vec![
        FailureReason::Cancelled,
        FailureReason::TargetExists { file: s("a.skrib") },
        FailureReason::TargetIsFolder { file: s("a.skrib") },
        FailureReason::NotWritten {
            file: s("a.skrib"),
            folder: s("/books"),
            reason: s("Permission denied (os error 13)"),
        },
        FailureReason::DiskFull {
            file: s("a.skrib"),
            folder: s("/tmp"),
        },
        FailureReason::SourceMissing { file: s("a.msk") },
        FailureReason::Unreadable {
            file: s("a.plume"),
            reason: s("Permission denied (os error 13)"),
        },
        FailureReason::ArchiveDamaged {
            file: s("a.plume"),
            reason: s("invalid checksum"),
        },
        FailureReason::ArchiveTooManyEntries {
            entries: 200_001,
            limit: 100_000,
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
            reason: s("unexpected end of stream"),
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
        FailureReason::Internal {
            detail: s("operation panicked: boom"),
        },
        FailureReason::ScrivenerRefused {
            refusal: frontend::import_management::ScrivenerFinding::Reported {
                key: s("scrivener-refusal-scrivener-one-for-mac"),
                family: frontend::import_management::ScrivenerFindingFamily::Refusal,
                group: frontend::import_management::ScrivenerReportGroup::Unfiled,
                place: frontend::import_management::ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: Vec::new(),
            },
        },
        FailureReason::ScrivenerQuestionFirst {
            question: frontend::import_management::ScrivenerQuestion::NoTextInCopy {
                text_items: 12,
            },
        },
        FailureReason::ScrivenerNotAccepted {
            questions: vec![
                frontend::import_management::ScrivenerQuestion::NoTextInCopy { text_items: 12 },
            ],
        },
        FailureReason::ScrivenerChangedSinceReview,
        FailureReason::ScrivenerDepthNotOffered { depth: 7 },
        FailureReason::ScrivenerRootNotOffered { item: 3 },
    ]
}

/// A distinct number per reason, by an exhaustive match.
#[cfg(test)]
fn reason_slot(reason: &FailureReason) -> usize {
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

/// Word every failure reason with `own` in both shipped locales, and check each
/// sentence reads as a sentence: resolved (not the message id), every argument
/// filled, and the importer's own words used where `expect_own` says they are.
///
/// `fr-FR` is not checked at compile time (`tr!` validates keys against `en-US`
/// only), so a French message with a misspelt argument or a missing key compiles,
/// and only resolving it says so.
#[cfg(test)]
pub(crate) fn assert_every_failure_worded(own: OwnWords, expect_own: fn(&FailureReason) -> bool) {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            for reason in every_reason() {
                let failure = ProjectImportFailure {
                    reason: reason.clone(),
                    details: "the technical chain".to_string(),
                };
                let (body, details) = failure_text(&failure, own);
                let body = body.resolve_now();
                assert!(
                    !body.trim().is_empty() && !body.starts_with("project-import-"),
                    "{locale} {reason:?}: {body}"
                );
                assert!(
                    !body.starts_with("import-plume-") && !body.starts_with("import-manuskript-"),
                    "{locale} {reason:?} resolved to its own id: {body}"
                );
                assert!(
                    !body.contains("{$") && !body.contains("{ $"),
                    "{locale} {reason:?} left an argument unfilled: {body}"
                );
                assert_eq!(details, "the technical chain", "{locale} {reason:?}");
                assert_eq!(
                    own(&reason).is_some(),
                    expect_own(&reason),
                    "{locale}: which reasons the importer words itself, {reason:?}"
                );
            }
        });
    }
}

/// Check an importer's own wording of a project refused for declaring an XML
/// entity against the shipped `.ftl` files of both locales: the importer words
/// it itself, and the sentence names `part`, fills every argument and is not a
/// message id, with the technical chain behind **Details**.
#[cfg(test)]
pub(crate) fn assert_entity_refusal_worded_in_both_locales(part: &str, own: OwnWords) {
    let failure = ProjectImportFailure {
        reason: FailureReason::DeclaresEntities {
            part: part.to_string(),
            line: 2,
        },
        details: format!("{part} declares an XML entity at line 2"),
    };
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            assert!(
                own(&failure.reason).is_some(),
                "{locale}: the importer words an entity refusal itself"
            );
            let (body, details) = failure_text(&failure, own);
            let body = body.resolve_now();
            assert!(body.contains(part), "{locale}: {body}");
            assert!(
                !body.contains("{$") && !body.contains("{ $"),
                "{locale} left an argument unfilled: {body}"
            );
            assert!(
                !body.contains("declares-entities"),
                "{locale} showed the message id: {body}"
            );
            assert_eq!(details, failure.details, "{locale}");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_of_reasons_holds_every_variant() {
        let slots: std::collections::BTreeSet<usize> =
            every_reason().iter().map(reason_slot).collect();
        assert_eq!(slots, (0..=31).collect());
        assert_eq!(every_reason().len(), slots.len());
    }

    /// With no importer's own words, every reason still reads as a sentence in
    /// both locales.
    #[test]
    fn every_reason_has_a_shared_sentence() {
        assert_every_failure_worded(|_| None, |_| false);
    }

    /// A Scrivener project refused, asking a question, not accepted or changed since its
    /// review is an answer the importer expected, never worded as an error it did not: in
    /// both locales each has a sentence of its own, which says nothing was written. Only
    /// a choice the review never offers is the defect the unexpected-error sentence is for.
    #[test]
    fn a_scrivener_answer_is_not_worded_as_an_unexpected_error() {
        let expected: Vec<FailureReason> = every_reason()
            .into_iter()
            .filter(|reason| {
                matches!(
                    reason,
                    FailureReason::ScrivenerRefused { .. }
                        | FailureReason::ScrivenerQuestionFirst { .. }
                        | FailureReason::ScrivenerNotAccepted { .. }
                        | FailureReason::ScrivenerChangedSinceReview
                )
            })
            .collect();
        assert_eq!(expected.len(), 4);
        for locale in ["en-US", "fr-FR"] {
            crate::test_support::with_shipped_messages(locale, || {
                let details = "the technical chain".to_string();
                let unexpected =
                    tr!(project_import_failed_internal(detail = details.clone())).resolve_now();
                let mut bodies = std::collections::BTreeSet::new();
                for reason in &expected {
                    let failure = ProjectImportFailure {
                        reason: reason.clone(),
                        details: details.clone(),
                    };
                    let body = failure_text(&failure, |_| None).0.resolve_now();
                    assert_ne!(body, unexpected, "{locale} {reason:?}");
                    assert!(!body.contains(&details), "{locale} {reason:?}: {body}");
                    bodies.insert(body);
                }
                // The review again for a question or a change, the refusal on its own.
                assert_eq!(bodies.len(), 4, "{locale}: {bodies:?}");
                for defect in [
                    FailureReason::ScrivenerDepthNotOffered { depth: 7 },
                    FailureReason::ScrivenerRootNotOffered { item: 3 },
                ] {
                    let failure = ProjectImportFailure {
                        reason: defect,
                        details: details.clone(),
                    };
                    let body = failure_text(&failure, |_| None).0.resolve_now();
                    assert_eq!(body, unexpected, "{locale}");
                }
            });
        }
    }

    /// A failure the backend did not type keeps its words, quoted inside a
    /// translated sentence, and behind Details.
    #[test]
    fn an_untyped_failure_is_quoted_in_a_sentence() {
        crate::test_support::with_real_messages(|| {
            let failure = ProjectImportFailure::from_any_message("operation panicked: boom");
            let (body, details) = failure_text(&failure, |_| None);
            let body = body.resolve_now();
            assert!(body.contains("operation panicked: boom"), "{body}");
            assert_ne!(
                body, "operation panicked: boom",
                "a sentence, not the chain"
            );
            assert_eq!(details, "operation panicked: boom");
        });
    }

    /// A project refused for declaring an XML entity, reaching the shared
    /// wording, names the part in both locales.
    #[test]
    fn an_entity_refusal_names_its_part_in_the_shared_words() {
        let failure = ProjectImportFailure {
            reason: FailureReason::DeclaresEntities {
                part: "world.opml".to_string(),
                line: 2,
            },
            details: String::new(),
        };
        for locale in ["en-US", "fr-FR"] {
            crate::test_support::with_shipped_messages(locale, || {
                let body = failure_text(&failure, |_| None).0.resolve_now();
                assert!(body.contains("world.opml"), "{locale}: {body}");
            });
        }
    }

    /// Running out of space says which folder ran out.
    #[test]
    fn a_full_disk_names_its_folder() {
        crate::test_support::with_real_messages(|| {
            let failure = ProjectImportFailure {
                reason: FailureReason::DiskFull {
                    file: "novel.skrib".to_string(),
                    folder: "/run/user/1000/tmp".to_string(),
                },
                details: String::new(),
            };
            let body = failure_text(&failure, |_| None).0.resolve_now();
            assert!(body.contains("/run/user/1000/tmp"), "{body}");
        });
    }

    /// The importer's own sentence wins over the shared one.
    #[test]
    fn an_importer_s_own_words_come_first() {
        let failure = ProjectImportFailure {
            reason: FailureReason::PlumeNoOutline,
            details: "d".to_string(),
        };
        let (body, _) = failure_text(&failure, |reason| {
            matches!(reason, FailureReason::PlumeNoOutline).then(|| lit!("own words"))
        });
        assert_eq!(body.resolve_now(), "own words");
    }
}
