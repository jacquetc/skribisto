// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every importable project of the real Scrivener corpus through the two use cases, as the
//! wizard drives them: analysed through the controller and the operation manager's JSON,
//! the plan recomputed by the review from what the analysis handed over, imported with the
//! analysis's fingerprint and choices, and every finding's row found in what was written.
//!
//! Opt-in, like the converter's own corpus tests (`scrivener_import/tests/all/corpus.rs`,
//! which say how to harvest the corpus): the projects are unlicensed and never enter the
//! repository.
//!
//! ```text
//! SKRIBISTO_SCRIVENER_CORPUS=~/scrivener-corpus \
//!     cargo test --release -p skribisto-import-management --test scrivener_corpus \
//!     -- --ignored --nocapture
//! ```
//!
//! It prints counts only: never a title, a path or a word of prose.
//!
//! What the review recomputes the plan from is checked to be exactly what a fresh read of
//! the project makes: the mapping is one pure function of the two, so the review and the
//! import then agree on every combination of choices, not only the ones tried here.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use common::database::db_context::DbContext;
use common::entities::StatusCategory;
use common::event::EventHub;
use common::long_operation::{LongOperationManager, OperationStatus};
use import_management::import_management_controller as controller;
use import_management::scrivener::{ScrivenerReview, TITLE_SLOT, every_key};
use import_management::{
    AnalyzeScrivenerProjectDto, ImportScrivenerProjectDto, ScrivenerAnalysisDto,
    ScrivenerAnalysisOutcome, ScrivenerDefaultRung, ScrivenerFinding, ScrivenerFindingFamily,
    ScrivenerPlace, ScrivenerStructure,
};

/// The projects whose analysis and import times are printed on their own: the largest.
const TIMED: &[&str] = &["m2-buhler-dissertation", "ll-manual-source"];

/// What an analysis hands the review, read back as the review reads it: the project and
/// what each of its documents holds.
#[derive(serde::Deserialize)]
struct ReviewData {
    project: scrivener_import::model::Project,
    content: scrivener_import::ContentFacts,
}

fn lock_path() -> PathBuf {
    match std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS_LOCK") {
        Some(path) => PathBuf::from(path),
        None => Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scripts/scrivener_corpus/corpus.lock.json"),
    }
}

/// Run `op` to its end; how it ended.
fn finish(long_ops: &LongOperationManager, op: &str) -> OperationStatus {
    assert!(
        long_ops.wait_for_operation(op, Some(Duration::from_secs(600))),
        "an operation never finished"
    );
    long_ops
        .get_operation_status(op)
        .expect("the operation has a status")
}

fn import_request(
    source: &Path,
    analysis: &ScrivenerAnalysisDto,
    output: &Path,
) -> ImportScrivenerProjectDto {
    let ScrivenerStructure::Planned { choices, .. } = &analysis.structure else {
        panic!("a read project always has a plan");
    };
    let rung = |name: &str, category| ScrivenerDefaultRung::Rung {
        name: name.to_string(),
        category,
    };
    ImportScrivenerProjectDto {
        source_path: source.to_string_lossy().into_owned(),
        fingerprint: analysis.fingerprint.clone(),
        choices: choices.clone(),
        blank_lines_as_scene_breaks: true,
        output_path: output.to_string_lossy().into_owned(),
        overwrite: true,
        title: Some(analysis.proposed_title.clone()),
        author: Some(analysis.proposed_authors.join(", ")),
        language: Some(if analysis.proposed_language.is_empty() {
            "en-US".to_string()
        } else {
            analysis.proposed_language.clone()
        }),
        copy_research_files: true,
        manuscript_binder_name: "Manuscript".into(),
        notes_binder_name: "Notes".into(),
        notes_of_pattern: format!("{TITLE_SLOT} (notes)"),
        project_notes_name: "Project notes".into(),
        recovered_name: "Recovered".into(),
        untitled_template_name: "Template".into(),
        research_folder_pattern: format!("{TITLE_SLOT} research files"),
        list_separator: ", ".into(),
        label_details_pattern: format!("{TITLE_SLOT} in Scrivener"),
        status_details_pattern: format!("{TITLE_SLOT} in Scrivener"),
        unnamed_label_pattern: format!("Label {TITLE_SLOT}"),
        unnamed_status_pattern: format!("Status {TITLE_SLOT}"),
        label_tag_pattern: format!("{TITLE_SLOT} (label)"),
        untitled_tag_name: "Untitled".into(),
        default_statuses: vec![
            rung("Idea", StatusCategory::Planned),
            rung("Draft", StatusCategory::Drafting),
            rung("Done", StatusCategory::Final),
        ],
        ..ImportScrivenerProjectDto::default()
    }
}

#[test]
#[ignore = "needs SKRIBISTO_SCRIVENER_CORPUS, a harvest of scripts/scrivener_corpus/corpus.lock.json"]
fn corpus_every_project_goes_through_both_use_cases() {
    let Some(corpus) = std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS").map(PathBuf::from) else {
        eprintln!("SKRIBISTO_SCRIVENER_CORPUS unset: skipping the real-project run.");
        return;
    };
    let lock: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(lock_path()).expect("the corpus lock reads"))
            .expect("the corpus lock is JSON");
    let scratch = tempfile::tempdir_in(env!("CARGO_TARGET_TMPDIR")).expect("a scratch folder");
    let db = DbContext::new().expect("in-memory store");
    let hub = Arc::new(EventHub::new());
    let mut long_ops = LongOperationManager::new();
    let keys: BTreeSet<&str> = every_key().into_iter().collect();

    let (mut analysed, mut pending, mut imported) = (0usize, 0usize, 0usize);
    let (mut findings, mut located, mut documents) = (0usize, 0usize, 0usize);
    let mut largest_review = 0usize;
    let mut by_family: BTreeMap<String, usize> = BTreeMap::new();
    let mut failures: Vec<String> = Vec::new();
    let (mut analysing, mut importing) = (Duration::ZERO, Duration::ZERO);

    let entries = lock["entries"].as_array().cloned().unwrap_or_default();
    // A manifest picked on its own is imported only where its folder is not already: the
    // same projects the converter's own corpus run converts.
    let imported_folders: Vec<String> = entries
        .iter()
        .flat_map(|e| e["roots"].as_array().cloned().unwrap_or_default())
        .filter(|r| r["pick"].as_str() == Some("folder") && r["expect"].as_str() == Some("import"))
        .filter_map(|r| r["path"].as_str().map(str::to_string))
        .collect();
    for entry in &entries {
        let id = entry["id"].as_str().unwrap_or_default().to_string();
        for (k, root) in entry["roots"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .enumerate()
        {
            let pick = root["pick"].as_str().unwrap_or_default();
            let rel = root["path"].as_str().unwrap_or_default();
            let folder_of = rel.rsplit_once('/').map_or("", |(folder, _)| folder);
            let wanted = match pick {
                "folder" | "archive" => true,
                "manifest" => !imported_folders.iter().any(|f| f == folder_of),
                _ => false,
            };
            if !wanted || root["expect"].as_str() != Some("import") {
                continue;
            }
            let source = corpus.join(rel);
            if !source.exists() {
                continue;
            }
            let label = format!("{id}#{k}");

            let started = Instant::now();
            let op = controller::analyze_scrivener_project(
                &db,
                &hub,
                &mut long_ops,
                &AnalyzeScrivenerProjectDto {
                    source_path: source.to_string_lossy().into_owned(),
                    ..AnalyzeScrivenerProjectDto::default()
                },
            )
            .expect("started");
            if finish(&long_ops, &op) != OperationStatus::Completed {
                failures.push(format!("{label}: the analysis failed"));
                continue;
            }
            let analysis = controller::get_analyze_scrivener_project_result(&long_ops, &op)
                .expect("the analysis reads back")
                .expect("and there is one");
            let analysed_in = started.elapsed();
            analysing += analysed_in;
            analysed += 1;
            if analysis.outcome != ScrivenerAnalysisOutcome::Read {
                failures.push(format!("{label}: not read ({:?})", analysis.outcome));
                continue;
            }
            if !analysis.pending.is_empty() {
                pending += 1;
                continue;
            }
            largest_review = largest_review.max(analysis.review_data.len());
            documents += usize::try_from(analysis.documents).unwrap_or(0);
            // The review plans from exactly what a fresh read of the project makes.
            let data: ReviewData = match serde_json::from_str(&analysis.review_data) {
                Ok(data) => data,
                Err(e) => {
                    failures.push(format!("{label}: the review data does not read: {e}"));
                    continue;
                }
            };
            match scrivener_import::analyze(
                &source,
                &scrivener_import::Answers::default(),
                &scrivener_import::Options::default(),
            ) {
                scrivener_import::Analysis::Read(read)
                    if read.project == data.project && read.content == data.content => {}
                _ => failures.push(format!(
                    "{label}: the review plans from something a fresh read does not make"
                )),
            }
            let ScrivenerStructure::Planned { choices, .. } = &analysis.structure else {
                failures.push(format!("{label}: no plan"));
                continue;
            };
            let review = match ScrivenerReview::open(&analysis.review_data) {
                Ok(review) => review,
                Err(e) => {
                    failures.push(format!("{label}: the review does not open: {e}"));
                    continue;
                }
            };
            if review.structure(choices).as_ref() != Ok(&analysis.structure) {
                failures.push(format!(
                    "{label}: the review's plan differs from the analysis's"
                ));
            }

            let output = scratch.path().join(format!("{imported}.skrib"));
            let started = Instant::now();
            let op = controller::import_scrivener_project(
                &db,
                &hub,
                &mut long_ops,
                &import_request(&source, &analysis, &output),
            )
            .expect("started");
            match finish(&long_ops, &op) {
                OperationStatus::Completed => {}
                other => {
                    failures.push(format!("{label}: the import ended {other:?}"));
                    continue;
                }
            }
            let result = controller::get_import_scrivener_project_result(&long_ops, &op)
                .expect("the result reads back")
                .expect("and there is one");
            let took = started.elapsed();
            importing += took;
            imported += 1;
            if TIMED.contains(&id.as_str()) {
                println!(
                    "  {label:<28} review data {:5} KiB, analysed in {:.2} s, imported in {:.2} s",
                    analysis.review_data.len() / 1024,
                    analysed_in.as_secs_f64(),
                    took.as_secs_f64(),
                );
            }

            let bundle = match skrib_format::read_bundle(&result.output_path) {
                Ok(bundle) => bundle,
                Err(e) => {
                    failures.push(format!("{label}: does not read back: {e:#}"));
                    continue;
                }
            };
            let uids: BTreeSet<uuid::Uuid> = bundle
                .binders
                .iter()
                .flat_map(|b| b.items.iter().map(|i| i.item.uid))
                .collect();
            for finding in &result.findings {
                let ScrivenerFinding::Reported {
                    key,
                    family,
                    place,
                    binder_path,
                    item_uid,
                    ..
                } = finding
                else {
                    failures.push(format!("{label}: an empty finding"));
                    continue;
                };
                findings += 1;
                // A recovered text is named as its row is titled, never by its file's name,
                // which a sync service writes a person's or a computer's name into.
                if *place == ScrivenerPlace::Recovered
                    && binder_path.iter().any(|part| {
                        let part = part.to_lowercase();
                        [".rtf", ".rtfd", ".txt"]
                            .iter()
                            .any(|ext| part.ends_with(ext))
                    })
                {
                    failures.push(format!("{label}: {key} names a recovered text by its file"));
                }
                *by_family.entry(format!("{family:?}")).or_default() += 1;
                if *family != ScrivenerFindingFamily::Document && !keys.contains(key.as_str()) {
                    failures.push(format!("{label}: an unlisted key {key}"));
                }
                if let Some(uid) = item_uid {
                    located += 1;
                    if !uids.contains(uid) {
                        failures.push(format!("{label}: {key} names a row not written"));
                    }
                }
            }
            // The research folder beside it goes with the scratch folder.
            let _ = std::fs::remove_file(&output);
        }
    }

    println!(
        "use cases: {analysed} analysed, {pending} awaiting an answer, {imported} imported; \
         {documents} documents; {findings} findings, {located} naming a written row; \
         by family {by_family:?}; largest review data {} KiB; analysing {:.1} s, \
         importing {:.1} s",
        largest_review / 1024,
        analysing.as_secs_f64(),
        importing.as_secs_f64(),
    );
    assert!(failures.is_empty(), "{failures:#?}");
    assert!(
        imported > 0,
        "nothing was imported: is the corpus harvested?"
    );
}
