// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Parse the `info` member for the authoritative project title + best-effort
//! creation/modification dates (from the `<prj>` element). The `info` file's
//! `<prj name=…>` — not the tree root's vestigial `projectName` — is the real
//! title.

use anyhow::Result;

use super::model::PlumeInfo;
use super::version::{check_root, parse_xml};

pub fn parse(xml: &str) -> Result<PlumeInfo> {
    let doc = parse_xml(xml)?;
    let root = doc.root_element();
    check_root(&root, &["plume-information"], "info")?;

    let prj = root
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "prj");

    let info = match prj {
        Some(p) => PlumeInfo {
            title: p.attribute("name").unwrap_or_default().trim().to_string(),
            created_at: parse_iso(p.attribute("creationDate")),
            updated_at: parse_iso(p.attribute("lastModified")),
        },
        None => PlumeInfo::default(),
    };
    Ok(info)
}

/// Parse an ISO-8601-ish timestamp to an RFC3339 string. Handles the common
/// `creationDate="2013-08-20T22:32:12"` form (assumed UTC) and full RFC3339;
/// returns `None` for Plume's localized `lastModified` strings
/// (`"lun. 7. janv. …"`) rather than guessing.
fn parse_iso(raw: Option<&str>) -> Option<String> {
    let s = raw?.trim();
    if s.is_empty() {
        return None;
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&chrono::Utc).to_rfc3339());
    }
    if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(ndt.and_utc().to_rfc3339());
    }
    None
}
