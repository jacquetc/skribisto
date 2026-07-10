//! Localized labels + rich tooltips for the context-dependent "Create"
//! recommendations. Keeps `skribisto_model` UI-string-free: the model returns
//! `(role, sub_role, relation)`, and this module maps each to `tr!` keys — so
//! all i18n stays in `bastyde_ui`.
//!
//! Both the header [`CreateSplitButton`](crate::docks::create_split_button) and
//! the outline "Add ▸" context submenu render through these helpers, so a type's
//! name and tooltip read identically wherever it's offered.

use bastyde::i18n::LocalizedString;
use bastyde::prelude::*; // tr!
use bastyde::widgets::tooltip::TooltipContent;

use skribisto_model::{CreateType, Recommendation, Relation};

/// The human label for a logical [`CreateType`] — the SplitButton title text and
/// each menu row's label. Icons come from
/// [`binder_icons::create_type_icon`](crate::binder_icons::create_type_icon).
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

/// A rich tooltip describing what the recommendation creates and where it lands.
/// `anchor_title` is the selected/right-clicked row's title, or `None` at the
/// top level (no item anchor) — which selects the "at the top level" phrasing.
pub fn recommendation_tooltip(rec: &Recommendation, anchor_title: Option<&str>) -> TooltipContent {
    let kind = recommendation_label(rec.create_type).resolve_now();
    // A stable-ish key per (type, relation); inline content doesn't require
    // registry uniqueness, but a key keeps the sticky-tooltip identity sensible.
    let key = format!("create-tip-{:?}-{:?}", rec.create_type, rec.relation);
    let text = match (anchor_title, rec.relation) {
        (Some(t), Relation::Child) => {
            tr!(create_tooltip_child(kind = kind, target = t.to_string()))
        }
        (Some(t), Relation::Sibling) => {
            tr!(create_tooltip_sibling(kind = kind, target = t.to_string()))
        }
        (Some(t), Relation::ParentSibling) => {
            tr!(create_tooltip_parent_sibling(kind = kind, target = t.to_string()))
        }
        (None, _) => tr!(create_tooltip_top(kind = kind)),
    };
    TooltipContent::new(key, text)
}
