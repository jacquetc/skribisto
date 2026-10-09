// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every notice an import of each named path raises, one per line, for comparing two copies
//! of the same projects: `scripts/scrivener_corpus/scrub_ground_truth.py` runs it on the
//! ground-truth handover before and after scrubbing it, and refuses a scrub that changes
//! what the importer says.
//!
//! ```text
//! cargo run -p skribisto-scrivener-import --example notice_set -- PATH...
//! ```
//!
//! For each path it prints a `== PATH` line, then one line per notice: the stage that
//! raised it (`analysis`, `notice`, `structure`, `prose`, `metadata`) and the notice in its
//! compact debug form. A refusal, a question or a failed conversion is a line of its own.
//! Nothing personal is printed: the card (the device and the lock's host) is never read,
//! and notices carry nothing personal by design. The conversion is written to a scratch
//! folder that is removed afterwards.

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use chrono::DateTime;
use scrivener_import::structure::Choices;
use scrivener_import::{
    Analysis, Answers, DefaultRung, MetadataWords, Options, ProseChoices, Request, TITLE_SLOT,
    VALUE_SLOT, Words, analyze, convert,
};

/// The host this run pretends to be on: a name no project in a handover carries, so a lock
/// left on the writer's Mac reads as left elsewhere in both copies.
const HOST: &str = "notice-set-host";

fn metadata_words() -> MetadataWords {
    let mut words = MetadataWords::default();
    for key in MetadataWords::ORDER {
        let value = match *key {
            "keyword-details" | "field-details" | "collection-details" => {
                format!("W-{key} {TITLE_SLOT}")
            }
            "unnamed-keyword" => format!("W-keyword {TITLE_SLOT}"),
            "keyword-tag" | "collection-tag" | "field-tag" => format!("{TITLE_SLOT} W-({key})"),
            "field-value" => format!("{TITLE_SLOT}: {VALUE_SLOT}"),
            other => format!("W-{other}"),
        };
        words.set(key, &value);
    }
    words
}

fn words() -> Words {
    Words {
        manuscript_binder: "W-Manuscript".to_string(),
        notes_binder: "W-Notes".to_string(),
        notes_of: format!("{TITLE_SLOT} W-notes"),
        project_notes: "W-ProjectNotes".to_string(),
        recovered: "W-Recovered".to_string(),
        untitled_template: "W-Untitled".to_string(),
        research_folder: format!("{TITLE_SLOT} W-research"),
        list_separator: " W+ ".to_string(),
        label_details: format!("W-field {TITLE_SLOT}"),
        status_details: format!("W-status-field {TITLE_SLOT}"),
        unnamed_label: format!("W-label {TITLE_SLOT}"),
        unnamed_status: format!("W-status {TITLE_SLOT}"),
        label_tag: format!("{TITLE_SLOT} W-(label)"),
        untitled_tag: "W-untitled-tag".to_string(),
        default_statuses: vec![
            DefaultRung {
                name: "W-planned".to_string(),
                category: common::entities::StatusCategory::Planned,
            },
            DefaultRung {
                name: "W-final".to_string(),
                category: common::entities::StatusCategory::Final,
            },
        ],
        metadata: metadata_words(),
    }
}

/// The notices of one path, each prefixed with its stage.
fn notices_of(source: &Path, scratch: &Path) -> Vec<String> {
    let options = Options {
        this_host: Some(HOST.to_string()),
    };
    let answers = Answers::default();
    let mut lines = Vec::new();
    match analyze(source, &answers, &options) {
        Analysis::Refused { refusal, notices } => {
            lines.push(format!("refused\t{refusal:?}"));
            lines.extend(notices.iter().map(|n| format!("analysis\t{n:?}")));
            return lines;
        }
        Analysis::Choose { question, notices } => {
            lines.push(format!("choose\t{question:?}"));
            lines.extend(notices.iter().map(|n| format!("analysis\t{n:?}")));
            return lines;
        }
        Analysis::Read(read) => {
            lines.extend(read.notices.iter().map(|n| format!("analysis\t{n:?}")));
            lines.extend(read.pending.iter().map(|q| format!("pending\t{q:?}")));
        }
    }
    let output = scratch.join("notice-set.skrib");
    let output = output.to_string_lossy().into_owned();
    let (choices, words) = (Choices::default(), words());
    let instant = DateTime::from_timestamp(1_790_000_000, 0).unwrap_or_default();
    let request = Request {
        source,
        answers: &answers,
        options: &options,
        choices: &choices,
        prose: ProseChoices::default(),
        fingerprint: None,
        output: &output,
        overwrite: true,
        words: &words,
        title: None,
        author: None,
        language: None,
        copy_research_files: true,
        now: instant,
        today: instant.date_naive(),
    };
    match convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        Ok(summary) => {
            lines.extend(summary.notices.iter().map(|n| format!("notice\t{n:?}")));
            lines.extend(
                summary
                    .structure
                    .iter()
                    .map(|n| format!("structure\t{n:?}")),
            );
            lines.extend(summary.prose.iter().map(|n| format!("prose\t{n:?}")));
            lines.extend(summary.metadata.iter().map(|n| format!("metadata\t{n:?}")));
        }
        Err(e) => lines.push(format!("failed\t{e:?}")),
    }
    lines
}

fn main() -> std::process::ExitCode {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("usage: notice_set PATH...");
        return std::process::ExitCode::FAILURE;
    }
    for path in paths {
        let scratch = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("a scratch folder: {e}");
                return std::process::ExitCode::FAILURE;
            }
        };
        println!("== {}", path.display());
        for line in notices_of(&path, scratch.path()) {
            println!("{line}");
        }
    }
    std::process::ExitCode::SUCCESS
}
