// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The "From Scrivener" folder's notes, and what a note-like row lists of its own, as Djot
//! (D4-20).
//!
//! Every piece of Scrivener text (a title, a path, a value, a link's label) goes through
//! `skrib_format`'s escaper (X-07), so it reads back exactly as written: `10:30:45` stays
//! `10:30:45`, straight quotes stay straight, `I. Introduction` stays a paragraph. A value
//! of several lines keeps each line as a paragraph of its list item, indented so it stays
//! in the item. Nothing is wrapped in a construct deeper than three levels, and every note
//! is checked against the format's nesting ceiling before it is stored.
//!
//! Words the import adds (a note's headings, a line's label, a unit) are the interface's,
//! in the writer's language ([`MetadataWords`]); a word left empty leaves its heading or
//! label out rather than writing an empty one. A link leads somewhere only when its scheme
//! is one a reader may follow from a note (`http`, `https`, `mailto`, I3 S2): any other
//! destination, and a file on the writer's disk (kept by its name alone), is kept as text.

use skrib_format::{
    DjotInlineStyle, EscapeContext, djot_link_destination, escape_djot_text, guard_djot_line_start,
    push_djot_run,
};

use super::collect::{Entry, LinkLine, Listing, ProjectLine, SheetExtra, TargetLine};
use super::notice::TargetUnit;
use super::words::{MetadataWords, VALUE_SLOT};
use crate::convert::TITLE_SLOT;
use crate::structure::NoteSection;

/// One piece of text as inline Djot that reads back as itself: line breaks become spaces.
fn inline(text: &str) -> String {
    let flat: String = text
        .chars()
        .map(|c| if matches!(c, '\n' | '\r') { ' ' } else { c })
        .collect();
    escape_djot_text(flat.trim(), EscapeContext::default())
}

/// A separator between the items of one line, its spaces kept.
fn separator_of(separator: &str) -> String {
    escape_djot_text(separator, EscapeContext::default())
}

/// `text` in bold.
fn strong(text: &str) -> String {
    let flat: String = text
        .chars()
        .map(|c| if matches!(c, '\n' | '\r') { ' ' } else { c })
        .collect();
    let mut out = String::new();
    push_djot_run(
        &mut out,
        flat.trim(),
        DjotInlineStyle {
            bold: true,
            ..DjotInlineStyle::default()
        },
        "",
        "",
    );
    out
}

/// `text` as code: written verbatim, nothing in it read as markup, every character kept,
/// the spaces at either end included (a compile replacement of two spaces by one is those
/// spaces). The fences are one backtick longer than the text's longest run of them; a text
/// opening or closing with a backtick takes one space on that side, which Djot removes
/// again on reading. A line break, which would end the listing's line, is shown as `↵`.
/// Djot also drops the spaces between a fence and a backtick at the edge of the text,
/// whatever pads them: those few are shown as `␣`, so the listing never shows fewer
/// spaces than the rule holds.
fn code(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let mut text: String = text
        .replace("\r\n", "\n")
        .chars()
        .map(|c| if matches!(c, '\n' | '\r') { '↵' } else { c })
        .collect();
    let space = |c: char| c.is_ascii_whitespace();
    let core = text.trim_matches(space);
    if !core.is_empty() && (core.starts_with('`') || core.ends_with('`')) {
        let lead = text.len() - text.trim_start_matches(space).len();
        let trail = text.len() - text.trim_end_matches(space).len();
        let visible = |n: usize, at_backtick: bool| {
            if at_backtick {
                "␣".repeat(n)
            } else {
                " ".repeat(n)
            }
        };
        text = format!(
            "{}{core}{}",
            visible(lead, core.starts_with('`')),
            visible(trail, core.ends_with('`'))
        );
    }
    let (mut longest, mut current) = (0, 0);
    for c in text.chars() {
        if c == '`' {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    let fence = "`".repeat(longest + 1);
    let pad_start = if text.starts_with('`') { " " } else { "" };
    let pad_end = if text.ends_with('`') { " " } else { "" };
    format!("{fence}{pad_start}{text}{pad_end}{fence}")
}

/// `label` and `value`, both inline Djot already, joined as the writer's language joins a
/// label and what follows it (`pattern`, [`MetadataWords::label_value`]): the pattern's own
/// characters escaped, a slot it lacks putting its value at that end. An empty pattern
/// joins the two with a space.
fn labelled(pattern: &str, label: &str, value: &str) -> String {
    if pattern.trim().is_empty() {
        return format!("{label} {value}").trim().to_string();
    }
    fn flush(literal: &mut String, out: &mut String) {
        if !literal.is_empty() {
            out.push_str(&escape_djot_text(literal, EscapeContext::default()));
            literal.clear();
        }
    }
    let mut out = String::new();
    let mut literal = String::new();
    if !pattern.contains(TITLE_SLOT) {
        out.push_str(label);
        out.push(' ');
    }
    for c in pattern.chars() {
        match c {
            TITLE_SLOT => {
                flush(&mut literal, &mut out);
                out.push_str(label);
            }
            VALUE_SLOT => {
                flush(&mut literal, &mut out);
                out.push_str(value);
            }
            c => literal.push(c),
        }
    }
    flush(&mut literal, &mut out);
    if !pattern.contains(VALUE_SLOT) {
        out.push(' ');
        out.push_str(value);
    }
    out.trim().to_string()
}

/// The non-empty lines of a value, each trimmed.
fn lines(value: &str) -> impl Iterator<Item = &str> {
    value
        .split(['\n', '\r'])
        .map(str::trim)
        .filter(|l| !l.is_empty())
}

/// A note's text, block by block.
#[derive(Default)]
struct Djot {
    blocks: Vec<String>,
}

impl Djot {
    /// A heading of `level`, when its words are not empty.
    fn heading(&mut self, level: usize, text: &str) {
        let text = inline(text);
        if !text.is_empty() {
            self.blocks.push(format!("{} {text}", "#".repeat(level)));
        }
    }

    /// A paragraph of inline Djot, its start guarded.
    fn paragraph(&mut self, content: &str) {
        if !content.is_empty() {
            self.blocks.push(guard_djot_line_start(content));
        }
    }

    /// A list item: its first line, then paragraphs and a nested list, all indented under
    /// it so they stay in the item.
    fn item(&mut self, first: &str, paragraphs: &[String], nested: &[String]) {
        let mut out = format!("- {}", guard_djot_line_start(first));
        for paragraph in paragraphs.iter().filter(|p| !p.is_empty()) {
            out.push_str("\n\n  ");
            out.push_str(&guard_djot_line_start(paragraph));
        }
        if !nested.is_empty() {
            out.push('\n');
            for line in nested {
                out.push_str("\n  - ");
                out.push_str(&guard_djot_line_start(line));
            }
        }
        self.blocks.push(out);
    }

    fn finish(self) -> String {
        let djot = self.blocks.join("\n\n");
        // The ceiling is far above anything written here; a note that met it all the same
        // keeps its words as plain paragraphs rather than be refused.
        if skrib_format::djot_depth::check(&djot).is_ok() {
            return djot;
        }
        match skrib_format::djot_plain_text(&djot) {
            Ok((plain, _)) => skrib_format::plain_text_to_djot_verbatim(&plain),
            Err(_) => String::new(),
        }
    }
}

/// A count and its unit, as a line says it: `1000 (words)`.
fn quantity(value: i64, unit: TargetUnit, words: &MetadataWords) -> String {
    let unit = match unit {
        TargetUnit::Words => &words.unit_words,
        TargetUnit::Characters => &words.unit_characters,
        TargetUnit::Pages => &words.unit_pages,
    };
    if unit.trim().is_empty() {
        value.to_string()
    } else {
        format!("{value} ({})", inline(unit))
    }
}

/// Whether a reader may follow `url` from a note.
fn followable(url: &str) -> bool {
    let lower = url.trim().to_ascii_lowercase();
    ["http://", "https://", "mailto:"]
        .iter()
        .any(|scheme| lower.starts_with(scheme) && lower.len() > scheme.len())
}

/// A bookmark as one line.
fn link_line(line: &LinkLine, words: &MetadataWords) -> String {
    match line {
        LinkLine::Item { path, note } => match note {
            Some(note) => labelled(&words.label_value, &inline(path), &inline(note)),
            None => inline(path),
        },
        LinkLine::External {
            label,
            url,
            local_file,
        } => {
            let label = label.as_deref().map(str::trim).filter(|l| !l.is_empty());
            if !local_file && followable(url) {
                let text = inline(label.unwrap_or(url));
                format!("[{text}]({})", djot_link_destination(url.trim()))
            } else {
                match label {
                    Some(label) => labelled(&words.label_value, &inline(label), &inline(url)),
                    None => inline(url),
                }
            }
        }
    }
}

fn project(listing: &Listing, words: &MetadataWords, separator: &str) -> String {
    let mut djot = Djot::default();
    for line in &listing.project {
        let (label, value) = match line {
            ProjectLine::AbbreviatedTitle(title) => (&words.abbreviated_title, inline(title)),
            ProjectLine::Subjects(subjects) => (
                &words.subjects,
                subjects
                    .iter()
                    .map(|s| inline(s))
                    .collect::<Vec<_>>()
                    .join(&separator_of(separator)),
            ),
            ProjectLine::Company(company) => (&words.company, inline(company)),
            ProjectLine::SessionTarget {
                value,
                unit,
                calculated,
            } => (
                if *calculated {
                    &words.session_target_calculated
                } else {
                    &words.session_target
                },
                quantity(*value, *unit, words),
            ),
            ProjectLine::WritingDays(raw) => (&words.writing_days, inline(raw)),
            ProjectLine::WritingWeekdays(mask) => (
                &words.writing_days,
                [
                    &words.weekday_monday,
                    &words.weekday_tuesday,
                    &words.weekday_wednesday,
                    &words.weekday_thursday,
                    &words.weekday_friday,
                    &words.weekday_saturday,
                    &words.weekday_sunday,
                ]
                .iter()
                .enumerate()
                .filter(|(bit, _)| mask & (1 << bit) != 0)
                .map(|(_, day)| inline(day))
                .collect::<Vec<_>>()
                .join(&separator_of(separator)),
            ),
            ProjectLine::DraftTarget { value, unit } => {
                (&words.draft_target, quantity(*value, *unit, words))
            }
            ProjectLine::Deadline(day) => (&words.deadline, inline(day)),
        };
        if label.trim().is_empty() {
            djot.item(&value, &[], &[]);
        } else {
            djot.item(&strong(label), &[value], &[]);
        }
    }
    djot.finish()
}

fn project_links(listing: &Listing, words: &MetadataWords) -> String {
    let mut djot = Djot::default();
    for line in &listing.project_links {
        djot.item(&link_line(line, words), &[], &[]);
    }
    djot.finish()
}

fn replacements(listing: &Listing, words: &MetadataWords, separator: &str) -> String {
    let mut djot = Djot::default();
    for replacement in &listing.replacements {
        let mut line = format!(
            "{} → {}",
            code(&replacement.replace),
            code(&replacement.with)
        );
        let flags: Vec<String> = [
            (replacement.regex, &words.replacement_regex),
            (replacement.ignored, &words.replacement_off),
        ]
        .into_iter()
        .filter(|(on, word)| *on && !word.trim().is_empty())
        .map(|(_, word)| inline(word))
        .collect();
        if !flags.is_empty() {
            line.push_str(&format!(" ({})", flags.join(&separator_of(separator))));
        }
        djot.item(&line, &[], &[]);
    }
    djot.finish()
}

fn completions(listing: &Listing) -> String {
    let mut djot = Djot::default();
    for entry in &listing.completions {
        djot.item(&inline(entry), &[], &[]);
    }
    djot.finish()
}

fn searches(listing: &Listing, words: &MetadataWords, separator: &str) -> String {
    let mut djot = Djot::default();
    for search in &listing.searches {
        djot.heading(2, &search.title);
        let mut line = |label: &str, value: String| {
            if value.is_empty() {
                return;
            }
            if label.trim().is_empty() {
                djot.item(&value, &[], &[]);
            } else {
                djot.item(&strong(label), &[value], &[]);
            }
        };
        line(&words.search_query, code(&search.query));
        // Scrivener's own tokens (`CustomMetaData`, `Any`) are no words of the writer's:
        // each is listed in the interface's word for it, or not at all.
        let mut scope = search
            .kind
            .as_deref()
            .and_then(|kind| words.search_scope(kind))
            .map(inline)
            .unwrap_or_default();
        if !search.fields.is_empty() {
            let fields = search
                .fields
                .iter()
                .map(|f| inline(f))
                .collect::<Vec<_>>()
                .join(&separator_of(separator));
            scope = if scope.is_empty() {
                fields
            } else {
                format!("{scope} ({fields})")
            };
        }
        line(&words.search_in, scope);
        line(
            &words.search_operator,
            search
                .operator
                .as_deref()
                .and_then(|operator| words.search_operator_word(operator))
                .map(inline)
                .unwrap_or_default(),
        );
    }
    djot.finish()
}

/// A document's entry: its path in bold, then what it holds, or that its own note holds
/// it.
fn entry_item<T>(
    djot: &mut Djot,
    entry: &Entry<T>,
    words: &MetadataWords,
    paragraphs: impl Fn(&T) -> Vec<String>,
    nested: impl Fn(&T) -> Vec<String>,
) {
    match entry {
        Entry::Listed { path, value } => {
            djot.item(&strong(path), &paragraphs(value), &nested(value))
        }
        Entry::InOwnNote { path } => djot.item(&strong(path), &[inline(&words.in_own_note)], &[]),
    }
}

fn custom(listing: &Listing, words: &MetadataWords) -> String {
    let mut djot = Djot::default();
    for field in &listing.fields {
        djot.heading(2, &field.title);
        for entry in &field.entries {
            entry_item(
                &mut djot,
                entry,
                words,
                |value: &String| lines(value).map(inline).collect(),
                |_| Vec::new(),
            );
        }
    }
    djot.finish()
}

fn targets(listing: &Listing, words: &MetadataWords) -> String {
    let mut djot = Djot::default();
    for TargetLine { path, value, unit } in &listing.targets {
        djot.item(&strong(path), &[quantity(*value, *unit, words)], &[]);
    }
    djot.finish()
}

fn bookmarks(listing: &Listing, words: &MetadataWords) -> String {
    let mut djot = Djot::default();
    for entry in &listing.bookmarks {
        entry_item(
            &mut djot,
            entry,
            words,
            |_| Vec::new(),
            |links: &Vec<LinkLine>| links.iter().map(|l| link_line(l, words)).collect(),
        );
    }
    djot.finish()
}

/// The note for `section`, as Djot. `separator` joins the items of one line (the
/// interface's list separator).
pub(crate) fn note(
    section: NoteSection,
    listing: &Listing,
    words: &MetadataWords,
    separator: &str,
) -> String {
    match section {
        NoteSection::Project => project(listing, words, separator),
        NoteSection::ProjectBookmarks => project_links(listing, words),
        NoteSection::Replacements => replacements(listing, words, separator),
        NoteSection::AutoComplete => completions(listing),
        NoteSection::SavedSearches => searches(listing, words, separator),
        NoteSection::CustomMetadata => custom(listing, words),
        NoteSection::Targets => targets(listing, words),
        NoteSection::Bookmarks => bookmarks(listing, words),
    }
}

/// What a note-like row lists of its own, as Djot to follow its text: its custom metadata
/// under the note's own headings, one per field, then its bookmarks.
pub(crate) fn sheet(extra: &SheetExtra, words: &MetadataWords) -> String {
    let mut djot = Djot::default();
    if !extra.fields.is_empty() {
        djot.heading(2, &words.note_custom_metadata);
        for (field, value) in &extra.fields {
            djot.heading(3, field);
            for line in lines(value) {
                djot.paragraph(&inline(line));
            }
        }
    }
    if !extra.links.is_empty() {
        djot.heading(2, &words.note_bookmarks);
        for line in &extra.links {
            djot.item(&link_line(line, words), &[], &[]);
        }
    }
    djot.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a reader of the note sees, through the format's own reading of Djot.
    fn plain(djot: &str) -> String {
        match skrib_format::djot_plain_text(djot) {
            Ok((text, _)) => text,
            Err(e) => panic!("the note does not read: {e}"),
        }
    }

    #[test]
    fn a_value_reads_back_as_written_inside_its_item() {
        let listing = Listing {
            fields: vec![super::super::collect::FieldListing {
                title: "Summary".to_string(),
                entries: vec![Entry::Listed {
                    path: "Draft › Chapter 1 › Arrival".to_string(),
                    value: "At 10:30:45 she said \"no\" -- twice...\nI. The second line"
                        .to_string(),
                }],
            }],
            ..Listing::default()
        };
        let djot = note(
            NoteSection::CustomMetadata,
            &listing,
            &MetadataWords::default(),
            ", ",
        );
        let text = plain(&djot);
        assert!(text.contains("Summary"), "{text}");
        assert!(text.contains("Draft › Chapter 1 › Arrival"), "{text}");
        assert!(
            text.contains("At 10:30:45 she said \"no\" -- twice..."),
            "{text}"
        );
        assert!(text.contains("I. The second line"), "{text}");
        assert!(
            djot.contains("\n\n  "),
            "the value is indented under its item: {djot}"
        );
    }

    #[test]
    fn a_link_leads_somewhere_only_for_a_followable_scheme() {
        let web = LinkLine::External {
            label: Some("The map".to_string()),
            url: "https://example.org/a(b)".to_string(),
            local_file: false,
        };
        assert_eq!(
            link_line(&web, &MetadataWords::default()),
            "[The map](https://example.org/a%28b%29)"
        );
        let mail = LinkLine::External {
            label: None,
            url: "mailto:a@example.org".to_string(),
            local_file: false,
        };
        assert!(link_line(&mail, &MetadataWords::default()).starts_with("[mailto:a@example.org]("));
        let file = LinkLine::External {
            label: None,
            url: "notes.txt".to_string(),
            local_file: true,
        };
        assert_eq!(link_line(&file, &MetadataWords::default()), "notes.txt");
        let message = LinkLine::External {
            label: Some("Mail".to_string()),
            url: "message://%3c123%3e".to_string(),
            local_file: false,
        };
        assert!(
            !link_line(&message, &MetadataWords::default()).contains("]("),
            "{}",
            link_line(&message, &MetadataWords::default())
        );
    }

    #[test]
    fn an_empty_word_leaves_its_heading_out() {
        let extra = SheetExtra {
            fields: vec![("Eyes".to_string(), "Grey".to_string())],
            links: Vec::new(),
        };
        let djot = sheet(&extra, &MetadataWords::default());
        assert!(djot.starts_with("### Eyes"), "{djot}");
        let words = MetadataWords {
            note_custom_metadata: "Champs".to_string(),
            ..MetadataWords::default()
        };
        let djot = sheet(&extra, &words);
        assert!(djot.starts_with("## Champs\n\n### Eyes\n\nGrey"), "{djot}");
    }

    /// A replacement of spaces lists its spaces: one made only of them, and one with
    /// spaces at either end, read back with every space where it was.
    #[test]
    fn a_replacement_keeps_the_spaces_at_its_edges() {
        let rule = |replace: &str, with: &str| crate::model::CompileReplacement {
            file: crate::model::CompileFile::Xml,
            replace: replace.to_string(),
            with: with.to_string(),
            regex: false,
            ignored: false,
        };
        let listing = Listing {
            replacements: vec![rule("  ", " "), rule(" -- ", " — "), rule("`a` ", "b")],
            ..Listing::default()
        };
        let djot = note(
            NoteSection::Replacements,
            &listing,
            &MetadataWords::default(),
            ", ",
        );
        let text = plain(&djot);
        assert!(text.contains("   →  "), "{text:?} from {djot:?}");
        assert!(text.contains(" --  →  — "), "{text:?} from {djot:?}");
        assert!(text.contains("`a`␣ → b"), "{text:?} from {djot:?}");
        assert_eq!(code("a\nb"), "`a↵b`");
    }

    /// A saved search's kind and operator are listed in the interface's words, never as
    /// Scrivener's own tokens; one this importer does not know is left out.
    #[test]
    fn a_saved_search_is_listed_in_the_writers_words() {
        let search = |kind: &str, operator: &str| super::super::collect::SearchLine {
            title: "Statut".to_string(),
            query: "brouillon".to_string(),
            kind: Some(kind.to_string()),
            operator: Some(operator.to_string()),
            fields: vec!["Statut".to_string()],
        };
        let listing = Listing {
            searches: vec![
                search("CustomMetaData", "Any"),
                search("Snapshots", "Fuzzy"),
            ],
            ..Listing::default()
        };
        let words = MetadataWords {
            search_in: "Dans".to_string(),
            search_operator: "Mots".to_string(),
            search_in_custom_metadata: "métadonnées personnalisées".to_string(),
            search_any: "un des mots".to_string(),
            ..MetadataWords::default()
        };
        let text = plain(&note(NoteSection::SavedSearches, &listing, &words, ", "));
        for token in ["CustomMetaData", "Any", "Snapshots", "Fuzzy"] {
            assert!(!text.contains(token), "{token} in {text}");
        }
        assert!(
            text.contains("métadonnées personnalisées (Statut)"),
            "{text}"
        );
        assert!(text.contains("un des mots"), "{text}");
    }

    /// A label and what follows it are joined as the writer's language joins them.
    #[test]
    fn a_label_and_its_value_are_joined_in_the_writers_language() {
        let words = MetadataWords {
            label_value: format!("{TITLE_SLOT}\u{a0}: {VALUE_SLOT}"),
            ..MetadataWords::default()
        };
        let bookmark = LinkLine::Item {
            path: "Brouillon › Chapitre 1".to_string(),
            note: Some("à revoir".to_string()),
        };
        assert_eq!(
            plain(&link_line(&bookmark, &words)).trim(),
            "Brouillon › Chapitre 1\u{a0}: à revoir"
        );
        let mail = LinkLine::External {
            label: Some("Message".to_string()),
            url: "message://%3cid%3e".to_string(),
            local_file: false,
        };
        assert_eq!(
            plain(&link_line(&mail, &words)).trim(),
            "Message\u{a0}: message://%3cid%3e"
        );
    }

    #[test]
    fn a_replacement_keeps_its_pattern_verbatim() {
        let listing = Listing {
            replacements: vec![crate::model::CompileReplacement {
                file: crate::model::CompileFile::Xml,
                replace: r"\s+\[@".to_string(),
                with: "[@".to_string(),
                regex: true,
                ignored: true,
            }],
            ..Listing::default()
        };
        let words = MetadataWords {
            replacement_regex: "RE".to_string(),
            replacement_off: "OFF".to_string(),
            ..MetadataWords::default()
        };
        let djot = note(NoteSection::Replacements, &listing, &words, " + ");
        assert!(
            plain(&djot).contains(r"\s+\[@ → [@ (RE + OFF)"),
            "{}",
            plain(&djot)
        );
    }
}
