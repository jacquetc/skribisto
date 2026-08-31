// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What an export sent out, as it appears in the `ExportWork` event's payload.
//!
//! The mirror of `import_management::events::ImportOrigins` (not linked: that
//! crate is not a dependency of this one, and should not become one), and deliberately
//! the same shape: that one says what came *home* and this one says what went
//! *out*. Until this existed the pair was half-built — a listener could see
//! every returning row and nothing about the copy it was returning from, which
//! is exactly the knowledge needed to tell a fresh return from a stale one.
//!
//! # Why the event payload rather than the result DTO
//!
//! `publish_export_work_event` has carried a `data: Option<String>` since it was
//! generated and has always been passed `None`; `ExportResultDto` is a
//! Qleany-generated file that already holds three unprotected hand-edits. Adding
//! a field there would mean a manifest change plus another correction to
//! re-apply after every regeneration, for a value no caller of the DTO wants —
//! the *result* is what the writer is shown, and this is bookkeeping.
//!
//! JSON rather than RON, like `ImportOrigins` and for the same reason: this is a
//! wire payload for whoever is listening, not a file in a writer's project.
//!
//! # What it deliberately does not carry
//!
//! No prose, and no digest that is not already in the exported file. Every
//! digest here was written into the document in plain sight, so a listener
//! learns nothing the recipient was not handed. See
//! [`skribisto_compiler::receipt`] for the same rule stated where it is enforced.

use serde::{Deserialize, Serialize};

/// One row as it left, flattened to strings for the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedRow {
    /// The row's durable `BinderItem.uid`, hyphenated.
    ///
    /// Never an `EntityId`: those are re-minted by every `load_work`, so an id
    /// cached from an event would name an unrelated row the next time the
    /// project opened. Same rule as `ImportedRow::item_uid`.
    pub item_uid: String,
    /// `round_trip::digest` of the prose this export wrote for that row — the
    /// twelve hex digits its bookmark also carries, and the baseline a returning
    /// copy is compared against.
    pub digest: String,
}

/// The whole payload, as it appears in `Event.data`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportOrigins {
    pub version: u32,
    /// Which project this export came out of — `Work.unique_id`, the durable one.
    ///
    /// Here rather than left for a listener to work out, for the reason
    /// `ImportOrigins` states: several `Work`s are open at once, the event's own
    /// `ids` say nothing about which, and an `EntityId` would have to be
    /// resolved through a store the listener may not reach from its thread.
    pub work_unique_id: String,
    /// The file that was written.
    pub output_path: String,
    /// The format's name, as `ExportFormat`'s `Debug` spells it (`Docx`, `Odt`).
    ///
    /// A string rather than the enum so this payload does not bind a listener to
    /// a type that lives in a generated file and gains variants over time.
    pub format: String,
    /// When the copy left, RFC 3339 UTC.
    pub exported_at: String,
    /// In export order, one per row that got a round-trip mark.
    ///
    /// Empty for every export written without marks — a plain PDF for a reader,
    /// say. An export nothing can be matched against has no receipt to keep, and
    /// an empty list says so honestly rather than implying rows went out
    /// unrecognisably.
    pub rows: Vec<ExportedRow>,
    /// The `Comment.uid`s carried into the file, hyphenated.
    pub comment_uids: Vec<String>,
}

/// The version this build writes.
pub const EXPORT_ORIGINS_VERSION: u32 = 1;

impl ExportOrigins {
    pub fn new(
        work_unique_id: impl Into<String>,
        output_path: impl Into<String>,
        format: impl Into<String>,
        exported_at: impl Into<String>,
        receipt: &skribisto_compiler::ExportReceipt,
    ) -> Self {
        Self {
            version: EXPORT_ORIGINS_VERSION,
            work_unique_id: work_unique_id.into(),
            output_path: output_path.into(),
            format: format.into(),
            exported_at: exported_at.into(),
            rows: receipt
                .rows
                .iter()
                .map(|r| ExportedRow {
                    item_uid: r.item_uid.to_string(),
                    digest: r.digest.clone(),
                })
                .collect(),
            comment_uids: receipt
                .comment_uids
                .iter()
                .map(uuid::Uuid::to_string)
                .collect(),
        }
    }

    /// Render for `Event.data`.
    ///
    /// `None` if it somehow will not serialise, because an event with no payload
    /// is a smaller problem than an export that reports a failure it did not
    /// have. Same judgement as `ImportOrigins::to_payload`.
    pub fn to_payload(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(text) => Some(text),
            Err(e) => {
                eprintln!("export_work: could not serialise the export receipt: {e}");
                None
            }
        }
    }

    /// Recover the payload a listener was handed.
    ///
    /// `None` for an absent, unparseable or foreign payload — a listener that
    /// cannot read an event should ignore it, not guess at it.
    pub fn from_payload(data: &str) -> Option<Self> {
        serde_json::from_str(data).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn receipt() -> skribisto_compiler::ExportReceipt {
        skribisto_compiler::ExportReceipt {
            rows: vec![
                skribisto_compiler::ExportedRow {
                    item_uid: uuid::Uuid::from_u128(1),
                    digest: "0123456789ab".into(),
                },
                skribisto_compiler::ExportedRow {
                    item_uid: uuid::Uuid::from_u128(2),
                    digest: "fe9876543210".into(),
                },
            ],
            comment_uids: vec![uuid::Uuid::from_u128(9)],
        }
    }

    #[test]
    fn a_payload_round_trips() {
        let origins = ExportOrigins::new(
            "the-lighthouse-uid",
            "/tmp/book.docx",
            "Docx",
            "2026-08-31T09:00:00Z",
            &receipt(),
        );
        let text = origins.to_payload().expect("serialises");
        assert_eq!(ExportOrigins::from_payload(&text), Some(origins));
    }

    /// The digests are the baseline a returning file is judged against, so they
    /// must survive the wire exactly — a truncated or re-cased digest silently
    /// reports every row as edited.
    #[test]
    fn the_digests_survive_the_round_trip_verbatim() {
        let origins = ExportOrigins::new(
            "uid",
            "/tmp/b.docx",
            "Docx",
            "2026-08-31T09:00:00Z",
            &receipt(),
        );
        let back = ExportOrigins::from_payload(&origins.to_payload().unwrap()).unwrap();
        assert_eq!(back.rows[0].digest, "0123456789ab");
        assert_eq!(back.rows[1].digest, "fe9876543210");
        assert_eq!(back.rows[0].item_uid, uuid::Uuid::from_u128(1).to_string());
    }

    #[test]
    fn an_export_with_no_marks_carries_an_empty_receipt() {
        let origins = ExportOrigins::new(
            "uid",
            "/tmp/b.pdf",
            "Pdf",
            "2026-08-31T09:00:00Z",
            &skribisto_compiler::ExportReceipt::default(),
        );
        assert!(origins.rows.is_empty());
        assert!(origins.comment_uids.is_empty());
        // Still serialisable — an empty receipt is a fact, not a failure.
        assert!(origins.to_payload().is_some());
    }

    #[test]
    fn a_foreign_payload_is_ignored_rather_than_guessed_at() {
        assert_eq!(ExportOrigins::from_payload(""), None);
        assert_eq!(ExportOrigins::from_payload("not json"), None);
        assert_eq!(ExportOrigins::from_payload("{}"), None);
    }
}
