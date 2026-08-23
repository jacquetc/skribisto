// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **search & replace** dock (leading side): the query, the matching options,
//! the field scopes, the six facet chips, and the result list. It is fronted by a
//! second activity-bar glyph beside the binder outline; activating a result
//! reveals the [bottom preview dock](super::preview_dock) over that result's
//! paragraph.
//!
//! [`search_dock`] packages it as a `DockWidget` for `App` to mount; the private
//! [`SearchDockRoot`] is the dock-content root whose `build` wires the
//! view-model's debounced re-search (it needs a `BuildContext`). Business logic
//! lives entirely in [`SearchReplaceViewModel`]; the widgets here are thin.

use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{
    ActivateOn, Button, ButtonVariant, Checkbox, Divider, DockOpenLocation, DockSide, DockWidget,
    DockWidgetId, Expand, FocusScope, HStack, IconButton, IconLocation, IconWidget, ListView,
    Padding, ScrollBarMode, SearchField, StandardListItem, TextWidget, TraversalScopePolicy,
    VStack, Wrap,
};

use crate::search::SearchReplaceViewModel;
use crate::tabs::shared::editor::VisibleWhen;

use frontend::common::entities::{BinderItemSubRole, MatchField};
use frontend::direct_access::SearchResultDto;

use skribisto_model::SearchFacet;

/// Build the search & replace panel as a leading-side `DockWidget`. Shares the
/// one `SearchReplaceViewModel` with the bottom preview dock.
pub fn search_dock(vm: SearchReplaceViewModel, dock_id: DockWidgetId) -> DockWidget {
    DockWidget::new(dock_id, tr!(search()), move |_id| {
        // Continue scope: groups the panel's Tab order without trapping the
        // keyboard inside it (same policy the outline dock uses).
        SearchDockRoot::new(
            vm.clone(),
            FocusScope::new(TraversalScopePolicy::Continue).child(search_panel(vm.clone())),
        )
    })
    .icon(crate::icons::activity::search_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Leading))
}

/// The panel: a fixed header (query, replace, options, facets, count) over a
/// scrolling result list.
fn search_panel(vm: SearchReplaceViewModel) -> impl Widget {
    let header = Padding::symmetric(8.0, 8.0).child(
        VStack::new()
            .spacing(6.0)
            .child(query_row(vm.clone()))
            .child(replace_row(vm.clone()))
            .child(matching_options(vm.clone()))
            .child(scope_options(vm.clone()))
            .child(facet_chips(vm.clone()))
            .child(status_line(vm.clone())),
    );

    VStack::new()
        .spacing(0.0)
        .child(header)
        .child(Divider::horizontal())
        .child(Expand::new().child(results_list(vm)))
}

/// The query field + the disclosure toggle for the replace row.
fn query_row(vm: SearchReplaceViewModel) -> impl Widget {
    let submit = vm.clone();
    let suggestions = vm.query_suggestions_signal();
    let field = SearchField::new(vm.query_signal())
        .placeholder(tr!(search_query_placeholder()))
        // History suggestions (MRU, per project) — the field filters them by the
        // typed prefix; an empty prefix offers the whole recent list.
        .with_suggestions(move |typed| {
            let typed = typed.to_lowercase();
            suggestions
                .get()
                .into_iter()
                .filter(|s| typed.is_empty() || s.to_lowercase().contains(&typed))
                .collect()
        })
        .on_submit_fn(move |_ctx| submit.commit_query())
        .rich_tooltip(crate::tooltip_registry::CONCEPT_SEARCH_REPLACE);

    HStack::new()
        .spacing(4.0)
        .child(Expand::horizontal().child(field))
        .child(
            // A distinct replace glyph (two swap arrows), NOT the search magnifier
            // — the toggle used to read as a second search field. Rich tooltip
            // explains it discloses the replacement row.
            IconButton::new(crate::icons::find::replace_icon())
                .toolbar()
                .toggle(vm.show_replace_signal())
                .rich_tooltip_content(TooltipContent::new(
                    "search-tip-replace".to_string(),
                    tr!(search_tip_replace()),
                )),
        )
}

/// The replace row, disclosed behind the query-row toggle: the replacement field
/// with the Replace All action beside it, and the preserve-case option.
///
/// Gated by [`VisibleWhen`] (not a `Switcher`): dormant and zero-height when
/// closed, the full controls when open — so the replacement field is actually
/// shown the moment the toggle is on, with no reserved empty gap.
fn replace_row(vm: SearchReplaceViewModel) -> impl Widget {
    // Replace All is enabled only for a complete (non-truncated) scan with
    // results — a capped scan cannot honestly claim completeness.
    let enabled = vm
        .ran_signal()
        .zip(&vm.truncated_signal())
        .zip(&vm.item_count_signal())
        .map(|((ran, trunc), items)| *ran && !*trunc && *items > 0);

    let replace_vm = vm.clone();
    let controls = VStack::new()
        .spacing(6.0)
        .child(
            HStack::new()
                .spacing(4.0)
                .child(
                    Expand::horizontal().child(
                        SearchField::new(vm.replacement_signal())
                            .placeholder(tr!(search_replace_placeholder())),
                    ),
                )
                .child(
                    // Replace All as a primary (filled), icon-only button beside
                    // the field — compact in the narrow dock. The label is still
                    // set for accessibility + the tooltip.
                    Button::new(tr!(search_replace_all()))
                        .variant(ButtonVariant::Filled)
                        .icon(crate::icons::find::replace_icon(), IconLocation::IconOnly)
                        .tooltip(tr!(search_replace_all()))
                        .enabled(enabled)
                        .on_activate_fn(move |ctx| {
                            crate::search::replace_flow::confirm_and_replace(&replace_vm, ctx)
                        }),
                ),
        )
        .child(Checkbox::new(vm.preserve_case_signal()).label(tr!(search_preserve_case())));

    VisibleWhen::new(vm.show_replace_signal(), controls)
}

/// A flat, flowing option toggle: a ghost (`.toolbar()`) `IconButton` bound to a
/// `bool` signal, with a rich tooltip explaining what it does. The building block
/// of the matching / scope / facet rows — an icon toggle rather than a labelled
/// checkbox, so the options flow compactly instead of stacking one per row.
fn option_toggle(
    icon: IconWidget,
    signal: Signal<bool>,
    tip_key: String,
    tip: LocalizedString,
) -> IconButton {
    IconButton::new(icon)
        .toolbar()
        .toggle(signal)
        .rich_tooltip_content(TooltipContent::new(tip_key, tip))
}

/// How a match is compared — case / whole word / accents. Flat toggle icons that
/// flow onto the next line when the dock is narrow.
fn matching_options(vm: SearchReplaceViewModel) -> impl Widget {
    use crate::icons::find as ic;
    Wrap::new()
        .spacing(3.0)
        .child(option_toggle(
            ic::case_icon(),
            vm.case_sensitive_signal(),
            "search-tip-case".into(),
            tr!(search_tip_case()),
        ))
        .child(option_toggle(
            ic::whole_word_icon(),
            vm.whole_word_signal(),
            "search-tip-whole-word".into(),
            tr!(search_tip_whole_word()),
        ))
        .child(option_toggle(
            ic::diacritics_icon(),
            vm.diacritic_sensitive_signal(),
            "search-tip-diacritics".into(),
            tr!(search_tip_diacritics()),
        ))
}

/// Which fields to search — body / title / synopsis / label.
fn scope_options(vm: SearchReplaceViewModel) -> impl Widget {
    use crate::icons::find as ic;
    Wrap::new()
        .spacing(3.0)
        .child(option_toggle(
            ic::body_icon(),
            vm.search_body_signal(),
            "search-tip-body".into(),
            tr!(search_tip_body()),
        ))
        .child(option_toggle(
            ic::title_icon(),
            vm.search_titles_signal(),
            "search-tip-title".into(),
            tr!(search_tip_title()),
        ))
        .child(option_toggle(
            ic::synopsis_icon(),
            vm.search_synopsis_signal(),
            "search-tip-synopsis".into(),
            tr!(search_tip_synopsis()),
        ))
        .child(option_toggle(
            ic::label_icon(),
            vm.search_labels_signal(),
            "search-tip-label".into(),
            tr!(search_tip_label()),
        ))
        // Comments are a scope of their own rather than part of the body: a comment
        // is *about* the manuscript, and a writer hunting a phrase in the prose does
        // not always want their own notes about that phrase back as well.
        .child(option_toggle(
            crate::icons::activity::comments_icon(),
            vm.search_comments_signal(),
            "search-tip-comment".into(),
            tr!(search_tip_comment()),
        ))
}

/// The six facet chips — which KINDS of item to show. Multi-select flat icon
/// toggles that flow; nothing ticked means "all kinds", which is what a filter
/// with nothing selected means. (A plain flow of `IconButton::toggle`s, not a
/// `Toolbar`: a toggle+icon `ToolbarAction` panics in the overflow menu, and this
/// narrow dock always overflows six chips.)
fn facet_chips(vm: SearchReplaceViewModel) -> impl Widget {
    let mut row = Wrap::new().spacing(3.0);
    for facet in SearchFacet::ALL {
        row = row.child(option_toggle(
            facet_icon(facet),
            vm.facet_signal(facet),
            format!("search-tip-facet-{facet:?}"),
            facet_tip(facet),
        ));
    }
    row
}

/// The reactive count / empty-state / error line under the options.
fn status_line(vm: SearchReplaceViewModel) -> impl Widget {
    // Error wins over the count: a failed scan must not read as "0 matches".
    let text = vm
        .error_signal()
        .zip(&vm.ran_signal())
        .zip(&vm.match_count_signal())
        .zip(&vm.item_count_signal())
        .zip(&vm.truncated_signal())
        .map(|((((err, ran), matches), items), trunc)| {
            if let Some(msg) = err {
                tr!(search_error(message = msg.clone())).resolve_now()
            } else if !*ran {
                String::new()
            } else if *matches == 0 {
                tr!(search_no_matches()).resolve_now()
            } else if *trunc {
                tr!(search_count_truncated(
                    matches = *matches as i64,
                    items = *items as i64
                ))
                .resolve_now()
            } else {
                tr!(search_count(
                    matches = *matches as i64,
                    items = *items as i64
                ))
                .resolve_now()
            }
        });

    TextWidget::new(lit!(""))
        .text(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// The result list — one row per matching field, grouped visually by its item
/// title. Activating a row previews it (and reveals the bottom band).
fn results_list(vm: SearchReplaceViewModel) -> impl Widget {
    let model = vm.results().list_model();
    let selected = vm.selected_result_signal();
    let row_vm = vm.clone();

    ListView::new(model.clone(), move |_i, row: &SearchResultDto, _sel| {
        let result_id = row.id;
        let is_selected = selected.map(move |s| *s == Some(result_id));
        let snippet = format!(
            "{}{}{}",
            row.snippet_before, row.snippet_match, row.snippet_after
        );
        let trailing = HStack::new()
            .spacing(6.0)
            .child(
                TextWidget::new(field_label(&row.match_field))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(
                TextWidget::new(tr!(search_occurrences(count = row.occurrence_count as i64)))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );
        Box::new(
            StandardListItem::new(lit!(row.item_title.clone()))
                .subtitle(lit!(snippet))
                .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                .leading_slot(ExclusionCheck::new(row_vm.clone(), result_id))
                .trailing_slot(trailing)
                .selected(is_selected),
        ) as Box<dyn Widget>
    })
    .auto_item_height(48.0)
    .scroll_bar_style(ScrollBarMode::Overlay)
    // Single-click previews a result (the outline convention); the default
    // is DoubleClick, which would leave the preview stubbornly empty on one click.
    .activate_on(ActivateOn::SingleClick)
    .on_activate({
        let vm = vm.clone();
        let model = model.clone();
        move |idx, _ctx| {
            if let Some(row) = model.with_item(idx, |r| r.clone()) {
                vm.select_result(row.id);
            }
        }
    })
}

/// The label for a result's matched field (the trailing badge).
fn field_label(field: &MatchField) -> LocalizedString {
    match field {
        MatchField::Body => tr!(search_field_body()),
        MatchField::Title => tr!(search_field_title()),
        MatchField::Synopsis => tr!(search_field_synopsis()),
        MatchField::Label => tr!(search_field_label()),
        MatchField::Epigraph => tr!(search_field_epigraph()),
        MatchField::Comment => tr!(search_field_comment()),
        MatchField::CommentReply => tr!(search_field_comment_reply()),
        MatchField::Footnote => tr!(search_field_footnote()),
    }
}

/// The rich-tooltip explanation for a facet chip.
fn facet_tip(facet: SearchFacet) -> LocalizedString {
    match facet {
        SearchFacet::Book => tr!(search_tip_book()),
        SearchFacet::Part => tr!(search_tip_part()),
        SearchFacet::Chapter => tr!(search_tip_chapter()),
        SearchFacet::Scene => tr!(search_tip_scene()),
        SearchFacet::Note => tr!(search_tip_note()),
        SearchFacet::Paratext => tr!(search_tip_paratext()),
        SearchFacet::Folder => tr!(search_tip_folder()),
    }
}

/// The binder glyph for a facet chip — the same icon the binder tree uses for
/// that kind of row, so the two stay coherent.
fn facet_icon(facet: SearchFacet) -> IconWidget {
    let sub_role = match facet {
        SearchFacet::Book => BinderItemSubRole::Book,
        SearchFacet::Part => BinderItemSubRole::Part,
        SearchFacet::Chapter => BinderItemSubRole::ChapterScene,
        SearchFacet::Scene => BinderItemSubRole::Scene,
        SearchFacet::Note => BinderItemSubRole::Note,
        SearchFacet::Paratext => BinderItemSubRole::Paratext,
        SearchFacet::Folder => BinderItemSubRole::None,
    };
    crate::binder::icons::sub_role_icon(&sub_role)
}

/// A per-result checkbox that ticks whether Replace All includes this field.
///
/// `Checkbox` binds a two-way `Signal<bool>` and has no separate change callback,
/// so this owns a **bridge** signal seeded from the view-model's exclusion set
/// (checked = *included*) and, in `build`, registers a `ctx.effect` on the bridge
/// that pushes each flip back into the set. Being a real widget, the effect is
/// owned here and torn down when the row rebuilds — no leak, and a fresh search
/// (which rebuilds every row) re-seeds each bridge from the reset set.
struct ExclusionCheck {
    vm: SearchReplaceViewModel,
    result_id: u64,
    child_id: Option<WidgetId>,
}

impl ExclusionCheck {
    fn new(vm: SearchReplaceViewModel, result_id: u64) -> Self {
        Self {
            vm,
            result_id,
            child_id: None,
        }
    }
}

impl std::fmt::Debug for ExclusionCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExclusionCheck").finish()
    }
}

impl Widget for ExclusionCheck {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let included = Signal::new(!self.vm.is_excluded(self.result_id));
        let vm = self.vm.clone();
        let id = self.result_id;
        ctx.effect(&included, move |checked| vm.set_excluded(id, !checked));
        let checkbox = Checkbox::new(included)
            .labels_hidden(true)
            .tooltip(tr!(search_include_in_replace()));
        self.child_id = Some(ctx.add(checkbox));
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.child_id.and_then(|id| ctx.child_size(id, proposal)) {
            Some(size) => size.into(),
            None => proposal.resolve(0.0, 0.0).into(),
        }
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
        self.child_id.into_iter().collect()
    }
}

/// Dock-content root: wires the view-model's debounced re-search on `build`
/// (which needs a `BuildContext`) and otherwise is a transparent pass-through
/// filling its bounds with the panel — same shape as the outline's `OutlineKeys`.
struct SearchDockRoot {
    vm: SearchReplaceViewModel,
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl SearchDockRoot {
    fn new(vm: SearchReplaceViewModel, child: impl Widget + 'static) -> Self {
        Self {
            vm,
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl std::fmt::Debug for SearchDockRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchDockRoot").finish()
    }
}

impl Widget for SearchDockRoot {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.child_id.and_then(|id| ctx.child_size(id, proposal)) {
            Some(size) => size.into(),
            None => proposal.resolve(0.0, 0.0).into(),
        }
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
        self.child_id.into_iter().collect()
    }
}
