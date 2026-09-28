// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a project importer's error toast says when its long operation fails.
//!
//! Not a view-model: one function the Plume and Manuskript import view-models
//! share, so the two toasts cannot drift apart. Each passes its own Fluent
//! wording for the one failure the writer gets in their language, and nothing
//! else differs between them.
//!
//! A long operation reports its failure as a string, so a project refused for
//! nesting its XML past `skrib_format::MAX_XML_DEPTH` arrives spelled as
//! `XmlTooDeep::failure_message`, and one refused for XML declaring an entity as
//! `XmlDeclaresEntities::failure_message` (see `import_management`'s `failure`
//! module). [`failure_text`](crate::shared::import_failure::failure_text) turns
//! either back into the typed value. Any other failure is the backend's own chain,
//! which is data (paths, parser output), so it stays `lit!`: translating the
//! importers' other messages is a separate decision. The Manuskript toast
//! recognises one refusal more before it asks here, `FoldersTooDeep`, which only
//! its outline can raise.

use skrib_format::{XmlDeclaresEntities, XmlTooDeep};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// What the error toast says for a failed import's `message`, and what its
/// **Details** shows.
///
/// A depth refusal is worded by `nested_too_deep` and an entity refusal by
/// `declares_entities`, the importer's own sentences, with the English
/// particulars behind **Details**. Anything else is shown as the backend said
/// it, in the body and behind **Details** alike.
pub(crate) fn failure_text(
    message: &str,
    nested_too_deep: impl FnOnce(&XmlTooDeep) -> LocalizedString,
    declares_entities: impl FnOnce(&XmlDeclaresEntities) -> LocalizedString,
) -> (LocalizedString, String) {
    if let Some(refused) = XmlTooDeep::from_failure_message(message) {
        return (nested_too_deep(&refused), refused.to_string());
    }
    if let Some(refused) = XmlDeclaresEntities::from_failure_message(message) {
        return (declares_entities(&refused), refused.to_string());
    }
    (lit!(message.to_string()), message.to_string())
}

/// Check an importer's wording of a depth refusal against the shipped `.ftl`
/// files of both locales: the sentence names `part` and the ceiling, fills every
/// argument, and never shows the wire form.
///
/// `fr-FR` is not checked at compile time (`tr!` validates keys against `en-US`
/// only), so a French message with a misspelt argument or a missing key compiles,
/// and only resolving it says so.
#[cfg(test)]
pub(crate) fn assert_worded_in_both_locales(
    part: &str,
    nested_too_deep: impl Fn(&XmlTooDeep) -> LocalizedString,
) {
    let refused = XmlTooDeep {
        part: part.to_string(),
        depth: skrib_format::MAX_XML_DEPTH + 1,
        line: 3,
    };
    let message = refused.failure_message();
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            let (body, details) = failure_text(&message, &nested_too_deep, |_| {
                panic!("a depth refusal is worded as one")
            });
            let body = body.resolve_now();
            assert!(body.contains(part), "{locale}: {body}");
            assert!(
                body.contains(&skrib_format::MAX_XML_DEPTH.to_string()),
                "{locale}: {body}"
            );
            assert!(
                !body.contains("{$") && !body.contains("{ $"),
                "{locale} left an argument unfilled: {body}"
            );
            assert!(
                !body.contains("xml-nested-too-deep"),
                "{locale} showed the wire form: {body}"
            );
            assert_eq!(details, refused.to_string(), "{locale}");
        });
    }
}

/// Check an importer's wording of an entity refusal against the shipped `.ftl`
/// files of both locales: the sentence names `part`, fills every argument, and
/// never shows the wire form. The same reasoning as
/// [`assert_worded_in_both_locales`].
#[cfg(test)]
pub(crate) fn assert_entity_refusal_worded_in_both_locales(
    part: &str,
    declares_entities: impl Fn(&XmlDeclaresEntities) -> LocalizedString,
) {
    let refused = XmlDeclaresEntities {
        part: part.to_string(),
        line: 2,
    };
    let message = refused.failure_message();
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            let (body, details) = failure_text(
                &message,
                |_| panic!("an entity refusal is worded as one"),
                &declares_entities,
            );
            let body = body.resolve_now();
            assert!(body.contains(part), "{locale}: {body}");
            assert!(
                !body.contains("{$") && !body.contains("{ $"),
                "{locale} left an argument unfilled: {body}"
            );
            assert!(
                !body.contains("xml-declares-entities"),
                "{locale} showed the wire form: {body}"
            );
            assert_eq!(details, refused.to_string(), "{locale}");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The refusal is recovered whole and handed to the importer's wording; the
    /// particulars go behind **Details**.
    #[test]
    fn a_nesting_refusal_is_worded_by_the_importer() {
        let refused = XmlTooDeep {
            part: "world.opml".to_string(),
            depth: 257,
            line: 3,
        };
        let (body, details) = failure_text(
            &refused.failure_message(),
            |seen| {
                assert_eq!(seen, &refused, "the refusal must arrive whole");
                lit!(format!("worded: {}", seen.part))
            },
            |_| panic!("a depth refusal is worded as one"),
        );
        assert_eq!(body.resolve_now(), "worded: world.opml");
        assert!(details.contains("world.opml") && details.contains("line 3"));
    }

    /// An entity refusal is recovered whole and handed to the importer's own
    /// wording for it, the particulars behind **Details**.
    #[test]
    fn an_entity_refusal_is_worded_by_the_importer() {
        let refused = XmlDeclaresEntities {
            part: "tree".to_string(),
            line: 2,
        };
        let (body, details) = failure_text(
            &refused.failure_message(),
            |_| panic!("an entity refusal is worded as one"),
            |seen| {
                assert_eq!(seen, &refused, "the refusal must arrive whole");
                lit!(format!("worded: {}", seen.part))
            },
        );
        assert_eq!(body.resolve_now(), "worded: tree");
        assert!(details.contains("tree") && details.contains("line 2"));
    }

    #[test]
    fn any_other_failure_is_shown_as_the_backend_said_it() {
        let (body, details) = failure_text(
            "reading 'x.msk': permission denied",
            |_| panic!("only a refusal is worded by the importer"),
            |_| panic!("only a refusal is worded by the importer"),
        );
        assert_eq!(body.resolve_now(), "reading 'x.msk': permission denied");
        assert_eq!(details, "reading 'x.msk': permission denied");
    }
}
