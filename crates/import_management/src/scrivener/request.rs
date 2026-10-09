// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the wizard sends, as the converter takes it: the writer's answers and the words
//! the import writes into the project.

use scrivener_import::{Answers, DefaultRung, Words};

use super::plan::sibling_choice;
use crate::dtos::{
    AnalyzeScrivenerProjectDto, ImportScrivenerProjectDto, ScrivenerDefaultRung,
    ScrivenerSiblingChoice,
};

/// The answers both requests carry, field for field. An empty choice is no answer.
fn answers(
    chosen_project: &str,
    chosen_manifest: &str,
    use_enclosing_project: bool,
    upgraded_sibling: &ScrivenerSiblingChoice,
    accept_unmerged_mobile_edits: bool,
    accept_no_text: bool,
) -> Answers {
    let answered = |value: &str| (!value.is_empty()).then(|| value.to_string());
    Answers {
        project: answered(chosen_project),
        manifest: answered(chosen_manifest),
        use_enclosing_project,
        upgraded_sibling: sibling_choice(upgraded_sibling),
        accept_unmerged_mobile_edits,
        accept_no_text,
    }
}

/// The answers an analysis request carries.
pub(crate) fn analysis_answers(dto: &AnalyzeScrivenerProjectDto) -> Answers {
    answers(
        &dto.chosen_project,
        &dto.chosen_manifest,
        dto.use_enclosing_project,
        &dto.upgraded_sibling,
        dto.accept_unmerged_mobile_edits,
        dto.accept_no_text,
    )
}

/// The answers an import request carries: the analysis's, again.
pub(crate) fn import_answers(dto: &ImportScrivenerProjectDto) -> Answers {
    answers(
        &dto.chosen_project,
        &dto.chosen_manifest,
        dto.use_enclosing_project,
        &dto.upgraded_sibling,
        dto.accept_unmerged_mobile_edits,
        dto.accept_no_text,
    )
}

/// The words the import writes into the project, exactly as the interface resolved them:
/// nothing is defaulted, so a caller that leaves one empty gets it empty.
pub(crate) fn words(dto: &ImportScrivenerProjectDto) -> Words {
    Words {
        manuscript_binder: dto.manuscript_binder_name.clone(),
        notes_binder: dto.notes_binder_name.clone(),
        notes_of: dto.notes_of_pattern.clone(),
        project_notes: dto.project_notes_name.clone(),
        recovered: dto.recovered_name.clone(),
        untitled_template: dto.untitled_template_name.clone(),
        research_folder: dto.research_folder_pattern.clone(),
        list_separator: dto.list_separator.clone(),
        label_details: dto.label_details_pattern.clone(),
        status_details: dto.status_details_pattern.clone(),
        unnamed_label: dto.unnamed_label_pattern.clone(),
        unnamed_status: dto.unnamed_status_pattern.clone(),
        label_tag: dto.label_tag_pattern.clone(),
        untitled_tag: dto.untitled_tag_name.clone(),
        default_statuses: dto
            .default_statuses
            .iter()
            .filter_map(|rung| match rung {
                ScrivenerDefaultRung::Empty => None,
                ScrivenerDefaultRung::Rung { name, category } => Some(DefaultRung {
                    name: name.clone(),
                    category: category.clone(),
                }),
            })
            .collect(),
    }
}
