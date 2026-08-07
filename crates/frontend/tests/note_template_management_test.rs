// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for `note_template_management::import_note_templates`.
//!
//! The behaviour worth pinning here is the one place this use case deliberately departs
//! from its `import_tags` sibling: **a colliding name is suffixed, not skipped**. Skipping
//! is right for a tag (a name and a colour) and wrong for a template (an entire document
//! the writer picked a file for), and it would silently turn "re-import my edited
//! character-sheet.md" into a permanent no-op. The rest — one undo step for the whole
//! batch, appended in order, scoped to the named Work — mirrors `import_tags` and is
//! pinned for the same reasons its own tests give.

use frontend::AppContext;
use frontend::commands::{
    note_template_commands, note_template_management_commands, smart_punctuation_commands,
    undo_redo_commands, work_commands,
};
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::QuoteStyle;
use frontend::common::types::EntityId;
use frontend::direct_access::{CreateNoteTemplateDto, CreateSmartPunctuationDto, CreateWorkDto};
use note_template_management::ImportNoteTemplatesDto;

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

struct Fixture {
    ctx: AppContext,
    setup: u64,
    work: EntityId,
}

/// A Work on a dedicated undo stack, so the arrange phase never pollutes the stack the
/// action under test runs on.
fn make_fixture() -> Fixture {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);
    let work = mk_work(&ctx, Some(setup), "The Lighthouse");
    Fixture { ctx, setup, work }
}

/// Every Work owns exactly one punctuation row (one_to_one, strong), created BEFORE the
/// Work so it can be built with the real id. Leaving the placeholder `0` (via
/// `..Default::default()`) makes the *second* Work in a test collide under the generated
/// uniqueness check — see `multi_work_scoping_test`'s own note on this ordering.
fn mk_work(ctx: &AppContext, stack: Option<u64>, title: &str) -> EntityId {
    let punctuation = smart_punctuation_commands::create_orphan_smart_punctuation(
        ctx,
        stack,
        &CreateSmartPunctuationDto {
            created_at: now(),
            updated_at: now(),
            override_app_default: false,
            dashes: false,
            ellipsis: false,
            quotes: false,
            quote_style: QuoteStyle::LocaleDefault,
            pre_punctuation_spacing: false,
            dialogue_marker: false,
        },
    )
    .expect("create smart_punctuation")
    .id;
    work_commands::create_orphan_work(
        ctx,
        stack,
        &CreateWorkDto {
            created_at: now(),
            updated_at: now(),
            title: title.into(),
            smart_punctuation: punctuation,
            ..Default::default()
        },
    )
    .expect("create work")
    .id
}

fn work_templates(fx: &Fixture) -> Vec<EntityId> {
    work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::NoteTemplates)
        .expect("read note_templates")
}

/// (name, body, starred) for every template the Work owns, in relationship order.
fn rows(fx: &Fixture) -> Vec<(String, String, bool)> {
    note_template_commands::get_note_template_multi(&fx.ctx, &work_templates(fx))
        .expect("read templates")
        .into_iter()
        .flatten()
        .map(|t| (t.name, t.body, t.starred))
        .collect()
}

fn seed(fx: &Fixture, name: &str) -> EntityId {
    let id = note_template_commands::create_orphan_note_template(
        &fx.ctx,
        Some(fx.setup),
        &CreateNoteTemplateDto {
            // Nil on purpose — `with_identity` in the controller is what mints it, and
            // this test goes through the controller, so it exercises that path.
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            name: name.into(),
            body: "seeded".into(),
            starred: false,
        },
    )
    .expect("create template")
    .id;
    let mut ids = work_templates(fx);
    ids.push(id);
    work_commands::set_work_relationship(
        &fx.ctx,
        Some(fx.setup),
        &frontend::direct_access::WorkRelationshipDto {
            id: fx.work,
            field: WorkRelationshipField::NoteTemplates,
            right_ids: ids,
        },
    )
    .expect("wire templates");
    id
}

fn dto(work: EntityId, rows: &[(&str, &str, bool)]) -> ImportNoteTemplatesDto {
    ImportNoteTemplatesDto {
        work_id: work,
        names: rows.iter().map(|r| r.0.to_string()).collect(),
        bodies: rows.iter().map(|r| r.1.to_string()).collect(),
        starreds: rows.iter().map(|r| r.2).collect(),
    }
}

#[test]
fn a_batch_imports_every_row_in_order() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(
            fx.work,
            &[
                ("Character sheet", "# Character\n", true),
                ("Location", "# Location\n", false),
            ],
        ),
    )
    .expect("import");

    assert_eq!(out.created_ids.len(), 2);
    assert!(
        out.renamed_to.is_empty(),
        "no collisions in an empty project"
    );
    assert_eq!(
        rows(&fx),
        vec![
            ("Character sheet".into(), "# Character\n".into(), true),
            ("Location".into(), "# Location\n".into(), false),
        ]
    );
}

/// The divergence from `import_tags`: a name already present is suffixed and reported,
/// never dropped. Re-importing an edited file must not silently do nothing.
#[test]
fn a_colliding_name_is_suffixed_rather_than_skipped() {
    let fx = make_fixture();
    seed(&fx, "Character sheet");
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(fx.work, &[("Character sheet", "# Revised\n", false)]),
    )
    .expect("import");

    assert_eq!(out.created_ids.len(), 1, "the row must still be created");
    assert_eq!(out.renamed_to, vec!["Character sheet (2)".to_string()]);
    let names: Vec<_> = rows(&fx).into_iter().map(|r| r.0).collect();
    assert_eq!(names, vec!["Character sheet", "Character sheet (2)"]);
    assert!(
        rows(&fx).iter().any(|r| r.1 == "# Revised\n"),
        "the imported body must survive — that is the whole point of suffixing"
    );
}

/// Collision matching ignores case and surrounding space, matching what the UI shows
/// while the writer types.
#[test]
fn collision_matching_ignores_case_and_space() {
    let fx = make_fixture();
    seed(&fx, "Character sheet");
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(fx.work, &[("  CHARACTER SHEET  ", "x", false)]),
    )
    .expect("import");

    assert_eq!(out.renamed_to, vec!["CHARACTER SHEET (2)".to_string()]);
}

/// Two identical names inside one batch collide with each other, not just with what was
/// already there — the `taken` set grows as rows are accepted.
#[test]
fn a_batch_that_repeats_a_name_suffixes_the_later_one() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(
            fx.work,
            &[("Beat sheet", "a", false), ("Beat sheet", "b", false)],
        ),
    )
    .expect("import");

    assert_eq!(out.created_ids.len(), 2);
    assert_eq!(out.renamed_to, vec!["Beat sheet (2)".to_string()]);
    let names: Vec<_> = rows(&fx).into_iter().map(|r| r.0).collect();
    assert_eq!(names, vec!["Beat sheet", "Beat sheet (2)"]);
}

/// Suffixing walks past an already-suffixed name rather than colliding with it.
#[test]
fn suffixing_finds_the_first_free_number() {
    let fx = make_fixture();
    seed(&fx, "Location");
    seed(&fx, "Location (2)");
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(fx.work, &[("Location", "x", false)]),
    )
    .expect("import");

    assert_eq!(out.renamed_to, vec!["Location (3)".to_string()]);
}

/// A blank name is the one thing genuinely dropped: there is nothing to disambiguate to,
/// and the row would be unpickable in the insert menu.
#[test]
fn a_blank_name_is_the_only_row_dropped() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(
            fx.work,
            &[("   ", "orphan", false), ("Kept", "body", false)],
        ),
    )
    .expect("import");

    assert_eq!(out.created_ids.len(), 1);
    let names: Vec<_> = rows(&fx).into_iter().map(|r| r.0).collect();
    assert_eq!(names, vec!["Kept"]);
}

/// The whole batch is ONE undo step — the entire reason this use case exists rather than
/// the UI looping over the generic create command.
#[test]
fn the_whole_batch_is_a_single_undo_step() {
    let fx = make_fixture();
    seed(&fx, "Existing");
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(
            fx.work,
            &[("A", "a", false), ("B", "b", false), ("C", "c", true)],
        ),
    )
    .expect("import");
    assert_eq!(rows(&fx).len(), 4);

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo");
    let names: Vec<_> = rows(&fx).into_iter().map(|r| r.0).collect();
    assert_eq!(
        names,
        vec!["Existing"],
        "one undo must reverse the entire batch and leave what was already there"
    );

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo");
    assert_eq!(rows(&fx).len(), 4, "and one redo must put it all back");
}

/// Importing into one Work must not touch another open Work's templates.
#[test]
fn import_is_scoped_to_the_named_work() {
    let fx = make_fixture();
    let other = mk_work(&fx.ctx, Some(fx.setup), "Other");

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(fx.work, &[("Only here", "x", false)]),
    )
    .expect("import");

    let other_ids = work_commands::get_work_relationship(
        &fx.ctx,
        &other,
        &WorkRelationshipField::NoteTemplates,
    )
    .expect("read other work's templates");
    assert!(
        other_ids.is_empty(),
        "the other Work must not have gained a template"
    );
}

/// Mismatched column lengths are a caller bug and must be loud, not silently truncated.
#[test]
fn mismatched_columns_are_rejected() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let bad = ImportNoteTemplatesDto {
        work_id: fx.work,
        names: vec!["A".into(), "B".into()],
        bodies: vec!["a".into()],
        starreds: vec![false, false],
    };
    let err = note_template_management_commands::import_note_templates(&fx.ctx, Some(stack), &bad)
        .expect_err("mismatched columns must fail");
    assert!(
        format!("{err:#}").contains("column lengths differ"),
        "got: {err:#}"
    );
}

/// An unknown Work id must fail rather than landing the batch somewhere arbitrary.
#[test]
fn an_unopen_work_is_rejected() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    let err = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(999_999, &[("A", "a", false)]),
    )
    .expect_err("an unknown work must fail");
    assert!(format!("{err:#}").contains("not open"), "got: {err:#}");
}

/// The result DTO must report what was **created**, not what was requested.
///
/// The UI builds its "N templates imported" summary from this, and the use case silently
/// drops a blank name — a file whose stem is only punctuation tidies to one. Reporting the
/// request count would tell the writer a file imported that did not, with no warning
/// anywhere, which is exactly what the frontend did until this was pinned.
#[test]
fn created_ids_counts_rows_actually_made_not_rows_requested() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(
            fx.work,
            &[
                ("Kept", "a", false),
                ("   ", "dropped", false),
                ("Also kept", "b", false),
            ],
        ),
    )
    .expect("import");

    assert_eq!(
        out.created_ids.len(),
        2,
        "the blank-named row is dropped, so two were created from three requested"
    );
    assert_eq!(rows(&fx).len(), 2, "and the store agrees");
}

/// Every template a writer can make comes out with a durable identity.
///
/// This is the guard for a failure that **still compiles**. A template's Djot blob is
/// named `templates/<blake3(uid)[..8]>-<slug>.djot`, so a nil uid puts every
/// nil-identified template on one filename — but nothing in the type system says so, and
/// the two places that mint one are easy to lose:
///
/// * `note_template_controller::with_identity` is a hand-added helper on a **generated**
///   file. Regenerating that controller deletes it (the project guide warns about exactly
///   this for the binder controllers), and every row would go back to being created nil.
/// * `import_note_templates` builds its entity with `..Default::default()` and mints the
///   uid by hand, because it writes through the unit of work and never passes the
///   controller at all.
///
/// Both doors are checked here, since the bug only ever becomes visible on disk, after a
/// save, in a project the writer has already been using.
#[test]
fn every_template_is_created_with_a_durable_identity() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    // Door one: the direct-access controller, which the UI's "new template" uses.
    let via_controller = seed(&fx, "Typed by hand");

    // Door two: the bulk import, which every built-in preset and every imported file uses.
    let out = note_template_management_commands::import_note_templates(
        &fx.ctx,
        Some(stack),
        &dto(fx.work, &[("Imported", "body", false)]),
    )
    .expect("import");

    let mut ids = vec![via_controller];
    ids.extend(out.created_ids.iter().copied());
    assert_eq!(ids.len(), 2, "both doors must have produced a row");

    for id in ids {
        let t = note_template_commands::get_note_template(&fx.ctx, &id)
            .expect("read template")
            .expect("the template exists");
        assert!(
            !t.uid.is_nil(),
            "template {:?} was created without an identity, so its .djot blob would \
             collide with every other nil-identified template",
            t.name
        );
    }
}
