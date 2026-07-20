// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Overview table's columns.
//!
//! Five in v1: **Title** (the tree column — twist, indent, icon), **Type**, **Label**,
//! **Own words** and **Total words**. Title and Label are editable in place; the three
//! derived columns are read-only, because a word count is not something you type.
//!
//! Every column id is a constant from `crate::models`, beside the comparator it selects —
//! a column whose id drifted from its comparator would render fine and silently stop
//! sorting.

#[allow(unused_imports)]
use super::*;

use bastyde::widgets::{
    CellContext, Column, ColumnWidth, PinnedSide, TableAlignment, TextInput, TruncationPolicy,
};
use uuid::Uuid;

use crate::models::{COL_LABEL, COL_OWN_WORDS, COL_TITLE, COL_TOTAL_WORDS, COL_TYPE};

/// Build the column set for a table bound to `vm`.
///
/// **The widths are a budget, not preferences.** The table has no horizontal scrolling
/// yet, so a column that doesn't fit is a column that is *clipped* — and the first
/// casualty is the trailing one, which is `Total`, the number the writer most wants. The
/// three fixed columns therefore cost 240 dp between them and the two flexible ones carry
/// low minimums, so the whole set still fits an editor pane in a split window (~610 dp)
/// with the outline and inspector docks open. Widen any of them and check that case.
pub(super) fn overview_columns(vm: &OverviewViewModel) -> Vec<Column<OverviewRow>> {
    vec![
        title_column(vm),
        type_column(vm),
        label_column(vm),
        own_words_column(vm),
        total_words_column(vm),
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
    Column::new(COL_TITLE, tr!(overview_col_title()), move |row: &OverviewRow, cx: &CellContext| {
        if cx.is_editing {
            return Box::new(cell_editor(&vm, row.uid, COL_TITLE));
        }
        // The row's icon comes from its sub-role, exactly as in the outline tree — the
        // same item must not wear two different glyphs in two views.
        with_row_menu(
            &vm,
            row,
            HStack::new()
                .spacing(6.0)
                .child(crate::binder::icons::sub_role_icon(&row.sub_role))
                .child(TextWidget::new(lit!(row.title.clone())).single_line()),
        )
    })
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
    Column::new(COL_LABEL, tr!(overview_col_label()), move |row: &OverviewRow, cx: &CellContext| {
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
    })
    .width(ColumnWidth::Flex(1.5))
    .min_width(72.0)
    .sortable(true)
    .editable(true)
    .truncation(TruncationPolicy::Ellipsis)
}

/// **Own words** — this row's own prose only.
///
/// Blank (not `0`) when the row carries no prose at all: a Part has nothing to count,
/// which is a different fact from an empty Scene, and printing `0` for both would make
/// the column lie about which pieces are still unwritten — the one question it exists to
/// answer.
fn own_words_column(vm: &OverviewViewModel) -> Column<OverviewRow> {
    let vm = vm.clone();
    Column::new(COL_OWN_WORDS, tr!(overview_col_own_words()), move |row, _cx| {
        with_row_menu(&vm, row, word_cell(row.own_words))
    })
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

// TAGS COLUMN SEAM: the tags column belongs here, after Label. `OverviewRow::tags` is
// already carried (always empty in this build) so adding it is additive — a `Column` with
// a chip renderer over `row.tags`, plus a comparator in `overview_rows_model`. Tags are
// being implemented in a parallel worktree; do not populate the field from here.

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
