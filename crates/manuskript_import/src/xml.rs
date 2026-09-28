// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared XML entry point for `world.opml`, `plots.xml`, `revisions.xml` and every
//! format-0 member.
//!
//! Manuskript writes these with lxml and no DOCTYPE, but a hand-edited or
//! third-party-written file may carry one, so the parser is told to allow it —
//! rejecting a project over a declaration nothing here reads would be a refusal
//! with no benefit. An entity declaration is the exception: allowed, `roxmltree`
//! would expand it as often as the member names it, into more memory than the
//! computer has, so a member declaring one is refused before it is parsed.
//!
//! Every parse goes through `skrib_format::xml_depth`, which refuses a member
//! declaring an entity, or nested past `MAX_XML_DEPTH`, before `roxmltree` sees
//! it, and runs the parse on a stack deep enough for anything under that. The
//! whole project is checked the same two ways before any member is read (see
//! [`crate::refuse_entity_declarations`] and [`crate::refuse_deep_xml`]), so the
//! refusal a reader could meet here is the second line of that defence, not the
//! first.

use anyhow::{Result, anyhow};
use skrib_format::xml_depth::{self, Dtd, XmlError};

/// Parse a Manuskript XML member. `member` names it in a refusal.
pub fn parse<'a>(member: &str, text: &'a str) -> Result<roxmltree::Document<'a>> {
    xml_depth::parse(member, text, Dtd::Allow).map_err(|e| match e {
        XmlError::Malformed(e) => anyhow!("parsing XML: {e}"),
        other => anyhow::Error::new(other),
    })
}

/// Whether a recursive walk may read the children of an element `depth` levels
/// deep, the root counting as one.
///
/// Every tree these readers walk came out of [`parse`], which refused any
/// document whose tree would nest past `MAX_XML_DEPTH`, levels an entity leaves
/// open included, so this never stops a walk over a document read here. It keeps each recursion bounded by the same ceiling on its own terms,
/// whatever handed it the node.
pub fn may_descend(depth: usize) -> bool {
    depth < skrib_format::MAX_XML_DEPTH
}

/// An element's attribute as an owned string, empty when absent.
pub fn attr(node: &roxmltree::Node, name: &str) -> String {
    node.attribute(name).unwrap_or_default().to_string()
}

/// An element's attribute, or `None` when absent or empty.
pub fn attr_opt(node: &roxmltree::Node, name: &str) -> Option<String> {
    node.attribute(name)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Manuskript's 0/1/2 importance scale, as written by both plots and characters.
pub fn importance(raw: Option<&str>) -> Option<u8> {
    let n: u8 = raw?.trim().parse().ok()?;
    (n <= 2).then_some(n)
}
