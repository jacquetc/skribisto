// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a format-1 project: everything Manuskript 0.3.0 onward writes.
//!
//! Each member is read on its own and a member that cannot be read costs only what
//! it held. A project whose `plots.xml` is malformed still brings its manuscript,
//! its cast and its world across, and says what it could not bring — which is the
//! whole reason the readers return notices instead of errors.

pub mod characters;
pub mod outline;
pub mod plots;
pub mod world;

use crate::mmd;
use crate::model::{Info, Label, Project, Settings, Summary, normalise_color};
use crate::outline_xml;
use crate::source::ManuskriptSource;

const INFOS_MEMBER: &str = "infos.txt";
const SUMMARY_MEMBER: &str = "summary.txt";
const LABELS_MEMBER: &str = "labels.txt";
const STATUS_MEMBER: &str = "status.txt";
const SETTINGS_MEMBER: &str = "settings.txt";
const REVISIONS_MEMBER: &str = "revisions.xml";

/// `infos.txt`'s keys. Note `Serie`: Manuskript's own spelling, never corrected.
const INFO_KEYS: &[&str] = &[
    "Title", "Subtitle", "Serie", "Volume", "Genre", "License", "Author", "Email",
];
/// `summary.txt`'s keys — the snowflake ladder, shortest first.
const SUMMARY_KEYS: &[&str] = &["Situation", "Sentence", "Paragraph", "Page", "Full"];

/// Read a whole format-1 project.
pub fn read(src: &ManuskriptSource) -> Project {
    let mut notices = src.notices.clone();

    let info = read_info(src, &mut notices);
    let summary = read_summary(src, &mut notices);
    let labels = read_labels(src, &mut notices);
    let statuses = read_statuses(src);
    let outline = outline::read(src, &mut notices);
    let characters = characters::read(src, &mut notices);
    let world = src
        .text(world::WORLD_MEMBER)
        .map(|t| world::read(&t, &mut notices))
        .unwrap_or_default();
    let plots = src
        .text(plots::PLOTS_MEMBER)
        .map(|t| plots::read(&t, &mut notices))
        .unwrap_or_default();
    let settings = read_settings(src, &mut notices);
    let revisions = read_revisions(src, &mut notices);

    report_unaccounted_members(src, &mut notices);

    Project {
        source_name: src.project_name.clone(),
        info,
        summary,
        labels,
        statuses,
        outline,
        characters,
        world,
        plots,
        settings,
        revisions,
        notices,
    }
}

fn read_info(src: &ManuskriptSource, notices: &mut Vec<String>) -> Info {
    let Some(text) = src.text(INFOS_MEMBER) else {
        return Info::default();
    };
    let file = mmd::parse(&text);
    for key in file.unknown_keys(INFO_KEYS) {
        notices.push(format!(
            "'{INFOS_MEMBER}' carries a field this importer does not know, '{key}'. It was not \
             imported."
        ));
    }
    Info {
        title: file.get("Title").unwrap_or_default().to_string(),
        subtitle: file.get("Subtitle").unwrap_or_default().to_string(),
        serie: file.get("Serie").unwrap_or_default().to_string(),
        volume: file.get("Volume").unwrap_or_default().to_string(),
        genre: file.get("Genre").unwrap_or_default().to_string(),
        license: file.get("License").unwrap_or_default().to_string(),
        author: file.get("Author").unwrap_or_default().to_string(),
        email: file.get("Email").unwrap_or_default().to_string(),
    }
}

fn read_summary(src: &ManuskriptSource, notices: &mut Vec<String>) -> Summary {
    let Some(text) = src.text(SUMMARY_MEMBER) else {
        return Summary::default();
    };
    let file = mmd::parse(&text);
    for key in file.unknown_keys(SUMMARY_KEYS) {
        notices.push(format!(
            "'{SUMMARY_MEMBER}' carries a field this importer does not know, '{key}'. It was not \
             imported."
        ));
    }
    Summary {
        situation: file.get("Situation").unwrap_or_default().to_string(),
        sentence: file.get("Sentence").unwrap_or_default().to_string(),
        paragraph: file.get("Paragraph").unwrap_or_default().to_string(),
        page: file.get("Page").unwrap_or_default().to_string(),
        full: file.get("Full").unwrap_or_default().to_string(),
    }
}

/// Read `labels.txt`: one `Name:<padding>#rrggbb` per line.
///
/// ⚠ Manuskript's own reader requires the colon and dereferences the match without
/// checking, so a colourless label crashes its load. Here a line with no colon is
/// a label with no colour, which is the only reading that does not throw away a
/// project over a hand edit.
fn read_labels(src: &ManuskriptSource, notices: &mut Vec<String>) -> Vec<Label> {
    let Some(text) = src.text(LABELS_MEMBER) else {
        return Vec::new();
    };
    let mut colourless = 0usize;
    let mut labels = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match line.split_once(':') {
            Some((name, color)) => labels.push(Label {
                name: name.trim().to_string(),
                color: normalise_color(color.trim()),
            }),
            None => {
                colourless += 1;
                labels.push(Label {
                    name: line.trim().to_string(),
                    color: None,
                });
            }
        }
    }
    if colourless > 0 {
        notices.push(format!(
            "'{LABELS_MEMBER}' has {colourless} line(s) with no colour. They were imported \
             without one; Manuskript stops loading the project on such a line."
        ));
    }
    labels
}

/// Read `status.txt`: one bare name per line, colour and all.
///
/// Asymmetric with `labels.txt` on purpose, and in Manuskript too: the writer
/// gives a status no colour and the reader takes the whole line as the name, so a
/// status whose name contains a colon is fine here and would be truncated there.
fn read_statuses(src: &ManuskriptSource) -> Vec<String> {
    src.text(STATUS_MEMBER)
        .map(|text| {
            text.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// Read the handful of `settings.txt` keys that describe the project.
///
/// JSON since the pickle was removed for the CVE. Everything else in the file is
/// window state — pane sizes, the cork-board background, the frequency analyzer's
/// word list — and belongs to the application the project is leaving.
fn read_settings(src: &ManuskriptSource, notices: &mut Vec<String>) -> Settings {
    let Some(text) = src.text(SETTINGS_MEMBER) else {
        return Settings::default();
    };
    let parsed: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            notices.push(format!(
                "'{SETTINGS_MEMBER}' is not readable JSON ({e}); the project's language was not \
                 imported."
            ));
            return Settings::default();
        }
    };
    Settings {
        dict: parsed
            .get("dict")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(str::to_string),
        revisions_keep: parsed
            .get("revisions")
            .and_then(|r| r.get("keep"))
            .and_then(serde_json::Value::as_bool),
    }
}

/// Read `revisions.xml`, if the project kept one.
///
/// It mirrors the entire outline a second time, prose included. Only the revisions
/// are taken: where the same row exists in both, `outline/` is the truth, which is
/// the precedence Manuskript applies for the same reason — the folder is what a
/// third-party editor would have touched.
fn read_revisions(
    src: &ManuskriptSource,
    notices: &mut Vec<String>,
) -> Vec<crate::model::Revision> {
    let Some(text) = src.text(REVISIONS_MEMBER) else {
        return Vec::new();
    };
    match outline_xml::parse(&text) {
        Ok(parsed) => {
            notices.extend(parsed.notices);
            parsed.revisions
        }
        Err(e) => {
            notices.push(format!(
                "'{REVISIONS_MEMBER}' could not be read ({e}); the project's history was not \
                 imported, but nothing else was affected."
            ));
            Vec::new()
        }
    }
}

/// Name every member that is not part of the format.
///
/// Manuskript reads such a file, keeps it in memory, regenerates everything else,
/// and then **deletes it** as a phantom on the next save — which is how it has
/// deleted writers' images out of their own project folders. Nothing is deleted
/// here, but a file that was not imported should be said out loud rather than left
/// for the writer to discover missing.
fn report_unaccounted_members(src: &ManuskriptSource, notices: &mut Vec<String>) {
    let known_roots = [
        "MANUSKRIPT",
        "VERSION",
        INFOS_MEMBER,
        SUMMARY_MEMBER,
        LABELS_MEMBER,
        STATUS_MEMBER,
        SETTINGS_MEMBER,
        REVISIONS_MEMBER,
        world::WORLD_MEMBER,
        plots::PLOTS_MEMBER,
        crate::source::PICKLE_MEMBER,
    ];
    let mut unaccounted: Vec<&str> = src
        .members()
        .into_iter()
        .filter(|m| {
            !known_roots.contains(m)
                && !m.starts_with(outline::OUTLINE_DIR)
                && !m.starts_with(characters::CHARACTERS_DIR)
        })
        .collect();
    unaccounted.sort_unstable();
    if unaccounted.is_empty() {
        return;
    }
    let shown: Vec<&str> = unaccounted.iter().copied().take(5).collect();
    let more = unaccounted.len().saturating_sub(shown.len());
    let tail = if more > 0 {
        format!(" and {more} more")
    } else {
        String::new()
    };
    notices.push(format!(
        "This project holds {} file(s) that are not part of the Manuskript format and were not \
         imported: {}{tail}.",
        unaccounted.len(),
        shown.join(", ")
    ));
}
