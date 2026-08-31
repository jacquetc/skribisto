// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `<outlineItem>` XML, which appears in two places a decade apart.
//!
//! It is format 0's `outline.xml` — the whole manuscript, prose and all, in one
//! file — and it is also format 1's `revisions.xml`, which mirrors the entire
//! outline a second time so each row can carry its past states. Same element, same
//! attributes, same nesting, so one reader serves both. That is what makes reading
//! ten-year-old projects cheap rather than a second codebase.
//!
//! Attributes are the `Outline` enum's member names, so a row reads the same here
//! as it does in a `.md` header, minus the computed fields the writer excludes.
//! Prose lives in a `text` **attribute**, which is why a real project's
//! `revisions.xml` outgrows everything else it ships beside.
//!
//! A `<revision>` carries a unix `timestamp` and the whole body at that moment —
//! snapshots, not diffs. Manuskript strips characters outside XML 1.0's legal set
//! on the way in, so a revision is very slightly lossy against the `.md` beside
//! it; where both exist, the `.md` is the truth and this is the history, which is
//! the precedence Manuskript itself applies.

use anyhow::Result;

use crate::model::{OutlineItem, OutlineKind, Revision};
use crate::xml::{self, attr, attr_opt};

/// One parsed `<outlineItem>` document.
pub struct OutlineXml {
    /// The root's children — the real top-level rows. The root element itself is
    /// Manuskript's synthetic "Root" holder and is never a row.
    pub items: Vec<OutlineItem>,
    /// Every revision found, flat, each naming the row it belongs to.
    pub revisions: Vec<Revision>,
    /// Anything the conversion had to report.
    pub notices: Vec<String>,
}

/// Parse an `<outlineItem>` tree.
pub fn parse(text: &str) -> Result<OutlineXml> {
    let doc = xml::parse(text)?;
    let root = doc.root_element();
    let mut revisions = Vec::new();
    let mut notices = Vec::new();
    let items = children_of(root, &mut revisions, &mut notices);
    Ok(OutlineXml {
        items,
        revisions,
        notices,
    })
}

fn children_of(
    node: roxmltree::Node,
    revisions: &mut Vec<Revision>,
    notices: &mut Vec<String>,
) -> Vec<OutlineItem> {
    node.children()
        .filter(|n| n.is_element() && n.tag_name().name() == "outlineItem")
        .map(|n| item_from(n, revisions, notices))
        .collect()
}

fn item_from(
    node: roxmltree::Node,
    revisions: &mut Vec<Revision>,
    notices: &mut Vec<String>,
) -> OutlineItem {
    let id = attr_opt(&node, "ID");
    let title = attr(&node, "title");
    // A pre-0.3.0 project stores prose in a `text` attribute and names its markup
    // in `type`. Both the body and every revision of it are that markup, so both
    // go through the one conversion boundary.
    let declared = node.attribute("type").unwrap_or_default();
    if let Some(id) = id.as_deref() {
        collect_revisions(node, id, declared, &title, revisions, notices);
    }
    let converted = crate::prose::to_djot(declared, &attr(&node, "text"), &title);
    if let Some(notice) = converted.notice {
        notices.push(notice);
    }
    OutlineItem {
        id: id.clone(),
        title,
        // `type` is the only signal here — unlike the folder layout, an XML tree
        // has no directory to read it from.
        kind: match node.attribute("type") {
            Some("folder") => OutlineKind::Folder,
            _ => OutlineKind::Text,
        },
        summary_sentence: node
            .attribute("summarySentence")
            .or_else(|| node.attribute("summarySentance"))
            .unwrap_or_default()
            .to_string(),
        summary_full: attr(&node, "summaryFull"),
        pov: attr_opt(&node, "POV"),
        notes: attr(&node, "notes"),
        label: node.attribute("label").and_then(parse_index),
        status: node.attribute("status").and_then(parse_index),
        compile: node
            .attribute("compile")
            .and_then(|v| v.trim().parse::<i64>().ok()),
        set_goal: node
            .attribute("setGoal")
            .and_then(|v| v.trim().parse::<i64>().ok())
            .filter(|n| *n > 0),
        custom_icon: attr(&node, "customIcon"),
        text: converted.djot,
        children: children_of(node, revisions, notices),
    }
}

/// Gather one row's `<revision>` children.
///
/// A revision whose row has no id is skipped: it cannot be attached to anything,
/// and there is nowhere to put it. Manuskript raises instead, which is why
/// "delete revisions.xml" is the standing answer to half its loading bugs.
fn collect_revisions(
    node: roxmltree::Node,
    item_id: &str,
    declared: &str,
    title: &str,
    out: &mut Vec<Revision>,
    notices: &mut Vec<String>,
) {
    for rev in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "revision")
    {
        let Some(timestamp) = rev
            .attribute("timestamp")
            .and_then(|t| t.trim().parse::<i64>().ok())
        else {
            continue;
        };
        let converted = crate::prose::to_djot(declared, &attr(&rev, "text"), title);
        if let Some(notice) = converted.notice {
            notices.push(notice);
        }
        out.push(Revision {
            item_id: item_id.to_string(),
            timestamp,
            text: converted.djot,
        });
    }
}

/// The same 1-based vocabulary index the folder reader uses, with `0` and absent
/// both meaning "none".
fn parse_index(raw: &str) -> Option<usize> {
    raw.trim().parse::<usize>().ok().filter(|n| *n > 0)
}

#[cfg(test)]
mod tests {

    /// The 2016 case, and the one that failed silently: a `type="html"` body in
    /// the `text` attribute handed straight to a Markdown parser comes back
    /// empty, and the scene is gone with no warning.
    #[test]
    fn a_format_zero_html_body_keeps_its_words() {
        let xml = r#"<outlineItem title="Root" type="folder">
            <outlineItem title="Old" ID="1" type="html"
                         text="&lt;p&gt;A &lt;b&gt;strong&lt;/b&gt; word.&lt;/p&gt;"/>
        </outlineItem>"#;
        let parsed = parse(xml).expect("parse");
        let body = &parsed.items[0].text;
        assert!(!body.trim().is_empty(), "the scene must not vanish");
        assert!(body.contains("strong"), "{body}");
        assert!(parsed.notices.is_empty(), "{:?}", parsed.notices);
    }

    /// A revision carries the same markup as the row it belongs to, so it goes
    /// through the same conversion.
    #[test]
    fn a_revision_of_an_html_row_is_converted_too() {
        let xml = r#"<outlineItem title="Root" type="folder">
            <outlineItem title="Old" ID="1" type="html" text="&lt;p&gt;now&lt;/p&gt;">
              <revision timestamp="1" text="&lt;p&gt;&lt;b&gt;then&lt;/b&gt;&lt;/p&gt;"/>
            </outlineItem>
        </outlineItem>"#;
        let parsed = parse(xml).expect("parse");
        assert_eq!(parsed.revisions.len(), 1);
        let text = &parsed.revisions[0].text;
        assert!(text.contains("then"), "{text}");
        assert!(
            !text.contains("<b>"),
            "the markup was converted, not carried: {text}"
        );
    }

    use super::*;

    const SAMPLE: &str = r#"<?xml version='1.0' encoding='UTF-8'?>
<outlineItem title="Root" ID="0" type="folder" compile="2" lastPath="">
  <outlineItem title="Jerusalem" ID="35" type="folder" POV="0" compile="2" lastPath="outline/0-Jerusalem">
    <outlineItem title="Chapter 1" ID="5" type="folder" compile="2" setGoal="1000">
      <outlineItem title="Introduction" ID="1" type="md" compile="2" label="3" status="4" text="First words.">
        <revision timestamp="1455033267" text="Older words."/>
        <revision timestamp="1455033999" text="Newer words."/>
      </outlineItem>
    </outlineItem>
  </outlineItem>
</outlineItem>"#;

    #[test]
    fn the_synthetic_root_is_not_a_row_and_its_children_are() {
        let parsed = parse(SAMPLE).expect("parse");
        assert_eq!(parsed.items.len(), 1);
        assert_eq!(parsed.items[0].title, "Jerusalem");
        assert!(parsed.items[0].is_folder());
        assert_eq!(parsed.items[0].pov.as_deref(), Some("0"));
    }

    #[test]
    fn a_row_carries_its_prose_in_an_attribute_and_its_vocabulary_indices() {
        let parsed = parse(SAMPLE).expect("parse");
        let scene = &parsed.items[0].children[0].children[0];
        assert_eq!(scene.title, "Introduction");
        assert_eq!(scene.kind, OutlineKind::Text);
        assert_eq!(scene.text, "First words.");
        assert_eq!(scene.label, Some(3));
        assert_eq!(scene.status, Some(4));
        assert_eq!(parsed.items[0].children[0].set_goal, Some(1000));
    }

    #[test]
    fn revisions_come_out_flat_and_named_by_their_row() {
        let parsed = parse(SAMPLE).expect("parse");
        assert_eq!(parsed.revisions.len(), 2);
        assert!(parsed.revisions.iter().all(|r| r.item_id == "1"));
        assert_eq!(parsed.revisions[0].timestamp, 1_455_033_267);
        assert_eq!(parsed.revisions[1].text, "Newer words.");
    }

    /// Manuskript aborts the load here. We drop the one revision we cannot place
    /// and keep the row.
    #[test]
    fn a_revision_on_an_idless_row_is_dropped_rather_than_fatal() {
        let xml = r#"<outlineItem title="Root" type="folder">
            <outlineItem title="Nameless" type="md"><revision timestamp="1" text="x"/></outlineItem>
        </outlineItem>"#;
        let parsed = parse(xml).expect("parse");
        assert_eq!(parsed.items.len(), 1);
        assert!(parsed.items[0].id.is_none());
        assert!(parsed.revisions.is_empty());
    }

    #[test]
    fn a_revision_without_a_timestamp_is_skipped() {
        let xml = r#"<outlineItem title="Root" type="folder">
            <outlineItem title="A" ID="7" type="md">
              <revision text="no when"/>
              <revision timestamp="99" text="yes"/>
            </outlineItem>
        </outlineItem>"#;
        let parsed = parse(xml).expect("parse");
        assert_eq!(parsed.revisions.len(), 1);
        assert_eq!(parsed.revisions[0].timestamp, 99);
    }

    /// The 2016 misspelling, which is what a real format-0 project carries.
    #[test]
    fn the_legacy_summary_spelling_is_read() {
        let xml = r#"<outlineItem title="Root" type="folder">
            <outlineItem title="A" ID="1" type="txt" summarySentance="The old spelling."/>
        </outlineItem>"#;
        let parsed = parse(xml).expect("parse");
        assert_eq!(parsed.items[0].summary_sentence, "The old spelling.");
        // A pre-0.3.0 type is not a folder, whatever it is called.
        assert_eq!(parsed.items[0].kind, OutlineKind::Text);
    }
}
