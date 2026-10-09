// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scrivener importer's inspection: reading a project before anything is written, and
//! planning its structure again whenever the writer changes a choice.
//!
//! A Layer-A handle, and the data seam of the Scrivener wizard. It owns the **whole**
//! lifecycle of the read-only `analyze_scrivener_project` long operation (start, the four
//! long-operation events, the result, Stop reading) and publishes one
//! [`Signal<InspectState>`](InspectState) the view-model reads. The view-model therefore walks
//! the same state machine in both builds and parses no long-operation event itself.
//!
//! Two `#[cfg]`-gated `mod imp` variants of the same type, identical signatures:
//!
//! * **real**: the long operation through `frontend`, and the review's synchronous
//!   recomputation through `import_management::scrivener::ScrivenerReview`, opened once per
//!   analysis from its `review_data` (the converter's own mapping function, so the preview
//!   and the import cannot disagree);
//! * **mock**: fabricated analyses, chosen by words in the picked path's last component, that
//!   cover every tier the wizard shows: refusals (`refuse`, `cloud`), questions to choose
//!   (`several`, `manifests`, `backup`), questions to accept (`ipad`, `empty`), and a
//!   readable novel with notices to know (anything else). Its recomputation applies the
//!   choices to the fabricated plan. The novel's draft target, deadline and status list are
//!   chosen by words too (`pages`, `characters`, `overdue`, `lastday`, `undated`,
//!   `statusless`), and its binder rows are named from one table the fabricated import
//!   (`models::scrivener_import_run`) reads as well. A project whose fabricated import
//!   failed on `question` or `unaccepted` is read again with that question asked.
//!
//! **Never log or `{:?}` an analysis.** Its `review_data` keeps the raw folder name and member
//! paths, where a sync copy's file name can name a person; `InspectState`'s own `Debug`
//! withholds it.

use std::rc::Rc;

use frontend::import_management::ScrivenerAnalysisDto;

/// Where the inspection of the project the writer chose stands.
#[derive(Clone)]
pub enum InspectState {
    /// Nothing chosen yet, or the reading was stopped.
    Idle,
    /// The project is being read. `percent` runs from 0 to 1.
    Reading { percent: f32 },
    /// The analysis came back: a project that reads, a question, or a refusal (the
    /// analysis's own `outcome` says which).
    Read(Rc<ScrivenerAnalysisDto>),
    /// The reading itself failed, which a refusal never does: the technical account, for
    /// **Details**.
    Failed(Rc<str>),
}

impl std::fmt::Debug for InspectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InspectState::Idle => f.write_str("Idle"),
            InspectState::Reading { percent } => {
                f.debug_struct("Reading").field("percent", percent).finish()
            }
            // The analysis carries the writer's file names: never printed.
            InspectState::Read(_) => f.write_str("Read(<analysis withheld>)"),
            InspectState::Failed(_) => f.write_str("Failed(<details withheld>)"),
        }
    }
}

impl InspectState {
    /// The analysis, when one came back.
    pub fn analysis(&self) -> Option<Rc<ScrivenerAnalysisDto>> {
        match self {
            InspectState::Read(analysis) => Some(analysis.clone()),
            _ => None,
        }
    }
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::RefCell;
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{import_management_commands, long_operation_commands};
    use frontend::common::event::{Event, LongOperationEvent, Origin};
    use frontend::import_management::failure::{FailureReason, ProjectImportFailure};
    use frontend::import_management::scrivener::{ScrivenerReview, ScrivenerReviewError};
    use frontend::import_management::{
        AnalyzeScrivenerProjectDto, ScrivenerAnalysisDto, ScrivenerChoice, ScrivenerStructure,
    };

    use super::InspectState;
    use crate::shared::long_op::{parse_payload, payload_id};

    /// A review opened from an analysis's review data, or why it would not open.
    type OpenedReview = Result<Rc<ScrivenerReview>, ScrivenerReviewError>;

    /// The inspection of one wizard. Cheap to clone; clones share everything.
    #[derive(Clone)]
    pub struct ScrivenerInspection {
        inner: Rc<Inner>,
    }

    struct Inner {
        app_ctx: Rc<AppContext>,
        state: Signal<InspectState>,
        /// The long operation reading now, if one is.
        op: RefCell<Option<String>>,
        /// The review opened from the current analysis's `review_data`, with the analysis
        /// it was opened from: opened once, planned again on every choice.
        review: RefCell<Option<(Rc<ScrivenerAnalysisDto>, OpenedReview)>>,
    }

    impl ScrivenerInspection {
        pub fn new(app_ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    app_ctx,
                    state: Signal::new(InspectState::Idle),
                    op: RefCell::new(None),
                    review: RefCell::new(None),
                }),
            }
        }

        /// Where the inspection stands. A plain (mutable) signal: observe it.
        pub fn state(&self) -> Signal<InspectState> {
            self.inner.state.clone()
        }

        /// Whether a reading is under way.
        pub fn is_reading(&self) -> bool {
            self.inner.op.borrow().is_some()
        }

        /// Read the project `request` names, stopping any reading under way first: only the
        /// latest request is ever answered.
        pub fn start(&self, request: AnalyzeScrivenerProjectDto) {
            self.forget_running();
            *self.inner.review.borrow_mut() = None;
            match import_management_commands::analyze_scrivener_project(
                &self.inner.app_ctx,
                &request,
            ) {
                Ok(op) => {
                    *self.inner.op.borrow_mut() = Some(op);
                    self.inner.state.set(InspectState::Reading { percent: 0.0 });
                }
                Err(e) => self
                    .inner
                    .state
                    .set(InspectState::Failed(Rc::from(format!("{e:#}")))),
            }
        }

        /// Stop the reading under way, if any, and forget it: a late event for it finds
        /// nothing to answer. The analysis writes nothing, so nothing is left to clean up.
        pub fn stop(&self) {
            self.forget_running();
            self.inner.state.set(InspectState::Idle);
        }

        fn forget_running(&self) {
            let running = self.inner.op.borrow_mut().take();
            if let Some(op) = running {
                long_operation_commands::cancel_operation(&self.inner.app_ctx, &op);
            }
        }

        /// The structure the current analysis plans with `choices`, recomputed now, with no
        /// round trip to a long operation. `Unreadable` when nothing was read.
        pub fn plan(
            &self,
            choices: &[ScrivenerChoice],
        ) -> Result<ScrivenerStructure, ScrivenerReviewError> {
            let Some(analysis) = self.inner.state.get().analysis() else {
                return Err(ScrivenerReviewError::Unreadable);
            };
            let review = {
                let mut cached = self.inner.review.borrow_mut();
                let fresh = cached
                    .as_ref()
                    .is_some_and(|(for_analysis, _)| Rc::ptr_eq(for_analysis, &analysis));
                if !fresh {
                    let opened = ScrivenerReview::open(&analysis.review_data).map(Rc::new);
                    *cached = Some((analysis.clone(), opened));
                }
                match cached.as_ref() {
                    Some((_, Ok(review))) => review.clone(),
                    Some((_, Err(e))) => return Err(e.clone()),
                    None => return Err(ScrivenerReviewError::Unreadable),
                }
            };
            // On the writer's own calendar, as the import will plan a deadline.
            review.structure(choices, crate::shared::stamps::today())
        }

        /// Subscribe this window's tree to the four long-operation events. Called on every
        /// build of the widget that owns the wizard's view-model: the framework drops a
        /// rebuilt widget's subscriptions, so each build registers its own again.
        pub fn wire(&self, ctx: &mut BuildContext) {
            type Handler = fn(&ScrivenerInspection, &Event);
            const HANDLERS: &[(LongOperationEvent, Handler)] = &[
                (LongOperationEvent::Progress, |s, e| s.on_progress(e)),
                (LongOperationEvent::Completed, |s, e| s.on_completed(e)),
                (LongOperationEvent::Cancelled, |s, e| s.on_cancelled(e)),
                (LongOperationEvent::Failed, |s, e| s.on_failed(e)),
            ];
            for (event, handler) in HANDLERS {
                let me = self.clone();
                let handler = *handler;
                ctx.subscribe_event(Origin::LongOperation(event.clone()), move |e: &Event| {
                    handler(&me, e)
                });
            }
        }

        /// Whether `id` names the reading under way.
        fn is_mine(&self, id: Option<&str>) -> bool {
            id.is_some() && self.inner.op.borrow().as_deref() == id
        }

        fn on_progress(&self, event: &Event) {
            let Some(payload) = parse_payload(event) else {
                return;
            };
            if !self.is_mine(payload_id(&payload)) {
                return;
            }
            let percent = payload
                .get("percentage")
                .and_then(|p| p.as_f64())
                .unwrap_or(0.0) as f32;
            self.inner.state.set(InspectState::Reading {
                percent: (percent / 100.0).clamp(0.0, 1.0),
            });
        }

        fn on_completed(&self, event: &Event) {
            let Some(payload) = parse_payload(event) else {
                return;
            };
            if !self.is_mine(payload_id(&payload)) {
                return;
            }
            let Some(op) = self.inner.op.borrow_mut().take() else {
                return;
            };
            let state = match import_management_commands::get_analyze_scrivener_project_result(
                &self.inner.app_ctx,
                &op,
            ) {
                Ok(Some(analysis)) => InspectState::Read(Rc::new(analysis)),
                Ok(None) => InspectState::Failed(Rc::from("the analysis returned no result")),
                Err(e) => InspectState::Failed(Rc::from(format!("{e:#}"))),
            };
            self.inner.state.set(state);
        }

        fn on_cancelled(&self, event: &Event) {
            let Some(payload) = parse_payload(event) else {
                return;
            };
            if !self.is_mine(payload_id(&payload)) {
                return;
            }
            self.inner.op.borrow_mut().take();
            self.inner.state.set(InspectState::Idle);
        }

        fn on_failed(&self, event: &Event) {
            let Some(payload) = parse_payload(event) else {
                return;
            };
            if !self.is_mine(payload_id(&payload)) {
                return;
            }
            self.inner.op.borrow_mut().take();
            let message = payload
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or_default();
            let failure = ProjectImportFailure::from_any_message(message);
            let state = match failure.reason {
                FailureReason::Cancelled => InspectState::Idle,
                _ => InspectState::Failed(Rc::from(failure.details)),
            };
            self.inner.state.set(state);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::RefCell;
    use std::collections::BTreeSet;
    use std::rc::Rc;
    use std::sync::{Mutex, PoisonError};

    use chrono::{DateTime, Days, NaiveDate, Utc};
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::import_management::scrivener::ScrivenerReviewError;
    use frontend::import_management::{
        AnalyzeScrivenerProjectDto, ScrivenerAnalysisDto, ScrivenerAnalysisOutcome, ScrivenerArg,
        ScrivenerBinder, ScrivenerChoice, ScrivenerComputers, ScrivenerContainer,
        ScrivenerContainerRung, ScrivenerControl, ScrivenerDestination, ScrivenerFinding,
        ScrivenerFindingFamily, ScrivenerGeneration, ScrivenerItemRef, ScrivenerLanguageSource,
        ScrivenerManifestCandidate, ScrivenerMobileEdits, ScrivenerPlace, ScrivenerPlatform,
        ScrivenerPreviewRow, ScrivenerQuestion, ScrivenerReportGroup, ScrivenerShape,
        ScrivenerSiblingChoice, ScrivenerSignal, ScrivenerStructure, ScrivenerTextKind,
        ScrivenerTitleKind, ScrivenerTitleSource, ScrivenerTrashChoice,
    };

    use super::InspectState;

    /// The fabricated inspection of one wizard. Cheap to clone; clones share everything.
    #[derive(Clone)]
    pub struct ScrivenerInspection {
        state: Signal<InspectState>,
        /// The project last read, which every choice plans again.
        project: Rc<RefCell<Fabricated>>,
    }

    impl ScrivenerInspection {
        pub fn new(_app_ctx: Rc<AppContext>) -> Self {
            Self {
                state: Signal::new(InspectState::Idle),
                project: Rc::new(RefCell::new(Fabricated::default())),
            }
        }

        pub fn state(&self) -> Signal<InspectState> {
            self.state.clone()
        }

        pub fn is_reading(&self) -> bool {
            false
        }

        /// Answer at once with the analysis the picked path's name asks for.
        pub fn start(&self, request: AnalyzeScrivenerProjectDto) {
            *self.project.borrow_mut() = Fabricated::of(&request.source_path, request.today);
            self.state
                .set(InspectState::Read(Rc::new(fabricate(&request))));
        }

        pub fn stop(&self) {
            self.state.set(InspectState::Idle);
        }

        /// The fabricated plan with `choices` applied.
        pub fn plan(
            &self,
            choices: &[ScrivenerChoice],
        ) -> Result<ScrivenerStructure, ScrivenerReviewError> {
            match self.state.get().analysis() {
                Some(analysis) if analysis.outcome == ScrivenerAnalysisOutcome::Read => {
                    Ok(novel_structure(&self.project.borrow(), choices))
                }
                _ => Err(ScrivenerReviewError::Unreadable),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}
    }

    fn count(name: &str, value: i64) -> ScrivenerArg {
        ScrivenerArg::Count {
            name: name.to_string(),
            value,
        }
    }

    fn whole(
        key: &str,
        family: ScrivenerFindingFamily,
        args: Vec<ScrivenerArg>,
    ) -> ScrivenerFinding {
        ScrivenerFinding::Reported {
            key: key.to_string(),
            family,
            group: ScrivenerReportGroup::Unfiled,
            place: ScrivenerPlace::Project,
            item: -1,
            binder_path: Vec::new(),
            item_uid: None,
            args,
        }
    }

    fn token_arg(name: &str, value: &str) -> ScrivenerArg {
        ScrivenerArg::Token {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// A finding of the metadata mapping, about the project as a whole, filed as the
    /// converter files it.
    fn metadata(
        key: &str,
        group: ScrivenerReportGroup,
        args: Vec<ScrivenerArg>,
    ) -> ScrivenerFinding {
        ScrivenerFinding::Reported {
            key: key.to_string(),
            family: ScrivenerFindingFamily::Metadata,
            group,
            place: ScrivenerPlace::Project,
            item: -1,
            binder_path: Vec::new(),
            item_uid: None,
            args,
        }
    }

    /// A finding of the structure mapping about the binder row `of`.
    fn at_item(
        key: &str,
        group: ScrivenerReportGroup,
        of: &FabricatedItem,
        args: Vec<ScrivenerArg>,
    ) -> ScrivenerFinding {
        ScrivenerFinding::Reported {
            key: key.to_string(),
            family: ScrivenerFindingFamily::Structure,
            group,
            place: ScrivenerPlace::Item,
            item: of.item,
            binder_path: of.path.clone(),
            item_uid: None,
            args,
        }
    }

    /// The analysis a path asks for, by the words in its last component.
    fn fabricate(request: &AnalyzeScrivenerProjectDto) -> ScrivenerAnalysisDto {
        let name = std::path::Path::new(&request.source_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let has = |word: &str| name.contains(word);
        if has("refuse") {
            return refused(whole(
                "scrivener-refusal-scrivener-one-for-mac",
                ScrivenerFindingFamily::Refusal,
                Vec::new(),
            ));
        }
        if has("cloud") {
            return refused(whole(
                "scrivener-refusal-cloud-placeholders",
                ScrivenerFindingFamily::Refusal,
                vec![
                    count("count", 3),
                    ScrivenerArg::Texts {
                        name: "examples".into(),
                        values: vec!["Files/Data/A/content.rtf".into()],
                    },
                ],
            ));
        }
        if has("several") && request.chosen_project.is_empty() {
            return choose(ScrivenerQuestion::SeveralProjects {
                candidates: vec![
                    "Book One.scriv".into(),
                    "Book Two.scriv".into(),
                    "Short stories/Winter.scriv".into(),
                ],
            });
        }
        // What a failed import said had happened since its review, this look sees.
        let failed_before = import_failed_before(&request.source_path);
        if (has("manifests") || (has("question") && failed_before))
            && request.chosen_manifest.is_empty()
        {
            return choose(several_manifests());
        }
        if (has("backup") || has("upgrade"))
            && request.upgraded_sibling == ScrivenerSiblingChoice::Unanswered
        {
            return choose(ScrivenerQuestion::UpgradedSibling {
                backup: "Novel Backup.scriv".into(),
                sibling: "Novel.scriv".into(),
                sibling_generation: Some(ScrivenerGeneration::Format20),
                preselect_sibling: true,
            });
        }
        let mut analysis = novel(&Fabricated::of(&request.source_path, request.today));
        analysis.project_path = located(request);
        if request.upgraded_sibling == ScrivenerSiblingChoice::Backup {
            analysis.notices.push(whole(
                "scrivener-notice-importing-pre-upgrade-copy",
                ScrivenerFindingFamily::Source,
                Vec::new(),
            ));
        }
        if (has("ipad") || (has("unaccepted") && failed_before))
            && !request.accept_unmerged_mobile_edits
        {
            analysis
                .pending
                .push(ScrivenerQuestion::UnmergedMobileEdits {
                    source: ScrivenerMobileEdits::MobileFolder,
                });
        }
        if has("empty") && !request.accept_no_text {
            analysis
                .pending
                .push(ScrivenerQuestion::NoTextInCopy { text_items: 24 });
        }
        analysis
    }

    /// Where the fabricated project is, as the real analysis says it: the project chosen
    /// below the picked folder, the upgraded project beside a pre-upgrade copy, else the
    /// pick itself.
    fn located(request: &AnalyzeScrivenerProjectDto) -> String {
        let source = std::path::Path::new(&request.source_path);
        let located = if !request.chosen_project.is_empty() {
            source.join(&request.chosen_project)
        } else if request.upgraded_sibling == ScrivenerSiblingChoice::Sibling {
            source.with_file_name("Novel.scriv")
        } else {
            source.to_path_buf()
        };
        located.to_string_lossy().into_owned()
    }

    fn manifest(
        name: &str,
        modified: Option<i64>,
        items: Option<i64>,
    ) -> ScrivenerManifestCandidate {
        ScrivenerManifestCandidate::Candidate {
            name: name.into(),
            creator: Some("SCRMAC-3.3.6-17321".into()),
            version: Some("2.0".into()),
            modified,
            items,
            matches_autosave: false,
            identical_to_preselected: false,
        }
    }

    fn refused(refusal: ScrivenerFinding) -> ScrivenerAnalysisDto {
        ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Refused,
            refusal,
            ..ScrivenerAnalysisDto::default()
        }
    }

    fn choose(question: ScrivenerQuestion) -> ScrivenerAnalysisDto {
        ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Choose,
            question,
            ..ScrivenerAnalysisDto::default()
        }
    }

    /// The question a project with several manifests asks: which one is the project.
    pub(crate) fn several_manifests() -> ScrivenerQuestion {
        ScrivenerQuestion::SeveralManifests {
            candidates: vec![
                manifest("Novel.scrivx", Some(1_717_236_000), Some(86)),
                manifest(
                    "Novel (conflicted copy).scrivx",
                    Some(1_717_100_000),
                    Some(84),
                ),
                manifest("Novel 2.scrivx", None, None),
            ],
            preselected: None,
            evidence: Vec::new(),
        }
    }

    /// The projects whose fabricated import failed on a word of their path, by the path
    /// the writer picked. What the word stands for happened between that review and that
    /// import: the next look at the project sees it (a manifest to choose, edits to
    /// accept), and the next import does not meet it again.
    static FAILED_IMPORTS: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

    /// The fabricated import of the project picked at `source` failed on a word of its path.
    pub(crate) fn remember_failed_import(source: &str) {
        FAILED_IMPORTS
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(source.to_string());
    }

    /// Whether a fabricated import of the project picked at `source` already failed on a
    /// word of its path.
    pub(crate) fn import_failed_before(source: &str) -> bool {
        FAILED_IMPORTS
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .contains(source)
    }

    /// A readable novel: a part, chapters and scenes in the Draft, characters and research
    /// beside it, and the notices a writer meets most.
    fn novel(project: &Fabricated) -> ScrivenerAnalysisDto {
        ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Read,
            notices: vec![
                whole(
                    "scrivener-notice-lock-present",
                    ScrivenerFindingFamily::Source,
                    vec![ScrivenerArg::Flag {
                        name: "this_machine".into(),
                        value: false,
                    }],
                ),
                whole(
                    "scrivener-notice-sync-debris-ignored",
                    ScrivenerFindingFamily::Source,
                    vec![count("count", 4)],
                ),
                whole(
                    "scrivener-notice-snapshots-not-imported",
                    ScrivenerFindingFamily::Source,
                    vec![
                        count("snapshots", 12),
                        count("documents", 5),
                        count("of_deleted", 0),
                    ],
                ),
                whole(
                    "scrivener-notice-writing-history-not-imported",
                    ScrivenerFindingFamily::Source,
                    vec![count("days", 41), count("unreadable", 0)],
                ),
                whole(
                    "scrivener-notice-title-from-project",
                    ScrivenerFindingFamily::Source,
                    vec![ScrivenerArg::Token {
                        name: "source".into(),
                        value: "compile-settings".into(),
                    }],
                ),
                whole(
                    "scrivener-notice-language-not-stated",
                    ScrivenerFindingFamily::Source,
                    Vec::new(),
                ),
            ],
            folder_title: "The Lighthouse Keeper".into(),
            container: ScrivenerContainer::Folder,
            generation: ScrivenerGeneration::Format20,
            platform: ScrivenerPlatform::Mac,
            writer_version: "3.3.6".into(),
            creator: "SCRMAC-3.3.6-17321".into(),
            manifest: "The Lighthouse Keeper.scrivx".into(),
            modified: Some(1_717_236_000),
            modified_offset_minutes: Some(120),
            items: 31,
            documents: 22,
            manuscript_documents: 12,
            documents_present: 22,
            research_files: 2,
            trashed_items: 2,
            snapshots: 12,
            computers: ScrivenerComputers::Named {
                device: Some("Studio Mac".into()),
                lock_host: Some("Studio Mac".into()),
            },
            proposed_title: "The Lighthouse Keeper".into(),
            proposed_title_source: ScrivenerTitleSource::CompileSettings,
            proposed_authors: vec!["Ada Marlowe".into()],
            proposed_language_source: ScrivenerLanguageSource::NotStated,
            structure: novel_structure(project, &[]),
            fingerprint: "mock".into(),
            ..ScrivenerAnalysisDto::default()
        }
    }

    // ── the fabricated project ───────────────────────────────────────────────────────────

    // The binder rows of the fabricated project, by their place in Scrivener's binder.
    pub(crate) const DRAFT: &str = "Draft";
    const CHARACTERS: i64 = 20;
    pub(crate) const RESEARCH: i64 = 24;
    pub(crate) const IDEAS: i64 = 27;
    const SCENE_DEPTH: i64 = 3;
    const PARTS: [&str; 2] = ["Part One: The Light", "Part Two: The Storm"];
    /// A title that only names its level: the import stores it blank and the binder
    /// numbers the row.
    const PLACEHOLDER_TITLE: &str = "Chapter";
    /// The chapters' titles in Scrivener, in binder order, two to a part.
    const CHAPTERS: [&str; 4] = ["Chapter 1", PLACEHOLDER_TITLE, "Chapter 3", "Chapter 4"];
    const SCENES: [&str; 3] = ["Scene 1", "Scene 2", "Scene 3"];
    /// The root folders beside the Draft, each with its documents' titles.
    const ROOTS: [(i64, &str, &[&str]); 3] = [
        (CHARACTERS, "Characters", &["Mara Varga", "Tomas Reed"]),
        (
            RESEARCH,
            "Research",
            &["Harbour chart", "Foghorn recording"],
        ),
        (
            IDEAS,
            "Old ideas",
            &[
                "A ghost in the lamp room",
                "A second narrator",
                "A winter ending",
            ],
        ),
    ];
    /// What the writer deleted, in Scrivener's Trash after the root folders' documents.
    const TRASH: &str = "Trash";
    const TRASHED: [&str; 2] = ["Deleted scene", "Old opening"];
    const FIRST_TRASHED: i64 = 31;

    /// One binder row of the fabricated project: its number in Scrivener's binder and
    /// where the writer knew it there. The one table of names the fabricated review and the
    /// fabricated import both read, so a row is named alike on every page and every line.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) struct FabricatedItem {
        pub(crate) item: i64,
        pub(crate) path: Vec<String>,
    }

    impl FabricatedItem {
        fn new(item: i64, path: &[&str]) -> Self {
            Self {
                item,
                path: path.iter().map(|p| p.to_string()).collect(),
            }
        }

        /// Its title in Scrivener.
        pub(crate) fn title(&self) -> &str {
            self.path.last().map(String::as_str).unwrap_or_default()
        }

        /// Whether its title only named its level, which the import stores blank.
        fn placeholder(&self) -> bool {
            self.title() == PLACEHOLDER_TITLE
        }
    }

    /// Every binder row of the fabricated project, in binder order: the Draft's parts,
    /// chapters and scenes, the root folders and their documents, then the Trash.
    pub(crate) fn fabricated_binder() -> Vec<FabricatedItem> {
        let mut items = Vec::new();
        let mut item = 1;
        let mut chapters = CHAPTERS.iter();
        for part in PARTS {
            items.push(FabricatedItem::new(item, &[DRAFT, part]));
            item += 1;
            for chapter in chapters.by_ref().take(2) {
                items.push(FabricatedItem::new(item, &[DRAFT, part, chapter]));
                item += 1;
                for scene in SCENES {
                    items.push(FabricatedItem::new(item, &[DRAFT, part, chapter, scene]));
                    item += 1;
                }
            }
        }
        for (root, title, children) in ROOTS {
            items.push(FabricatedItem::new(root, &[title]));
            for (n, child) in (1..).zip(children) {
                items.push(FabricatedItem::new(root + n, &[title, child]));
            }
        }
        for (n, title) in (0..).zip(TRASHED) {
            items.push(FabricatedItem::new(FIRST_TRASHED + n, &[TRASH, title]));
        }
        items
    }

    /// The binder row `item` of the fabricated project.
    pub(crate) fn fabricated_item(item: i64) -> Option<FabricatedItem> {
        fabricated_binder().into_iter().find(|i| i.item == item)
    }

    /// The documents directly below the root folder or the Trash titled `title`.
    fn below(binder: &[FabricatedItem], title: &str) -> Vec<FabricatedItem> {
        binder
            .iter()
            .filter(|i| i.path.len() == 2 && i.path[0] == title)
            .cloned()
            .collect()
    }

    /// What the fabricated project holds that a project has only one of (one draft target,
    /// one deadline, a status list or none), chosen like every other case by a word in the
    /// last component of the picked path: `pages` (a draft target in pages), `characters`
    /// (in characters), `overdue` (a deadline already past), `lastday` (a deadline that is
    /// the day of the import), `undated` (a deadline that cannot be read) and `statusless`
    /// (no status list). Anything else has a draft target in words, a deadline far ahead
    /// and a status list. The review plans it and the import reports it from here, so the
    /// two cannot disagree.
    #[derive(Clone, Debug, Default)]
    pub(crate) struct Fabricated {
        /// The last component of the picked path, lower-cased.
        name: String,
        /// The writer's day: the import plans from it, as the converter does.
        today: NaiveDate,
    }

    /// A draft target: its count, and its unit as the converter's token names it.
    #[derive(Clone, Copy)]
    struct DraftTarget {
        value: i64,
        unit: &'static str,
    }

    impl Fabricated {
        /// The project picked at `source_path`, read on the writer's day `today`.
        pub(crate) fn of(source_path: &str, today: DateTime<Utc>) -> Self {
            Self {
                name: std::path::Path::new(source_path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_lowercase())
                    .unwrap_or_default(),
                today: today.date_naive(),
            }
        }

        fn has(&self, word: &str) -> bool {
            self.name.contains(word)
        }

        /// Whether the project has a status list.
        pub(crate) fn has_statuses(&self) -> bool {
            !self.has("statusless")
        }

        fn draft_target(&self) -> DraftTarget {
            if self.has("pages") {
                DraftTarget {
                    value: 320,
                    unit: "pages",
                }
            } else if self.has("characters") {
                DraftTarget {
                    value: 480_000,
                    unit: "characters",
                }
            } else {
                DraftTarget {
                    value: 90_000,
                    unit: "words",
                }
            }
        }

        /// The deadline's day, `None` when it cannot be read.
        fn deadline(&self) -> Option<NaiveDate> {
            if self.has("undated") {
                None
            } else if self.has("overdue") {
                // Two months before the import, whatever day it is.
                self.today.checked_sub_days(Days::new(61))
            } else if self.has("lastday") {
                Some(self.today)
            } else {
                NaiveDate::from_ymd_opt(2099, 12, 1)
            }
        }
    }

    fn text_arg(name: &str, value: &str) -> ScrivenerArg {
        ScrivenerArg::Text {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    fn texts_arg(name: &str, values: &[&str]) -> ScrivenerArg {
        ScrivenerArg::Texts {
            name: name.to_string(),
            values: values.iter().map(|v| v.to_string()).collect(),
        }
    }

    /// A calendar day as the converter gives it, `YYYY-MM-DD`: a date's own spelling, read
    /// on no clock.
    fn day_arg(day: NaiveDate) -> ScrivenerArg {
        ScrivenerArg::Day {
            name: "day".to_string(),
            value: day.to_string(),
        }
    }

    /// What the metadata mapping foresees for `project` when its manuscript holds `books`
    /// books: every notice the converter gives before the import, filed as it files them.
    fn foreseen(project: &Fabricated, books: usize) -> Vec<ScrivenerFinding> {
        use ScrivenerReportGroup::{
            BroughtAcrossDifferently as Differently, CheckThese as Check, LeftBehind as Left,
        };
        let characters = ROOTS[0].2;
        let single_book = books == 1;
        let target = project.draft_target();
        let mut notices = vec![
            // Check these.
            metadata(
                "scrivener-metadata-pov-hint",
                Check,
                vec![
                    texts_arg("values", characters),
                    texts_arg("matches", characters),
                ],
            ),
            metadata(
                "scrivener-metadata-goals-in-other-unit",
                Check,
                vec![count("count", 2)],
            ),
            metadata(
                "scrivener-metadata-script-mode-rows",
                Check,
                vec![count("rows", 1)],
            ),
            // Brought across differently.
            metadata(
                "scrivener-metadata-keywords-became-tags",
                Differently,
                vec![count("tags", 4), count("rows", 9)],
            ),
            metadata(
                "scrivener-metadata-keyword-hierarchy-flattened",
                Differently,
                vec![count("keywords", 3)],
            ),
            metadata(
                "scrivener-metadata-fields-became-tags",
                Differently,
                vec![count("fields", 4), count("tags", 9), count("rows", 12)],
            ),
            metadata(
                "scrivener-metadata-collections-became-tags",
                Differently,
                vec![count("tags", 2), count("rows", 5)],
            ),
            metadata(
                "scrivener-metadata-collection-order-lost",
                Differently,
                vec![texts_arg("names", &["Harbour", "Night scenes"])],
            ),
            metadata(
                "scrivener-metadata-bookmarks-became-links",
                Differently,
                vec![count("count", 5)],
            ),
            metadata(
                "scrivener-metadata-goals-kept",
                Differently,
                vec![count("rows", 6)],
            ),
            metadata(
                "scrivener-metadata-dictionary-words",
                Differently,
                vec![count("count", 12)],
            ),
            metadata(
                "scrivener-metadata-targets-on-notes",
                Differently,
                vec![count("count", 2)],
            ),
            metadata(
                "scrivener-metadata-targets-on-split-documents",
                Differently,
                vec![count("count", 1)],
            ),
            metadata(
                "scrivener-metadata-bookmark-ends-without-row",
                Differently,
                vec![count("count", 1)],
            ),
            // Left behind.
            metadata(
                "scrivener-metadata-unused-keywords-skipped",
                Left,
                vec![texts_arg("names", &["Unused"])],
            ),
            metadata(
                "scrivener-metadata-undefined-custom-field",
                Left,
                vec![texts_arg("ids", &["season", "act"]), count("values", 3)],
            ),
            metadata(
                "scrivener-metadata-empty-collections-skipped",
                Left,
                vec![texts_arg("names", &["To read aloud"])],
            ),
            metadata(
                "scrivener-metadata-bookmarks-unresolved",
                Left,
                vec![count("count", 2)],
            ),
            metadata(
                "scrivener-metadata-targets-in-pages",
                Left,
                vec![count("count", 1)],
            ),
            metadata(
                "scrivener-metadata-targets-unreadable",
                Left,
                vec![count("count", 1)],
            ),
            metadata(
                "scrivener-metadata-session-target-not-imported",
                Left,
                vec![count("value", 1_500), token_arg("unit", "words")],
            ),
            metadata(
                "scrivener-metadata-icons-dropped",
                Left,
                vec![count("rows", 2), texts_arg("names", &["Book (Blue Book)"])],
            ),
        ];
        // One line per field, per collection: two of each.
        for field in ["Needs research", "Revisit"] {
            notices.push(metadata(
                "scrivener-metadata-checkbox-default-unknown",
                Check,
                vec![text_arg("field", field)],
            ));
        }
        for (field, values) in [("Mood", 2), ("Weather", 1)] {
            notices.push(metadata(
                "scrivener-metadata-list-value-invalid",
                Left,
                vec![text_arg("field", field), count("count", values)],
            ));
        }
        for (name, missing) in [("Harbour", 1), ("Night scenes", 2)] {
            notices.push(metadata(
                "scrivener-metadata-collection-members-missing",
                Left,
                vec![text_arg("name", name), count("count", missing)],
            ));
        }
        // The labels and statuses with no name, and the metadata of rows the new project
        // does not have, one line per field: a status only where there is a status list.
        let statuses = project.has_statuses();
        for (field, id) in [("label", 7), ("status", 4)] {
            if field != "status" || statuses {
                notices.push(metadata(
                    "scrivener-metadata-unnamed-value",
                    Differently,
                    vec![token_arg("field", field), count("id", id)],
                ));
            }
        }
        for (field, items) in [
            ("label", 2),
            ("status", 2),
            ("keyword", 3),
            ("custom-field", 1),
            ("bookmark", 1),
            ("target", 1),
        ] {
            if field != "status" || statuses {
                notices.push(metadata(
                    "scrivener-metadata-values-without-row",
                    Left,
                    vec![token_arg("field", field), count("items", items)],
                ));
            }
        }

        // The draft target: the book's goal when there is one book and it counts no pages.
        let target_args = |reason: Option<&str>| {
            let mut args: Vec<ScrivenerArg> = reason
                .map(|reason| token_arg("reason", reason))
                .into_iter()
                .collect();
            args.extend([count("value", target.value), token_arg("unit", target.unit)]);
            args
        };
        if target.unit == "pages" {
            notices.push(metadata(
                "scrivener-metadata-draft-target-not-kept",
                Left,
                target_args(Some("pages")),
            ));
        } else if !single_book {
            notices.push(metadata(
                "scrivener-metadata-draft-target-not-kept",
                Left,
                target_args(Some("no-single-book")),
            ));
        } else {
            notices.push(metadata(
                "scrivener-metadata-draft-target-kept",
                Differently,
                target_args(None),
            ));
            notices.push(metadata(
                "scrivener-metadata-counting-differs",
                Check,
                Vec::new(),
            ));
        }

        // The deadline: the book's pace plan, unless something stands in its way.
        let deadline = project.deadline();
        let blocked = match deadline {
            None => Some("unreadable"),
            Some(_) if !single_book => Some("no-single-book"),
            Some(day) if day < project.today => Some("passed"),
            Some(day) if day == project.today => Some("no-writing-day-left"),
            Some(_) if target.unit == "pages" => Some("no-word-goal"),
            Some(_) if target.unit == "characters" => Some("character-goal"),
            Some(_) => None,
        };
        match (blocked, deadline) {
            (Some(reason), day) => {
                let mut args = vec![token_arg("reason", reason)];
                args.extend(day.map(day_arg));
                notices.push(metadata(
                    "scrivener-metadata-deadline-not-planned",
                    Left,
                    args,
                ));
            }
            (None, Some(day)) => {
                notices.push(metadata(
                    "scrivener-metadata-pace-planned",
                    Differently,
                    vec![
                        day_arg(day),
                        ScrivenerArg::Flag {
                            name: "day_off".into(),
                            value: true,
                        },
                    ],
                ));
                notices.push(metadata(
                    "scrivener-metadata-writing-days-not-read",
                    Check,
                    vec![text_arg("raw", "Mon Wed Fri")],
                ));
            }
            (None, None) => {}
        }
        notices
    }

    fn row(
        binder: ScrivenerBinder,
        depth: i64,
        kind: (BinderItemRole, BinderItemSubRole),
        title: &str,
        item: i64,
    ) -> ScrivenerPreviewRow {
        ScrivenerPreviewRow::Row {
            binder,
            depth,
            role: kind.0,
            sub_role: kind.1,
            title: title.into(),
            title_kind: ScrivenerTitleKind::Own,
            exportable: true,
            activated: true,
            item,
            why: ScrivenerSignal::Shape,
        }
    }

    fn item_ref(of: &FabricatedItem) -> ScrivenerItemRef {
        ScrivenerItemRef::Item {
            item: of.item,
            binder_path: of.path.clone(),
        }
    }

    /// The fabricated plan of `project` with `choices` applied, as the real review applies
    /// them onto what was detected.
    fn novel_structure(project: &Fabricated, choices: &[ScrivenerChoice]) -> ScrivenerStructure {
        use BinderItemRole as R;
        use BinderItemSubRole as S;
        use ScrivenerBinder::{Manuscript, Notes};

        let binder = fabricated_binder();
        let mut part_rung = ScrivenerContainerRung::Part;
        let mut chapter_rung = ScrivenerContainerRung::Chapter;
        let mut texts = ScrivenerTextKind::Scenes;
        let mut trash = ScrivenerTrashChoice::LeaveOut;
        let mut roots: Vec<(FabricatedItem, Vec<FabricatedItem>, ScrivenerDestination)> = ROOTS
            .iter()
            .map(|(item, title, _)| {
                let detected = if *item == CHARACTERS {
                    ScrivenerDestination::StoryBible
                } else {
                    ScrivenerDestination::Notes
                };
                (
                    FabricatedItem::new(*item, &[title]),
                    below(&binder, title),
                    detected,
                )
            })
            .collect();
        let detected: Vec<ScrivenerDestination> = roots.iter().map(|r| r.2.clone()).collect();
        for choice in choices {
            match choice {
                ScrivenerChoice::Containers { depth: 1, rung } => part_rung = rung.clone(),
                ScrivenerChoice::Containers { depth: 2, rung } => chapter_rung = rung.clone(),
                ScrivenerChoice::Texts { kind, .. } => texts = kind.clone(),
                ScrivenerChoice::Trash { choice } => trash = choice.clone(),
                ScrivenerChoice::Root { item, destination } => {
                    if let Some(root) = roots.iter_mut().find(|r| r.0.item == *item) {
                        root.2 = destination.clone();
                    }
                }
                _ => {}
            }
        }
        let folder = |rung: &ScrivenerContainerRung| match rung {
            ScrivenerContainerRung::Part => (R::Folder, S::Part),
            ScrivenerContainerRung::Chapter => (R::Folder, S::ChapterScene),
            ScrivenerContainerRung::Plain => (R::Folder, S::None),
        };
        let text = match texts {
            ScrivenerTextKind::Scenes => (R::Item, S::Scene),
            ScrivenerTextKind::Chapters => (R::Item, S::ChapterScene),
        };

        let mut rows = vec![ScrivenerPreviewRow::Row {
            binder: Manuscript,
            depth: 0,
            role: R::Folder,
            sub_role: S::Book,
            title: "The Lighthouse Keeper".into(),
            title_kind: ScrivenerTitleKind::ProjectTitle,
            exportable: true,
            activated: true,
            item: 0,
            why: ScrivenerSignal::Synthesised,
        }];
        let mut notices = Vec::new();
        let draft: Vec<&FabricatedItem> = binder
            .iter()
            .filter(|i| i.path.first().map(String::as_str) == Some(DRAFT))
            .collect();
        for entry in &draft {
            let depth = entry.path.len() as i64 - 1;
            let kind = match depth {
                1 => folder(&part_rung),
                2 => folder(&chapter_rung),
                _ => text.clone(),
            };
            let title = if entry.placeholder() {
                ""
            } else {
                entry.title()
            };
            rows.push(row(Manuscript, depth, kind, title, entry.item));
            if entry.placeholder() {
                notices.push(at_item(
                    "scrivener-structure-placeholder-title",
                    ScrivenerReportGroup::BroughtAcrossDifferently,
                    entry,
                    Vec::new(),
                ));
            }
        }
        let mut books = 1;
        for (root, children, destination) in &roots {
            let items = children.len() as i64;
            match destination {
                ScrivenerDestination::LeaveOut => notices.push(at_item(
                    "scrivener-structure-left-out",
                    ScrivenerReportGroup::LeftBehind,
                    root,
                    vec![count("items", items)],
                )),
                ScrivenerDestination::Manuscript => {
                    books += 1;
                    rows.push(row(
                        Manuscript,
                        0,
                        (R::Folder, S::Book),
                        root.title(),
                        root.item,
                    ));
                    for child in children {
                        rows.push(row(Manuscript, 1, text.clone(), child.title(), child.item));
                    }
                }
                ScrivenerDestination::Notes | ScrivenerDestination::StoryBible => {
                    rows.push(row(Notes, 0, (R::Folder, S::Note), root.title(), root.item));
                    for child in children {
                        rows.push(row(Notes, 1, (R::Item, S::Note), child.title(), child.item));
                    }
                    if *destination == ScrivenerDestination::StoryBible {
                        notices.push(at_item(
                            "scrivener-structure-story-bible-folder",
                            ScrivenerReportGroup::BroughtAcrossDifferently,
                            root,
                            Vec::new(),
                        ));
                    }
                }
            }
        }
        // What the project holds with no field to go to, listed in the "From Scrivener"
        // folder after the writer's own notes, one note per kind, named by its token.
        rows.push(ScrivenerPreviewRow::Row {
            binder: Notes,
            depth: 0,
            role: R::Folder,
            sub_role: S::Note,
            title: String::new(),
            title_kind: ScrivenerTitleKind::FromScrivener,
            exportable: false,
            activated: true,
            item: -1,
            why: ScrivenerSignal::Synthesised,
        });
        for (token, listed) in [("project", 3), ("custom-metadata", 4)] {
            rows.push(ScrivenerPreviewRow::Row {
                binder: Notes,
                depth: 1,
                role: R::Item,
                sub_role: S::Note,
                title: token.into(),
                title_kind: ScrivenerTitleKind::FromScrivenerNote,
                exportable: false,
                activated: true,
                item: -1,
                why: ScrivenerSignal::Synthesised,
            });
            notices.push(metadata(
                "scrivener-metadata-listed-in-import-note",
                ScrivenerReportGroup::BroughtAcrossDifferently,
                vec![token_arg("section", token), count("count", listed)],
            ));
        }
        // What the metadata mapping foresees beyond the folder: the tags, the goals, the
        // pace, the links and what has no row to go to.
        notices.extend(foreseen(project, books));
        let trashed = below(&binder, TRASH);
        let trashed_items = trashed.len() as i64;
        match trash {
            ScrivenerTrashChoice::Import => {
                for entry in &trashed {
                    rows.push(ScrivenerPreviewRow::Row {
                        binder: Manuscript,
                        depth: 1,
                        role: R::Item,
                        sub_role: S::Scene,
                        title: entry.title().into(),
                        title_kind: ScrivenerTitleKind::Own,
                        exportable: false,
                        activated: false,
                        item: entry.item,
                        why: ScrivenerSignal::Text,
                    });
                }
                notices.push(whole(
                    "scrivener-structure-trash-brought-across",
                    ScrivenerFindingFamily::Structure,
                    vec![count("items", trashed_items), count("with_writing", 1)],
                ));
            }
            ScrivenerTrashChoice::LeaveOut => notices.push(whole(
                "scrivener-structure-trash-left-out",
                ScrivenerFindingFamily::Structure,
                vec![count("items", trashed_items), count("with_writing", 1)],
            )),
        }

        let example = |depth: usize| draft.iter().find(|i| i.path.len() == depth + 1);
        let mut controls = vec![
            ScrivenerControl::Containers {
                depth: 1,
                containers: 2,
                adjustable: 2,
                detected: ScrivenerContainerRung::Part,
                mixed: false,
                chosen: (part_rung != ScrivenerContainerRung::Part).then_some(part_rung),
                examples: example(1).map(|i| item_ref(i)).into_iter().collect(),
                signals: Vec::new(),
                flags: Vec::new(),
            },
            ScrivenerControl::Containers {
                depth: 2,
                containers: 4,
                adjustable: 4,
                detected: ScrivenerContainerRung::Chapter,
                mixed: false,
                chosen: (chapter_rung != ScrivenerContainerRung::Chapter).then_some(chapter_rung),
                examples: example(2).map(|i| item_ref(i)).into_iter().collect(),
                signals: Vec::new(),
                flags: Vec::new(),
            },
            ScrivenerControl::Texts {
                depth: SCENE_DEPTH,
                texts: 12,
                detected: ScrivenerTextKind::Scenes,
                chosen: (texts != ScrivenerTextKind::Scenes).then_some(texts),
                examples: Vec::new(),
                median_words: 1_840,
            },
        ];
        for ((root, children, destination), detected) in roots.iter().zip(detected) {
            controls.push(ScrivenerControl::Root {
                item: item_ref(root),
                items: children.len() as i64,
                chosen: (*destination != detected).then(|| destination.clone()),
                detected,
                looks_like_manuscript: root.item == IDEAS,
            });
        }
        controls.push(ScrivenerControl::Trash {
            items: trashed_items,
            with_writing: 1,
            chosen: trash.clone(),
        });

        ScrivenerStructure::Planned {
            rows,
            controls,
            set_aside_trashed: 0,
            set_aside_recovered: 0,
            shape: ScrivenerShape::Empty,
            chapter_encoding: Default::default(),
            templates: Vec::new(),
            tags: Vec::new(),
            notices,
            choices: vec![ScrivenerChoice::Trash { choice: trash }],
        }
    }

    /// The analysis this fabricated inspection gives `request`: what the fabricated import
    /// (`models::scrivener_import_run`) reads again, as the converter reads a real project
    /// again before it writes anything.
    pub(crate) fn fabricated_analysis(
        request: &AnalyzeScrivenerProjectDto,
    ) -> ScrivenerAnalysisDto {
        fabricate(request)
    }

    /// The plan this fabricated inspection gives `project` with `choices`, for the
    /// fabricated import.
    pub(crate) fn fabricated_plan(
        project: &Fabricated,
        choices: &[ScrivenerChoice],
    ) -> ScrivenerStructure {
        novel_structure(project, choices)
    }

    #[cfg(test)]
    pub(crate) fn fabricated_for_test(path: &str) -> ScrivenerAnalysisDto {
        fabricate(&AnalyzeScrivenerProjectDto {
            source_path: path.into(),
            ..AnalyzeScrivenerProjectDto::default()
        })
    }
}

pub use imp::ScrivenerInspection;

/// The fabricated reading and plan, for the fabricated import to read the same project again.
#[cfg(feature = "mocks")]
pub(crate) use imp::{
    DRAFT, Fabricated, FabricatedItem, RESEARCH, fabricated_analysis, fabricated_binder,
    fabricated_item, fabricated_plan, import_failed_before, remember_failed_import,
    several_manifests,
};

/// The fabricated analyses cover every tier the wizard shows, so the mock build and its
/// screenshots can walk each of them.
#[cfg(all(test, feature = "mocks"))]
mod mock_tests {
    use frontend::import_management::{
        ScrivenerAnalysisOutcome, ScrivenerQuestion, ScrivenerStructure,
    };

    use super::imp::fabricated_for_test as fabricated;
    use crate::import_scrivener::messages::{Tier, entry, key_of};

    #[test]
    fn the_fabricated_analyses_cover_every_tier() {
        assert_eq!(
            fabricated("/b/refuse.scriv").outcome,
            ScrivenerAnalysisOutcome::Refused
        );
        assert_eq!(
            fabricated("/b/cloud.scriv").outcome,
            ScrivenerAnalysisOutcome::Refused
        );
        for path in ["/b/several", "/b/manifests.scriv", "/b/Novel Backup.scriv"] {
            assert_eq!(
                fabricated(path).outcome,
                ScrivenerAnalysisOutcome::Choose,
                "{path}"
            );
        }
        assert!(matches!(
            fabricated("/b/ipad.scriv").pending.as_slice(),
            [ScrivenerQuestion::UnmergedMobileEdits { .. }]
        ));
        assert!(matches!(
            fabricated("/b/empty.scriv").pending.as_slice(),
            [ScrivenerQuestion::NoTextInCopy { .. }]
        ));
        let novel = fabricated("/b/Novel.scriv");
        assert_eq!(novel.outcome, ScrivenerAnalysisOutcome::Read);
        let tiers: Vec<Tier> = novel
            .notices
            .iter()
            .filter_map(|n| entry(key_of(n)).map(|e| e.tier))
            .collect();
        for tier in [Tier::Card, Tier::SaveHint, Tier::BeforeImport] {
            assert!(tiers.contains(&tier), "{tier:?} is not fabricated");
        }
        assert_eq!(tiers.len(), novel.notices.len(), "every notice is worded");
        // The plan foresees what the metadata mapping does, as the real one does, and
        // every one of it is worded for the "before you import" strip.
        let ScrivenerStructure::Planned { notices, .. } = &novel.structure else {
            panic!("the novel plans");
        };
        let metadata: Vec<&str> = notices
            .iter()
            .map(key_of)
            .filter(|k| k.starts_with("scrivener-metadata-"))
            .collect();
        assert!(metadata.len() >= 5, "{metadata:?}");
        for key in metadata {
            assert_eq!(
                entry(key).map(|e| e.tier),
                Some(Tier::BeforeImport),
                "{key}"
            );
        }
    }
}
