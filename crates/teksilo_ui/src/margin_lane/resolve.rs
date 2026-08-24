// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Every enabled provider's marks, for one surface.** The one place a provider's
//! output is turned into something the widget draws.
//!
//! Three switches decide whether a provider runs at all, and they are checked in
//! this order because each makes the next irrelevant: the lane itself (the View
//! menu's switch), this surface, and this provider. A writer who turned the lane
//! off must not be paying for a document walk on every keystroke.
//!
//! ## What the host takes back from a provider
//!
//! A provider decides **where** its marks are, **what they say**, and which of the
//! shape classes each one is. It does not decide its **colour** or its **group**,
//! and those are overwritten here whatever it returned:
//!
//! * Colour, because a provider naming one could ship a mark that fails contrast on
//!   a theme it never saw — and because nothing may paint a red stripe down the
//!   side of a manuscript. A provider names a palette slot; the theme decides what
//!   that looks like.
//! * Group, because it is half of a mark's accessibility node identity and all of
//!   what may merge with what. Two providers each numbering their marks from one
//!   would otherwise collide, and a screen reader would be handed two different
//!   things under one node id.

use crate::widgets::LaneMark;
use teksilo::tokens::ColorTokens;

use super::{LaneContext, resolve_slot};

/// A provider's stable group ordinal.
///
/// Hashed from the id rather than taken from its position in the registry: a writer
/// toggling one provider off would otherwise renumber every provider after it, and
/// with it the accessibility node identity of every mark they own. The id is
/// already frozen the moment a provider ships, so this is frozen with it.
///
/// A collision between two providers is possible and harmless on its own: `group`
/// only decides paint order and merge eligibility, and two marks merge only when
/// their column and shape match as well.
pub fn group_of(id: &str) -> u16 {
    // FNV-1a, 16 bits. A named, stable, trivially re-implementable hash rather than
    // `DefaultHasher`, whose output std explicitly does not promise across releases
    // — and this number reaches an accessibility tree that has to hold still.
    let mut hash: u32 = 0x811c_9dc5;
    for byte in id.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    ((hash >> 16) ^ hash) as u16
}

/// Whether the lane is drawn on `surface` at all.
///
/// Read separately from [`marks`] because the host needs it before it builds a
/// context: a surface the writer turned off should not be resolving geometry or
/// walking a document to find out it has nothing to show.
pub fn is_enabled(store: &teksilo::settings::SettingsStore, surface: super::LaneSurface) -> bool {
    store
        .signal(
            crate::MARGIN_LANE_ENABLED_KEY,
            crate::MARGIN_LANE_ENABLED_DEFAULT,
        )
        .get()
        && store
            .signal(
                &crate::margin_lane_surface_key(surface),
                crate::margin_lane_surface_default(surface),
            )
            .get()
}

/// Whether the texture column is drawn.
pub fn texture_enabled(store: &teksilo::settings::SettingsStore) -> bool {
    store
        .signal(
            crate::MARGIN_LANE_TEXTURE_KEY,
            crate::MARGIN_LANE_TEXTURE_DEFAULT,
        )
        .get()
}

/// Every enabled provider's marks for one surface, colour- and group-normalised.
///
/// `build_context` is handed the provider's own resolved colour and group and must
/// return the context to call it with. A closure rather than a plain `&LaneContext`
/// because those two fields differ per provider, and building the whole context
/// once per provider is what lets the caller keep everything else in it borrowed.
pub fn marks<'a>(
    store: &teksilo::settings::SettingsStore,
    colors: &ColorTokens,
    surface: super::LaneSurface,
    call: impl Fn(&super::LaneProviderSpec, teksilo::tokens::Color, u16) -> Vec<LaneMark> + 'a,
) -> Vec<LaneMark> {
    if !is_enabled(store, surface) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for spec in super::registered_for(surface) {
        if !store.signal(&spec.settings_key(), spec.default_on).get() {
            continue;
        }
        let color = resolve_slot(colors, spec.palette_slot);
        let group = group_of(&spec.id);
        out.extend(call(&spec, color, group).into_iter().map(|mut mark| {
            // Overwritten rather than trusted. A provider that got these right sees
            // no difference; one that did not cannot paint outside the theme.
            mark.color = color;
            mark.group = group;
            mark
        }));
    }
    // Ascending group, so paint order is decided by the id rather than by the order
    // extensions happened to install in. Within a group the provider's own order is
    // preserved, which for search hits and boundaries is document order.
    out.sort_by_key(|m| m.group);
    out
}

/// The context a provider is called with, for the common case of one editor
/// mapping the whole extent.
///
/// A convenience over building [`LaneContext`] by hand, and the shape a caller
/// should copy for the stream case, where the extent is a slice per row.
pub struct LaneCall<'a> {
    pub app_ctx: &'a std::rc::Rc<frontend::AppContext>,
    pub ids: &'a crate::app_ids::AppIds,
    pub surface: super::LaneSurface,
    pub doc: &'a teksilo::text_document::TextDocument,
    pub item_id: common::types::EntityId,
    /// Which field `doc` is -- see [`LaneContext::kind`](super::LaneContext::kind).
    pub kind: crate::format::EditorKind,
    pub comment_anchors: &'a [super::CommentAnchor],
    pub misspellings: &'a [(usize, usize)],
    pub locate: &'a dyn Fn(usize) -> Option<f32>,
}

impl LaneCall<'_> {
    /// Run one provider.
    pub fn run(
        &self,
        spec: &super::LaneProviderSpec,
        color: teksilo::tokens::Color,
        group: u16,
    ) -> Vec<LaneMark> {
        let ctx = LaneContext {
            app_ctx: self.app_ctx,
            ids: self.ids,
            surface: self.surface,
            doc: self.doc,
            item_id: self.item_id,
            kind: self.kind,
            comment_anchors: self.comment_anchors,
            misspellings: self.misspellings,
            color,
            group,
            locate: self.locate,
        };
        (spec.marks)(&ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The number reaches an accessibility tree, so it has to be the same number on
    /// every run of every build. Asserted against literals rather than against
    /// itself, which is what would catch a hash function quietly swapped out.
    #[test]
    fn a_providers_group_is_the_same_number_every_time() {
        assert_eq!(group_of("comments"), group_of("comments"));
        assert_eq!(group_of("search"), 0xa89a);
        assert_eq!(group_of("comments"), 0x8a51);
        assert_eq!(group_of("boundaries"), 0x4782);
        assert_eq!(group_of("spelling"), 0x4281);
    }

    /// Toggling one provider must not renumber the others: the ids are what the
    /// numbers come from, and a writer turning off comments would otherwise move
    /// every search hit to a different accessibility node.
    #[test]
    fn the_group_does_not_depend_on_what_else_is_registered() {
        let before = group_of("search");
        let _h = super::super::register_lane_provider(
            "test.resolve",
            super::super::LaneProviderSpec {
                id: "resolve-probe".to_string(),
                label: std::rc::Rc::new(|| teksilo::prelude::lit!("Probe")),
                hint: std::rc::Rc::new(|| teksilo::prelude::lit!("Probe")),
                column: crate::widgets::LaneColumn::Left,
                shape: crate::widgets::LaneShape::Dot,
                palette_slot: 1,
                surfaces: &[super::super::LaneSurface::Editor],
                default_on: true,
                refresh: super::super::LaneRefresh::Manual,
                marks: std::rc::Rc::new(|_| Vec::new()),
            },
        );
        assert_eq!(group_of("search"), before);
    }
}
