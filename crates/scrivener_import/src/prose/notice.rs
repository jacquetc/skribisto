// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What converting the prose did that the writer should know or check.
//!
//! One value per instance, never per rule: each names where it happened ([`Place`]) and
//! carries at most one number and one string, so the report can list the documents behind
//! each line. Like every other notice of this importer, none words anything: the interface
//! translates the [`key`](ProseNotice::key) in the writer's language and files it under its
//! [`group`](ProseNotice::group). Every key starts with `scrivener-prose-`.
//!
//! The document importer's own findings about a document's prose (a comment that could not
//! be placed on its words, a picture in a format the editor cannot show, a paragraph stored
//! without its formatting) arrive as [`ProseNotice::Document`], carrying that importer's key,
//! so their sentences are the ones the document import already has.

use serde::{Deserialize, Serialize};

use crate::structure::ReportGroup;

/// Where in the Scrivener project something was found. Items are binder rows, by index into
/// the model's binder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Place {
    /// An item's main text.
    Text { item: u32 },
    /// An item's document notes.
    Notes { item: u32 },
    /// An item's synopsis.
    Synopsis { item: u32 },
    /// An item's picture, PDF or other file.
    Media { item: u32 },
    /// The picture on an item's index card.
    CardImage { item: u32 },
    /// A format 1.x project note, by index.
    ProjectNote { note: u32 },
    /// A recovered text, by index.
    Recovered { text: u32 },
}

/// Which reading of a format 1.0 range put a comment on word boundaries (D5.11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RangeReading {
    /// Inline notes' markers not counted, pictures counted: the reading that fits every
    /// measured range.
    MarkersRemoved,
    /// Inline notes' markers counted.
    MarkersCounted,
    /// Pictures not counted.
    PicturesNotCounted,
}

/// Formatting the editor has no place for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Formatting {
    /// Text colour (black and the automatic colour excluded).
    Colour,
    /// Highlighting (a white background excluded).
    Highlight,
    SmallCaps,
    /// Text shown in capitals whatever its letters.
    AllCaps,
    /// Hidden text, left out.
    Hidden,
}

/// What a compile does with a style's text, when it does not print it as prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CompileTreatment {
    /// The compile format deletes it (`DeleteText`): notes to oneself, text for one
    /// platform only.
    Deleted,
    /// The compile format passes it through as raw markup (`RawMarkup`): code for another
    /// program (LaTeX, XML) that Scrivener never printed as prose.
    RawMarkup,
}

/// Content of an RTF file that no text carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeftOut {
    /// Pieces of the file after the document's own closing brace: a damaged file ends
    /// there for every reader, and the words after it with it.
    AfterDocument,
    /// Headers and footers, which are not part of the text's flow.
    HeaderOrFooter,
    /// Pictures a list's labels showed (picture bullets): the label is not kept.
    ListLabelPicture,
}

/// Something converting the prose did that the writer should know.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProseNotice {
    // ------------------------------------------------ could not be read
    /// A text that could not be read as RTF, nor recovered: it arrives empty.
    Unreadable { at: Place },
    /// A text Scrivener stored as plain text rather than RTF, read as such.
    ReadAsPlainText { at: Place },
    /// The file holding a document's comments and linked footnotes could not be read: the
    /// words they marked are kept, the comments and notes are not.
    CommentsUnreadable { at: Place },

    // ------------------------------------------------ check these
    /// Inline note markers that do not pair (an opener never closed, a closer with nothing
    /// open, a note inside a note): kept as the text they are, for the writer to see.
    InlineNoteUnmatched { at: Place, count: u32 },
    /// Style markers naming a style the project does not define: the markers are removed,
    /// the text keeps its formatting.
    StyleUnresolved { at: Place, count: u32 },
    /// Paragraphs of a project Scrivener 3 exported to its older format that may be headings
    /// whose style the export dropped (`<$ScrKeepWithNext>` with no style): they come across
    /// as ordinary paragraphs.
    StylesLostInExport { at: Place, count: u32 },
    /// A comment or footnote anchor whose body the project no longer holds: the words stay,
    /// the anchor does not.
    CommentAnchorWithoutEntry { at: Place, count: u32 },
    /// Comments the project holds with no place in the text: kept on the first paragraph.
    CommentEntryWithoutAnchor { at: Place, count: u32 },
    /// Linked footnotes with no place in the text: cited at the end of the text.
    FootnoteEntryWithoutAnchor { at: Place, count: u32 },
    /// Format 1.0 comments whose place was read another way than usual (D5.11).
    RangeCommentReading {
        at: Place,
        reading: RangeReading,
        count: u32,
    },
    /// Format 1.0 comments whose place fits no reading exactly: placed on the nearest
    /// reading, possibly a few characters off.
    RangeCommentApproximate { at: Place, count: u32 },
    /// Links to documents the import has no row for (in the Trash left out, missing, or
    /// never imported): the words stay.
    DocumentLinkUnresolved { at: Place, count: u32 },
    /// A picture linked to a file on the writer's disk, not read: `name` is its file name
    /// only.
    LinkedPictureNotRead { at: Place, name: String },
    /// The first paragraph is a heading that repeats the document's title: kept, and an
    /// export may print the title twice.
    HeadingRepeatsTitle { at: Place },
    /// Paragraphs written in Markdown (headings, emphasis, links typed as marks): Scrivener
    /// compiled them through Markdown, Skribisto keeps them as the text they are (A7).
    MarkdownAsText { at: Place, paragraphs: u32 },
    /// Paragraphs holding citation keys (`[@key]`) for a bibliography tool: kept as text.
    CitationKeysAsText { at: Place, paragraphs: u32 },
    /// Compile placeholders (`<$n>`, `<$projecttitle>`) in text that exports: kept as the
    /// text they are, which Skribisto prints as written. `kind` is the placeholder as the
    /// project writes it (`<$R>`, `<$custom:Mood>`), for the report to quote.
    PlaceholdersInExportedText { at: Place, kind: String, count: u32 },
    /// The same, in text that does not export.
    PlaceholdersInOtherText { at: Place, kind: String, count: u32 },
    /// Centred lines of ornaments (`❃ ✽ ✾`) that may mark scene breaks, kept as written.
    OrnamentLinesKept { at: Place, count: u32 },
    /// A very long document: Skribisto will be slow to open it (X-11).
    LongDocument { at: Place, paragraphs: u32 },
    /// Pictures with no description for a reader who cannot see them (X1): the writer can
    /// add one in the editor.
    PicturesWithoutDescription { at: Place, count: u32 },
    /// Struck-through text, kept struck through, in a project whose compile removed it:
    /// Scrivener left it out of the book, Skribisto's export prints it (D5.21). `count`
    /// characters.
    StruckTextCompiledAway { at: Place, count: u32 },
    /// Text in a style the project's compile format deleted or passed through as raw
    /// markup, kept as prose: Skribisto's export prints it (D5.8). `name` is the style's,
    /// `count` the paragraphs or characters it covered.
    CompiledAwayStylePrinted {
        at: Place,
        name: String,
        treatment: CompileTreatment,
        count: u32,
    },

    // ------------------------------------------------ brought across differently
    /// Inline notes Scrivener split where their formatting changed, joined back.
    InlineNotesMerged { at: Place, count: u32 },
    /// Inline notes written one per paragraph (Scrivener for Windows), joined into
    /// multi-paragraph notes.
    InlineNotesJoinedAcrossParagraphs { at: Place, count: u32 },
    /// Inline notes holding nothing, left out.
    BlankNotesDropped { at: Place, count: u32 },
    /// Inline annotations, which Scrivener leaves out of a compile, became comments.
    AnnotationsBecameComments { at: Place, count: u32 },
    /// A document holding nothing but an annotation: its text is kept as prose.
    AnnotationOnlyDocumentKept { at: Place },
    /// Preserve Formatting passages: the protection is gone, the text kept.
    PreserveFormattingPassages { at: Place, count: u32 },
    /// A note inside a comment or a footnote, which holds none: its text is kept in place.
    NestedNoteUnwrapped { at: Place, count: u32 },
    /// A style Skribisto has no equivalent for: `name` is the style's, `count` the
    /// paragraphs or characters it covered.
    StyleNotCarried { at: Place, name: String, count: u32 },
    /// Links to other documents of the project whose link Skribisto has no place for: the
    /// words stay (D5.16).
    DocumentLinksDropped { at: Place, count: u32 },
    /// Links to other documents kept as references where Skribisto shows them.
    DocumentLinksKept { at: Place, count: u32 },
    /// Links to anything but a web or mail address (another project, a file): the words
    /// stay.
    OtherLinksKeptAsText { at: Place, count: u32 },
    /// Line breaks inside paragraphs, each made a paragraph of its own.
    SoftBreaksSplit { at: Place, count: u32 },
    /// Indents typed at the start of paragraphs, removed.
    TypedIndentsRemoved { at: Place, count: u32 },
    /// Paragraphs indented from the margin: the indent is not kept.
    LeftIndentsNotKept { at: Place, count: u32 },
    /// Empty paragraphs between other paragraphs, removed.
    BlankLinesRemoved { at: Place, count: u32 },
    /// Blank lines between the stanzas of a poem, removed with the others (I3 A14): the
    /// stanzas now run on.
    VerseStanzaGapsLost { at: Place, count: u32 },
    /// Blank lines between paragraphs of prose, made scene breaks.
    BlankLinesBecameSceneBreaks { at: Place, count: u32 },
    /// Centred ornament lines, made scene breaks.
    OrnamentsBecameSceneBreaks { at: Place, count: u32 },
    /// Tables, each now opening with a header row.
    TableFirstRowHeader { at: Place, count: u32 },
    /// Table cells holding several paragraphs, their paragraphs joined.
    TableCellsJoined { at: Place, count: u32 },
    /// Formatting the editor has no place for, left out.
    FormattingLost {
        at: Place,
        what: Formatting,
        count: u32,
    },
    /// Justified paragraphs: the export style decides justification.
    JustifiedTextNotKept { at: Place, count: u32 },
    /// Comments whose colour is not kept.
    CommentColoursLost { at: Place, count: u32 },
    /// Invisible control characters and stray object markers, removed.
    ControlCharactersRemoved { at: Place, count: u32 },
    /// Content of the file no text carries: `count` pieces, headers or pictures.
    LeftOut {
        at: Place,
        what: LeftOut,
        count: u32,
    },
    /// Characters the file wrote in no encoding it names, or half of a pair: each is shown
    /// as a replacement mark (U+FFFD) where it stood.
    UndecodableCharacters { at: Place, count: u32 },
    /// A footnote or comment on a chapter heading that became a title: a title holds none,
    /// so it went to the chapter's first paragraph (a comment) or was left out (a note).
    NoteOnHeadingLost { at: Place, count: u32 },

    /// A research file Skribisto cannot open, copied into the folder beside the project:
    /// `path` is where, relative to the project's folder, as its note names it.
    MediaFileCopied { at: Place, path: String },

    // ------------------------------------------------ left behind
    /// A research file Skribisto cannot open, left in the Scrivener project: its note names
    /// the file.
    MediaFileNotCarried { at: Place, name: String },

    // ------------------------------------------------ the document importer's own
    /// One of the document importer's findings about this text, under its own key
    /// (`ImportDiagnostic::key`), with the one number and the one string that importer's
    /// own review row carries for it (`ImportDiagnostic::detail_and_count`: a picture's
    /// name, a comment's first words, the level a list was flattened to), so the sentences
    /// written for that row word it alike.
    Document {
        at: Place,
        key: String,
        warning: bool,
        count: u32,
        detail: String,
    },
}

impl ProseNotice {
    /// Every key a notice of this importer's own can have, for the interface's translation
    /// tests. [`ProseNotice::Document`] carries the document importer's keys, which that
    /// importer's own tests cover.
    pub const KEYS: &'static [&'static str] = &[
        "scrivener-prose-unreadable",
        "scrivener-prose-read-as-plain-text",
        "scrivener-prose-comments-unreadable",
        "scrivener-prose-inline-note-unmatched",
        "scrivener-prose-style-unresolved",
        "scrivener-prose-comment-anchor-without-entry",
        "scrivener-prose-comment-entry-without-anchor",
        "scrivener-prose-footnote-entry-without-anchor",
        "scrivener-prose-range-comment-reading",
        "scrivener-prose-range-comment-approximate",
        "scrivener-prose-document-link-unresolved",
        "scrivener-prose-linked-picture-not-read",
        "scrivener-prose-heading-repeats-title",
        "scrivener-prose-markdown-as-text",
        "scrivener-prose-citation-keys-as-text",
        "scrivener-prose-placeholders-in-exported-text",
        "scrivener-prose-placeholders-in-other-text",
        "scrivener-prose-ornament-lines-kept",
        "scrivener-prose-long-document",
        "scrivener-prose-pictures-without-description",
        "scrivener-prose-struck-text-compiled-away",
        "scrivener-prose-compile-deleted-style-printed",
        "scrivener-prose-compile-raw-style-printed",
        "scrivener-prose-inline-notes-merged",
        "scrivener-prose-inline-notes-joined-across-paragraphs",
        "scrivener-prose-blank-notes-dropped",
        "scrivener-prose-annotations-became-comments",
        "scrivener-prose-annotation-only-document-kept",
        "scrivener-prose-preserve-formatting-passages",
        "scrivener-prose-nested-note-unwrapped",
        "scrivener-prose-style-not-carried",
        "scrivener-prose-document-links-dropped",
        "scrivener-prose-document-links-kept",
        "scrivener-prose-other-links-kept-as-text",
        "scrivener-prose-soft-breaks-split",
        "scrivener-prose-typed-indents-removed",
        "scrivener-prose-left-indents-not-kept",
        "scrivener-prose-blank-lines-removed",
        "scrivener-prose-verse-stanza-gaps-lost",
        "scrivener-prose-blank-lines-became-scene-breaks",
        "scrivener-prose-ornaments-became-scene-breaks",
        "scrivener-prose-table-first-row-header",
        "scrivener-prose-table-cells-joined",
        "scrivener-prose-formatting-lost",
        "scrivener-prose-justified-text-not-kept",
        "scrivener-prose-comment-colours-lost",
        "scrivener-prose-control-characters-removed",
        "scrivener-prose-note-on-heading-lost",
        "scrivener-prose-left-out",
        "scrivener-prose-undecodable-characters",
        "scrivener-prose-media-file-copied",
        "scrivener-prose-media-file-not-carried",
        "scrivener-prose-styles-lost-in-export",
    ];

    /// The key the interface translates.
    pub fn key(&self) -> &str {
        match self {
            ProseNotice::Unreadable { .. } => "scrivener-prose-unreadable",
            ProseNotice::ReadAsPlainText { .. } => "scrivener-prose-read-as-plain-text",
            ProseNotice::CommentsUnreadable { .. } => "scrivener-prose-comments-unreadable",
            ProseNotice::InlineNoteUnmatched { .. } => "scrivener-prose-inline-note-unmatched",
            ProseNotice::StyleUnresolved { .. } => "scrivener-prose-style-unresolved",
            ProseNotice::CommentAnchorWithoutEntry { .. } => {
                "scrivener-prose-comment-anchor-without-entry"
            }
            ProseNotice::CommentEntryWithoutAnchor { .. } => {
                "scrivener-prose-comment-entry-without-anchor"
            }
            ProseNotice::FootnoteEntryWithoutAnchor { .. } => {
                "scrivener-prose-footnote-entry-without-anchor"
            }
            ProseNotice::RangeCommentReading { .. } => "scrivener-prose-range-comment-reading",
            ProseNotice::RangeCommentApproximate { .. } => {
                "scrivener-prose-range-comment-approximate"
            }
            ProseNotice::DocumentLinkUnresolved { .. } => {
                "scrivener-prose-document-link-unresolved"
            }
            ProseNotice::LinkedPictureNotRead { .. } => "scrivener-prose-linked-picture-not-read",
            ProseNotice::HeadingRepeatsTitle { .. } => "scrivener-prose-heading-repeats-title",
            ProseNotice::MarkdownAsText { .. } => "scrivener-prose-markdown-as-text",
            ProseNotice::CitationKeysAsText { .. } => "scrivener-prose-citation-keys-as-text",
            ProseNotice::PlaceholdersInExportedText { .. } => {
                "scrivener-prose-placeholders-in-exported-text"
            }
            ProseNotice::PlaceholdersInOtherText { .. } => {
                "scrivener-prose-placeholders-in-other-text"
            }
            ProseNotice::OrnamentLinesKept { .. } => "scrivener-prose-ornament-lines-kept",
            ProseNotice::LongDocument { .. } => "scrivener-prose-long-document",
            ProseNotice::PicturesWithoutDescription { .. } => {
                "scrivener-prose-pictures-without-description"
            }
            ProseNotice::StruckTextCompiledAway { .. } => {
                "scrivener-prose-struck-text-compiled-away"
            }
            ProseNotice::CompiledAwayStylePrinted {
                treatment: CompileTreatment::Deleted,
                ..
            } => "scrivener-prose-compile-deleted-style-printed",
            ProseNotice::CompiledAwayStylePrinted {
                treatment: CompileTreatment::RawMarkup,
                ..
            } => "scrivener-prose-compile-raw-style-printed",
            ProseNotice::InlineNotesMerged { .. } => "scrivener-prose-inline-notes-merged",
            ProseNotice::InlineNotesJoinedAcrossParagraphs { .. } => {
                "scrivener-prose-inline-notes-joined-across-paragraphs"
            }
            ProseNotice::BlankNotesDropped { .. } => "scrivener-prose-blank-notes-dropped",
            ProseNotice::AnnotationsBecameComments { .. } => {
                "scrivener-prose-annotations-became-comments"
            }
            ProseNotice::AnnotationOnlyDocumentKept { .. } => {
                "scrivener-prose-annotation-only-document-kept"
            }
            ProseNotice::PreserveFormattingPassages { .. } => {
                "scrivener-prose-preserve-formatting-passages"
            }
            ProseNotice::NestedNoteUnwrapped { .. } => "scrivener-prose-nested-note-unwrapped",
            ProseNotice::StyleNotCarried { .. } => "scrivener-prose-style-not-carried",
            ProseNotice::DocumentLinksDropped { .. } => "scrivener-prose-document-links-dropped",
            ProseNotice::DocumentLinksKept { .. } => "scrivener-prose-document-links-kept",
            ProseNotice::OtherLinksKeptAsText { .. } => "scrivener-prose-other-links-kept-as-text",
            ProseNotice::SoftBreaksSplit { .. } => "scrivener-prose-soft-breaks-split",
            ProseNotice::TypedIndentsRemoved { .. } => "scrivener-prose-typed-indents-removed",
            ProseNotice::LeftIndentsNotKept { .. } => "scrivener-prose-left-indents-not-kept",
            ProseNotice::BlankLinesRemoved { .. } => "scrivener-prose-blank-lines-removed",
            ProseNotice::VerseStanzaGapsLost { .. } => "scrivener-prose-verse-stanza-gaps-lost",
            ProseNotice::BlankLinesBecameSceneBreaks { .. } => {
                "scrivener-prose-blank-lines-became-scene-breaks"
            }
            ProseNotice::OrnamentsBecameSceneBreaks { .. } => {
                "scrivener-prose-ornaments-became-scene-breaks"
            }
            ProseNotice::TableFirstRowHeader { .. } => "scrivener-prose-table-first-row-header",
            ProseNotice::TableCellsJoined { .. } => "scrivener-prose-table-cells-joined",
            ProseNotice::FormattingLost { .. } => "scrivener-prose-formatting-lost",
            ProseNotice::JustifiedTextNotKept { .. } => "scrivener-prose-justified-text-not-kept",
            ProseNotice::CommentColoursLost { .. } => "scrivener-prose-comment-colours-lost",
            ProseNotice::ControlCharactersRemoved { .. } => {
                "scrivener-prose-control-characters-removed"
            }
            ProseNotice::NoteOnHeadingLost { .. } => "scrivener-prose-note-on-heading-lost",
            ProseNotice::LeftOut { .. } => "scrivener-prose-left-out",
            ProseNotice::UndecodableCharacters { .. } => "scrivener-prose-undecodable-characters",
            ProseNotice::MediaFileCopied { .. } => "scrivener-prose-media-file-copied",
            ProseNotice::MediaFileNotCarried { .. } => "scrivener-prose-media-file-not-carried",
            ProseNotice::StylesLostInExport { .. } => "scrivener-prose-styles-lost-in-export",
            ProseNotice::Document { key, .. } => key,
        }
    }

    /// Which group of the report it belongs to.
    pub fn group(&self) -> ReportGroup {
        match self {
            ProseNotice::Unreadable { .. } | ProseNotice::CommentsUnreadable { .. } => {
                ReportGroup::CouldNotBeRead
            }
            ProseNotice::ReadAsPlainText { .. }
            | ProseNotice::InlineNoteUnmatched { .. }
            | ProseNotice::StyleUnresolved { .. }
            | ProseNotice::StylesLostInExport { .. }
            | ProseNotice::CommentAnchorWithoutEntry { .. }
            | ProseNotice::CommentEntryWithoutAnchor { .. }
            | ProseNotice::FootnoteEntryWithoutAnchor { .. }
            | ProseNotice::RangeCommentReading { .. }
            | ProseNotice::RangeCommentApproximate { .. }
            | ProseNotice::DocumentLinkUnresolved { .. }
            | ProseNotice::LinkedPictureNotRead { .. }
            | ProseNotice::HeadingRepeatsTitle { .. }
            | ProseNotice::MarkdownAsText { .. }
            | ProseNotice::CitationKeysAsText { .. }
            | ProseNotice::PlaceholdersInExportedText { .. }
            | ProseNotice::OrnamentLinesKept { .. }
            | ProseNotice::LongDocument { .. }
            | ProseNotice::PicturesWithoutDescription { .. }
            | ProseNotice::StruckTextCompiledAway { .. }
            | ProseNotice::CompiledAwayStylePrinted { .. }
            | ProseNotice::LeftOut { .. }
            | ProseNotice::UndecodableCharacters { .. } => ReportGroup::CheckThese,
            ProseNotice::MediaFileNotCarried { .. } => ReportGroup::LeftBehind,
            ProseNotice::Document { warning: true, .. } => ReportGroup::CheckThese,
            _ => ReportGroup::BroughtAcrossDifferently,
        }
    }

    /// Where it happened.
    pub fn place(&self) -> Place {
        match self {
            ProseNotice::Unreadable { at }
            | ProseNotice::ReadAsPlainText { at }
            | ProseNotice::CommentsUnreadable { at }
            | ProseNotice::InlineNoteUnmatched { at, .. }
            | ProseNotice::StyleUnresolved { at, .. }
            | ProseNotice::StylesLostInExport { at, .. }
            | ProseNotice::CommentAnchorWithoutEntry { at, .. }
            | ProseNotice::CommentEntryWithoutAnchor { at, .. }
            | ProseNotice::FootnoteEntryWithoutAnchor { at, .. }
            | ProseNotice::RangeCommentReading { at, .. }
            | ProseNotice::RangeCommentApproximate { at, .. }
            | ProseNotice::DocumentLinkUnresolved { at, .. }
            | ProseNotice::LinkedPictureNotRead { at, .. }
            | ProseNotice::HeadingRepeatsTitle { at }
            | ProseNotice::MarkdownAsText { at, .. }
            | ProseNotice::CitationKeysAsText { at, .. }
            | ProseNotice::PlaceholdersInExportedText { at, .. }
            | ProseNotice::PlaceholdersInOtherText { at, .. }
            | ProseNotice::OrnamentLinesKept { at, .. }
            | ProseNotice::LongDocument { at, .. }
            | ProseNotice::PicturesWithoutDescription { at, .. }
            | ProseNotice::StruckTextCompiledAway { at, .. }
            | ProseNotice::CompiledAwayStylePrinted { at, .. }
            | ProseNotice::VerseStanzaGapsLost { at, .. }
            | ProseNotice::InlineNotesMerged { at, .. }
            | ProseNotice::InlineNotesJoinedAcrossParagraphs { at, .. }
            | ProseNotice::BlankNotesDropped { at, .. }
            | ProseNotice::AnnotationsBecameComments { at, .. }
            | ProseNotice::AnnotationOnlyDocumentKept { at }
            | ProseNotice::PreserveFormattingPassages { at, .. }
            | ProseNotice::NestedNoteUnwrapped { at, .. }
            | ProseNotice::StyleNotCarried { at, .. }
            | ProseNotice::DocumentLinksDropped { at, .. }
            | ProseNotice::DocumentLinksKept { at, .. }
            | ProseNotice::OtherLinksKeptAsText { at, .. }
            | ProseNotice::SoftBreaksSplit { at, .. }
            | ProseNotice::TypedIndentsRemoved { at, .. }
            | ProseNotice::LeftIndentsNotKept { at, .. }
            | ProseNotice::BlankLinesRemoved { at, .. }
            | ProseNotice::BlankLinesBecameSceneBreaks { at, .. }
            | ProseNotice::OrnamentsBecameSceneBreaks { at, .. }
            | ProseNotice::TableFirstRowHeader { at, .. }
            | ProseNotice::TableCellsJoined { at, .. }
            | ProseNotice::FormattingLost { at, .. }
            | ProseNotice::JustifiedTextNotKept { at, .. }
            | ProseNotice::CommentColoursLost { at, .. }
            | ProseNotice::ControlCharactersRemoved { at, .. }
            | ProseNotice::NoteOnHeadingLost { at, .. }
            | ProseNotice::LeftOut { at, .. }
            | ProseNotice::UndecodableCharacters { at, .. }
            | ProseNotice::MediaFileCopied { at, .. }
            | ProseNotice::MediaFileNotCarried { at, .. }
            | ProseNotice::Document { at, .. } => *at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_is_the_importers_own_or_the_document_importers() {
        let at = Place::Text { item: 1 };
        let own = [
            ProseNotice::Unreadable { at },
            ProseNotice::InlineNotesMerged { at, count: 1 },
            ProseNotice::MediaFileNotCarried {
                at,
                name: "x.pdf".into(),
            },
        ];
        for notice in own {
            assert!(notice.key().starts_with("scrivener-prose-"), "{notice:?}");
        }
        let passed = ProseNotice::Document {
            at,
            key: "comment-unanchored".into(),
            warning: true,
            count: 1,
            detail: String::new(),
        };
        assert_eq!(passed.key(), "comment-unanchored");
        assert_eq!(passed.group(), ReportGroup::CheckThese);
    }

    /// One notice of each of this importer's own kinds, both compile treatments apart: they
    /// are two keys.
    fn one_of_each() -> Vec<ProseNotice> {
        let at = Place::Text { item: 1 };
        let name = || "Scene".to_string();
        vec![
            ProseNotice::Unreadable { at },
            ProseNotice::ReadAsPlainText { at },
            ProseNotice::CommentsUnreadable { at },
            ProseNotice::InlineNoteUnmatched { at, count: 1 },
            ProseNotice::StyleUnresolved { at, count: 1 },
            ProseNotice::CommentAnchorWithoutEntry { at, count: 1 },
            ProseNotice::CommentEntryWithoutAnchor { at, count: 1 },
            ProseNotice::FootnoteEntryWithoutAnchor { at, count: 1 },
            ProseNotice::RangeCommentReading {
                at,
                reading: RangeReading::MarkersCounted,
                count: 1,
            },
            ProseNotice::RangeCommentApproximate { at, count: 1 },
            ProseNotice::DocumentLinkUnresolved { at, count: 1 },
            ProseNotice::LinkedPictureNotRead { at, name: name() },
            ProseNotice::HeadingRepeatsTitle { at },
            ProseNotice::MarkdownAsText { at, paragraphs: 1 },
            ProseNotice::CitationKeysAsText { at, paragraphs: 1 },
            ProseNotice::PlaceholdersInExportedText {
                at,
                kind: name(),
                count: 1,
            },
            ProseNotice::PlaceholdersInOtherText {
                at,
                kind: name(),
                count: 1,
            },
            ProseNotice::OrnamentLinesKept { at, count: 1 },
            ProseNotice::LongDocument { at, paragraphs: 1 },
            ProseNotice::PicturesWithoutDescription { at, count: 1 },
            ProseNotice::StruckTextCompiledAway { at, count: 1 },
            ProseNotice::CompiledAwayStylePrinted {
                at,
                name: name(),
                treatment: CompileTreatment::Deleted,
                count: 1,
            },
            ProseNotice::CompiledAwayStylePrinted {
                at,
                name: name(),
                treatment: CompileTreatment::RawMarkup,
                count: 1,
            },
            ProseNotice::InlineNotesMerged { at, count: 1 },
            ProseNotice::InlineNotesJoinedAcrossParagraphs { at, count: 1 },
            ProseNotice::BlankNotesDropped { at, count: 1 },
            ProseNotice::AnnotationsBecameComments { at, count: 1 },
            ProseNotice::AnnotationOnlyDocumentKept { at },
            ProseNotice::PreserveFormattingPassages { at, count: 1 },
            ProseNotice::NestedNoteUnwrapped { at, count: 1 },
            ProseNotice::StyleNotCarried {
                at,
                name: name(),
                count: 1,
            },
            ProseNotice::DocumentLinksDropped { at, count: 1 },
            ProseNotice::DocumentLinksKept { at, count: 1 },
            ProseNotice::OtherLinksKeptAsText { at, count: 1 },
            ProseNotice::SoftBreaksSplit { at, count: 1 },
            ProseNotice::TypedIndentsRemoved { at, count: 1 },
            ProseNotice::LeftIndentsNotKept { at, count: 1 },
            ProseNotice::BlankLinesRemoved { at, count: 1 },
            ProseNotice::VerseStanzaGapsLost { at, count: 1 },
            ProseNotice::BlankLinesBecameSceneBreaks { at, count: 1 },
            ProseNotice::OrnamentsBecameSceneBreaks { at, count: 1 },
            ProseNotice::TableFirstRowHeader { at, count: 1 },
            ProseNotice::TableCellsJoined { at, count: 1 },
            ProseNotice::FormattingLost {
                at,
                what: Formatting::Colour,
                count: 1,
            },
            ProseNotice::JustifiedTextNotKept { at, count: 1 },
            ProseNotice::CommentColoursLost { at, count: 1 },
            ProseNotice::ControlCharactersRemoved { at, count: 1 },
            ProseNotice::NoteOnHeadingLost { at, count: 1 },
            ProseNotice::LeftOut {
                at,
                what: LeftOut::HeaderOrFooter,
                count: 1,
            },
            ProseNotice::UndecodableCharacters { at, count: 1 },
            ProseNotice::MediaFileCopied { at, path: name() },
            ProseNotice::MediaFileNotCarried { at, name: name() },
            ProseNotice::StylesLostInExport { at, count: 1 },
        ]
    }

    /// Which entry of [`one_of_each`] a notice is, by an exhaustive match: a new kind fails
    /// to compile here until it has an entry there and a key in [`ProseNotice::KEYS`].
    fn slot(notice: &ProseNotice) -> Option<usize> {
        Some(match notice {
            ProseNotice::Unreadable { .. } => 0,
            ProseNotice::ReadAsPlainText { .. } => 1,
            ProseNotice::CommentsUnreadable { .. } => 2,
            ProseNotice::InlineNoteUnmatched { .. } => 3,
            ProseNotice::StyleUnresolved { .. } => 4,
            ProseNotice::CommentAnchorWithoutEntry { .. } => 5,
            ProseNotice::CommentEntryWithoutAnchor { .. } => 6,
            ProseNotice::FootnoteEntryWithoutAnchor { .. } => 7,
            ProseNotice::RangeCommentReading { .. } => 8,
            ProseNotice::RangeCommentApproximate { .. } => 9,
            ProseNotice::DocumentLinkUnresolved { .. } => 10,
            ProseNotice::LinkedPictureNotRead { .. } => 11,
            ProseNotice::HeadingRepeatsTitle { .. } => 12,
            ProseNotice::MarkdownAsText { .. } => 13,
            ProseNotice::CitationKeysAsText { .. } => 14,
            ProseNotice::PlaceholdersInExportedText { .. } => 15,
            ProseNotice::PlaceholdersInOtherText { .. } => 16,
            ProseNotice::OrnamentLinesKept { .. } => 17,
            ProseNotice::LongDocument { .. } => 18,
            ProseNotice::PicturesWithoutDescription { .. } => 19,
            ProseNotice::StruckTextCompiledAway { .. } => 20,
            ProseNotice::CompiledAwayStylePrinted {
                treatment: CompileTreatment::Deleted,
                ..
            } => 21,
            ProseNotice::CompiledAwayStylePrinted {
                treatment: CompileTreatment::RawMarkup,
                ..
            } => 22,
            ProseNotice::InlineNotesMerged { .. } => 23,
            ProseNotice::InlineNotesJoinedAcrossParagraphs { .. } => 24,
            ProseNotice::BlankNotesDropped { .. } => 25,
            ProseNotice::AnnotationsBecameComments { .. } => 26,
            ProseNotice::AnnotationOnlyDocumentKept { .. } => 27,
            ProseNotice::PreserveFormattingPassages { .. } => 28,
            ProseNotice::NestedNoteUnwrapped { .. } => 29,
            ProseNotice::StyleNotCarried { .. } => 30,
            ProseNotice::DocumentLinksDropped { .. } => 31,
            ProseNotice::DocumentLinksKept { .. } => 32,
            ProseNotice::OtherLinksKeptAsText { .. } => 33,
            ProseNotice::SoftBreaksSplit { .. } => 34,
            ProseNotice::TypedIndentsRemoved { .. } => 35,
            ProseNotice::LeftIndentsNotKept { .. } => 36,
            ProseNotice::BlankLinesRemoved { .. } => 37,
            ProseNotice::VerseStanzaGapsLost { .. } => 38,
            ProseNotice::BlankLinesBecameSceneBreaks { .. } => 39,
            ProseNotice::OrnamentsBecameSceneBreaks { .. } => 40,
            ProseNotice::TableFirstRowHeader { .. } => 41,
            ProseNotice::TableCellsJoined { .. } => 42,
            ProseNotice::FormattingLost { .. } => 43,
            ProseNotice::JustifiedTextNotKept { .. } => 44,
            ProseNotice::CommentColoursLost { .. } => 45,
            ProseNotice::ControlCharactersRemoved { .. } => 46,
            ProseNotice::NoteOnHeadingLost { .. } => 47,
            ProseNotice::LeftOut { .. } => 48,
            ProseNotice::UndecodableCharacters { .. } => 49,
            ProseNotice::MediaFileCopied { .. } => 50,
            ProseNotice::MediaFileNotCarried { .. } => 51,
            ProseNotice::StylesLostInExport { .. } => 52,
            ProseNotice::Document { .. } => return None,
        })
    }

    #[test]
    fn every_own_key_is_listed_once() {
        let all = one_of_each();
        let slots: Vec<Option<usize>> = all.iter().map(slot).collect();
        assert_eq!(slots, (0..all.len()).map(Some).collect::<Vec<_>>());
        assert_eq!(all.len(), ProseNotice::KEYS.len());
        for (notice, listed) in all.iter().zip(ProseNotice::KEYS) {
            assert_eq!(notice.key(), *listed, "{notice:?}");
        }
        let mut keys = ProseNotice::KEYS.to_vec();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), ProseNotice::KEYS.len(), "a key listed twice");
    }
}
