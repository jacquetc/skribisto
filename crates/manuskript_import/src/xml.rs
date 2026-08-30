// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared XML entry point for `world.opml`, `plots.xml`, `revisions.xml` and every
//! format-0 member.
//!
//! Manuskript writes these with lxml and no DOCTYPE, but a hand-edited or
//! third-party-written file may carry one, so the parser is told to allow it —
//! rejecting a project over a declaration nothing here reads would be a refusal
//! with no benefit.

use anyhow::{Result, anyhow};

/// Parse a Manuskript XML member.
pub fn parse(text: &str) -> Result<roxmltree::Document<'_>> {
    let opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..Default::default()
    };
    roxmltree::Document::parse_with_options(text, opts).map_err(|e| anyhow!("parsing XML: {e}"))
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
