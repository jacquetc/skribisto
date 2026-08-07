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

use std::collections::BTreeMap;

use skribisto_model::scene_break::SceneBreakTier;

use crate::diagnostics::ImportDiagnostic;

/// One structural unit of a scanned document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceBlock {
    /// A heading and its depth as the source expressed it. Depth is *as written*
    /// — normalising a gap (an `h1` followed by an `h4`) is the pipeline's job,
    /// not a scanner's, so that every format gets the same treatment.
    Heading { level: u8, text: String },
    /// A run of prose, already converted to Djot. Conversion is per-format and
    /// therefore a scanner's responsibility: Markdown goes through
    /// `skrib_format::markdown_to_djot`, and a future DOCX scanner will have to
    /// build Djot some other way, since `text-document` has no DOCX reader.
    Prose { djot: String },
    /// A scene break the scanner recognised. Carrying the tier here — rather
    /// than the raw glyph — is what keeps `skribisto_model`'s vocabulary the one
    /// authority on what counts as a break.
    SceneBreak { tier: SceneBreakTier },
}

impl SourceBlock {
    /// True when this block carries no text worth importing. A scanner may emit
    /// an empty prose run at a boundary; the planner drops them rather than
    /// creating a row with nothing in it.
    pub fn is_empty(&self) -> bool {
        match self {
            SourceBlock::Heading { text, .. } => text.trim().is_empty(),
            SourceBlock::Prose { djot } => djot.trim().is_empty(),
            SourceBlock::SceneBreak { .. } => false,
        }
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
