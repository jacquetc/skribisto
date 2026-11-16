// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every translated string the application ships, with where it comes from.
//!
//! A translation report is only actionable when it names the exact line to change.
//! Every catalogue the app displays is compiled into the binary, so the report window
//! can search them all and the report can carry the message id, the file and the
//! line instead of a paraphrase. Five catalogues, each with its own place in the
//! repository and its own fix:
//!
//! | [`Catalogue`] | Compiled in through | Fixed in |
//! |---|---|---|
//! | `App` | [`crate::startup::app_locales`] | `crates/teksilo_ui/locales/<locale>/<file>.ftl` |
//! | `Framework` | `teksilo::widgets::framework_locales` | teksilo's `crates/teksilo-widgets/locales/<locale>.ftl` |
//! | `WidgetOverride` | [`crate::startup::WIDGET_LOCALE_OVERRIDES`] | `crates/teksilo_ui/widget_locales/<file>` |
//! | `Export` | [`skribisto_compiler::catalogue_sources`] | `crates/skribisto_compiler/locales/<lang>.ftl` |
//! | `Help` | [`crate::help::builtin_topics`] | `crates/teksilo_ui/help/<locale>/<page>.djot` |
//!
//! **Provenance follows the resolve order**, not the file a string happens to sit in.
//! An Esperanto framework string defined by both teksilo and the app's override is
//! shown from the override, because that is what wins on screen, so it is attributed
//! to the override and fixed there.
//!
//! **What a string is for** is a separate axis ([`Area`]): an app string can be an
//! interface label, the text of a help topic built from a tooltip, or a name the app
//! stores into projects ([`crate::shared::seeded_strings`]).
//!
//! Extension strings are not indexed: they are merged into the app bundle with no
//! file of their own to point at, and the community build has none. They fall to the
//! report's "I can't find it" path.

use fluent_syntax::ast::Entry as FluentEntry;
use teksilo::text_document::{DjotImportOptions, djot_to_plain_text};

use super::display::{self, Form, Markup};
use crate::help::{HelpBody, builtin_topics};
use crate::startup::{LOCALE_FILES, SUPPORTED_LOCALES, WIDGET_LOCALE_OVERRIDES, app_locales};

/// The source language of every catalogue.
pub(crate) const SOURCE_LOCALE: &str = "en-US";

/// Which catalogue a string comes from: where it is compiled from, and so where it is
/// fixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Catalogue {
    /// The application's own interface strings.
    App,
    /// teksilo's shared controls (dialog buttons, the colour picker, the file
    /// chooser's own words).
    Framework,
    /// The application's override of a framework string.
    WidgetOverride,
    /// The words an exported book prints (headings, byline), in the book's language.
    Export,
    /// A help page.
    Help,
}

impl Catalogue {
    /// The fixed name the report's machine block and the triage workflow use.
    pub(crate) fn slug(self) -> &'static str {
        match self {
            Catalogue::App => "app",
            Catalogue::Framework => "framework",
            Catalogue::WidgetOverride => "widget-override",
            Catalogue::Export => "export",
            Catalogue::Help => "help",
        }
    }

    /// Every catalogue, for the tests that check the slugs against the triage script.
    #[cfg(test)]
    pub(crate) const ALL: [Catalogue; 5] = [
        Catalogue::App,
        Catalogue::Framework,
        Catalogue::WidgetOverride,
        Catalogue::Export,
        Catalogue::Help,
    ];
}

/// What a string is for, which decides what the report says about a fix and which
/// `area:` label the issue gets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Area {
    /// A label, a message, a tooltip.
    Interface,
    /// A help page, or the text of a help topic built from a tooltip.
    Help,
    /// A name Skribisto stores into projects: fixing it changes what is created from
    /// then on, never what a project already holds.
    DefaultContent,
    /// Words an exported book prints.
    Export,
}

impl Area {
    /// The fixed name of the issue's `area:` label.
    pub(crate) fn slug(self) -> &'static str {
        match self {
            Area::Interface => "interface",
            Area::Help => "help",
            Area::DefaultContent => "default content",
            Area::Export => "export",
        }
    }
}

/// One translated string: a Fluent message, or a paragraph of a help page.
#[derive(Clone, Debug)]
pub(crate) struct CatalogueEntry {
    pub catalogue: Catalogue,
    pub area: Area,
    /// The locale the string is written in, as the app names it (`fr-FR`), even for
    /// the export catalogue, whose files are named by language alone.
    pub locale: &'static str,
    /// The file's name: `main.ftl`, `fr-FR.ftl`, `fr.ftl`, `export.djot`.
    pub file: String,
    /// 1-based line of the message's `id =`, or of the paragraph's first line.
    pub line: u32,
    /// The Fluent message id, or the help topic's key for a help paragraph.
    pub id: String,
    /// The value exactly as written in the file, after `id =`: what a fix edits. For a
    /// help paragraph, the paragraph's Djot.
    pub source: String,
    /// Whether the message chooses between variants (a plural, a gender). Its fix
    /// needs every variant, so the suggestion is prefilled with the whole source.
    pub has_selector: bool,
    /// The sentences the screen can show, at least one.
    pub forms: Vec<Form>,
}

impl CatalogueEntry {
    /// The path of the file in its repository: Skribisto's, or teksilo's for a
    /// [`Catalogue::Framework`] string.
    pub(crate) fn repository_path(&self) -> String {
        match self.catalogue {
            Catalogue::App => format!("crates/teksilo_ui/locales/{}/{}", self.locale, self.file),
            Catalogue::Framework => format!("crates/teksilo-widgets/locales/{}", self.file),
            Catalogue::WidgetOverride => format!("crates/teksilo_ui/widget_locales/{}", self.file),
            Catalogue::Export => format!("crates/skribisto_compiler/locales/{}", self.file),
            Catalogue::Help => format!("crates/teksilo_ui/help/{}/{}", self.locale, self.file),
        }
    }

    /// Whether the fix belongs in teksilo rather than in Skribisto.
    pub(crate) fn is_upstream(&self) -> bool {
        self.catalogue == Catalogue::Framework
    }

    /// The first form, as printed in a result row.
    pub(crate) fn shown(&self) -> &str {
        self.forms.first().map(|f| f.shown.as_str()).unwrap_or("")
    }
}

/// The supported locale `tag` names, as `'static` (so entries can hold it), or
/// `None` for a tag the app does not ship.
pub(crate) fn supported_locale(tag: &str) -> Option<&'static str> {
    SUPPORTED_LOCALES.iter().copied().find(|l| *l == tag)
}

/// The language an export catalogue is named by: the primary subtag, with the three
/// Norwegian tags sharing the Bokmål file, as the compiler resolves them.
pub(crate) fn export_language(tag: &str) -> String {
    let primary = tag
        .split(['-', '_'])
        .next()
        .unwrap_or(tag)
        .to_ascii_lowercase();
    match primary.as_str() {
        "no" | "nn" => "nb".to_string(),
        _ => primary,
    }
}

/// Every string shipped in `locale`, from every catalogue. Empty for a locale the app
/// does not ship.
pub(crate) fn entries_for(locale: &str) -> Vec<CatalogueEntry> {
    let Some(locale) = supported_locale(locale) else {
        return Vec::new();
    };
    let tooltip_topics = tooltip_topic_keys();
    let mut out = Vec::new();
    app_entries(locale, &tooltip_topics, &mut out);
    framework_entries(locale, &mut out);
    export_entries(locale, &mut out);
    help_entries(locale, &mut out);
    out
}

/// The tooltip keys a help topic is built from. Their text (the key and its `-more`)
/// is a help page as far as the writer is concerned.
fn tooltip_topic_keys() -> Vec<&'static str> {
    builtin_topics()
        .into_iter()
        .filter_map(|t| match t.body {
            HelpBody::Tooltip(key) => Some(key),
            HelpBody::Djot(_) => None,
        })
        .collect()
}

fn app_entries(locale: &'static str, tooltip_topics: &[&str], out: &mut Vec<CatalogueEntry>) {
    let Some((_, files)) = app_locales().iter().find(|(tag, _)| *tag == locale) else {
        return;
    };
    // `app_locales` expands `locales × files` in `LOCALE_FILES` order; the drift test
    // in `startup` holds the two lists in step.
    for (source, file) in files.iter().zip(LOCALE_FILES) {
        let markup = if *file == "tooltips.ftl" {
            Markup::Markdown
        } else {
            Markup::Plain
        };
        for message in parse_ftl(source, markup) {
            let area = if *file == "tooltips.ftl"
                && tooltip_topics
                    .iter()
                    .any(|k| message.id == *k || message.id.strip_suffix("-more") == Some(*k))
            {
                Area::Help
            } else if crate::shared::seeded_strings::is_seeded(&message.id) {
                Area::DefaultContent
            } else {
                Area::Interface
            };
            out.push(message.into_entry(Catalogue::App, area, locale, (*file).to_string()));
        }
    }
}

fn framework_entries(locale: &'static str, out: &mut Vec<CatalogueEntry>) {
    let overrides: Vec<ParsedMessage> = WIDGET_LOCALE_OVERRIDES
        .iter()
        .filter(|(tag, _)| *tag == locale)
        .flat_map(|(_, sources)| sources.iter())
        .flat_map(|source| parse_ftl(source, Markup::Plain))
        .collect();

    if let Some((_, sources)) = teksilo::widgets::framework_locales()
        .iter()
        .find(|(tag, _)| *tag == locale)
    {
        for source in *sources {
            for message in parse_ftl(source, Markup::Plain) {
                // The override is consulted first, so a key it defines is never shown
                // from the framework's file.
                if overrides.iter().any(|o| o.id == message.id) {
                    continue;
                }
                out.push(message.into_entry(
                    Catalogue::Framework,
                    Area::Interface,
                    locale,
                    format!("{locale}.ftl"),
                ));
            }
        }
    }
    for message in overrides {
        out.push(message.into_entry(
            Catalogue::WidgetOverride,
            Area::Interface,
            locale,
            format!("{locale}.ftl"),
        ));
    }
}

fn export_entries(locale: &'static str, out: &mut Vec<CatalogueEntry>) {
    let language = export_language(locale);
    let Some((_, source)) = skribisto_compiler::catalogue_sources()
        .iter()
        .find(|(lang, _)| *lang == language)
    else {
        return;
    };
    for message in parse_ftl(source, Markup::Plain) {
        out.push(message.into_entry(
            Catalogue::Export,
            Area::Export,
            locale,
            format!("{language}.ftl"),
        ));
    }
}

/// The page file a built-in Djot topic is compiled from: its key without the
/// `help-` prefix. `tests::every_help_page_names_its_file` reads each file back.
pub(crate) fn help_page_file(topic_key: &str) -> Option<String> {
    topic_key
        .strip_prefix("help-")
        .map(|name| format!("{name}.djot"))
}

fn help_entries(locale: &'static str, out: &mut Vec<CatalogueEntry>) {
    for topic in builtin_topics() {
        let HelpBody::Djot(table) = topic.body else {
            continue;
        };
        let Some(file) = help_page_file(topic.key) else {
            continue;
        };
        // Only a page written in this locale. A page this locale lacks is shown in
        // English, and reported against English.
        let Some((_, source)) = table.iter().find(|(tag, _)| *tag == locale) else {
            continue;
        };
        for (line, paragraph) in djot_paragraphs(source) {
            // On macOS a page's chords are rewritten before display; index what is
            // displayed.
            let rendered = crate::help::chords::platform_chords(&paragraph);
            let plain = djot_to_plain_text(&rendered, &DjotImportOptions::default());
            let search = display::normalize_for_search(&plain);
            if search.is_empty() {
                continue;
            }
            out.push(CatalogueEntry {
                catalogue: Catalogue::Help,
                area: Area::Help,
                locale,
                file: file.clone(),
                line,
                id: topic.key.to_string(),
                source: paragraph,
                has_selector: false,
                forms: vec![Form {
                    shown: plain.split_whitespace().collect::<Vec<_>>().join(" "),
                    search,
                }],
            });
        }
    }
}

/// A Djot page's blocks, split at blank lines, each with the 1-based line it starts
/// on. A block of attributes only (`{#id}`) is not text and is skipped.
fn djot_paragraphs(source: &str) -> Vec<(u32, String)> {
    let mut out = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    let mut start = 0u32;
    for (i, line) in source.lines().enumerate() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                out.push((start, current.join("\n")));
                current.clear();
            }
            continue;
        }
        if current.is_empty() {
            start = u32::try_from(i + 1).unwrap_or(u32::MAX);
        }
        current.push(line);
    }
    if !current.is_empty() {
        out.push((start, current.join("\n")));
    }
    out.retain(|(_, block)| {
        let t = block.trim();
        !(t.starts_with('{') && t.ends_with('}') && !t.contains('\n'))
    });
    out
}

/// A message as parsed, before it knows its catalogue.
struct ParsedMessage {
    id: String,
    line: u32,
    source: String,
    has_selector: bool,
    forms: Vec<Form>,
}

impl ParsedMessage {
    fn into_entry(
        self,
        catalogue: Catalogue,
        area: Area,
        locale: &'static str,
        file: String,
    ) -> CatalogueEntry {
        CatalogueEntry {
            catalogue,
            area,
            locale,
            file,
            line: self.line,
            id: self.id,
            source: self.source,
            has_selector: self.has_selector,
            forms: self.forms,
        }
    }
}

/// Every message of a Fluent file that has a value. A malformed entry is skipped:
/// `check_locales.py` refuses one in CI, and the rest of the file is still worth
/// searching.
fn parse_ftl(source: &str, markup: Markup) -> Vec<ParsedMessage> {
    let resource = match fluent_syntax::parser::parse(source) {
        Ok(resource) => resource,
        Err((resource, _errors)) => resource,
    };
    let positions = message_positions(source);
    resource
        .body
        .iter()
        .filter_map(|entry| {
            let FluentEntry::Message(message) = entry else {
                return None;
            };
            let value = message.value.as_ref()?;
            let id = message.id.name.to_string();
            let (line, raw) = positions
                .iter()
                .find(|(pid, _, _)| *pid == id)
                .map(|(_, line, raw)| (*line, raw.clone()))
                .unwrap_or_default();
            let forms = display::forms(value, markup);
            if forms.is_empty() {
                return None;
            }
            let has_selector = value.elements.iter().any(|e| {
                matches!(
                    e,
                    fluent_syntax::ast::PatternElement::Placeable {
                        expression: fluent_syntax::ast::Expression::Select { .. }
                    }
                )
            });
            Some(ParsedMessage {
                id,
                line,
                source: raw,
                has_selector,
                forms,
            })
        })
        .collect()
}

/// For every message in `source`: its id, the 1-based line of `id =`, and its value
/// as written (the rest of that line, then every indented continuation line).
fn message_positions(source: &str) -> Vec<(String, u32, String)> {
    let lines: Vec<&str> = source.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let starts_entry = line.chars().next().is_some_and(|c| c.is_ascii_alphabetic());
        let Some((id, first)) = line.split_once('=').filter(|_| starts_entry) else {
            i += 1;
            continue;
        };
        let id = id.trim();
        if id.is_empty()
            || !id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            i += 1;
            continue;
        }
        let mut value = vec![first.trim_start().to_string()];
        let mut j = i + 1;
        while j < lines.len() && (lines[j].starts_with(' ') || lines[j].trim().is_empty()) {
            value.push(lines[j].to_string());
            j += 1;
        }
        while value.len() > 1 && value.last().is_some_and(|l| l.trim().is_empty()) {
            value.pop();
        }
        let raw = value.join("\n");
        let raw = raw.trim_start_matches('\n').to_string();
        out.push((
            id.to_string(),
            u32::try_from(i + 1).unwrap_or(u32::MAX),
            raw,
        ));
        i = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find<'a>(
        entries: &'a [CatalogueEntry],
        catalogue: Catalogue,
        id: &str,
    ) -> &'a CatalogueEntry {
        entries
            .iter()
            .find(|e| e.catalogue == catalogue && e.id == id)
            .unwrap_or_else(|| panic!("{} {id} not indexed", catalogue.slug()))
    }

    /// Every catalogue, area and problem name the application writes into a report
    /// is one the triage script accepts: a name it refuses leaves the issue with
    /// `needs-triage` and nothing else, silently.
    #[test]
    fn every_name_a_report_carries_is_one_the_triage_accepts() {
        let script = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../tools/translation_report_triage.py"),
        )
        .unwrap_or_default();
        let set_of = |name: &str| -> String {
            let start = script
                .find(&format!("{name} = {{"))
                .unwrap_or_else(|| panic!("{name} not found in the triage script"));
            let end = start + script[start..].find('}').unwrap_or(0);
            script[start..end].to_string()
        };
        let catalogues = set_of("CATALOGUES");
        for catalogue in Catalogue::ALL {
            assert!(
                catalogues.contains(&format!("\"{}\"", catalogue.slug())),
                "{}",
                catalogue.slug()
            );
        }
        let areas = set_of("AREAS");
        for area in [
            Area::Interface,
            Area::Help,
            Area::DefaultContent,
            Area::Export,
        ] {
            assert!(
                areas.contains(&format!("\"{}\"", area.slug())),
                "{}",
                area.slug()
            );
        }
        assert!(
            areas.contains("\"unknown\""),
            "a text not found has area unknown"
        );
        let kinds = set_of("KINDS");
        for kind in super::super::report::ProblemKind::ALL {
            assert!(
                kinds.contains(&format!("\"{}\"", kind.slug())),
                "{}",
                kind.slug()
            );
        }
    }

    #[test]
    fn an_unsupported_locale_has_no_entries() {
        assert!(entries_for("xx-YY").is_empty());
    }

    #[test]
    fn every_supported_locale_indexes_every_catalogue() {
        for locale in SUPPORTED_LOCALES {
            let entries = entries_for(locale);
            for catalogue in [Catalogue::App, Catalogue::Export] {
                assert!(
                    entries.iter().any(|e| e.catalogue == catalogue),
                    "{locale}: nothing from {}",
                    catalogue.slug()
                );
            }
            assert!(
                entries.iter().any(|e| matches!(
                    e.catalogue,
                    Catalogue::Framework | Catalogue::WidgetOverride
                )),
                "{locale}: no framework string"
            );
        }
    }

    #[test]
    fn a_message_names_its_file_line_and_source() {
        let entries = entries_for("fr-FR");
        let undo = find(&entries, Catalogue::App, "menu-edit-undo-target");
        assert_eq!(undo.file, "main.ftl");
        assert_eq!(undo.source, "A&nnuler { $target }");
        assert_eq!(undo.shown(), "Annuler { $target }");
        assert_eq!(
            undo.repository_path(),
            "crates/teksilo_ui/locales/fr-FR/main.ftl"
        );

        let text = app_locales()
            .iter()
            .find(|(t, _)| *t == "fr-FR")
            .map(|(_, files)| files[0])
            .unwrap_or_default();
        let line = text.lines().nth(undo.line as usize - 1).unwrap_or_default();
        assert!(
            line.starts_with("menu-edit-undo-target"),
            "line {} is {line:?}",
            undo.line
        );
    }

    #[test]
    fn a_select_keeps_its_whole_source() {
        let entries = entries_for("en-US");
        let select = entries
            .iter()
            .find(|e| e.catalogue == Catalogue::App && e.has_selector)
            .expect("the app catalogue has select expressions");
        assert!(select.source.contains("->"), "{}", select.source);
        assert!(select.source.contains("*["), "{}", select.source);
        assert!(select.forms.len() >= 2, "{:?}", select.forms);
    }

    #[test]
    fn every_message_of_every_app_file_is_indexed() {
        for (tag, files) in app_locales() {
            let entries = entries_for(tag);
            for (source, file) in files.iter().zip(LOCALE_FILES) {
                let ids = message_positions(source);
                for (id, _, _) in ids {
                    assert!(
                        entries.iter().any(|e| e.catalogue == Catalogue::App
                            && e.file == *file
                            && e.id == id),
                        "{tag}/{file}: {id} not indexed"
                    );
                }
            }
        }
    }

    /// teksilo ships Esperanto, and the app's override still wins for every key it
    /// defines: such a key is attributed to the override, the rest to teksilo.
    #[test]
    fn an_esperanto_framework_string_is_attributed_to_whichever_source_wins() {
        let entries = entries_for("eo");
        let override_ids: Vec<String> = WIDGET_LOCALE_OVERRIDES
            .iter()
            .flat_map(|(_, s)| s.iter())
            .flat_map(|s| message_positions(s))
            .map(|(id, _, _)| id)
            .collect();
        assert!(!override_ids.is_empty());
        for id in &override_ids {
            assert!(
                !entries
                    .iter()
                    .any(|e| e.catalogue == Catalogue::Framework && &e.id == id),
                "{id} is overridden, so it must not be attributed to teksilo"
            );
            let attributed = find(&entries, Catalogue::WidgetOverride, id);
            assert_eq!(
                attributed.repository_path(),
                "crates/teksilo_ui/widget_locales/eo.ftl"
            );
        }
        assert!(
            entries.iter().any(|e| e.catalogue == Catalogue::Framework),
            "the keys the override leaves alone come from teksilo"
        );
        let french = entries_for("fr-FR");
        assert!(
            !french
                .iter()
                .any(|e| e.catalogue == Catalogue::WidgetOverride)
        );
    }

    #[test]
    fn a_framework_string_is_fixed_upstream() {
        let entries = entries_for("de-DE");
        let framework = entries
            .iter()
            .find(|e| e.catalogue == Catalogue::Framework)
            .expect("German has framework strings");
        assert!(framework.is_upstream());
        assert_eq!(framework.file, "de-DE.ftl");
        assert_eq!(
            framework.repository_path(),
            "crates/teksilo-widgets/locales/de-DE.ftl"
        );
    }

    #[test]
    fn the_export_catalogue_is_named_by_language() {
        let entries = entries_for("nb-NO");
        let export = entries
            .iter()
            .find(|e| e.catalogue == Catalogue::Export)
            .expect("Norwegian has an export catalogue");
        assert_eq!(export.file, "nb.ftl");
        assert_eq!(export.area, Area::Export);
        assert_eq!(export_language("nn-NO"), "nb");
        assert_eq!(export_language("pt-PT"), "pt");
    }

    #[test]
    fn seeded_and_tooltip_topic_strings_are_classified() {
        let entries = entries_for("fr-FR");
        assert_eq!(
            find(&entries, Catalogue::App, "status-draft").area,
            Area::DefaultContent
        );
        assert_eq!(
            find(&entries, Catalogue::App, "menu-help-report").area,
            Area::Interface
        );
        assert_eq!(find(&entries, Catalogue::App, "wm-scene").area, Area::Help);
        assert_eq!(
            find(&entries, Catalogue::App, "wm-scene-more").area,
            Area::Help
        );
        // Markdown is stripped from tooltip text, so the link shows as its label.
        let more = find(&entries, Catalogue::App, "wm-scene-more");
        assert!(!more.shown().contains("](:"), "{}", more.shown());
    }

    #[test]
    fn every_help_page_names_its_file() {
        for topic in builtin_topics() {
            let HelpBody::Djot(table) = topic.body else {
                continue;
            };
            let file = help_page_file(topic.key)
                .unwrap_or_else(|| panic!("{}: not a help-… key", topic.key));
            for (locale, source) in table {
                let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("help")
                    .join(locale)
                    .join(&file);
                let on_disk = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                assert_eq!(
                    &on_disk,
                    source,
                    "{} is not the file it names",
                    path.display()
                );
            }
        }
    }

    #[test]
    fn a_help_paragraph_names_its_page_and_line() {
        let entries = entries_for("fr-FR");
        let paragraph = entries
            .iter()
            .find(|e| e.catalogue == Catalogue::Help && e.id == "help-getting-started")
            .expect("the French getting-started page is indexed");
        assert_eq!(paragraph.file, "getting-started.djot");
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("help/fr-FR/getting-started.djot"),
        )
        .unwrap_or_default();
        let first_line = page
            .lines()
            .nth(paragraph.line as usize - 1)
            .unwrap_or_default();
        assert!(
            paragraph.source.starts_with(first_line),
            "line {} is {first_line:?}, the paragraph starts {:?}",
            paragraph.line,
            paragraph.source
        );
        // A locale with no help pages of its own indexes none.
        assert!(
            !entries_for("ja-JP")
                .iter()
                .any(|e| e.catalogue == Catalogue::Help)
        );
    }

    #[test]
    fn a_paragraph_split_skips_attribute_blocks() {
        let p = djot_paragraphs("# Title\n\n{#anchor}\n\nOne\ntwo\n\n\nThree\n");
        assert_eq!(
            p,
            vec![
                (1, "# Title".to_string()),
                (5, "One\ntwo".to_string()),
                (9, "Three".to_string())
            ]
        );
    }

    #[test]
    fn a_multiline_value_keeps_its_continuation_lines() {
        let src = "a = one\n    two\n\n    three\n# comment\nb = x\n";
        let p = message_positions(src);
        assert_eq!(
            p[0],
            ("a".to_string(), 1, "one\n    two\n\n    three".to_string())
        );
        assert_eq!(p[1], ("b".to_string(), 6, "x".to_string()));
        let block = "c =\n    { $n ->\n       *[other] x\n    }\n";
        assert_eq!(
            message_positions(block)[0].2,
            "    { $n ->\n       *[other] x\n    }"
        );
    }
}
