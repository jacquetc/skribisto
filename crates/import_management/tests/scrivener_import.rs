// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two Scrivener use cases, end to end through the controller and a real
//! `LongOperationManager`: a project of each generation analysed, imported with the
//! analysis's fingerprint and the writer's words, and the result opened by the ordinary
//! `load_work`. What each returns crosses the manager's JSON round trip typed and whole;
//! failure and progress cross as strings, read back as the interface reads them.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use common::database::db_context::DbContext;
use common::entities::StatusCategory;
use common::event::EventHub;
use common::long_operation::{LongOperationManager, OperationStatus};
use import_management::failure::{FailureReason, ProjectImportFailure};
use import_management::import_management_controller as controller;
use import_management::scrivener::{ScrivenerReview, TITLE_SLOT, every_key};
use import_management::{
    AnalyzeScrivenerProjectDto, ImportScrivenerProjectDto, ImportScrivenerProjectResultDto,
    ScrivenerAnalysisDto, ScrivenerAnalysisOutcome, ScrivenerArg, ScrivenerComputers,
    ScrivenerDefaultRung, ScrivenerFinding, ScrivenerFindingFamily, ScrivenerGeneration,
    ScrivenerPlace, ScrivenerPlatform, ScrivenerQuestion, ScrivenerStructure, ScrivenerTotals,
};
use scrivener_fixtures::{Generation, Item, Kind, Platform, Spec};

struct Backend {
    db: DbContext,
    hub: Arc<EventHub>,
    long_ops: LongOperationManager,
}

impl Backend {
    fn new() -> Self {
        Self {
            db: DbContext::new().expect("in-memory store"),
            hub: Arc::new(EventHub::new()),
            long_ops: LongOperationManager::new(),
        }
    }

    /// Wait for `op` and return how it ended.
    fn finish(&self, op: &str) -> OperationStatus {
        assert!(
            self.long_ops
                .wait_for_operation(op, Some(Duration::from_secs(120))),
            "the operation never finished"
        );
        self.long_ops
            .get_operation_status(op)
            .expect("the operation has a status")
    }

    /// The phase code the operation last reported.
    fn last_phase(&self, op: &str) -> Option<String> {
        self.long_ops.get_operation_progress(op)?.message
    }

    fn analyse(&mut self, request: &AnalyzeScrivenerProjectDto) -> ScrivenerAnalysisDto {
        let op =
            controller::analyze_scrivener_project(&self.db, &self.hub, &mut self.long_ops, request)
                .expect("started");
        assert_eq!(self.finish(&op), OperationStatus::Completed);
        assert_eq!(self.last_phase(&op).as_deref(), Some("import.done"));
        controller::get_analyze_scrivener_project_result(&self.long_ops, &op)
            .expect("the analysis reads back")
            .expect("and there is one")
    }

    /// Start an import and return how it ended.
    fn import(&mut self, request: &ImportScrivenerProjectDto) -> (String, OperationStatus) {
        let op =
            controller::import_scrivener_project(&self.db, &self.hub, &mut self.long_ops, request)
                .expect("started");
        let status = self.finish(&op);
        (op, status)
    }

    fn imported(&mut self, request: &ImportScrivenerProjectDto) -> ImportScrivenerProjectResultDto {
        let (op, status) = self.import(request);
        assert_eq!(status, OperationStatus::Completed, "{status:?}");
        assert_eq!(self.last_phase(&op).as_deref(), Some("import.done"));
        controller::get_import_scrivener_project_result(&self.long_ops, &op)
            .expect("the result reads back")
            .expect("and there is one")
    }

    /// Why an import failed, as the interface reads it.
    fn failed(&mut self, request: &ImportScrivenerProjectDto) -> FailureReason {
        match self.import(request).1 {
            OperationStatus::Failed(message) => {
                ProjectImportFailure::from_any_message(&message).reason
            }
            other => panic!("expected the import to fail, it ended {other:?}"),
        }
    }
}

fn write(spec: &Spec, parent: &Path) -> PathBuf {
    let folder = parent.join(spec.folder());
    spec.build().write_to(&folder).expect("the fixture writes");
    folder
}

/// The writer's calendar day every request of these tests is made on, at midnight UTC as
/// the interface sends it: the analysis and the review plan on the same day.
fn today() -> chrono::DateTime<chrono::Utc> {
    chrono::NaiveDate::from_ymd_opt(2026, 10, 5)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc())
        .unwrap_or_default()
}

fn analysis_request(source: &Path) -> AnalyzeScrivenerProjectDto {
    AnalyzeScrivenerProjectDto {
        source_path: source.to_string_lossy().into_owned(),
        today: today(),
        ..AnalyzeScrivenerProjectDto::default()
    }
}

/// The words a French interface resolves, every one of them.
fn french(request: ImportScrivenerProjectDto) -> ImportScrivenerProjectDto {
    let rung = |name: &str, category| ScrivenerDefaultRung::Rung {
        name: name.to_string(),
        category,
    };
    ImportScrivenerProjectDto {
        manuscript_binder_name: "Manuscrit".into(),
        notes_binder_name: "Notes et documentation".into(),
        notes_of_pattern: format!("Notes de « {TITLE_SLOT} »"),
        project_notes_name: "Notes du projet".into(),
        recovered_name: "Textes retrouvés".into(),
        untitled_template_name: "Modèle sans titre".into(),
        research_folder_pattern: format!("{TITLE_SLOT} - documentation"),
        list_separator: " et ".into(),
        label_details_pattern: format!("Venu de Scrivener, champ « {TITLE_SLOT} »"),
        status_details_pattern: format!("Venu de Scrivener, champ « {TITLE_SLOT} »"),
        unnamed_label_pattern: format!("Étiquette {TITLE_SLOT}"),
        unnamed_status_pattern: format!("Statut {TITLE_SLOT}"),
        label_tag_pattern: format!("{TITLE_SLOT} (étiquette)"),
        untitled_tag_name: "Sans titre".into(),
        default_statuses: vec![
            rung("À écrire", StatusCategory::Planned),
            rung("Premier jet", StatusCategory::Drafting),
            rung("À reprendre", StatusCategory::NeedsWork),
            rung("Relu", StatusCategory::Revised),
            rung("Terminé", StatusCategory::Final),
        ],
        metadata_words: french_metadata_words(),
        ..request
    }
}

/// The second half's words, in French, one per key in the converter's order.
fn french_metadata_words() -> Vec<String> {
    use import_management::scrivener::{METADATA_WORD_ORDER, VALUE_SLOT};
    METADATA_WORD_ORDER
        .iter()
        .map(|key| match *key {
            "keyword-details" => format!("Le mot-clé « {TITLE_SLOT} » de Scrivener"),
            "unnamed-keyword" => format!("Mot-clé {TITLE_SLOT}"),
            "keyword-tag" => format!("{TITLE_SLOT} (mot-clé)"),
            "collection-details" => "Une collection de Scrivener".to_string(),
            "collection-tag" => format!("{TITLE_SLOT} (collection)"),
            "field-details" => format!("Du champ « {TITLE_SLOT} » de Scrivener"),
            "field-value" => format!("{TITLE_SLOT} : {VALUE_SLOT}"),
            "field-tag" => format!("{TITLE_SLOT} (champ)"),
            "from-scrivener" => "Venu de Scrivener".to_string(),
            other => format!("Mot français {other}"),
        })
        .collect()
}

/// The import of what `analysis` read, with its fingerprint and its detected choices, into
/// `output`, in French.
fn import_request(
    source: &Path,
    analysis: &ScrivenerAnalysisDto,
    output: &Path,
) -> ImportScrivenerProjectDto {
    let ScrivenerStructure::Planned { choices, .. } = &analysis.structure else {
        panic!("the analysis planned no structure");
    };
    french(ImportScrivenerProjectDto {
        source_path: source.to_string_lossy().into_owned(),
        today: today(),
        fingerprint: analysis.fingerprint.clone(),
        choices: choices.clone(),
        blank_lines_as_scene_breaks: true,
        ornaments_as_scene_breaks: false,
        output_path: output.to_string_lossy().into_owned(),
        overwrite: false,
        title: Some(analysis.proposed_title.clone()),
        author: Some(analysis.proposed_authors.join(" et ")),
        language: Some("fr-FR".into()),
        copy_research_files: true,
        ..ImportScrivenerProjectDto::default()
    })
}

/// Open `output` the ordinary way, and return the uid of every row it holds.
fn open(output: &Path, media: &Path) -> BTreeSet<uuid::Uuid> {
    use work_management::{LoadWorkDto, work_management_controller as work};
    let db = DbContext::new().expect("in-memory store");
    let hub = Arc::new(EventHub::new());
    if let Err(e) = work::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            file_name: output.to_string_lossy().into_owned(),
            media_root: media.to_string_lossy().into_owned(),
        },
    ) {
        panic!("load_work refuses the imported project: {e:#}");
    }
    db.get_store()
        .binder_items
        .read()
        .expect("the store reads")
        .values()
        .map(|item| item.uid)
        .collect()
}

fn reported(finding: &ScrivenerFinding) -> (&str, &ScrivenerFindingFamily, &ScrivenerPlace) {
    match finding {
        ScrivenerFinding::Reported {
            key, family, place, ..
        } => (key, family, place),
        ScrivenerFinding::Empty => panic!("an empty finding was sent"),
    }
}

/// Analysis, then import with the analysis's fingerprint, then `load_work`: for a project
/// of each desktop generation, and one made on an iPad.
#[test]
fn a_project_is_analysed_imported_and_opened() {
    for (generation, platform, expected) in [
        (
            Generation::Format20,
            Platform::Mac,
            ScrivenerGeneration::Format20,
        ),
        (
            Generation::Format15,
            Platform::Windows,
            ScrivenerGeneration::Format15,
        ),
        (
            Generation::Format10,
            Platform::Linux,
            ScrivenerGeneration::Format10,
        ),
        (
            Generation::IosStub,
            Platform::Ios,
            ScrivenerGeneration::IosStub,
        ),
    ] {
        let dir = tempfile::tempdir().expect("tempdir");
        let spec = Spec::novel(generation, platform).with_metadata();
        let source = write(&spec, dir.path());
        let mut backend = Backend::new();

        let analysis = backend.analyse(&analysis_request(&source));
        assert_eq!(
            analysis.outcome,
            ScrivenerAnalysisOutcome::Read,
            "{generation:?}"
        );
        assert_eq!(analysis.generation, expected);
        assert!(!analysis.fingerprint.is_empty());
        assert!(analysis.pending.is_empty(), "{:?}", analysis.pending);
        assert!(analysis.documents > 0 && analysis.manuscript_documents > 0);
        assert!(analysis.manuscript_documents <= analysis.documents);
        if generation != Generation::IosStub {
            assert_ne!(
                analysis.platform,
                ScrivenerPlatform::Unknown,
                "{generation:?}"
            );
        }
        // The Save page's proposals: the title the project states (an iPad keeps no
        // compile settings, so there it is the folder's).
        let title = match generation {
            Generation::IosStub => "Lighthouse",
            _ => "The Lighthouse Keepers",
        };
        assert_eq!(analysis.proposed_title, title, "{generation:?}");
        // Every notice is keyed, and each key is one the interface has a sentence for.
        for notice in &analysis.notices {
            let (key, family, _) = reported(notice);
            assert_eq!(family, &ScrivenerFindingFamily::Source);
            assert!(every_key().contains(&key), "{key}");
        }
        // The review recomputes exactly the plan the analysis made, from what it handed
        // over, with the choices the plan reports.
        let ScrivenerStructure::Planned { rows, choices, .. } = &analysis.structure else {
            panic!("no structure planned for {generation:?}");
        };
        assert!(!rows.is_empty());
        let review = ScrivenerReview::open(&analysis.review_data).expect("the review opens");
        let day = today().date_naive();
        assert_eq!(review.structure(&[], day).as_ref(), Ok(&analysis.structure));
        assert_eq!(
            review.structure(choices, day).as_ref(),
            Ok(&analysis.structure)
        );

        let output = dir.path().join("novel.skrib");
        let result = backend.imported(&import_request(&source, &analysis, &output));
        assert_eq!(result.output_path, output.to_string_lossy());
        assert!(output.is_file());
        let ScrivenerTotals::Counted { rows, words, .. } = result.totals else {
            panic!("no totals");
        };
        assert!(rows > 0 && words > 0, "{generation:?}");

        // Every row a finding names is a row of the project `load_work` opens.
        let uids = open(&output, &dir.path().join("media"));
        assert_eq!(uids.len() as i64, rows);
        let mut named = 0;
        for finding in &result.findings {
            let ScrivenerFinding::Reported {
                key,
                family,
                item_uid,
                ..
            } = finding
            else {
                panic!("an empty finding was sent");
            };
            if *family != ScrivenerFindingFamily::Document {
                assert!(every_key().contains(&key.as_str()), "{key}");
            }
            if let Some(uid) = item_uid {
                named += 1;
                assert!(uids.contains(uid), "{key} names a row the project lacks");
            }
        }
        assert!(named > 0, "no finding names a row for {generation:?}");
    }
}

/// A project changed after its review is refused before anything is written, with its own
/// reason, for the interface to offer the review again.
#[test]
fn a_project_changed_since_the_review_is_refused_for_review_again() {
    let dir = tempfile::tempdir().expect("tempdir");
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let source = write(&spec, dir.path());
    let mut backend = Backend::new();
    let analysis = backend.analyse(&analysis_request(&source));

    // The writer goes back to Scrivener and types a word into a scene.
    let index = spec.index_of("Arrival").expect("the scene");
    let text = source.join(spec.text_path(index));
    let mut rtf = std::fs::read(&text).expect("the scene reads");
    let at = rtf.iter().rposition(|b| *b == b'}').expect("an RTF group");
    rtf.splice(at..at, b"\\par One more line.".iter().copied());
    std::fs::write(&text, rtf).expect("the scene writes");

    let output = dir.path().join("novel.skrib");
    assert_eq!(
        backend.failed(&import_request(&source, &analysis, &output)),
        FailureReason::ScrivenerChangedSinceReview
    );
    assert!(!output.exists());

    // Analysed again, it imports.
    let again = backend.analyse(&analysis_request(&source));
    assert_ne!(again.fingerprint, analysis.fingerprint);
    backend.imported(&import_request(&source, &again, &output));
}

/// A project that cannot be imported is an answer of the analysis, and the same refusal,
/// keyed and typed alike, is the import's failure.
#[test]
fn a_refusal_is_the_analysis_s_answer_and_the_import_s_failure() {
    let dir = tempfile::tempdir().expect("tempdir");
    let missing = dir.path().join("Gone.scriv");
    let mut backend = Backend::new();
    let analysis = backend.analyse(&analysis_request(&missing));
    assert_eq!(analysis.outcome, ScrivenerAnalysisOutcome::Refused);
    let ScrivenerFinding::Reported {
        key, family, args, ..
    } = &analysis.refusal
    else {
        panic!("no refusal");
    };
    assert_eq!(key, "scrivener-refusal-missing");
    assert_eq!(family, &ScrivenerFindingFamily::Refusal);
    assert_eq!(
        args,
        &vec![ScrivenerArg::Text {
            name: "file".into(),
            value: "Gone.scriv".into()
        }]
    );
    assert!(matches!(analysis.structure, ScrivenerStructure::Empty));
    assert!(analysis.review_data.is_empty() && analysis.fingerprint.is_empty());

    let request = french(ImportScrivenerProjectDto {
        source_path: missing.to_string_lossy().into_owned(),
        output_path: dir.path().join("out.skrib").to_string_lossy().into_owned(),
        ..ImportScrivenerProjectDto::default()
    });
    assert_eq!(
        backend.failed(&request),
        FailureReason::ScrivenerRefused {
            refusal: analysis.refusal.clone()
        }
    );
}

/// A folder holding two projects asks which; the answer reads the one chosen, and the
/// import given the same answer converts it.
#[test]
fn a_question_is_asked_then_answered() {
    let dir = tempfile::tempdir().expect("tempdir");
    let shelf = dir.path().join("Shelf");
    let first = Spec::novel(Generation::Format20, Platform::Mac);
    let second = Spec {
        name: "Harbour".to_string(),
        ..Spec::novel(Generation::Format15, Platform::Windows)
    };
    write(&first, &shelf);
    write(&second, &shelf);
    let mut backend = Backend::new();

    let asked = backend.analyse(&analysis_request(&shelf));
    assert_eq!(asked.outcome, ScrivenerAnalysisOutcome::Choose);
    let ScrivenerQuestion::SeveralProjects { candidates } = &asked.question else {
        panic!(
            "expected the pick to ask which project, got {:?}",
            asked.question
        );
    };
    assert_eq!(asked.question.key(), "scrivener-question-several-projects");
    let chosen = candidates
        .iter()
        .find(|c| c.contains("Harbour"))
        .expect("the second project is a candidate")
        .clone();

    let request = AnalyzeScrivenerProjectDto {
        chosen_project: chosen.clone(),
        ..analysis_request(&shelf)
    };
    let answered = backend.analyse(&request);
    assert_eq!(answered.outcome, ScrivenerAnalysisOutcome::Read);
    assert_eq!(answered.generation, ScrivenerGeneration::Format15);

    let output = dir.path().join("harbour.skrib");
    let mut import = import_request(&shelf, &answered, &output);
    // Without the answer, the import asks again rather than guessing.
    assert!(matches!(
        backend.failed(&import),
        FailureReason::ScrivenerQuestionFirst {
            question: ScrivenerQuestion::SeveralProjects { .. }
        }
    ));
    import.chosen_project = chosen;
    backend.imported(&import);
    assert!(output.is_file());
}

/// The computers a project names reach the card, and nowhere else: not a finding, not the
/// imported project.
#[test]
fn the_lock_s_host_is_shown_and_never_kept() {
    let dir = tempfile::tempdir().expect("tempdir");
    let spec = Spec {
        lock_host: Some("writers-desk.local".to_string()),
        ..Spec::novel(Generation::Format20, Platform::Mac)
    };
    let source = write(&spec, dir.path());
    let mut backend = Backend::new();
    let analysis = backend.analyse(&analysis_request(&source));
    let ScrivenerComputers::Named { lock_host, .. } = &analysis.computers else {
        panic!("the lock's host is not on the card");
    };
    assert_eq!(lock_host.as_deref(), Some("writers-desk.local"));
    let mut notices = analysis.notices.clone();
    notices.push(analysis.refusal.clone());
    let spelled = serde_json::to_string(&notices).expect("serialises");
    assert!(!spelled.contains("writers-desk"));
    // What the review recomputes the plan from holds the project, never its computers.
    assert!(!analysis.review_data.contains("writers-desk"));
    if let ScrivenerComputers::Named {
        device: Some(device),
        ..
    } = &analysis.computers
    {
        assert!(!analysis.review_data.contains(device.as_str()));
        assert!(!spelled.contains(device.as_str()));
    }

    let output = dir.path().join("novel.skrib");
    let result = backend.imported(&import_request(&source, &analysis, &output));
    assert!(
        !serde_json::to_string(&result)
            .expect("serialises")
            .contains("writers-desk")
    );
    let written = every_text(&output);
    for computer in ["writers-desk", "Test-Mac"] {
        assert!(
            !written.iter().any(|text| text.contains(computer)),
            "{computer} was written into the project"
        );
    }
}

/// A sync service's copy of a document whose item is gone is imported as a recovered text,
/// and the report names it as its row is titled: by the document number its file's name
/// gives, never by the name itself, which carries whoever's copy it was.
#[test]
fn a_sync_copy_s_file_name_names_no_finding() {
    for generation in [Generation::Format20, Generation::Format15] {
        let dir = tempfile::tempdir().expect("tempdir");
        let spec = Spec::novel(generation, Platform::Mac);
        let mut files = spec.build();
        // Red words and a typed indent, so its conversion has something to report.
        let rtf = "{\\rtf1\\ansi{\\colortbl;\\red255\\green0\\blue0;}\\cf1 Red words.\\par\\tab Indented.\\par}";
        let (copy, number) = match generation {
            Generation::Format20 => (
                format!(
                    "Files/Data/{}/content (Jane Doe's conflicted copy 2019-03-04).rtf",
                    scrivener_fixtures::project::uuid_for(spec.seed, 900)
                ),
                None,
            ),
            _ => (
                "Files/Docs/900 (Jane Doe's conflicted copy 2019-03-04).rtf".to_string(),
                Some("900"),
            ),
        };
        files.put(copy, rtf);
        let source = dir.path().join(spec.folder());
        files.write_to(&source).expect("the fixture writes");
        let mut backend = Backend::new();
        let analysis = backend.analyse(&analysis_request(&source));
        let output = dir.path().join("novel.skrib");
        let result = backend.imported(&import_request(&source, &analysis, &output));

        let recovered: Vec<&ScrivenerFinding> = result
            .findings
            .iter()
            .filter(|f| {
                matches!(
                    f,
                    ScrivenerFinding::Reported {
                        place: ScrivenerPlace::Recovered,
                        ..
                    }
                )
            })
            .collect();
        assert!(!recovered.is_empty(), "{generation:?}: nothing reported");
        for finding in recovered {
            let ScrivenerFinding::Reported {
                binder_path,
                item_uid,
                ..
            } = finding
            else {
                unreachable!();
            };
            let expected: Vec<String> = number.iter().map(|n| n.to_string()).collect();
            assert_eq!(binder_path, &expected, "{generation:?}");
            assert!(item_uid.is_some(), "{generation:?}: the row holding it");
        }
        let spelled = serde_json::to_string(&result).expect("serialises");
        assert!(!spelled.contains("Jane"), "{generation:?}: {spelled}");
        assert!(
            !every_text(&output).iter().any(|t| t.contains("Jane")),
            "{generation:?}: the name was written into the project"
        );
    }
}

/// A one-document manuscript is split at its chapter headings into one row per chapter, and
/// what converting the document found is still one row per kind for that document: the
/// pictures of every chapter that have no description are one finding, counted together.
#[test]
fn a_split_document_s_findings_are_one_row_per_kind() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.items = vec![
        Item::new(0, Kind::Draft, "Manuscript").compiled(),
        Item::new(1, Kind::Text, "Whole book")
            .with_text(3, 1)
            .compiled(),
        Item::new(0, Kind::Research, "Research"),
        Item::new(0, Kind::Trash, "Trash"),
    ];
    spec.template_folder = None;
    let mut files = spec.build();
    let picture = || -> String {
        let hex: String = scrivener_fixtures::TINY_PNG
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        format!("{{\\*\\shppict{{\\pict\\picw1\\pich1\\picwgoal300\\pichgoal300\\pngblip {hex}}}}}")
    };
    let rtf = [
        "{\\rtf1\\ansi\\ansicpg1252\\cocoartf2761\n{\\fonttbl\\f0\\fswiss\\fcharset0 Helvetica;}\n\\pard\\f0\\fs24 Chapter One\\\nIt began at the harbour. ".to_string(),
        picture(),
        "\\\nChapter Two\\\nIt went on at sea. ".to_string(),
        picture(),
        "\\\nChapter Three\\\nIt ended at the lighthouse.}".to_string(),
    ]
    .concat();
    let index = spec.index_of("Whole book").expect("the document");
    files.put(spec.text_path(index), rtf);
    let source = dir.path().join(spec.folder());
    files.write_to(&source).expect("the fixture writes");

    let mut backend = Backend::new();
    let analysis = backend.analyse(&analysis_request(&source));
    let output = dir.path().join("book.skrib");
    let result = backend.imported(&import_request(&source, &analysis, &output));
    let chapters = skrib_format::read_bundle(&output.to_string_lossy())
        .expect("the project reads back")
        .binders
        .iter()
        .flat_map(|b| b.items.iter())
        .filter(|i| i.item.title.starts_with("Chapter"))
        .count();
    assert_eq!(chapters, 3, "the document was split at its headings");

    let undescribed: Vec<&ScrivenerFinding> = result
        .findings
        .iter()
        .filter(|f| {
            matches!(f, ScrivenerFinding::Reported { key, .. }
                if key == "scrivener-prose-pictures-without-description")
        })
        .collect();
    assert_eq!(undescribed.len(), 1, "{undescribed:#?}");
    let ScrivenerFinding::Reported { args, .. } = undescribed[0] else {
        unreachable!();
    };
    assert_eq!(
        args,
        &vec![ScrivenerArg::Count {
            name: "count".into(),
            value: 2
        }]
    );
    // No two rows are the same kind about the same place.
    let mut seen = BTreeSet::new();
    for finding in &result.findings {
        let ScrivenerFinding::Reported {
            key,
            family,
            place,
            item,
            args,
            ..
        } = finding
        else {
            continue;
        };
        if *family != ScrivenerFindingFamily::Prose {
            continue;
        }
        let texts: Vec<&ScrivenerArg> = args
            .iter()
            .filter(|a| !matches!(a, ScrivenerArg::Count { .. }))
            .collect();
        assert!(
            seen.insert(format!("{key} {place:?} {item} {texts:?}")),
            "{key} twice about item {item}"
        );
    }
}

/// Every piece of text a written project holds: its title and author, every binder's name,
/// every row's title, prose, comments and footnotes, every tag, rung and note template.
fn every_text(output: &Path) -> Vec<String> {
    let bundle =
        skrib_format::read_bundle(&output.to_string_lossy()).expect("the project reads back");
    let work = &bundle.manifest.work;
    let mut out = vec![work.title.clone(), work.author_name.clone()];
    for binder in &bundle.binders {
        out.push(binder.binder.name.clone());
        for item in &binder.items {
            out.push(item.item.title.clone());
            out.extend(item.prose.values().cloned());
            for comment in item.comments.values().flatten() {
                out.push(comment.body.clone());
                out.push(comment.author_name.clone());
            }
            out.extend(item.footnotes.values().flatten().map(|f| f.body.clone()));
        }
    }
    for tag in &bundle.tags {
        out.push(tag.name.clone());
        out.push(tag.details.clone());
    }
    for rung in &bundle.statuses {
        out.push(rung.name.clone());
        out.push(rung.details.clone());
    }
    out.extend(bundle.note_templates.iter().map(|t| t.name.clone()));
    out.extend(bundle.note_template_bodies.values().cloned());
    out
}

/// Every name the import writes into the project is the interface's word, in its language:
/// with a French interface, nothing English is written that the project did not hold.
#[test]
fn with_french_words_no_english_reaches_the_project() {
    for generation in [Generation::Format20, Generation::Format15] {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut spec = Spec::novel(generation, Platform::Windows).with_metadata();
        // No status list: the project starts with the default ladder, in the writer's words.
        spec.statuses.clear();
        for item in &mut spec.items {
            item.status = None;
        }
        // A document template with no title of its own.
        let after = spec.index_of("Scene sheet").expect("a template") + 1;
        spec.items.insert(
            after,
            Item {
                title: None,
                ..Item::new(1, Kind::Text, "").with_text(12, 1)
            },
        );
        let source = write(&spec, dir.path());
        let mut backend = Backend::new();
        let analysis = backend.analyse(&analysis_request(&source));
        let output = dir.path().join("roman.skrib");
        let request = import_request(&source, &analysis, &output);
        backend.imported(&request);
        let bundle =
            skrib_format::read_bundle(&output.to_string_lossy()).expect("the project reads back");

        // What the project itself holds, which it may bring in any language.
        let mut own: BTreeSet<String> = spec.items.iter().filter_map(|i| i.title.clone()).collect();
        own.extend(spec.labels.iter().map(|(_, name)| name.clone()));
        own.extend(spec.keywords.iter().map(|(_, _, name)| name.clone()));
        // The novel's collection of the scenes to revise.
        own.insert("To revise".to_string());
        // The fixture's format 1.x project note, titled as Scrivener titles its first one.
        own.insert("General".to_string());
        own.insert(analysis.proposed_title.clone());
        own.insert(String::new());
        let french_words = [
            "Manuscrit",
            "Notes et documentation",
            "Notes de « ",
            "Notes du projet",
            "Textes retrouvés",
            "Modèle sans titre",
            "Venu de Scrivener",
            "(étiquette)",
            "Sans titre",
            "(mot-clé)",
            "Le mot-clé « ",
            "Une collection de Scrivener",
            "Du champ « ",
            "Mot français ",
        ];
        let from_the_interface = |text: &str| french_words.iter().any(|w| text.contains(w));
        let rungs: BTreeSet<String> = request
            .default_statuses
            .iter()
            .filter_map(|r| match r {
                ScrivenerDefaultRung::Rung { name, .. } => Some(name.clone()),
                ScrivenerDefaultRung::Empty => None,
            })
            .collect();

        let binders: Vec<&str> = bundle
            .binders
            .iter()
            .map(|b| b.binder.name.as_str())
            .collect();
        assert_eq!(binders, ["Manuscrit", "Notes et documentation"]);
        let mut notes_titles = 0;
        for item in bundle.binders.iter().flat_map(|b| b.items.iter()) {
            let title = &item.item.title;
            assert!(
                own.contains(title) || from_the_interface(title),
                "{generation:?}: the import titled a row {title:?}"
            );
            if title.starts_with("Notes de « ") {
                notes_titles += 1;
            }
        }
        assert!(notes_titles > 0, "no document notes came across");
        if generation == Generation::Format15 {
            assert!(
                bundle
                    .binders
                    .iter()
                    .flat_map(|b| b.items.iter())
                    .any(|i| i.item.title == "Notes du projet"),
                "the project notes' folder"
            );
        }
        for tag in &bundle.tags {
            assert!(
                own.contains(&tag.name) || from_the_interface(&tag.name),
                "{}",
                tag.name
            );
            assert!(
                tag.details.is_empty() || from_the_interface(&tag.details),
                "{}",
                tag.details
            );
        }
        let ladder: BTreeSet<String> = bundle.statuses.iter().map(|s| s.name.clone()).collect();
        assert_eq!(ladder, rungs, "the default ladder, in the writer's words");
        assert!(
            bundle
                .note_templates
                .iter()
                .any(|t| t.name == "Modèle sans titre"),
            "the untitled template"
        );
        for template in &bundle.note_templates {
            assert!(own.contains(&template.name) || from_the_interface(&template.name));
        }
        // The research files' folder beside the project, named in the writer's words.
        let research: Vec<String> = std::fs::read_dir(dir.path())
            .expect("the folder lists")
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name.contains("documentation"))
            .collect();
        assert_eq!(research, ["roman - documentation"], "{generation:?}");
    }
}

/// The review page foresees what the metadata mapping does: every metadata finding of the
/// plan the analysis hands over is one the import then reports, word for word, so the
/// "before you import" strip never says what the report does not.
#[test]
fn the_review_foresees_the_metadata_the_import_reports() {
    for generation in [Generation::Format20, Generation::Format15] {
        let dir = tempfile::tempdir().expect("tempdir");
        let spec = Spec::novel(generation, Platform::Windows)
            .with_metadata()
            .with_s2();
        let source = write(&spec, dir.path());
        let mut backend = Backend::new();
        let analysis = backend.analyse(&analysis_request(&source));
        let ScrivenerStructure::Planned { notices, .. } = &analysis.structure else {
            panic!("{generation:?}: the analysis planned no structure");
        };
        let foreseen: Vec<&ScrivenerFinding> = notices
            .iter()
            .filter(|f| reported(f).1 == &ScrivenerFindingFamily::Metadata)
            .collect();
        let keys: BTreeSet<&str> = foreseen.iter().map(|f| reported(f).0).collect();
        for key in [
            "scrivener-metadata-keywords-became-tags",
            "scrivener-metadata-collections-became-tags",
            "scrivener-metadata-pace-planned",
            "scrivener-metadata-listed-in-import-note",
            "scrivener-metadata-icons-dropped",
            "scrivener-metadata-unused-keywords-skipped",
        ] {
            assert!(
                keys.contains(key),
                "{generation:?}: {key} is not foreseen in {keys:?}"
            );
        }
        let output = dir.path().join("roman.skrib");
        let result = backend.imported(&import_request(&source, &analysis, &output));
        for finding in foreseen {
            assert!(
                result.findings.contains(finding),
                "{generation:?}: the review said {finding:?}, the import did not"
            );
        }
    }
}
