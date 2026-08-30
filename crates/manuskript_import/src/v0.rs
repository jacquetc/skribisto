// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a format-0 project: Manuskript 0.1.0 and 0.2.0, February 2016.
//!
//! One zip of XML. The outline is the same `<outlineItem>` tree format 1 still
//! writes into `revisions.xml`, so [`crate::outline_xml`] reads it unchanged and
//! this module only has to deal with the five generic `<model>` dumps beside it.
//! That sharing is the whole reason a ten-year-old project is worth supporting: it
//! costs one column map per file, not a second importer.
//!
//! Nothing normalises these projects on the way out of Manuskript, because format
//! 0 is effectively **read-only** there: its own writer refuses to save characters
//! (`"File format 0 does not save characters!"`, with the call commented out), so
//! opening such a project and saving converts it to format 1. A project still in
//! format 0 has not been opened in Manuskript since 2016 — which is exactly the
//! writer most likely to be looking for a way out of it.
//!
//! # What changed at 0.3.0, and what this does about it
//!
//! - `summarySentance` was **misspelled**, on outline items and on characters
//!   alike. Manuskript's own upgrade matches attribute names against the current
//!   enum, so its reading of a format-0 project silently drops every one-line
//!   summary in the book. Both spellings are read here.
//! - A plot's characters were `persos` and its beats were `subplots`, and both
//!   were stored as sub-rows rather than as an attribute.
//! - Item types `txt`, `t2t` and `html` still exist. They are coerced to Markdown,
//!   and an HTML body keeps its shape rather than being flattened.
//! - The label and status tables include Manuskript's "none" row, which format 1
//!   drops from the file and re-adds on load. It is dropped here so both
//!   generations index identically.
//!
//! `settings.pickle` is never deserialized; see [`crate::source`].

use crate::model::{
    Character, Info, Label, Plot, PlotStep, Project, Summary, WorldItem, normalise_color,
};
use crate::model_xml::{self, Row};
use crate::outline_xml;
use crate::source::ManuskriptSource;

const OUTLINE_MEMBER: &str = "outline.xml";
const CHARACTERS_MEMBER: &str = "perso.xml";
const WORLD_MEMBER: &str = "world.xml";
const PLOTS_MEMBER: &str = "plots.xml";
const LABELS_MEMBER: &str = "labels.xml";
const STATUS_MEMBER: &str = "status.xml";
const FLAT_MEMBER: &str = "flatModel.xml";

/// Read a whole format-0 project.
pub fn read(src: &ManuskriptSource) -> Project {
    let mut notices = src.notices.clone();
    notices.push(
        "This project is in Manuskript's original 2016 format. It was read in full; nothing in \
         it was changed."
            .to_string(),
    );

    let (outline, revisions) = read_outline(src, &mut notices);
    let (info, summary) = read_flat(src, &mut notices);

    Project {
        source_name: src.project_name.clone(),
        info,
        summary,
        labels: read_vocabulary(src, LABELS_MEMBER, &mut notices)
            .into_iter()
            .map(|(name, color)| Label { name, color })
            .collect(),
        statuses: read_vocabulary(src, STATUS_MEMBER, &mut notices)
            .into_iter()
            .map(|(name, _)| name)
            .collect(),
        outline,
        characters: read_characters(src, &mut notices),
        world: read_world(src, &mut notices),
        plots: read_plots(src, &mut notices),
        // Format 0 kept its settings in the pickle, which is never opened, so the
        // project's language is not recoverable. Left unset rather than guessed.
        settings: Default::default(),
        revisions,
        notices,
    }
}

/// Read a member as a `<model>` table, reporting rather than failing.
fn rows(src: &ManuskriptSource, member: &str, notices: &mut Vec<String>) -> Vec<Row> {
    let Some(text) = src.text(member) else {
        return Vec::new();
    };
    match model_xml::parse(&text) {
        Ok(rows) => rows,
        Err(e) => {
            notices.push(format!(
                "'{member}' could not be read ({e}); what it held was not imported."
            ));
            Vec::new()
        }
    }
}

fn read_outline(
    src: &ManuskriptSource,
    notices: &mut Vec<String>,
) -> (Vec<crate::model::OutlineItem>, Vec<crate::model::Revision>) {
    let Some(text) = src.text(OUTLINE_MEMBER) else {
        notices.push(format!(
            "'{OUTLINE_MEMBER}' is missing, so this project has no manuscript to import."
        ));
        return (Vec::new(), Vec::new());
    };
    match outline_xml::parse(&text) {
        Ok(parsed) => {
            notices.extend(parsed.notices);
            (parsed.items, parsed.revisions)
        }
        Err(e) => {
            notices.push(format!(
                "'{OUTLINE_MEMBER}' could not be read ({e}); no manuscript was imported."
            ));
            (Vec::new(), Vec::new())
        }
    }
}

/// Read `labels.xml` or `status.xml`, dropping Manuskript's "none" row.
///
/// Format 0 stores that row; format 1 leaves it out of the file and re-adds it on
/// load. Dropping it here is what makes an item's stored index mean the same thing
/// in both generations, so the mapper never has to ask which it is reading.
fn read_vocabulary(
    src: &ManuskriptSource,
    member: &str,
    notices: &mut Vec<String>,
) -> Vec<(String, Option<String>)> {
    let table = rows(src, member, notices);
    let Some((first, rest)) = table.split_first() else {
        return Vec::new();
    };
    if !first.text(0).trim().is_empty() {
        notices.push(format!(
            "'{member}' opens with a named row where Manuskript keeps its empty \"none\" entry \
             ('{}'). It was left out, so the rows below it keep the numbers the manuscript \
             cites.",
            first.text(0).trim()
        ));
    }
    rest.iter()
        .map(|r| {
            (
                r.text(0).trim().to_string(),
                r.color().and_then(normalise_color),
            )
        })
        .collect()
}

/// `flatModel.xml`: row 0 is the project's details, row 1 the summary ladder.
fn read_flat(src: &ManuskriptSource, notices: &mut Vec<String>) -> (Info, Summary) {
    let table = rows(src, FLAT_MEMBER, notices);
    let info = table
        .first()
        .map(|r| Info {
            title: r.text(0).to_string(),
            subtitle: r.text(1).to_string(),
            serie: r.text(2).to_string(),
            volume: r.text(3).to_string(),
            genre: r.text(4).to_string(),
            license: r.text(5).to_string(),
            author: r.text(6).to_string(),
            email: r.text(7).to_string(),
        })
        .unwrap_or_default();
    let summary = table
        .get(1)
        .map(|r| Summary {
            situation: r.text(0).to_string(),
            sentence: r.text(1).to_string(),
            paragraph: r.text(2).to_string(),
            page: r.text(3).to_string(),
            full: r.text(4).to_string(),
        })
        .unwrap_or_default();
    (info, summary)
}

/// `perso.xml`. Columns 0..=10 hold the same fields, in the same order, as the
/// modern `Character` enum — which is why Manuskript's own upgrade can copy them
/// across by index. Columns 11 and 12 belong to the **sub-rows**, and hold the
/// name and value of one field the writer added.
fn read_characters(src: &ManuskriptSource, notices: &mut Vec<String>) -> Vec<Character> {
    rows(src, CHARACTERS_MEMBER, notices)
        .iter()
        .map(|r| Character {
            name: r.text(0).to_string(),
            id: r.text_opt(1),
            importance: crate::xml::importance(Some(r.text(2))),
            // Whether a character may hold the camera is a 0.12.0 field. A project
            // this old has no answer, and `false` would be an invented one.
            pov_enabled: None,
            color: r.color().and_then(normalise_color).unwrap_or_default(),
            motivation: r.text(3).to_string(),
            goal: r.text(4).to_string(),
            conflict: r.text(5).to_string(),
            epiphany: r.text(6).to_string(),
            summary_sentence: r.text(7).to_string(),
            summary_paragraph: r.text(8).to_string(),
            summary_full: r.text(9).to_string(),
            notes: r.text(10).to_string(),
            infos: r
                .all_children()
                .iter()
                .filter_map(|child| {
                    let key = child.text_opt(11)?;
                    Some((key, child.text(12).to_string()))
                })
                .collect(),
        })
        .collect()
}

/// `world.xml`: name, ID, description, passion, conflict, with children nested in
/// the first cell.
fn read_world(src: &ManuskriptSource, notices: &mut Vec<String>) -> Vec<WorldItem> {
    fn convert(rows: &[Row]) -> Vec<WorldItem> {
        rows.iter()
            .map(|r| WorldItem {
                name: r.text(0).to_string(),
                id: r.text_opt(1),
                description: r.text(2).to_string(),
                passion: r.text(3).to_string(),
                conflict: r.text(4).to_string(),
                children: convert(r.children(0)),
            })
            .collect()
    }
    convert(&rows(src, WORLD_MEMBER, notices))
}

/// `plots.xml`, the format-0 shape — a different file from format 1's `plots.xml`
/// despite the identical name.
///
/// Columns are the `Plot` enum's order: name, ID, importance, characters,
/// description, result, steps, summary. The characters cell and the steps cell
/// each hold their contents as **sub-rows**, and each carries a literal
/// placeholder string ("Persos", "Subplots") that the model seeds it with, so the
/// cell's own text is never data.
fn read_plots(src: &ManuskriptSource, notices: &mut Vec<String>) -> Vec<Plot> {
    const CHARACTERS_COL: usize = 3;
    const STEPS_COL: usize = 6;
    rows(src, PLOTS_MEMBER, notices)
        .iter()
        .map(|r| Plot {
            name: r.text(0).to_string(),
            id: r.text_opt(1),
            importance: crate::xml::importance(Some(r.text(2))),
            characters: r
                .children(CHARACTERS_COL)
                .iter()
                .filter_map(|c| c.text_opt(0))
                .collect(),
            description: r.text(4).to_string(),
            result: r.text(5).to_string(),
            summary: r.text(7).to_string(),
            steps: r
                .children(STEPS_COL)
                .iter()
                .map(|s| PlotStep {
                    name: s.text(0).to_string(),
                    id: s.text_opt(1),
                    meta: s.text(2).to_string(),
                    summary: s.text(3).to_string(),
                })
                .collect(),
        })
        .collect()
}
