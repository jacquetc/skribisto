// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a translation report says, and the link that carries it to the tracker.
//!
//! The report opens a GitHub issue **form** in the writer's browser, prefilled
//! through the link's query. Three things shape it:
//!
//! - **The form decides the field names.** Every name sent here is an `id:` of
//!   `.github/ISSUE_TEMPLATE/translation-error.yml`; `tests::every_field_is_a_form_id`
//!   reads the form and checks them, and the problem kinds' English labels too.
//! - **A machine block** ([`Report::block`]) goes in a field of its own: the locale,
//!   the catalogue, the file, the message id, the build. The triage workflow parses it
//!   (`tools/translation_report_triage.py`) to label the issue and link the exact line,
//!   so its format is a contract, pinned on both sides by one fixture file.
//! - **The link has a budget** ([`LINK_BUDGET`]). GitHub refuses a request once the
//!   address and the cookies together pass about 6.9 KB, and a signed-out visitor's
//!   address is first re-encoded into its sign-in page's `return_to`, where every `%`
//!   becomes `%25`. Measured on 2026-11-16 in Firefox and Safari on macOS and in Edge on
//!   Windows: a 4,000-character link of Japanese text already fails there, while the
//!   launchers themselves pass 32,000 intact. So the budget is counted on the sign-in
//!   address GitHub would build, and a field that does not fit is replaced by a note
//!   asking the writer to paste the whole report, which the window has copied. For a
//!   long report in a non-Latin script that is the normal case, not an edge case.
//!
//! Nothing personal travels: no project, no file name, no manuscript text, no path,
//! no user name. Only the catalogue's own text (public anyway), the writer's words
//! and the build's identity.

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

pub(crate) use crate::version::BuildInfo;

/// The first line of the machine block, and the marker the triage workflow looks for
/// in an issue body.
pub(crate) const BLOCK_MARKER: &str = "skribisto-translation-report: 1";

/// What GitHub's sign-in address starts with. A signed-out visitor is sent there with
/// the whole report link re-encoded into `return_to`.
const SIGN_IN_PREFIX: &str = "https://github.com/login?return_to=";

/// The longest sign-in address a report link may produce: under the ~6.9 KB GitHub
/// accepts with room for a signed-out visitor's cookies (measured at ~0.4 KB), and
/// with ~2.9 KB of cookies left for a signed-in one, who is not redirected at all.
pub(crate) const LINK_BUDGET: usize = 4_000;

/// What replaces a field that did not fit in the link. English, like the form it
/// lands in.
pub(crate) const OVERFLOW_NOTE: &str = "Part of the report did not fit in the link. \
     Skribisto copied the whole report to the clipboard: paste it here.";

/// How long the title's quotation of the text may be, in characters.
const TITLE_QUOTE: usize = 50;

/// RFC 3986 unreserved characters pass; everything else is escaped.
const QUERY_VALUE: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// The issue form's field ids, as `.github/ISSUE_TEMPLATE/translation-error.yml`
/// declares them.
pub(crate) mod field {
    pub const LANGUAGE: &str = "language";
    pub const MESSAGE: &str = "message";
    pub const CURRENT: &str = "current";
    pub const ENGLISH: &str = "english";
    pub const KIND: &str = "kind";
    pub const EXPLANATION: &str = "explanation";
    pub const SUGGESTION: &str = "suggestion";
    pub const REPORT_DATA: &str = "report-data";

    /// Every field a report link may fill.
    #[cfg(test)]
    pub const ALL: [&str; 8] = [
        LANGUAGE,
        MESSAGE,
        CURRENT,
        ENGLISH,
        KIND,
        EXPLANATION,
        SUGGESTION,
        REPORT_DATA,
    ];
}

/// What is wrong with the string. The slug and the English label travel to GitHub
/// and are never translated: they are what the issue's `kind:` label and the triage
/// read. The window shows each in the writer's language.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum ProblemKind {
    #[default]
    WrongMeaning,
    Spelling,
    NotTranslated,
    Unnatural,
    Inconsistent,
    PluralForm,
    CutOff,
    Other,
}

impl ProblemKind {
    /// In the order the window offers them.
    pub(crate) const ALL: [ProblemKind; 8] = [
        ProblemKind::WrongMeaning,
        ProblemKind::Spelling,
        ProblemKind::NotTranslated,
        ProblemKind::Unnatural,
        ProblemKind::Inconsistent,
        ProblemKind::PluralForm,
        ProblemKind::CutOff,
        ProblemKind::Other,
    ];

    /// The fixed name of the `kind:` label and of the block's `kind:` line.
    pub(crate) fn slug(self) -> &'static str {
        match self {
            ProblemKind::WrongMeaning => "wrong-meaning",
            ProblemKind::Spelling => "spelling",
            ProblemKind::NotTranslated => "not-translated",
            ProblemKind::Unnatural => "unnatural",
            ProblemKind::Inconsistent => "inconsistent",
            ProblemKind::PluralForm => "plural-form",
            ProblemKind::CutOff => "cut-off",
            ProblemKind::Other => "other",
        }
    }

    /// The English label the form's `kind` field receives.
    pub(crate) fn english_label(self) -> &'static str {
        match self {
            ProblemKind::WrongMeaning => "The meaning is wrong",
            ProblemKind::Spelling => "Spelling, grammar or punctuation",
            ProblemKind::NotTranslated => "Still in English",
            ProblemKind::Unnatural => "Understandable but unnatural, or the wrong tone",
            ProblemKind::Inconsistent => "Named differently elsewhere",
            ProblemKind::PluralForm => "Wrong form for a number",
            ProblemKind::CutOff => "Cut off or does not fit",
            ProblemKind::Other => "Something else",
        }
    }

    /// The short phrase the issue title ends with.
    fn title_phrase(self) -> &'static str {
        match self {
            ProblemKind::WrongMeaning => "wrong meaning",
            ProblemKind::Spelling => "spelling",
            ProblemKind::NotTranslated => "not translated",
            ProblemKind::Unnatural => "unnatural",
            ProblemKind::Inconsistent => "inconsistent",
            ProblemKind::PluralForm => "plural form",
            ProblemKind::CutOff => "cut off",
            ProblemKind::Other => "other",
        }
    }
}

/// Which string the report is about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Target {
    /// A string the search found.
    Message {
        /// [`super::catalogue::Catalogue::slug`].
        catalogue: &'static str,
        /// [`super::catalogue::Area::slug`].
        area: &'static str,
        file: String,
        line: u32,
        id: String,
        /// The message that filled one of its variables, for a composed label.
        composed: Option<String>,
        /// The other messages reading the same text, when the writer reports every
        /// place at once.
        also: Vec<String>,
        /// Whether the fix belongs in teksilo.
        upstream: bool,
    },
    /// A string the search did not find, described in the writer's words.
    NotFound {
        /// Where the writer saw it.
        seen_at: String,
    },
}

/// One report, as the writer has filled it in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Report {
    /// The language the string is in, as the app names it (`fr-FR`).
    pub locale: String,
    pub target: Target,
    /// The text as the screen shows it, empty for a string not found.
    pub current: String,
    /// The English source it translates, empty when there is none to show.
    pub english: String,
    pub kind: ProblemKind,
    /// The writer's explanation, in any language. Optional.
    pub explanation: String,
    /// The better translation.
    pub suggestion: String,
    pub build: BuildInfo,
}

/// The link to open, and what had to be left out of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Link {
    pub url: String,
    /// Fields replaced by [`OVERFLOW_NOTE`] (or, when even the note did not fit,
    /// left out). Non-empty means the window must copy the whole report and say so.
    pub dropped: Vec<&'static str>,
}

impl Link {
    pub(crate) fn overflowed(&self) -> bool {
        !self.dropped.is_empty()
    }
}

impl Report {
    /// The issue title: `fr-FR: "Annuler" (wrong meaning)`.
    pub(crate) fn title(&self) -> String {
        let quoted = match &self.target {
            Target::Message { .. } if !self.current.trim().is_empty() => self.current.trim(),
            Target::Message { id, .. } => id.as_str(),
            Target::NotFound { seen_at } => seen_at.trim(),
        };
        let one_line = quoted.split_whitespace().collect::<Vec<_>>().join(" ");
        let cut: String = if one_line.chars().count() > TITLE_QUOTE {
            let head: String = one_line.chars().take(TITLE_QUOTE - 1).collect();
            format!("{}…", head.trim_end())
        } else {
            one_line
        };
        format!(
            "{}: \"{}\" ({})",
            self.locale,
            cut,
            self.kind.title_phrase()
        )
    }

    /// The machine block the triage workflow parses. One `key: value` per line, a
    /// line with nothing after the colon for an absent value, never a value that
    /// spans lines.
    pub(crate) fn block(&self) -> String {
        let mut lines = vec![BLOCK_MARKER.to_string()];
        let mut push = |key: &str, value: &str| {
            let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
            if value.is_empty() {
                lines.push(format!("{key}:"));
            } else {
                lines.push(format!("{key}: {value}"));
            }
        };
        push("locale", &self.locale);
        match &self.target {
            Target::Message {
                catalogue,
                area,
                file,
                line,
                id,
                composed,
                also,
                upstream,
            } => {
                push("catalogue", catalogue);
                push("area", area);
                push("file", file);
                push("line", &line.to_string());
                push("message", id);
                push("composed", composed.as_deref().unwrap_or(""));
                push("also", &also.join(", "));
                push("upstream", if *upstream { "teksilo" } else { "" });
            }
            Target::NotFound { .. } => {
                push("catalogue", "");
                push("area", "unknown");
            }
        }
        push("kind", self.kind.slug());
        push("version", &self.build.version);
        push("crate-version", &self.build.crate_version);
        push(
            "framework",
            &self
                .build
                .framework
                .as_deref()
                .map(|v| format!("teksilo-widgets {v}"))
                .unwrap_or_default(),
        );
        push("channel", self.build.channel);
        push("os", self.build.os);
        lines.join("\n")
    }

    /// What the `message` field says: the message id, or where the writer saw a string
    /// the search did not find.
    fn message_field(&self) -> String {
        match &self.target {
            Target::Message {
                id, composed, also, ..
            } => {
                let mut out = id.clone();
                if let Some(c) = composed {
                    out.push_str(" (composed with ");
                    out.push_str(c);
                    out.push(')');
                }
                if !also.is_empty() {
                    out.push_str(", and the same text in ");
                    out.push_str(&also.join(", "));
                }
                out
            }
            Target::NotFound { seen_at } => {
                format!("Not found in the catalogues. Seen at: {seen_at}")
            }
        }
    }

    /// The whole report as text, for the clipboard: everything the link carries and
    /// everything it may have had to leave out. Field names in English, like the form
    /// it is pasted into.
    pub(crate) fn clipboard_text(&self) -> String {
        let mut out = String::new();
        let mut section = |label: &str, value: &str| {
            if value.trim().is_empty() {
                return;
            }
            out.push_str("### ");
            out.push_str(label);
            out.push_str("\n\n");
            out.push_str(value.trim_end());
            out.push_str("\n\n");
        };
        section("Title", &self.title());
        section("Language", &self.locale);
        section("Message", &self.message_field());
        section("Current text", &self.current);
        section("English source", &self.english);
        section("What is wrong", self.kind.english_label());
        section("Explanation", &self.explanation);
        section("Better translation", &self.suggestion);
        section("Report data", &format!("```text\n{}\n```", self.block()));
        out.trim_end().to_string()
    }

    /// The prefilled form link, within [`LINK_BUDGET`]. `form` is the edition's
    /// new-issue address with its template, for example
    /// `https://github.com/<owner>/<repo>/issues/new?template=translation-error.yml`.
    pub(crate) fn link(&self, form: &str) -> Link {
        let mut url = form.to_string();
        let mut dropped = Vec::new();
        let add = |url: &mut String, key: &str, value: &str| {
            url.push(if url.contains('?') { '&' } else { '?' });
            url.push_str(key);
            url.push('=');
            url.extend(utf8_percent_encode(value, QUERY_VALUE));
        };

        // Always in: what triage and labelling need, and what the writer would be
        // unable to reconstruct on GitHub.
        add(&mut url, "title", &self.title());
        add(&mut url, field::LANGUAGE, &self.locale);
        add(&mut url, field::MESSAGE, &self.message_field());
        add(&mut url, field::KIND, self.kind.english_label());
        add(&mut url, field::REPORT_DATA, &self.block());

        // Then, while they fit: the writer's suggestion first, then the text as this
        // build showed it (the workflow cannot always read what an old build showed),
        // then the explanation, then the English source, which the workflow re-posts.
        for (key, value) in [
            (field::SUGGESTION, self.suggestion.as_str()),
            (field::CURRENT, self.current.as_str()),
            (field::EXPLANATION, self.explanation.as_str()),
            (field::ENGLISH, self.english.as_str()),
        ] {
            if value.trim().is_empty() {
                continue;
            }
            let mut candidate = url.clone();
            add(&mut candidate, key, value);
            if sign_in_length(&candidate) <= LINK_BUDGET {
                url = candidate;
                continue;
            }
            dropped.push(key);
            let mut noted = url.clone();
            add(&mut noted, key, OVERFLOW_NOTE);
            if sign_in_length(&noted) <= LINK_BUDGET {
                url = noted;
            }
        }
        Link { url, dropped }
    }
}

/// How long GitHub's sign-in address would be for `url`: the prefix, then `url`'s
/// path and query re-encoded, every byte outside the unreserved set escaped. A
/// slight overestimate (GitHub leaves `/` alone), which is the safe side.
pub(crate) fn sign_in_length(url: &str) -> usize {
    let path_and_query = url
        .split_once("://")
        .and_then(|(_, rest)| rest.find('/').map(|i| &rest[i..]))
        .unwrap_or(url);
    SIGN_IN_PREFIX.len() + utf8_percent_encode(path_and_query, QUERY_VALUE).count_bytes()
}

/// Byte length of an encoder's output without building it.
trait CountBytes {
    fn count_bytes(self) -> usize;
}

impl CountBytes for percent_encoding::PercentEncode<'_> {
    fn count_bytes(self) -> usize {
        self.map(str::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An edition's form address, as `ProjectLinks::translation_form` carries one.
    const FORM: &str =
        "https://github.com/example/writer/issues/new?template=translation-error.yml";

    pub(crate) fn sample() -> Report {
        Report {
            locale: "fr-FR".to_string(),
            target: Target::Message {
                catalogue: "app",
                area: "interface",
                file: "main.ftl".to_string(),
                line: 3080,
                id: "menu-edit-undo-target".to_string(),
                composed: Some("undo-target-trash".to_string()),
                also: Vec::new(),
                upstream: false,
            },
            current: "Annuler la mise à la corbeille".to_string(),
            english: "Undo { $target }".to_string(),
            kind: ProblemKind::WrongMeaning,
            explanation: "« Annuler » se lit comme « Cancel » ici.".to_string(),
            suggestion: "&Défaire { $target }".to_string(),
            build: BuildInfo {
                version: "3.3.1+5c62c8cac".to_string(),
                crate_version: "3.3.1".to_string(),
                framework: Some("0.26.0".to_string()),
                channel: "flathub",
                os: "linux",
            },
        }
    }

    fn fixture_path() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tools/fixtures/translation_report_block.txt")
    }

    /// The machine block is a contract with the triage script, which parses this same
    /// file in `tools/test_translation_report_triage.py`. A change to the format fails
    /// here first; update the fixture and the script together.
    #[test]
    fn the_block_matches_the_fixture_the_triage_script_parses() {
        let fixture = std::fs::read_to_string(fixture_path()).unwrap_or_default();
        assert_eq!(sample().block(), fixture.trim_end());
    }

    #[test]
    fn the_title_quotes_the_text_and_names_the_problem() {
        assert_eq!(
            sample().title(),
            "fr-FR: \"Annuler la mise à la corbeille\" (wrong meaning)"
        );
        let mut long = sample();
        long.current = "x".repeat(80);
        let title = long.title();
        assert!(title.contains('…'), "{title}");
        assert!(title.chars().count() < 80, "{title}");
    }

    #[test]
    fn a_report_of_a_string_not_found_says_where_it_was_seen() {
        let mut r = sample();
        r.target = Target::NotFound {
            seen_at: "The file dialog's Cancel button".to_string(),
        };
        r.current.clear();
        let block = r.block();
        assert!(block.contains("area: unknown"), "{block}");
        assert!(!block.contains("message:"), "{block}");
        assert!(r.title().contains("file dialog"), "{}", r.title());
        assert!(r.clipboard_text().contains("Seen at: The file dialog"));
    }

    #[test]
    fn a_value_never_spans_lines_in_the_block() {
        let mut r = sample();
        r.locale = "fr-FR\nkind: other".to_string();
        let block = r.block();
        assert!(block.lines().all(|l| l.contains(':')), "{block}");
        assert_eq!(block.lines().filter(|l| l.starts_with("kind:")).count(), 1);
    }

    #[test]
    fn a_short_report_fits_whole() {
        let link = sample().link(FORM);
        assert!(!link.overflowed(), "{:?}", link.dropped);
        assert!(sign_in_length(&link.url) <= LINK_BUDGET);
        for key in [
            "title",
            "language",
            "message",
            "kind",
            "report-data",
            "suggestion",
            "current",
            "explanation",
            "english",
        ] {
            assert!(link.url.contains(&format!("&{key}=")), "{key} missing");
        }
        assert!(link.url.starts_with(FORM));
    }

    /// A long suggestion in Japanese: each character is nine bytes encoded and fifteen
    /// once GitHub re-encodes it, so 1,000 of them cannot travel. The link stays under
    /// the budget, says so in place, and the copied text still holds everything.
    #[test]
    fn a_long_report_in_a_non_latin_script_overflows_to_the_clipboard() {
        let mut r = sample();
        r.locale = "ja-JP".to_string();
        r.suggestion = "語".repeat(1_000);
        let link = r.link(FORM);
        assert!(
            sign_in_length(&link.url) <= LINK_BUDGET,
            "{}",
            sign_in_length(&link.url)
        );
        assert!(link.overflowed());
        assert!(link.dropped.contains(&field::SUGGESTION));
        let note: String = utf8_percent_encode(OVERFLOW_NOTE, QUERY_VALUE).collect();
        assert!(link.url.contains(&format!("&suggestion={note}")));
        assert!(r.clipboard_text().contains(&"語".repeat(1_000)));
        // What did fit is still in the link.
        assert!(link.url.contains("&current="));
    }

    #[test]
    fn the_sign_in_length_counts_every_escape_twice_over() {
        // `%` → `%25`: an escaped character costs five bytes per byte, not three.
        let url = "https://github.com/a?b=%C3%A9";
        assert_eq!(
            sign_in_length(url),
            SIGN_IN_PREFIX.len() + "%2Fa%3Fb%3D%25C3%25A9".len()
        );
    }

    #[test]
    fn every_problem_kind_has_its_own_slug_and_label() {
        let slugs: std::collections::HashSet<_> =
            ProblemKind::ALL.iter().map(|k| k.slug()).collect();
        let labels: std::collections::HashSet<_> =
            ProblemKind::ALL.iter().map(|k| k.english_label()).collect();
        assert_eq!(slugs.len(), ProblemKind::ALL.len());
        assert_eq!(labels.len(), ProblemKind::ALL.len());
    }

    /// The ids a link fills and the kind labels it sends must be the form's, or GitHub
    /// silently drops them. The form is YAML; its `id:` lines and its dropdown options
    /// are read as lines, without a YAML parser.
    #[test]
    fn every_field_is_a_form_id() {
        let form = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../.github/ISSUE_TEMPLATE/translation-error.yml"),
        )
        .unwrap_or_default();
        let ids: Vec<&str> = form
            .lines()
            .filter_map(|l| l.trim().strip_prefix("id:"))
            .map(str::trim)
            .collect();
        for key in field::ALL {
            assert!(ids.contains(&key), "the form has no field {key:?}: {ids:?}");
        }
        // A folded YAML string wraps over lines; GitHub shows it as one.
        let flat = form.split_whitespace().collect::<Vec<_>>().join(" ");
        for kind in ProblemKind::ALL {
            assert!(
                flat.contains(kind.english_label()),
                "the form does not mention the kind {:?}",
                kind.english_label()
            );
            assert!(
                flat.contains(&format!("({})", kind.slug())),
                "the form does not list the slug {}",
                kind.slug()
            );
        }
        assert!(form.contains(BLOCK_MARKER.split(':').next().unwrap_or_default()));
    }
}
