// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Overview table's columns.
//!
//! Eight: **Title** (the tree column — twist, indent, icon), **Type**, **Label**, **Tags**,
//! **Own words**, **Total words**, **Comments** (open) and **Total comments**. Title, Label
//! and Target are editable in place; Status and Tags are edited through their own pickers;
//! the derived columns are read-only, because a word count is not something you type.
//!
//! ## One rule for the mouse
//!
//! **Every editable cell answers a single click, except the tree column, where selection
//! has to win.** Status and Tags have always worked that way (their cell *is* the picker),
//! so Label and Target join them: [`EditTriggers::SINGLE_CLICK`] on those two columns, and
//! the writer never has to know a key to change a value.
//!
//! Title is the exception and stays as it was — a click selects, a double-click opens the
//! item — because it is the row's own column: the one you click to pick a row and
//! double-click to go and write in it. It renames from **F2** (a click already moves the
//! cell cursor there, so a click then F2 works with no keyboard navigation at all) and from
//! the context menu's Rename.
//!
//! The table therefore takes [`EditTriggers::F2`] as its own set, which switches **two**
//! defaults off deliberately:
//! * `DOUBLE_CLICK`, which would fight `on_row_activate` on the Title column, and
//! * `ANY_KEY` (type-to-edit), which does not survive contact with this table: the keystroke
//!   that opens the editor is lost (the editor is not built until the next frame), so typing
//!   `N` over a scene left the *old* title with no `N` in it — and it shadowed type-ahead on
//!   the one column where jumping to a row by title is worth having.
//!
//! Every column id is a constant from `crate::models`, beside the comparator it selects —
//! a column whose id drifted from its comparator would render fine and silently stop
//! sorting.

#[allow(unused_imports)]
use super::*;

use std::rc::Rc;
use teksilo::widgets::{
    CellContext, Column, ColumnWidth, EditTriggers, PinnedSide, TableAlignment, TextInput,
    TruncationPolicy,
};

use uuid::Uuid;

// The same grouping every other count in the app uses — the Inspector's readout, the
// status bar, the Distribute preview — so one number never looks like two.
use crate::goals::format_count;
use crate::models::COL_GOAL;
use crate::models::{
    COL_BOOKS, COL_LABEL, COL_OPEN_COMMENTS, COL_OWN_WORDS, COL_STATUS, COL_TAGS, COL_TITLE,
    COL_TOTAL_COMMENTS, COL_TOTAL_WORDS, COL_TYPE,
};

/// Build the column set for a table bound to `vm`.
///
/// **The widths are a budget.** `TreeTableView` falls back to an internal horizontal
/// scrollbar for overflow, but the goal is a set that never needs it: the seven fixed
/// columns cost 540 dp between them and the two flexible ones carry low minimums (120 +
/// 72), so the whole set still fits an editor pane in a split window (~610 dp) with the
/// outline and inspector docks open. Widen any of them and check that case — the margin
/// is thin, which is exactly why the Target column prints a bare number rather than the
/// "1 234 / 2 000" string that would read better and cost another 130 dp.
pub(super) fn overview_columns(
    vm: &OverviewViewModel,
    books: &[(u64, String)],
) -> Vec<Column<OverviewRow>> {
    let mut cols = vec![
        title_column(vm),
        type_column(vm),
        label_column(vm),
        status_column(vm),
        tags_column(vm),
    ];
    // Gated the same way every other Books surface in this edition is: below two
    // Books in the Work, no column at all, not a disabled or an empty one. See
    // `docks::inspector::live_books`'s own doc for why this reads the same
    // candidate table rather than a second, independently-drifting one.
    //
    // **Taken as an argument, not read here.** Both the gate and the titles it resolves
    // are only as current as the build that ran them, and this one runs inside
    // `OverviewTable::build`, which rebuilds for the edit cursor and the projection flag
    // and nothing else. `OverviewTable` keeps them in a signal it re-reads on the binder
    // events that can change them; see its own `build`.
    if books.len() >= 2 {
        cols.push(books_column(vm, books));
    }
    cols.extend([
        own_words_column(vm),
        total_words_column(vm),
        goal_column(vm),
        open_comments_column(vm),
        total_comments_column(vm),
    ]);
    cols
}

/// Every live Book in the Work, as `(id, title)` in binder order.
///
/// [`crate::docks::inspector::live_books`] answered as-is, so the Overview cannot drift
/// from the Inspector's own candidate table about which rows count as a Book or which of
/// them are still live. Reduced to a plain, comparable pair so
/// [`OverviewTable`] can hold it in a `Signal` and rebuild
/// only when the answer actually changes - a `CastCandidate` is not `PartialEq`, and a
/// rebuild per unrelated binder event would tear down the table under the writer.
pub(super) fn live_book_titles(
    app_ctx: &std::rc::Rc<frontend::AppContext>,
    ids: &crate::app_ids::AppIds,
) -> Vec<(u64, String)> {
    crate::docks::inspector::live_books(app_ctx, ids)
        .into_iter()
        .map(|c| (c.id, c.title))
        .collect()
}

/// Give a cell the row's context menu.
///
/// Attached per **cell** rather than to the table, so right-clicking anywhere along a row
/// opens that row's menu: a cell already knows which row it belongs to, whereas a menu on
/// the table would only know a pointer position and would have to hit-test its own rows
/// back into one.
///
/// The menu deliberately does **not** select the row first. Selecting rebuilds the table,
/// which destroys the widget the overlay is anchored to — and the menu then opens in the
/// window corner. (The outline tree carries the same note for the same reason.) The
/// batch convention in `OverviewViewModel::batch_for` is what makes not-selecting correct
/// rather than merely convenient.
fn with_row_menu(
    vm: &OverviewViewModel,
    row: &OverviewRow,
    cell: impl Widget + 'static,
) -> Box<dyn Widget> {
    let vm = vm.clone();
    // Capture the **uid only**, and re-resolve the row when the menu is actually built.
    // Cloning the row here would clone its title, label and synopsis excerpt once per
    // cell per row per rebuild — five copies of every row in the container, on every
    // repaint of the table — to serve a menu that opens on maybe one of them. Resolving
    // late also means the menu reflects the row as it is *when right-clicked*, not as it
    // was when the cell was last built.
    let uid = row.uid;
    Box::new(cell.context_menu(move |_pos, _ctx| {
        let row = vm.row_of(&uid)?;
        Some(Box::new(overview_context_menu(vm.clone(), uid, row)) as Box<dyn Widget>)
    }))
}

/// **Title** — the tree column: it hosts the twist and the depth indent, so it must stay
/// leftmost. Pinned leading and non-reorderable for that reason: a tree column dragged
/// into the middle would leave the hierarchy drawn in a column the eye no longer reads as
/// the spine. (It is also what keyboard expand/collapse targets, via the widget's
/// `tree_column_display_pos`.)
///
/// Editable in place — renaming a row is the single most common thing to do in an
/// outliner, and the alternative is a modal for a five-character change.
fn title_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_TITLE,
        tr!(overview_col_title()),
        move |row: &OverviewRow, cx: &CellContext| {
            if cx.is_editing {
                return Box::new(cell_editor(&vm, row.uid, COL_TITLE));
            }
            // The row's icon comes from its sub-role, exactly as in the outline tree — the
            // same item must not wear two different glyphs in two views.
            let (label, badge) = crate::models::label_and_badge(
                &row.title,
                row.fallback_label.as_deref(),
                row.number,
            );
            with_row_menu(
                &vm,
                row,
                HStack::new()
                    .spacing(6.0)
                    .child(crate::binder::icons::sub_role_icon(&row.sub_role))
                    // The ordinal, in the read cell only. The edit branch above returns
                    // early to `cell_editor`, which seeds from `row.title` — so neither the
                    // number nor the generated fallback name can end up inside a rename.
                    .child(crate::widgets::StructureNumber::new(badge))
                    .child(TextWidget::new(lit!(label)).single_line()),
            )
        },
    )
    .width(ColumnWidth::Flex(3.0))
    .min_width(120.0)
    .pinned(PinnedSide::Leading)
    .reorderable(false)
    .sortable(true)
    .editable(true)
    // Inherits the table's `F2` — deliberately no click trigger. This is the
    // column you click to select a row and double-click to open it; a click
    // that renamed instead would make the table unusable for its main job, and
    // a click trigger also *claims the press*, so the row would stop selecting.
    .truncation(TruncationPolicy::Ellipsis)
}

/// **Type** — what this row is structurally (Book / Part / Chapter / Scene / Note / …).
/// Read-only: a type *change* is a conversion with guards, so it belongs in "Convert to
/// ▸", not in a cell you can tab into.
fn type_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(COL_TYPE, tr!(overview_col_type()), move |row, _cx| {
        with_row_menu(
            &vm,
            row,
            TextWidget::new(crate::binder::create_labels::item_type_label(
                &row.role,
                &row.sub_role,
            ))
            .color(TextRole::Secondary)
            .single_line(),
        )
    })
    .width(ColumnWidth::Fixed(88.0))
    .sortable(true)
    .truncation(TruncationPolicy::Ellipsis)
}

/// **Label** — the writer's own status note on the row ("needs a pass", "cut?"). Free
/// text by design: a fixed status vocabulary is someone else's process.
fn label_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_LABEL,
        tr!(overview_col_label()),
        move |row: &OverviewRow, cx: &CellContext| {
            if cx.is_editing {
                return Box::new(cell_editor(&vm, row.uid, COL_LABEL));
            }
            with_row_menu(
                &vm,
                row,
                crate::widgets::tip::RichTip::new(
                    crate::tooltip_registry::CONCEPT_LABEL,
                    TextWidget::new(lit!(row.label.clone()))
                        .color(TextRole::Secondary)
                        .single_line(),
                ),
            )
        },
    )
    .width(ColumnWidth::Flex(1.5))
    .min_width(72.0)
    .sortable(true)
    .editable(true)
    // One click edits: a label is nothing but its value, so there is no other
    // meaning for a click on it to have. See the module doc's "One rule for the
    // mouse".
    .edit_triggers(EditTriggers::F2 | EditTriggers::SINGLE_CLICK)
    .truncation(TruncationPolicy::Ellipsis)
}

/// **Status** — where this row is on the project's workflow ladder.
///
/// The glyph alone, never glyph + word: this table's width budget is already thin (the
/// fixed columns cost 540 dp against the ~610 dp an editor pane has in a split window with
/// both docks open, which is why the Target column prints a bare number). The name arrives
/// on hover, and the Inspector shows both.
///
/// **Sortable**, unlike Tags — and that is the whole difference between the two axes. A set
/// of dots has no natural order; a ladder is nothing but an order, so "show me the least
/// finished first" is a real question with a real answer. It sorts by ladder position, not
/// by name: see `comparator`.
///
/// An unset row renders a **faint dashed ring**, not nothing — and that is a concession the
/// framework forces rather than the design's first choice. The cell is the picker, and a
/// button with no glyph is a target the writer cannot aim at; rendering it only on hover is
/// not available either, because `CellContext::is_hovered` is hardcoded `false` in the
/// table body. What saves it is the tint: unset draws in `TextRole::Disabled` while every
/// real rung draws in a live role, so a column of unset rows still recedes and a set one
/// still pops. The Inspector, which is not a grid of buttons, does say "No status" in
/// words.
///
/// ⚠ **This column costs the width budget.** `columns.rs`' own header records that the
/// fixed columns already come to ~540 dp against the ~610 dp an editor pane has in a split
/// window with the outline and inspector docks open. This one is deliberately the narrowest
/// interactive cell in the table, and it still pushes that case into a horizontal scroll.
/// The honest options if that matters more than having it here are to drop another column
/// or to make the set user-choosable; neither is decided.
fn status_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_STATUS,
        tr!(overview_col_status()),
        move |row: &OverviewRow, _cx| {
            let statuses = vm.statuses();
            // A container whose subtree disagrees with it gets one asterisk beside the
            // glyph — the *derived* half of the answer, and marked as derived rather than
            // drawn as a second status. An authored value always wins; this only ever
            // annotates it. Never in the binder tree, only here, where the row already sits
            // among subtree sums.
            let mark = if row.subtree_differs { "∗" } else { "" };
            let set: crate::statuses::SetStatus = {
                let statuses = statuses.clone();
                let item_id = row.item_id;
                Rc::new(move |status| statuses.set_item_status(item_id, status))
            };
            Box::new(
                HStack::new()
                    .spacing(1.0)
                    .child(crate::statuses::status_picker_dense(
                        &statuses,
                        row.status,
                        row.subtree_differs,
                        set,
                    ))
                    // The asterisk carries the concept tooltip, so hovering the mark
                    // explains what a derived value is rather than only naming the rung.
                    .child(crate::widgets::tip::RichTip::new(
                        crate::tooltip_registry::CONCEPT_STATUS,
                        TextWidget::new(lit!(mark.to_string()))
                            .color(TextRole::Secondary)
                            .single_line(),
                    )),
            )
        },
    )
    .width(ColumnWidth::Fixed(44.0))
    .min_width(44.0)
    .sortable(true)
    .truncation(TruncationPolicy::None)
}

/// **Tags** — the row's tags as the same coloured dot row the stream, corkboard and
/// editor use ([`TagDotsRow`](crate::tags::TagDotsRow)). Hover inspects one, click opens
/// the picker for all of them.
///
/// Dots rather than named chips, for the same reason as everywhere outside the Inspector:
/// away from the place you *manage* tags, the job is passive awareness — noticing a scene
/// is still a draft without having asked — and a table row has no width to spend on names.
///
/// **An untagged row gets a muted `+`, not an empty cell** — a reversal of what this
/// column used to say, and worth recording. The reasoning was "the whole point of the
/// column is that a glance distinguishes tagged from untagged", which is right; what it
/// missed is that the cell *is* the picker here, so a blank cell was not a restrained
/// empty state but a **dead one**: there was no route to a first tag anywhere in the
/// Overview, which is precisely the surface a writer opens to go down the rows filling
/// things in. The Status column beside it had already made the same concession for the
/// same reason (`statuses::picker::trigger_icon` draws unset in `TextRole::Disabled`
/// rather than nothing), and the tint is what keeps the original goal: a column of
/// untagged rows still recedes at a glance, and a tagged one still pops. The other three
/// dot-row surfaces keep the blank — see [`crate::tags::TagDotsRow::offering_when_empty`].
///
/// **Not sortable.** A set of dots has no natural order: by count is not a question anyone
/// asks, and by "first tag" would depend on an order the writer never chose. Finding
/// tagged rows is a filter question, not a sort one.
///
/// The per-cell `Signal` is created here rather than owned by the view-model — the
/// opposite of [`cell_editor`]'s buffer, and deliberately. This one *mirrors committed
/// state*: the picker's commit writes through to the backend and the resulting reload
/// re-seeds it from the truth, so a rebuild re-seeding it is correct. The edit buffer
/// holds **uncommitted input**, which a rebuild would destroy.
pub(super) fn tags_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_TAGS,
        tr!(overview_col_tags()),
        move |row: &OverviewRow, _cx| {
            let value = Signal::new(row.tags.clone());
            let set: crate::tags::tag_pill_field::SetTags = {
                let vm = vm.clone();
                let item_id = row.item_id;
                let mirror = value.clone();
                Rc::new(move |ids: Vec<u64>, _ctx| {
                    vm.set_tags(item_id, &ids);
                    mirror.set(ids); // optimistic; the reload re-seeds from the backend
                })
            };
            Box::new(
                crate::tags::TagDotsRow::new(
                    value,
                    set,
                    crate::tags::tag_chip::MAX_VISIBLE_OVERVIEW,
                )
                .offering_when_empty()
                .with_palette(vm.tags()),
            )
        },
    )
    .width(ColumnWidth::Fixed(72.0))
    // The dot row caps itself at MAX_VISIBLE_OVERVIEW and shows its own overflow
    // count, so there is nothing for the column to elide - an ellipsis after the
    // dots would read as one more glyph rather than as truncation.
    .truncation(TruncationPolicy::None)
}

/// **Books**: which Book or Books the row's own `book_ids` declares it filed under,
/// resolved to titles. Only ever built when the Work has two or more live Books
/// ([`overview_columns`]'s own gate); a row with no filing prints nothing at all:
/// **empty means "not yet filed", never "every Book"** (see `common::entities::BinderItem::books`'s
/// own doc), so a blank cell here is the honest answer, not a missing one.
///
/// Read-only and unsortable, for the same reason the Tags column beside it is
/// unsortable: which Book or Books a row answers to is a fact you narrow by, the
/// filter chip row does exactly that job for tags, not one you'd ever want the
/// table's own row order to follow.
pub(super) fn books_column(vm: &OverviewViewModel, books: &[(u64, String)]) -> Column<OverviewRow> {
    let vm = vm.clone();
    let titles: std::collections::HashMap<u64, String> = books.iter().cloned().collect();
    Column::new(
        COL_BOOKS,
        tr!(overview_col_books()),
        move |row: &OverviewRow, _cx| {
            // An id that no longer resolves to a live Book (trashed, deleted, or never
            // valid) is dropped rather than shown as a blank placeholder chip: the
            // same unresolved-target rule the Inspector's own book chip row applies.
            let text = row
                .book_ids
                .iter()
                .filter_map(|id| titles.get(id))
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            with_row_menu(
                &vm,
                row,
                TextWidget::new(lit!(text))
                    .color(TextRole::Secondary)
                    .single_line(),
            )
        },
    )
    .width(ColumnWidth::Flex(1.2))
    .min_width(90.0)
    .truncation(TruncationPolicy::Ellipsis)
}

/// **Own words** — this row's own prose only.
///
/// Blank (not `0`) when the row carries no prose at all: a Part has nothing to count,
/// which is a different fact from an empty Scene, and printing `0` for both would make
/// the column lie about which pieces are still unwritten — the one question it exists to
/// answer.
///
/// A row the export leaves out still prints its **real** length, dimmed. It contributes
/// nothing to any total beside it, and the two columns stop agreeing for that row — which
/// is the honest outcome: "how long is this piece" stays true of a scene the writer has
/// cut from the book, while "how much book is in here" does not. Printing `0` instead
/// would make a 1 200-word scene read as unwritten.
fn own_words_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_OWN_WORDS,
        tr!(overview_col_own_words()),
        move |row: &OverviewRow, _cx: &CellContext| {
            if row.is_exportable {
                with_row_menu(&vm, row, word_cell(row.own_words))
            } else {
                with_row_menu(&vm, row, excluded_word_cell(row.own_words))
            }
        },
    )
    .width(ColumnWidth::Fixed(76.0))
    .alignment(TableAlignment::Trailing)
    .sortable(true)
}

/// **Total words** — this row plus its whole subtree, correct even while collapsed.
fn total_words_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_TOTAL_WORDS,
        tr!(overview_col_total_words()),
        move |row, _cx| {
            with_row_menu(
                &vm,
                row,
                crate::widgets::tip::RichTip::new(
                    crate::tooltip_registry::GOAL_MANUSCRIPT_WORDS,
                    word_cell(Some(row.total_words)),
                ),
            )
        },
    )
    .width(ColumnWidth::Fixed(76.0))
    .alignment(TableAlignment::Trailing)
    .sortable(true)
}

/// **Comments** — open threads anchored to this row's own prose.
///
/// Open rather than total, and blank rather than `0`: a row with nothing left to
/// address should read as quiet. Printing `0` on every settled scene would turn a
/// column meant to draw the eye into visual noise on the majority of rows.
fn open_comments_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_OPEN_COMMENTS,
        tr!(overview_col_comments()),
        move |row: &OverviewRow, _cx| {
            let n = row.own_comments;
            with_row_menu(&vm, row, word_cell((n > 0).then_some(n)))
        },
    )
    .width(ColumnWidth::Fixed(76.0))
    .alignment(TableAlignment::Trailing)
    .sortable(true)
}

/// **Total comments** — this row plus its whole subtree, correct while collapsed.
/// The number that makes a collapsed chapter say "there is still work inside me".
fn total_comments_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_TOTAL_COMMENTS,
        tr!(overview_col_total_comments()),
        move |row: &OverviewRow, _cx| {
            let n = row.total_comments;
            with_row_menu(&vm, row, word_cell((n > 0).then_some(n)))
        },
    )
    .width(ColumnWidth::Fixed(76.0))
    .alignment(TableAlignment::Trailing)
    .sortable(true)
}

/// **Target** — how long this row is meant to be, in the project's unit.
///
/// A bare number, editable in place. Deliberately **not** a "1 234 / 2 000" string and
/// deliberately not paired with a "total target" column: the width budget above has no
/// room for the first, and the second would be the mistake every writer of a competing
/// tool has complained about for fifteen years — a container's figure moving on its own
/// because a scene inside it was given one. Progress rolls up in this app; targets do not.
///
/// The colour carries the progress instead, against the same number the **Total** column
/// shows for the row, so the two cells cannot tell different stories.
fn goal_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_GOAL,
        tr!(overview_col_goal()),
        move |row: &OverviewRow, cx: &CellContext| {
            if cx.is_editing {
                return Box::new(cell_editor(&vm, row.uid, COL_GOAL));
            }
            if row.goal <= 0 {
                return with_row_menu(&vm, row, word_cell(None));
            }
            // Measured against this row's own prose when it has any, else against
            // everything beneath it — the same rule the Inspector's readout uses.
            let written = row.own_words.unwrap_or(row.total_words) as i64;
            let ratio = crate::goals::ratio(written, row.goal).unwrap_or(0.0);
            with_row_menu(
                &vm,
                row,
                TextWidget::new(lit!(crate::goals::format_goal(row.goal)))
                    .color(crate::goals::target_role(ratio))
                    .single_line(),
            )
        },
    )
    .width(ColumnWidth::Fixed(76.0))
    .alignment(TableAlignment::Trailing)
    .sortable(true)
    .editable(true)
    // One click edits, as on Label — and here it matters more: an unset target
    // renders as a muted dash, so without a click route the writer would have
    // to guess that the empty-looking cell was typeable at all.
    .edit_triggers(EditTriggers::F2 | EditTriggers::SINGLE_CLICK)
}

/// A right-aligned count, or a muted dash when there is nothing to count.
fn word_cell(words: Option<usize>) -> impl Widget {
    match words {
        Some(n) => TextWidget::new(lit!(format_count(n))).color(TextRole::Secondary),
        // An em dash, not "0" — see `own_words_column`.
        None => TextWidget::new(lit!("—".to_string())).color(TextRole::Disabled),
    }
    .single_line()
}

/// The same count, dimmed, for a row the export leaves out — with the reason on hover.
///
/// Dimmed rather than hidden or zeroed: the writing is still there and its length is still
/// the answer to "how long is this piece". What it no longer is, is part of the book, which
/// is what the neighbouring **Total** column stops counting it in.
fn excluded_word_cell(words: Option<usize>) -> impl Widget {
    let text = match words {
        Some(n) => TextWidget::new(lit!(format_count(n))),
        None => TextWidget::new(lit!("—".to_string())),
    }
    .color(TextRole::Disabled)
    .single_line();
    crate::widgets::tip::RichTip::new(crate::tooltip_registry::GOAL_EXPORTABLE, text)
}

/// The in-place cell editor: a text input over the **view-model's** buffer, committing on
/// Enter, abandoning on Esc.
///
/// The buffer is `vm.edit_buffer().text`, deliberately **not** a `Signal` created here.
/// A cell delegate re-runs on every table rebuild, and this table rebuilds on any reload
/// — the other pane's autosave firing `Content(Updated)`, an undo, a rename elsewhere. A
/// locally-created buffer would be re-seeded from the model each time and the writer's
/// half-typed name would vanish with no warning. Owned by the view-model, it survives
/// every rebuild.
///
/// **Clicking away commits too**, and that arrives from the table
/// (`on_cell_edit_dismissed`, wired in [`OverviewTable`]),
/// not from here. It used to be an `on_focus` handler on this very `TextInput` — which
/// compiles, reads correctly, and **never fires**: the focusable node is the inner
/// `TextInputField`, which registers an `on_focus` of its own, and a handler that fires
/// answers `Handled`, so the bubble stops one node below this wrapper. For as long as
/// that was the mechanism, an open editor could not be dismissed by clicking anywhere at
/// all — and, still holding the keyboard, it swallowed every click and keystroke after
/// it, which is what made double-click-to-open and the row context menu look broken too.
///
/// The view-model decides whether anything is actually written (an unchanged value, or a
/// blank title, writes nothing), so no path leaves a stray undo entry.
fn cell_editor(vm: &OverviewViewModel, uid: Uuid, col_id: &'static str) -> impl Widget {
    let buffer = vm.edit_buffer();
    let commit_vm = vm.clone();
    let commit_text = buffer.text.clone();
    let cancel_vm = vm.clone();
    TextInput::new(buffer.text)
        .on_submit_fn(move |_ctx| commit_vm.commit_edit(uid, col_id, &commit_text.get()))
        .on_key(move |ev, _ctx| {
            if let WidgetEvent::KeyDown {
                key: Key::Escape, ..
            } = ev
            {
                cancel_vm.cancel_edit();
                return EventResponse::Handled;
            }
            EventResponse::Ignored
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Counts are grouped for readability, and short ones are left alone.
    ///
    /// The grouping itself is covered where it lives (`crate::goals::format`); this pins
    /// the fact that the table still uses that one, so the Overview's numbers cannot start
    /// looking different from the Inspector's.
    #[test]
    fn counts_group_in_threes() {
        assert_eq!(format_count(0), "0");
        assert_eq!(format_count(999), "999");
        assert_eq!(format_count(1_000), "1\u{202F}000");
        assert_eq!(format_count(12_345), "12\u{202F}345");
        assert_eq!(format_count(1_234_567), "1\u{202F}234\u{202F}567");
    }

    /// **One rule for the mouse**, pinned as a table rather than as prose.
    ///
    /// Every editable cell answers a single click except the tree column, where
    /// selection has to win. The row this exists to stop coming back is the Title
    /// one: a click trigger there *claims the press*, so the row would stop
    /// selecting and double-click would stop opening the item — the two things the
    /// Overview is mostly used for.
    ///
    /// Reading `effective_edit_triggers` and not the column's own field on purpose:
    /// that is the answer the body pane and the key handler actually act on, and it
    /// folds in both the table's set and the `editable` flag, so a column that
    /// forgot `editable(true)` fails here rather than looking configured.
    #[cfg(feature = "mocks")]
    #[test]
    fn only_the_non_tree_value_columns_edit_on_a_single_click() {
        use teksilo::widgets::EditTriggers;

        let vm = crate::overview::OverviewViewModel::new(
            std::rc::Rc::new(frontend::AppContext::new()),
            crate::app_ids::AppIds::new(),
            101,
            &frontend::common::entities::BinderItemRole::Folder,
            &frontend::common::entities::BinderItemSubRole::Book,
            Signal::new(Default::default()),
            crate::settings::TreeExpansionViewModel::new(
                std::rc::Rc::new(frontend::AppContext::new()),
                crate::app_ids::AppIds::new(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(Default::default()),
        )
        .expect("a Book is overview-capable");

        // The set `OverviewTable::build` gives the table. Named here rather than
        // read back, so a change to it has to be made in both places on purpose.
        let table_set = EditTriggers::F2;
        let columns = overview_columns(&vm, &live_book_titles(&vm.app_ctx(), &vm.ids()));

        let triggers = |id: &str| {
            columns
                .iter()
                .find(|c| c.id() == id)
                .unwrap_or_else(|| panic!("no {id} column"))
                .effective_edit_triggers(table_set)
        };

        for id in [COL_LABEL, COL_GOAL] {
            assert!(
                triggers(id).contains(EditTriggers::SINGLE_CLICK),
                "{id} must open its editor on one click"
            );
        }

        assert!(
            !triggers(COL_TITLE).contains(EditTriggers::SINGLE_CLICK),
            "the tree column must not edit on a click: it would claim the press, and \
             clicking a row to select it (or double-clicking to open it) would stop working"
        );
        assert!(
            triggers(COL_TITLE).contains(EditTriggers::F2),
            "the tree column still renames from F2"
        );

        // ...and nothing anywhere edits on a double-click, which is what keeps
        // `on_row_activate` — open this scene — reachable on every column.
        for col in &columns {
            assert!(
                !col.effective_edit_triggers(table_set)
                    .contains(EditTriggers::DOUBLE_CLICK),
                "{} edits on double-click, which takes that gesture away from opening \
                 the row",
                col.id()
            );
        }

        // Type-to-edit is off everywhere: the keystroke that opens the editor is lost
        // (it is not built until the next frame), so it left the old value unchanged
        // *and* shadowed type-ahead on the Title column.
        for col in &columns {
            assert!(
                !col.effective_edit_triggers(table_set)
                    .contains(EditTriggers::ANY_KEY),
                "{} still type-to-edits, which eats the keystroke and kills type-ahead",
                col.id()
            );
        }
    }

    /// The Target column exists, sits with the other counts, and every column id has a
    /// comparator.
    ///
    /// The ids are the persistence key for sort, width and order, and the module's own doc
    /// warns that one drifting from its comparator would render fine and silently stop
    /// sorting — so the second half of this is the part worth having.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_target_column_is_in_the_set_and_sorts() {
        use crate::models::{COL_GOAL, COL_TOTAL_WORDS};
        let vm = crate::overview::OverviewViewModel::new(
            std::rc::Rc::new(frontend::AppContext::new()),
            crate::app_ids::AppIds::new(),
            101,
            &frontend::common::entities::BinderItemRole::Folder,
            &frontend::common::entities::BinderItemSubRole::Book,
            Signal::new(Default::default()),
            crate::settings::TreeExpansionViewModel::new(
                std::rc::Rc::new(frontend::AppContext::new()),
                crate::app_ids::AppIds::new(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(Default::default()),
        )
        .expect("a Book is overview-capable");

        let ids: Vec<String> = overview_columns(&vm, &live_book_titles(&vm.app_ctx(), &vm.ids()))
            .iter()
            .map(|c| c.id().to_string())
            .collect();
        assert!(
            ids.contains(&COL_GOAL.to_string()),
            "no Target column: {ids:?}"
        );
        let goal_at = ids.iter().position(|i| i == COL_GOAL).unwrap();
        let total_at = ids.iter().position(|i| i == COL_TOTAL_WORDS).unwrap();
        assert_eq!(
            goal_at,
            total_at + 1,
            "the Target column belongs with the counts, right after Total"
        );
    }

    /// **Below two Books, no column at all; at two, it appears.** The same
    /// "gated at the source, not per surface" discipline every other Books
    /// control in this edition follows. Deliberately **not** gated to a single
    /// feature set: `overview_columns` reads `live_books`, which goes straight
    /// through `frontend::commands::*`, real in both builds, unlike
    /// `OverviewViewModel`'s own row source, so a real, freshly seeded backend
    /// (not the `mocks` fixture ids the test above depends on) proves this in
    /// either build.
    #[test]
    fn the_books_column_appears_only_with_two_or_more_books() {
        use frontend::AppContext;
        use frontend::commands::{binder_commands, binder_item_commands, work_commands};
        use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
        use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

        let app_ctx = std::rc::Rc::new(AppContext::new());
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
        .expect("create binder");
        let container = binder_item_commands::create_binder_item(
            &app_ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: "Book One".into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder.id,
            0,
        )
        .expect("create Book One");
        let ids = crate::app_ids::AppIds::new();
        ids.work_id.set(Some(work.id));

        let vm = crate::overview::OverviewViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            container.id,
            &BinderItemRole::Folder,
            &BinderItemSubRole::Book,
            Signal::new(Default::default()),
            crate::settings::TreeExpansionViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(Default::default()),
        )
        .expect("a Book is overview-capable");

        let one_book: Vec<String> =
            overview_columns(&vm, &live_book_titles(&vm.app_ctx(), &vm.ids()))
                .iter()
                .map(|c| c.id().to_string())
                .collect();
        assert!(
            !one_book.contains(&COL_BOOKS.to_string()),
            "one Book: no Books column: {one_book:?}"
        );

        binder_item_commands::create_binder_item(
            &app_ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: "Book Two".into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder.id,
            1,
        )
        .expect("create Book Two");

        let two_books: Vec<String> =
            overview_columns(&vm, &live_book_titles(&vm.app_ctx(), &vm.ids()))
                .iter()
                .map(|c| c.id().to_string())
                .collect();
        assert!(
            two_books.contains(&COL_BOOKS.to_string()),
            "two Books: the Books column must appear: {two_books:?}"
        );
    }
}
