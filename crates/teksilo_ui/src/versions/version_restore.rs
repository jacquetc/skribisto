// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Putting a past version back.
//!
//! The whole feature is about not losing work, so the one operation that
//! overwrites a writer's current text has to be the most careful thing in it.
//! Everything here exists to make a restore either **exactly right or refused
//! out loud** — never quietly wrong.
//!
//! ## One edit, not a document reset
//!
//! The replacement is a cursor edit inside a composite block:
//!
//! ```ignore
//! cursor.begin_edit_block();
//! cursor.select(SelectionType::Document);
//! cursor.insert_djot(&past);
//! cursor.end_edit_block();
//! ```
//!
//! **Never `TextDocument::set_djot` / `set_djot_sync`**, whose own documentation
//! says it *clears undo history*. That single difference is what makes a restore
//! reversible with one Ctrl+Z instead of permanent — and a document reset also
//! strands every comment highlight anchored in the text, because the highlight
//! layer is retired with the document rather than re-anchored against the new
//! one. A restore has to be the most undoable thing in the app, not the least.
//!
//! Going through Djot rather than plain text is what carries the italics, the
//! blockquotes and the scene breaks back with the words.
//!
//! ## Closed rows restore too
//!
//! Browsing a row's past does not require its tab to be open, and the common
//! case while browsing is that it is not. There is no `EditorHandle` then — but
//! there is still a document, because `OpenDocsStore::open` builds-or-reuses
//! one regardless of whether any widget is showing it. So the closed path is the
//! open path plus an explicit flush and release; it needs no backend use case of
//! its own.
//!
//! ## What can refuse, and why each one has to
//!
//! * The row is **gone** — trashed and purged, or never in this project.
//! * The recorded role has **no home** in what the row has since become. A
//!   promote can turn a Scene into a Note, and `SceneText` is not a role a Note
//!   has; [`skribisto_model::remap_content`] knows the swaps that *are* legal
//!   (title → title, `SceneText` ↔ `NoteText`), and `None` from it means the
//!   text would be dropped on the floor. Refusing by name beats writing nowhere.
//! * The remapped role is **not an editable text field** — a title lives in a
//!   `Signal<String>`, not a document, so it cannot take a Djot fragment.
//! * The **safety backup did not run**. See [`crate::backup::BackupSchedulerViewModel`]:
//!   `backup_now` returns early on three separate conditions, each with nothing
//!   but a toast, so calling it before a destructive write and assuming it
//!   happened is not a safety net at all.

use anyhow::Result;

use teksilo::text_document::{SelectionType, TextDocument};

use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};

use crate::models::{OpenDoc, OpenDocsStore};

/// What a restore is being asked to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreRequest {
    pub item_id: u64,
    /// The role as the **version** recorded it, which is not necessarily the
    /// role the row carries today.
    pub recorded_role: ContentRole,
    /// The Djot to put back.
    pub past: String,
    /// When that text was the row's text, for the confirmation and the toast.
    pub taken_at: chrono::DateTime<chrono::Utc>,
}

/// Why a restore is not going to happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreRefusal {
    /// The row no longer exists in this project.
    RowGone,
    /// The row has since become something with no room for this text.
    RoleHasNoHome { recorded: ContentRole },
    /// The role remaps, but not onto a field that holds a document.
    NotEditableText { target: ContentRole },
    /// The safety copy could not be taken, so the destructive write was not
    /// attempted.
    SafetyBackupDidNotRun,
    /// The write itself failed.
    WriteFailed { reason: String },
}

/// Which of an [`OpenDoc`]'s prose documents a content role is edited in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Main,
    Synopsis,
    Epigraph,
}

/// The document slot a content role is edited in, or `None` for the roles that
/// are not documents at all.
///
/// The two *names* — a book's title and subtitle — are edited as the item's
/// title fields and mirrored into their content rows on save, so they are plain
/// `Signal<String>`s with no document to insert a fragment into.
pub fn slot_for(role: &ContentRole) -> Option<Slot> {
    match role {
        ContentRole::SynopsisText => Some(Slot::Synopsis),
        ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText => {
            Some(Slot::Main)
        }
        ContentRole::EpigraphText => Some(Slot::Epigraph),
        ContentRole::BookTitle
        | ContentRole::PartTitle
        | ContentRole::ChapterTitle
        | ContentRole::BookSubtitle => None,
    }
}

/// Where a recorded role's text belongs in the row **as it is now**.
///
/// A row can have been promoted since the version was taken, so the role the
/// version recorded is a claim about the past, not about the present.
pub fn resolve_role(
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
    recorded: &ContentRole,
) -> Result<ContentRole, RestoreRefusal> {
    let target = skribisto_model::remap_content(role, sub_role, recorded).ok_or(
        RestoreRefusal::RoleHasNoHome {
            recorded: recorded.clone(),
        },
    )?;
    if slot_for(&target).is_none() {
        return Err(RestoreRefusal::NotEditableText { target });
    }
    Ok(target)
}

/// Replace a document's entire content with `past`, as **one** undoable edit.
///
/// The composite block is the point: without it the select-and-insert lands as
/// two entries and a careless Ctrl+Z leaves the row empty — the worst possible
/// halfway state for an operation whose promise is that it can be taken back.
pub fn replace_all(doc: &TextDocument, past: &str) -> Result<()> {
    let cursor = doc.cursor();
    cursor.begin_edit_block();
    cursor.select(SelectionType::Document);
    let result = cursor.insert_djot(past);
    // Ended even on failure: an unclosed composite would swallow every
    // subsequent edit in the session into one undo entry.
    cursor.end_edit_block();
    result.map_err(anyhow::Error::from)
}

/// The document a slot names, if this row has one.
pub fn document_for(doc: &OpenDoc, slot: Slot) -> Option<&TextDocument> {
    let field = match slot {
        Slot::Main => doc.main.as_ref(),
        Slot::Synopsis => doc.synopsis.as_ref(),
        Slot::Epigraph => doc.epigraph.as_ref(),
    };
    field.map(|f| &f.doc)
}

/// The `Content` row id behind a slot — what a comment on that text anchors to.
pub fn content_id_for(doc: &OpenDoc, slot: Slot) -> Option<u64> {
    match slot {
        Slot::Main => doc.main_content_id(),
        Slot::Synopsis => doc.synopsis_content_id(),
        // No comment layer is created on an epigraph (quoted matter is not the
        // author's own text to annotate), so nothing can be anchored there.
        Slot::Epigraph => None,
    }
}

/// Put `past` back into `item_id`'s `target` role and persist it.
///
/// Works whether or not the row is open: `open` builds-or-reuses the document,
/// and the explicit flush is what makes the closed case land on disk — `release`
/// only flushes on the *last* reference, which a row with a visible tab does not
/// reach.
pub fn apply(
    docs: &OpenDocsStore,
    item_id: u64,
    target: &ContentRole,
    past: &str,
    stack: Option<u64>,
) -> Result<(), RestoreRefusal> {
    let Some(slot) = slot_for(target) else {
        return Err(RestoreRefusal::NotEditableText {
            target: target.clone(),
        });
    };
    let Some(doc) = docs.open(item_id) else {
        return Err(RestoreRefusal::RowGone);
    };
    let outcome = (|| -> Result<()> {
        let document = document_for(&doc, slot)
            .ok_or_else(|| anyhow::anyhow!("this row has no {target:?} to restore into"))?;
        replace_all(document, past)?;
        // Explicitly, before the release below: a row with an open tab still has
        // references, so `release` would not flush it and the restore would sit
        // in memory until something else happened to save.
        doc.flush(stack)
    })();
    docs.release(item_id, stack);
    outcome.map_err(|e| RestoreRefusal::WriteFailed {
        reason: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── the write itself ────────────────────────────────────────────────────

    #[test]
    fn the_past_replaces_the_present_entirely() {
        let doc = TextDocument::new();
        doc.set_djot_sync("What the scene says now.").unwrap();
        replace_all(&doc, "What it said in March.").unwrap();
        assert_eq!(doc.to_plain_text().unwrap(), "What it said in March.");
    }

    /// The property the whole operation rests on: it can be taken back, and by
    /// **one** Ctrl+Z rather than by however many edits the replacement happened
    /// to decompose into.
    #[test]
    fn one_undo_brings_the_present_back() {
        let doc = TextDocument::new();
        doc.set_djot_sync("What the scene says now.").unwrap();
        replace_all(&doc, "What it said in March.").unwrap();

        doc.undo().unwrap();
        assert_eq!(
            doc.to_plain_text().unwrap(),
            "What the scene says now.",
            "a single undo must restore the text the writer had, not half of it",
        );
    }

    /// A restore that flattened the writer's formatting would be a different
    /// kind of loss, quieter and harder to notice.
    #[test]
    fn formatting_survives_the_round_trip() {
        let doc = TextDocument::new();
        doc.set_djot_sync("Plain.").unwrap();
        replace_all(&doc, "She said *nothing* at all.").unwrap();

        let djot = doc.to_djot().unwrap();
        assert!(
            djot.contains("*nothing*") || djot.contains("_nothing_"),
            "the emphasis did not survive: {djot}",
        );
        assert_eq!(doc.to_plain_text().unwrap(), "She said nothing at all.");
    }

    #[test]
    fn several_paragraphs_and_a_blockquote_come_back_whole() {
        let doc = TextDocument::new();
        doc.set_djot_sync("One line.").unwrap();
        replace_all(
            &doc,
            "First paragraph.\n\n> A quotation.\n\nLast paragraph.",
        )
        .unwrap();
        let text = doc.to_plain_text().unwrap();
        for part in ["First paragraph.", "A quotation.", "Last paragraph."] {
            assert!(text.contains(part), "{part:?} missing from {text:?}");
        }
    }

    /// Restoring an empty past is a legitimate request — the row was blank then.
    #[test]
    fn restoring_an_empty_past_empties_the_row_and_is_still_undoable() {
        let doc = TextDocument::new();
        doc.set_djot_sync("Something.").unwrap();
        replace_all(&doc, "").unwrap();
        assert_eq!(doc.to_plain_text().unwrap().trim(), "");
        doc.undo().unwrap();
        assert_eq!(doc.to_plain_text().unwrap(), "Something.");
    }

    // ── which field, and whether there is one ───────────────────────────────

    #[test]
    fn every_prose_role_names_a_document_and_every_name_role_names_none() {
        assert_eq!(slot_for(&ContentRole::SceneText), Some(Slot::Main));
        assert_eq!(slot_for(&ContentRole::NoteText), Some(Slot::Main));
        assert_eq!(slot_for(&ContentRole::ParatextText), Some(Slot::Main));
        assert_eq!(slot_for(&ContentRole::SynopsisText), Some(Slot::Synopsis));
        assert_eq!(slot_for(&ContentRole::EpigraphText), Some(Slot::Epigraph));
        // The names are `Signal<String>`s, not documents.
        for role in [
            ContentRole::BookTitle,
            ContentRole::PartTitle,
            ContentRole::ChapterTitle,
            ContentRole::BookSubtitle,
        ] {
            assert_eq!(slot_for(&role), None, "{role:?} is not a document");
        }
    }

    /// The trap this guard exists for: a row promoted between the backup and now.
    #[test]
    fn a_scene_promoted_to_a_note_restores_its_prose_into_the_notes_field() {
        let target = resolve_role(
            &BinderItemRole::Item,
            &BinderItemSubRole::Note,
            &ContentRole::SceneText,
        )
        .expect("a note has somewhere to put a scene's prose");
        assert_eq!(target, ContentRole::NoteText);
        assert_eq!(slot_for(&target), Some(Slot::Main));
    }

    #[test]
    fn a_role_with_no_home_in_what_the_row_became_is_refused_by_name() {
        // A Part carries a title and an epigraph — it has no prose of its own,
        // so a scene's text would be dropped rather than written anywhere.
        let err = resolve_role(
            &BinderItemRole::Folder,
            &BinderItemSubRole::Part,
            &ContentRole::SceneText,
        )
        .expect_err("a part has nowhere to put a scene's prose");
        assert_eq!(
            err,
            RestoreRefusal::RoleHasNoHome {
                recorded: ContentRole::SceneText
            },
        );
    }

    #[test]
    fn a_synopsis_restores_into_a_synopsis_wherever_the_row_ended_up() {
        for (role, sub) in [
            (BinderItemRole::Item, BinderItemSubRole::Scene),
            (BinderItemRole::Item, BinderItemSubRole::Note),
            (BinderItemRole::Folder, BinderItemSubRole::ChapterScene),
        ] {
            let target = resolve_role(&role, &sub, &ContentRole::SynopsisText)
                .unwrap_or_else(|e| panic!("{role:?}/{sub:?} refused a synopsis: {e:?}"));
            assert_eq!(target, ContentRole::SynopsisText);
        }
    }

    /// A title remaps to a title, which is a legal remap and still not something
    /// this can write — the refusal has to distinguish the two.
    #[test]
    fn a_title_remaps_but_is_still_refused_as_not_a_document() {
        let err = resolve_role(
            &BinderItemRole::Folder,
            &BinderItemSubRole::Part,
            &ContentRole::ChapterTitle,
        )
        .expect_err("a title is not a document this can insert a fragment into");
        assert!(
            matches!(err, RestoreRefusal::NotEditableText { .. }),
            "expected a not-a-document refusal, got {err:?}",
        );
    }
}
