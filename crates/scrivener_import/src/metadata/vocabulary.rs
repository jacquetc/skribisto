// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Labels become tags, the status list becomes the workflow ladder (D4-2, D4-3, D4-4).
//!
//! # What "none" is (D4-2)
//!
//! An item wears no label (no status) when its id is `-1` or absent, when the id names no
//! entry of the list (a dangling id, which the analysis reports), or when it names an entry
//! whose name is the project's own "none" name: the `-1` entry's, compared trimmed, NFC and
//! case-folded. Scrivener 1 kept its "none" as entry `0`, and an upgrade leaves that entry
//! beside the real `-1` one, worn by every item that had no label; the rule needs no list
//! of English words, since it compares against the project's own. Where that upgrade
//! signature is on the labels (entry `0` named like `-1`), status `0` is the upgrade's none
//! too (X-38): the same items wore both, and a rung of Scrivener 1's `N/A` would otherwise
//! mark them all. `0` is otherwise a real value, like any other id.
//!
//! # Labels (D4-3, X-39)
//!
//! One tag per label that at least one row of the new project wears (a row of the Trash
//! counts, since restoring it brings the label back), in the list's menu order, on every
//! row that carries the item: the item's own row, the first child holding a container's
//! own text, each chapter split out of it. Never on the note holding the item's document
//! notes. The name as the writer wrote it (cleaned as titles are), the colour kept exactly
//! (never lightened for contrast: the interface draws a hairline around a pale tag), the
//! field's own title in `details` whatever it is ("Label", "Étiquette", or "Time" in a
//! project that renamed it), never discoverable. A label no row wears is not imported and
//! is listed.
//!
//! # The ladder (D4-4)
//!
//! Every entry of the status list but the none ones, in the list's menu order (ids are not
//! positions), whether or not a row wears it: a project keeps its whole vocabulary, as a
//! new project gets its whole preset. Each rung's kind ([`StatusCategory`]) comes from a
//! table of the names Scrivener ships in the languages measured; any other name takes its
//! place in the ladder (the first to start, the last to finish, the one before it revised,
//! the rest drafting). Every row carrying an item that wears a status gets its rung, when
//! its kind of row may hold one. A project with no status list starts with Skribisto's
//! default ladder, in the writer's language, as a new project does.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use common::entities::StatusCategory;
use unicode_normalization::UnicodeNormalization;

use super::notice::{Field, MetadataNotice};
use crate::convert::{Words, fill};
use crate::model::{Project, Rgb, Vocabulary};
use crate::structure::{Origin, PlannedRow, StructurePlan, clean_title};

/// Colours for the tags a project gives none: away from black and white, since a tag's
/// colour is the same in both themes.
pub(crate) const PALETTE: [&str; 6] = [
    "#2980b9", "#16a085", "#8e44ad", "#c0392b", "#d35400", "#27ae60",
];

/// A tag made from a label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelTag {
    pub name: String,
    /// `#rrggbb`.
    pub color: String,
    pub details: String,
    /// The label's id in the project.
    pub label: i64,
}

/// A rung of the ladder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rung {
    pub name: String,
    pub category: StatusCategory,
    pub details: String,
    /// The status's id in the project; `None` for a rung of the default ladder.
    pub status: Option<i64>,
}

/// The labels and the status list, mapped onto a planned project.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Vocabularies {
    pub tags: Vec<LabelTag>,
    pub ladder: Vec<Rung>,
    /// Per binder (the manuscript's, then the notes'), per row: the label tag it wears.
    pub row_tags: [Vec<Option<u32>>; 2],
    /// Per binder, per row: its rung.
    pub row_rungs: [Vec<Option<u32>>; 2],
    pub notices: Vec<MetadataNotice>,
}

/// What an item's label or status id means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Worn {
    Nothing,
    /// Names no entry: reported by the analysis.
    Dangling,
    /// Names an entry that means none (D4-2), by index into the list.
    Collapsed(usize),
    /// Names a real entry, by index into the list.
    Value(usize),
}

/// The field titles Scrivener ships, in the languages measured: any other is a rename.
const SHIPPED_LABEL_TITLES: &[&str] = &["label", "étiquette", "etiqueta", "etiquetas", "标签"];
const SHIPPED_STATUS_TITLES: &[&str] = &["status", "statut", "estado", "状态"];

/// The rung names Scrivener ships, in the languages the corpus holds (English, French,
/// Spanish, Portuguese, Chinese), with the kind each is (D4-4). Folded as [`folded`] folds.
const SHIPPED_RUNGS: &[(&str, StatusCategory)] = &[
    ("to do", StatusCategory::Planned),
    ("à faire", StatusCategory::Planned),
    ("cosas pendientes", StatusCategory::Planned),
    ("1 tarefa", StatusCategory::Planned),
    ("待办事项", StatusCategory::Planned),
    ("n/a", StatusCategory::Planned),
    ("in progress", StatusCategory::Drafting),
    ("en cours", StatusCategory::Drafting),
    ("2 em andamento", StatusCategory::Drafting),
    ("正在进行", StatusCategory::Drafting),
    ("first draft", StatusCategory::Drafting),
    ("rough draft", StatusCategory::Drafting),
    ("draft", StatusCategory::Drafting),
    ("première ébauche", StatusCategory::Drafting),
    ("primer borrador", StatusCategory::Drafting),
    ("3 primeiro rascunho", StatusCategory::Drafting),
    ("首个草稿", StatusCategory::Drafting),
    ("needs research", StatusCategory::NeedsWork),
    ("revised draft", StatusCategory::Revised),
    ("ébauche révisée", StatusCategory::Revised),
    ("borrador revisado", StatusCategory::Revised),
    ("4 rascunho revisado", StatusCategory::Revised),
    ("已修订草稿", StatusCategory::Revised),
    ("final draft", StatusCategory::Final),
    ("ébauche finale", StatusCategory::Final),
    ("borrador final", StatusCategory::Final),
    ("5 versão para submissão", StatusCategory::Final),
    ("最终草稿", StatusCategory::Final),
    ("done", StatusCategory::Final),
    ("terminé", StatusCategory::Final),
    ("hecho", StatusCategory::Final),
    ("ok", StatusCategory::Final),
    ("6 submetido e finalizado", StatusCategory::Final),
    ("完成", StatusCategory::Final),
    ("title page", StatusCategory::Final),
    ("cover", StatusCategory::Final),
];

/// A name as names are compared: NFC, case-folded, white space collapsed.
pub(crate) fn folded(name: &str) -> String {
    let nfc: String = name.nfc().collect();
    nfc.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// A colour as a tag stores it: `#rrggbb`, each channel rounded, never adjusted.
pub(crate) fn hex(colour: Rgb) -> String {
    let channel = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        channel(colour.r),
        channel(colour.g),
        channel(colour.b)
    )
}

/// The kind of a rung Scrivener ships, by its name.
pub(crate) fn shipped_category(name: &str) -> Option<StatusCategory> {
    let key = folded(name);
    SHIPPED_RUNGS
        .iter()
        .find(|(shipped, _)| *shipped == key)
        .map(|(_, category)| category.clone())
}

/// The kind a rung takes from its place in a ladder of `count`: the first starts, the last
/// finishes, the one before it is revision, the rest drafting; a ladder of one drafts.
pub(crate) fn positional_category(rank: usize, count: usize) -> StatusCategory {
    if count <= 1 {
        StatusCategory::Drafting
    } else if rank == 0 {
        StatusCategory::Planned
    } else if rank + 1 == count {
        StatusCategory::Final
    } else if rank + 2 == count {
        StatusCategory::Revised
    } else {
        StatusCategory::Drafting
    }
}

/// The item a row carries the labels and status of: its own row, a container's own text,
/// a chapter split out of it, the Draft's own writing. Never the note of its document notes
/// nor what a template could not hold.
pub(crate) fn item_of(origin: &Origin) -> Option<u32> {
    match origin {
        Origin::Item { item }
        | Origin::OwnText { item }
        | Origin::SplitPart { item, .. }
        | Origin::DraftContent { item } => Some(*item),
        Origin::DocumentNotes { .. }
        | Origin::TemplateLeftover { .. }
        | Origin::Book
        | Origin::BookEnd
        | Origin::ProjectNotes
        | Origin::ProjectNote { .. }
        | Origin::Recovered
        | Origin::RecoveredText { .. }
        | Origin::FromScrivener
        | Origin::FromScrivenerNote { .. } => None,
    }
}

/// The folded name of a list's "none" entry (`-1`), when it has one with a name.
fn none_name(list: &Vocabulary) -> Option<String> {
    list.entries
        .iter()
        .find(|e| e.id == -1)
        .map(|e| folded(&e.name))
        .filter(|n| !n.is_empty())
}

/// A list read once for the lookups every item makes: each id's entry, and whether the
/// entry means none. An item then finds what it wears without searching the list, so a
/// project's cost stays linear in its items and its entries together.
struct Lookup {
    /// Each id's entry, the first one listed when two share an id.
    entry_of: HashMap<i64, usize>,
    /// Per entry: it is named like the list's none, or has an id that means none.
    none: Vec<bool>,
}

impl Lookup {
    /// `none_ids` are ids that mean none whatever they are named.
    fn new(list: &Vocabulary, none_name: Option<&str>, none_ids: &[i64]) -> Lookup {
        let mut entry_of = HashMap::with_capacity(list.entries.len());
        for (index, entry) in list.entries.iter().enumerate() {
            entry_of.entry(entry.id).or_insert(index);
        }
        let none = list
            .entries
            .iter()
            .map(|e| {
                none_ids.contains(&e.id) || none_name.is_some_and(|none| folded(&e.name) == none)
            })
            .collect();
        Lookup { entry_of, none }
    }

    /// What `id` means.
    fn worn(&self, id: Option<i64>) -> Worn {
        let Some(id) = id.filter(|id| *id != -1) else {
            return Worn::Nothing;
        };
        match self.entry_of.get(&id) {
            None => Worn::Dangling,
            Some(&index) if self.none[index] => Worn::Collapsed(index),
            Some(&index) => Worn::Value(index),
        }
    }
}

/// Whether a field's title is not one Scrivener ships, and so was renamed.
fn renamed(title: Option<&str>, shipped: &[&str]) -> Option<String> {
    let title = clean_title(title).0;
    (!title.is_empty() && !shipped.contains(&folded(&title).as_str())).then_some(title)
}

/// Map the labels and the status list of `project` onto the rows `plan` makes, naming what
/// the import adds with `words`. `bible_tags` are the story-bible tags' names as written,
/// which a label tag's name must not take (D4-9).
pub fn vocabularies(
    project: &Project,
    plan: &StructurePlan,
    bible_tags: &[String],
    words: &Words,
) -> Vocabularies {
    let mut out = Vocabularies::default();
    let binders: [&[PlannedRow]; 2] = [&plan.manuscript, &plan.notes];

    // What each item wears.
    let label_none = none_name(&project.labels);
    let status_none = none_name(&project.statuses);
    // The Scrivener 1 upgrade signature: label 0 named like the labels' none (X-38).
    let upgraded = project
        .labels
        .entries
        .iter()
        .any(|e| e.id == 0 && label_none.as_deref() == Some(folded(&e.name).as_str()));
    let status_none_ids: &[i64] = if upgraded { &[0] } else { &[] };
    let label_lookup = Lookup::new(&project.labels, label_none.as_deref(), &[]);
    let status_lookup = Lookup::new(&project.statuses, status_none.as_deref(), status_none_ids);
    let labels: Vec<Worn> = project
        .binder
        .iter()
        .map(|row| label_lookup.worn(row.meta.label))
        .collect();
    let statuses: Vec<Worn> = project
        .binder
        .iter()
        .map(|row| status_lookup.worn(row.meta.status))
        .collect();

    // Which items some row carries, and which label entries a row wears.
    let mut carried: HashSet<u32> = HashSet::new();
    let mut worn_labels: BTreeSet<usize> = BTreeSet::new();
    for rows in binders {
        for row in rows {
            if let Some(item) = item_of(&row.origin) {
                carried.insert(item);
                if let Some(Worn::Value(e)) = labels.get(item as usize) {
                    worn_labels.insert(*e);
                }
            }
        }
    }

    // Labels: one tag per worn label, in menu order.
    let label_details = |words: &Words| {
        let title = clean_title(project.labels.title.as_deref()).0;
        if title.is_empty() {
            String::new()
        } else {
            fill(&words.label_details, &title)
        }
    };
    let mut taken: HashSet<String> = bible_tags.iter().map(|n| folded(n)).collect();
    // Per marked name, the ordinal its next clash tries first: each clash resumes where
    // the last one of that name stopped, never counting up from 2 again.
    let mut next_ordinal: HashMap<String, u32> = HashMap::new();
    let mut tag_of_entry: HashMap<usize, u32> = HashMap::new();
    let mut disambiguated = Vec::new();
    for (e, entry) in project.labels.entries.iter().enumerate() {
        if !worn_labels.contains(&e) {
            continue;
        }
        let mut name = clean_title(Some(&entry.name)).0;
        if name.is_empty() {
            name = fill(&words.unnamed_label, &entry.id.to_string());
            out.notices.push(MetadataNotice::UnnamedValue {
                field: Field::Label,
                id: entry.id,
            });
        }
        let original = name.clone();
        if taken.contains(&folded(&name)) {
            let marked = fill(&words.label_tag, &original);
            name = marked.clone();
            if taken.contains(&folded(&name)) {
                let n = next_ordinal.entry(folded(&marked)).or_insert(2);
                loop {
                    name = format!("{marked} ({n})");
                    *n = n.saturating_add(1);
                    if !taken.contains(&folded(&name)) {
                        break;
                    }
                }
            }
            disambiguated.push(name.clone());
        }
        taken.insert(folded(&name));
        let color = entry
            .color
            .map_or_else(|| PALETTE[out.tags.len() % PALETTE.len()].to_string(), hex);
        tag_of_entry.insert(e, out.tags.len() as u32);
        out.tags.push(LabelTag {
            name,
            color,
            details: label_details(words),
            label: entry.id,
        });
    }

    // The ladder: every real entry, in menu order.
    let status_title = clean_title(project.statuses.title.as_deref()).0;
    let status_details = if status_title.is_empty() {
        String::new()
    } else {
        fill(&words.status_details, &status_title)
    };
    let real: Vec<usize> = project
        .statuses
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            e.id != -1
                && !status_none_ids.contains(&e.id)
                && status_none.as_deref() != Some(folded(&e.name).as_str())
        })
        .map(|(i, _)| i)
        .collect();
    let mut rung_of_entry: HashMap<usize, u32> = HashMap::new();
    let mut guessed = Vec::new();
    for (rank, e) in real.iter().enumerate() {
        let entry = &project.statuses.entries[*e];
        let mut name = clean_title(Some(&entry.name)).0;
        if name.is_empty() {
            name = fill(&words.unnamed_status, &entry.id.to_string());
            out.notices.push(MetadataNotice::UnnamedValue {
                field: Field::Status,
                id: entry.id,
            });
        }
        let category = match shipped_category(&name) {
            Some(category) => category,
            None => {
                guessed.push(name.clone());
                positional_category(rank, real.len())
            }
        };
        rung_of_entry.insert(*e, out.ladder.len() as u32);
        out.ladder.push(Rung {
            name,
            category,
            details: status_details.clone(),
            status: Some(entry.id),
        });
    }
    let from_project = !out.ladder.is_empty();
    if !from_project && !words.default_statuses.is_empty() {
        for rung in &words.default_statuses {
            out.ladder.push(Rung {
                name: rung.name.clone(),
                category: rung.category.clone(),
                details: String::new(),
                status: None,
            });
        }
        out.notices.push(MetadataNotice::LadderSeeded {
            rungs: out.ladder.len() as u32,
        });
    }

    // Every row: its label tag and its rung.
    let (mut tagged_rows, mut rung_rows) = (0u32, 0u32);
    let mut rung_items: HashSet<u32> = HashSet::new();
    for (b, rows) in binders.iter().enumerate() {
        out.row_tags[b] = vec![None; rows.len()];
        out.row_rungs[b] = vec![None; rows.len()];
        for (r, row) in rows.iter().enumerate() {
            let Some(item) = item_of(&row.origin) else {
                continue;
            };
            if let Some(Worn::Value(e)) = labels.get(item as usize)
                && let Some(tag) = tag_of_entry.get(e)
            {
                out.row_tags[b][r] = Some(*tag);
                tagged_rows += 1;
            }
            if let Some(Worn::Value(e)) = statuses.get(item as usize)
                && let Some(rung) = rung_of_entry.get(e)
                && skribisto_model::status_capable(&row.role, &row.sub_role)
            {
                out.row_rungs[b][r] = Some(*rung);
                rung_rows += 1;
                rung_items.insert(item);
            }
        }
    }

    // What the writer should know.
    if !out.tags.is_empty() {
        out.notices.push(MetadataNotice::LabelsBecameTags {
            tags: out.tags.len() as u32,
            rows: tagged_rows,
        });
        if let Some(title) = renamed(project.labels.title.as_deref(), SHIPPED_LABEL_TITLES) {
            out.notices.push(MetadataNotice::FieldRenamed {
                field: Field::Label,
                title,
            });
        }
    }
    if from_project {
        out.notices.push(MetadataNotice::StatusesBecameLadder {
            rungs: out.ladder.len() as u32,
            rows: rung_rows,
        });
        if let Some(title) = renamed(project.statuses.title.as_deref(), SHIPPED_STATUS_TITLES) {
            out.notices.push(MetadataNotice::FieldRenamed {
                field: Field::Status,
                title,
            });
        }
    }
    if !guessed.is_empty() {
        out.notices
            .push(MetadataNotice::StatusCategoriesGuessed { rungs: guessed });
    }
    for (field, list, worn) in [
        (Field::Label, &project.labels, &labels),
        (Field::Status, &project.statuses, &statuses),
    ] {
        let mut collapsed: BTreeMap<usize, u32> = BTreeMap::new();
        for w in worn {
            if let Worn::Collapsed(e) = w {
                *collapsed.entry(*e).or_default() += 1;
            }
        }
        for (e, items) in collapsed {
            out.notices.push(MetadataNotice::NoneValueCollapsed {
                field,
                name: list.entries[e].name.clone(),
                items,
            });
        }
    }
    if !disambiguated.is_empty() {
        out.notices.push(MetadataNotice::TagNameDisambiguated {
            names: disambiguated,
        });
    }
    // Labels worn by no document at all; a label worn only where no row goes is said below.
    let worn_anywhere: BTreeSet<usize> = labels
        .iter()
        .filter_map(|w| match w {
            Worn::Value(e) => Some(*e),
            _ => None,
        })
        .collect();
    let unused: Vec<String> = project
        .labels
        .entries
        .iter()
        .enumerate()
        .filter(|(e, entry)| {
            entry.id != -1
                && !worn_anywhere.contains(e)
                && label_none.as_deref() != Some(folded(&entry.name).as_str())
        })
        .map(|(_, entry)| {
            let name = clean_title(Some(&entry.name)).0;
            if name.is_empty() {
                fill(&words.unnamed_label, &entry.id.to_string())
            } else {
                name
            }
        })
        .collect();
    if !unused.is_empty() {
        out.notices
            .push(MetadataNotice::UnusedLabelsSkipped { names: unused });
    }
    let label_orphans = labels
        .iter()
        .enumerate()
        .filter(|(i, w)| matches!(w, Worn::Value(_)) && !carried.contains(&(*i as u32)))
        .count() as u32;
    if label_orphans > 0 {
        out.notices.push(MetadataNotice::ValuesWithoutRow {
            field: Field::Label,
            items: label_orphans,
        });
    }
    let status_orphans = statuses
        .iter()
        .enumerate()
        .filter(|(i, w)| matches!(w, Worn::Value(_)) && !rung_items.contains(&(*i as u32)))
        .count() as u32;
    if status_orphans > 0 {
        out.notices.push(MetadataNotice::ValuesWithoutRow {
            field: Field::Status,
            items: status_orphans,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_colour_is_kept_exactly() {
        let hex_of = |r, g, b| hex(Rgb { r, g, b });
        assert_eq!(hex_of(0.993_5, 0.701_213, 0.732_586), "#fdb3bb");
        assert_eq!(hex_of(1.0, 1.0, 1.0), "#ffffff", "white stays white");
        assert_eq!(hex_of(0.0, 0.0, 0.0), "#000000");
        assert_eq!(hex_of(1.2, -0.5, 0.5), "#ff0080", "clamped, rounded");
        assert_eq!(
            crate::manifest::rgb("0.5,0.25,1").map(hex).as_deref(),
            Some("#8040ff"),
            "comma separated"
        );
        assert_eq!(
            crate::manifest::rgb("0,5 0,25 1").map(hex).as_deref(),
            Some("#8040ff"),
            "a decimal comma"
        );
        assert_eq!(
            crate::manifest::rgb("1 1 1").map(hex).as_deref(),
            Some("#ffffff"),
            "whole numbers"
        );
        assert_eq!(
            crate::manifest::rgb("0.5, 0.25, 1").map(hex).as_deref(),
            Some("#8040ff"),
            "a comma and a space"
        );
        assert_eq!(
            crate::manifest::rgb("1, 0, 0.5").map(hex).as_deref(),
            Some("#ff0080"),
            "a comma and a space, a decimal point in one number only"
        );
        assert_eq!(crate::manifest::rgb("0.5 0.5"), None);
        assert_eq!(crate::manifest::rgb("red"), None);
        assert_eq!(crate::manifest::rgb("NaN 0 0"), None);
    }

    #[test]
    fn a_shipped_rung_knows_its_kind_in_every_language_measured() {
        for (name, category) in [
            ("To Do", StatusCategory::Planned),
            ("À faire", StatusCategory::Planned),
            ("N/A", StatusCategory::Planned),
            ("First Draft", StatusCategory::Drafting),
            ("Première ébauche", StatusCategory::Drafting),
            ("3 Primeiro rascunho", StatusCategory::Drafting),
            ("首个草稿", StatusCategory::Drafting),
            ("Needs research", StatusCategory::NeedsWork),
            ("Revised Draft", StatusCategory::Revised),
            ("Borrador revisado", StatusCategory::Revised),
            ("Final Draft", StatusCategory::Final),
            ("Title Page", StatusCategory::Final),
            ("Terminé", StatusCategory::Final),
            ("完成", StatusCategory::Final),
            ("  done ", StatusCategory::Final),
        ] {
            assert_eq!(shipped_category(name), Some(category), "{name}");
        }
        assert_eq!(shipped_category("★★★"), None);
        assert_eq!(shipped_category("Relecture 1"), None);
    }

    #[test]
    fn an_unknown_rung_takes_its_place_in_the_ladder() {
        let kinds: Vec<StatusCategory> = (0..5).map(|r| positional_category(r, 5)).collect();
        assert_eq!(
            kinds,
            [
                StatusCategory::Planned,
                StatusCategory::Drafting,
                StatusCategory::Drafting,
                StatusCategory::Revised,
                StatusCategory::Final
            ]
        );
        assert_eq!(positional_category(0, 1), StatusCategory::Drafting);
        assert_eq!(positional_category(0, 2), StatusCategory::Planned);
        assert_eq!(positional_category(1, 2), StatusCategory::Final);
    }
}
