// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What an export sent out — kept, because the file itself cannot say.
//!
//! # Why this exists
//!
//! A `.docx`/`.odt` export written with round-trip marks carries, per row, a
//! bookmark naming that row's `BinderItem.uid` and a digest of its prose *as
//! exported* ([`skribisto_model::round_trip`]). That is what lets a file coming
//! back from an editor be recognised, and it deliberately stores nothing on this
//! side: the baseline travels in the file, so a three-way comparison needs no
//! bookkeeping.
//!
//! That design answers "what did this row say when it left?" — but only for a
//! file that comes back. It cannot answer the questions that arise once more
//! than one copy is out at a time, because the mark names are **one-way**: a
//! `uid_tag` is a 64-bit hash of the uuid and the digest is a 48-bit truncation,
//! so nothing can be read back out of a name.
//!
//! * *Which rows went out, and when?* — so a writer can be told "three copies
//!   out since 14 August, four chapters have changed since".
//! * *Has this row already been merged from an earlier return?* —
//!   [`reconcile::RowStatus::YouEdited`](skribisto_model::reconcile::RowStatus)
//!   means "you changed it and the reader did not", and after the first merge
//!   that is a lie: the change came from reader one, not from the writer.
//!
//! Neither is answerable from the file. Both are answerable from a receipt.
//!
//! # What it deliberately does not hold
//!
//! **No prose, and no hash of prose beyond the digest already written into the
//! file.** The digest is public — it is in the exported document, in plain sight
//! — so keeping it here discloses nothing that was not already handed to the
//! recipient. A receipt that stored the text itself would be a second copy of
//! the manuscript with a different retention story, which is exactly the shape
//! that turns a convenience into a liability.

/// One row as it left: its durable identity, and what it said at the time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedRow {
    /// `BinderItem.uid` — durable across `load_work`, unlike an `EntityId`.
    pub item_uid: uuid::Uuid,
    /// [`round_trip::digest`](skribisto_model::round_trip::digest) of the row's
    /// prose as this export wrote it. The same twelve hex digits the file's own
    /// bookmark carries, and the baseline a returning copy is compared against.
    pub digest: String,
}

/// Everything one export sent out that a later return may refer back to.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExportReceipt {
    /// In export order, one per row that actually got a mark.
    ///
    /// Shorter than the export's row count whenever a row had a nil uid or its
    /// prose could not be located in the compiled text — precisely the rows a
    /// returning file could not have been matched on either, so the receipt and
    /// the file agree about what is recognisable.
    pub rows: Vec<ExportedRow>,
    /// The `Comment.uid`s written into the file, in the order their marks were
    /// emitted.
    ///
    /// Not the same as the count of comments written: a comment with a nil uid
    /// is placed in the document but gets no mark, so it can never be matched on
    /// the way home and is not listed here.
    pub comment_uids: Vec<uuid::Uuid>,
}

impl ExportReceipt {
    /// Whether this export recorded anything a return could be matched against.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.comment_uids.is_empty()
    }
}
