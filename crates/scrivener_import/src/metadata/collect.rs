// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the second half of the metadata mapping decides, before a word is chosen: which
//! keywords, collections and field options become tags and on which rows, which targets
//! become goals, which bookmarks become links, what the dictionary takes, whether the
//! deadline can become a pace plan, and what the "From Scrivener" folder lists.
//!
//! [`collect()`] is pure, word-free and clock-free, so the structure plan can ask it which
//! notes the folder will hold ([`super::import_note_sections`]) and the review and the
//! import agree on them by construction. The converter's [`super::extended()`] names and
//! words what it decided.
//!
//! Every rule names its decision in the design (D4-n, X-n). Every walk is a loop over flat
//! lists, linear in the binder and the plan: the review recomputes this on every choice.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use chrono::NaiveDate;
use common::entities::GoalUnit;

use super::notice::{DeadlineReason, DraftTargetReason, Field, MetadataNotice, TargetUnit};
use super::vocabulary::{folded, item_of};
use crate::model::{
    CompileReplacement, ItemKey, KeywordRow, Link, Project, RawTarget, Rgb, parents_of,
};
use crate::structure::{
    BinderKind, NoteSection, Origin, PlannedRow, Role, RowRef, StructurePlan, SubRole, clean_title,
};

/// What a tag is made from, in the order names are claimed (D4-9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TagSource {
    Keyword,
    Collection,
    Field,
}

/// A tag decided, not yet named: the writer's words it is named from.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RawTag {
    pub(crate) source: TagSource,
    /// A keyword's or a collection's own name, cleaned; a checkbox field's name; empty when
    /// it has none.
    pub(crate) name: String,
    /// A list field's option, cleaned: the tag is named `<field>: <option>`.
    pub(crate) option: Option<String>,
    /// A keyword's number, to name one that has no name.
    pub(crate) id: i64,
    /// A keyword's parent's name, to tell two keywords of one name apart.
    pub(crate) parent: Option<String>,
    /// A keyword's place in the keyword list (`Characters › Ysenn`); a field's name.
    pub(crate) path: String,
    pub(crate) color: Option<Rgb>,
}

/// A goal one row takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Goal {
    pub(crate) row: RowRef,
    pub(crate) value: i64,
    pub(crate) unit: GoalUnit,
}

/// A line of the project's note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProjectLine {
    AbbreviatedTitle(String),
    Subjects(Vec<String>),
    Company(String),
    SessionTarget {
        value: i64,
        unit: TargetUnit,
    },
    /// The writing days, as written, when they are some days only.
    WritingDays(String),
    /// The draft target, when no book's goal took it.
    DraftTarget {
        value: i64,
        unit: TargetUnit,
    },
    /// The deadline in force: its calendar day (`YYYY-MM-DD`), or the text as written when
    /// it could not be read.
    Deadline(String),
}

/// Where a bookmark points, as a line says it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LinkLine {
    /// A document of the project, by its path in Scrivener's binder; a format 1.x
    /// reference's description, when it has one.
    Item { path: String, note: Option<String> },
    /// Somewhere outside the project, as written; a file on the writer's disk by its name
    /// alone.
    External {
        label: Option<String>,
        url: String,
        local_file: bool,
    },
}

/// One document's entry in a listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Entry<T> {
    Listed {
        path: String,
        value: T,
    },
    /// The document is a note: its own text lists it.
    InOwnNote {
        path: String,
    },
}

/// One field's values, in the custom metadata note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldListing {
    pub(crate) title: String,
    pub(crate) entries: Vec<Entry<String>>,
}

/// A target no goal can measure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TargetLine {
    pub(crate) path: String,
    pub(crate) value: i64,
    pub(crate) unit: TargetUnit,
}

/// A saved search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SearchLine {
    pub(crate) title: String,
    pub(crate) query: String,
    pub(crate) kind: Option<String>,
    pub(crate) operator: Option<String>,
    /// The fields a search of custom metadata reads, by their names.
    pub(crate) fields: Vec<String>,
}

/// What the "From Scrivener" folder lists, note by note.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Listing {
    pub(crate) project: Vec<ProjectLine>,
    pub(crate) project_links: Vec<LinkLine>,
    pub(crate) replacements: Vec<CompileReplacement>,
    pub(crate) completions: Vec<String>,
    pub(crate) searches: Vec<SearchLine>,
    pub(crate) fields: Vec<FieldListing>,
    pub(crate) targets: Vec<TargetLine>,
    pub(crate) bookmarks: Vec<Entry<Vec<LinkLine>>>,
}

fn listed<T>(entries: &[Entry<T>]) -> u32 {
    entries
        .iter()
        .filter(|e| matches!(e, Entry::Listed { .. }))
        .count() as u32
}

impl Listing {
    /// How many things the note for `section` lists.
    pub(crate) fn count(&self, section: NoteSection) -> u32 {
        match section {
            NoteSection::Project => self.project.len() as u32,
            NoteSection::ProjectBookmarks => self.project_links.len() as u32,
            NoteSection::Replacements => self.replacements.len() as u32,
            NoteSection::AutoComplete => self.completions.len() as u32,
            NoteSection::SavedSearches => self.searches.len() as u32,
            NoteSection::CustomMetadata => self.fields.iter().map(|f| listed(&f.entries)).sum(),
            NoteSection::Targets => self.targets.len() as u32,
            NoteSection::Bookmarks => self
                .bookmarks
                .iter()
                .map(|e| match e {
                    Entry::Listed { value, .. } => value.len() as u32,
                    Entry::InOwnNote { .. } => 0,
                })
                .sum(),
        }
    }

    /// The notes the folder holds: those listing something. A note that would only point
    /// at documents listing their own values says nothing the documents do not.
    pub(crate) fn sections(&self) -> Vec<NoteSection> {
        NoteSection::ALL
            .into_iter()
            .filter(|s| self.count(*s) > 0)
            .collect()
    }
}

/// What a note-like row lists of its own: its custom metadata values and its bookmarks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SheetExtra {
    /// Field name, value; in the fields' order.
    pub(crate) fields: Vec<(String, String)>,
    pub(crate) links: Vec<LinkLine>,
}

/// The deadline, when Scrivener's calculator would plan towards it and a plan can be made
/// from it: the decision whether its day is still ahead is the converter's, which knows
/// the day of the import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PaceInputs {
    pub(crate) book: RowRef,
    pub(crate) end: NaiveDate,
    /// No writing scheduled on the deadline itself (`CanWriteOnDeadlineDate="No"`).
    pub(crate) day_off: bool,
    /// The writing days, as written, when they are some days only (the plan counts every
    /// day, said).
    pub(crate) partial_days: Option<String>,
}

/// Everything decided.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Collected {
    pub(crate) tags: Vec<RawTag>,
    /// Per binder (the manuscript's, the notes'), per row: the tags it wears.
    pub(crate) row_tags: [Vec<Vec<u32>>; 2],
    pub(crate) goals: Vec<Goal>,
    pub(crate) book_goal: Option<Goal>,
    /// The unit the project counts in, when its targets decide it; `None` leaves it to the
    /// language (X-30).
    pub(crate) unit: Option<GoalUnit>,
    /// References to write, owner row to target row (X-35), each once.
    pub(crate) edges: Vec<(RowRef, RowRef)>,
    pub(crate) dictionary: Vec<String>,
    pub(crate) pace: Option<PaceInputs>,
    /// Why the deadline in force makes no plan, before the day is known.
    pub(crate) deadline_blocked: Option<(DeadlineReason, Option<i64>)>,
    pub(crate) listing: Listing,
    /// Per note-like row: what its own text lists (D4-8, D4-20).
    pub(crate) sheets: HashMap<RowRef, SheetExtra>,
    pub(crate) notices: Vec<MetadataNotice>,
}

/// The words a field's title is when it names a point of view, in the languages the
/// corpus's projects are written in. A hint only: a match writes nothing (D4-6).
const POV_TITLES: &[&str] = &[
    "pov",
    "point of view",
    "viewpoint",
    "point de vue",
    "punto de vista",
    "ponto de vista",
    "视角",
];

/// The plan's rows of one binder.
fn rows_of(plan: &StructurePlan, binder: BinderKind) -> &[PlannedRow] {
    match binder {
        BinderKind::Manuscript => &plan.manuscript,
        BinderKind::Notes => &plan.notes,
    }
}

fn binder_index(binder: BinderKind) -> usize {
    match binder {
        BinderKind::Manuscript => 0,
        BinderKind::Notes => 1,
    }
}

/// The most characters of one title a path writes: a longer one is cut, with an ellipsis.
/// Scrivener bounds no title, and a path repeats every title above an item for every item
/// under it.
const MAX_SEGMENT_CHARS: usize = 200;

/// The most characters a whole path writes: past it, the titles nearest the top give way
/// to an ellipsis, so the path still ends with the item it names.
const MAX_PATH_CHARS: usize = 1_000;

/// `title` as a path writes it: its first [`MAX_SEGMENT_CHARS`] characters, an ellipsis
/// after them when it is longer.
fn segment(title: &str) -> String {
    match title.char_indices().nth(MAX_SEGMENT_CHARS) {
        Some((cut, _)) => format!("{}…", &title[..cut]),
        None => title.to_string(),
    }
}

/// The path to `at` in a tree given by `parents`, each step written as `titles` has it,
/// from the top down and bounded to [`MAX_PATH_CHARS`]: the walk up stops once the path is
/// full, so a path costs no more than its bound whatever the depth.
fn bounded_path(parents: &[Option<u32>], titles: &[String], at: u32) -> String {
    const JOINT: &str = " › ";
    let mut steps: Vec<&str> = Vec::new();
    let mut length = 0usize;
    let mut cut = false;
    let mut next = Some(at);
    while let Some(i) = next {
        let title = titles.get(i as usize).map_or("", String::as_str);
        let cost = title.chars().count() + if steps.is_empty() { 0 } else { 3 };
        if !steps.is_empty() && length + cost > MAX_PATH_CHARS {
            cut = true;
            break;
        }
        steps.push(title);
        length += cost;
        next = parents.get(i as usize).copied().flatten();
    }
    if cut {
        steps.push("…");
    }
    steps.reverse();
    steps.join(JOINT)
}

/// What the walks share: the plan's rows by item, the items by key, the paths the writer
/// knew them by.
struct Ctx<'a> {
    project: &'a Project,
    plan: &'a StructurePlan,
    /// Per item: every row carrying it (its own, a container's own text, a split chapter).
    carriers: Vec<Vec<RowRef>>,
    /// Per item: the item it sits in, in Scrivener's binder.
    parents: Vec<Option<u32>>,
    /// Per item: its own title as a path writes it ([`segment`]).
    titles: Vec<String>,
    /// Per item: 1, or which holder of a path it is when items before it have the same
    /// one; a line adds the ordinal after the path.
    ordinals: Vec<u32>,
    by_key: HashMap<ItemKey, u32>,
}

impl<'a> Ctx<'a> {
    fn new(project: &'a Project, plan: &'a StructurePlan) -> Self {
        let n = project.binder.len();
        let mut carriers: Vec<Vec<RowRef>> = vec![Vec::new(); n];
        for binder in [BinderKind::Manuscript, BinderKind::Notes] {
            for (r, row) in rows_of(plan, binder).iter().enumerate() {
                if let Some(item) = item_of(&row.origin)
                    && let Some(list) = carriers.get_mut(item as usize)
                {
                    list.push(RowRef {
                        binder,
                        row: r as u32,
                    });
                }
            }
        }
        // Each item's path is its parent's and its own title, never written out here: a
        // path is built when a line names it. Two items with the same path are told apart
        // by an ordinal, found by naming each path once, as its parent's and its title.
        let parents: Vec<Option<u32>> = parents_of(project.binder.iter().map(|r| r.depth))
            .into_iter()
            .map(|p| p.map(|p| p as u32))
            .collect();
        let titles: Vec<String> = project
            .binder
            .iter()
            .map(|row| segment(&clean_title(row.title.as_deref()).0))
            .collect();
        let ordinals = {
            let mut path_of: Vec<u32> = Vec::with_capacity(n);
            let mut named: HashMap<(Option<u32>, &str), u32> = HashMap::new();
            let mut holders: Vec<u32> = Vec::new();
            let mut ordinals = Vec::with_capacity(n);
            for (i, title) in titles.iter().enumerate() {
                let above = parents[i].and_then(|p| path_of.get(p as usize).copied());
                let next = named.len() as u32;
                let path = *named.entry((above, title.as_str())).or_insert(next);
                if path as usize == holders.len() {
                    holders.push(0);
                }
                holders[path as usize] += 1;
                ordinals.push(holders[path as usize]);
                path_of.push(path);
            }
            ordinals
        };
        let mut by_key = HashMap::new();
        for (i, row) in project.binder.iter().enumerate() {
            if row.owns_key
                && let Some(key) = row.key
            {
                by_key.entry(key).or_insert(i as u32);
            }
        }
        Ctx {
            project,
            plan,
            carriers,
            parents,
            titles,
            ordinals,
            by_key,
        }
    }

    /// Whether `item` is a document split at its headings: its writing spread over rows
    /// side by side, none of which measures the whole of it.
    fn split(&self, item: u32) -> bool {
        self.carriers
            .get(item as usize)
            .into_iter()
            .flatten()
            .any(|at| {
                self.row(*at)
                    .is_some_and(|row| matches!(row.origin, Origin::SplitPart { .. }))
            })
    }

    fn carried(&self, item: u32) -> bool {
        self.carriers
            .get(item as usize)
            .is_some_and(|rows| !rows.is_empty())
    }

    /// `item`'s path in Scrivener's binder, as a line says it ([`bounded_path`]), with its
    /// ordinal after a path an item before it already has.
    fn path(&self, item: u32) -> String {
        if item as usize >= self.titles.len() {
            return String::new();
        }
        let path = bounded_path(&self.parents, &self.titles, item);
        match self.ordinals.get(item as usize).copied().unwrap_or(1) {
            0 | 1 => path,
            n => format!("{path} ({n})"),
        }
    }

    fn row(&self, at: RowRef) -> Option<&'a PlannedRow> {
        rows_of(self.plan, at.binder).get(at.row as usize)
    }

    /// The row representing `item`: the one its metadata goes to (D3.21).
    fn item_row(&self, item: u32) -> Option<RowRef> {
        self.plan
            .placements
            .get(item as usize)
            .and_then(|p| p.item)
            .or_else(|| {
                self.carriers
                    .get(item as usize)
                    .and_then(|rows| rows.first().copied())
            })
    }

    /// The row holding `item`'s own writing, else the row representing it: where its
    /// targets and its listings go (D4-1, D4-8 critique).
    fn home_row(&self, item: u32) -> Option<RowRef> {
        self.plan
            .placements
            .get(item as usize)
            .and_then(|p| p.text.as_ref().map(|(at, _)| *at))
            .or_else(|| self.item_row(item))
    }

    /// The home row, when it is a note that can hold text of its own: a character sheet,
    /// a research note. What the item lists goes into that note rather than the folder.
    fn sheet(&self, item: u32) -> Option<RowRef> {
        let at = self.home_row(item)?;
        let row = self.row(at)?;
        (row.role == Role::Item && row.sub_role == SubRole::Note).then_some(at)
    }
}

/// Decide everything the second half of the metadata mapping does over `plan`.
pub(crate) fn collect(project: &Project, plan: &StructurePlan) -> Collected {
    let ctx = Ctx::new(project, plan);
    let mut out = Collected {
        row_tags: [
            vec![Vec::new(); plan.manuscript.len()],
            vec![Vec::new(); plan.notes.len()],
        ],
        ..Collected::default()
    };
    keywords(&ctx, &mut out);
    collections(&ctx, &mut out);
    fields(&ctx, &mut out);
    bookmarks(&ctx, &mut out);
    targets(&ctx, &mut out);
    project_lines(&ctx, &mut out);
    completions(&ctx, &mut out);
    out.listing.replacements = project
        .compile
        .replacements
        .iter()
        .filter(|r| !r.replace.is_empty())
        .cloned()
        .collect();
    pov_hint(&ctx, &mut out);
    icons_and_script(&ctx, &mut out);
    for section in out.listing.sections() {
        out.notices.push(MetadataNotice::ListedInImportNote {
            section,
            count: out.listing.count(section),
        });
    }
    out
}

/// Put tag `tag` on every row carrying `item`, once.
fn wear(ctx: &Ctx<'_>, out: &mut Collected, item: u32, tag: u32) -> u32 {
    let mut rows = 0;
    for at in ctx.carriers.get(item as usize).into_iter().flatten() {
        let list = &mut out.row_tags[binder_index(at.binder)][at.row as usize];
        if !list.contains(&tag) {
            list.push(tag);
            rows += 1;
        }
    }
    rows
}

// ---------------------------------------------------------------- keywords (D4-7)

fn keywords(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    let rows: &[KeywordRow] = &project.keywords;
    if rows.is_empty() {
        return;
    }
    let parents = parents_of(rows.iter().map(|k| k.depth));
    let mut index_of: HashMap<i64, usize> = HashMap::new();
    for (k, row) in rows.iter().enumerate() {
        index_of.entry(row.id).or_insert(k);
    }
    // Each keyword's name once, and as a path writes it: a name is not bounded, and a
    // path or a parent's name repeats it for every keyword under it.
    let names: Vec<String> = rows
        .iter()
        .map(|k| clean_title(k.title.as_deref()).0)
        .collect();
    let steps: Vec<String> = names.iter().map(|n| segment(n)).collect();
    let keyword_parents: Vec<Option<u32>> = parents.iter().map(|p| p.map(|p| p as u32)).collect();
    let name = |k: usize| names[k].clone();
    // Who wears what, and which of the wearers have rows.
    let mut worn_anywhere: BTreeSet<usize> = BTreeSet::new();
    let mut used: BTreeSet<usize> = BTreeSet::new();
    let mut without_row = 0u32;
    let mut worn_by: Vec<Vec<usize>> = vec![Vec::new(); project.binder.len()];
    for (item, row) in project.binder.iter().enumerate() {
        let mut mine: Vec<usize> = Vec::new();
        for id in &row.keyword_ids {
            if let Some(&k) = index_of.get(id)
                && !mine.contains(&k)
            {
                mine.push(k);
            }
        }
        if mine.is_empty() {
            continue;
        }
        worn_anywhere.extend(mine.iter().copied());
        if ctx.carried(item as u32) {
            used.extend(mine.iter().copied());
        } else {
            without_row += 1;
        }
        worn_by[item] = mine;
    }
    // One tag per keyword a row wears, in the list's order.
    let mut tag_of: HashMap<usize, u32> = HashMap::new();
    let mut nested = 0u32;
    for &k in &used {
        let path = bounded_path(&keyword_parents, &steps, k as u32);
        let parent = parents[k].map(|p| steps[p].clone());
        if parent.is_some() {
            nested += 1;
        }
        let own = name(k);
        if own.is_empty() {
            out.notices.push(MetadataNotice::UnnamedValue {
                field: Field::Keyword,
                id: rows[k].id,
            });
        }
        tag_of.insert(k, out.tags.len() as u32);
        out.tags.push(RawTag {
            source: TagSource::Keyword,
            name: own,
            option: None,
            id: rows[k].id,
            parent,
            path,
            color: rows[k].color,
        });
    }
    let mut tagged_rows = 0;
    for (item, mine) in worn_by.iter().enumerate() {
        for k in mine {
            if let Some(&tag) = tag_of.get(k) {
                tagged_rows += wear(ctx, out, item as u32, tag);
            }
        }
    }
    if !tag_of.is_empty() {
        out.notices.push(MetadataNotice::KeywordsBecameTags {
            tags: tag_of.len() as u32,
            rows: tagged_rows,
        });
    }
    if nested > 0 {
        out.notices
            .push(MetadataNotice::KeywordHierarchyFlattened { keywords: nested });
    }
    // Keywords no document wears, groups holding a worn one aside.
    let mut holds_worn = vec![false; rows.len()];
    for &k in &worn_anywhere {
        let mut at = parents[k];
        while let Some(p) = at {
            if holds_worn[p] {
                break;
            }
            holds_worn[p] = true;
            at = parents[p];
        }
    }
    let unused: Vec<String> = (0..rows.len())
        .filter(|k| !worn_anywhere.contains(k) && !holds_worn[*k])
        .map(name)
        .filter(|n| !n.is_empty())
        .collect();
    if !unused.is_empty() {
        out.notices
            .push(MetadataNotice::UnusedKeywordsSkipped { names: unused });
    }
    if without_row > 0 {
        out.notices.push(MetadataNotice::ValuesWithoutRow {
            field: Field::Keyword,
            items: without_row,
        });
    }
}

// ---------------------------------------------------------------- collections (D4-13)

fn collections(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    let mut tags = 0u32;
    let mut tagged_rows = 0u32;
    let mut order_lost = Vec::new();
    let mut empty = Vec::new();
    for collection in &project.collections {
        let title = clean_title(collection.title.as_deref()).0;
        match collection.kind.as_deref().map(str::trim) {
            // The binder and the last search: what Scrivener's window showed.
            Some("Binder") | Some("RecentSearch") => continue,
            Some("SavedSearch") => {
                let search = collection.search.clone().unwrap_or_default();
                let field_name = |id: &str| {
                    project
                        .custom_fields
                        .iter()
                        .find(|f| f.id == id)
                        .map(|f| clean_title(f.title.as_deref()).0)
                        .filter(|t| !t.is_empty())
                        .unwrap_or_else(|| id.to_string())
                };
                out.listing.searches.push(SearchLine {
                    title,
                    query: search.query,
                    kind: search.kind,
                    operator: search.operator,
                    fields: search.fields.iter().map(|id| field_name(id)).collect(),
                });
                continue;
            }
            _ => {}
        }
        let mut members: Vec<u32> = Vec::new();
        let mut missing = 0u32;
        let mut seen: HashSet<ItemKey> = HashSet::new();
        for key in &collection.members {
            if !seen.insert(*key) {
                continue;
            }
            match ctx.by_key.get(key) {
                Some(&item) if ctx.carried(item) => members.push(item),
                _ => missing += 1,
            }
        }
        if collection.members.is_empty() {
            empty.push(title);
            continue;
        }
        if missing > 0 {
            out.notices.push(MetadataNotice::CollectionMembersMissing {
                name: title.clone(),
                count: missing,
            });
        }
        if members.is_empty() {
            continue;
        }
        let tag = out.tags.len() as u32;
        out.tags.push(RawTag {
            source: TagSource::Collection,
            name: title.clone(),
            option: None,
            id: 0,
            parent: None,
            path: String::new(),
            color: collection.color,
        });
        tags += 1;
        for item in &members {
            tagged_rows += wear(ctx, out, *item, tag);
        }
        if members.len() >= 2 {
            order_lost.push(title);
        }
    }
    if tags > 0 {
        out.notices.push(MetadataNotice::CollectionsBecameTags {
            tags,
            rows: tagged_rows,
        });
    }
    if !order_lost.is_empty() {
        out.notices
            .push(MetadataNotice::CollectionOrderLost { names: order_lost });
    }
    if !empty.is_empty() {
        out.notices
            .push(MetadataNotice::EmptyCollectionsSkipped { names: empty });
    }
}

// ---------------------------------------------------------------- custom metadata (D4-8)

/// What a field definition is, for the mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldKind {
    List,
    Checkbox,
    /// Text, a date (kept as written), and every format 1.x field.
    Text,
}

fn field_kind(kind: Option<&str>) -> FieldKind {
    match kind.map(str::trim) {
        Some("List") => FieldKind::List,
        Some("Checkbox") => FieldKind::Checkbox,
        _ => FieldKind::Text,
    }
}

fn fields(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    let defs = &project.custom_fields;
    let mut def_of: HashMap<&str, usize> = HashMap::new();
    for (d, def) in defs.iter().enumerate() {
        def_of.entry(def.id.as_str()).or_insert(d);
    }
    // Each field's name once, bounded as a path's step is: a sheet repeats it for every
    // document holding a value, and a tag for every option.
    let titles: Vec<String> = defs
        .iter()
        .map(|def| {
            let title = clean_title(def.title.as_deref()).0;
            segment(if title.is_empty() { &def.id } else { &title })
        })
        .collect();
    let title_of = |d: usize| titles[d].clone();
    for (d, def) in defs.iter().enumerate() {
        if field_kind(def.kind.as_deref()) == FieldKind::Checkbox
            && !def.other_attributes.is_empty()
        {
            out.notices
                .push(MetadataNotice::CheckboxDefaultUnknown { field: title_of(d) });
        }
    }
    // Worn tags per (field, option), in definition then option order, decided after the
    // walk so the tag order is the fields' and not the documents'.
    let mut list_worn: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();
    let mut box_worn: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    let mut invalid: BTreeMap<usize, u32> = BTreeMap::new();
    let mut undefined_ids: BTreeSet<String> = BTreeSet::new();
    let mut undefined_values = 0u32;
    let mut without_row = 0u32;
    let mut texts: BTreeMap<usize, Vec<Entry<String>>> = BTreeMap::new();
    // A note-like row's own values, by field, kept in the fields' order below.
    let mut sheet_values: HashMap<RowRef, Vec<(usize, String)>> = HashMap::new();
    for (item, row) in project.binder.iter().enumerate() {
        let item = item as u32;
        let carried = ctx.carried(item);
        let mut holds_any = false;
        let mut seen_fields: HashSet<usize> = HashSet::new();
        for value in &row.meta.custom {
            let Some(&d) = def_of.get(value.field.as_str()) else {
                undefined_ids.insert(value.field.clone());
                undefined_values += 1;
                continue;
            };
            // A field written twice on one document: the first value is Scrivener's.
            if !seen_fields.insert(d) {
                continue;
            }
            let raw = value.value.trim();
            match field_kind(defs[d].kind.as_deref()) {
                FieldKind::List => {
                    if raw.is_empty() || raw == "-1" {
                        continue;
                    }
                    let option = raw
                        .parse::<i64>()
                        .ok()
                        .and_then(|id| defs[d].options.iter().position(|o| o.id == id));
                    match option {
                        Some(o) => {
                            holds_any = true;
                            if carried {
                                list_worn.entry((d, o)).or_default().push(item);
                            }
                        }
                        None => *invalid.entry(d).or_default() += 1,
                    }
                }
                FieldKind::Checkbox => {
                    if raw.eq_ignore_ascii_case("yes") {
                        holds_any = true;
                        if carried {
                            box_worn.entry(d).or_default().push(item);
                        }
                    }
                }
                FieldKind::Text => {
                    if raw.is_empty() {
                        continue;
                    }
                    holds_any = true;
                    if !carried {
                        continue;
                    }
                    let path = ctx.path(item);
                    match ctx.sheet(item) {
                        Some(at) => {
                            sheet_values
                                .entry(at)
                                .or_default()
                                .push((d, value.value.clone()));
                            texts.entry(d).or_default().push(Entry::InOwnNote { path });
                        }
                        None => texts.entry(d).or_default().push(Entry::Listed {
                            path,
                            value: value.value.clone(),
                        }),
                    }
                }
            }
        }
        if holds_any && !carried {
            without_row += 1;
        }
    }
    for (at, mut values) in sheet_values {
        values.sort_by_key(|(d, _)| *d);
        out.sheets.entry(at).or_default().fields = values
            .into_iter()
            .map(|(d, value)| (title_of(d), value))
            .collect();
    }
    let mut fields_tagged: BTreeSet<usize> = BTreeSet::new();
    let (mut tags, mut tagged_rows) = (0u32, 0u32);
    for (d, def) in defs.iter().enumerate() {
        let field = title_of(d);
        for (o, option_def) in def.options.iter().enumerate() {
            let Some(items) = list_worn.get(&(d, o)) else {
                continue;
            };
            let tag = out.tags.len() as u32;
            let option = clean_title(Some(&option_def.name)).0;
            out.tags.push(RawTag {
                source: TagSource::Field,
                name: field.clone(),
                option: Some(if option.is_empty() {
                    option_def.id.to_string()
                } else {
                    option
                }),
                id: 0,
                parent: None,
                path: field.clone(),
                color: None,
            });
            tags += 1;
            fields_tagged.insert(d);
            for item in items {
                tagged_rows += wear(ctx, out, *item, tag);
            }
        }
        if let Some(items) = box_worn.get(&d) {
            let tag = out.tags.len() as u32;
            out.tags.push(RawTag {
                source: TagSource::Field,
                name: field.clone(),
                option: None,
                id: 0,
                parent: None,
                path: field.clone(),
                color: None,
            });
            tags += 1;
            fields_tagged.insert(d);
            for item in items {
                tagged_rows += wear(ctx, out, *item, tag);
            }
        }
        if let Some(entries) = texts.remove(&d)
            && entries.iter().any(|e| matches!(e, Entry::Listed { .. }))
        {
            out.listing.fields.push(FieldListing {
                title: field,
                entries,
            });
        }
    }
    if tags > 0 {
        out.notices.push(MetadataNotice::FieldsBecameTags {
            fields: fields_tagged.len() as u32,
            tags,
            rows: tagged_rows,
        });
    }
    for (d, count) in invalid {
        out.notices.push(MetadataNotice::ListValueInvalid {
            field: title_of(d),
            count,
        });
    }
    if undefined_values > 0 {
        out.notices.push(MetadataNotice::UndefinedCustomField {
            ids: undefined_ids.into_iter().collect(),
            values: undefined_values,
        });
    }
    if without_row > 0 {
        out.notices.push(MetadataNotice::ValuesWithoutRow {
            field: Field::CustomField,
            items: without_row,
        });
    }
}

// ---------------------------------------------------------------- bookmarks (D4-14, X-35)

/// Whether Skribisto shows a reference from `owner` to `target`: a note to a note (the
/// note's *Links*, on its Details page), or prose or a notes folder to a story-bible entry
/// (its *Cast*, in the Inspector). A notes folder has no Details page, so it shows no other
/// reference. The same rule as the links inside the prose (X-35).
pub(crate) fn shown(owner: &PlannedRow, target: &PlannedRow) -> bool {
    let item_note = owner.role == Role::Item && owner.sub_role == SubRole::Note;
    let notes_folder = owner.role == Role::Folder && owner.sub_role == SubRole::Note;
    (item_note && target.sub_role == SubRole::Note)
        || ((notes_folder || skribisto_model::counts_prose(&owner.role, &owner.sub_role))
            && target.tag.is_some())
}

/// A link outside the project, as a line keeps it.
fn external(link: &Link) -> Option<LinkLine> {
    let url = link.destination.as_deref()?.trim();
    (!url.is_empty()).then(|| LinkLine::External {
        label: link.text.clone().filter(|t| !t.trim().is_empty()),
        url: url.to_string(),
        local_file: link.local_file,
    })
}

fn bookmarks(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    let mut edges: HashSet<(RowRef, RowRef)> = HashSet::new();
    let (mut unresolved, mut without_row, mut ends_without_row) = (0u32, 0u32, 0u32);
    for (item, row) in project.binder.iter().enumerate() {
        let item = item as u32;
        if row.links.is_empty() {
            continue;
        }
        if !ctx.carried(item) {
            if row
                .links
                .iter()
                .any(|l| l.target.is_some() || l.destination.is_some())
            {
                without_row += 1;
            }
            continue;
        }
        let owner = ctx.home_row(item);
        let mut lines: Vec<LinkLine> = Vec::new();
        for link in &row.links {
            let Some(key) = link.target else {
                if let Some(line) = external(link) {
                    lines.push(line);
                }
                continue;
            };
            let Some(&target) = ctx.by_key.get(&key) else {
                unresolved += 1;
                continue;
            };
            if target == item {
                continue;
            }
            let note = link.text.clone().filter(|t| !t.trim().is_empty());
            let line = LinkLine::Item {
                path: ctx.path(target),
                note,
            };
            let Some(target_row) = ctx.item_row(target) else {
                ends_without_row += 1;
                lines.push(line);
                continue;
            };
            let edge = owner.and_then(|o| {
                let (owner_row, target_planned) = (ctx.row(o)?, ctx.row(target_row)?);
                (o != target_row && shown(owner_row, target_planned)).then_some((o, target_row))
            });
            match edge {
                Some(edge) => {
                    if edges.insert(edge) {
                        out.edges.push(edge);
                    }
                }
                None => lines.push(line),
            }
        }
        if lines.is_empty() {
            continue;
        }
        let path = ctx.path(item);
        match ctx.sheet(item) {
            Some(at) => {
                out.sheets.entry(at).or_default().links.extend(lines);
                out.listing.bookmarks.push(Entry::InOwnNote { path });
            }
            None => out
                .listing
                .bookmarks
                .push(Entry::Listed { path, value: lines }),
        }
    }
    // The project's own bookmarks and favourites, whatever has a row (D4-14).
    for link in &project.project_links {
        match link.target {
            Some(key) => match ctx.by_key.get(&key) {
                Some(&target) => out.listing.project_links.push(LinkLine::Item {
                    path: ctx.path(target),
                    note: link.text.clone().filter(|t| !t.trim().is_empty()),
                }),
                None => unresolved += 1,
            },
            None => {
                if let Some(line) = external(link) {
                    out.listing.project_links.push(line);
                }
            }
        }
    }
    for key in &project.favorites {
        match ctx.by_key.get(key) {
            Some(&target) => out.listing.project_links.push(LinkLine::Item {
                path: ctx.path(target),
                note: None,
            }),
            None => unresolved += 1,
        }
    }
    if !out.edges.is_empty() {
        out.notices.push(MetadataNotice::BookmarksBecameLinks {
            count: out.edges.len() as u32,
        });
    }
    if ends_without_row > 0 {
        out.notices.push(MetadataNotice::BookmarkEndsWithoutRow {
            count: ends_without_row,
        });
    }
    if unresolved > 0 {
        out.notices
            .push(MetadataNotice::BookmarksUnresolved { count: unresolved });
    }
    if without_row > 0 {
        out.notices.push(MetadataNotice::ValuesWithoutRow {
            field: Field::Bookmark,
            items: without_row,
        });
    }
}

// ---------------------------------------------------------------- targets (D4-11, D4-12, X-30)

/// A target's count as written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Count {
    /// Nothing, or zero: no target.
    None,
    /// A whole number, or a decimal one rounded, from 1 to
    /// [`MAX_GOAL`](skribisto_model::goal_unit::MAX_GOAL), the most a goal holds.
    Of(i64),
    /// Text that is no number, a number below zero, or one past what a goal holds.
    Unreadable,
}

impl Count {
    fn value(self) -> Option<i64> {
        match self {
            Count::Of(value) => Some(value),
            Count::None | Count::Unreadable => None,
        }
    }
}

/// What `raw` counts. A number is read as it is written, never saturated: a count past
/// what a goal holds is unreadable rather than the most a goal holds.
fn count_of(raw: &str) -> Count {
    let raw = raw.trim();
    if raw.is_empty() {
        return Count::None;
    }
    let value = raw.parse::<i64>().ok().or_else(|| {
        raw.parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && v.abs() <= skribisto_model::goal_unit::MAX_GOAL as f64)
            .map(|v| v.round() as i64)
    });
    match value {
        Some(0) => Count::None,
        Some(v) if (1..=skribisto_model::goal_unit::MAX_GOAL).contains(&v) => Count::Of(v),
        _ => Count::Unreadable,
    }
}

/// The unit a target's `Type` names: words when it names none, as Scrivener counts.
fn unit_of(kind: Option<&str>) -> TargetUnit {
    match kind.map(str::trim) {
        None | Some("") | Some("Words") => TargetUnit::Words,
        Some("Characters") => TargetUnit::Characters,
        Some(_) => TargetUnit::Pages,
    }
}

fn goal_unit(unit: TargetUnit) -> Option<GoalUnit> {
    match unit {
        TargetUnit::Words => Some(GoalUnit::Words),
        TargetUnit::Characters => Some(GoalUnit::Characters),
        TargetUnit::Pages => None,
    }
}

fn attribute<'t>(target: &'t RawTarget, name: &str) -> Option<&'t str> {
    target
        .attributes
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, v)| v.trim())
}

/// Whether a goal on `row` measures something: a row whose own prose counts, or a
/// container of the book's stream in the manuscript (a book, a part, a chapter, a plain
/// folder), whose goal measures the scenes under it (X-30). A notes or paratext folder holds
/// no counted prose, so its goal would read zero forever.
fn measurable(binder: BinderKind, row: &PlannedRow) -> bool {
    skribisto_model::counts_prose(&row.role, &row.sub_role)
        || (binder == BinderKind::Manuscript
            && row.role == Role::Folder
            && matches!(
                row.sub_role,
                SubRole::Book | SubRole::Part | SubRole::ChapterScene | SubRole::None
            ))
}

/// Which days of the week `raw` names (Scrivener's `WritingDays`: `1-2, 4-6`), when it
/// names all seven or none: every day. `None` for some days only, or a list this cannot
/// read: the day numbering is not established (D4-12), so no single day is assumed.
fn every_day(raw: &str) -> Option<()> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Some(());
    }
    let mut days = [false; 7];
    for part in raw.split(',') {
        let part = part.trim();
        let (from, to) = match part.split_once('-') {
            Some((a, b)) => (
                a.trim().parse::<usize>().ok()?,
                b.trim().parse::<usize>().ok()?,
            ),
            None => {
                let d = part.parse::<usize>().ok()?;
                (d, d)
            }
        };
        if !(1..=7).contains(&from) || !(1..=7).contains(&to) || from > to {
            return None;
        }
        for day in &mut days[from - 1..to] {
            *day = true;
        }
    }
    days.iter().all(|d| *d).then_some(())
}

fn targets(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    // Document targets.
    let (mut pages, mut on_notes, mut on_split, mut without_row, mut unreadable) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    for (item, row) in project.binder.iter().enumerate() {
        let item = item as u32;
        let Some(target) = &row.target else {
            continue;
        };
        let value = match count_of(&target.value) {
            Count::None => continue,
            Count::Unreadable => {
                if ctx.carried(item) {
                    unreadable += 1;
                } else {
                    without_row += 1;
                }
                continue;
            }
            Count::Of(value) => value,
        };
        if !ctx.carried(item) {
            without_row += 1;
            continue;
        }
        let unit = unit_of(target.kind.as_deref());
        let Some(goal) = goal_unit(unit) else {
            pages += 1;
            continue;
        };
        let listed = TargetLine {
            path: ctx.path(item),
            value,
            unit,
        };
        // A document split at its headings has its writing in chapters side by side: a
        // goal on the first would measure that chapter alone against the whole target.
        if ctx.split(item) {
            on_split += 1;
            out.listing.targets.push(listed);
            continue;
        }
        let home = ctx.home_row(item);
        match home.and_then(|at| ctx.row(at).map(|row| (at, row))) {
            Some((at, planned)) if measurable(at.binder, planned) => out.goals.push(Goal {
                row: at,
                value,
                unit: goal,
            }),
            _ => {
                on_notes += 1;
                out.listing.targets.push(listed);
            }
        }
    }
    if !out.goals.is_empty() {
        out.notices.push(MetadataNotice::GoalsKept {
            rows: out.goals.len() as u32,
        });
    }
    if on_notes > 0 {
        out.notices
            .push(MetadataNotice::TargetsOnNotes { count: on_notes });
    }
    if on_split > 0 {
        out.notices
            .push(MetadataNotice::TargetsOnSplitDocuments { count: on_split });
    }
    if pages > 0 {
        out.notices
            .push(MetadataNotice::TargetsInPages { count: pages });
    }
    if without_row > 0 {
        out.notices.push(MetadataNotice::ValuesWithoutRow {
            field: Field::Target,
            items: without_row,
        });
    }

    // The draft target, on the one book the Draft became.
    let books: Vec<RowRef> = ctx
        .plan
        .manuscript
        .iter()
        .enumerate()
        .filter(|(_, row)| row.sub_role == SubRole::Book)
        .map(|(r, _)| RowRef {
            binder: BinderKind::Manuscript,
            row: r as u32,
        })
        .collect();
    let single_book = (books.len() == 1).then(|| books[0]);
    let draft = project.targets.draft.as_ref();
    let draft_count = draft.map_or(Count::None, |t| count_of(&t.value));
    if draft_count == Count::Unreadable {
        unreadable += 1;
    }
    if unreadable > 0 {
        out.notices
            .push(MetadataNotice::TargetsUnreadable { count: unreadable });
    }
    let draft_value = draft_count.value();
    let draft_unit = unit_of(draft.and_then(|t| attribute(t, "Type")));
    if let (Some(target), Some(value)) = (draft, draft_value) {
        match (goal_unit(draft_unit), single_book) {
            (Some(unit), Some(book)) => {
                out.book_goal = Some(Goal {
                    row: book,
                    value,
                    unit,
                });
                out.notices.push(MetadataNotice::DraftTargetKept {
                    value,
                    unit: draft_unit,
                });
                let counts_all = attribute(target, "CountIncludedOnly") == Some("No");
                let compile_group = attribute(target, "CurrentCompileGroupOnly") == Some("Yes");
                if counts_all || compile_group {
                    out.notices.push(MetadataNotice::CountingDiffers);
                }
            }
            (unit, _) => {
                let reason = if unit.is_none() {
                    DraftTargetReason::Pages
                } else {
                    DraftTargetReason::NoSingleBook
                };
                out.listing.project.push(ProjectLine::DraftTarget {
                    value,
                    unit: draft_unit,
                });
                out.notices.push(MetadataNotice::DraftTargetNotKept {
                    reason,
                    value,
                    unit: draft_unit,
                });
            }
        }
    }

    // The unit the project counts in: the draft target's, else what most goals count
    // (X-30); a tie or nothing leaves it to the language.
    out.unit = match &out.book_goal {
        Some(goal) => Some(goal.unit.clone()),
        None => {
            let characters = out
                .goals
                .iter()
                .filter(|g| g.unit == GoalUnit::Characters)
                .count();
            let words = out.goals.len() - characters;
            match words.cmp(&characters) {
                std::cmp::Ordering::Greater => Some(GoalUnit::Words),
                std::cmp::Ordering::Less => Some(GoalUnit::Characters),
                std::cmp::Ordering::Equal => None,
            }
        }
    };

    // The deadline in force (D4-12).
    let session = project.targets.session.as_ref();
    let writing_days = session
        .and_then(|s| attribute(s, "WritingDays"))
        .unwrap_or_default();
    let partial_days = every_day(writing_days)
        .is_none()
        .then(|| writing_days.to_string());
    if let Some(raw) = &partial_days {
        out.listing
            .project
            .push(ProjectLine::WritingDays(raw.clone()));
    }
    let Some(target) = draft else {
        return;
    };
    let in_force = attribute(target, "IgnoreDeadline") == Some("No");
    let Some(raw) = attribute(target, "Deadline").filter(|_| in_force) else {
        return;
    };
    let stamp = crate::dates::stamp(raw);
    let day = crate::dates::local_date(&stamp);
    out.listing.project.push(ProjectLine::Deadline(match day {
        Some(day) => day.format("%Y-%m-%d").to_string(),
        None => raw.to_string(),
    }));
    let unix_day = day
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc().timestamp());
    let blocked = match (day, draft_value, goal_unit(draft_unit), single_book) {
        (None, ..) => Some(DeadlineReason::Unreadable),
        (_, None, ..) | (_, _, None, _) => Some(DeadlineReason::NoWordGoal),
        (_, _, Some(GoalUnit::Characters), _) => Some(DeadlineReason::CharacterGoal),
        (_, _, _, None) => Some(DeadlineReason::NoSingleBook),
        _ => None,
    };
    match (blocked, day, single_book) {
        (None, Some(end), Some(book)) => {
            out.pace = Some(PaceInputs {
                book,
                end,
                day_off: session.and_then(|s| attribute(s, "CanWriteOnDeadlineDate")) == Some("No"),
                partial_days,
            });
        }
        (reason, _, _) => {
            out.deadline_blocked = Some((reason.unwrap_or(DeadlineReason::Unreadable), unix_day));
        }
    }
}

// ---------------------------------------------------------------- the project (D4-15, D4-20)

fn project_lines(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    let mut lines = Vec::new();
    let stated = super::stated_title(project, &mut Vec::new()).map(|(t, _)| folded(&t));
    let abbreviated = project
        .compile
        .abbreviated_title
        .as_deref()
        .or(project.properties.abbreviated_title.as_deref())
        .map(|t| clean_title(Some(t)).0)
        .filter(|t| !t.is_empty() && !super::is_placeholder(t))
        .filter(|t| stated.as_deref() != Some(folded(t).as_str()));
    if let Some(abbreviated) = abbreviated {
        lines.push(ProjectLine::AbbreviatedTitle(abbreviated));
    }
    let subjects: Vec<String> = project
        .compile
        .subjects
        .iter()
        .map(|s| clean_title(Some(s)).0)
        .filter(|s| !s.is_empty() && !super::is_placeholder(s))
        .collect();
    if !subjects.is_empty() {
        lines.push(ProjectLine::Subjects(subjects));
    }
    if let Some(company) = project
        .compile
        .company
        .as_deref()
        .map(|c| clean_title(Some(c)).0)
        .filter(|c| !c.is_empty() && !super::is_placeholder(c))
    {
        lines.push(ProjectLine::Company(company));
    }
    if let Some(session) = &project.targets.session
        && let Some(value) = count_of(&session.value).value()
    {
        let unit = unit_of(attribute(session, "Type"));
        lines.push(ProjectLine::SessionTarget { value, unit });
        out.notices
            .push(MetadataNotice::SessionTargetNotImported { value, unit });
    }
    // The lines the targets wrote (a draft target no book took, partial writing days, the
    // deadline) come after these.
    lines.append(&mut out.listing.project);
    out.listing.project = lines;
}

// ---------------------------------------------------------------- auto-complete (D4-17)

fn completions(ctx: &Ctx<'_>, out: &mut Collected) {
    let mut words: HashSet<String> = HashSet::new();
    let mut phrases: HashSet<String> = HashSet::new();
    for entry in &ctx.project.auto_complete {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        if entry.contains(char::is_whitespace) {
            if phrases.insert(entry.to_string()) {
                out.listing.completions.push(entry.to_string());
            }
        } else if words.insert(entry.to_string()) {
            // The dictionary matches exact case: `LUKE` stays `LUKE`.
            out.dictionary.push(entry.to_string());
        }
    }
    if !out.dictionary.is_empty() {
        out.notices.push(MetadataNotice::DictionaryWords {
            count: out.dictionary.len() as u32,
        });
    }
}

// ---------------------------------------------------------------- the point of view (D4-6)

fn pov_hint(ctx: &Ctx<'_>, out: &mut Collected) {
    let project = ctx.project;
    // The story bible's entries: notes wearing a story-bible tag.
    let entries: Vec<String> = ctx
        .plan
        .notes
        .iter()
        .filter(|row| row.tag.is_some() && row.role == Role::Item)
        .map(|row| row.title.text().to_string())
        .filter(|t| !t.trim().is_empty())
        .collect();
    if entries.is_empty() {
        return;
    }
    let folded_entries: Vec<String> = entries.iter().map(|e| folded(e)).collect();
    let first_words: Vec<String> = folded_entries
        .iter()
        .map(|e| e.split_whitespace().next().unwrap_or_default().to_string())
        .collect();
    // The candidates: labels a row wears, and the values of fields named like a point of
    // view.
    let none_label = project
        .labels
        .entries
        .iter()
        .find(|e| e.id == -1)
        .map(|e| folded(&e.name));
    // The fields named like a point of view, by id: the first definition of an id is
    // Scrivener's.
    let mut first_def: HashMap<&str, usize> = HashMap::new();
    for (d, def) in project.custom_fields.iter().enumerate() {
        first_def.entry(def.id.as_str()).or_insert(d);
    }
    let pov_fields: HashMap<&str, usize> = first_def
        .into_iter()
        .filter(|(_, d)| {
            let title = folded(&clean_title(project.custom_fields[*d].title.as_deref()).0);
            POV_TITLES.iter().any(|w| title == *w || title.contains(w))
        })
        .collect();
    let mut candidates: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for (item, row) in project.binder.iter().enumerate() {
        if !ctx.carried(item as u32) {
            continue;
        }
        if let Some(label) = row.meta.label
            && let Some(entry) = project.labels.entries.iter().find(|e| e.id == label)
        {
            let name = clean_title(Some(&entry.name)).0;
            if !name.is_empty()
                && none_label.as_deref() != Some(folded(&name).as_str())
                && seen.insert(folded(&name))
            {
                candidates.push(name);
            }
        }
        for value in &row.meta.custom {
            let Some(&d) = pov_fields.get(value.field.as_str()) else {
                continue;
            };
            let def = &project.custom_fields[d];
            // A text field holds the name itself; a list field the number of one of its
            // options, whose name is the candidate.
            let raw = value.value.trim();
            let named = match field_kind(def.kind.as_deref()) {
                FieldKind::Text => Some(raw),
                FieldKind::List if !raw.is_empty() && raw != "-1" => raw
                    .parse::<i64>()
                    .ok()
                    .and_then(|id| def.options.iter().find(|o| o.id == id))
                    .map(|o| o.name.as_str()),
                FieldKind::List | FieldKind::Checkbox => None,
            };
            let name = clean_title(named).0;
            if !name.is_empty() && seen.insert(folded(&name)) {
                candidates.push(name);
            }
        }
    }
    let mut values = Vec::new();
    let mut matches: Vec<String> = Vec::new();
    for candidate in candidates {
        let key = folded(&candidate);
        let exact: Vec<usize> = (0..entries.len())
            .filter(|e| folded_entries[*e] == key)
            .collect();
        let found = if exact.is_empty() {
            let by_first: Vec<usize> = (0..entries.len())
                .filter(|e| first_words[*e] == key)
                .collect();
            if by_first.len() == 1 {
                by_first
            } else {
                Vec::new()
            }
        } else {
            exact
        };
        if found.is_empty() {
            continue;
        }
        values.push(candidate);
        for e in found {
            if !matches.contains(&entries[e]) {
                matches.push(entries[e].clone());
            }
        }
    }
    if !values.is_empty() {
        out.notices
            .push(MetadataNotice::PovHint { values, matches });
    }
}

// ---------------------------------------------------------------- icons and script mode

fn icons_and_script(ctx: &Ctx<'_>, out: &mut Collected) {
    let mut icon_counts: BTreeMap<String, u32> = BTreeMap::new();
    let (mut with_icon, mut script) = (0u32, 0u32);
    for (item, row) in ctx.project.binder.iter().enumerate() {
        if !ctx.carried(item as u32) {
            continue;
        }
        if let Some(icon) = row
            .meta
            .icon
            .as_deref()
            .map(str::trim)
            .filter(|i| !i.is_empty())
        {
            with_icon += 1;
            *icon_counts.entry(icon.to_string()).or_default() += 1;
        }
        if row.script_mode {
            script += 1;
        }
    }
    if script > 0 {
        out.notices
            .push(MetadataNotice::ScriptModeRows { rows: script });
    }
    if with_icon > 0 {
        let mut ranked: Vec<(String, u32)> = icon_counts.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        out.notices.push(MetadataNotice::IconsDropped {
            rows: with_icon,
            names: ranked.into_iter().take(5).map(|(name, _)| name).collect(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writing_days_are_every_day_only_when_all_seven_or_none() {
        assert!(every_day("").is_some());
        assert!(every_day("1-7").is_some());
        assert!(every_day("1, 2,3-5 ,6-7").is_some());
        assert!(every_day("1-2, 4-6").is_none());
        assert!(every_day("0-7").is_none());
        assert!(every_day("Monday").is_none());
        assert!(every_day("5-3").is_none());
    }

    #[test]
    fn a_count_is_a_number_above_zero() {
        assert_eq!(count_of(" 1500 "), Count::Of(1500));
        assert_eq!(count_of("80000.0"), Count::Of(80_000));
        assert_eq!(count_of("0"), Count::None);
        assert_eq!(count_of(""), Count::None);
        assert_eq!(count_of("-3"), Count::Unreadable);
        assert_eq!(count_of("lots"), Count::Unreadable);
        assert_eq!(count_of("NaN"), Count::Unreadable);
    }

    /// A count past what a goal holds is unreadable, never saturated into the most a
    /// number holds: two such goals in one chapter would overflow the chapter's sum.
    #[test]
    fn a_count_past_what_a_goal_holds_is_unreadable() {
        use skribisto_model::goal_unit::MAX_GOAL;
        assert_eq!(count_of(&MAX_GOAL.to_string()), Count::Of(MAX_GOAL));
        assert_eq!(count_of(&(MAX_GOAL + 1).to_string()), Count::Unreadable);
        assert_eq!(count_of("9223372036854775807"), Count::Unreadable);
        assert_eq!(count_of("5000000000000000000"), Count::Unreadable);
        assert_eq!(count_of("1e300"), Count::Unreadable);
        assert_eq!(count_of("1e8"), Count::Of(100_000_000));
        assert_eq!(count_of("inf"), Count::Unreadable);
    }

    /// A path names its item whatever the titles above it: each step and the whole are
    /// bounded, the ellipsis standing for what was cut.
    #[test]
    fn a_path_is_bounded_whatever_the_titles() {
        let long = "x".repeat(10 * MAX_SEGMENT_CHARS);
        let step = segment(&long);
        assert_eq!(step.chars().count(), MAX_SEGMENT_CHARS + 1);
        assert!(step.ends_with('…'));
        assert_eq!(segment("Chapter 1"), "Chapter 1");
        // A chain deeper than a path holds keeps its end, the item it names.
        let n = 50u32;
        let parents: Vec<Option<u32>> = (0..n).map(|i| i.checked_sub(1)).collect();
        let titles: Vec<String> = (0..n).map(|i| format!("{i}{}", step)).collect();
        let path = bounded_path(&parents, &titles, n - 1);
        assert!(path.chars().count() <= MAX_PATH_CHARS + 4, "{}", path.len());
        assert!(path.starts_with("… › "), "{path}");
        assert!(path.ends_with(titles[(n - 1) as usize].as_str()));
        let short = vec!["Draft".to_string(), "Arrival".to_string()];
        assert_eq!(bounded_path(&[None, Some(0)], &short, 1), "Draft › Arrival");
    }

    #[test]
    fn a_unit_is_words_unless_named_otherwise() {
        assert_eq!(unit_of(None), TargetUnit::Words);
        assert_eq!(unit_of(Some("Words")), TargetUnit::Words);
        assert_eq!(unit_of(Some("Characters")), TargetUnit::Characters);
        assert_eq!(unit_of(Some("Pages")), TargetUnit::Pages);
    }
}
