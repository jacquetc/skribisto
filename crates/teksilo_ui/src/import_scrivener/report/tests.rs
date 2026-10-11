// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::import_management::{
    ScrivenerArg, ScrivenerFindingFamily, ScrivenerPlace, ScrivenerReportGroup,
};

use crate::import_scrivener::ChoiceLine;

fn count(name: &str, value: i64) -> ScrivenerArg {
    ScrivenerArg::Count {
        name: name.into(),
        value,
    }
}

/// A finding of `key`, filed under `group`, about the item at binder row `item`.
fn found(
    key: &str,
    family: ScrivenerFindingFamily,
    group: ScrivenerReportGroup,
    item: i64,
    path: &[&str],
    args: Vec<ScrivenerArg>,
) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family,
        group,
        place: if item < 0 {
            ScrivenerPlace::Project
        } else {
            ScrivenerPlace::Text
        },
        item,
        binder_path: path.iter().map(|p| p.to_string()).collect(),
        item_uid: (item >= 0).then(|| Uuid::from_u128(item as u128 + 1)),
        args,
    }
}

fn annotations(item: i64, title: &str, n: i64) -> ScrivenerFinding {
    found(
        "scrivener-prose-annotations-became-comments",
        ScrivenerFindingFamily::Prose,
        ScrivenerReportGroup::BroughtAcrossDifferently,
        item,
        &["Draft", title],
        vec![count("count", n)],
    )
}

/// What a wizard of this session recorded, the writer having changed nothing.
fn record() -> ImportRecord {
    ImportRecord {
        source: "/books/The Lighthouse.scriv".into(),
        project_answers: Vec::new(),
        accepted: Vec::new(),
        choices: Vec::new(),
        worded: Vec::new(),
        blank_lines_as_scene_breaks: true,
        ornaments_as_scene_breaks: false,
        copy_research_files: true,
        research_files: true,
        title: "The Lighthouse".into(),
        author: String::new(),
        language: "en-US".into(),
        title_changed: false,
        author_changed: false,
        language_changed: false,
    }
}

/// A research file the import copied, about binder row `item`.
fn copied(item: i64, name: &str) -> ScrivenerFinding {
    found(
        "scrivener-prose-media-file-copied",
        ScrivenerFindingFamily::Prose,
        ScrivenerReportGroup::BroughtAcrossDifferently,
        item,
        &["Research", name],
        vec![ScrivenerArg::Text {
            name: "path".into(),
            value: format!("The Lighthouse research files/{name}"),
        }],
    )
}

/// A research file the import left in the Scrivener project.
fn stayed(item: i64, name: &str) -> ScrivenerFinding {
    found(
        "scrivener-prose-media-file-not-carried",
        ScrivenerFindingFamily::Prose,
        ScrivenerReportGroup::LeftBehind,
        item,
        &["Research", name],
        vec![ScrivenerArg::Text {
            name: "name".into(),
            value: name.into(),
        }],
    )
}

fn result(findings: Vec<ScrivenerFinding>, record: Option<ImportRecord>) -> ScrivenerImportResult {
    ScrivenerImportResult {
        output_path: "/books/The Lighthouse.skrib".into(),
        research_folder: String::new(),
        totals: ScrivenerTotals::Counted {
            rows: 12,
            words: 3400,
            footnotes: 0,
            comments: 5,
            pictures: 0,
            templates: 0,
            tags: 2,
            rungs: 0,
            trashed: 0,
            research_files: 0,
            synopses: 4,
            document_notes: 0,
        },
        findings,
        record,
    }
}

/// One of each group but *Brought across differently*, given out of order.
fn mixed() -> Vec<ScrivenerFinding> {
    vec![
        found(
            "scrivener-prose-media-file-not-carried",
            ScrivenerFindingFamily::Prose,
            ScrivenerReportGroup::LeftBehind,
            7,
            &["Research", "Map.pdf"],
            vec![ScrivenerArg::Text {
                name: "name".into(),
                value: "Map.pdf".into(),
            }],
        ),
        found(
            "scrivener-prose-heading-repeats-title",
            ScrivenerFindingFamily::Prose,
            ScrivenerReportGroup::CheckThese,
            3,
            &["Draft", "Arrival"],
            Vec::new(),
        ),
        found(
            "scrivener-prose-unreadable",
            ScrivenerFindingFamily::Prose,
            ScrivenerReportGroup::CouldNotBeRead,
            2,
            &["Draft", "Storm"],
            Vec::new(),
        ),
    ]
}

/// The groups come in their fixed order, an empty one is left out, and the two that need
/// the writer open expanded.
#[test]
fn the_groups_come_in_their_order_and_an_empty_one_is_left_out() {
    let report = ImportReport::from_result(&result(mixed(), None));
    let groups: Vec<Group> = report.groups.iter().map(|g| g.group).collect();
    assert_eq!(
        groups,
        vec![Group::CouldNotBeRead, Group::CheckThese, Group::LeftBehind]
    );
    let expanded: Vec<bool> = report
        .groups
        .iter()
        .map(ReportGroup::expanded_by_default)
        .collect();
    assert_eq!(expanded, vec![true, true, false]);
    assert!(report.has_findings());
    assert!(!ImportReport::from_result(&result(Vec::new(), None)).has_findings());
}

/// A kind found in several texts is one line, counting them, with its items listed once
/// each, in Scrivener's binder order.
#[test]
fn a_kind_is_one_line_with_its_items_in_binder_order() {
    crate::test_support::with_real_messages(|| {
        let report = ImportReport::from_result(&result(
            vec![
                annotations(9, "Storm", 2),
                annotations(4, "Arrival", 1),
                annotations(9, "Storm", 1),
            ],
            None,
        ));
        assert_eq!(report.groups.len(), 1);
        let lines = &report.groups[0].lines;
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0].text.resolve_now(),
            "4 inline annotations became comments."
        );
        let paths: Vec<String> = lines[0]
            .items
            .iter()
            .map(|i| i.label().resolve_now())
            .collect();
        assert_eq!(paths, vec!["Draft ▸ Arrival", "Draft ▸ Storm"]);
        assert_eq!(lines[0].items[0].uid, Some(Uuid::from_u128(5)));
    });
}

/// A notice the Project page showed about one item keeps a line of its own, which names
/// it; a key nobody words still says itself, among what to check.
#[test]
fn a_notice_keeps_its_own_line_and_an_unknown_key_says_itself() {
    crate::test_support::with_real_messages(|| {
        let missing = |item: i64, title: &str| {
            found(
                "scrivener-notice-content-missing",
                ScrivenerFindingFamily::Source,
                ScrivenerReportGroup::Unfiled,
                item,
                &["Draft", title],
                Vec::new(),
            )
        };
        let report = ImportReport::from_result(&result(
            vec![
                missing(2, "Storm"),
                missing(5, "Harbour"),
                found(
                    "scrivener-structure-made-up",
                    ScrivenerFindingFamily::Structure,
                    ScrivenerReportGroup::Unfiled,
                    6,
                    &["Draft", "Odd"],
                    Vec::new(),
                ),
            ],
            None,
        ));
        let unread = &report.groups[0];
        assert_eq!(unread.group, Group::CouldNotBeRead);
        let lines: Vec<String> = unread.lines.iter().map(|l| l.text.resolve_now()).collect();
        assert_eq!(lines.len(), 2, "{lines:?}");
        assert!(
            lines[0].contains("Storm") && lines[1].contains("Harbour"),
            "{lines:?}"
        );
        let check = &report.groups[1];
        assert_eq!(check.group, Group::CheckThese);
        assert_eq!(
            check.lines[0].text.resolve_now(),
            "scrivener-structure-made-up: Draft ▸ Odd"
        );
    });
}

/// What came across leaves out what the import did not count.
#[test]
fn what_came_across_leaves_out_the_zeros() {
    crate::test_support::with_real_messages(|| {
        let report = ImportReport::from_result(&result(Vec::new(), None));
        assert_eq!(
            report.came_across_line().resolve_now(),
            "12 items, 3,400 words, 4 synopses, 5 comments, 2 tags"
        );
        let empty = ImportReport::from_result(&ScrivenerImportResult {
            totals: ScrivenerTotals::Empty,
            ..result(Vec::new(), None)
        });
        assert_eq!(
            empty.came_across_line().resolve_now(),
            "Nothing was counted."
        );
    });
}

/// The writer's own choices are in the report: what they accepted, each choice of the
/// Review page, and what became of the research files (D7 critique 4).
#[test]
fn the_writers_choices_are_in_the_report() {
    crate::test_support::with_real_messages(|| {
        let record = ImportRecord {
            accepted: vec![UNMERGED_MOBILE_EDITS.to_string()],
            worded: vec![ChoiceLine {
                subject: lit!("Ideas"),
                answer: tr!(import_scrivener_dest_leave_out()),
            }],
            ..record()
        };
        let mut imported = result(vec![copied(7, "Map.pdf")], Some(record));
        imported.research_folder = "The Lighthouse research files".into();
        let report = ImportReport::from_result(&imported);
        let choices: Vec<String> = report.choices.iter().map(|c| c.resolve_now()).collect();
        assert_eq!(
            choices,
            vec![
                "You imported the project without the changes made in Scrivener for iPhone \
                 or iPad that were not in it yet."
                    .to_string(),
                "Ideas: Leave out".to_string(),
                "Title: The Lighthouse".to_string(),
                "Language: en-US".to_string(),
                "Research files Skribisto cannot open were copied into “The Lighthouse \
                 research files”, beside the new project."
                    .to_string(),
            ]
        );
        assert_eq!(
            report.source.as_deref(),
            Some("/books/The Lighthouse.scriv")
        );
        let text = report.text();
        assert!(text.contains("What you chose\n- You imported"), "{text}");
        assert!(
            text.contains("Read from /books/The Lighthouse.scriv"),
            "{text}"
        );
    });
}

/// The dialog lists fifty items of a line and says how many more; the copied report lists
/// them all.
#[test]
fn a_long_line_shows_fifty_items_and_copies_all() {
    crate::test_support::with_real_messages(|| {
        let findings: Vec<ScrivenerFinding> = (0..60)
            .map(|i| annotations(i, &format!("Scene {i:02}"), 1))
            .collect();
        let report = ImportReport::from_result(&result(findings, None));
        let line = &report.groups[0].lines[0];
        assert_eq!(line.items.len(), 60);
        assert_eq!(line.shown_items().len(), SHOWN_ITEMS);
        assert_eq!(line.more(), 10);
        let text = report.text();
        assert!(text.contains("    Draft ▸ Scene 59\n"), "{text}");
    });
}

/// The copied report is in the language Skribisto shows itself in when it is copied.
#[test]
fn the_copied_report_is_in_the_interface_language() {
    let report = ImportReport::from_result(&result(mixed(), None));
    let english = {
        let mut text = String::new();
        crate::test_support::with_shipped_messages("en-US", || text = report.text());
        text
    };
    let french = {
        let mut text = String::new();
        crate::test_support::with_shipped_messages("fr-FR", || text = report.text());
        text
    };
    for heading in [
        "About this import",
        "Could not be read",
        "Check these",
        "Left behind",
    ] {
        assert!(english.contains(heading), "{english}");
        assert!(!french.contains(heading), "{french}");
    }
    for heading in [
        "À propos de cet import",
        "Illisible",
        "À vérifier",
        "Laissé de côté",
    ] {
        assert!(french.contains(heading), "{french}");
    }
    assert!(french.contains("    Draft ▸ Storm\n"), "{french}");
}

/// A metadata finding about the project, as the converter sends it.
fn metadata(key: &str, args: Vec<ScrivenerArg>) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Metadata,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args,
    }
}

fn field(value: &str) -> ScrivenerArg {
    ScrivenerArg::Token {
        name: "field".into(),
        value: value.into(),
    }
}

fn text(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Text {
        name: name.into(),
        value: value.into(),
    }
}

/// Every line of `report`, as read now.
fn all_lines(report: &ImportReport) -> Vec<String> {
    report
        .groups
        .iter()
        .flat_map(|g| g.lines.iter())
        .map(|l| l.text.resolve_now())
        .collect()
}

/// The Label and the Status field each say what became of them: a finding about one
/// field never stands for the other's, and a value is never another value's line.
#[test]
fn metadata_findings_about_each_field_each_say_themselves() {
    crate::test_support::with_real_messages(|| {
        let report = ImportReport::from_result(&result(
            vec![
                metadata(
                    "scrivener-metadata-field-renamed",
                    vec![field("label"), text("title", "Time")],
                ),
                metadata(
                    "scrivener-metadata-field-renamed",
                    vec![field("status"), text("title", "Rating")],
                ),
                metadata(
                    "scrivener-metadata-unnamed-value",
                    vec![field("label"), count("id", 5)],
                ),
                metadata(
                    "scrivener-metadata-unnamed-value",
                    vec![field("label"), count("id", 7)],
                ),
                metadata(
                    "scrivener-metadata-unnamed-value",
                    vec![field("status"), count("id", 2)],
                ),
                metadata(
                    "scrivener-metadata-none-value-collapsed",
                    vec![
                        field("label"),
                        text("name", " no label "),
                        count("items", 2),
                    ],
                ),
                metadata(
                    "scrivener-metadata-none-value-collapsed",
                    vec![field("status"), text("name", "N/A"), count("items", 4)],
                ),
                metadata(
                    "scrivener-metadata-values-without-row",
                    vec![field("label"), count("items", 3)],
                ),
                metadata(
                    "scrivener-metadata-values-without-row",
                    vec![field("status"), count("items", 5)],
                ),
            ],
            None,
        ));
        let lines = all_lines(&report);
        for said in [
            "The Label field was called “Time”",
            "The Status field was called “Rating”",
            "A label with no name is named after its number, 5.",
            "A label with no name is named after its number, 7.",
            "A status with no name is named after its number, 2.",
            "The label “ no label ”",
            "The status “N/A”",
            "on 4 items",
            "The labels of 3 items",
            "The statuses of 5 items",
        ] {
            assert!(
                lines.iter().any(|l| l.contains(said)),
                "{said:?} in {lines:#?}"
            );
        }
        assert_eq!(lines.len(), 9, "{lines:#?}");
    });
}

/// A line about the project lists the names it is about under it, as the project wrote
/// them, and the copied report lists every one: the keywords no document
/// wears, the icons dropped. A line that names nothing lists nothing.
#[test]
fn a_metadata_line_lists_the_names_it_is_about() {
    crate::test_support::with_real_messages(|| {
        let names = |values: &[&str]| ScrivenerArg::Texts {
            name: "names".into(),
            values: values.iter().map(|v| v.to_string()).collect(),
        };
        let report = ImportReport::from_result(&result(
            vec![
                metadata(
                    "scrivener-metadata-unused-keywords-skipped",
                    vec![names(&["Unused", "Places"])],
                ),
                metadata(
                    "scrivener-metadata-icons-dropped",
                    vec![count("rows", 3), names(&["Book (Blue Book)"])],
                ),
                metadata("scrivener-metadata-goals-kept", vec![count("rows", 2)]),
            ],
            None,
        ));
        let lines: Vec<&ReportLine> = report.groups.iter().flat_map(|g| &g.lines).collect();
        let named = |prefix: &str| {
            lines
                .iter()
                .find(|l| l.text.resolve_now().starts_with(prefix))
                .unwrap_or_else(|| panic!("no line starts {prefix:?}"))
        };
        let unused = named("2 keywords no document wears");
        assert_eq!(unused.listed, Listed::Names);
        let shown: Vec<String> = unused
            .items
            .iter()
            .map(|i| i.label().resolve_now())
            .collect();
        assert_eq!(shown, ["Unused", "Places"]);
        assert!(unused.items.iter().all(|i| i.uid.is_none()));
        assert_eq!(named("3 items had binder icons").items.len(), 1);
        let goals = named("The targets Scrivener set on 2 documents become");
        assert_eq!(goals.listed, Listed::Items);
        assert!(goals.items.is_empty());
        let copied = report.text();
        assert!(copied.contains("\n    Places\n"), "{copied}");
        assert!(copied.contains("\n    Book (Blue Book)\n"), "{copied}");
    });
}

/// A document importer's finding met in many texts is one line, saying how many more
/// places there are, with every place listed under it; a warning and an information of
/// the same kind are two lines.
#[test]
fn a_document_finding_in_many_texts_is_one_line() {
    crate::test_support::with_real_messages(|| {
        let document = |item: i64, title: &str, warning: bool| {
            found(
                "table-read-as-paragraphs",
                ScrivenerFindingFamily::Document,
                ScrivenerReportGroup::CheckThese,
                item,
                &["Draft", title],
                vec![
                    ScrivenerArg::Flag {
                        name: "warning".into(),
                        value: warning,
                    },
                    count("count", 1),
                    text("detail", ""),
                ],
            )
        };
        let findings: Vec<ScrivenerFinding> = (0..200)
            .map(|i| document(i, &format!("Scene {i:03}"), true))
            .chain([document(300, "Coda", false)])
            .collect();
        let report = ImportReport::from_result(&result(findings, None));
        let check = &report.groups[0];
        assert_eq!(check.group, Group::CheckThese);
        assert_eq!(check.lines.len(), 2, "{:#?}", all_lines(&report));
        let line = &check.lines[0];
        assert_eq!(line.items.len(), 200);
        let text = line.text.resolve_now();
        assert!(
            text.contains("Scene 000") && text.ends_with("199 more places, listed below."),
            "{text}"
        );
        assert!(!check.lines[1].text.resolve_now().contains("more place"));
    });
}

/// A missing text whose words the search index kept is something to check, said as
/// such; one that arrives empty stays among what could not be read.
#[test]
fn a_salvaged_text_is_to_check_not_unread() {
    crate::test_support::with_real_messages(|| {
        let missing = |item: i64, title: &str, salvaged: bool| {
            found(
                "scrivener-notice-content-missing",
                ScrivenerFindingFamily::Source,
                ScrivenerReportGroup::Unfiled,
                item,
                &["Draft", title],
                vec![ScrivenerArg::Flag {
                    name: "salvaged".into(),
                    value: salvaged,
                }],
            )
        };
        let report = ImportReport::from_result(&result(
            vec![missing(2, "Storm", false), missing(3, "Arrival", true)],
            None,
        ));
        let groups: Vec<Group> = report.groups.iter().map(|g| g.group).collect();
        assert_eq!(groups, vec![Group::CouldNotBeRead, Group::CheckThese]);
        let salvaged = report.groups[1].lines[0].text.resolve_now();
        assert!(
            salvaged.contains("Draft ▸ Arrival") && salvaged.contains("search index"),
            "{salvaged}"
        );
    });
}

/// An item's text and its notes, found the same, are two names, and an item Scrivener
/// left unnamed is listed with a name rather than as an empty row.
#[test]
fn every_item_has_a_name_of_its_own() {
    crate::test_support::with_real_messages(|| {
        let at = |place: ScrivenerPlace, item: i64, path: &[&str], uid: u128| {
            ScrivenerFinding::Reported {
                key: "scrivener-prose-unreadable".into(),
                family: ScrivenerFindingFamily::Prose,
                group: ScrivenerReportGroup::CouldNotBeRead,
                place,
                item,
                binder_path: path.iter().map(|p| p.to_string()).collect(),
                item_uid: Some(Uuid::from_u128(uid)),
                args: Vec::new(),
            }
        };
        let report = ImportReport::from_result(&result(
            vec![
                at(ScrivenerPlace::Text, 3, &["Draft", "Storm"], 1),
                at(ScrivenerPlace::Notes, 3, &["Draft", "Storm"], 2),
                at(ScrivenerPlace::ProjectNote, -1, &[""], 3),
                at(ScrivenerPlace::Recovered, -1, &[], 4),
            ],
            None,
        ));
        let line = &report.groups[0].lines[0];
        let names: Vec<String> = line.items.iter().map(|i| i.label().resolve_now()).collect();
        assert_eq!(
            names,
            vec![
                "Draft ▸ Storm".to_string(),
                "Draft ▸ Storm (document notes)".to_string(),
                "Untitled (recovered text)".to_string(),
                "Untitled (project note)".to_string(),
            ]
        );
    });
}

/// What became of the research files is what the import did with them, not what the
/// analysis counted: nothing when none was imported, each case said as it happened.
#[test]
fn the_research_line_says_what_the_import_did() {
    crate::test_support::with_real_messages(|| {
        let research = |findings: Vec<ScrivenerFinding>, copy: bool| {
            let mut imported = result(
                findings,
                Some(ImportRecord {
                    copy_research_files: copy,
                    ..record()
                }),
            );
            if imported
                .findings
                .iter()
                .any(|f| messages::key_of(f) == "scrivener-prose-media-file-copied")
            {
                imported.research_folder = "The Lighthouse research files".into();
            }
            ImportReport::from_result(&imported)
                .choices
                .iter()
                .map(|c| c.resolve_now())
                .find(|c| c.contains("esearch files"))
        };
        assert_eq!(research(Vec::new(), true), None, "none was imported");
        let mixed =
            research(vec![copied(7, "Map.pdf"), stayed(8, "Huge.mov")], true).unwrap_or_default();
        assert!(
            mixed.contains("“The Lighthouse research files”") && mixed.contains("except 1"),
            "{mixed}"
        );
        let none = research(vec![stayed(8, "Huge.mov")], true).unwrap_or_default();
        assert!(none.starts_with("You asked for research files"), "{none}");
        let off = research(vec![stayed(8, "Huge.mov")], false).unwrap_or_default();
        assert!(off.contains("were not copied"), "{off}");
    });
}

/// A Save page hint says in the report what Skribisto proposed, and that the writer
/// changed it when they did; once they typed their own author, the proposal is no longer
/// something to check.
#[test]
fn a_save_page_hint_says_what_was_proposed_and_what_the_writer_did() {
    crate::test_support::with_real_messages(|| {
        let hint = |key: &str, args: Vec<ScrivenerArg>| ScrivenerFinding::Reported {
            key: key.into(),
            family: ScrivenerFindingFamily::Source,
            group: ScrivenerReportGroup::Unfiled,
            place: ScrivenerPlace::Project,
            item: -1,
            binder_path: Vec::new(),
            item_uid: None,
            args,
        };
        let findings = vec![
            hint(
                "scrivener-notice-title-from-project",
                vec![ScrivenerArg::Token {
                    name: "source".into(),
                    value: "compile-settings".into(),
                }],
            ),
            hint("scrivener-notice-author-from-project", Vec::new()),
        ];
        let kept = ImportReport::from_result(&result(findings.clone(), Some(record())));
        let lines = all_lines(&kept);
        assert!(
            lines.contains(
                &"Skribisto proposed the title the project’s compile settings state: the name \
                  the project had when it was created, unless a title was set there in \
                  Scrivener."
                    .to_string()
            ),
            "{lines:#?}"
        );
        assert!(lines.iter().all(|l| !l.starts_with("This ")), "{lines:#?}");
        assert_eq!(kept.lines_to_check(), 1, "the author is to check");

        let changed = ImportReport::from_result(&result(
            findings,
            Some(ImportRecord {
                title_changed: true,
                author_changed: true,
                author: "Ada Marlowe".into(),
                ..record()
            }),
        ));
        let lines = all_lines(&changed);
        assert!(
            lines
                .iter()
                .filter(|l| l.ends_with("You replaced this proposal."))
                .count()
                == 2,
            "{lines:#?}"
        );
        assert_eq!(changed.lines_to_check(), 0, "an author the writer typed");
        assert!(
            changed
                .choices
                .iter()
                .any(|c| c.resolve_now() == "Author: Ada Marlowe")
        );
    });
}

/// The Project page's answers come first among the writer's choices, and the report
/// names the project the import read.
#[test]
fn the_project_pages_answers_come_first() {
    crate::test_support::with_real_messages(|| {
        let answer = crate::import_scrivener::messages::answered(
            &frontend::import_management::ScrivenerQuestion::UpgradedSibling {
                backup: "Lighthouse Backup.scriv".into(),
                sibling: "Lighthouse.scriv".into(),
                sibling_generation: None,
                preselect_sibling: true,
            },
            0,
        )
        .expect("an answer the question offers");
        let report = ImportReport::from_result(&result(
            Vec::new(),
            Some(ImportRecord {
                source: "/books/Lighthouse.scriv".into(),
                project_answers: vec![answer],
                ..record()
            }),
        ));
        let first = report.choices[0].resolve_now();
        assert!(
            first.starts_with("You imported “Lighthouse.scriv”, the project you went on"),
            "{first}"
        );
        assert!(
            report
                .text()
                .contains("Read from /books/Lighthouse.scriv\n")
        );
    });
}

/// A report about a project with a great many names, or a great many findings of one
/// kind, is built in time linear in them: it is built on the interface's thread, and a
/// crafted project can define as many unused keywords as its manifest holds.
#[test]
fn a_report_of_many_names_and_findings_is_built_in_linear_time() {
    crate::test_support::with_real_messages(|| {
        let n = 60_000;
        let names = ScrivenerArg::Texts {
            name: "names".into(),
            values: (0..n).map(|k| format!("Keyword {k}")).collect(),
        };
        let mut findings = vec![metadata(
            "scrivener-metadata-unused-keywords-skipped",
            vec![names],
        )];
        findings.extend((0..n).map(|k| {
            metadata(
                "scrivener-metadata-collection-members-missing",
                vec![text("name", &format!("Collection {k}")), count("count", 1)],
            )
        }));
        findings.extend((0..n).map(|k| annotations(k, &format!("Scene {k}"), 1)));
        let started = std::time::Instant::now();
        let report = ImportReport::from_result(&result(findings, None));
        let seconds = started.elapsed().as_secs_f64();
        assert!(seconds < 10.0, "{seconds:.1} s");
        let lines: Vec<&ReportLine> = report.groups.iter().flat_map(|g| &g.lines).collect();
        let unused = lines
            .iter()
            .find(|l| l.listed == Listed::Names)
            .unwrap_or_else(|| panic!("no line lists the names"));
        assert_eq!(unused.items.len(), n as usize, "each name once");
    });
}
