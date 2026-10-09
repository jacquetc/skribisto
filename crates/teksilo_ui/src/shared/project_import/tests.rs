// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The shared job, driven through the two real importers' forms: one import at a
//! time across importers, the progress notice in the writer's language, and
//! stopping for a quit.

use super::*;

use std::any::{Any, TypeId};
use std::path::PathBuf;
use std::time::Instant;

use teksilo::core::widget_tree::WidgetTree;
use teksilo::widgets::{
    NotificationArchiveModel, NotificationEntry, ToastInstallOptions, ToastRegistry,
};

use crate::import_manuskript::ImportManuskriptViewModel;
use crate::import_plume::ImportPlumeViewModel;
use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};

/// A tree whose app state holds a toast registry filing into an in-memory
/// notification archive, as the running app's does.
fn tree_with_archive(app_ctx: &Rc<AppContext>) -> (WidgetTree, Rc<NotificationArchiveModel>) {
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
    (
        crate::test_support::tree_with_app_state(app_ctx, state),
        archive,
    )
}

/// Every row of `archive`, oldest first.
fn archived(archive: &NotificationArchiveModel) -> Vec<NotificationEntry> {
    (0..archive.entries().len())
        .filter_map(|i| archive.entries().with_item(i, |e| e.clone()))
        .collect()
}

/// A zipped Plume project holding one chapter with nothing but a separator in
/// it: an import that always has one notice to report.
fn lonely_plume(dir: &Path) -> PathBuf {
    use std::io::Write;
    let source = dir.join("Lonely.plume");
    let mut zip = zip::ZipWriter::new(std::fs::File::create(&source).unwrap());
    zip.start_file("tree", zip::write::SimpleFileOptions::default())
        .unwrap();
    zip.write_all(
        br#"<!DOCTYPE plume-tree><plume-tree version="0.5" projectName="Lonely">
            <book number="1" name="Book">
              <chapter number="2" name="Only a break">
                <separator number="10001" name="* * *"/>
              </chapter>
            </book>
          </plume-tree>"#,
    )
    .unwrap();
    zip.finish().unwrap();
    source
}

/// A Plume form over `job`, filled in to import the lonely project into `dir`
/// as `<name>.skrib`.
fn plume_form(job: &ProjectImportJob, dir: &Path, name: &str) -> ImportPlumeViewModel {
    let vm = ImportPlumeViewModel::new(job.clone());
    vm.source()
        .set(lonely_plume(dir).to_string_lossy().into_owned());
    vm.location().set(dir.to_string_lossy().into_owned());
    vm.name().set(name.into());
    vm
}

/// Wait for the operation `op` to finish in the backend: its worker has
/// returned. Not `is_operation_finished`, which answers `true` the moment a
/// cancel is requested, while the converter may still be writing.
fn wait_for(app_ctx: &Rc<AppContext>, op: &str) {
    let completion =
        frontend::common::long_operation::lock_or_recover(&app_ctx.long_operation_manager)
            .completion_signal();
    assert!(
        completion.wait_for(op, Some(Duration::from_secs(60))),
        "the import never finished"
    );
}

/// A long-operation event for `op`, as the manager sends it.
fn event(kind: LongOperationEvent, payload: serde_json::Value) -> Event {
    Event {
        origin: Origin::LongOperation(kind),
        ids: Vec::new(),
        data: Some(payload.to_string()),
    }
}

/// One import at a time in the whole process, whichever importer asks: a
/// Manuskript import pressed while a Plume import runs is refused with a
/// message, starts nothing, and leaves the first one's target held.
#[test]
fn a_second_importer_waits_until_the_first_import_has_finished() {
    let _registry = IsolatedOpenRegistry::new();
    let dir = tempfile::tempdir().unwrap();
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let (mut tree, _archive) = tree_with_archive(&app_ctx);

    let plume = plume_form(&job, dir.path(), "novel");
    let importing = plume.clone();
    press(&mut tree, move |c| importing.import(c));
    let op = job.active_operation().expect("the Plume import started");

    let manuskript = ImportManuskriptViewModel::new(job.clone());
    manuskript.source().set(format!(
        "{}/../manuskript_import/tests/fixtures/tour-du-monde.msk",
        env!("CARGO_MANIFEST_DIR")
    ));
    manuskript
        .location()
        .set(dir.path().to_string_lossy().into_owned());
    manuskript.name().set("other".into());
    let pressing = manuskript.clone();
    press(&mut tree, move |c| pressing.import(c));

    assert_eq!(
        drain_dialog_titles(&mut tree),
        vec![tr!(project_import_busy_title()).resolve_now()],
        "the second import is refused with a message"
    );
    assert_eq!(
        job.active_operation(),
        Some(op.clone()),
        "the first is still the one"
    );
    assert!(crate::shell::open_registry::importing(
        &dir.path().join("novel.skrib").to_string_lossy()
    ));

    wait_for(&app_ctx, &op);
    let completed = event(LongOperationEvent::Completed, serde_json::json!({"id": op}));
    let finishing = job.clone();
    press(&mut tree, move |c| {
        finishing.on_long_op_completed(c, &completed)
    });
    assert!(!job.is_running());
    assert!(
        !dir.path().join("other.skrib").exists(),
        "nothing was written for the refused one"
    );
}

/// The progress notice says what the import is doing in the writer's language,
/// from the phase code the converter reports; a message that is not a code (the
/// entry being converted) is shown as it was sent rather than dropped.
#[test]
fn progress_is_worded_in_the_writers_language() {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            let _registry = IsolatedOpenRegistry::new();
            let dir = tempfile::tempdir().unwrap();
            let app_ctx = Rc::new(AppContext::new());
            let job = ProjectImportJob::new(app_ctx.clone());
            let (mut tree, archive) = tree_with_archive(&app_ctx);
            let vm = plume_form(&job, dir.path(), "novel");
            let importing = vm.clone();
            press(&mut tree, move |c| importing.import(c));
            let op = job.active_operation().expect("started");

            let body = |tree: &mut WidgetTree, message: String| {
                let tick = event(
                    LongOperationEvent::Progress,
                    serde_json::json!({"id": op, "percentage": 45.0, "message": message}),
                );
                let ticking = job.clone();
                press(tree, move |c| ticking.on_long_op_progress(c, &tick));
                archived(&archive)
                    .into_iter()
                    .find(|e| e.dedup_id.as_deref() == Some("import.plume"))
                    .and_then(|e| e.body)
                    .unwrap_or_default()
            };

            let converting = body(&mut tree, "import.converting".to_string());
            let expected = progress_text::progress_body(45.0, "import.converting").resolve_now();
            assert_eq!(converting, expected, "{locale}");
            assert!(converting.contains("45"), "{locale}: {converting}");
            assert!(
                !converting.contains("import.converting"),
                "{locale} showed the code: {converting}"
            );
            assert!(
                !converting.contains("Converting") || locale == "en-US",
                "{locale} is not in English: {converting}"
            );

            let said = body(&mut tree, "Something else".to_string());
            assert!(said.contains("Something else"), "{locale}: {said}");

            wait_for(&app_ctx, &op);
        });
    }
}

/// Every phase code a project importer reports has a name, in both locales: the
/// name the notice announces while the entries under it tick by.
#[test]
fn every_import_phase_is_named_in_both_locales() {
    for locale in ["en-US", "fr-FR"] {
        crate::test_support::with_shipped_messages(locale, || {
            for code in [
                "import.opening",
                "import.outline",
                "import.converting",
                "import.writing",
                "import.done",
            ] {
                let name = progress_text::phase_name(code)
                    .map(|n| n.resolve_now())
                    .unwrap_or_default();
                assert!(
                    !name.trim().is_empty() && name != code && !name.starts_with("progress-"),
                    "{locale} {code}: {name}"
                );
            }
            assert!(
                progress_text::phase_name("Chapter One").is_none(),
                "an entry's title is data, not a phase"
            );
        });
    }
}

/// Stopping for a quit cancels the running import and waits for its end: the
/// continuation runs only once the operation's own terminal event has arrived,
/// by which time the converter has cleaned up after itself.
#[test]
fn stopping_waits_for_the_import_to_end() {
    use std::cell::Cell;

    let _registry = IsolatedOpenRegistry::new();
    let dir = tempfile::tempdir().unwrap();
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let (mut tree, _archive) = tree_with_archive(&app_ctx);
    let vm = plume_form(&job, dir.path(), "novel");
    let importing = vm.clone();
    press(&mut tree, move |c| importing.import(c));
    let op = job.active_operation().expect("started");

    let went_on = Rc::new(Cell::new(false));
    let stopping = job.clone();
    let flag = went_on.clone();
    press(&mut tree, move |c| {
        let flag = flag.clone();
        stopping.stop_then(c, move |_| flag.set(true));
    });
    assert!(!went_on.get(), "not before the import has ended");
    assert!(job.is_running());

    wait_for(&app_ctx, &op);
    // However it ended (cancelled at a checkpoint, or finished before the cancel
    // reached it), the manager says so once, and the quit goes on then.
    let ended = match long_operation_commands::get_operation_status(&app_ctx, &op) {
        Some(frontend::common::long_operation::OperationStatus::Completed) => {
            LongOperationEvent::Completed
        }
        _ => LongOperationEvent::Cancelled,
    };
    let terminal = event(ended.clone(), serde_json::json!({"id": op}));
    let ending = job.clone();
    press(&mut tree, move |c| match ended {
        LongOperationEvent::Completed => ending.on_long_op_completed(c, &terminal),
        _ => ending.on_long_op_cancelled(c, &terminal),
    });
    assert!(went_on.get(), "the quit goes on once the import has ended");
    assert!(!job.is_running());
    assert!(
        !dir.path().join("novel.skrib.importing").exists(),
        "no half-written file is left"
    );
}

/// With nothing running, stopping is immediate.
#[test]
fn stopping_with_nothing_running_goes_on_at_once() {
    use std::cell::Cell;

    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    let went_on = Rc::new(Cell::new(false));
    let flag = went_on.clone();
    press(&mut tree, move |c| {
        let flag = flag.clone();
        job.stop_then(c, move |_| flag.set(true));
    });
    assert!(went_on.get());
}

/// An import that could not even start reports why in a translated sentence,
/// the backend's words quoted inside it, and lets go of its target.
#[test]
fn a_failure_to_start_is_worded_and_lets_go_of_the_target() {
    let _registry = IsolatedOpenRegistry::new();
    let dir = tempfile::tempdir().unwrap();
    let target = dir
        .path()
        .join("novel.skrib")
        .to_string_lossy()
        .into_owned();
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let (mut tree, archive) = tree_with_archive(&app_ctx);

    static REFUSES: Importer<String> = Importer {
        toast_id: "import.test",
        notice: &crate::shared::import_warnings::PLUME,
        progress_title: || lit!("Importing"),
        own_words: |_| None,
        target: |request| request.clone(),
        allow_overwrite: |request| request,
        launch: |_, _| Err(anyhow::anyhow!("the manager is gone")),
        outcome: |_, _| None,
    };
    let starting = job.clone();
    let request = target.clone();
    press(&mut tree, move |c| {
        starting.start(c, &REFUSES, request.clone())
    });

    assert!(!job.is_running());
    assert!(!crate::shell::open_registry::importing(&target));
    let error = archived(&archive)
        .into_iter()
        .find(|e| e.dedup_id.as_deref() == Some("import.test"))
        .expect("the failure is reported");
    let body = error.body.unwrap_or_default();
    assert!(body.contains("the manager is gone"), "{body}");
    assert_ne!(
        body, "the manager is gone",
        "a sentence, not the bare chain"
    );
}

/// Hand `job` every long-operation event the backend has sent so far, in the
/// order it sent them, as each window's subscription would.
fn deliver_sent(tree: &mut WidgetTree, job: &ProjectImportJob, sent: Vec<Event>) {
    for event in sent {
        let job = job.clone();
        press(tree, move |c| match &event.origin {
            Origin::LongOperation(LongOperationEvent::Progress) => {
                job.on_long_op_progress(c, &event)
            }
            Origin::LongOperation(LongOperationEvent::Completed) => {
                job.on_long_op_completed(c, &event)
            }
            Origin::LongOperation(LongOperationEvent::Cancelled) => {
                job.on_long_op_cancelled(c, &event)
            }
            Origin::LongOperation(LongOperationEvent::Failed) => job.on_long_op_failed(c, &event),
            _ => {}
        });
    }
}

/// A converter in its writing phase, which polls no cancel flag: it has its
/// staged file on disk, keeps writing until a file appears at `release`, then
/// removes the staged file and reports the cancellation, as the converters'
/// staged output does on its way out. A file rather than a shared flag, so tests
/// running side by side each hold their own.
struct StillWriting {
    staged: PathBuf,
    release: PathBuf,
}

impl frontend::common::long_operation::LongOperation for StillWriting {
    type Output = i64;

    fn execute(
        &self,
        _progress: Box<dyn Fn(frontend::common::long_operation::OperationProgress) + Send>,
        _cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> anyhow::Result<i64> {
        std::fs::write(&self.staged, b"half a project")?;
        let deadline = Instant::now() + Duration::from_secs(60);
        while !self.release.exists() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(5));
        }
        std::fs::remove_file(&self.staged)?;
        Err(anyhow::Error::new(skrib_format::importing::ImportCancelled))
    }
}

/// An importer whose conversion is [`StillWriting`] into `<target>.importing`,
/// let go of by `<target>.release`.
static STILL_WRITING: Importer<String> = Importer {
    toast_id: "import.test.writing",
    notice: &crate::shared::import_warnings::PLUME,
    progress_title: || lit!("Importing"),
    own_words: |_| None,
    target: |request| request.clone(),
    allow_overwrite: |request| request,
    launch: |app_ctx, target| {
        let staged = PathBuf::from(format!("{target}.importing"));
        let release = PathBuf::from(format!("{target}.release"));
        Ok(
            frontend::common::long_operation::lock_or_recover(&app_ctx.long_operation_manager)
                .start_operation(StillWriting { staged, release }),
        )
    },
    outcome: |_, _| None,
};

/// Start [`STILL_WRITING`] into `target` on `job`, and wait until its staged
/// file is on disk. Returns the operation's id.
fn start_still_writing(tree: &mut WidgetTree, job: &ProjectImportJob, target: &str) -> String {
    let starting = job.clone();
    let request = target.to_string();
    press(tree, move |c| {
        starting.start(c, &STILL_WRITING, request.clone())
    });
    let op = job.active_operation().expect("started");
    let staged = PathBuf::from(format!("{target}.importing"));
    let deadline = Instant::now() + Duration::from_secs(60);
    while !staged.exists() {
        assert!(Instant::now() < deadline, "the conversion never started");
        std::thread::sleep(Duration::from_millis(5));
    }
    op
}

/// The manager announces a cancellation the moment it is asked for, while the
/// converter may still be writing: in its writing phase it looks at no cancel
/// flag until the whole file is written. The job must not take that notice for
/// the end. A quit waiting on it would let the process exit under a converter
/// still writing, leaving its half-written file behind; and the target, let go
/// of, could be opened, or imported into again, while the first converter
/// still means to write it.
#[test]
fn a_cancel_is_not_the_end_while_the_converter_still_writes() {
    use std::cell::Cell;

    let _registry = IsolatedOpenRegistry::new();
    let dir = tempfile::tempdir().unwrap();
    let target = dir
        .path()
        .join("novel.skrib")
        .to_string_lossy()
        .into_owned();
    let staged = PathBuf::from(format!("{target}.importing"));
    let app_ctx = Rc::new(AppContext::new());
    // Every event the backend sends, as the running app's windows receive them.
    let sent = app_ctx.event_hub.subscribe_receiver();
    let job = ProjectImportJob::new(app_ctx.clone());
    let (mut tree, archive) = tree_with_archive(&app_ctx);
    let notice = |archive: &NotificationArchiveModel| {
        archived(archive)
            .into_iter()
            .find(|e| e.dedup_id.as_deref() == Some("import.test.writing"))
            .and_then(|e| e.body)
            .unwrap_or_default()
    };

    let op = start_still_writing(&mut tree, &job, &target);

    // Quit: stop the import, then go on.
    let went_on = Rc::new(Cell::new(false));
    let stopping = job.clone();
    let flag = went_on.clone();
    press(&mut tree, move |c| {
        let flag = flag.clone();
        stopping.stop_then(c, move |_| flag.set(true));
    });
    deliver_sent(&mut tree, &job, sent.try_iter().collect());

    assert!(staged.exists(), "the converter is still writing");
    assert!(
        !went_on.get(),
        "the quit went on while the converter was still writing"
    );
    assert!(job.is_running(), "the import has not ended");
    assert!(
        crate::shell::open_registry::importing(&target),
        "the target is still held"
    );
    assert_eq!(
        notice(&archive),
        tr!(project_import_stopping()).resolve_now(),
        "the notice says the import is stopping"
    );

    std::fs::write(format!("{target}.release"), b"").unwrap();
    let completion =
        frontend::common::long_operation::lock_or_recover(&app_ctx.long_operation_manager)
            .completion_signal();
    assert!(completion.wait_for(&op, Some(Duration::from_secs(60))));
    deliver_sent(&mut tree, &job, sent.try_iter().collect());

    assert!(
        went_on.get(),
        "the quit goes on once the converter has stopped"
    );
    assert!(!job.is_running());
    assert!(!staged.exists(), "and nothing it wrote is left");
    assert!(!crate::shell::open_registry::importing(&target));
    let cancelled = archived(&archive)
        .into_iter()
        .find(|e| e.dedup_id.as_deref() == Some("import.test.writing"))
        .map(|e| e.title)
        .unwrap_or_default();
    assert_eq!(cancelled, tr!(project_import_cancelled()).resolve_now());
}

/// The last window closing ends the process without the quit sequence: the
/// converter is stopped and waited for once the event loop is over, and leaves
/// nothing behind.
#[test]
fn stopping_at_exit_waits_for_the_converter() {
    let _registry = IsolatedOpenRegistry::new();
    let dir = tempfile::tempdir().unwrap();
    let target = dir
        .path()
        .join("novel.skrib")
        .to_string_lossy()
        .into_owned();
    let staged = PathBuf::from(format!("{target}.importing"));
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let (mut tree, _archive) = tree_with_archive(&app_ctx);
    let op = start_still_writing(&mut tree, &job, &target);

    // The converter finishes its file a little after the exit began.
    let release = PathBuf::from(format!("{target}.release"));
    let finishing = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(200));
        std::fs::write(release, b"")
    });
    assert!(job.stop_at_exit(Duration::from_secs(60)), "it stopped");
    assert!(!staged.exists(), "nothing it wrote is left");
    assert!(
        !crate::shell::open_registry::importing(&target),
        "and the target is let go of"
    );
    assert_eq!(
        long_operation_commands::get_operation_status(&app_ctx, &op),
        Some(frontend::common::long_operation::OperationStatus::Cancelled)
    );
    assert!(finishing.join().is_ok_and(|written| written.is_ok()));

    // With nothing running, there is nothing to wait for.
    let idle = ProjectImportJob::new(app_ctx);
    assert!(idle.stop_at_exit(Duration::ZERO));
}

/// Two running copies of Skribisto (one started with `--new-instance`) importing
/// into one target: each looks for another import when Import is pressed, then
/// claims the target. Another copy's claim landing between the look and the
/// claim was not looked for again, so both went on to write the same
/// `<target>.importing`, and the first to finish moved the other's half-written
/// file into place. Claiming and then looking again, as a load does, means
/// whichever copy looks second backs out.
#[test]
fn an_import_another_copy_started_meanwhile_is_not_joined() {
    let _registry = IsolatedOpenRegistry::new();
    let dir = tempfile::tempdir().unwrap();
    let target = dir
        .path()
        .join("novel.skrib")
        .to_string_lossy()
        .into_owned();
    let app_ctx = Rc::new(AppContext::new());
    let job = ProjectImportJob::new(app_ctx.clone());
    let (mut tree, _archive) = tree_with_archive(&app_ctx);

    // The other copy claimed the target after this one looked (Import pressed,
    // or the overwrite question answered) and before it claims it itself.
    crate::shell::open_registry::pretend_another_copy_imports(999_311, &target);
    let starting = job.clone();
    let request = target.clone();
    press(&mut tree, move |c| {
        starting.start(c, &STILL_WRITING, request.clone())
    });

    assert!(!job.is_running(), "this copy backs out");
    assert_eq!(
        drain_dialog_titles(&mut tree),
        vec![tr!(target_importing_title()).resolve_now()],
        "and says why"
    );
    assert!(!Path::new(&format!("{target}.importing")).exists());
    std::fs::write(format!("{target}.release"), b"").unwrap();
}
