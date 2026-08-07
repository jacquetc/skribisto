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
//!
//! # Drawing rules for this set
//!
//! These glyphs ship at [`ICON_SIZE`] — 16 dp, which on a 1× display is 16
//! physical pixels. That is small enough that two of the obvious ways to tell
//! icons apart simply do not survive rasterisation, and both had produced real
//! collisions here before:
//!
//! * **Differentiate by silhouette or by ink mass, never by counting interior
//!   strokes.** Anything closer than about two units merges into one grey smear,
//!   so "one bar vs two bars" is not a distinction at this size. Scene and
//!   Chapter share a frame and are separated by two thin strokes against one
//!   solid band; Part is the set's only diagonal; Note keeps its tail.
//! * **Never distinguish a pair by mirroring it.** Left/right mirrored glyphs are
//!   the hardest pair to tell apart at a glance, and RTL layout flips them again.
//!   BookBegin and BookEnd are unrelated shapes for exactly this reason.
//! * **Axis-aligned strokes belong on the half-grid**, at `stroke-width: 1` with
//!   butt caps: a 1-wide stroke centred on `y.5` covers exactly one pixel row.
//!   Off-grid strokes render at inconsistent weight — `text.svg` used to draw
//!   four identical bars as black, grey, black, grey. Filled shapes take integer
//!   bounds for the same reason. One weight (1) across the whole set.
//! * **Check both themes.** Light-on-dark blooms, so a gap that just barely reads
//!   on the light theme closes up on the dark one. Every collision this set has
//!   had showed up worse in dark.
//!
//! The tree gives an icon the help of indentation and an expand chevron; the
//! editor tab headers, which share this mapping, give it none. Judge a new glyph
//! in the tab strip, where it stands alone.

use teksilo::res;
use teksilo::widgets::IconWidget;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

/// Leading-icon size (dp) — matches the outline rows and the tab headers.
pub const ICON_SIZE: f32 = 16.0;

/// Icon for a binder item, chosen by its `sub_role`.
///
/// The leaf reading. Every sub_role but one looks the same whether it is carried by an
/// item or a folder — a notes folder and a note share a glyph, and always have — so this
/// is the right answer nearly everywhere. Paratext is the exception, and callers that
/// know the role should use [`role_sub_role_icon`].
pub fn sub_role_icon(sub_role: &BinderItemSubRole) -> IconWidget {
    role_sub_role_icon(&BinderItemRole::Item, sub_role)
}

/// Icon for a binder item, chosen by its `(role, sub_role)`.
///
/// Only paratexts distinguish the two, and they have to: a paratext folder and the pages
/// inside it are both new, both unfamiliar, and sat next to each other in the same
/// subtree — with one glyph between them the tree read as a list of identical rows.
/// They differ by ink mass (a line for the leaf, a band for the container), the same
/// device that separates Scene from Chapter, because a raster at 16px preserves mass and
/// destroys stroke counts.
pub fn role_sub_role_icon(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> IconWidget {
    if matches!(
        (role, sub_role),
        (BinderItemRole::Folder, BinderItemSubRole::Paratext)
    ) {
        return IconWidget::from_svg_icon(res!("assets/icons/binder/paratext-folder.svg"))
            .icon_size(ICON_SIZE);
    }
    let svg = match sub_role {
        BinderItemSubRole::Text => res!("assets/icons/binder/text.svg"),
        BinderItemSubRole::None => res!("assets/icons/binder/folder.svg"),
        BinderItemSubRole::Note => res!("assets/icons/binder/note.svg"),
        BinderItemSubRole::Paratext => res!("assets/icons/binder/paratext.svg"),
        BinderItemSubRole::Book => res!("assets/icons/binder/book.svg"),
        BinderItemSubRole::Part => res!("assets/icons/binder/part.svg"),
        BinderItemSubRole::Scene => res!("assets/icons/binder/scene.svg"),
        BinderItemSubRole::ChapterScene => res!("assets/icons/binder/chapter.svg"),
        BinderItemSubRole::BookBegin => res!("assets/icons/binder/book-begin.svg"),
        BinderItemSubRole::BookEnd => res!("assets/icons/binder/book-end.svg"),
    };
    IconWidget::from_svg_icon(svg).icon_size(ICON_SIZE)
}

/// [`role_sub_role_icon`] for the tree rows, which carry their role as the string their
/// data source speaks (`"binder" | "folder" | "item"`).
///
/// One translation, here, rather than a `role()` on each of the five node types that
/// would each have to agree with the others.
pub fn kind_sub_role_icon(kind: &str, sub_role: &BinderItemSubRole) -> IconWidget {
    let role = if kind == "item" {
        BinderItemRole::Item
    } else {
        // A binder row is not an item at all, and reads as a folder because that is what
        // it behaves like: a container with children.
        BinderItemRole::Folder
    };
    role_sub_role_icon(&role, sub_role)
}

/// Icon for a binder root row (outline `kind == "binder"`).
pub fn binder_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/binder/binder.svg")).icon_size(ICON_SIZE)
}

/// Icon for a logical "Create" type — reuses the same `sub_role` glyphs the tree
/// shows, so a "+ Chapter" in the menu matches the chapter rows below it.
pub fn create_type_icon(t: skribisto_model::CreateType) -> IconWidget {
    use skribisto_model::CreateType;
    // The role matters for exactly one pair, so it is carried alongside the sub_role
    // rather than derived: a "+ Paratext folder" row must show the container glyph, or
    // the menu promises one thing and the tree shows another.
    let (role, sub_role) = match t {
        CreateType::Book => (BinderItemRole::Folder, BinderItemSubRole::Book),
        CreateType::Part => (BinderItemRole::Folder, BinderItemSubRole::Part),
        CreateType::Chapter => (BinderItemRole::Folder, BinderItemSubRole::ChapterScene),
        CreateType::Scene => (BinderItemRole::Item, BinderItemSubRole::Scene),
        CreateType::Note => (BinderItemRole::Item, BinderItemSubRole::Note),
        CreateType::NoteFolder => (BinderItemRole::Folder, BinderItemSubRole::Note),
        CreateType::Paratext => (BinderItemRole::Item, BinderItemSubRole::Paratext),
        CreateType::ParatextFolder => (BinderItemRole::Folder, BinderItemSubRole::Paratext),
        CreateType::Folder => (BinderItemRole::Folder, BinderItemSubRole::None),
        CreateType::EndOfBook => (BinderItemRole::Item, BinderItemSubRole::BookEnd),
    };
    role_sub_role_icon(&role, &sub_role)
}
