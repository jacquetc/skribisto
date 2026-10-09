// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The structure plan an analysis returns: the same structure from every
//! generation, decisions made on what documents hold rather than on their
//! size, and the invariants over any binder the fixture builder can write, under
//! any choice the review offers.

use proptest::prelude::*;
use scrivener_fixtures::{Generation, Item, Kind, Platform, Spec};
use scrivener_import::content::Carries;
use scrivener_import::structure::{
    self, Adjustment, BooksChoice, ContainerRung, Destination, PlannedRow, PlannedTitle, Source,
    StructureNotice, StructurePlan, TextKind, TrashChoice,
};
use scrivener_import::{Analysis, ReadProject};

use crate::support::{read, run, summary, temp, write};

fn title_of(row: &PlannedRow) -> String {
    match &row.title {
        PlannedTitle::Own(t) => t.clone(),
        PlannedTitle::NotesOf(t) => format!("{t} (notes)"),
        PlannedTitle::ProjectTitle(t) => format!("[{t}]"),
        PlannedTitle::ProjectNotes => "<project notes>".to_string(),
        PlannedTitle::Recovered => "<recovered>".to_string(),
        PlannedTitle::FromScrivener => "<from scrivener>".to_string(),
        PlannedTitle::FromScrivenerNote(section) => format!("<{}>", section.token()),
    }
}

fn show(rows: &[PlannedRow]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            format!(
                "{}{:?}/{:?} {}{}",
                "  ".repeat(r.depth as usize),
                r.role,
                r.sub_role,
                title_of(r),
                if r.activated { "" } else { " ~" }
            )
            .trim_end()
            .to_string()
        })
        .collect()
}

fn analyse(spec: &Spec) -> ReadProject {
    let dir = temp();
    let path = write(spec, dir.path());
    read(run(&path))
}

fn checked(read: &ReadProject) -> &StructurePlan {
    let plan = &read.structure;
    assert_eq!(
        structure::check(&read.project, &read.content, plan),
        Ok(()),
        "the plan an analysis returns passes its own check"
    );
    plan
}

#[test]
fn every_generation_plans_the_same_novel() {
    let expected_manuscript = [
        "Folder/Book [Lighthouse]",
        "  Folder/Part Part One",
        "    Folder/ChapterScene Chapter 1",
        "      Item/Scene Arrival",
        "      Item/Note Arrival (notes)",
        "      Item/Scene The Harbour",
        "    Folder/ChapterScene Chapter 2",
        "      Item/Scene Night Watch",
        "      Item/Scene",
        "  Item/BookEnd",
    ];
    for (generation, platform) in [
        (Generation::Format20, Platform::Mac),
        (Generation::Format20, Platform::Windows),
        (Generation::Format15, Platform::Mac),
        (Generation::Format15, Platform::Windows),
        (Generation::Format15, Platform::Linux),
        (Generation::Format10, Platform::Mac),
        (Generation::Format10, Platform::Windows),
        (Generation::IosStub, Platform::Ios),
    ] {
        let read = analyse(&Spec::novel(generation, platform));
        let plan = checked(&read);
        let label = format!("{generation:?} {platform:?}");
        let stub = generation == Generation::IosStub;
        let manuscript: Vec<String> = show(&plan.manuscript)
            .into_iter()
            .filter(|l| !(stub && l.ends_with("(notes)")))
            .collect();
        let expected: Vec<String> = expected_manuscript
            .iter()
            .filter(|l| !(stub && l.ends_with("(notes)")))
            .map(|s| s.to_string())
            .collect();
        assert_eq!(manuscript, expected, "{label}");
        let mut notes = vec!["Folder/Note Research"];
        if !stub {
            notes.extend([
                "  Item/Note Harbour map",
                "  Item/Note Tide tables",
                "  Item/Note Mara",
                "  Item/Note Mara (notes)",
            ]);
        } else {
            notes.push("  Item/Note Mara");
        }
        notes.extend(["Folder/Note Notes", "  Item/Note Ideas"]);
        if stub {
            // An iPad binder names no Templates folder: it is a notes folder.
            notes.extend(["Folder/Note Templates", "  Item/Note Scene sheet"]);
        }
        if matches!(generation, Generation::Format10 | Generation::Format15) {
            notes.extend(["Folder/Note <project notes>", "  Item/Note General"]);
        }
        if !stub {
            // The session target and the custom value on a scene have no field to go to:
            // the "From Scrivener" folder lists them, before the Trash (X-29).
            notes.extend([
                "Folder/Note <from scrivener>",
                "  Item/Note <project>",
                "  Item/Note <custom-metadata>",
            ]);
        }
        notes.push("Item/Note Deleted scene ~");
        assert_eq!(show(&plan.notes), notes, "{label}");
        if stub {
            assert!(plan.templates.is_empty(), "{label}");
        } else {
            assert_eq!(plan.templates.len(), 1, "{label}");
            assert_eq!(plan.templates[0].name, "Scene sheet");
        }
        assert_eq!(plan.trash_roots.len(), 1);
        // The chapter folder keeps its own prose; the untitled scene with no
        // file carries nothing and has no content.
        let chapter = &plan.manuscript[2];
        assert!(
            chapter
                .contents
                .iter()
                .any(|c| c.role == structure::ContentRole::SceneText)
        );
        assert!(plan.manuscript[8].contents.is_empty(), "{label}");
    }
}

#[test]
fn an_empty_rtf_file_is_not_writing() {
    // Scrivener writes a few hundred bytes of RTF for a folder holding nothing:
    // the plan must not give such a folder a first child for its "text".
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.items[1].text = vec!["placeholder".to_string()];
    let dir = temp();
    let folder = write(&spec, dir.path());
    let shell = br"{\rtf1\ansi\ansicpg1252\cocoartf2580{\fonttbl\f0\fnil Palatino;}\pard\f0 <!$Scr_Ps::0>\par}";
    if let Err(e) = std::fs::write(folder.join(spec.text_path(1)), shell) {
        panic!("writing the shell: {e}");
    }
    let read = read(run(&folder));
    let plan = checked(&read);
    assert!(
        read.project.binder[1].payload.text.is_some(),
        "the file is there"
    );
    assert_eq!(read.content.items[1].text.carries, Carries::No);
    assert_eq!(
        show(&plan.manuscript)[1..3],
        [
            "  Folder/Part Part One",
            "    Folder/ChapterScene Chapter 1"
        ],
        "no own-text child for a part holding nothing"
    );
}

#[test]
fn a_manuscript_kept_in_one_document_is_offered_its_chapters() {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.items.truncate(1);
    spec.template_folder = None;
    let mut doc = Item::new(1, Kind::Text, "Manuscript").compiled();
    doc.text = vec![
        "A word before it all begins.".to_string(),
        "CHAPTER I".to_string(),
        "The keeper counted the ships.".to_string(),
        "CHAPTER II".to_string(),
        "Fog came in from the west.".to_string(),
        "CHAPTER III".to_string(),
        "The ferry was late again.".to_string(),
    ];
    spec.items.push(doc);
    let read = read(run(&write(&spec, temp().path())));
    let plan = checked(&read);
    assert_eq!(read.content.items[1].text.headings.len(), 3);
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Lighthouse]",
            "  Item/Scene Manuscript",
            "  Item/ChapterScene CHAPTER I",
            "  Item/ChapterScene CHAPTER II",
            "  Item/ChapterScene CHAPTER III",
            "  Item/BookEnd",
        ]
    );
    assert!(
        plan.notices
            .contains(&StructureNotice::OneDocumentManuscript {
                item: 1,
                headings: 3,
                split: true
            })
    );
    assert!(
        !plan
            .notices
            .contains(&StructureNotice::TitleRepeatsNumber { item: 1 }),
        "a scene's title is not a heading"
    );
}

#[test]
fn the_plan_travels_with_the_analysis_as_json() {
    let read = analyse(&Spec::novel(Generation::Format15, Platform::Windows));
    let analysis = Analysis::Read(Box::new(read.clone()));
    let json = serde_json::to_string(&analysis).unwrap_or_default();
    let back: Result<Analysis, _> = serde_json::from_str(&json);
    assert!(back.is_ok_and(|b| b == analysis));
    // And the writer's choices re-derive it on the other side.
    let adjusted = structure::adjust(
        &read.project,
        &read.content,
        &read.structure,
        &Adjustment::Trash(TrashChoice::LeaveOut),
    );
    let adjusted = match adjusted {
        Ok(plan) => plan,
        Err(e) => panic!("{e}"),
    };
    assert!(adjusted.trash_roots.is_empty());
    assert!(
        adjusted
            .notices
            .iter()
            .any(|n| matches!(n, StructureNotice::TrashLeftOut { items: 1, .. }))
    );
}

// ---------------------------------------------------------------- properties

/// One generated item: depth step, kind, title, writing.
fn item() -> impl Strategy<Value = (u16, u8, u8, u8, bool, bool, u8)> {
    (
        0u16..4,
        0u8..10,
        0u8..16,
        0u8..3,
        any::<bool>(),
        any::<bool>(),
        0u8..4,
    )
}

const TITLES: &[&str] = &[
    "Chapter 1",
    "Chapter 1",
    "Part Two",
    "Book One",
    "Book Two",
    "Prologue",
    "Title Page",
    "Appendix A",
    "Characters",
    "Chapter",
    "Front Matter",
    "Notes",
    "Arrival",
    "Episode 3",
    "Draft - old",
    "Scene 2",
];

fn generation() -> impl Strategy<Value = (Generation, Platform)> {
    prop_oneof![
        Just((Generation::Format20, Platform::Mac)),
        Just((Generation::Format20, Platform::Windows)),
        Just((Generation::Format15, Platform::Windows)),
        Just((Generation::Format10, Platform::Mac)),
    ]
}

/// Every choice the review offers, one per control.
fn offered(plan: &StructurePlan, pick: u8) -> Vec<Adjustment> {
    let mut out = Vec::new();
    let rungs = [
        ContainerRung::Part,
        ContainerRung::Chapter,
        ContainerRung::Plain,
    ];
    for c in &plan.review.containers {
        out.push(Adjustment::Containers {
            depth: c.depth,
            rung: Some(rungs[(pick as usize + c.depth as usize) % 3]),
        });
    }
    for t in &plan.review.texts {
        out.push(Adjustment::Texts {
            depth: t.depth,
            kind: Some(TextKind::Chapters),
        });
    }
    let destinations = [
        Destination::Manuscript,
        Destination::StoryBible,
        Destination::LeaveOut,
        Destination::Notes,
    ];
    for (k, r) in plan.review.roots.iter().enumerate() {
        out.push(Adjustment::Root {
            item: r.item,
            destination: Some(destinations[(pick as usize + k) % 4]),
        });
    }
    if plan.review.books.is_some() {
        out.push(Adjustment::Books(Some(if pick.is_multiple_of(2) {
            BooksChoice::Several
        } else {
            BooksChoice::One
        })));
    }
    out.push(Adjustment::Trash(if pick.is_multiple_of(2) {
        TrashChoice::LeaveOut
    } else {
        TrashChoice::Import
    }));
    out
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 24, ..ProptestConfig::default() })]

    /// Any binder the fixture builder writes, in any generation, analyses to a
    /// plan that passes every invariant, places every text the builder wrote
    /// exactly once, and stays within the invariants under every choice the
    /// review offers.
    #[test]
    fn any_written_binder_plans_within_the_invariants(
        rows in prop::collection::vec(item(), 1..40),
        (g, p) in generation(),
        picks in prop::collection::vec(any::<u8>(), 1..3),
    ) {
        let mut spec = Spec::novel(g, p);
        spec.template_folder = None;
        spec.items = vec![Item::new(0, Kind::Draft, "Manuscript").compiled()];
        let mut previous = 0u16;
        let mut texts = 0usize;
        for (i, (step, kind, title, writing, notes, compiled, section)) in rows.iter().enumerate() {
            let depth = (*step).min(previous + 1);
            previous = depth;
            let kind = match kind {
                0 if depth == 0 => Kind::Research,
                1 if depth == 0 => Kind::Trash,
                0..=3 => Kind::Folder,
                4 => Kind::Image,
                _ => Kind::Text,
            };
            let mut item = Item::new(depth, kind.clone(), TITLES[*title as usize]);
            if matches!(kind, Kind::Image) {
                item.media = Some(("png".to_string(), scrivener_fixtures::TINY_PNG.to_vec()));
            }
            if *writing > 0 && !matches!(kind, Kind::Research | Kind::Trash) {
                item = item.with_text(i, *writing as usize);
                texts += 1;
            }
            if *notes && !matches!(kind, Kind::Research | Kind::Trash) {
                item = item.with_notes(i);
            }
            if *compiled {
                item = item.compiled();
            }
            if *section > 0 && g == Generation::Format20 {
                item.section_type = Some(*section as usize % 2);
            }
            spec.items.push(item);
        }
        let dir = temp();
        let analysis = run(&write(&spec, dir.path()));
        let Analysis::Read(read) = analysis else {
            return Err(TestCaseError::fail(format!("not read: {}", summary(&analysis))));
        };
        prop_assert_eq!(structure::check(&read.project, &read.content, &read.structure), Ok(()));
        // Independently of the checker: every text the builder wrote lands once.
        let plan = &read.structure;
        let placed_texts = plan
            .manuscript
            .iter()
            .chain(&plan.notes)
            .flat_map(|r| &r.contents)
            .filter_map(|c| match &c.body {
                structure::Body::From(sources) => Some(sources.clone()),
                structure::Body::Title | structure::Body::Generated(_) => None,
            })
            .flatten()
            .filter(|s| matches!(s, Source::Text { .. } | Source::TextPart { part: 0, .. }))
            .count();
        let split_without_lead = plan
            .review
            .splits
            .iter()
            .filter(|s| read.content.items[s.item as usize].text.lead == Carries::No)
            .count();
        prop_assert_eq!(placed_texts + split_without_lead, texts);
        let mut current = read.structure.clone();
        for pick in picks {
            for adjustment in offered(&current, pick) {
                let next = structure::adjust(&read.project, &read.content, &current, &adjustment);
                prop_assert!(next.is_ok(), "{:?} after {:?}", next.err(), adjustment);
                current = next.unwrap_or_else(|e| panic!("{e}"));
            }
        }
    }
}
