// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Binder icons — the single `sub_role → icon` decision, shared by the outline
//! tree (view) and the editor tabs (view-model) so both surfaces stay in sync.
//!
//! The icon is chosen purely by `sub_role` (the writing model's meaningful
//! structural axis; role Folder/Item is UI-only): a structural folder inherits
//! its sub_role's glyph (a Book folder shows `book.svg`), a generic folder
//! (`sub_role == None`) shows `folder.svg`. Icons default to `TextRole::Primary`
//! and follow the theme; callers may re-tint via `IconWidget::color`.
//!
//! A chapter shows the same glyph in both of its encodings — they are one concept
//! (`ChapterScene`), differing only on the UI-only `role` axis, and the tree's
//! expand chevron already shows which one contains its scenes.
//!
//! `res!` embeds each asset at compile time and needs a literal path per call
//! site, so the mapping is a `match` with one `res!(...)` per arm.

use bastyde::res;
use bastyde::widgets::IconWidget;
use frontend::common::entities::BinderItemSubRole;

/// Leading-icon size (dp) — matches the outline rows and the tab headers.
pub const ICON_SIZE: f32 = 16.0;

/// Icon for a binder item, chosen purely by its `sub_role`.
pub fn sub_role_icon(sub_role: &BinderItemSubRole) -> IconWidget {
    let svg = match sub_role {
        BinderItemSubRole::Text => res!("assets/icons/binder/text.svg"),
        BinderItemSubRole::None => res!("assets/icons/binder/folder.svg"),
        BinderItemSubRole::Note => res!("assets/icons/binder/note.svg"),
        BinderItemSubRole::Book => res!("assets/icons/binder/book.svg"),
        BinderItemSubRole::Part => res!("assets/icons/binder/part.svg"),
        BinderItemSubRole::Scene => res!("assets/icons/binder/scene.svg"),
        BinderItemSubRole::ChapterScene => res!("assets/icons/binder/chapter.svg"),
        BinderItemSubRole::BookBegin => res!("assets/icons/binder/book-begin.svg"),
        BinderItemSubRole::BookEnd => res!("assets/icons/binder/book-end.svg"),
    };
    IconWidget::from_svg_icon(svg).icon_size(ICON_SIZE)
}

/// Icon for a binder root row (outline `kind == "binder"`).
pub fn binder_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/binder/binder.svg")).icon_size(ICON_SIZE)
}

/// Icon for a logical "Create" type — reuses the same `sub_role` glyphs the tree
/// shows, so a "+ Chapter" in the menu matches the chapter rows below it.
pub fn create_type_icon(t: skribisto_model::CreateType) -> IconWidget {
    use skribisto_model::CreateType;
    let sub_role = match t {
        CreateType::Book => BinderItemSubRole::Book,
        CreateType::Part => BinderItemSubRole::Part,
        CreateType::Chapter => BinderItemSubRole::ChapterScene,
        CreateType::Scene => BinderItemSubRole::Scene,
        CreateType::Note | CreateType::NoteFolder => BinderItemSubRole::Note,
        CreateType::Folder => BinderItemSubRole::None,
        CreateType::EndOfBook => BinderItemSubRole::BookEnd,
    };
    sub_role_icon(&sub_role)
}
