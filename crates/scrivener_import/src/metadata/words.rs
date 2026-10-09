// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The words the second half of the metadata mapping writes into the project: the details
//! of the tags it makes, the names it gives what had none, the "From Scrivener" folder, its
//! notes and the labels of what they list.
//!
//! The interface resolves each in the writer's language when the import starts, and the
//! converter stores them as they are, once (CLAUDE.md, i18n): none reaches a reader of the
//! finished book, since the compiler holds no reference to a project's tags, its pace plan
//! or its notes. Nothing here has an English default: a word the interface leaves empty is
//! written empty.
//!
//! On the wire the words travel as one list in [`MetadataWords::ORDER`], as the Plume
//! Creator and Manuskript importers' seeded names do, so a word added here is one more key
//! for the interface to resolve, appended to the order, not one more field of every
//! request.

use serde::{Deserialize, Serialize};

use crate::convert::TITLE_SLOT;

/// Where a pattern holding two values puts its second one: an option's name in
/// [`MetadataWords::field_value`], whose first slot ([`TITLE_SLOT`]) takes the field's.
/// Private use, like the first.
pub const VALUE_SLOT: char = '\u{E001}';

/// The words of the second half of the metadata mapping, in the writer's language.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataWords {
    /// A keyword tag's details, with [`TITLE_SLOT`] where the keyword's path in Scrivener's
    /// keyword list goes (`Characters › Ysenn`).
    pub keyword_details: String,
    /// A keyword with no name, with [`TITLE_SLOT`] where its number goes.
    pub unnamed_keyword: String,
    /// A keyword tag whose name another tag already holds, with [`TITLE_SLOT`] where the
    /// keyword's name goes (the English interface's `… (keyword)`).
    pub keyword_tag: String,
    /// A collection tag's details, saying where it came from; [`TITLE_SLOT`], when the
    /// pattern holds it, takes the collection's name.
    pub collection_details: String,
    /// A collection tag whose name another tag already holds, the same way as
    /// [`keyword_tag`](Self::keyword_tag).
    pub collection_tag: String,
    /// A custom field tag's details, with [`TITLE_SLOT`] where the field's name goes.
    pub field_details: String,
    /// The name of a tag made from one option of a list field: [`TITLE_SLOT`] takes the
    /// field's name, [`VALUE_SLOT`] the option's (`Platform: Mac`).
    pub field_value: String,
    /// A field tag whose name another tag already holds, the same way as
    /// [`keyword_tag`](Self::keyword_tag).
    pub field_tag: String,
    /// The label of the day off a pace plan takes on its deadline, when Scrivener's
    /// calculator would not schedule writing on it.
    pub deadline_day: String,
    /// The "From Scrivener" folder.
    pub from_scrivener: String,
    /// Its notes, one per [`NoteSection`](crate::structure::NoteSection).
    pub note_project: String,
    pub note_project_bookmarks: String,
    pub note_replacements: String,
    pub note_auto_complete: String,
    pub note_saved_searches: String,
    pub note_custom_metadata: String,
    pub note_targets: String,
    pub note_bookmarks: String,
    /// The labels of the lines of the project's note.
    pub abbreviated_title: String,
    pub subjects: String,
    pub company: String,
    pub session_target: String,
    pub writing_days: String,
    pub draft_target: String,
    pub deadline: String,
    /// The units a target counts in, as a heading names them.
    pub unit_words: String,
    pub unit_characters: String,
    pub unit_pages: String,
    /// The labels of a saved search's lines: what it looks for, where, and how the words
    /// combine.
    pub search_query: String,
    pub search_in: String,
    pub search_operator: String,
    /// What a saved search looks in, one word per kind Scrivener's `Type` names: every
    /// field, titles, the text, notes, synopses, keywords, labels, statuses, section types,
    /// custom metadata. A kind this importer does not know is not listed: Scrivener's own
    /// token is no word of the writer's.
    pub search_in_all: String,
    pub search_in_title: String,
    pub search_in_text: String,
    pub search_in_notes: String,
    pub search_in_synopsis: String,
    pub search_in_keywords: String,
    pub search_in_label: String,
    pub search_in_status: String,
    pub search_in_section_type: String,
    pub search_in_custom_metadata: String,
    /// How a saved search's words combine, one word per `Operator` Scrivener writes: any of
    /// them, all of them, the exact phrase. An operator this importer does not know is not
    /// listed.
    pub search_any: String,
    pub search_all: String,
    pub search_exact: String,
    /// A label and what follows it, on one line of a listing: [`TITLE_SLOT`] takes the
    /// label (a bookmark's path, a link's words), [`VALUE_SLOT`] what it says (the
    /// bookmark's note, the link's address). The English interface's `label: value`.
    pub label_value: String,
    /// What a compile replacement's line says about it: its search is a regular
    /// expression; it is switched off.
    pub replacement_regex: String,
    pub replacement_off: String,
    /// Said of a document whose values or bookmarks are listed in its own note rather
    /// than here.
    pub in_own_note: String,
}

impl MetadataWords {
    /// The order the words travel in, one key per word: the interface resolves each key
    /// and sends the words as one list in this order. Append, never reorder: a list built
    /// against an older order would put every word after the change in the wrong place.
    pub const ORDER: &'static [&'static str] = &[
        "keyword-details",
        "unnamed-keyword",
        "keyword-tag",
        "collection-details",
        "collection-tag",
        "field-details",
        "field-value",
        "field-tag",
        "deadline-day",
        "from-scrivener",
        "note-project",
        "note-project-bookmarks",
        "note-replacements",
        "note-auto-complete",
        "note-saved-searches",
        "note-custom-metadata",
        "note-targets",
        "note-bookmarks",
        "abbreviated-title",
        "subjects",
        "company",
        "session-target",
        "writing-days",
        "draft-target",
        "deadline",
        "unit-words",
        "unit-characters",
        "unit-pages",
        "search-query",
        "search-in",
        "search-operator",
        "search-in-all",
        "search-in-title",
        "search-in-text",
        "search-in-notes",
        "search-in-synopsis",
        "search-in-keywords",
        "search-in-label",
        "search-in-status",
        "search-in-section-type",
        "search-in-custom-metadata",
        "search-any",
        "search-all",
        "search-exact",
        "label-value",
        "replacement-regex",
        "replacement-off",
        "in-own-note",
    ];

    /// The word `key` names, to be set.
    fn slot(&mut self, key: &str) -> Option<&mut String> {
        Some(match key {
            "keyword-details" => &mut self.keyword_details,
            "unnamed-keyword" => &mut self.unnamed_keyword,
            "keyword-tag" => &mut self.keyword_tag,
            "collection-details" => &mut self.collection_details,
            "collection-tag" => &mut self.collection_tag,
            "field-details" => &mut self.field_details,
            "field-value" => &mut self.field_value,
            "field-tag" => &mut self.field_tag,
            "deadline-day" => &mut self.deadline_day,
            "from-scrivener" => &mut self.from_scrivener,
            "note-project" => &mut self.note_project,
            "note-project-bookmarks" => &mut self.note_project_bookmarks,
            "note-replacements" => &mut self.note_replacements,
            "note-auto-complete" => &mut self.note_auto_complete,
            "note-saved-searches" => &mut self.note_saved_searches,
            "note-custom-metadata" => &mut self.note_custom_metadata,
            "note-targets" => &mut self.note_targets,
            "note-bookmarks" => &mut self.note_bookmarks,
            "abbreviated-title" => &mut self.abbreviated_title,
            "subjects" => &mut self.subjects,
            "company" => &mut self.company,
            "session-target" => &mut self.session_target,
            "writing-days" => &mut self.writing_days,
            "draft-target" => &mut self.draft_target,
            "deadline" => &mut self.deadline,
            "unit-words" => &mut self.unit_words,
            "unit-characters" => &mut self.unit_characters,
            "unit-pages" => &mut self.unit_pages,
            "search-query" => &mut self.search_query,
            "search-in" => &mut self.search_in,
            "search-operator" => &mut self.search_operator,
            "search-in-all" => &mut self.search_in_all,
            "search-in-title" => &mut self.search_in_title,
            "search-in-text" => &mut self.search_in_text,
            "search-in-notes" => &mut self.search_in_notes,
            "search-in-synopsis" => &mut self.search_in_synopsis,
            "search-in-keywords" => &mut self.search_in_keywords,
            "search-in-label" => &mut self.search_in_label,
            "search-in-status" => &mut self.search_in_status,
            "search-in-section-type" => &mut self.search_in_section_type,
            "search-in-custom-metadata" => &mut self.search_in_custom_metadata,
            "search-any" => &mut self.search_any,
            "search-all" => &mut self.search_all,
            "search-exact" => &mut self.search_exact,
            "label-value" => &mut self.label_value,
            "replacement-regex" => &mut self.replacement_regex,
            "replacement-off" => &mut self.replacement_off,
            "in-own-note" => &mut self.in_own_note,
            _ => return None,
        })
    }

    /// Set the word `key` names to `value`. `false` for a key this importer does not know,
    /// which is set nowhere.
    pub fn set(&mut self, key: &str, value: &str) -> bool {
        match self.slot(key) {
            Some(word) => {
                *word = value.to_string();
                true
            }
            None => false,
        }
    }

    /// The word for a saved search's `Type` as Scrivener writes it (`CustomMetaData`), `None`
    /// for one this importer does not know.
    pub(crate) fn search_scope(&self, token: &str) -> Option<&str> {
        let word = match token.trim().to_ascii_lowercase().as_str() {
            "all" => &self.search_in_all,
            "title" | "titles" => &self.search_in_title,
            "text" => &self.search_in_text,
            "notes" => &self.search_in_notes,
            "synopsis" | "synopses" => &self.search_in_synopsis,
            "keywords" | "keyword" => &self.search_in_keywords,
            "label" => &self.search_in_label,
            "status" => &self.search_in_status,
            "sectiontype" => &self.search_in_section_type,
            "custommetadata" => &self.search_in_custom_metadata,
            _ => return None,
        };
        Some(word.as_str())
    }

    /// The word for a saved search's `Operator` as Scrivener writes it (`Any`), `None` for
    /// one this importer does not know.
    pub(crate) fn search_operator_word(&self, token: &str) -> Option<&str> {
        let word = match token.trim().to_ascii_lowercase().as_str() {
            "any" => &self.search_any,
            "all" => &self.search_all,
            "exact" => &self.search_exact,
            _ => return None,
        };
        Some(word.as_str())
    }

    /// The words from a list in [`Self::ORDER`], as the wire carries them. A list shorter
    /// than the order leaves the rest empty, and an entry past its end is skipped.
    pub fn from_list(list: &[String]) -> Self {
        let mut words = MetadataWords::default();
        for (key, value) in Self::ORDER.iter().zip(list) {
            words.set(key, value);
        }
        words
    }
}

/// `pattern` with `first` where its [`TITLE_SLOT`] is and `second` where its
/// [`VALUE_SLOT`] is, in one pass, so a value holding either character is never filled
/// into. A slot the pattern lacks puts its value at that end: the first before the
/// pattern, the second after it. The result is trimmed.
pub(crate) fn fill_two(pattern: &str, first: &str, second: &str) -> String {
    if pattern.trim().is_empty() {
        return format!("{first} {second}").trim().to_string();
    }
    let mut out = String::with_capacity(pattern.len() + first.len() + second.len() + 2);
    if !pattern.contains(TITLE_SLOT) {
        out.push_str(first);
        out.push(' ');
    }
    for c in pattern.chars() {
        match c {
            TITLE_SLOT => out.push_str(first),
            VALUE_SLOT => out.push_str(second),
            c => out.push(c),
        }
    }
    if !pattern.contains(VALUE_SLOT) {
        out.push(' ');
        out.push_str(second);
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_sets_its_own_word_once() {
        let mut words = MetadataWords::default();
        for (n, key) in MetadataWords::ORDER.iter().enumerate() {
            assert!(words.set(key, &format!("w{n}")), "{key}");
        }
        assert!(!words.set("label-details", "x"), "a key of the first half");
        let json = serde_json::to_value(&words).unwrap_or_default();
        let values: Vec<&str> = json
            .as_object()
            .into_iter()
            .flat_map(|o| o.values())
            .filter_map(|v| v.as_str())
            .collect();
        assert_eq!(
            values.len(),
            MetadataWords::ORDER.len(),
            "one key per field"
        );
        let mut distinct = values.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(distinct.len(), values.len(), "no two keys share a field");
    }

    #[test]
    fn two_values_fill_their_own_slots() {
        let pattern = format!("{TITLE_SLOT} : {VALUE_SLOT}");
        assert_eq!(fill_two(&pattern, "Plateforme", "Mac"), "Plateforme : Mac");
        assert_eq!(
            fill_two(&format!("{VALUE_SLOT} ({TITLE_SLOT})"), "F", "o"),
            "o (F)"
        );
        assert_eq!(fill_two("", "Field", "Option"), "Field Option");
        // A value holding a slot's character is never filled into.
        assert_eq!(
            fill_two(&pattern, &format!("a{VALUE_SLOT}"), "b"),
            format!("a{VALUE_SLOT} : b")
        );
    }
}
