// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Manuskript importer says in its own words, in the writer's language.
//!
//! Not a view-model: the sentences for the failures only a Manuskript project
//! meets, or whose wording is Manuskript's own; every other failure is the shared
//! wording of `shared::import_failure`, and what an import that went through has to
//! tell is the shared wording of the importers' notices (`shared::import_warnings`).
//! The tests below resolve every one in both shipped locales, since `fr-FR` is only
//! checked by resolving it. The data a sentence quotes (a file inside the project
//! named as the project names it, what a parser said) stays as it was written.

use frontend::import_management::failure::FailureReason;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// The Manuskript importer's own sentence for a failure only a Manuskript project
/// meets, or for one whose wording is its own; `None` leaves it to the shared
/// wording.
pub(crate) fn own_failure(reason: &FailureReason) -> Option<LocalizedString> {
    let count = |n: u64| i64::try_from(n).unwrap_or(i64::MAX);
    Some(match reason {
        FailureReason::NestedTooDeep { part, .. } => tr!(import_manuskript_nested_too_deep(
            part = part.clone(),
            limit = skrib_format::MAX_XML_DEPTH as i64
        )),
        FailureReason::DeclaresEntities { part, .. } => {
            tr!(import_manuskript_declares_entities(part = part.clone()))
        }
        // The path itself is behind Details: at that depth it runs to hundreds of
        // folder names.
        FailureReason::FoldersTooDeep { .. } => tr!(import_manuskript_folders_too_deep(
            limit = skrib_format::MAX_XML_DEPTH as i64
        )),
        FailureReason::ManuskriptNotAProjectFile { file } => {
            tr!(import_manuskript_not_a_project_file(file = file.clone()))
        }
        FailureReason::ManuskriptFolderMissing { stub, folder } => {
            tr!(import_manuskript_folder_missing(
                stub = stub.clone(),
                folder = folder.clone()
            ))
        }
        FailureReason::ManuskriptNotAProject => tr!(import_manuskript_not_a_project()),
        FailureReason::ManuskriptFormatTooNew { version, supported } => {
            tr!(import_manuskript_format_too_new(
                version = count(*version),
                supported = count(*supported)
            ))
        }
        FailureReason::ManuskriptTooManyFiles { limit } => {
            tr!(import_manuskript_too_many_files(limit = count(*limit)))
        }
        FailureReason::ManuskriptTooLarge { limit_mib } => {
            tr!(import_manuskript_too_large(limit = count(*limit_mib)))
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The failures only a Manuskript project meets, and its own wording of a
    /// file nested too deep or declaring an XML entity, are the importer's; every
    /// other one is the shared wording, and all of them read in both locales.
    #[test]
    fn every_failure_is_worded_and_the_manuskript_ones_in_manuskript_s_words() {
        crate::shared::import_failure::assert_every_failure_worded(own_failure, |reason| {
            matches!(
                reason,
                FailureReason::NestedTooDeep { .. }
                    | FailureReason::DeclaresEntities { .. }
                    | FailureReason::FoldersTooDeep { .. }
                    | FailureReason::ManuskriptNotAProjectFile { .. }
                    | FailureReason::ManuskriptFolderMissing { .. }
                    | FailureReason::ManuskriptNotAProject
                    | FailureReason::ManuskriptFormatTooNew { .. }
                    | FailureReason::ManuskriptTooManyFiles { .. }
                    | FailureReason::ManuskriptTooLarge { .. }
            )
        });
    }

    /// The folder refusal names the ceiling and keeps the path, which at that
    /// depth runs to hundreds of folder names, out of the sentence.
    #[test]
    fn a_folder_refusal_names_the_ceiling_but_not_the_path() {
        for locale in ["en-US", "fr-FR"] {
            crate::test_support::with_shipped_messages(locale, || {
                let body = own_failure(&FailureReason::FoldersTooDeep {
                    part: format!("outline/{}0-Scene.md", "0-Part/".repeat(300)),
                    depth: 301,
                })
                .map(|l| l.resolve_now())
                .unwrap_or_default();
                assert!(
                    body.contains(&skrib_format::MAX_XML_DEPTH.to_string()),
                    "{locale}: {body}"
                );
                assert!(!body.contains("0-Part/"), "{locale}: {body}");
            });
        }
    }

    /// A project refused for declaring an entity reaches the writer as a sentence
    /// in their language, naming the file, in both shipped locales.
    #[test]
    fn an_entity_refusal_is_worded_for_the_writer_in_both_locales() {
        crate::shared::import_failure::assert_entity_refusal_worded_in_both_locales(
            "world.opml",
            own_failure,
        );
    }
}
