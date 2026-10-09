// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Plume Creator importer says in its own words, in the writer's language.
//!
//! Not a view-model: the sentences for the failures only a Plume project meets, or
//! whose wording is Plume's own; every other failure is the shared wording of
//! `shared::import_failure`, and what an import that went through has to tell is the
//! shared wording of the importers' notices (`shared::import_warnings`). The tests
//! below resolve every one in both shipped locales, since `fr-FR` is only checked by
//! resolving it. The data a sentence quotes (a part's name, a parser's own words)
//! stays as it was written.

use frontend::import_management::failure::FailureReason;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// The Plume importer's own sentence for a failure only a Plume project meets,
/// or for one whose wording is its own; `None` leaves it to the shared wording.
pub(crate) fn own_failure(reason: &FailureReason) -> Option<LocalizedString> {
    Some(match reason {
        FailureReason::NestedTooDeep { part, .. } => tr!(import_plume_nested_too_deep(
            part = part.clone(),
            limit = skrib_format::MAX_XML_DEPTH as i64
        )),
        FailureReason::DeclaresEntities { part, .. } => {
            tr!(import_plume_declares_entities(part = part.clone()))
        }
        FailureReason::PlumeNoOutline => tr!(import_plume_no_outline()),
        FailureReason::PlumeEmptyOutline => tr!(import_plume_empty_outline()),
        FailureReason::PlumeWrongDocument { part, tag } => tr!(import_plume_wrong_document(
            part = part.clone(),
            tag = tag.clone()
        )),
        FailureReason::PlumeTooNew {
            part,
            version,
            supported,
        } => tr!(import_plume_too_new(
            part = part.clone(),
            version = version.clone(),
            supported = supported.clone()
        )),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The failures only a Plume project meets, and its own wording of a part
    /// nested too deep or declaring an XML entity, are the importer's; every
    /// other one is the shared wording, and all of them read in both locales.
    #[test]
    fn every_failure_is_worded_and_the_plume_ones_in_plume_s_words() {
        crate::shared::import_failure::assert_every_failure_worded(own_failure, |reason| {
            matches!(
                reason,
                FailureReason::NestedTooDeep { .. }
                    | FailureReason::DeclaresEntities { .. }
                    | FailureReason::PlumeNoOutline
                    | FailureReason::PlumeEmptyOutline
                    | FailureReason::PlumeWrongDocument { .. }
                    | FailureReason::PlumeTooNew { .. }
            )
        });
    }

    /// The depth refusal names the part and the ceiling, as it did before it
    /// crossed the wire typed.
    #[test]
    fn a_nesting_refusal_names_the_part_and_the_ceiling() {
        for locale in ["en-US", "fr-FR"] {
            crate::test_support::with_shipped_messages(locale, || {
                let body = own_failure(&FailureReason::NestedTooDeep {
                    part: "tree".into(),
                    depth: 257,
                    line: 3,
                })
                .map(|l| l.resolve_now())
                .unwrap_or_default();
                assert!(body.contains("tree"), "{locale}: {body}");
                assert!(
                    body.contains(&skrib_format::MAX_XML_DEPTH.to_string()),
                    "{locale}: {body}"
                );
            });
        }
    }

    /// A project refused for declaring an entity reaches the writer as a sentence
    /// in their language, naming the part, in both shipped locales.
    #[test]
    fn an_entity_refusal_is_worded_for_the_writer_in_both_locales() {
        crate::shared::import_failure::assert_entity_refusal_worded_in_both_locales(
            "tree",
            own_failure,
        );
    }
}
