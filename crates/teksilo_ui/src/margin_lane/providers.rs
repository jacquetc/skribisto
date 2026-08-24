// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The community edition's own mark sources.
//!
//! Three, registered through the same door an extension uses, under the ids
//! [`builtin_ids`](super::builtin_ids) reserves. Going through the public registry
//! rather than a private list is deliberate: it is what keeps the settings page,
//! the enabled check and the draw order one code path for everybody, so a bug that
//! would only show up for an extension shows up here first.
//!
//! | Provider | Column | Shape | Where |
//! |---|---|---|---|
//! | Comments | Right | Square | Editor, stream |
//! | Search hits | Centre, and full width for the one you are on | Bar | Every surface |
//! | Document boundaries | Left | Rule | Streams only |
//!
//! Different columns, so none of them can ever fight another for a pixel.

use std::rc::Rc;

use common::types::EntityId;

use crate::widgets::{LaneColumn, LaneMark, LaneShape, LaneSpan};

use super::{LaneContext, LaneProviderHandle, LaneProviderSpec, LaneRefresh, LaneSurface, query};

/// The namespace the community edition registers its own providers under.
///
/// Not the bare crate name: a namespace is what a re-registration replaces and a
/// handle drop removes, so the built-ins share one and are installed together.
const NAMESPACE: &str = "skribisto.builtin.lane";

/// Registers every built-in provider and returns the handles that keep them alive.
///
/// Handles, plural and returned rather than leaked, because a registration that
/// outlives the thing it draws for is exactly the bug the drop-handle shape exists
/// to prevent. The application holds these for as long as it runs.
pub fn install() -> Vec<LaneProviderHandle> {
    // Each takes its own namespace segment. Sharing one would make them replace
    // each other on registration, which is silent: the last one registered would be
    // the only one that ever appeared, with no error and nothing to grep for.
    [
        (format!("{NAMESPACE}.comments"), comments()),
        (format!("{NAMESPACE}.search"), search()),
        (format!("{NAMESPACE}.boundaries"), boundaries()),
    ]
    .into_iter()
    .filter_map(
        |(ns, spec)| match super::register_builtin_lane_provider(ns, spec) {
            Ok(handle) => Some(handle),
            // A built-in that cannot register is a programming error in this crate, not
            // a condition to recover from; `install_builtin_providers`'s own test is
            // what catches it. Refusing to start the application over a margin strip
            // would be a worse answer than one missing column.
            Err(why) => {
                eprintln!("margin lane: a built-in provider was refused: {why}");
                None
            }
        },
    )
    .collect()
}

// ── comments ─────────────────────────────────────────────────────────────────

/// Where the writer, or an editor, left a note.
///
/// Reads [`LaneContext::comment_anchors`], never the stored `range_start` on the
/// comment row. The stored offsets are only rewritten when some editor's comment
/// margin rebuilds; the anchors handed down here are the ones the highlight session
/// shifts on every keystroke, so a mark stays on its sentence while the writer types
/// above it instead of drifting a paragraph at a time.
fn comments() -> LaneProviderSpec {
    LaneProviderSpec {
        id: "comments".to_string(),
        label: Rc::new(|| crate::tr!(margin_lane_provider_comments())),
        hint: Rc::new(|| crate::tr!(margin_lane_provider_comments_hint())),
        column: LaneColumn::Right,
        shape: LaneShape::Square,
        palette_slot: 3,
        surfaces: &[LaneSurface::Editor, LaneSurface::Stream],
        default_on: true,
        // The set of comments changes; the prose does not move them, because the
        // anchors move themselves. Binding to document changes instead would
        // recompute on every keystroke to produce the same marks.
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|ctx| {
            ctx.comment_anchors
                .iter()
                .filter_map(|a| {
                    let start = (ctx.locate)(a.start)?;
                    let end = (ctx.locate)(a.end)?;
                    Some(LaneMark {
                        id: a.id,
                        span: LaneSpan::new(start, end),
                        column: LaneColumn::Right,
                        shape: LaneShape::Square,
                        color: ctx.color,
                        label: a.label.clone(),
                        group: ctx.group,
                    })
                })
                .collect()
        }),
    }
}

// ── search hits ──────────────────────────────────────────────────────────────

/// Every hit for whatever the writer last searched for.
///
/// The hit the writer is standing on spans the lane's full width; the rest sit in
/// the centre column. A width difference rather than a colour one, because the
/// distinction has to survive a reader who cannot separate two hues, and at three
/// pixels tall hue is a weak channel even with normal vision.
fn search() -> LaneProviderSpec {
    LaneProviderSpec {
        id: "search".to_string(),
        label: Rc::new(|| crate::tr!(margin_lane_provider_search())),
        hint: Rc::new(|| crate::tr!(margin_lane_provider_search_hint())),
        column: LaneColumn::Center,
        shape: LaneShape::Bar,
        palette_slot: 0,
        surfaces: &[
            LaneSurface::Editor,
            LaneSurface::Stream,
            LaneSurface::SearchPreview,
        ],
        default_on: true,
        // Neither of the other two refresh rules fits, and the reason is worth
        // recording: a query edit, a case-sensitivity toggle or closing the banner
        // all change the match set without touching the document, so
        // `OnDocumentChange` would sit still through every one of them. The signal
        // that actually carries this is the arbiter's.
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|ctx| {
            let Some(q) = query::active_query().get() else {
                return Vec::new();
            };
            if q.text.is_empty() {
                return Vec::new();
            }
            // Re-run against the live document rather than carry offsets across an
            // edit. Same matcher the find banner's session uses, so the lane and the
            // banner cannot disagree about what a match is.
            let Ok(hits) = ctx.doc.find_all(&q.text, &q.options()) else {
                return Vec::new();
            };
            let current = q.current_in(ctx.item_id);
            let total = hits.len() as i64;
            hits.iter()
                .enumerate()
                .filter_map(|(i, m)| {
                    let start = (ctx.locate)(m.position)?;
                    let end = (ctx.locate)(m.position + m.length)?;
                    let is_current = current == Some(i);
                    Some(LaneMark {
                        // The **item and** the ordinal, not the offset. The offset is
                        // out because it changes on every edit above it, and a mark
                        // whose id changes every frame gives a screen reader a tree
                        // it cannot hold still. The item is in because a stream calls
                        // this once per row, each starting its own hit list at zero,
                        // while the group is one constant for the whole provider — so
                        // the bare ordinal made every row's first hit share an
                        // accessibility node with every other row's first hit.
                        id: mark_id(ctx.item_id, i),
                        span: LaneSpan::new(start, end),
                        // **One column for every hit, current or not.** The current
                        // match used to sit in `Full` while its siblings sat in
                        // `Center`, which is a third of the width -- so the column's
                        // occupied width depended on how many hits there were and
                        // where the current one was. Typing a query read as the
                        // column growing and shrinking, and that is what a reader
                        // notices, long before they notice which mark is current.
                        // The emphasis moved to length, where it costs nothing.
                        column: LaneColumn::Center,
                        shape: if is_current {
                            LaneShape::BarCurrent
                        } else {
                            LaneShape::Bar
                        },
                        color: ctx.color,
                        label: if is_current {
                            crate::tr!(margin_lane_search_current(
                                text = m.matched_text.clone(),
                                index = i as i64 + 1,
                                total = total
                            ))
                        } else {
                            crate::tr!(margin_lane_search_hit(
                                text = m.matched_text.clone(),
                                index = i as i64 + 1,
                                total = total
                            ))
                        },
                        group: ctx.group,
                    })
                })
                .collect()
        }),
    }
}

// ── document boundaries ──────────────────────────────────────────────────────

/// Where one document ends and the next begins, in a stream.
///
/// Streams only, and not because the others are unsupported: a tab holds one
/// document, so a boundary mark there would be a single rule at the very top saying
/// what the tab title already says.
///
/// A `Rule` rather than a mark, because it means a different kind of thing. Every
/// other shape on the lane answers "there is something here"; this one answers
/// "this is where you crossed over", and drawing it as another dot would put a
/// division and a finding in the same visual language.
fn boundaries() -> LaneProviderSpec {
    LaneProviderSpec {
        id: "boundaries".to_string(),
        label: Rc::new(|| crate::tr!(margin_lane_provider_boundaries())),
        hint: Rc::new(|| crate::tr!(margin_lane_provider_boundaries_hint())),
        column: LaneColumn::Left,
        shape: LaneShape::Rule,
        palette_slot: 6,
        surfaces: &[LaneSurface::Stream],
        default_on: true,
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|ctx| {
            // Offset zero is the top of this row's text, which is what the boundary
            // is. `None` before the row has been laid out, and the mark is simply
            // absent for that frame rather than drawn at the top of the lane.
            let Some(top) = (ctx.locate)(0) else {
                return Vec::new();
            };
            vec![LaneMark {
                id: ctx.item_id,
                span: LaneSpan::at(top),
                column: LaneColumn::Left,
                shape: LaneShape::Rule,
                color: ctx.color,
                label: crate::tr!(margin_lane_boundary(title = document_title(ctx))),
                group: ctx.group,
            }]
        }),
    }
}

/// A mark id unique across every document one lane maps.
///
/// FNV-1a over the item and the ordinal. A named, stable hash rather than
/// `DefaultHasher`, whose output std explicitly does not promise across releases —
/// and this number becomes an accessibility node's identity, which has to hold
/// still across repaints and across builds.
fn mark_id(item: EntityId, ordinal: usize) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |v: u64| {
        for byte in v.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    mix(item);
    mix(ordinal as u64);
    hash
}

/// This row's title, or a stated absence.
///
/// Read through the backend rather than from a stream view-model, because a
/// provider is called for a surface and holds no view-model: the same read the
/// binder itself performs, for one item.
fn document_title(ctx: &LaneContext<'_>) -> String {
    let title =
        frontend::commands::binder_item_commands::get_binder_item(ctx.app_ctx, &ctx.item_id)
            .ok()
            .flatten()
            .map(|item| item.title)
            .unwrap_or_default();
    if title.trim().is_empty() {
        crate::tr!(margin_lane_boundary_untitled()).resolve_now()
    } else {
        title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A mark id has to be unique across every document one lane maps.**
    ///
    /// A stream calls the search provider once per row, each starting its own hit
    /// list at zero, while `group` is one constant for the whole provider. The bare
    /// ordinal therefore made every row's first hit share an accessibility node with
    /// every other row's first hit — a screen-reader user searching a common word
    /// across a Full Book got a truncated set of "hit N of M" announcements, with
    /// nothing visibly wrong on screen.
    #[test]
    fn the_same_ordinal_in_two_rows_is_two_different_marks() {
        assert_ne!(mark_id(101, 0), mark_id(202, 0));
        assert_ne!(mark_id(101, 0), mark_id(101, 1));
        assert_eq!(
            mark_id(101, 3),
            mark_id(101, 3),
            "and stable across repaints"
        );
    }

    /// The number reaches an accessibility tree, so it must be the same number on
    /// every run of every build. Asserted against a literal rather than against
    /// itself, which is what would catch the hash being quietly swapped out.
    #[test]
    fn a_mark_id_is_the_same_number_on_every_build() {
        assert_eq!(mark_id(101, 0), 0xdcec_6bd3_6438_f340);
        assert_eq!(mark_id(202, 0), 0xdc9d_4f98_6fa4_87af);
        assert_eq!(mark_id(101, 1), 0xfbe7_32dc_6f28_3d61);
    }
}
