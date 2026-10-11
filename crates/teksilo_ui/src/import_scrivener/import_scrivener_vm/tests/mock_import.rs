// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The mocks build walks the wizard to the report: the fabricated inspection reads a
//! project, the fabricated run (`models::scrivener_import_run`) imports it on the shared
//! job's own event path, and the result, its report, a cancel and every Scrivener failure
//! come out as they do in the real build.

use super::*;

use std::any::{Any, TypeId};
use std::time::Duration;

use frontend::common::event::{Event, LongOperationEvent, Origin};
use frontend::common::long_operation::lock_or_recover;
use frontend::import_management::failure::{FailureReason, ProjectImportFailure};
use frontend::import_management::{ScrivenerFindingFamily, ScrivenerPlace};
use teksilo::core::widget_tree::WidgetTree;
use teksilo::widgets::{
    BannerSeverity, NotificationArchiveModel, NotificationEntry, ToastInstallOptions, ToastRegistry,
};

use crate::import_scrivener::messages::{self, Group};
use crate::import_scrivener::report::{ImportReport, SHOWN_ITEMS};
use crate::import_scrivener::report_panel::ImportReportPanel;
use crate::models::scrivener_import_run::{EITHER_OR, FAILURE_WORDS, chosen_failure};
use crate::shared::import_failure;
use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};

/// A wizard ready to import, its job, the tree its notices go to and the log they are
/// filed in, and every event the backend sends from now on.
struct Walk {
    app_ctx: Rc<AppContext>,
    job: ProjectImportJob,
    vm: ImportScrivenerViewModel,
    tree: WidgetTree,
    archive: Rc<NotificationArchiveModel>,
    sent: frontend::flume::Receiver<Event>,
    /// The folder the project is in, and the new project goes beside it.
    dir: tempfile::TempDir,
}

impl Walk {
    /// The wizard on the fabricated project `<name>.scriv`, read, reviewed and saved with
    /// what its pages propose: the next press of Import starts the fabricated run.
    fn to_the_save_page(name: &str) -> Self {
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("a scratch folder: {e}"),
        };
        let project = dir.path().join(format!("{name}.scriv"));
        if let Err(e) = std::fs::create_dir(&project) {
            panic!("a project folder: {e}");
        }
        let app_ctx = Rc::new(AppContext::new());
        let sent = app_ctx.event_hub.subscribe_receiver();
        let job = ProjectImportJob::new(app_ctx.clone());
        let vm = ImportScrivenerViewModel::new(app_ctx.clone(), job.clone());
        let archive = Rc::new(NotificationArchiveModel::in_memory());
        let toasts = ToastRegistry::with_archive(
            ToastInstallOptions {
                archive: None,
                ..ToastInstallOptions::default()
            },
            archive.clone(),
        );
        let state: HashMap<TypeId, Box<dyn Any>> = [(
            TypeId::of::<ToastRegistry>(),
            Box::new(toasts) as Box<dyn Any>,
        )]
        .into();
        let tree = crate::test_support::tree_with_app_state(&app_ctx, state);

        // Project: the fabricated inspection answers at once.
        vm.source_text().set(project.to_string_lossy().into_owned());
        vm.submit_source();
        assert_eq!(vm.inspection_page().get(), PAGE_CARD, "{name} reads");
        assert!(vm.can_leave_project_step().get());
        // Review: one choice of the writer's, which the report records.
        vm.pick(PickTarget::Root(27), 3);
        assert!(vm.can_leave_review_step().get());
        // Save: what the page proposes, beside the project.
        assert!(vm.can_import().get(), "the Save page is complete");
        Walk {
            app_ctx,
            job,
            vm,
            tree,
            archive,
            sent,
            dir,
        }
    }

    /// The file the import writes.
    fn target(&self) -> String {
        self.vm.target_path().get()
    }

    /// Press Import, and return the run's operation id.
    fn import(&mut self) -> String {
        let vm = self.vm.clone();
        press(&mut self.tree, move |c| {
            vm.import(c);
        });
        self.job
            .active_operation()
            .unwrap_or_else(|| panic!("the import started"))
    }

    /// Wait until the run `op` has ended, then hand the job every event sent so far, as
    /// each window's subscription would. Returns those events.
    fn finish(&mut self, op: &str) -> Vec<Event> {
        let completion = lock_or_recover(&self.app_ctx.long_operation_manager).completion_signal();
        assert!(
            completion.wait_for(op, Some(Duration::from_secs(60))),
            "the fabricated run never ended"
        );
        let sent: Vec<Event> = self.sent.try_iter().collect();
        for event in sent.clone() {
            let job = self.job.clone();
            press(&mut self.tree, move |c| match &event.origin {
                Origin::LongOperation(LongOperationEvent::Progress) => {
                    job.on_long_op_progress(c, &event)
                }
                Origin::LongOperation(LongOperationEvent::Completed) => {
                    job.on_long_op_completed(c, &event)
                }
                Origin::LongOperation(LongOperationEvent::Cancelled) => {
                    job.on_long_op_cancelled(c, &event)
                }
                Origin::LongOperation(LongOperationEvent::Failed) => {
                    job.on_long_op_failed(c, &event)
                }
                _ => {}
            });
        }
        sent
    }

    /// The import's own notice in the log, as it ended.
    fn notice(&self) -> NotificationEntry {
        let entries = self.archive.entries();
        (0..entries.len())
            .filter_map(|i| entries.with_item(i, |e| e.clone()))
            .find(|e| e.dedup_id.as_deref() == Some(SCRIVENER.toast_id))
            .unwrap_or_else(|| panic!("the import's notice is in the log"))
    }

    /// Whether anything of the import is left beside the target.
    fn leftovers(&self) -> Vec<String> {
        match std::fs::read_dir(self.dir.path()) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|name| !name.ends_with(".scriv"))
                .collect(),
            Err(e) => panic!("the scratch folder: {e}"),
        }
    }
}

/// What the progress events carried, in order: each percentage and its message, a phase
/// code or the title of the row just converted.
fn progress_sent(sent: &[Event]) -> Vec<(f64, String)> {
    sent.iter()
        .filter(|e| e.origin == Origin::LongOperation(LongOperationEvent::Progress))
        .filter_map(|e| {
            let payload: serde_json::Value = serde_json::from_str(e.data.as_deref()?).ok()?;
            Some((
                payload.get("percentage")?.as_f64()?,
                payload.get("message")?.as_str()?.to_string(),
            ))
        })
        .collect()
}

/// The typed failure a `Failed` event carried.
fn failure_sent(sent: &[Event]) -> Option<ProjectImportFailure> {
    sent.iter()
        .filter(|e| e.origin == Origin::LongOperation(LongOperationEvent::Failed))
        .find_map(|e| {
            let payload: serde_json::Value = serde_json::from_str(e.data.as_deref()?).ok()?;
            Some(ProjectImportFailure::from_any_message(
                payload.get("error")?.as_str()?,
            ))
        })
}

/// The walk from the Project page to the report: the run goes through its phases, counting
/// rows while it converts, ends in a result offering **Open now** and **See report**, writes
/// a project the mocks build opens, and its report has every group, every kind of line, a
/// list longer than the dialog shows, a line about the project as a whole, the writer's
/// choices and the research files.
#[test]
fn the_mocks_build_walks_an_import_to_its_report() {
    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let mut walk = Walk::to_the_save_page("Harbour Lights");
        let target = walk.target();
        let op = walk.import();
        let sent = walk.finish(&op);

        // The progress the notice shows: every phase, in order, and the rows it converts
        // named as it goes, up to the last.
        let seen = progress_sent(&sent);
        let said = |code: &str| seen.iter().position(|(_, message)| message == code);
        assert_eq!(
            seen.first().map(|(_, m)| m.as_str()),
            Some("import.opening")
        );
        assert_eq!(seen.last().map(|(_, m)| m.as_str()), Some("import.done"));
        let (Some(converting), Some(writing)) = (said("import.converting"), said("import.writing"))
        else {
            panic!("the run says it converts, then writes: {seen:?}");
        };
        assert!(said("import.outline").is_some(), "it reads: {seen:?}");
        let rows = &seen[converting + 1..writing];
        assert!(rows.len() > 2, "converting names its rows: {rows:?}");
        assert!(
            rows.iter().all(|(_, title)| !title.starts_with("import.")),
            "{rows:?}"
        );
        assert!(rows.windows(2).all(|w| w[0].0 < w[1].0), "{rows:?}");
        assert_eq!(rows.last().map(|(percent, _)| *percent), Some(90.0));

        // The result: Open now on the project it wrote, and See report.
        assert!(!walk.job.is_running());
        let notice = walk.notice();
        assert_eq!(notice.severity, BannerSeverity::Success);
        assert_eq!(
            notice.title,
            tr!(import_scrivener_done_title()).resolve_now()
        );
        let open_now = crate::shared::project_import::archived_open_name(&target);
        assert!(
            notice
                .actions
                .iter()
                .any(|a| a.intent_name.as_deref() == Some(open_now.as_str())),
            "Open now opens {target}: {:?}",
            notice.actions
        );
        assert!(
            notice
                .actions
                .iter()
                .any(|a| a.label == tr!(import_scrivener_see_report()).resolve_now()),
            "{:?}",
            notice.actions
        );

        // Open now opens a real project: it loads, under the title the Save page sent.
        assert!(walk.leftovers().iter().all(|f| !f.ends_with(".importing")));
        let opened = AppContext::new();
        let loaded = frontend::commands::work_management_commands::load_work(
            &opened,
            &frontend::work_management::LoadWorkDto {
                media_root: crate::media_paths::media_root_string(),
                file_name: target.clone(),
            },
        );
        assert!(loaded.is_ok(), "the new project opens: {loaded:?}");
        let bundle = match skrib_format::read_bundle(&target) {
            Ok(bundle) => bundle,
            Err(e) => panic!("the new project reads: {e:#}"),
        };
        assert_eq!(bundle.manifest.work.title, walk.vm.title().get());
        assert_eq!(bundle.manifest.work.author_name, walk.vm.author().get());

        // See report shows the dialog over the window.
        let vm = walk.vm.clone();
        let output = target.clone();
        press(&mut walk.tree, move |c| {
            vm.show_report_of(c, &output, None, None)
        });
        assert_eq!(
            drain_dialog_titles(&mut walk.tree),
            vec![tr!(project_import_report_title()).resolve_now()]
        );

        // What the dialog holds.
        let result = match report_of(&target) {
            Ok(result) => result,
            Err(why) => panic!("the report is kept: {}", why.resolve_now()),
        };
        let report = ImportReport::from_result(&result);
        let groups: Vec<Group> = report.groups.iter().map(|g| g.group).collect();
        assert_eq!(
            groups,
            vec![
                Group::CouldNotBeRead,
                Group::CheckThese,
                Group::BroughtAcrossDifferently,
                Group::LeftBehind
            ]
        );
        let families: Vec<ScrivenerFindingFamily> = result
            .findings
            .iter()
            .filter_map(|f| match f {
                ScrivenerFinding::Reported { family, .. } => Some(family.clone()),
                ScrivenerFinding::Empty => None,
            })
            .collect();
        for family in [
            ScrivenerFindingFamily::Prose,
            ScrivenerFindingFamily::Document,
            ScrivenerFindingFamily::Structure,
            ScrivenerFindingFamily::Metadata,
            ScrivenerFindingFamily::Source,
        ] {
            assert!(families.contains(&family), "a {family:?} finding");
        }
        let keys: Vec<&str> = result.findings.iter().map(messages::key_of).collect();
        // Every metadata kind the converter has, but for the one of each pair a single
        // project cannot have both of: the other is in this report.
        for key in frontend::import_management::scrivener::every_key()
            .into_iter()
            .filter(|k| k.starts_with("scrivener-metadata-"))
        {
            let instead = EITHER_OR.iter().find_map(|(one, other)| {
                (key == *one)
                    .then_some(*other)
                    .or((key == *other).then_some(*one))
            });
            assert!(
                keys.contains(&key) || instead.is_some_and(|k| keys.contains(&k)),
                "{key} is not in the report"
            );
        }
        let lines: Vec<&crate::import_scrivener::report::ReportLine> =
            report.groups.iter().flat_map(|g| g.lines.iter()).collect();
        assert!(
            lines
                .iter()
                .any(|l| l.more() > 0 && l.items.len() > SHOWN_ITEMS),
            "a line lists more items than the dialog shows"
        );
        assert!(
            lines.iter().any(|l| l.items.is_empty()),
            "a line is about the project as a whole"
        );
        assert!(
            result.findings.iter().any(|f| matches!(
                f,
                ScrivenerFinding::Reported {
                    place: ScrivenerPlace::Project,
                    ..
                }
            )),
            "a finding about the project as a whole"
        );
        // Every line is worded, none says a key.
        for line in &lines {
            let text = line.text.resolve_now();
            assert!(
                !text.trim().is_empty()
                    && !text.starts_with("scrivener-")
                    && !text.contains("table-read"),
                "{text}"
            );
        }
        // The writer's choices: the folder left out, the title, and the research files.
        let choices: Vec<String> = report.choices.iter().map(|c| c.resolve_now()).collect();
        assert!(
            choices.iter().any(|c| c.contains("Old ideas")),
            "the folder the writer left out: {choices:?}"
        );
        assert!(
            choices.iter().any(|c| c.contains(&walk.vm.title().get())),
            "{choices:?}"
        );
        assert!(
            !result.research_folder.is_empty(),
            "the research files were copied"
        );
        assert!(
            choices.iter().any(|c| c.contains(&result.research_folder)),
            "{choices:?}"
        );

        // The dialog builds, lays out, and says how many more items a line has.
        let app_ctx = Rc::new(AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&app_ctx).with_text_backend(Rc::new(
            RefCell::new(teksilo::canvas::MockTextBackend::new()),
        ));
        let dialog = tree.add_boxed(Box::new(ImportReportPanel::new(
            ImportReportViewModel::new(report.clone(), None),
        )));
        tree.layout(SizeProposal::exact(900.0, 700.0));
        assert!(tree.bounds(dialog).height > 0.0);
        let more = lines
            .iter()
            .map(|l| l.more())
            .find(|more| *more > 0)
            .unwrap_or_default();
        let and_more = tr!(import_scrivener_report_more(count = more as i64)).resolve_now();
        assert!(
            tree.find_by_label(&and_more).is_some(),
            "the dialog says {and_more:?}"
        );
    });
}

/// The fabricated report is worded in French too: every line resolves, none to its key.
#[test]
fn the_fabricated_report_reads_in_french() {
    let _registry = IsolatedOpenRegistry::new();
    let mut walk = Walk::to_the_save_page("Phare");
    let target = walk.target();
    let op = walk.import();
    walk.finish(&op);
    let result = match report_of(&target) {
        Ok(result) => result,
        Err(_) => panic!("the report is kept"),
    };
    crate::test_support::with_shipped_messages("fr-FR", || {
        let report = ImportReport::from_result(&result);
        assert_eq!(report.groups.len(), 4);
        for line in report.groups.iter().flat_map(|g| g.lines.iter()) {
            let text = line.text.resolve_now();
            assert!(
                !text.trim().is_empty()
                    && !text.starts_with("scrivener-")
                    && !text.starts_with("import-"),
                "{text}"
            );
        }
        assert!(!report.text().is_empty());
    });
}

/// Cancel on the progress notice stops the fabricated run at its next step: the notice
/// says the import was cancelled, and nothing is left beside the target.
#[test]
fn a_cancelled_fabricated_import_leaves_nothing() {
    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let mut walk = Walk::to_the_save_page("Harbour Lights");
        let target = walk.target();
        let op = walk.import();
        let job = walk.job.clone();
        press(&mut walk.tree, move |c| job.cancel(c));
        walk.finish(&op);

        assert!(!walk.job.is_running());
        assert_eq!(
            walk.notice().title,
            tr!(project_import_cancelled()).resolve_now()
        );
        assert!(!Path::new(&target).exists(), "no project");
        assert!(walk.leftovers().is_empty(), "{:?}", walk.leftovers());
        assert!(!crate::shell::open_registry::importing(&target));
    });
}

/// Each word of the project's path that chooses a failure ends the fabricated run in that
/// failure: the typed reason reaches the job, the notice says it in the importer's own
/// words, offers **Review the import again** where a fresh review answers it, and nothing
/// is written.
#[test]
fn every_scrivener_failure_can_be_walked() {
    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let mut reasons = Vec::new();
        for word in FAILURE_WORDS {
            let mut walk = Walk::to_the_save_page(&format!("Harbour {word}"));
            let target = walk.target();
            let request = walk
                .vm
                .request()
                .unwrap_or_else(|| panic!("{word}: a request"));
            let expected = chosen_failure(&request).unwrap_or_else(|| panic!("{word} fails"));
            let op = walk.import();
            let sent = walk.finish(&op);

            let failure = failure_sent(&sent).unwrap_or_else(|| panic!("{word}: a failure"));
            assert_eq!(failure.reason, expected, "{word}");
            let notice = walk.notice();
            assert_eq!(notice.severity, BannerSeverity::Error, "{word}");
            let (body, _) = import_failure::failure_text(&failure, messages::own_failure);
            assert_eq!(notice.body, Some(body.resolve_now()), "{word}");
            assert!(
                messages::own_failure(&failure.reason).is_some(),
                "{word}: the importer words it itself"
            );
            let review = tr!(import_scrivener_review_again()).resolve_now();
            assert_eq!(
                notice.actions.iter().any(|a| a.label == review),
                messages::reviewing_again_helps(&failure.reason),
                "{word}: {:?}",
                notice.actions
            );
            assert!(!Path::new(&target).exists(), "{word}: nothing written");
            assert!(
                walk.leftovers().is_empty(),
                "{word}: {:?}",
                walk.leftovers()
            );
            reasons.push(std::mem::discriminant(&failure.reason));
        }
        // Every Scrivener reason, the refusal both with and without a review to offer.
        for reason in [
            FailureReason::ScrivenerChangedSinceReview,
            FailureReason::ScrivenerQuestionFirst {
                question: ScrivenerQuestion::default(),
            },
            FailureReason::ScrivenerNotAccepted {
                questions: Vec::new(),
            },
            FailureReason::ScrivenerDepthNotOffered { depth: 0 },
            FailureReason::ScrivenerRootNotOffered { item: 0 },
            FailureReason::ScrivenerRefused {
                refusal: ScrivenerFinding::default(),
            },
        ] {
            assert!(
                reasons.contains(&std::mem::discriminant(&reason)),
                "{reason:?} is walked"
            );
        }
    });
}

/// **Review the import again** after a failure a fresh review answers leads to an import
/// that writes the project: the review asks what the import said was missing (a manifest to choose,
/// edits to accept) and nothing else, the writer answers, and the next import succeeds.
#[test]
fn reviewing_again_after_a_fabricated_failure_leads_to_an_import() {
    let _registry = IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let mut walked = 0;
        for word in FAILURE_WORDS {
            let mut walk = Walk::to_the_save_page(&format!("Harbour {word}"));
            let target = walk.target();
            let source = walk.vm.source_text().get();
            let op = walk.import();
            let sent = walk.finish(&op);
            let failure = failure_sent(&sent).unwrap_or_else(|| panic!("{word}: a failure"));
            if !messages::reviewing_again_helps(&failure.reason) {
                continue;
            }
            walked += 1;

            walk.vm.review_again(&source);
            assert_eq!(walk.vm.inspection_page().get(), PAGE_CARD, "{word}");
            let questions = walk.vm.questions();
            let asked: Vec<(String, bool)> = (0..questions.len())
                .filter_map(|i| questions.with_item(i, |q| (q.key.clone(), q.accept.is_some())))
                .collect();
            match &failure.reason {
                FailureReason::ScrivenerQuestionFirst { .. } => assert!(
                    asked.iter().any(|(_, accept)| !accept),
                    "{word}: the review asks the question the import named: {asked:?}"
                ),
                FailureReason::ScrivenerNotAccepted { .. } => assert!(
                    asked
                        .iter()
                        .any(|(key, accept)| *accept && key == UNMERGED_MOBILE_EDITS),
                    "{word}: the review asks for the acceptance the import missed: {asked:?}"
                ),
                _ => assert!(asked.is_empty(), "{word}: nothing to answer: {asked:?}"),
            }

            // The writer answers: the first candidate of a question to choose, a tick for
            // a question to accept.
            for _ in 0..4 {
                let questions = walk.vm.questions();
                let open = (0..questions.len()).find_map(|i| {
                    questions
                        .with_item(i, |q| {
                            if !q.options.is_empty() && q.chosen.get() == NO_ANSWER {
                                Some((q.index, None))
                            } else if q.accept.is_some() && !q.accepted.get() {
                                Some((q.index, Some(q.key.clone())))
                            } else {
                                None
                            }
                        })
                        .flatten()
                });
                match open {
                    Some((index, None)) => walk.vm.choose(index, 0),
                    Some((_, Some(key))) => walk.vm.accept(&key, true),
                    None => break,
                }
            }
            assert!(walk.vm.can_leave_project_step().get(), "{word}");
            assert!(walk.vm.can_leave_review_step().get(), "{word}");
            assert!(walk.vm.can_import().get(), "{word}");

            let op = walk.import();
            let sent = walk.finish(&op);
            assert_eq!(
                failure_sent(&sent).map(|f| f.reason),
                None,
                "{word}: the import after a fresh review succeeds"
            );
            assert!(
                Path::new(&target).exists(),
                "{word}: the project is written"
            );
        }
        assert!(walked >= 5, "the failures a review answers are walked");
    });
}
