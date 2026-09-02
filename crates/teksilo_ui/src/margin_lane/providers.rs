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
            //
            // `to_addressable_text()`, NOT `to_plain_text()`: a misspelling's offset is
            // a document char offset, where an embedded table occupies its `U+FFFC`
            // anchor plus a `\n` separator, and the human-readable export drops both.
            // Sliced from the export, the accessible name of every mark after a table
            // was two characters off per table -- a screen reader was read a word that
            // is not in the prose.
            let text: Vec<char> = ctx
                .doc
                .to_addressable_text()
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
/// This provider's id.
///
/// **Frozen.** The lane synthesises `editor.margin_lane.provider.story_bible` from it and
/// writes that into the writer's `general.toml` the first time the key is read, so
/// renaming it silently resets whether the marks are on. Named rather than repeated as a
/// literal because a second surface draws the same finding in the text and takes its
/// colour from this spec — see [`crate::story_bible::highlight`].
pub const STORY_BIBLE_PROVIDER_ID: &str = "story_bible";

fn story_bible() -> LaneProviderSpec {
    LaneProviderSpec {
        id: STORY_BIBLE_PROVIDER_ID.into(),
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
            // project's names, the same guard every other work-scoped lane applies.
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
            .is_some_and(|it| it.point_of_view.contains(&subject.note_id()));
            if declared_pov && let Some(at) = (ctx.locate)(0) {
                out.push(LaneMark {
                    // Ordinal zero of *this row*, never a bare `0`: a stream calls this
                    // provider once per row and `group` is one constant for the whole
                    // provider, so a bare ordinal would give every row's point-of-view
                    // mark the same accessibility node. See `mark_id`, and the test that
                    // records the same bug shipping once for search.
                    id: mark_id(ctx.item_id, 0),
                    span: LaneSpan::new(at, at),
                    column: LaneColumn::Right,
                    shape: LaneShape::Dot,
                    color: ctx.color,
                    label: crate::tr!(margin_lane_mark_point_of_view()),
                    group: ctx.group,
                });
            }

            // `to_addressable_text()`, NOT `to_plain_text()`: `ctx.locate` takes a
            // document char offset, the same space the comment anchors above are in,
            // and the export drops each table's `U+FFFC` anchor and its `\n`. Marks
            // taken from the export therefore landed two characters early per preceding
            // table, so the strip and the wash `story_bible::highlight` draws on the
            // very same finding disagreed about where the name is.
            let Ok(text) = ctx.doc.to_addressable_text() else {
                return out;
            };
            // Scanned against the Work's whole table and filtered back to this entry --
            // see `subject::hits`, which is also what the underline layer calls.
            for (i, (start, end)) in super::subject::hits(&text, &subject.entity, &subject.table)
                .into_iter()
                .enumerate()
            {
                let (Some(a), Some(b)) = ((ctx.locate)(start), (ctx.locate)(end)) else {
                    continue;
                };
                out.push(LaneMark {
                    // Offset by one so the point-of-view mark keeps ordinal zero: the two
                    // kinds of mark share one ordinal space per row, and hashing the row
                    // in is what keeps this row's first hit distinct from every other
                    // row's.
                    id: mark_id(ctx.item_id, i + 1),
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

    /// **The same bug, on the story-bible provider, caught through the provider itself.**
    ///
    /// A stream calls it once per row and each call restarts its enumeration, so the bare
    /// ordinal it used to write gave every row's point-of-view mark the id `0` and every
    /// row's first name the id `1`. `MarginLane::element_id` hashes only `(group, id)` and
    /// `group` is one constant for the whole provider, so five rows naming the entry once
    /// became one accessibility node: a screen-reader user was told "Named here" once and
    /// could never reach the other four, with nothing visibly wrong on screen.
    ///
    /// Asserted against the spec's own closure rather than against [`mark_id`], because
    /// what shipped wrong was the call site, not the hash.
    #[test]
    fn two_rows_of_one_stream_get_distinct_story_bible_marks() {
        use crate::app_ids::AppIds;
        use frontend::commands::{binder_commands, binder_item_commands, work_commands};
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
        use frontend::direct_access::{
            BinderItemRelationshipDto, CreateBinderDto, CreateBinderItemDto, CreateWorkDto,
        };
        use skribisto_model::mentions::DiscoverableEntity;
        use teksilo::text_document::TextDocument;

        let app_ctx = Rc::new(frontend::AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &app_ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("create binder")
        .id;
        let mut next = 0i32;
        let mut add = |sub_role: BinderItemSubRole| {
            let id = binder_item_commands::create_binder_item(
                &app_ctx,
                None,
                &CreateBinderItemDto {
                    status: None,
                    title: "row".into(),
                    role: BinderItemRole::Item,
                    sub_role,
                    activated: true,
                    is_exportable: true,
                    ..Default::default()
                },
                binder,
                next,
            )
            .expect("create item")
            .id;
            next += 1;
            id
        };
        let note = add(BinderItemSubRole::Note);
        let row_a = add(BinderItemSubRole::Scene);
        let row_b = add(BinderItemSubRole::Scene);
        for row in [row_a, row_b] {
            binder_item_commands::set_binder_item_relationship(
                &app_ctx,
                None,
                &BinderItemRelationshipDto {
                    id: row,
                    field: BinderItemRelationshipField::PointOfView,
                    right_ids: vec![note],
                },
            )
            .expect("declare the point of view");
        }

        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
        crate::margin_lane::subject::set_active_subject(Some(
            crate::margin_lane::subject::LaneSubject {
                entity: DiscoverableEntity {
                    id: note,
                    title: "Elizabeth".into(),
                    aliases: Vec::new(),
                },
                table: Vec::new(),
                work_uid: work.unique_id.clone(),
                publisher: crate::margin_lane::LaneScope::fresh(),
            },
        ));

        // The two rows carry the same prose on purpose: identical text is exactly what
        // made the two enumerations agree, ordinal for ordinal.
        let doc = TextDocument::new();
        doc.set_plain_text("Elizabeth waited. Elizabeth left.")
            .expect("set the row's prose");

        let spec = story_bible();
        let anchors: Vec<crate::margin_lane::CommentAnchor> = Vec::new();
        let locate = |_: usize| Some(0.25_f32);
        let marks_of = |item_id: EntityId| {
            let ctx = LaneContext {
                kind: crate::format::EditorKind::Prose,
                app_ctx: &app_ctx,
                ids: &ids,
                surface: LaneSurface::Stream,
                doc: &doc,
                item_id,
                comment_anchors: &anchors,
                misspellings: &[],
                color: teksilo::tokens::Color::from_hex("#009E73"),
                group: 3,
                locate: &locate,
            };
            (spec.marks)(&ctx)
                .into_iter()
                .map(|m| m.id)
                .collect::<Vec<u64>>()
        };
        let a = marks_of(row_a);
        let b = marks_of(row_b);
        // Withdraw before asserting: the subject is a thread-local, and a failing assert
        // must not leave it published for whatever test runs next on this thread.
        crate::margin_lane::subject::set_active_subject(None);

        assert_eq!(a.len(), 3, "the declaration and both names: {a:?}");
        assert_eq!(b.len(), 3, "and the same for the second row: {b:?}");
        let mut all = [a.clone(), b.clone()].concat();
        all.sort_unstable();
        all.dedup();
        assert_eq!(
            all.len(),
            6,
            "six marks, six accessibility nodes, but row one gave {a:?} and row two {b:?}"
        );
    }

    /// A table, then a block, then `prose`: the shape that puts a `U+FFFC` anchor and
    /// its separator ahead of every offset in the text, which is the whole of what the
    /// two providers below are about.
    fn doc_after_a_table(prose: &str) -> teksilo::text_document::TextDocument {
        let d = teksilo::text_document::TextDocument::new();
        d.cursor().insert_table(2, 2).expect("insert a table");
        let cursor = d.cursor_at(d.character_count());
        cursor.insert_block().expect("a block after the table");
        cursor.insert_text(prose).expect("prose after the table");
        d
    }

    /// The **char** index of `needle`, which `str::find` does not answer: it answers in
    /// bytes, and a `U+FFFC` anchor is one char and three bytes.
    fn char_index_of(haystack: &str, needle: &str) -> usize {
        let byte = haystack.find(needle).expect("present");
        haystack[..byte].chars().count()
    }

    /// **A misspelling's offset is a document offset, and the label was sliced from the
    /// export.**
    ///
    /// `LaneContext::misspellings` carries the offsets the editor's own spell session
    /// underlines, in the document char space where a table occupies its anchor plus a
    /// separator. `to_plain_text()` drops both, so the accessible name of every mark
    /// after a table was cut two characters early per table: a screen reader was read a
    /// word that is not in the prose, with the dot itself in the right place.
    #[test]
    fn a_table_ahead_of_a_flagged_word_does_not_shift_its_label() {
        use crate::app_ids::AppIds;

        let d = doc_after_a_table("teh quiet room.");
        let addressable = d.to_addressable_text().expect("addressable text");
        let exported = d.to_plain_text().expect("exported text");
        let at = char_index_of(&addressable, "teh");
        assert_ne!(
            at,
            char_index_of(&exported, "teh"),
            "the table must be what the two strings disagree about"
        );

        let app_ctx = Rc::new(frontend::AppContext::new());
        let ids = AppIds::new();
        let anchors: Vec<crate::margin_lane::CommentAnchor> = Vec::new();
        let flagged = [(at, 3usize)];
        let locate = |_: usize| Some(0.25_f32);
        let ctx = LaneContext {
            kind: crate::format::EditorKind::Prose,
            app_ctx: &app_ctx,
            ids: &ids,
            surface: LaneSurface::Editor,
            doc: &d,
            item_id: 1,
            comment_anchors: &anchors,
            misspellings: &flagged,
            color: teksilo::tokens::Color::from_hex("#009E73"),
            group: 1,
            locate: &locate,
        };
        let marks = (spelling().marks)(&ctx);
        assert_eq!(marks.len(), 1, "one flagged word, one dot");
        assert!(
            marks[0].label.resolve_now().contains("teh"),
            "the flagged word itself, not the two characters before it: {:?}",
            marks[0].label.resolve_now()
        );
    }

    /// **A table ahead of a name must not move its mark on the strip.**
    ///
    /// `ctx.locate` takes a document char offset, the same space the comment anchors are
    /// in. Reading the names out of `to_plain_text()` handed it offsets two characters
    /// early per preceding table, so the dot on the strip and the wash
    /// `story_bible::highlight` draws on the identical finding pointed at different
    /// words.
    #[test]
    fn a_table_ahead_of_a_name_does_not_shift_its_mark_on_the_strip() {
        use crate::app_ids::AppIds;
        use frontend::commands::work_commands;
        use frontend::direct_access::CreateWorkDto;
        use skribisto_model::mentions::DiscoverableEntity;
        use std::cell::RefCell;

        let app_ctx = Rc::new(frontend::AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));

        let d = doc_after_a_table("Elizabeth waited.");
        let addressable = d.to_addressable_text().expect("addressable text");
        let at = char_index_of(&addressable, "Elizabeth");
        assert_ne!(
            at,
            char_index_of(&d.to_plain_text().expect("exported text"), "Elizabeth"),
            "the table must be what the two strings disagree about"
        );

        crate::margin_lane::subject::set_active_subject(Some(
            crate::margin_lane::subject::LaneSubject {
                entity: DiscoverableEntity {
                    id: 4_242,
                    title: "Elizabeth".into(),
                    aliases: Vec::new(),
                },
                table: Vec::new(),
                work_uid: work.unique_id.clone(),
                publisher: crate::margin_lane::LaneScope::fresh(),
            },
        ));

        let asked: RefCell<Vec<usize>> = RefCell::new(Vec::new());
        let anchors: Vec<crate::margin_lane::CommentAnchor> = Vec::new();
        let locate = |offset: usize| {
            asked.borrow_mut().push(offset);
            Some(0.25_f32)
        };
        let ctx = LaneContext {
            kind: crate::format::EditorKind::Prose,
            app_ctx: &app_ctx,
            ids: &ids,
            surface: LaneSurface::Stream,
            doc: &d,
            item_id: 1,
            comment_anchors: &anchors,
            misspellings: &[],
            color: teksilo::tokens::Color::from_hex("#009E73"),
            group: 3,
            locate: &locate,
        };
        let marks = (spec_marks_of(&ctx)).len();
        // Withdraw before asserting: the subject is a thread-local, and a failing assert
        // must not leave it published for whatever test runs next on this thread.
        crate::margin_lane::subject::set_active_subject(None);

        assert_eq!(marks, 1, "the one name in the prose");
        assert_eq!(
            asked.borrow().first().copied(),
            Some(at),
            "the strip is asked for the document's own offset, not the export's"
        );
    }

    /// The story-bible provider's own closure, so the test above exercises what ships
    /// rather than a copy of it.
    fn spec_marks_of(ctx: &LaneContext<'_>) -> Vec<LaneMark> {
        (story_bible().marks)(ctx)
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
