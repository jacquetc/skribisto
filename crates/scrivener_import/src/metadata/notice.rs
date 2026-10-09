// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What mapping the metadata did that the writer should know: the labels, the status list
//! and the dates (S1), the keywords, custom metadata, collections, bookmarks, targets, the
//! deadline, the auto-complete list, the icons and script mode (S2).
//!
//! Like every other notice of this importer, none words anything: the interface translates
//! the [`key`](MetadataNotice::key) in the writer's language and files it under its
//! [`group`](MetadataNotice::group). Names a notice carries (a label's, a rung's, a field's
//! title) are the writer's own words from the project, shown as written. Every key starts
//! with `scrivener-metadata-`.

use serde::{Deserialize, Serialize};

use crate::structure::{NoteSection, ReportGroup};

/// Which kind of a document's metadata a notice is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Field {
    Label,
    Status,
    Keyword,
    /// A custom metadata value.
    CustomField,
    /// A document bookmark (format 2.0) or reference (format 1.x).
    Bookmark,
    /// A document target.
    Target,
}

/// What a target counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetUnit {
    Words,
    Characters,
    /// Pages, which Skribisto does not count.
    Pages,
}

/// Why the draft target did not become the book's goal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DraftTargetReason {
    /// It counts pages.
    Pages,
    /// The Draft became several books, or none: no one book is the draft.
    NoSingleBook,
}

/// Why the deadline did not become a pace plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeadlineReason {
    /// The day is behind the import.
    Passed,
    /// No writing day is left from the day of the import to the deadline: none of the
    /// writing days falls in between, the deadline itself a day off when Scrivener's
    /// calculator keeps it free of writing.
    NoWritingDayLeft,
    /// The draft has no word target to plan towards.
    NoWordGoal,
    /// The draft target counts characters; a pace plan counts words.
    CharacterGoal,
    /// The Draft became several books, or none.
    NoSingleBook,
    /// The date could not be read.
    Unreadable,
    /// The project is a template: its deadline is the one saved in the template, not one
    /// set for the project being started from it.
    Template,
}

/// Something mapping the metadata did that the writer should know.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetadataNotice {
    // ------------------------------------------------ check these
    /// Status rungs whose name the import does not know, given a kind by their place in
    /// the ladder (the first to start, the last to finish): the writer can change a rung's
    /// kind in Settings ▸ Work ▸ Statuses. The names, as written.
    StatusCategoriesGuessed { rungs: Vec<String> },
    /// Items whose creation or last change date could not be read: their rows carry the
    /// date of the import instead.
    DatesUnreadable { items: u32 },
    /// Labels, or the values of a field named like a point of view, that match the titles of
    /// story-bible entries: the writer may want them as the point of view of the documents
    /// that wore them (Inspector). Nothing was linked (D4-6). Both lists as written.
    PovHint {
        values: Vec<String>,
        matches: Vec<String>,
    },
    /// A checkbox field whose definition carries a setting the import cannot read (a tick
    /// by default, perhaps): only the documents ticked explicitly wear its tag. `field` as
    /// written.
    CheckboxDefaultUnknown { field: String },
    /// Goals in the unit the project does not count in, kept and hidden until the unit is
    /// switched (Settings ▸ Work).
    GoalsInOtherUnit { count: u32 },
    /// Scrivener counted the draft target over another set of documents than Skribisto
    /// does (every document, or only the compile group): the book's goal reads against its
    /// exported scenes.
    CountingDiffers,
    /// Writing days set to some days of the week only, which the import cannot read: the
    /// pace plan counts every day. `raw` as written.
    WritingDaysNotRead { raw: String },
    /// Documents Scrivener wrote in script mode, brought across as prose.
    ScriptModeRows { rows: u32 },

    // ------------------------------------------------ brought across differently
    /// The labels documents wear, now tags on those documents (never story-bible tags).
    LabelsBecameTags { tags: u32, rows: u32 },
    /// The status list, now the project's workflow ladder in the same order, and the rows
    /// wearing a rung.
    StatusesBecameLadder { rungs: u32, rows: u32 },
    /// The project had no status list: it starts with Skribisto's default ladder.
    LadderSeeded { rungs: u32 },
    /// The field was renamed in Scrivener ("Label" to "Time"): its name is in the details
    /// of each tag or rung it became. `title` as written.
    FieldRenamed { field: Field, title: String },
    /// A value Scrivener kept beside its own "none" under the same name (what an upgrade
    /// from Scrivener 1 leaves), read as none: `name` as written, on `items` items.
    NoneValueCollapsed {
        field: Field,
        name: String,
        items: u32,
    },
    /// A label or a status with no name, named with its number.
    UnnamedValue { field: Field, id: i64 },
    /// Tags whose name another tag already held, renamed: the names they took.
    TagNameDisambiguated { names: Vec<String> },
    /// Statuses whose name an earlier status already held (Scrivener allows two of one
    /// name), their rungs renamed with an ordinal: the names they took.
    RungNamesDisambiguated { names: Vec<String> },
    /// The keywords documents wear, now tags on those documents.
    KeywordsBecameTags { tags: u32, rows: u32 },
    /// Keywords nested in Scrivener's keyword list, flat tags now: each tag's details name
    /// its place in the list.
    KeywordHierarchyFlattened { keywords: u32 },
    /// List and checkbox fields, now tags on the documents that had a value: one tag per
    /// option worn, one per checkbox.
    FieldsBecameTags { fields: u32, tags: u32, rows: u32 },
    /// Collections, now tags on their documents.
    CollectionsBecameTags { tags: u32, rows: u32 },
    /// Collections whose own order of documents a tag cannot hold. Names as written.
    CollectionOrderLost { names: Vec<String> },
    /// Bookmarks and references that became links Skribisto shows: a note's *Links*, a
    /// scene's *Cast*.
    BookmarksBecameLinks { count: u32 },
    /// Document targets, now goals on the documents whose writing they measure.
    GoalsKept { rows: u32 },
    /// The draft target, now the book's goal.
    DraftTargetKept { value: i64, unit: TargetUnit },
    /// The deadline, now the book's pace plan, from the day of the import to `day` (a Unix
    /// timestamp, midnight UTC of the deadline's own calendar day). `day_off`: the plan
    /// keeps the deadline itself free of writing, as Scrivener's calculator did.
    PacePlanned { day: i64, day_off: bool },
    /// Auto-complete entries of one word, now words of the project's dictionary.
    DictionaryWords { count: u32 },
    /// Things Skribisto has no place for, listed in one note of the "From Scrivener"
    /// folder: `count` of them in the note for `section`.
    ListedInImportNote { section: NoteSection, count: u32 },
    /// Document targets on documents whose writing no goal measures (a note), listed in the
    /// "From Scrivener" folder rather than kept as goals that would read zero forever.
    TargetsOnNotes { count: u32 },
    /// Document targets on documents split at their headings, whose writing is spread over
    /// chapters side by side that no one goal measures: listed in the "From Scrivener"
    /// folder rather than kept on the first chapter alone.
    TargetsOnSplitDocuments { count: u32 },
    /// Bookmarks pointing at documents that have no row in the new project (a document
    /// template, a folder left out), listed by where the documents were in Scrivener.
    BookmarkEndsWithoutRow { count: u32 },

    // ------------------------------------------------ left behind
    /// Labels no document wears, not imported: their names, as written.
    UnusedLabelsSkipped { names: Vec<String> },
    /// Metadata on items that have no row in the new project (document templates, a folder
    /// or a Trash left out): it did not come across.
    ValuesWithoutRow { field: Field, items: u32 },
    /// Keywords no document wears, not imported: their names, as written.
    UnusedKeywordsSkipped { names: Vec<String> },
    /// Values of a list field that name none of its options. `field` as written.
    ListValueInvalid { field: String, count: u32 },
    /// Values of fields the project does not define, which Scrivener does not show either:
    /// the field ids as written, and how many values.
    UndefinedCustomField { ids: Vec<String>, values: u32 },
    /// Documents of a collection that have no row in the new project, or that the project
    /// does not hold. `name` as written.
    CollectionMembersMissing { name: String, count: u32 },
    /// Collections holding no document, not imported. Names as written.
    EmptyCollectionsSkipped { names: Vec<String> },
    /// Bookmarks naming a document the project does not hold.
    BookmarksUnresolved { count: u32 },
    /// Document targets counting pages, which Skribisto does not count.
    TargetsInPages { count: u32 },
    /// Document targets, and the draft target, whose count is no number, is below zero, or
    /// is more than a goal holds: not imported.
    TargetsUnreadable { count: u32 },
    /// The draft target, which no book's goal took, listed in the "From Scrivener" folder.
    DraftTargetNotKept {
        reason: DraftTargetReason,
        value: i64,
        unit: TargetUnit,
    },
    /// The deadline, which no pace plan took, listed in the "From Scrivener" folder. `day`
    /// as [`PacePlanned`](Self::PacePlanned)'s, `None` when it could not be read.
    DeadlineNotPlanned {
        reason: DeadlineReason,
        day: Option<i64>,
    },
    /// The session target, an application setting in Skribisto rather than a project's:
    /// listed in the "From Scrivener" folder. `calculated` when Scrivener worked it out from
    /// the deadline and the writing days (`DeterminedFromDeadline="Yes"`) rather than the
    /// writer typing it: Scrivener 3 for Mac stores no typed target once writing days are
    /// set (M19).
    SessionTargetNotImported {
        value: i64,
        unit: TargetUnit,
        calculated: bool,
    },
    /// Binder icons, which Skribisto draws from the kind of each row: `rows` rows had one,
    /// `names` the five most used, as written.
    IconsDropped { rows: u32, names: Vec<String> },
}

impl MetadataNotice {
    /// Every key a metadata notice can have.
    pub const KEYS: &'static [&'static str] = &[
        "scrivener-metadata-status-categories-guessed",
        "scrivener-metadata-dates-unreadable",
        "scrivener-metadata-labels-became-tags",
        "scrivener-metadata-statuses-became-ladder",
        "scrivener-metadata-ladder-seeded",
        "scrivener-metadata-field-renamed",
        "scrivener-metadata-none-value-collapsed",
        "scrivener-metadata-unnamed-value",
        "scrivener-metadata-tag-name-disambiguated",
        "scrivener-metadata-rung-names-disambiguated",
        "scrivener-metadata-unused-labels-skipped",
        "scrivener-metadata-values-without-row",
        "scrivener-metadata-pov-hint",
        "scrivener-metadata-checkbox-default-unknown",
        "scrivener-metadata-goals-in-other-unit",
        "scrivener-metadata-counting-differs",
        "scrivener-metadata-writing-days-not-read",
        "scrivener-metadata-script-mode-rows",
        "scrivener-metadata-keywords-became-tags",
        "scrivener-metadata-keyword-hierarchy-flattened",
        "scrivener-metadata-fields-became-tags",
        "scrivener-metadata-collections-became-tags",
        "scrivener-metadata-collection-order-lost",
        "scrivener-metadata-bookmarks-became-links",
        "scrivener-metadata-goals-kept",
        "scrivener-metadata-draft-target-kept",
        "scrivener-metadata-pace-planned",
        "scrivener-metadata-dictionary-words",
        "scrivener-metadata-listed-in-import-note",
        "scrivener-metadata-targets-on-notes",
        "scrivener-metadata-targets-on-split-documents",
        "scrivener-metadata-bookmark-ends-without-row",
        "scrivener-metadata-unused-keywords-skipped",
        "scrivener-metadata-list-value-invalid",
        "scrivener-metadata-undefined-custom-field",
        "scrivener-metadata-collection-members-missing",
        "scrivener-metadata-empty-collections-skipped",
        "scrivener-metadata-bookmarks-unresolved",
        "scrivener-metadata-targets-in-pages",
        "scrivener-metadata-targets-unreadable",
        "scrivener-metadata-draft-target-not-kept",
        "scrivener-metadata-deadline-not-planned",
        "scrivener-metadata-session-target-not-imported",
        "scrivener-metadata-icons-dropped",
    ];

    /// The stable key the interface words this notice by.
    pub fn key(&self) -> &'static str {
        match self {
            MetadataNotice::StatusCategoriesGuessed { .. } => {
                "scrivener-metadata-status-categories-guessed"
            }
            MetadataNotice::DatesUnreadable { .. } => "scrivener-metadata-dates-unreadable",
            MetadataNotice::LabelsBecameTags { .. } => "scrivener-metadata-labels-became-tags",
            MetadataNotice::StatusesBecameLadder { .. } => {
                "scrivener-metadata-statuses-became-ladder"
            }
            MetadataNotice::LadderSeeded { .. } => "scrivener-metadata-ladder-seeded",
            MetadataNotice::FieldRenamed { .. } => "scrivener-metadata-field-renamed",
            MetadataNotice::NoneValueCollapsed { .. } => "scrivener-metadata-none-value-collapsed",
            MetadataNotice::UnnamedValue { .. } => "scrivener-metadata-unnamed-value",
            MetadataNotice::TagNameDisambiguated { .. } => {
                "scrivener-metadata-tag-name-disambiguated"
            }
            MetadataNotice::RungNamesDisambiguated { .. } => {
                "scrivener-metadata-rung-names-disambiguated"
            }
            MetadataNotice::UnusedLabelsSkipped { .. } => {
                "scrivener-metadata-unused-labels-skipped"
            }
            MetadataNotice::ValuesWithoutRow { .. } => "scrivener-metadata-values-without-row",
            MetadataNotice::PovHint { .. } => "scrivener-metadata-pov-hint",
            MetadataNotice::CheckboxDefaultUnknown { .. } => {
                "scrivener-metadata-checkbox-default-unknown"
            }
            MetadataNotice::GoalsInOtherUnit { .. } => "scrivener-metadata-goals-in-other-unit",
            MetadataNotice::CountingDiffers => "scrivener-metadata-counting-differs",
            MetadataNotice::WritingDaysNotRead { .. } => "scrivener-metadata-writing-days-not-read",
            MetadataNotice::ScriptModeRows { .. } => "scrivener-metadata-script-mode-rows",
            MetadataNotice::KeywordsBecameTags { .. } => "scrivener-metadata-keywords-became-tags",
            MetadataNotice::KeywordHierarchyFlattened { .. } => {
                "scrivener-metadata-keyword-hierarchy-flattened"
            }
            MetadataNotice::FieldsBecameTags { .. } => "scrivener-metadata-fields-became-tags",
            MetadataNotice::CollectionsBecameTags { .. } => {
                "scrivener-metadata-collections-became-tags"
            }
            MetadataNotice::CollectionOrderLost { .. } => {
                "scrivener-metadata-collection-order-lost"
            }
            MetadataNotice::BookmarksBecameLinks { .. } => {
                "scrivener-metadata-bookmarks-became-links"
            }
            MetadataNotice::GoalsKept { .. } => "scrivener-metadata-goals-kept",
            MetadataNotice::DraftTargetKept { .. } => "scrivener-metadata-draft-target-kept",
            MetadataNotice::PacePlanned { .. } => "scrivener-metadata-pace-planned",
            MetadataNotice::DictionaryWords { .. } => "scrivener-metadata-dictionary-words",
            MetadataNotice::ListedInImportNote { .. } => "scrivener-metadata-listed-in-import-note",
            MetadataNotice::TargetsOnNotes { .. } => "scrivener-metadata-targets-on-notes",
            MetadataNotice::TargetsOnSplitDocuments { .. } => {
                "scrivener-metadata-targets-on-split-documents"
            }
            MetadataNotice::BookmarkEndsWithoutRow { .. } => {
                "scrivener-metadata-bookmark-ends-without-row"
            }
            MetadataNotice::UnusedKeywordsSkipped { .. } => {
                "scrivener-metadata-unused-keywords-skipped"
            }
            MetadataNotice::ListValueInvalid { .. } => "scrivener-metadata-list-value-invalid",
            MetadataNotice::UndefinedCustomField { .. } => {
                "scrivener-metadata-undefined-custom-field"
            }
            MetadataNotice::CollectionMembersMissing { .. } => {
                "scrivener-metadata-collection-members-missing"
            }
            MetadataNotice::EmptyCollectionsSkipped { .. } => {
                "scrivener-metadata-empty-collections-skipped"
            }
            MetadataNotice::BookmarksUnresolved { .. } => "scrivener-metadata-bookmarks-unresolved",
            MetadataNotice::TargetsInPages { .. } => "scrivener-metadata-targets-in-pages",
            MetadataNotice::TargetsUnreadable { .. } => "scrivener-metadata-targets-unreadable",
            MetadataNotice::DraftTargetNotKept { .. } => "scrivener-metadata-draft-target-not-kept",
            MetadataNotice::DeadlineNotPlanned { .. } => "scrivener-metadata-deadline-not-planned",
            MetadataNotice::SessionTargetNotImported { .. } => {
                "scrivener-metadata-session-target-not-imported"
            }
            MetadataNotice::IconsDropped { .. } => "scrivener-metadata-icons-dropped",
        }
    }

    /// Which group of the report it belongs to.
    pub fn group(&self) -> ReportGroup {
        match self {
            MetadataNotice::StatusCategoriesGuessed { .. }
            | MetadataNotice::DatesUnreadable { .. }
            | MetadataNotice::PovHint { .. }
            | MetadataNotice::CheckboxDefaultUnknown { .. }
            | MetadataNotice::GoalsInOtherUnit { .. }
            | MetadataNotice::CountingDiffers
            | MetadataNotice::WritingDaysNotRead { .. }
            | MetadataNotice::ScriptModeRows { .. } => ReportGroup::CheckThese,
            MetadataNotice::LabelsBecameTags { .. }
            | MetadataNotice::StatusesBecameLadder { .. }
            | MetadataNotice::LadderSeeded { .. }
            | MetadataNotice::FieldRenamed { .. }
            | MetadataNotice::NoneValueCollapsed { .. }
            | MetadataNotice::UnnamedValue { .. }
            | MetadataNotice::TagNameDisambiguated { .. }
            | MetadataNotice::RungNamesDisambiguated { .. }
            | MetadataNotice::KeywordsBecameTags { .. }
            | MetadataNotice::KeywordHierarchyFlattened { .. }
            | MetadataNotice::FieldsBecameTags { .. }
            | MetadataNotice::CollectionsBecameTags { .. }
            | MetadataNotice::CollectionOrderLost { .. }
            | MetadataNotice::BookmarksBecameLinks { .. }
            | MetadataNotice::GoalsKept { .. }
            | MetadataNotice::DraftTargetKept { .. }
            | MetadataNotice::PacePlanned { .. }
            | MetadataNotice::DictionaryWords { .. }
            | MetadataNotice::ListedInImportNote { .. }
            | MetadataNotice::TargetsOnNotes { .. }
            | MetadataNotice::TargetsOnSplitDocuments { .. }
            | MetadataNotice::BookmarkEndsWithoutRow { .. } => {
                ReportGroup::BroughtAcrossDifferently
            }
            MetadataNotice::UnusedLabelsSkipped { .. }
            | MetadataNotice::ValuesWithoutRow { .. }
            | MetadataNotice::UnusedKeywordsSkipped { .. }
            | MetadataNotice::ListValueInvalid { .. }
            | MetadataNotice::UndefinedCustomField { .. }
            | MetadataNotice::CollectionMembersMissing { .. }
            | MetadataNotice::EmptyCollectionsSkipped { .. }
            | MetadataNotice::BookmarksUnresolved { .. }
            | MetadataNotice::TargetsInPages { .. }
            | MetadataNotice::TargetsUnreadable { .. }
            | MetadataNotice::DraftTargetNotKept { .. }
            | MetadataNotice::DeadlineNotPlanned { .. }
            | MetadataNotice::SessionTargetNotImported { .. }
            | MetadataNotice::IconsDropped { .. } => ReportGroup::LeftBehind,
        }
    }
}

/// One notice of every kind, for the tests of every layer that words them.
#[doc(hidden)]
pub fn every_example() -> Vec<MetadataNotice> {
    let names = || vec!["One".to_string(), "Two".to_string()];
    vec![
        MetadataNotice::StatusCategoriesGuessed { rungs: names() },
        MetadataNotice::DatesUnreadable { items: 1 },
        MetadataNotice::LabelsBecameTags { tags: 1, rows: 2 },
        MetadataNotice::StatusesBecameLadder { rungs: 1, rows: 2 },
        MetadataNotice::LadderSeeded { rungs: 5 },
        MetadataNotice::FieldRenamed {
            field: Field::Label,
            title: "Time".into(),
        },
        MetadataNotice::NoneValueCollapsed {
            field: Field::Status,
            name: "N/A".into(),
            items: 3,
        },
        MetadataNotice::UnnamedValue {
            field: Field::Keyword,
            id: 3,
        },
        MetadataNotice::TagNameDisambiguated { names: names() },
        MetadataNotice::RungNamesDisambiguated { names: names() },
        MetadataNotice::UnusedLabelsSkipped { names: names() },
        MetadataNotice::ValuesWithoutRow {
            field: Field::CustomField,
            items: 1,
        },
        MetadataNotice::PovHint {
            values: names(),
            matches: names(),
        },
        MetadataNotice::CheckboxDefaultUnknown {
            field: "Done".into(),
        },
        MetadataNotice::GoalsInOtherUnit { count: 2 },
        MetadataNotice::CountingDiffers,
        MetadataNotice::WritingDaysNotRead {
            raw: "1-2, 4-6".into(),
        },
        MetadataNotice::ScriptModeRows { rows: 4 },
        MetadataNotice::KeywordsBecameTags { tags: 2, rows: 3 },
        MetadataNotice::KeywordHierarchyFlattened { keywords: 2 },
        MetadataNotice::FieldsBecameTags {
            fields: 1,
            tags: 2,
            rows: 3,
        },
        MetadataNotice::CollectionsBecameTags { tags: 1, rows: 2 },
        MetadataNotice::CollectionOrderLost { names: names() },
        MetadataNotice::BookmarksBecameLinks { count: 2 },
        MetadataNotice::GoalsKept { rows: 2 },
        MetadataNotice::DraftTargetKept {
            value: 80_000,
            unit: TargetUnit::Words,
        },
        MetadataNotice::PacePlanned {
            day: 1_800_000_000,
            day_off: true,
        },
        MetadataNotice::DictionaryWords { count: 4 },
        MetadataNotice::ListedInImportNote {
            section: NoteSection::Bookmarks,
            count: 3,
        },
        MetadataNotice::TargetsOnNotes { count: 2 },
        MetadataNotice::TargetsOnSplitDocuments { count: 1 },
        MetadataNotice::BookmarkEndsWithoutRow { count: 1 },
        MetadataNotice::UnusedKeywordsSkipped { names: names() },
        MetadataNotice::ListValueInvalid {
            field: "Platform".into(),
            count: 1,
        },
        MetadataNotice::UndefinedCustomField {
            ids: names(),
            values: 2,
        },
        MetadataNotice::CollectionMembersMissing {
            name: "To revise".into(),
            count: 1,
        },
        MetadataNotice::EmptyCollectionsSkipped { names: names() },
        MetadataNotice::BookmarksUnresolved { count: 1 },
        MetadataNotice::TargetsInPages { count: 1 },
        MetadataNotice::TargetsUnreadable { count: 2 },
        MetadataNotice::DraftTargetNotKept {
            reason: DraftTargetReason::Pages,
            value: 8,
            unit: TargetUnit::Pages,
        },
        MetadataNotice::DeadlineNotPlanned {
            reason: DeadlineReason::Passed,
            day: Some(1_300_000_000),
        },
        MetadataNotice::SessionTargetNotImported {
            value: 1_000,
            unit: TargetUnit::Words,
            calculated: false,
        },
        MetadataNotice::IconsDropped {
            rows: 3,
            names: names(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_is_listed_once_and_is_the_importers_own() {
        let all = super::every_example();
        assert_eq!(all.len(), MetadataNotice::KEYS.len());
        for notice in &all {
            assert!(MetadataNotice::KEYS.contains(&notice.key()), "{notice:?}");
            assert!(notice.key().starts_with("scrivener-metadata-"));
        }
        let mut keys: Vec<&str> = all.iter().map(MetadataNotice::key).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), all.len());
    }
}
