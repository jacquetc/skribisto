// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the structure mapping decided that the writer should know or check.
//!
//! One value per instance, never per rule: the report lists the documents behind
//! each line, so each notice names its item (a binder row), and the interface
//! groups them by [`key`](StructureNotice::key) under one of the report's four
//! [`ReportGroup`]s. Like the source layer's notices, none words anything: the
//! key is translated in the writer's language, and the data is what the sentence
//! needs. Every key starts with `scrivener-structure-`.

use serde::{Deserialize, Serialize};

use super::Level;

/// The four fixed groups of the import report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportGroup {
    CouldNotBeRead,
    CheckThese,
    BroughtAcrossDifferently,
    LeftBehind,
}

/// Something the mapping decided that the writer should know.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureNotice {
    // ------------------------------------------------ check these
    /// At least half the Draft's documents are in script mode: Skribisto keeps
    /// a script's words, not its screenplay layout (A12).
    ScriptModeDraft {
        script_documents: u32,
        draft_documents: u32,
    },
    /// A Draft document holds its chapters as headings (A2). `split` says
    /// whether the plan splits it into chapters.
    OneDocumentManuscript {
        item: u32,
        headings: u32,
        split: bool,
    },
    /// A chapter or part title that also spells its number ("CHAPTER II",
    /// "Part 1: Basics"): the export's heading prints the number too.
    TitleRepeatsNumber { item: u32 },
    /// A level a signal claimed, refused because an ancestor already holds it
    /// (a chapter inside a chapter): the item took the level below.
    ClaimOverruled { item: u32, claimed: Level },
    /// A text that follows a chapter among its siblings: in the book, it is the
    /// end of that chapter.
    AdoptedByChapter { item: u32, chapter: u32 },
    /// A root folder kept in the notes that reads like manuscript: the review
    /// can send it to the manuscript as another book.
    LooksLikeManuscript { item: u32 },
    /// A story-bible entry titled like a template's sheet ("Hero",
    /// "Untitled"): the mention index will look for that word in the prose.
    GenericSheetTitle { item: u32 },

    // ------------------------------------------------ brought across differently
    /// A title that only named its level ("Chapter"), stored blank: the
    /// binder and the export name the row by its number (A3).
    PlaceholderTitle { item: u32 },
    /// A title with line breaks, tabs or decomposed accents, cleaned (D3.18).
    TitleCleaned { item: u32 },
    /// An item with no title at all (Scrivener's Blank template starts the Draft with
    /// one), stored blank: the binder shows it untitled.
    UntitledItem { item: u32 },
    /// A container's own text, moved to a first child with its title (D3.6).
    OwnTextMoved { item: u32 },
    /// A text with documents under it, which became a folder (D3.6).
    TextWithChildren { item: u32 },
    /// An empty folder, kept as a plain folder rather than a chapter that would
    /// print a heading Scrivener never printed (D3.4).
    EmptyFolderPlain { item: u32 },
    /// A book inside a book, mapped as a part (D3.8).
    BookInsideBook { item: u32 },
    /// A prologue or epilogue chapter, left out of the chapter numbering.
    PrologueNotNumbered { item: u32 },
    /// Front or back matter from outside the Draft, placed inside the book and
    /// left out of exports until the writer turns it on (D3.11).
    MatterInsideBook { item: u32, back: bool },
    /// Draft documents that became front or back matter: their words no
    /// longer count as manuscript, so the word count differs from Scrivener's
    /// (D3.7).
    ParatextNotCounted { documents: u32, words: u64 },
    /// An item's document notes, now a note beside it (D3.10).
    DocumentNotes { item: u32 },
    /// With several books, the Draft's own text or synopsis, now a note at the
    /// top of the manuscript (D3.8).
    DraftContentAsNote { item: u32 },
    /// A character or place folder: its entries wear a story-bible tag named
    /// after it (D3.14). `template` when new entries start from its template.
    StoryBibleFolder { item: u32, template: bool },
    /// Scrivener's document templates, now note templates (D3.16).
    TemplatesCreated { count: u32 },
    /// A template folder nested in the Templates folder: each text in it is a
    /// template of its own.
    TemplateNestingFlattened { item: u32 },
    /// What a note template cannot hold (a synopsis, document notes, a picture),
    /// kept as a note.
    TemplateKeptAsNote { item: u32 },
    /// Texts found on disk that no binder item listed, in a folder of their own.
    RecoveredTexts { count: u32 },
    /// Format 1.x project notes, in a folder of their own (D3.15).
    ProjectNotes { count: u32 },
    /// The Trash, brought across as deleted items the Trash view can restore.
    TrashBroughtAcross { items: u32, with_writing: u32 },
    /// A manuscript item Scrivener left out of its compile: left out of exports
    /// too.
    NotCompiled { item: u32 },
    /// A picture or file inside the Draft: a note, since a book cannot hold it.
    MediaInDraft { item: u32 },

    // ------------------------------------------------ left behind
    /// A root folder the writer left out, and how many items it held.
    LeftOut { item: u32, items: u32 },
    /// The Trash, left out.
    TrashLeftOut { items: u32, with_writing: u32 },
    /// An index-card picture on a manuscript item: it would print in the book.
    CardImageLeftBehind { item: u32 },
    /// A folder's default document template that no story-bible tag can carry.
    DefaultTemplateNotLinked { item: u32 },
    /// "Page break before" on an item that does not open a book, part or
    /// chapter: no slot holds it.
    PageBreakNotKept { item: u32 },
    /// "Compile as-is", which has no counterpart.
    CompileAsIsNotKept { item: u32 },
}

impl StructureNotice {
    /// Every key a structure notice can have.
    pub const KEYS: &'static [&'static str] = &[
        "scrivener-structure-script-mode-draft",
        "scrivener-structure-one-document-manuscript",
        "scrivener-structure-title-repeats-number",
        "scrivener-structure-claim-overruled",
        "scrivener-structure-adopted-by-chapter",
        "scrivener-structure-looks-like-manuscript",
        "scrivener-structure-generic-sheet-title",
        "scrivener-structure-placeholder-title",
        "scrivener-structure-title-cleaned",
        "scrivener-structure-untitled-item",
        "scrivener-structure-own-text-moved",
        "scrivener-structure-text-with-children",
        "scrivener-structure-empty-folder-plain",
        "scrivener-structure-book-inside-book",
        "scrivener-structure-prologue-not-numbered",
        "scrivener-structure-matter-inside-book",
        "scrivener-structure-paratext-not-counted",
        "scrivener-structure-document-notes",
        "scrivener-structure-draft-content-as-note",
        "scrivener-structure-story-bible-folder",
        "scrivener-structure-templates-created",
        "scrivener-structure-template-nesting-flattened",
        "scrivener-structure-template-kept-as-note",
        "scrivener-structure-recovered-texts",
        "scrivener-structure-project-notes",
        "scrivener-structure-trash-brought-across",
        "scrivener-structure-not-compiled",
        "scrivener-structure-media-in-draft",
        "scrivener-structure-left-out",
        "scrivener-structure-trash-left-out",
        "scrivener-structure-card-image-left-behind",
        "scrivener-structure-default-template-not-linked",
        "scrivener-structure-page-break-not-kept",
        "scrivener-structure-compile-as-is-not-kept",
    ];

    /// The stable key the interface words this notice by.
    pub fn key(&self) -> &'static str {
        match self {
            StructureNotice::ScriptModeDraft { .. } => "scrivener-structure-script-mode-draft",
            StructureNotice::OneDocumentManuscript { .. } => {
                "scrivener-structure-one-document-manuscript"
            }
            StructureNotice::TitleRepeatsNumber { .. } => {
                "scrivener-structure-title-repeats-number"
            }
            StructureNotice::ClaimOverruled { .. } => "scrivener-structure-claim-overruled",
            StructureNotice::AdoptedByChapter { .. } => "scrivener-structure-adopted-by-chapter",
            StructureNotice::LooksLikeManuscript { .. } => {
                "scrivener-structure-looks-like-manuscript"
            }
            StructureNotice::GenericSheetTitle { .. } => "scrivener-structure-generic-sheet-title",
            StructureNotice::PlaceholderTitle { .. } => "scrivener-structure-placeholder-title",
            StructureNotice::UntitledItem { .. } => "scrivener-structure-untitled-item",
            StructureNotice::TitleCleaned { .. } => "scrivener-structure-title-cleaned",
            StructureNotice::OwnTextMoved { .. } => "scrivener-structure-own-text-moved",
            StructureNotice::TextWithChildren { .. } => "scrivener-structure-text-with-children",
            StructureNotice::EmptyFolderPlain { .. } => "scrivener-structure-empty-folder-plain",
            StructureNotice::BookInsideBook { .. } => "scrivener-structure-book-inside-book",
            StructureNotice::PrologueNotNumbered { .. } => {
                "scrivener-structure-prologue-not-numbered"
            }
            StructureNotice::MatterInsideBook { .. } => "scrivener-structure-matter-inside-book",
            StructureNotice::ParatextNotCounted { .. } => {
                "scrivener-structure-paratext-not-counted"
            }
            StructureNotice::DocumentNotes { .. } => "scrivener-structure-document-notes",
            StructureNotice::DraftContentAsNote { .. } => {
                "scrivener-structure-draft-content-as-note"
            }
            StructureNotice::StoryBibleFolder { .. } => "scrivener-structure-story-bible-folder",
            StructureNotice::TemplatesCreated { .. } => "scrivener-structure-templates-created",
            StructureNotice::TemplateNestingFlattened { .. } => {
                "scrivener-structure-template-nesting-flattened"
            }
            StructureNotice::TemplateKeptAsNote { .. } => {
                "scrivener-structure-template-kept-as-note"
            }
            StructureNotice::RecoveredTexts { .. } => "scrivener-structure-recovered-texts",
            StructureNotice::ProjectNotes { .. } => "scrivener-structure-project-notes",
            StructureNotice::TrashBroughtAcross { .. } => {
                "scrivener-structure-trash-brought-across"
            }
            StructureNotice::NotCompiled { .. } => "scrivener-structure-not-compiled",
            StructureNotice::MediaInDraft { .. } => "scrivener-structure-media-in-draft",
            StructureNotice::LeftOut { .. } => "scrivener-structure-left-out",
            StructureNotice::TrashLeftOut { .. } => "scrivener-structure-trash-left-out",
            StructureNotice::CardImageLeftBehind { .. } => {
                "scrivener-structure-card-image-left-behind"
            }
            StructureNotice::DefaultTemplateNotLinked { .. } => {
                "scrivener-structure-default-template-not-linked"
            }
            StructureNotice::PageBreakNotKept { .. } => "scrivener-structure-page-break-not-kept",
            StructureNotice::CompileAsIsNotKept { .. } => {
                "scrivener-structure-compile-as-is-not-kept"
            }
        }
    }

    /// Which group of the report the notice is listed in.
    pub fn group(&self) -> ReportGroup {
        match self {
            StructureNotice::ScriptModeDraft { .. }
            | StructureNotice::OneDocumentManuscript { .. }
            | StructureNotice::TitleRepeatsNumber { .. }
            | StructureNotice::ClaimOverruled { .. }
            | StructureNotice::AdoptedByChapter { .. }
            | StructureNotice::LooksLikeManuscript { .. }
            | StructureNotice::GenericSheetTitle { .. } => ReportGroup::CheckThese,
            StructureNotice::PlaceholderTitle { .. }
            | StructureNotice::UntitledItem { .. }
            | StructureNotice::TitleCleaned { .. }
            | StructureNotice::OwnTextMoved { .. }
            | StructureNotice::TextWithChildren { .. }
            | StructureNotice::EmptyFolderPlain { .. }
            | StructureNotice::BookInsideBook { .. }
            | StructureNotice::PrologueNotNumbered { .. }
            | StructureNotice::MatterInsideBook { .. }
            | StructureNotice::ParatextNotCounted { .. }
            | StructureNotice::DocumentNotes { .. }
            | StructureNotice::DraftContentAsNote { .. }
            | StructureNotice::StoryBibleFolder { .. }
            | StructureNotice::TemplatesCreated { .. }
            | StructureNotice::TemplateNestingFlattened { .. }
            | StructureNotice::TemplateKeptAsNote { .. }
            | StructureNotice::RecoveredTexts { .. }
            | StructureNotice::ProjectNotes { .. }
            | StructureNotice::TrashBroughtAcross { .. }
            | StructureNotice::NotCompiled { .. }
            | StructureNotice::MediaInDraft { .. } => ReportGroup::BroughtAcrossDifferently,
            StructureNotice::LeftOut { .. }
            | StructureNotice::TrashLeftOut { .. }
            | StructureNotice::CardImageLeftBehind { .. }
            | StructureNotice::DefaultTemplateNotLinked { .. }
            | StructureNotice::PageBreakNotKept { .. }
            | StructureNotice::CompileAsIsNotKept { .. } => ReportGroup::LeftBehind,
        }
    }

    /// The binder row the notice is about, when it is about one.
    pub fn item(&self) -> Option<u32> {
        match self {
            StructureNotice::OneDocumentManuscript { item, .. }
            | StructureNotice::TitleRepeatsNumber { item }
            | StructureNotice::ClaimOverruled { item, .. }
            | StructureNotice::AdoptedByChapter { item, .. }
            | StructureNotice::LooksLikeManuscript { item }
            | StructureNotice::GenericSheetTitle { item }
            | StructureNotice::PlaceholderTitle { item }
            | StructureNotice::UntitledItem { item }
            | StructureNotice::TitleCleaned { item }
            | StructureNotice::OwnTextMoved { item }
            | StructureNotice::TextWithChildren { item }
            | StructureNotice::EmptyFolderPlain { item }
            | StructureNotice::BookInsideBook { item }
            | StructureNotice::PrologueNotNumbered { item }
            | StructureNotice::MatterInsideBook { item, .. }
            | StructureNotice::DocumentNotes { item }
            | StructureNotice::DraftContentAsNote { item }
            | StructureNotice::StoryBibleFolder { item, .. }
            | StructureNotice::TemplateNestingFlattened { item }
            | StructureNotice::TemplateKeptAsNote { item }
            | StructureNotice::NotCompiled { item }
            | StructureNotice::MediaInDraft { item }
            | StructureNotice::LeftOut { item, .. }
            | StructureNotice::CardImageLeftBehind { item }
            | StructureNotice::DefaultTemplateNotLinked { item }
            | StructureNotice::PageBreakNotKept { item }
            | StructureNotice::CompileAsIsNotKept { item } => Some(*item),
            StructureNotice::ScriptModeDraft { .. }
            | StructureNotice::ParatextNotCounted { .. }
            | StructureNotice::TemplatesCreated { .. }
            | StructureNotice::RecoveredTexts { .. }
            | StructureNotice::ProjectNotes { .. }
            | StructureNotice::TrashBroughtAcross { .. }
            | StructureNotice::TrashLeftOut { .. } => None,
        }
    }
}
