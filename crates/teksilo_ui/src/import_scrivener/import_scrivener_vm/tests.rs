// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::import_management::{
    ScrivenerArg, ScrivenerFindingFamily, ScrivenerMobileEdits, ScrivenerPlace,
    ScrivenerReportGroup,
};

/// A wizard over a job of its own, as each running copy of Skribisto has one.
fn wizard() -> ImportScrivenerViewModel {
    let app_ctx = Rc::new(AppContext::new());
    ImportScrivenerViewModel::new(app_ctx.clone(), ProjectImportJob::new(app_ctx))
}

fn row(
    binder: ScrivenerBinder,
    depth: i64,
    role: BinderItemRole,
    sub_role: BinderItemSubRole,
    title: &str,
    item: i64,
) -> ScrivenerPreviewRow {
    ScrivenerPreviewRow::Row {
        binder,
        depth,
        role,
        sub_role,
        title: title.into(),
        title_kind: ScrivenerTitleKind::Own,
        exportable: true,
        activated: true,
        item,
        why: Default::default(),
    }
}

fn item(item: i64, title: &str) -> ScrivenerItemRef {
    ScrivenerItemRef::Item {
        item,
        binder_path: vec![title.into()],
    }
}

fn finding(key: &str, group: ScrivenerReportGroup, args: Vec<ScrivenerArg>) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Structure,
        group,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args,
    }
}

/// A project that reads, at `source`: a book with a chapter of two scenes, a Research
/// folder beside it, a lock, and the notices a writer meets most.
fn readable(source: &Path) -> ScrivenerAnalysisDto {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole as S;
    use ScrivenerBinder::{Manuscript, Notes};
    ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Read,
        notices: vec![
            ScrivenerFinding::Reported {
                key: "scrivener-notice-lock-present".into(),
                family: ScrivenerFindingFamily::Source,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: vec![ScrivenerArg::Flag {
                    name: "this_machine".into(),
                    value: false,
                }],
            },
            ScrivenerFinding::Reported {
                key: "scrivener-notice-language-not-stated".into(),
                family: ScrivenerFindingFamily::Source,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: Vec::new(),
            },
            ScrivenerFinding::Reported {
                key: "scrivener-notice-snapshots-not-imported".into(),
                family: ScrivenerFindingFamily::Source,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: vec![ScrivenerArg::Count {
                    name: "snapshots".into(),
                    value: 4,
                }],
            },
        ],
        folder_title: "The Lighthouse: Keeper".into(),
        platform: ScrivenerPlatform::Mac,
        writer_version: "3.3.6".into(),
        modified: Some(1_717_236_000),
        documents: 3,
        manuscript_documents: 2,
        research_files: 2,
        proposed_title: "The Lighthouse Keeper".into(),
        proposed_authors: vec!["Ada Marlowe".into(), "Ben Hart".into()],
        structure: ScrivenerStructure::Planned {
            rows: vec![
                row(Manuscript, 0, Folder, S::Book, "The Lighthouse Keeper", 0),
                row(Manuscript, 1, Folder, S::ChapterScene, "", 1),
                row(Manuscript, 2, Item, S::Scene, "Arrival", 2),
                row(Manuscript, 2, Item, S::Scene, "Storm", 3),
                row(Notes, 0, Folder, S::Note, "Research", 4),
                row(Notes, 1, Item, S::Note, "Tides", 5),
            ],
            controls: vec![
                ScrivenerControl::Containers {
                    depth: 1,
                    containers: 1,
                    adjustable: 1,
                    detected: ScrivenerContainerRung::Chapter,
                    mixed: false,
                    chosen: None,
                    examples: vec![item(1, "Chapter")],
                    signals: Vec::new(),
                    flags: Vec::new(),
                },
                ScrivenerControl::Root {
                    item: item(4, "Research"),
                    items: 1,
                    detected: ScrivenerDestination::Notes,
                    chosen: None,
                    looks_like_manuscript: false,
                },
                ScrivenerControl::Trash {
                    items: 2,
                    with_writing: 1,
                    chosen: ScrivenerTrashChoice::LeaveOut,
                },
            ],
            notices: vec![finding(
                "scrivener-structure-trash-left-out",
                ScrivenerReportGroup::LeftBehind,
                vec![
                    ScrivenerArg::Count {
                        name: "items".into(),
                        value: 2,
                    },
                    ScrivenerArg::Count {
                        name: "with_writing".into(),
                        value: 1,
                    },
                ],
            )],
            choices: vec![ScrivenerChoice::Trash {
                choice: ScrivenerTrashChoice::LeaveOut,
            }],
            set_aside_trashed: 0,
            set_aside_recovered: 0,
            shape: Default::default(),
            chapter_encoding: Default::default(),
            templates: Vec::new(),
            tags: Vec::new(),
        },
        fingerprint: format!("fingerprint of {}", source.display()),
        ..ScrivenerAnalysisDto::default()
    }
}

/// The wizard as it stands once `analysis` came back for `source`.
fn read(vm: &ImportScrivenerViewModel, source: &Path, analysis: ScrivenerAnalysisDto) {
    *vm.inner.inspected.borrow_mut() = source.to_string_lossy().into_owned();
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(Rc::new(analysis)));
}

/// A Scrivener project folder on disk, with a writable folder beside it.
fn project_on_disk() -> (tempfile::TempDir, PathBuf) {
    let dir = match tempfile::tempdir() {
        Ok(dir) => dir,
        Err(e) => panic!("a scratch folder: {e}"),
    };
    let project = dir.path().join("Lighthouse.scriv");
    if let Err(e) = std::fs::create_dir(&project) {
        panic!("a project folder: {e}");
    }
    (dir, project)
}

// ── pure helpers ─────────────────────────────────────────────────────────────────────────

#[test]
fn a_file_name_drops_what_a_platform_refuses() {
    assert_eq!(
        file_name_from("Lantern and Pulleys: The scuffle"),
        "Lantern and Pulleys The scuffle"
    );
    assert_eq!(file_name_from("  A/B\\C?  "), "A B C");
    assert_eq!(file_name_from("Draft..."), "Draft");
    assert_eq!(file_name_from("***"), "");
}

/// A template is named by its own title, what the writer saw in Scrivener's template
/// chooser; any other project by its folder's name, which the analysis already cleaned of
/// backup and sync decorations.
#[test]
fn the_default_name_is_the_projects_name() {
    let mut analysis = ScrivenerAnalysisDto {
        folder_title: "quorvelmantis".into(),
        template_title: "Ao3 Fan Fiction".into(),
        container: ScrivenerContainer::Template,
        ..ScrivenerAnalysisDto::default()
    };
    assert_eq!(default_name(&analysis), "Ao3 Fan Fiction");
    analysis.container = ScrivenerContainer::Folder;
    assert_eq!(default_name(&analysis), "quorvelmantis");
    analysis.folder_title = "::".into();
    crate::test_support::with_real_messages(|| {
        assert_eq!(default_name(&analysis), "Scrivener project");
    });
}

#[test]
fn a_picked_manifest_stands_for_its_project_folder() {
    assert_eq!(
        project_folder(Path::new("/books/Novel.scriv/project.scrivx")),
        PathBuf::from("/books/Novel.scriv")
    );
    assert_eq!(
        project_folder(Path::new("/books/Novel.scriv")),
        PathBuf::from("/books/Novel.scriv")
    );
    assert_eq!(
        project_folder(Path::new("/books/Novel.zip")),
        PathBuf::from("/books/Novel.zip")
    );
}

/// Beside the project, unless that folder cannot be written to or is one of Scrivener's
/// backup folders; then the remembered folder; then home.
#[test]
fn the_default_folder_is_beside_the_project() {
    let all = |_: &Path| true;
    let remembered = Some(PathBuf::from("/remembered"));
    let home = Some(PathBuf::from("/home/writer"));
    assert_eq!(
        default_location(
            Path::new("/books/Novel.scriv/Novel.scrivx"),
            false,
            &all,
            remembered.clone(),
            home.clone()
        ),
        Some(PathBuf::from("/books"))
    );
    let not_books = |p: &Path| p != Path::new("/books");
    assert_eq!(
        default_location(
            Path::new("/books/Novel.scriv"),
            false,
            &not_books,
            remembered.clone(),
            home.clone()
        ),
        Some(PathBuf::from("/remembered"))
    );
    assert_eq!(
        default_location(
            Path::new("/Users/a/Library/Application Support/Scrivener/Backups/Novel.bak1.zip"),
            false,
            &all,
            None,
            home.clone()
        ),
        home,
        "never inside Scrivener's backups folder"
    );
    assert_eq!(
        default_location(
            Path::new("/home/a/Dropbox/Scrivener backups/Novel.bak3.zip"),
            true,
            &all,
            None,
            home.clone()
        ),
        home,
        "nor in a backup folder of another name"
    );
    assert_eq!(
        default_location(
            Path::new("/books/Novel.scriv"),
            false,
            &|_| false,
            None,
            None
        ),
        None
    );
}

/// Who last saved the project, as the manifest says; a project that names no version
/// says its format, and a platform the import does not recognise reads as another
/// system, never as an iPhone.
#[test]
fn made_with_follows_what_the_manifest_says() {
    crate::test_support::with_real_messages(|| {
        let said = |platform, version: &str| {
            made_with(&platform, version, &ScrivenerGeneration::Format15).resolve_now()
        };
        assert_eq!(
            said(ScrivenerPlatform::Mac, "3.3.6"),
            "Last saved by Scrivener 3.3.6 for macOS"
        );
        assert_eq!(
            said(ScrivenerPlatform::Windows, "1.9.16"),
            "Last saved by Scrivener 1.9.16 for Windows"
        );
        assert_eq!(
            said(ScrivenerPlatform::Unknown, "2.9"),
            "Last saved by Scrivener 2.9"
        );
        assert_eq!(
            said(ScrivenerPlatform::Mac, ""),
            "Scrivener project, format 1.5"
        );
    });
}

#[test]
fn a_question_is_answered_for_the_writer_only_on_evidence() {
    let manifests = |preselected: Option<&str>| ScrivenerQuestion::SeveralManifests {
        candidates: vec![
            ScrivenerManifestCandidate::Candidate {
                name: "A.scrivx".into(),
                creator: None,
                version: None,
                modified: None,
                items: None,
                matches_autosave: false,
                identical_to_preselected: false,
            },
            ScrivenerManifestCandidate::Candidate {
                name: "B.scrivx".into(),
                creator: None,
                version: None,
                modified: None,
                items: None,
                matches_autosave: true,
                identical_to_preselected: false,
            },
        ],
        preselected: preselected.map(str::to_string),
        evidence: Vec::new(),
    };
    assert_eq!(preselection(&manifests(Some("B.scrivx"))), Some(1));
    assert_eq!(preselection(&manifests(None)), None);
    assert_eq!(
        preselection(&ScrivenerQuestion::SeveralProjects {
            candidates: vec!["a".into(), "b".into()]
        }),
        None
    );
    let sibling = |clean| ScrivenerQuestion::UpgradedSibling {
        backup: "X Backup.scriv".into(),
        sibling: "X.scriv".into(),
        sibling_generation: None,
        preselect_sibling: clean,
    };
    assert_eq!(preselection(&sibling(true)), Some(0));
    assert_eq!(preselection(&sibling(false)), None);

    let mut answers = Answers::default();
    assert!(answer(&mut answers, &manifests(None), 1));
    assert_eq!(answers.manifest, "B.scrivx");
    assert_eq!(chosen_answer(&answers, &manifests(None)), 1);
    assert!(answer(&mut answers, &sibling(false), 1));
    assert_eq!(answers.upgraded_sibling, ScrivenerSiblingChoice::Backup);
    assert!(!answer(&mut answers, &sibling(false), 7));
    forget_answer(&mut answers, &manifests(None));
    assert!(answers.manifest.is_empty());
}

/// A later choice about the same thing replaces the earlier one; choices about different
/// things stand together.
#[test]
fn a_choice_replaces_only_its_own_subject() {
    let root = |item, destination| ScrivenerChoice::Root { item, destination };
    assert!(same_subject(
        &root(3, ScrivenerDestination::Notes),
        &root(3, ScrivenerDestination::LeaveOut)
    ));
    assert!(!same_subject(
        &root(3, ScrivenerDestination::Notes),
        &root(4, ScrivenerDestination::Notes)
    ));
    assert!(same_subject(
        &ScrivenerChoice::Trash {
            choice: ScrivenerTrashChoice::Import
        },
        &ScrivenerChoice::Trash {
            choice: ScrivenerTrashChoice::LeaveOut
        }
    ));
}

/// The preview gives each binder a band, words a blank title by its type, and opens the
/// manuscript to its chapters with everything else folded.
#[test]
fn the_preview_words_the_planned_binder() {
    crate::test_support::with_real_messages(|| {
        let ScrivenerStructure::Planned { rows, .. } = readable(Path::new("/b")).structure else {
            panic!("a planned structure");
        };
        let preview = preview_rows(&rows);
        let titles: Vec<String> = preview.iter().map(|r| r.title.resolve_now()).collect();
        assert_eq!(
            titles,
            [
                "Manuscript",
                "The Lighthouse Keeper",
                "Chapter",
                "Arrival",
                "Storm",
                "Notes",
                "Research",
                "Tides"
            ]
        );
        assert!(preview[0].band && preview[5].band);
        assert!(
            preview[2].title_is_placeholder,
            "a blank title shows its type"
        );
        assert_eq!(preview[3].becomes.resolve_now(), "Scene");
        assert!(!preview[2].folded_by_default && preview[6].folded_by_default);
        assert_eq!(preview[3].depth, 3);
    });
}

/// The choices the Review page shows: only the controls the plan offers, each worded,
/// then the text choices that are always there.
#[test]
fn the_review_rows_follow_the_plans_controls() {
    crate::test_support::with_real_messages(|| {
        let ScrivenerStructure::Planned { controls, .. } = readable(Path::new("/b")).structure
        else {
            panic!("a planned structure");
        };
        let rows = review_rows(&controls, &Signal::new(true), &Signal::new(false));
        let labels: Vec<String> = rows.iter().map(|r| r.label.resolve_now()).collect();
        assert_eq!(
            labels,
            [
                "Levels",
                "Folders at level 1",
                "Top-level folders",
                "Research",
                "Bring the Trash across (2 items)",
                "Text",
                "Blank lines become scene breaks",
                "Lines of ornaments become scene breaks",
            ]
        );
        assert!(matches!(
            rows[1].control,
            ReviewControl::Pick {
                target: PickTarget::Containers(1),
                ref selected,
                ..
            } if selected.get() == Some(1)
        ));
        assert!(matches!(
            rows[4].control,
            ReviewControl::Toggle {
                target: ToggleTarget::Trash,
                ref on,
            } if !on.get()
        ));
    });
}

/// The strip says each kind once, in the report groups' order.
#[test]
fn the_strip_lists_one_line_per_kind_in_group_order() {
    crate::test_support::with_real_messages(|| {
        let left = finding(
            "scrivener-structure-trash-left-out",
            ScrivenerReportGroup::LeftBehind,
            vec![ScrivenerArg::Count {
                name: "items".into(),
                value: 2,
            }],
        );
        let moved = finding(
            "scrivener-structure-own-text-moved",
            ScrivenerReportGroup::BroughtAcrossDifferently,
            Vec::new(),
        );
        let lines: Vec<String> = before_lines(&[&left, &moved, &moved])
            .iter()
            .map(|l| l.resolve_now())
            .collect();
        assert_eq!(
            lines,
            [
                "2 folders have text of their own, which moves to a first document inside them.",
                "The Trash is left behind: 2 items, none of them with writing.",
            ]
        );
    });
}

/// The strip says what the metadata mapping will do, one line for each field, collection
/// or note a kind names: the custom metadata and the targets of items without a row are
/// two facts, and two notes of the "From Scrivener" folder list two things.
#[test]
fn the_strip_says_each_field_and_each_note_of_its_own() {
    crate::test_support::with_real_messages(|| {
        let token = |name: &str, value: &str| ScrivenerArg::Token {
            name: name.into(),
            value: value.into(),
        };
        let count = |name: &str, value: i64| ScrivenerArg::Count {
            name: name.into(),
            value,
        };
        let without_row = |field: &str, items: i64| {
            finding(
                "scrivener-metadata-values-without-row",
                ScrivenerReportGroup::LeftBehind,
                vec![token("field", field), count("items", items)],
            )
        };
        let listed = |section: &str, n: i64| {
            finding(
                "scrivener-metadata-listed-in-import-note",
                ScrivenerReportGroup::BroughtAcrossDifferently,
                vec![token("section", section), count("count", n)],
            )
        };
        let (custom, target) = (without_row("custom-field", 7), without_row("target", 1));
        let (bookmarks, project) = (listed("bookmarks", 12), listed("project", 3));
        let keywords = finding(
            "scrivener-metadata-keywords-became-tags",
            ScrivenerReportGroup::BroughtAcrossDifferently,
            vec![count("tags", 4), count("rows", 9)],
        );
        let lines: Vec<String> = before_lines(&[&custom, &bookmarks, &keywords, &target, &project])
            .iter()
            .map(|l| l.resolve_now())
            .collect();
        assert_eq!(
            lines,
            [
                "12 bookmarks no link of Skribisto shows are listed in the note “Bookmarks” of \
                 the “From Scrivener” folder.",
                "4 keywords become tags, on 9 items.",
                "3 details of the project are listed in the note \
                 “Project” of the “From Scrivener” folder.",
                "The custom metadata of 7 items with no place in the new project does not come \
                 across.",
                "The target of 1 item with no place in the new project does not come across.",
            ]
        );
    });
}

/// The headline counts the kinds that could not be read or need checking, never what
/// came across differently, and the result waits for the writer only when it counts one.
#[test]
fn the_headline_counts_only_what_needs_a_look() {
    let result = ImportScrivenerProjectResultDto {
        output_path: "/out/Novel.skrib".into(),
        research_folder: String::new(),
        totals: ScrivenerTotals::Counted {
            rows: 412,
            words: 0,
            footnotes: 0,
            comments: 0,
            pictures: 0,
            templates: 0,
            tags: 0,
            rungs: 0,
            trashed: 0,
            research_files: 0,
            synopses: 0,
            document_notes: 0,
        },
        findings: vec![
            finding(
                "scrivener-prose-unreadable",
                ScrivenerReportGroup::CouldNotBeRead,
                Vec::new(),
            ),
            finding(
                "scrivener-prose-unreadable",
                ScrivenerReportGroup::CouldNotBeRead,
                Vec::new(),
            ),
            // Filed by the table: a lock is something to check.
            ScrivenerFinding::Reported {
                key: "scrivener-notice-lock-present".into(),
                family: ScrivenerFindingFamily::Source,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: Vec::new(),
            },
            finding(
                "scrivener-structure-own-text-moved",
                ScrivenerReportGroup::BroughtAcrossDifferently,
                Vec::new(),
            ),
        ],
    };
    crate::test_support::with_real_messages(|| {
        let outcome = record_outcome(result.clone());
        assert_eq!(
            outcome.result.resolve_now(),
            "The new project holds 412 items. 2 things need a look."
        );
        assert!(outcome.persistent);
        assert!(
            outcome.report.is_empty(),
            "the report is a dialog of its own"
        );
        let last = last_import().map(|l| (l.items(), l.findings.len(), l.findings_to_check()));
        assert_eq!(last, Some((412, 4, 2)));

        let quiet = ImportScrivenerProjectResultDto {
            findings: Vec::new(),
            ..result.clone()
        };
        let outcome = record_outcome(quiet);
        assert_eq!(
            outcome.result.resolve_now(),
            "The new project holds 412 items."
        );
        assert!(!outcome.persistent);
    });
}

// ── the wizard ───────────────────────────────────────────────────────────────────────────

/// A project that reads fills every page: the card, the notices on it, the Save page's
/// hints and fields, the review, the strip.
#[test]
fn a_readable_project_fills_every_page() {
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let vm = wizard();
        read(&vm, &project, readable(&project));

        assert_eq!(vm.inspection_page().get(), PAGE_CARD);
        assert_eq!(vm.card_name().get(), "The Lighthouse: Keeper");
        assert_eq!(
            vm.card_made_with().get(),
            "Last saved by Scrivener 3.3.6 for macOS"
        );
        assert_eq!(vm.card_counts().get(), "3 documents, 2 in the manuscript");
        assert!(vm.card_saved_on().get().starts_with("Saved on 2024-06-01"));
        assert_eq!(vm.notices().len(), 1, "the lock; the rest belong elsewhere");
        assert_eq!(vm.save_hints().len(), 1, "the language");
        assert_eq!(vm.before_you_import().len(), 2, "snapshots and the Trash");
        assert!(vm.can_leave_project_step().get());
        assert!(vm.can_leave_review_step().get());
        assert_eq!(vm.preview().rows().len(), 8);

        assert_eq!(vm.name().get(), "The Lighthouse Keeper");
        assert_eq!(
            vm.location().get(),
            dir.path().to_string_lossy().into_owned(),
            "beside the project"
        );
        assert_eq!(vm.title().get(), "The Lighthouse Keeper");
        assert_eq!(vm.author().get(), "Ada Marlowe, Ben Hart");
        assert!(vm.language().get().is_some(), "the interface's own");
        assert!(vm.can_import().get());
    });
}

/// A template's card names no computer: the device its manifest names is the computer of
/// whoever made the template, someone else (C18). A project's card names its own.
#[test]
fn a_templates_card_names_no_computer() {
    crate::test_support::with_real_messages(|| {
        let (_dir, project) = project_on_disk();
        let named = || ScrivenerComputers::Named {
            device: Some("Ada's Laptop".into()),
            lock_host: None,
        };
        let vm = wizard();
        let mut analysis = readable(&project);
        analysis.computers = named();
        read(&vm, &project, analysis);
        assert!(
            vm.card_saved_on().get().contains("Ada's Laptop"),
            "{}",
            vm.card_saved_on().get()
        );

        let vm = wizard();
        let mut analysis = readable(&project);
        analysis.computers = named();
        analysis.container = ScrivenerContainer::Template;
        analysis.template_title = "Starter".into();
        read(&vm, &project, analysis);
        let saved_on = vm.card_saved_on().get();
        assert!(saved_on.starts_with("Saved on 2024-06-01"), "{saved_on}");
        assert!(!saved_on.contains("Ada"), "{saved_on}");
    });
}

/// A question to accept keeps Next off until it is accepted, and the import then carries
/// the acceptance.
#[test]
fn a_question_to_accept_gates_next() {
    crate::test_support::with_real_messages(|| {
        let (_dir, project) = project_on_disk();
        let vm = wizard();
        let mut analysis = readable(&project);
        analysis.pending = vec![ScrivenerQuestion::UnmergedMobileEdits {
            source: ScrivenerMobileEdits::NewerMobileBinder,
        }];
        read(&vm, &project, analysis);

        assert!(!vm.can_leave_project_step().get());
        assert!(!vm.can_import().get());
        assert!(vm.request().is_none());
        let question = vm
            .questions()
            .with_item(0, |q| (q.key.clone(), q.accept.is_some()));
        assert_eq!(
            question,
            Some(("scrivener-question-unmerged-mobile-edits".to_string(), true))
        );

        vm.accept("scrivener-question-unmerged-mobile-edits", true);
        assert!(vm.can_leave_project_step().get());
        let request = vm.request();
        assert!(
            request
                .as_ref()
                .is_some_and(|r| r.accept_unmerged_mobile_edits)
        );
        assert!(request.is_some_and(|r| !r.accept_no_text));

        vm.accept("scrivener-question-unmerged-mobile-edits", false);
        assert!(!vm.can_leave_project_step().get());
    });
}

/// A refused project shows its refusal and keeps Next off; nothing is offered to import.
#[test]
fn a_refused_project_says_why() {
    crate::test_support::with_real_messages(|| {
        let vm = wizard();
        let analysis = ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Refused,
            refusal: ScrivenerFinding::Reported {
                key: "scrivener-refusal-scrivener-one-for-mac".into(),
                family: ScrivenerFindingFamily::Refusal,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: Vec::new(),
            },
            ..ScrivenerAnalysisDto::default()
        };
        read(&vm, Path::new("/books/Old.scriv"), analysis);
        assert_eq!(vm.inspection_page().get(), PAGE_REFUSED);
        let title = vm.refusal().with_item(0, |r| r.title.resolve_now());
        assert_eq!(
            title.as_deref(),
            Some("This project was made by Scrivener 1 for Mac")
        );
        assert!(!vm.can_leave_project_step().get());
        assert!(vm.request().is_none());
    });
}

/// A question to choose with no right answer waits for the writer, with nothing
/// selected; the card stays empty until a project reads.
#[test]
fn a_question_with_no_right_answer_waits() {
    let vm = wizard();
    let analysis = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Choose,
        question: ScrivenerQuestion::SeveralProjects {
            candidates: vec!["One.scriv".into(), "Two.scriv".into()],
        },
        ..ScrivenerAnalysisDto::default()
    };
    read(&vm, Path::new("/books"), analysis);
    assert_eq!(vm.inspection_page().get(), PAGE_CARD);
    assert_eq!(vm.questions().len(), 1);
    assert_eq!(
        vm.questions()
            .with_item(0, |q| (q.options.len(), q.chosen.get())),
        Some((2, NO_ANSWER))
    );
    assert!(!vm.can_leave_project_step().get());
    assert!(vm.card().get().is_none());
}

/// The request carries every word the import writes into the project, resolved in the
/// interface's language, in both shipped locales: never a message id, a slot exactly
/// where a value goes, and the default ladder's rungs.
#[test]
fn the_words_written_into_the_project_are_the_interfaces() {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            let (_dir, project) = project_on_disk();
            let vm = wizard();
            read(&vm, &project, readable(&project));
            let Some(dto) = vm.request() else {
                panic!("{locale}: a project that reads makes a request");
            };
            for word in [
                &dto.manuscript_binder_name,
                &dto.notes_binder_name,
                &dto.project_notes_name,
                &dto.recovered_name,
                &dto.untitled_template_name,
                &dto.untitled_tag_name,
            ] {
                assert!(
                    !word.trim().is_empty() && !word.starts_with("import-scrivener-"),
                    "{locale}: {word:?}"
                );
            }
            for pattern in [
                &dto.notes_of_pattern,
                &dto.research_folder_pattern,
                &dto.label_details_pattern,
                &dto.status_details_pattern,
                &dto.unnamed_label_pattern,
                &dto.unnamed_status_pattern,
                &dto.label_tag_pattern,
            ] {
                assert_eq!(
                    pattern.matches(TITLE_SLOT).count(),
                    1,
                    "{locale}: {pattern:?}"
                );
            }
            assert_eq!(dto.list_separator, ", ", "{locale}");
            // The second half's words: one per key in the converter's order, each resolved,
            // each pattern holding the slots its key takes.
            assert_eq!(
                dto.metadata_words.len(),
                METADATA_WORD_ORDER.len(),
                "{locale}"
            );
            for (key, value) in METADATA_WORD_ORDER.iter().zip(&dto.metadata_words) {
                assert!(
                    !value.trim().is_empty() && !value.starts_with("import-scrivener-"),
                    "{locale}: {key} {value:?}"
                );
                let slots = match *key {
                    "keyword-details" | "unnamed-keyword" | "keyword-tag" | "collection-tag"
                    | "field-details" | "field-tag" => 1,
                    "field-value" | "label-value" => 2,
                    _ => 0,
                };
                let found = value.matches(TITLE_SLOT).count() + value.matches(VALUE_SLOT).count();
                assert_eq!(found, slots, "{locale}: {key} {value:?}");
            }
            assert_eq!(
                dto.default_statuses.len(),
                4,
                "{locale}: the default ladder"
            );
            assert!(dto.default_statuses.iter().all(|r| matches!(
                r,
                ScrivenerDefaultRung::Rung { name, .. } if !name.starts_with("status-")
            )));
            assert_eq!(
                dto.fingerprint,
                format!("fingerprint of {}", project.display())
            );
            assert_eq!(dto.title.as_deref(), Some("The Lighthouse Keeper"));
            assert!(dto.copy_research_files, "on by default when there are some");
            assert!(dto.blank_lines_as_scene_breaks && !dto.ornaments_as_scene_breaks);
            assert!(dto.output_path.ends_with("The Lighthouse Keeper.skrib"));
        });
    }
}

/// A name the writer typed stays when a later project is read; an untouched one follows.
#[test]
fn a_name_the_writer_typed_survives_a_new_project() {
    let (_dir, project) = project_on_disk();
    let vm = wizard();
    read(&vm, &project, readable(&project));
    vm.name().set("My own name".into());
    let mut again = readable(&project);
    again.folder_title = "Another".into();
    read(&vm, &project, again);
    assert_eq!(vm.name().get(), "My own name");
}

/// The destination may not be inside the Scrivener project: Scrivener keeps only its
/// own files there.
#[test]
fn a_folder_inside_the_project_is_refused() {
    crate::test_support::with_real_messages(|| {
        let (_dir, project) = project_on_disk();
        let vm = wizard();
        read(&vm, &project, readable(&project));
        vm.location()
            .set(project.join("Files").to_string_lossy().into_owned());
        assert!(matches!(
            vm.location_validation().get(),
            ValidationState::Error(ref e) if e.resolve_now().starts_with("Choose a folder outside")
        ));
        assert!(!vm.can_import().get());
    });
}

/// Reading the form, as its fields and Import do on every frame, never touches the disk:
/// a folder deleted behind the wizard is reported as it was until something changes.
#[test]
fn reading_the_form_never_touches_the_disk() {
    let (dir, project) = project_on_disk();
    let vm = wizard();
    read(&vm, &project, readable(&project));
    let gate = vm.can_import();
    let location = vm.location_validation();
    assert!(gate.get());
    std::fs::remove_dir_all(dir.path()).unwrap();
    for _ in 0..20 {
        assert!(gate.get());
        assert!(matches!(location.get(), ValidationState::None));
    }
    vm.location().set(dir.path().to_string_lossy().into_owned());
    assert!(!gate.get(), "a change checks again");
}

/// The path field refuses a path with nothing at it, and says nothing about an empty one.
#[test]
fn the_path_field_refuses_only_nothing() {
    crate::test_support::with_real_messages(|| {
        let vm = wizard();
        assert!(matches!(
            vm.source_validation().get(),
            ValidationState::None
        ));
        vm.source_text().set("/no/such/project.scriv".into());
        vm.submit_source();
        assert!(matches!(
            vm.source_validation().get(),
            ValidationState::Error(ref e) if e.resolve_now() == "Nothing is at this path"
        ));
        assert_eq!(vm.inspection_page().get(), PAGE_EMPTY, "nothing to read");
    });
}

/// Leaving the path field reads what it holds, but never the project already read.
#[test]
fn the_same_project_is_not_read_twice() {
    let (_dir, project) = project_on_disk();
    let vm = wizard();
    vm.source_text().set(project.to_string_lossy().into_owned());
    read(&vm, &project, readable(&project));
    let before = vm.inspection().state().get().analysis();
    vm.submit_source();
    let after = vm.inspection().state().get().analysis();
    assert!(
        matches!((before, after), (Some(a), Some(b)) if Rc::ptr_eq(&a, &b)),
        "the analysis on the page stays"
    );
}

/// Import refuses while the wizard has nothing to import, and leaves the job idle.
#[test]
fn import_refuses_until_a_project_reads() {
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let vm = ImportScrivenerViewModel::new(app_ctx.clone(), job.clone());
    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    let importing = vm.clone();
    let started = Rc::new(Cell::new(true));
    let seen = started.clone();
    crate::test_support::press(&mut tree, move |c| seen.set(importing.import(c)));
    assert!(!started.get());
    assert!(!job.is_running());
}

/// The Review page's checkboxes for the prose change the request, not the plan.
#[test]
fn the_prose_choices_reach_the_request() {
    let (_dir, project) = project_on_disk();
    let vm = wizard();
    read(&vm, &project, readable(&project));
    vm.toggle(ToggleTarget::BlankLines, false);
    vm.toggle(ToggleTarget::Ornaments, true);
    let request = vm.request();
    assert!(request.is_some_and(|r| !r.blank_lines_as_scene_breaks && r.ornaments_as_scene_breaks));
}

/// A choice re-plans through the seam. The real review opens the analysis's own review
/// data, so a hand-made analysis with none is refused, in words, and the choice is still
/// recorded for the request; the fabricated one plans again.
#[test]
fn a_choice_plans_the_binder_again() {
    crate::test_support::with_real_messages(|| {
        let (_dir, project) = project_on_disk();
        let vm = wizard();
        read(&vm, &project, readable(&project));
        vm.pick(PickTarget::Root(4), 3);
        assert!(vm.choices().contains(&ScrivenerChoice::Root {
            item: 4,
            destination: ScrivenerDestination::LeaveOut
        }));
        assert_eq!(
            vm.choices()
                .iter()
                .filter(|c| matches!(c, ScrivenerChoice::Trash { .. }))
                .count(),
            1
        );
        #[cfg(not(feature = "mocks"))]
        {
            assert_eq!(
                vm.review_error().get().map(|e| e.resolve_now()).as_deref(),
                Some("The review could not be prepared. Choose the project again.")
            );
            assert!(!vm.can_leave_review_step().get());
        }
        #[cfg(feature = "mocks")]
        assert!(vm.review_error().get().is_none());
    });
}

/// The fabricated inspection answers a preselected question at once and reads the
/// project it chose; the radio stays, with the answer selected.
#[cfg(feature = "mocks")]
#[test]
fn a_preselected_question_is_answered_and_read() {
    let vm = wizard();
    let (_dir, base) = project_on_disk();
    let project = base.with_file_name("Novel Backup.scriv");
    std::fs::create_dir(&project).unwrap();
    vm.source_text().set(project.to_string_lossy().into_owned());
    vm.submit_source();
    assert_eq!(vm.inspection_page().get(), PAGE_CARD);
    assert!(vm.card().get().is_some(), "the upgraded project was read");
    assert_eq!(
        vm.questions().with_item(0, |q| q.chosen.get()),
        Some(0),
        "the upgraded sibling, chosen"
    );
    vm.choose(0, 1);
    assert_eq!(vm.questions().with_item(0, |q| q.chosen.get()), Some(1));
    assert_eq!(
        vm.notices().len(),
        3,
        "the lock, the debris, and the older copy"
    );
}

/// The fabricated inspection's choices change the preview: leaving a folder out takes
/// its band away.
#[cfg(feature = "mocks")]
#[test]
fn leaving_a_folder_out_takes_it_off_the_preview() {
    let vm = wizard();
    let (_dir, project) = project_on_disk();
    vm.source_text().set(project.to_string_lossy().into_owned());
    vm.submit_source();
    let research = |vm: &ImportScrivenerViewModel| {
        vm.preview()
            .rows()
            .iter()
            .any(|r| r.identity.item == 24 && !r.band)
    };
    assert!(research(&vm));
    vm.pick(PickTarget::Root(24), 3);
    assert!(!research(&vm));
    assert!(vm.can_leave_review_step().get());
}

/// A question answered for the writer and asked again with that answer given is the
/// writer's to answer: the wizard does not answer it a second time, which would read the
/// project in a loop.
#[test]
fn a_question_asked_again_is_left_to_the_writer() {
    let vm = wizard();
    let sibling = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Choose,
        question: ScrivenerQuestion::UpgradedSibling {
            backup: "Novel Backup.scriv".into(),
            sibling: "Novel.scriv".into(),
            sibling_generation: None,
            preselect_sibling: true,
        },
        ..ScrivenerAnalysisDto::default()
    };
    read(&vm, Path::new("/books/Novel Backup.scriv"), sibling.clone());
    assert_eq!(
        vm.inner.answers.borrow().upgraded_sibling,
        ScrivenerSiblingChoice::Sibling,
        "answered for the writer"
    );
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(Rc::new(sibling)));
    assert_eq!(vm.inspection_page().get(), PAGE_CARD);
    assert!(vm.card().get().is_none(), "nothing read again");
    assert_eq!(vm.questions().with_item(0, |q| q.chosen.get()), Some(0));
}

/// The new project would replace one open in Skribisto, here or in another running copy:
/// the import is refused with a message saying so, and nothing starts. An open project's
/// next save would otherwise write it back over the import.
#[test]
fn an_import_over_a_project_open_elsewhere_is_refused() {
    use crate::shell::open_registry;
    use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};

    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let app_ctx = Rc::new(AppContext::new());
        let job = ProjectImportJob::new(app_ctx.clone());
        let vm = ImportScrivenerViewModel::new(app_ctx.clone(), job.clone());
        read(&vm, &project, readable(&project));
        let target = dir.path().join("The Lighthouse Keeper.skrib");
        std::fs::write(&target, b"PK").unwrap();
        open_registry::claim(&target.to_string_lossy(), "The Lighthouse Keeper");

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let importing = vm.clone();
        press(&mut tree, move |c| {
            importing.import(c);
        });
        open_registry::release(&target.to_string_lossy());

        assert_eq!(
            drain_dialog_titles(&mut tree),
            vec![tr!(import_target_open_title()).resolve_now()],
            "the refusal, not the overwrite question"
        );
        assert!(!job.is_running(), "nothing may start");
        assert_eq!(
            std::fs::read(&target).unwrap(),
            b"PK",
            "the project is untouched"
        );
    });
}

// ── what the import reads, and where the new project goes ────────────────────────────────

/// `analysis` as the analysis says it once it read the project at `project`.
fn located(mut analysis: ScrivenerAnalysisDto, project: &Path) -> ScrivenerAnalysisDto {
    analysis.project_path = project.to_string_lossy().into_owned();
    analysis
}

fn text_of(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn refuses(state: ValidationState) -> bool {
    matches!(state, ValidationState::Error(_))
}

/// A folder or a file picked inside the project means the project around it (the wizard
/// answers that question itself): the new project goes beside the project, never into
/// it, and a folder inside it is refused whichever of its files was picked.
#[test]
fn a_pick_inside_the_project_defaults_beside_the_project() {
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let files = project.join("Files");
        let data = files.join("Data").join("0A1B2C3D");
        std::fs::create_dir_all(&data).unwrap();
        for pick in [
            data.clone(),
            files.join("Data"),
            files.join("binder.autosave"),
            files.join("Workflow.scrivx"),
        ] {
            let vm = wizard();
            read(&vm, &pick, located(readable(&project), &project));
            assert_eq!(vm.location().get(), text_of(dir.path()), "{pick:?}");
            assert!(vm.can_import().get(), "{pick:?}");
            for inside in [&project, &files, &files.join("Data")] {
                vm.location().set(text_of(inside));
                assert!(
                    refuses(vm.location_validation().get()),
                    "{pick:?}: {inside:?}"
                );
                assert!(!vm.can_import().get(), "{pick:?}: {inside:?}");
            }
        }
    });
}

/// A folder holding the project is where the writer keeps their projects, not the
/// project: the new project goes into it, beside the project, and it is not refused.
#[test]
fn the_folder_around_the_project_is_not_the_project() {
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let vm = wizard();
        read(&vm, dir.path(), located(readable(&project), &project));
        assert_eq!(
            vm.location().get(),
            text_of(dir.path()),
            "beside the project"
        );
        assert!(!refuses(vm.location_validation().get()));
        assert!(vm.can_import().get());
        vm.location().set(text_of(&project.join("Files")));
        assert!(
            refuses(vm.location_validation().get()),
            "inside it still is"
        );
    });
}

/// A pre-upgrade copy picked, its upgraded project read: the project read is the one a
/// destination may not be inside, not the copy.
#[test]
fn the_upgraded_project_read_is_the_one_protected() {
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let copy = dir.path().join("Lighthouse Backup.scriv");
        let inside = project.join("Files");
        std::fs::create_dir_all(&copy).unwrap();
        std::fs::create_dir_all(&inside).unwrap();
        let vm = wizard();
        read(&vm, &copy, located(readable(&project), &project));
        vm.location().set(text_of(&inside));
        assert!(refuses(vm.location_validation().get()));
        vm.location().set(text_of(&copy));
        assert!(
            !refuses(vm.location_validation().get()),
            "the copy is not the project read"
        );
    });
}

/// The folder check compares the folders themselves, not the way they are spelled: a
/// `..` or a link does not take a destination out of the project.
#[test]
fn a_folder_inside_the_project_is_refused_however_it_is_spelled() {
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let files = project.join("Files");
        let elsewhere = dir.path().join("elsewhere");
        std::fs::create_dir_all(&files).unwrap();
        std::fs::create_dir_all(&elsewhere).unwrap();
        let vm = wizard();
        read(&vm, &project, located(readable(&project), &project));
        vm.location().set(text_of(
            &elsewhere.join("..").join("Lighthouse.scriv").join("Files"),
        ));
        assert!(refuses(vm.location_validation().get()), "through `..`");

        #[cfg(unix)]
        {
            let link = dir.path().join("Shortcut.scriv");
            std::os::unix::fs::symlink(&project, &link).unwrap();
            let vm = wizard();
            read(&vm, &link, located(readable(&project), &link));
            vm.location().set(text_of(&files));
            assert!(refuses(vm.location_validation().get()), "through a link");
            vm.location().set(text_of(&link.join("Files")));
            assert!(refuses(vm.location_validation().get()), "inside the link");
        }
    });
}

/// A backup Scrivener made, wherever its folder is and whatever it is called: the new
/// project does not go into the backup folder by default.
#[test]
fn a_backup_folder_is_never_the_default_whatever_its_name() {
    crate::test_support::with_real_messages(|| {
        let (dir, _project) = project_on_disk();
        let backups = dir.path().join("Scrivener backups");
        std::fs::create_dir_all(&backups).unwrap();
        let zip = backups.join("Lighthouse.bak3.zip");
        std::fs::write(&zip, b"PK").unwrap();
        let mut analysis = located(readable(&zip), &zip);
        analysis.container = ScrivenerContainer::Zip;
        analysis.notices.push(ScrivenerFinding::Reported {
            key: "scrivener-notice-source-is-backup".into(),
            family: ScrivenerFindingFamily::Source,
            group: ScrivenerReportGroup::Unfiled,
            place: ScrivenerPlace::Project,
            item: -1,
            binder_path: Vec::new(),
            item_uid: None,
            args: vec![ScrivenerArg::Token {
                name: "sign".into(),
                value: "bak-archive".into(),
            }],
        });
        let vm = wizard();
        vm.set_remembered_folder(Some(dir.path().to_path_buf()));
        read(&vm, &zip, analysis);
        assert_eq!(
            vm.location().get(),
            text_of(dir.path()),
            "the remembered folder, not the backups"
        );
    });
}

/// A refusal reached through an answer keeps the question that led to it on the page,
/// with its answer, so the writer can choose another.
#[test]
fn a_refusal_after_an_answer_keeps_the_question() {
    let vm = wizard();
    let several = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Choose,
        question: ScrivenerQuestion::SeveralProjects {
            candidates: vec!["One.scriv".into(), "Two.scriv".into()],
        },
        ..ScrivenerAnalysisDto::default()
    };
    read(&vm, Path::new("/books"), several);
    vm.choose(0, 1);
    let refused = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Refused,
        refusal: ScrivenerFinding::Reported {
            key: "scrivener-refusal-scrivener-one-for-mac".into(),
            family: ScrivenerFindingFamily::Refusal,
            group: ScrivenerReportGroup::Unfiled,
            place: ScrivenerPlace::Project,
            item: -1,
            binder_path: Vec::new(),
            item_uid: None,
            args: Vec::new(),
        },
        ..ScrivenerAnalysisDto::default()
    };
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(Rc::new(refused)));
    assert_eq!(vm.inspection_page().get(), PAGE_REFUSED);
    assert_eq!(vm.questions().len(), 1, "the question stays");
    assert_eq!(vm.questions().with_item(0, |q| q.chosen.get()), Some(1));
    vm.inner
        .inspection
        .state()
        .set(InspectState::Failed(Rc::from("an I/O error")));
    assert_eq!(vm.inspection_page().get(), PAGE_FAILED);
    assert_eq!(vm.questions().len(), 1, "and after a reading that failed");
}

/// An acceptance is given for one project: choosing another project does not carry it
/// over, and the request and the record hold only what the project read asks.
#[test]
fn an_acceptance_does_not_carry_over_to_another_project() {
    let key = "scrivener-question-unmerged-mobile-edits";
    let (dir, project) = project_on_disk();
    let vm = wizard();
    let several = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Choose,
        question: ScrivenerQuestion::SeveralProjects {
            candidates: vec!["One.scriv".into(), "Lighthouse.scriv".into()],
        },
        ..ScrivenerAnalysisDto::default()
    };
    let with_edits = || {
        let mut analysis = located(readable(&project), &project);
        analysis.pending = vec![ScrivenerQuestion::UnmergedMobileEdits {
            source: ScrivenerMobileEdits::NewerMobileBinder,
        }];
        Rc::new(analysis)
    };
    read(&vm, dir.path(), several);
    vm.choose(0, 0);
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(with_edits()));
    vm.accept(key, true);
    assert!(vm.can_leave_project_step().get());

    vm.choose(0, 1);
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(with_edits()));
    assert!(
        !vm.can_leave_project_step().get(),
        "the other project's edits were never accepted"
    );
    let ticked = vm
        .questions()
        .with_item(1, |q| (q.key.clone(), q.accepted.get()));
    assert_eq!(ticked, Some((key.to_string(), false)));

    vm.accept(key, true);
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(Rc::new(located(
            readable(&project),
            &project,
        ))));
    let request = vm.request();
    assert!(
        request.is_some_and(|r| !r.accept_unmerged_mobile_edits),
        "a project that no longer asks is not answered"
    );
}

/// The title, author and language the writer edited survive the project being read
/// again, as the name and the folder do.
#[test]
fn the_writers_title_author_and_language_survive_a_new_reading() {
    let (_dir, project) = project_on_disk();
    let vm = wizard();
    read(&vm, &project, readable(&project));
    vm.title().set("The Keeper".into());
    vm.author().set("A. Marlowe".into());
    vm.language().set(Some("fr-FR".into()));
    vm.inner
        .inspection
        .state()
        .set(InspectState::Reading { percent: 0.0 });
    read(&vm, &project, readable(&project));
    assert_eq!(vm.title().get(), "The Keeper");
    assert_eq!(vm.author().get(), "A. Marlowe");
    assert_eq!(vm.language().get().as_deref(), Some("fr-FR"));
    assert!(
        vm.languages().with_item(0, |t| t.clone()).is_some(),
        "the list still offers the languages"
    );
}

/// A path whose name is not Unicode is said to be one, not missing: the writer just
/// picked it.
#[cfg(unix)]
#[test]
fn a_name_that_is_not_unicode_is_not_called_missing() {
    use std::os::unix::ffi::OsStrExt;
    crate::test_support::with_real_messages(|| {
        let (dir, _project) = project_on_disk();
        let odd = dir
            .path()
            .join(std::ffi::OsStr::from_bytes(b"Roman d'\xe9t\xe9.scriv"));
        if std::fs::create_dir(&odd).is_err() {
            // A filesystem that only takes Unicode names cannot hold one.
            return;
        }
        let vm = wizard();
        vm.accept_dropped(std::slice::from_ref(&odd));
        let said = match vm.source_validation().get() {
            ValidationState::Error(e) => e.resolve_now(),
            _ => String::new(),
        };
        assert_eq!(
            said,
            "This name cannot be read. Rename the project in your file manager, then choose it again."
        );
        assert_eq!(vm.inspection_page().get(), PAGE_EMPTY, "nothing is read");
    });
}

/// The path field looks at the disk when the path is submitted, never on a keystroke:
/// a path on a network share that does not answer would otherwise stop the window on
/// every key. Nor is a refused path looked at again behind the writer's back.
#[test]
fn the_path_is_looked_at_when_submitted_not_while_typed() {
    crate::test_support::with_real_messages(|| {
        let vm = wizard();
        vm.source_text().set("/no/such/project.scriv".into());
        assert!(
            matches!(vm.source_validation().get(), ValidationState::None),
            "not while typed"
        );
        vm.submit_source();
        assert!(refuses(vm.source_validation().get()), "when submitted");
        assert!(!vm.refused_on_disk(), "and not looked at again unasked");
        vm.source_text().set("/no/such/project.scri".into());
        assert!(
            matches!(vm.source_validation().get(), ValidationState::None),
            "an edit clears what was said about the path before"
        );
    });
}

/// Two wizards import into two files, the first waiting on its overwrite question while
/// the second runs: each result keeps the record of the wizard that started it.
#[test]
fn each_import_keeps_the_record_of_its_own_wizard() {
    use crate::test_support::{IsolatedOpenRegistry, press};
    let _registry = IsolatedOpenRegistry::new();
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    let mut targets = Vec::new();
    let mut dirs = Vec::new();
    for blank_lines in [true, false] {
        let (dir, project) = project_on_disk();
        let vm = ImportScrivenerViewModel::new(app_ctx.clone(), job.clone());
        read(&vm, &project, readable(&project));
        vm.toggle(ToggleTarget::BlankLines, blank_lines);
        let target = dir.path().join("The Lighthouse Keeper.skrib");
        // An existing file: the job asks before replacing it, and nothing starts.
        std::fs::write(&target, b"PK").unwrap();
        let importing = vm.clone();
        press(&mut tree, move |c| {
            importing.import(c);
        });
        targets.push((text_of(&target), blank_lines));
        dirs.push(dir);
    }
    assert!(!job.is_running());
    for (target, blank_lines) in targets.into_iter().rev() {
        let outcome = record_outcome(ImportScrivenerProjectResultDto {
            output_path: target.clone(),
            ..ImportScrivenerProjectResultDto::default()
        });
        assert_eq!(outcome.output_path, target);
        let record = last_import().and_then(|last| last.record.clone());
        assert_eq!(
            record.map(|r| r.blank_lines_as_scene_breaks),
            Some(blank_lines),
            "{target}"
        );
    }
}

/// **Review the import again** after an import found the project changed reads it again with what
/// the writer answered, accepted and chose, and keeps the Save page as they left it.
#[test]
fn review_again_keeps_what_the_writer_chose() {
    use crate::test_support::{IsolatedOpenRegistry, press};
    let _registry = IsolatedOpenRegistry::new();
    let (dir, project) = project_on_disk();
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let vm = ImportScrivenerViewModel::new(app_ctx.clone(), job);
    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    let several = ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Choose,
        question: ScrivenerQuestion::SeveralProjects {
            candidates: vec!["One.scriv".into(), "Lighthouse.scriv".into()],
        },
        ..ScrivenerAnalysisDto::default()
    };
    read(&vm, dir.path(), several.clone());
    vm.choose(0, 1);
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(Rc::new(located(
            readable(&project),
            &project,
        ))));
    vm.toggle(ToggleTarget::BlankLines, false);
    vm.toggle(ToggleTarget::Ornaments, true);
    vm.copy_research_files().set(false);
    vm.title().set("The Keeper".into());
    vm.name().set("Keeper".into());
    let target = dir.path().join("Keeper.skrib");
    std::fs::write(&target, b"PK").unwrap();
    let importing = vm.clone();
    press(&mut tree, move |c| {
        importing.import(c);
    });

    let reviewing = vm.clone();
    let source = text_of(dir.path());
    press(&mut tree, move |c| {
        crate::import_scrivener::panel::present_import_scrivener(
            c,
            reviewing.clone(),
            Some(source.clone()),
        );
    });
    // The project as it is now: a question the answer given still answers, then the
    // project.
    vm.inner
        .inspection
        .state()
        .set(InspectState::Read(Rc::new(located(
            readable(&project),
            &project,
        ))));
    let Some(request) = vm.request() else {
        panic!("the project reads again");
    };
    assert_eq!(request.chosen_project, "Lighthouse.scriv");
    assert!(!request.blank_lines_as_scene_breaks && request.ornaments_as_scene_breaks);
    assert!(!request.copy_research_files);
    assert_eq!(request.title.as_deref(), Some("The Keeper"));
    assert_eq!(request.output_path, text_of(&target));
}

/// A choice that keeps the plan's controls writes into the rows on screen instead of
/// rebuilding them, and the row shows the answer chosen.
#[cfg(feature = "mocks")]
#[test]
fn a_choice_updates_the_rows_in_place() {
    let vm = wizard();
    let (_dir, project) = project_on_disk();
    vm.source_text().set(text_of(&project));
    vm.submit_source();
    let rebuilt = Rc::new(Cell::new(0));
    let seen = rebuilt.clone();
    let _watch = vm.review_rows().observe_changes(move |change| {
        if matches!(change, teksilo::data::DataChange::Reset) {
            seen.set(seen.get() + 1);
        }
    });
    vm.pick(PickTarget::Root(24), 3);
    assert_eq!(rebuilt.get(), 0, "the rows were rebuilt");
    let rows = vm.review_rows();
    let selected = (0..rows.len()).find_map(|i| {
        rows.with_item(i, |row| match &row.control {
            ReviewControl::Pick {
                target: PickTarget::Root(24),
                selected,
                ..
            } => Some(selected.get()),
            _ => None,
        })
        .flatten()
    });
    assert_eq!(selected, Some(Some(3)));
}

// ── the report ───────────────────────────────────────────────────────────────────────────

/// A finished import of `output_path`, with one finding, as the converter sends it.
fn finished(output_path: &str) -> ImportScrivenerProjectResultDto {
    ImportScrivenerProjectResultDto {
        output_path: output_path.into(),
        findings: vec![finding(
            "scrivener-structure-own-text-moved",
            ScrivenerReportGroup::BroughtAcrossDifferently,
            Vec::new(),
        )],
        ..ImportScrivenerProjectResultDto::default()
    }
}

/// The result offers **See report** when the report has a line to read, archived under a
/// name that says which import's report it opens; with nothing to say, there is no button, and the
/// command palette still reopens the counts.
#[test]
fn the_result_offers_its_report_when_it_has_something_to_say() {
    crate::test_support::with_real_messages(|| {
        let outcome = record_outcome(finished("/books/A.skrib"));
        let see = outcome.see_report.expect("See report");
        assert_eq!((see.label)().resolve_now(), "See report");
        let archived = (see.archived_as)("/books/A.skrib");
        assert_eq!(archived, "work.import_scrivener_report_of:/books/A.skrib");
        assert_eq!(archived_report_path(&archived), Some("/books/A.skrib"));
        assert_eq!(archived_report_path(REPORT_ACTION), None);
        assert!(matches!(
            (see.intent)("/books/A.skrib".into()),
            AppIntent::ImportScrivenerReportOf { output_path } if output_path == "/books/A.skrib"
        ));
        let quiet = record_outcome(ImportScrivenerProjectResultDto {
            output_path: "/books/B.skrib".into(),
            ..ImportScrivenerProjectResultDto::default()
        });
        assert!(quiet.see_report.is_none());
    });
}

/// The palette's entry reopens this window's own report first, the newer of the one it
/// showed and the one its wizard started; else the one about the project the window
/// shows; else the session's last.
#[test]
fn a_window_reopens_its_own_report_first() {
    let vm = wizard();
    assert!(vm.report_for_window(None).is_none(), "nothing finished yet");

    record_outcome(finished("/books/A.skrib"));
    record_outcome(finished("/books/B.skrib"));
    let path = |r: Option<Rc<ScrivenerImportResult>>| r.map(|r| r.output_path.clone());
    assert_eq!(
        path(vm.report_for_window(None)).as_deref(),
        Some("/books/B.skrib")
    );
    assert_eq!(
        path(vm.report_for_window(Some("/books/A.skrib"))).as_deref(),
        Some("/books/A.skrib"),
        "the report about the project this window shows"
    );

    vm.showed_report(import_of("/books/A.skrib").expect("kept"));
    assert_eq!(
        path(vm.report_for_window(Some("/books/B.skrib"))).as_deref(),
        Some("/books/A.skrib"),
        "the one this window showed"
    );

    // This window's wizard starts an import, which finishes after the one it showed.
    *vm.inner.started.borrow_mut() = Some("/books/C.skrib".into());
    assert_eq!(
        path(vm.report_for_window(None)).as_deref(),
        Some("/books/A.skrib"),
        "not finished yet"
    );
    record_outcome(finished("/books/C.skrib"));
    assert_eq!(
        path(vm.report_for_window(None)).as_deref(),
        Some("/books/C.skrib")
    );

    // Showing an older one again makes it the newest of this window's.
    vm.showed_report(import_of("/books/B.skrib").expect("kept"));
    assert_eq!(
        path(vm.report_for_window(None)).as_deref(),
        Some("/books/C.skrib"),
        "the import it started finished after the one it showed"
    );
}

/// The session keeps the last sixteen finished imports, and finds one by the project it
/// wrote whatever the spelling.
#[test]
fn the_session_keeps_its_recent_reports() {
    for i in 0..20 {
        record_outcome(finished(&format!("/books/{i}.skrib")));
    }
    assert!(
        import_of("/books/3.skrib").is_none(),
        "let go of past sixteen"
    );
    assert!(import_of("/books/4.skrib").is_some());
    assert!(import_of("/books/./19.skrib").is_some());
    assert_eq!(
        last_import().map(|r| r.output_path.clone()).as_deref(),
        Some("/books/19.skrib")
    );
}

/// The record keeps the writer's choices as the Review page worded them, and the prose
/// ones always.
#[test]
fn the_record_words_the_writers_choices() {
    use crate::test_support::{IsolatedOpenRegistry, press};
    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let app_ctx = Rc::new(AppContext::new());
        let vm =
            ImportScrivenerViewModel::new(app_ctx.clone(), ProjectImportJob::new(app_ctx.clone()));
        read(&vm, &project, readable(&project));
        vm.toggle(ToggleTarget::Ornaments, true);
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let importing = vm.clone();
        press(&mut tree, move |c| {
            importing.import(c);
        });
        let target = text_of(&dir.path().join("The Lighthouse Keeper.skrib"));
        let record = PENDING_RECORDS
            .with(|pending| pending.borrow().get(&target).cloned())
            .expect("the import was handed to the job");
        let worded: Vec<String> = record
            .worded
            .iter()
            .map(|line| {
                format!(
                    "{} = {}",
                    line.subject.resolve_now(),
                    line.answer.resolve_now()
                )
            })
            .collect();
        assert!(
            worded.contains(&"Lines of ornaments become scene breaks = yes".to_string()),
            "{worded:?}"
        );
        assert!(
            worded.contains(&"Blank lines become scene breaks = yes".to_string()),
            "{worded:?}"
        );
    });
}

/// The wizard's **Help** opens "Coming from Scrivener", a topic the help window lists.
#[test]
fn help_opens_the_scrivener_topic() {
    let topic = crate::help::topic(HELP_TOPIC).expect("the topic is registered");
    assert_eq!(topic.key, "help-import-scrivener");
}

/// The headline counts the report's lines in its first two groups, so the notice and the
/// dialog give the writer one number: three missing texts are three lines, and comments
/// placed by two readings are two.
#[test]
fn the_headline_counts_what_the_report_lists() {
    let missing = |item: i64| ScrivenerFinding::Reported {
        key: "scrivener-notice-content-missing".into(),
        family: ScrivenerFindingFamily::Source,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Text,
        item,
        binder_path: vec!["Draft".into(), format!("Scene {item}")],
        item_uid: None,
        args: Vec::new(),
    };
    let reading = |item: i64, how: &str| ScrivenerFinding::Reported {
        key: "scrivener-prose-range-comment-reading".into(),
        family: ScrivenerFindingFamily::Prose,
        group: ScrivenerReportGroup::CheckThese,
        place: ScrivenerPlace::Text,
        item,
        binder_path: vec!["Draft".into(), format!("Scene {item}")],
        item_uid: None,
        args: vec![
            ScrivenerArg::Token {
                name: "reading".into(),
                value: how.into(),
            },
            ScrivenerArg::Count {
                name: "count".into(),
                value: 1,
            },
        ],
    };
    let result = ImportScrivenerProjectResultDto {
        output_path: "/out/Novel.skrib".into(),
        findings: vec![
            missing(1),
            missing(2),
            missing(3),
            reading(4, "markers-counted"),
            reading(5, "markers-removed"),
        ],
        ..ImportScrivenerProjectResultDto::default()
    };
    crate::test_support::with_real_messages(|| {
        let outcome = record_outcome(result);
        let imported = last_import().expect("kept");
        let report = ImportReport::from_result(&imported);
        let listed: usize = report
            .groups
            .iter()
            .filter(|g| g.group.counts_in_headline())
            .map(|g| g.lines.len())
            .sum();
        assert_eq!(listed, 5);
        assert_eq!(imported.findings_to_check(), listed);
        assert!(
            outcome
                .result
                .resolve_now()
                .ends_with("5 things need a look."),
            "{}",
            outcome.result.resolve_now()
        );
    });
}

/// A result notice that offers **See report** stays until the writer closes it, even
/// when nothing needs a look: in the Launcher it is the only way back to the report.
#[test]
fn a_notice_offering_its_report_stays() {
    crate::test_support::with_real_messages(|| {
        let outcome = record_outcome(finished("/books/A.skrib"));
        assert!(outcome.see_report.is_some());
        assert!(outcome.persistent, "a timer would take See report away");
    });
}

/// A report asked for by an import older than the ones the session keeps is said to be
/// no longer kept, not that no import finished.
#[test]
fn a_report_let_go_says_so() {
    crate::test_support::with_real_messages(|| {
        let why = |path: &str| match report_of(path) {
            Ok(_) => String::new(),
            Err(why) => why.resolve_now(),
        };
        assert!(why("/books/0.skrib").starts_with("No Scrivener import has finished"));
        for i in 0..=KEPT_REPORTS {
            record_outcome(finished(&format!("/books/{i}.skrib")));
        }
        let gone = why("/books/0.skrib");
        assert!(
            gone.starts_with("The report of this import is no longer kept.")
                && gone.contains(&KEPT_REPORTS.to_string()),
            "{gone}"
        );
        assert!(report_of(&format!("/books/{KEPT_REPORTS}.skrib")).is_ok());
    });
}

/// The record names the project the analysis read, not the folder picked around it, and
/// keeps the Project page's answers and what the writer did with the Save page's
/// proposals.
#[test]
fn the_record_names_the_project_read_and_the_writers_answers() {
    use crate::test_support::{IsolatedOpenRegistry, press};
    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let (dir, project) = project_on_disk();
        let app_ctx = Rc::new(AppContext::new());
        let vm =
            ImportScrivenerViewModel::new(app_ctx.clone(), ProjectImportJob::new(app_ctx.clone()));
        let analysis = ScrivenerAnalysisDto {
            project_path: text_of(&project),
            ..readable(&project)
        };
        // The writer picked the folder around the project.
        read(&vm, dir.path(), analysis);
        vm.inner
            .asked
            .borrow_mut()
            .push(ScrivenerQuestion::UpgradedSibling {
                backup: "Lighthouse Backup.scriv".into(),
                sibling: "Lighthouse.scriv".into(),
                sibling_generation: None,
                preselect_sibling: true,
            });
        vm.inner.answers.borrow_mut().upgraded_sibling = ScrivenerSiblingChoice::Sibling;
        vm.inner.title.set("A Title of My Own".into());
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let importing = vm.clone();
        press(&mut tree, move |c| {
            importing.import(c);
        });
        let target = text_of(&dir.path().join("The Lighthouse Keeper.skrib"));
        let record = PENDING_RECORDS
            .with(|pending| pending.borrow().get(&target).cloned())
            .expect("the import was handed to the job");
        assert_eq!(record.source, text_of(&project));
        let answers: Vec<String> = record
            .project_answers
            .iter()
            .map(|a| a.resolve_now())
            .collect();
        assert_eq!(answers.len(), 1, "{answers:?}");
        assert!(answers[0].starts_with("You imported “Lighthouse.scriv”"));
        assert_eq!(record.title, "A Title of My Own");
        assert!(record.title_changed);
        assert!(!record.author_changed);
        assert_eq!(record.author, "Ada Marlowe, Ben Hart");
    });
}

/// The notification log keeps one row for every Scrivener import, which shows the latest
/// one, and its **See report** opens that import's report: never the one the window it is
/// pressed in showed last, which an older import's broadcast notice can have made another.
#[test]
fn the_logs_see_report_opens_the_import_its_row_describes() {
    use crate::shared::import_warnings::replay_archived_action;
    use crate::test_support::press;
    use teksilo::core::action::Action;
    use teksilo::core::styles::BannerSeverity;
    use teksilo::widgets::{
        ArchivedAction, ArchivedActionStyle, NotificationEntry, ToastPriority, ToastRoute,
    };

    /// A project window answering the report's two actions as `App` does.
    struct Window(ImportScrivenerViewModel);
    impl std::fmt::Debug for Window {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Window").finish()
        }
    }
    impl Widget for Window {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let import = self.0.clone();
            ctx.register_action_global(
                Action::new(REPORT_ACTION)
                    .on_invoke(move |_i, c| import.show_window_report(c, None, None)),
            );
            let import = self.0.clone();
            ctx.register_action_global(Action::new("work.import_scrivener_report_of").on_invoke(
                move |i, c| {
                    if let Some(AppIntent::ImportScrivenerReportOf { output_path }) =
                        AppIntent::from_intent(i)
                    {
                        import.show_report_of(c, output_path, None, None);
                    }
                },
            ));
            Vec::new()
        }

        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    crate::test_support::with_real_messages(|| {
        let app_ctx = Rc::new(AppContext::new());
        let window = ImportScrivenerViewModel::new(app_ctx.clone(), ProjectImportJob::new(app_ctx));
        record_outcome(finished("/books/A.skrib"));
        // The first import's notice reached this window, which showed its report.
        window.showed_report(import_of("/books/A.skrib").expect("kept"));
        // A second import, started elsewhere, finishes: the log's one row now shows it.
        let newer = record_outcome(finished("/books/B.skrib"));
        let see = newer.see_report.expect("See report");
        let entry = NotificationEntry {
            id: 1,
            severity: BannerSeverity::Success,
            priority: ToastPriority::High,
            title: "Scrivener project imported".into(),
            body: None,
            actions: Vec::new(),
            timestamp: jiff::Timestamp::UNIX_EPOCH,
            group: None,
            source: None,
            read: false,
            dedup_id: Some(SCRIVENER.toast_id.into()),
            updates: Vec::new(),
            route: ToastRoute::Broadcast,
        };
        let action = ArchivedAction {
            label: "See report".into(),
            intent_name: Some((see.archived_as)(&newer.output_path)),
            style: ArchivedActionStyle::SecondaryButton,
            closes_on_invoke: false,
        };

        let mut tree = crate::test_support::tree_with_events(&Rc::new(AppContext::new()));
        tree.add(Window(window.clone()));
        tree.layout(SizeProposal::exact(480.0, 360.0));
        press(&mut tree, move |c| {
            replay_archived_action(&entry, &action, c)
        });
        let shown = window.inner.report.borrow().clone();
        assert_eq!(
            shown.map(|r| r.output_path.clone()).as_deref(),
            Some("/books/B.skrib"),
            "the row's See report opened the report of the import the row describes"
        );
    });
}

/// The import and the review ask on the writer's own calendar day, as every pace plan
/// counts its days: a writer a day ahead of UTC is never on UTC's date.
#[test]
fn the_import_asks_on_the_writers_own_day() {
    use crate::shared::stamps::{Zone, override_writer_zone};
    let _ahead = override_writer_zone(Zone::a_day_ahead());
    let today = writer_today();
    assert_eq!(today.date_naive(), crate::shared::stamps::today());
    assert_ne!(today.date_naive(), chrono::Utc::now().date_naive());
    assert_eq!(today.time(), chrono::NaiveTime::MIN, "kept at midnight UTC");
}

/// The mocks build walks an import to its report on the fabricated run.
#[cfg(feature = "mocks")]
mod mock_import;
