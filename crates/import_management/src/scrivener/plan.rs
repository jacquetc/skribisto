// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The structure mapping, the questions and the project card as the wizard receives them,
//! and the writer's structure choices as the converter takes them.
//!
//! Field for field, with no interpretation: every decision was the converter's, and a
//! second opinion here would be one the import does not share.

use scrivener_import::analyze::SiblingChoice;
use scrivener_import::metadata::{AuthorSource, LanguageSource, TitleSource};
use scrivener_import::model::{Container, Generation, ItemKind, ManifestChoice, Platform, Project};
use scrivener_import::outcome::{ManifestEvidence, MobileEdits};
use scrivener_import::structure::{
    Adjustment, BinderKind, BooksChoice, ChapterEncoding, Choices, ContainerRung, Destination,
    Origin, PlannedTitle, ShapeFlag, Signal, StructurePlan, TextKind, TrashChoice,
};
use scrivener_import::{Question, SourceNames};

use super::findings::{self, Locator};
use crate::dtos::{
    ScrivenerBinder, ScrivenerBooksChoice, ScrivenerChapterEncoding, ScrivenerChoice,
    ScrivenerContainer, ScrivenerContainerRung, ScrivenerControl, ScrivenerDestination,
    ScrivenerGeneration, ScrivenerItemRef, ScrivenerManifestCandidate, ScrivenerManifestChoice,
    ScrivenerManifestEvidence, ScrivenerMobileEdits, ScrivenerPlatform, ScrivenerPreviewRow,
    ScrivenerQuestion, ScrivenerShape, ScrivenerShapeFlag, ScrivenerSignal, ScrivenerSignalCount,
    ScrivenerStructure, ScrivenerTextKind, ScrivenerTitleKind, ScrivenerTrashChoice,
};

// ── the structure plan ───────────────────────────────────────────────────────────────────

/// The plan as the review page shows it. Its notices are named with `names`, which the
/// analysis of the same project made; nothing is written yet, so no finding has a uid.
pub(crate) fn structure(plan: &StructurePlan, names: &SourceNames) -> ScrivenerStructure {
    let locator = Locator::before_import(names);
    let rows = [
        (BinderKind::Manuscript, &plan.manuscript),
        (BinderKind::Notes, &plan.notes),
    ]
    .into_iter()
    .flat_map(|(kind, rows)| rows.iter().map(move |row| (kind, row)))
    .map(|(kind, row)| {
        let (title_kind, title) = match &row.title {
            PlannedTitle::Own(title) => (ScrivenerTitleKind::Own, title.clone()),
            PlannedTitle::NotesOf(title) => (ScrivenerTitleKind::NotesOf, title.clone()),
            PlannedTitle::ProjectTitle(title) => (ScrivenerTitleKind::ProjectTitle, title.clone()),
            PlannedTitle::ProjectNotes => (ScrivenerTitleKind::ProjectNotes, String::new()),
            PlannedTitle::Recovered => (ScrivenerTitleKind::Recovered, String::new()),
        };
        ScrivenerPreviewRow::Row {
            binder: binder(kind),
            depth: i64::from(row.depth),
            role: row.role.clone(),
            sub_role: row.sub_role.clone(),
            title,
            title_kind,
            exportable: row.exportable,
            activated: row.activated,
            item: origin_item(&row.origin),
            why: signal(row.why.signal()),
        }
    })
    .collect();

    let review = &plan.review;
    let examples = |items: &[u32]| -> Vec<ScrivenerItemRef> {
        items.iter().map(|item| item_ref(names, *item)).collect()
    };
    let mut controls = Vec::new();
    controls.extend(review.containers.iter().map(|c| {
        ScrivenerControl::Containers {
            depth: i64::from(c.depth),
            containers: i64::from(c.containers),
            adjustable: i64::from(c.adjustable),
            detected: container_rung(c.detected),
            mixed: c.mixed,
            chosen: c.chosen.map(container_rung),
            examples: examples(&c.examples),
            signals: c
                .signals
                .iter()
                .map(|(s, n)| ScrivenerSignalCount::Counted {
                    signal: signal(*s),
                    count: i64::from(*n),
                })
                .collect(),
            flags: c.flags.iter().map(|f| shape_flag(*f)).collect(),
        }
    }));
    controls.extend(review.texts.iter().map(|t| ScrivenerControl::Texts {
        depth: i64::from(t.depth),
        texts: i64::from(t.texts),
        detected: text_kind(t.detected),
        chosen: t.chosen.map(text_kind),
        examples: examples(&t.examples),
        median_words: i64::from(t.median_words),
    }));
    controls.extend(review.roots.iter().map(|r| ScrivenerControl::Root {
        item: item_ref(names, r.item),
        items: i64::from(r.items),
        detected: destination(r.detected),
        chosen: r.chosen.map(destination),
        looks_like_manuscript: r.looks_like_manuscript,
    }));
    controls.extend(review.books.iter().map(|b| ScrivenerControl::Books {
        claimed: examples(&b.claimed),
        detected: books_choice(b.detected),
        chosen: b.chosen.map(books_choice),
    }));
    controls.extend(review.splits.iter().map(|s| ScrivenerControl::Split {
        item: item_ref(names, s.item),
        headings: i64::from(s.headings),
        detected: s.detected,
        chosen: s.chosen,
    }));
    controls.extend(review.trash.iter().map(|t| ScrivenerControl::Trash {
        items: i64::from(t.items),
        with_writing: i64::from(t.with_writing),
        chosen: trash_choice(t.chosen),
    }));

    let shape = &plan.shape;
    ScrivenerStructure::Planned {
        rows,
        controls,
        set_aside_trashed: i64::from(review.set_aside.trashed),
        set_aside_recovered: i64::from(review.set_aside.recovered),
        shape: ScrivenerShape::Counted {
            books: i64::from(shape.books),
            parts: i64::from(shape.parts),
            chapter_folders: i64::from(shape.chapter_folders),
            flat_chapters: i64::from(shape.flat_chapters),
            scenes: i64::from(shape.scenes),
            paratexts: i64::from(shape.paratexts),
            plain_folders: i64::from(shape.plain_folders),
            manuscript_notes: i64::from(shape.manuscript_notes),
            notes_rows: i64::from(shape.notes_rows),
            trashed_rows: i64::from(shape.trashed_rows),
            templates: i64::from(shape.templates),
            tags: i64::from(shape.tags),
        },
        chapter_encoding: match plan.chapter_encoding {
            ChapterEncoding::Folder => ScrivenerChapterEncoding::Folder,
            ChapterEncoding::Flat => ScrivenerChapterEncoding::Flat,
        },
        templates: plan.templates.iter().map(|t| t.name.clone()).collect(),
        tags: plan.tags.iter().map(|t| t.name.clone()).collect(),
        notices: plan
            .notices
            .iter()
            .map(|n| findings::structure_notice(n, &locator))
            .collect(),
        choices: choices_to_dto(&plan.choices),
    }
}

fn binder(kind: BinderKind) -> ScrivenerBinder {
    match kind {
        BinderKind::Manuscript => ScrivenerBinder::Manuscript,
        BinderKind::Notes => ScrivenerBinder::Notes,
    }
}

/// The binder row a planned row represents, or -1 for one the import makes.
fn origin_item(origin: &Origin) -> i64 {
    match origin {
        Origin::Item { item }
        | Origin::OwnText { item }
        | Origin::DocumentNotes { item }
        | Origin::SplitPart { item, .. }
        | Origin::TemplateLeftover { item }
        | Origin::DraftContent { item } => i64::from(*item),
        Origin::Book
        | Origin::BookEnd
        | Origin::ProjectNotes
        | Origin::ProjectNote { .. }
        | Origin::Recovered
        | Origin::RecoveredText { .. } => -1,
    }
}

/// A Scrivener binder row as a review control names it.
fn item_ref(names: &SourceNames, item: u32) -> ScrivenerItemRef {
    ScrivenerItemRef::Item {
        item: i64::from(item),
        binder_path: names.items.get(item as usize).cloned().unwrap_or_default(),
    }
}

fn signal(signal: Signal) -> ScrivenerSignal {
    match signal {
        Signal::SectionType => ScrivenerSignal::SectionType,
        Signal::InheritedSectionType => ScrivenerSignal::InheritedSectionType,
        Signal::Title => ScrivenerSignal::Title,
        Signal::SiblingTitles => ScrivenerSignal::SiblingTitles,
        Signal::LevelType => ScrivenerSignal::LevelType,
        Signal::Shape => ScrivenerSignal::Shape,
        Signal::Minority => ScrivenerSignal::Minority,
        Signal::ClaimsBelow => ScrivenerSignal::ClaimsBelow,
        Signal::ParatextChildren => ScrivenerSignal::ParatextChildren,
        Signal::EmptyFolder => ScrivenerSignal::EmptyFolder,
        Signal::Floor => ScrivenerSignal::Floor,
        Signal::Choice => ScrivenerSignal::Choice,
        Signal::Text => ScrivenerSignal::Text,
        Signal::Heading => ScrivenerSignal::Heading,
        Signal::Inside => ScrivenerSignal::Inside,
        Signal::Media => ScrivenerSignal::Media,
        Signal::Matter => ScrivenerSignal::Matter,
        Signal::StoryBible => ScrivenerSignal::StoryBible,
        Signal::Notes => ScrivenerSignal::Notes,
        Signal::Synthesised => ScrivenerSignal::Synthesised,
    }
}

fn shape_flag(flag: ShapeFlag) -> ScrivenerShapeFlag {
    match flag {
        ShapeFlag::MedianDiffersFromMaximum => ScrivenerShapeFlag::MedianDiffersFromMaximum,
        ShapeFlag::LoneWrapper => ScrivenerShapeFlag::LoneWrapper,
        ShapeFlag::MinorityContainers => ScrivenerShapeFlag::MinorityContainers,
        ShapeFlag::AdoptsTexts => ScrivenerShapeFlag::AdoptsTexts,
    }
}

fn container_rung(rung: ContainerRung) -> ScrivenerContainerRung {
    match rung {
        ContainerRung::Part => ScrivenerContainerRung::Part,
        ContainerRung::Chapter => ScrivenerContainerRung::Chapter,
        ContainerRung::Plain => ScrivenerContainerRung::Plain,
    }
}

fn container_rung_back(rung: &ScrivenerContainerRung) -> ContainerRung {
    match rung {
        ScrivenerContainerRung::Part => ContainerRung::Part,
        ScrivenerContainerRung::Chapter => ContainerRung::Chapter,
        ScrivenerContainerRung::Plain => ContainerRung::Plain,
    }
}

fn text_kind(kind: TextKind) -> ScrivenerTextKind {
    match kind {
        TextKind::Scenes => ScrivenerTextKind::Scenes,
        TextKind::Chapters => ScrivenerTextKind::Chapters,
    }
}

fn text_kind_back(kind: &ScrivenerTextKind) -> TextKind {
    match kind {
        ScrivenerTextKind::Scenes => TextKind::Scenes,
        ScrivenerTextKind::Chapters => TextKind::Chapters,
    }
}

fn destination(destination: Destination) -> ScrivenerDestination {
    match destination {
        Destination::Notes => ScrivenerDestination::Notes,
        Destination::StoryBible => ScrivenerDestination::StoryBible,
        Destination::Manuscript => ScrivenerDestination::Manuscript,
        Destination::LeaveOut => ScrivenerDestination::LeaveOut,
    }
}

fn destination_back(destination: &ScrivenerDestination) -> Destination {
    match destination {
        ScrivenerDestination::Notes => Destination::Notes,
        ScrivenerDestination::StoryBible => Destination::StoryBible,
        ScrivenerDestination::Manuscript => Destination::Manuscript,
        ScrivenerDestination::LeaveOut => Destination::LeaveOut,
    }
}

fn books_choice(choice: BooksChoice) -> ScrivenerBooksChoice {
    match choice {
        BooksChoice::One => ScrivenerBooksChoice::One,
        BooksChoice::Several => ScrivenerBooksChoice::Several,
    }
}

fn books_choice_back(choice: &ScrivenerBooksChoice) -> BooksChoice {
    match choice {
        ScrivenerBooksChoice::One => BooksChoice::One,
        ScrivenerBooksChoice::Several => BooksChoice::Several,
    }
}

fn trash_choice(choice: TrashChoice) -> ScrivenerTrashChoice {
    match choice {
        TrashChoice::Import => ScrivenerTrashChoice::Import,
        TrashChoice::LeaveOut => ScrivenerTrashChoice::LeaveOut,
    }
}

fn trash_choice_back(choice: &ScrivenerTrashChoice) -> TrashChoice {
    match choice {
        ScrivenerTrashChoice::Import => TrashChoice::Import,
        ScrivenerTrashChoice::LeaveOut => TrashChoice::LeaveOut,
    }
}

// ── the writer's choices ─────────────────────────────────────────────────────────────────

/// A structure choice that names a depth or a binder row no plan can have: a number past
/// what the converter counts in. The review never offers one; the import refuses it by
/// name rather than bending it into range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceOutOfRange {
    Depth(i64),
    Item(i64),
}

impl std::fmt::Display for ChoiceOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceOutOfRange::Depth(depth) => write!(f, "no structure choice at depth {depth}"),
            ChoiceOutOfRange::Item(item) => write!(f, "no binder row {item}"),
        }
    }
}

impl std::error::Error for ChoiceOutOfRange {}

/// The choices a plan was made with, as the list the wizard holds: one entry per choice
/// that differs from what was detected, and the Trash's, always.
pub(crate) fn choices_to_dto(choices: &Choices) -> Vec<ScrivenerChoice> {
    let mut out = Vec::new();
    out.extend(
        choices
            .containers
            .iter()
            .map(|(depth, rung)| ScrivenerChoice::Containers {
                depth: i64::from(*depth),
                rung: container_rung(*rung),
            }),
    );
    out.extend(
        choices
            .texts
            .iter()
            .map(|(depth, kind)| ScrivenerChoice::Texts {
                depth: i64::from(*depth),
                kind: text_kind(*kind),
            }),
    );
    out.extend(
        choices
            .roots
            .iter()
            .map(|(item, destination_)| ScrivenerChoice::Root {
                item: i64::from(*item),
                destination: destination(*destination_),
            }),
    );
    out.extend(choices.books.map(|choice| ScrivenerChoice::Books {
        choice: books_choice(choice),
    }));
    out.extend(
        choices
            .splits
            .iter()
            .map(|(item, split)| ScrivenerChoice::Split {
                item: i64::from(*item),
                split: *split,
            }),
    );
    out.push(ScrivenerChoice::Trash {
        choice: trash_choice(choices.trash),
    });
    out
}

/// The wizard's list of choices as the converter takes them, applied in order onto what
/// was detected. `Empty` entries are the generator's unit variant and change nothing.
pub fn choices_from_dto(list: &[ScrivenerChoice]) -> Result<Choices, ChoiceOutOfRange> {
    let depth = |d: i64| u16::try_from(d).map_err(|_| ChoiceOutOfRange::Depth(d));
    let item = |i: i64| u32::try_from(i).map_err(|_| ChoiceOutOfRange::Item(i));
    let mut choices = Choices::default();
    for choice in list {
        let adjustment = match choice {
            ScrivenerChoice::Empty => continue,
            ScrivenerChoice::Containers { depth: d, rung } => Adjustment::Containers {
                depth: depth(*d)?,
                rung: Some(container_rung_back(rung)),
            },
            ScrivenerChoice::Texts { depth: d, kind } => Adjustment::Texts {
                depth: depth(*d)?,
                kind: Some(text_kind_back(kind)),
            },
            ScrivenerChoice::Root {
                item: i,
                destination,
            } => Adjustment::Root {
                item: item(*i)?,
                destination: Some(destination_back(destination)),
            },
            ScrivenerChoice::Books { choice } => Adjustment::Books(Some(books_choice_back(choice))),
            ScrivenerChoice::Split { item: i, split } => Adjustment::Split {
                item: item(*i)?,
                split: Some(*split),
            },
            ScrivenerChoice::Trash { choice } => Adjustment::Trash(trash_choice_back(choice)),
        };
        choices.apply(&adjustment);
    }
    Ok(choices)
}

pub(crate) fn sibling_choice(
    choice: &crate::dtos::ScrivenerSiblingChoice,
) -> Option<SiblingChoice> {
    match choice {
        crate::dtos::ScrivenerSiblingChoice::Unanswered => None,
        crate::dtos::ScrivenerSiblingChoice::Sibling => Some(SiblingChoice::Sibling),
        crate::dtos::ScrivenerSiblingChoice::Backup => Some(SiblingChoice::Backup),
    }
}

// ── the questions ────────────────────────────────────────────────────────────────────────

/// A question the writer must answer, field for field.
pub fn question(question: &Question) -> ScrivenerQuestion {
    let count = |n: u32| i64::from(n);
    match question {
        Question::SeveralProjects { candidates } => ScrivenerQuestion::SeveralProjects {
            candidates: candidates.iter().map(|c| c.path.clone()).collect(),
        },
        Question::SeveralManifests {
            candidates,
            preselected,
            evidence,
        } => ScrivenerQuestion::SeveralManifests {
            candidates: candidates
                .iter()
                .map(|c| ScrivenerManifestCandidate::Candidate {
                    name: c.name.clone(),
                    creator: c.creator.clone(),
                    version: c.version.clone(),
                    modified: c.modified,
                    items: c.items.map(count),
                    matches_autosave: c.matches_autosave,
                    identical_to_preselected: c.identical_to_preselected,
                })
                .collect(),
            preselected: preselected.clone(),
            evidence: evidence
                .iter()
                .map(|e| match e {
                    ManifestEvidence::MatchesAutosave => ScrivenerManifestEvidence::MatchesAutosave,
                    ManifestEvidence::FolderName => ScrivenerManifestEvidence::FolderName,
                    ManifestEvidence::Newest => ScrivenerManifestEvidence::Newest,
                })
                .collect(),
        },
        Question::EnclosingProject { project, levels_up } => ScrivenerQuestion::EnclosingProject {
            project: project.clone(),
            levels_up: count(*levels_up),
        },
        Question::UpgradedSibling {
            backup,
            sibling,
            sibling_generation,
            preselect_sibling,
        } => ScrivenerQuestion::UpgradedSibling {
            backup: backup.clone(),
            sibling: sibling.clone(),
            sibling_generation: sibling_generation.map(generation),
            preselect_sibling: *preselect_sibling,
        },
        Question::UnmergedMobileEdits { source } => ScrivenerQuestion::UnmergedMobileEdits {
            source: match source {
                MobileEdits::MobileFolder => ScrivenerMobileEdits::MobileFolder,
                MobileEdits::NewerMobileBinder => ScrivenerMobileEdits::NewerMobileBinder,
            },
        },
        Question::NoTextInCopy { text_items } => ScrivenerQuestion::NoTextInCopy {
            text_items: count(*text_items),
        },
    }
}

// ── the project card ─────────────────────────────────────────────────────────────────────

pub(crate) fn container(container: Container) -> ScrivenerContainer {
    match container {
        Container::Folder => ScrivenerContainer::Folder,
        Container::Zip => ScrivenerContainer::Zip,
        Container::Template(_) => ScrivenerContainer::Template,
    }
}

pub(crate) fn generation(generation: Generation) -> ScrivenerGeneration {
    match generation {
        Generation::Format10 => ScrivenerGeneration::Format10,
        Generation::Format15 => ScrivenerGeneration::Format15,
        Generation::Format20 => ScrivenerGeneration::Format20,
        Generation::IosStub => ScrivenerGeneration::IosStub,
    }
}

pub(crate) fn platform(platform: Platform) -> ScrivenerPlatform {
    match platform {
        Platform::Mac => ScrivenerPlatform::Mac,
        Platform::Windows => ScrivenerPlatform::Windows,
        Platform::Linux => ScrivenerPlatform::Linux,
        Platform::Ios => ScrivenerPlatform::Ios,
        Platform::Unknown => ScrivenerPlatform::Unknown,
    }
}

pub(crate) fn manifest_choice(choice: ManifestChoice) -> ScrivenerManifestChoice {
    match choice {
        ManifestChoice::Only => ScrivenerManifestChoice::Only,
        ManifestChoice::Picked => ScrivenerManifestChoice::Picked,
        ManifestChoice::Answered => ScrivenerManifestChoice::Answered,
        ManifestChoice::Identical => ScrivenerManifestChoice::Identical,
        ManifestChoice::Automatic => ScrivenerManifestChoice::Automatic,
        ManifestChoice::MobileBinder => ScrivenerManifestChoice::MobileBinder,
    }
}

pub(crate) fn title_source(source: TitleSource) -> crate::dtos::ScrivenerTitleSource {
    use crate::dtos::ScrivenerTitleSource as Dto;
    match source {
        TitleSource::CompileSettings => Dto::CompileSettings,
        TitleSource::ProjectProperties => Dto::ProjectProperties,
        TitleSource::Template => Dto::Template,
        TitleSource::FolderName => Dto::FolderName,
    }
}

pub(crate) fn author_source(source: Option<AuthorSource>) -> crate::dtos::ScrivenerAuthorSource {
    use crate::dtos::ScrivenerAuthorSource as Dto;
    match source {
        None => Dto::NotStated,
        Some(AuthorSource::CompileSettings) => Dto::CompileSettings,
        Some(AuthorSource::ProjectProperties) => Dto::ProjectProperties,
    }
}

pub(crate) fn language_source(
    source: Option<LanguageSource>,
) -> crate::dtos::ScrivenerLanguageSource {
    use crate::dtos::ScrivenerLanguageSource as Dto;
    match source {
        None => Dto::NotStated,
        Some(LanguageSource::CompileSettings) => Dto::CompileSettings,
        Some(LanguageSource::Script) => Dto::Script,
    }
}

/// Documents (texts) inside the Draft folder, at any depth: the card's "in the
/// manuscript". One walk with the depth at which the Draft opened, never recursion.
pub(crate) fn manuscript_documents(project: &Project) -> u32 {
    let mut draft_at: Option<u16> = None;
    let mut documents = 0u32;
    for row in &project.binder {
        if draft_at.is_some_and(|depth| row.depth <= depth) {
            draft_at = None;
        }
        match (&row.kind, draft_at) {
            (ItemKind::DraftFolder, None) => draft_at = Some(row.depth),
            (ItemKind::Text, Some(_)) => documents = documents.saturating_add(1),
            _ => {}
        }
    }
    documents
}
