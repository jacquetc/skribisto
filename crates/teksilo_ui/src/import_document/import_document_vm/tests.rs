// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use document_ingest::plan::PlannedRow;

/// Run `f` with the **real shipped** `en-US` messages installed.
///
/// Not `I18nConfig::test_only` with a hand-copied list of patterns, which is
/// the other precedent in this crate (`view_models::open_failure`): that
/// proves a copy agrees with itself, and the whole point here is to catch a
/// diagnostic whose key never reached `main.ftl`.
///
/// It also has to exist at all: with no manager installed, a message
/// carrying a `{ $count -> … }` plural selector resolves to its own id —
/// which would have made this test pass for the wrong reason on every
/// plural diagnostic.
fn with_real_messages(f: impl FnOnce()) {
    use teksilo::i18n::config::I18nConfig;
    use teksilo::i18n::manager::I18nManager;
    use teksilo::i18n::thread_local::{clear, install};

    clear();
    let cfg = I18nConfig::new()
        .source_locale("en-US".parse().unwrap())
        .supported_locales(["en-US".parse().unwrap()])
        .compile_in(&[(
            "en-US",
            &[
                include_str!("../../../locales/en-US/main.ftl"),
                include_str!("../../../locales/en-US/tooltips.ftl"),
                include_str!("../../../locales/en-US/tags.ftl"),
                include_str!("../../../locales/en-US/templates.ftl"),
            ],
        )])
        .auto_detect_os_locale(false)
        .fallback_locale("en-US".parse().unwrap());
    install(I18nManager::from_config(&cfg));
    f();
    clear();
}

/// A container row: a real one carries no prose of its own.
///
/// The fixture used to give *every* row prose, including its Book — a shape the
/// analyser cannot produce (`infer_rules` keeps the deepest level prose-bearing)
/// and one `apply_document_import` refuses outright. It went unnoticed while
/// nothing checked; the live type check now does, which is the point of it.
fn container(indent: i64, title: &str, kind: CreateType) -> PlannedRow {
    PlannedRow {
        djot: String::new(),
        word_count: 0,
        ..planned(indent, title, kind)
    }
}

fn planned(indent: i64, title: &str, kind: CreateType) -> PlannedRow {
    PlannedRow {
        indent,
        create_type: kind,
        title: title.into(),
        stripped_ordinal: None,
        djot: format!("{title} prose."),
        epigraph: String::new(),
        scene_breaks: 0,
        word_count: 2,
        // A fixture, not a file: nothing was read, so nothing digests.
        source_file_digest: String::new(),
        origin: "a.md".into(),
        included: true,
        comments: Vec::new(),
        footnotes: Vec::new(),
        source_uid_tag: None,
        source_digest: None,
        diagnostics: Vec::new(),
    }
}

/// Book / Chapter One (Scene A, Scene B) / Chapter Two, from heading levels
/// 1, 2, 3, 3, 2.
fn vm() -> ImportDocumentViewModel {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    let plan = ImportPlan {
        rows: vec![
            container(0, "Book", CreateType::Book),
            planned(1, "Chapter One", CreateType::Chapter),
            planned(2, "Scene A", CreateType::Scene),
            planned(2, "Scene B", CreateType::Scene),
            planned(1, "Chapter Two", CreateType::Chapter),
        ],
        diagnostics: Vec::new(),
    };
    vm.on_plan_ready(
        &plan,
        vec![1, 2, 3, 3, 2],
        vec![
            (1, CreateType::Book),
            (2, CreateType::Chapter),
            (3, CreateType::Scene),
        ],
    );
    vm
}

fn created_titles(vm: &ImportDocumentViewModel) -> Vec<String> {
    vm.rows_to_create()
        .into_iter()
        .filter_map(|r| match r {
            ApplyImportRow::Create { title, .. } => Some(title),
            // An update names no title — it points at a row that already has one.
            ApplyImportRow::Update { .. } | ApplyImportRow::Empty => None,
        })
        .collect()
}

#[test]
fn a_ready_plan_moves_to_the_review_step() {
    let vm = vm();
    assert_eq!(vm.step().get(), STEP_REVIEW);
    assert_eq!(vm.included_count(), 5);
}

/// Chapter files without a Book heading — wrap them under a writer-added root.
#[test]
fn a_top_level_header_wraps_every_analysed_row() {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    vm.on_plan_ready(
        &ImportPlan {
            rows: vec![
                planned(0, "Chapter One", CreateType::Chapter),
                planned(0, "Chapter Two", CreateType::Chapter),
            ],
            diagnostics: Vec::new(),
        },
        vec![1, 1],
        vec![(1, CreateType::Chapter)],
    );
    vm.add_top_level_header();

    assert_eq!(vm.plan().visible_count(), 3);
    assert_eq!(vm.plan().type_of(PlanRowKey(0)), Some(CreateType::Book));
    assert_eq!(vm.plan().row(PlanRowKey(1)).map(|r| r.indent), Some(1));
    assert_eq!(vm.plan().row(PlanRowKey(2)).map(|r| r.indent), Some(1));
    // The synthetic root is included and carries no prose, so Import is not blocked.
    assert!(vm.blocking_rows().is_empty());
    let titles = created_titles(&vm);
    assert_eq!(titles.len(), 3, "root + two chapters");
    assert_eq!(titles[1], "Chapter One");
    assert_eq!(titles[2], "Chapter Two");
}

/// One rule change instead of two hundred corrections — the reason the rule
/// table exists at all.
#[test]
fn a_level_rule_retypes_every_row_that_came_from_that_level() {
    let vm = vm();
    vm.set_level_rule(3, CreateType::Note);
    assert_eq!(vm.plan().type_of(PlanRowKey(2)), Some(CreateType::Note));
    assert_eq!(vm.plan().type_of(PlanRowKey(3)), Some(CreateType::Note));
    assert_eq!(
        vm.plan().type_of(PlanRowKey(1)),
        Some(CreateType::Chapter),
        "a different level is untouched"
    );
}

/// The failure that would make the rule table and the per-row combo unusable
/// together: a bulk change quietly reverting a deliberate one.
#[test]
fn a_level_rule_never_overrules_a_row_the_writer_retyped() {
    let vm = vm();
    vm.retype_row(PlanRowKey(2), CreateType::Paratext);
    vm.set_level_rule(3, CreateType::Note);

    assert_eq!(
        vm.plan().type_of(PlanRowKey(2)),
        Some(CreateType::Paratext),
        "the pinned row keeps what the writer chose"
    );
    assert_eq!(
        vm.plan().type_of(PlanRowKey(3)),
        Some(CreateType::Note),
        "its unpinned sibling still follows the rule"
    );
}

#[test]
fn excluding_a_chapter_excludes_its_scenes() {
    let vm = vm();
    vm.set_included(PlanRowKey(1), false);
    assert_eq!(created_titles(&vm), vec!["Book", "Chapter Two"]);
    assert_eq!(vm.included_count(), 2);
}

#[test]
fn including_it_again_brings_the_whole_subtree_back() {
    let vm = vm();
    vm.set_included(PlanRowKey(1), false);
    vm.set_included(PlanRowKey(1), true);
    assert_eq!(vm.included_count(), 5);
    assert_eq!(created_titles(&vm).len(), 5);
}

/// What effective inclusion buys over a destructive cascade: a scene the
/// writer unticked on its own stays unticked when its chapter comes back. A
/// cascade would have forgotten that decision and re-included it.
#[test]
fn re_including_a_chapter_does_not_resurrect_a_scene_the_writer_unticked() {
    let vm = vm();
    vm.set_included(PlanRowKey(2), false); // Scene A, on its own
    vm.set_included(PlanRowKey(1), false); // then the whole chapter
    vm.set_included(PlanRowKey(1), true); // and back

    assert!(!vm.is_included(PlanRowKey(2)), "Scene A stays out");
    assert!(vm.is_included(PlanRowKey(3)), "Scene B comes back");
    assert_eq!(
        created_titles(&vm),
        vec!["Book", "Chapter One", "Scene B", "Chapter Two"]
    );
}

/// What the panel binds a descendant checkbox's `enabled` to, so a ticked row
/// under an unticked chapter reads as "fine, but its chapter is not coming"
/// rather than as a contradiction.
#[test]
fn a_descendant_knows_when_an_ancestor_is_holding_it_back() {
    let vm = vm();
    assert!(vm.ancestors_included(PlanRowKey(2)));
    vm.set_included(PlanRowKey(1), false);
    assert!(!vm.ancestors_included(PlanRowKey(2)));
    assert!(
        vm.plan().is_ticked(PlanRowKey(2)),
        "its own tick is untouched — only its effect is suspended"
    );
}

#[test]
fn what_would_be_created_carries_the_retyped_kind_not_the_analysed_one() {
    let vm = vm();
    vm.retype_row(PlanRowKey(4), CreateType::Note);
    let rows = vm.rows_to_create();
    let ApplyImportRow::Create { kind, title, .. } = &rows[4] else {
        panic!("expected a create row");
    };
    assert_eq!(title, "Chapter Two");
    assert_eq!(*kind, ImportRowKind::Note);
}

#[test]
fn indent_and_prose_survive_into_what_gets_created() {
    let vm = vm();
    let rows = vm.rows_to_create();
    let ApplyImportRow::Create { indent, djot, .. } = &rows[2] else {
        panic!("expected a create row");
    };
    assert_eq!(*indent, 2);
    assert_eq!(djot, "Scene A prose.");
}

#[test]
fn nothing_can_be_applied_with_every_row_excluded() {
    let vm = vm();
    vm.set_included(PlanRowKey(0), false);
    assert_eq!(vm.included_count(), 0);
    assert!(!vm.can_apply());
    assert!(vm.apply().is_err());
}

#[test]
fn files_keep_the_order_they_arrived_in_and_never_double_up() {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    vm.add_files([PathBuf::from("b.md"), PathBuf::from("a.md")]);
    vm.add_files([PathBuf::from("b.md")]);
    assert_eq!(
        vm.file_paths(),
        vec![PathBuf::from("b.md"), PathBuf::from("a.md")]
    );

    vm.move_file(1, -1);
    assert_eq!(
        vm.file_paths(),
        vec![PathBuf::from("a.md"), PathBuf::from("b.md")]
    );

    vm.remove_file(0);
    assert_eq!(vm.file_paths(), vec![PathBuf::from("b.md")]);
}

#[test]
fn moving_a_file_off_either_end_does_nothing() {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    vm.add_files([PathBuf::from("a.md"), PathBuf::from("b.md")]);
    vm.move_file(0, -1);
    vm.move_file(1, 1);
    assert_eq!(
        vm.file_paths(),
        vec![PathBuf::from("a.md"), PathBuf::from("b.md")]
    );
}

/// The picker and the drop zone must offer exactly what a scanner can read —
/// Markdown, plain text, Word and ODT (default `document_ingest` features).
#[test]
fn the_accepted_extensions_come_from_the_scanners_themselves() {
    let extensions = ImportDocumentViewModel::accepted_extensions();
    for expected in ["md", "markdown", "txt", "docx", "odt"] {
        assert!(
            extensions.iter().any(|e| e == expected),
            "missing {expected} in {extensions:?}"
        );
    }
}

#[test]
fn resetting_clears_the_files_the_plan_and_the_step() {
    let vm = vm();
    vm.add_files([PathBuf::from("a.md")]);
    vm.reset();
    assert!(vm.file_paths().is_empty());
    assert!(vm.plan().is_empty());
    assert_eq!(vm.step().get(), STEP_FILES);
    assert_eq!(vm.included_count(), 0);
}

/// One drop can carry the same path twice. The dedup has to see what this
/// very call already added, not only what was there when it started.
#[test]
fn one_call_cannot_add_the_same_file_twice() {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    vm.add_files([PathBuf::from("a.md"), PathBuf::from("a.md")]);
    assert_eq!(vm.file_paths(), vec![PathBuf::from("a.md")]);
    assert_eq!(vm.file_count().get(), 1);
}

/// What the footer's Next button greys out on — the count has to be a
/// signal, because `ListModel` reports through observers and not a version.
#[test]
fn the_file_count_follows_every_way_the_list_changes() {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    assert_eq!(vm.file_count().get(), 0);
    vm.add_files([PathBuf::from("a.md"), PathBuf::from("b.md")]);
    assert_eq!(vm.file_count().get(), 2);
    vm.remove_file(0);
    assert_eq!(vm.file_count().get(), 1);
    vm.reset();
    assert_eq!(vm.file_count().get(), 0);
}

/// The rule table is read back off the rows, so it can never claim a level
/// produced something the plan disagrees with.
#[test]
fn the_level_rules_are_what_the_analysed_rows_actually_say() {
    let plan = ImportPlan {
        rows: vec![
            container(0, "Book", CreateType::Book),
            planned(1, "One", CreateType::Chapter),
            planned(2, "A", CreateType::Scene),
            planned(1, "Two", CreateType::Chapter),
        ],
        diagnostics: Vec::new(),
    };
    assert_eq!(
        infer_level_rules(&plan, &[1, 2, 3, 2]),
        vec![
            (1, CreateType::Book),
            (2, CreateType::Chapter),
            (3, CreateType::Scene),
        ]
    );
}

/// Every background job in the app reports through the same four events.
/// A wizard that moved its step on somebody else's export finishing would
/// be unusable — and the failure would look like a random UI jump.
#[test]
fn an_event_for_another_operation_is_ignored() {
    let app_ctx = Rc::new(AppContext::new());
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
    vm.add_files([PathBuf::from("a.md")]);

    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
        let event = long_op_event("someone-elses-op");
        vm.on_long_op_progress(ctx, &event);
        vm.on_long_op_completed(ctx, &event);
        vm.on_long_op_cancelled(ctx, &event);
        vm.on_long_op_failed(ctx, &event);
    });

    assert_eq!(vm.step().get(), STEP_FILES, "the step must not have moved");
    assert_eq!(vm.file_paths().len(), 1, "the chosen files survive");
    assert_eq!(vm.progress().get(), 0.0);
}

/// A `LongOperation` event carrying `id`, shaped like the manager's own
/// JSON payload (`view_models::long_op` parses it).
fn long_op_event(id: &str) -> Event {
    use frontend::common::event::{LongOperationEvent, Origin};
    Event {
        origin: Origin::LongOperation(LongOperationEvent::Completed),
        ids: Vec::new(),
        data: Some(format!(
            r#"{{"id":"{id}","percentage":50.0,"message":"reading"}}"#
        )),
    }
}

/// A match with no wildcard arm, so a new `ImportDiagnostic` variant upstream
/// stops this crate from compiling until the sample list below, the `message`
/// arms, and both `.ftl` files have caught up.
///
/// It does nothing at run time on purpose — the compiler is the assertion.
fn exhaustive_over_every_variant(d: &document_ingest::ImportDiagnostic) {
    use document_ingest::ImportDiagnostic::*;
    match d {
        FileUnreadable { .. }
        | LossyDecode { .. }
        | DecodedFromBom { .. }
        | EmptyFile { .. }
        | NoHeadings { .. }
        | UnsupportedFormat { .. }
        | FrontMatterNotFlat { .. }
        | FootnotesDegraded { .. }
        | FootnoteNotCarried { .. }
        | RawHtmlDropped { .. }
        | NestedBreakDropped { .. }
        | ImageNotIngested { .. }
        | DuplicateTitle { .. }
        | HeadingLevelJump { .. }
        | IllegalCombination { .. }
        | TrackedChangesFlattened { .. }
        | TextBoxDropped { .. }
        | EmbeddedObjectDropped { .. }
        | FieldFlattened { .. }
        | UnknownStyleLevel { .. }
        | CommentUnanchored { .. }
        | CommentRepliesFlattened { .. }
        | EpigraphNotCarried { .. }
        | EpigraphPlacementAmbiguous { .. } => {}
    }
}

/// **Every** diagnostic `document_ingest` can raise must reach a translated
/// sentence.
///
/// The samples are real `ImportDiagnostic` variants, mapped through the real
/// DTO mapper, so a variant added upstream fails here rather than silently
/// landing in the fallback arm. Collected diagnostics that nothing rendered is
/// the bug this whole surface exists to close; a new one quietly joining them
/// would be the same bug again.
///
/// [`exhaustive_over_every_variant`] is what keeps the list honest: a `vec!` of
/// samples is only as complete as whoever last edited it, and this test's own
/// note used to claim it was built from the variants when it was not. That
/// claim is now enforced by the compiler.
#[test]
fn every_diagnostic_the_importer_can_raise_has_a_sentence() {
    use document_ingest::ImportDiagnostic as D;

    let every = vec![
        D::FileUnreadable {
            path: "/tmp/a.md".into(),
            reason: "permission denied".into(),
        },
        D::LossyDecode {
            path: "/tmp/a.md".into(),
            replacements: 3,
        },
        D::DecodedFromBom {
            path: "/tmp/a.md".into(),
            encoding: "UTF-16LE",
        },
        D::EmptyFile {
            path: "/tmp/a.md".into(),
        },
        D::NoHeadings {
            path: "/tmp/a.md".into(),
        },
        D::UnsupportedFormat {
            path: "/tmp/a.odt".into(),
            extension: "odt".into(),
        },
        D::FrontMatterNotFlat {
            path: "/tmp/a.md".into(),
            key: "tags".into(),
        },
        D::FootnotesDegraded {
            path: "/tmp/a.md".into(),
            count: 2,
        },
        D::FootnoteNotCarried {
            path: "/tmp/a.docx".into(),
            count: 1,
        },
        D::RawHtmlDropped {
            path: "/tmp/a.md".into(),
            count: 1,
        },
        D::NestedBreakDropped {
            path: "/tmp/a.md".into(),
            count: 4,
        },
        D::ImageNotIngested {
            path: "/tmp/a.md".into(),
            target: "cover.png".into(),
        },
        D::DuplicateTitle {
            title: "Later".into(),
            occurrences: 2,
        },
        D::HeadingLevelJump {
            title: "Deep".into(),
            from: 1,
            to: 4,
        },
        D::IllegalCombination {
            title: "A part".into(),
            kind: CreateType::Part,
        },
        D::TrackedChangesFlattened {
            path: "/tmp/a.docx".into(),
            count: 6,
        },
        D::TextBoxDropped {
            path: "/tmp/a.docx".into(),
            count: 1,
        },
        D::EmbeddedObjectDropped {
            path: "/tmp/a.odt".into(),
            count: 2,
        },
        D::FieldFlattened {
            path: "/tmp/a.docx".into(),
            count: 9,
        },
        D::UnknownStyleLevel {
            path: "/tmp/a.docx".into(),
            style: "HeadingChapter".into(),
        },
        D::CommentUnanchored {
            path: "/tmp/a.docx".into(),
            quote: "Is this the right word?".into(),
        },
        D::CommentRepliesFlattened {
            path: "/tmp/a.odt".into(),
            count: 3,
        },
        D::EpigraphNotCarried {
            title: "A scene".into(),
            kind: CreateType::Scene,
        },
        D::EpigraphPlacementAmbiguous {
            above: "Part One".into(),
            below: "Chapter One".into(),
        },
    ];

    for raised in &every {
        exhaustive_over_every_variant(raised);
    }

    with_real_messages(|| {
        for raised in &every {
            let dto = frontend::import_management::diagnostic_to_dto(raised, 0);
            let (_, _, parsed) = plan_from_dto(
                &DocumentImportRows::Empty,
                &ImportDiagnosticRows::Reported(vec![dto]),
            );
            let d = parsed.first().expect("the DTO round-trips");
            let text = d.message("A part", Some(CreateType::Part)).resolve_now();

            assert!(
                !text.contains(raised.key()),
                "{} fell through to the untranslated fallback: {text:?}",
                raised.key()
            );
            assert!(!text.trim().is_empty(), "{} rendered nothing", raised.key());
            // A Fluent argument the message references but nobody supplied is
            // rendered as `{$name}` rather than failing — which reads as a
            // corrupt sentence and is exactly the mistake a plural selector
            // invites.
            assert!(
                !text.contains("{$") && !text.contains("{ $"),
                "{} left an argument unfilled: {text:?}",
                raised.key()
            );
        }
    });
}

/// Worst first. A file that could not be opened at all must not sit under
/// three notes about footnotes.
/// Retyping a row into something that cannot hold its prose still *marks* it, and the
/// marker still clears when it is put right.
///
/// It no longer stops the import — the prose is given a paratext instead — but the writer
/// still has to be told, because a row silently sprouting a child is a surprise. The
/// diagnostic was once computed only at analysis time, so the strip reported the pre-retype
/// state and no marker ever appeared; that is what this pins.
#[test]
fn retyping_a_row_so_it_cannot_hold_its_prose_marks_it() {
    let vm = vm();
    assert!(vm.diagnostics_for_row(PlanRowKey(2)).is_empty());

    // Scene A carries prose; a Part cannot hold any.
    vm.retype_row(PlanRowKey(2), CreateType::Part);
    assert_eq!(
        vm.diagnostics_for_row(PlanRowKey(2)).len(),
        1,
        "the row must be marked in the tree"
    );
    assert!(
        vm.blocking_rows().is_empty(),
        "marked, but not a dead end — there is a resolution"
    );

    // Retyping it back clears the marker and the resolution alike.
    vm.retype_row(PlanRowKey(2), CreateType::Scene);
    assert!(vm.diagnostics_for_row(PlanRowKey(2)).is_empty());
    assert_eq!(vm.stray_prose_for(PlanRowKey(2)), None);
}

#[test]
fn a_row_that_cannot_hold_its_prose_no_longer_stops_the_writer() {
    let vm = vm();
    let gate = vm.can_proceed_from_review_signal();
    assert!(gate.get());

    vm.retype_row(PlanRowKey(2), CreateType::Part);
    assert!(
        gate.get(),
        "there is a resolution for this row, so it is not a dead end"
    );
    assert!(vm.blocking_rows().is_empty());
    assert_eq!(
        vm.stray_prose_for(PlanRowKey(2)),
        Some(StrayProse::AsParatext)
    );
}

/// And what that resolution actually produces: the container, empty, with the text just
/// inside it as a paratext — the shape the writer had before they exported.
#[test]
fn stray_prose_becomes_a_paratext_inside_the_row_it_arrived_on() {
    let vm = vm();
    vm.retype_row(PlanRowKey(2), CreateType::Part);

    let rows = vm.rows_to_create();
    let titles: Vec<(String, i64, bool)> = rows
        .iter()
        .filter_map(|r| match r {
            ApplyImportRow::Create {
                title,
                indent,
                djot,
                ..
            } => Some((title.clone(), *indent, djot.trim().is_empty())),
            _ => None,
        })
        .collect();

    let at = titles
        .iter()
        .position(|(t, _, empty)| t == "Scene A" && *empty)
        .expect("the container is created, and holds no prose");
    let (_, container_indent, _) = titles[at];
    let (child_title, child_indent, child_empty) = titles[at + 1].clone();
    assert_eq!(child_title, "Scene A", "the paratext is named for its row");
    assert_eq!(child_indent, container_indent + 1, "just inside it");
    assert!(!child_empty, "and it is where the prose went");
}

/// The same shape on a row that came home from a returning file.
///
/// M8's own tests all build their plan from `vm()`, whose rows carry no mark, so the
/// scenario the milestone was written for — a book's front matter coming back on the Book
/// row, from a file this project exported — had no coverage at all. The paratext this
/// mints is a row *this* import is creating, so it carries no borrowed identity: the mark
/// named the container, and the container is the row that keeps it.
#[test]
fn stray_prose_on_a_returning_row_still_becomes_a_paratext_carrying_no_identity() {
    let vm = vm_from_a_returning_file();
    // `vm_from_a_returning_file`'s row 2 is a tagged Scene; retyping it to a Part makes it
    // a container carrying prose, which is exactly the shape M8 exists for.
    vm.retype_row(PlanRowKey(2), CreateType::Part);

    let created: Vec<(String, i64, bool, String)> = vm
        .rows_to_create()
        .iter()
        .filter_map(|r| match r {
            ApplyImportRow::Create {
                title,
                indent,
                djot,
                source_uid_tag,
                ..
            } => Some((
                title.clone(),
                *indent,
                djot.trim().is_empty(),
                source_uid_tag.clone(),
            )),
            _ => None,
        })
        .collect();

    let at = created
        .iter()
        .position(|(t, _, empty, _)| t == "Scene A" && *empty)
        .expect("the container is created, and holds no prose");
    let (_, container_indent, _, container_tag) = created[at].clone();
    assert_eq!(
        container_tag, "tag-a",
        "the container keeps the identity the mark named"
    );

    let (child_title, child_indent, child_empty, child_tag) = created[at + 1].clone();
    assert_eq!(child_title, "Scene A");
    assert_eq!(child_indent, container_indent + 1, "just inside it");
    assert!(!child_empty, "and it is where the prose went");
    assert!(
        child_tag.is_empty(),
        "a row this import is minting has no history to claim: {child_tag:?}"
    );
}

/// The other choice: drop the text and keep the row.
#[test]
fn discarding_stray_prose_creates_the_row_alone() {
    let vm = vm();
    vm.retype_row(PlanRowKey(2), CreateType::Part);
    vm.set_stray_prose(PlanRowKey(2), StrayProse::Discard);

    let created = vm.rows_to_create().len();
    assert_eq!(
        created, 5,
        "one row per plan row — the text is gone, not given a home"
    );
}

/// The strip's rows are the view-model's own, written on every diagnostics
/// change — the view used to mirror them from a side effect inside a mapped
/// signal, which is how a headline said "3 things to know" over an empty box.
#[test]
fn the_diagnostic_rows_follow_every_change() {
    let vm = vm();
    let rows = vm.diagnostic_rows();
    assert_eq!(rows.len(), vm.diagnostic_messages().len());

    // A retype adds an illegal-combination entry; the rows must gain it
    // without anyone reading a signal to make it happen.
    let before = rows.len();
    vm.retype_row(PlanRowKey(2), CreateType::Part);
    assert_eq!(rows.len(), before + 1);
    assert_eq!(rows.len(), vm.diagnostic_messages().len());

    // And the headline counts exactly what the list shows — infos included.
    let (errors, others) = vm.diagnostic_counts();
    assert_eq!(errors + others, rows.len());

    vm.reset();
    assert_eq!(rows.len(), 0, "a reset empties the strip too");
}

/// A row the writer unticked is never created, so it needs no resolution either.
#[test]
fn an_excluded_row_is_not_split() {
    let vm = vm();
    vm.retype_row(PlanRowKey(2), CreateType::Part);
    vm.set_included(PlanRowKey(2), false);

    let rows = vm.rows_to_create();
    assert!(
        !rows.iter().any(|r| matches!(
            r,
            ApplyImportRow::Create { title, .. } if title == "Scene A"
        )),
        "an unticked row produces nothing at all, split or otherwise: {rows:#?}"
    );
}

/// The same shape reached the other way: a *bulk* level rule can make a whole level unable
/// to hold its prose at once, and every row it touched is resolved the same way one retype
/// is.
#[test]
fn a_bulk_level_rule_resolves_every_row_it_breaks() {
    let vm = vm();
    vm.set_level_rule(3, CreateType::Part);

    assert!(vm.blocking_rows().is_empty(), "none of them is a dead end");
    for key in [PlanRowKey(2), PlanRowKey(3)] {
        assert_eq!(vm.stray_prose_for(key), Some(StrayProse::AsParatext));
    }
    // Two containers plus two paratexts, on top of the three rows that were always fine.
    assert_eq!(vm.rows_to_create().len(), 7);
}

#[test]
fn diagnostics_are_read_worst_first() {
    let vm = vm();
    vm.set_diagnostics(vec![
        Diagnostic {
            key: "no-headings".into(),
            severity: "info".into(),
            path: "/tmp/c.md".into(),
            detail: String::new(),
            count: 0,
            row: None,
        },
        Diagnostic {
            key: "footnotes-degraded".into(),
            severity: "warning".into(),
            path: "/tmp/b.md".into(),
            detail: String::new(),
            count: 1,
            row: None,
        },
        Diagnostic {
            key: "file-unreadable".into(),
            severity: "error".into(),
            path: "/tmp/a.md".into(),
            detail: "denied".into(),
            count: 0,
            row: None,
        },
    ]);

    let severities: Vec<String> = vm
        .diagnostic_messages()
        .into_iter()
        .map(|(s, _)| s)
        .collect();
    assert_eq!(severities, vec!["error", "warning", "info"]);
    // One error, and two entries that are not errors. The second number is
    // everything the list shows below the headline, not the `warning`
    // severity alone — a headline that counted warnings only said "1 thing to
    // know" above three visible sentences.
    assert_eq!(vm.diagnostic_counts(), (1, 2));
}

/// A row-scoped diagnostic names its row, and the sentence is built from the
/// row's own title and type rather than from a second copy on the wire.
#[test]
fn a_row_scoped_diagnostic_reads_its_row_for_the_words_it_needs() {
    let vm = vm();
    vm.retype_row(PlanRowKey(1), CreateType::Part);
    vm.set_diagnostics(vec![Diagnostic {
        key: "illegal-combination".into(),
        severity: "warning".into(),
        path: String::new(),
        detail: String::new(),
        count: 0,
        row: Some(PlanRowKey(1)),
    }]);

    assert_eq!(vm.diagnostics_for_row(PlanRowKey(1)).len(), 1);
    assert!(vm.diagnostics_for_row(PlanRowKey(0)).is_empty());

    with_real_messages(|| {
        let (_, message) = vm.diagnostic_messages().remove(0);
        let text = message.resolve_now();
        assert!(
            text.contains("Chapter One"),
            "the sentence must name the row: {text:?}"
        );
        assert!(
            text.contains("Part"),
            "…and the type it was retyped to: {text:?}"
        );
    });
}

/// The whole analysis half, end to end against the real backend: two
/// Markdown files on disk → `analyze_document_import` → the plan on the
/// review step, with the structure the headings imply.
///
/// The one test that would catch the wiring being wrong rather than the
/// logic: the DTO round-trip, the level inference, the step move and the
/// event filter all have to agree, and each of them compiles fine alone
/// while disagreeing with the others.
#[test]
fn a_real_analysis_lands_a_plan_on_the_review_step() {
    use frontend::commands::{handling_app_lifecycle_commands, work_management_commands};
    use frontend::work_management::{NewWorkDto, NewWorkTemplate};

    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("01-one.md"),
        "# The Book\n\n## Chapter One\n\nIt began.\n\n* * *\n\nAnd continued.\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("02-two.md"),
        "## Chapter Two\n\nIt ended.\n",
    )
    .unwrap();

    let app_ctx = Rc::new(AppContext::new());
    handling_app_lifecycle_commands::initialize_app(&app_ctx).unwrap();
    work_management_commands::new_work(
        &app_ctx,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: dir.path().join("p.skrib").to_string_lossy().into_owned(),
            title: String::new(),
            is_folder: false,
            template_kind: NewWorkTemplate::Novel,
            labels: vec![],
            language: vec!["en".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .unwrap();
    let work_id = frontend::commands::work_commands::get_all_work(&app_ctx)
        .unwrap()
        .first()
        .map(|w| w.id)
        .expect("the work the test just created");

    let ids = AppIds::default();
    ids.work_id.set(Some(work_id));
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), ids);
    vm.add_files([dir.path().join("01-one.md"), dir.path().join("02-two.md")]);

    let op = vm.start_analysis().expect("the analysis starts");
    // The Stepper footer's Next advances after `validate_on_next` returns true;
    // tests that call `start_analysis` directly must do the same jump — into
    // Review, which shows the progress UI while `busy`.
    vm.controller().next();
    assert_eq!(vm.step().get(), STEP_REVIEW);
    assert!(vm.busy().get(), "analysis is in flight on the review step");

    // A long operation runs on its own thread; poll for its result rather
    // than sleeping a guessed interval.
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
    loop {
        let done =
            frontend::commands::import_management_commands::get_analyze_document_import_result(
                &app_ctx, &op,
            )
            .ok()
            .flatten()
            .is_some();
        if done {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "analysis never finished"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
        vm.on_long_op_completed(ctx, &long_op_event(&op));
    });

    assert_eq!(vm.step().get(), STEP_REVIEW, "the plan must be on screen");
    assert!(!vm.busy().get());
    assert_eq!(vm.active_op_id(), None, "the op is no longer in flight");

    let titles: Vec<String> = vm.plan().rows().iter().map(|r| r.title.clone()).collect();
    assert_eq!(titles, vec!["The Book", "Chapter One", "Chapter Two"]);

    // The heading ladder became indents, and the rule table agrees with it.
    let indents: Vec<i64> = vm.plan().rows().iter().map(|r| r.indent).collect();
    assert_eq!(indents, vec![0, 1, 1]);
    assert_eq!(
        vm.level_rules().get(),
        vec![(1, CreateType::Book), (2, CreateType::Chapter)]
    );

    // The `* * *` is preserved inside its chapter's prose and counted, not
    // split into a second row — the design decision this whole feature
    // rests on.
    let breaks: usize = vm.plan().rows().iter().map(|r| r.scene_breaks).sum();
    assert_eq!(
        breaks, 1,
        "the scene break is reported, not silently dropped"
    );
}

/// Build a `.skrib` holding an imported `.docx`, comments and all, at
/// `$SKRIBISTO_IMPORT_FIXTURE_OUT`.
///
/// `#[ignore]` because it is a tool, not an assertion: it exists so a real
/// project carrying imported comments can be opened in the running app. The
/// store-level correctness is asserted by `skribisto-import-management`'s
/// integration tests; what only the live app can show is whether an imported
/// comment *renders* — in the margin beside the sentence it names, and in the
/// comments dock — which is exactly the failure the anchor module's own doc
/// calls invisible until someone's comment has moved to the wrong sentence.
///
///     SKRIBISTO_IMPORT_FIXTURE_OUT=/tmp/imported.skrib \
///       cargo test -p teksilo_ui a_docx_import_saved_as_a_project -- --ignored --nocapture
#[test]
#[ignore = "a fixture builder for live inspection, not a check"]
fn a_docx_import_saved_as_a_project() {
    use frontend::commands::{handling_app_lifecycle_commands, work_management_commands};
    use frontend::work_management::{NewWorkDto, NewWorkTemplate, SaveWorkDto};

    let out = std::env::var("SKRIBISTO_IMPORT_FIXTURE_OUT")
        .expect("set SKRIBISTO_IMPORT_FIXTURE_OUT to the .skrib to write");
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../document_ingest/tests/fixtures/word-shaped.docx");
    assert!(
        source.exists(),
        "run document_ingest's fixtures/generate.py"
    );

    let app_ctx = Rc::new(AppContext::new());
    handling_app_lifecycle_commands::initialize_app(&app_ctx).unwrap();
    work_management_commands::new_work(
        &app_ctx,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: out.clone(),
            title: String::new(),
            is_folder: false,
            // The emptiest template there is: the point of this fixture is to
            // look at what the *import* produced, and a dozen seeded chapters
            // above it are a dozen rows of noise.
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en".to_string()],
            author_name: "Cyril".into(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .unwrap();
    let work_id = frontend::commands::work_commands::get_all_work(&app_ctx)
        .unwrap()
        .first()
        .map(|w| w.id)
        .expect("the work just created");

    let ids = AppIds::default();
    ids.work_id.set(Some(work_id));
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), ids);
    vm.add_files([source]);

    let op = vm.start_analysis().expect("the analysis starts");
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        if frontend::commands::import_management_commands::get_analyze_document_import_result(
            &app_ctx, &op,
        )
        .ok()
        .flatten()
        .is_some()
        {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "analysis never finished"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
        vm.on_long_op_completed(ctx, &long_op_event(&op));
    });
    assert_eq!(vm.step().get(), STEP_REVIEW);

    let carried: usize = vm.plan().rows().iter().map(|r| r.comments.len()).sum();
    assert_eq!(carried, 2, "two comment threads reached the review step");

    // Land it in the first binder, at the end. The picker is keyed by durable
    // uid, so the binder's own uid is the destination.
    let binder_id = frontend::commands::work_commands::get_work_relationship(
        &app_ctx,
        &work_id,
        &frontend::common::direct_access::work::WorkRelationshipField::Binders,
    )
    .unwrap()
    .first()
    .copied()
    .expect("the template made a binder");
    let binder_uid = frontend::commands::binder_commands::get_binder(&app_ctx, &binder_id)
        .unwrap()
        .expect("binder row")
        .uid;
    vm.destination().reload();
    vm.destination()
        .preselect(crate::models::BinderTreeKey::Binder(binder_uid));
    let created = vm.apply().expect("apply");
    assert!(!created.is_empty());

    let path = work_management_commands::save_work(
        &app_ctx,
        &SaveWorkDto {
            media_root: crate::media_paths::media_root_string(),
            work_id,
            file_name: out.clone(),
            overwrite: true,
        },
    )
    .expect("start save");
    // `save_work` is a long operation; wait for the file to appear.
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    while !std::path::Path::new(&out).exists() {
        assert!(std::time::Instant::now() < deadline, "save never finished");
        std::thread::sleep(Duration::from_millis(50));
    }
    println!("WROTE {out} (op {path})");
}
// ── the merge, and what it makes of the plan ────────────────────────────────────────

/// Three beta readers send their copies back and the writer adds all three to the
/// wizard at once — the obvious gesture, and the one that used to duplicate the
/// book twice with nothing said.
///
/// `reconcile::pair` claims each existing row at most once, so the first file's
/// rows match and every later file's rows fall through to `New`, whose default
/// action is `CreateNew`.
#[test]
fn several_returns_of_one_manuscript_block_the_import() {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    let tagged = |indent: i64, title: &str, kind: CreateType, tag: &str| PlannedRow {
        source_uid_tag: Some(tag.into()),
        source_digest: Some("aaaaaaaaaaaa".into()),
        ..planned(indent, title, kind)
    };
    // Two files, each carrying the same two rows of the same project.
    let plan = ImportPlan {
        rows: vec![
            container(0, "Book", CreateType::Book),
            tagged(1, "Chapter One", CreateType::Chapter, "tag-one"),
            tagged(2, "Scene A", CreateType::Scene, "tag-a"),
            tagged(1, "Chapter One", CreateType::Chapter, "tag-one"),
            tagged(2, "Scene A", CreateType::Scene, "tag-a"),
        ],
        diagnostics: Vec::new(),
    };
    vm.on_plan_ready(
        &plan,
        vec![1, 2, 3, 2, 3],
        vec![
            (1, CreateType::Book),
            (2, CreateType::Chapter),
            (3, CreateType::Scene),
        ],
    );

    let dupes = vm.duplicate_return_titles();
    assert_eq!(
        dupes,
        vec!["Chapter One".to_string(), "Scene A".to_string()],
        "both repeated rows must be reported"
    );
    assert!(
        !vm.can_apply(),
        "the import must be blocked, not silently doubled"
    );
}

/// One return of one manuscript is the ordinary case and must stay unblocked —
/// the guard keys on the round-trip tag, which is unique per row within a file.
#[test]
fn a_single_returning_file_is_not_mistaken_for_duplicates() {
    let vm = vm_from_a_returning_file();
    assert!(
        vm.duplicate_return_titles().is_empty(),
        "one copy of each row is not a duplicate return"
    );
}

/// The same shape as [`vm`], but every row carrying the round-trip mark a returning file
/// would have brought — which is what makes an `Update` possible at all.
fn vm_from_a_returning_file() -> ImportDocumentViewModel {
    let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
    let tagged = |indent: i64, title: &str, kind: CreateType, tag: &str| PlannedRow {
        source_uid_tag: Some(tag.into()),
        source_digest: Some("aaaaaaaaaaaa".into()),
        ..planned(indent, title, kind)
    };
    let plan = ImportPlan {
        rows: vec![
            container(0, "Book", CreateType::Book),
            tagged(1, "Chapter One", CreateType::Chapter, "tag-one"),
            tagged(2, "Scene A", CreateType::Scene, "tag-a"),
            tagged(2, "Scene B", CreateType::Scene, "tag-b"),
            tagged(1, "Chapter Two", CreateType::Chapter, "tag-two"),
        ],
        diagnostics: Vec::new(),
    };
    vm.on_plan_ready(
        &plan,
        vec![1, 2, 3, 3, 2],
        vec![
            (1, CreateType::Book),
            (2, CreateType::Chapter),
            (3, CreateType::Scene),
        ],
    );
    vm
}

use skribisto_model::reconcile::RowAction;

/// A merge row as the reconcile step would hold one, without a store behind it.
fn merged(key: MergeRowKey, incoming: Option<PlanRowKey>, actions: Vec<RowAction>) -> MergeRowView {
    MergeRowView {
        key,
        indent: 0,
        current_title: matches!(key, MergeRowKey::Current(_)).then(|| "Chapter One".into()),
        current_item_id: matches!(key, MergeRowKey::Current(_)).then_some(7),
        incoming_title: incoming.map(|_| "Chapter One".into()),
        incoming_key: incoming,
        status: RowStatus::EditorEdited,
        moved: false,
        actions,
    }
}

/// With no merge at all — a first import, or a plan accepted before the reconcile step
/// existed — every included row is created, exactly as it always was.
#[test]
fn without_a_merge_every_row_is_still_created() {
    let vm = vm();
    let rows = vm.rows_to_create();
    assert_eq!(rows.len(), 5);
    assert!(
        rows.iter()
            .all(|r| matches!(r, ApplyImportRow::Create { .. }))
    );
}

/// The case the feature exists for: bring the remarks, leave the manuscript alone.
#[test]
fn comments_only_sends_an_update_that_does_not_replace_the_prose() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];

    vm.seed_merge_for_test(vec![merged(
        MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    )]);

    let rows = vm.rows_to_create();
    let update = rows
        .iter()
        .find_map(|r| match r {
            ApplyImportRow::Update {
                target_uid_tag,
                replace_prose,
                ..
            } => Some((target_uid_tag.clone(), *replace_prose)),
            _ => None,
        })
        .expect("the matched row became an update");
    // The **destination's** tag, not the incoming row's own mark. The two coincide on a
    // file this project exported and never touched, which is why asserting the incoming
    // one here looked right for a release — see the test below for where they part.
    assert_eq!(
        update.0,
        skribisto_model::round_trip::uid_tag(&uuid::Uuid::from_u128(1))
    );
    assert!(!update.1, "comments-only must not replace the prose");
}

/// A row paired by **title**, with no mark anywhere in the file.
///
/// `reconcile::pair`'s second rung matches on type and title, for a file exported with
/// `include_round_trip_marks` off or produced by another tool entirely — and it offers
/// Take-import and Comments-only on the rows it pairs, like any other. The instruction
/// used to be built from the *incoming* row's mark, which such a row does not have, so the
/// writer's explicit "take the editor's wording" silently became a second copy of the
/// chapter. Both existing update tests use a file whose marks happen to name the right
/// destination, so neither could see it.
#[test]
fn a_row_paired_without_any_mark_still_updates_the_row_it_was_paired_with() {
    // `vm()`, not `vm_from_a_returning_file()`: not one of its plan rows carries a tag.
    let vm = vm();
    let key = vm.plan().keys_in_order()[1];
    let destination = uuid::Uuid::from_u128(42);
    let row_key = MergeRowKey::Current(destination);
    vm.seed_merge_for_test(vec![merged(
        row_key,
        Some(key),
        vec![RowAction::TakeImport, RowAction::KeepCurrent],
    )]);
    vm.set_action(row_key, RowAction::TakeImport);

    let rows = vm.rows_to_create();
    let update = rows
        .iter()
        .find_map(|r| match r {
            ApplyImportRow::Update {
                target_uid_tag,
                replace_prose,
                ..
            } => Some((target_uid_tag.clone(), *replace_prose)),
            _ => None,
        })
        .expect("a title-paired row the writer chose to take must become an update");
    assert_eq!(
        update.0,
        skribisto_model::round_trip::uid_tag(&destination),
        "the update names the row it was paired with"
    );
    assert!(update.1, "take-import replaces the prose");
    assert!(
        !created_titles(&vm).contains(&"Chapter One".to_string()),
        "and no second copy of it is created: {:#?}",
        created_titles(&vm)
    );
}

/// The same shape, chosen as Comments-only: the remarks land on the paired row, and the
/// 90 000 words underneath it are not re-imported beside themselves.
#[test]
fn a_title_paired_row_taking_only_comments_creates_nothing() {
    let vm = vm();
    let key = vm.plan().keys_in_order()[1];
    let destination = uuid::Uuid::from_u128(42);
    let row_key = MergeRowKey::Current(destination);
    vm.seed_merge_for_test(vec![merged(
        row_key,
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    )]);

    let rows = vm.rows_to_create();
    assert!(
        rows.iter().any(|r| matches!(
            r,
            ApplyImportRow::Update {
                replace_prose: false,
                ..
            }
        )),
        "comments-only on a title-paired row is still an update: {rows:#?}"
    );
    assert!(
        !created_titles(&vm).contains(&"Chapter One".to_string()),
        "and creates no duplicate"
    );
}

#[test]
fn take_import_sends_an_update_that_does() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];
    let row_key = MergeRowKey::Current(uuid::Uuid::from_u128(1));
    vm.seed_merge_for_test(vec![merged(
        row_key,
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    )]);
    vm.set_action(row_key, RowAction::TakeImport);

    assert!(vm.rows_to_create().iter().any(|r| matches!(
        r,
        ApplyImportRow::Update {
            replace_prose: true,
            ..
        }
    )));
}

/// A row the writer is keeping produces no instruction at all — the same way an unticked
/// row does. There is nothing to send, and sending a no-op would be an invitation to write
/// one by accident later.
#[test]
fn keeping_a_row_sends_nothing_for_it() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];
    let row_key = MergeRowKey::Current(uuid::Uuid::from_u128(1));
    vm.seed_merge_for_test(vec![merged(
        row_key,
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::KeepCurrent],
    )]);
    vm.set_action(row_key, RowAction::KeepCurrent);

    let rows = vm.rows_to_create();
    assert_eq!(rows.len(), 4, "the kept row is absent: {rows:#?}");
    assert!(
        rows.iter()
            .all(|r| matches!(r, ApplyImportRow::Create { .. }))
    );
}

/// A block-level decision must reach the write, and must override the whole-row
/// action rather than being quietly dropped beside it.
///
/// `RowAction` is all-or-nothing: an editor who fixed thirty commas offered the
/// writer "take all thirty and any rewriting with them, or none". Accepting one
/// hunk is a strictly more specific answer to the same question, so it wins.
#[test]
fn an_accepted_hunk_overrides_the_whole_row_action() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];

    // `CommentsOnly` — the row would otherwise leave the prose alone entirely.
    vm.seed_merge_for_test(vec![merged(
        MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    )]);
    let row = vm.merge_rows().remove(0);
    assert!(
        vm.merged_prose(&row).is_none(),
        "no decision made yet, so nothing overrides the row action"
    );

    vm.set_hunk_accepted(row.key, 0, true);
    assert_eq!(
        vm.accepted_hunks(row.key).len(),
        1,
        "the decision is remembered against the row's own key"
    );

    let rows = vm.rows_to_create();
    let update = rows
        .iter()
        .find(|r| matches!(r, ApplyImportRow::Update { .. }))
        .expect("the matched row still updates");
    match update {
        ApplyImportRow::Update { replace_prose, .. } => assert!(
            *replace_prose,
            "a block the writer accepted has to be written, so the prose is replaced"
        ),
        _ => unreachable!(),
    }
}

/// Un-taking the last block returns the row to its whole-row meaning, rather
/// than leaving an empty decision behind that still forces a prose rewrite.
#[test]
fn clearing_every_hunk_restores_the_plain_row_action() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];
    vm.seed_merge_for_test(vec![merged(
        MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    )]);
    let row = vm.merge_rows().remove(0);

    vm.set_hunk_accepted(row.key, 0, true);
    vm.set_hunk_accepted(row.key, 0, false);
    assert!(vm.accepted_hunks(row.key).is_empty());
    assert!(
        vm.merged_prose(&row).is_none(),
        "an emptied decision must not keep forcing a rewrite"
    );
}

/// The beta-reader case: a copy went out, came back with remarks and untouched
/// prose, so the Reconcile step is dropped out of the flow entirely rather than
/// showing a page of identical "comments only" dropdowns.
#[test]
fn a_return_that_only_brings_remarks_hides_the_reconcile_step() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];

    let mut row = merged(
        MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    );
    // What `reconcile` reports for a row neither side touched.
    row.status = RowStatus::Identical;
    vm.seed_merge_for_test(vec![row]);

    assert!(
        vm.has_anything_to_reconcile(),
        "the row still matched — this is not the first-import case"
    );
    assert!(
        !vm.needs_reconcile().get(),
        "nothing is left to decide, so the step must not be shown"
    );
}

/// …and a row the editor actually rewrote still stops the writer.
#[test]
fn a_return_that_rewrote_prose_still_shows_the_reconcile_step() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];

    vm.seed_merge_for_test(vec![merged(
        MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        Some(key),
        vec![RowAction::TakeImport, RowAction::CommentsOnly],
    )]);

    assert!(
        vm.needs_reconcile().get(),
        "an edited row is exactly what the step exists to show"
    );
}

/// ⚠ The hazard the step-hiding must not create.
///
/// `rows_to_create`'s catch-all arm treats "no decision" as `CreateNew`, so if
/// hiding the step ever stopped `rebuild_merge` running, every matched row would
/// be created afresh — the manuscript duplicated, silently. Hiding the step must
/// leave the decisions exactly as they were.
#[test]
fn hiding_the_reconcile_step_still_updates_rather_than_duplicates() {
    let vm = vm_from_a_returning_file();
    let key = vm.plan().keys_in_order()[1];

    let mut row = merged(
        MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        Some(key),
        vec![RowAction::CommentsOnly, RowAction::TakeImport],
    );
    row.status = RowStatus::Identical;
    vm.seed_merge_for_test(vec![row]);
    assert!(!vm.needs_reconcile().get());

    let rows = vm.rows_to_create();
    let updates: Vec<_> = rows
        .iter()
        .filter(|r| matches!(r, ApplyImportRow::Update { .. }))
        .collect();
    assert_eq!(updates.len(), 1, "the matched row must update: {rows:#?}");
    match updates[0] {
        ApplyImportRow::Update { replace_prose, .. } => assert!(
            !replace_prose,
            "a comments-only return must not rewrite the writer's prose"
        ),
        _ => unreachable!(),
    }
}

/// A decision is remembered against the row's own identity, so re-sourcing the merge —
/// which is what going back and choosing a different destination does — cannot hand one
/// row's instruction to another.
#[test]
fn a_decision_survives_the_merge_being_rebuilt() {
    let vm = vm();
    let row_key = MergeRowKey::Current(uuid::Uuid::from_u128(1));
    let other = MergeRowKey::Current(uuid::Uuid::from_u128(2));
    vm.seed_merge_for_test(vec![
        merged(
            row_key,
            None,
            vec![RowAction::CommentsOnly, RowAction::TakeImport],
        ),
        merged(
            other,
            None,
            vec![RowAction::CommentsOnly, RowAction::TakeImport],
        ),
    ]);
    vm.set_action(row_key, RowAction::TakeImport);

    // The same two rows, in the other order — as a different destination might produce.
    vm.seed_merge_for_test(vec![
        merged(
            other,
            None,
            vec![RowAction::CommentsOnly, RowAction::TakeImport],
        ),
        merged(
            row_key,
            None,
            vec![RowAction::CommentsOnly, RowAction::TakeImport],
        ),
    ]);
    let rows = vm.merge_rows();
    assert_eq!(
        vm.action_for(&rows[1]),
        RowAction::TakeImport,
        "kept its own"
    );
    assert_eq!(
        vm.action_for(&rows[0]),
        RowAction::CommentsOnly,
        "and did not inherit it"
    );
}

/// A first import into an empty destination has nothing to line up, and the step says so
/// in one sentence instead of a table of identical dropdowns.
#[test]
fn nothing_to_reconcile_when_every_row_is_new() {
    let vm = vm();
    vm.seed_merge_for_test(vec![merged(
        MergeRowKey::Incoming(vm.plan().keys_in_order()[1]),
        Some(vm.plan().keys_in_order()[1]),
        vec![RowAction::CreateNew],
    )]);
    assert!(!vm.has_anything_to_reconcile());
    assert_eq!(vm.matched_count(), 0);
}
