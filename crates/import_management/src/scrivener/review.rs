// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The review page's own recomputation of the structure plan, on every choice the writer
//! makes, with no round trip to a long operation.
//!
//! The analysis hands over what the plan is made from (the project read and what each of
//! its documents holds) as `ScrivenerAnalysisDto::review_data`; [`ScrivenerReview::open`]
//! reads it once, and [`ScrivenerReview::structure`] runs the converter's own pure mapping
//! with the writer's choices. The import runs the same mapping on the same inputs, so the
//! preview and the import cannot disagree. Nothing here reads a file or touches a store.

use chrono::NaiveDate;
use scrivener_import::ContentFacts;
use scrivener_import::SourceNames;
use scrivener_import::model::Project;
use scrivener_import::structure::{self, ChoiceError, PlanError};
use serde::{Deserialize, Serialize};

use super::plan::{self, ChoiceOutOfRange};
use crate::dtos::{ScrivenerChoice, ScrivenerStructure};

/// What the review recomputes the plan from, as the analysis writes it.
#[derive(Serialize, Deserialize)]
pub(crate) struct ReviewData {
    pub(crate) project: Project,
    pub(crate) content: ContentFacts,
}

/// A project read, ready to be planned again with other choices.
pub struct ScrivenerReview {
    project: Project,
    content: ContentFacts,
    names: SourceNames,
}

/// Why the plan could not be recomputed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrivenerReviewError {
    /// `review_data` is not what an analysis wrote: analyse the project again.
    Unreadable,
    /// A choice names a depth the review does not offer (only depths 1 to 3 are).
    DepthNotOffered { depth: i64 },
    /// A choice names a binder row that is not a root item the review offers.
    RootNotOffered { item: i64 },
    /// The mapping broke one of its own invariants with these choices: a defect of the
    /// importer, reported rather than shown. `detail` is the technical account.
    Unmappable { detail: String },
}

impl std::fmt::Display for ScrivenerReviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrivenerReviewError::Unreadable => f.write_str("the review data does not read"),
            ScrivenerReviewError::DepthNotOffered { depth } => {
                write!(f, "no choice is offered at depth {depth}")
            }
            ScrivenerReviewError::RootNotOffered { item } => {
                write!(f, "binder row {item} is not a root item the review offers")
            }
            ScrivenerReviewError::Unmappable { detail } => f.write_str(detail),
        }
    }
}

impl std::error::Error for ScrivenerReviewError {}

impl ScrivenerReview {
    /// Read what an analysis handed over.
    pub fn open(review_data: &str) -> Result<Self, ScrivenerReviewError> {
        let data: ReviewData =
            serde_json::from_str(review_data).map_err(|_| ScrivenerReviewError::Unreadable)?;
        let names = data.project.source_names();
        Ok(ScrivenerReview {
            project: data.project,
            content: data.content,
            names,
        })
    }

    /// The plan with the writer's `choices`, each a change from what was detected, as it
    /// stands on `today`, the writer's calendar day: a deadline is ahead or behind the day
    /// the writer looks, as the import will count it.
    pub fn structure(
        &self,
        choices: &[ScrivenerChoice],
        today: NaiveDate,
    ) -> Result<ScrivenerStructure, ScrivenerReviewError> {
        let choices = plan::choices_from_dto(choices).map_err(|out| match out {
            ChoiceOutOfRange::Depth(depth) => ScrivenerReviewError::DepthNotOffered { depth },
            ChoiceOutOfRange::Item(item) => ScrivenerReviewError::RootNotOffered { item },
        })?;
        let planned =
            structure::plan(&self.project, &self.content, &choices).map_err(|e| match e {
                PlanError::Choice(ChoiceError::DepthOutOfRange { depth }) => {
                    ScrivenerReviewError::DepthNotOffered {
                        depth: i64::from(depth),
                    }
                }
                PlanError::Choice(ChoiceError::RootNotOffered { item }) => {
                    ScrivenerReviewError::RootNotOffered {
                        item: i64::from(item),
                    }
                }
                other => ScrivenerReviewError::Unmappable {
                    detail: other.to_string(),
                },
            })?;
        Ok(plan::structure(&planned, &self.project, &self.names, today))
    }
}
