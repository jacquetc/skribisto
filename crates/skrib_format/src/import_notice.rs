// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a project importer tells the writer about the conversion.
//!
//! The Plume Creator and Manuskript importers record everything they had to
//! drop, guess at or read imperfectly, and the writer reads that list after the
//! import. The importers run below the UI, with no translation catalogue, so a
//! notice is **data, not a sentence**: one variant per thing that can be said,
//! carrying only the values that differ (a title, a count, a file name). The UI
//! turns each variant into a sentence in the interface language.
//!
//! The type lives here, in the format crate, because it is the one crate both
//! importers and the UI already depend on; `import_management` carries notices
//! across its DTO as JSON (`serde`), and the UI reads them back with the same
//! derive.
//!
//! [`fmt::Display`](std::fmt::Display) is the English wording, for logs, tests and a
//! notice the UI cannot decode (one archived by an older build). It is not what
//! the writer reads in a translated interface.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Which of a Manuskript project's two storage modes was read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectContainer {
    /// A single `.msk` zip.
    SingleFile,
    /// A project folder.
    Folder,
}

/// The kind of text a row carried that could not be placed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextKind {
    Scene,
    Note,
    Synopsis,
    Title,
}

/// One thing an importer has to tell the writer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ImportNotice {
    // --- Plume Creator ------------------------------------------------------
    /// Rich texts nested past the format's depth ceiling, kept as plain text.
    FlattenedRichTexts { count: u64 },
    /// Scene badges that read as free text, kept as labels rather than tags.
    FreeTextBadges { count: u64 },
    /// A separator's own text, which the writing model has no place for.
    SeparatorTextDropped { name: String },
    /// A separator with no scene before it in its chapter.
    SeparatorWithoutScene { name: String },
    /// A chapter with no scenes, whose separators were therefore dropped.
    ChapterWithoutScenes { name: String },
    /// A rich text the converter refused.
    RichTextUnconverted { error: String },
    /// A link to an attendance (cast) entry that is missing or trashed.
    UnresolvedCrossLink { number: i64 },

    // --- Manuskript: which copy, which format -------------------------------
    /// Which copy of the project was read, and when it last changed.
    ReadCopy {
        container: ProjectContainer,
        day: Option<String>,
    },
    /// The project is in Manuskript's original 2016 format.
    OriginalFormat,
    /// A newer `.msk` sits beside the folder that was opened.
    NewerSingleFileBeside {
        sibling: String,
        sibling_day: String,
        folder_day: String,
    },

    // --- Manuskript: files ---------------------------------------------------
    /// The settings pickle, deliberately not read (CVE-2021-35196).
    PickleSkipped { member: String },
    /// A file of the project folder that the system would not let us read.
    FileUnreadable { member: String, error: String },
    /// A member that is not valid UTF-8, read with replacement characters.
    NotUtf8 { member: String },
    /// A member whose table could not be parsed.
    TableUnreadable { member: String, error: String },
    /// The outline is missing: there is no manuscript.
    OutlineMissing { member: String },
    /// The outline could not be parsed: no manuscript was imported.
    OutlineUnreadable { member: String, error: String },
    /// A label or status list whose first row is named, where Manuskript keeps
    /// its empty "none" entry.
    NamedNoneRow { member: String, name: String },
    /// A field this importer does not know.
    UnknownField { member: String, field: String },
    /// Label lines with no colour.
    ColourlessLabels { member: String, count: u64 },
    /// The settings JSON could not be read: no project language.
    SettingsUnreadable { member: String, error: String },
    /// The revisions file could not be read: no history.
    RevisionsUnreadable { member: String, error: String },
    /// Files that are not part of the Manuskript format. `shown` names at
    /// most a handful; `count` is all of them.
    ForeignFiles { count: u64, shown: Vec<String> },
    /// A file in the characters folder that is not a character file.
    NotACharacterFile { member: String },
    /// A file in the outline folder that is not a text item.
    NotATextItem { member: String },
    /// A metadata header with no blank line after it.
    MissingBlankLine { member: String },
    /// An outline folder with no metadata file, imported as a plain folder.
    FolderWithoutMetadata { member: String, metadata: String },
    /// Outline rows with no id of their own, given one.
    RowsGivenIds { count: u64 },
    /// The plots file could not be read: no plots.
    PlotsUnreadable { member: String, error: String },
    /// The world file could not be read: no world tree.
    WorldUnreadable { member: String, error: String },
    /// The world file is not OPML: no world tree.
    WorldNotOpml { member: String },

    // --- Manuskript: rows ----------------------------------------------------
    /// A row's text nested past the format's depth ceiling, kept as plain text.
    TextFlattened { title: String },
    /// A row's text the converter refused, kept as written.
    TextUnconverted { title: String, error: String },
    /// A row's text that could not be given a file name.
    TextWithoutFileName { title: String, text: TextKind },
    /// A row wearing a label index outside the label list.
    UnknownLabel { title: String, index: i64 },
    /// A row at a status index outside the status list.
    UnknownStatus { title: String, index: i64 },
    /// A row told through a character not in the cast.
    UnknownPointOfView { title: String },
    /// References in a row's text that could not be resolved.
    UnresolvedReferences { title: String, error: String },
    /// Rows that carried a custom icon.
    CustomIcons { count: u64 },
    /// Characters with a colour beside their name.
    CharacterColours { count: u64 },
    /// Earlier versions found, and how many were kept.
    VersionsKept { found: u64, kept: u64 },
    /// Earlier versions naming a row no longer in the project.
    VersionsUnplaced { count: u64 },
}

impl ProjectContainer {
    fn english(self) -> &'static str {
        match self {
            ProjectContainer::SingleFile => "single file",
            ProjectContainer::Folder => "folder",
        }
    }
}

impl TextKind {
    fn english(self) -> &'static str {
        match self {
            TextKind::Scene => "scene text",
            TextKind::Note => "note text",
            TextKind::Synopsis => "synopsis",
            TextKind::Title => "title",
        }
    }
}

/// `(s)` is how the English fallback hedges a plural it does not select on;
/// the translated sentences use real plural forms.
fn s(count: u64) -> &'static str {
    if count == 1 { "" } else { "s" }
}

impl fmt::Display for ImportNotice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImportNotice::*;
        match self {
            FlattenedRichTexts { count } => write!(
                f,
                "{count} rich text{} nested deeper than a Skribisto project can hold; their \
                 words were kept as plain text, without their formatting",
                if *count == 1 { " was" } else { "s were" }
            ),
            FreeTextBadges { count } => write!(
                f,
                "{count} distinct scene badge{} like free text rather than a status \
                 vocabulary — left as item labels instead of tags",
                if *count == 1 { " looks" } else { "s look" }
            ),
            SeparatorTextDropped { name } => write!(
                f,
                "separator '{name}' carried text that has no place in the writing model and \
                 was dropped"
            ),
            SeparatorWithoutScene { name } => write!(
                f,
                "separator '{name}' had no preceding scene in its chapter to attach to and \
                 was dropped"
            ),
            ChapterWithoutScenes { name } => write!(
                f,
                "chapter '{name}' has no scenes, so its separators were dropped"
            ),
            RichTextUnconverted { error } => {
                write!(f, "could not convert some rich text: {error}")
            }
            UnresolvedCrossLink { number } => write!(
                f,
                "cross-link to attendance #{number} could not be resolved (missing or trashed)"
            ),
            ReadCopy {
                container,
                day: Some(day),
            } => write!(
                f,
                "Read the {} copy of this project, last changed {day}.",
                container.english()
            ),
            ReadCopy {
                container,
                day: None,
            } => write!(f, "Read the {} copy of this project.", container.english()),
            OriginalFormat => f.write_str(
                "This project is in Manuskript's original 2016 format. It was read in full; \
                 nothing in it was changed.",
            ),
            NewerSingleFileBeside {
                sibling,
                sibling_day,
                folder_day,
            } => write!(
                f,
                "'{sibling}' is newer than this folder ({sibling_day} against {folder_day}). \
                 Manuskript leaves the old folder in place when a project moves to single-file \
                 mode, so this may be an abandoned copy; importing the .msk instead would read \
                 the newer one."
            ),
            PickleSkipped { member } => write!(
                f,
                "'{member}' was left unread. It is a Python pickle, and loading one from a \
                 project file was a remote-code-execution flaw in Manuskript itself \
                 (CVE-2021-35196); the settings it holds are display preferences, not writing."
            ),
            FileUnreadable { member, error } => write!(
                f,
                "'{member}' could not be read ({error}) and was skipped; everything else was \
                 imported."
            ),
            NotUtf8 { member } => write!(
                f,
                "'{member}' is not valid UTF-8; unreadable bytes were replaced rather than \
                 failing the import."
            ),
            TableUnreadable { member, error } => write!(
                f,
                "'{member}' could not be read ({error}); what it held was not imported."
            ),
            OutlineMissing { member } => write!(
                f,
                "'{member}' is missing, so this project has no manuscript to import."
            ),
            OutlineUnreadable { member, error } => write!(
                f,
                "'{member}' could not be read ({error}); no manuscript was imported."
            ),
            NamedNoneRow { member, name } => write!(
                f,
                "'{member}' opens with a named row where Manuskript keeps its empty \"none\" \
                 entry ('{name}'). It was left out, so the rows below it keep the numbers the \
                 manuscript cites."
            ),
            UnknownField { member, field } => write!(
                f,
                "'{member}' carries a field this importer does not know, '{field}'. It was not \
                 imported."
            ),
            ColourlessLabels { member, count } => write!(
                f,
                "'{member}' has {count} line{} with no colour. They were imported without one; \
                 Manuskript stops loading the project on such a line.",
                s(*count)
            ),
            SettingsUnreadable { member, error } => write!(
                f,
                "'{member}' is not readable JSON ({error}); the project's language was not \
                 imported."
            ),
            RevisionsUnreadable { member, error } => write!(
                f,
                "'{member}' could not be read ({error}); the project's history was not \
                 imported, but nothing else was affected."
            ),
            ForeignFiles { count, shown } => {
                let more = (*count).saturating_sub(shown.len() as u64);
                let tail = if more > 0 {
                    format!(" and {more} more")
                } else {
                    String::new()
                };
                write!(
                    f,
                    "This project holds {count} file{} that are not part of the Manuskript \
                     format and were not imported: {}{tail}.",
                    s(*count),
                    shown.join(", ")
                )
            }
            NotACharacterFile { member } => write!(
                f,
                "'{member}' is not a Manuskript character file and was left out."
            ),
            NotATextItem { member } => write!(
                f,
                "'{member}' is not a Manuskript text item and was left out of the outline."
            ),
            MissingBlankLine { member } => write!(
                f,
                "'{member}' ended without a blank line after its metadata. Its last field was \
                 kept; Manuskript loses it."
            ),
            FolderWithoutMetadata { member, metadata } => write!(
                f,
                "'{member}' has no {metadata}, so it was imported as a plain folder named after \
                 the directory. Manuskript skips such a directory and everything inside it."
            ),
            RowsGivenIds { count } => write!(
                f,
                "{count} outline {} no ID of its own and was given one. Manuskript drops such a \
                 row, or stops loading the project entirely when it is a folder.",
                if *count == 1 { "row had" } else { "rows had" }
            ),
            PlotsUnreadable { member, error } => write!(
                f,
                "'{member}' could not be read ({error}); the plots were not imported."
            ),
            WorldUnreadable { member, error } => write!(
                f,
                "'{member}' could not be read ({error}); the world tree was not imported."
            ),
            WorldNotOpml { member } => write!(
                f,
                "'{member}' is not an OPML document; the world tree was not imported."
            ),
            TextFlattened { title } => write!(
                f,
                "The text of '{title}' is nested deeper than a Skribisto project can hold; its \
                 words were kept as plain text, without their formatting."
            ),
            TextUnconverted { title, error } => write!(
                f,
                "The text of '{title}' could not be converted ({error}); it was kept exactly as \
                 it was written."
            ),
            TextWithoutFileName { title, text } => write!(
                f,
                "'{title}' has {} content that could not be given a file name; it was not \
                 imported.",
                text.english()
            ),
            UnknownLabel { title, index } => write!(
                f,
                "'{title}' wears label {index}, which is not in this project's label list. It \
                 arrived without one."
            ),
            UnknownStatus { title, index } => write!(
                f,
                "'{title}' is at status {index}, which is not in this project's status list. It \
                 arrived without one."
            ),
            UnknownPointOfView { title } => write!(
                f,
                "'{title}' is told through a character who is not in this project's cast. Its \
                 point of view was not set."
            ),
            UnresolvedReferences { title, error } => write!(
                f,
                "The references in the text of '{title}' could not be resolved ({error}); the \
                 text was kept as it was converted, markers included."
            ),
            CustomIcons { count } => write!(
                f,
                "{count} row{} carried a custom icon. Skribisto gives a row its icon from what \
                 the row is, so the choice was not imported; nothing else about those rows \
                 changed.",
                s(*count)
            ),
            CharacterColours { count } => write!(
                f,
                "{count} character{} had a color beside their name. Skribisto colors tags \
                 rather than rows, so the colors were not imported; the characters themselves \
                 all arrived.",
                s(*count)
            ),
            VersionsKept { found, kept } => write!(
                f,
                "This project recorded {found} earlier version{}; {kept} {} kept, thinned the \
                 same way Skribisto thins its own history. They are in the Versions panel.",
                s(*found),
                if *kept == 1 { "was" } else { "were" }
            ),
            VersionsUnplaced { count } => write!(
                f,
                "{count} earlier version{} named a row that is no longer in the project and \
                 could not be placed.",
                s(*count)
            ),
        }
    }
}

impl ImportNotice {
    /// The notice as it travels through a DTO: JSON, decoded by the UI with
    /// [`ImportNotice::decode`].
    pub fn encode(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| self.to_string())
    }

    /// Read back a notice [`ImportNotice::encode`] wrote. `None` for anything
    /// else, which the caller shows as it is.
    pub fn decode(text: &str) -> Option<Self> {
        if !text.starts_with('{') {
            return None;
        }
        serde_json::from_str(text).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_notice_survives_its_trip_through_the_dto() {
        let notices = [
            ImportNotice::UnknownLabel {
                title: "Chapter 'One'".into(),
                index: 4,
            },
            ImportNotice::ReadCopy {
                container: ProjectContainer::Folder,
                day: Some("2026-10-05".into()),
            },
            ImportNotice::OriginalFormat,
            ImportNotice::ForeignFiles {
                count: 9,
                shown: vec!["a.txt".into(), "b \"c\".png".into()],
            },
        ];
        for notice in notices {
            assert_eq!(ImportNotice::decode(&notice.encode()), Some(notice));
        }
    }

    #[test]
    fn plain_text_is_not_mistaken_for_a_notice() {
        assert_eq!(ImportNotice::decode("could not convert"), None);
        assert_eq!(ImportNotice::decode("{not json"), None);
    }

    #[test]
    fn the_english_fallback_agrees_with_its_count() {
        assert!(
            ImportNotice::CustomIcons { count: 1 }
                .to_string()
                .starts_with("1 row carried")
        );
        assert!(
            ImportNotice::CustomIcons { count: 3 }
                .to_string()
                .starts_with("3 rows carried")
        );
        let more = ImportNotice::ForeignFiles {
            count: 7,
            shown: vec!["a".into(), "b".into()],
        }
        .to_string();
        assert!(more.ends_with("a, b and 5 more."), "{more}");
    }
}
