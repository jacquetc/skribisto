// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The words an export writes into the book, in the **book's** language.
//!
//! A heading the exporter generates ("Chapter 3"), the byline under a title, the
//! word-count estimate on a title page: furniture the writer did not type, which
//! must read in the language the book is written in. That is the manuscript's
//! language (a scene's own, or the export style's fixed heading language), and
//! never the language the app's interface happens to be shown in — so this is a
//! catalogue of its own, beside the interface's, chosen per call by a language
//! tag rather than by the app's locale.
//!
//! Fluent rather than a Rust table so that word order, plural forms and spacing
//! are the translator's, not the code's: Japanese puts the number inside the word
//! (`第3章`), Hungarian before it (`3. fejezet`), and Romanian adds "de" from
//! twenty up. One file per language in `locales/`, named by its primary subtag;
//! `en` is the source and the fallback, for a language with no file and for a
//! message one file lacks.
//!
//! Bidi isolation marks are off. The interface bundle wraps every argument in
//! U+2068/U+2069 for the screen; in a book those characters would travel into the
//! DOCX, the EPUB and the PDF's text layer as invisible junk inside a heading.

use std::collections::HashMap;
use std::sync::LazyLock;

use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

/// The language the others fall back to, and the source the rest translate.
const SOURCE: &str = "en";

/// Every catalogue, by primary language subtag. A file added to `locales/` must be
/// listed here to be compiled in; the catalogue test fails on one that is not.
const CATALOGUES: &[(&str, &str)] = &[
    ("en", include_str!("../locales/en.ftl")),
    ("ar", include_str!("../locales/ar.ftl")),
    ("cs", include_str!("../locales/cs.ftl")),
    ("da", include_str!("../locales/da.ftl")),
    ("de", include_str!("../locales/de.ftl")),
    ("el", include_str!("../locales/el.ftl")),
    ("eo", include_str!("../locales/eo.ftl")),
    ("es", include_str!("../locales/es.ftl")),
    ("fi", include_str!("../locales/fi.ftl")),
    ("fr", include_str!("../locales/fr.ftl")),
    ("he", include_str!("../locales/he.ftl")),
    ("hu", include_str!("../locales/hu.ftl")),
    ("it", include_str!("../locales/it.ftl")),
    ("ja", include_str!("../locales/ja.ftl")),
    ("ko", include_str!("../locales/ko.ftl")),
    ("nb", include_str!("../locales/nb.ftl")),
    ("nl", include_str!("../locales/nl.ftl")),
    ("pl", include_str!("../locales/pl.ftl")),
    ("pt", include_str!("../locales/pt.ftl")),
    ("ro", include_str!("../locales/ro.ftl")),
    ("ru", include_str!("../locales/ru.ftl")),
    ("sv", include_str!("../locales/sv.ftl")),
    ("tr", include_str!("../locales/tr.ftl")),
    ("uk", include_str!("../locales/uk.ftl")),
];

static BUNDLES: LazyLock<HashMap<&'static str, FluentBundle<FluentResource>>> =
    LazyLock::new(|| {
        CATALOGUES
            .iter()
            .map(|(lang, source)| (*lang, bundle(lang, source)))
            .collect()
    });

fn bundle(lang: &str, source: &str) -> FluentBundle<FluentResource> {
    let langid: LanguageIdentifier = lang.parse().unwrap_or_default();
    let mut bundle = FluentBundle::new_concurrent(vec![langid]);
    bundle.set_use_isolating(false);
    // A file with a malformed entry still yields the entries that parsed; the
    // catalogue test is what refuses the malformed one, and an export is better off
    // with the rest of a language than with none of it.
    let resource = FluentResource::try_new(source.to_string()).unwrap_or_else(|(res, _)| res);
    // Only a duplicated id fails here, which the test refuses too.
    let _ = bundle.add_resource(resource);
    bundle
}

/// The catalogue a language tag reads from: its primary subtag, lowercased, with
/// Norwegian's three tags sharing the Bokmål file. `None` for a language with no
/// file of its own.
fn catalogue_for(lang: &str) -> Option<&'static str> {
    let primary = lang
        .trim()
        .split(['-', '_'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let primary = match primary.as_str() {
        "no" | "nn" => "nb".to_string(),
        _ => primary,
    };
    CATALOGUES.iter().map(|(l, _)| *l).find(|l| *l == primary)
}

/// Message `key` in the book's language `lang`, with `args`. Falls back to the
/// source language when `lang` has no catalogue or its catalogue lacks the key, and
/// to the empty string only if the source lacks it too, which the tests forbid.
pub(crate) fn text(lang: &str, key: &str, args: &[(&str, FluentValue<'_>)]) -> String {
    let mut fluent_args = FluentArgs::new();
    for (name, value) in args {
        fluent_args.set(*name, value.clone());
    }
    let try_lang = |l: &str| -> Option<String> {
        let bundle = BUNDLES.get(l)?;
        let pattern = bundle.get_message(key)?.value()?;
        let mut errors = Vec::new();
        Some(
            bundle
                .format_pattern(pattern, Some(&fluent_args), &mut errors)
                .into_owned(),
        )
    };
    catalogue_for(lang)
        .and_then(try_lang)
        .or_else(|| try_lang(SOURCE))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_syntax::ast;

    fn keys_of(source: &str) -> Vec<String> {
        let resource = fluent_syntax::parser::parse(source).expect("every catalogue must parse");
        resource
            .body
            .iter()
            .filter_map(|entry| match entry {
                ast::Entry::Message(m) => Some(m.id.name.to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn every_catalogue_parses_and_matches_the_source_keys() {
        let mut source = keys_of(CATALOGUES[0].1);
        source.sort();
        for (lang, text) in CATALOGUES {
            let mut keys = keys_of(text);
            keys.sort();
            assert_eq!(
                keys, source,
                "{lang} must define exactly the source's messages"
            );
        }
    }

    #[test]
    fn every_file_in_the_directory_is_compiled_in() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("locales");
        let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
            .expect("locales/ exists")
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.file_name()
                    .to_str()
                    .and_then(|n| n.strip_suffix(".ftl"))
                    .map(str::to_string)
            })
            .collect();
        on_disk.sort();
        let mut listed: Vec<String> = CATALOGUES.iter().map(|(l, _)| l.to_string()).collect();
        listed.sort();
        assert_eq!(on_disk, listed);
    }

    #[test]
    fn an_unknown_language_reads_the_source() {
        assert_eq!(
            text("tlh", "heading-chapter", &[("n", "3".into())]),
            "Chapter 3"
        );
        assert_eq!(text("", "heading-word-part", &[]), "Part");
    }

    #[test]
    fn a_region_or_script_tag_reads_its_language() {
        assert_eq!(
            text("fr-CA", "heading-chapter", &[("n", "3".into())]),
            "Chapitre 3"
        );
        assert_eq!(
            text("pt_BR", "heading-chapter", &[("n", "3".into())]),
            "Capítulo 3"
        );
        assert_eq!(
            text("nn", "heading-chapter", &[("n", "3".into())]),
            "Kapittel 3"
        );
    }

    #[test]
    fn the_number_goes_where_the_language_puts_it() {
        let n = [("n", FluentValue::from("3"))];
        assert_eq!(text("ja", "heading-chapter", &n), "第3章");
        assert_eq!(text("hu", "heading-chapter", &n), "3. fejezet");
        assert_eq!(text("tr", "heading-part", &n), "3. Kısım");
    }

    /// A book is not a screen: no U+2068/U+2069 may reach a heading or a byline.
    #[test]
    fn no_isolation_marks_reach_the_book() {
        for (lang, _) in CATALOGUES {
            let out = text(lang, "byline", &[("author", "Jules Verne".into())]);
            assert!(!out.contains(['\u{2068}', '\u{2069}']), "{lang}: {out:?}");
            assert!(out.contains("Jules Verne"), "{lang}: {out:?}");
        }
    }
}
