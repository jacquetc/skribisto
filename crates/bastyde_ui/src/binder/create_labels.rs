// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Localized labels + the placement hint + rich-tooltip *keys* for the
//! context-dependent "Create" / "Convert to" vocabulary. Keeps
//! `skribisto_model` UI-string-free: the model returns
//! `(role, sub_role, relation)`, and this module maps each to `tr!` keys, so
//! all i18n stays in `bastyde_ui`.
//!
//! The rich-tooltip *content* is not built here — it is registered once in
//! [`crate::tooltip_registry`] and referenced by key, so a type's explainer
//! reads identically across the header [`CreateSplitButton`](crate::docks::create_split_button),
//! the outline "Add ▸" submenu, and the "Convert to ▸" menu, and so cited types
//! cascade to their own tooltips. This module only maps a `CreateType` /
//! `PromoteTarget` to its registry key.

use bastyde::i18n::LocalizedString;
use bastyde::prelude::*; // tr!

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
        CreateType::EndOfBook => tr!(create_book_end()),
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
        (Folder, S::None) => tr!(create_folder()),
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
        CreateType::EndOfBook => tt::WM_END_OF_BOOK,
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
