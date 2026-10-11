// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scrivener importer's run: converting the reviewed project into a new `.skrib`.
//!
//! A Layer-A handle, and the second data seam of the Scrivener wizard
//! (`models::scrivener_inspection` is the first). The shared job
//! (`shared::project_import::ProjectImportJob`) starts an import through
//! [`ScrivenerImportRun::start`] and reads a finished one through
//! [`ScrivenerImportRun::result`]; everything between the two is the job's, on the four
//! long-operation events it already listens to, so the progress notice, Cancel, the result
//! with **Open now**, a failure worded in the writer's language and the report dialog behave
//! the same in both builds.
//!
//! Two `#[cfg]`-gated `mod imp` variants of the same type, identical signatures:
//!
//! * **real**: the `import_scrivener_project` long operation through `frontend`, and its
//!   result as the backend stored it.
//! * **mock**: a fabricated run started on the same long-operation manager, so it reaches the
//!   job through the same events: its phases (*Converting* counting rows done out of the
//!   total), the cancel flag looked at between every two steps, then a fabricated result or
//!   a typed failure. It reads the fabricated project again, as the converter reads the real
//!   one again (`models::scrivener_inspection`'s mock analysis and plan, with the
//!   writer's choices), and adds what only an import finds: a finding for every group and
//!   every family, a line listing more items than the dialog shows, and the research files,
//!   copied or not as the writer chose. Every row it names, it names from the inspection's
//!   one table of the fabricated binder, so the report and the Review page agree. Between
//!   the plan and the run, every metadata kind the converter has reaches a report; the ones
//!   a single project has only one of (its draft target, its deadline, its status list)
//!   follow the inspection's words and the writer's choices, never two at once. A failure
//!   is chosen by a word in the last component of the project's path, as the inspection's
//!   tiers are: `changed`, `question`, `unaccepted`, `depth`, `root`, `syncing` and `huge`.
//!   All but `huge` happen once per project: its next review sees what changed and its next
//!   import succeeds, so **Review the import again** leads to a project, as it does in the
//!   real build.
//!
//! **What the mock writes.** The new project, and nothing else: a small `.skrib` holding the
//! title, author and language the Save page sent, written through `skrib_format` with the
//! converter's own staged output, so an existing file is refused or replaced as the real
//! import does it, and **Open now** opens a project the mocks build can load. Its research
//! folder, like every line of its report, is fabricated and never made on disk.

#[cfg(not(feature = "mocks"))]
mod imp {
    use frontend::AppContext;
    use frontend::commands::import_management_commands;
    use frontend::import_management::{ImportScrivenerProjectDto, ImportScrivenerProjectResultDto};

    /// The import of a reviewed Scrivener project.
    pub struct ScrivenerImportRun;

    impl ScrivenerImportRun {
        /// Start the `import_scrivener_project` long operation, returning its id.
        pub fn start(
            app_ctx: &AppContext,
            request: &ImportScrivenerProjectDto,
        ) -> anyhow::Result<String> {
            import_management_commands::import_scrivener_project(app_ctx, request)
        }

        /// The result of the finished import `op_id`, or `None` when there is none to read.
        pub fn result(
            app_ctx: &AppContext,
            op_id: &str,
        ) -> Option<ImportScrivenerProjectResultDto> {
            import_management_commands::get_import_scrivener_project_result(app_ctx, op_id).ok()?
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};

    use frontend::AppContext;
    use frontend::commands::import_management_commands;
    use frontend::common::long_operation::{LongOperation, OperationProgress, lock_or_recover};
    use frontend::import_management::failure::{FailureReason, ProjectImportFailure};
    use frontend::import_management::{
        AnalyzeScrivenerProjectDto, ImportScrivenerProjectDto, ImportScrivenerProjectResultDto,
        ScrivenerAnalysisDto, ScrivenerAnalysisOutcome, ScrivenerArg, ScrivenerChoice,
        ScrivenerDestination, ScrivenerFinding, ScrivenerFindingFamily, ScrivenerMobileEdits,
        ScrivenerPlace, ScrivenerPreviewRow, ScrivenerQuestion, ScrivenerReportGroup,
        ScrivenerStructure, ScrivenerTotals, ScrivenerTrashChoice,
    };
    use skrib_format::importing::{OutputRefused, StagedOutput, fill_title};
    use skrib_format::{
        BinderFile, BundleKind, BundledBinder, FORMAT_VERSION, ProjectManifest, ShapeTag,
        WorkBundle, WorkFile,
    };

    use crate::models::scrivener_inspection::{
        DRAFT, Fabricated, FabricatedItem, RESEARCH, fabricated_analysis, fabricated_binder,
        fabricated_item, fabricated_plan, import_failed_before, remember_failed_import,
        several_manifests,
    };

    /// The fabricated import of a reviewed Scrivener project.
    pub struct ScrivenerImportRun;

    impl ScrivenerImportRun {
        /// Start the fabricated import on the long-operation manager every import runs on,
        /// returning its id: its events reach the job as a real import's do.
        pub fn start(
            app_ctx: &AppContext,
            request: &ImportScrivenerProjectDto,
        ) -> anyhow::Result<String> {
            let run = FabricatedRun {
                request: request.clone(),
                pace: PACE,
            };
            Ok(lock_or_recover(&app_ctx.long_operation_manager).start_operation(run))
        }

        /// The result the fabricated import `op_id` left with the manager, read back the way
        /// a real import's is.
        pub fn result(
            app_ctx: &AppContext,
            op_id: &str,
        ) -> Option<ImportScrivenerProjectResultDto> {
            import_management_commands::get_import_scrivener_project_result(app_ctx, op_id).ok()?
        }
    }

    /// How long each step of the fabricated import takes: long enough in the running app to
    /// see the progress notice move and to press its Cancel, short under test.
    const PACE: Duration = if cfg!(test) {
        Duration::from_millis(2)
    } else {
        Duration::from_millis(180)
    };

    /// How often a pause looks at the cancel flag.
    const SLICE: Duration = Duration::from_millis(10);

    /// How many rows one *Converting* step stands for.
    const ROWS_PER_STEP: usize = 2;

    /// The fabricated conversion of `request`, as a long operation.
    pub(crate) struct FabricatedRun {
        pub(crate) request: ImportScrivenerProjectDto,
        pub(crate) pace: Duration,
    }

    impl LongOperation for FabricatedRun {
        type Output = ImportScrivenerProjectResultDto;

        fn execute(
            &self,
            progress: Box<dyn Fn(OperationProgress) + Send>,
            cancel: Arc<AtomicBool>,
        ) -> anyhow::Result<ImportScrivenerProjectResultDto> {
            // As the converter reports: a phase code, or the title of the row just
            // converted.
            let report = |percent: f32, label: &str| {
                progress(OperationProgress::new(percent, Some(label.to_string())));
            };
            let request = &self.request;
            // As the converter does it: the target is asked about before the project is
            // opened, and a staged file left by a run that was cancelled goes on the way out.
            let output = StagedOutput::begin(&request.output_path, request.overwrite)
                .map_err(not_written)?;
            report(2.0, "import.opening");
            self.pause(&cancel)?;
            report(6.0, "import.outline");
            self.pause(&cancel)?;
            self.pause(&cancel)?;

            // The project read again, as the converter reads it again: it may now refuse,
            // ask, or wait for an acceptance, and a word in its path stands for what a real
            // project can do between the review and the import.
            let analysis = fabricated_analysis(&reading(request));
            if let Some(reason) = refused_on_reading(&analysis) {
                return Err(failed(reason));
            }
            if let Some(reason) = chosen_failure(request) {
                remember_failed_import(&request.source_path);
                return Err(failed(reason));
            }
            let project = Fabricated::of(&request.source_path, request.today);
            let plan = fabricated_plan(&project, &request.choices);
            let titles = planned_titles(&plan);
            let total = titles.len();
            report(10.0, "import.converting");
            self.pause(&cancel)?;
            let mut done = 0;
            while done < total {
                done = (done + ROWS_PER_STEP).min(total);
                report(10.0 + 80.0 * done as f32 / total as f32, &titles[done - 1]);
                self.pause(&cancel)?;
            }

            report(92.0, "import.writing");
            output
                .write(&bundle(request, &analysis))
                .map_err(not_written)?;
            self.pause(&cancel)?;
            output.commit().map_err(not_written)?;
            report(100.0, "import.done");
            Ok(fabricated_result(request, &analysis, &plan))
        }
    }

    impl FabricatedRun {
        /// Wait one step, looking at the cancel flag as it goes; a cancel ends the run here,
        /// as the converter stops at its next row.
        fn pause(&self, cancel: &AtomicBool) -> anyhow::Result<()> {
            let until = Instant::now() + self.pace;
            loop {
                if cancel.load(Ordering::Relaxed) {
                    return Err(failed_with(
                        FailureReason::Cancelled,
                        "the import was cancelled",
                    ));
                }
                let now = Instant::now();
                if now >= until {
                    return Ok(());
                }
                std::thread::sleep(SLICE.min(until - now));
            }
        }
    }

    // ── failures ─────────────────────────────────────────────────────────────────────────

    /// A failed import, as the long operation hands it to the manager: the typed reason, in
    /// the spelling the job reads back.
    fn failed(reason: FailureReason) -> anyhow::Error {
        let details = details_of(&reason);
        failed_with(reason, details)
    }

    fn failed_with(reason: FailureReason, details: &str) -> anyhow::Error {
        let failure = ProjectImportFailure {
            reason,
            details: format!("{details} (fabricated by the mocks build)"),
        };
        anyhow::anyhow!(failure.failure_message())
    }

    /// The technical account a fabricated failure carries behind **Details**.
    fn details_of(reason: &FailureReason) -> &'static str {
        match reason {
            FailureReason::ScrivenerChangedSinceReview => {
                "the project changed since it was reviewed: its fingerprint moved"
            }
            FailureReason::ScrivenerQuestionFirst { .. } => {
                "the project asks a question the review did not answer"
            }
            FailureReason::ScrivenerNotAccepted { .. } => "a question to accept was not accepted",
            FailureReason::ScrivenerDepthNotOffered { .. } => {
                "a structure choice names a depth the plan does not offer"
            }
            FailureReason::ScrivenerRootNotOffered { .. } => {
                "a structure choice names a binder row that is not a root item of the plan"
            }
            FailureReason::ScrivenerRefused { .. } => "the project cannot be imported",
            _ => "the import failed",
        }
    }

    /// The new project could not be put in place, in the words the job knows.
    fn not_written(refused: OutputRefused) -> anyhow::Error {
        let details = refused.to_string();
        let reason = match refused {
            OutputRefused::TargetExists { file } => FailureReason::TargetExists { file },
            OutputRefused::TargetIsFolder { file } => FailureReason::TargetIsFolder { file },
            OutputRefused::NotWritten {
                file,
                folder,
                disk_full: true,
                ..
            } => FailureReason::DiskFull { file, folder },
            OutputRefused::NotWritten {
                file,
                folder,
                reason,
                ..
            } => FailureReason::NotWritten {
                file,
                folder,
                reason,
            },
        };
        anyhow::anyhow!(ProjectImportFailure { reason, details }.failure_message())
    }

    /// The fabricated analysis's own answer, read again at import: the project is refused,
    /// asks a question, or waits for an acceptance the request does not carry.
    fn refused_on_reading(analysis: &ScrivenerAnalysisDto) -> Option<FailureReason> {
        match analysis.outcome {
            ScrivenerAnalysisOutcome::Refused => Some(FailureReason::ScrivenerRefused {
                refusal: analysis.refusal.clone(),
            }),
            ScrivenerAnalysisOutcome::Choose => Some(FailureReason::ScrivenerQuestionFirst {
                question: analysis.question.clone(),
            }),
            ScrivenerAnalysisOutcome::Read if !analysis.pending.is_empty() => {
                Some(FailureReason::ScrivenerNotAccepted {
                    questions: analysis.pending.clone(),
                })
            }
            ScrivenerAnalysisOutcome::Read => None,
        }
    }

    /// The words of the project's path that choose a failure, each with the failure: what a
    /// real project can do between its review and its import.
    pub(crate) const FAILURE_WORDS: [&str; 7] = [
        "changed",
        "question",
        "unaccepted",
        "depth",
        "root",
        "syncing",
        "huge",
    ];

    /// The word of [`FAILURE_WORDS`] that stands for the project itself rather than for
    /// something that happened to it since its review: a project too large to import stays
    /// too large.
    const LASTING: &str = "huge";

    /// The failure the last component of the project's path asks for, if any. Every word
    /// but [`LASTING`] stands for something that happened between the review and the
    /// import, once: after its import failed, the project's next review sees what happened
    /// (the manifest to choose, the edits to accept, see `models::scrivener_inspection`),
    /// and its next import does not meet it again, so **Review the import again** leads somewhere.
    pub(crate) fn chosen_failure(request: &ImportScrivenerProjectDto) -> Option<FailureReason> {
        let name = Path::new(&request.source_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let word = FAILURE_WORDS.iter().find(|word| name.contains(*word))?;
        if *word != LASTING && import_failed_before(&request.source_path) {
            return None;
        }
        Some(match *word {
            "changed" => FailureReason::ScrivenerChangedSinceReview,
            "question" => FailureReason::ScrivenerQuestionFirst {
                question: several_manifests(),
            },
            "unaccepted" => FailureReason::ScrivenerNotAccepted {
                questions: vec![ScrivenerQuestion::UnmergedMobileEdits {
                    source: ScrivenerMobileEdits::MobileFolder,
                }],
            },
            "depth" => FailureReason::ScrivenerDepthNotOffered { depth: 6 },
            "root" => FailureReason::ScrivenerRootNotOffered { item: 41 },
            "syncing" => FailureReason::ScrivenerRefused {
                refusal: project_finding(
                    "scrivener-refusal-project-changed-during-read",
                    ScrivenerFindingFamily::Refusal,
                    ScrivenerReportGroup::Unfiled,
                    Vec::new(),
                ),
            },
            _ => FailureReason::ScrivenerRefused {
                refusal: project_finding(
                    "scrivener-refusal-too-large",
                    ScrivenerFindingFamily::Refusal,
                    ScrivenerReportGroup::Unfiled,
                    vec![count("limit_mib", 2_048)],
                ),
            },
        })
    }

    // ── what the run reads and writes ─────────────────────────────────────────────────────

    /// The reading the import does again, with the answers the review settled.
    fn reading(request: &ImportScrivenerProjectDto) -> AnalyzeScrivenerProjectDto {
        AnalyzeScrivenerProjectDto {
            source_path: request.source_path.clone(),
            chosen_project: request.chosen_project.clone(),
            chosen_manifest: request.chosen_manifest.clone(),
            use_enclosing_project: request.use_enclosing_project,
            upgraded_sibling: request.upgraded_sibling.clone(),
            accept_unmerged_mobile_edits: request.accept_unmerged_mobile_edits,
            accept_no_text: request.accept_no_text,
            today: request.today,
        }
    }

    /// The titles of the rows the plan makes, in the order they are converted.
    fn planned_titles(plan: &ScrivenerStructure) -> Vec<String> {
        match plan {
            ScrivenerStructure::Planned { rows, .. } => rows
                .iter()
                .filter_map(|row| match row {
                    ScrivenerPreviewRow::Row { title, .. } => Some(title.clone()),
                    ScrivenerPreviewRow::Empty => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    /// The new project: the title, author and language the Save page sent, else the ones the
    /// project states, and its two binders, named in the interface's words. Their rows are
    /// left out: the mocks build fabricates every row it shows.
    pub(crate) fn bundle(
        request: &ImportScrivenerProjectDto,
        analysis: &ScrivenerAnalysisDto,
    ) -> WorkBundle {
        let now = chrono::Utc::now().to_rfc3339();
        let chosen = |value: &Option<String>| {
            value
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_string)
        };
        let title = chosen(&request.title).unwrap_or_else(|| analysis.proposed_title.clone());
        let author = chosen(&request.author)
            .or_else(|| analysis.proposed_authors.first().cloned())
            .unwrap_or_default();
        let language: Vec<String> = chosen(&request.language).into_iter().collect();
        let binder = |file_id: u64, name: &str| BundledBinder {
            binder: BinderFile {
                file_id,
                uid: frontend::common::uid::new_uid(),
                created_at: now.clone(),
                updated_at: now.clone(),
                name: name.to_string(),
                activated: true,
                item_order: Vec::new(),
            },
            items: Vec::new(),
        };
        let binders = vec![
            binder(2, &request.manuscript_binder_name),
            binder(3, &request.notes_binder_name),
        ];
        WorkBundle {
            manifest: ProjectManifest {
                format_version: FORMAT_VERSION,
                // Stamped by the writer, as for every bundle built by hand.
                format_min_read_version: None,
                shape: ShapeTag::Zip,
                work: WorkFile {
                    file_id: 1,
                    created_at: now.clone(),
                    updated_at: now.clone(),
                    title,
                    author_name: author,
                    dict_language: language,
                    tag_ids: Vec::new(),
                    dict_word_ids: Vec::new(),
                    unique_id: skrib_format::new_unique_id(),
                    goal_unit: Default::default(),
                    chapter_flat: false,
                    text_replacement_rule_ids: Vec::new(),
                    custom_replacement_rules_enabled: false,
                    smart_punctuation: None,
                    number_chapters: true,
                    part_resets_chapter: false,
                },
                binder_order: binders.iter().map(|b| b.binder.file_id).collect(),
                kind: BundleKind::Regular,
                backup_of: None,
                backup_created_at: None,
            },
            tags: Vec::new(),
            dict_words: Vec::new(),
            text_replacement_rules: Vec::new(),
            note_templates: Vec::new(),
            statuses: Vec::new(),
            assets: Vec::new(),
            note_template_bodies: Default::default(),
            asset_bytes: Default::default(),
            trash_infos: Vec::new(),
            paces: Vec::new(),
            progress_snapshots: Vec::new(),
            orphan_comments: Vec::new(),
            orphan_footnotes: Vec::new(),
            history: Default::default(),
            binders,
            carried: Default::default(),
        }
    }

    // ── the fabricated result ─────────────────────────────────────────────────────────────

    /// The fabricated project's binder row `item`, named as the review named it.
    fn text(item: i64) -> FabricatedItem {
        fabricated_item(item).unwrap_or(FabricatedItem {
            item,
            path: vec![DRAFT.into()],
        })
    }

    fn count(name: &str, value: i64) -> ScrivenerArg {
        ScrivenerArg::Count {
            name: name.into(),
            value,
        }
    }

    fn words(name: &str, values: &[&str]) -> ScrivenerArg {
        ScrivenerArg::Texts {
            name: name.into(),
            values: values.iter().map(|v| v.to_string()).collect(),
        }
    }

    fn token(name: &str, value: &str) -> ScrivenerArg {
        ScrivenerArg::Token {
            name: name.into(),
            value: value.into(),
        }
    }

    fn said(name: &str, value: &str) -> ScrivenerArg {
        ScrivenerArg::Text {
            name: name.into(),
            value: value.into(),
        }
    }

    /// A finding about the project as a whole, filed under `group`.
    fn project_finding(
        key: &str,
        family: ScrivenerFindingFamily,
        group: ScrivenerReportGroup,
        args: Vec<ScrivenerArg>,
    ) -> ScrivenerFinding {
        ScrivenerFinding::Reported {
            key: key.into(),
            family,
            group,
            place: ScrivenerPlace::Project,
            item: -1,
            binder_path: Vec::new(),
            item_uid: None,
            args,
        }
    }

    /// A finding about `place` of `text`, filed under `group`. No row uid: the rows the
    /// mocks build shows are fabricated, and none of them is one of these.
    fn at(
        key: &str,
        family: ScrivenerFindingFamily,
        group: ScrivenerReportGroup,
        text: &FabricatedItem,
        place: ScrivenerPlace,
        args: Vec<ScrivenerArg>,
    ) -> ScrivenerFinding {
        ScrivenerFinding::Reported {
            key: key.into(),
            family,
            group,
            place,
            item: text.item,
            binder_path: text.path.clone(),
            item_uid: None,
            args,
        }
    }

    /// The rungs of the ladder a project with no status list starts with.
    const SEEDED_RUNGS: i64 = 5;

    /// The fabricated result of importing `request`: what the analysis and the plan said,
    /// with the writer's choices, and what only the conversion finds.
    pub(crate) fn fabricated_result(
        request: &ImportScrivenerProjectDto,
        analysis: &ScrivenerAnalysisDto,
        plan: &ScrivenerStructure,
    ) -> ImportScrivenerProjectResultDto {
        use ScrivenerFindingFamily::{Document, Metadata, Prose, Source, Structure};
        use ScrivenerPlace::{Item, Notes, Synopsis, Text as InText};
        use ScrivenerReportGroup::{
            BroughtAcrossDifferently as Differently, CheckThese as Check, CouldNotBeRead as Unread,
            LeftBehind as Left,
        };

        let project = Fabricated::of(&request.source_path, request.today);
        let draft: Vec<FabricatedItem> = fabricated_binder()
            .into_iter()
            .filter(|i| i.path.first().map(String::as_str) == Some(DRAFT))
            .collect();
        let scene = text;
        let mut findings: Vec<ScrivenerFinding> = analysis.notices.clone();
        if let ScrivenerStructure::Planned { notices, .. } = plan {
            findings.extend(notices.iter().cloned());
        }

        // Could not be read.
        findings.extend([
            at(
                "scrivener-prose-unreadable",
                Prose,
                Unread,
                &scene(13),
                InText,
                Vec::new(),
            ),
            at(
                "scrivener-prose-comments-unreadable",
                Prose,
                Unread,
                &scene(16),
                InText,
                Vec::new(),
            ),
            at(
                "scrivener-notice-content-missing",
                Source,
                Unread,
                &scene(8),
                Synopsis,
                vec![ScrivenerArg::Flag {
                    name: "salvaged".into(),
                    value: false,
                }],
            ),
        ]);

        // Check these.
        for item in [3, 7, 12] {
            findings.push(at(
                "scrivener-prose-heading-repeats-title",
                Prose,
                Check,
                &scene(item),
                InText,
                Vec::new(),
            ));
        }
        // The parts and chapters whose title spells their number; the chapter whose title
        // only named its level is the plan's placeholder.
        for item in [1, 2, 10, 11, 15] {
            findings.push(at(
                "scrivener-structure-title-repeats-number",
                Structure,
                Check,
                &scene(item),
                Item,
                Vec::new(),
            ));
        }
        for item in [9, 18] {
            findings.push(at(
                "table-read-as-paragraphs",
                Document,
                Check,
                &scene(item),
                InText,
                vec![
                    ScrivenerArg::Flag {
                        name: "warning".into(),
                        value: true,
                    },
                    count("count", 1),
                    said("detail", ""),
                ],
            ));
        }
        // What only the conversion finds about the metadata; what the mapping foresaw is
        // the plan's, above. A status list's rungs only where there is one.
        let statuses = project.has_statuses();
        if statuses {
            findings.push(project_finding(
                "scrivener-metadata-status-categories-guessed",
                Metadata,
                Check,
                vec![words("rungs", &["Second pass", "With the editor"])],
            ));
        }
        findings.push(project_finding(
            "scrivener-metadata-dates-unreadable",
            Metadata,
            Check,
            vec![count("items", 2)],
        ));

        // Brought across differently.
        for (item, comments) in [(4, 2), (8, 3), (17, 1)] {
            findings.push(at(
                "scrivener-prose-annotations-became-comments",
                Prose,
                Differently,
                &scene(item),
                InText,
                vec![count("count", comments)],
            ));
        }
        for (item, what) in [(5, "highlight"), (9, "highlight"), (14, "colour")] {
            findings.push(at(
                "scrivener-prose-formatting-lost",
                Prose,
                Differently,
                &scene(item),
                InText,
                vec![token("what", what)],
            ));
        }
        // Typed indents in every text, note and synopsis of the Draft: more items than the
        // dialog lists under one line.
        for entry in &draft {
            for place in [InText, Notes, Synopsis] {
                findings.push(at(
                    "scrivener-prose-typed-indents-removed",
                    Prose,
                    Differently,
                    entry,
                    place,
                    vec![count("count", 4)],
                ));
            }
        }
        findings.extend([
            project_finding(
                "scrivener-metadata-labels-became-tags",
                Metadata,
                Differently,
                vec![count("tags", 3), count("rows", 11)],
            ),
            if statuses {
                project_finding(
                    "scrivener-metadata-statuses-became-ladder",
                    Metadata,
                    Differently,
                    vec![count("rungs", 4), count("rows", 14)],
                )
            } else {
                project_finding(
                    "scrivener-metadata-ladder-seeded",
                    Metadata,
                    Differently,
                    vec![count("rungs", SEEDED_RUNGS)],
                )
            },
            project_finding(
                "scrivener-metadata-field-renamed",
                Metadata,
                Differently,
                vec![token("field", "label"), said("title", "Point of view")],
            ),
            project_finding(
                "scrivener-metadata-tag-name-disambiguated",
                Metadata,
                Differently,
                vec![words("names", &["Harbour (collection)"])],
            ),
        ]);
        // Two statuses of one name, the second told apart: only a project with a list.
        if statuses {
            findings.push(project_finding(
                "scrivener-metadata-rung-names-disambiguated",
                Metadata,
                Differently,
                vec![words("names", &["Revised (2)"])],
            ));
        }
        // A value kept beside "none" under the same name, one line per field and name.
        for (field, name, items) in [("label", "None", 2), ("status", "N/A", 3)] {
            if field != "status" || statuses {
                findings.push(project_finding(
                    "scrivener-metadata-none-value-collapsed",
                    Metadata,
                    Differently,
                    vec![
                        token("field", field),
                        said("name", name),
                        count("items", items),
                    ],
                ));
            }
        }

        // Left behind.
        findings.push(project_finding(
            "scrivener-metadata-unused-labels-skipped",
            Metadata,
            Left,
            vec![words("names", &["Flashback", "Dream"])],
        ));

        // The research files Skribisto cannot open, unless their folder was left out:
        // copied beside the new project when the writer asked, the one too large to copy
        // left in the Scrivener project either way.
        let research_kept = !request.choices.iter().any(|choice| {
            matches!(
                choice,
                ScrivenerChoice::Root {
                    item: RESEARCH,
                    destination: ScrivenerDestination::LeaveOut,
                }
            )
        });
        let mut research_folder = String::new();
        let mut research_files = 0;
        if research_kept {
            let chart = text(RESEARCH + 1);
            let recording = text(RESEARCH + 2);
            if request.copy_research_files {
                let stem = Path::new(&request.output_path)
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                research_folder = fill_title(&request.research_folder_pattern, &stem);
                research_files = 1;
                findings.push(at(
                    "scrivener-prose-media-file-copied",
                    Prose,
                    Differently,
                    &chart,
                    ScrivenerPlace::Media,
                    vec![said(
                        "path",
                        &format!("{research_folder}/Harbour chart.pdf"),
                    )],
                ));
            } else {
                findings.push(at(
                    "scrivener-prose-media-file-not-carried",
                    Prose,
                    Left,
                    &chart,
                    ScrivenerPlace::Media,
                    vec![said("name", "Harbour chart.pdf")],
                ));
            }
            findings.push(at(
                "scrivener-prose-media-file-not-carried",
                Prose,
                Left,
                &recording,
                ScrivenerPlace::Media,
                vec![said("name", "Foghorn recording.m4a")],
            ));
        }

        let trash_imported = request.choices.iter().any(|choice| {
            matches!(
                choice,
                ScrivenerChoice::Trash {
                    choice: ScrivenerTrashChoice::Import
                }
            )
        });
        let trashed = if trash_imported { 2 } else { 0 };
        ImportScrivenerProjectResultDto {
            output_path: request.output_path.clone(),
            research_folder,
            totals: ScrivenerTotals::Counted {
                rows: planned_titles(plan).len() as i64,
                words: 61_240,
                footnotes: 7,
                comments: 23,
                pictures: 3,
                templates: 2,
                tags: 8,
                rungs: if statuses { 4 } else { SEEDED_RUNGS },
                trashed,
                research_files,
                synopses: 18,
                document_notes: 5,
            },
            findings,
        }
    }
}

pub use imp::ScrivenerImportRun;

/// The fabricated run's parts, for the tests that walk each case.
#[cfg(all(test, feature = "mocks"))]
pub(crate) use imp::{FAILURE_WORDS, FabricatedRun, bundle, chosen_failure, fabricated_result};

/// Two metadata kinds one project can never have together (one draft target, one
/// deadline, a status list or none): no fabricated report holds both.
#[cfg(all(test, feature = "mocks"))]
pub(crate) const EITHER_OR: [(&str, &str); 3] = [
    (
        "scrivener-metadata-draft-target-kept",
        "scrivener-metadata-draft-target-not-kept",
    ),
    (
        "scrivener-metadata-pace-planned",
        "scrivener-metadata-deadline-not-planned",
    ),
    (
        "scrivener-metadata-statuses-became-ladder",
        "scrivener-metadata-ladder-seeded",
    ),
];

/// The fabricated run behaves as the converter does where the job can tell: it stops at a
/// cancel, refuses or replaces an existing file as asked, follows the writer's choices, and
/// says nothing the report cannot word.
#[cfg(all(test, feature = "mocks"))]
mod mock_tests {
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;

    use frontend::common::long_operation::{LongOperation, OperationProgress};
    use frontend::import_management::failure::{FailureReason, ProjectImportFailure};
    use frontend::import_management::{
        ImportScrivenerProjectDto, ScrivenerArg, ScrivenerChoice, ScrivenerDestination,
        ScrivenerFinding, ScrivenerFindingFamily, ScrivenerPreviewRow, ScrivenerStructure,
        ScrivenerTitleKind, ScrivenerTotals, ScrivenerTrashChoice,
    };

    use super::{
        EITHER_OR, FAILURE_WORDS, FabricatedRun, bundle, chosen_failure, fabricated_result,
    };
    use crate::import_scrivener::messages;
    use crate::models::scrivener_inspection::{Fabricated, fabricated_analysis, fabricated_plan};

    /// A request to import the fabricated project `Harbour.scriv` in `dir` beside it.
    fn request(dir: &Path) -> ImportScrivenerProjectDto {
        ImportScrivenerProjectDto {
            source_path: dir.join("Harbour.scriv").to_string_lossy().into_owned(),
            output_path: dir.join("Harbour.skrib").to_string_lossy().into_owned(),
            title: Some("Harbour Lights".into()),
            author: Some("Mina Okafor".into()),
            language: Some("en-GB".into()),
            copy_research_files: true,
            manuscript_binder_name: "Manuscript".into(),
            notes_binder_name: "Notes".into(),
            research_folder_pattern: format!(
                "{} research files",
                skrib_format::importing::TITLE_SLOT
            ),
            ..ImportScrivenerProjectDto::default()
        }
    }

    /// Run the fabricated import of `request` to its end, cancelled from the start when
    /// `cancelled`.
    fn run(
        request: ImportScrivenerProjectDto,
        cancelled: bool,
    ) -> anyhow::Result<frontend::import_management::ImportScrivenerProjectResultDto> {
        let run = FabricatedRun {
            request,
            pace: Duration::from_millis(1),
        };
        run.execute(
            Box::new(|_: OperationProgress| {}),
            Arc::new(AtomicBool::new(cancelled)),
        )
    }

    fn reason(error: &anyhow::Error) -> FailureReason {
        ProjectImportFailure::from_any_message(&error.to_string()).reason
    }

    fn scratch() -> tempfile::TempDir {
        match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("a scratch folder: {e}"),
        }
    }

    #[test]
    fn a_cancelled_run_stops_and_writes_nothing() {
        let dir = scratch();
        let Err(error) = run(request(dir.path()), true) else {
            panic!("a cancelled run must not finish");
        };
        assert_eq!(reason(&error), FailureReason::Cancelled);
        let left: Vec<_> = match std::fs::read_dir(dir.path()) {
            Ok(entries) => entries.flatten().map(|e| e.file_name()).collect(),
            Err(e) => panic!("the scratch folder: {e}"),
        };
        assert!(left.is_empty(), "{left:?}");
    }

    #[test]
    fn an_existing_project_is_refused_unless_it_may_be_replaced() {
        let dir = scratch();
        let asked = request(dir.path());
        if let Err(e) = std::fs::write(&asked.output_path, b"PK") {
            panic!("a file at the target: {e}");
        }
        let Err(error) = run(asked.clone(), false) else {
            panic!("an existing file is not replaced unasked");
        };
        assert_eq!(
            reason(&error),
            FailureReason::TargetExists {
                file: "Harbour.skrib".into()
            }
        );
        let replaced = run(
            ImportScrivenerProjectDto {
                overwrite: true,
                ..asked.clone()
            },
            false,
        );
        assert!(replaced.is_ok(), "{replaced:?}");
        match skrib_format::read_bundle(&asked.output_path) {
            Ok(written) => {
                assert_eq!(written.manifest.work.title, "Harbour Lights");
                assert_eq!(written.manifest.work.author_name, "Mina Okafor");
                assert_eq!(
                    written.manifest.work.dict_language,
                    vec!["en-GB".to_string()]
                );
                let binders: Vec<&str> = written
                    .binders
                    .iter()
                    .map(|b| b.binder.name.as_str())
                    .collect();
                assert_eq!(binders, vec!["Manuscript", "Notes"]);
            }
            Err(e) => panic!("the new project reads: {e:#}"),
        }
    }

    #[test]
    fn every_word_chooses_its_own_failure() {
        let dir = scratch();
        let mut chosen = Vec::new();
        for word in FAILURE_WORDS {
            let asked = ImportScrivenerProjectDto {
                source_path: dir
                    .path()
                    .join(format!("Harbour {word}.scriv"))
                    .to_string_lossy()
                    .into_owned(),
                ..request(dir.path())
            };
            let Some(reason) = chosen_failure(&asked) else {
                panic!("{word} chooses a failure");
            };
            assert!(
                messages::own_failure(&reason).is_some(),
                "{word}: the importer words it"
            );
            assert!(!chosen.contains(&reason), "{word} repeats a failure");
            chosen.push(reason);
        }
        assert_eq!(chosen_failure(&request(dir.path())), None);
    }

    /// The research files follow the writer: copied, left in the project, or not met at all
    /// when their folder was left out; the trash comes across when asked.
    #[test]
    fn the_result_follows_the_writers_choices() {
        let dir = scratch();
        let keys = |choices: Vec<ScrivenerChoice>, copy: bool| {
            let asked = ImportScrivenerProjectDto {
                choices: choices.clone(),
                copy_research_files: copy,
                ..request(dir.path())
            };
            let analysis = fabricated_analysis(&Default::default());
            let project = Fabricated::of(&asked.source_path, asked.today);
            let result = fabricated_result(&asked, &analysis, &fabricated_plan(&project, &choices));
            let research: Vec<String> = result
                .findings
                .iter()
                .map(|f| messages::key_of(f).to_string())
                .filter(|k| k.starts_with("scrivener-prose-media-file"))
                .collect();
            (result, research)
        };
        let (copied, research) = keys(Vec::new(), true);
        assert_eq!(copied.research_folder, "Harbour research files");
        assert_eq!(
            research,
            vec![
                "scrivener-prose-media-file-copied",
                "scrivener-prose-media-file-not-carried"
            ]
        );
        let (kept, research) = keys(Vec::new(), false);
        assert!(kept.research_folder.is_empty());
        assert_eq!(research.len(), 2);
        assert!(research.iter().all(|k| k.ends_with("not-carried")));
        let left_out = ScrivenerChoice::Root {
            item: 24,
            destination: ScrivenerDestination::LeaveOut,
        };
        let (none, research) = keys(vec![left_out], true);
        assert!(none.research_folder.is_empty() && research.is_empty());

        let trash = ScrivenerChoice::Trash {
            choice: ScrivenerTrashChoice::Import,
        };
        let (with_trash, _) = keys(vec![trash], true);
        assert!(matches!(
            with_trash.totals,
            ScrivenerTotals::Counted { trashed: 2, .. }
        ));
        assert!(
            with_trash
                .findings
                .iter()
                .any(|f| messages::key_of(f) == "scrivener-structure-trash-brought-across")
        );
    }

    /// Every finding the fabricated run gives has a row in the wizard's table, or is one of
    /// the document importer's, worded by that importer: none can say its key.
    #[test]
    fn every_fabricated_finding_is_worded() {
        let dir = scratch();
        let asked = request(dir.path());
        let analysis = fabricated_analysis(&Default::default());
        let project = Fabricated::of(&asked.source_path, asked.today);
        let result = fabricated_result(&asked, &analysis, &fabricated_plan(&project, &[]));
        for finding in &result.findings {
            let document = matches!(
                finding,
                ScrivenerFinding::Reported {
                    family: ScrivenerFindingFamily::Document,
                    ..
                }
            );
            let key = messages::key_of(finding);
            assert!(
                document || messages::entry(key).is_some(),
                "{key} has no sentence"
            );
        }
        // The bundle carries no row of its own: the mocks build fabricates every row.
        let written = bundle(&asked, &analysis);
        assert!(written.binders.iter().all(|b| b.items.is_empty()));
    }

    /// The fabricated import of the project named `name`, beside it in `dir`, with
    /// `choices`.
    fn imported(
        dir: &Path,
        name: &str,
        choices: Vec<ScrivenerChoice>,
    ) -> frontend::import_management::ImportScrivenerProjectResultDto {
        let asked = ImportScrivenerProjectDto {
            source_path: dir
                .join(format!("{name}.scriv"))
                .to_string_lossy()
                .into_owned(),
            output_path: dir
                .join(format!("{name}.skrib"))
                .to_string_lossy()
                .into_owned(),
            choices,
            ..request(dir)
        };
        match run(asked, false) {
            Ok(result) => result,
            Err(e) => panic!("{name} imports: {e:#}"),
        }
    }

    /// The `name` argument of `finding`, as the report reads it.
    fn arg<'a>(finding: &'a ScrivenerFinding, name: &str) -> Option<&'a ScrivenerArg> {
        let ScrivenerFinding::Reported { args, .. } = finding else {
            return None;
        };
        args.iter().find(|arg| match arg {
            ScrivenerArg::Count { name: n, .. }
            | ScrivenerArg::Text { name: n, .. }
            | ScrivenerArg::Texts { name: n, .. }
            | ScrivenerArg::Flag { name: n, .. }
            | ScrivenerArg::Token { name: n, .. }
            | ScrivenerArg::Day { name: n, .. }
            | ScrivenerArg::Item { name: n, .. } => n == name,
            ScrivenerArg::Empty => false,
        })
    }

    fn token_of(finding: &ScrivenerFinding, name: &str) -> String {
        match arg(finding, name) {
            Some(ScrivenerArg::Token { value, .. }) => value.clone(),
            _ => String::new(),
        }
    }

    /// Every metadata kind the converter has reaches a fabricated report, each branch of
    /// the ones whose sentence branches included, and no one report holds two kinds a
    /// single project cannot have together.
    #[test]
    fn every_metadata_kind_reaches_a_fabricated_report() {
        let dir = scratch();
        let mut results = vec![
            imported(dir.path(), "Harbour", Vec::new()),
            // A second book: the draft target and the deadline have no one book to go to.
            imported(
                dir.path(),
                "Harbour books",
                vec![ScrivenerChoice::Root {
                    item: 27,
                    destination: ScrivenerDestination::Manuscript,
                }],
            ),
        ];
        for word in [
            "pages",
            "characters",
            "overdue",
            "lastday",
            "undated",
            "statusless",
        ] {
            results.push(imported(dir.path(), &format!("Harbour {word}"), Vec::new()));
        }

        let mut seen: Vec<String> = Vec::new();
        let mut deadline_reasons: Vec<String> = Vec::new();
        let mut draft_reasons: Vec<String> = Vec::new();
        let mut fields_without_row: Vec<String> = Vec::new();
        for result in &results {
            let keys: Vec<&str> = result.findings.iter().map(messages::key_of).collect();
            for (one, other) in EITHER_OR {
                assert!(
                    !(keys.contains(&one) && keys.contains(&other)),
                    "{one} and {other} in one report"
                );
            }
            for finding in &result.findings {
                let key = messages::key_of(finding);
                if !seen.iter().any(|k| k == key) {
                    seen.push(key.to_string());
                }
                match key {
                    "scrivener-metadata-deadline-not-planned" => {
                        let day = matches!(arg(finding, "day"), Some(ScrivenerArg::Day { .. }));
                        let reason = token_of(finding, "reason");
                        deadline_reasons.push(if day { reason } else { "no day".to_string() });
                    }
                    "scrivener-metadata-draft-target-not-kept" => {
                        draft_reasons.push(token_of(finding, "reason"));
                    }
                    "scrivener-metadata-values-without-row" => {
                        fields_without_row.push(token_of(finding, "field"));
                    }
                    _ => {}
                }
            }
        }
        let missing: Vec<&str> = frontend::import_management::scrivener::every_key()
            .into_iter()
            .filter(|k| k.starts_with("scrivener-metadata-"))
            .filter(|k| !seen.iter().any(|s| s == k))
            .collect();
        assert!(missing.is_empty(), "never fabricated: {missing:?}");
        for reason in [
            "passed",
            "no-writing-day-left",
            "no-word-goal",
            "character-goal",
            "no-single-book",
            "no day",
        ] {
            assert!(
                deadline_reasons.iter().any(|r| r == reason),
                "the deadline, {reason}: {deadline_reasons:?}"
            );
        }
        for reason in ["pages", "no-single-book"] {
            assert!(
                draft_reasons.iter().any(|r| r == reason),
                "the draft target, {reason}: {draft_reasons:?}"
            );
        }
        for field in [
            "label",
            "status",
            "keyword",
            "custom-field",
            "bookmark",
            "target",
        ] {
            assert!(
                fields_without_row.iter().any(|f| f == field),
                "{field} without a row: {fields_without_row:?}"
            );
        }
    }

    /// One binder row has one name: every finding about a row, from the review's plan or
    /// from the run, gives it the same path, the Review page shows it under the last part
    /// of that path (blank only where a finding says its title was a placeholder), and no
    /// two rows of the plan are the same binder row.
    #[test]
    fn a_row_has_one_name_in_the_review_and_the_report() {
        let dir = scratch();
        let trash = ScrivenerChoice::Trash {
            choice: ScrivenerTrashChoice::Import,
        };
        let ideas_kept = ScrivenerChoice::Root {
            item: 27,
            destination: ScrivenerDestination::Notes,
        };
        for (n, choices) in [Vec::new(), vec![trash, ideas_kept]]
            .into_iter()
            .enumerate()
        {
            let result = imported(dir.path(), &format!("Harbour {n}"), choices.clone());
            let mut paths: std::collections::BTreeMap<i64, Vec<String>> = Default::default();
            let mut placeholders = Vec::new();
            for finding in &result.findings {
                let ScrivenerFinding::Reported {
                    key,
                    item,
                    binder_path,
                    ..
                } = finding
                else {
                    continue;
                };
                if *item < 0 {
                    continue;
                }
                if key == "scrivener-structure-placeholder-title" {
                    placeholders.push(*item);
                }
                let known = paths.entry(*item).or_insert_with(|| binder_path.clone());
                assert_eq!(known, binder_path, "row {item} has two names");
            }
            let source = dir.path().join(format!("Harbour {n}.scriv"));
            let project = Fabricated::of(&source.to_string_lossy(), request(dir.path()).today);
            let ScrivenerStructure::Planned { rows, .. } = fabricated_plan(&project, &choices)
            else {
                panic!("the novel plans");
            };
            let mut planned: Vec<i64> = Vec::new();
            for row in &rows {
                let ScrivenerPreviewRow::Row {
                    item,
                    title,
                    title_kind: ScrivenerTitleKind::Own,
                    ..
                } = row
                else {
                    continue;
                };
                if *item < 0 {
                    continue;
                }
                assert!(
                    !planned.contains(item),
                    "binder row {item} is planned twice"
                );
                planned.push(*item);
                if let Some(path) = paths.get(item) {
                    if title.is_empty() {
                        assert!(placeholders.contains(item), "row {item} is blank");
                    } else {
                        assert_eq!(
                            Some(title),
                            path.last(),
                            "row {item}: the review and the report name it apart"
                        );
                    }
                }
            }
            // Every row a finding names is one the review showed.
            for item in paths.keys() {
                assert!(
                    planned.contains(item),
                    "row {item} is named in the report but not in the review"
                );
            }
        }
    }
}
