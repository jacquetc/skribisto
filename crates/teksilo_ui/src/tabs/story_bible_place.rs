// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **C2, the Story bible place.** A card grid on every notes-folder tab, grouped by
//! discoverable tag: the community edition finally giving the discoverable-tag /
//! alias / notes-folder machinery it already had a place to be *browsed* as a
//! bible, rather than clicked through as rows in a table or entries in a folder
//! that looks like every other folder.
//!
//! **Ordinary community code, hardcoded into [`super::shared::panes::folder_synopsis_with_overview`]
//! the same way `SEG_NOTES` and `SEG_OVERVIEW` are.** It does not go through
//! [`super::shared::segments::register_container_segment`]: that door is for an
//! out-of-tree extension; this is the community edition finishing a feature it
//! already half-built. Segment id: [`super::shared::segments::SEG_STORY_BIBLE`],
//! **persisted verbatim** as a remembered view the moment a writer visits this
//! segment, frozen the instant it ships, exactly like every other `SEG_*`.
//!
//! **Do not reuse the Corkboard.** `CorkboardViewModel` assumes scene-shaped
//! children (split, merge, drag-reorder) and none of that means anything for a
//! bible entry: a note has no manuscript position to merge into a neighbour's.
//! This builds its own, purpose-built grid from existing teksilo-data widgets
//! (`GridView` + `grouping_sections`, the same widget the Corkboard itself uses,
//! wired for a flat list rather than a manuscript stream).
//!
//! **The mentioned-in-N-scenes badge is Work-wide, on purpose, and worded so.** A
//! subject that outlives a single Book (a bible entry chief among them) is read
//! across the whole Work, and a per-card count here answers "how often does the
//! manuscript name this entry", never "how often does *this Book* name it". A
//! trilogy writer must never be able to read the number as being about whichever
//! Book they happen to have open. It sums
//! [`crate::mentions::MentionIndex::backlinks_for`], which is already Work-wide
//! by construction, narrowed to **scene-owned** hits only (a worldbuilding note
//! naming a character is a real mention, but it is not "a scene": a Note has no
//! reading order), so the word "scenes" in the badge is never a loose one.
//!
//! **The Books filter chip narrows by declaration, never by measurement.** A
//! card's own `book_ids` (the writer's filing, see `common::entities::BinderItem::books`'s
//! own doc) decides whether a chip hides it; the mention badge is never
//! filtered by it, and never could be: a badge that vanished the moment a
//! stale or never-filed declaration disagreed with the manuscript would be
//! exactly the drift-inducing failure `book_ids` exists to keep out of this
//! model. **Empty `book_ids` means "not yet filed", never "every book"** (see
//! the field's own doc): with a specific chip active, an unfiled entry is
//! absent from the grid, exactly as an unfiled entry is absent from the
//! Inspector's own per-Book reasoning. The unfiltered default ("All books",
//! also what a one-Book project always shows, since the row never renders
//! below two Books) shows every entry regardless of filing status: nothing
//! here ever narrows a series-wide glossary term out of its own default view.

use std::collections::HashMap;
use std::rc::Rc;

use teksilo::canvas::EdgeInsets;
use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Center, GridSizing, GridView, Padding, TextWidget, TileContext, VStack,
    Wrap, grouping_sections,
};

use frontend::AppContext;
use frontend::commands::binder_item_commands;
use frontend::common::entities::BinderItemSubRole;

use crate::app_ids::AppIds;
use crate::intents::AppIntent;
use crate::mentions::MentionIndex;
use crate::models::TagRow;
use crate::tabs::ContentTab;
use crate::tags::TagsViewModel;
use crate::tags::cast_add::CastCandidate;

/// One bible entry read off the binder: everything a card needs that does **not**
/// depend on the mention index. Cheap, non-reactive: read fresh on every build,
/// the same way [`crate::docks::inspector::live_books`] and
/// [`crate::story_bible::modal::EntryPanel::book_candidates`] read their own
/// candidate tables.
#[derive(Clone, Debug, PartialEq)]
struct BibleEntry {
    item_id: u64,
    title: String,
    tags: Vec<u64>,
    alias_count: usize,
    /// The writer's own filing: see the module doc's "narrows by declaration"
    /// paragraph.
    book_ids: Vec<u64>,
}

/// Every Note-facet row (`Folder/Note` and `Item/Note` alike, the same union the
/// Inspector's own Books section gates on) strictly beneath `container_id`, in
/// document order.
///
/// A small, local re-derivation of "the subtree below one row", not a call
/// through `OutlineViewModel::subtree_descendants` (which needs a live outline
/// view-model this pane is never handed). Three relationship hops plus one
/// indent scan: cheap enough that duplicating it costs far less than
/// threading a whole outline view-model through just to reach it, the same
/// trade-off `story_bible::infer_book::book_containing` already makes for the
/// same reason.
fn subtree_notes(ctx: &AppContext, work_id: u64, container_id: u64) -> Vec<BibleEntry> {
    let items = crate::models::binder_stream::ordered_flat_items(ctx, work_id);
    let Some(pos) = items.iter().position(|(_, it)| it.id == container_id) else {
        return Vec::new();
    };
    let (container_binder, container) = &items[pos];
    let container_binder = *container_binder;
    let base_indent = container.indent;
    items[pos + 1..]
        .iter()
        .take_while(|(binder, it)| *binder == container_binder && it.indent > base_indent)
        .filter(|(_, it)| {
            matches!(
                skribisto_model::search_facet_of(&it.role, &it.sub_role),
                Some(skribisto_model::SearchFacet::Note)
            )
        })
        .map(|(_, it)| BibleEntry {
            item_id: it.id,
            title: it.title.clone(),
            tags: it.tags.clone(),
            alias_count: it.aliases.len(),
            book_ids: it.books.clone(),
        })
        .collect()
}

/// How many of `entries`' Work-wide mentions land in an actual scene, per entry.
///
/// One batched `get_binder_item_multi` for the union of every owner id across
/// every entry, rather than one lookup per entry per card: a bible folder with
/// forty entries must not cost forty extra backend reads to paint a badge.
///
/// **Scene-owned only**, on purpose: `MentionIndex` legitimately includes
/// Note-owned hits (a worldbuilding note naming a character), but a Note has no
/// reading order and is not "a scene" this entry appeared in, the same
/// restriction the plan's own Appearances reading applies, restated here so the
/// badge's own wording ("mentioned in N scenes") is never a loose claim.
fn scene_mention_counts(
    ctx: &AppContext,
    index: &MentionIndex,
    entries: &[BibleEntry],
) -> HashMap<u64, usize> {
    let mut owners_by_entry: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut all_owner_ids: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for entry in entries {
        let owners: Vec<u64> = index
            .backlinks_for(entry.item_id)
            .into_iter()
            .map(|row| row.owner_id)
            .collect();
        all_owner_ids.extend(owners.iter().copied());
        owners_by_entry.insert(entry.item_id, owners);
    }
    if all_owner_ids.is_empty() {
        return HashMap::new();
    }
    let owner_ids: Vec<u64> = all_owner_ids.into_iter().collect();
    let scene_owners: std::collections::HashSet<u64> =
        binder_item_commands::get_binder_item_multi(ctx, &owner_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(|it| {
                matches!(
                    it.sub_role,
                    BinderItemSubRole::Scene | BinderItemSubRole::ChapterScene
                )
            })
            .map(|it| it.id)
            .collect();
    owners_by_entry
        .into_iter()
        .map(|(item_id, owners)| {
            (
                item_id,
                owners.iter().filter(|o| scene_owners.contains(o)).count(),
            )
        })
        .collect()
}

/// One card, already sorted into its group. `group` is the discoverable tag's own
/// name, or the "not yet tagged" catch-all: see [`grouped_cards`].
#[derive(Clone, Debug, PartialEq)]
struct GroupedCard {
    entry: BibleEntry,
    mentions: usize,
    group: String,
}

/// Fan each entry out into one row per **matching discoverable tag** (an entry
/// carrying two discoverable tags browses under both, the same multi-membership
/// a photo grouped into two albums would show under both), falling back to one
/// "not yet tagged" row for an entry that matches none. Pre-sorted so
/// [`grouping_sections`] (which partitions *consecutive* equal-key runs and does
/// not sort on its own) sees each group as one contiguous band, ordered the same
/// way the discoverable palette itself is ordered, with the catch-all last.
fn grouped_cards(
    entries: Vec<BibleEntry>,
    mentions: &HashMap<u64, usize>,
    discoverable: &[TagRow],
) -> Vec<GroupedCard> {
    let rank: HashMap<u64, usize> = discoverable
        .iter()
        .enumerate()
        .map(|(i, t)| (t.id, i))
        .collect();
    let untagged_rank = discoverable.len();
    let untagged_label = tr!(story_bible_grid_untagged()).resolve_now();

    let mut ranked: Vec<(usize, GroupedCard)> = Vec::new();
    for entry in entries {
        let mention_count = mentions.get(&entry.item_id).copied().unwrap_or(0);
        let mut matches: Vec<&TagRow> = discoverable
            .iter()
            .filter(|t| entry.tags.contains(&t.id))
            .collect();
        matches.sort_by_key(|t| rank.get(&t.id).copied().unwrap_or(usize::MAX));
        if matches.is_empty() {
            ranked.push((
                untagged_rank,
                GroupedCard {
                    entry: entry.clone(),
                    mentions: mention_count,
                    group: untagged_label.clone(),
                },
            ));
        } else {
            for tag in matches {
                ranked.push((
                    rank[&tag.id],
                    GroupedCard {
                        entry: entry.clone(),
                        mentions: mention_count,
                        group: tag.name.clone(),
                    },
                ));
            }
        }
    }
    ranked.sort_by(|(ra, a), (rb, b)| {
        ra.cmp(rb).then_with(|| {
            a.entry
                .title
                .to_lowercase()
                .cmp(&b.entry.title.to_lowercase())
        })
    });
    ranked.into_iter().map(|(_, c)| c).collect()
}

/// The pane: `Some(TagsViewModel)`/`Some(MentionIndex)` read from `app_state`
/// (Tier 1, one instance app-wide), exactly the way [`crate::tags::tag_chip::TagDotsRow`]
/// already does and for the same reason: every call site here is a plain
/// composition function with no constructor-threaded handle to reach for, and a
/// notes-folder tab is always about the one open project a single window has.
pub(crate) fn story_bible_pane(tab: &ContentTab) -> Box<dyn Widget> {
    Box::new(StoryBiblePane {
        app_ctx: tab.app_ctx(),
        ids: tab.ids().clone(),
        container_id: tab.item_id(),
        book_filter: tab.story_bible_book_filter.clone(),
        root: None,
        #[cfg(test)]
        book_filter_rendered: false,
        #[cfg(test)]
        cards_rendered: Vec::new(),
    })
}

struct StoryBiblePane {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    container_id: u64,
    book_filter: Signal<Option<u64>>,
    root: Option<WidgetId>,
    /// Captured on every `build()`, read back only by this module's own tests via
    /// `Widget::as_any`, the same introspection technique
    /// `story_bible::modal::EntryPanel` already uses for its own gated section,
    /// since there is no "find a widget by its content" query in this toolkit.
    #[cfg(test)]
    book_filter_rendered: bool,
    #[cfg(test)]
    cards_rendered: Vec<GroupedCard>,
}

impl std::fmt::Debug for StoryBiblePane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoryBiblePane").finish()
    }
}

impl Widget for StoryBiblePane {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let Some(tags_vm) = ctx.app_state::<TagsViewModel>().cloned() else {
            self.root = None;
            return Vec::new();
        };
        let Some(mention_index) = ctx.app_state::<MentionIndex>().cloned() else {
            self.root = None;
            return Vec::new();
        };
        let Some(work_id) = self.ids.work_id.get() else {
            self.root = None;
            return Vec::new();
        };

        // Rebuild whenever a scan lands (covers every binder-item mutation this
        // grid cares about: a new entry created, a tag/alias/books edit, a trash;
        // see `app::wiring::long_ops`'s own rescan wiring, which fires on every
        // `BinderItem` Created/Updated/Removed event), the tag palette itself
        // changing, or the writer picking a different Books chip.
        mention_index.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        tags_vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.book_filter
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let entries = subtree_notes(&self.app_ctx, work_id, self.container_id);
        let candidates = crate::docks::inspector::live_books(&self.app_ctx, &self.ids);
        let selected_book = self.book_filter.get();

        // Narrow by declaration, never hide by measurement: see the module doc.
        let visible: Vec<BibleEntry> = entries
            .into_iter()
            .filter(|e| match selected_book {
                None => true,
                Some(book_id) => e.book_ids.contains(&book_id),
            })
            .collect();

        let mut col = VStack::new().spacing(10.0);

        let book_filter_rendered = candidates.len() >= 2;
        #[cfg(test)]
        {
            self.book_filter_rendered = book_filter_rendered;
        }
        if book_filter_rendered {
            col = col.child(BookFilterChips {
                candidates: candidates.clone(),
                selected: self.book_filter.clone(),
                root: None,
            });
        }

        if visible.is_empty() {
            #[cfg(test)]
            {
                self.cards_rendered = Vec::new();
            }
            col = col.child(empty_hint());
        } else {
            let mentions = scene_mention_counts(&self.app_ctx, &mention_index, &visible);
            let discoverable: Vec<TagRow> = tags_vm
                .rows()
                .into_iter()
                .filter(|t| t.discoverable)
                .collect();
            let cards = grouped_cards(visible, &mentions, &discoverable);
            #[cfg(test)]
            {
                self.cards_rendered = cards.clone();
            }
            col = col.child(BibleGrid { cards, root: None });
        }

        let id = ctx.add(col);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }

    #[cfg(test)]
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

/// Nothing filed here yet: distinct from "no candidates", which the tag palette
/// (not this pane) is responsible for.
fn empty_hint() -> impl Widget {
    Center::new().child(
        VStack::new()
            .spacing(6.0)
            .child(
                TextWidget::new(tr!(story_bible_grid_empty_title()))
                    .style(TextStyleRole::BodyBold)
                    .color(TextRole::Secondary),
            )
            .child(TextWidget::new(tr!(story_bible_grid_empty_hint())).color(TextRole::Secondary)),
    )
}

/// The Books filter row: one toggle per live `Folder/Book`, plus "All books" to
/// clear it. Single-select, like a segmented filter: clicking the already-active
/// chip (including "All") is a no-op, and clicking a different one replaces the
/// choice rather than adding to it, so the grid's own default ("show everything,
/// filed or not") is always exactly one click away.
struct BookFilterChips {
    candidates: Vec<CastCandidate>,
    selected: Signal<Option<u64>>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for BookFilterChips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BookFilterChips").finish()
    }
}

impl Widget for BookFilterChips {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.selected
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let current = self.selected.get();

        let mut row = Wrap::new().spacing(6.0).line_spacing(6.0);
        {
            let selected = self.selected.clone();
            row = row.child(
                Button::new(tr!(story_bible_books_filter_all()))
                    .variant(if current.is_none() {
                        ButtonVariant::Tinted
                    } else {
                        ButtonVariant::Plain
                    })
                    .on_activate_fn(move |_c| selected.set(None)),
            );
        }
        for candidate in &self.candidates {
            let id = candidate.id;
            let on = current == Some(id);
            let selected = self.selected.clone();
            row = row.child(
                Button::new(lit!(candidate.title.clone()))
                    .variant(if on {
                        ButtonVariant::Tinted
                    } else {
                        ButtonVariant::Plain
                    })
                    .on_activate_fn(move |_c| selected.set(if on { None } else { Some(id) })),
            );
        }
        self.root = Some(ctx.add(row));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The card grid itself: a fresh, non-reactive `ListModel` built from `cards`
/// every time this widget builds (its parent, [`StoryBiblePane`], already rebuilds
/// on every event that could change the roster, see its own `build`), sectioned
/// by [`grouping_sections`] over the pre-sorted group name.
struct BibleGrid {
    cards: Vec<GroupedCard>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for BibleGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BibleGrid").finish()
    }
}

impl Widget for BibleGrid {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let model = teksilo::data::ListModel::from_vec(self.cards.clone());
        let sections = grouping_sections(&model, |c: &GroupedCard| c.group.clone());

        let delegate = move |tc: &TileContext<'_, GroupedCard>| -> Box<dyn Widget> {
            Box::new(BibleCard {
                card: tc.item.clone(),
                root: None,
            })
        };

        let a11y_model = model.clone();
        let activate_model = model.clone();

        let grid = GridView::new(model, delegate)
            .sizing(GridSizing::Adaptive {
                min_width: 220.0,
                max_width: Some(340.0),
                height: 96.0,
            })
            .spacing(12.0)
            .content_inset(EdgeInsets::uniform(4.0))
            .sections(sections)
            .pinned_section_headers(true)
            .a11y_label(tr!(story_bible_grid_label()).resolve_now())
            .tile_a11y_label(move |i| {
                a11y_model
                    .with_item(i, |c| {
                        format!(
                            "{}, {}",
                            c.entry.title,
                            tr!(story_bible_grid_mention_count(count = c.mentions as i64))
                                .resolve_now()
                        )
                    })
                    .unwrap_or_default()
            })
            .on_tile_activate(move |i, ectx: &mut EventContext| {
                if let Some((item_id, title)) =
                    activate_model.with_item(i, |c| (c.entry.item_id, c.entry.title.clone()))
                {
                    ectx.send_intent(AppIntent::OpenItem { item_id, title });
                }
            });

        let id = ctx.add(grid);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// One card: title, alias count, mention badge. Opened on double-click / Enter
/// (the grid's own `on_tile_activate`): a single click selects, matching every
/// other `GridView` in this app, so this stays consistent with the Corkboard's
/// own interaction model rather than inventing a second convention.
struct BibleCard {
    card: GroupedCard,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for BibleCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BibleCard").finish()
    }
}

impl Widget for BibleCard {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let entry = &self.card.entry;
        let col = VStack::new()
            .spacing(4.0)
            .child(
                TextWidget::new(lit!(entry.title.clone()))
                    .style(TextStyleRole::BodyBold)
                    .single_line(),
            )
            .child(
                TextWidget::new(tr!(story_bible_grid_alias_count(
                    count = entry.alias_count as i64
                )))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary)
                .single_line(),
            )
            .child(
                TextWidget::new(tr!(story_bible_grid_mention_count(
                    count = self.card.mentions as i64
                )))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary)
                .single_line(),
            );
        let id = ctx.add(Padding::uniform(10.0).child(col));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests;
