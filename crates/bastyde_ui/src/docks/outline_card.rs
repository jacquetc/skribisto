// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The outline row's hover card — what a row *is*, without opening it.
//!
//! A composite tooltip body built per outline row and shown when the pointer
//! rests on it. It answers the questions the row has no space for: what this is
//! and where it sits, whose eyes the scene is through, what it is tagged and
//! labelled, whether it leaves the export or the numbering, how long it is
//! against its goal, and when it was last touched. Creation and modification
//! times reach the UI here for the first time — no other surface shows them.
//!
//! **Read-only, by construction.** Not "controls that happen to be disabled":
//! no interactive widget exists in this tree, so the W3C rule that a tooltip
//! must not contain interactive content holds structurally rather than by
//! hoping nobody clicks. It also opts out of dwell promotion (see the outline
//! dock's `row_tooltip_sticky(false)`) — with nothing to reach into there is
//! nothing to pin. Everything shown stays editable where it already was: the
//! Inspector's toggles and tag field, the Overview's label cell, F2 for the
//! name. The card adds a glance, never a sole path to a control.
//!
//! **Cheap by default, expensive only when seen.** The body is built for every
//! *realized* row and rebuilt with it, so everything above comes from fields
//! the row source already fetched and used to discard — no backend read per
//! row. The two facts that cannot come for free, the synopsis excerpt and the
//! word count, are deferred to the card's first `paint()`: a tooltip body only
//! paints when it is actually shown, so an unhovered row pays nothing. That is
//! the same hook `CompositeTooltipWidget` itself uses to drive its dwell clock.
//!
//! **Colours come from the tooltip roles, not the surface roles.** The body
//! paints on the tooltip's own chrome, which is dark in *both* themes, so
//! `Primary`/`Secondary` are the wrong contrast pair there — `Primary` on
//! `tooltip_bg` measures 1.27:1. `TooltipText` and `TooltipShortcut` exist for
//! exactly this, and are what `tag_tooltip` and the export-preset sheet use.
//!
//! The card branches on what the row actually is: a `Binder` has only a name
//! and its timestamps — no label, tags, export flag or ordinal exist on that
//! entity — so it shows those and nothing else, rather than a column of blanks.

use std::cell::Cell;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::*;

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands};
use frontend::common::entities::{BinderItemSubRole, ContentRole};
use skribisto_model::counting::{self, CountMethod, CountingMethodSetting};

use crate::models::TreeNode;
use crate::tags::tag_chip::{MAX_VISIBLE_EDITOR, TagChipRow};

/// Gap between the card's stacked lines.
const LINE_SPACING: f32 = 4.0;
/// Gap between its sections.
const SECTION_SPACING: f32 = 10.0;
/// Wide enough for two short fact columns side by side and for a breadcrumb of
/// three or four titles to read as a path, narrow enough not to blanket the
/// tree it is describing.
const CARD_WIDTH: f32 = 400.0;
/// How much synopsis is a glance. Past this it stops being a card and starts
/// being the editor.
const EXCERPT_CHARS: usize = 220;

/// The card for one outline row.
///
/// Everything the constructor takes is resolved by the row delegate, which has
/// the tree model and the app context the card cannot reach itself: the
/// breadcrumb and sibling position need an ancestor walk, and `paint()` gets a
/// `PaintContext`, which has no `app_state`.
pub struct OutlineCard {
    node: TreeNode,
    breadcrumb: String,
    /// 1-based position among siblings, and how many there are.
    position: Option<(usize, usize)>,
    /// Direct children, for a container row.
    child_count: usize,
    app_ctx: Rc<AppContext>,
    counting_method: Signal<CountingMethodSetting>,
    /// Fires the deferred read at most once per card instance. A fresh card is
    /// built whenever the row rebuilds, so this resets exactly when the data
    /// might have changed — which is the behaviour we want, not a leak.
    loaded: Cell<bool>,
    synopsis: Signal<Option<String>>,
    own_words: Signal<Option<usize>>,
    root: Option<WidgetId>,
}

impl OutlineCard {
    pub fn new(
        node: TreeNode,
        breadcrumb: String,
        position: Option<(usize, usize)>,
        child_count: usize,
        app_ctx: Rc<AppContext>,
        counting_method: Signal<CountingMethodSetting>,
    ) -> Self {
        Self {
            node,
            breadcrumb,
            position,
            child_count,
            app_ctx,
            counting_method,
            loaded: Cell::new(false),
            synopsis: Signal::new(None),
            own_words: Signal::new(None),
            root: None,
        }
    }

    fn is_binder(&self) -> bool {
        self.node.kind == "binder"
    }

    /// Two backend reads and a parse — paid once, and only for a card the
    /// writer actually rested on long enough to see.
    fn load_deferred(&self) {
        let Some(item_id) = self.node.item_id else {
            return;
        };
        let Some(dto) = binder_item_commands::get_binder_item(&self.app_ctx, &item_id)
            .ok()
            .flatten()
        else {
            return;
        };
        let contents: Vec<_> = content_commands::get_content_multi(&self.app_ctx, &dto.contents)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .collect();

        if let Some(synopsis) = contents
            .iter()
            .find(|c| c.role == ContentRole::SynopsisText)
        {
            let plain = bastyde::text_document::djot_to_plain_text(
                &synopsis.data,
                &bastyde::text_document::DjotImportOptions::default(),
            );
            // Collapsed to one run: a synopsis is written in paragraphs, and a
            // card is one glance — the line breaks would only cost height.
            let flat = plain.split_whitespace().collect::<Vec<_>>().join(" ");
            let excerpt = if flat.chars().count() > EXCERPT_CHARS {
                let cut: String = flat.chars().take(EXCERPT_CHARS).collect();
                format!("{}…", cut.trim_end())
            } else {
                flat
            };
            self.synopsis.set((!excerpt.is_empty()).then_some(excerpt));
        }

        if let Some(scene) = contents.iter().find(|c| c.role == ContentRole::SceneText) {
            // The writer's chosen method, resolved for this item's own
            // language — the same call every other counting surface makes, so
            // the card can never disagree with the status bar.
            let method = counting::resolve_method(
                self.counting_method.get(),
                CountMethod::UnicodeWords,
                skribisto_model::language::primary(&dto.dict_language),
            );
            self.own_words
                .set(Some(counting::cached_count(&scene.data, method).words));
        }
    }
}

impl std::fmt::Debug for OutlineCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutlineCard")
            .field("title", &self.node.title)
            .finish()
    }
}

/// `dd/mm/yyyy hh:mm`, matching the comment card's stamp — the app's only other
/// rendered timestamp, and not worth a second spelling.
fn stamp(t: chrono::DateTime<chrono::Utc>) -> String {
    use chrono::{Datelike, Timelike};
    // `DateTime<Utc>::naive_local` is a no-op: chrono reads "local" as
    // local-to-the-`Tz`-parameter, and UTC's offset is zero by definition. It
    // reads like a conversion and performs none, so every timestamp in the app
    // was shown in UTC while claiming to be the writer's own clock. Convert to
    // the machine timezone first, which is what was meant.
    let t = t.with_timezone(&chrono::Local).naive_local();
    format!(
        "{:02}/{:02}/{:04} {:02}:{:02}",
        t.day(),
        t.month(),
        t.year(),
        t.hour(),
        t.minute()
    )
}

/// One `label / value` pair, stacked so a long value keeps the full width.
///
/// The dim/bright pairing is `TooltipShortcut`/`TooltipText`, not
/// `Secondary`/`Primary`: this paints on the tooltip's permanently dark chrome.
fn field(label: LocalizedString, value: String) -> impl Widget {
    VStack::new()
        .spacing(1.0)
        .child(
            TextWidget::new(label)
                .style(TextStyleRole::Tiny)
                .color(TextRole::TooltipShortcut),
        )
        .child(
            TextWidget::new(lit!(value))
                .style(TextStyleRole::Small)
                .color(TextRole::TooltipText),
        )
}

/// The short pairs, two per row, so the card uses the width it has instead of
/// growing into a tall ribbon.
///
/// A `Grid` rather than a `ColumnFlow`: the flow packs a flat child list
/// column-major with uniform column widths, which would scatter these pairs
/// across unrelated columns, and with a fixed card width its reflow-by-width
/// behaviour buys nothing.
fn facts_grid(cells: Vec<WidgetId>) -> impl Widget {
    let mut grid = Grid::new()
        .columns(vec![TrackSize::Fractional(1.0), TrackSize::Fractional(1.0)])
        .column_gap(12.0)
        .row_gap(LINE_SPACING);
    for cell in cells {
        grid = grid.add_child(cell);
    }
    grid
}

impl Widget for OutlineCard {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let (name, badge) = crate::models::label_and_badge(
            &self.node.title,
            self.node.fallback_label.as_deref(),
            self.node.number,
        );

        // ── The short facts, paired into two columns ────────────────────────
        let mut cells: Vec<WidgetId> = Vec::new();
        if !self.is_binder() {
            cells.push(
                ctx.add(field(
                    tr!(card_type()),
                    crate::binder::create_labels::item_type_label(
                        &self.node.role,
                        &self.node.sub_role,
                    )
                    .resolve_now(),
                )),
            );
        }
        if let Some((pos, total)) = self.position {
            cells.push(ctx.add(field(tr!(card_position()), format!("{pos} / {total}"))));
        }
        if self.child_count > 0 {
            cells.push(ctx.add(field(tr!(card_children()), self.child_count.to_string())));
        }
        if !self.is_binder() {
            cells.push(ctx.add(field(
                tr!(card_exportable()),
                if self.node.is_exportable {
                    tr!(card_yes()).resolve_now()
                } else {
                    tr!(card_no()).resolve_now()
                },
            )));
            // Only rows that open a structural level have an ordinal to
            // suppress, and the stored field is the inverted exception — both
            // rules mirrored from the Inspector, which owns the toggle.
            if skribisto_model::numbering::level_of(&self.node.sub_role).is_some() {
                cells.push(ctx.add(field(
                    tr!(card_numbered()),
                    if self.node.exclude_from_numbering {
                        tr!(card_no()).resolve_now()
                    } else {
                        tr!(card_yes()).resolve_now()
                    },
                )));
            }
            // 0 is the "no goal" sentinel — the field is a bare i64, so an
            // unconditional row would print "Goal: 0" on every untargeted row.
            if self.node.word_count_goal > 0 {
                cells.push(ctx.add(field(
                    tr!(card_goal()),
                    self.node.word_count_goal.to_string(),
                )));
            }
        }
        if let Some(created) = self.node.created_at {
            cells.push(ctx.add(field(tr!(card_created()), stamp(created))));
        }
        if let Some(updated) = self.node.updated_at {
            cells.push(ctx.add(field(tr!(card_modified()), stamp(updated))));
        }

        // ── Tags, resolved against the cached palette ───────────────────────
        let tag_rows = if self.node.tags.is_empty() {
            Vec::new()
        } else {
            ctx.app_state::<crate::view_models::TagsViewModel>()
                .cloned()
                .map(|vm| {
                    let lookup = vm.lookup_signal().get();
                    let mut rows: Vec<_> = self
                        .node
                        .tags
                        .iter()
                        .filter_map(|id| lookup.get(id).cloned())
                        .collect();
                    crate::models::sort_rows(&mut rows);
                    rows
                })
                .unwrap_or_default()
        };

        // ── Point of view, resolved against the mention index ───────────────
        let pov = if self.node.point_of_view.is_empty() {
            String::new()
        } else {
            ctx.app_state::<crate::view_models::MentionIndex>()
                .cloned()
                .map(|mi| {
                    crate::tags::pov::pov_chips(&mi.discoverable_table(), &self.node.point_of_view)
                        .into_iter()
                        .map(|c| c.title)
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default()
        };

        let mut column = VStack::new().spacing(LINE_SPACING);

        // ── Identity ────────────────────────────────────────────────────────
        let mut header = HStack::new().spacing(6.0);
        if badge.is_some() {
            header = header.child(crate::widgets::StructureNumber::new(badge));
        }
        column = column.child(
            header.child(
                TextWidget::new(lit!(name))
                    .style(TextStyleRole::BodyBold)
                    .color(TextRole::TooltipText),
            ),
        );
        if !self.node.sub_title.is_empty() {
            column = column.child(
                TextWidget::new(lit!(self.node.sub_title.clone()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::TooltipText),
            );
        }
        if !self.breadcrumb.is_empty() {
            column = column.child(
                TextWidget::new(lit!(self.breadcrumb.clone()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::TooltipShortcut),
            );
        }

        // ── What it is about ────────────────────────────────────────────────
        if !self.node.label.is_empty() {
            column = column.child(field(tr!(card_label()), self.node.label.clone()));
        }
        // Absent until the first paint fills it, and absent for good on a row
        // with no synopsis — so the card never reserves space for a blank.
        column = column.child(DeferredLine::new(
            tr!(card_synopsis()),
            self.synopsis.clone(),
        ));

        // ── Who and what ────────────────────────────────────────────────────
        if !pov.is_empty() {
            column = column.child(field(tr!(card_point_of_view()), pov));
        }
        if !self.node.aliases.is_empty() {
            column = column.child(field(tr!(card_aliases()), self.node.aliases.join(", ")));
        }
        if !tag_rows.is_empty() {
            column = column.child(TagChipRow::new(tag_rows, MAX_VISIBLE_EDITOR));
        }

        // ── The facts ───────────────────────────────────────────────────────
        column = column.child(Spacer::new().min_length(SECTION_SPACING - LINE_SPACING));
        column = column.child(WordsLine::new(
            self.own_words.clone(),
            self.node.word_count_goal,
        ));
        column = column.child(facts_grid(cells));

        let id = ctx.add(FixedSize::new().width(CARD_WIDTH).child(column));
        self.root = Some(id);
        vec![id]
    }

    fn paint(&self, _bounds: Rect, _canvas: &mut Canvas, _ctx: &PaintContext) {
        // The visibility hook: a tooltip body only paints once it is shown, so
        // the reads below are never paid for a row nobody hovered.
        if !self.loaded.replace(true) {
            self.load_deferred();
        }
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

/// A `label / value` pair whose value arrives after the first paint.
///
/// Renders nothing until its signal holds something, so a row with no synopsis
/// shows no empty row.
struct DeferredLine {
    label: LocalizedString,
    value: Signal<Option<String>>,
    root: Option<WidgetId>,
}

impl DeferredLine {
    fn new(label: LocalizedString, value: Signal<Option<String>>) -> Self {
        Self {
            label,
            value,
            root: None,
        }
    }
}

impl std::fmt::Debug for DeferredLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeferredLine").finish()
    }
}

impl Widget for DeferredLine {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the deferred read lands.
        self.value.bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            bastyde::core::binding::BindingLevel::Rebuild,
        );
        let Some(text) = self.value.get() else {
            self.root = None;
            return Vec::new();
        };
        let id = ctx.add(field(self.label.clone(), text));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.root {
            Some(id) => ctx
                .child_size(id, proposal)
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into()),
            None => proposal.resolve(0.0, 0.0).into(),
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The word count, against its goal when there is one.
///
/// Its own widget for the same reason as [`DeferredLine`]: the count arrives on
/// the first paint.
struct WordsLine {
    words: Signal<Option<usize>>,
    goal: i64,
    root: Option<WidgetId>,
}

impl WordsLine {
    fn new(words: Signal<Option<usize>>, goal: i64) -> Self {
        Self {
            words,
            goal,
            root: None,
        }
    }
}

impl std::fmt::Debug for WordsLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WordsLine").finish()
    }
}

impl Widget for WordsLine {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.words.bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            bastyde::core::binding::BindingLevel::Rebuild,
        );
        let Some(words) = self.words.get() else {
            self.root = None;
            return Vec::new();
        };
        let value = if self.goal > 0 {
            format!("{words} / {}", self.goal)
        } else {
            words.to_string()
        };
        let id = ctx.add(field(tr!(card_words()), value));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.root {
            Some(id) => ctx
                .child_size(id, proposal)
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into()),
            None => proposal.resolve(0.0, 0.0).into(),
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// Whether this row is worth a card at all.
///
/// A row with nothing but a bare title would open a card repeating what the row
/// already shows, which is worse than no card: it costs a surface and says
/// nothing. Deliberately does **not** consider the structural facts (position,
/// child count) or the always-safe optional ones: position is near-universal,
/// so counting it would make every row qualify and the gate would do nothing.
pub fn worth_a_card(node: &TreeNode) -> bool {
    node.kind == "binder"
        || !node.label.is_empty()
        || !node.tags.is_empty()
        || !node.is_exportable
        || node.created_at.is_some()
        || node.number.is_some()
        || node.sub_role != BinderItemSubRole::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    fn node(kind: &str) -> TreeNode {
        TreeNode {
            title: "Welcome".into(),
            label: "first draft".into(),
            kind: kind.into(),
            sub_role: BinderItemSubRole::Scene,
            item_id: Some(7),
            binder_id: Some(1),
            uid: uuid::Uuid::from_u128(7),
            number: None,
            fallback_label: None,
            tags: Vec::new(),
            role: frontend::common::entities::BinderItemRole::Item,
            sub_title: String::new(),
            word_count_goal: 0,
            aliases: Vec::new(),
            dict_language: Vec::new(),
            point_of_view: Vec::new(),
            is_exportable: true,
            exclude_from_numbering: false,
            created_at: chrono::DateTime::from_timestamp(1_700_000_000, 0),
            updated_at: chrono::DateTime::from_timestamp(1_700_500_000, 0),
        }
    }

    fn card(node: TreeNode, breadcrumb: &str) -> OutlineCard {
        OutlineCard::new(
            node,
            breadcrumb.to_string(),
            Some((3, 7)),
            0,
            std::rc::Rc::new(frontend::AppContext::new()),
            Signal::new(Default::default()),
        )
    }

    fn built(card: OutlineCard) -> WidgetTree {
        let mut tree = WidgetTree::new();
        tree.add(card);
        tree.layout(bastyde::canvas::SizeProposal::exact(500.0, 600.0));
        tree
    }

    /// The card says where the row sits and what it is, and lays out.
    #[test]
    fn an_item_card_carries_its_breadcrumb_label_and_stamps() {
        let tree = built(card(node("item"), "Manuscript \u{203a} Book One"));
        assert!(
            tree.find_by_label("Manuscript \u{203a} Book One").is_some(),
            "the breadcrumb is the whole point of the card"
        );
        assert!(tree.find_by_label("first draft").is_some(), "the label");
        assert!(
            tree.find_by_label("Welcome").is_some(),
            "and the row's own name"
        );
    }

    /// Position among siblings is the orientation the breadcrumb cannot give.
    #[test]
    fn the_card_says_where_the_row_sits_among_its_siblings() {
        let tree = built(card(node("item"), ""));
        assert!(
            tree.find_by_label("3 / 7").is_some(),
            "the third of seven is what a writer scanning a chapter wants"
        );
    }

    /// A `Binder` has no label, tags, export flag or ordinal — the card must
    /// omit those rows rather than render them blank or, worse, as `no`.
    #[test]
    fn a_binder_card_omits_the_fields_that_do_not_exist_on_a_binder() {
        let mut n = node("binder");
        n.label.clear();
        let tree = built(card(n, ""));
        assert!(
            tree.find_by_label("Exported").is_none(),
            "a binder is never excluded from the export, so the row is a lie"
        );
        assert!(
            tree.find_by_label("Numbered").is_none(),
            "and it carries no ordinal to suppress"
        );
        assert!(
            tree.find_by_label("Type").is_none(),
            "nor does it have an item type"
        );
    }

    /// `exclude_from_numbering` is stored inverted; the card must read it that
    /// way, and only for rows that open a structural level.
    #[test]
    fn the_numbered_row_inverts_the_stored_field_and_is_gated_by_level() {
        let tree = built(card(node("item"), ""));
        assert!(
            tree.find_by_label("Numbered").is_none(),
            "a scene has no ordinal, so the row must not appear at all"
        );

        let mut chapter = node("item");
        chapter.sub_role = BinderItemSubRole::ChapterScene;
        chapter.exclude_from_numbering = true;
        let tree = built(card(chapter, ""));
        assert!(
            tree.find_by_label("Numbered").is_some(),
            "a chapter-opening row does carry the numbering row"
        );
    }

    /// 0 is the "no goal" sentinel, not a goal of zero.
    #[test]
    fn a_row_with_no_goal_shows_no_goal_row() {
        let tree = built(card(node("item"), ""));
        assert!(
            tree.find_by_label("Goal").is_none(),
            "an unconditional row would print a goal of 0 on every untargeted row"
        );

        let mut targeted = node("item");
        targeted.word_count_goal = 2_000;
        let tree = built(card(targeted, ""));
        assert!(tree.find_by_label("Goal").is_some());
        assert!(tree.find_by_label("2000").is_some());
    }

    /// Aliases and subtitle appear only when they exist.
    #[test]
    fn optional_text_fields_appear_only_when_filled() {
        let tree = built(card(node("item"), ""));
        assert!(tree.find_by_label("Also known as").is_none());

        let mut named = node("item");
        named.aliases = vec!["Lizzy".into(), "Miss Bennet".into()];
        named.sub_title = "a beginning".into();
        let tree = built(card(named, ""));
        assert!(tree.find_by_label("Lizzy, Miss Bennet").is_some());
        assert!(tree.find_by_label("a beginning").is_some());
    }

    /// The deferred rows are absent until something paints.
    ///
    /// This is the whole point of the lazy design: an unhovered row's card is
    /// built but never painted, so it never pays the two backend reads. Laying
    /// out alone must therefore leave the synopsis and word rows empty — only
    /// `render()`, a real paint pass driven headlessly, can fill them.
    #[test]
    fn the_deferred_rows_are_absent_until_the_card_is_painted() {
        let mut tree = built(card(node("item"), ""));
        assert!(
            tree.find_by_label("Synopsis").is_none(),
            "laying out must not trigger the deferred read"
        );
        assert!(tree.find_by_label("Words").is_none());

        // A real paint pass. With no project loaded there is nothing to fetch,
        // so the rows stay absent — which is also the contract: a row with no
        // synopsis shows no empty synopsis row.
        let _ = tree.render();
        assert!(
            tree.find_by_label("Synopsis").is_none(),
            "an item with no content must not sprout a blank row"
        );
    }

    /// A row with nothing to add beyond its own title gets no card: a surface
    /// that repeats the row is worse than none.
    #[test]
    fn a_bare_row_is_not_worth_a_card() {
        let bare = TreeNode {
            title: "Scene".into(),
            kind: "item".into(),
            ..Default::default()
        };
        assert!(!worth_a_card(&bare));
        assert!(
            worth_a_card(&node("item")),
            "but one carrying a label and timestamps is"
        );
    }
}
