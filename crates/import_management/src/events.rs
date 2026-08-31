// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What an import did, and where each row's words came from — the payload on
//! `Origin::ImportManagement(ApplyDocumentImport)`.
//!
//! ## Why the event carries it at all
//!
//! The moment an import commits is the **only** moment the answer exists. The
//! plan knows which file each row was read out of; the store never will, because
//! a `BinderItem` records what it is and not how it arrived. Anything that wants
//! to say *"these forty scenes arrived from `novel-draft-3.docx` on 14 March"*
//! has to hear it here or not at all.
//!
//! That matters most for the case it exists to stop being misread. A 90,000-word
//! import produces the same shape as a manuscript written in an afternoon, and
//! the difference between a neutral record and an accidental accusation is
//! whether anything downstream can say which of the two it was.
//!
//! ## What is in it, and what is deliberately not
//!
//! A **file name**, never a path. A writer's directory tree says where they keep
//! their work and sometimes who they are; the file name says which document a
//! row came out of, which is all the question needs. The narrowing happens at the
//! UI call site that builds the apply DTO, so the full path cannot reach here by
//! construction rather than by a filter somebody has to remember.
//!
//! A **character count**, and no word count. The count is a pure function of the
//! prose already in the DTO, so nothing extra crosses the seam for it. A word
//! count would be a second answer to a question the manuscript's own counter
//! already answers, and two counters disagreeing about one book is worse than
//! either being wrong.
//!
//! No prose, no titles, no comment bodies.
//!
//! ## Nothing here can affect the import
//!
//! The event is published **after** `commit()`, and publishing is a send on a
//! channel drained by another thread. A subscriber that panics, that cannot parse
//! this, or that is not there at all changes nothing about what was written.

use serde::{Deserialize, Serialize};

/// What the import did to one row.
///
/// The three are genuinely different provenance, and collapsing them would be
/// the kind of small dishonesty this whole payload exists to avoid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportAction {
    /// A row that did not exist before. Its words arrived with this file.
    Created,
    /// A row the project already had, whose prose this file replaced. Its words
    /// arrived with this file too, over words that were there before.
    ProseReplaced,
    /// A row the project already had, which this file brought remarks home to
    /// and **did not touch a word of**.
    ///
    /// The case the returning-file feature exists for, and the one that must
    /// never be reported as text arriving: nothing did.
    CommentsOnly,
}

/// One row, and where its words came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedRow {
    /// The row's durable `BinderItem.uid`, hyphenated.
    ///
    /// Never an `EntityId`: those are re-minted by every `load_work`, so an id
    /// cached from an event would name an unrelated row the next time the
    /// project opened.
    pub item_uid: String,
    pub action: ImportAction,
    /// The source document's file name, extension included. Empty for a row the
    /// writer typed into the review step, which came from no file.
    pub source_file_name: String,
    /// blake3 hex of that file's own bytes. Empty alongside an empty name.
    pub source_file_digest: String,
    /// Characters of prose this import wrote into the row. `0` for
    /// [`ImportAction::CommentsOnly`], which wrote none.
    pub char_count: u64,
}

/// The whole payload, as it appears in `Event.data`.
///
/// A struct rather than a bare list so a field can be added without every reader
/// having to change shape, and versioned for the same reason one level up: a
/// reader that does not recognise [`Self::version`] should say so rather than
/// guess.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportOrigins {
    pub version: u32,
    /// Which project this import landed in — `Work.unique_id`, the durable one.
    ///
    /// Here rather than left for a listener to work out, and that is the
    /// difference between a payload anyone can use and one only the application
    /// can. Several `Work`s are open at once; the event's own `ids` are the
    /// created rows and say nothing about whose binder they joined; and an
    /// `EntityId` would have to be resolved through a store the listener may not
    /// reach from the thread it is on. Everything needed to record this import is
    /// in the payload, or the payload is not finished.
    ///
    /// Empty only if the Work could not be read, which would mean the import had
    /// nothing to write into either.
    pub work_unique_id: String,
    pub rows: Vec<ImportedRow>,
}

/// The version this build writes.
pub const IMPORT_ORIGINS_VERSION: u32 = 1;

impl ImportOrigins {
    pub fn new(work_unique_id: impl Into<String>, rows: Vec<ImportedRow>) -> Self {
        Self {
            version: IMPORT_ORIGINS_VERSION,
            work_unique_id: work_unique_id.into(),
            rows,
        }
    }

    /// Render for `Event.data`.
    ///
    /// JSON rather than RON, unlike everything this workspace writes into a
    /// writer's project: this is a wire payload for whoever is listening, not a
    /// file in a bundle, and `serde_json` is already here. Returns `None` if it
    /// somehow will not serialise, because an event with no payload is a smaller
    /// problem than an import that reports an error it did not have.
    pub fn to_payload(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(text) => Some(text),
            Err(e) => {
                eprintln!("skribisto: could not describe the import's origins: {e}");
                None
            }
        }
    }

    /// Read one back. `None` for anything that is not this payload.
    pub fn from_payload(text: &str) -> Option<Self> {
        serde_json::from_str(text).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ImportOrigins {
        ImportOrigins::new(
            "the-lighthouse-uid",
            vec![
                ImportedRow {
                    item_uid: "8f14e45f-ceea-467a-9c05-1b4e0d2f9a11".into(),
                    action: ImportAction::Created,
                    source_file_name: "novel-draft-3.docx".into(),
                    source_file_digest: "abc123".into(),
                    char_count: 4200,
                },
                ImportedRow {
                    item_uid: "b3d9a1c2-0000-4000-8000-000000000002".into(),
                    action: ImportAction::CommentsOnly,
                    source_file_name: "chapter-7-editor.docx".into(),
                    source_file_digest: "def456".into(),
                    char_count: 0,
                },
            ],
        )
    }

    #[test]
    fn a_payload_round_trips() {
        let origins = sample();
        let text = origins.to_payload().expect("serialises");
        assert_eq!(ImportOrigins::from_payload(&text), Some(origins));
    }

    /// The payload is read by code outside this workspace, so its field names are
    /// as much a contract as any id. Pinned literally rather than by round trip,
    /// which would pass just as happily if every name changed at once.
    #[test]
    fn the_payload_looks_exactly_like_this() {
        let text = sample().to_payload().unwrap();
        assert_eq!(
            text,
            r#"{"version":1,"work_unique_id":"the-lighthouse-uid","rows":[{"item_uid":"8f14e45f-ceea-467a-9c05-1b4e0d2f9a11","action":"created","source_file_name":"novel-draft-3.docx","source_file_digest":"abc123","char_count":4200},{"item_uid":"b3d9a1c2-0000-4000-8000-000000000002","action":"comments_only","source_file_name":"chapter-7-editor.docx","source_file_digest":"def456","char_count":0}]}"#
        );
    }

    /// **A returning file that touched no prose must not look like text
    /// arriving.** It is the whole point of the returning-file feature that a
    /// writer can bring an editor's remarks home without a word of their
    /// manuscript changing, and a record that counted those rows as imported
    /// would be describing something that did not happen.
    #[test]
    fn a_comments_only_row_reports_no_characters() {
        let origins = sample();
        let comments_only = &origins.rows[1];
        assert_eq!(comments_only.action, ImportAction::CommentsOnly);
        assert_eq!(comments_only.char_count, 0);
    }

    #[test]
    fn anything_that_is_not_this_payload_reads_as_none() {
        assert_eq!(ImportOrigins::from_payload(""), None);
        assert_eq!(ImportOrigins::from_payload("not json"), None);
        assert_eq!(ImportOrigins::from_payload("{}"), None);
        assert_eq!(ImportOrigins::from_payload("[1,2,3]"), None);
    }

    /// A payload from a later build reads back with its version intact, so a
    /// reader can say "I do not know this one" instead of guessing at fields it
    /// does not recognise.
    #[test]
    fn the_version_survives_the_round_trip() {
        let text = sample().to_payload().unwrap();
        let back = ImportOrigins::from_payload(&text).unwrap();
        assert_eq!(back.version, IMPORT_ORIGINS_VERSION);
    }
}
