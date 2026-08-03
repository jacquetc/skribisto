// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for the app-lifecycle use cases — `initialize_app`
//! (startup seed) and `clean_up_before_exit` (exit teardown) — and the
//! single-Root/single-System invariant they establish across load/new opens.
//!
//! The store holds **exactly one** `Root` and **one** `System` for the whole
//! process; multiple open `Work`s hang off `Root.works`. `initialize_app` seeds
//! that shared frame at startup and returns the `Root` id; `new_work`/`load_work`
//! reuse it instead of minting a fresh pair (which historically leaked a second
//! Root + System on the second open in a session).

use frontend::AppContext;
use frontend::commands::{
    handling_app_lifecycle_commands, root_commands, system_commands, work_commands,
    work_management_commands,
};
use frontend::common::direct_access::system::SystemRelationshipField;
use work_management::{NewWorkDto, NewWorkTemplate};

fn root_count(ctx: &AppContext) -> usize {
    root_commands::get_all_root(ctx)
        .expect("get_all_root")
        .len()
}
fn system_count(ctx: &AppContext) -> usize {
    system_commands::get_all_system(ctx)
        .expect("get_all_system")
        .len()
}
fn the_root_id(ctx: &AppContext) -> u64 {
    root_commands::get_all_root(ctx).expect("get_all_root")[0].id
}
fn new_work_dto(name: &str) -> NewWorkDto {
    NewWorkDto {
        file_name: format!("/tmp/{name}.skrib"),
        is_folder: false,
        template_kind: NewWorkTemplate::Novel,
        labels: vec![],
        language: vec!["en".to_string()],
        chapter_scene_mode: false,
        author_name: String::new(),
    }
}

#[test]
fn initialize_app_seeds_exactly_one_root_and_system() {
    let ctx = AppContext::new();
    // Store starts empty — nothing is seeded until initialize_app runs.
    assert_eq!(root_count(&ctx), 0);
    assert_eq!(system_count(&ctx), 0);

    let res = handling_app_lifecycle_commands::initialize_app(&ctx)
        .expect("initialize_app should succeed");

    assert_eq!(root_count(&ctx), 1, "exactly one Root seeded");
    assert_eq!(system_count(&ctx), 1, "exactly one System seeded");
    assert_ne!(res.root_id, 0, "returns a real Root id");
    assert_eq!(
        res.root_id,
        the_root_id(&ctx),
        "returned id is the seeded Root"
    );
    // No Work is opened at startup — the frame is empty.
    assert!(
        work_commands::get_all_work(&ctx)
            .expect("get_all_work")
            .is_empty(),
        "no Work should exist right after initialize_app"
    );
}

#[test]
fn initialize_app_is_idempotent() {
    let ctx = AppContext::new();
    let first = handling_app_lifecycle_commands::initialize_app(&ctx).unwrap();
    let second = handling_app_lifecycle_commands::initialize_app(&ctx).unwrap();
    assert_eq!(
        first.root_id, second.root_id,
        "a second initialize_app reuses the existing Root"
    );
    assert_eq!(root_count(&ctx), 1, "no duplicate Root");
    assert_eq!(system_count(&ctx), 1, "no duplicate System");
}

#[test]
fn new_work_reuses_the_shared_frame() {
    let ctx = AppContext::new();
    let init = handling_app_lifecycle_commands::initialize_app(&ctx).unwrap();

    work_management_commands::new_work(&ctx, &new_work_dto("one")).expect("new_work one");
    assert_eq!(
        root_count(&ctx),
        1,
        "new_work must reuse the Root, not add one"
    );
    assert_eq!(system_count(&ctx), 1, "new_work must reuse the System");
    assert_eq!(
        the_root_id(&ctx),
        init.root_id,
        "Root id unchanged across new_work"
    );

    // A second new_work in one session used to leak a second Root + System —
    // must remain exactly one of each.
    work_management_commands::new_work(&ctx, &new_work_dto("two")).expect("new_work two");
    assert_eq!(
        root_count(&ctx),
        1,
        "second new_work must not duplicate the Root"
    );
    assert_eq!(
        system_count(&ctx),
        1,
        "second new_work must not duplicate the System"
    );
    assert_eq!(
        the_root_id(&ctx),
        init.root_id,
        "Root id stable across opens"
    );
    // Opening a second Work no longer replaces the first (see
    // `frontend::tests::multi_work_scoping_test`) — both remain open, exactly
    // as two simultaneously-open project windows require.
    assert_eq!(
        work_commands::get_all_work(&ctx).unwrap().len(),
        2,
        "opening a second work must leave the first one open too"
    );

    // Recents accumulate on the shared System — the trunk *appends* rather than
    // replacing the `RecentWorks` list, so two opens leave two entries linked.
    let system_id = system_commands::get_all_system(&ctx).unwrap()[0].id;
    let recents = system_commands::get_system_relationship(
        &ctx,
        &system_id,
        &SystemRelationshipField::RecentWorks,
    )
    .expect("get_system_relationship RecentWorks");
    assert_eq!(
        recents.len(),
        2,
        "each open appends a RecentWork to the shared System"
    );
}

#[test]
fn new_work_without_init_is_still_singleton() {
    // Even if initialize_app never ran, the get-or-create in the trunk keeps the
    // store to exactly one Root/System across repeated opens.
    let ctx = AppContext::new();
    work_management_commands::new_work(&ctx, &new_work_dto("a")).expect("new_work a");
    work_management_commands::new_work(&ctx, &new_work_dto("b")).expect("new_work b");
    assert_eq!(root_count(&ctx), 1);
    assert_eq!(system_count(&ctx), 1);
}

#[test]
fn clean_up_before_exit_tears_down_the_frame() {
    let ctx = AppContext::new();
    handling_app_lifecycle_commands::initialize_app(&ctx).unwrap();
    work_management_commands::new_work(&ctx, &new_work_dto("bye")).unwrap();

    handling_app_lifecycle_commands::clean_up_before_exit(&ctx)
        .expect("clean_up_before_exit should succeed");

    assert_eq!(root_count(&ctx), 0, "Root removed at teardown");
    assert_eq!(system_count(&ctx), 0, "System removed at teardown");
}
