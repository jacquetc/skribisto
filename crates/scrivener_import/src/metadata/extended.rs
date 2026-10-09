// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The second half of the metadata mapping, named and worded: the keywords, the list and
//! checkbox fields and the collections as tags (D4-7, D4-8, D4-13), the targets as goals
//! and the project's unit (D4-11, X-30), the deadline as a pace plan (D4-12), bookmarks as
//! links where Skribisto shows them (D4-14, X-35), the auto-complete list as the
//! dictionary (D4-17), and the "From Scrivener" folder's notes (D4-20).
//!
//! [`extended()`] takes what [`super::collect::collect()`] decided and gives every tag a name no other
//! tag holds (D4-9), every made-up word from the interface ([`MetadataWords`]), and the
//! deadline its plan once the day of the import is known.

use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use common::entities::GoalUnit;

use super::collect::{Collected, Goal, PaceInputs, RawTag, TagSource, collect};
use super::listing;
use super::notice::{DeadlineReason, MetadataNotice};
use super::vocabulary::{PALETTE, folded, hex};
use super::words::{MetadataWords, fill_two};
use crate::convert::{Words, fill};
use crate::model::Project;
use crate::structure::{NoteSection, RowRef, StructurePlan};

/// A tag made by the second half, named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedTag {
    pub name: String,
    /// `#rrggbb`.
    pub color: String,
    pub details: String,
}

/// The pace plan the deadline became.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedPace {
    /// The book's row.
    pub book: RowRef,
    /// The day of the import, on the writer's calendar.
    pub start: NaiveDate,
    /// The deadline's own calendar day.
    pub end: NaiveDate,
    /// Writing days, Monday = 1 to Sunday = 64: every day (D4-12).
    pub weekday_mask: i64,
    /// The label of a day off on the deadline itself, when Scrivener's calculator kept it
    /// free.
    pub day_off: Option<String>,
}

/// Everything the second half writes, by row of the plan it was made for.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Extended {
    pub tags: Vec<ExtendedTag>,
    /// Per binder (the manuscript's, the notes'), per row: indices into `tags`.
    pub row_tags: [Vec<Vec<u32>>; 2],
    /// Per row: its word goal and its character goal.
    pub goals: HashMap<RowRef, (i64, i64)>,
    pub goal_unit: GoalUnit,
    /// References, owner row to target row.
    pub edges: Vec<(RowRef, RowRef)>,
    pub dictionary: Vec<String>,
    pub pace: Option<PlannedPace>,
    /// The "From Scrivener" folder's notes, in its order, as Djot.
    pub notes: Vec<(NoteSection, String)>,
    /// Per note-like row: Djot to follow its own text.
    pub sheets: HashMap<RowRef, String>,
    pub notices: Vec<MetadataNotice>,
}

impl Extended {
    /// The Djot of the note for `section`, when the folder holds it.
    pub fn note(&self, section: NoteSection) -> Option<&str> {
        self.notes
            .iter()
            .find(|(s, _)| *s == section)
            .map(|(_, djot)| djot.as_str())
    }
}

/// Every tag name already held, and who holds it, as names are compared (NFC, case-folded).
struct Namer {
    taken: HashMap<String, Option<TagSource>>,
    next_ordinal: HashMap<String, u32>,
    disambiguated: Vec<String>,
}

impl Namer {
    /// `earlier`: the names the story bible's tags and the labels' already hold.
    fn new(earlier: &[String]) -> Self {
        Namer {
            taken: earlier.iter().map(|n| (folded(n), None)).collect(),
            next_ordinal: HashMap::new(),
            disambiguated: Vec::new(),
        }
    }

    fn holder(&self, name: &str) -> Option<Option<TagSource>> {
        self.taken.get(&folded(name)).copied()
    }

    /// `name` when free, else the first of `alternatives` that is, else the last
    /// alternative with the next ordinal its clashes have not used.
    fn claim(&mut self, name: &str, source: TagSource, alternatives: &[String]) -> String {
        let chosen = if self.holder(name).is_none() {
            name.to_string()
        } else if let Some(free) = alternatives.iter().find(|a| self.holder(a).is_none()) {
            free.clone()
        } else {
            let base = alternatives.last().map_or(name, String::as_str).to_string();
            let n = self.next_ordinal.entry(folded(&base)).or_insert(2);
            loop {
                let candidate = format!("{base} ({n})");
                *n = n.saturating_add(1);
                if !self.taken.contains_key(&folded(&candidate)) {
                    break candidate;
                }
            }
        };
        if chosen != name {
            self.disambiguated.push(chosen.clone());
        }
        self.taken.insert(folded(&chosen), Some(source));
        chosen
    }
}

/// `pattern` filled with `value` when it holds the slot, else as it is: a provenance line
/// that names nothing.
fn fill_if_slotted(pattern: &str, value: &str) -> String {
    if pattern.contains(crate::convert::TITLE_SLOT) {
        fill(pattern, value)
    } else {
        pattern.trim().to_string()
    }
}

/// Name one tag.
fn name_tag(tag: &RawTag, namer: &mut Namer, words: &Words) -> ExtendedTag {
    let w: &MetadataWords = &words.metadata;
    let (name, details) = match tag.source {
        TagSource::Keyword => {
            let own = if tag.name.is_empty() {
                fill(&w.unnamed_keyword, &tag.id.to_string())
            } else {
                tag.name.clone()
            };
            let marked = fill(&w.keyword_tag, &own);
            let under = tag.parent.as_ref().map(|p| format!("{own} ({p})"));
            // Two keywords of one name are told apart by where they sit (D4-7); a keyword
            // meeting another kind of tag's name says it is a keyword (D4-9).
            let alternatives: Vec<String> = match (namer.holder(&own), under) {
                (Some(Some(TagSource::Keyword)), Some(under)) => vec![under, marked],
                (_, Some(under)) => vec![marked, under],
                (_, None) => vec![marked],
            };
            let path = if tag.path.is_empty() {
                own.clone()
            } else {
                tag.path.clone()
            };
            (
                namer.claim(&own, TagSource::Keyword, &alternatives),
                fill(&w.keyword_details, &path),
            )
        }
        TagSource::Collection => {
            let own = if tag.name.is_empty() {
                words.untitled_tag.clone()
            } else {
                tag.name.clone()
            };
            let marked = fill(&w.collection_tag, &own);
            (
                namer.claim(&own, TagSource::Collection, &[marked]),
                fill_if_slotted(&w.collection_details, &own),
            )
        }
        TagSource::Field => {
            let own = match &tag.option {
                Some(option) => fill_two(&w.field_value, &tag.name, option),
                None => tag.name.clone(),
            };
            let marked = fill(&w.field_tag, &own);
            (
                namer.claim(&own, TagSource::Field, &[marked]),
                fill(&w.field_details, &tag.path),
            )
        }
    };
    ExtendedTag {
        name,
        color: tag.color.map(hex).unwrap_or_default(),
        details,
    }
}

/// Map the second half of `project`'s metadata onto the rows `plan` makes.
///
/// `earlier_tags` are the names the story bible's tags and the label tags already hold;
/// `language` the manuscript's (the goal unit falls back to its own, as New Work's does);
/// `today` the day of the import on the writer's own calendar (the interface's, as every
/// pace plan's days are), which a deadline must not be behind.
pub fn extended(
    project: &Project,
    plan: &StructurePlan,
    earlier_tags: &[String],
    words: &Words,
    language: Option<&str>,
    today: NaiveDate,
) -> Extended {
    let Collected {
        tags,
        row_tags,
        goals,
        book_goal,
        unit,
        edges,
        dictionary,
        pace,
        deadline_blocked,
        listing,
        sheets,
        mut notices,
    } = collect(project, plan);
    let w = &words.metadata;

    // Names, each once across every kind of tag (D4-9).
    let mut namer = Namer::new(earlier_tags);
    let mut named: Vec<ExtendedTag> = tags
        .iter()
        .map(|t| name_tag(t, &mut namer, words))
        .collect();
    let earlier = earlier_tags.len();
    for (k, tag) in named.iter_mut().enumerate() {
        if tag.color.is_empty() {
            tag.color = PALETTE[(earlier + k) % PALETTE.len()].to_string();
        }
    }
    if !namer.disambiguated.is_empty() {
        notices.push(MetadataNotice::TagNameDisambiguated {
            names: namer.disambiguated,
        });
    }

    // Goals, and the unit the project counts in (X-30).
    let goal_unit = unit.unwrap_or_else(|| {
        skribisto_model::goal_unit::default_unit_for_language(language.unwrap_or_default())
    });
    let mut by_row: HashMap<RowRef, (i64, i64)> = HashMap::new();
    for Goal { row, value, unit } in goals.iter().chain(book_goal.iter()) {
        let entry = by_row.entry(*row).or_insert((0, 0));
        match unit {
            GoalUnit::Words => entry.0 = *value,
            GoalUnit::Characters => entry.1 = *value,
        }
    }
    notices.extend(other_unit_notice(&goals, book_goal.as_ref(), &goal_unit));

    // The pace plan, for a deadline still ahead (D4-12).
    let (planned, pace_notices) = plan_pace(pace, deadline_blocked, today, &w.deadline_day);
    notices.extend(pace_notices);

    // The folder's notes, and what each note-like row lists of its own.
    let notes = listing
        .sections()
        .into_iter()
        .map(|section| {
            (
                section,
                listing::note(section, &listing, w, &words.list_separator),
            )
        })
        .collect();
    let sheets = sheets
        .iter()
        .map(|(at, extra)| (*at, listing::sheet(extra, w)))
        .filter(|(_, djot)| !djot.is_empty())
        .collect();

    // Each word once in the dictionary, as the dictionary compares them.
    let mut seen: HashSet<&str> = HashSet::new();
    let dictionary = dictionary
        .iter()
        .filter(|word| seen.insert(word.as_str()))
        .cloned()
        .collect();

    Extended {
        tags: named,
        row_tags,
        goals: by_row,
        goal_unit,
        edges,
        dictionary,
        pace: planned,
        notes,
        sheets,
        notices,
    }
}

/// What the second half of the mapping will say of `project` over `plan`, before any of it
/// is written: every notice [`extended()`] gives, but the names of tags it renames, which
/// depend on the words the import is given. For the review page's "before you import"
/// strip (D7-5), which shows the binder before the import makes it; `today` the day the
/// writer looks, on their own calendar, which a deadline must not be behind.
pub fn foreseen(project: &Project, plan: &StructurePlan, today: NaiveDate) -> Vec<MetadataNotice> {
    let Collected {
        goals,
        book_goal,
        unit,
        pace,
        deadline_blocked,
        mut notices,
        ..
    } = collect(project, plan);
    // A unit the targets leave undecided is the language's, which the Save page has not
    // settled yet. It makes no difference to the count: the targets leave it undecided
    // when there are no goals, or as many in words as in characters, and then either unit
    // leaves the same number in the other.
    let unit = unit.unwrap_or(GoalUnit::Words);
    notices.extend(other_unit_notice(&goals, book_goal.as_ref(), &unit));
    notices.extend(plan_pace(pace, deadline_blocked, today, "").1);
    notices
}

/// The notice for the goals written in the unit the project does not count in, if any.
fn other_unit_notice(
    goals: &[Goal],
    book_goal: Option<&Goal>,
    goal_unit: &GoalUnit,
) -> Option<MetadataNotice> {
    let other_unit = goals
        .iter()
        .chain(book_goal)
        .filter(|goal| goal.unit != *goal_unit)
        .count() as u32;
    (other_unit > 0).then_some(MetadataNotice::GoalsInOtherUnit { count: other_unit })
}

/// The pace plan the deadline becomes when a writing day is left before it, from `today` on,
/// and what the writer is told either way. A deadline that is today leaves today to write
/// on, unless the deadline itself is a day off. `deadline_day` labels that day off.
fn plan_pace(
    pace: Option<PaceInputs>,
    deadline_blocked: Option<(DeadlineReason, Option<i64>)>,
    today: NaiveDate,
    deadline_day: &str,
) -> (Option<PlannedPace>, Vec<MetadataNotice>) {
    let midnight = |day: NaiveDate| day.and_hms_opt(0, 0, 0).map(|d| d.and_utc().timestamp());
    let mut notices = Vec::new();
    let planned = match (pace, deadline_blocked) {
        (Some(inputs), _) if inputs.end > today || (inputs.end == today && !inputs.day_off) => {
            notices.push(MetadataNotice::PacePlanned {
                day: midnight(inputs.end).unwrap_or_default(),
                day_off: inputs.day_off,
            });
            if let Some(raw) = &inputs.partial_days {
                notices.push(MetadataNotice::WritingDaysNotRead { raw: raw.clone() });
            }
            Some(PlannedPace {
                book: inputs.book,
                start: today,
                end: inputs.end,
                weekday_mask: 127,
                day_off: inputs.day_off.then(|| deadline_day.to_string()),
            })
        }
        (Some(inputs), _) => {
            notices.push(MetadataNotice::DeadlineNotPlanned {
                reason: if inputs.end == today {
                    DeadlineReason::NoWritingDayLeft
                } else {
                    DeadlineReason::Passed
                },
                day: midnight(inputs.end),
            });
            None
        }
        (None, Some((reason, day))) => {
            notices.push(MetadataNotice::DeadlineNotPlanned { reason, day });
            None
        }
        (None, None) => None,
    };
    (planned, notices)
}
