// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What mapping the labels, the status list and the dates did that the writer should know.
//!
//! Like every other notice of this importer, none words anything: the interface translates
//! the [`key`](MetadataNotice::key) in the writer's language and files it under its
//! [`group`](MetadataNotice::group). Names a notice carries (a label's, a rung's, a field's
//! title) are the writer's own words from the project, shown as written. Every key starts
//! with `scrivener-metadata-`.

use serde::{Deserialize, Serialize};

use crate::structure::ReportGroup;

/// Which of Scrivener's two single-valued fields a notice is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Field {
    Label,
    Status,
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
    /// Label tags whose name another tag already held, renamed: the names they took.
    TagNameDisambiguated { names: Vec<String> },

    // ------------------------------------------------ left behind
    /// Labels no document wears, not imported: their names, as written.
    UnusedLabelsSkipped { names: Vec<String> },
    /// Labels or statuses on items that have no row in the new project (document
    /// templates, a folder or a Trash left out): they did not come across.
    ValuesWithoutRow { field: Field, items: u32 },
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
        "scrivener-metadata-unused-labels-skipped",
        "scrivener-metadata-values-without-row",
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
            MetadataNotice::UnusedLabelsSkipped { .. } => {
                "scrivener-metadata-unused-labels-skipped"
            }
            MetadataNotice::ValuesWithoutRow { .. } => "scrivener-metadata-values-without-row",
        }
    }

    /// Which group of the report it belongs to.
    pub fn group(&self) -> ReportGroup {
        match self {
            MetadataNotice::StatusCategoriesGuessed { .. }
            | MetadataNotice::DatesUnreadable { .. } => ReportGroup::CheckThese,
            MetadataNotice::LabelsBecameTags { .. }
            | MetadataNotice::StatusesBecameLadder { .. }
            | MetadataNotice::LadderSeeded { .. }
            | MetadataNotice::FieldRenamed { .. }
            | MetadataNotice::NoneValueCollapsed { .. }
            | MetadataNotice::UnnamedValue { .. }
            | MetadataNotice::TagNameDisambiguated { .. } => ReportGroup::BroughtAcrossDifferently,
            MetadataNotice::UnusedLabelsSkipped { .. }
            | MetadataNotice::ValuesWithoutRow { .. } => ReportGroup::LeftBehind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_is_listed_once_and_is_the_importers_own() {
        let all = [
            MetadataNotice::StatusCategoriesGuessed { rungs: Vec::new() },
            MetadataNotice::DatesUnreadable { items: 1 },
            MetadataNotice::LabelsBecameTags { tags: 1, rows: 1 },
            MetadataNotice::StatusesBecameLadder { rungs: 1, rows: 1 },
            MetadataNotice::LadderSeeded { rungs: 1 },
            MetadataNotice::FieldRenamed {
                field: Field::Label,
                title: "Time".into(),
            },
            MetadataNotice::NoneValueCollapsed {
                field: Field::Status,
                name: "N/A".into(),
                items: 1,
            },
            MetadataNotice::UnnamedValue {
                field: Field::Label,
                id: 3,
            },
            MetadataNotice::TagNameDisambiguated { names: Vec::new() },
            MetadataNotice::UnusedLabelsSkipped { names: Vec::new() },
            MetadataNotice::ValuesWithoutRow {
                field: Field::Label,
                items: 1,
            },
        ];
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
