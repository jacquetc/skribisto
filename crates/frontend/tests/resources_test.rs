// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `resource_management::add_resources` through the commands the UI calls: one undoable
//! step that creates the rows, lists them on the Work and attaches them to the items, and
//! the generated CRUD around it (remove, detach), which a resource leans on for everything
//! else.

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, resource_commands, resource_management_commands, undo_redo_commands,
    work_commands,
};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ResourceKind};
use frontend::common::types::EntityId;
use frontend::direct_access::CreateBinderItemDto;
use resource_management::AddResourcesDto;
use work_management::{NewWorkDto, NewWorkTemplate};

struct Fixture {
    ctx: AppContext,
    stack: u64,
    work: EntityId,
    scene: EntityId,
    note: EntityId,
}

fn item(sub_role: BinderItemSubRole, title: &str, n: u64) -> CreateBinderItemDto {
    CreateBinderItemDto {
        status: None,
        cover: None,
        uid: common::uid::fixture_uid(n),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        title: title.to_string(),
        sub_title: String::new(),
        role: BinderItemRole::Item,
        sub_role,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        exclude_from_numbering: false,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: Vec::new(),
        contents: vec![],
        references: vec![],
        point_of_view: vec![],
        books: vec![],
        tags: vec![],
        resources: vec![],
    }
}

fn new_project(ctx: &AppContext, dir: &std::path::Path) -> EntityId {
    frontend::commands::work_management_commands::new_work(
        ctx,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: dir.to_string_lossy().to_string(),
            title: String::new(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");
    work_commands::get_all_work(ctx)
        .unwrap()
        .into_iter()
        .map(|w| w.id)
        .max()
        .unwrap()
}

fn fixture() -> Fixture {
    let ctx = AppContext::new();
    let stack = undo_redo_commands::create_new_stack(&ctx);
    let dir = tempfile::tempdir().unwrap();
    let work = new_project(&ctx, &dir.path().join("Novel"));
    let binder = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();
    let created = binder_item_commands::create_binder_item_multi(
        &ctx,
        Some(stack),
        &[
            item(BinderItemSubRole::Scene, "The lighthouse", 1),
            item(BinderItemSubRole::Note, "The tannery", 2),
        ],
        binder,
        -1,
    )
    .unwrap();
    Fixture {
        ctx,
        stack,
        work,
        scene: created[0].id,
        note: created[1].id,
    }
}

/// One of each kind, attached to the scene and the note.
fn one_of_each(fx: &Fixture) -> AddResourcesDto {
    AddResourcesDto {
        work_id: fx.work,
        item_ids: vec![fx.scene, fx.note],
        stored_uids: vec![common::uid::fixture_uid(900).hyphenated().to_string()],
        stored_titles: vec![String::new()],
        stored_file_names: vec!["Village map.pdf".into()],
        stored_mime_types: vec!["application/pdf".into()],
        stored_byte_sizes: vec![8],
        stored_content_hashes: vec!["hash".into()],
        stored_widths: vec![0],
        stored_heights: vec![0],
        linked_paths: vec![if cfg!(windows) {
            r"C:\Users\someone\Research\interview.mp3".into()
        } else {
            "/home/someone/Research/interview.mp3".into()
        }],
        web_urls: vec!["https://www.example.org/tanneries".into()],
        web_titles: vec![String::new()],
    }
}

fn work_resources(fx: &Fixture) -> Vec<EntityId> {
    work_commands::get_work_relationship(&fx.ctx, &fx.work, &WorkRelationshipField::Resources)
        .unwrap()
}

fn item_resources(fx: &Fixture, item: EntityId) -> Vec<EntityId> {
    binder_item_commands::get_binder_item_relationship(
        &fx.ctx,
        &item,
        &BinderItemRelationshipField::Resources,
    )
    .unwrap()
}

#[test]
fn adding_resources_lists_them_on_the_work_and_attaches_them_to_every_item_named() {
    let fx = fixture();
    let out =
        resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &one_of_each(&fx))
            .unwrap();
    assert_eq!(out.created_ids.len(), 3);
    assert_eq!(work_resources(&fx), out.created_ids);
    assert_eq!(item_resources(&fx, fx.scene), out.created_ids);
    assert_eq!(item_resources(&fx, fx.note), out.created_ids);

    let rows: Vec<_> = resource_commands::get_resource_multi(&fx.ctx, &out.created_ids)
        .unwrap()
        .into_iter()
        .flatten()
        .collect();
    let titles: Vec<&str> = rows.iter().map(|r| r.title.as_str()).collect();
    // A title left empty is the file's name without its extension, or the page's host.
    assert_eq!(titles, vec!["Village map", "interview", "example.org"]);
    assert_eq!(rows[0].kind, ResourceKind::StoredFile);
    assert_eq!(rows[0].uid, common::uid::fixture_uid(900));
    assert_eq!(rows[1].kind, ResourceKind::LinkedFile);
    assert_eq!(rows[1].file_name, "interview.mp3");
    assert_eq!(rows[2].kind, ResourceKind::WebLink);
    assert_eq!(rows[2].location, "https://www.example.org/tanneries");
    // A minted identity on every row, none of them nil.
    assert!(rows.iter().all(|r| !r.uid.is_nil()));
}

#[test]
fn one_undo_takes_back_the_rows_the_list_and_the_attachments_and_redo_restores_them() {
    let fx = fixture();
    let out =
        resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &one_of_each(&fx))
            .unwrap();
    undo_redo_commands::undo(&fx.ctx, Some(fx.stack)).unwrap();
    assert!(work_resources(&fx).is_empty());
    assert!(item_resources(&fx, fx.scene).is_empty());
    assert!(
        resource_commands::get_resource_multi(&fx.ctx, &out.created_ids)
            .unwrap()
            .iter()
            .all(Option::is_none)
    );

    undo_redo_commands::redo(&fx.ctx, Some(fx.stack)).unwrap();
    assert_eq!(work_resources(&fx), out.created_ids);
    assert_eq!(item_resources(&fx, fx.note), out.created_ids);
}

#[test]
fn removing_a_resource_detaches_it_from_every_item() {
    let fx = fixture();
    let out =
        resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &one_of_each(&fx))
            .unwrap();
    resource_commands::remove_resource(&fx.ctx, Some(fx.stack), &out.created_ids[0]).unwrap();
    assert_eq!(item_resources(&fx, fx.scene), out.created_ids[1..].to_vec());
    assert_eq!(work_resources(&fx), out.created_ids[1..].to_vec());
}

#[test]
fn an_item_of_another_project_is_refused_and_nothing_is_written() {
    let fx = fixture();
    let dir = tempfile::tempdir().unwrap();
    let other = new_project(&fx.ctx, &dir.path().join("Other"));
    let binder =
        work_commands::get_work_relationship(&fx.ctx, &other, &WorkRelationshipField::Binders)
            .unwrap()
            .pop()
            .unwrap();
    let stranger = binder_item_commands::create_binder_item_multi(
        &fx.ctx,
        Some(fx.stack),
        &[item(BinderItemSubRole::Scene, "Elsewhere", 3)],
        binder,
        -1,
    )
    .unwrap()[0]
        .id;
    let mut dto = one_of_each(&fx);
    dto.item_ids.push(stranger);
    assert!(resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &dto).is_err());
    assert!(work_resources(&fx).is_empty());
}

#[test]
fn a_link_that_is_not_a_web_page_is_refused() {
    let fx = fixture();
    let mut dto = one_of_each(&fx);
    dto.web_urls = vec!["file:///etc/passwd".into()];
    assert!(resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &dto).is_err());
    assert!(work_resources(&fx).is_empty());
}

// ── Search ────────────────────────────────────────────────────────────────────

fn search(
    fx: &Fixture,
    query: &str,
    resources: bool,
) -> Vec<frontend::direct_access::SearchResultDto> {
    use frontend::commands::{
        search_commands, search_management_commands, search_result_commands, work_info_commands,
    };
    use frontend::common::direct_access::search::SearchRelationshipField;
    search_management_commands::run_search(
        &fx.ctx,
        &search_management::RunSearchDto {
            work_id: fx.work,
            query: query.to_string(),
            case_sensitive: false,
            whole_word: false,
            diacritic_sensitive: false,
            facets: vec![],
            search_body: true,
            search_titles: true,
            search_synopsis: true,
            search_labels: false,
            search_comments: false,
            search_resources: resources,
            include_trashed: false,
        },
    )
    .expect("run_search");
    let info = work_info_commands::get_all_work_info(&fx.ctx)
        .unwrap()
        .into_iter()
        .find(|wi| wi.work == Some(fx.work))
        .unwrap();
    let ids = search_commands::get_search_relationship(
        &fx.ctx,
        &info.search,
        &SearchRelationshipField::Results,
    )
    .unwrap();
    search_result_commands::get_search_result_multi(&fx.ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .collect()
}

#[test]
fn a_resource_is_found_under_each_item_it_is_attached_to_and_only_when_asked() {
    use frontend::common::entities::MatchField;
    let fx = fixture();
    let out =
        resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &one_of_each(&fx))
            .unwrap();
    assert!(search(&fx, "tanneries", false).is_empty());

    let hits = search(&fx, "tanneries", true);
    let mut items: Vec<EntityId> = hits.iter().map(|h| h.binder_item_id).collect();
    items.sort();
    let mut expected = vec![fx.scene, fx.note];
    expected.sort();
    assert_eq!(items, expected);
    assert!(hits.iter().all(|h| h.match_field == MatchField::Resource));
    assert!(hits.iter().all(|h| h.resource_id == out.created_ids[2]));

    // The occurrence the row stands for is found again by the reveal.
    let occ = frontend::commands::search_management_commands::occurrences_for_result(
        &fx.ctx,
        &search_management::OccurrencesForResultDto {
            work_id: fx.work,
            result_id: hits[0].id,
        },
    )
    .unwrap();
    assert_eq!(occ.char_starts.len(), hits[0].occurrence_count as usize);
}

#[test]
fn a_resource_attached_to_nothing_is_found_once_with_no_item() {
    let fx = fixture();
    let mut dto = one_of_each(&fx);
    dto.item_ids.clear();
    resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &dto).unwrap();
    let hits = search(&fx, "village", true);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].binder_item_id, 0);
}

#[test]
fn a_linked_files_folders_are_not_searched() {
    let fx = fixture();
    resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &one_of_each(&fx))
        .unwrap();
    assert!(search(&fx, "someone", true).is_empty());
    let hits = search(&fx, "interview", true);
    assert_eq!(hits.len(), 2, "one row per item the resource is on");
    // Its title and its file name, never the folders on its path.
    assert!(hits.iter().all(|h| h.occurrence_count == 2));
}

#[test]
fn a_duplicated_item_keeps_its_resources() {
    let fx = fixture();
    let out =
        resource_management_commands::add_resources(&fx.ctx, Some(fx.stack), &one_of_each(&fx))
            .unwrap();
    let res = frontend::commands::binder_item_management_commands::duplicate(
        &fx.ctx,
        Some(fx.stack),
        &frontend::binder_item_management::DuplicateDto {
            item_ids: vec![fx.scene],
        },
    )
    .unwrap();
    assert_eq!(item_resources(&fx, res.new_item_ids[0]), out.created_ids);
    // Shared, not copied: the Work still lists three.
    assert_eq!(work_resources(&fx).len(), 3);
}
