// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The structure mapping: where every Scrivener binder item goes in the new
//! project, why, and what the writer may change before anything is written.
//!
//! [`plan`] turns a read [`Project`] and the [`ContentFacts`] measured with it
//! into a [`StructurePlan`]: two binders of rows (the manuscript, and the notes),
//! the note templates, the story-bible tags, the trashed roots, where each source
//! item landed, what the review step offers, and what the writer should check.
//! It is pure and store-free: the review step recomputes it on every choice
//! ([`adjust`]), and the conversion writes exactly what it says, so the preview
//! and the import cannot disagree.
//!
//! # The mapping, in one paragraph
//!
//! Two binders. The **manuscript** holds one book (a `Folder/Book` synthesised
//! from the Draft), or one book per book-claimed Draft folder, closed by a
//! `BookEnd`; inside, each item's level is decided by the first signal that
//! claims one: its section type, a section type its ancestors hand down, a title
//! that names its level ("Chapter 7"), the template's level default, and last
//! the tree's shape. Chapters from folders keep their own prose
//! (`Folder/ChapterScene`); every other container hands its prose to a first
//! child carrying its title. Root front and back matter goes inside the book, out
//! of export. The **notes** binder holds every other root item in Scrivener's
//! order (story-bible folders tag their entries), then the format 1.x project
//! notes, the recovered texts, and the Trash, as deleted rows. The Templates
//! folder becomes note templates. The decisions and their measurements are in
//! the design note `D3-structure.md` and its critique; each rule below names its
//! decision (D3.n).
//!
//! # The invariants
//!
//! A plan is returned only after [`check`] passes it:
//!
//! * every row's `(role, sub_role)` is in the writing model's matrix and carries
//!   only the content that combination allows (`skribisto_model::validate_item`);
//! * each binder starts at depth 0, grows by at most one level a row, and never
//!   puts a row under an `Item`;
//! * along the manuscript's nesting, book, part and chapter strictly deepen, and
//!   every row that can export sits inside a book;
//! * the notes binder holds notes only, and no note is ever exported;
//! * every text, synopsis, document note, picture or file, card image, project
//!   note and recovered text that carries anything lands in exactly one row or
//!   note template, or is reported as left behind;
//! * every item is represented by one row, a note template, or a reported
//!   absence; items keep their order among their siblings, and a child whose
//!   parent has a row sits under that row.
//!
//! A violation is a typed [`PlanError`], never a panic and never a partial plan.
//!
//! # Flat, never recursive
//!
//! Like the model, the plan is lists of rows carrying their depth. Every walk
//! here is a loop over the flat binder with a stack of open ancestors: a binder
//! nested a thousand deep costs a thousand frames of a `Vec`, never of the call
//! stack.
//!
//! # Nothing it says is a sentence
//!
//! Titles the import adds ("(notes)", "Project notes", "Recovered from
//! Scrivener") are [`PlannedTitle`] values the converter words in the writer's
//! language; notices are [`StructureNotice`] values with stable keys.

mod build;
mod check;
pub(crate) mod lexicon;
mod notice;

use std::collections::BTreeMap;

/// The writing model's vocabulary the plan is written in, re-exported so a
/// caller reads a plan without naming the generated crate.
pub use common::entities::{BinderItemRole as Role, BinderItemSubRole as SubRole, ContentRole};
use serde::{Deserialize, Serialize};

use crate::content::ContentFacts;
use crate::model::{Project, ShapeError};

pub(crate) use build::clean_title;
pub use check::{Violation, check};
pub use notice::{ReportGroup, StructureNotice};

/// The whole mapping of one project, as the review shows it and the conversion
/// writes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructurePlan {
    /// The choices this plan was derived with.
    pub choices: Choices,
    /// The manuscript binder, in order.
    pub manuscript: Vec<PlannedRow>,
    /// The notes binder, in order.
    pub notes: Vec<PlannedRow>,
    /// Note templates, from the Templates folder (D3.16).
    pub templates: Vec<PlannedTemplate>,
    /// Story-bible tags, one per recognised folder title (D3.14).
    pub tags: Vec<PlannedTag>,
    /// The notes rows that open a trashed subtree, one trash entry each (D3.17).
    pub trash_roots: Vec<u32>,
    /// How new chapters are made in this project: the encoding most of its
    /// chapters already use (D3.5).
    pub chapter_encoding: ChapterEncoding,
    /// Where each binder item went, one per source row (D3.21).
    pub placements: Vec<Placement>,
    /// What the review step offers, with what was detected.
    pub review: Review,
    /// Counts of the result, for the review's summary.
    pub shape: Shape,
    /// What the writer should know or check.
    pub notices: Vec<StructureNotice>,
}

/// Which binder a row is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinderKind {
    Manuscript,
    Notes,
}

/// A row of a planned binder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlannedRow {
    /// Its depth in its binder: 0 at the top.
    pub depth: u16,
    pub role: Role,
    pub sub_role: SubRole,
    pub title: PlannedTitle,
    /// The contents it carries, each with where it comes from. Empty writing is
    /// not listed: an empty scene is a scene with no content yet.
    pub contents: Vec<PlannedContent>,
    pub exportable: bool,
    /// False for a row of the Trash.
    pub activated: bool,
    /// A prologue or epilogue chapter, left out of the numbering.
    pub exclude_from_numbering: bool,
    /// Which item it comes from, and in what capacity.
    pub origin: Origin,
    /// Why it has this role and sub-role.
    pub why: Why,
    /// The story-bible tag it wears, an index into [`StructurePlan::tags`].
    pub tag: Option<u32>,
}

/// A row's title.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlannedTitle {
    /// The item's own title, cleaned (D3.18). May be empty: a blank title is
    /// named by its number in the binder, never by an invented word.
    Own(String),
    /// A note holding another item's document notes: the converter adds the
    /// "(notes)" suffix in the writer's language. The item's title, cleaned.
    NotesOf(String),
    /// The project's title, which the Save page edits (the book takes it when
    /// the Draft kept a default name). The value known now: the project's own
    /// title when it states one, else its folder's name.
    ProjectTitle(String),
    /// The folder of format 1.x project notes.
    ProjectNotes,
    /// The folder of texts found on disk that no binder item listed.
    Recovered,
}

impl PlannedTitle {
    /// The words known now, empty for a folder the converter names.
    pub fn text(&self) -> &str {
        match self {
            PlannedTitle::Own(t) | PlannedTitle::NotesOf(t) | PlannedTitle::ProjectTitle(t) => t,
            PlannedTitle::ProjectNotes | PlannedTitle::Recovered => "",
        }
    }
}

/// One content of a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedContent {
    pub role: ContentRole,
    pub body: Body,
}

/// What a content holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Body {
    /// The row's title, as its heading.
    Title,
    /// What these sources hold, one after the other.
    From(Vec<Source>),
}

/// A piece of the Scrivener project a content is made from. Items are binder
/// rows, by index into [`Project::binder`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Source {
    /// An item's main text.
    Text {
        item: u32,
    },
    /// Part of an item's main text split at its chapter headings: part 0 is what
    /// comes before the first heading, part `n` what follows heading `n`.
    TextPart {
        item: u32,
        part: u32,
    },
    /// An item's document notes.
    Notes {
        item: u32,
    },
    Synopsis {
        item: u32,
    },
    /// A picture, PDF or other file the item is.
    Media {
        item: u32,
    },
    /// The picture on the item's index card.
    CardImage {
        item: u32,
    },
    /// A format 1.x project note, by index.
    ProjectNote {
        note: u32,
    },
    /// A recovered text, by index.
    Recovered {
        text: u32,
    },
}

/// Which item a row comes from, and as what.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    /// The row that represents the item.
    Item {
        item: u32,
    },
    /// A container's own text, picture or card image, as its first child (D3.6).
    OwnText {
        item: u32,
    },
    /// An item's document notes, beside it (D3.10).
    DocumentNotes {
        item: u32,
    },
    /// A chapter split out of a one-document manuscript at heading `part`.
    SplitPart {
        item: u32,
        part: u32,
    },
    /// What a note template could not hold: a template's synopsis, a picture in
    /// the Templates folder (D3.16).
    TemplateLeftover {
        item: u32,
    },
    /// With several books, the Draft's own text and synopsis (D3.8).
    DraftContent {
        item: u32,
    },
    /// A book made for a project without a Draft folder.
    Book,
    BookEnd,
    ProjectNotes,
    ProjectNote {
        note: u32,
    },
    Recovered,
    RecoveredText {
        text: u32,
    },
}

/// Why a row has its role and sub-role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Why {
    /// The item's own section type names its level.
    SectionType,
    /// The section type an ancestor hands down to its descendants.
    InheritedSectionType { from: u32 },
    /// Its title names its level ("Chapter 7").
    Title,
    /// Most of its siblings' titles name a level with the same word.
    SiblingTitles,
    /// The template's default section type for its level.
    LevelType { level: u16 },
    /// The book's shape: `height` container levels, upper median (D3.3).
    Shape { height: u16 },
    /// A container among mostly texts: plain, so it does not swallow them.
    Minority,
    /// Its descendants claim this level or a higher one: plain (D3.3).
    ClaimsBelow,
    /// Every child is front or back matter (D3.7).
    ParatextChildren,
    /// An empty folder: its siblings' level, or plain (D3.4).
    EmptyFolder,
    /// A level an ancestor already holds: demoted.
    Floor,
    /// The writer's choice in the review.
    Choice,
    /// An unclaimed text: a scene.
    Text,
    /// A chapter heading inside one document (A2).
    Heading,
    /// Inside a front-matter, back-matter or notes subtree.
    Inside,
    /// A picture or file.
    Media,
    /// Front or back matter at the root (D3.11): front matter opens the book
    /// and back matter closes it, whatever their order among the root items.
    Matter { back: bool },
    /// A story-bible folder (D3.14).
    StoryBible,
    /// Outside the Draft (D3.12).
    Notes,
    /// Made by the import: a book, its end, a notes row, an own-text child.
    Synthesised,
}

/// What kind of signal decided a row, without its details: for counting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Signal {
    SectionType,
    InheritedSectionType,
    Title,
    SiblingTitles,
    LevelType,
    Shape,
    Minority,
    ClaimsBelow,
    ParatextChildren,
    EmptyFolder,
    Floor,
    Choice,
    Text,
    Heading,
    Inside,
    Media,
    Matter,
    StoryBible,
    Notes,
    Synthesised,
}

impl Why {
    pub fn signal(&self) -> Signal {
        match self {
            Why::SectionType => Signal::SectionType,
            Why::InheritedSectionType { .. } => Signal::InheritedSectionType,
            Why::Title => Signal::Title,
            Why::SiblingTitles => Signal::SiblingTitles,
            Why::LevelType { .. } => Signal::LevelType,
            Why::Shape { .. } => Signal::Shape,
            Why::Minority => Signal::Minority,
            Why::ClaimsBelow => Signal::ClaimsBelow,
            Why::ParatextChildren => Signal::ParatextChildren,
            Why::EmptyFolder => Signal::EmptyFolder,
            Why::Floor => Signal::Floor,
            Why::Choice => Signal::Choice,
            Why::Text => Signal::Text,
            Why::Heading => Signal::Heading,
            Why::Inside => Signal::Inside,
            Why::Media => Signal::Media,
            Why::Matter { .. } => Signal::Matter,
            Why::StoryBible => Signal::StoryBible,
            Why::Notes => Signal::Notes,
            Why::Synthesised => Signal::Synthesised,
        }
    }
}

/// A level of the book, as a signal claims it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    Book,
    Part,
    Chapter,
    Scene,
    /// Front or back matter, or a standalone text.
    Paratext,
    Note,
}

/// A note template made from a text of the Templates folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedTemplate {
    /// The item's title, cleaned. Empty when it had none: the converter names
    /// it in the writer's language.
    pub name: String,
    /// The binder row it comes from.
    pub item: u32,
    /// Its body, when the text carries anything.
    pub body: Option<Source>,
}

/// A story-bible tag (D3.14).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedTag {
    /// The folder's own title, cleaned: the word the writer recognises.
    pub name: String,
    /// The first folder that named it, a binder row.
    pub folder: u32,
    /// The notes row new entries under this tag are created in.
    pub creates_in: u32,
    /// The note template new entries start from, an index into
    /// [`StructurePlan::templates`].
    pub template: Option<u32>,
}

/// How new chapters are made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChapterEncoding {
    /// A chapter is a folder (`Folder/ChapterScene`).
    Folder,
    /// A chapter is a text (`Item/ChapterScene`).
    Flat,
}

/// A row, by binder and index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RowRef {
    pub binder: BinderKind,
    pub row: u32,
}

/// Where one binder item went (D3.21).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Placement {
    /// The row that represents it; metadata (labels, keywords, links) goes here.
    pub item: Option<RowRef>,
    /// The row holding its main text, and in which content.
    pub text: Option<(RowRef, ContentRole)>,
    /// The row holding its document notes.
    pub notes: Option<RowRef>,
    /// The row holding its synopsis.
    pub synopsis: Option<RowRef>,
    /// The note template it became.
    pub template: Option<u32>,
    /// Why it has no row, when it has none.
    pub absence: Option<Absence>,
}

/// Why an item has no row of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Absence {
    /// The Draft, the Trash or the Templates folder: represented by what it
    /// holds.
    SpecialFolder,
    /// A text of the Templates folder: a note template.
    Template,
    /// Inside the Trash, which the writer left out.
    TrashLeftOut,
    /// A root folder, or inside one, the writer left out.
    LeftOut,
}

/// What the review step offers, each control with what was detected and what
/// is chosen.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Review {
    /// One per manuscript depth 1 to 3 that holds containers.
    pub containers: Vec<DepthControl>,
    /// One per manuscript depth 1 to 3 that holds texts no signal claims and
    /// that could be chapters.
    pub texts: Vec<TextsControl>,
    /// One per root item the mapping did not recognise.
    pub roots: Vec<RootControl>,
    /// Present when a Draft folder claims to be a book.
    pub books: Option<BooksControl>,
    /// One per Draft document holding chapter headings (A2).
    pub splits: Vec<SplitControl>,
    /// Present when the Trash holds anything (D3.17).
    pub trash: Option<TrashControl>,
    /// Text the writer had deleted or that Scrivener kept aside, shown beside
    /// the Trash switch (S3).
    pub set_aside: SetAside,
}

/// The level containers at one depth take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ContainerRung {
    Part,
    Chapter,
    /// A folder that only groups; its title never prints.
    Plain,
}

/// What texts at one depth are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextKind {
    Scenes,
    Chapters,
}

/// Where a root item goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Destination {
    Notes,
    /// Notes, with a story-bible tag on its entries.
    StoryBible,
    /// Another book of the manuscript.
    Manuscript,
    LeaveOut,
}

/// One book, or one per book-claimed Draft folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BooksChoice {
    One,
    Several,
}

/// Whether the Trash comes across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TrashChoice {
    /// As deleted rows the Trash view can restore (the default, D3.17).
    #[default]
    Import,
    LeaveOut,
}

/// Containers at one depth of the manuscript.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthControl {
    /// 1 for folders directly in the Draft (or a book).
    pub depth: u16,
    /// Containers at this depth.
    pub containers: u32,
    /// Those no signal claims: the ones a choice applies to.
    pub adjustable: u32,
    /// The level most unclaimed containers were given.
    pub detected: ContainerRung,
    /// Some containers at this depth ended at another level than `detected`.
    pub mixed: bool,
    pub chosen: Option<ContainerRung>,
    /// Up to two items at this depth, binder rows.
    pub examples: Vec<u32>,
    /// What decided the containers here, and how many each.
    pub signals: Vec<(Signal, u32)>,
    /// Why the detected level is a guess the writer should look at.
    pub flags: Vec<ShapeFlag>,
}

/// A reason to look twice at a depth's detected level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeFlag {
    /// The median and the tallest container disagree about parts and chapters.
    MedianDiffersFromMaximum,
    /// A single container holds the whole book and is taken as a part.
    LoneWrapper,
    /// Some containers here were left plain for being few among texts.
    MinorityContainers,
    /// Texts after a chapter at this depth export as that chapter's end.
    AdoptsTexts,
}

/// Unclaimed texts at one depth of the manuscript.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextsControl {
    pub depth: u16,
    pub texts: u32,
    pub detected: TextKind,
    pub chosen: Option<TextKind>,
    pub examples: Vec<u32>,
    /// The median length of those texts, in words: long texts are often
    /// chapters.
    pub median_words: u32,
}

/// A root item the mapping did not recognise.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootControl {
    /// The binder row.
    pub item: u32,
    /// Items in it, itself included.
    pub items: u32,
    pub detected: Destination,
    pub chosen: Option<Destination>,
    /// Its title or its content reads like more of the book.
    pub looks_like_manuscript: bool,
}

/// Book-claimed Draft folders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooksControl {
    /// Draft folders claimed as books, binder rows.
    pub claimed: Vec<u32>,
    pub detected: BooksChoice,
    pub chosen: Option<BooksChoice>,
}

/// A Draft document holding its chapters as headings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitControl {
    pub item: u32,
    pub headings: u32,
    /// Split by default: three chapter headings or more.
    pub detected: bool,
    pub chosen: Option<bool>,
}

/// The Trash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrashControl {
    /// Items in it, at any depth.
    pub items: u32,
    /// Of those, items with any writing.
    pub with_writing: u32,
    pub chosen: TrashChoice,
}

/// Text that comes back although the writer may believe it gone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetAside {
    /// Items with writing in the Trash, brought across when the Trash is.
    pub trashed: u32,
    /// Recovered texts (sync copies, documents the binder did not list).
    pub recovered: u32,
}

/// Counts of the planned structure.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shape {
    pub books: u32,
    pub parts: u32,
    pub chapter_folders: u32,
    pub flat_chapters: u32,
    pub scenes: u32,
    pub paratexts: u32,
    pub plain_folders: u32,
    /// Notes rows in the manuscript binder (document notes, notes-claimed items).
    pub manuscript_notes: u32,
    pub notes_rows: u32,
    pub trashed_rows: u32,
    pub templates: u32,
    pub tags: u32,
}

/// What the writer chose in the review. Every field left at its default means
/// "what was detected".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choices {
    /// The level containers no signal claims take, per manuscript depth 1 to 3.
    pub containers: BTreeMap<u16, ContainerRung>,
    /// What unclaimed texts are, per manuscript depth 1 to 3.
    pub texts: BTreeMap<u16, TextKind>,
    /// Where each unrecognised root item goes, by binder row.
    pub roots: BTreeMap<u32, Destination>,
    pub books: Option<BooksChoice>,
    /// Whether to split each document at its chapter headings, by binder row.
    pub splits: BTreeMap<u32, bool>,
    pub trash: TrashChoice,
}

/// One change the writer makes in the review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Adjustment {
    /// Containers no signal claims at `depth` take `rung`; `None` restores the
    /// detected level.
    Containers {
        depth: u16,
        rung: Option<ContainerRung>,
    },
    /// Unclaimed texts at `depth` are `kind`; `None` restores the default.
    Texts {
        depth: u16,
        kind: Option<TextKind>,
    },
    /// Root item `item` goes to `destination`; `None` restores the default.
    Root {
        item: u32,
        destination: Option<Destination>,
    },
    Books(Option<BooksChoice>),
    /// Split document `item` at its headings, or not; `None` restores the
    /// default.
    Split {
        item: u32,
        split: Option<bool>,
    },
    Trash(TrashChoice),
}

impl Choices {
    /// Apply one adjustment.
    pub fn apply(&mut self, adjustment: &Adjustment) {
        fn set<K: Ord, V>(map: &mut BTreeMap<K, V>, key: K, value: Option<V>) {
            match value {
                Some(value) => {
                    map.insert(key, value);
                }
                None => {
                    map.remove(&key);
                }
            }
        }
        match adjustment {
            Adjustment::Containers { depth, rung } => set(&mut self.containers, *depth, *rung),
            Adjustment::Texts { depth, kind } => set(&mut self.texts, *depth, *kind),
            Adjustment::Root { item, destination } => set(&mut self.roots, *item, *destination),
            Adjustment::Books(choice) => self.books = *choice,
            Adjustment::Split { item, split } => set(&mut self.splits, *item, *split),
            Adjustment::Trash(choice) => self.trash = *choice,
        }
    }
}

/// Why a plan could not be made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanError {
    /// The model's depths do not describe a tree.
    Shape(ShapeError),
    /// The content facts were measured for another binder.
    FactsMismatch { items: u32, facts: u32 },
    /// A choice names something the review does not offer.
    Choice(ChoiceError),
    /// The mapping broke one of its own invariants: a defect of this importer,
    /// reported rather than written.
    Violation(Violation),
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::Shape(e) => write!(f, "{e}"),
            PlanError::FactsMismatch { items, facts } => {
                write!(f, "{facts} content facts for {items} binder items")
            }
            PlanError::Choice(e) => write!(f, "choice not offered: {e:?}"),
            PlanError::Violation(v) => write!(f, "structure invariant broken: {v:?}"),
        }
    }
}

impl std::error::Error for PlanError {}

/// A choice the review does not offer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceError {
    /// Levels are chosen for depths 1 to 3 only.
    DepthOutOfRange { depth: u16 },
    /// This binder row is not a root item the review offers.
    RootNotOffered { item: u32 },
}

/// Map `project` onto a plan, with `facts` measured from it and the writer's
/// `choices`.
pub fn plan(
    project: &Project,
    facts: &ContentFacts,
    choices: &Choices,
) -> Result<StructurePlan, PlanError> {
    project.check_shape().map_err(PlanError::Shape)?;
    if facts.items.len() != project.binder.len()
        || facts.project_notes.len() != project.project_notes.len()
        || facts.recovered.len() != project.recovered.len()
    {
        return Err(PlanError::FactsMismatch {
            items: project.binder.len() as u32,
            facts: facts.items.len() as u32,
        });
    }
    for depth in choices.containers.keys().chain(choices.texts.keys()) {
        if !(1..=3).contains(depth) {
            return Err(PlanError::Choice(ChoiceError::DepthOutOfRange {
                depth: *depth,
            }));
        }
    }
    let plan = build::build(project, facts, choices)?;
    check(project, facts, &plan).map_err(PlanError::Violation)?;
    Ok(plan)
}

/// The plan with one more of the writer's choices applied.
pub fn adjust(
    project: &Project,
    facts: &ContentFacts,
    current: &StructurePlan,
    adjustment: &Adjustment,
) -> Result<StructurePlan, PlanError> {
    let mut choices = current.choices.clone();
    choices.apply(adjustment);
    plan(project, facts, &choices)
}

#[cfg(test)]
mod tests;
