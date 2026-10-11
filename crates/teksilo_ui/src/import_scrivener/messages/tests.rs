// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::import_management::scrivener::every_key;
use frontend::import_management::{
    ScrivenerFindingFamily, ScrivenerManifestEvidence, ScrivenerMobileEdits,
};

/// Every argument any sentence reads, with a plausible value, so one finding can stand
/// for any key: a sentence picks the names it needs.
fn every_arg() -> Vec<ScrivenerArg> {
    let count = |name: &str, value: i64| ScrivenerArg::Count {
        name: name.into(),
        value,
    };
    let text = |name: &str, value: &str| ScrivenerArg::Text {
        name: name.into(),
        value: value.into(),
    };
    let token = |name: &str, value: &str| ScrivenerArg::Token {
        name: name.into(),
        value: value.into(),
    };
    let texts = |name: &str| ScrivenerArg::Texts {
        name: name.into(),
        values: vec!["one".into(), "two".into()],
    };
    let mut args = vec![
        text("file", "Novel.scrivx"),
        text("reason", "permission denied"),
        text("manifest", "Novel.scrivx"),
        text("member", "Files/Data/A/content.rtf"),
        text("version", "3.0"),
        text("method", "bzip2"),
        text("path", "Books/Novel.scriv"),
        text("title", "Novel"),
        text("name", "Novel 2.scrivx"),
        text("original", "Novel.scrivx"),
        text("chosen", "Novel.scrivx"),
        text("creator", "SCRWIN-CONFLICT"),
        text("detail", "a broken invariant"),
        text("tag", "ja"),
        text("item_key", "1F2E"),
        text("placeholder", "<$n>"),
        token("damage", "not-xml"),
        token("why", "parent-folder"),
        token("error", "violation"),
        token("sign", "date-stamped"),
        token("problem", "malformed"),
        token("kind", "keyword"),
        token("source", "compile-settings"),
        token("field", "label"),
        token("what", "colour"),
        token("reading", "markers-counted"),
        text("raw", "1-2, 4-6"),
        token("section", "bookmarks"),
        token("unit", "words"),
        texts("examples"),
        texts("copies"),
        texts("rungs"),
        texts("names"),
        texts("values"),
        texts("matches"),
        texts("ids"),
        ScrivenerArg::Day {
            name: "day".into(),
            value: "2027-03-01".into(),
        },
        ScrivenerArg::Flag {
            name: "day_off".into(),
            value: true,
        },
        ScrivenerArg::Flag {
            name: "this_machine".into(),
            value: false,
        },
        ScrivenerArg::Flag {
            name: "salvaged".into(),
            value: true,
        },
    ];
    for name in [
        "count",
        "limit",
        "limit_mib",
        "stamp",
        "newest",
        "spare_copies",
        "others",
        "original_missing",
        "text_items",
        "snapshots",
        "documents",
        "of_deleted",
        "days",
        "unreadable",
        "script_documents",
        "draft_documents",
        "headings",
        "words",
        "items",
        "with_writing",
        "tags",
        "rows",
        "id",
        "paragraphs",
        "fields",
        "keywords",
        "value",
        "values",
    ] {
        args.push(count(name, 3));
    }
    args
}

fn finding(key: &str) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Source,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Item,
        item: 4,
        binder_path: vec!["Draft".into(), "Chapter 1".into()],
        item_uid: None,
        args: every_arg(),
    }
}

/// A question of each key, with what its sentence names.
fn question_of(key: &str) -> ScrivenerQuestion {
    match key {
        "scrivener-question-several-projects" => ScrivenerQuestion::SeveralProjects {
            candidates: vec!["One.scriv".into(), "Two.scriv".into()],
        },
        "scrivener-question-several-manifests" => ScrivenerQuestion::SeveralManifests {
            candidates: vec![ScrivenerManifestCandidate::Candidate {
                name: "Novel.scrivx".into(),
                creator: None,
                version: None,
                modified: Some(1_717_236_000),
                items: Some(12),
                matches_autosave: true,
                identical_to_preselected: false,
            }],
            preselected: Some("Novel.scrivx".into()),
            evidence: vec![ScrivenerManifestEvidence::MatchesAutosave],
        },
        "scrivener-question-enclosing-project" => ScrivenerQuestion::EnclosingProject {
            project: "Novel.scriv".into(),
            levels_up: 2,
        },
        "scrivener-question-upgraded-sibling" => ScrivenerQuestion::UpgradedSibling {
            backup: "Novel Backup.scriv".into(),
            sibling: "Novel.scriv".into(),
            sibling_generation: None,
            preselect_sibling: true,
        },
        "scrivener-question-unmerged-mobile-edits" => ScrivenerQuestion::UnmergedMobileEdits {
            source: ScrivenerMobileEdits::MobileFolder,
        },
        "scrivener-question-no-text-in-copy" => ScrivenerQuestion::NoTextInCopy { text_items: 24 },
        other => panic!("{other} is not a question"),
    }
}

/// A sentence reads as one: resolved, not the message id, every argument filled.
fn assert_reads(locale: &str, key: &str, sentence: &LocalizedString) {
    let text = sentence.resolve_now();
    assert!(!text.trim().is_empty(), "{locale} {key}: empty");
    assert!(
        !text.starts_with("import-scrivener-"),
        "{locale} {key}: resolved to its own id: {text}"
    );
    assert!(
        !text.contains("{$") && !text.contains("{ $"),
        "{locale} {key}: an argument left unfilled: {text}"
    );
}

/// Every key the converter can give, the prose notices of its report included, has a row,
/// and every row names a key the converter gives: a new converter key cannot ship
/// unworded, and a row cannot outlive its key.
#[test]
fn the_table_and_the_converters_keys_agree_both_ways() {
    let converter: Vec<&str> = every_key();
    let missing: Vec<&&str> = converter.iter().filter(|k| entry(k).is_none()).collect();
    assert!(missing.is_empty(), "keys with no row: {missing:?}");
    let stale: Vec<&str> = TABLE
        .iter()
        .map(|e| e.key)
        .filter(|k| !converter.contains(k))
        .collect();
    assert!(stale.is_empty(), "rows naming no converter key: {stale:?}");
    let mut seen = std::collections::HashSet::new();
    for row in TABLE {
        assert!(seen.insert(row.key), "{} has two rows", row.key);
    }
}

/// Every key is worded where its row says it shows, in both shipped locales.
#[test]
fn every_key_is_worded_in_both_locales() {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            for row in TABLE {
                let found = finding(row.key);
                match row.tier {
                    Tier::Refusal => {
                        let worded = refusal(&found)
                            .unwrap_or_else(|| panic!("{} has no refusal arm", row.key));
                        assert_reads(locale, row.key, &worded.title);
                        if let Some(body) = &worded.body {
                            assert_reads(locale, row.key, body);
                        }
                        let paragraph = refusal_paragraph(&found)
                            .unwrap_or_else(|| panic!("{} has no paragraph", row.key));
                        assert_reads(locale, row.key, &paragraph);
                    }
                    Tier::Choose | Tier::Accept => {
                        let asked = question(&question_of(row.key))
                            .unwrap_or_else(|| panic!("{} has no question arm", row.key));
                        assert_reads(locale, row.key, &asked.title);
                        assert_reads(locale, row.key, &asked.body);
                        for option in &asked.options {
                            assert_reads(locale, row.key, option);
                        }
                        assert_eq!(
                            asked.accept.is_some(),
                            row.tier == Tier::Accept,
                            "{}: an accepted question carries a checkbox label",
                            row.key
                        );
                        if let Some(accept) = &asked.accept {
                            assert_reads(locale, row.key, accept);
                        }
                    }
                    Tier::Card | Tier::SaveHint => {
                        let worded = notice(&found, &ScrivenerComputers::Unknown)
                            .unwrap_or_else(|| panic!("{} has no notice arm", row.key));
                        assert_reads(locale, row.key, &worded.title);
                        if let Some(body) = &worded.body {
                            assert_reads(locale, row.key, body);
                        }
                    }
                    Tier::BeforeImport | Tier::Report => {
                        let line = summary_line(row.key, &[&found])
                            .unwrap_or_else(|| panic!("{} has no line", row.key));
                        assert_reads(locale, row.key, &line);
                    }
                }
            }
        });
    }
}

/// Every key the report will word is filed under a group, and every refusal and question
/// under none: a refusal ends an import rather than describing one.
#[test]
fn every_finding_has_a_report_group() {
    for row in TABLE {
        match row.tier {
            Tier::Refusal | Tier::Choose | Tier::Accept => {
                assert!(row.group.is_none(), "{}", row.key)
            }
            _ => assert!(row.group.is_some(), "{} has no group", row.key),
        }
    }
}

/// The converter files its own findings; the table files the source notices it leaves
/// unfiled, and only those.
#[test]
fn the_converters_filing_wins_over_the_tables() {
    let mut found = finding("scrivener-notice-lock-present");
    assert_eq!(group_of(&found), Some(Group::CheckThese));
    if let ScrivenerFinding::Reported { group, .. } = &mut found {
        *group = ScrivenerReportGroup::LeftBehind;
    }
    assert_eq!(group_of(&found), Some(Group::LeftBehind));
    assert!(Group::CouldNotBeRead.counts_in_headline());
    assert!(Group::CheckThese.counts_in_headline());
    assert!(!Group::BroughtAcrossDifferently.counts_in_headline());
    assert!(!Group::LeftBehind.counts_in_headline());
}

/// A count takes its plural form: French counts 0 and 1 as one, and a sentence never
/// reads "1 files".
#[test]
fn counts_take_their_plural_form() {
    let debris = |n: i64| ScrivenerFinding::Reported {
        key: "scrivener-notice-sync-debris-ignored".into(),
        family: ScrivenerFindingFamily::Source,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args: vec![ScrivenerArg::Count {
            name: "count".into(),
            value: n,
        }],
    };
    let title = |n: i64| {
        notice(&debris(n), &ScrivenerComputers::Unknown)
            .map(|w| w.title.resolve_now())
            .unwrap_or_default()
    };
    crate::test_support::with_shipped_messages("en-US", || {
        assert!(title(1).starts_with("1 extra copy "), "{}", title(1));
        assert!(title(4).starts_with("4 extra copies "), "{}", title(4));
    });
    crate::test_support::with_shipped_messages("fr-FR", || {
        assert!(title(0).contains("copie en trop"), "{}", title(0));
        assert!(title(1).contains("copie en trop"), "{}", title(1));
        assert!(title(2).contains("copies en trop"), "{}", title(2));
    });
}

/// The lock names the computer the project card may show, and says so when it is this
/// one, as a warning; the computer's name reaches no other sentence.
#[test]
fn the_lock_names_its_computer_only_on_the_card() {
    let lock = |here: bool| ScrivenerFinding::Reported {
        key: "scrivener-notice-lock-present".into(),
        family: ScrivenerFindingFamily::Source,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args: vec![ScrivenerArg::Flag {
            name: "this_machine".into(),
            value: here,
        }],
    };
    let computers = ScrivenerComputers::Named {
        device: None,
        lock_host: Some("Studio Mac".into()),
    };
    crate::test_support::with_real_messages(|| {
        let elsewhere =
            notice(&lock(false), &computers).map(|w| (w.title.resolve_now(), w.warning));
        assert_eq!(
            elsewhere,
            Some((
                "Scrivener had this project open on “Studio Mac”".to_string(),
                false
            ))
        );
        let here = notice(&lock(true), &computers).map(|w| (w.title.resolve_now(), w.warning));
        assert_eq!(
            here,
            Some((
                "Scrivener has this project open on this computer".to_string(),
                true
            ))
        );
        let unnamed = notice(&lock(false), &ScrivenerComputers::Unknown)
            .map(|w| w.title.resolve_now())
            .unwrap_or_default();
        assert!(!unnamed.contains('“'), "{unnamed}");
    });
}

/// A refusal whose cause may go away by itself offers Read the project again; a fault of the
/// importer's own carries its technical account for Details.
#[test]
fn refusals_say_what_can_be_done() {
    let retry = |key: &str| refusal(&finding(key)).is_some_and(|w| w.retry);
    assert!(retry("scrivener-refusal-cloud-placeholders"));
    assert!(retry("scrivener-refusal-project-changed-during-read"));
    assert!(retry("scrivener-refusal-incomplete-copy"));
    assert!(!retry("scrivener-refusal-scrivener-one-for-mac"));
    let details =
        refusal(&finding("scrivener-refusal-structure-unmappable")).and_then(|w| w.details);
    assert_eq!(details.as_deref(), Some("a broken invariant"));
    assert!(refusal(&finding("scrivener-refusal-made-up")).is_none());
}

/// The items a notice is about are named by their path in Scrivener's binder.
#[test]
fn a_notice_names_its_item_by_its_binder_path() {
    crate::test_support::with_real_messages(|| {
        let mut missing = finding("scrivener-notice-content-missing");
        if let ScrivenerFinding::Reported { place, args, .. } = &mut missing {
            *place = ScrivenerPlace::Synopsis;
            args.retain(|a| !matches!(a, ScrivenerArg::Flag { name, .. } if name == "salvaged"));
        }
        let worded = notice(&missing, &ScrivenerComputers::Unknown);
        let title = worded
            .as_ref()
            .map(|w| w.title.resolve_now())
            .unwrap_or_default();
        assert_eq!(title, "The synopsis of “Draft ▸ Chapter 1” is missing");
        assert!(
            worded.is_some_and(|w| w.warning),
            "text lost reads as a warning"
        );
    });
}

/// Every failure reason is worded in both locales, and the Scrivener importer words the
/// ones that are its own: the refusal as the Project page says it, a project that changed
/// or asks again as an invitation to review it again.
#[test]
fn every_failure_is_worded_and_the_scrivener_ones_in_its_own_words() {
    crate::shared::import_failure::assert_every_failure_worded(own_failure, |reason| {
        matches!(
            reason,
            FailureReason::ScrivenerRefused { .. }
                | FailureReason::ScrivenerQuestionFirst { .. }
                | FailureReason::ScrivenerNotAccepted { .. }
                | FailureReason::ScrivenerChangedSinceReview
                | FailureReason::ScrivenerDepthNotOffered { .. }
                | FailureReason::ScrivenerRootNotOffered { .. }
        )
    });
    crate::test_support::with_real_messages(|| {
        let refused = own_failure(&FailureReason::ScrivenerRefused {
            refusal: finding("scrivener-refusal-scrivener-one-for-mac"),
        })
        .map(|l| l.resolve_now())
        .unwrap_or_default();
        assert!(
            refused.starts_with("This project was made by Scrivener 1 for Mac. Skribisto reads"),
            "{refused}"
        );
        assert!(refused.ends_with("Nothing was written."), "{refused}");
    });
    assert!(reviewing_again_helps(
        &FailureReason::ScrivenerChangedSinceReview
    ));
    assert!(!reviewing_again_helps(&FailureReason::Cancelled));
}

/// The structure lines count the items behind them, and a folder left out counts what
/// it held.
#[test]
fn a_line_counts_the_findings_of_its_kind() {
    crate::test_support::with_real_messages(|| {
        let left = |items: i64| ScrivenerFinding::Reported {
            key: "scrivener-structure-left-out".into(),
            family: ScrivenerFindingFamily::Structure,
            group: ScrivenerReportGroup::LeftBehind,
            place: ScrivenerPlace::Item,
            item: 3,
            binder_path: vec!["Ideas".into()],
            item_uid: None,
            args: vec![ScrivenerArg::Count {
                name: "items".into(),
                value: items,
            }],
        };
        let (a, b) = (left(2), left(5));
        let line = summary_line("scrivener-structure-left-out", &[&a, &b])
            .map(|l| l.resolve_now())
            .unwrap_or_default();
        assert_eq!(line, "2 top-level folders are left out, with 7 items.");
        assert!(summary_line("scrivener-structure-made-up", &[&a]).is_none());
    });
}

// ── what the sentences say ───────────────────────────────────────────────────────────────

fn refusal_with(key: &str, args: Vec<ScrivenerArg>) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Refusal,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args,
    }
}

fn arg_text(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Text {
        name: name.into(),
        value: value.into(),
    }
}

fn arg_token(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Token {
        name: name.into(),
        value: value.into(),
    }
}

fn arg_count(name: &str, value: i64) -> ScrivenerArg {
    ScrivenerArg::Count {
        name: name.into(),
        value,
    }
}

/// The title and body of a refusal, resolved.
fn said(finding: &ScrivenerFinding) -> (String, String) {
    let Some(worded) = refusal(finding) else {
        panic!("{} is not worded", key_of(finding));
    };
    (
        worded.title.resolve_now(),
        worded.body.map(|b| b.resolve_now()).unwrap_or_default(),
    )
}

/// A label with its mnemonic marker taken out, as the menu shows it.
fn shown(label: LocalizedString) -> String {
    label.resolve_now().replace('&', "")
}

const LOCALES: [&str; 2] = ["en-US", "fr-FR"];

/// The script-mode line agrees with how many documents are in script mode, not with how
/// many the manuscript holds.
#[test]
fn the_script_mode_line_agrees_with_its_subject() {
    crate::test_support::with_real_messages(|| {
        let line = tr!(import_scrivener_structure_script_mode_draft_line(
            script = 1_i64,
            documents = 2_i64
        ))
        .resolve_now();
        assert!(
            line.starts_with("1 of the 2 manuscript documents is "),
            "{line}"
        );
        assert!(line.contains(" its words"), "{line}");
    });
    crate::test_support::with_shipped_messages("fr-FR", || {
        let line = tr!(import_scrivener_structure_script_mode_draft_line(
            script = 1_i64,
            documents = 2_i64
        ))
        .resolve_now();
        assert!(
            line.starts_with("1 document du manuscrit sur 2 est écrit"),
            "{line}"
        );
    });
}

/// The way to the statuses names the sections the Settings window shows.
#[test]
fn the_way_to_the_statuses_names_the_settings_sections() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let path = format!(
                "{} ▸ {} ▸ {}",
                tr!(settings_title()).resolve_now(),
                tr!(settings_sec_work()).resolve_now(),
                tr!(settings_page_statuses()).resolve_now()
            );
            for count in [1_i64, 3] {
                let line = tr!(import_scrivener_metadata_status_categories_guessed_line(
                    count = count
                ))
                .resolve_now();
                assert!(line.contains(&path), "{locale}: {line} does not say {path}");
            }
        });
    }
}

/// The story bible is called what the app calls it.
#[test]
fn the_story_bible_is_called_what_the_app_calls_it() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let name = tr!(segment_story_bible()).resolve_now();
            assert_eq!(tr!(import_scrivener_dest_story_bible()).resolve_now(), name);
            let lower = name.to_lowercase();
            for line in [
                tr!(import_scrivener_structure_generic_sheet_title_line(
                    count = 2_i64
                )),
                tr!(import_scrivener_structure_story_bible_folder_line(
                    count = 2_i64
                )),
            ] {
                let line = line.resolve_now();
                assert!(line.to_lowercase().contains(&lower), "{locale}: {line}");
            }
        });
    }
}

/// A refusal states the cause the analysis found, not another one.
#[test]
fn a_refusal_says_the_cause_it_was_given() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let unsafe_name = |why: &str| {
                said(&refusal_with(
                    "scrivener-refusal-archive-unsafe-name",
                    vec![
                        arg_text("file", "Novel.zip"),
                        arg_text("member", if why == "empty" { "" } else { "../x" }),
                        arg_token("why", why),
                    ],
                ))
                .1
            };
            let outside = unsafe_name("parent-folder");
            for why in ["empty", "control-character"] {
                assert_ne!(unsafe_name(why), outside, "{locale}: {why}");
            }
            assert_eq!(unsafe_name("absolute"), outside, "{locale}");

            let template = |damage: Vec<ScrivenerArg>| {
                let mut args = vec![arg_text("file", "Novel.scrivtemplate")];
                args.extend(damage);
                said(&refusal_with(
                    "scrivener-refusal-template-undecodable",
                    args,
                ))
            };
            let incomplete = template(vec![arg_token("damage", "undecodable")]);
            let too_large = template(vec![
                arg_token("damage", "too-large"),
                arg_count("limit_mib", 256),
            ]);
            let not_a_template = template(vec![arg_token("damage", "not-a-template")]);
            assert_ne!(too_large.0, incomplete.0, "{locale}: too large");
            assert!(too_large.1.contains("256"), "{locale}: {}", too_large.1);
            assert_ne!(not_a_template.0, incomplete.0, "{locale}: not a template");
            assert_ne!(not_a_template.1, incomplete.1, "{locale}: not a template");

            let manifest = |damage: &str| {
                said(&refusal_with(
                    "scrivener-refusal-damaged-manifest",
                    vec![
                        arg_text("manifest", "Novel.scrivx"),
                        arg_token("damage", damage),
                        arg_count("spare_copies", 0),
                    ],
                ))
                .1
            };
            assert_ne!(
                manifest("new-project-placeholder"),
                manifest("not-xml"),
                "{locale}: a placeholder is not damaged"
            );
        });
    }
}

/// Every count a sentence quotes takes its own plural form.
#[test]
fn every_count_takes_its_own_plural() {
    crate::test_support::with_real_messages(|| {
        assert_eq!(
            tr!(import_scrivener_metadata_labels_became_tags_line(
                tags = 1_i64,
                rows = 1_i64
            ))
            .resolve_now(),
            "The labels become 1 tag, on 1 item."
        );
        assert_eq!(
            tr!(import_scrivener_metadata_statuses_became_ladder_line(
                rungs = 4_i64,
                rows = 1_i64
            ))
            .resolve_now(),
            "The status list comes across as the project’s list of statuses: 4 statuses, on 1 item."
        );
        assert_eq!(
            tr!(import_scrivener_metadata_fields_became_tags_line(
                fields = 1_i64,
                tags = 2_i64,
                rows = 1_i64
            ))
            .resolve_now(),
            "1 list or checkbox field becomes 2 tags, on 1 item."
        );
        assert_eq!(
            tr!(import_scrivener_metadata_keywords_became_tags_line(
                tags = 3_i64,
                rows = 1_i64
            ))
            .resolve_now(),
            "3 keywords become tags, on 1 item."
        );
        let line = tr!(import_scrivener_structure_paratext_not_counted_line(
            documents = 1_i64,
            words = 1_i64
        ))
        .resolve_now();
        assert!(line.contains(", so 1 word no longer counts "), "{line}");
    });
    crate::test_support::with_shipped_messages("fr-FR", || {
        let line = tr!(import_scrivener_metadata_statuses_became_ladder_line(
            rungs = 4_i64,
            rows = 0_i64
        ))
        .resolve_now();
        assert!(line.ends_with("sur 0 élément."), "{line}");
        let line = tr!(import_scrivener_structure_paratext_not_counted_line(
            documents = 1_i64,
            words = 1_i64
        ))
        .resolve_now();
        assert!(
            line.contains("\u{a0}: 1 mot ne compte donc plus "),
            "{line}"
        );
    });
}

/// The result says what the number is: the rows the new project holds, some of which the
/// import made (a document's notes, the project notes), not documents that came across.
#[test]
fn the_result_counts_what_the_new_project_holds() {
    crate::test_support::with_real_messages(|| {
        assert_eq!(
            tr!(import_scrivener_done(items = 22_i64)).resolve_now(),
            "The new project holds 22 items."
        );
    });
    crate::test_support::with_shipped_messages("fr-FR", || {
        let line = tr!(import_scrivener_done(items = 22_i64)).resolve_now();
        // What the project holds, not what "arrived".
        assert!(line.starts_with("Le nouveau projet "), "{line}");
        assert!(line.ends_with(" 22 éléments."), "{line}");
    });
}

/// The research folder's name reads right whatever the project's name begins with: no
/// word before it that would have to change with it.
#[test]
fn the_research_folders_name_needs_no_elision() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let named = tr!(import_scrivener_research_folder(
                name = "Ombres".to_string()
            ))
            .resolve_now();
            assert!(!named.contains(" de Ombres"), "{locale}: {named}");
            assert!(named.contains("Ombres"), "{locale}: {named}");
        });
    }
}

/// The review's hint names the binder command that changes an item's type.
#[test]
fn the_review_hint_names_the_command() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let hint = tr!(import_scrivener_review_hint()).resolve_now();
            let command = shown(tr!(ctx_promote()));
            assert!(
                hint.contains(&command),
                "{locale}: {hint} does not name {command}"
            );
            assert!(!hint.to_lowercase().contains("retyp"), "{locale}: {hint}");
        });
    }
}

/// The before-import lines call front and back matter by the type the preview shows.
#[test]
fn front_and_back_matter_is_called_paratext() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let paratext = tr!(create_paratext()).resolve_now().to_lowercase();
            for line in [
                tr!(import_scrivener_structure_matter_inside_book_line(
                    count = 2_i64
                )),
                tr!(import_scrivener_structure_paratext_not_counted_line(
                    documents = 2_i64,
                    words = 300_i64
                )),
            ] {
                let line = line.resolve_now();
                assert!(line.to_lowercase().contains(&paratext), "{locale}: {line}");
            }
        });
    }
}

/// A refusal whose cause may go away by itself tells the writer to read the project again;
/// when an import fails on one, the failure offers that reading: **Review the import again**.
#[test]
fn a_refused_import_offers_the_check_its_words_name() {
    crate::test_support::with_real_messages(|| {
        for key in [
            "scrivener-refusal-cloud-placeholders",
            "scrivener-refusal-incomplete-copy",
            "scrivener-refusal-unreadable-members",
            "scrivener-refusal-project-changed-during-read",
        ] {
            let finding = refusal_with(key, vec![arg_count("count", 2)]);
            assert!(refusal(&finding).is_some_and(|w| w.retry), "{key}");
            assert!(
                reviewing_again_helps(&FailureReason::ScrivenerRefused { refusal: finding }),
                "{key}"
            );
        }
        let lasting = refusal_with("scrivener-refusal-scrivener-one-for-mac", Vec::new());
        assert!(!reviewing_again_helps(&FailureReason::ScrivenerRefused {
            refusal: lasting
        }));
    });
}

// ── the report's prose lines ─────────────────────────────────────────────────────────────

/// A prose finding in `place`, with `args`.
fn prose(key: &str, binder_path: &[&str], args: Vec<ScrivenerArg>) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Prose,
        group: ScrivenerReportGroup::BroughtAcrossDifferently,
        place: ScrivenerPlace::Text,
        item: 3,
        binder_path: binder_path.iter().map(|p| p.to_string()).collect(),
        item_uid: None,
        args,
    }
}

/// A key whose token makes another sentence says each one, in both locales, and each says
/// something the others do not.
#[test]
fn every_variant_of_a_prose_line_is_its_own_sentence() {
    let variants: [(&str, &str, &[&str]); 3] = [
        (
            "scrivener-prose-formatting-lost",
            "what",
            &[
                "colour",
                "highlight",
                "small-caps",
                "all-caps",
                "hidden",
                "unknown",
            ],
        ),
        (
            "scrivener-prose-left-out",
            "what",
            &[
                "after-document",
                "header-or-footer",
                "list-label-picture",
                "unknown",
            ],
        ),
        (
            "scrivener-prose-range-comment-reading",
            "reading",
            &["markers-counted", "pictures-not-counted", "markers-removed"],
        ),
    ];
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            for (key, name, tokens) in variants {
                let mut said = std::collections::HashSet::new();
                for token in tokens {
                    let found = prose(
                        key,
                        &["Draft", "Chapter 1"],
                        vec![arg_token(name, token), arg_count("count", 2)],
                    );
                    assert_eq!(line_variant(&found).as_deref(), Some(*token));
                    let line = summary_line(key, &[&found])
                        .unwrap_or_else(|| panic!("{key} {token} has no line"));
                    assert_reads(locale, key, &line);
                    assert!(said.insert(line.resolve_now()), "{locale} {key} {token}");
                }
            }
        });
    }
}

/// A prose line counts the texts its kind was found in, or adds up what each of them
/// counted, and quotes the project's own words.
#[test]
fn a_prose_line_counts_its_places_and_adds_their_counts() {
    crate::test_support::with_real_messages(|| {
        let line = |key: &str, findings: &[&ScrivenerFinding]| {
            summary_line(key, findings)
                .map(|l| l.resolve_now())
                .unwrap_or_default()
        };
        let (a, b) = (
            prose(
                "scrivener-prose-annotations-became-comments",
                &["Draft", "One"],
                vec![arg_count("count", 2)],
            ),
            prose(
                "scrivener-prose-annotations-became-comments",
                &["Draft", "Two"],
                vec![arg_count("count", 3)],
            ),
        );
        assert_eq!(
            line("scrivener-prose-annotations-became-comments", &[&a, &b]),
            "5 inline annotations became comments."
        );
        let unread = prose("scrivener-prose-unreadable", &["Draft", "One"], Vec::new());
        assert_eq!(
            line("scrivener-prose-unreadable", &[&unread, &unread]),
            "2 texts could not be read. They arrive empty."
        );
        let placeholder = prose(
            "scrivener-prose-placeholders-in-exported-text",
            &["Draft", "Title page"],
            vec![
                arg_text("placeholder", "<$PROJECTTITLE>"),
                arg_count("count", 1),
            ],
        );
        assert_eq!(
            line(
                "scrivener-prose-placeholders-in-exported-text",
                &[&placeholder]
            ),
            "1 Scrivener placeholder, <$PROJECTTITLE>, is kept as plain text in what you \
             export. Replace it by hand.",
            "quoted as the project wrote it, so the writer can search for it"
        );
        let styles = [
            prose(
                "scrivener-prose-style-not-carried",
                &["Draft", "One"],
                vec![arg_text("name", "Letter"), arg_count("count", 4)],
            ),
            prose(
                "scrivener-prose-style-not-carried",
                &["Draft", "Two"],
                vec![arg_text("name", "Letter"), arg_count("count", 1)],
            ),
            prose(
                "scrivener-prose-style-not-carried",
                &["Draft", "Two"],
                vec![arg_text("name", "Sign"), arg_count("count", 1)],
            ),
        ];
        let refs: Vec<&ScrivenerFinding> = styles.iter().collect();
        assert_eq!(
            line("scrivener-prose-style-not-carried", &refs),
            "2 styles, such as “Letter”, have no counterpart in Skribisto. Their text keeps \
             its words."
        );
    });
}

/// A finding of the document importer keeps that importer's sentence, naming the item by
/// its title where the sentence names a file.
#[test]
fn a_document_importers_finding_keeps_its_own_sentence() {
    crate::test_support::with_real_messages(|| {
        let found = ScrivenerFinding::Reported {
            key: "table-read-as-paragraphs".into(),
            family: ScrivenerFindingFamily::Document,
            group: ScrivenerReportGroup::CheckThese,
            place: ScrivenerPlace::Text,
            item: 5,
            binder_path: vec!["Draft".into(), "Act 1/2".into()],
            item_uid: None,
            args: vec![
                ScrivenerArg::Flag {
                    name: "warning".into(),
                    value: true,
                },
                arg_count("count", 2),
                arg_text("detail", ""),
            ],
        };
        assert!(is_document_finding(&found));
        let line = document_line(&found)
            .map(|l| l.resolve_now())
            .unwrap_or_default();
        assert!(line.starts_with("2 tables in Act 1\u{2215}2 "), "{line}");
    });
}

/// The report says a notice the Project page showed by its title, and names no computer.
#[test]
fn the_report_names_no_computer() {
    crate::test_support::with_real_messages(|| {
        let lock = ScrivenerFinding::Reported {
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
        };
        let line = notice_line(&lock, false)
            .map(|l| l.resolve_now())
            .unwrap_or_default();
        let named = notice(
            &lock,
            &ScrivenerComputers::Named {
                lock_host: Some("writers-laptop".into()),
                device: None,
            },
        )
        .map(|w| w.title.resolve_now())
        .unwrap_or_default();
        assert!(named.contains("writers-laptop"), "{named}");
        assert!(
            !line.is_empty() && !line.contains("writers-laptop"),
            "{line}"
        );
    });
}

/// A Save page hint in the report says, in the past, what Skribisto proposed, never the
/// Save page's "This title is …", which points at a field the report does not show; and
/// that the writer changed it, when they did. In both shipped locales.
#[test]
fn a_save_page_hint_in_the_report_says_what_was_proposed() {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            for row in TABLE.iter().filter(|r| r.tier == Tier::SaveHint) {
                let found = finding(row.key);
                let hint = notice(&found, &ScrivenerComputers::Unknown)
                    .map(|w| w.title.resolve_now())
                    .unwrap_or_default();
                for changed in [false, true] {
                    let line = notice_line(&found, changed)
                        .unwrap_or_else(|| panic!("{} has no report line", row.key));
                    assert_reads(locale, row.key, &line);
                    let line = line.resolve_now();
                    assert!(!line.contains(&hint), "{locale} {}: {line}", row.key);
                }
                assert!(save_field_of(row.key).is_some(), "{}", row.key);
            }
        });
    }
}

/// The sentences only the report says read in both shipped locales: the metadata lines
/// of each field, a salvaged text, a document finding found in several places, the
/// Project page's answers.
#[test]
fn the_reports_own_sentences_read_in_both_locales() {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            for key in [
                "scrivener-metadata-none-value-collapsed",
                "scrivener-metadata-values-without-row",
                "scrivener-metadata-field-renamed",
                "scrivener-metadata-unnamed-value",
            ] {
                let mut said = std::collections::HashSet::new();
                for field in ["label", "status"] {
                    let found = prose(
                        key,
                        &[],
                        vec![
                            arg_token("field", field),
                            arg_text("name", "None"),
                            arg_text("title", "Time"),
                            arg_count("items", 3),
                            arg_count("id", 4),
                        ],
                    );
                    let line = summary_line(key, &[&found])
                        .unwrap_or_else(|| panic!("{key} {field} has no line"));
                    assert_reads(locale, key, &line);
                    assert!(said.insert(line.resolve_now()), "{locale} {key} {field}");
                    assert!(line_variant(&found).is_some_and(|v| v.contains(field)));
                }
            }
            for place in [
                ScrivenerPlace::Text,
                ScrivenerPlace::Notes,
                ScrivenerPlace::Synopsis,
            ] {
                let mut found = finding("scrivener-notice-content-missing");
                if let ScrivenerFinding::Reported { place: p, .. } = &mut found {
                    *p = place;
                }
                let line = notice_line(&found, false).expect("a salvaged line");
                assert_reads(locale, "content-missing salvaged", &line);
                assert_eq!(group_of(&found), Some(Group::CheckThese));
            }
            let document = ScrivenerFinding::Reported {
                key: "list-nesting-flattened".into(),
                family: ScrivenerFindingFamily::Document,
                group: ScrivenerReportGroup::CheckThese,
                place: ScrivenerPlace::Text,
                item: 5,
                binder_path: vec!["Draft".into(), "Arrival".into()],
                item_uid: None,
                args: vec![arg_count("count", 2), arg_text("detail", "4")],
            };
            for n in [2, 3] {
                let several: Vec<&ScrivenerFinding> = (0..n).map(|_| &document).collect();
                let line = document_lines(&several).expect("a line");
                assert_reads(locale, "document-others", &line);
            }
            for key in [
                "scrivener-question-several-projects",
                "scrivener-question-several-manifests",
                "scrivener-question-enclosing-project",
                "scrivener-question-upgraded-sibling",
            ] {
                let question = question_of(key);
                let options = super::question(&question).map_or(0, |a| a.options.len());
                for option in 0..options {
                    let line = answered(&question, option)
                        .unwrap_or_else(|| panic!("{key} answer {option} is not said"));
                    assert_reads(locale, key, &line);
                }
            }
        });
    }
}

/// A metadata finding of `key` with `args`, about the project as a whole.
fn metadata(key: &str, args: Vec<ScrivenerArg>) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Metadata,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Item,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args,
    }
}

fn arg_day(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Day {
        name: name.into(),
        value: value.into(),
    }
}

fn arg_flag(name: &str, value: bool) -> ScrivenerArg {
    ScrivenerArg::Flag {
        name: name.into(),
        value,
    }
}

fn arg_texts(name: &str, values: &[&str]) -> ScrivenerArg {
    ScrivenerArg::Texts {
        name: name.into(),
        values: values.iter().map(|v| v.to_string()).collect(),
    }
}

/// Every sentence a metadata key of the second half can say, one finding each: a note of
/// the "From Scrivener" folder, a unit, a reason, a field, a day off or none.
fn every_metadata_variant() -> Vec<(&'static str, ScrivenerFinding)> {
    let mut out = Vec::new();
    for section in [
        "project",
        "project-bookmarks",
        "replacements",
        "auto-complete",
        "saved-searches",
        "custom-metadata",
        "targets",
        "bookmarks",
    ] {
        out.push((
            section,
            metadata(
                "scrivener-metadata-listed-in-import-note",
                vec![arg_token("section", section), arg_count("count", 3)],
            ),
        ));
    }
    for unit in ["words", "characters"] {
        out.push((
            unit,
            metadata(
                "scrivener-metadata-draft-target-kept",
                vec![arg_count("value", 80_000), arg_token("unit", unit)],
            ),
        ));
    }
    for (reason, unit) in [
        ("pages", "pages"),
        ("no-single-book", "words"),
        ("no-single-book", "characters"),
    ] {
        out.push((
            unit,
            metadata(
                "scrivener-metadata-draft-target-not-kept",
                vec![
                    arg_token("reason", reason),
                    arg_count("value", 8),
                    arg_token("unit", unit),
                ],
            ),
        ));
    }
    for reason in [
        "passed",
        "no-writing-day-left",
        "no-word-goal",
        "character-goal",
        "no-single-book",
        "template",
        "unreadable",
    ] {
        let mut args = vec![arg_token("reason", reason)];
        if reason != "unreadable" {
            args.push(arg_day("day", "2012-07-01"));
        }
        out.push((
            reason,
            metadata("scrivener-metadata-deadline-not-planned", args),
        ));
    }
    for unit in ["words", "characters", "pages"] {
        for calculated in [false, true] {
            out.push((
                if calculated { "calculated" } else { unit },
                metadata(
                    "scrivener-metadata-session-target-not-imported",
                    vec![
                        arg_count("value", 500),
                        arg_token("unit", unit),
                        arg_flag("calculated", calculated),
                    ],
                ),
            ));
        }
    }
    for day_off in [false, true] {
        out.push((
            if day_off { "day off" } else { "no day off" },
            metadata(
                "scrivener-metadata-pace-planned",
                vec![arg_day("day", "2099-12-01"), arg_flag("day_off", day_off)],
            ),
        ));
    }
    for field in [
        "label",
        "status",
        "keyword",
        "custom-field",
        "bookmark",
        "target",
    ] {
        out.push((
            field,
            metadata(
                "scrivener-metadata-values-without-row",
                vec![arg_token("field", field), arg_count("items", 3)],
            ),
        ));
    }
    for field in ["label", "status", "keyword"] {
        out.push((
            field,
            metadata(
                "scrivener-metadata-unnamed-value",
                vec![arg_token("field", field), arg_count("id", 7)],
            ),
        ));
    }
    out
}

/// Each variant of a metadata key is a sentence of its own, in both shipped locales: a
/// unit, a reason or a field the finding names is never said as another's.
#[test]
fn every_metadata_variant_says_its_own_sentence() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let mut said: std::collections::HashMap<String, Vec<String>> = Default::default();
            for (variant, found) in every_metadata_variant() {
                let key = key_of(&found).to_string();
                let line = summary_line(&key, &[&found])
                    .unwrap_or_else(|| panic!("{key} {variant} has no line"));
                assert_reads(locale, &key, &line);
                let text = line.resolve_now();
                let before = said.entry(key.clone()).or_default();
                assert!(
                    !before.contains(&text),
                    "{locale} {key} {variant}: said as another variant: {text}"
                );
                before.push(text);
            }
        });
    }
}

/// The notes of the "From Scrivener" folder are named as the import titles them, in the
/// writer's language, and a line about what one lists names that note and the folder.
#[test]
fn a_listed_line_names_the_note_and_the_folder_the_import_makes() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let folder = tr!(import_scrivener_from_scrivener()).resolve_now();
            for (section, found) in every_metadata_variant()
                .into_iter()
                .filter(|(_, f)| key_of(f) == "scrivener-metadata-listed-in-import-note")
            {
                let note = note_title(section)
                    .unwrap_or_else(|| panic!("{section} has no title"))
                    .resolve_now();
                let line = summary_line(key_of(&found), &[&found])
                    .expect("a line")
                    .resolve_now();
                assert!(line.contains(&note), "{locale} {section}: {line}");
                assert!(line.contains(&folder), "{locale} {section}: {line}");
            }
            assert!(note_title("a-later-note").is_none());
        });
    }
}

/// The way to the counting unit names the sections the Settings window shows.
#[test]
fn the_way_to_the_counting_unit_names_the_settings_sections() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let path = format!(
                "{} ▸ {} ▸ {}",
                tr!(settings_title()).resolve_now(),
                tr!(settings_sec_work()).resolve_now(),
                tr!(settings_page_structure()).resolve_now()
            );
            for count in [1_i64, 3] {
                let line = tr!(import_scrivener_metadata_goals_in_other_unit_line(
                    count = count
                ))
                .resolve_now();
                assert!(line.contains(&path), "{locale}: {line} does not say {path}");
            }
        });
    }
}

/// Lines about the links and the story bible use the names the app shows for them.
#[test]
fn the_links_and_the_story_bible_are_called_what_the_app_calls_them() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let bible = tr!(segment_story_bible()).resolve_now().to_lowercase();
            let links = tr!(note_details_links()).resolve_now();
            let cast = tr!(cast_section()).resolve_now();
            for count in [1_i64, 3] {
                let line = tr!(import_scrivener_metadata_bookmarks_became_links_line(
                    count = count
                ))
                .resolve_now();
                assert!(line.to_lowercase().contains(&bible), "{locale}: {line}");
                assert!(line.contains(&links), "{locale}: {line}");
                assert!(line.contains(&cast), "{locale}: {line}");
                let hint = tr!(import_scrivener_metadata_pov_hint_line(
                    count = count,
                    values = "Ysolt",
                    matches = "Ysolt Brenn"
                ))
                .resolve_now();
                assert!(hint.to_lowercase().contains(&bible), "{locale}: {hint}");
                assert!(
                    hint.contains(&tr!(inspector()).resolve_now()),
                    "{locale}: {hint}"
                );
            }
        });
    }
}

/// A finding that lists names gives them to the report, as the project wrote them; one
/// that lists none gives none.
#[test]
fn a_finding_lists_the_names_it_is_about() {
    let unused = metadata(
        "scrivener-metadata-unused-keywords-skipped",
        vec![arg_texts("names", &["Unused", "Places"])],
    );
    assert_eq!(listed_names(&unused), ["Unused", "Places"]);
    let unknown = metadata(
        "scrivener-metadata-undefined-custom-field",
        vec![arg_texts("ids", &["ghost"]), arg_count("values", 1)],
    );
    assert_eq!(listed_names(&unknown), ["ghost"]);
    let kept = metadata("scrivener-metadata-goals-kept", vec![arg_count("rows", 2)]);
    assert!(listed_names(&kept).is_empty());
}

// ── the two "again" buttons, and the sentences that send the writer to one ──────────────

/// Reading the project again and reviewing the import again are two actions: in both
/// locales neither label is the other or holds it, and every sentence that sends the writer
/// to one of the buttons quotes it as it is.
#[test]
fn the_two_again_buttons_are_told_apart_and_quoted_as_they_read() {
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let read = tr!(import_scrivener_check_again()).resolve_now();
            let review = tr!(import_scrivener_review_again()).resolve_now();
            assert!(
                !read.contains(&review) && !review.contains(&read),
                "{locale}: “{read}” and “{review}” must not read alike"
            );
            let failed = tr!(import_scrivener_read_failed_body()).resolve_now();
            assert!(failed.contains(&read), "{locale}: {failed}");
            let details = tr!(import_scrivener_details()).resolve_now();
            assert!(failed.contains(&details), "{locale}: {failed}");
            // Every failure the notice answers with the review, the refusals aside: those
            // are the Project page's own sentences, which quote no button.
            for reason in crate::shared::import_failure::every_reason() {
                if !reviewing_again_helps(&reason)
                    || matches!(reason, FailureReason::ScrivenerRefused { .. })
                {
                    continue;
                }
                let Some(said) = own_failure(&reason) else {
                    panic!("{locale}: {reason:?} is not the importer's own");
                };
                let said = said.resolve_now();
                assert!(said.contains(&review), "{locale}: {said}");
                assert!(!said.contains(&read), "{locale}: {said}");
            }
        });
    }
}

/// A missing or emptied writing is spoken of as what it is: an item's notes, its synopsis
/// or its text, each with its own pronouns, whether its words were found again or not.
#[test]
fn a_missing_writing_is_spoken_of_as_what_it_is() {
    let body = |key: &str, place: ScrivenerPlace, salvaged: bool| {
        let mut found = finding(key);
        if let ScrivenerFinding::Reported {
            place: at, args, ..
        } = &mut found
        {
            *at = place;
            args.retain(|a| !matches!(a, ScrivenerArg::Flag { name, .. } if name == "salvaged"));
            args.push(arg_flag("salvaged", salvaged));
        }
        notice(&found, &ScrivenerComputers::Unknown)
            .and_then(|w| w.body)
            .map(|b| b.resolve_now())
            .unwrap_or_default()
    };
    let cases = [
        ("scrivener-notice-content-missing", false),
        ("scrivener-notice-content-missing", true),
        ("scrivener-notice-content-truncated", false),
    ];
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            for (key, salvaged) in cases {
                let notes = body(key, ScrivenerPlace::Notes, salvaged);
                let text = body(key, ScrivenerPlace::Text, salvaged);
                assert!(!notes.is_empty() && !text.is_empty(), "{locale} {key}");
                assert_ne!(notes, text, "{locale} {key}: the notes are not the text");
            }
        });
    }
    crate::test_support::with_real_messages(|| {
        for (key, salvaged) in cases {
            let notes = body(key, ScrivenerPlace::Notes, salvaged);
            assert!(
                notes.contains("their file") || notes.starts_with("Their file"),
                "{key}: {notes}"
            );
            let synopsis = body(key, ScrivenerPlace::Synopsis, salvaged);
            assert!(synopsis.contains("its file") || synopsis.starts_with("Its file"));
        }
    });
}

/// A single file still in the cloud is named, not spoken of as several; several are given
/// as examples.
#[test]
fn a_lone_file_in_the_cloud_is_named_not_counted() {
    let body = |count: i64, examples: &[&str]| {
        said(&refusal_with(
            "scrivener-refusal-cloud-placeholders",
            vec![arg_count("count", count), arg_texts("examples", examples)],
        ))
        .1
    };
    for locale in LOCALES {
        crate::test_support::with_shipped_messages(locale, || {
            let one = body(1, &["Files/Data/A/content.rtf"]);
            let three = body(3, &["a.rtf", "b.rtf", "c.rtf"]);
            assert!(one.contains("Files/Data/A/content.rtf"), "{locale}: {one}");
            assert!(three.contains("a.rtf, b.rtf, c.rtf"), "{locale}: {three}");
            assert_ne!(
                one.replace("Files/Data/A/content.rtf", "X"),
                three.replace("a.rtf, b.rtf, c.rtf", "X")
            );
        });
    }
    crate::test_support::with_real_messages(|| {
        assert!(body(1, &["a.rtf"]).starts_with("It has not been downloaded"));
        assert!(body(2, &["a.rtf", "b.rtf"]).starts_with("They have not been downloaded"));
    });
}

/// A Trash with nothing written in it says so in words, not as a count of nothing.
#[test]
fn a_trash_with_no_writing_says_none() {
    let lines = |items: i64, writing: i64| {
        [
            tr!(import_scrivener_structure_trash_brought_across_line(
                items = items,
                writing = writing
            ))
            .resolve_now(),
            tr!(import_scrivener_structure_trash_left_out_line(
                items = items,
                writing = writing
            ))
            .resolve_now(),
        ]
    };
    crate::test_support::with_real_messages(|| {
        for line in lines(5, 0) {
            assert!(
                line.contains("5 items, none of them with writing"),
                "{line}"
            );
        }
        for line in lines(5, 1) {
            assert!(line.contains("5 items, 1 of them with writing"), "{line}");
        }
        for line in lines(1, 1) {
            assert!(line.contains(": 1 item"), "{line}");
        }
    });
    crate::test_support::with_shipped_messages("fr-FR", || {
        for line in lines(5, 0) {
            assert!(line.contains("dont aucun avec du texte"), "{line}");
        }
        for line in lines(5, 2) {
            assert!(line.contains("dont 2 avec du texte"), "{line}");
        }
    });
}
