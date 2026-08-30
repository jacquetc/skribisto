// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The generic `<model>` dump that format 0 used for everything except the
//! outline.
//!
//! It is a `QStandardItemModel` written out verbatim, so a field's identity is its
//! **column number** and nothing else. `<header>` names the columns in some files
//! and not others — `perso.xml` names all thirteen, `world.xml` names none — so the
//! numbers are what the readers key on, and the header is ignored.
//!
//! ```xml
//! <model version="0.1.1">
//!   <header>…</header>
//!   <data>
//!     <row row="0">
//!       <col col="0" color="#aarrggbb">Places<row row="0">…</row></col>
//!       <col col="1">0</col>
//!     </row>
//!   </data>
//! </model>
//! ```
//!
//! Two things about that shape matter. A cell's **children hang inside the `<col>`
//! element**, after its text, which is how a tree is stored in a flat table — the
//! world tree, a plot's characters, a plot's steps and a character's own fields all
//! arrive that way. And a colour is an attribute on the cell, in `#aarrggbb` with
//! the alpha first, which is one digit-pair longer than every colour format 1
//! writes.

use anyhow::Result;

use crate::xml;

/// One cell of a row.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub text: String,
    /// `#aarrggbb`, as format 0 wrote it. Normalising is the caller's job, since
    /// only some callers have anywhere to put a colour.
    pub color: Option<String>,
    /// Sub-rows nested inside this cell.
    pub children: Vec<Row>,
}

/// One row: its cells, by column number.
#[derive(Debug, Clone, Default)]
pub struct Row {
    cells: Vec<(usize, Cell)>,
}

impl Row {
    /// The text in `col`, empty when the cell is absent or empty.
    pub fn text(&self, col: usize) -> &str {
        self.cell(col).map(|c| c.text.as_str()).unwrap_or_default()
    }

    /// The text in `col`, or `None` when it is absent or empty.
    pub fn text_opt(&self, col: usize) -> Option<String> {
        Some(self.text(col).trim())
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    }

    /// The first colour on any cell of this row.
    ///
    /// Any cell, because that is where Manuskript's own character reader looks: it
    /// walks every column and takes whichever carries one. In practice only column
    /// zero ever does.
    pub fn color(&self) -> Option<&str> {
        self.cells.iter().find_map(|(_, c)| c.color.as_deref())
    }

    /// The sub-rows nested in `col`.
    pub fn children(&self, col: usize) -> &[Row] {
        self.cell(col).map(|c| c.children.as_slice()).unwrap_or(&[])
    }

    /// Every sub-row of this row, whichever cell holds it.
    pub fn all_children(&self) -> Vec<&Row> {
        self.cells
            .iter()
            .flat_map(|(_, c)| c.children.iter())
            .collect()
    }

    fn cell(&self, col: usize) -> Option<&Cell> {
        self.cells
            .iter()
            .find(|(index, _)| *index == col)
            .map(|(_, c)| c)
    }
}

/// Parse a `<model>` document into its data rows, in document order.
pub fn parse(text: &str) -> Result<Vec<Row>> {
    let doc = xml::parse(text)?;
    let root = doc.root_element();
    let Some(data) = root
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "data")
    else {
        // A model with a header and no data is an empty table, not a broken file.
        return Ok(Vec::new());
    };
    Ok(rows_of(data))
}

fn rows_of(node: roxmltree::Node) -> Vec<Row> {
    node.children()
        .filter(|n| n.is_element() && n.tag_name().name() == "row")
        .map(row_of)
        .collect()
}

fn row_of(node: roxmltree::Node) -> Row {
    let mut cells = Vec::new();
    for col in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "col")
    {
        let Some(index) = col
            .attribute("col")
            .and_then(|v| v.trim().parse::<usize>().ok())
        else {
            continue;
        };
        cells.push((
            index,
            Cell {
                // Only the cell's own text, never a descendant's: a nested row's
                // text belongs to that row.
                text: col
                    .children()
                    .filter(roxmltree::Node::is_text)
                    .filter_map(|n| n.text())
                    .collect::<String>(),
                color: col
                    .attribute("color")
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string),
                children: rows_of(col),
            },
        ));
    }
    Row { cells }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORLD: &str = r##"<?xml version='1.0' encoding='UTF-8'?>
<model version="0.1.1">
  <header><vertical><label row="0" text="1"/></vertical></header>
  <data>
    <row row="0">
      <col col="0" color="#ffff0000">Places<row row="0"><col col="0">Jerusalem</col><col col="1">1</col></row><row row="1"><col col="0">Judea</col><col col="1">2</col></row></col>
      <col col="1">0</col>
      <col col="2"/>
    </row>
  </data>
</model>"##;

    #[test]
    fn a_cells_text_stops_at_its_children() {
        let rows = parse(WORLD).expect("parse");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].text(0), "Places");
        assert_eq!(rows[0].text(1), "0");
        assert_eq!(rows[0].text(2), "");
    }

    #[test]
    fn children_hang_inside_the_cell_that_owns_them() {
        let rows = parse(WORLD).expect("parse");
        let kids = rows[0].children(0);
        assert_eq!(kids.len(), 2);
        assert_eq!(kids[0].text(0), "Jerusalem");
        assert_eq!(kids[0].text(1), "1");
        assert_eq!(kids[1].text(0), "Judea");
        // A column with no children has none, rather than borrowing its sibling's.
        assert!(rows[0].children(1).is_empty());
        assert_eq!(rows[0].all_children().len(), 2);
    }

    #[test]
    fn a_colour_is_found_on_whichever_cell_carries_it() {
        let rows = parse(WORLD).expect("parse");
        assert_eq!(rows[0].color(), Some("#ffff0000"));
    }

    #[test]
    fn an_absent_cell_reads_as_empty_rather_than_failing() {
        let rows = parse(WORLD).expect("parse");
        assert_eq!(rows[0].text(99), "");
        assert!(rows[0].text_opt(99).is_none());
        assert!(
            rows[0].text_opt(2).is_none(),
            "an empty cell is not a value"
        );
        assert_eq!(rows[0].text_opt(0).as_deref(), Some("Places"));
    }

    #[test]
    fn a_model_with_no_data_section_is_an_empty_table() {
        let rows = parse(r#"<model version="0.1.1"><header/></model>"#).expect("parse");
        assert!(rows.is_empty());
    }

    #[test]
    fn a_malformed_document_is_an_error_the_caller_can_report() {
        assert!(parse("<model><data>").is_err());
    }
}
