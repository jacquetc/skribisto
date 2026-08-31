// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading `world.opml` — the worldbuilding tree.
//!
//! OPML 1.0, one `<outline>` element per node, nested. The attributes are the
//! `World` enum's member names: `name`, `ID`, `description`, `passion`,
//! `conflict`. Empty values are omitted by the writer.
//!
//! `passion` and `conflict` are Manuskript's two prompts for a world entry — what
//! a reader might love about it, and what trouble it might cause — not free
//! metadata.
//!
//! This is the one model in the format that has **never changed**: one commit
//! touches the enum since 2015, and it only moved the file.

use crate::model::WorldItem;
use crate::xml::{self, attr, attr_opt};

/// The member holding the tree.
pub const WORLD_MEMBER: &str = "world.opml";

/// Read the tree. A malformed file costs the world tree and nothing else.
pub fn read(text: &str, notices: &mut Vec<String>) -> Vec<WorldItem> {
    let doc = match xml::parse(text) {
        Ok(doc) => doc,
        Err(e) => {
            notices.push(format!(
                "'{WORLD_MEMBER}' could not be read ({e}); the world tree was not imported."
            ));
            return Vec::new();
        }
    };
    let root = doc.root_element();
    if root.tag_name().name() != "opml" {
        notices.push(format!(
            "'{WORLD_MEMBER}' is not an OPML document; the world tree was not imported."
        ));
        return Vec::new();
    }
    // The nodes hang under <body>; anything else at the top is OPML's own <head>.
    root.children()
        .filter(|n| n.is_element() && n.tag_name().name() == "body")
        .flat_map(|body| children_of(body))
        .collect()
}

fn children_of(node: roxmltree::Node) -> Vec<WorldItem> {
    node.children()
        .filter(|n| n.is_element() && n.tag_name().name() == "outline")
        .map(|n| WorldItem {
            id: attr_opt(&n, "ID"),
            name: attr(&n, "name"),
            description: attr(&n, "description"),
            passion: attr(&n, "passion"),
            conflict: attr(&n, "conflict"),
            children: children_of(n),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version='1.0' encoding='UTF-8'?>
<opml version="1.0">
  <body>
    <outline name="Places" ID="0">
      <outline name="Jerusalem" ID="1" description="The city." passion="Its walls."
               conflict="Everyone wants it."/>
      <outline name="Judea" ID="2"/>
    </outline>
    <outline name="Cultures" ID="6"/>
  </body>
</opml>"#;

    #[test]
    fn the_tree_nests_and_keeps_its_order() {
        let mut notices = Vec::new();
        let world = read(SAMPLE, &mut notices);
        assert!(notices.is_empty(), "{notices:?}");
        assert_eq!(world.len(), 2);
        assert_eq!(world[0].name, "Places");
        assert_eq!(world[1].name, "Cultures");
        assert_eq!(world[0].children.len(), 2);
        assert_eq!(world[0].children[1].name, "Judea");
    }

    #[test]
    fn the_two_prompts_are_read_as_their_own_fields() {
        let mut notices = Vec::new();
        let world = read(SAMPLE, &mut notices);
        let jerusalem = &world[0].children[0];
        assert_eq!(jerusalem.description, "The city.");
        assert_eq!(jerusalem.passion, "Its walls.");
        assert_eq!(jerusalem.conflict, "Everyone wants it.");
        assert_eq!(jerusalem.id.as_deref(), Some("1"));
        // An omitted attribute is an empty field, not a missing one.
        assert_eq!(world[0].children[1].description, "");
    }

    #[test]
    fn a_document_that_is_not_opml_is_refused_by_name() {
        let mut notices = Vec::new();
        let world = read("<root><outline name=\"X\"/></root>", &mut notices);
        assert!(world.is_empty());
        assert!(notices.iter().any(|n| n.contains("OPML")), "{notices:?}");
    }

    #[test]
    fn a_malformed_file_costs_the_world_and_says_so() {
        let mut notices = Vec::new();
        assert!(read("<opml><body>", &mut notices).is_empty());
        assert!(
            notices.iter().any(|n| n.contains(WORLD_MEMBER)),
            "{notices:?}"
        );
    }

    #[test]
    fn an_empty_world_is_not_a_problem() {
        let mut notices = Vec::new();
        assert!(read(r#"<opml version="1.0"><body/></opml>"#, &mut notices).is_empty());
        assert!(notices.is_empty());
    }
}
