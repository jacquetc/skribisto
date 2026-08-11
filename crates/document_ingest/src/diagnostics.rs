// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the importer tells the writer, and how loudly.
//!
//! The loudest finding in the prior-art survey behind this feature was that
//! manuscript import fails *silently*: Scrivener's split deleting the word it
//! split on, Joplin dropping every relative-path image for years, md2nw quietly
//! starting an unnamed scene whose body begins with a literal `#### `. The
//! response is not to fail harder — it is to make every lossy decision nameable,
//! attributable to a file or a row, and visible before anything is committed.
//!
//! So a diagnostic is data, never a log line: it carries the source it belongs
//! to, it is translated at the UI boundary rather than here, and it survives into
//! the review step where the writer can act on it.

use std::fmt;

use skribisto_model::CreateType;

/// How much a diagnostic should interrupt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// Worth knowing, nothing was lost. "This file had no headings."
    Info,
    /// Something was lost or changed, and the import continues. This is the
    /// common case, and the one the review step exists to surface.
    Warning,
    /// This source contributed nothing. The *other* sources still import — a
    /// batch is never abandoned over one bad file.
    Error,
}

/// Something the importer wants the writer to know.
///
/// Variants carry their own data rather than a pre-built sentence so the UI can
/// translate them (house rule: user-visible strings go through Fluent, data
/// stays literal) and so the review tree can attach one to the right row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportDiagnostic {
    // ── file-level: about a whole source ────────────────────────────────────
    /// The file could not be read at all.
    FileUnreadable { path: String, reason: String },
    /// The file is not valid UTF-8 and had no byte-order mark to explain itself.
    /// It was decoded anyway, lossily — `replacements` counts the characters that
    /// did not survive, so the writer can judge whether to re-save and retry.
    LossyDecode { path: String, replacements: usize },
    /// Decoded from a non-UTF-8 encoding its byte-order mark declared.
    DecodedFromBom {
        path: String,
        encoding: &'static str,
    },
    /// The file held no text.
    EmptyFile { path: String },
    /// No heading anywhere, so the whole file becomes a single row. Not a
    /// failure — a folder of one-scene-per-file imports exactly this way — but
    /// worth saying, because it is also what a 120,000-word manuscript that
    /// nobody marked up looks like.
    NoHeadings { path: String },
    /// A file was offered whose extension no scanner claims.
    UnsupportedFormat { path: String, extension: String },

    // ── content-level: about something inside a source ──────────────────────
    /// Front matter was found but is not flat `key: value` scalars. The keys that
    /// did parse were kept; this names what was skipped.
    FrontMatterNotFlat { path: String, key: String },
    /// Footnotes were found. `text-document`'s Markdown reader has
    /// `ENABLE_FOOTNOTES` off, so a `[^1]` reference degrades to literal text
    /// rather than becoming a footnote — the writer would otherwise discover
    /// this by reading their own book.
    FootnotesDegraded { path: String, count: usize },
    /// Raw HTML was found and dropped. Djot has no general HTML passthrough and
    /// the document model does not carry one.
    RawHtmlDropped { path: String, count: usize },
    /// A thematic break (`***`, `---`, `* * *`) was found *nested* inside
    /// another construct — a block quote, a list item — rather than as its own
    /// top-level block. Only a top-level break is recognised as a scene-break
    /// marker; a nested one falls through to `text-document`'s Markdown reader,
    /// which has no arm for it and drops it outright, the same silent loss the
    /// top-level case exists to prevent.
    NestedBreakDropped { path: String, count: usize },
    /// An image reference was found. Asset ingestion is not wired from this
    /// layer, so the reference survives as text but the file is not copied in.
    ImageNotIngested { path: String, target: String },
    /// The document was mid-revision: tracked insertions were accepted and tracked
    /// deletions dropped, which is what "the final text" means — but a writer
    /// handed a file with somebody's unaccepted edits in it should be told, not
    /// left to notice later that a sentence they remember rejecting is in their
    /// manuscript.
    TrackedChangesFlattened { path: String, count: usize },
    /// A text box, shape or frame was found. Its text is not part of the document's
    /// flow, so where it belongs in a linear manuscript is genuinely unanswerable —
    /// it is named rather than guessed at.
    TextBoxDropped { path: String, count: usize },
    /// An embedded object (a chart, an equation, an OLE object) was found. There is
    /// nothing in the prose model that could hold it.
    EmbeddedObjectDropped { path: String, count: usize },
    /// A field — a page number, a cross-reference, a table of contents, a date —
    /// was replaced by the text it was last showing. That text is right today and
    /// will not update again.
    FieldFlattened { path: String, count: usize },
    /// A paragraph was styled as a heading but named no outline level this scanner
    /// could read, so it was imported as prose. Names it rather than guessing a
    /// depth, because a wrong depth silently reshapes the book.
    UnknownStyleLevel { path: String, style: String },
    /// A comment's quoted text was not found in the converted prose, so it was
    /// attached to its row as a whole rather than to a span. The comment is kept —
    /// it is the writer's, and the most valuable thing an editor sends back.
    CommentUnanchored { path: String, quote: String },
    /// Replies could not be recovered as replies and became comments of their own.
    /// ODF has no standardised threading, so this is the honest outcome for a file
    /// whose producer did not use one this scanner recognises.
    CommentRepliesFlattened { path: String, count: usize },
    /// Two rows want the same title inside one destination. Not an error — a
    /// manuscript may legitimately have two scenes called "Later" — but it is
    /// the shape a double-import takes, which is worth catching before commit.
    DuplicateTitle { title: String, occurrences: usize },

    // ── plan-level: about the tree that would be created ────────────────────
    /// A heading was deeper than its parent by more than one level. The row is
    /// still created, clamped one level under its parent, rather than growing
    /// the phantom nesting that Scrivener is documented to produce.
    HeadingLevelJump { title: String, from: u8, to: u8 },
    /// The type a row resolved to cannot hold the content it carries, so the
    /// content would be dropped at save time. Caught here instead, where it can
    /// still be corrected.
    ///
    /// Carries the offending **type**, not a sentence about it. It used to carry
    /// `detail: String` built as `format!("a {role:?}/{sub_role:?} row cannot
    /// hold prose")` — an English sentence with `Debug`-formatted enum names in
    /// it, produced by the one module whose own doc says a diagnostic is *"data,
    /// never a log line… translated at the UI boundary rather than here"*. It
    /// went unnoticed for as long as nothing rendered it.
    IllegalCombination { title: String, kind: CreateType },
    /// An epigraph was found beside a heading whose type cannot hold one, so it was
    /// kept as ordinary prose instead.
    ///
    /// The constraint matrix allows `EpigraphText` on a Part or a Chapter and nowhere
    /// else — a Book's epigraph is the book's own front matter, and a Scene has no head
    /// to set a quotation at. Naming it is what stops the writer discovering later that
    /// the quotation they wrote as an epigraph is now the first paragraph of a chapter.
    EpigraphNotCarried { title: String, kind: CreateType },
    /// An epigraph sat between two headings that could both hold it, so it was given to
    /// the one **above** it.
    ///
    /// Both readings are real: `skribisto_compiler`'s `EpigraphPlacement` exports an
    /// epigraph after its heading (the documented convention, and the default) or before
    /// it (a designer's choice with real currency), and nothing in a returning file
    /// records which was used. The preceding heading wins because it is the convention;
    /// the writer is told because the other reading is not wrong, only less likely.
    EpigraphPlacementAmbiguous { above: String, below: String },
}

impl ImportDiagnostic {
    pub fn severity(&self) -> DiagnosticSeverity {
        use DiagnosticSeverity::*;
        use ImportDiagnostic::*;
        match self {
            FileUnreadable { .. } | UnsupportedFormat { .. } => Error,
            EmptyFile { .. } | NoHeadings { .. } | DecodedFromBom { .. } => Info,
            LossyDecode { .. }
            | FrontMatterNotFlat { .. }
            | FootnotesDegraded { .. }
            | RawHtmlDropped { .. }
            | NestedBreakDropped { .. }
            | ImageNotIngested { .. }
            | DuplicateTitle { .. }
            | HeadingLevelJump { .. }
            | IllegalCombination { .. }
            | TrackedChangesFlattened { .. }
            | TextBoxDropped { .. }
            | EmbeddedObjectDropped { .. }
            | FieldFlattened { .. }
            | UnknownStyleLevel { .. }
            | CommentUnanchored { .. }
            | CommentRepliesFlattened { .. }
            | EpigraphNotCarried { .. } => Warning,
            // Nothing was lost and nothing needs correcting — the epigraph landed on one
            // of the two rows it could have. Said once so a writer who meant the other
            // one can move it, which is a click.
            EpigraphPlacementAmbiguous { .. } => Info,
        }
    }

    /// The source this diagnostic is about, when it is about one file.
    pub fn path(&self) -> Option<&str> {
        use ImportDiagnostic::*;
        match self {
            FileUnreadable { path, .. }
            | LossyDecode { path, .. }
            | DecodedFromBom { path, .. }
            | EmptyFile { path }
            | NoHeadings { path }
            | UnsupportedFormat { path, .. }
            | FrontMatterNotFlat { path, .. }
            | FootnotesDegraded { path, .. }
            | RawHtmlDropped { path, .. }
            | NestedBreakDropped { path, .. }
            | ImageNotIngested { path, .. }
            | TrackedChangesFlattened { path, .. }
            | TextBoxDropped { path, .. }
            | EmbeddedObjectDropped { path, .. }
            | FieldFlattened { path, .. }
            | UnknownStyleLevel { path, .. }
            | CommentUnanchored { path, .. }
            | CommentRepliesFlattened { path, .. } => Some(path),
            DuplicateTitle { .. }
            | HeadingLevelJump { .. }
            | IllegalCombination { .. }
            | EpigraphNotCarried { .. }
            | EpigraphPlacementAmbiguous { .. } => None,
        }
    }

    /// A stable identifier for the UI to key a translation on. Deliberately not
    /// the message itself: the sentence the writer reads is assembled in
    /// `teksilo_ui` from this key plus the variant's own data, in their locale.
    pub fn key(&self) -> &'static str {
        use ImportDiagnostic::*;
        match self {
            FileUnreadable { .. } => "file-unreadable",
            LossyDecode { .. } => "lossy-decode",
            DecodedFromBom { .. } => "decoded-from-bom",
            EmptyFile { .. } => "empty-file",
            NoHeadings { .. } => "no-headings",
            UnsupportedFormat { .. } => "unsupported-format",
            FrontMatterNotFlat { .. } => "front-matter-not-flat",
            FootnotesDegraded { .. } => "footnotes-degraded",
            RawHtmlDropped { .. } => "raw-html-dropped",
            NestedBreakDropped { .. } => "nested-break-dropped",
            ImageNotIngested { .. } => "image-not-ingested",
            DuplicateTitle { .. } => "duplicate-title",
            HeadingLevelJump { .. } => "heading-level-jump",
            IllegalCombination { .. } => "illegal-combination",
            TrackedChangesFlattened { .. } => "tracked-changes-flattened",
            TextBoxDropped { .. } => "text-box-dropped",
            EmbeddedObjectDropped { .. } => "embedded-object-dropped",
            FieldFlattened { .. } => "field-flattened",
            UnknownStyleLevel { .. } => "unknown-style-level",
            CommentUnanchored { .. } => "comment-unanchored",
            CommentRepliesFlattened { .. } => "comment-replies-flattened",
            EpigraphNotCarried { .. } => "epigraph-not-carried",
            EpigraphPlacementAmbiguous { .. } => "epigraph-placement-ambiguous",
        }
    }
}

/// An untranslated rendering, for tests and logs. The UI never uses this — it
/// builds the writer's sentence from [`ImportDiagnostic::key`] and the variant's
/// fields, in their own locale.
impl fmt::Display for ImportDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImportDiagnostic::*;
        match self {
            FileUnreadable { path, reason } => write!(f, "{path}: unreadable ({reason})"),
            LossyDecode { path, replacements } => {
                write!(f, "{path}: {replacements} character(s) did not decode")
            }
            DecodedFromBom { path, encoding } => write!(f, "{path}: decoded as {encoding}"),
            EmptyFile { path } => write!(f, "{path}: empty"),
            NoHeadings { path } => write!(f, "{path}: no headings, imported as one row"),
            UnsupportedFormat { path, extension } => {
                write!(f, "{path}: no scanner handles '.{extension}'")
            }
            FrontMatterNotFlat { path, key } => {
                write!(f, "{path}: front-matter key '{key}' is not a scalar")
            }
            FootnotesDegraded { path, count } => {
                write!(f, "{path}: {count} footnote(s) kept as plain text")
            }
            RawHtmlDropped { path, count } => {
                write!(f, "{path}: {count} raw HTML block(s) dropped")
            }
            NestedBreakDropped { path, count } => {
                write!(
                    f,
                    "{path}: {count} scene-break-looking line(s) inside a nested block were dropped"
                )
            }
            ImageNotIngested { path, target } => {
                write!(f, "{path}: image '{target}' referenced but not imported")
            }
            DuplicateTitle { title, occurrences } => {
                write!(f, "'{title}' appears {occurrences} times")
            }
            HeadingLevelJump { title, from, to } => {
                write!(f, "'{title}': heading jumped from level {from} to {to}")
            }
            IllegalCombination { title, kind } => {
                write!(f, "'{title}': a {kind:?} row cannot hold prose")
            }
            EpigraphNotCarried { title, kind } => {
                write!(
                    f,
                    "'{title}': a {kind:?} row cannot hold an epigraph, so it was kept as prose"
                )
            }
            EpigraphPlacementAmbiguous { above, below } => {
                write!(
                    f,
                    "an epigraph between '{above}' and '{below}' was given to '{above}'"
                )
            }
            TrackedChangesFlattened { path, count } => {
                write!(f, "{path}: {count} tracked change(s) accepted")
            }
            TextBoxDropped { path, count } => write!(f, "{path}: {count} text box(es) dropped"),
            EmbeddedObjectDropped { path, count } => {
                write!(f, "{path}: {count} embedded object(s) dropped")
            }
            FieldFlattened { path, count } => {
                write!(f, "{path}: {count} field(s) replaced by their text")
            }
            UnknownStyleLevel { path, style } => {
                write!(f, "{path}: style '{style}' names no heading level")
            }
            CommentUnanchored { path, quote } => {
                write!(f, "{path}: comment '{quote}' could not be anchored")
            }
            CommentRepliesFlattened { path, count } => {
                write!(f, "{path}: {count} reply/replies became separate comments")
            }
        }
    }
}
