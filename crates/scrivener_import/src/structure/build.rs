// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The mapper: from the flat Scrivener binder to the two planned binders.
//!
//! Every walk is a loop over a contiguous range of the flat binder (a subtree is
//! the item and every following row deeper than it) with a stack of open
//! ancestors. The manuscript is walked twice per range: once to decide each
//! item's level, once to emit rows, because an empty folder takes the level its
//! siblings take (D3.4) and its later siblings are not decided yet when it is met.

use std::collections::{BTreeMap, HashMap, HashSet};

use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::entities::ContentRole;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::is_nfc;

use super::lexicon::{self, MatterSide, SectionClass, TitleClass, TitleMatch};
use super::{
    Absence, BinderKind, Body, BooksChoice, BooksControl, ChapterEncoding, ChoiceError, Choices,
    ContainerRung, DepthControl, Destination, Level, Origin, Placement, PlanError, PlannedContent,
    PlannedRow, PlannedTag, PlannedTemplate, PlannedTitle, Review, RootControl, RowRef, Shape,
    ShapeFlag, Signal, Source, SplitControl, StructureNotice, StructurePlan, TextKind,
    TextsControl, TrashChoice, TrashControl, Why,
};
use crate::content::ContentFacts;
use crate::model::{ItemKey, ItemKind, Project, TextRole};

/// The deepest manuscript level the review offers a choice for.
const REVIEW_DEPTHS: u16 = 3;
/// A document with this many chapter headings is offered a split (A2).
const SPLIT_HEADINGS: usize = 3;

pub(super) fn build(
    project: &Project,
    facts: &ContentFacts,
    choices: &Choices,
) -> Result<StructurePlan, PlanError> {
    let mut b = Builder::new(project, facts, choices);
    b.run()?;
    Ok(b.finish())
}

// ---------------------------------------------------------------- the flat tree

/// The binder's shape, computed once, without recursion.
struct Tree {
    depth: Vec<u16>,
    parent: Vec<Option<u32>>,
    /// One past the last row of each item's subtree.
    end: Vec<u32>,
    children: Vec<Vec<u32>>,
    /// A folder of any kind, or any item with children.
    container: Vec<bool>,
    /// How many of each item's children are containers.
    child_containers: Vec<u32>,
    /// Container levels in the subtree, the item included: 0 for a leaf text, 1
    /// for a folder (even empty) or a text with children, plus the tallest child.
    height: Vec<u16>,
}

fn is_folder_kind(kind: &ItemKind) -> bool {
    matches!(
        kind,
        ItemKind::Folder | ItemKind::DraftFolder | ItemKind::ResearchFolder | ItemKind::TrashFolder
    )
}

impl Tree {
    fn new(project: &Project) -> Tree {
        let n = project.binder.len();
        let depth: Vec<u16> = project.binder.iter().map(|r| r.depth).collect();
        let parent: Vec<Option<u32>> = project
            .parents()
            .into_iter()
            .map(|p| p.map(|p| p as u32))
            .collect();
        let mut children = vec![Vec::new(); n];
        for (i, p) in parent.iter().enumerate() {
            if let Some(p) = p {
                children[*p as usize].push(i as u32);
            }
        }
        let mut end = vec![n as u32; n];
        let mut open: Vec<usize> = Vec::new();
        for i in 0..n {
            while let Some(&top) = open.last() {
                if depth[top] >= depth[i] {
                    end[top] = i as u32;
                    open.pop();
                } else {
                    break;
                }
            }
            open.push(i);
        }
        let container: Vec<bool> = (0..n)
            .map(|i| is_folder_kind(&project.binder[i].kind) || !children[i].is_empty())
            .collect();
        let child_containers: Vec<u32> = children
            .iter()
            .map(|c| c.iter().filter(|k| container[**k as usize]).count() as u32)
            .collect();
        let mut height = vec![0u16; n];
        let mut tallest_child = vec![0u16; n];
        for i in (0..n).rev() {
            let h = if container[i] {
                tallest_child[i].saturating_add(1)
            } else {
                0
            };
            height[i] = h;
            if let Some(p) = parent[i] {
                let p = p as usize;
                tallest_child[p] = tallest_child[p].max(h);
            }
        }
        Tree {
            depth,
            parent,
            end,
            children,
            container,
            child_containers,
            height,
        }
    }

    /// Whether `ancestor` is `item` or holds it.
    fn within(&self, item: u32, ancestor: u32) -> bool {
        item >= ancestor && item < self.end[ancestor as usize]
    }
}

// ---------------------------------------------------------------- titles

/// A title cleaned as D3.18 says: NFC, line breaks and tabs to one space, runs
/// of white space collapsed, ends trimmed. Never invented. Whether cleaning
/// changed more than the ends.
pub(crate) fn clean_title(raw: Option<&str>) -> (String, bool) {
    let Some(raw) = raw else {
        return (String::new(), false);
    };
    let edited = !is_nfc(raw)
        || raw.contains(['\n', '\r', '\t', '\u{2028}', '\u{2029}', '\u{0B}', '\u{0C}']);
    let nfc: String = raw.nfc().collect();
    let cleaned = nfc.split_whitespace().collect::<Vec<_>>().join(" ");
    (cleaned, edited)
}

// ---------------------------------------------------------------- claims

/// What the first speaking signal says about an item's level (D3.3).
#[derive(Debug, Clone)]
struct Claim {
    level: Level,
    why: Why,
}

fn level_of_section(class: SectionClass) -> (Level, bool) {
    match class {
        SectionClass::Book => (Level::Book, false),
        SectionClass::Part => (Level::Part, false),
        SectionClass::Chapter => (Level::Chapter, false),
        SectionClass::Scene => (Level::Scene, false),
        SectionClass::Front | SectionClass::Back | SectionClass::Paratext => {
            (Level::Paratext, false)
        }
        SectionClass::Notes => (Level::Note, false),
        SectionClass::Prologue => (Level::Chapter, true),
    }
}

fn level_of_title(class: TitleClass) -> Level {
    match class {
        TitleClass::Book => Level::Book,
        TitleClass::Part => Level::Part,
        TitleClass::Chapter => Level::Chapter,
        TitleClass::Scene => Level::Scene,
        TitleClass::Front | TitleClass::Back => Level::Paratext,
    }
}

/// How high a structural claim reaches: a book inside a book counts as a part.
fn rank_of(level: Level) -> Option<u8> {
    match level {
        Level::Book | Level::Part => Some(1),
        Level::Chapter => Some(2),
        _ => None,
    }
}

struct Claims {
    claim: Vec<Option<Claim>>,
    /// Prologues and epilogues, chapters left out of the numbering.
    prologue: Vec<bool>,
    /// The title's own match, before the sibling rules.
    title: Vec<Option<TitleMatch>>,
    /// The highest structural rank any strict descendant claims.
    below: Vec<Option<u8>>,
    /// The section-type claim alone (explicit or inherited), for root matter.
    section: Vec<Option<SectionClass>>,
}

impl Claims {
    fn new(project: &Project, tree: &Tree, titles: &[String]) -> Claims {
        let n = project.binder.len();
        let names: HashMap<String, &str> = project
            .section_types
            .definitions
            .iter()
            .map(|d| (d.id.trim().to_lowercase(), d.name.as_str()))
            .collect();
        let class_of = |id: &str| {
            names
                .get(&id.trim().to_lowercase())
                .and_then(|name| lexicon::section_class(name))
        };
        // The section type an ancestor hands down: the nearest one that sets it.
        let mut inherited: Vec<Option<(u32, String)>> = vec![None; n];
        for i in 0..n {
            if let Some(p) = tree.parent[i] {
                let p = p as usize;
                inherited[i] = match &project.binder[p].meta.section_child_default {
                    Some(id) => Some((p as u32, id.clone())),
                    None => inherited[p].clone(),
                };
            }
        }
        let title: Vec<Option<TitleMatch>> =
            titles.iter().map(|t| lexicon::title_match(t)).collect();
        // The title tier, after the sibling rules.
        let mut by_title: Vec<Option<(Level, Why)>> = title
            .iter()
            .map(|m| m.as_ref().map(|m| (level_of_title(m.class), Why::Title)))
            .collect();
        let mut groups: Vec<&[u32]> = tree.children.iter().map(Vec::as_slice).collect();
        let roots: Vec<u32> = (0..n as u32)
            .filter(|i| tree.parent[*i as usize].is_none())
            .collect();
        groups.push(&roots);
        for group in groups {
            sibling_rules(group, titles, &title, &mut by_title);
        }

        let mut claim: Vec<Option<Claim>> = vec![None; n];
        let mut prologue = vec![false; n];
        let mut section = vec![None; n];
        for i in 0..n {
            let row = &project.binder[i];
            prologue[i] = lexicon::is_prologue_title(&titles[i]);
            let explicit = row.meta.section_type.as_deref().and_then(class_of);
            let handed = inherited[i]
                .as_ref()
                .and_then(|(from, id)| class_of(id).map(|c| (*from, c)));
            section[i] = explicit.or(handed.map(|(_, c)| c));
            if let Some(class) = explicit {
                let (level, pro) = level_of_section(class);
                prologue[i] |= pro;
                claim[i] = Some(Claim {
                    level,
                    why: Why::SectionType,
                });
                continue;
            }
            if let Some((from, class)) = handed {
                let (level, pro) = level_of_section(class);
                prologue[i] |= pro;
                claim[i] = Some(Claim {
                    level,
                    why: Why::InheritedSectionType { from },
                });
                continue;
            }
            if let Some((level, why)) = by_title[i].take() {
                claim[i] = Some(Claim { level, why });
                continue;
            }
            // The template's default for this level: index 0 is the binder's top
            // level, the last entry covers every deeper one.
            let list = if row.kind == ItemKind::Folder {
                &project.section_types.folders
            } else if !tree.children[i].is_empty() {
                &project.section_types.containers
            } else {
                &project.section_types.files
            };
            let level_index = (tree.depth[i] as usize).min(list.len().saturating_sub(1));
            if let Some(class) = list.get(level_index).and_then(|id| class_of(id)) {
                let (level, pro) = level_of_section(class);
                prologue[i] |= pro;
                claim[i] = Some(Claim {
                    level,
                    why: Why::LevelType {
                        level: tree.depth[i],
                    },
                });
            }
        }
        let mut below: Vec<Option<u8>> = vec![None; n];
        for i in (0..n).rev() {
            let own = claim[i].as_ref().and_then(|c| rank_of(c.level));
            let reach = match (own, below[i]) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
            if let Some(p) = tree.parent[i] {
                let p = p as usize;
                below[p] = match (below[p], reach) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (a, b) => a.or(b),
                };
            }
        }
        Claims {
            claim,
            prologue,
            title,
            below,
            section,
        }
    }
}

/// The sibling rules of D3.3 over one group of siblings: two siblings claiming
/// one level with the same number claim nothing ("Chapter 1 - Scene 2" and
/// "Chapter 1 - Scene 3" are scenes), and a sibling whose title starts with the
/// same word as a majority of titles that claim a level claims it too ("Chapter:
/// Ironmongery" among "Chapter 3: Tinsmiths" and its kin).
fn sibling_rules(
    group: &[u32],
    titles: &[String],
    matches: &[Option<TitleMatch>],
    by_title: &mut [Option<(Level, Why)>],
) {
    if group.len() < 2 {
        return;
    }
    let mut numbers: HashMap<(Level, &str), u32> = HashMap::new();
    for &i in group {
        if let Some(m) = &matches[i as usize]
            && let Some(number) = &m.number
            && !matches!(m.class, TitleClass::Front | TitleClass::Back)
        {
            *numbers
                .entry((level_of_title(m.class), number.as_str()))
                .or_default() += 1;
        }
    }
    let mut dropped = vec![false; group.len()];
    for (k, &i) in group.iter().enumerate() {
        if let Some(m) = &matches[i as usize]
            && let Some(number) = &m.number
            && numbers
                .get(&(level_of_title(m.class), number.as_str()))
                .is_some_and(|c| *c >= 2)
        {
            by_title[i as usize] = None;
            dropped[k] = true;
        }
    }
    let mut by_word: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (k, &i) in group.iter().enumerate() {
        if let Some(word) = lexicon::first_word(&titles[i as usize]) {
            by_word.entry(word).or_default().push(k);
        }
    }
    for members in by_word.values() {
        if members.len() < 2 {
            continue;
        }
        let mut levels: HashMap<Level, usize> = HashMap::new();
        for &k in members {
            if let Some((level, Why::Title)) = &by_title[group[k] as usize] {
                *levels.entry(*level).or_default() += 1;
            }
        }
        let Some((&level, &count)) = levels.iter().max_by_key(|(_, c)| **c) else {
            continue;
        };
        if count * 2 <= members.len() || !matches!(level, Level::Part | Level::Chapter) {
            continue;
        }
        for &k in members {
            let i = group[k] as usize;
            if by_title[i].is_none() && !dropped[k] {
                by_title[i] = Some((level, Why::SiblingTitles));
            }
        }
    }
}

// ---------------------------------------------------------------- decisions

/// The structural rungs, in nesting order. A folder at `Below` only groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Rung {
    Book,
    Part,
    Chapter,
    Below,
}

fn rung_rank(rung: Rung) -> u8 {
    match rung {
        Rung::Book => 0,
        Rung::Part => 1,
        Rung::Chapter => 2,
        Rung::Below => 3,
    }
}

impl From<ContainerRung> for Rung {
    fn from(rung: ContainerRung) -> Rung {
        match rung {
            ContainerRung::Part => Rung::Part,
            ContainerRung::Chapter => Rung::Chapter,
            ContainerRung::Plain => Rung::Below,
        }
    }
}

fn container_rung(rung: Rung) -> ContainerRung {
    match rung {
        Rung::Book | Rung::Part => ContainerRung::Part,
        Rung::Chapter => ContainerRung::Chapter,
        Rung::Below => ContainerRung::Plain,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Struct,
    Paratext,
    Note,
}

#[derive(Debug, Clone)]
enum Decision {
    /// A folder at a structural rung (part, chapter, or plain).
    Container {
        rung: Rung,
        why: Why,
    },
    /// A folder whose children are all front or back matter.
    ParatextFolder,
    /// An unclaimed empty folder, waiting for its siblings.
    Empty,
    /// A text at the chapter rung.
    FlatChapter {
        why: Why,
    },
    /// A text claimed as a part: a part folder holding the text.
    PartLeaf {
        why: Why,
    },
    Scene {
        why: Why,
    },
    /// A text split at its chapter headings; `chapter` when the text itself is a
    /// chapter.
    Split {
        chapter: bool,
        why: Why,
    },
    /// A childless chapter folder split at the headings of its own text.
    SplitFolder {
        why: Why,
    },
    Paratext {
        why: Why,
    },
    Note {
        why: Why,
    },
}

#[derive(Debug, Clone)]
struct Planned {
    decision: Decision,
    indent: u16,
    /// Depth under the book root: 1 for the book's own children.
    rel: u16,
    floor: Rung,
    /// The level a claim wanted and an ancestor already held.
    overruled: Option<Level>,
    /// Decided by the shape and left plain for being few among texts.
    minority: bool,
}

/// A decided range of the manuscript: its rows, in binder order.
type Decided = Vec<(u32, Planned)>;

#[derive(Default)]
struct DepthStats {
    containers: u32,
    adjustable: u32,
    /// Final level of each unclaimed container.
    adjustable_rungs: BTreeMap<ContainerRung, u32>,
    rungs: BTreeMap<ContainerRung, u32>,
    signals: BTreeMap<Signal, u32>,
    examples: Vec<u32>,
    flags: Vec<ShapeFlag>,
}

#[derive(Default)]
struct TextStats {
    texts: u32,
    examples: Vec<u32>,
    words: Vec<u32>,
}

/// What a root item is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootKind {
    Notes,
    StoryBible,
    Templates,
    Matter(MatterSide),
    ExtraBook,
    LeftOut,
}

// ---------------------------------------------------------------- the builder

struct Builder<'a> {
    project: &'a Project,
    facts: &'a ContentFacts,
    choices: &'a Choices,
    tree: Tree,
    titles: Vec<String>,
    edited: Vec<bool>,
    claims: Claims,
    draft: Option<u32>,
    trash: Option<u32>,
    templates_folder: Option<u32>,
    template_of: HashMap<u32, u32>,
    manuscript: Vec<PlannedRow>,
    notes: Vec<PlannedRow>,
    /// Each manuscript row's depth under its book root, when it has one.
    ms_rel: Vec<Option<u16>>,
    templates: Vec<PlannedTemplate>,
    tags: Vec<PlannedTag>,
    tag_by_name: HashMap<String, u32>,
    /// Every recognised folder of each tag, in order.
    tag_folders: Vec<Vec<u32>>,
    trash_roots: Vec<u32>,
    absence: Vec<Option<Absence>>,
    notices: Vec<StructureNotice>,
    depth_stats: BTreeMap<u16, DepthStats>,
    text_stats: BTreeMap<u16, TextStats>,
    splits: Vec<SplitControl>,
    roots: Vec<RootControl>,
    books: Option<BooksControl>,
}

impl<'a> Builder<'a> {
    fn new(project: &'a Project, facts: &'a ContentFacts, choices: &'a Choices) -> Self {
        let tree = Tree::new(project);
        let (titles, edited): (Vec<String>, Vec<bool>) = project
            .binder
            .iter()
            .map(|r| clean_title(r.title.as_deref()))
            .unzip();
        let claims = Claims::new(project, &tree, &titles);
        let n = project.binder.len();
        let top = |kind: ItemKind| {
            (0..n as u32)
                .find(|i| tree.depth[*i as usize] == 0 && project.binder[*i as usize].kind == kind)
        };
        let draft = top(ItemKind::DraftFolder);
        let trash = top(ItemKind::TrashFolder);
        let mut by_key: HashMap<ItemKey, u32> = HashMap::new();
        for (i, row) in project.binder.iter().enumerate() {
            if row.owns_key
                && let Some(key) = row.key
            {
                by_key.entry(key).or_insert(i as u32);
            }
        }
        // The Templates folder is honoured outside the Draft and the Trash only:
        // one the writer threw away is trash, not templates.
        let templates_folder = project
            .template_folder
            .and_then(|key| by_key.get(&key).copied())
            .filter(|t| {
                tree.container[*t as usize]
                    && Some(*t) != draft
                    && Some(*t) != trash
                    && !draft.is_some_and(|d| tree.within(*t, d))
                    && !trash.is_some_and(|x| tree.within(*t, x))
            });
        Builder {
            project,
            facts,
            choices,
            titles,
            edited,
            claims,
            draft,
            trash,
            templates_folder,
            template_of: HashMap::new(),
            manuscript: Vec::new(),
            notes: Vec::new(),
            ms_rel: Vec::new(),
            templates: Vec::new(),
            tags: Vec::new(),
            tag_by_name: HashMap::new(),
            tag_folders: Vec::new(),
            trash_roots: Vec::new(),
            absence: vec![None; n],
            notices: Vec::new(),
            depth_stats: BTreeMap::new(),
            text_stats: BTreeMap::new(),
            splits: Vec::new(),
            roots: Vec::new(),
            books: None,
            tree,
        }
    }

    // ------------------------------------------------ source facts

    fn row(&self, i: u32) -> &crate::model::BinderRow {
        &self.project.binder[i as usize]
    }

    fn text(&self, i: u32) -> bool {
        self.facts.items[i as usize].text.carries.counts()
    }

    fn has_notes(&self, i: u32) -> bool {
        self.facts.items[i as usize].notes.counts()
    }

    fn synopsis(&self, i: u32) -> bool {
        self.facts.items[i as usize].synopsis.counts()
    }

    fn media(&self, i: u32) -> bool {
        self.row(i).payload.file.is_some()
    }

    fn card(&self, i: u32) -> bool {
        self.row(i).payload.card_image.is_some()
    }

    fn writing(&self, i: u32) -> bool {
        self.text(i) || self.has_notes(i) || self.synopsis(i) || self.media(i) || self.card(i)
    }

    fn title(&self, i: u32) -> String {
        self.titles[i as usize].clone()
    }

    fn synopsis_body(&self, i: u32) -> Option<Body> {
        self.synopsis(i)
            .then(|| Body::From(vec![Source::Synopsis { item: i }]))
    }

    fn text_source(&self, i: u32) -> Option<Source> {
        self.text(i).then_some(Source::Text { item: i })
    }

    // ------------------------------------------------ emitting

    fn push(&mut self, binder: BinderKind, row: PlannedRow, rel: Option<u16>) -> u32 {
        match binder {
            BinderKind::Manuscript => {
                self.manuscript.push(row);
                self.ms_rel.push(rel);
                self.manuscript.len() as u32 - 1
            }
            BinderKind::Notes => {
                self.notes.push(row);
                self.notes.len() as u32 - 1
            }
        }
    }

    /// An item's document notes, as a note (D3.10): the first child of a
    /// container, the next sibling of a leaf.
    fn notes_row(&mut self, binder: BinderKind, i: u32, depth: u16, activated: bool, of: &str) {
        if !self.has_notes(i) {
            return;
        }
        let row = PlannedRow {
            depth,
            role: Role::Item,
            sub_role: SubRole::Note,
            title: PlannedTitle::NotesOf(of.to_string()),
            contents: contents(vec![(
                ContentRole::NoteText,
                Some(Body::From(vec![Source::Notes { item: i }])),
            )]),
            exportable: false,
            activated,
            exclude_from_numbering: false,
            origin: Origin::DocumentNotes { item: i },
            why: Why::Synthesised,
            tag: None,
        };
        self.push(binder, row, None);
        self.notices
            .push(StructureNotice::DocumentNotes { item: i });
    }

    /// A container's own writing as its first child, titled as the container
    /// (D3.6). `sub_role` is the leaf kind of where it sits.
    #[allow(clippy::too_many_arguments)]
    fn own_text(
        &mut self,
        binder: BinderKind,
        i: u32,
        depth: u16,
        sub_role: SubRole,
        exportable: bool,
        activated: bool,
        tag: Option<u32>,
        rel: Option<u16>,
    ) {
        let (role, sources) = match sub_role {
            SubRole::Paratext => (
                ContentRole::ParatextText,
                vec![
                    self.media(i).then_some(Source::Media { item: i }),
                    self.text_source(i),
                ],
            ),
            SubRole::Note => (
                ContentRole::NoteText,
                vec![
                    self.card(i).then_some(Source::CardImage { item: i }),
                    self.media(i).then_some(Source::Media { item: i }),
                    self.text_source(i),
                ],
            ),
            _ => (ContentRole::SceneText, vec![self.text_source(i)]),
        };
        // A picture or file cannot be manuscript: beside the prose, as a note.
        if sub_role == SubRole::Scene && self.media(i) {
            let row = PlannedRow {
                depth,
                role: Role::Item,
                sub_role: SubRole::Note,
                title: PlannedTitle::Own(self.title(i)),
                contents: contents(vec![(
                    ContentRole::NoteText,
                    Some(Body::From(vec![Source::Media { item: i }])),
                )]),
                exportable: false,
                activated,
                exclude_from_numbering: false,
                origin: Origin::OwnText { item: i },
                why: Why::Media,
                tag: None,
            };
            self.push(binder, row, rel);
        }
        let Some(body) = body(sources) else {
            return;
        };
        let row = PlannedRow {
            depth,
            role: Role::Item,
            sub_role,
            title: PlannedTitle::Own(self.title(i)),
            contents: contents(vec![(role, Some(body))]),
            exportable,
            activated,
            exclude_from_numbering: false,
            origin: Origin::OwnText { item: i },
            why: Why::Synthesised,
            tag,
        };
        self.push(binder, row, rel);
        if self.text(i) {
            self.notices.push(StructureNotice::OwnTextMoved { item: i });
        }
    }

    /// The title of a structural row: a title that only names its level is
    /// stored blank (A3); one that also spells its number is reported.
    fn structural_title(&mut self, i: u32, rung: Rung) -> PlannedTitle {
        let title = self.title(i);
        if lexicon::is_placeholder_title(&title) {
            self.notices
                .push(StructureNotice::PlaceholderTitle { item: i });
            return PlannedTitle::Own(String::new());
        }
        if rung != Rung::Book
            && self.claims.title[i as usize].as_ref().is_some_and(|m| {
                m.number.is_some() && matches!(m.class, TitleClass::Part | TitleClass::Chapter)
            })
        {
            self.notices
                .push(StructureNotice::TitleRepeatsNumber { item: i });
        }
        PlannedTitle::Own(title)
    }

    fn card_left_behind(&mut self, i: u32) {
        if self.card(i) {
            self.notices
                .push(StructureNotice::CardImageLeftBehind { item: i });
        }
    }

    // ------------------------------------------------ the whole project

    fn run(&mut self) -> Result<(), PlanError> {
        let kinds = self.classify_roots()?;

        // The manuscript binder.
        let front: Vec<u32> = kinds
            .iter()
            .filter(|(_, k)| *k == RootKind::Matter(MatterSide::Front))
            .map(|(r, _)| *r)
            .collect();
        let back: Vec<u32> = kinds
            .iter()
            .filter(|(_, k)| *k == RootKind::Matter(MatterSide::Back))
            .map(|(r, _)| *r)
            .collect();
        let claimed_books: Vec<u32> = match self.draft {
            Some(d) => self.tree.children[d as usize]
                .iter()
                .copied()
                .filter(|c| {
                    self.tree.container[*c as usize]
                        && self.claims.claim[*c as usize]
                            .as_ref()
                            .is_some_and(|cl| cl.level == Level::Book)
                })
                .collect(),
            None => Vec::new(),
        };
        let detected = if claimed_books.len() >= 2 {
            BooksChoice::Several
        } else {
            BooksChoice::One
        };
        if !claimed_books.is_empty() {
            self.books = Some(BooksControl {
                claimed: claimed_books.clone(),
                detected,
                chosen: self.choices.books,
            });
        }
        let several = !claimed_books.is_empty()
            && self.choices.books.unwrap_or(detected) == BooksChoice::Several;
        // Root folders sent to the manuscript keep their place among the root
        // items: a book before the Draft comes before the Draft's books.
        let draft_at = self.draft.unwrap_or(0);
        for (r, kind) in &kinds {
            if *kind == RootKind::ExtraBook && self.draft.is_some() && *r < draft_at {
                self.extra_book(*r);
            }
        }
        match self.draft {
            Some(draft) if several => self.several_books(draft, &claimed_books, &front, &back),
            draft => self.one_book(draft, &front, &back),
        }
        for (r, kind) in &kinds {
            if *kind == RootKind::ExtraBook && (self.draft.is_none() || *r > draft_at) {
                self.extra_book(*r);
            }
        }

        // The notes binder: Scrivener's root order (D3.1, X-29).
        for (r, kind) in &kinds {
            match kind {
                RootKind::Notes => {
                    let (r, stop) = (*r, self.tree.end[*r as usize]);
                    self.walk_notes(r, stop, 0, true, false);
                }
                RootKind::StoryBible => {
                    let (r, stop) = (*r, self.tree.end[*r as usize]);
                    self.walk_notes(r, stop, 0, true, true);
                }
                RootKind::Templates => self.templates(*r, 0),
                RootKind::LeftOut => {
                    let r = *r;
                    let stop = self.tree.end[r as usize];
                    for i in r..stop {
                        self.absence[i as usize] = Some(Absence::LeftOut);
                    }
                    self.notices.push(StructureNotice::LeftOut {
                        item: r,
                        items: stop - r,
                    });
                }
                RootKind::Matter(_) | RootKind::ExtraBook => {}
            }
        }
        self.project_notes();
        self.recovered();
        self.trash_binder();
        Ok(())
    }

    /// What each root item is (D3.1, D3.11, D3.16, D3.20).
    fn classify_roots(&mut self) -> Result<Vec<(u32, RootKind)>, PlanError> {
        let n = self.project.binder.len() as u32;
        let mut out = Vec::new();
        let mut offered: HashSet<u32> = HashSet::new();
        for r in 0..n {
            if self.tree.depth[r as usize] != 0 || Some(r) == self.draft || Some(r) == self.trash {
                continue;
            }
            let row = self.row(r);
            let kind = if Some(r) == self.templates_folder {
                RootKind::Templates
            } else if row.kind == ItemKind::ResearchFolder {
                RootKind::Notes
            } else if let Some(side) = self.matter_side(r) {
                RootKind::Matter(side)
            } else if self.is_bible_folder(r, false) || !self.tree.container[r as usize] {
                RootKind::Notes
            } else {
                // Not recognised: the review offers where it goes.
                offered.insert(r);
                let chosen = self.choices.roots.get(&r).copied();
                let looks = self.looks_like_manuscript(r);
                if looks {
                    let leaving = matches!(
                        chosen,
                        Some(Destination::Manuscript) | Some(Destination::LeaveOut)
                    );
                    if !leaving {
                        self.notices
                            .push(StructureNotice::LooksLikeManuscript { item: r });
                    }
                }
                self.roots.push(RootControl {
                    item: r,
                    items: self.tree.end[r as usize] - r,
                    detected: Destination::Notes,
                    chosen,
                    looks_like_manuscript: looks,
                });
                match chosen.unwrap_or(Destination::Notes) {
                    Destination::Notes => RootKind::Notes,
                    Destination::StoryBible => RootKind::StoryBible,
                    Destination::Manuscript => RootKind::ExtraBook,
                    Destination::LeaveOut => RootKind::LeftOut,
                }
            };
            out.push((r, kind));
        }
        for item in self.choices.roots.keys() {
            if !offered.contains(item) {
                return Err(PlanError::Choice(ChoiceError::RootNotOffered {
                    item: *item,
                }));
            }
        }
        Ok(out)
    }

    /// Root front or back matter: by icon, then section type, then title
    /// (D3.11).
    fn matter_side(&self, r: u32) -> Option<MatterSide> {
        let row = self.row(r);
        if let Some(side) = row.meta.icon.as_deref().and_then(lexicon::matter_icon) {
            return Some(side);
        }
        match self.claims.section[r as usize] {
            Some(SectionClass::Front) => return Some(MatterSide::Front),
            Some(SectionClass::Back) => return Some(MatterSide::Back),
            _ => {}
        }
        let title = &self.titles[r as usize];
        if let Some(side) = lexicon::matter_title(title) {
            return Some(side);
        }
        if row.kind == ItemKind::Folder {
            return match self.claims.title[r as usize].as_ref().map(|m| m.class) {
                Some(TitleClass::Front) => Some(MatterSide::Front),
                Some(TitleClass::Back) => Some(MatterSide::Back),
                _ => None,
            };
        }
        None
    }

    /// A character or place folder (D3.14): by icon, then title. Never in the
    /// Trash: a tag whose entries are created in the Trash would lose them.
    fn is_bible_folder(&self, i: u32, in_trash: bool) -> bool {
        let row = self.row(i);
        !in_trash
            && row.kind == ItemKind::Folder
            && (row.meta.icon.as_deref().is_some_and(lexicon::is_bible_icon)
                || lexicon::is_bible_title(&self.titles[i as usize]))
    }

    /// A root folder whose title or content reads like more of the book.
    fn looks_like_manuscript(&self, r: u32) -> bool {
        let title = &self.titles[r as usize];
        let claims_level = |i: u32| {
            self.claims.title[i as usize].as_ref().is_some_and(|m| {
                matches!(
                    m.class,
                    TitleClass::Book | TitleClass::Part | TitleClass::Chapter
                )
            })
        };
        claims_level(r)
            || lexicon::has_manuscript_word(title)
            || (r + 1..self.tree.end[r as usize]).any(|i| {
                self.claims.title[i as usize]
                    .as_ref()
                    .is_some_and(|m| matches!(m.class, TitleClass::Part | TitleClass::Chapter))
            })
    }

    // ------------------------------------------------ books

    /// The book's title (X-37): the title the project's settings state, when they state
    /// one that is words (compile settings, project properties), else the Draft's name when
    /// the writer gave it one, else the project's name (a template's own title, or the
    /// folder's, cleaned of what backups and sync services add to it).
    fn book_title(&self, draft: Option<u32>) -> PlannedTitle {
        let stated = crate::metadata::stated_title(self.project, &mut Vec::new())
            .map(|(title, _)| title)
            .filter(|t| !lexicon::is_default_draft_name(t));
        if let Some(title) = stated {
            return PlannedTitle::ProjectTitle(title);
        }
        if let Some(d) = draft {
            let title = &self.titles[d as usize];
            if !lexicon::is_default_draft_name(title) && !lexicon::is_placeholder_title(title) {
                return PlannedTitle::Own(title.clone());
            }
        }
        PlannedTitle::ProjectTitle(self.folder_stem())
    }

    /// The project's own name: a `.scrivtemplate`'s title, else the folder's.
    fn folder_stem(&self) -> String {
        if let Some(title) = &self.project.source.template_title {
            let cleaned = clean_title(Some(title)).0;
            if !cleaned.is_empty() {
                return cleaned;
            }
        }
        crate::metadata::folder_title(self.project)
    }

    fn book_row(&mut self, title: PlannedTitle, synopsis: Option<Body>, origin: Origin, why: Why) {
        let row = PlannedRow {
            depth: 0,
            role: Role::Folder,
            sub_role: SubRole::Book,
            title,
            contents: contents(vec![
                (ContentRole::BookTitle, Some(Body::Title)),
                (ContentRole::SynopsisText, synopsis),
            ]),
            exportable: true,
            activated: true,
            exclude_from_numbering: false,
            origin,
            why,
            tag: None,
        };
        self.push(BinderKind::Manuscript, row, None);
    }

    fn book_end(&mut self) {
        let row = PlannedRow {
            depth: 1,
            role: Role::Item,
            sub_role: SubRole::BookEnd,
            title: PlannedTitle::Own(String::new()),
            contents: Vec::new(),
            exportable: true,
            activated: true,
            exclude_from_numbering: false,
            origin: Origin::BookEnd,
            why: Why::Synthesised,
            tag: None,
        };
        self.push(BinderKind::Manuscript, row, None);
    }

    /// One book synthesised from the Draft (D3.8, D3.9).
    fn one_book(&mut self, draft: Option<u32>, front: &[u32], back: &[u32]) {
        let title = self.book_title(draft);
        match draft {
            Some(d) => {
                let synopsis = self.synopsis_body(d);
                self.book_row(
                    title.clone(),
                    synopsis,
                    Origin::Item { item: d },
                    Why::Synthesised,
                );
            }
            None => self.book_row(title.clone(), None, Origin::Book, Why::Synthesised),
        }
        for f in front {
            self.matter(*f, 1);
        }
        if let Some(d) = draft {
            // The Draft's own text opens the book, titled as the book.
            if let Some(source) = self.text_source(d) {
                let row = PlannedRow {
                    depth: 1,
                    role: Role::Item,
                    sub_role: SubRole::Scene,
                    title: PlannedTitle::Own(title.text().to_string()),
                    contents: contents(vec![(
                        ContentRole::SceneText,
                        Some(Body::From(vec![source])),
                    )]),
                    exportable: true,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::OwnText { item: d },
                    why: Why::Synthesised,
                    tag: None,
                };
                self.push(BinderKind::Manuscript, row, Some(0));
                self.notices.push(StructureNotice::OwnTextMoved { item: d });
            }
            if self.media(d) {
                // Only a crafted Draft carries a file; it still lands somewhere.
                let row = PlannedRow {
                    depth: 1,
                    role: Role::Item,
                    sub_role: SubRole::Note,
                    title: PlannedTitle::Own(self.title(d)),
                    contents: contents(vec![(
                        ContentRole::NoteText,
                        Some(Body::From(vec![Source::Media { item: d }])),
                    )]),
                    exportable: false,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::OwnText { item: d },
                    why: Why::Media,
                    tag: None,
                };
                self.push(BinderKind::Manuscript, row, Some(0));
            }
            self.card_left_behind(d);
            self.notes_row(BinderKind::Manuscript, d, 1, true, title.text());
            let height = self.book_height(&self.tree.children[d as usize].clone(), true);
            let stop = self.tree.end[d as usize];
            let depth = self.tree.depth[d as usize];
            self.manuscript_range(d + 1, stop, depth, height);
        }
        for b in back {
            self.matter(*b, 1);
        }
        self.book_end();
    }

    /// One book per book-claimed Draft folder (D3.8). A Draft item that is not
    /// a book joins the book it compiles with: the next one before the first
    /// book, the one before it otherwise. Front matter opens the first book and
    /// back matter closes the last, so neither falls outside every book.
    fn several_books(&mut self, draft: u32, books: &[u32], front: &[u32], back: &[u32]) {
        let books: HashSet<u32> = books.iter().copied().collect();
        if self.text(draft) || self.synopsis(draft) || self.card(draft) || self.media(draft) {
            let row = PlannedRow {
                depth: 0,
                role: Role::Item,
                sub_role: SubRole::Note,
                title: PlannedTitle::Own(self.title(draft)),
                contents: contents(vec![
                    (
                        ContentRole::NoteText,
                        body(vec![
                            self.card(draft)
                                .then_some(Source::CardImage { item: draft }),
                            self.media(draft).then_some(Source::Media { item: draft }),
                            self.text_source(draft),
                        ]),
                    ),
                    (ContentRole::SynopsisText, self.synopsis_body(draft)),
                ]),
                exportable: false,
                activated: true,
                exclude_from_numbering: false,
                origin: Origin::DraftContent { item: draft },
                why: Why::Synthesised,
                tag: None,
            };
            self.push(BinderKind::Manuscript, row, None);
            self.notices
                .push(StructureNotice::DraftContentAsNote { item: draft });
        }
        let draft_title = self.title(draft);
        self.notes_row(BinderKind::Manuscript, draft, 0, true, &draft_title);
        self.absence[draft as usize] = Some(Absence::SpecialFolder);
        let draft_depth = self.tree.depth[draft as usize];
        let mut pending: Vec<u32> = Vec::new();
        let mut open = false;
        for c in self.tree.children[draft as usize].clone() {
            if books.contains(&c) {
                if open {
                    self.book_end();
                }
                let title = self.structural_title(c, Rung::Book);
                let synopsis = self.synopsis_body(c);
                let why = self.claims.claim[c as usize]
                    .as_ref()
                    .map_or(Why::Synthesised, |cl| cl.why.clone());
                self.book_row(title, synopsis, Origin::Item { item: c }, why);
                self.edited_notice(c);
                if !open {
                    for f in front {
                        self.matter(*f, 1);
                    }
                }
                let exportable = self.row(c).meta.include_in_compile;
                self.own_text(
                    BinderKind::Manuscript,
                    c,
                    1,
                    SubRole::Scene,
                    exportable,
                    true,
                    None,
                    Some(1),
                );
                self.card_left_behind(c);
                let title = self.title(c);
                self.notes_row(BinderKind::Manuscript, c, 1, true, &title);
                for p in std::mem::take(&mut pending) {
                    let stop = self.tree.end[p as usize];
                    let height = self.book_height(&[p], false);
                    self.manuscript_range(p, stop, draft_depth, height);
                }
                let stop = self.tree.end[c as usize];
                let height = self.book_height(&self.tree.children[c as usize].clone(), true);
                let depth = self.tree.depth[c as usize];
                self.manuscript_range(c + 1, stop, depth, height);
                open = true;
            } else if open {
                let stop = self.tree.end[c as usize];
                let height = self.book_height(&[c], false);
                self.manuscript_range(c, stop, draft_depth, height);
            } else {
                pending.push(c);
            }
        }
        for b in back {
            self.matter(*b, 1);
        }
        self.book_end();
    }

    /// A root folder the writer sent to the manuscript, as another book.
    fn extra_book(&mut self, r: u32) {
        let title = self.structural_title(r, Rung::Book);
        let synopsis = self.synopsis_body(r);
        self.book_row(title, synopsis, Origin::Item { item: r }, Why::Choice);
        self.edited_notice(r);
        let exportable = self.row(r).meta.include_in_compile;
        self.own_text(
            BinderKind::Manuscript,
            r,
            1,
            SubRole::Scene,
            exportable,
            true,
            None,
            Some(1),
        );
        self.card_left_behind(r);
        let title = self.title(r);
        self.notes_row(BinderKind::Manuscript, r, 1, true, &title);
        let stop = self.tree.end[r as usize];
        let height = self.book_height(&self.tree.children[r as usize].clone(), true);
        let depth = self.tree.depth[r as usize];
        self.manuscript_range(r + 1, stop, depth, height);
        self.book_end();
    }

    /// The container height that decides whether a book's first level holds
    /// parts or chapters: the upper median of its containers' heights, not the
    /// tallest, so one chapter holding a sub-folder does not turn every sibling
    /// chapter into a part (D3.3).
    fn book_height(&mut self, children: &[u32], root: bool) -> u16 {
        let mut heights: Vec<u16> = children
            .iter()
            .filter(|c| self.tree.container[**c as usize])
            .map(|c| self.tree.height[*c as usize])
            .collect();
        heights.sort_unstable();
        let Some(&tallest) = heights.last() else {
            return 0;
        };
        let median = heights[heights.len() / 2];
        if !root {
            return median;
        }
        let stats = self.depth_stats.entry(1).or_default();
        if shape_rung(median, 1) != shape_rung(tallest, 1)
            && !stats.flags.contains(&ShapeFlag::MedianDiffersFromMaximum)
        {
            stats.flags.push(ShapeFlag::MedianDiffersFromMaximum);
        }
        if heights.len() == 1 && median >= 2 && !stats.flags.contains(&ShapeFlag::LoneWrapper) {
            stats.flags.push(ShapeFlag::LoneWrapper);
        }
        median
    }

    // ------------------------------------------------ the manuscript

    /// Decide then emit one contiguous range of the manuscript.
    fn manuscript_range(&mut self, first: u32, stop: u32, root_depth: u16, height: u16) {
        let decided = self.decide_range(first, stop, root_depth, height);
        self.gather_review(&decided);
        self.emit_range(&decided);
    }

    fn decide_range(&mut self, first: u32, stop: u32, root_depth: u16, height: u16) -> Decided {
        struct Frame {
            depth: u16,
            indent: u16,
            floor: Rung,
            mode: Mode,
        }
        let mut frames: Vec<Frame> = Vec::new();
        let mut out: Decided = Vec::with_capacity((stop - first) as usize);
        for j in first..stop {
            let d = self.tree.depth[j as usize];
            while frames.last().is_some_and(|f| f.depth >= d) {
                frames.pop();
            }
            let (indent, floor, mode) = frames.last().map_or((1, Rung::Book, Mode::Struct), |f| {
                (f.indent, f.floor, f.mode)
            });
            let rel = d.saturating_sub(root_depth);
            let planned = self.decide(j, rel, indent, floor, mode, height);
            let (child_floor, child_mode) = match &planned.decision {
                Decision::Container { rung, .. } if *rung != Rung::Below => (*rung, Mode::Struct),
                Decision::PartLeaf { .. } => (Rung::Part, Mode::Struct),
                Decision::Paratext { .. } => (floor, Mode::Paratext),
                Decision::Note { .. } => (floor, Mode::Note),
                _ => (floor, Mode::Struct),
            };
            if self.tree.container[j as usize] {
                frames.push(Frame {
                    depth: d,
                    indent: indent + 1,
                    floor: child_floor,
                    mode: child_mode,
                });
            }
            out.push((j, planned));
        }
        // Empty folders take the level at least half their siblings take (D3.4).
        // The siblings' levels are counted once per parent, before any empty
        // folder is settled: an empty sibling counts for no level either way,
        // and counting per folder cost the square of a large sibling group.
        let mut levels: HashMap<u32, (usize, usize)> = HashMap::new();
        for (j, planned) in &out {
            let Some(p) = self.tree.parent[*j as usize] else {
                continue;
            };
            let rung = match &planned.decision {
                Decision::Container { rung, .. } => Some(*rung),
                Decision::FlatChapter { .. } | Decision::SplitFolder { .. } => Some(Rung::Chapter),
                Decision::PartLeaf { .. } => Some(Rung::Part),
                _ => None,
            };
            let entry = levels.entry(p).or_default();
            match rung {
                Some(Rung::Chapter) => entry.0 += 1,
                Some(Rung::Part) => entry.1 += 1,
                _ => {}
            }
        }
        for (j, planned) in &mut out {
            if !matches!(planned.decision, Decision::Empty) {
                continue;
            }
            let j = *j;
            let floor = planned.floor;
            let (siblings, (chapters, parts)) = match self.tree.parent[j as usize] {
                Some(p) => (
                    self.tree.children[p as usize].len().saturating_sub(1),
                    levels.get(&p).copied().unwrap_or_default(),
                ),
                None => (0, (0, 0)),
            };
            let count = |rung: Rung| match rung {
                Rung::Chapter => chapters,
                Rung::Part => parts,
                _ => 0,
            };
            let taken = [Rung::Chapter, Rung::Part]
                .into_iter()
                .find(|r| count(*r) > 0 && count(*r) * 2 >= siblings && *r > floor);
            planned.decision = match taken {
                Some(rung) => Decision::Container {
                    rung,
                    why: Why::EmptyFolder,
                },
                None => {
                    self.notices
                        .push(StructureNotice::EmptyFolderPlain { item: j });
                    Decision::Container {
                        rung: Rung::Below,
                        why: Why::EmptyFolder,
                    }
                }
            };
        }
        out
    }

    fn decide(
        &mut self,
        j: u32,
        rel: u16,
        indent: u16,
        floor: Rung,
        mode: Mode,
        height: u16,
    ) -> Planned {
        let mut planned = Planned {
            decision: Decision::Scene { why: Why::Text },
            indent,
            rel,
            floor,
            overruled: None,
            minority: false,
        };
        let container = self.tree.container[j as usize];
        let kind = self.row(j).kind.clone();
        let claim = self.claims.claim[j as usize].clone();
        if mode == Mode::Paratext {
            planned.decision = Decision::Paratext { why: Why::Inside };
            return planned;
        }
        if mode == Mode::Note {
            planned.decision = Decision::Note { why: Why::Inside };
            return planned;
        }
        // A picture or file cannot be manuscript: a note in place (D3.13).
        if kind.is_media() {
            self.notices.push(StructureNotice::MediaInDraft { item: j });
            planned.decision = Decision::Note { why: Why::Media };
            return planned;
        }
        match claim.as_ref().map(|c| c.level) {
            Some(Level::Paratext) => {
                planned.decision = Decision::Paratext {
                    why: claim.map_or(Why::Inside, |c| c.why),
                };
                return planned;
            }
            Some(Level::Note) => {
                planned.decision = Decision::Note {
                    why: claim.map_or(Why::Inside, |c| c.why),
                };
                return planned;
            }
            _ => {}
        }
        let claim_why = claim.as_ref().map(|c| c.why.clone());
        if matches!(claim.as_ref().map(|c| c.level), Some(Level::Book)) {
            // A book inside a book is a part (D3.8).
            self.notices
                .push(StructureNotice::BookInsideBook { item: j });
        }
        if container {
            self.decide_container(j, rel, floor, height, &claim, claim_why, &mut planned);
            self.split_folder(j, &mut planned);
            return planned;
        }
        // A text.
        let leaf = match claim.as_ref().map(|c| c.level) {
            Some(Level::Chapter) if floor >= Rung::Chapter => {
                planned.overruled = Some(Level::Chapter);
                Decision::Scene { why: Why::Floor }
            }
            Some(Level::Chapter) => Decision::FlatChapter {
                why: claim_why.unwrap_or(Why::Title),
            },
            Some(level @ (Level::Book | Level::Part)) if floor >= Rung::Part => {
                planned.overruled = Some(level);
                Decision::Scene { why: Why::Floor }
            }
            Some(Level::Book | Level::Part) => Decision::PartLeaf {
                why: claim_why.unwrap_or(Why::Title),
            },
            Some(_) => Decision::Scene {
                why: claim_why.unwrap_or(Why::Title),
            },
            None if floor < Rung::Chapter
                && self.choices.texts.get(&rel) == Some(&TextKind::Chapters) =>
            {
                Decision::FlatChapter { why: Why::Choice }
            }
            None => Decision::Scene { why: Why::Text },
        };
        // A document holding its chapters as headings may be split (A2), where a
        // chapter can sit: inside a chapter it can only be reported.
        let headings = self.facts.items[j as usize].text.headings.len();
        let holds_chapters = matches!(leaf, Decision::Scene { .. } | Decision::FlatChapter { .. })
            && headings >= SPLIT_HEADINGS
            && self.facts.items[j as usize].text.carries == crate::content::Carries::Yes;
        if holds_chapters && floor >= Rung::Chapter {
            self.notices.push(StructureNotice::OneDocumentManuscript {
                item: j,
                headings: headings as u32,
                split: false,
            });
        }
        if holds_chapters && floor < Rung::Chapter {
            let chosen = self.choices.splits.get(&j).copied();
            let split = chosen.unwrap_or(true);
            self.splits.push(SplitControl {
                item: j,
                headings: headings as u32,
                detected: true,
                chosen,
            });
            self.notices.push(StructureNotice::OneDocumentManuscript {
                item: j,
                headings: headings as u32,
                split,
            });
            if split {
                let (chapter, why) = match leaf {
                    Decision::FlatChapter { why } => (true, why),
                    Decision::Scene { why } => (false, why),
                    _ => (false, Why::Text),
                };
                planned.decision = Decision::Split { chapter, why };
                return planned;
            }
        }
        planned.decision = leaf;
        planned
    }

    /// The level a container takes: its claim, else the shape, else the
    /// writer's choice for its depth (D3.3, D3.4, D3.7, D3.20).
    #[allow(clippy::too_many_arguments)]
    fn decide_container(
        &mut self,
        j: u32,
        rel: u16,
        floor: Rung,
        height: u16,
        claim: &Option<Claim>,
        claim_why: Option<Why>,
        planned: &mut Planned,
    ) {
        let wanted = match claim.as_ref().map(|c| c.level) {
            Some(Level::Book) | Some(Level::Part) => Some(Rung::Part),
            Some(Level::Chapter) => Some(Rung::Chapter),
            Some(Level::Scene) => {
                planned.decision = Decision::Container {
                    rung: Rung::Below,
                    why: claim_why.unwrap_or(Why::Title),
                };
                return;
            }
            _ => None,
        };
        if let Some(rung) = wanted {
            if rung <= floor {
                planned.overruled = claim.as_ref().map(|c| c.level);
                planned.decision = Decision::Container {
                    rung: Rung::Below,
                    why: Why::Floor,
                };
            } else {
                planned.decision = Decision::Container {
                    rung,
                    why: claim_why.unwrap_or(Why::Title),
                };
            }
            return;
        }
        // Unclaimed: the shape decides.
        if self.tree.children[j as usize].is_empty() && !self.writing(j) {
            planned.decision = Decision::Empty;
            return;
        }
        let (mut rung, mut why) = (shape_rung(height, rel), Why::Shape { height });
        if let Some(chosen) = self.choices.containers.get(&rel) {
            rung = (*chosen).into();
            why = Why::Choice;
        } else if rung != Rung::Below && self.minority(j) {
            rung = Rung::Below;
            why = Why::Minority;
            planned.minority = true;
        }
        if rung != Rung::Below
            && self.claims.below[j as usize].is_some_and(|b| b <= rung_rank(rung))
        {
            rung = Rung::Below;
            why = Why::ClaimsBelow;
        } else if self.children_all_paratext(j) {
            planned.decision = Decision::ParatextFolder;
            return;
        } else if rung != Rung::Below && rung <= floor {
            rung = Rung::Below;
            why = Why::Floor;
        }
        planned.decision = Decision::Container { rung, why };
    }

    /// A childless chapter folder whose own text holds its chapters as headings
    /// (A2, the way one of the measured manuscripts keeps a whole book): split
    /// like a document, the text before the first heading staying its prose.
    fn split_folder(&mut self, j: u32, planned: &mut Planned) {
        let Decision::Container {
            rung: Rung::Chapter,
            why,
        } = &planned.decision
        else {
            return;
        };
        let text = &self.facts.items[j as usize].text;
        if !self.tree.children[j as usize].is_empty()
            || text.headings.len() < SPLIT_HEADINGS
            || text.carries != crate::content::Carries::Yes
        {
            return;
        }
        let headings = text.headings.len() as u32;
        let chosen = self.choices.splits.get(&j).copied();
        let split = chosen.unwrap_or(true);
        self.splits.push(SplitControl {
            item: j,
            headings,
            detected: true,
            chosen,
        });
        self.notices.push(StructureNotice::OneDocumentManuscript {
            item: j,
            headings,
            split,
        });
        if split {
            planned.decision = Decision::SplitFolder { why: why.clone() };
        }
    }

    /// A container is structural by shape only where containers are at least
    /// half of its sibling group: a lone folder among texts would otherwise
    /// become the book's only chapter and swallow every later sibling in the
    /// stream (the critique's measured rule, 150 adopted texts to 51).
    fn minority(&self, j: u32) -> bool {
        let Some(p) = self.tree.parent[j as usize] else {
            return false;
        };
        let p = p as usize;
        (self.tree.child_containers[p] as usize) * 2 < self.tree.children[p].len()
    }

    fn children_all_paratext(&self, j: u32) -> bool {
        let children = &self.tree.children[j as usize];
        !children.is_empty()
            && children.iter().all(|c| {
                self.claims.claim[*c as usize]
                    .as_ref()
                    .is_some_and(|cl| cl.level == Level::Paratext)
            })
    }

    /// The review's per-depth rows, from one decided range.
    fn gather_review(&mut self, decided: &Decided) {
        for (j, planned) in decided {
            let j = *j;
            let rel = planned.rel;
            if rel == 0 || rel > REVIEW_DEPTHS {
                continue;
            }
            let claimed = self.claims.claim[j as usize].is_some();
            let (rung, why, container) = match &planned.decision {
                Decision::Container { rung, why } => (Some(*rung), why.clone(), true),
                Decision::SplitFolder { why } => (Some(Rung::Chapter), why.clone(), true),
                Decision::ParatextFolder => (None, Why::ParatextChildren, true),
                Decision::Scene { why }
                | Decision::FlatChapter { why }
                | Decision::Split { why, .. } => (None, why.clone(), false),
                Decision::PartLeaf { why } => (Some(Rung::Part), why.clone(), true),
                Decision::Paratext { why } | Decision::Note { why } => {
                    (None, why.clone(), self.tree.container[j as usize])
                }
                Decision::Empty => (None, Why::EmptyFolder, true),
            };
            if container {
                let is_folder_row = matches!(
                    planned.decision,
                    Decision::Container { .. }
                        | Decision::ParatextFolder
                        | Decision::SplitFolder { .. }
                );
                if !is_folder_row {
                    continue;
                }
                let stats = self.depth_stats.entry(rel).or_default();
                stats.containers += 1;
                *stats.signals.entry(why.signal()).or_default() += 1;
                let final_rung = rung.map_or(ContainerRung::Plain, container_rung);
                *stats.rungs.entry(final_rung).or_default() += 1;
                let adjustable =
                    !claimed && !matches!(why, Why::EmptyFolder | Why::ParatextChildren);
                if adjustable {
                    stats.adjustable += 1;
                    *stats.adjustable_rungs.entry(final_rung).or_default() += 1;
                }
                if stats.examples.len() < 2 {
                    stats.examples.push(j);
                }
                if planned.minority && !stats.flags.contains(&ShapeFlag::MinorityContainers) {
                    stats.flags.push(ShapeFlag::MinorityContainers);
                }
            } else if !claimed
                && planned.floor < Rung::Chapter
                && matches!(
                    planned.decision,
                    Decision::Scene { .. } | Decision::FlatChapter { .. } | Decision::Split { .. }
                )
            {
                let words = self.facts.items[j as usize].text.words;
                let stats = self.text_stats.entry(rel).or_default();
                stats.texts += 1;
                stats.words.push(words);
                if stats.examples.len() < 2 {
                    stats.examples.push(j);
                }
            }
        }
    }

    fn emit_range(&mut self, decided: &Decided) {
        for (j, planned) in decided {
            self.emit(*j, planned);
        }
    }

    fn emit(&mut self, j: u32, planned: &Planned) {
        let ms = BinderKind::Manuscript;
        let depth = planned.indent;
        let rel = Some(planned.rel);
        let row = self.row(j);
        let exportable = row.meta.include_in_compile;
        let page_break = row.meta.page_break_before;
        let as_is = row.meta.compile_as_is;
        let is_text_kind = !is_folder_kind(&row.kind);
        let prologue = self.claims.prologue[j as usize];
        let title = self.title(j);
        if let Some(level) = planned.overruled {
            self.notices.push(StructureNotice::ClaimOverruled {
                item: j,
                claimed: level,
            });
        }
        if as_is {
            self.notices
                .push(StructureNotice::CompileAsIsNotKept { item: j });
        }
        self.edited_notice(j);
        let container = self.tree.container[j as usize];
        let mut opens_heading = false;
        match planned.decision.clone() {
            Decision::Container {
                rung: Rung::Chapter,
                why,
            } => {
                let rung = Rung::Chapter;
                opens_heading = true;
                let title_value = self.structural_title(j, rung);
                let row = PlannedRow {
                    depth,
                    role: Role::Folder,
                    sub_role: SubRole::ChapterScene,
                    title: title_value,
                    contents: contents(vec![
                        (ContentRole::ChapterTitle, Some(Body::Title)),
                        (
                            ContentRole::SceneText,
                            self.text_source(j).map(|s| Body::From(vec![s])),
                        ),
                        (ContentRole::SynopsisText, self.synopsis_body(j)),
                    ]),
                    exportable,
                    activated: true,
                    exclude_from_numbering: prologue,
                    origin: Origin::Item { item: j },
                    why,
                    tag: None,
                };
                self.push(ms, row, rel);
                self.chapter_notices(j, prologue, exportable);
                if is_text_kind {
                    self.notices
                        .push(StructureNotice::TextWithChildren { item: j });
                }
                self.card_left_behind(j);
                self.notes_row(ms, j, depth + 1, true, &title);
            }
            Decision::Container { rung, why } => {
                let (sub_role, heading) = match rung {
                    Rung::Book | Rung::Part => (SubRole::Part, Some(ContentRole::PartTitle)),
                    _ => (SubRole::None, None),
                };
                opens_heading = heading.is_some();
                let title_value = if heading.is_some() {
                    self.structural_title(j, rung)
                } else {
                    PlannedTitle::Own(title.clone())
                };
                let mut parts = Vec::new();
                if let Some(heading) = &heading {
                    parts.push((heading.clone(), Some(Body::Title)));
                }
                parts.push((ContentRole::SynopsisText, self.synopsis_body(j)));
                let row = PlannedRow {
                    depth,
                    role: Role::Folder,
                    sub_role,
                    title: title_value,
                    contents: contents(parts),
                    exportable,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::Item { item: j },
                    why,
                    tag: None,
                };
                self.push(ms, row, rel);
                if heading.is_some() && !exportable {
                    self.notices.push(StructureNotice::NotCompiled { item: j });
                }
                if is_text_kind && container {
                    self.notices
                        .push(StructureNotice::TextWithChildren { item: j });
                }
                self.own_text(
                    ms,
                    j,
                    depth + 1,
                    SubRole::Scene,
                    exportable,
                    true,
                    None,
                    rel,
                );
                self.card_left_behind(j);
                self.notes_row(ms, j, depth + 1, true, &title);
            }
            Decision::ParatextFolder => {
                self.paratext_folder(j, depth, exportable, Why::ParatextChildren, rel);
            }
            Decision::Empty => {}
            Decision::FlatChapter { why } => {
                opens_heading = true;
                let title_value = self.structural_title(j, Rung::Chapter);
                let row = PlannedRow {
                    depth,
                    role: Role::Item,
                    sub_role: SubRole::ChapterScene,
                    title: title_value,
                    contents: contents(vec![
                        (ContentRole::ChapterTitle, Some(Body::Title)),
                        (
                            ContentRole::SceneText,
                            self.text_source(j).map(|s| Body::From(vec![s])),
                        ),
                        (ContentRole::SynopsisText, self.synopsis_body(j)),
                    ]),
                    exportable,
                    activated: true,
                    exclude_from_numbering: prologue,
                    origin: Origin::Item { item: j },
                    why,
                    tag: None,
                };
                self.push(ms, row, rel);
                self.chapter_notices(j, prologue, exportable);
                self.card_left_behind(j);
                self.notes_row(ms, j, depth, true, &title);
            }
            Decision::PartLeaf { why } => {
                opens_heading = true;
                let title_value = self.structural_title(j, Rung::Part);
                let row = PlannedRow {
                    depth,
                    role: Role::Folder,
                    sub_role: SubRole::Part,
                    title: title_value,
                    contents: contents(vec![
                        (ContentRole::PartTitle, Some(Body::Title)),
                        (ContentRole::SynopsisText, self.synopsis_body(j)),
                    ]),
                    exportable,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::Item { item: j },
                    why,
                    tag: None,
                };
                self.push(ms, row, rel);
                if !exportable {
                    self.notices.push(StructureNotice::NotCompiled { item: j });
                }
                self.own_text(
                    ms,
                    j,
                    depth + 1,
                    SubRole::Scene,
                    exportable,
                    true,
                    None,
                    rel,
                );
                self.card_left_behind(j);
                self.notes_row(ms, j, depth + 1, true, &title);
            }
            Decision::Scene { why } => {
                let row = PlannedRow {
                    depth,
                    role: Role::Item,
                    sub_role: SubRole::Scene,
                    title: PlannedTitle::Own(title.clone()),
                    contents: contents(vec![
                        (
                            ContentRole::SceneText,
                            self.text_source(j).map(|s| Body::From(vec![s])),
                        ),
                        (ContentRole::SynopsisText, self.synopsis_body(j)),
                    ]),
                    exportable,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::Item { item: j },
                    why,
                    tag: None,
                };
                self.push(ms, row, rel);
                if !exportable {
                    self.notices.push(StructureNotice::NotCompiled { item: j });
                }
                self.card_left_behind(j);
                self.notes_row(ms, j, depth, true, &title);
            }
            Decision::Split { chapter, why } => {
                self.split(j, depth, chapter, why, exportable, rel);
            }
            Decision::SplitFolder { why } => {
                opens_heading = true;
                let title_value = self.structural_title(j, Rung::Chapter);
                let text = &self.facts.items[j as usize].text;
                let headings: Vec<String> = text.headings.iter().map(|h| h.title.clone()).collect();
                let first = if text.lead.counts() { 0 } else { 1 };
                let row = PlannedRow {
                    depth,
                    role: Role::Folder,
                    sub_role: SubRole::ChapterScene,
                    title: title_value,
                    contents: contents(vec![
                        (ContentRole::ChapterTitle, Some(Body::Title)),
                        (
                            ContentRole::SceneText,
                            Some(Body::From(vec![Source::TextPart {
                                item: j,
                                part: first,
                            }])),
                        ),
                        (ContentRole::SynopsisText, self.synopsis_body(j)),
                    ]),
                    exportable,
                    activated: true,
                    exclude_from_numbering: prologue,
                    origin: Origin::Item { item: j },
                    why,
                    tag: None,
                };
                self.push(ms, row, rel);
                self.chapter_notices(j, prologue, exportable);
                self.card_left_behind(j);
                self.notes_row(ms, j, depth + 1, true, &title);
                for (k, heading) in headings.iter().enumerate() {
                    let part = k as u32 + 1;
                    if part <= first {
                        continue;
                    }
                    let row = PlannedRow {
                        depth,
                        role: Role::Item,
                        sub_role: SubRole::ChapterScene,
                        title: PlannedTitle::Own(heading.clone()),
                        contents: contents(vec![
                            (ContentRole::ChapterTitle, Some(Body::Title)),
                            (
                                ContentRole::SceneText,
                                Some(Body::From(vec![Source::TextPart { item: j, part }])),
                            ),
                        ]),
                        exportable,
                        activated: true,
                        exclude_from_numbering: false,
                        origin: Origin::SplitPart { item: j, part },
                        why: Why::Heading,
                        tag: None,
                    };
                    self.push(ms, row, rel);
                }
            }
            Decision::Paratext { why } => {
                if container {
                    self.paratext_folder(j, depth, exportable, why, rel);
                } else {
                    let row = PlannedRow {
                        depth,
                        role: Role::Item,
                        sub_role: SubRole::Paratext,
                        title: PlannedTitle::Own(title.clone()),
                        contents: contents(vec![
                            (
                                ContentRole::ParatextText,
                                body(vec![
                                    self.media(j).then_some(Source::Media { item: j }),
                                    self.text_source(j),
                                ]),
                            ),
                            (ContentRole::SynopsisText, self.synopsis_body(j)),
                        ]),
                        exportable,
                        activated: true,
                        exclude_from_numbering: false,
                        origin: Origin::Item { item: j },
                        why,
                        tag: None,
                    };
                    self.push(ms, row, rel);
                    if !exportable {
                        self.notices.push(StructureNotice::NotCompiled { item: j });
                    }
                    self.card_left_behind(j);
                    self.notes_row(ms, j, depth, true, &title);
                }
            }
            Decision::Note { why } => {
                self.note_item(ms, j, depth, true, None, why, rel);
            }
        }
        if page_break && !opens_heading {
            self.notices
                .push(StructureNotice::PageBreakNotKept { item: j });
        }
    }

    fn chapter_notices(&mut self, j: u32, prologue: bool, exportable: bool) {
        if prologue {
            self.notices
                .push(StructureNotice::PrologueNotNumbered { item: j });
        }
        if !exportable {
            self.notices.push(StructureNotice::NotCompiled { item: j });
        }
    }

    fn edited_notice(&mut self, j: u32) {
        if self.edited[j as usize] {
            self.notices.push(StructureNotice::TitleCleaned { item: j });
        }
    }

    /// A paratext folder: its own text goes to a first paratext child (D3.7).
    fn paratext_folder(
        &mut self,
        j: u32,
        depth: u16,
        exportable: bool,
        why: Why,
        rel: Option<u16>,
    ) {
        let ms = BinderKind::Manuscript;
        let title = self.title(j);
        let row = PlannedRow {
            depth,
            role: Role::Folder,
            sub_role: SubRole::Paratext,
            title: PlannedTitle::Own(title.clone()),
            contents: contents(vec![(ContentRole::SynopsisText, self.synopsis_body(j))]),
            exportable,
            activated: true,
            exclude_from_numbering: false,
            origin: Origin::Item { item: j },
            why,
            tag: None,
        };
        self.push(ms, row, rel);
        self.own_text(
            ms,
            j,
            depth + 1,
            SubRole::Paratext,
            exportable,
            true,
            None,
            rel,
        );
        self.card_left_behind(j);
        self.notes_row(ms, j, depth + 1, true, &title);
    }

    /// A note item in either binder: a folder with its own writing as a first
    /// child, or a note holding its card image, its file and its text.
    #[allow(clippy::too_many_arguments)]
    fn note_item(
        &mut self,
        binder: BinderKind,
        j: u32,
        depth: u16,
        activated: bool,
        entry_tag: Option<u32>,
        why: Why,
        rel: Option<u16>,
    ) {
        let title = self.title(j);
        if self.tree.container[j as usize] {
            let row = PlannedRow {
                depth,
                role: Role::Folder,
                sub_role: SubRole::Note,
                title: PlannedTitle::Own(title.clone()),
                contents: contents(vec![(ContentRole::SynopsisText, self.synopsis_body(j))]),
                exportable: false,
                activated,
                exclude_from_numbering: false,
                origin: Origin::Item { item: j },
                why,
                tag: None,
            };
            self.push(binder, row, rel);
            // A text with entries under it is an entry itself; a folder's own text
            // is not (it would make the folder's name a mention target).
            let own_tag = if is_folder_kind(&self.row(j).kind) {
                None
            } else {
                entry_tag
            };
            if !is_folder_kind(&self.row(j).kind) {
                self.notices
                    .push(StructureNotice::TextWithChildren { item: j });
            }
            self.own_text(
                binder,
                j,
                depth + 1,
                SubRole::Note,
                false,
                activated,
                own_tag,
                rel,
            );
            self.notes_row(binder, j, depth + 1, activated, &title);
        } else {
            let row = PlannedRow {
                depth,
                role: Role::Item,
                sub_role: SubRole::Note,
                title: PlannedTitle::Own(title.clone()),
                contents: contents(vec![
                    (
                        ContentRole::NoteText,
                        body(vec![
                            self.card(j).then_some(Source::CardImage { item: j }),
                            self.media(j).then_some(Source::Media { item: j }),
                            self.text_source(j),
                        ]),
                    ),
                    (ContentRole::SynopsisText, self.synopsis_body(j)),
                ]),
                exportable: false,
                activated,
                exclude_from_numbering: false,
                origin: Origin::Item { item: j },
                why,
                tag: entry_tag,
            };
            self.push(binder, row, rel);
            if entry_tag.is_some() && lexicon::is_generic_sheet_title(&title) {
                self.notices
                    .push(StructureNotice::GenericSheetTitle { item: j });
            }
            self.notes_row(binder, j, depth, activated, &title);
        }
    }

    /// A one-document manuscript split at its chapter headings (A2): what comes
    /// before the first heading stays the document, each heading opens a
    /// chapter titled with its words.
    fn split(
        &mut self,
        j: u32,
        depth: u16,
        chapter: bool,
        why: Why,
        exportable: bool,
        rel: Option<u16>,
    ) {
        let ms = BinderKind::Manuscript;
        let title = self.title(j);
        let text = &self.facts.items[j as usize].text;
        let headings: Vec<String> = text.headings.iter().map(|h| h.title.clone()).collect();
        let lead = text.lead.counts();
        let mut first = true;
        let mut synopsis = self.synopsis_body(j);
        if lead {
            let (role, sub_role, mut parts) = if chapter {
                (
                    Role::Item,
                    SubRole::ChapterScene,
                    vec![(ContentRole::ChapterTitle, Some(Body::Title))],
                )
            } else {
                (Role::Item, SubRole::Scene, Vec::new())
            };
            parts.push((
                ContentRole::SceneText,
                Some(Body::From(vec![Source::TextPart { item: j, part: 0 }])),
            ));
            parts.push((ContentRole::SynopsisText, synopsis.take()));
            let title_value = if chapter {
                self.structural_title(j, Rung::Chapter)
            } else {
                PlannedTitle::Own(title.clone())
            };
            let row = PlannedRow {
                depth,
                role,
                sub_role,
                title: title_value,
                contents: contents(parts),
                exportable,
                activated: true,
                exclude_from_numbering: false,
                origin: Origin::Item { item: j },
                why: why.clone(),
                tag: None,
            };
            self.push(ms, row, rel);
            first = false;
        }
        for (k, heading) in headings.iter().enumerate() {
            let part = k as u32 + 1;
            let row = PlannedRow {
                depth,
                role: Role::Item,
                sub_role: SubRole::ChapterScene,
                title: PlannedTitle::Own(heading.clone()),
                contents: contents(vec![
                    (ContentRole::ChapterTitle, Some(Body::Title)),
                    (
                        ContentRole::SceneText,
                        Some(Body::From(vec![Source::TextPart { item: j, part }])),
                    ),
                    (ContentRole::SynopsisText, synopsis.take()),
                ]),
                exportable,
                activated: true,
                exclude_from_numbering: false,
                origin: if first {
                    Origin::Item { item: j }
                } else {
                    Origin::SplitPart { item: j, part }
                },
                why: Why::Heading,
                tag: None,
            };
            first = false;
            self.push(ms, row, rel);
        }
        self.card_left_behind(j);
        self.notes_row(ms, j, depth, true, &title);
    }

    /// Root front or back matter, inside the book and out of export (D3.11).
    fn matter(&mut self, root: u32, indent: u16) {
        let ms = BinderKind::Manuscript;
        let back = self.matter_side(root) == Some(MatterSide::Back);
        self.notices
            .push(StructureNotice::MatterInsideBook { item: root, back });
        let stop = self.tree.end[root as usize];
        let mut frames: Vec<(u16, u16)> = Vec::new();
        for j in root..stop {
            let d = self.tree.depth[j as usize];
            while frames.last().is_some_and(|(fd, _)| *fd >= d) {
                frames.pop();
            }
            let depth = frames.last().map_or(indent, |(_, i)| *i);
            self.edited_notice(j);
            let title = self.title(j);
            if self.tree.container[j as usize] {
                let row = PlannedRow {
                    depth,
                    role: Role::Folder,
                    sub_role: SubRole::Paratext,
                    title: PlannedTitle::Own(title.clone()),
                    contents: contents(vec![(ContentRole::SynopsisText, self.synopsis_body(j))]),
                    exportable: false,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::Item { item: j },
                    why: if j == root {
                        Why::Matter { back }
                    } else {
                        Why::Inside
                    },
                    tag: None,
                };
                self.push(ms, row, None);
                self.own_text(ms, j, depth + 1, SubRole::Paratext, false, true, None, None);
                self.card_left_behind(j);
                self.notes_row(ms, j, depth + 1, true, &title);
                frames.push((d, depth + 1));
            } else {
                let row = PlannedRow {
                    depth,
                    role: Role::Item,
                    sub_role: SubRole::Paratext,
                    title: PlannedTitle::Own(title.clone()),
                    contents: contents(vec![
                        (
                            ContentRole::ParatextText,
                            body(vec![
                                self.media(j).then_some(Source::Media { item: j }),
                                self.text_source(j),
                            ]),
                        ),
                        (ContentRole::SynopsisText, self.synopsis_body(j)),
                    ]),
                    exportable: false,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::Item { item: j },
                    why: if j == root {
                        Why::Matter { back }
                    } else {
                        Why::Inside
                    },
                    tag: None,
                };
                self.push(ms, row, None);
                self.card_left_behind(j);
                self.notes_row(ms, j, depth, true, &title);
            }
        }
    }

    // ------------------------------------------------ the notes binder

    /// The notes rules (D3.12) over one subtree: every folder a notes folder,
    /// every leaf a note, story-bible folders tagging their entries (D3.14).
    fn walk_notes(
        &mut self,
        first: u32,
        stop: u32,
        indent: u16,
        activated: bool,
        bible_root: bool,
    ) {
        let nb = BinderKind::Notes;
        // Open ancestors: source depth, indent for children, the tag entries take.
        let mut frames: Vec<(u16, u16, Option<u32>)> = Vec::new();
        let mut j = first;
        while j < stop {
            let d = self.tree.depth[j as usize];
            while frames.last().is_some_and(|(fd, _, _)| *fd >= d) {
                frames.pop();
            }
            let (depth, entry_tag) = frames.last().map_or((indent, None), |(_, i, t)| (*i, *t));
            if activated && Some(j) == self.templates_folder {
                self.templates(j, depth);
                j = self.tree.end[j as usize];
                continue;
            }
            self.edited_notice(j);
            let recognised = (bible_root && j == first && self.tree.container[j as usize])
                || self.is_bible_folder(j, !activated);
            let why = if recognised {
                Why::StoryBible
            } else if self.row(j).kind.is_media() {
                Why::Media
            } else {
                Why::Notes
            };
            if self.tree.container[j as usize] {
                let row_index = self.notes.len() as u32;
                let own_tag = recognised.then(|| self.tag_for(j, row_index));
                self.note_item(nb, j, depth, activated, entry_tag, why, None);
                frames.push((d, depth + 1, own_tag.or(entry_tag)));
            } else {
                self.note_item(nb, j, depth, activated, entry_tag, why, None);
            }
            j += 1;
        }
    }

    /// The tag a recognised folder names, merged with any earlier folder of
    /// the same title (NFC, case-folded).
    fn tag_for(&mut self, folder: u32, row: u32) -> u32 {
        let name = self.title(folder);
        let key: String = name.nfc().collect::<String>().to_lowercase();
        if let Some(&tag) = self.tag_by_name.get(&key) {
            self.tag_folders[tag as usize].push(folder);
            return tag;
        }
        let tag = self.tags.len() as u32;
        self.tags.push(PlannedTag {
            name,
            folder,
            creates_in: row,
            template: None,
        });
        self.tag_folders.push(vec![folder]);
        self.tag_by_name.insert(key, tag);
        tag
    }

    /// The Templates folder: one note template per text, and a note for what a
    /// template cannot hold (D3.16). The folder itself has no row.
    fn templates(&mut self, t: u32, indent: u16) {
        let nb = BinderKind::Notes;
        self.absence[t as usize] = Some(Absence::SpecialFolder);
        // The folder's own writing, which no template holds.
        let own = body(vec![
            self.card(t).then_some(Source::CardImage { item: t }),
            self.media(t).then_some(Source::Media { item: t }),
            self.text_source(t),
        ]);
        let synopsis = self.synopsis_body(t);
        let title_t = self.title(t);
        if own.is_some() || synopsis.is_some() {
            let row = PlannedRow {
                depth: indent,
                role: Role::Item,
                sub_role: SubRole::Note,
                title: PlannedTitle::Own(title_t.clone()),
                contents: contents(vec![
                    (ContentRole::NoteText, own),
                    (ContentRole::SynopsisText, synopsis),
                ]),
                exportable: false,
                activated: true,
                exclude_from_numbering: false,
                origin: Origin::TemplateLeftover { item: t },
                why: Why::Synthesised,
                tag: None,
            };
            self.push(nb, row, None);
            self.notices
                .push(StructureNotice::TemplateKeptAsNote { item: t });
        }
        self.notes_row(nb, t, indent, true, &title_t);
        let stop = self.tree.end[t as usize];
        let before = self.templates.len();
        for k in t + 1..stop {
            let kind = self.row(k).kind.clone();
            let text = self.text(k);
            let becomes = matches!(kind, ItemKind::Text | ItemKind::Unknown(_))
                || (kind == ItemKind::Folder && text);
            if becomes {
                self.template_of.insert(k, self.templates.len() as u32);
                self.templates.push(PlannedTemplate {
                    name: self.title(k),
                    item: k,
                    body: self.text_source(k),
                });
            }
            self.absence[k as usize] = Some(Absence::Template);
            if !self.tree.children[k as usize].is_empty() {
                self.notices
                    .push(StructureNotice::TemplateNestingFlattened { item: k });
            }
            let leftover = body(vec![
                self.card(k).then_some(Source::CardImage { item: k }),
                self.media(k).then_some(Source::Media { item: k }),
                (!becomes).then(|| self.text_source(k)).flatten(),
            ]);
            let synopsis = self.synopsis_body(k);
            let title = self.title(k);
            if leftover.is_some() || synopsis.is_some() || self.has_notes(k) {
                self.notices
                    .push(StructureNotice::TemplateKeptAsNote { item: k });
            }
            if leftover.is_some() || synopsis.is_some() {
                let row = PlannedRow {
                    depth: indent,
                    role: Role::Item,
                    sub_role: SubRole::Note,
                    title: PlannedTitle::Own(title.clone()),
                    contents: contents(vec![
                        (ContentRole::NoteText, leftover),
                        (ContentRole::SynopsisText, synopsis),
                    ]),
                    exportable: false,
                    activated: true,
                    exclude_from_numbering: false,
                    origin: Origin::TemplateLeftover { item: k },
                    why: Why::Synthesised,
                    tag: None,
                };
                self.push(nb, row, None);
            }
            self.notes_row(nb, k, indent, true, &title);
        }
        let count = (self.templates.len() - before) as u32;
        if count > 0 {
            self.notices
                .push(StructureNotice::TemplatesCreated { count });
        }
    }

    /// Format 1.x project notes, in one notes folder (D3.15). Empty ones are
    /// skipped.
    fn project_notes(&mut self) {
        let kept: Vec<u32> = (0..self.project.project_notes.len() as u32)
            .filter(|k| self.facts.project_notes[*k as usize].counts())
            .collect();
        if kept.is_empty() {
            return;
        }
        let nb = BinderKind::Notes;
        self.push(
            nb,
            synthetic_folder(PlannedTitle::ProjectNotes, Origin::ProjectNotes),
            None,
        );
        for k in &kept {
            let note = &self.project.project_notes[*k as usize];
            let row = PlannedRow {
                depth: 1,
                role: Role::Item,
                sub_role: SubRole::Note,
                title: PlannedTitle::Own(clean_title(note.title.as_deref()).0),
                contents: contents(vec![(
                    ContentRole::NoteText,
                    Some(Body::From(vec![Source::ProjectNote { note: *k }])),
                )]),
                exportable: false,
                activated: true,
                exclude_from_numbering: false,
                origin: Origin::ProjectNote { note: *k },
                why: Why::Notes,
                tag: None,
            };
            self.push(nb, row, None);
        }
        self.notices.push(StructureNotice::ProjectNotes {
            count: kept.len() as u32,
        });
    }

    /// Texts found on disk that no binder item listed, in one notes folder,
    /// each titled after the item it was a copy of (X-18, X-29).
    fn recovered(&mut self) {
        let kept: Vec<u32> = (0..self.project.recovered.len() as u32)
            .filter(|k| self.facts.recovered[*k as usize].counts())
            .collect();
        if kept.is_empty() {
            return;
        }
        let nb = BinderKind::Notes;
        self.push(
            nb,
            synthetic_folder(PlannedTitle::Recovered, Origin::Recovered),
            None,
        );
        let mut owner: HashMap<ItemKey, usize> = HashMap::new();
        for (i, row) in self.project.binder.iter().enumerate() {
            if row.owns_key
                && let Some(key) = row.key
            {
                owner.entry(key).or_insert(i);
            }
        }
        for k in &kept {
            let text = &self.project.recovered[*k as usize];
            // After the item it is a copy of (a copy of its document notes says so), else
            // the file's own name when it is a Scrivener document number (`630.rtf`), else
            // blank: the other names Scrivener gives files (`content.rtf`) are English words
            // it chose, and a sync copy's name may carry a person's or a computer's.
            let title = match text.names.and_then(|key| owner.get(&key).copied()) {
                Some(i) if text.role == TextRole::Notes => {
                    PlannedTitle::NotesOf(self.titles[i].clone())
                }
                Some(i) => PlannedTitle::Own(self.titles[i].clone()),
                None => PlannedTitle::Own(crate::names::recovered_number(&text.member.path)),
            };
            let row = PlannedRow {
                depth: 1,
                role: Role::Item,
                sub_role: SubRole::Note,
                title,
                contents: contents(vec![(
                    ContentRole::NoteText,
                    Some(Body::From(vec![Source::Recovered { text: *k }])),
                )]),
                exportable: false,
                activated: true,
                exclude_from_numbering: false,
                origin: Origin::RecoveredText { text: *k },
                why: Why::Notes,
                tag: None,
            };
            self.push(nb, row, None);
        }
        self.notices.push(StructureNotice::RecoveredTexts {
            count: kept.len() as u32,
        });
    }

    /// The Trash: its items as deleted rows at the end of the notes binder, one
    /// trash entry per root, or left out (D3.17).
    fn trash_binder(&mut self) {
        let Some(t) = self.trash else {
            return;
        };
        self.absence[t as usize] = Some(Absence::SpecialFolder);
        let stop = self.tree.end[t as usize];
        let items = stop - t - 1;
        let with_writing = (t + 1..stop).filter(|i| self.writing(*i)).count() as u32;
        let own =
            self.text(t) || self.synopsis(t) || self.card(t) || self.media(t) || self.has_notes(t);
        if items == 0 && !own {
            return;
        }
        match self.choices.trash {
            TrashChoice::Import => {
                let nb = BinderKind::Notes;
                let title = self.title(t);
                if own && (self.text(t) || self.synopsis(t) || self.card(t) || self.media(t)) {
                    // Writing on the Trash folder itself: a deleted note of its own.
                    let root = self.notes.len() as u32;
                    let row = PlannedRow {
                        depth: 0,
                        role: Role::Item,
                        sub_role: SubRole::Note,
                        title: PlannedTitle::Own(title.clone()),
                        contents: contents(vec![
                            (
                                ContentRole::NoteText,
                                body(vec![
                                    self.card(t).then_some(Source::CardImage { item: t }),
                                    self.media(t).then_some(Source::Media { item: t }),
                                    self.text_source(t),
                                ]),
                            ),
                            (ContentRole::SynopsisText, self.synopsis_body(t)),
                        ]),
                        exportable: false,
                        activated: false,
                        exclude_from_numbering: false,
                        origin: Origin::OwnText { item: t },
                        why: Why::Notes,
                        tag: None,
                    };
                    self.push(nb, row, None);
                    self.trash_roots.push(root);
                }
                if self.has_notes(t) {
                    let root = self.notes.len() as u32;
                    self.notes_row(nb, t, 0, false, &title);
                    self.trash_roots.push(root);
                }
                for c in self.tree.children[t as usize].clone() {
                    let root = self.notes.len() as u32;
                    let stop = self.tree.end[c as usize];
                    self.walk_notes(c, stop, 0, false, false);
                    // A root can end with its notes row, a sibling at depth 0:
                    // each depth-0 row it made is a trash entry of its own.
                    let end = self.notes.len() as u32;
                    for r in root..end {
                        if self.notes[r as usize].depth == 0 {
                            self.trash_roots.push(r);
                        }
                    }
                }
                if items > 0 {
                    self.notices.push(StructureNotice::TrashBroughtAcross {
                        items,
                        with_writing,
                    });
                }
            }
            TrashChoice::LeaveOut => {
                for i in t + 1..stop {
                    self.absence[i as usize] = Some(Absence::TrashLeftOut);
                }
                if own {
                    self.absence[t as usize] = Some(Absence::TrashLeftOut);
                }
                self.notices.push(StructureNotice::TrashLeftOut {
                    items,
                    with_writing,
                });
            }
        }
    }

    // ------------------------------------------------ afterwards

    fn finish(mut self) -> StructurePlan {
        self.untitled_notices();
        self.adoption();
        self.link_templates();
        self.draft_notices();
        let shape = self.shape();
        let chapter_encoding = if shape.flat_chapters > shape.chapter_folders {
            ChapterEncoding::Flat
        } else {
            ChapterEncoding::Folder
        };
        let placements = placements(
            self.project.binder.len(),
            &self.manuscript,
            &self.notes,
            &self.templates,
            &self.absence,
        );
        let review = self.review();
        StructurePlan {
            choices: self.choices.clone(),
            manuscript: self.manuscript,
            notes: self.notes,
            templates: self.templates,
            tags: self.tags,
            trash_roots: self.trash_roots,
            chapter_encoding,
            placements,
            review,
            shape,
            notices: self.notices,
        }
    }

    /// Items with no title at all, each once, wherever their own row went: the Draft,
    /// Research and Trash folders aside, which Scrivener always names.
    fn untitled_notices(&mut self) {
        let mut said: HashSet<u32> = HashSet::new();
        let mut untitled = Vec::new();
        for row in self.manuscript.iter().chain(&self.notes) {
            let Origin::Item { item } = row.origin else {
                continue;
            };
            let source = &self.project.binder[item as usize];
            if self.titles[item as usize].is_empty()
                && !matches!(
                    source.kind,
                    ItemKind::DraftFolder | ItemKind::ResearchFolder | ItemKind::TrashFolder
                )
                && source.title.as_deref().is_none_or(|t| t.trim().is_empty())
                && said.insert(item)
            {
                untitled.push(StructureNotice::UntitledItem { item });
            }
        }
        self.notices.extend(untitled);
    }

    /// Texts that follow a chapter among their siblings end that chapter in the
    /// stream: say which.
    fn adoption(&mut self) {
        // Open ancestors: indent, the chapter seen last among its children.
        let mut frames: Vec<(u16, Option<u32>)> = Vec::new();
        let mut flagged: Vec<u16> = Vec::new();
        for r in 0..self.manuscript.len() {
            let row = &self.manuscript[r];
            while frames.last().is_some_and(|(d, _)| *d >= row.depth) {
                frames.pop();
            }
            let chapter = row.sub_role == SubRole::ChapterScene;
            let is_scene = row.sub_role == SubRole::Scene;
            if let Some((_, last)) = frames.last_mut() {
                if chapter {
                    *last = row_item(row);
                } else if matches!(row.sub_role, SubRole::Part | SubRole::Book) {
                    *last = None;
                } else if is_scene
                    && let Origin::Item { item } = row.origin
                    && let Some(ch) = *last
                {
                    self.notices
                        .push(StructureNotice::AdoptedByChapter { item, chapter: ch });
                    if let Some(rel) = self.ms_rel.get(r).copied().flatten()
                        && !flagged.contains(&rel)
                    {
                        flagged.push(rel);
                    }
                }
            }
            frames.push((row.depth, None));
        }
        for rel in flagged {
            let stats = self.depth_stats.entry(rel).or_default();
            if !stats.flags.contains(&ShapeFlag::AdoptsTexts) {
                stats.flags.push(ShapeFlag::AdoptsTexts);
            }
        }
    }

    /// Story-bible tags start new entries from the template their folder names;
    /// a default template no tag can carry is reported (D3.14, D3.16).
    fn link_templates(&mut self) {
        let by_key: HashMap<ItemKey, u32> = self
            .project
            .binder
            .iter()
            .enumerate()
            .filter(|(_, r)| r.owns_key)
            .filter_map(|(i, r)| r.key.map(|k| (k, i as u32)))
            .collect();
        let template_named = |key: ItemKey| {
            by_key
                .get(&key)
                .and_then(|i| self.template_of.get(i).copied())
        };
        let mut linked_folders: HashSet<u32> = HashSet::new();
        for (tag, folders) in self.tag_folders.iter().enumerate() {
            for folder in folders {
                if let Some(template) = self.project.binder[*folder as usize]
                    .default_child_template
                    .and_then(template_named)
                {
                    if self.tags[tag].template.is_none() {
                        self.tags[tag].template = Some(template);
                    }
                    linked_folders.insert(*folder);
                }
            }
        }
        for (tag, folders) in self.tag_folders.iter().enumerate() {
            for folder in folders {
                let template = linked_folders.contains(folder) && self.tags[tag].template.is_some();
                self.notices.push(StructureNotice::StoryBibleFolder {
                    item: *folder,
                    template,
                });
            }
        }
        for (i, row) in self.project.binder.iter().enumerate() {
            let i = i as u32;
            if row
                .default_child_template
                .and_then(template_named)
                .is_some()
                && !linked_folders.contains(&i)
                && self.absence[i as usize].is_none()
            {
                self.notices
                    .push(StructureNotice::DefaultTemplateNotLinked { item: i });
            }
        }
    }

    /// What the Draft as a whole says: a script (A12), words that stop counting
    /// as manuscript (D3.7).
    fn draft_notices(&mut self) {
        let Some(draft) = self.draft else {
            return;
        };
        let stop = self.tree.end[draft as usize];
        let texts: Vec<u32> = (draft + 1..stop)
            .filter(|i| self.row(*i).kind == ItemKind::Text)
            .collect();
        let script = texts.iter().filter(|i| self.row(**i).script_mode).count() as u32;
        if !texts.is_empty() && script > 0 && script as usize * 2 >= texts.len() {
            self.notices.push(StructureNotice::ScriptModeDraft {
                script_documents: script,
                draft_documents: texts.len() as u32,
            });
        }
        let mut documents = 0u32;
        let mut words = 0u64;
        for row in &self.manuscript {
            if row.sub_role != SubRole::Paratext || row.role != Role::Item {
                continue;
            }
            let item = match row.origin {
                Origin::Item { item } | Origin::OwnText { item } => item,
                _ => continue,
            };
            if item > draft && item < stop && self.facts.items[item as usize].text.carries.counts()
            {
                documents += 1;
                words += u64::from(self.facts.items[item as usize].text.words);
            }
        }
        if documents > 0 {
            self.notices
                .push(StructureNotice::ParatextNotCounted { documents, words });
        }
    }

    fn shape(&self) -> Shape {
        let mut shape = Shape::default();
        for row in &self.manuscript {
            match (&row.role, &row.sub_role) {
                (_, SubRole::Book) | (_, SubRole::BookBegin) => shape.books += 1,
                (_, SubRole::Part) => shape.parts += 1,
                (Role::Folder, SubRole::ChapterScene) => shape.chapter_folders += 1,
                (Role::Item, SubRole::ChapterScene) => shape.flat_chapters += 1,
                (_, SubRole::Scene) => shape.scenes += 1,
                (_, SubRole::Paratext) => shape.paratexts += 1,
                (_, SubRole::None) => shape.plain_folders += 1,
                (_, SubRole::Note) => shape.manuscript_notes += 1,
                _ => {}
            }
        }
        for row in &self.notes {
            shape.notes_rows += 1;
            if !row.activated {
                shape.trashed_rows += 1;
            }
        }
        shape.templates = self.templates.len() as u32;
        shape.tags = self.tags.len() as u32;
        shape
    }

    fn review(&mut self) -> Review {
        let mut review = Review::default();
        for (depth, stats) in std::mem::take(&mut self.depth_stats) {
            if stats.containers == 0 || depth == 0 || depth > REVIEW_DEPTHS {
                continue;
            }
            let pick = |m: &BTreeMap<ContainerRung, u32>| {
                m.iter()
                    .max_by_key(|(r, c)| (**c, std::cmp::Reverse(**r)))
                    .map(|(r, _)| *r)
            };
            let detected = pick(&stats.adjustable_rungs)
                .or_else(|| pick(&stats.rungs))
                .unwrap_or(ContainerRung::Plain);
            review.containers.push(DepthControl {
                depth,
                containers: stats.containers,
                adjustable: stats.adjustable,
                detected,
                mixed: stats.rungs.len() > 1,
                chosen: self.choices.containers.get(&depth).copied(),
                examples: stats.examples,
                signals: stats.signals.into_iter().collect(),
                flags: stats.flags,
            });
        }
        for (depth, mut stats) in std::mem::take(&mut self.text_stats) {
            if stats.texts == 0 {
                continue;
            }
            stats.words.sort_unstable();
            let median_words = stats.words.get(stats.words.len() / 2).copied().unwrap_or(0);
            review.texts.push(TextsControl {
                depth,
                texts: stats.texts,
                detected: TextKind::Scenes,
                chosen: self.choices.texts.get(&depth).copied(),
                examples: stats.examples,
                median_words,
            });
        }
        review.roots = std::mem::take(&mut self.roots);
        review.books = self.books.take();
        review.splits = std::mem::take(&mut self.splits);
        if let Some(t) = self.trash {
            let stop = self.tree.end[t as usize];
            let items = stop - t - 1;
            let with_writing = (t + 1..stop).filter(|i| self.writing(*i)).count() as u32;
            if items > 0 {
                review.trash = Some(TrashControl {
                    items,
                    with_writing,
                    chosen: self.choices.trash,
                });
            }
            review.set_aside.trashed = with_writing;
        }
        review.set_aside.recovered =
            self.facts.recovered.iter().filter(|c| c.counts()).count() as u32;
        review
    }
}

/// The item a row represents, when it represents one.
fn row_item(row: &PlannedRow) -> Option<u32> {
    match row.origin {
        Origin::Item { item } | Origin::SplitPart { item, .. } => Some(item),
        _ => None,
    }
}

fn synthetic_folder(title: PlannedTitle, origin: Origin) -> PlannedRow {
    PlannedRow {
        depth: 0,
        role: Role::Folder,
        sub_role: SubRole::Note,
        title,
        contents: Vec::new(),
        exportable: false,
        activated: true,
        exclude_from_numbering: false,
        origin,
        why: Why::Synthesised,
        tag: None,
    }
}

/// The rung the shape gives a container at depth `rel` of a book whose median
/// container height is `height` (D3.3): one container level makes chapters;
/// two or more make parts then chapters; anything deeper is a plain folder.
fn shape_rung(height: u16, rel: u16) -> Rung {
    if height <= 1 {
        if rel == 1 { Rung::Chapter } else { Rung::Below }
    } else {
        match rel {
            1 => Rung::Part,
            2 => Rung::Chapter,
            _ => Rung::Below,
        }
    }
}

fn body(sources: Vec<Option<Source>>) -> Option<Body> {
    let sources: Vec<Source> = sources.into_iter().flatten().collect();
    (!sources.is_empty()).then_some(Body::From(sources))
}

fn contents(parts: Vec<(ContentRole, Option<Body>)>) -> Vec<PlannedContent> {
    parts
        .into_iter()
        .filter_map(|(role, body)| body.map(|body| PlannedContent { role, body }))
        .collect()
}

/// Where every item went, read back from the rows (D3.21).
pub(super) fn placements(
    items: usize,
    manuscript: &[PlannedRow],
    notes: &[PlannedRow],
    templates: &[PlannedTemplate],
    absence: &[Option<Absence>],
) -> Vec<Placement> {
    let mut out = vec![Placement::default(); items];
    for (binder, rows) in [
        (BinderKind::Manuscript, manuscript),
        (BinderKind::Notes, notes),
    ] {
        for (r, row) in rows.iter().enumerate() {
            let at = RowRef {
                binder,
                row: r as u32,
            };
            if let Origin::Item { item } = row.origin
                && let Some(p) = out.get_mut(item as usize)
                && p.item.is_none()
            {
                p.item = Some(at);
            }
            for content in &row.contents {
                let Body::From(sources) = &content.body else {
                    continue;
                };
                for source in sources {
                    match *source {
                        Source::Text { item } | Source::TextPart { item, .. } => {
                            if let Some(p) = out.get_mut(item as usize)
                                && p.text.is_none()
                            {
                                p.text = Some((at, content.role.clone()));
                            }
                        }
                        Source::Notes { item } => {
                            if let Some(p) = out.get_mut(item as usize) {
                                p.notes.get_or_insert(at);
                            }
                        }
                        Source::Synopsis { item } => {
                            if let Some(p) = out.get_mut(item as usize) {
                                p.synopsis.get_or_insert(at);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    for (index, template) in templates.iter().enumerate() {
        if let Some(p) = out.get_mut(template.item as usize) {
            p.template = Some(index as u32);
        }
    }
    for (i, a) in absence.iter().enumerate() {
        if let Some(p) = out.get_mut(i)
            && p.item.is_none()
        {
            p.absence = *a;
        }
    }
    out
}
