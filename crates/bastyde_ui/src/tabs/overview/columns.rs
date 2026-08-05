// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Overview table's columns.
//!
//! Eight: **Title** (the tree column — twist, indent, icon), **Type**, **Label**, **Tags**,
//! **Own words**, **Total words**, **Comments** (open) and **Total comments**. Title and
//! Label are editable in place; Tags is edited through its own picker; the derived columns
//! are read-only, because a word count is not something you type.
//!
//! Every column id is a constant from `crate::models`, beside the comparator it selects —
//! a column whose id drifted from its comparator would render fine and silently stop
//! sorting.

#[allow(unused_imports)]
use super::*;

use bastyde::widgets::{
    CellContext, Column, ColumnWidth, PinnedSide, TableAlignment, TextInput, TruncationPolicy,
};
use std::rc::Rc;

use uuid::Uuid;

use crate::models::{
    COL_LABEL, COL_OPEN_COMMENTS, COL_OWN_WORDS, COL_TAGS, COL_TITLE, COL_TOTAL_COMMENTS,
    COL_TOTAL_WORDS, COL_TYPE,
};

/// Build the column set for a table bound to `vm`.
///
/// **The widths are a budget.** `TreeTableView` falls back to an internal horizontal
/// scrollbar for overflow, but the goal is a set that never needs it: the six fixed
/// columns cost 464 dp between them and the two flexible ones carry low minimums (120 +
/// 72), so the whole set still fits an editor pane in a split window (~610 dp) with the
/// outline and inspector docks open. Widen any of them and check that case — the margin
/// is now thin.
pub(super) fn overview_columns(vm: &OverviewViewModel) -> Vec<Column<OverviewRow>> {
    vec![
        title_column(vm),
        type_column(vm),
        label_column(vm),
        tags_column(vm),
        own_words_column(vm),
        total_words_column(vm),
        open_comments_column(vm),
        total_comments_column(vm),
    ]
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
                TextWidget::new(lit!(row.label.clone()))
                    .color(TextRole::Secondary)
                    .single_line(),
            )
        },
    )
    .width(ColumnWidth::Flex(1.5))
    .min_width(72.0)
    .sortable(true)
    .editable(true)
    .truncation(TruncationPolicy::Ellipsis)
}

/// **Tags** — the row's tags as the same coloured dot row the stream, corkboard and
/// editor use ([`TagDotsRow`](crate::tags::TagDotsRow)). Hover inspects one, click opens
/// the picker for all of them.
///
/// Dots rather than named chips, for the same reason as everywhere outside the Inspector:
/// away from the place you *manage* tags, the job is passive awareness — noticing a scene
/// is still a draft without having asked — and a table row has no width to spend on names.
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
fn tags_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_TAGS,
        tr!(overview_col_tags()),
        move |row: &OverviewRow, _cx| {
            if row.tags.is_empty() {
                // An untagged row gets an empty cell, not an empty dot row — the whole point
                // of the column is that a glance distinguishes tagged from untagged.
                return with_row_menu(&vm, row, TextWidget::new(lit!(String::new())));
            }
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
            Box::new(crate::tags::TagDotsRow::new(
                value,
                set,
                crate::tags::tag_chip::MAX_VISIBLE_OVERVIEW,
            ))
        },
    )
    .width(ColumnWidth::Fixed(72.0))
    // The dot row caps itself at MAX_VISIBLE_OVERVIEW and shows its own overflow
    // count, so there is nothing for the column to elide - an ellipsis after the
    // dots would read as one more glyph rather than as truncation.
    .truncation(TruncationPolicy::None)
}

/// **Own words** — this row's own prose only.
///
/// Blank (not `0`) when the row carries no prose at all: a Part has nothing to count,
/// which is a different fact from an empty Scene, and printing `0` for both would make
/// the column lie about which pieces are still unwritten — the one question it exists to
/// answer.
fn own_words_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(
        COL_OWN_WORDS,
        tr!(overview_col_own_words()),
        move |row, _cx| with_row_menu(&vm, row, word_cell(row.own_words)),
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
        move |row, _cx| with_row_menu(&vm, row, word_cell(Some(row.total_words))),
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

/// A right-aligned count, or a muted dash when there is nothing to count.
fn word_cell(words: Option<usize>) -> impl Widget {
    match words {
        Some(n) => TextWidget::new(lit!(format_count(n))).color(TextRole::Secondary),
        // An em dash, not "0" — see `own_words_column`.
        None => TextWidget::new(lit!("—".to_string())).color(TextRole::Disabled),
    }
    .single_line()
}

/// Group a count with thin spaces, so a five-figure book total stays readable at a
/// glance. Locale-independent on purpose: a thin space reads correctly everywhere a
/// comma or a period would be ambiguous between the two conventions.
fn format_count(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('\u{202F}'); // narrow no-break space
        }
        out.push(c);
    }
    out
}

/// The in-place cell editor: a text input over the **view-model's** buffer, committing on
/// Enter and on focus loss, abandoning on Esc.
///
/// The buffer is `vm.edit_buffer().text`, deliberately **not** a `Signal` created here.
/// A cell delegate re-runs on every table rebuild, and this table rebuilds on any reload
/// — the other pane's autosave firing `Content(Updated)`, an undo, a rename elsewhere. A
/// locally-created buffer would be re-seeded from the model each time and the writer's
/// half-typed name would vanish with no warning. Owned by the view-model, it survives
/// every rebuild.
///
/// **Focus loss commits.** A text field that disappears without writing is the writer's
/// edit thrown away; clicking another cell, switching segment, or closing the tab all
/// reach `commit_open_edit`. The view-model decides whether anything is actually written
/// (an unchanged value, or a blank title, writes nothing), so no path leaves a stray undo
/// entry.
fn cell_editor(vm: &OverviewViewModel, uid: Uuid, col_id: &'static str) -> impl Widget {
    let buffer = vm.edit_buffer();
    let commit_vm = vm.clone();
    let commit_text = buffer.text.clone();
    let blur_vm = vm.clone();
    let cancel_vm = vm.clone();
    TextInput::new(buffer.text)
        .on_submit_fn(move |_ctx| commit_vm.commit_edit(uid, col_id, &commit_text.get()))
        .on_focus(move |focused, _ctx| {
            if !focused {
                blur_vm.commit_open_edit();
            }
        })
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
    #[test]
    fn counts_group_in_threes() {
        assert_eq!(format_count(0), "0");
        assert_eq!(format_count(999), "999");
        assert_eq!(format_count(1_000), "1\u{202F}000");
        assert_eq!(format_count(12_345), "12\u{202F}345");
        assert_eq!(format_count(1_234_567), "1\u{202F}234\u{202F}567");
    }
}
