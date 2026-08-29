// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Parse the `tree` member into a version-neutral [`PlumeTree`].
//!
//! Lenient by design (it normalizes every Plume tree version, 0.2–0.5, in one
//! pass — mirroring `FileUpdater::updateTreeFile`):
//!  - root tag is `plume-tree` (0.3+) **or** the legacy `plume` (≤0.2);
//!  - a `<trash>` container's children are trashed subtrees, **and** any node with
//!    `isTrashed="yes"` (left in place in newer files) is trashed too — both are
//!    flagged and propagated to descendants;
//!  - older files simply lack `<trash>` and separator `number`s — harmless.
//!
//! Only a wrong root tag (not a Plume tree) or a version newer than the terminal
//! `0.5` is rejected.

use anyhow::{Result, bail};

use super::model::{PlumeKind, PlumeNode, PlumeTree};
use super::version::{check_root, parse_xml, version_newer_than};

/// The newest tree schema this importer understands.
const TREE_TERMINAL: f64 = 0.5;

pub fn parse(xml: &str) -> Result<PlumeTree> {
    let doc = parse_xml(xml)?;
    let root = doc.root_element();

    check_root(&root, &["plume-tree", "plume"], "tree")?;
    if version_newer_than(root.attribute("version"), TREE_TERMINAL) {
        bail!(
            "this Plume project's tree is version {} — newer than this importer supports ({TREE_TERMINAL})",
            root.attribute("version").unwrap_or("?")
        );
    }

    Ok(PlumeTree {
        project_name: root
            .attribute("projectName")
            .unwrap_or_default()
            .to_string(),
        roots: parse_children(root, false),
    })
}

/// Parse the element children of `node`. A `<trash>` child contributes its own
/// children as trashed subtrees; every other recognised child is a node.
fn parse_children(node: roxmltree::Node, parent_trashed: bool) -> Vec<PlumeNode> {
    let mut out = Vec::new();
    for child in node.children().filter(roxmltree::Node::is_element) {
        if child.tag_name().name() == "trash" {
            out.extend(parse_children(child, true));
        } else if let Some(n) = parse_node(child, parent_trashed) {
            out.push(n);
        }
    }
    out
}

fn parse_node(node: roxmltree::Node, parent_trashed: bool) -> Option<PlumeNode> {
    let kind = kind_of(node.tag_name().name())?;
    let is_trashed = parent_trashed || node.attribute("isTrashed") == Some("yes");

    // `attend` (present) + `pov` (point-of-view) both reference attendance ids.
    let mut attend = parse_ref_list(node.attribute("attend"));
    for id in parse_ref_list(node.attribute("pov")) {
        if !attend.contains(&id) {
            attend.push(id);
        }
    }

    Some(PlumeNode {
        kind,
        number: node
            .attribute("number")
            .and_then(|s| s.trim().parse::<u32>().ok()),
        name: node.attribute("name").unwrap_or_default().to_string(),
        is_trashed,
        badge: node.attribute("badge").unwrap_or_default().to_string(),
        status: parse_status(node.attribute("status")),
        attend,
        children: parse_children(node, is_trashed),
    })
}

fn kind_of(tag: &str) -> Option<PlumeKind> {
    match tag {
        "book" => Some(PlumeKind::Book),
        "act" => Some(PlumeKind::Act),
        "chapter" => Some(PlumeKind::Chapter),
        "scene" => Some(PlumeKind::Scene),
        "separator" => Some(PlumeKind::Separator),
        _ => None,
    }
}

/// Plume's revision ladder has eight rungs; `MainTreeAbstractModel::giveStatusList()`
/// builds it and `setStatus` stores the index. Anything outside `0..LADDER_LEN` — Plume
/// itself initialises `MainTreeItem::m_status` to `-1`, and separators keep it — means
/// "no status".
const LADDER_LEN: u8 = 8;

/// Read Plume's per-node `status` attribute: a 0-based index into its fixed ladder.
///
/// Deliberately **not** defaulting a missing attribute to `0`. Plume's own reader does
/// (`child.attribute("status", "0").toInt()`), which is why every untouched node in a Plume
/// project displays "1st draft" — but importing that would stamp a stage on hundreds of
/// scenes the writer never marked, which is precisely the failure bibisco is known for. An
/// absent or out-of-range value reads as "no status" here and the item arrives unmarked.
fn parse_status(raw: Option<&str>) -> Option<u8> {
    let n: i64 = raw?.trim().parse().ok()?;
    (0..LADDER_LEN as i64).contains(&n).then_some(n as u8)
}

/// Parse Plume's leading-dash integer list (`"-3-7-12"`, `"0"`, `""`) into ids,
/// dropping empties and the `0` "none" sentinel (attendance numbers start at 1).
fn parse_ref_list(raw: Option<&str>) -> Vec<u32> {
    let Some(raw) = raw else { return Vec::new() };
    let mut ids = Vec::new();
    for part in raw.split('-') {
        if let Ok(n) = part.trim().parse::<u32>()
            && n != 0
            && !ids.contains(&n)
        {
            ids.push(n);
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_reads_a_ladder_index_and_rejects_everything_else() {
        assert_eq!(parse_status(Some("0")), Some(0));
        assert_eq!(parse_status(Some("7")), Some(7));
        assert_eq!(parse_status(Some(" 3 ")), Some(3));
        // Plume's own "never set" sentinel, and the one separators keep.
        assert_eq!(parse_status(Some("-1")), None);
        // Past the end of the ladder, or not a number at all.
        assert_eq!(parse_status(Some("8")), None);
        assert_eq!(parse_status(Some("wat")), None);
        // Absent: NOT defaulted to 0, unlike Plume's own reader.
        assert_eq!(parse_status(None), None);
    }

    #[test]
    fn ref_list_parses_dash_list_and_drops_zero_sentinel() {
        assert_eq!(parse_ref_list(Some("-3-7-12")), vec![3, 7, 12]);
        assert_eq!(parse_ref_list(Some("0")), Vec::<u32>::new());
        assert_eq!(parse_ref_list(Some("-0-1")), vec![1]);
        assert_eq!(parse_ref_list(Some("")), Vec::<u32>::new());
        assert_eq!(parse_ref_list(None), Vec::<u32>::new());
        // duplicates collapse
        assert_eq!(parse_ref_list(Some("-2-2-3")), vec![2, 3]);
    }

    #[test]
    fn legacy_root_tag_and_missing_trash_are_accepted() {
        let xml = r#"<!DOCTYPE plume><plume version="0.4">
            <book number="1" name="B"><chapter number="2" name="C">
              <scene number="3" name="S"/>
            </chapter></book></plume>"#;
        let t = parse(xml).unwrap();
        assert_eq!(t.roots.len(), 1);
        assert_eq!(t.roots[0].kind, PlumeKind::Book);
        assert_eq!(t.roots[0].children[0].children[0].kind, PlumeKind::Scene);
    }

    #[test]
    fn both_trash_encodings_are_flagged() {
        let xml = r#"<plume-tree version="0.5">
            <book number="1" name="Live">
              <chapter number="2" name="Kept"/>
              <chapter number="3" name="Gone" isTrashed="yes"/>
            </book>
            <trash number="20000" name="">
              <book number="4" name="DeletedBook"/>
            </trash>
        </plume-tree>"#;
        let t = parse(xml).unwrap();
        assert_eq!(t.roots.len(), 2); // the live book + the reparented deleted book
        let live = &t.roots[0];
        assert!(!live.is_trashed);
        assert!(!live.children[0].is_trashed); // "Kept"
        assert!(live.children[1].is_trashed); // "Gone" (isTrashed=yes)
        assert!(t.roots[1].is_trashed); // whole <trash> subtree
    }

    #[test]
    fn wrong_root_and_future_version_are_rejected() {
        assert!(parse(r#"<not-plume version="0.5"/>"#).is_err());
        assert!(parse(r#"<plume-tree version="0.9"/>"#).is_err());
    }
}
