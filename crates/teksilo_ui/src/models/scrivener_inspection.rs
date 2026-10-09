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
//!   choices to the fabricated plan.
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
            review.structure(choices)
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
    use std::rc::Rc;

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
    }

    impl ScrivenerInspection {
        pub fn new(_app_ctx: Rc<AppContext>) -> Self {
            Self {
                state: Signal::new(InspectState::Idle),
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
                    Ok(novel_structure(choices))
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

    fn at_item(
        key: &str,
        group: ScrivenerReportGroup,
        item: i64,
        path: &[&str],
        args: Vec<ScrivenerArg>,
    ) -> ScrivenerFinding {
        ScrivenerFinding::Reported {
            key: key.to_string(),
            family: ScrivenerFindingFamily::Structure,
            group,
            place: ScrivenerPlace::Item,
            item,
            binder_path: path.iter().map(|p| p.to_string()).collect(),
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
        if has("manifests") && request.chosen_manifest.is_empty() {
            return choose(ScrivenerQuestion::SeveralManifests {
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
            });
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
        let mut analysis = novel();
        analysis.project_path = located(request);
        if request.upgraded_sibling == ScrivenerSiblingChoice::Backup {
            analysis.notices.push(whole(
                "scrivener-notice-importing-pre-upgrade-copy",
                ScrivenerFindingFamily::Source,
                Vec::new(),
            ));
        }
        if has("ipad") && !request.accept_unmerged_mobile_edits {
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

    /// A readable novel: a part, chapters and scenes in the Draft, characters and research
    /// beside it, and the notices a writer meets most.
    fn novel() -> ScrivenerAnalysisDto {
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
            structure: novel_structure(&[]),
            fingerprint: "mock".into(),
            ..ScrivenerAnalysisDto::default()
        }
    }

    // The binder rows of the fabricated project, by their place in Scrivener's binder.
    const CHARACTERS: i64 = 20;
    const RESEARCH: i64 = 24;
    const IDEAS: i64 = 27;
    const SCENE_DEPTH: i64 = 3;

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

    fn item_ref(item: i64, path: &[&str]) -> ScrivenerItemRef {
        ScrivenerItemRef::Item {
            item,
            binder_path: path.iter().map(|p| p.to_string()).collect(),
        }
    }

    /// The fabricated plan with `choices` applied, as the real review applies them onto
    /// what was detected.
    fn novel_structure(choices: &[ScrivenerChoice]) -> ScrivenerStructure {
        use BinderItemRole as R;
        use BinderItemSubRole as S;
        use ScrivenerBinder::{Manuscript, Notes};

        let mut part_rung = ScrivenerContainerRung::Part;
        let mut chapter_rung = ScrivenerContainerRung::Chapter;
        let mut texts = ScrivenerTextKind::Scenes;
        let mut trash = ScrivenerTrashChoice::LeaveOut;
        let mut roots = vec![
            (
                CHARACTERS,
                "Characters",
                2,
                ScrivenerDestination::StoryBible,
            ),
            (RESEARCH, "Research", 2, ScrivenerDestination::Notes),
            (IDEAS, "Old ideas", 3, ScrivenerDestination::Notes),
        ];
        let detected: Vec<ScrivenerDestination> = roots.iter().map(|r| r.3.clone()).collect();
        for choice in choices {
            match choice {
                ScrivenerChoice::Containers { depth: 1, rung } => part_rung = rung.clone(),
                ScrivenerChoice::Containers { depth: 2, rung } => chapter_rung = rung.clone(),
                ScrivenerChoice::Texts { kind, .. } => texts = kind.clone(),
                ScrivenerChoice::Trash { choice } => trash = choice.clone(),
                ScrivenerChoice::Root { item, destination } => {
                    if let Some(root) = roots.iter_mut().find(|r| r.0 == *item) {
                        root.3 = destination.clone();
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
        let mut item = 1;
        let mut chapter_number = 0;
        for part in ["Part One: The Light", "Part Two: The Storm"] {
            rows.push(row(Manuscript, 1, folder(&part_rung), part, item));
            item += 1;
            for _ in 0..2 {
                chapter_number += 1;
                rows.push(row(
                    Manuscript,
                    2,
                    folder(&chapter_rung),
                    &format!("Chapter {chapter_number}"),
                    item,
                ));
                item += 1;
                for scene in 1..=3 {
                    rows.push(row(
                        Manuscript,
                        SCENE_DEPTH,
                        text.clone(),
                        &format!("Scene {scene}"),
                        item,
                    ));
                    item += 1;
                }
            }
        }
        let mut notices = vec![at_item(
            "scrivener-structure-placeholder-title",
            ScrivenerReportGroup::BroughtAcrossDifferently,
            3,
            &["Draft", "Part One: The Light", "Chapter"],
            Vec::new(),
        )];
        for (root, title, items, destination) in &roots {
            match destination {
                ScrivenerDestination::LeaveOut => notices.push(at_item(
                    "scrivener-structure-left-out",
                    ScrivenerReportGroup::LeftBehind,
                    *root,
                    &[title],
                    vec![count("items", *items)],
                )),
                ScrivenerDestination::Manuscript => {
                    rows.push(row(Manuscript, 0, (R::Folder, S::Book), title, *root));
                    for n in 1..=*items {
                        rows.push(row(
                            Manuscript,
                            1,
                            text.clone(),
                            &format!("{title} {n}"),
                            root + n,
                        ));
                    }
                }
                ScrivenerDestination::Notes | ScrivenerDestination::StoryBible => {
                    rows.push(row(Notes, 0, (R::Folder, S::Note), title, *root));
                    for n in 1..=*items {
                        rows.push(row(
                            Notes,
                            1,
                            (R::Item, S::Note),
                            &format!("{title} {n}"),
                            root + n,
                        ));
                    }
                    if *destination == ScrivenerDestination::StoryBible {
                        notices.push(at_item(
                            "scrivener-structure-story-bible-folder",
                            ScrivenerReportGroup::BroughtAcrossDifferently,
                            *root,
                            &[title],
                            Vec::new(),
                        ));
                    }
                }
            }
        }
        match trash {
            ScrivenerTrashChoice::Import => {
                for (n, title) in ["Deleted scene", "Old opening"].iter().enumerate() {
                    rows.push(ScrivenerPreviewRow::Row {
                        binder: Manuscript,
                        depth: 1,
                        role: R::Item,
                        sub_role: S::Scene,
                        title: (*title).into(),
                        title_kind: ScrivenerTitleKind::Own,
                        exportable: false,
                        activated: false,
                        item: 30 + n as i64,
                        why: ScrivenerSignal::Text,
                    });
                }
                notices.push(whole(
                    "scrivener-structure-trash-brought-across",
                    ScrivenerFindingFamily::Structure,
                    vec![count("items", 2), count("with_writing", 1)],
                ));
            }
            ScrivenerTrashChoice::LeaveOut => notices.push(whole(
                "scrivener-structure-trash-left-out",
                ScrivenerFindingFamily::Structure,
                vec![count("items", 2), count("with_writing", 1)],
            )),
        }

        let mut controls = vec![
            ScrivenerControl::Containers {
                depth: 1,
                containers: 2,
                adjustable: 2,
                detected: ScrivenerContainerRung::Part,
                mixed: false,
                chosen: (part_rung != ScrivenerContainerRung::Part).then_some(part_rung),
                examples: vec![item_ref(1, &["Draft", "Part One: The Light"])],
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
                examples: vec![item_ref(2, &["Draft", "Part One: The Light", "Chapter 1"])],
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
        for ((root, title, items, destination), detected) in roots.iter().zip(detected) {
            controls.push(ScrivenerControl::Root {
                item: item_ref(*root, &[title]),
                items: *items,
                chosen: (*destination != detected).then(|| destination.clone()),
                detected,
                looks_like_manuscript: *root == IDEAS,
            });
        }
        controls.push(ScrivenerControl::Trash {
            items: 2,
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

    #[cfg(test)]
    pub(crate) fn fabricated_for_test(path: &str) -> ScrivenerAnalysisDto {
        fabricate(&AnalyzeScrivenerProjectDto {
            source_path: path.into(),
            ..AnalyzeScrivenerProjectDto::default()
        })
    }
}

pub use imp::ScrivenerInspection;

/// The fabricated analyses cover every tier the wizard shows, so the mock build and its
/// screenshots can walk each of them.
#[cfg(all(test, feature = "mocks"))]
mod mock_tests {
    use frontend::import_management::{ScrivenerAnalysisOutcome, ScrivenerQuestion};

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
    }
}
