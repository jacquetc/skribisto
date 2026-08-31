// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Two properties of `BinderTag` that a Qleany regeneration has already silently taken
//! away once, and that nothing else asserts.
//!
//! 1. **A tag created through the controller carries a durable identity.** The UI builds
//!    its `CreateBinderTagDto` with `uid: Default::default()` (see
//!    `work_tags_list_model::create`), so the nil uid has to be replaced at the creation
//!    boundary by `binder_tag_controller::with_identity`. That helper is hand-written into
//!    a generated file, and a regeneration deletes it without breaking the build: every
//!    tag is then born `00000000-0000-0000-0000-000000000000`, they all compare equal, and
//!    `note_capture.toml`'s `recent_tags` degenerates into a single always-matching slot.
//!    Only a test can notice.
//!
//! 2. **Removing the row a tag points at clears the tag's side of the junction.** The two
//!    relationships `creates_in` (a folder) and `note_template` (a template) exist as
//!    relationships rather than raw id fields precisely so that the store sweeps them on
//!    delete. The sweep lives in the *target* entity's table, which is generated from a
//!    `backward_junctions` list, and an entry missing from that list also fails silently:
//!    the tag keeps hydrating a dead id, the settings picker quietly falls back to its
//!    default, and the writer's filing choice evaporates on the next save/open round trip.
//!
//! Both are asserted against the ordinary commands, because those are the doors the app
//! actually uses.

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, binder_tag_commands, note_template_commands, undo_redo_commands,
    work_commands,
};
use frontend::common::direct_access::binder_tag::BinderTagRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    BinderTagRelationshipDto, CreateBinderItemDto, CreateBinderTagDto, CreateNoteTemplateDto,
    CreateWorkDto,
};

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
    let work = work_commands::create_orphan_work(
        &ctx,
        Some(setup),
        &CreateWorkDto {
            statuses: Vec::new(),
            created_at: now(),
            updated_at: now(),
            title: "The Lighthouse".into(),
            ..Default::default()
        },
    )
    .expect("create work")
    .id;
    Fixture { ctx, setup, work }
}

/// Built exactly the way the Tags settings page builds it: uid left at `Default::default()`,
/// which is nil. Supplying one here would test the fixture instead of the boundary.
fn tag_dto(name: &str) -> CreateBinderTagDto {
    CreateBinderTagDto {
        uid: Default::default(),
        created_at: now(),
        updated_at: now(),
        name: name.into(),
        color: "#f00".into(),
        details: String::new(),
        discoverable: false,
        creates_in: None,
        note_template: None,
    }
}

fn mk_folder(fx: &Fixture, title: &str) -> EntityId {
    binder_item_commands::create_orphan_binder_item(
        &fx.ctx,
        Some(fx.setup),
        &CreateBinderItemDto {
            status: None,
            created_at: now(),
            updated_at: now(),
            title: title.into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::None,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
    )
    .expect("create folder")
    .id
}

fn mk_template(fx: &Fixture, name: &str) -> EntityId {
    note_template_commands::create_orphan_note_template(
        &fx.ctx,
        Some(fx.setup),
        &CreateNoteTemplateDto {
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            name: name.into(),
            body: "# Sheet".into(),
            starred: false,
        },
    )
    .expect("create template")
    .id
}

fn set_tag_relationship(
    fx: &Fixture,
    tag: EntityId,
    field: BinderTagRelationshipField,
    to: &[EntityId],
) {
    binder_tag_commands::set_binder_tag_relationship(
        &fx.ctx,
        Some(fx.setup),
        &BinderTagRelationshipDto {
            id: tag,
            field,
            right_ids: to.to_vec(),
        },
    )
    .expect("wire tag relationship");
}

fn tag_relationship(
    fx: &Fixture,
    tag: EntityId,
    field: BinderTagRelationshipField,
) -> Vec<EntityId> {
    binder_tag_commands::get_binder_tag_relationship(&fx.ctx, &tag, &field)
        .expect("read tag relationship")
}

/// Both controller doors, because `create_orphan` and `create` mint through separate call
/// sites and a regeneration removes them one at a time.
#[test]
fn every_tag_created_through_the_controller_gets_a_durable_identity() {
    let fx = make_fixture();

    let orphan =
        binder_tag_commands::create_orphan_binder_tag(&fx.ctx, Some(fx.setup), &tag_dto("Cast"))
            .expect("create orphan tag");

    let owned = binder_tag_commands::create_binder_tag(
        &fx.ctx,
        Some(fx.setup),
        &tag_dto("Places"),
        fx.work,
        -1,
    )
    .expect("create owned tag");

    for created in [&orphan, &owned] {
        assert!(
            !created.uid.is_nil(),
            "tag {:?} was created without an identity, so `note_capture.toml`'s recents \
             and every out-of-tree consumer would see it as the same tag as all the others",
            created.name
        );
        // Read back too: the mint has to survive the store, not just the return value.
        let stored = binder_tag_commands::get_binder_tag(&fx.ctx, &created.id)
            .expect("read tag")
            .expect("the tag exists");
        assert_eq!(stored.uid, created.uid, "the stored uid is the minted one");
    }

    assert_ne!(
        orphan.uid, owned.uid,
        "two tags made the same way must still be told apart"
    );
}

/// The multi doors are separate call sites again, and the batch one is what a preset import
/// through the controller would use.
#[test]
fn tags_created_in_a_batch_each_get_their_own_identity() {
    let fx = make_fixture();

    let made = binder_tag_commands::create_orphan_binder_tag_multi(
        &fx.ctx,
        Some(fx.setup),
        &[tag_dto("Cast"), tag_dto("Places"), tag_dto("Props")],
    )
    .expect("create tags");
    assert_eq!(made.len(), 3);

    let mut seen = std::collections::HashSet::new();
    for tag in &made {
        assert!(!tag.uid.is_nil(), "tag {:?} has no identity", tag.name);
        assert!(
            seen.insert(tag.uid),
            "two tags share the uid {}, which is what a nil mint looks like",
            tag.uid
        );
    }
}

/// Trashing and purging the folder a tag files into must take the tag's pointer with it.
#[test]
fn removing_the_folder_a_tag_files_into_clears_the_tag_s_pointer() {
    let fx = make_fixture();
    let folder = mk_folder(&fx, "Cast");
    let keeper = mk_folder(&fx, "Places");

    let doomed_user = binder_tag_commands::create_orphan_binder_tag(
        &fx.ctx,
        Some(fx.setup),
        &tag_dto("Character"),
    )
    .expect("create tag")
    .id;
    let bystander =
        binder_tag_commands::create_orphan_binder_tag(&fx.ctx, Some(fx.setup), &tag_dto("Setting"))
            .expect("create tag")
            .id;
    set_tag_relationship(
        &fx,
        doomed_user,
        BinderTagRelationshipField::CreatesIn,
        &[folder],
    );
    set_tag_relationship(
        &fx,
        bystander,
        BinderTagRelationshipField::CreatesIn,
        &[keeper],
    );

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_commands::remove_binder_item_multi(&fx.ctx, Some(stack), &[folder])
        .expect("remove folder");

    assert!(
        tag_relationship(&fx, doomed_user, BinderTagRelationshipField::CreatesIn).is_empty(),
        "the tag still points at a purged folder; it would render as \"ask me the first \
         time\" while writing the dead id back into tags.ron on every save"
    );
    assert_eq!(
        tag_relationship(&fx, bystander, BinderTagRelationshipField::CreatesIn),
        vec![keeper],
        "the sweep must not touch a tag that filed somewhere else"
    );
}

/// The same hole on the other new relationship, in a different generated table.
#[test]
fn removing_the_template_a_tag_starts_from_clears_the_tag_s_pointer() {
    let fx = make_fixture();
    let template = mk_template(&fx, "Character sheet");
    let keeper = mk_template(&fx, "Location sheet");

    let doomed_user = binder_tag_commands::create_orphan_binder_tag(
        &fx.ctx,
        Some(fx.setup),
        &tag_dto("Character"),
    )
    .expect("create tag")
    .id;
    let bystander =
        binder_tag_commands::create_orphan_binder_tag(&fx.ctx, Some(fx.setup), &tag_dto("Setting"))
            .expect("create tag")
            .id;
    set_tag_relationship(
        &fx,
        doomed_user,
        BinderTagRelationshipField::NoteTemplate,
        &[template],
    );
    set_tag_relationship(
        &fx,
        bystander,
        BinderTagRelationshipField::NoteTemplate,
        &[keeper],
    );

    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    note_template_commands::remove_note_template_multi(&fx.ctx, Some(stack), &[template])
        .expect("remove template");

    assert!(
        tag_relationship(&fx, doomed_user, BinderTagRelationshipField::NoteTemplate).is_empty(),
        "the tag still points at a deleted template, so capture would silently start the \
         note from an empty body while the picker showed \"Blank note\""
    );
    assert_eq!(
        tag_relationship(&fx, bystander, BinderTagRelationshipField::NoteTemplate),
        vec![keeper],
        "the sweep must not touch a tag that uses another template"
    );
}
