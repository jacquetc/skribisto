// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The report of a finished Scrivener import (D7-8): what came across, what the writer
//! chose, and every finding in four fixed groups.
//!
//! Not a view-model: the report's content, built once from the import's typed result
//! ([`ImportReport::from_result`]) and read by `report_vm` and the dialog. Pure, so its
//! grouping and its plain text are unit-tested without a widget tree.
//!
//! * **Four groups, in a fixed order** ([`Group`]): *Could not be read*, *Check these*,
//!   *Brought across differently*, *Left behind*. An empty one is left out. The converter
//!   files each finding; the table in `messages` files the source notices it leaves
//!   unfiled.
//! * **One line per kind, not per instance** (D7-8): a key's findings make one sentence
//!   with their count, and the items behind it are listed under it, in Scrivener's binder
//!   order and named by their binder path (with the writing it is about, when that is not
//!   the text). A finding of the document importer says its own sentence about the first
//!   place and how many more there are. A notice the Project page already showed about one
//!   item keeps its own sentence, which names it.
//! * **The writer's own choices** (D7 critique 4): which project, binder file or copy was
//!   read, what they accepted, how each level or folder they changed came across, the
//!   prose choices, the title, author and language the new project carries, and what
//!   became of the research files, as the import did it. After the fact, the copied report
//!   is the writer's record of the import.
//! * **Words are resolved when they are read**: every line is a `LocalizedString`, so the
//!   dialog follows a change of language, and [`ImportReport::text`] is in the language
//!   Skribisto shows itself in at the moment it is copied.
//!
//! Nothing here is written into the project (D7 Q2): the report lives as long as the
//! session that made it.

use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;
use uuid::Uuid;

use frontend::import_management::{ScrivenerFinding, ScrivenerPlace, ScrivenerTotals};

use super::import_scrivener_vm::{
    ImportRecord, NO_TEXT_IN_COPY, ScrivenerImportResult, UNMERGED_MOBILE_EDITS,
};
use super::messages::{self, Group, SaveField, Tier};

/// How many items a line lists in the dialog; *and N more* says the rest. The copied report
/// lists them all.
pub(crate) const SHOWN_ITEMS: usize = 50;

/// The four groups, in the order the report shows them.
const GROUPS: [Group; 4] = [
    Group::CouldNotBeRead,
    Group::CheckThese,
    Group::BroughtAcrossDifferently,
    Group::LeftBehind,
];

/// A finished import's report.
#[derive(Clone)]
pub(crate) struct ImportReport {
    /// The project the import wrote.
    pub(crate) output_path: String,
    /// The project it read, when the wizard that started it is known.
    pub(crate) source: Option<String>,
    /// What came across, one phrase per kind of thing it counted, zeros left out.
    pub(crate) came_across: Vec<LocalizedString>,
    /// What the writer chose, one sentence each.
    pub(crate) choices: Vec<LocalizedString>,
    /// The groups that have a line, in their fixed order.
    pub(crate) groups: Vec<ReportGroup>,
}

/// One of the four groups, with its lines.
#[derive(Clone)]
pub(crate) struct ReportGroup {
    pub(crate) group: Group,
    pub(crate) lines: Vec<ReportLine>,
}

/// One line: a sentence, and the items it is about.
#[derive(Clone)]
pub(crate) struct ReportLine {
    pub(crate) text: LocalizedString,
    /// The items, in Scrivener's binder order, each once.
    pub(crate) items: Vec<ReportItem>,
}

/// An item a line is about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReportItem {
    /// Where the writer knew it in Scrivener: its binder path, one name per level.
    path: Vec<String>,
    /// Which of its writings the line is about.
    place: ScrivenerPlace,
    /// The row of the new project that holds it, when there is one.
    pub(crate) uid: Option<Uuid>,
    /// Its place in Scrivener's binder; items with no binder row (a project note, a
    /// recovered text) come after the binder's.
    order: (bool, i64),
}

impl ReportItem {
    /// The item as the report names it: its binder path, a name Scrivener left empty
    /// said as untitled, and which writing it is when it is not the item's text, so an
    /// item's text and its notes, found the same, are two names and not one twice.
    pub(crate) fn label(&self) -> LocalizedString {
        let (path, place) = (self.path.clone(), self.place.clone());
        localized(move || {
            let untitled = tr!(import_scrivener_report_untitled()).resolve_now();
            let names: Vec<String> = if path.is_empty() {
                vec![untitled.clone()]
            } else {
                path.iter()
                    .map(|name| {
                        if name.trim().is_empty() {
                            untitled.clone()
                        } else {
                            name.clone()
                        }
                    })
                    .collect()
            };
            let path = messages::binder_path(&names);
            match place {
                ScrivenerPlace::Notes => tr!(import_scrivener_report_item_notes(path = path)),
                ScrivenerPlace::Synopsis => {
                    tr!(import_scrivener_report_item_synopsis(path = path))
                }
                ScrivenerPlace::ProjectNote => {
                    tr!(import_scrivener_report_item_project_note(path = path))
                }
                ScrivenerPlace::Recovered => {
                    tr!(import_scrivener_report_item_recovered(path = path))
                }
                _ => lit!(path),
            }
            .resolve_now()
        })
    }
}

impl ReportGroup {
    /// The group's title, with how many lines it holds.
    pub(crate) fn title(&self) -> LocalizedString {
        let group = self.group;
        let count = self.lines.len() as i64;
        localized(move || {
            tr!(import_scrivener_report_group_count(
                group = group_name(group).resolve_now(),
                count = count
            ))
            .resolve_now()
        })
    }

    /// Whether the group opens expanded: the two that need the writer (D7-8).
    pub(crate) fn expanded_by_default(&self) -> bool {
        self.group.counts_in_headline()
    }
}

impl ReportLine {
    /// The items the dialog lists: the first [`SHOWN_ITEMS`].
    pub(crate) fn shown_items(&self) -> &[ReportItem] {
        &self.items[..self.items.len().min(SHOWN_ITEMS)]
    }

    /// How many items the dialog leaves to *and N more*.
    pub(crate) fn more(&self) -> usize {
        self.items.len().saturating_sub(SHOWN_ITEMS)
    }
}

/// A group's name.
pub(crate) fn group_name(group: Group) -> LocalizedString {
    match group {
        Group::CouldNotBeRead => tr!(import_scrivener_report_group_unread()),
        Group::CheckThese => tr!(import_scrivener_report_group_check()),
        Group::BroughtAcrossDifferently => tr!(import_scrivener_report_group_changed()),
        Group::LeftBehind => tr!(import_scrivener_report_group_left()),
    }
}

impl ImportReport {
    /// The report of `result`.
    pub(crate) fn from_result(result: &ScrivenerImportResult) -> Self {
        Self {
            output_path: result.output_path.clone(),
            source: result.record.as_ref().map(|r| r.source.clone()),
            came_across: came_across(&result.totals),
            choices: result
                .record
                .as_ref()
                .map(|record| choices(record, &result.findings, &result.research_folder))
                .unwrap_or_default(),
            groups: group_findings(&result.findings, result.record.as_ref()),
        }
    }

    /// Whether the report has a finding to say, beyond the counts and the choices.
    pub(crate) fn has_findings(&self) -> bool {
        !self.groups.is_empty()
    }

    /// How many lines say something could not be read or needs checking: what the
    /// result's headline counts, so the notice and the dialog give the writer one number
    /// (Q7). What came across differently or stayed behind raises no alarm.
    pub(crate) fn lines_to_check(&self) -> usize {
        self.groups
            .iter()
            .filter(|g| g.group.counts_in_headline())
            .map(|g| g.lines.len())
            .sum()
    }

    /// What came across, as one sentence of phrases.
    pub(crate) fn came_across_line(&self) -> LocalizedString {
        let phrases = self.came_across.clone();
        localized(move || {
            if phrases.is_empty() {
                return tr!(import_scrivener_report_nothing_counted()).resolve_now();
            }
            let separator = tr!(import_scrivener_list_separator()).resolve_now();
            phrases
                .iter()
                .map(LocalizedString::resolve_now)
                .collect::<Vec<_>>()
                .join(&separator)
        })
    }

    /// The whole report as plain text, in the language Skribisto shows itself in now:
    /// what **Copy report** puts on the clipboard. Every item is listed, none capped.
    pub(crate) fn text(&self) -> String {
        let mut out = Vec::new();
        out.push(tr!(project_import_report_title()).resolve_now());
        if let Some(source) = &self.source {
            out.push(tr!(import_scrivener_report_from(path = source.clone())).resolve_now());
        }
        out.push(tr!(import_scrivener_report_to(path = self.output_path.clone())).resolve_now());
        out.push(String::new());
        out.push(tr!(import_scrivener_report_came_across()).resolve_now());
        out.push(self.came_across_line().resolve_now());
        if !self.choices.is_empty() {
            out.push(String::new());
            out.push(tr!(import_scrivener_report_choices()).resolve_now());
            for choice in &self.choices {
                out.push(format!("- {}", choice.resolve_now()));
            }
        }
        if self.groups.is_empty() {
            out.push(String::new());
            out.push(tr!(import_scrivener_report_no_findings()).resolve_now());
        }
        for group in &self.groups {
            out.push(String::new());
            out.push(group_name(group.group).resolve_now());
            for line in &group.lines {
                out.push(format!("- {}", line.text.resolve_now()));
                for item in &line.items {
                    out.push(format!("    {}", item.label().resolve_now()));
                }
            }
        }
        let mut text = out.join("\n");
        text.push('\n');
        text
    }
}

/// A count's phrase, such as "12 items".
type Phrase = fn(i64) -> LocalizedString;

/// What came across, one phrase per kind of thing counted, zeros left out.
fn came_across(totals: &ScrivenerTotals) -> Vec<LocalizedString> {
    let ScrivenerTotals::Counted {
        rows,
        words,
        footnotes,
        comments,
        pictures,
        templates,
        tags,
        rungs,
        trashed,
        research_files,
        synopses,
        document_notes,
    } = totals
    else {
        return Vec::new();
    };
    let phrases: [(i64, Phrase); 12] = [
        (*rows, |n| tr!(import_scrivener_report_items(count = n))),
        (*words, |n| tr!(import_scrivener_report_words(count = n))),
        (*synopses, |n| {
            tr!(import_scrivener_report_synopses(count = n))
        }),
        (*document_notes, |n| {
            tr!(import_scrivener_report_document_notes(count = n))
        }),
        (*comments, |n| {
            tr!(import_scrivener_report_comments(count = n))
        }),
        (*footnotes, |n| {
            tr!(import_scrivener_report_footnotes(count = n))
        }),
        (*pictures, |n| {
            tr!(import_scrivener_report_pictures(count = n))
        }),
        (*templates, |n| {
            tr!(import_scrivener_report_templates(count = n))
        }),
        (*tags, |n| tr!(import_scrivener_report_tags(count = n))),
        (*rungs, |n| tr!(import_scrivener_report_statuses(count = n))),
        (*trashed, |n| {
            tr!(import_scrivener_report_trashed(count = n))
        }),
        (*research_files, |n| {
            tr!(import_scrivener_report_research_files(count = n))
        }),
    ];
    phrases
        .into_iter()
        .filter(|(n, _)| *n > 0)
        .map(|(n, phrase)| phrase(n))
        .collect()
}

/// What the writer chose, one sentence each: which project, binder file or copy the
/// import read, what they accepted, the choices of the Review page, what the new project
/// is called and by whom, and what became of the research files.
fn choices(
    record: &ImportRecord,
    findings: &[ScrivenerFinding],
    research_folder: &str,
) -> Vec<LocalizedString> {
    let mut out = record.project_answers.clone();
    for key in &record.accepted {
        out.push(match key.as_str() {
            UNMERGED_MOBILE_EDITS => tr!(import_scrivener_report_accepted_mobile()),
            NO_TEXT_IN_COPY => tr!(import_scrivener_report_accepted_no_text()),
            // A question this version does not word: its key, rather than nothing.
            other => lit!(other.to_string()),
        });
    }
    let saved = [
        (tr!(import_scrivener_title_field()), record.title.clone()),
        (tr!(import_scrivener_author()), record.author.clone()),
        (tr!(import_scrivener_language()), record.language.clone()),
    ];
    let lines = record
        .worded
        .iter()
        .map(|line| (line.subject.clone(), line.answer.clone()))
        .chain(
            saved
                .into_iter()
                .filter(|(_, value)| !value.trim().is_empty())
                .map(|(subject, value)| (subject, lit!(value))),
        );
    for (subject, answer) in lines {
        out.push(localized(move || {
            tr!(import_scrivener_report_choice(
                subject = subject.resolve_now(),
                answer = answer.resolve_now()
            ))
            .resolve_now()
        }));
    }
    out.extend(research_line(record, findings, research_folder));
    out
}

/// What became of the research files Skribisto cannot open, as the import did it: the
/// files it copied and the ones it left in the Scrivener project, never what the
/// analysis counted before the writer left a folder out. Nothing when it met none.
fn research_line(
    record: &ImportRecord,
    findings: &[ScrivenerFinding],
    research_folder: &str,
) -> Option<LocalizedString> {
    let items = |key: &str| {
        findings
            .iter()
            .filter(|f| messages::key_of(f) == key)
            .count() as i64
    };
    let copied = items("scrivener-prose-media-file-copied");
    let stayed = items("scrivener-prose-media-file-not-carried");
    let folder = research_folder.trim().to_string();
    Some(match (copied > 0 && !folder.is_empty(), stayed) {
        (true, 0) => tr!(import_scrivener_report_research_copied(folder = folder)),
        (true, stayed) => tr!(import_scrivener_report_research_copied_except(
            folder = folder,
            count = stayed
        )),
        (false, 0) => return None,
        (false, stayed) if record.copy_research_files => {
            tr!(import_scrivener_report_research_none_copied(count = stayed))
        }
        (false, _) => tr!(import_scrivener_report_research_not_copied()),
    })
}

/// Whether the writer changed, on the Save page, the field a hint was said beside.
fn changed_by_writer(key: &str, record: Option<&ImportRecord>) -> bool {
    let Some(record) = record else {
        return false;
    };
    match messages::save_field_of(key) {
        Some(SaveField::Title) => record.title_changed,
        Some(SaveField::Author) => record.author_changed,
        Some(SaveField::Language) => record.language_changed,
        None => false,
    }
}

/// A line being gathered: its group, what makes a finding belong to it, its findings.
struct Gathering<'a> {
    group: Group,
    /// The key and the token that split it, for a line every finding of the kind shares;
    /// `None` for a line of one finding.
    shared: Option<(String, Option<String>)>,
    findings: Vec<&'a ScrivenerFinding>,
    /// For a Save page hint: the writer changed the field it was said beside.
    changed: bool,
}

/// The findings, as lines in their groups. A line takes the place of its first finding.
fn group_findings(
    findings: &[ScrivenerFinding],
    record: Option<&ImportRecord>,
) -> Vec<ReportGroup> {
    let mut gathered: Vec<Gathering<'_>> = Vec::new();
    for finding in findings {
        if matches!(finding, ScrivenerFinding::Empty) {
            continue;
        }
        let key = messages::key_of(finding);
        let changed = changed_by_writer(key, record);
        // A finding nobody filed is surfaced among what to check rather than lost. A hint
        // to check the author the project states is answered once the writer typed their
        // own: it then only says what was proposed.
        let group = match messages::group_of(finding).unwrap_or(Group::CheckThese) {
            Group::CheckThese if changed => Group::BroughtAcrossDifferently,
            group => group,
        };
        // A document importer's finding shares a line with the same kind found elsewhere,
        // a warning apart from an information.
        let identity = if messages::is_document_finding(finding) {
            Some(messages::document_variant(finding))
        } else if messages::entry(key)
            .is_some_and(|e| matches!(e.tier, Tier::BeforeImport | Tier::Report))
        {
            Some(messages::line_variant(finding))
        } else {
            None
        };
        let Some(variant) = identity else {
            gathered.push(Gathering {
                group,
                shared: None,
                findings: vec![finding],
                changed,
            });
            continue;
        };
        let identity = (key.to_string(), variant);
        match gathered
            .iter_mut()
            .find(|g| g.group == group && g.shared.as_ref() == Some(&identity))
        {
            Some(line) => line.findings.push(finding),
            None => gathered.push(Gathering {
                group,
                shared: Some(identity),
                findings: vec![finding],
                changed,
            }),
        }
    }
    GROUPS
        .iter()
        .filter_map(|&group| {
            let lines: Vec<ReportLine> = gathered
                .iter()
                .filter(|g| g.group == group)
                .map(|g| ReportLine {
                    text: line_text(g),
                    items: items_of(&g.findings),
                })
                .collect();
            (!lines.is_empty()).then_some(ReportGroup { group, lines })
        })
        .collect()
}

/// A line's sentence: the kind's sentence with its count, a notice's own title, the
/// document importer's sentence; a key this version does not word says itself rather
/// than disappearing (D7-12).
fn line_text(line: &Gathering<'_>) -> LocalizedString {
    let Some(first) = line.findings.first() else {
        return lit!("");
    };
    let worded = if messages::is_document_finding(first) {
        messages::document_lines(&line.findings)
    } else {
        match &line.shared {
            Some((key, _)) => messages::summary_line(key, &line.findings),
            None => messages::notice_line(first, line.changed),
        }
    };
    worded.unwrap_or_else(|| unworded(first))
}

/// A finding no sentence words: its key and what it names.
fn unworded(finding: &ScrivenerFinding) -> LocalizedString {
    let key = messages::key_of(finding).to_string();
    let detail = match finding {
        ScrivenerFinding::Reported { binder_path, .. } => messages::binder_path(binder_path),
        ScrivenerFinding::Empty => String::new(),
    };
    if detail.is_empty() {
        lit!(key)
    } else {
        lit!(format!("{key}: {detail}"))
    }
}

/// The items findings are about, each once, in Scrivener's binder order. A finding about
/// the project as a whole names none.
fn items_of(findings: &[&ScrivenerFinding]) -> Vec<ReportItem> {
    let mut items: Vec<ReportItem> = Vec::new();
    for finding in findings {
        let ScrivenerFinding::Reported {
            item,
            binder_path,
            item_uid,
            place,
            ..
        } = finding
        else {
            continue;
        };
        // A project note or a recovered text Scrivener left unnamed is still an item.
        let named = !binder_path.is_empty()
            || matches!(
                place,
                ScrivenerPlace::ProjectNote | ScrivenerPlace::Recovered
            );
        if !named {
            continue;
        }
        let candidate = ReportItem {
            path: binder_path.clone(),
            place: place.clone(),
            uid: *item_uid,
            order: (*item < 0, *item),
        };
        if !items.iter().any(|i| {
            i.path == candidate.path && i.place == candidate.place && i.uid == candidate.uid
        }) {
            items.push(candidate);
        }
    }
    items.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.path.cmp(&b.path)));
    items
}

#[cfg(test)]
mod tests;
