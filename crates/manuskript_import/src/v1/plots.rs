// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading `plots.xml`.
//!
//! One `<plot>` per plot, with a `<step>` child per resolution beat. Two of the
//! attributes are not what the enum suggests:
//!
//! - `characters` is a **comma-joined list of character ids**, not names. The
//!   writer replaces the cell's contents with that list, and drops the attribute
//!   entirely when a plot names nobody — which it must, because a fresh plot's
//!   cell holds the literal placeholder string "Characters".
//! - `steps` never appears as an attribute: the writer drops it and emits the
//!   beats as child elements instead.
//!
//! Both attributes were called something else before 0.3.0 (`persos` and
//! `subplots`, renamed in March 2016). Format-0 projects store plots through the
//! generic model dump rather than this file, so the old names should never reach
//! here, but they are accepted anyway: the cost is one `or_else` and the
//! alternative is losing a project's whole plot list to a rename.
//!
//! A plot links to characters and to nothing else. There is no plot-to-scene link
//! in the format at all: the only one that exists is the `{P:id:…}` reference a
//! writer drops into a scene's notes by hand.

use crate::model::{Plot, PlotStep};
use crate::xml::{self, attr, attr_opt, importance};

/// The member holding the plots.
pub const PLOTS_MEMBER: &str = "plots.xml";

/// Read every plot. A malformed file costs the plots and nothing else.
pub fn read(text: &str, notices: &mut Vec<String>) -> Vec<Plot> {
    let doc = match xml::parse(text) {
        Ok(doc) => doc,
        Err(e) => {
            notices.push(format!(
                "'{PLOTS_MEMBER}' could not be read ({e}); the plots were not imported."
            ));
            return Vec::new();
        }
    };
    doc.root_element()
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "plot")
        .map(|n| Plot {
            id: attr_opt(&n, "ID"),
            name: attr(&n, "name"),
            importance: importance(n.attribute("importance")),
            characters: character_ids(&n),
            description: attr(&n, "description"),
            result: attr(&n, "result"),
            summary: attr(&n, "summary"),
            steps: n
                .children()
                .filter(|s| s.is_element() && s.tag_name().name() == "step")
                .map(|s| PlotStep {
                    id: attr_opt(&s, "ID"),
                    name: attr(&s, "name"),
                    meta: attr(&s, "meta"),
                    summary: attr(&s, "summary"),
                })
                .collect(),
        })
        .collect()
}

/// Split the comma-joined id list, dropping blanks and repeats.
fn character_ids(node: &roxmltree::Node) -> Vec<String> {
    let raw = node
        .attribute("characters")
        .or_else(|| node.attribute("persos"))
        .unwrap_or_default();
    let mut ids: Vec<String> = Vec::new();
    for part in raw.split(',') {
        let id = part.trim();
        // A plot that names nobody can still carry the placeholder the model seeds
        // the cell with, if it was written by a version that failed to drop it.
        if id.is_empty() || id == "Characters" || ids.iter().any(|k| k == id) {
            continue;
        }
        ids.push(id.to_string());
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version='1.0' encoding='UTF-8'?>
<root>
  <plot name="The good news spreads" ID="0" importance="2"/>
  <plot name="Peter broadens" ID="1" importance="1" characters="0"/>
  <plot name="Paul and Barnabas fight" ID="2" importance="0" characters="1,5"
        description="They disagree." result="They part." summary="A rift.">
    <step name="The argument" ID="10" meta="midpoint" summary="It boils over."/>
    <step name="The parting" ID="11" meta="" summary=""/>
  </plot>
</root>"#;

    #[test]
    fn every_plot_is_read_with_its_scale_and_its_prose() {
        let mut notices = Vec::new();
        let plots = read(SAMPLE, &mut notices);
        assert!(notices.is_empty(), "{notices:?}");
        assert_eq!(plots.len(), 3);
        assert_eq!(plots[0].name, "The good news spreads");
        assert_eq!(plots[0].importance, Some(2));
        assert_eq!(plots[2].description, "They disagree.");
        assert_eq!(plots[2].result, "They part.");
        assert_eq!(plots[2].summary, "A rift.");
    }

    #[test]
    fn a_plot_names_its_characters_by_id() {
        let mut notices = Vec::new();
        let plots = read(SAMPLE, &mut notices);
        assert!(plots[0].characters.is_empty());
        assert_eq!(plots[1].characters, ["0"]);
        assert_eq!(plots[2].characters, ["1", "5"]);
    }

    #[test]
    fn steps_are_children_and_keep_their_order() {
        let mut notices = Vec::new();
        let plots = read(SAMPLE, &mut notices);
        assert_eq!(plots[2].steps.len(), 2);
        assert_eq!(plots[2].steps[0].name, "The argument");
        assert_eq!(plots[2].steps[0].meta, "midpoint");
        assert_eq!(plots[2].steps[1].name, "The parting");
        assert!(plots[0].steps.is_empty());
    }

    /// The model seeds the cell with a placeholder that the writer is supposed to
    /// drop; a version that failed to would otherwise import a character named
    /// "Characters".
    #[test]
    fn placeholders_blanks_and_repeats_are_not_character_ids() {
        let mut notices = Vec::new();
        let plots = read(
            r#"<root><plot name="A" characters="Characters"/>
                     <plot name="B" characters=" 3 , ,3, 4 "/></root>"#,
            &mut notices,
        );
        assert!(plots[0].characters.is_empty());
        assert_eq!(plots[1].characters, ["3", "4"]);
    }

    /// The pre-0.3.0 attribute name. Format-0 projects store plots elsewhere, but
    /// accepting it costs one fallback and losing a whole plot list costs a book.
    #[test]
    fn the_legacy_characters_attribute_is_accepted() {
        let mut notices = Vec::new();
        let plots = read(
            r#"<root><plot name="A" persos="2,7"/></root>"#,
            &mut notices,
        );
        assert_eq!(plots[0].characters, ["2", "7"]);
    }

    #[test]
    fn a_malformed_file_costs_the_plots_and_says_so() {
        let mut notices = Vec::new();
        let plots = read("<root><plot name=", &mut notices);
        assert!(plots.is_empty());
        assert!(
            notices.iter().any(|n| n.contains(PLOTS_MEMBER)),
            "{notices:?}"
        );
    }

    #[test]
    fn an_importance_outside_the_scale_is_no_importance() {
        let mut notices = Vec::new();
        let plots = read(
            r#"<root><plot name="A" importance="9"/><plot name="B" importance="x"/></root>"#,
            &mut notices,
        );
        assert!(plots[0].importance.is_none());
        assert!(plots[1].importance.is_none());
    }
}
