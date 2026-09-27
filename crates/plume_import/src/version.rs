// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared root-tag and version gating for the Plume parsers.
//!
//! Plume's DOCTYPE is decorative and unreliable (its own bootstrap code leaves it
//! mismatched), so validation is by **root element tag name + numeric `version`
//! attribute** only — exactly what Plume itself checks (`Hub::loadTemp`).

use anyhow::{Result, bail};
use skrib_format::xml_depth::{self, Dtd, XmlError};

/// Parse Plume XML. **Must** allow a DTD: every Plume member starts with a
/// `<!DOCTYPE …>` declaration, which roxmltree rejects by default.
///
/// Through `skrib_format::xml_depth`, so a member nested past `MAX_XML_DEPTH` is
/// refused with a typed `XmlTooDeep` naming `part` before `roxmltree` can recurse
/// into it, and the parse runs on a stack deep enough for anything under that.
/// A DTD's entities are counted where they are expanded, which matters here more
/// than anywhere: this is one of the two readers that allows one.
pub fn parse_xml<'a>(part: &str, xml: &'a str) -> Result<roxmltree::Document<'a>> {
    xml_depth::parse(part, xml, Dtd::Allow).map_err(|e| match e {
        XmlError::Malformed(e) => anyhow::anyhow!("parsing XML: {e}"),
        other => anyhow::Error::new(other),
    })
}

/// Ensure `root`'s tag name is one of `allowed`; otherwise this isn't a Plume
/// `what` document at all.
pub fn check_root(root: &roxmltree::Node, allowed: &[&str], what: &str) -> Result<()> {
    let tag = root.tag_name().name();
    if !allowed.contains(&tag) {
        bail!("<{tag}> is not a Plume Creator {what} document");
    }
    Ok(())
}

/// True when `version` (e.g. `"0.6"`) parses to a number strictly greater than
/// `max`. A non-numeric or absent version is treated as acceptable (returns
/// `false`) — we only reject a clearly-newer numeric schema, never an unparsable
/// or missing one (older Plume files are read leniently and normalized).
pub fn version_newer_than(version: Option<&str>, max: f64) -> bool {
    match version.and_then(|v| v.trim().parse::<f64>().ok()) {
        Some(v) => v > max + 1e-9,
        None => false,
    }
}
