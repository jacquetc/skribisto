// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scrivener importer's side of the feature: what `analyze_scrivener_project` and
//! `import_scrivener_project` both need, in one module both call, since no use case calls
//! another. Hand-written, like `events` and `failure`: the generator knows nothing of the
//! converter's types.
//!
//! * the writer's answers, structure choices and words, as the converter takes them
//!   (`request`, [`choices_from_dto`](crate::scrivener::choices_from_dto));
//! * every refusal, question and notice the converter gives, as the typed row the
//!   interface words ([`ScrivenerFinding`](crate::dtos::ScrivenerFinding),
//!   [`every_key`](crate::scrivener::every_key));
//! * the analysis and the conversion's summary as the wizard receives them;
//! * the review page's own recomputation of the plan
//!   ([`ScrivenerReview`](crate::scrivener::ScrivenerReview)).
//!
//! # Nothing personal crosses into the project
//!
//! The computers a project names (the manifest's `Device`, the lock file's host) reach the
//! project card through [`ScrivenerComputers`](crate::dtos::ScrivenerComputers) and nowhere
//! else: not a finding, not the import's result, not the new project, and not a `{:?}` (its
//! `Debug` withholds them).

mod findings;
mod plan;
mod request;
mod review;
#[cfg(test)]
mod tests;

use scrivener_import::{Analysis, ReadProject, Summary};

use crate::dtos::{
    ImportScrivenerProjectResultDto, ScrivenerAnalysisDto, ScrivenerAnalysisOutcome,
    ScrivenerComputers, ScrivenerFinding, ScrivenerQuestion, ScrivenerTotals,
};
pub(crate) use findings::{
    Locator, metadata_notice, notice, one_per_kind, prose_notice, structure_notice,
};
pub use findings::{every_key, refusal};
pub use plan::{ChoiceOutOfRange, choices_from_dto, question};
pub(crate) use request::{analysis_answers, import_answers, words};
pub use review::{ScrivenerReview, ScrivenerReviewError};

/// Where a pattern's value goes in the words an import request carries: the item's title
/// in `notes_of_pattern`, the project's file name in `research_folder_pattern`, a field's
/// name in the two details patterns, a number in the two unnamed-value patterns, a label's
/// name in `label_tag_pattern`. A pattern without it has the value put before it.
pub const TITLE_SLOT: char = scrivener_import::TITLE_SLOT;

impl std::fmt::Debug for ScrivenerComputers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let withheld = |value: &Option<String>| value.as_ref().map(|_| "<withheld>");
        match self {
            ScrivenerComputers::Unknown => f.write_str("Unknown"),
            ScrivenerComputers::Named { device, lock_host } => f
                .debug_struct("Named")
                .field("device", &withheld(device))
                .field("lock_host", &withheld(lock_host))
                .finish(),
        }
    }
}

/// Whether the writer answers a question by choosing between things (nothing more can be
/// read until they do) or by accepting one (the project was read; the import waits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrivenerQuestionTier {
    Choose,
    Accept,
}

impl ScrivenerQuestion {
    /// The stable key the interface words this question by; empty for `Empty`, which is
    /// never sent.
    pub fn key(&self) -> &'static str {
        match self {
            ScrivenerQuestion::Empty => "",
            ScrivenerQuestion::SeveralProjects { .. } => "scrivener-question-several-projects",
            ScrivenerQuestion::SeveralManifests { .. } => "scrivener-question-several-manifests",
            ScrivenerQuestion::EnclosingProject { .. } => "scrivener-question-enclosing-project",
            ScrivenerQuestion::UpgradedSibling { .. } => "scrivener-question-upgraded-sibling",
            ScrivenerQuestion::UnmergedMobileEdits { .. } => {
                "scrivener-question-unmerged-mobile-edits"
            }
            ScrivenerQuestion::NoTextInCopy { .. } => "scrivener-question-no-text-in-copy",
        }
    }

    /// How the writer answers it.
    pub fn tier(&self) -> ScrivenerQuestionTier {
        match self {
            ScrivenerQuestion::UnmergedMobileEdits { .. }
            | ScrivenerQuestion::NoTextInCopy { .. } => ScrivenerQuestionTier::Accept,
            ScrivenerQuestion::Empty
            | ScrivenerQuestion::SeveralProjects { .. }
            | ScrivenerQuestion::SeveralManifests { .. }
            | ScrivenerQuestion::EnclosingProject { .. }
            | ScrivenerQuestion::UpgradedSibling { .. } => ScrivenerQuestionTier::Choose,
        }
    }
}

/// An analysis as the wizard receives it.
///
/// Fails only when what the review recomputes the plan from cannot be written out, which a
/// model the analysis itself built never does; the use case then reports it as the defect
/// it would be.
pub(crate) fn analysis_to_dto(analysis: Analysis) -> anyhow::Result<ScrivenerAnalysisDto> {
    match analysis {
        Analysis::Refused { refusal, notices } => {
            let names = scrivener_import::SourceNames::default();
            let locator = Locator::before_import(&names);
            Ok(ScrivenerAnalysisDto {
                outcome: ScrivenerAnalysisOutcome::Refused,
                refusal: findings::refusal(&refusal),
                notices: notices.iter().map(|n| notice(n, &locator)).collect(),
                ..ScrivenerAnalysisDto::default()
            })
        }
        Analysis::Choose { question, notices } => {
            let names = scrivener_import::SourceNames::default();
            let locator = Locator::before_import(&names);
            Ok(ScrivenerAnalysisDto {
                outcome: ScrivenerAnalysisOutcome::Choose,
                question: plan::question(&question),
                notices: notices.iter().map(|n| notice(n, &locator)).collect(),
                ..ScrivenerAnalysisDto::default()
            })
        }
        Analysis::Read(read) => read_to_dto(*read),
    }
}

fn read_to_dto(read: ReadProject) -> anyhow::Result<ScrivenerAnalysisDto> {
    let ReadProject {
        project,
        content,
        structure,
        proposals,
        card,
        notices,
        pending,
        fingerprint,
    } = read;
    let names = project.source_names();
    let locator = Locator::before_import(&names);
    let source = &project.source;
    let counts = &project.counts;
    let count = |n: u32| i64::from(n);
    let computers = if card.device.is_some() || card.lock_host.is_some() {
        ScrivenerComputers::Named {
            device: card.device.clone(),
            lock_host: card.lock_host.clone(),
        }
    } else {
        ScrivenerComputers::Unknown
    };
    let mut dto = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Read,
        pending: pending.iter().map(plan::question).collect(),
        notices: notices.iter().map(|n| notice(n, &locator)).collect(),
        folder_title: source.folder_title.clone(),
        template_title: source.template_title.clone().unwrap_or_default(),
        container: plan::container(source.container),
        generation: plan::generation(source.generation),
        platform: plan::platform(source.writer.platform),
        writer_version: source.writer.version.clone().unwrap_or_default(),
        creator: source.writer.creator.clone().unwrap_or_default(),
        manifest: source.manifest.name.clone(),
        manifest_choice: plan::manifest_choice(source.manifest.chosen),
        modified: source.modified.as_ref().and_then(|m| m.unix_seconds),
        modified_offset_minutes: source
            .modified
            .as_ref()
            .and_then(|m| m.offset_minutes.map(i64::from)),
        items: count(counts.items),
        documents: count(counts.text_items),
        manuscript_documents: count(plan::manuscript_documents(&project)),
        documents_present: count(counts.texts_present),
        media_items: count(counts.media_items),
        research_files: count(counts.research_files),
        trashed_items: count(counts.trashed_items),
        snapshots: count(counts.snapshots),
        computers,
        proposed_title: proposals.title.title.clone(),
        proposed_title_source: plan::title_source(proposals.title.source),
        proposed_authors: proposals
            .author
            .as_ref()
            .map(|a| a.names.clone())
            .unwrap_or_default(),
        proposed_author_source: plan::author_source(proposals.author.as_ref().map(|a| a.source)),
        proposed_language: proposals
            .language
            .as_ref()
            .map(|l| l.tag.clone())
            .unwrap_or_default(),
        proposed_language_source: plan::language_source(
            proposals.language.as_ref().map(|l| l.source),
        ),
        structure: plan::structure(&structure, &names),
        fingerprint,
        ..ScrivenerAnalysisDto::default()
    };
    dto.review_data = serde_json::to_string(&review::ReviewData { project, content })?;
    Ok(dto)
}

/// A finished conversion as the wizard receives it: where the project went, what it
/// holds, and every notice as a row naming the item where the writer knew it in Scrivener
/// and the row that holds it now, one row per kind and place.
pub(crate) fn summary_to_dto(summary: Summary) -> ImportScrivenerProjectResultDto {
    let locator = Locator::after_import(&summary.names, &summary.placements, &summary.rows);
    let mut findings: Vec<ScrivenerFinding> = summary
        .notices
        .iter()
        .map(|n| notice(n, &locator))
        .collect();
    findings.extend(
        summary
            .structure
            .iter()
            .map(|n| structure_notice(n, &locator)),
    );
    findings.extend(summary.prose.iter().map(|n| prose_notice(n, &locator)));
    findings.extend(summary.metadata.iter().map(metadata_notice));
    let findings = one_per_kind(findings);

    let totals = &summary.totals;
    let count = |n: u32| i64::from(n);
    let synopses = summary
        .placements
        .iter()
        .filter(|p| p.synopsis.is_some())
        .count();
    let document_notes = summary
        .placements
        .iter()
        .filter(|p| p.notes.is_some())
        .count();
    ImportScrivenerProjectResultDto {
        output_path: summary.output_path.clone(),
        research_folder: summary.research_folder.clone().unwrap_or_default(),
        totals: ScrivenerTotals::Counted {
            rows: count(totals.rows),
            words: i64::try_from(totals.words).unwrap_or(i64::MAX),
            footnotes: count(totals.footnotes),
            comments: count(totals.comments),
            pictures: count(totals.pictures),
            templates: count(totals.templates),
            tags: count(totals.tags),
            rungs: count(totals.rungs),
            trashed: count(totals.trashed),
            research_files: count(totals.research_files),
            synopses: i64::try_from(synopses).unwrap_or(i64::MAX),
            document_notes: i64::try_from(document_notes).unwrap_or(i64::MAX),
        },
        findings,
    }
}
