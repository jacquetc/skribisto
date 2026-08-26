// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The community edition's own mark sources.
//!
//! Four, registered through the same door an extension uses, under the ids
//! [`builtin_ids`](super::builtin_ids) reserves. Going through the public registry
//! rather than a private list is deliberate: it is what keeps the settings page,
//! the enabled check and the draw order one code path for everybody, so a bug that
//! would only show up for an extension shows up here first.
//!
//! | Provider | Column | Shape | Where | Default |
//! |---|---|---|---|---|
//! | Comments | Right | Square | Editor, stream | on |
//! | Search hits | Centre, and full width for the one you are on | Bar | Every surface | on |
//! | Document boundaries | Left | Rule | Streams only | on |
//! | Spelling | Left | Dot | Editor, stream | **off** |
//!
//! Three on and one off, and the split is not a quota: the three mark what a
//! person put there or went looking for, and are silent on a manuscript nobody
//! has annotated. The fourth reports a machine's opinion of the prose. See
//! [`LaneProviderSpec::default_on`](super::LaneProviderSpec::default_on), which
//! is where an extension reads the rule.
//!
//! ## The left column is shared, and that is the first time
//!
//! Three providers meant three columns, and none could ever fight another for a
//! pixel. A fourth cannot have that: there are three columns, and spelling has to
//! go somewhere. It takes the left, which is the emptiest -- document boundaries
//! are one mark per document and only on a stream, where spelling is per word and
//! everywhere.
//!
//! What keeps them apart is the thing that was always meant to: the shape. A
//! hairline rule with a notch is not a dot, and the two are told apart by a reader
//! who cannot separate their hues. Where they do land on the same pixel -- a
//! misspelling in the first line of a scene -- the boundary draws over the dot,
//! because a later group paints last and the division is the more structural fact.

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
        (format!("{NAMESPACE}.spelling"), spelling()),
        (format!("{NAMESPACE}.story_bible"), story_bible()),
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

// ── spelling ─────────────────────────────────────────────────────────────────

/// Where the spell checker has flagged a word.
///
/// **Off by default**, and the only built-in that is. The others mark things a
/// writer put there or went looking for; this one marks a machine's opinion of
/// their prose, and a lane that lights up with it unasked would be a proofreading
/// tool wearing a manuscript's clothes. A writer who wants it turns it on -- from
/// Settings, or from the lane's own context menu, where they can see the effect.
///
/// Reads [`LaneContext::misspellings`], never the checker: the set handed down is
/// the one the editor is showing, filtered by the caret exemption, so the lane
/// cannot flag the word being typed after the page has stopped. Empty where the
/// surface keeps no spell session, and empty for a language with no dictionary
/// installed -- which is not "no mistakes", and is why nothing here says so.
///
/// A dot per flagged word, at the word rather than spanning it. Spanning is what
/// a comment does, and it is meaningless here: `locate` answers with the middle of
/// the *line* an offset sits on, so both ends of a word inside one line give the
/// same fraction and the span collapses. A point mark is grown to
/// `MIN_MARK_HEIGHT` around its position, which is the reading that was wanted.
fn spelling() -> LaneProviderSpec {
    LaneProviderSpec {
        id: "spelling".to_string(),
        label: Rc::new(|| crate::tr!(margin_lane_provider_spelling())),
        hint: Rc::new(|| crate::tr!(margin_lane_provider_spelling_hint())),
        column: LaneColumn::Left,
        // A low-emphasis finding, which is exactly what this is. Not a square:
        // filled shapes are what a person put there.
        shape: LaneShape::Dot,
        palette_slot: 1,
        surfaces: &[LaneSurface::Editor, LaneSurface::Stream],
        default_on: false,
        // Driven by the host's recompute guard, which carries the session's own
        // generation: the flagged set moves without the document changing --
        // a dictionary installed, a language muted, the caret leaving the word it
        // was exempting -- and `OnDocumentChange` would miss every one of those.
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|ctx| {
            // Nothing flagged, nothing to read: the guard is what keeps a
            // document with no misspellings -- or a language with no dictionary
            // -- from paying for the text below.
            if ctx.misspellings.is_empty() {
                return Vec::new();
            }
            // Once per row, not once per mark. The offsets are character indices
            // and there is no range read on a document, so the alternative is an
            // O(document) extraction per flagged word.
            let text: Vec<char> = ctx
                .doc
                .to_plain_text()
                .unwrap_or_default()
                .chars()
                .collect();
            ctx.misspellings
                .iter()
                .filter_map(|&(start, length)| {
                    let at = (ctx.locate)(start)?;
                    Some(LaneMark {
                        // The offset, not an ordinal: it holds still across a
                        // repaint or a recompute that changes nothing, which is
                        // the case that matters -- it is what happens sixty
                        // times a second while a writer types. An ordinal fails
                        // that same case in a way the offset does not: fixing
                        // or adding a *different* misspelling elsewhere shifts
                        // every ordinal after it, even though the word this
                        // mark points at never moved.
                        id: start as u64,
                        span: LaneSpan::at(at),
                        column: LaneColumn::Left,
                        shape: LaneShape::Dot,
                        color: ctx.color,
                        label: crate::tr!(margin_lane_spelling(
                            word = word_at(&text, start, length)
                        )),
                        group: ctx.group,
                    })
                })
                .collect()
        }),
    }
}

/// The flagged word itself, for the mark's accessible name.
///
/// A screen reader hearing "possible misspelling" fourteen times learns nothing;
/// hearing the word is the whole content of the mark. Sliced from the document's
/// own text rather than carried on the range, because the ranges are shifted by
/// every edit and a word captured when the range was made would be the one that
/// used to be there.
///
/// Clamped rather than trusted: an offset can outlive the text it pointed into by
/// the width of one frame, and a mark that panics is worse than one that says
/// nothing.
fn word_at(text: &[char], start: usize, length: usize) -> String {
    let end = start.saturating_add(length).min(text.len());
    text.get(start..end)
        .map(String::from_iter)
        .unwrap_or_default()
}

// ── search hits ──────────────────────────────────────────────────────────────

/// Every hit for whatever the writer last searched for.
///
/// The hit the writer is standing on draws longer than the rest; every hit,
/// current or not, sits in the same centre column. A length difference rather
/// than a colour one, because the distinction has to survive a reader who cannot
/// separate two hues, and at three pixels tall hue is a weak channel even with
/// normal vision. Why length carries it and not a second column: see the comment
/// on `column` below.
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
            // Offset zero is this row's first line, not its top: `locate` answers
            // with the middle of whatever line an offset sits on, and that is
            // exactly right here too. The notch on a `Rule` is drawn at
            // `m.top - 2.0`, so a mark anchored to the true top of the strip
            // would be clipped rather than drawn -- the same reason a point mark
            // is grown around its position instead of pinned to a line's start.
            // `None` before the row has been laid out, and the mark is simply
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

// ── the story-bible subject ──────────────────────────────────────────────────

/// **Where the entry a reading is about is named**, plus one mark per row that
/// declares her as its point of view.
///
/// Only ever draws on a note's **In prose** reading, because that is the only surface
/// with a subject: [`super::subject::active_subject`] is set while such a reading is on
/// screen and withdrawn when it goes away. Everywhere else this contributes nothing, so
/// a writer who never opens one never sees it.
///
/// ## The zero-width mark
///
/// A row is in this reading because the writer *declared* it, and the strongest
/// declaration is a point of view: this scene is told through her eyes. Such a scene
/// very often never writes her name at all, which is the whole reason deep third person
/// exists. Marking only textual hits would leave exactly those rows blank, so the row
/// earns a mark at its own first character: a stop the reader can move to, saying she is
/// here even though the prose does not say so.
///
/// Zero-width by construction rather than by convention. It is a position, not a range,
/// and giving it an arbitrary length would claim a stretch of prose that has nothing to
/// do with it.
///
/// ## Default on
///
/// It marks what a person put there, never a machine's opinion of the prose: every mark
/// is either a name the writer wrote or a declaration they made by hand. That is the
/// rule [`LaneProviderSpec::default_on`](super::LaneProviderSpec::default_on) states,
/// and the same one comments and boundaries pass.
fn story_bible() -> LaneProviderSpec {
    LaneProviderSpec {
        id: "story_bible".into(),
        label: Rc::new(|| crate::tr!(margin_lane_provider_story_bible())),
        hint: Rc::new(|| crate::tr!(margin_lane_provider_story_bible_hint())),
        column: LaneColumn::Right,
        shape: LaneShape::Dot,
        palette_slot: 4,
        surfaces: &[LaneSurface::Stream],
        default_on: true,
        refresh: LaneRefresh::OnDocumentChange,
        marks: Rc::new(|ctx| {
            let Some(subject) = super::subject::active_subject().get() else {
                return Vec::new();
            };
            // A second window on a second project marks nothing rather than this
            // project's names, the same guard the Atelier's own cast lane applies.
            let here = frontend::commands::work_commands::get_work(
                ctx.app_ctx,
                &match ctx.ids.work_id.get() {
                    Some(w) => w,
                    None => return Vec::new(),
                },
            )
            .ok()
            .flatten()
            .map(|w| w.unique_id)
            .unwrap_or_default();
            if here != subject.work_uid {
                return Vec::new();
            }

            let mut out = Vec::new();

            // The declaration first, so it sorts to the top of the row.
            let declared_pov = frontend::commands::binder_item_commands::get_binder_item(
                ctx.app_ctx,
                &ctx.item_id,
            )
            .ok()
            .flatten()
            .is_some_and(|it| it.point_of_view.contains(&subject.note_id));
            if declared_pov && let Some(at) = (ctx.locate)(0) {
                out.push(LaneMark {
                    id: 0,
                    span: LaneSpan::new(at, at),
                    column: LaneColumn::Right,
                    shape: LaneShape::Dot,
                    color: ctx.color,
                    label: crate::tr!(margin_lane_mark_point_of_view()),
                    group: ctx.group,
                });
            }

            let Ok(text) = ctx.doc.to_plain_text() else {
                return out;
            };
            for (i, (start, end)) in super::subject::hits(&text, &subject.names)
                .into_iter()
                .enumerate()
            {
                let (Some(a), Some(b)) = ((ctx.locate)(start), (ctx.locate)(end)) else {
                    continue;
                };
                out.push(LaneMark {
                    // Offset by one so the point-of-view mark keeps id zero: ids need
                    // only be stable within one provider across repaints, and a hit's
                    // position is exactly that.
                    id: i as u64 + 1,
                    span: LaneSpan::new(a, b),
                    column: LaneColumn::Right,
                    shape: LaneShape::Dot,
                    color: ctx.color,
                    label: crate::tr!(margin_lane_mark_named_here()),
                    group: ctx.group,
                });
            }
            out
        }),
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
