// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Drift tests for the help set.
//!
//! Modelled on [`crate::settings_keys`]'s walk-the-source-tree test and
//! [`crate::tooltip_registry`]'s cascade-link test, and for the same reason: a help
//! set rots quietly. A dead link, a topic whose French file was never written, a body
//! that stopped parsing, a file added to the directory that no topic includes — none of
//! those fail a build, and none of them are visible until a reader hits one.
//!
//! What these tests deliberately do **not** claim to check is whether a topic is still
//! *true*. Nothing here can tell that the prose describes a dialog that changed last
//! week. That is a human review cost, and pretending a test covers it would be the
//! exact failure this project keeps naming: a surface claiming what it cannot prove.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::*;
use teksilo::text_document::TextDocument;

/// Every locale the help set ships. Read from the directory rather than a list, so a
/// locale added on disk is checked without anyone remembering to name it here.
fn shipped_locales() -> Vec<String> {
    let mut locales: Vec<String> = std::fs::read_dir(help_dir())
        .expect("crates/teksilo_ui/help must exist")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry.file_type().ok()?.is_dir().then(|| {
                entry
                    .file_name()
                    .to_str()
                    .expect("a locale directory name must be UTF-8")
                    .to_string()
            })
        })
        .collect();
    locales.sort();
    locales
}

fn help_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("help")
}

/// Every `.djot` file on disk, as `(locale, file stem)`.
fn djot_files() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for locale in shipped_locales() {
        let dir = help_dir().join(&locale);
        for entry in std::fs::read_dir(&dir).expect("a locale directory must be readable") {
            let path = entry.expect("a directory entry must be readable").path();
            if path.extension().and_then(|e| e.to_str()) == Some("djot") {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .expect("a topic file name must be UTF-8")
                    .to_string();
                out.push((locale.clone(), stem));
            }
        }
    }
    out.sort();
    out
}

/// Extract every `(:key)` link target from a body, skipping fenced code so an example
/// showing the link syntax is not read as a link.
fn link_targets(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in source.lines() {
        let mut rest = line;
        while let Some(i) = rest.find("](:") {
            rest = &rest[i + 3..];
            let end = rest.find(')').unwrap_or(rest.len());
            out.push(rest[..end].to_string());
            rest = &rest[end..];
        }
    }
    out
}

#[test]
fn every_topic_key_is_unique() {
    let topics = builtin_topics();
    let mut seen = HashSet::new();
    for topic in &topics {
        assert!(
            seen.insert(topic.key),
            "help topic key '{}' is registered twice; a link cannot resolve to two pages",
            topic.key
        );
    }
}

#[test]
fn every_djot_topic_has_a_source_locale_body() {
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = &topic.body else {
            continue;
        };
        assert!(
            table.iter().any(|(locale, _)| *locale == SOURCE_LOCALE),
            "topic '{}' has no {SOURCE_LOCALE} body; that is the one locale every reader \
             can fall back to",
            topic.key
        );
    }
}

#[test]
fn every_shipped_locale_has_a_body_for_every_topic() {
    // A topic missing a locale is not a build error at runtime (it falls back to
    // English behind a visible banner), but it *is* something that must be a decision
    // rather than an oversight. Adding a locale directory is a promise to fill it.
    let locales = shipped_locales();
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = &topic.body else {
            continue;
        };
        for locale in &locales {
            assert!(
                table.iter().any(|(tag, _)| tag == locale),
                "topic '{}' has no body for locale '{locale}', which the help directory \
                 ships; add crates/teksilo_ui/help/{locale}/<topic>.djot and its row in \
                 help.rs, or delete the locale directory",
                topic.key
            );
        }
    }
}

#[test]
fn every_file_on_disk_is_reachable_from_a_topic() {
    // The mirror of the test above: a file nobody includes ships as dead weight and,
    // worse, reads as done. `include_str!` needs literal paths, so nothing but a test
    // can catch a file the table forgot.
    let compiled: HashSet<usize> = builtin_topics()
        .iter()
        .filter_map(|t| match &t.body {
            HelpBody::Djot(table) => Some(table.len()),
            _ => None,
        })
        .collect();
    let _ = compiled;

    let topic_stems: HashSet<String> = builtin_topics()
        .iter()
        .filter(|t| matches!(t.body, HelpBody::Djot(_)))
        .map(|t| {
            t.key
                .strip_prefix("help-")
                .expect("a built-in topic key starts with `help-`")
                .to_string()
        })
        .collect();

    for (locale, stem) in djot_files() {
        assert!(
            topic_stems.contains(&stem),
            "crates/teksilo_ui/help/{locale}/{stem}.djot is not included by any topic in \
             help.rs, so it ships unreachable"
        );
    }
}

#[test]
fn every_body_parses_as_djot() {
    // A malformed body renders as a blank page for the reader and says nothing in the
    // log. Parse every one of them here instead.
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = &topic.body else {
            continue;
        };
        for (locale, source) in table.iter() {
            let doc = TextDocument::new();
            assert!(
                doc.set_djot_sync(source).is_ok(),
                "topic '{}' ({locale}) does not parse as Djot",
                topic.key
            );
            assert!(
                !doc.to_plain_text().unwrap_or_default().trim().is_empty(),
                "topic '{}' ({locale}) parses to an empty document",
                topic.key
            );
        }
    }
}

#[test]
fn every_link_resolves_in_every_locale() {
    // The same contract `tooltip_registry`'s cascade test pins, extended across topic
    // bodies: a `[label](:key)` pointing at nothing renders as text that looks like a
    // link and does nothing when clicked.
    let known: HashSet<&str> = builtin_topics().iter().map(|t| t.key).collect();
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = &topic.body else {
            continue;
        };
        for (locale, source) in table.iter() {
            for target in link_targets(source) {
                assert!(
                    known.contains(target.as_str()),
                    "topic '{}' ({locale}) links to (:{target}), which no topic registers",
                    topic.key
                );
                assert_ne!(
                    target, topic.key,
                    "topic '{}' ({locale}) links to itself",
                    topic.key
                );
            }
        }
    }
}

#[test]
fn no_topic_uses_an_em_dash() {
    // House style, and not a matter of taste here: the em-dash is the single most
    // reliable tell of machine-drafted prose, and this help set is drafted with
    // assistance. Enforcing it in a test rather than a style note is the difference
    // between a rule and a hope.
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = &topic.body else {
            continue;
        };
        for (locale, source) in table.iter() {
            for (number, line) in source.lines().enumerate() {
                assert!(
                    !line.contains('\u{2014}'),
                    "topic '{}' ({locale}) line {} uses an em-dash, which the help set \
                     forbids: {line}",
                    topic.key,
                    number + 1
                );
            }
        }
    }
}

#[test]
fn no_tooltip_body_uses_an_em_dash() {
    // The same rule as the Djot topics, and it has to be a separate test because the
    // Tier-1 bodies do not live in the topic table at all: they are Fluent values, and
    // the test above walks `HelpBody::Djot` only. Rendering those values as help pages
    // is what brought them under the rule.
    //
    // `tooltip_registry`'s own module doc has said "keep the copy em-dash-free" since it
    // was written. It was never enforced, and 14 had accumulated across the two locales
    // by the time this test was added, which is the whole argument for the test.
    for (locale, ftl) in [
        ("en-US", include_str!("../../locales/en-US/tooltips.ftl")),
        ("fr-FR", include_str!("../../locales/fr-FR/tooltips.ftl")),
    ] {
        for (number, line) in ftl.lines().enumerate() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            assert!(
                !line.contains('\u{2014}'),
                "{locale}/tooltips.ftl line {} uses an em-dash, which the help set \
                 forbids: {line}",
                number + 1
            );
        }
    }
}

#[test]
fn no_tooltip_body_uses_markup_the_parser_cannot_read() {
    // The tooltip markup parser (text-typeset's `inline_markup`) understands exactly
    // three forms: `**bold**`, `*italic*` and `[label](url)`. Anything else is literal.
    //
    // Two consequences, and both had already shipped as wrong text:
    //
    //  * a backtick renders as a backtick, so `code` reads as `code` with the ticks;
    //  * a bare asterisk is an italic delimiter, so "* * *" is eaten. The scene-break
    //    entry told the reader to type "` *`" for two years because of exactly this,
    //    which is worse than a cosmetic bug: the sentence taught the wrong keystrokes.
    //
    // `#` is not a delimiter and stays literal, which is why the fixed copy uses it.
    for (locale, ftl) in [
        ("en-US", include_str!("../../locales/en-US/tooltips.ftl")),
        ("fr-FR", include_str!("../../locales/fr-FR/tooltips.ftl")),
    ] {
        for (number, line) in ftl.lines().enumerate() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            assert!(
                !line.contains('`'),
                "{locale}/tooltips.ftl line {}: the tooltip parser has no code spans, so \
                 a backtick renders as a backtick: {line}",
                number + 1
            );
            // A literal asterisk is only safe as a markup delimiter, i.e. in pairs with
            // no space just inside. The cheap, honest test is a space-flanked asterisk,
            // which is exactly the shape that gets eaten.
            assert!(
                !line.contains(" * "),
                "{locale}/tooltips.ftl line {}: a space-flanked asterisk is read as an \
                 italic delimiter and disappears; spell it out instead: {line}",
                number + 1
            );
        }
    }
}

#[test]
fn no_topic_opens_with_a_level_one_heading() {
    // The window prints the topic's title above the body, so a `# Title` in the source
    // would show it twice.
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = &topic.body else {
            continue;
        };
        for (locale, source) in table.iter() {
            for line in source.lines() {
                assert!(
                    !line.starts_with("# "),
                    "topic '{}' ({locale}) uses a level-one heading; the window already \
                     prints the title",
                    topic.key
                );
            }
        }
    }
}

#[test]
fn every_tooltip_backed_topic_names_a_registered_tooltip() {
    // A `HelpBody::Tooltip` whose key nothing registers renders an empty page. The
    // tooltip registry is a thread-local installed by the app, so this test reads the
    // same table the app installs rather than the registry itself.
    let known: HashSet<&str> = crate::tooltip_registry::all_keys().collect();
    let mut checked = 0;
    for topic in builtin_topics() {
        let HelpBody::Tooltip(key) = &topic.body else {
            continue;
        };
        checked += 1;
        assert!(
            known.contains(key),
            "topic '{}' renders tooltip '{key}', which tooltip_registry does not register",
            topic.key
        );
    }
    // Without this the test passes by checking nothing: it filters the built-ins down to
    // tooltip-backed ones, and for one build there were none, so a green run proved only
    // that the loop body never ran. Assert the tier is actually populated.
    assert!(
        checked > 0,
        "no built-in topic uses HelpBody::Tooltip, so this test asserts nothing"
    );
}

#[test]
fn every_registered_concept_is_browsable() {
    // The other direction, and the one that keeps the two tiers in step: a concept the
    // tooltip registry teaches but the Help window does not list is a concept you can
    // only find by already knowing which control to hover, which is exactly the gap the
    // window exists to close.
    let topics: HashSet<&str> = builtin_topics().iter().map(|t| t.key).collect();
    let missing: Vec<&str> = crate::tooltip_registry::all_keys()
        .filter(|key| !topics.contains(key))
        .collect();
    assert!(
        missing.is_empty(),
        "these registered concepts are not listed in the Help window: {missing:?}\n\
         Add each to help.rs::concept_topics with a title."
    );
}

#[test]
fn every_cascade_link_in_a_tooltip_body_resolves_to_a_topic() {
    // `tooltip_registry`'s own test already pins that every `[label](:key)` in the
    // tooltip corpus points at a registered tooltip. This pins the consequence of
    // *rendering* those bodies in the Help window: the same link now has to resolve to
    // a browsable topic too, or clicking a citation inside a glossary entry does
    // nothing.
    let topics: HashSet<&str> = builtin_topics().iter().map(|t| t.key).collect();
    for (locale, ftl) in [
        ("en-US", include_str!("../../locales/en-US/tooltips.ftl")),
        ("fr-FR", include_str!("../../locales/fr-FR/tooltips.ftl")),
    ] {
        for line in ftl.lines() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            let mut rest = line;
            while let Some(i) = rest.find("](:") {
                rest = &rest[i + 3..];
                let end = rest.find(')').unwrap_or(rest.len());
                let target = &rest[..end];
                assert!(
                    topics.contains(target),
                    "{locale}: tooltip cascade link (:{target}) resolves to no help topic"
                );
                rest = &rest[end..];
            }
        }
    }
}

#[test]
fn the_default_topic_exists() {
    assert!(
        builtin_topics().iter().any(|t| t.key == DEFAULT_TOPIC),
        "DEFAULT_TOPIC '{DEFAULT_TOPIC}' names no built-in topic, so opening Help lands \
         on the missing-topic message"
    );
}

// ── Locale resolution ───────────────────────────────────────────────────────

const TABLE: &[LocalizedSource] = &[("en-US", "english"), ("fr-FR", "francais")];

#[test]
fn an_exact_locale_match_wins() {
    let r = resolve_source(TABLE, "fr-FR");
    assert_eq!(r.source, "francais");
    assert!(!r.is_fallback);
}

#[test]
fn another_region_of_the_same_language_is_not_a_fallback() {
    // A Quebecois reader shown the French page has been served their language. Marking
    // that as a fallback would print "this page is in English" over French text.
    let r = resolve_source(TABLE, "fr-CA");
    assert_eq!(r.source, "francais");
    assert!(
        !r.is_fallback,
        "same language, different region is not a fallback"
    );
}

#[test]
fn an_untranslated_language_falls_back_to_the_source_locale_and_says_so() {
    let r = resolve_source(TABLE, "de-DE");
    assert_eq!(r.source, "english");
    assert!(
        r.is_fallback,
        "a reader asking for German and getting English must be told"
    );
}

#[test]
fn a_table_with_no_source_locale_still_renders_something() {
    // Only reachable from a contributed topic; the drift test above forbids it for
    // built-ins. An empty page would be worse than the wrong language.
    const ODD: &[LocalizedSource] = &[("fr-FR", "francais")];
    let r = resolve_source(ODD, "de-DE");
    assert_eq!(r.source, "francais");
    assert!(r.is_fallback);
}

#[test]
fn language_of_handles_both_separators_and_a_bare_language() {
    assert_eq!(language_of("fr-FR"), "fr");
    assert_eq!(language_of("fr_FR"), "fr");
    assert_eq!(language_of("fr"), "fr");
}

// ── The registry ────────────────────────────────────────────────────────────

fn spec(key: &'static str) -> HelpTopicSpec {
    HelpTopicSpec {
        key,
        section: HelpSection::Extensions,
        title: Rc::new(|| lit!("Contributed")),
        body: HelpBody::Djot(&[("en-US", "Some prose.")]),
    }
}

#[test]
fn a_registered_topic_joins_the_set_and_leaves_on_drop() {
    let before = all_topics().len();
    {
        let _handle = register_topics("ext.test", vec![spec("ext-topic")])
            .expect("a fresh namespace and key must register");
        assert_eq!(all_topics().len(), before + 1);
        assert!(topic("ext-topic").is_some());
    }
    assert_eq!(
        all_topics().len(),
        before,
        "dropping the handle must unregister its topics"
    );
}

#[test]
fn a_built_in_key_cannot_be_claimed() {
    let err = register_topics("ext.thief", vec![spec("help-export")])
        .expect_err("claiming a built-in key must be refused");
    assert!(err.contains("built-in"), "got: {err}");
}

#[test]
fn two_namespaces_cannot_claim_one_key() {
    let _first = register_topics("ext.one", vec![spec("ext-shared")]).expect("first claim");
    let err = register_topics("ext.two", vec![spec("ext-shared")])
        .expect_err("a second namespace must be refused");
    assert!(err.contains("already registered"), "got: {err}");
}

#[test]
fn re_registering_a_namespace_replaces_its_topics() {
    let _first = register_topics("ext.replace", vec![spec("ext-old")]).expect("first");
    let _second = register_topics("ext.replace", vec![spec("ext-new")]).expect("second");
    assert!(topic("ext-old").is_none(), "the old topic must be gone");
    assert!(topic("ext-new").is_some(), "the new topic must be present");
}
