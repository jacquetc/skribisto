//! Binder icons — the single `sub_role → icon` decision, shared by the outline
//! tree (view) and the editor tabs (view-model) so both surfaces stay in sync.
//!
//! The icon is chosen purely by `sub_role` (the writing model's meaningful
//! structural axis; role Folder/Item is UI-only): a structural folder inherits
//! its sub_role's glyph (a Book folder shows `book.svg`), a generic folder
//! (`sub_role == None`) shows `folder.svg`. Icons default to `TextRole::Primary`
//! and follow the theme; callers may re-tint via `IconWidget::color`.
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
        BinderItemSubRole::Chapter => res!("assets/icons/binder/chapter.svg"),
        BinderItemSubRole::Scene => res!("assets/icons/binder/scene.svg"),
        BinderItemSubRole::ChapterScene => res!("assets/icons/binder/chapter-scene.svg"),
        BinderItemSubRole::BookBegin => res!("assets/icons/binder/book-begin.svg"),
        BinderItemSubRole::BookEnd => res!("assets/icons/binder/book-end.svg"),
    };
    IconWidget::from_svg_icon(svg).icon_size(ICON_SIZE)
}

/// Icon for a binder root row (outline `kind == "binder"`).
pub fn binder_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/binder/binder.svg")).icon_size(ICON_SIZE)
}
