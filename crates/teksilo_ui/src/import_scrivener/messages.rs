// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Scrivener importer says, in the writer's language.
//!
//! Not a view-model: the one table from the converter's typed findings to Fluent (D7-12).
//! Every key the analysis can give (a refusal, a question, a source notice, a structure
//! notice, a metadata notice; `import_management::scrivener::every_key`) has a row in
//! [`TABLE`] saying where the wizard shows it ([`Tier`]) and which group of the import's
//! report it belongs to ([`Group`]), and an arm below that words it through `tr!`, so each
//! sentence is compile-checked against `en-US`. The tests resolve every one in both shipped
//! locales, since `fr-FR` is only checked by resolving it, and pin the table against the
//! converter's key list both ways, so a new converter key cannot ship unworded.
//!
//! The prose notices (`scrivener-prose-*`) are the import report's alone: the analysis never
//! gives one, so their rows say [`Tier::Report`]. A finding of the document importer's own
//! (`ScrivenerFindingFamily::Document`) keeps that importer's key and is worded by its
//! sentences ([`document_line`]).
//!
//! The data a sentence quotes stays as the project wrote it: a file or folder named as the
//! project names it, an item by its path in Scrivener's binder, what a reader or the system
//! said. A count is passed as a number, so its plural form follows the language.

use frontend::import_management::failure::FailureReason;
use frontend::import_management::{
    ScrivenerArg, ScrivenerComputers, ScrivenerFinding, ScrivenerFindingFamily,
    ScrivenerManifestCandidate, ScrivenerPlace, ScrivenerQuestion, ScrivenerReportGroup,
};
use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;

/// Where the wizard shows a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tier {
    /// The project cannot be imported: the Project page's error banner.
    Refusal,
    /// A question answered by choosing: a radio group on the Project page.
    Choose,
    /// A question answered by accepting: a checkbox that gates Next.
    Accept,
    /// Something to know about the project: an information banner on the Project page.
    Card,
    /// Something about what the project states (title, author, language): a hint on the
    /// Save page, beside the field it is about.
    SaveHint,
    /// Something the import does with the binder or the metadata, or leaves behind: a line
    /// in the Review page's "before you import" strip, and again in the import's report.
    BeforeImport,
    /// Said only after the import, in its report: the prose, and what became of the labels,
    /// the status list and the dates.
    Report,
}

/// The four fixed groups of the import report, in their order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Group {
    CouldNotBeRead,
    CheckThese,
    BroughtAcrossDifferently,
    LeftBehind,
}

impl Group {
    /// Whether a finding in this group counts towards the headline of the result: only
    /// what could not be read and what the writer should check (Q7). What came across
    /// differently or stayed behind is in the report, and raises no alarm.
    pub(crate) fn counts_in_headline(self) -> bool {
        matches!(self, Group::CouldNotBeRead | Group::CheckThese)
    }

    /// The group the converter filed a finding under, when it filed one.
    pub(crate) fn of_dto(group: &ScrivenerReportGroup) -> Option<Group> {
        match group {
            ScrivenerReportGroup::Unfiled => None,
            ScrivenerReportGroup::CouldNotBeRead => Some(Group::CouldNotBeRead),
            ScrivenerReportGroup::CheckThese => Some(Group::CheckThese),
            ScrivenerReportGroup::BroughtAcrossDifferently => Some(Group::BroughtAcrossDifferently),
            ScrivenerReportGroup::LeftBehind => Some(Group::LeftBehind),
        }
    }
}

/// One key: where it is shown, and its report group (none for a refusal or a question,
/// which end an import rather than describe one).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Entry {
    pub(crate) key: &'static str,
    pub(crate) tier: Tier,
    pub(crate) group: Option<Group>,
}

const fn e(key: &'static str, tier: Tier, group: Option<Group>) -> Entry {
    Entry { key, tier, group }
}

use Group::{
    BroughtAcrossDifferently as Differently, CheckThese as Check, CouldNotBeRead as Unread,
    LeftBehind as Left,
};
use Tier::{Accept, BeforeImport, Card, Choose, Refusal, Report, SaveHint};

/// Every key the wizard words, with where it goes.
pub(crate) const TABLE: &[Entry] = &[
    // ── refusals ─────────────────────────────────────────────────────────────────────────
    e("scrivener-refusal-missing", Refusal, None),
    e("scrivener-refusal-unreadable", Refusal, None),
    e("scrivener-refusal-not-a-scrivener-project", Refusal, None),
    e("scrivener-refusal-manifest-without-files", Refusal, None),
    e("scrivener-refusal-search-too-wide", Refusal, None),
    e("scrivener-refusal-scrivener-one-for-mac", Refusal, None),
    e(
        "scrivener-refusal-ios-stub-without-mobile-data",
        Refusal,
        None,
    ),
    e("scrivener-refusal-unknown-format-version", Refusal, None),
    e("scrivener-refusal-newer-format-stamp", Refusal, None),
    e("scrivener-refusal-version-stamp-unreadable", Refusal, None),
    e("scrivener-refusal-damaged-manifest", Refusal, None),
    e("scrivener-refusal-manifest-too-large", Refusal, None),
    e("scrivener-refusal-no-item-identifiers", Refusal, None),
    e("scrivener-refusal-incomplete-copy", Refusal, None),
    e("scrivener-refusal-archive-unreadable", Refusal, None),
    e("scrivener-refusal-archive-unsafe-name", Refusal, None),
    e("scrivener-refusal-archive-too-many-members", Refusal, None),
    e("scrivener-refusal-archive-member-too-large", Refusal, None),
    e("scrivener-refusal-archive-too-large", Refusal, None),
    e("scrivener-refusal-archive-encrypted", Refusal, None),
    e(
        "scrivener-refusal-archive-unsupported-compression",
        Refusal,
        None,
    ),
    e("scrivener-refusal-duplicate-archive-members", Refusal, None),
    e("scrivener-refusal-template-undecodable", Refusal, None),
    e("scrivener-refusal-cloud-placeholders", Refusal, None),
    e("scrivener-refusal-unreadable-members", Refusal, None),
    e("scrivener-refusal-needed-member-is-link", Refusal, None),
    e(
        "scrivener-refusal-project-changed-during-read",
        Refusal,
        None,
    ),
    e("scrivener-refusal-too-many-members", Refusal, None),
    e("scrivener-refusal-too-large", Refusal, None),
    e("scrivener-refusal-structure-unmappable", Refusal, None),
    // ── questions ────────────────────────────────────────────────────────────────────────
    e("scrivener-question-several-projects", Choose, None),
    e("scrivener-question-several-manifests", Choose, None),
    e("scrivener-question-enclosing-project", Choose, None),
    e("scrivener-question-upgraded-sibling", Choose, None),
    e("scrivener-question-unmerged-mobile-edits", Accept, None),
    e("scrivener-question-no-text-in-copy", Accept, None),
    // ── what the analysis found ──────────────────────────────────────────────────────────
    e(
        "scrivener-notice-project-found-below-pick",
        Card,
        Some(Differently),
    ),
    e(
        "scrivener-notice-source-is-template",
        Card,
        Some(Differently),
    ),
    e("scrivener-notice-source-is-backup", Card, Some(Check)),
    e(
        "scrivener-notice-importing-pre-upgrade-copy",
        Card,
        Some(Check),
    ),
    e(
        "scrivener-notice-imported-from-ios-stub",
        Card,
        Some(Differently),
    ),
    e(
        "scrivener-notice-stale-mobile-binder-ignored",
        Card,
        Some(Left),
    ),
    e(
        "scrivener-notice-identical-manifest-copies",
        Card,
        Some(Differently),
    ),
    e("scrivener-notice-manifest-copies-ignored", Card, Some(Left)),
    e(
        "scrivener-notice-pinned-manifest-looks-like-a-copy",
        Card,
        Some(Check),
    ),
    e(
        "scrivener-notice-pinned-manifest-has-siblings",
        Card,
        Some(Left),
    ),
    e("scrivener-notice-legacy-v1-files-beside", Card, Some(Left)),
    e(
        "scrivener-notice-mixed-payload-trees",
        Card,
        Some(Differently),
    ),
    e(
        "scrivener-notice-id-scheme-mismatch",
        Card,
        Some(Differently),
    ),
    e(
        "scrivener-notice-version-stamp-mismatch",
        Card,
        Some(Differently),
    ),
    e("scrivener-notice-unrecognised-writer", Card, Some(Check)),
    e("scrivener-notice-invalid-item-identifier", Card, Some(Left)),
    e(
        "scrivener-notice-items-without-identifier",
        Card,
        Some(Unread),
    ),
    e("scrivener-notice-duplicate-item-key", Card, Some(Check)),
    e("scrivener-notice-lock-present", Card, Some(Check)),
    e("scrivener-notice-archived-while-open", Card, Some(Check)),
    e("scrivener-notice-content-missing", Card, Some(Unread)),
    e("scrivener-notice-content-truncated", Card, Some(Unread)),
    e("scrivener-notice-missing-media", Card, Some(Unread)),
    e("scrivener-notice-sync-conflict-copies", Card, Some(Check)),
    e(
        "scrivener-notice-recovered-unreferenced-text",
        Card,
        Some(Check),
    ),
    e("scrivener-notice-sync-debris-ignored", Card, Some(Left)),
    e("scrivener-notice-unrecognised-files", Card, Some(Left)),
    e("scrivener-notice-symlink-not-followed", Card, Some(Left)),
    e(
        "scrivener-notice-unreadable-files-skipped",
        Card,
        Some(Left),
    ),
    e(
        "scrivener-notice-cloud-placeholders-skipped",
        Card,
        Some(Left),
    ),
    e("scrivener-notice-no-text-found", Card, Some(Check)),
    e("scrivener-notice-metadata-unreadable", Card, Some(Unread)),
    e("scrivener-notice-dangling-references", Card, Some(Left)),
    e(
        "scrivener-notice-snapshots-not-imported",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-notice-writing-history-not-imported",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-notice-title-placeholder-rejected",
        SaveHint,
        Some(Left),
    ),
    e(
        "scrivener-notice-author-placeholder-rejected",
        SaveHint,
        Some(Left),
    ),
    e(
        "scrivener-notice-title-from-project",
        SaveHint,
        Some(Differently),
    ),
    e(
        "scrivener-notice-author-from-project",
        SaveHint,
        Some(Check),
    ),
    e(
        "scrivener-notice-language-not-stated",
        SaveHint,
        Some(Differently),
    ),
    e(
        "scrivener-notice-language-from-script",
        SaveHint,
        Some(Differently),
    ),
    // ── the structure mapping ────────────────────────────────────────────────────────────
    e(
        "scrivener-structure-script-mode-draft",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-one-document-manuscript",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-title-repeats-number",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-claim-overruled",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-adopted-by-chapter",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-looks-like-manuscript",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-generic-sheet-title",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-structure-placeholder-title",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-untitled-item",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-title-cleaned",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-own-text-moved",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-text-with-children",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-empty-folder-plain",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-book-inside-book",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-prologue-not-numbered",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-matter-inside-book",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-paratext-not-counted",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-document-notes",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-draft-content-as-note",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-story-bible-folder",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-templates-created",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-template-nesting-flattened",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-template-kept-as-note",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-recovered-texts",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-project-notes",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-trash-brought-across",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-not-compiled",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-structure-media-in-draft",
        BeforeImport,
        Some(Differently),
    ),
    e("scrivener-structure-left-out", BeforeImport, Some(Left)),
    e(
        "scrivener-structure-trash-left-out",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-structure-card-image-left-behind",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-structure-default-template-not-linked",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-structure-page-break-not-kept",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-structure-compile-as-is-not-kept",
        BeforeImport,
        Some(Left),
    ),
    // ── the labels, the status list and the dates: the import's own ──────────────────────
    e(
        "scrivener-metadata-status-categories-guessed",
        Report,
        Some(Check),
    ),
    e("scrivener-metadata-dates-unreadable", Report, Some(Check)),
    e(
        "scrivener-metadata-labels-became-tags",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-statuses-became-ladder",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-ladder-seeded",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-field-renamed",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-none-value-collapsed",
        Report,
        Some(Differently),
    ),
    // A keyword with no name is foreseen before the import (below); a label or a status
    // with none is said by the import alone.
    e(
        "scrivener-metadata-unnamed-value",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-tag-name-disambiguated",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-rung-names-disambiguated",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-unused-labels-skipped",
        Report,
        Some(Left),
    ),
    e(
        "scrivener-metadata-values-without-row",
        BeforeImport,
        Some(Left),
    ),
    // ── the keywords, custom metadata, collections, bookmarks, targets and the rest ──────
    // The Review page foresees every one of these from the plan it shows (D7-5), and the
    // import says them again in its report: their sentences read the same before and
    // after.
    e("scrivener-metadata-pov-hint", BeforeImport, Some(Check)),
    e(
        "scrivener-metadata-checkbox-default-unknown",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-metadata-goals-in-other-unit",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-metadata-counting-differs",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-metadata-writing-days-not-read",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-metadata-script-mode-rows",
        BeforeImport,
        Some(Check),
    ),
    e(
        "scrivener-metadata-keywords-became-tags",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-keyword-hierarchy-flattened",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-fields-became-tags",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-collections-became-tags",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-collection-order-lost",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-bookmarks-became-links",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-goals-kept",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-draft-target-kept",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-pace-planned",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-dictionary-words",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-listed-in-import-note",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-targets-on-notes",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-targets-on-split-documents",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-bookmark-ends-without-row",
        BeforeImport,
        Some(Differently),
    ),
    e(
        "scrivener-metadata-unused-keywords-skipped",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-list-value-invalid",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-undefined-custom-field",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-collection-members-missing",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-empty-collections-skipped",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-bookmarks-unresolved",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-targets-in-pages",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-targets-unreadable",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-draft-target-not-kept",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-deadline-not-planned",
        BeforeImport,
        Some(Left),
    ),
    e(
        "scrivener-metadata-session-target-not-imported",
        BeforeImport,
        Some(Left),
    ),
    e("scrivener-metadata-icons-dropped", BeforeImport, Some(Left)),
    // ── the prose: the import's own, said only in its report ─────────────────────────────
    // The converter files each of these itself (`group_of` reads its filing first); the
    // groups here are the same, for a test to hold every row to one.
    e("scrivener-prose-unreadable", Report, Some(Unread)),
    e("scrivener-prose-comments-unreadable", Report, Some(Unread)),
    e("scrivener-prose-read-as-plain-text", Report, Some(Check)),
    e("scrivener-prose-inline-note-unmatched", Report, Some(Check)),
    e("scrivener-prose-style-unresolved", Report, Some(Check)),
    e("scrivener-prose-styles-lost-in-export", Report, Some(Check)),
    e(
        "scrivener-prose-comment-anchor-without-entry",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-comment-entry-without-anchor",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-footnote-entry-without-anchor",
        Report,
        Some(Check),
    ),
    e("scrivener-prose-range-comment-reading", Report, Some(Check)),
    e(
        "scrivener-prose-range-comment-approximate",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-document-link-unresolved",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-linked-picture-not-read",
        Report,
        Some(Check),
    ),
    e("scrivener-prose-heading-repeats-title", Report, Some(Check)),
    e("scrivener-prose-markdown-as-text", Report, Some(Check)),
    e("scrivener-prose-citation-keys-as-text", Report, Some(Check)),
    e(
        "scrivener-prose-placeholders-in-exported-text",
        Report,
        Some(Check),
    ),
    e("scrivener-prose-ornament-lines-kept", Report, Some(Check)),
    e("scrivener-prose-long-document", Report, Some(Check)),
    e(
        "scrivener-prose-pictures-without-description",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-struck-text-compiled-away",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-compile-deleted-style-printed",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-compile-raw-style-printed",
        Report,
        Some(Check),
    ),
    e("scrivener-prose-left-out", Report, Some(Check)),
    e(
        "scrivener-prose-undecodable-characters",
        Report,
        Some(Check),
    ),
    e(
        "scrivener-prose-placeholders-in-other-text",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-inline-notes-merged",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-inline-notes-joined-across-paragraphs",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-blank-notes-dropped",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-annotations-became-comments",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-annotation-only-document-kept",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-preserve-formatting-passages",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-nested-note-unwrapped",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-style-not-carried",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-document-links-dropped",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-document-links-kept",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-other-links-kept-as-text",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-typed-indents-removed",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-left-indents-not-kept",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-blank-lines-removed",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-blank-lines-became-scene-breaks",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-ornaments-became-scene-breaks",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-table-first-row-header",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-table-cells-joined",
        Report,
        Some(Differently),
    ),
    e("scrivener-prose-formatting-lost", Report, Some(Differently)),
    e(
        "scrivener-prose-justified-text-not-kept",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-comment-colours-lost",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-control-characters-removed",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-note-on-heading-lost",
        Report,
        Some(Differently),
    ),
    e(
        "scrivener-prose-media-file-copied",
        Report,
        Some(Differently),
    ),
    e("scrivener-prose-media-file-not-carried", Report, Some(Left)),
];

/// The row of `key`, if the wizard words it.
pub(crate) fn entry(key: &str) -> Option<&'static Entry> {
    TABLE.iter().find(|e| e.key == key)
}

/// The report group a finding belongs to: the converter's own filing when it made one,
/// else this table's (the analysis leaves its source notices unfiled, for the interface
/// to place). `None` for a key the table does not know.
pub(crate) fn group_of(finding: &ScrivenerFinding) -> Option<Group> {
    let ScrivenerFinding::Reported {
        key, group, args, ..
    } = finding
    else {
        return None;
    };
    Group::of_dto(group).or_else(|| {
        // A missing text whose words Scrivener's search index still held came across, as
        // plain text and perhaps older: something to check, not something lost.
        if key == "scrivener-notice-content-missing" && flag(args, "salvaged") {
            return Some(Group::CheckThese);
        }
        entry(key).and_then(|e| e.group)
    })
}

/// The key of a finding, or `""` for the generator's empty variant.
pub(crate) fn key_of(finding: &ScrivenerFinding) -> &str {
    match finding {
        ScrivenerFinding::Reported { key, .. } => key,
        ScrivenerFinding::Empty => "",
    }
}

// ── reading a finding's arguments ────────────────────────────────────────────────────────

fn args_of(finding: &ScrivenerFinding) -> &[ScrivenerArg] {
    match finding {
        ScrivenerFinding::Reported { args, .. } => args,
        ScrivenerFinding::Empty => &[],
    }
}

/// A count argument, or 0 when absent.
fn num(args: &[ScrivenerArg], wanted: &str) -> i64 {
    args.iter()
        .find_map(|a| match a {
            ScrivenerArg::Count { name, value } if name == wanted => Some(*value),
            _ => None,
        })
        .unwrap_or(0)
}

/// A text, token or day argument, or `None` when absent.
fn opt_text(args: &[ScrivenerArg], wanted: &str) -> Option<String> {
    args.iter().find_map(|a| match a {
        ScrivenerArg::Text { name, value }
        | ScrivenerArg::Token { name, value }
        | ScrivenerArg::Day { name, value }
            if name == wanted =>
        {
            Some(value.clone())
        }
        _ => None,
    })
}

/// A text, token or day argument, or empty when absent.
fn txt(args: &[ScrivenerArg], wanted: &str) -> String {
    opt_text(args, wanted).unwrap_or_default()
}

/// A list argument.
fn list(args: &[ScrivenerArg], wanted: &str) -> Vec<String> {
    args.iter()
        .find_map(|a| match a {
            ScrivenerArg::Texts { name, values } if name == wanted => Some(values.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

/// A flag argument, or `false` when absent.
fn flag(args: &[ScrivenerArg], wanted: &str) -> bool {
    args.iter()
        .any(|a| matches!(a, ScrivenerArg::Flag { name, value } if name == wanted && *value))
}

/// A few of a list's values, as the project names them, for a sentence to quote.
fn examples(args: &[ScrivenerArg]) -> String {
    list(args, "examples")
        .into_iter()
        .take(3)
        .collect::<Vec<_>>()
        .join(", ")
}

/// An item as the writer knew it in Scrivener: its path in the binder.
pub(crate) fn binder_path(path: &[String]) -> String {
    path.join(" ▸ ")
}

/// Where a finding happened, as the item's path, or empty for the project as a whole.
fn item_of(finding: &ScrivenerFinding) -> String {
    match finding {
        ScrivenerFinding::Reported { binder_path: p, .. } => binder_path(p),
        ScrivenerFinding::Empty => String::new(),
    }
}

/// Which of an item's writings a finding is about.
enum WritingPlace {
    Text,
    Notes,
    Synopsis,
}

fn place_token(finding: &ScrivenerFinding) -> WritingPlace {
    match finding {
        ScrivenerFinding::Reported {
            place: ScrivenerPlace::Notes,
            ..
        } => WritingPlace::Notes,
        ScrivenerFinding::Reported {
            place: ScrivenerPlace::Synopsis,
            ..
        } => WritingPlace::Synopsis,
        _ => WritingPlace::Text,
    }
}

/// Which of an item's writings a finding is about, as the token its body selects on: the
/// notes, the synopsis or the text each take their own pronouns.
fn place_name(finding: &ScrivenerFinding) -> &'static str {
    match place_token(finding) {
        WritingPlace::Notes => "notes",
        WritingPlace::Synopsis => "synopsis",
        WritingPlace::Text => "text",
    }
}

/// A finding as a banner says it: a title, an optional body, and how it reads.
#[derive(Clone)]
pub(crate) struct Worded {
    pub(crate) title: LocalizedString,
    pub(crate) body: Option<LocalizedString>,
    /// A warning rather than an information: something on this computer, or text lost.
    pub(crate) warning: bool,
    /// The refusal's cause may go away by itself (a sync finishing, a download): the page
    /// offers **Read the project again**.
    pub(crate) retry: bool,
    /// The technical account of a defect, for **Details**.
    pub(crate) details: Option<String>,
}

impl Worded {
    fn new(title: LocalizedString, body: LocalizedString) -> Self {
        Self {
            title,
            body: Some(body),
            warning: false,
            retry: false,
            details: None,
        }
    }

    fn retry(mut self) -> Self {
        self.retry = true;
        self
    }

    fn warning(mut self, warning: bool) -> Self {
        self.warning = warning;
        self
    }
}

// ── refusals ─────────────────────────────────────────────────────────────────────────────

/// Why the project cannot be imported, worded; `None` for a key this table does not know.
pub(crate) fn refusal(finding: &ScrivenerFinding) -> Option<Worded> {
    let a = args_of(finding);
    let w = |title: LocalizedString, body: LocalizedString| Some(Worded::new(title, body));
    match key_of(finding) {
        "scrivener-refusal-missing" => w(
            tr!(import_scrivener_refusal_missing_title()),
            tr!(import_scrivener_refusal_missing_body(file = txt(a, "file"))),
        ),
        "scrivener-refusal-unreadable" => w(
            tr!(import_scrivener_refusal_unreadable_title()),
            tr!(import_scrivener_refusal_unreadable_body(
                file = txt(a, "file"),
                reason = txt(a, "reason")
            )),
        ),
        "scrivener-refusal-not-a-scrivener-project" => w(
            tr!(import_scrivener_refusal_not_a_scrivener_project_title()),
            tr!(import_scrivener_refusal_not_a_scrivener_project_body()),
        ),
        "scrivener-refusal-manifest-without-files" => w(
            tr!(import_scrivener_refusal_manifest_without_files_title()),
            tr!(import_scrivener_refusal_manifest_without_files_body(
                manifest = txt(a, "manifest")
            )),
        )
        .map(Worded::retry),
        "scrivener-refusal-search-too-wide" => w(
            tr!(import_scrivener_refusal_search_too_wide_title()),
            tr!(import_scrivener_refusal_search_too_wide_body(
                limit = num(a, "limit")
            )),
        ),
        "scrivener-refusal-scrivener-one-for-mac" => w(
            tr!(import_scrivener_refusal_scrivener_one_for_mac_title()),
            tr!(import_scrivener_refusal_scrivener_one_for_mac_body()),
        ),
        "scrivener-refusal-ios-stub-without-mobile-data" => w(
            tr!(import_scrivener_refusal_ios_stub_without_mobile_data_title()),
            tr!(import_scrivener_refusal_ios_stub_without_mobile_data_body()),
        ),
        "scrivener-refusal-unknown-format-version" => w(
            tr!(import_scrivener_refusal_newer_title()),
            tr!(import_scrivener_refusal_unknown_format_version_body(
                version = txt(a, "version")
            )),
        ),
        "scrivener-refusal-newer-format-stamp" => w(
            tr!(import_scrivener_refusal_newer_title()),
            tr!(import_scrivener_refusal_newer_format_stamp_body(
                stamp = num(a, "stamp"),
                newest = num(a, "newest")
            )),
        ),
        "scrivener-refusal-version-stamp-unreadable" => w(
            tr!(import_scrivener_refusal_version_stamp_unreadable_title()),
            tr!(import_scrivener_refusal_version_stamp_unreadable_body()),
        ),
        "scrivener-refusal-damaged-manifest" => w(
            tr!(import_scrivener_refusal_damaged_manifest_title()),
            // The empty file Scrivener writes for a project it has not saved yet is not
            // damage, and Scrivener has nothing in it to repair.
            match txt(a, "damage").as_str() {
                "new-project-placeholder" => tr!(
                    import_scrivener_refusal_damaged_manifest_new_project_placeholder_body(
                        manifest = txt(a, "manifest")
                    )
                ),
                _ => tr!(import_scrivener_refusal_damaged_manifest_body(
                    manifest = txt(a, "manifest"),
                    spare_copies = num(a, "spare_copies")
                )),
            },
        ),
        "scrivener-refusal-manifest-too-large" => w(
            tr!(import_scrivener_refusal_manifest_too_large_title()),
            tr!(import_scrivener_refusal_manifest_too_large_body(
                manifest = txt(a, "manifest"),
                limit = num(a, "limit_mib")
            )),
        ),
        "scrivener-refusal-no-item-identifiers" => w(
            tr!(import_scrivener_refusal_no_item_identifiers_title()),
            tr!(import_scrivener_refusal_no_item_identifiers_body()),
        ),
        "scrivener-refusal-incomplete-copy" => w(
            tr!(import_scrivener_refusal_incomplete_copy_title()),
            tr!(import_scrivener_refusal_incomplete_copy_body()),
        )
        .map(Worded::retry),
        "scrivener-refusal-archive-unreadable" => w(
            tr!(import_scrivener_refusal_archive_title()),
            tr!(import_scrivener_refusal_archive_unreadable_body(
                file = txt(a, "file"),
                reason = txt(a, "reason")
            )),
        ),
        "scrivener-refusal-archive-unsafe-name" => w(
            tr!(import_scrivener_refusal_archive_unsafe_name_title()),
            // Only a name with `..` in it, or an absolute one, would land outside.
            match txt(a, "why").as_str() {
                "empty" => tr!(import_scrivener_refusal_archive_unsafe_name_empty_body(
                    file = txt(a, "file")
                )),
                "control-character" => tr!(
                    import_scrivener_refusal_archive_unsafe_name_control_character_body(
                        file = txt(a, "file")
                    )
                ),
                _ => tr!(import_scrivener_refusal_archive_unsafe_name_body(
                    file = txt(a, "file"),
                    member = txt(a, "member")
                )),
            },
        ),
        "scrivener-refusal-archive-too-many-members" => w(
            tr!(import_scrivener_refusal_archive_too_large_title()),
            tr!(import_scrivener_refusal_archive_too_many_members_body(
                limit = num(a, "limit")
            )),
        ),
        "scrivener-refusal-archive-member-too-large" => w(
            tr!(import_scrivener_refusal_archive_too_large_title()),
            tr!(import_scrivener_refusal_archive_member_too_large_body(
                member = txt(a, "member"),
                limit = num(a, "limit_mib")
            )),
        ),
        "scrivener-refusal-archive-too-large" => w(
            tr!(import_scrivener_refusal_archive_too_large_title()),
            tr!(import_scrivener_refusal_archive_too_large_body(
                limit = num(a, "limit_mib")
            )),
        ),
        "scrivener-refusal-archive-encrypted" => w(
            tr!(import_scrivener_refusal_archive_encrypted_title()),
            tr!(import_scrivener_refusal_archive_encrypted_body(
                file = txt(a, "file")
            )),
        ),
        "scrivener-refusal-archive-unsupported-compression" => w(
            tr!(import_scrivener_refusal_archive_unsupported_compression_title()),
            tr!(
                import_scrivener_refusal_archive_unsupported_compression_body(
                    member = txt(a, "member"),
                    method = txt(a, "method")
                )
            ),
        ),
        "scrivener-refusal-duplicate-archive-members" => w(
            tr!(import_scrivener_refusal_duplicate_archive_members_title()),
            tr!(import_scrivener_refusal_duplicate_archive_members_body(
                file = txt(a, "file")
            )),
        ),
        // A file too large, one that is not a template and one encoded another way each
        // say so; the rest is what a file cut short looks like.
        "scrivener-refusal-template-undecodable" => {
            let file = txt(a, "file");
            match txt(a, "damage").as_str() {
                "too-large" => w(
                    tr!(import_scrivener_refusal_template_undecodable_too_large_title()),
                    tr!(
                        import_scrivener_refusal_template_undecodable_too_large_body(
                            file = file,
                            limit = num(a, "limit_mib")
                        )
                    ),
                ),
                "not-a-template" => w(
                    tr!(import_scrivener_refusal_template_undecodable_not_a_template_title()),
                    tr!(
                        import_scrivener_refusal_template_undecodable_not_a_template_body(
                            file = file
                        )
                    ),
                ),
                "unknown-encoding" => w(
                    tr!(import_scrivener_refusal_template_undecodable_unknown_encoding_title()),
                    tr!(
                        import_scrivener_refusal_template_undecodable_unknown_encoding_body(
                            file = file
                        )
                    ),
                ),
                _ => w(
                    tr!(import_scrivener_refusal_template_undecodable_title()),
                    tr!(import_scrivener_refusal_template_undecodable_body(
                        file = file
                    )),
                ),
            }
        }
        "scrivener-refusal-cloud-placeholders" => w(
            tr!(import_scrivener_refusal_cloud_placeholders_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_refusal_cloud_placeholders_body(
                count = num(a, "count"),
                examples = examples(a)
            )),
        )
        .map(Worded::retry),
        "scrivener-refusal-unreadable-members" => w(
            tr!(import_scrivener_refusal_unreadable_members_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_refusal_unreadable_members_body(
                examples = examples(a)
            )),
        )
        .map(Worded::retry),
        "scrivener-refusal-needed-member-is-link" => w(
            tr!(import_scrivener_refusal_needed_member_is_link_title()),
            tr!(import_scrivener_refusal_needed_member_is_link_body(
                member = txt(a, "member")
            )),
        ),
        "scrivener-refusal-project-changed-during-read" => w(
            tr!(import_scrivener_refusal_project_changed_during_read_title()),
            tr!(import_scrivener_refusal_project_changed_during_read_body()),
        )
        .map(Worded::retry),
        "scrivener-refusal-too-many-members" => w(
            tr!(import_scrivener_refusal_too_many_members_title()),
            tr!(import_scrivener_refusal_too_many_members_body(
                limit = num(a, "limit")
            )),
        ),
        "scrivener-refusal-too-large" => w(
            tr!(import_scrivener_refusal_too_large_title()),
            tr!(import_scrivener_refusal_too_large_body(
                limit = num(a, "limit_mib")
            )),
        ),
        "scrivener-refusal-structure-unmappable" => {
            let mut worded = Worded::new(
                tr!(import_scrivener_refusal_structure_unmappable_title()),
                tr!(import_scrivener_refusal_structure_unmappable_body()),
            );
            worded.details = Some(txt(a, "detail"));
            Some(worded)
        }
        _ => None,
    }
}

/// A refusal as one paragraph: its title and its body. For a failure notice, which has a
/// single line to say it in.
pub(crate) fn refusal_paragraph(finding: &ScrivenerFinding) -> Option<LocalizedString> {
    let worded = refusal(finding)?;
    Some(localized(move || {
        let title = worded.title.resolve_now();
        match &worded.body {
            Some(body) => tr!(import_scrivener_paragraph(
                title = title,
                body = body.resolve_now()
            ))
            .resolve_now(),
            None => title,
        }
    }))
}

// ── questions ────────────────────────────────────────────────────────────────────────────

/// A question as the Project page asks it.
#[derive(Clone)]
pub(crate) struct Asked {
    pub(crate) title: LocalizedString,
    pub(crate) body: LocalizedString,
    /// The checkbox's label, for a question answered by accepting.
    pub(crate) accept: Option<LocalizedString>,
    /// One label per answer, in the order the question lists them, for a question answered
    /// by choosing.
    pub(crate) options: Vec<LocalizedString>,
}

/// A manifest candidate as a radio row: its name, when it was saved, how many items it
/// lists.
fn manifest_row(candidate: &ScrivenerManifestCandidate) -> LocalizedString {
    let ScrivenerManifestCandidate::Candidate {
        name,
        modified,
        items,
        ..
    } = candidate
    else {
        return lit!("");
    };
    let date = modified.and_then(|m| chrono::DateTime::from_timestamp(m, 0));
    match (date, items) {
        (Some(date), Some(items)) => tr!(import_scrivener_manifest_row(
            file = name.clone(),
            date = crate::shared::stamps::iso_stamp(date),
            documents = *items
        )),
        (None, Some(items)) => tr!(import_scrivener_manifest_row_undated(
            file = name.clone(),
            documents = *items
        )),
        (_, None) => tr!(import_scrivener_manifest_row_unread(file = name.clone())),
    }
}

/// How the Project page asks `question`; `None` for the generator's empty variant.
pub(crate) fn question(question: &ScrivenerQuestion) -> Option<Asked> {
    Some(match question {
        ScrivenerQuestion::Empty => return None,
        ScrivenerQuestion::SeveralProjects { candidates } => Asked {
            title: tr!(import_scrivener_roots_title(
                count = candidates.len() as i64
            )),
            body: tr!(import_scrivener_roots_hint()),
            accept: None,
            options: candidates.iter().map(|c| lit!(c.clone())).collect(),
        },
        ScrivenerQuestion::SeveralManifests {
            candidates,
            preselected,
            ..
        } => Asked {
            title: tr!(import_scrivener_manifests_title(
                count = candidates.len() as i64
            )),
            body: if preselected.is_some() {
                tr!(import_scrivener_manifests_hint_preselected())
            } else {
                tr!(import_scrivener_manifests_hint())
            },
            accept: None,
            options: candidates.iter().map(manifest_row).collect(),
        },
        ScrivenerQuestion::EnclosingProject { project, .. } => Asked {
            title: tr!(import_scrivener_enclosing_title(project = project.clone())),
            body: tr!(import_scrivener_enclosing_body()),
            accept: None,
            options: vec![tr!(import_scrivener_enclosing_use(
                project = project.clone()
            ))],
        },
        ScrivenerQuestion::UpgradedSibling {
            backup,
            sibling,
            preselect_sibling,
            ..
        } => Asked {
            title: tr!(import_scrivener_upgrade_title()),
            body: if *preselect_sibling {
                tr!(import_scrivener_upgrade_body(
                    backup = backup.clone(),
                    current = sibling.clone()
                ))
            } else {
                tr!(import_scrivener_upgrade_body_unclear(
                    backup = backup.clone(),
                    current = sibling.clone()
                ))
            },
            accept: None,
            options: vec![
                tr!(import_scrivener_upgrade_use_current(
                    current = sibling.clone()
                )),
                tr!(import_scrivener_upgrade_keep_backup(
                    backup = backup.clone()
                )),
            ],
        },
        ScrivenerQuestion::UnmergedMobileEdits { .. } => Asked {
            title: tr!(import_scrivener_ios_pending_title()),
            body: tr!(import_scrivener_ios_pending_body()),
            accept: Some(tr!(import_scrivener_ios_pending_accept())),
            options: Vec::new(),
        },
        ScrivenerQuestion::NoTextInCopy { text_items } => Asked {
            title: tr!(import_scrivener_no_text_title(count = *text_items)),
            body: tr!(import_scrivener_no_text_body()),
            accept: Some(tr!(import_scrivener_no_text_accept())),
            options: Vec::new(),
        },
    })
}

// ── what the analysis found ──────────────────────────────────────────────────────────────

/// A notice as the Project page shows it, or the Save page beside its field. `computers`
/// names the computer a lock was left on, which the card may show and nothing keeps.
pub(crate) fn notice(finding: &ScrivenerFinding, computers: &ScrivenerComputers) -> Option<Worded> {
    let a = args_of(finding);
    let w = |title: LocalizedString, body: LocalizedString| Some(Worded::new(title, body));
    let title_only = |title: LocalizedString| {
        Some(Worded {
            title,
            body: None,
            warning: false,
            retry: false,
            details: None,
        })
    };
    match key_of(finding) {
        "scrivener-notice-project-found-below-pick" => w(
            tr!(import_scrivener_notice_project_found_below_pick_title()),
            tr!(import_scrivener_notice_project_found_below_pick_body(
                path = txt(a, "path")
            )),
        ),
        "scrivener-notice-source-is-template" => w(
            match opt_text(a, "title") {
                Some(title) => tr!(import_scrivener_notice_source_is_template_named(
                    title = title
                )),
                None => tr!(import_scrivener_notice_source_is_template_title()),
            },
            tr!(import_scrivener_notice_source_is_template_body()),
        ),
        "scrivener-notice-source-is-backup" => w(
            tr!(import_scrivener_notice_source_is_backup_title()),
            tr!(import_scrivener_notice_source_is_backup_body()),
        ),
        "scrivener-notice-importing-pre-upgrade-copy" => w(
            tr!(import_scrivener_notice_importing_pre_upgrade_copy_title()),
            tr!(import_scrivener_notice_importing_pre_upgrade_copy_body()),
        ),
        "scrivener-notice-imported-from-ios-stub" => w(
            tr!(import_scrivener_notice_imported_from_ios_stub_title()),
            tr!(import_scrivener_notice_imported_from_ios_stub_body()),
        ),
        "scrivener-notice-stale-mobile-binder-ignored" => w(
            tr!(import_scrivener_notice_stale_mobile_binder_ignored_title()),
            tr!(import_scrivener_notice_stale_mobile_binder_ignored_body()),
        ),
        "scrivener-notice-identical-manifest-copies" => w(
            tr!(import_scrivener_notice_identical_manifest_copies_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_identical_manifest_copies_body()),
        ),
        "scrivener-notice-manifest-copies-ignored" => w(
            tr!(import_scrivener_notice_manifest_copies_ignored_title()),
            tr!(import_scrivener_notice_manifest_copies_ignored_body(
                chosen = txt(a, "chosen"),
                count = list(a, "copies").len() as i64
            )),
        ),
        "scrivener-notice-pinned-manifest-looks-like-a-copy" => w(
            tr!(import_scrivener_notice_pinned_manifest_looks_like_a_copy_title()),
            tr!(
                import_scrivener_notice_pinned_manifest_looks_like_a_copy_body(
                    name = txt(a, "name"),
                    original = txt(a, "original")
                )
            ),
        ),
        "scrivener-notice-pinned-manifest-has-siblings" => w(
            tr!(import_scrivener_notice_pinned_manifest_has_siblings_title(
                count = num(a, "others")
            )),
            tr!(import_scrivener_notice_pinned_manifest_has_siblings_body()),
        ),
        "scrivener-notice-legacy-v1-files-beside" => w(
            tr!(import_scrivener_notice_legacy_v1_files_beside_title()),
            tr!(import_scrivener_notice_legacy_v1_files_beside_body()),
        ),
        "scrivener-notice-mixed-payload-trees" => w(
            tr!(import_scrivener_notice_mixed_payload_trees_title()),
            tr!(import_scrivener_notice_mixed_payload_trees_body()),
        ),
        "scrivener-notice-id-scheme-mismatch" => w(
            tr!(import_scrivener_notice_version_disagreement_title()),
            tr!(import_scrivener_notice_id_scheme_mismatch_body(
                version = txt(a, "version")
            )),
        ),
        "scrivener-notice-version-stamp-mismatch" => w(
            tr!(import_scrivener_notice_version_disagreement_title()),
            tr!(import_scrivener_notice_version_stamp_mismatch_body()),
        ),
        "scrivener-notice-unrecognised-writer" => w(
            tr!(import_scrivener_notice_unrecognised_writer_title()),
            match opt_text(a, "creator") {
                Some(creator) => tr!(import_scrivener_notice_unrecognised_writer_named(
                    creator = creator
                )),
                None => tr!(import_scrivener_notice_unrecognised_writer_body()),
            },
        ),
        "scrivener-notice-invalid-item-identifier" => w(
            tr!(import_scrivener_notice_invalid_item_identifier_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_invalid_item_identifier_body()),
        ),
        "scrivener-notice-items-without-identifier" => w(
            tr!(import_scrivener_notice_items_without_identifier_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_items_without_identifier_body()),
        )
        .map(|w| w.warning(true)),
        "scrivener-notice-duplicate-item-key" => w(
            tr!(import_scrivener_notice_duplicate_item_key_title(
                item = item_or_key(finding, a)
            )),
            tr!(import_scrivener_notice_duplicate_item_key_body()),
        ),
        "scrivener-notice-lock-present" => {
            let here = flag(a, "this_machine");
            let host = match computers {
                ScrivenerComputers::Named { lock_host, .. } => lock_host.clone(),
                ScrivenerComputers::Unknown => None,
            };
            let worded = match (here, host) {
                (true, _) => Worded::new(
                    tr!(import_scrivener_notice_lock_here_title()),
                    tr!(import_scrivener_notice_lock_here_body()),
                ),
                (false, Some(host)) => Worded::new(
                    tr!(import_scrivener_notice_lock_title(host = host)),
                    tr!(import_scrivener_notice_lock_body()),
                ),
                (false, None) => Worded::new(
                    tr!(import_scrivener_notice_lock_elsewhere_title()),
                    tr!(import_scrivener_notice_lock_body()),
                ),
            };
            Some(worded.warning(here))
        }
        "scrivener-notice-archived-while-open" => w(
            tr!(import_scrivener_notice_archived_while_open_title()),
            tr!(import_scrivener_notice_archived_while_open_body()),
        ),
        "scrivener-notice-content-missing" => {
            let salvaged = flag(a, "salvaged");
            let item = item_or_key(finding, a);
            w(
                match place_token(finding) {
                    WritingPlace::Notes => {
                        tr!(import_scrivener_notice_content_missing_notes(item = item))
                    }
                    WritingPlace::Synopsis => {
                        tr!(import_scrivener_notice_content_missing_synopsis(
                            item = item
                        ))
                    }
                    WritingPlace::Text => {
                        tr!(import_scrivener_notice_content_missing_text(item = item))
                    }
                },
                if salvaged {
                    tr!(import_scrivener_notice_content_missing_salvaged(
                        place = place_name(finding)
                    ))
                } else {
                    tr!(import_scrivener_notice_content_missing_body(
                        place = place_name(finding)
                    ))
                },
            )
            .map(|w| w.warning(!salvaged))
        }
        "scrivener-notice-content-truncated" => w(
            {
                let item = item_or_key(finding, a);
                match place_token(finding) {
                    WritingPlace::Notes => {
                        tr!(import_scrivener_notice_content_truncated_notes(item = item))
                    }
                    WritingPlace::Synopsis => {
                        tr!(import_scrivener_notice_content_truncated_synopsis(
                            item = item
                        ))
                    }
                    WritingPlace::Text => {
                        tr!(import_scrivener_notice_content_truncated_text(item = item))
                    }
                }
            },
            tr!(import_scrivener_notice_content_truncated_body(
                place = place_name(finding)
            )),
        )
        .map(|w| w.warning(true)),
        "scrivener-notice-missing-media" => w(
            tr!(import_scrivener_notice_missing_media_title(
                item = item_or_key(finding, a)
            )),
            tr!(import_scrivener_notice_missing_media_body()),
        ),
        "scrivener-notice-sync-conflict-copies" => w(
            tr!(import_scrivener_notice_sync_conflict_copies_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_sync_conflict_copies_body(
                missing = num(a, "original_missing")
            )),
        ),
        "scrivener-notice-recovered-unreferenced-text" => w(
            tr!(import_scrivener_notice_recovered_unreferenced_text_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_recovered_unreferenced_text_body()),
        ),
        "scrivener-notice-sync-debris-ignored" => w(
            tr!(import_scrivener_notice_sync_debris_ignored_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_sync_debris_ignored_body()),
        ),
        "scrivener-notice-unrecognised-files" => w(
            tr!(import_scrivener_notice_unrecognised_files_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_examples(examples = examples(a))),
        ),
        "scrivener-notice-symlink-not-followed" => w(
            tr!(import_scrivener_notice_symlink_not_followed_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_examples(examples = examples(a))),
        ),
        "scrivener-notice-unreadable-files-skipped" => w(
            tr!(import_scrivener_notice_unreadable_files_skipped_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_examples(examples = examples(a))),
        ),
        "scrivener-notice-cloud-placeholders-skipped" => w(
            tr!(import_scrivener_notice_cloud_placeholders_skipped_title(
                count = num(a, "count")
            )),
            tr!(import_scrivener_notice_cloud_placeholders_skipped_body()),
        ),
        "scrivener-notice-no-text-found" => w(
            tr!(import_scrivener_notice_no_text_found_title(
                count = num(a, "text_items")
            )),
            tr!(import_scrivener_notice_no_text_found_body()),
        ),
        "scrivener-notice-metadata-unreadable" => w(
            tr!(import_scrivener_notice_metadata_unreadable_title(
                member = txt(a, "member")
            )),
            tr!(import_scrivener_notice_metadata_unreadable_body()),
        ),
        "scrivener-notice-dangling-references" => w(
            {
                let count = num(a, "count");
                match txt(a, "kind").as_str() {
                    "keyword" => tr!(import_scrivener_notice_dangling_keyword_title(
                        count = count
                    )),
                    "label" => tr!(import_scrivener_notice_dangling_label_title(count = count)),
                    "status" => tr!(import_scrivener_notice_dangling_status_title(count = count)),
                    "section-type" => {
                        tr!(import_scrivener_notice_dangling_section_type_title(
                            count = count
                        ))
                    }
                    "custom-field" => {
                        tr!(import_scrivener_notice_dangling_custom_field_title(
                            count = count
                        ))
                    }
                    _ => tr!(import_scrivener_notice_dangling_item_title(count = count)),
                }
            },
            tr!(import_scrivener_notice_dangling_references_body()),
        ),
        "scrivener-notice-snapshots-not-imported" => title_only(tr!(
            import_scrivener_notice_snapshots_not_imported_line(count = num(a, "snapshots"))
        )),
        "scrivener-notice-writing-history-not-imported" => title_only(tr!(
            import_scrivener_notice_writing_history_not_imported_line(days = num(a, "days"))
        )),
        "scrivener-notice-title-placeholder-rejected" => title_only(tr!(
            import_scrivener_notice_title_placeholder_rejected_hint()
        )),
        "scrivener-notice-author-placeholder-rejected" => title_only(tr!(
            import_scrivener_notice_author_placeholder_rejected_hint()
        )),
        "scrivener-notice-title-from-project" => title_only(match txt(a, "source").as_str() {
            "compile-settings" => {
                tr!(import_scrivener_notice_title_from_project_hint_compile_settings())
            }
            "template" => tr!(import_scrivener_notice_title_from_project_hint_template()),
            "folder-name" => tr!(import_scrivener_notice_title_from_project_hint_folder_name()),
            _ => tr!(import_scrivener_notice_title_from_project_hint_project_properties()),
        }),
        "scrivener-notice-author-from-project" => {
            title_only(tr!(import_scrivener_notice_author_from_project_hint()))
        }
        "scrivener-notice-language-not-stated" => {
            title_only(tr!(import_scrivener_notice_language_not_stated_hint()))
        }
        "scrivener-notice-language-from-script" => title_only(tr!(
            import_scrivener_notice_language_from_script_hint(tag = txt(a, "tag"))
        )),
        _ => None,
    }
}

/// The item a notice is about: its binder path, else the key the binder no longer lists.
fn item_or_key(finding: &ScrivenerFinding, a: &[ScrivenerArg]) -> String {
    let path = item_of(finding);
    if path.is_empty() {
        txt(a, "item_key")
    } else {
        path
    }
}

// ── one line per kind ────────────────────────────────────────────────────────────────────

/// One line for every finding of one key, as the Review page's "before you import" strip
/// and the import's headline list it. `findings` are the findings of that key, in order; a
/// key about items counts them, a key about the project reads its own numbers from the
/// first. `None` for a key with no line of this kind.
pub(crate) fn summary_line(key: &str, findings: &[&ScrivenerFinding]) -> Option<LocalizedString> {
    let first = findings.first()?;
    let a = args_of(first);
    let count = findings.len() as i64;
    Some(match key {
        // What the analysis found, said once for the project.
        "scrivener-notice-snapshots-not-imported" => {
            tr!(import_scrivener_notice_snapshots_not_imported_line(
                count = num(a, "snapshots")
            ))
        }
        "scrivener-notice-writing-history-not-imported" => {
            tr!(import_scrivener_notice_writing_history_not_imported_line(
                days = num(a, "days")
            ))
        }
        // The structure mapping.
        "scrivener-structure-script-mode-draft" => {
            tr!(import_scrivener_structure_script_mode_draft_line(
                script = num(a, "script_documents"),
                documents = num(a, "draft_documents")
            ))
        }
        "scrivener-structure-one-document-manuscript" => {
            tr!(import_scrivener_structure_one_document_manuscript_line(
                count = count
            ))
        }
        "scrivener-structure-title-repeats-number" => {
            tr!(import_scrivener_structure_title_repeats_number_line(
                count = count
            ))
        }
        "scrivener-structure-claim-overruled" => {
            tr!(import_scrivener_structure_claim_overruled_line(
                count = count
            ))
        }
        "scrivener-structure-adopted-by-chapter" => {
            tr!(import_scrivener_structure_adopted_by_chapter_line(
                count = count
            ))
        }
        "scrivener-structure-looks-like-manuscript" => {
            tr!(import_scrivener_structure_looks_like_manuscript_line(
                count = count
            ))
        }
        "scrivener-structure-generic-sheet-title" => {
            tr!(import_scrivener_structure_generic_sheet_title_line(
                count = count
            ))
        }
        "scrivener-structure-placeholder-title" => {
            tr!(import_scrivener_structure_placeholder_title_line(
                count = count
            ))
        }
        "scrivener-structure-untitled-item" => {
            tr!(import_scrivener_structure_untitled_item_line(count = count))
        }
        "scrivener-structure-title-cleaned" => {
            tr!(import_scrivener_structure_title_cleaned_line(count = count))
        }
        "scrivener-structure-own-text-moved" => {
            tr!(import_scrivener_structure_own_text_moved_line(
                count = count
            ))
        }
        "scrivener-structure-text-with-children" => {
            tr!(import_scrivener_structure_text_with_children_line(
                count = count
            ))
        }
        "scrivener-structure-empty-folder-plain" => {
            tr!(import_scrivener_structure_empty_folder_plain_line(
                count = count
            ))
        }
        "scrivener-structure-book-inside-book" => {
            tr!(import_scrivener_structure_book_inside_book_line(
                count = count
            ))
        }
        "scrivener-structure-prologue-not-numbered" => {
            tr!(import_scrivener_structure_prologue_not_numbered_line(
                count = count
            ))
        }
        "scrivener-structure-matter-inside-book" => {
            tr!(import_scrivener_structure_matter_inside_book_line(
                count = count
            ))
        }
        "scrivener-structure-paratext-not-counted" => {
            tr!(import_scrivener_structure_paratext_not_counted_line(
                documents = num(a, "documents"),
                words = num(a, "words")
            ))
        }
        "scrivener-structure-document-notes" => {
            tr!(import_scrivener_structure_document_notes_line(
                count = count
            ))
        }
        "scrivener-structure-draft-content-as-note" => {
            tr!(import_scrivener_structure_draft_content_as_note_line())
        }
        "scrivener-structure-story-bible-folder" => {
            tr!(import_scrivener_structure_story_bible_folder_line(
                count = count
            ))
        }
        "scrivener-structure-templates-created" => {
            tr!(import_scrivener_structure_templates_created_line(
                count = num(a, "count")
            ))
        }
        "scrivener-structure-template-nesting-flattened" => {
            tr!(import_scrivener_structure_template_nesting_flattened_line(
                count = count
            ))
        }
        "scrivener-structure-template-kept-as-note" => {
            tr!(import_scrivener_structure_template_kept_as_note_line(
                count = count
            ))
        }
        "scrivener-structure-recovered-texts" => {
            tr!(import_scrivener_structure_recovered_texts_line(
                count = num(a, "count")
            ))
        }
        "scrivener-structure-project-notes" => {
            tr!(import_scrivener_structure_project_notes_line(
                count = num(a, "count")
            ))
        }
        "scrivener-structure-trash-brought-across" => {
            tr!(import_scrivener_structure_trash_brought_across_line(
                items = num(a, "items"),
                writing = num(a, "with_writing")
            ))
        }
        "scrivener-structure-not-compiled" => {
            tr!(import_scrivener_structure_not_compiled_line(count = count))
        }
        "scrivener-structure-media-in-draft" => {
            tr!(import_scrivener_structure_media_in_draft_line(
                count = count
            ))
        }
        "scrivener-structure-left-out" => tr!(import_scrivener_structure_left_out_line(
            count = count,
            items = findings
                .iter()
                .map(|f| num(args_of(f), "items"))
                .sum::<i64>()
        )),
        "scrivener-structure-trash-left-out" => {
            tr!(import_scrivener_structure_trash_left_out_line(
                items = num(a, "items"),
                writing = num(a, "with_writing")
            ))
        }
        "scrivener-structure-card-image-left-behind" => {
            tr!(import_scrivener_structure_card_image_left_behind_line(
                count = count
            ))
        }
        "scrivener-structure-default-template-not-linked" => {
            tr!(import_scrivener_structure_default_template_not_linked_line(
                count = count
            ))
        }
        "scrivener-structure-page-break-not-kept" => {
            tr!(import_scrivener_structure_page_break_not_kept_line(
                count = count
            ))
        }
        "scrivener-structure-compile-as-is-not-kept" => {
            tr!(import_scrivener_structure_compile_as_is_not_kept_line(
                count = count
            ))
        }
        // The labels, the status list and the dates.
        "scrivener-metadata-status-categories-guessed" => {
            tr!(import_scrivener_metadata_status_categories_guessed_line(
                count = list(a, "rungs").len() as i64
            ))
        }
        "scrivener-metadata-dates-unreadable" => {
            tr!(import_scrivener_metadata_dates_unreadable_line(
                count = num(a, "items")
            ))
        }
        "scrivener-metadata-labels-became-tags" => {
            tr!(import_scrivener_metadata_labels_became_tags_line(
                tags = num(a, "tags"),
                rows = num(a, "rows")
            ))
        }
        "scrivener-metadata-statuses-became-ladder" => {
            tr!(import_scrivener_metadata_statuses_became_ladder_line(
                rungs = num(a, "rungs"),
                rows = num(a, "rows")
            ))
        }
        "scrivener-metadata-ladder-seeded" => {
            tr!(import_scrivener_metadata_ladder_seeded_line(
                count = num(a, "rungs")
            ))
        }
        "scrivener-metadata-field-renamed" => match txt(a, "field").as_str() {
            "label" => tr!(import_scrivener_metadata_label_renamed_line(
                title = txt(a, "title")
            )),
            _ => tr!(import_scrivener_metadata_status_renamed_line(
                title = txt(a, "title")
            )),
        },
        // One line per field and value (`line_variant`): each names its own.
        "scrivener-metadata-none-value-collapsed" => match txt(a, "field").as_str() {
            "label" => tr!(import_scrivener_metadata_none_label_collapsed_line(
                name = txt(a, "name"),
                count = total(findings, "items")
            )),
            _ => tr!(import_scrivener_metadata_none_status_collapsed_line(
                name = txt(a, "name"),
                count = total(findings, "items")
            )),
        },
        "scrivener-metadata-unnamed-value" => match txt(a, "field").as_str() {
            "label" => tr!(import_scrivener_metadata_unnamed_label_line(
                id = num(a, "id")
            )),
            "keyword" => tr!(import_scrivener_metadata_unnamed_keyword_line(
                id = num(a, "id")
            )),
            _ => tr!(import_scrivener_metadata_unnamed_status_line(
                id = num(a, "id")
            )),
        },
        "scrivener-metadata-tag-name-disambiguated" => {
            tr!(import_scrivener_metadata_tag_name_disambiguated_line(
                count = list(a, "names").len() as i64
            ))
        }
        "scrivener-metadata-rung-names-disambiguated" => {
            tr!(import_scrivener_metadata_rung_names_disambiguated_line(
                count = list(a, "names").len() as i64
            ))
        }
        "scrivener-metadata-unused-labels-skipped" => {
            tr!(import_scrivener_metadata_unused_labels_skipped_line(
                count = list(a, "names").len() as i64
            ))
        }
        "scrivener-metadata-values-without-row" => {
            let count = total(findings, "items");
            match txt(a, "field").as_str() {
                "label" => tr!(import_scrivener_metadata_labels_without_row_line(
                    count = count
                )),
                "keyword" => tr!(import_scrivener_metadata_keywords_without_row_line(
                    count = count
                )),
                "custom-field" => tr!(import_scrivener_metadata_custom_values_without_row_line(
                    count = count
                )),
                "bookmark" => tr!(import_scrivener_metadata_bookmarks_without_row_line(
                    count = count
                )),
                "target" => tr!(import_scrivener_metadata_targets_without_row_line(
                    count = count
                )),
                _ => tr!(import_scrivener_metadata_statuses_without_row_line(
                    count = count
                )),
            }
        }
        // The keywords, custom metadata, collections, bookmarks, targets and the rest:
        // whole-project findings, one per kind, but a field's, a collection's and a note's
        // own (`line_variant`).
        "scrivener-metadata-pov-hint" => {
            let (values, matches) = (list(a, "values"), list(a, "matches"));
            let count = values.len() as i64;
            localized(move || {
                tr!(import_scrivener_metadata_pov_hint_line(
                    count = count,
                    values = joined(&values),
                    matches = joined(&matches)
                ))
                .resolve_now()
            })
        }
        "scrivener-metadata-checkbox-default-unknown" => {
            tr!(import_scrivener_metadata_checkbox_default_unknown_line(
                field = txt(a, "field")
            ))
        }
        "scrivener-metadata-goals-in-other-unit" => {
            tr!(import_scrivener_metadata_goals_in_other_unit_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-counting-differs" => {
            tr!(import_scrivener_metadata_counting_differs_line())
        }
        "scrivener-metadata-writing-days-not-read" => {
            tr!(import_scrivener_metadata_writing_days_not_read_line(
                raw = txt(a, "raw")
            ))
        }
        "scrivener-metadata-script-mode-rows" => {
            tr!(import_scrivener_metadata_script_mode_rows_line(
                count = num(a, "rows")
            ))
        }
        "scrivener-metadata-keywords-became-tags" => {
            tr!(import_scrivener_metadata_keywords_became_tags_line(
                tags = num(a, "tags"),
                rows = num(a, "rows")
            ))
        }
        "scrivener-metadata-keyword-hierarchy-flattened" => {
            tr!(import_scrivener_metadata_keyword_hierarchy_flattened_line(
                count = num(a, "keywords")
            ))
        }
        "scrivener-metadata-fields-became-tags" => {
            tr!(import_scrivener_metadata_fields_became_tags_line(
                fields = num(a, "fields"),
                tags = num(a, "tags"),
                rows = num(a, "rows")
            ))
        }
        "scrivener-metadata-collections-became-tags" => {
            tr!(import_scrivener_metadata_collections_became_tags_line(
                tags = num(a, "tags"),
                rows = num(a, "rows")
            ))
        }
        "scrivener-metadata-collection-order-lost" => {
            tr!(import_scrivener_metadata_collection_order_lost_line(
                count = list(a, "names").len() as i64
            ))
        }
        "scrivener-metadata-bookmarks-became-links" => {
            tr!(import_scrivener_metadata_bookmarks_became_links_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-goals-kept" => {
            tr!(import_scrivener_metadata_goals_kept_line(
                count = num(a, "rows")
            ))
        }
        "scrivener-metadata-draft-target-kept" => match txt(a, "unit").as_str() {
            "characters" => tr!(import_scrivener_metadata_draft_target_kept_characters_line(
                value = num(a, "value")
            )),
            _ => tr!(import_scrivener_metadata_draft_target_kept_words_line(
                value = num(a, "value")
            )),
        },
        "scrivener-metadata-pace-planned" => {
            if flag(a, "day_off") {
                tr!(import_scrivener_metadata_pace_planned_day_off_line(
                    day = txt(a, "day")
                ))
            } else {
                tr!(import_scrivener_metadata_pace_planned_line(
                    day = txt(a, "day")
                ))
            }
        }
        "scrivener-metadata-dictionary-words" => {
            tr!(import_scrivener_metadata_dictionary_words_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-listed-in-import-note" => {
            listed_line(txt(a, "section"), num(a, "count"))
        }
        "scrivener-metadata-targets-on-notes" => {
            tr!(import_scrivener_metadata_targets_on_notes_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-targets-on-split-documents" => {
            tr!(import_scrivener_metadata_targets_on_split_documents_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-bookmark-ends-without-row" => {
            tr!(import_scrivener_metadata_bookmark_ends_without_row_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-unused-keywords-skipped" => {
            tr!(import_scrivener_metadata_unused_keywords_skipped_line(
                count = list(a, "names").len() as i64
            ))
        }
        "scrivener-metadata-list-value-invalid" => {
            tr!(import_scrivener_metadata_list_value_invalid_line(
                field = txt(a, "field"),
                count = total(findings, "count")
            ))
        }
        "scrivener-metadata-undefined-custom-field" => {
            tr!(import_scrivener_metadata_undefined_custom_field_line(
                count = num(a, "values")
            ))
        }
        "scrivener-metadata-collection-members-missing" => {
            tr!(import_scrivener_metadata_collection_members_missing_line(
                name = txt(a, "name"),
                count = total(findings, "count")
            ))
        }
        "scrivener-metadata-empty-collections-skipped" => {
            tr!(import_scrivener_metadata_empty_collections_skipped_line(
                count = list(a, "names").len() as i64
            ))
        }
        "scrivener-metadata-bookmarks-unresolved" => {
            tr!(import_scrivener_metadata_bookmarks_unresolved_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-targets-in-pages" => {
            tr!(import_scrivener_metadata_targets_in_pages_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-targets-unreadable" => {
            tr!(import_scrivener_metadata_targets_unreadable_line(
                count = num(a, "count")
            ))
        }
        "scrivener-metadata-draft-target-not-kept" => {
            let value = num(a, "value");
            let (reason, unit) = (txt(a, "reason"), txt(a, "unit"));
            in_note("project", move |note, folder| {
                match (reason.as_str(), unit.as_str()) {
                    ("pages", _) | (_, "pages") => {
                        tr!(import_scrivener_metadata_draft_target_in_pages_line(
                            value = value,
                            note = note,
                            folder = folder
                        ))
                    }
                    (_, "characters") => {
                        tr!(
                            import_scrivener_metadata_draft_target_no_book_characters_line(
                                value = value,
                                note = note,
                                folder = folder
                            )
                        )
                    }
                    _ => tr!(import_scrivener_metadata_draft_target_no_book_words_line(
                        value = value,
                        note = note,
                        folder = folder
                    )),
                }
            })
        }
        "scrivener-metadata-deadline-not-planned" => {
            let (day, reason) = (txt(a, "day"), txt(a, "reason"));
            in_note("project", move |note, folder| {
                let day = day.clone();
                match reason.as_str() {
                    _ if day.is_empty() => {
                        tr!(import_scrivener_metadata_deadline_unreadable_line(
                            note = note,
                            folder = folder
                        ))
                    }
                    "passed" => tr!(import_scrivener_metadata_deadline_passed_line(
                        day = day,
                        note = note,
                        folder = folder
                    )),
                    "no-writing-day-left" => {
                        tr!(import_scrivener_metadata_deadline_no_writing_day_line(
                            day = day,
                            note = note,
                            folder = folder
                        ))
                    }
                    "character-goal" => {
                        tr!(import_scrivener_metadata_deadline_character_goal_line(
                            day = day,
                            note = note,
                            folder = folder
                        ))
                    }
                    "no-single-book" => tr!(import_scrivener_metadata_deadline_no_book_line(
                        day = day,
                        note = note,
                        folder = folder
                    )),
                    "template" => tr!(import_scrivener_metadata_deadline_template_line(
                        day = day,
                        note = note,
                        folder = folder
                    )),
                    // No word goal to plan towards, and any reason this version does not
                    // know: a day that reads and no plan is what the line says.
                    _ => tr!(import_scrivener_metadata_deadline_no_word_goal_line(
                        day = day,
                        note = note,
                        folder = folder
                    )),
                }
            })
        }
        "scrivener-metadata-session-target-not-imported" if flag(a, "calculated") => {
            let (value, unit) = (num(a, "value"), txt(a, "unit"));
            in_note("project", move |note, folder| match unit.as_str() {
                "characters" => tr!(
                    import_scrivener_metadata_session_target_calculated_characters_line(
                        value = value,
                        note = note,
                        folder = folder
                    )
                ),
                "pages" => tr!(
                    import_scrivener_metadata_session_target_calculated_pages_line(
                        value = value,
                        note = note,
                        folder = folder
                    )
                ),
                _ => tr!(
                    import_scrivener_metadata_session_target_calculated_words_line(
                        value = value,
                        note = note,
                        folder = folder
                    )
                ),
            })
        }
        "scrivener-metadata-session-target-not-imported" => {
            let (value, unit) = (num(a, "value"), txt(a, "unit"));
            in_note("project", move |note, folder| match unit.as_str() {
                "characters" => tr!(import_scrivener_metadata_session_target_characters_line(
                    value = value,
                    note = note,
                    folder = folder
                )),
                "pages" => tr!(import_scrivener_metadata_session_target_pages_line(
                    value = value,
                    note = note,
                    folder = folder
                )),
                _ => tr!(import_scrivener_metadata_session_target_words_line(
                    value = value,
                    note = note,
                    folder = folder
                )),
            })
        }
        "scrivener-metadata-icons-dropped" => {
            tr!(import_scrivener_metadata_icons_dropped_line(
                count = num(a, "rows")
            ))
        }
        // The prose: `count` is how many places (texts, notes, synopses) a key was found
        // in, `total` adds up what it counted in each.
        "scrivener-prose-unreadable" => tr!(import_scrivener_prose_unreadable_line(count = count)),
        "scrivener-prose-comments-unreadable" => tr!(
            import_scrivener_prose_comments_unreadable_line(count = count)
        ),
        "scrivener-prose-read-as-plain-text" => tr!(
            import_scrivener_prose_read_as_plain_text_line(count = count)
        ),
        "scrivener-prose-heading-repeats-title" => tr!(
            import_scrivener_prose_heading_repeats_title_line(count = count)
        ),
        "scrivener-prose-long-document" => {
            tr!(import_scrivener_prose_long_document_line(count = count))
        }
        "scrivener-prose-struck-text-compiled-away" => tr!(
            import_scrivener_prose_struck_text_compiled_away_line(count = count)
        ),
        "scrivener-prose-annotation-only-document-kept" => tr!(
            import_scrivener_prose_annotation_only_document_kept_line(count = count)
        ),
        "scrivener-prose-media-file-copied" => {
            tr!(import_scrivener_prose_media_file_copied_line(count = count))
        }
        "scrivener-prose-media-file-not-carried" => tr!(
            import_scrivener_prose_media_file_not_carried_line(count = count)
        ),
        "scrivener-prose-inline-note-unmatched" => tr!(
            import_scrivener_prose_inline_note_unmatched_line(count = total(findings, "count"))
        ),
        "scrivener-prose-style-unresolved" => tr!(import_scrivener_prose_style_unresolved_line(
            count = total(findings, "count")
        )),
        "scrivener-prose-styles-lost-in-export" => tr!(
            import_scrivener_prose_styles_lost_in_export_line(count = total(findings, "count"))
        ),
        "scrivener-prose-comment-anchor-without-entry" => {
            tr!(import_scrivener_prose_comment_anchor_without_entry_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-comment-entry-without-anchor" => {
            tr!(import_scrivener_prose_comment_entry_without_anchor_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-footnote-entry-without-anchor" => {
            tr!(import_scrivener_prose_footnote_entry_without_anchor_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-range-comment-approximate" => tr!(
            import_scrivener_prose_range_comment_approximate_line(count = total(findings, "count"))
        ),
        "scrivener-prose-document-link-unresolved" => tr!(
            import_scrivener_prose_document_link_unresolved_line(count = total(findings, "count"))
        ),
        "scrivener-prose-ornament-lines-kept" => tr!(
            import_scrivener_prose_ornament_lines_kept_line(count = total(findings, "count"))
        ),
        "scrivener-prose-pictures-without-description" => {
            tr!(import_scrivener_prose_pictures_without_description_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-inline-notes-merged" => tr!(
            import_scrivener_prose_inline_notes_merged_line(count = total(findings, "count"))
        ),
        "scrivener-prose-inline-notes-joined-across-paragraphs" => tr!(
            import_scrivener_prose_inline_notes_joined_across_paragraphs_line(
                count = total(findings, "count")
            )
        ),
        "scrivener-prose-blank-notes-dropped" => tr!(
            import_scrivener_prose_blank_notes_dropped_line(count = total(findings, "count"))
        ),
        "scrivener-prose-annotations-became-comments" => {
            tr!(import_scrivener_prose_annotations_became_comments_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-preserve-formatting-passages" => {
            tr!(import_scrivener_prose_preserve_formatting_passages_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-nested-note-unwrapped" => tr!(
            import_scrivener_prose_nested_note_unwrapped_line(count = total(findings, "count"))
        ),
        "scrivener-prose-document-links-dropped" => tr!(
            import_scrivener_prose_document_links_dropped_line(count = total(findings, "count"))
        ),
        "scrivener-prose-document-links-kept" => tr!(
            import_scrivener_prose_document_links_kept_line(count = total(findings, "count"))
        ),
        "scrivener-prose-other-links-kept-as-text" => tr!(
            import_scrivener_prose_other_links_kept_as_text_line(count = total(findings, "count"))
        ),
        "scrivener-prose-typed-indents-removed" => tr!(
            import_scrivener_prose_typed_indents_removed_line(count = total(findings, "count"))
        ),
        "scrivener-prose-left-indents-not-kept" => tr!(
            import_scrivener_prose_left_indents_not_kept_line(count = total(findings, "count"))
        ),
        "scrivener-prose-blank-lines-removed" => tr!(
            import_scrivener_prose_blank_lines_removed_line(count = total(findings, "count"))
        ),
        "scrivener-prose-blank-lines-became-scene-breaks" => {
            tr!(import_scrivener_prose_blank_lines_became_scene_breaks_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-ornaments-became-scene-breaks" => {
            tr!(import_scrivener_prose_ornaments_became_scene_breaks_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-table-first-row-header" => tr!(
            import_scrivener_prose_table_first_row_header_line(count = total(findings, "count"))
        ),
        "scrivener-prose-table-cells-joined" => tr!(
            import_scrivener_prose_table_cells_joined_line(count = total(findings, "count"))
        ),
        "scrivener-prose-justified-text-not-kept" => tr!(
            import_scrivener_prose_justified_text_not_kept_line(count = total(findings, "count"))
        ),
        "scrivener-prose-comment-colours-lost" => tr!(
            import_scrivener_prose_comment_colours_lost_line(count = total(findings, "count"))
        ),
        "scrivener-prose-control-characters-removed" => {
            tr!(import_scrivener_prose_control_characters_removed_line(
                count = total(findings, "count")
            ))
        }
        "scrivener-prose-note-on-heading-lost" => tr!(
            import_scrivener_prose_note_on_heading_lost_line(count = total(findings, "count"))
        ),
        "scrivener-prose-undecodable-characters" => tr!(
            import_scrivener_prose_undecodable_characters_line(count = total(findings, "count"))
        ),
        "scrivener-prose-markdown-as-text" => tr!(import_scrivener_prose_markdown_as_text_line(
            count = total(findings, "paragraphs")
        )),
        "scrivener-prose-citation-keys-as-text" => {
            tr!(import_scrivener_prose_citation_keys_as_text_line(
                count = total(findings, "paragraphs")
            ))
        }
        "scrivener-prose-range-comment-reading" => {
            let count = total(findings, "count");
            match txt(a, "reading").as_str() {
                "markers-counted" => {
                    tr!(
                        import_scrivener_prose_range_comment_reading_markers_counted_line(
                            count = count
                        )
                    )
                }
                "pictures-not-counted" => tr!(
                    import_scrivener_prose_range_comment_reading_pictures_not_counted_line(
                        count = count
                    )
                ),
                _ => tr!(import_scrivener_prose_range_comment_reading_line(
                    count = count
                )),
            }
        }
        "scrivener-prose-linked-picture-not-read" => {
            tr!(import_scrivener_prose_linked_picture_not_read_line(
                count = count,
                example = first_value(findings, "name")
            ))
        }
        "scrivener-prose-placeholders-in-exported-text" => {
            tr!(import_scrivener_prose_placeholders_in_exported_text_line(
                count = total(findings, "count"),
                example = first_value(findings, "placeholder")
            ))
        }
        "scrivener-prose-placeholders-in-other-text" => {
            tr!(import_scrivener_prose_placeholders_in_other_text_line(
                count = total(findings, "count"),
                example = first_value(findings, "placeholder")
            ))
        }
        "scrivener-prose-compile-deleted-style-printed" => {
            tr!(import_scrivener_prose_compile_deleted_style_printed_line(
                count = distinct(findings, "name").len() as i64,
                example = first_value(findings, "name")
            ))
        }
        "scrivener-prose-compile-raw-style-printed" => {
            tr!(import_scrivener_prose_compile_raw_style_printed_line(
                count = distinct(findings, "name").len() as i64,
                example = first_value(findings, "name")
            ))
        }
        "scrivener-prose-style-not-carried" => tr!(import_scrivener_prose_style_not_carried_line(
            count = distinct(findings, "name").len() as i64,
            example = first_value(findings, "name")
        )),
        "scrivener-prose-formatting-lost" => match txt(a, "what").as_str() {
            "colour" => tr!(import_scrivener_prose_formatting_lost_colour_line(
                count = count
            )),
            "highlight" => {
                tr!(import_scrivener_prose_formatting_lost_highlight_line(
                    count = count
                ))
            }
            "small-caps" => {
                tr!(import_scrivener_prose_formatting_lost_small_caps_line(
                    count = count
                ))
            }
            "all-caps" => tr!(import_scrivener_prose_formatting_lost_all_caps_line(
                count = count
            )),
            "hidden" => tr!(import_scrivener_prose_formatting_lost_hidden_line(
                count = count
            )),
            _ => tr!(import_scrivener_prose_formatting_lost_line(count = count)),
        },
        "scrivener-prose-left-out" => {
            let count = total(findings, "count");
            match txt(a, "what").as_str() {
                "after-document" => {
                    tr!(import_scrivener_prose_left_out_after_document_line(
                        count = count
                    ))
                }
                "header-or-footer" => {
                    tr!(import_scrivener_prose_left_out_header_or_footer_line(
                        count = count
                    ))
                }
                "list-label-picture" => {
                    tr!(import_scrivener_prose_left_out_list_label_picture_line(
                        count = count
                    ))
                }
                _ => tr!(import_scrivener_prose_left_out_line(count = count)),
            }
        }
        _ => return None,
    })
}

/// Every count named `name` the findings carry, added up.
fn total(findings: &[&ScrivenerFinding], name: &str) -> i64 {
    findings
        .iter()
        .map(|f| num(args_of(f), name))
        .fold(0i64, i64::saturating_add)
}

/// The text values named `name` the findings carry, each once, in order.
fn distinct(findings: &[&ScrivenerFinding], name: &str) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for finding in findings {
        if let Some(value) = opt_text(args_of(finding), name)
            && !value.is_empty()
            && !seen.contains(&value)
        {
            seen.push(value);
        }
    }
    seen
}

/// The first text value named `name` the findings carry, for a sentence to quote.
fn first_value(findings: &[&ScrivenerFinding], name: &str) -> String {
    distinct(findings, name)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// `values` as one phrase of the interface's language, each as the project wrote it.
fn joined(values: &[String]) -> String {
    values.join(&tr!(import_scrivener_list_separator()).resolve_now())
}

/// The name of a note of the "From Scrivener" folder, by the token the converter gives
/// it: the words the interface gave the converter for that note's title.
pub(crate) fn note_title(section: &str) -> Option<LocalizedString> {
    Some(match section {
        "project" => tr!(import_scrivener_note_project()),
        "project-bookmarks" => tr!(import_scrivener_note_project_bookmarks()),
        "replacements" => tr!(import_scrivener_note_replacements()),
        "auto-complete" => tr!(import_scrivener_note_auto_complete()),
        "saved-searches" => tr!(import_scrivener_note_saved_searches()),
        "custom-metadata" => tr!(import_scrivener_note_custom_metadata()),
        "targets" => tr!(import_scrivener_note_targets()),
        "bookmarks" => tr!(import_scrivener_note_bookmarks()),
        _ => return None,
    })
}

/// A sentence naming the note of `section` in the "From Scrivener" folder, and the folder,
/// as the import titles them: resolved when it is read, so it follows a change of
/// language as the sentence around them does. A section this version does not know is
/// named by its token rather than by nothing.
fn in_note(
    section: &'static str,
    sentence: impl Fn(String, String) -> LocalizedString + 'static,
) -> LocalizedString {
    localized(move || {
        let note = note_title(section).map_or_else(|| section.to_string(), |t| t.resolve_now());
        let folder = tr!(import_scrivener_from_scrivener()).resolve_now();
        sentence(note, folder).resolve_now()
    })
}

/// What one note of the "From Scrivener" folder lists, and that it lists it there.
fn listed_line(section: String, count: i64) -> LocalizedString {
    localized(move || {
        let note = note_title(&section).map_or_else(|| section.clone(), |t| t.resolve_now());
        let folder = tr!(import_scrivener_from_scrivener()).resolve_now();
        match section.as_str() {
            "project" => tr!(import_scrivener_metadata_listed_project_line(
                count = count,
                note = note,
                folder = folder
            )),
            "project-bookmarks" => tr!(import_scrivener_metadata_listed_project_bookmarks_line(
                count = count,
                note = note,
                folder = folder
            )),
            "replacements" => tr!(import_scrivener_metadata_listed_replacements_line(
                count = count,
                note = note,
                folder = folder
            )),
            "auto-complete" => tr!(import_scrivener_metadata_listed_auto_complete_line(
                count = count,
                note = note,
                folder = folder
            )),
            "saved-searches" => tr!(import_scrivener_metadata_listed_saved_searches_line(
                count = count,
                note = note,
                folder = folder
            )),
            "custom-metadata" => tr!(import_scrivener_metadata_listed_custom_metadata_line(
                count = count,
                note = note,
                folder = folder
            )),
            "targets" => tr!(import_scrivener_metadata_listed_targets_line(
                count = count,
                note = note,
                folder = folder
            )),
            // The bookmarks, and a note this version does not know, named by its token.
            _ => tr!(import_scrivener_metadata_listed_bookmarks_line(
                count = count,
                note = note,
                folder = folder
            )),
        }
        .resolve_now()
    })
}

/// The names a finding lists, as the project wrote them, for the report to list under
/// its line: the labels and keywords no document wears, the collections it leaves out or
/// whose order it loses, the field ids it does not know, the rungs it guessed a kind
/// for, the icons it drops, the tags it renamed. Empty for a finding that lists none.
pub(crate) fn listed_names(finding: &ScrivenerFinding) -> Vec<String> {
    let a = args_of(finding);
    let name = match key_of(finding) {
        "scrivener-metadata-unused-labels-skipped"
        | "scrivener-metadata-unused-keywords-skipped"
        | "scrivener-metadata-empty-collections-skipped"
        | "scrivener-metadata-collection-order-lost"
        | "scrivener-metadata-tag-name-disambiguated"
        | "scrivener-metadata-rung-names-disambiguated"
        | "scrivener-metadata-icons-dropped" => "names",
        "scrivener-metadata-undefined-custom-field" => "ids",
        "scrivener-metadata-status-categories-guessed" => "rungs",
        _ => return Vec::new(),
    };
    list(a, name)
}

/// The token that makes one key say several sentences, one line each: what formatting
/// was lost, what content was left out, which reading placed a comment. `None` for a key
/// whose findings make one line.
///
/// The metadata findings are said once per field (a renamed Label field and a renamed
/// Status field are two facts; so are two checkbox fields, or the invalid values of two
/// list fields), once per collection missing documents, once per note of the "From
/// Scrivener" folder, and a value without a name or a value read as none once per value:
/// each names what it is about, so none may stand for another.
pub(crate) fn line_variant(finding: &ScrivenerFinding) -> Option<String> {
    let a = args_of(finding);
    let name = match key_of(finding) {
        "scrivener-prose-formatting-lost" | "scrivener-prose-left-out" => "what",
        "scrivener-prose-range-comment-reading" => "reading",
        "scrivener-metadata-field-renamed"
        | "scrivener-metadata-values-without-row"
        | "scrivener-metadata-checkbox-default-unknown"
        | "scrivener-metadata-list-value-invalid" => "field",
        "scrivener-metadata-collection-members-missing" => "name",
        "scrivener-metadata-listed-in-import-note" => "section",
        "scrivener-metadata-unnamed-value" => {
            return Some(format!("{}:{}", txt(a, "field"), num(a, "id")));
        }
        "scrivener-metadata-none-value-collapsed" => {
            return Some(format!("{}:{}", txt(a, "field"), txt(a, "name")));
        }
        _ => return None,
    };
    opt_text(a, name)
}

/// What splits a document importer's kind into lines: a warning and an information of one
/// key say different things.
pub(crate) fn document_variant(finding: &ScrivenerFinding) -> Option<String> {
    Some(if flag(args_of(finding), "warning") {
        "warning".to_string()
    } else {
        "info".to_string()
    })
}

/// Whether a finding is one of the document importer's, under its key.
pub(crate) fn is_document_finding(finding: &ScrivenerFinding) -> bool {
    matches!(
        finding,
        ScrivenerFinding::Reported {
            family: ScrivenerFindingFamily::Document,
            ..
        }
    )
}

/// A document importer's finding, worded by that importer's own sentence: it names the
/// item, by its title, where that sentence names a file.
pub(crate) fn document_line(finding: &ScrivenerFinding) -> Option<LocalizedString> {
    let ScrivenerFinding::Reported {
        key,
        binder_path,
        args,
        ..
    } = finding
    else {
        return None;
    };
    let title = binder_path.last().cloned().unwrap_or_default();
    let diagnostic = crate::import_document::import_document_vm::Diagnostic {
        key: key.clone(),
        severity: if flag(args, "warning") {
            "warning".to_string()
        } else {
            "info".to_string()
        },
        // The sentence quotes the file name of its path, so a title holding a slash
        // would lose its first part: the slash is shown as the division slash, which
        // looks the same and divides nothing.
        path: title.replace(['/', '\\'], "\u{2215}"),
        detail: txt(args, "detail"),
        count: num(args, "count"),
        row: None,
    };
    Some(diagnostic.message(&title, None))
}

/// The line of a document importer's kind found in several places: the first one's own
/// sentence, then how many more places the items under the line list.
pub(crate) fn document_lines(findings: &[&ScrivenerFinding]) -> Option<LocalizedString> {
    let first = document_line(findings.first()?)?;
    let others = findings.len().saturating_sub(1) as i64;
    if others == 0 {
        return Some(first);
    }
    Some(localized(move || {
        tr!(import_scrivener_report_document_others(
            sentence = first.resolve_now(),
            others = others
        ))
        .resolve_now()
    }))
}

/// The line the import's report gives a finding the analysis already said on the Project
/// page: that notice's title, with no computer named. A Save page hint is said beside a
/// filled field there ("This title is …"); the report shows no field, so it says in the
/// past what Skribisto proposed, and that the writer changed it when they did
/// (`changed`). A missing text whose words were salvaged says so, and asks for a look.
pub(crate) fn notice_line(finding: &ScrivenerFinding, changed: bool) -> Option<LocalizedString> {
    let a = args_of(finding);
    let proposed = match key_of(finding) {
        "scrivener-notice-title-placeholder-rejected" => {
            tr!(import_scrivener_report_title_placeholder_rejected())
        }
        "scrivener-notice-author-placeholder-rejected" => {
            tr!(import_scrivener_report_author_placeholder_rejected())
        }
        "scrivener-notice-title-from-project" => match txt(a, "source").as_str() {
            "compile-settings" => tr!(import_scrivener_report_title_from_compile_settings()),
            "template" => tr!(import_scrivener_report_title_from_template()),
            "folder-name" => tr!(import_scrivener_report_title_from_folder_name()),
            _ => tr!(import_scrivener_report_title_from_project_properties()),
        },
        "scrivener-notice-author-from-project" => {
            tr!(import_scrivener_report_author_from_project())
        }
        "scrivener-notice-language-not-stated" => {
            tr!(import_scrivener_report_language_not_stated())
        }
        "scrivener-notice-language-from-script" => {
            tr!(import_scrivener_report_language_from_script(
                tag = txt(a, "tag")
            ))
        }
        "scrivener-notice-content-missing" if flag(a, "salvaged") => {
            let item = item_or_key(finding, a);
            return Some(match place_token(finding) {
                WritingPlace::Notes => {
                    tr!(import_scrivener_report_content_salvaged_notes(item = item))
                }
                WritingPlace::Synopsis => {
                    tr!(import_scrivener_report_content_salvaged_synopsis(
                        item = item
                    ))
                }
                WritingPlace::Text => {
                    tr!(import_scrivener_report_content_salvaged_text(item = item))
                }
            });
        }
        _ => return notice(finding, &ScrivenerComputers::Unknown).map(|w| w.title),
    };
    if !changed {
        return Some(proposed);
    }
    Some(localized(move || {
        tr!(import_scrivener_report_you_changed_it(
            hint = proposed.resolve_now()
        ))
        .resolve_now()
    }))
}

/// The Save page field a hint is about, for the report to ask whether the writer changed
/// it; `None` for any other key.
pub(crate) fn save_field_of(key: &str) -> Option<SaveField> {
    match key {
        "scrivener-notice-title-placeholder-rejected" | "scrivener-notice-title-from-project" => {
            Some(SaveField::Title)
        }
        "scrivener-notice-author-placeholder-rejected" | "scrivener-notice-author-from-project" => {
            Some(SaveField::Author)
        }
        "scrivener-notice-language-not-stated" | "scrivener-notice-language-from-script" => {
            Some(SaveField::Language)
        }
        _ => None,
    }
}

/// A field of the Save page a hint is said beside.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SaveField {
    Title,
    Author,
    Language,
}

/// A question of the Project page the writer answered with `option`, as the report states
/// it after the fact: which project, binder file or copy the import read. `None` for a
/// question to accept, which the report words on its own, or an answer it does not offer.
pub(crate) fn answered(question: &ScrivenerQuestion, option: usize) -> Option<LocalizedString> {
    Some(match question {
        ScrivenerQuestion::SeveralProjects { candidates } => {
            tr!(import_scrivener_report_chose_project(
                project = candidates.get(option)?.clone()
            ))
        }
        ScrivenerQuestion::SeveralManifests { candidates, .. } => match candidates.get(option)? {
            ScrivenerManifestCandidate::Candidate { name, .. } => {
                tr!(import_scrivener_report_chose_manifest(file = name.clone()))
            }
            ScrivenerManifestCandidate::Empty => return None,
        },
        ScrivenerQuestion::EnclosingProject { project, .. } if option == 0 => {
            tr!(import_scrivener_report_chose_enclosing(
                project = project.clone()
            ))
        }
        ScrivenerQuestion::UpgradedSibling {
            backup, sibling, ..
        } => match option {
            0 => tr!(import_scrivener_report_chose_upgraded(
                current = sibling.clone()
            )),
            1 => tr!(import_scrivener_report_chose_backup(
                backup = backup.clone()
            )),
            _ => return None,
        },
        _ => return None,
    })
}

// ── a failed import ──────────────────────────────────────────────────────────────────────

/// The Scrivener importer's own sentence for a failure it words itself, or `None` for the
/// shared wording (`shared::import_failure`).
pub(crate) fn own_failure(reason: &FailureReason) -> Option<LocalizedString> {
    match reason {
        // The very refusal the Project page would have shown, worded the same way.
        FailureReason::ScrivenerRefused { refusal } => {
            refusal_paragraph(refusal).map(|paragraph| {
                localized(move || {
                    tr!(import_scrivener_error_refused(
                        refusal = paragraph.resolve_now()
                    ))
                    .resolve_now()
                })
            })
        }
        FailureReason::ScrivenerQuestionFirst { .. } => {
            Some(tr!(import_scrivener_error_question()))
        }
        FailureReason::ScrivenerNotAccepted { .. } => {
            Some(tr!(import_scrivener_error_not_accepted()))
        }
        FailureReason::ScrivenerChangedSinceReview => Some(tr!(import_scrivener_error_changed())),
        FailureReason::ScrivenerDepthNotOffered { .. }
        | FailureReason::ScrivenerRootNotOffered { .. } => {
            Some(tr!(import_scrivener_error_choice()))
        }
        _ => None,
    }
}

/// Whether a failure is one a fresh review answers: the project changed, or now asks
/// something the review did not, or the review's choices no longer fit it, or it was
/// refused for a cause that may go away by itself (files still in the cloud, a sync
/// writing), whose words tell the writer to read the project again. The failure notice then
/// offers **Review the import again**, which reads it again.
pub(crate) fn reviewing_again_helps(reason: &FailureReason) -> bool {
    match reason {
        FailureReason::ScrivenerChangedSinceReview
        | FailureReason::ScrivenerQuestionFirst { .. }
        | FailureReason::ScrivenerNotAccepted { .. }
        | FailureReason::ScrivenerDepthNotOffered { .. }
        | FailureReason::ScrivenerRootNotOffered { .. } => true,
        FailureReason::ScrivenerRefused { refusal: finding } => {
            refusal(finding).is_some_and(|worded| worded.retry)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests;
