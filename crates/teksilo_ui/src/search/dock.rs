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
    ActivateOn, Badge, Button, ButtonVariant, Divider, DockOpenLocation, DockSide, DockWidget,
    DockWidgetId, Expand, FixedSize, FocusScope, HStack, IconButton, IconButtonSize, IconLocation,
    IconWidget, MenuItem, MenuList, Padding, ScrollBarMode, SearchField, StandardTreeItem,
    TextWidget, Toast, TraversalScopePolicy, TreeRow, TreeView, VStack, Wrap,
};

use teksilo::widgets::button::InteractionState;

use crate::app_ids::HasWorkId;
use crate::icons::find as ic;
use crate::models::SearchNode;
use crate::search::SearchReplaceViewModel;
use crate::tabs::shared::editor::VisibleWhen;
use crate::toast_scope::ToastWorkExt;

use frontend::common::entities::{BinderItemSubRole, MatchField};

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

/// The replace row, disclosed behind the query-row toggle: the replacement field,
/// the preserve-case option and the Replace All action, on one line.
///
/// Preserve case is an icon toggle beside the field rather than a labelled
/// checkbox under it, which is what every other option in this dock already is:
/// a labelled checkbox on its own row spent a whole line of a narrow dock saying
/// what the three matching toggles above it say in an icon each, and read as a
/// different *kind* of control than the option it is.
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
    let controls = VStack::new().spacing(6.0).child(
        HStack::new()
            .spacing(4.0)
            .child(
                Expand::horizontal().child(
                    SearchField::new(vm.replacement_signal())
                        .placeholder(tr!(search_replace_placeholder())),
                ),
            )
            // Between the field and the action, where it belongs: it is about
            // what goes IN, and a writer sets it before pressing the button.
            .child(option_toggle(
                crate::icons::find::preserve_case_icon(),
                vm.preserve_case_signal(),
                "search-tip-preserve-case".into(),
                tr!(search_tip_preserve_case()),
            ))
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
    );

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
    use crate::icons::find as ic;
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

    // The count on the left, the control that closes the tree on the right. It
    // lives here rather than in the query row because it is about the *results*,
    // and because the query row is already seven controls wide.
    let tree = vm.tree();
    let any_open = {
        let tree = tree.clone();
        tree.slice()
            .version_signal()
            .map(move |_| tree.any_expanded())
    };
    HStack::new()
        .child(
            // Width only: a plain `Expand` would stretch the count line to the
            // height of the button beside it and strand it at the top.
            Expand::horizontal().child(
                TextWidget::new(lit!(""))
                    .text(text)
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
        .child({
            // Beside the collapse, because both are about the *shape* of the
            // result rather than about the search: one puts the outline back, the
            // other puts a row back.
            let vm = vm.clone();
            IconButton::new(ic::undo_dismiss_icon())
                .toolbar()
                .enabled(vm.can_undo_dismiss_signal())
                .tooltip(tr!(search_undo_dismiss()))
                .on_activate_fn(move |_ctx| vm.undo_last_dismiss())
        })
        .child(
            IconButton::new(ic::collapse_all_icon())
                .toolbar()
                .enabled(any_open)
                .tooltip(tr!(search_collapse_all()))
                .on_activate_fn(move |_ctx| tree.collapse_all()),
        )
}

/// The result list — one row per matching field, grouped visually by its item
/// title. Activating a row previews it (and reveals the bottom band).
fn results_list(vm: SearchReplaceViewModel) -> impl Widget {
    let tree = vm.tree();
    let selected = vm.selected_occurrence_signal();
    let row_vm = vm.clone();
    let activate_vm = vm.clone();

    TreeView::from_source(
        // The **model**, not its slice. The model is the data source, and its
        // `set_expanded` is what fetches a branch before it opens -- every route to
        // opening one (the chevron, the keyboard, an accessibility action) funnels
        // through the source, and through nothing else.
        tree.clone(),
        move |node: &SearchNode, row: &TreeRow, _sel| {
            if node.is_item {
                // An item: its title, how many times the query occurs anywhere in it,
                // and one checkbox for the whole of it.
                // The framework's own badge, which the standard item's docs name as
                // what a trailing slot is for. It carries its own pill and its own
                // contrast pairing, where a bare `TextWidget` carried a hand-written
                // "x" to say what it was — and the pill says that on its own.
                let count = Badge::new(tr!(search_occurrences(
                    count = node.occurrence_count as i64
                )));
                let hover = Signal::new(InteractionState::Idle);
                let item = StandardTreeItem::new(lit!(node.title.clone()))
                    .depth(row.depth)
                    .has_children(row.has_children)
                    .is_expanded(row.is_expanded)
                    .interaction_signal(hover.clone())
                    .trailing_slot(row_actions(
                        &row_vm,
                        &hover,
                        count,
                        RowTarget::Item(node.binder_item_id),
                    ))
                    // The framework's own toggle, not a handler of our own. It
                    // routes through the data source, and the source is the model,
                    // which fetches the branch before it opens -- so the chevron,
                    // the keyboard and an accessibility action all take one path.
                    .on_toggle_rc(row.toggle_callback());
                let menu_vm = row_vm.clone();
                let target = RowTarget::Item(node.binder_item_id);
                Box::new(item.context_menu(move |_pos, _ctx| {
                    Some(Box::new(row_context_menu(&menu_vm, target)) as Box<dyn Widget>)
                })) as Box<dyn Widget>
            } else {
                // An occurrence: where it landed, and the text around it.
                let result_id = node.result_id;
                let hover = Signal::new(InteractionState::Idle);
                // Keyed on the occurrence, not on the row it is in: a result row is
                // a whole field, so every hit inside one carries the same id and
                // highlighting by it lit the whole scene at once.
                let at = (result_id, node.char_start);
                let is_selected = selected.map(move |s| *s == Some(at));
                let snippet = format!(
                    "{}{}{}",
                    node.snippet_before, node.snippet_match, node.snippet_after
                );
                // The excerpt with the match picked out of it. Three runs, not one
                // string: the label stays the whole excerpt, because that is what a
                // screen reader reads, while what is *drawn* is the run before, the
                // match, and the run after. See `StandardTreeItem::label_slot`.
                let excerpt = HStack::new()
                    .child(
                        // **Deliberately not shrinkable.** An overflow mode is what
                        // lets a run give up width, and this one has nothing to
                        // give: the backend already cut it to two words and marked
                        // the cut. Shrinking it further deletes the run-up entirely
                        // and leaves every row opening on the match itself, which
                        // reads as a list of the query repeated.
                        //
                        // The run *after* the match carries the whole deficit
                        // instead, and can: it is sixty characters where this is a
                        // handful. Something must be able to shrink, or the stack
                        // overflows the dock -- which this application has wedged a
                        // renderer with before, and which shows as hazard striping
                        // rather than as clipped text.
                        TextWidget::new(lit!(node.snippet_before.clone()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    )
                    .child(
                        // The match itself does not shrink. It is the one thing the
                        // row exists to show, and a hit ellipsed into "th…" has
                        // stopped being a search result.
                        TextWidget::new(lit!(node.snippet_match.clone()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Primary),
                    )
                    .child(
                        // **Not** expanded to fill: a greedy run after the match
                        // claims the row and the run *before* it gets shrunk away,
                        // which is the wrong half to lose. The words leading into a
                        // hit are what make it readable; the words after it are
                        // where the eye stops. Both flanks shrink when the column is
                        // tight, and this one has the most to give.
                        TextWidget::new(lit!(node.snippet_after.clone()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary)
                            .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing)),
                    );
                Box::new(
                    StandardTreeItem::new(lit!(snippet))
                        .depth(row.depth)
                        .has_children(false)
                        .label_slot(excerpt)
                        // The icon is the glance; the label beside it is the name. An
                        // `IconWidget` carries no accessible name and a row is named
                        // from its label alone, so an icon-only source would be
                        // invisible to a screen reader -- which is what the flat list
                        // this replaces did not do.
                        .interaction_signal(hover.clone())
                        .leading_slot(
                            field_icon(&node.match_field)
                                .icon_size(14.0)
                                .color(TextRole::Secondary),
                        )
                        .trailing_slot(row_actions(
                            &row_vm,
                            &hover,
                            TextWidget::new(field_label(&node.match_field))
                                .style(TextStyleRole::Tiny)
                                .color(TextRole::Secondary),
                            RowTarget::Occurrence(result_id, node.char_start),
                        ))
                        .selected(is_selected)
                        .context_menu({
                            let menu_vm = row_vm.clone();
                            let target = RowTarget::Occurrence(result_id, node.char_start);
                            move |_pos, _ctx| {
                                Some(Box::new(row_context_menu(&menu_vm, target))
                                    as Box<dyn Widget>)
                            }
                        }),
                ) as Box<dyn Widget>
            }
        },
    )
    .auto_item_height(32.0)
    // The model's, not the view's: the dock's content is rebuilt whenever the
    // layout changes, and activating the first result reveals the preview band,
    // which is such a change. See `SearchTreeModel::scroll`.
    .scroll_signal(vm.tree().scroll())
    .scroll_bar_style(ScrollBarMode::Overlay)
    // Single-click previews (the outline convention); the default DoubleClick would
    // leave the preview stubbornly empty on one click.
    .activate_on(ActivateOn::SingleClick)
    .on_activate(move |idx, _ctx| {
        // Activating an occurrence selects the row it is in. Activating an item does
        // nothing on its own -- its chevron is what it is for, and selecting the
        // first of its fields would send a writer somewhere they did not point at.
        activate_vm.activate_tree_row(idx);
    })
}

/// **The icon for a result's source**, for the occurrence rows under an item.
///
/// The reader's question at the second level of the tree is *where in this scene*,
/// and the answer has to be readable without going through a word: forty rows of
/// prose and one comment should separate at a glance.
///
/// A total `match` rather than a lookup with a fallback, deliberately. A ninth
/// `MatchField` would then fail the build here, where the answer is a five-minute
/// decision, instead of shipping as a blank column nobody notices for a release.
///
/// Two of the eight take glyphs of their own rather than the one their *scope
/// toggle* uses. The toggles answer "search here as well" and legitimately gate
/// epigraphs with the prose and replies with their threads; this answers "your hit
/// is here", and collapsing either pair would make the column stop doing the one
/// thing it is for. See [`icons::find::epigraph_icon`] and
/// [`icons::find::comment_reply_icon`].
fn field_icon(field: &MatchField) -> IconWidget {
    use crate::icons::activity;
    use crate::icons::find as ic;
    match field {
        MatchField::Body => ic::body_icon(),
        MatchField::Title => ic::title_icon(),
        MatchField::Synopsis => ic::synopsis_icon(),
        MatchField::Label => ic::label_icon(),
        MatchField::Epigraph => ic::epigraph_icon(),
        MatchField::Comment => activity::comments_icon(),
        MatchField::CommentReply => ic::comment_reply_icon(),
        MatchField::Footnote => activity::footnotes_icon(),
    }
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

/// How big a row's hover controls are, square.
///
/// Smaller than any of `IconButtonSize`'s own steps, because those are calibrated
/// to a toolbar and this has to fit inside a 32dp tree row without changing its
/// height. A row that grows under the pointer moves the control out from under
/// the hand reaching for it.
const ACTION_BUTTON: f32 = 18.0;

/// What a row's actions act on.
#[derive(Clone, Copy, Debug)]
enum RowTarget {
    /// A whole item: every hit in it.
    Item(u64),
    /// One occurrence, by the row it is in and where it starts.
    Occurrence(u64, i64),
}

/// **A row's trailing slot**: what it says at rest, and what it offers on hover.
///
/// The count or the field name is what the row is *for*; the actions are what a
/// writer does to it, and showing them on every row at all times turns a list into
/// a control panel. So they appear under the pointer, as they do in the panel this
/// is modelled on.
///
/// ⚠ **The buttons are `access_hidden`, and the context menu is why that is not a
/// regression.** A control that only exists while a pointer is over it does not
/// exist at all for a writer working from the keyboard or hearing the tree read
/// out, and an accessibility tree that announces two buttons nobody can reach is
/// worse than one that announces none. The same two actions are on every row's
/// context menu, which the keyboard opens and a screen reader reads -- that is the
/// route, and the hover buttons are the shortcut.
fn row_actions(
    vm: &SearchReplaceViewModel,
    hover: &Signal<InteractionState>,
    at_rest: impl Widget + 'static,
    target: RowTarget,
) -> impl Widget {
    let active = hover.map(|s| matches!(s, InteractionState::Hovered | InteractionState::Pressed));
    let idle = active.map(|a| !*a);

    // Boxed to the row's own text height. `Compact` is 24dp and the row is 32,
    // which with the row's padding is enough to grow it -- so the tree jumped
    // every time the pointer crossed a row, and the control moved out from under
    // the pointer reaching for it.
    let boxed = |button: IconButton| {
        FixedSize::new()
            .width(ACTION_BUTTON)
            .height(ACTION_BUTTON)
            .child(button)
    };
    let replace = {
        let vm = vm.clone();
        boxed(
            IconButton::new(ic::replace_icon())
                // `Compact`, not `toolbar`: a row is 32dp and a 30dp button plus its
                // padding grows it, so the tree jumped every time the pointer crossed
                // a row. The controls have to fit inside the row they belong to.
                .size(IconButtonSize::Compact)
                .tooltip(tr!(search_replace_here()))
                .on_activate_fn(move |ctx| run_row_replace(&vm, target, ctx)),
        )
        .access_hidden(true)
    };
    let dismiss = {
        let vm = vm.clone();
        boxed(
            IconButton::new(ic::dismiss_icon())
                .size(IconButtonSize::Compact)
                .tooltip(tr!(search_dismiss()))
                .on_activate_fn(move |_ctx| dismiss_row(&vm, target)),
        )
        .access_hidden(true)
    };
    HStack::new()
        .spacing(2.0)
        // Both halves are in the slot at once, each gated, so the row does not
        // change width under the pointer that is trying to hit the buttons.
        .child(VisibleWhen::new(idle, at_rest))
        .child(VisibleWhen::new(
            active.clone(),
            HStack::new()
                .spacing(2.0)
                .child(VisibleWhen::new(vm.show_replace_signal(), replace))
                .child(dismiss),
        ))
}

/// The same two actions, by the route a keyboard and a screen reader can take.
fn row_context_menu(vm: &SearchReplaceViewModel, target: RowTarget) -> MenuList {
    let mut menu = MenuList::new();
    if vm.show_replace_signal().get() {
        let vm = vm.clone();
        menu = menu.item(
            MenuItem::new(tr!(search_replace_here()))
                .on_activate_fn(move |ctx| run_row_replace(&vm, target, ctx)),
        );
    }
    let vm = vm.clone();
    menu.item(
        MenuItem::new(tr!(search_dismiss())).on_activate_fn(move |_ctx| dismiss_row(&vm, target)),
    )
}

/// Take a row out of the results.
fn dismiss_row(vm: &SearchReplaceViewModel, target: RowTarget) {
    match target {
        RowTarget::Item(item_id) => vm.dismiss_item(item_id),
        RowTarget::Occurrence(row, at) => vm.dismiss_occurrence(row, at),
    }
}

/// Replace a row's hits and nothing else.
///
/// The scope is read **before** the replace runs, because a row that loses its last
/// hit stops existing and there would be nothing left to name. Afterwards the panel
/// settles in place: this is one hit of one field, and re-running the whole search
/// over it would collapse every open row and scroll the writer back to the top to
/// re-derive a result set that differs only in what is named here.
fn run_row_replace(vm: &SearchReplaceViewModel, target: RowTarget, ctx: &mut EventContext) {
    let (rows, items) = match target {
        RowTarget::Item(item_id) => vm.scope_of(&[], &[item_id]),
        RowTarget::Occurrence(row, _) => vm.scope_of(&[row], &[]),
    };
    match match target {
        RowTarget::Item(item_id) => vm.replace_item(item_id),
        RowTarget::Occurrence(row, at) => vm.replace_occurrence(row, at),
    } {
        Ok(res) => {
            vm.settle_after_scoped_replace(&rows, &items);
            // A field whose text moved since the writer opened the row is refused
            // rather than guessed at, and refusing in silence looks exactly like a
            // button that does nothing. Replace All says so in its own toast; this
            // is the same sentence for the one-row route.
            if !res.skipped_stale.is_empty() {
                ctx.show_toast(
                    Toast::warning(tr!(search_replace_skipped_title()))
                        .body(tr!(search_replace_skipped_body(
                            fields = res.skipped_stale.len() as i64
                        )))
                        .scoped_id("search-replace-row", vm.work_id())
                        .target_work(vm.work_id()),
                );
            }
        }
        Err(e) => {
            ctx.show_toast(
                Toast::error(tr!(search_replace_failed_title()))
                    .body(lit!(e.to_string()))
                    .scoped_id("search-replace-row", vm.work_id())
                    .target_work(vm.work_id()),
            );
        }
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
