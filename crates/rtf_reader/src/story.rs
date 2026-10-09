// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One story being built: the main text, or one note's or one comment's.
//!
//! RTF writes a table as a flat run of paragraphs, each saying how deep in tables it sits
//! (`\intbl`, `\itapN`), with `\cell` and `\row` marks between them. The structure is
//! rebuilt here from those marks: a stack of open tables, one per nesting level, each with
//! the row and the cell being filled. A paragraph at a shallower level closes the deeper
//! tables, and a closed nested table becomes a block of the cell it sits in.
//!
//! Each step that builds structure says how many bytes it allocated, for the reading to
//! count against [`crate::Limits::max_document_bytes`]. Structure is most of what a hostile
//! file can make cost: a paragraph mark that opens every table level down to
//! [`MAX_TABLE_NESTING`] allocates tens of kilobytes from twenty bytes of input.

use std::mem::size_of;

use crate::model::{Block, Cell, Inline, Paragraph, ParagraphEnd, ParagraphProps, Row, Table};

/// How deeply tables may nest in the model. A paragraph deeper than this is kept in the
/// deepest table allowed and counted (see
/// [`crate::Diagnostic::TableNestingFlattened`]): Word nests tables a handful of levels at
/// most, and a bound here is what keeps every walk over the model shallow.
pub const MAX_TABLE_NESTING: usize = 32;

/// How many elements a list allocates room for when its first one is added: what the
/// standard library's growing list starts with for elements of this model's sizes.
const FIRST_ALLOCATION: usize = 4;

/// What opening one table level allocates by the time it is closed, beyond the blocks in
/// it: the first allocation of the list it is placed in, of its rows and of its first row's
/// cells, then the one row and one cell closing it may add.
const TABLE_BYTES: usize = FIRST_ALLOCATION
    * (size_of::<Block>() + size_of::<Row>() + size_of::<Cell>())
    + size_of::<Row>()
    + size_of::<Cell>();

/// What adding one element to `list` allocates, as the reading counts it: room for
/// [`FIRST_ALLOCATION`] elements for the first, then one element each, which the list's
/// doubling keeps within twice what it holds.
pub(crate) fn slot_bytes<T>(list: &[T]) -> usize {
    if list.is_empty() {
        FIRST_ALLOCATION * size_of::<T>()
    } else {
        size_of::<T>()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Story {
    /// Finished blocks at the story's own level.
    pub blocks: Vec<Block>,
    /// Open tables, the first at nesting level 1.
    tables: Vec<OpenTable>,
    /// The paragraph being read.
    pub inlines: Vec<Inline>,
    pub label: Option<String>,
    /// Whether anything was read into the paragraph since the last mark.
    pub has_content: bool,
}

#[derive(Debug, Default)]
struct OpenTable {
    rows: Vec<Row>,
    /// Finished cells of the row being read.
    cells: Vec<Cell>,
    /// Blocks of the cell being read.
    blocks: Vec<Block>,
}

impl OpenTable {
    fn finish(mut self) -> Table {
        if !self.blocks.is_empty() {
            self.cells.push(Cell {
                blocks: std::mem::take(&mut self.blocks),
            });
        }
        if !self.cells.is_empty() {
            self.rows.push(Row {
                cells: std::mem::take(&mut self.cells),
            });
        }
        Table { rows: self.rows }
    }
}

impl Story {
    /// End the paragraph being read and place it at table `level`. Returns the bytes of
    /// structure this allocated.
    pub fn end_paragraph(
        &mut self,
        props: ParagraphProps,
        end: ParagraphEnd,
        level: usize,
    ) -> usize {
        let paragraph = Paragraph {
            props,
            label: self.label.take(),
            inlines: std::mem::take(&mut self.inlines),
            end,
        };
        self.has_content = false;
        self.place(Block::Paragraph(paragraph), level)
    }

    fn place(&mut self, block: Block, level: usize) -> usize {
        self.close_tables_deeper_than(level);
        let opened = self.open_tables_to(level);
        let list = match level.checked_sub(1).and_then(|i| self.tables.get_mut(i)) {
            Some(table) => &mut table.blocks,
            None => &mut self.blocks,
        };
        let slot = slot_bytes(list);
        list.push(block);
        opened + slot
    }

    /// Open tables until `level` are open. Returns what the new ones will take once closed.
    fn open_tables_to(&mut self, level: usize) -> usize {
        let mut bytes = 0;
        while self.tables.len() < level {
            self.tables.push(OpenTable::default());
            bytes += TABLE_BYTES;
        }
        bytes
    }

    /// Close every table nested deeper than `level`, each into the cell it sits in. What
    /// this allocates was counted when the tables were opened.
    fn close_tables_deeper_than(&mut self, level: usize) {
        while self.tables.len() > level {
            let Some(table) = self.tables.pop() else {
                break;
            };
            let block = Block::Table(table.finish());
            match self.tables.last_mut() {
                Some(parent) => parent.blocks.push(block),
                None => self.blocks.push(block),
            }
        }
    }

    /// `\cell` at `level`: the cell being read there is finished. Returns the bytes of
    /// structure this allocated.
    pub fn close_cell(&mut self, level: usize) -> usize {
        if level == 0 {
            return 0;
        }
        self.close_tables_deeper_than(level);
        let mut bytes = self.open_tables_to(level);
        if let Some(table) = self.tables.get_mut(level - 1) {
            let blocks = std::mem::take(&mut table.blocks);
            bytes += slot_bytes(&table.cells);
            table.cells.push(Cell { blocks });
        }
        bytes
    }

    /// `\row` at `level`: the row being read there is finished. A row mark with no table
    /// open at that level closes nothing. Returns the bytes of structure this allocated.
    pub fn close_row(&mut self, level: usize) -> usize {
        if level == 0 || self.tables.len() < level {
            return 0;
        }
        self.close_tables_deeper_than(level);
        let mut bytes = 0;
        if let Some(table) = self.tables.get_mut(level - 1) {
            if !table.blocks.is_empty() {
                let blocks = std::mem::take(&mut table.blocks);
                bytes += slot_bytes(&table.cells);
                table.cells.push(Cell { blocks });
            }
            if !table.cells.is_empty() {
                let cells = std::mem::take(&mut table.cells);
                bytes += slot_bytes(&table.rows);
                table.rows.push(Row { cells });
            }
        }
        bytes
    }

    /// Finish the story: a paragraph read after the last mark is kept, open tables closed.
    ///
    /// The reading ends a story's last paragraph itself before finishing it, so that what
    /// the paragraph allocates is counted; this one is a safeguard.
    pub fn finish(mut self, props: ParagraphProps, level: usize) -> Vec<Block> {
        if self.has_content {
            self.end_paragraph(props, ParagraphEnd::Implicit, level);
        }
        self.close_tables_deeper_than(0);
        self.blocks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CharProps, Run};

    fn text(story: &mut Story, s: &str) {
        story.inlines.push(Inline::Text(Run {
            text: s.to_string(),
            props: CharProps::default(),
            group: 0,
            field: None,
        }));
        story.has_content = true;
    }

    fn texts(blocks: &[Block]) -> Vec<String> {
        crate::model::paragraphs(blocks)
            .map(|p| p.runs().map(|r| r.text.as_str()).collect())
            .collect()
    }

    #[test]
    fn a_table_is_rebuilt_from_its_marks() {
        let mut story = Story::default();
        let props = ParagraphProps::default();
        text(&mut story, "before");
        story.end_paragraph(props, ParagraphEnd::Paragraph, 0);
        for cell in ["a", "b"] {
            text(&mut story, cell);
            story.end_paragraph(props, ParagraphEnd::Cell, 1);
            story.close_cell(1);
        }
        story.close_row(1);
        text(&mut story, "after");
        let blocks = story.finish(props, 0);
        assert_eq!(blocks.len(), 3);
        let Block::Table(table) = &blocks[1] else {
            panic!("expected a table, got {:?}", blocks[1]);
        };
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0].cells.len(), 2);
        assert_eq!(texts(&blocks), ["before", "a", "b", "after"]);
    }

    #[test]
    fn a_nested_table_lands_in_the_cell_it_was_written_in() {
        let mut story = Story::default();
        let props = ParagraphProps::default();
        text(&mut story, "outer");
        story.end_paragraph(props, ParagraphEnd::Paragraph, 1);
        text(&mut story, "inner");
        story.end_paragraph(props, ParagraphEnd::Cell, 2);
        story.close_cell(2);
        story.close_row(2);
        story.end_paragraph(props, ParagraphEnd::Cell, 1);
        story.close_cell(1);
        story.close_row(1);
        let blocks = story.finish(props, 0);
        assert_eq!(blocks.len(), 1);
        let Block::Table(outer) = &blocks[0] else {
            panic!("expected a table");
        };
        let cell = &outer.rows[0].cells[0];
        assert_eq!(cell.blocks.len(), 3);
        assert!(matches!(cell.blocks[1], Block::Table(_)));
        assert_eq!(texts(&blocks), ["outer", "inner", ""]);
    }

    #[test]
    fn a_table_left_open_is_closed_with_its_last_cell() {
        let mut story = Story::default();
        let props = ParagraphProps::default();
        text(&mut story, "only");
        story.end_paragraph(props, ParagraphEnd::Paragraph, 1);
        let blocks = story.finish(props, 0);
        assert_eq!(texts(&blocks), ["only"]);
        assert!(matches!(blocks[0], Block::Table(_)));
    }
}
