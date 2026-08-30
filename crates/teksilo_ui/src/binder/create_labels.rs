// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Localized labels + the placement hint + rich-tooltip *keys* for the
//! context-dependent "Create" / "Convert to" vocabulary. Keeps
//! `skribisto_model` UI-string-free: the model returns
//! `(role, sub_role, relation)`, and this module maps each to `tr!` keys, so
//! all i18n stays in `teksilo_ui`.
//!
//! The rich-tooltip *content* is not built here — it is registered once in
//! [`crate::tooltip_registry`] and referenced by key, so a type's explainer
//! reads identically across the header [`CreateSplitButton`](crate::docks::create_split_button),
//! the outline "Add ▸" submenu, and the "Convert to ▸" menu, and so cited types
//! cascade to their own tooltips. This module only maps a `CreateType` /
//! `PromoteTarget` to its registry key.

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*; // tr!

use crate::tooltip_registry as tt;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use skribisto_model::{CreateType, PromoteTarget, Relation};

/// The human label for a logical [`CreateType`] — the SplitButton title text and
/// each menu row's label. Icons come from
/// [`icons::create_type_icon`](crate::binder::icons::create_type_icon).
pub fn recommendation_label(create_type: CreateType) -> LocalizedString {
    match create_type {
        CreateType::Book => tr!(create_book()),
        CreateType::Part => tr!(create_part()),
        CreateType::Chapter => tr!(create_chapter()),
        CreateType::Scene => tr!(create_scene()),
        CreateType::Note => tr!(create_note()),
        CreateType::NoteFolder => tr!(create_note_folder()),
        CreateType::Folder => tr!(create_folder()),
        CreateType::Paratext => tr!(create_paratext()),
        CreateType::ParatextFolder => tr!(create_paratext_folder()),
        CreateType::EndOfBook => tr!(create_book_end()),
        CreateType::StoryBibleEntry => tr!(create_story_bible_entry()),
    }
}

/// The per-type **placeholder name** for a freshly created row — "New chapter" for a
/// chapter, "New scene" for a scene, and so on, rather than one generic "New Item"
/// for everything that is not a folder.
///
/// ⚠ **Not, by itself, the title a row is stored with.** Every creation path calls
/// [`initial_title`] instead, which returns this for the types the writer names and
/// **nothing** for the three that open a structural level — because a stored
/// structural title is printed into the exported book, placeholder and all. The two
/// split apart the day that came to light; this half is the vocabulary, that half is
/// the decision.
///
/// Where it *is* stored, it produces *data*, not chrome. An entity title is persisted
/// to the `.skrib` file and is the writer's to edit, so it must be resolved to an
/// owned `String` **once, at creation time**, in whatever language is active then —
/// and never re-translated afterwards. Switching the app to French must not silently
/// retitle notes the writer created in English, any more than it should retitle ones
/// they named themselves.
///
/// That is why this returns a `LocalizedString` and the caller resolves it
/// immediately: the boundary between chrome and data is exactly the `create_item_at`
/// call. Contrast [`recommendation_label`], which stays localized all the way to the
/// widget precisely because a menu label *is* chrome — and which is what the import
/// wizard's review tree shows in the Title cell of a row stored untitled.
///
/// `EndOfBook` reuses the type name: it is a singleton structural marker the writer
/// does not name, so "New end of book" would be noise.
pub fn default_title(create_type: CreateType) -> LocalizedString {
    match create_type {
        CreateType::Book => tr!(new_item_book()),
        CreateType::Part => tr!(new_item_part()),
        CreateType::Chapter => tr!(new_item_chapter()),
        // Reuses the key `split_scene` already writes (`corkboard.rs` / `stream.rs`),
        // so a scene born from a split and one born from Create read the same.
        CreateType::Scene => tr!(new_scene_title()),
        CreateType::Note => tr!(new_item_note()),
        CreateType::NoteFolder => tr!(new_item_note_folder()),
        CreateType::Folder => tr!(new_item_folder()),
        CreateType::Paratext => tr!(new_item_paratext()),
        CreateType::ParatextFolder => tr!(new_item_paratext_folder()),
        CreateType::EndOfBook => tr!(create_book_end()),
        CreateType::StoryBibleEntry => tr!(new_item_story_bible_entry()),
    }
}

/// The title a freshly created row is **stored** with: [`default_title`] for every
/// row the writer names, and **nothing at all** for one that opens a structural
/// level (Book, Part, Chapter).
///
/// # Why a structural row is born untitled
///
/// A stored title is not private to the app: `heading_text` feeds it to
/// `HeadingScheme::NumberAndTitle`, the default scheme of every built-in export
/// preset, so whatever sits in that field is **printed into the finished book**.
/// A placeholder there reads, in a French project:
///
/// ```text
/// Chapitre 1 — New Chapter
/// ```
///
/// and `headings::is_redundant_number_title` cannot suppress it: that guard folds a
/// title against `"{word} {n}"` and the bare numeral only, so it sees no relation
/// between "New Chapter" and "Chapitre 1". Note this is **not** a locale bug — a
/// French interface fails identically, printing `Chapitre 1 — Nouveau chapitre`. The
/// defect is storing a placeholder in a field the exporter prints at all.
///
/// Nothing is lost by leaving it empty. `models::numbering::fallback_label_for` names
/// the row on every binder surface and `NumberAndTitle`'s number-only arm names it in
/// the book — both from the *manuscript's* language and the same numbering pass, so
/// the binder and the book agree. That is the whole reason that function exists.
///
/// This is the fix `new_work_uc::templates::manuscript_binder` already made for the
/// project-template path (its retired label slot 4 held the word "Chapter" and printed
/// `Chapter 1 — Chapitre 1`), carried across at last to the two live doors onto the
/// same field: ＋Create and the import wizard's synthetic root.
///
/// Every other type keeps its placeholder: a scene, note, folder or paratext title is
/// never composed into a generated heading, and `fallback_label_for` deliberately
/// returns `None` for them — so blanking those would leave genuinely nameless rows in
/// the binder.
pub fn initial_title(create_type: CreateType) -> String {
    match create_type {
        CreateType::Book | CreateType::Part | CreateType::Chapter => String::new(),
        other => default_title(other).into(),
    }
}

/// The label for a promote target — the rows of the "Convert to ▸" submenu.
///
/// Unlike the Create menu (which hides the two chapter encodings behind one "Chapter"),
/// Convert is exactly where the writer chooses between them, so they get distinct
/// labels.
pub fn promote_target_label(target: PromoteTarget) -> LocalizedString {
    use PromoteTarget as T;
    match target {
        T::Folder => tr!(create_folder()),
        T::ChapterFolder => tr!(promote_chapter_folder()),
        T::PartFolder => tr!(create_part()),
        T::BookFolder => tr!(create_book()),
        T::NoteFolder => tr!(create_note_folder()),
        T::FlatChapter => tr!(promote_flat_chapter()),
        T::Scene => tr!(create_scene()),
        T::Note => tr!(create_note()),
    }
}

/// The human name of an item's **type**, for the Overview table's Type column.
///
/// Answers "what is this row, structurally?" — so both encodings of a chapter read
/// "Chapter". That is the opposite of the "Convert to ▸" menu
/// ([`promote_target_label`]), which is precisely where the writer chooses *between* the
/// two encodings and so must name them apart. The distinction they draw is visible here
/// anyway: a chapter folder has a twist and children, a flat chapter does not.
///
/// Mirrors `skribisto_model::COMBINATIONS` — every valid pair has a name. An invalid pair
/// cannot occur (the matrix rejects it at creation), so the fallback exists only to keep
/// the function total.
pub fn item_type_label(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> LocalizedString {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole as S;
    match (role, sub_role) {
        (_, S::Book) => tr!(create_book()),
        (_, S::BookBegin) => tr!(type_book_start()),
        (_, S::BookEnd) => tr!(create_book_end()),
        (_, S::Part) => tr!(create_part()),
        (_, S::ChapterScene) => tr!(create_chapter()),
        (_, S::Scene) => tr!(create_scene()),
        (Folder, S::Note) => tr!(create_note_folder()),
        (Item, S::Note) => tr!(create_note()),
        (_, S::Text) => tr!(type_text()),
        (Folder, S::Paratext) => tr!(create_paratext_folder()),
        (Item, S::Paratext) => tr!(create_paratext()),
        (Folder, S::None) => tr!(create_folder()),
        // The catch-all is why a new sub_role reads as "Folder" until someone names it.
        // Every combination above is listed deliberately; add yours rather than leaving
        // it to fall through here.
        _ => tr!(create_folder()),
    }
}

/// The human name of a content role — for explaining what a conversion would discard
/// ("a Part has nowhere to keep: Scene text").
pub fn content_role_label(role: &ContentRole) -> LocalizedString {
    match role {
        ContentRole::SceneText => tr!(content_scene_text()),
        ContentRole::NoteText => tr!(content_note_text()),
        ContentRole::SynopsisText => tr!(synopsis()),
        ContentRole::BookTitle => tr!(content_book_title()),
        ContentRole::BookSubtitle => tr!(content_book_subtitle()),
        ContentRole::PartTitle => tr!(content_part_title()),
        ContentRole::ChapterTitle => tr!(content_chapter_title()),
        ContentRole::EpigraphText => tr!(content_epigraph_text()),
        ContentRole::ParatextText => tr!(content_paratext_text()),
    }
}

/// The [`crate::tooltip_registry`] key for a create type's rich tooltip. Both
/// the header SplitButton and the "Add ▸" submenu bind rows by this key
/// (`.rich_tooltip(key)`), so the row tooltip and every `[label](:key)` cascade
/// link that cites the type render identical, registered content.
pub fn recommendation_tooltip_key(create_type: CreateType) -> &'static str {
    match create_type {
        CreateType::Book => tt::WM_BOOK,
        CreateType::Part => tt::WM_PART,
        CreateType::Chapter => tt::WM_CHAPTER,
        CreateType::Scene => tt::WM_SCENE,
        CreateType::Note => tt::WM_NOTE,
        CreateType::NoteFolder => tt::WM_NOTE_FOLDER,
        CreateType::Folder => tt::WM_FOLDER,
        CreateType::Paratext => tt::WM_PARATEXT,
        CreateType::ParatextFolder => tt::WM_PARATEXT_FOLDER,
        CreateType::EndOfBook => tt::WM_END_OF_BOOK,
        CreateType::StoryBibleEntry => tt::WM_STORY_BIBLE_ENTRY,
    }
}

/// The registry key for a promote target's rich tooltip — reused by the
/// "Convert to ▸" menu. Both chapter encodings (`ChapterFolder`, `FlatChapter`)
/// point at the one `wm-chapter` tooltip, which already explains the two shapes.
pub fn promote_target_tooltip_key(target: PromoteTarget) -> &'static str {
    use PromoteTarget as T;
    match target {
        T::Folder => tt::WM_FOLDER,
        T::ChapterFolder | T::FlatChapter => tt::WM_CHAPTER,
        T::PartFolder => tt::WM_PART,
        T::BookFolder => tt::WM_BOOK,
        T::NoteFolder => tt::WM_NOTE_FOLDER,
        T::Scene => tt::WM_SCENE,
        T::Note => tt::WM_NOTE,
    }
}

/// The trailing "where it lands" hint shown on each create row — the placement
/// that used to be buried in the tooltip, now always visible. `anchor` is the
/// selected / right-clicked row's title, or `None` at the top level.
pub fn recommendation_placement(anchor: Option<&str>, relation: Relation) -> LocalizedString {
    match (anchor, relation) {
        (Some(_), Relation::Child) => tr!(placement_inside()),
        (Some(_), Relation::Sibling) => tr!(placement_after()),
        (Some(_), Relation::ParentSibling) => tr!(placement_after_parent()),
        (None, _) => tr!(placement_top_level()),
    }
}
