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
//!
//! # Story bible entries
//!
//! One glyph is not chosen by `sub_role` alone. A note item wearing a discoverable
//! ("find in prose") tag is a story bible entry, and shows `story-bible-entry.svg`: the
//! Note bubble made solid, with its corner set apart. It is still a note (same storage,
//! same editor), so it keeps the bubble and its tail, and differs by ink mass, the one
//! difference this set trusts at 16 px. Which items qualify is
//! [`skribisto_model::story_bible`]'s decision, never this module's: a row only says
//! whether its item [`is an entry`](skribisto_model::story_bible::is_entry), and
//! [`item_icon`] applies the rule.
//!
//! Entry status is *derived*: turning "find in prose" off on a tag changes it for every
//! item wearing that tag, without any item changing. So a surface that stays on screen
//! has to hear about tag edits. The binder trees carry the answer on their rows and
//! reload on tag events; the corkboard and the overview, which already repaint their
//! tag dots from the live palette, use [`LiveItemIcon`] over that same palette.

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use teksilo::core::BindingLevel;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::IconWidget;

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

/// The story bible entry glyph: a note item that wears a discoverable tag.
pub fn story_bible_entry_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/binder/story-bible-entry.svg"))
        .icon_size(ICON_SIZE)
}

/// Icon for a binder item that knows whether it is a story bible entry.
///
/// The entry-aware counterpart of [`role_sub_role_icon`], and what every row that can
/// tell should call: it shows [`story_bible_entry_icon`] exactly when
/// [`skribisto_model::story_bible::shows_entry_glyph`] says so, and the type's own glyph
/// otherwise. A caller that cannot tell (no tags on its rows) keeps calling
/// [`role_sub_role_icon`], which is the same as passing `is_entry = false`.
pub fn item_icon(
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
    is_entry: bool,
) -> IconWidget {
    if skribisto_model::story_bible::shows_entry_glyph(role, sub_role, is_entry) {
        story_bible_entry_icon()
    } else {
        role_sub_role_icon(role, sub_role)
    }
}

/// [`item_icon`] for the tree rows, which carry their role as the string their data
/// source speaks (`"binder" | "folder" | "item"`).
///
/// One translation, here, rather than a `role()` on each of the node types that would
/// each have to agree with the others.
pub fn kind_item_icon(kind: &str, sub_role: &BinderItemSubRole, is_entry: bool) -> IconWidget {
    item_icon(&role_of_kind(kind), sub_role, is_entry)
}

/// The accessible description of a tree row that may wear the story bible entry glyph.
///
/// A screen reader hears a row's title as its name and its label (the subtitle) as its
/// description, and never the icon, so the one thing the entry glyph says would be lost
/// to it. `None` whenever [`item_icon`] would not show the glyph, which leaves the row's
/// own description (its label) exactly as it was; `Some` replaces it with one that names
/// the entry first and still carries the label. Same rule as the icon, so a row never
/// says aloud what it does not show.
pub fn entry_access_description(
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
    is_entry: bool,
    label: &str,
) -> Option<LocalizedString> {
    if !skribisto_model::story_bible::shows_entry_glyph(role, sub_role, is_entry) {
        return None;
    }
    Some(if label.is_empty() {
        tr!(binder_row_story_bible_entry())
    } else {
        tr!(binder_row_story_bible_entry_with_label(
            label = label.to_string()
        ))
    })
}

/// Box a tree row, giving it [`entry_access_description`] when it wears the entry glyph.
///
/// The one way every binder-shaped tree (the binder, its pickers, Go to, the trash)
/// finishes a row, so none of them shows the glyph and stays silent about it.
pub fn describe_entry_row<W: Widget + 'static>(
    row: W,
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
    is_entry: bool,
    label: &str,
) -> Box<dyn Widget> {
    match entry_access_description(role, sub_role, is_entry, label) {
        Some(description) => Box::new(row.access_description(description)),
        None => Box::new(row),
    }
}

/// The role a tree row's `kind` string stands for.
pub(crate) fn role_of_kind(kind: &str) -> BinderItemRole {
    if kind == "item" {
        BinderItemRole::Item
    } else {
        // A binder row is not an item at all, and reads as a folder because that is what
        // it behaves like: a container with children.
        BinderItemRole::Folder
    }
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
        // Structurally a Note (see the variant's own doc), and shown with the entry glyph
        // its rows will wear: the solid bubble is still a bubble, so the row in the menu
        // reads as kin to "Note" and matches what lands in the tree.
        CreateType::StoryBibleEntry => {
            return item_icon(&BinderItemRole::Item, &BinderItemSubRole::Note, true);
        }
    };
    role_sub_role_icon(&role, &sub_role)
}

/// A row icon that follows the project's live tag palette.
///
/// For a surface whose rows carry their item's tags but are not rebuilt when a *tag*
/// changes: turning "find in prose" on or off for a tag makes every note wearing it an
/// entry, or a plain note again, with no item event at all. This binds the palette's
/// version and rebuilds its one glyph, the same way the tag dots beside it repaint
/// ([`crate::tags::tag_chip`]'s `ChipDots`).
///
/// Anything that can never wear the entry glyph (every type but a note item) is drawn as
/// its plain glyph and binds nothing, so a board full of scenes pays nothing for it.
pub struct LiveItemIcon {
    role: BinderItemRole,
    sub_role: BinderItemSubRole,
    tags: Vec<u64>,
    palette: crate::tags::TagsViewModel,
    size: f32,
    child: Option<WidgetId>,
}

impl LiveItemIcon {
    pub fn new(
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
        tags: Vec<u64>,
        palette: crate::tags::TagsViewModel,
    ) -> Self {
        Self {
            role,
            sub_role,
            tags,
            palette,
            size: ICON_SIZE,
            child: None,
        }
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Whether this row is an entry against the palette as it stands now.
    fn is_entry(&self) -> bool {
        let lookup = self.palette.lookup_signal().get();
        let discoverable = skribisto_model::story_bible::discoverable_tag_ids(
            lookup.values().map(|t| (t.id, t.discoverable)),
        );
        skribisto_model::story_bible::is_entry(&self.tags, &discoverable)
    }
}

impl std::fmt::Debug for LiveItemIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiveItemIcon").finish()
    }
}

impl Widget for LiveItemIcon {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let can_be_entry =
            skribisto_model::story_bible::shows_entry_glyph(&self.role, &self.sub_role, true);
        if can_be_entry {
            self.palette.changed_signal().bind_to(
                ctx.self_id(),
                ctx.binding_registry(),
                BindingLevel::Rebuild,
            );
        }
        let is_entry = can_be_entry && self.is_entry();
        let icon = item_icon(&self.role, &self.sub_role, is_entry).icon_size(self.size);
        self.child = Some(ctx.add(icon));
        self.child.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A screen reader is told exactly what the icon shows, and nothing it does not.
    #[test]
    fn only_a_row_wearing_the_entry_glyph_describes_itself_as_an_entry() {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{Note, Scene};
        assert!(entry_access_description(&Item, &Note, false, "").is_none());
        assert!(entry_access_description(&Folder, &Note, true, "").is_none());
        assert!(entry_access_description(&Item, &Scene, true, "").is_none());
        assert!(entry_access_description(&Item, &Note, true, "").is_some());
    }

    /// The description replaces the row's own, which is its label: so it has to carry
    /// the label on, or describing the entry would silence it.
    #[test]
    fn an_entry_description_keeps_the_rows_label() {
        let described = entry_access_description(
            &BinderItemRole::Item,
            &BinderItemSubRole::Note,
            true,
            "the smuggler",
        )
        .expect("an entry")
        .resolve_now();
        assert!(described.contains("the smuggler"), "{described}");
        let bare =
            entry_access_description(&BinderItemRole::Item, &BinderItemSubRole::Note, true, "")
                .expect("an entry")
                .resolve_now();
        assert!(!bare.is_empty() && !bare.contains(','), "{bare}");
    }
}
