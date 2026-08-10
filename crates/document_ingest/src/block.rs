// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The neutral block model every scanner produces and the whole pipeline
//! consumes.
//!
//! Three variants, and the discipline that keeps it at three: **every
//! format-specific structural heuristic is resolved inside its scanner and never
//! crosses into this enum.** A DOCX scanner deciding that "Heading 1" (or
//! "Titre 1", or the style id `Heading1`) means level 1 is its own business; the
//! pipeline only ever sees a `u8`. A page break used as evidence that a
//! paragraph opens a chapter is likewise private — and a page break the writer
//! wants *kept* already has a lossless carrier in Djot's `{page_break_before=true}`
//! attribute line, inside a [`SourceBlock::Prose`].
//!
//! One temptation deliberately refused: alignment. Screenplay importers detect a
//! scene break from a centred paragraph, and `SourceBlock` could carry that hint
//! — but Skribisto's scene-break vocabulary is content-based by design
//! (`skribisto_model::scene_break`: *"fixed and preset-independent"*), matching
//! text and never layout. A format that could detect breaks Markdown structurally
//! cannot would be a cross-format inconsistency, not a feature.
//!
//! ## Annotations ride a side channel, and that is why the enum is still three
//!
//! An editor's comment annotates a *span inside* a block, so it can never be a
//! block. [`SourceDocument::annotations`] is the document-level side channel this
//! module reserved by name when there was nothing to put in it; the `.docx` and
//! `.odt` scanners fill it, Markdown leaves it empty because CommonMark has no
//! comment syntax. That is not the cross-format inconsistency the rule above
//! forbids — that rule is about *inventing structure from layout*, not about one
//! format genuinely carrying data another does not.

use std::collections::BTreeMap;

use skribisto_model::comment_anchor::Anchor;
use skribisto_model::scene_break::SceneBreakTier;

use crate::diagnostics::ImportDiagnostic;

/// One structural unit of a scanned document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceBlock {
    /// A heading and its depth as the source expressed it. Depth is *as written*
    /// — normalising a gap (an `h1` followed by an `h4`) is the pipeline's job,
    /// not a scanner's, so that every format gets the same treatment.
    Heading { level: u8, text: String },
    /// A run of prose, already converted to Djot, alongside the plain text that
    /// Djot renders to.
    ///
    /// Conversion is per-format and therefore a scanner's responsibility: Markdown
    /// goes through `skrib_format::markdown_to_djot_and_text`, and the OOXML/ODF
    /// scanners build HTML from their styled runs and go through
    /// `skrib_format::html_to_djot_and_text`, since `text-document` has no DOCX or
    /// ODT reader.
    ///
    /// **`text` is not a convenience copy.** It is the coordinate space an
    /// annotation's quote and *hint* offsets are measured in, and it comes from the
    /// *same parse* as `djot` so the two cannot describe different content. Deriving
    /// it later would mean parsing the Djot a second time and hoping the answer
    /// matched. It is the plain-text *export* (no `U+FFFC` table anchors), which is
    /// fine for a hint: before anything is stored, `plan::anchor_comments` proves
    /// every quote against the row's **addressable** text
    /// (`skrib_format::djot_plain_text`, anchors counted) and re-captures the anchor
    /// there, so a hint that drifts past a table is corrected by the quote match.
    Prose { djot: String, text: String },
    /// A scene break the scanner recognised. Carrying the tier here — rather
    /// than the raw glyph — is what keeps `skribisto_model`'s vocabulary the one
    /// authority on what counts as a break.
    SceneBreak { tier: SceneBreakTier },
}

impl SourceBlock {
    /// A prose block from Djot and the plain text of the same conversion.
    pub fn prose(djot: impl Into<String>, text: impl Into<String>) -> Self {
        SourceBlock::Prose {
            djot: djot.into(),
            text: text.into(),
        }
    }

    /// True when this block carries no text worth importing. A scanner may emit
    /// an empty prose run at a boundary; the planner drops them rather than
    /// creating a row with nothing in it.
    pub fn is_empty(&self) -> bool {
        match self {
            SourceBlock::Heading { text, .. } => text.trim().is_empty(),
            SourceBlock::Prose { djot, .. } => djot.trim().is_empty(),
            SourceBlock::SceneBreak { .. } => false,
        }
    }

    /// This block's contribution to the plain text of the row it lands in.
    ///
    /// A scene break contributes the glyph a writer actually sees, not the escaped
    /// Djot the row stores — `\* \* \*` reads back as `* * *`, and an annotation
    /// after a break would be off by the difference if this said otherwise.
    pub fn plain_text(&self) -> &str {
        match self {
            SourceBlock::Heading { text, .. } => text,
            SourceBlock::Prose { text, .. } => text,
            SourceBlock::SceneBreak { tier } => {
                skribisto_model::scene_break::canonical_plain(*tier)
            }
        }
    }
}

/// What an annotation is attached to — the same three shapes `CommentAnchorKind`
/// offers, because these become `Comment` rows unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationKind {
    /// A character range inside the prose.
    Range,
    /// A whole paragraph. Both Word and LibreOffice allow a comment with no range
    /// — the marker simply sits in a paragraph — and this is what that becomes.
    Paragraph,
    /// The row as a whole, carrying no text anchor. Where a comment on something
    /// that is not prose lands: a heading becomes a row's *title*, and a title has
    /// no `Content` for a quote to point into.
    Document,
}

/// One reply inside an annotation's thread. Flat, never nested — which is both
/// what Skribisto's comment card is and what OOXML actually stores (`w15:paraIdParent`
/// always names the thread root, never a peer reply).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceAnnotationReply {
    /// The identity this reply carried in the source file, when the file is one
    /// Skribisto itself exported — the DOCX writer's `skrb:uid` on `<w:comment>`
    /// (M-S1) or the ODT writer's `skrb:uid` on `<office:annotation>` (M-S2).
    /// `None` when the scanner found no such attribute: a reply an editor typed
    /// straight into Word or LibreOffice, never one Skribisto minted. This is
    /// exactly what lets a re-import recognise "the same reply come back" instead
    /// of creating a second one — see `apply_document_import_uc`'s own doc for how
    /// the distinction is used.
    pub uid: Option<uuid::Uuid>,
    pub author: String,
    /// Read from `w:initials` on DOCX. Always empty on ODT — ODF's
    /// `office:annotation` has no carrier for it (a documented format ceiling, see
    /// `text-document`'s `export_odt_uc` module doc, not a bug to fix here). Empty
    /// means "none", the same convention [`SourceAnnotation::author`]'s sibling
    /// fields already use — never `Option`, so an empty string from a source that
    /// genuinely has no initials cannot be confused with an absent read.
    pub author_initials: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub body: String,
}

/// An editor's comment, recovered from a format that carries them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAnnotation {
    /// Index into [`SourceDocument::blocks`] of the block the annotated text is in.
    pub block_index: usize,
    pub kind: AnnotationKind,
    /// The selector, in the coordinate space of that block's own
    /// [`SourceBlock::plain_text`].
    ///
    /// Block-relative rather than document-absolute because a block is the unit
    /// that survives into a row: the planner knows which blocks it concatenated and
    /// in what order, so it can rebase these exactly — whereas a document-absolute
    /// offset would have to be corrected for every block the row did *not* take.
    pub anchor: Anchor,
    /// See [`SourceAnnotationReply::uid`] — the same recognition mechanism, for the
    /// thread's opening comment rather than one of its replies.
    pub uid: Option<uuid::Uuid>,
    /// The identity a **round-trip mark** carries for this comment: a bookmark pair named
    /// `skrb_c<tag>` bracketing exactly the characters the comment covers.
    ///
    /// The carrier that actually survives. `uid` above only ever arrives on a file no editor
    /// has saved — Word and LibreOffice both delete the private attribute it comes from,
    /// measured against a real returning file. A bookmark is first-class in both formats and
    /// comes back untouched, so on any realistic round trip this is what identifies the
    /// comment and `uid` is `None`.
    ///
    /// **A tag, not a uid**: [`skribisto_model::round_trip::uid_tag`] is a one-way hash, so
    /// this cannot be turned back into a `Uuid`. It is a *lookup key* — the importer computes
    /// the tag of each comment the project already holds and matches. That is by design: the
    /// full 36-character uuid does not fit in a bookmark name beside anything else, and Word
    /// caps a name at 40 characters.
    pub uid_tag: Option<String>,
    pub author: String,
    /// See [`SourceAnnotationReply::author_initials`].
    pub author_initials: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub body: String,
    pub resolved: bool,
    pub replies: Vec<SourceAnnotationReply>,
}

/// One row's identity, recovered from a round-trip mark the export wrote.
///
/// A zero-length bookmark named `skrb_r<tag>_<digest>`, sitting at the first character of the
/// row's prose. It answers the only question a returning file cannot otherwise answer: *which
/// of the writer's rows is this passage?*
///
/// The alternative — matching by heading text and position — is a guess, and a bad one on a
/// real manuscript. In the file this was first read back from, the commented word ("Québec")
/// occurs sixty-one times and half the chapters share a title shape; the mark is what makes
/// the answer exact instead of plausible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRowMark {
    /// Index into [`SourceDocument::blocks`] of the block the mark sits in.
    pub block_index: usize,
    /// A lookup key for `BinderItem.uid` — see [`SourceAnnotation::uid_tag`] for why this is
    /// a hash rather than the uid itself.
    pub uid_tag: String,
    /// The digest of this row's prose **as it was exported**, from
    /// [`skribisto_model::round_trip::digest`]. Compared against the digest of the prose in
    /// this file and of the prose the project holds now, it says which side changed the row —
    /// the baseline for a three-way merge, travelling in the file rather than stored anywhere.
    pub digest: String,
}

impl SourceAnnotation {
    /// Total turns in this thread — the opening comment plus its replies. What the
    /// review tree counts, so the writer knows what is coming.
    pub fn turns(&self) -> usize {
        1 + self.replies.len()
    }
}

/// Document-level metadata a scanner recovered — front matter for Markdown,
/// document properties for a future DOCX or ODT.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceMetadata {
    /// An explicit title, which outranks both the document's own first heading
    /// and its file name.
    pub title: Option<String>,
    pub author: Option<String>,
    /// Explicit cross-document ordering (a front-matter `order:` key, or a
    /// document property). `None` falls back to a natural sort of the file name.
    pub order_hint: Option<i64>,
    /// Everything else the scanner found. Kept rather than dropped so a key the
    /// pipeline does not interpret can still be reported to the writer instead
    /// of vanishing.
    pub raw: BTreeMap<String, String>,
}

/// One scanned document: where it came from, what it said about itself, and the
/// blocks it decomposed into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    /// What to show the writer for this document — a cleaned-up file stem.
    pub display_name: String,
    /// Provenance, verbatim: the path it was read from. Used for diagnostics and
    /// for the deterministic ordering of a multi-file import.
    pub origin: String,
    pub metadata: SourceMetadata,
    pub blocks: Vec<SourceBlock>,
    /// Editors' comments the scanner recovered, in document order. Empty for a
    /// format that has no comments — see the module note above.
    pub annotations: Vec<SourceAnnotation>,
    /// Round-trip row marks the scanner recovered, in document order. Empty unless this file
    /// is one Skribisto exported — which is exactly the case where the import is a *return*
    /// rather than a first arrival, and the only case where matching onto existing rows is
    /// even a question.
    pub row_marks: Vec<SourceRowMark>,
    /// Everything the scanner wants the writer to know about this document.
    /// A scanner reports rather than refuses wherever it can: one unreadable
    /// file in sixty must not abort the other fifty-nine.
    pub diagnostics: Vec<ImportDiagnostic>,
}

impl SourceDocument {
    /// An empty document from `origin`, for a scanner to fill.
    pub fn new(display_name: impl Into<String>, origin: impl Into<String>) -> Self {
        SourceDocument {
            display_name: display_name.into(),
            origin: origin.into(),
            metadata: SourceMetadata::default(),
            blocks: Vec::new(),
            annotations: Vec::new(),
            row_marks: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// The title this document claims: explicit metadata first, then its own
    /// first heading, then the display name.
    ///
    /// Front matter outranks the heading because it is the more deliberate
    /// signal — a writer who typed `title:` meant it — and the file name comes
    /// last because it is the one an operating system constrained.
    pub fn effective_title(&self) -> &str {
        if let Some(t) = self.metadata.title.as_deref().map(str::trim)
            && !t.is_empty()
        {
            return t;
        }
        for block in &self.blocks {
            if let SourceBlock::Heading { text, .. } = block
                && !text.trim().is_empty()
            {
                return text.trim();
            }
        }
        &self.display_name
    }

    /// True when nothing in the document would become a row.
    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(SourceBlock::is_empty)
    }
}
