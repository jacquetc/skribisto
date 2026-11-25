// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The scratchpad (`ContentRole::ScratchpadText`) end to end through the backend: the
//! writer's private notes beside an item, saved and reopened with the project, and
//! never part of the book.
//!
//! Each test builds its own project, with one scene carrying a scene text, a synopsis
//! and a scratchpad, and checks one place a scratchpad must leave alone: the search
//! (it has no toggle yet, so it is never searched), Replace All, and the word count.
//! The last ones save and reopen the project, which is where a blank scratchpad is left
//! out and a written one comes back.

use std::time::Duration;

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, content_commands, progress_management_commands, search_commands,
    search_management_commands, search_result_commands, undo_redo_commands, work_commands,
    work_info_commands, work_management_commands,
};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::search::SearchRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole, MatchField};
use frontend::common::types::EntityId;
use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
use progress_management::CountWordsDto;
use search_management::{ReplaceInProjectDto, RunSearchDto};
use work_management::{LoadWorkDto, NewWorkDto, NewWorkTemplate, SaveWorkDto};

const SCENE: &str = "The keeper climbed the stair and lit the lamp.";
const SYNOPSIS: &str = "The keeper lights the lamp.";
const SCRATCHPAD: &str = "Is the keeper too old? Ask about the *lighthouse* records.";

struct Project {
    ctx: AppContext,
    work: EntityId,
    scene: EntityId,
    dir: std::path::PathBuf,
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// A new project whose one scene carries its text, its synopsis and, when `scratchpad`
/// is given, a scratchpad.
fn project(tag: &str, scratchpad: Option<&str>) -> Project {
    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-scratchpad-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    work_management_commands::new_work(
        &ctx,
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
    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap().id;
    let binder = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .expect("the new Work has a binder");
    let now = chrono::Utc::now();
    let scene = binder_item_commands::create_binder_item_multi(
        &ctx,
        None,
        &[CreateBinderItemDto {
            uid: common::uid::fixture_uid(77),
            created_at: now,
            updated_at: now,
            title: "The lamp".to_string(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            is_exportable: true,
            ..Default::default()
        }],
        binder,
        -1,
    )
    .expect("create the scene")[0]
        .id;
    let mut rows = vec![
        (ContentRole::SceneText, SCENE),
        (ContentRole::SynopsisText, SYNOPSIS),
    ];
    if let Some(text) = scratchpad {
        rows.push((ContentRole::ScratchpadText, text));
    }
    let dtos: Vec<CreateContentDto> = rows
        .into_iter()
        .map(|(role, data)| CreateContentDto {
            uid: Default::default(),
            created_at: now,
            updated_at: now,
            activated: true,
            role,
            data: data.to_string(),
        })
        .collect();
    content_commands::create_content_multi(&ctx, None, &dtos, scene, -1).expect("contents");
    Project {
        ctx,
        work,
        scene,
        dir,
    }
}

fn row_of(ctx: &AppContext, item: EntityId, role: ContentRole) -> Option<String> {
    let ids = binder_item_commands::get_binder_item_relationship(
        ctx,
        &item,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    content_commands::get_content_multi(ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| c.role == role)
        .map(|c| c.data)
}

/// Every scope on: body, titles, synopses, labels, comments, resources.
fn search(work: EntityId, query: &str) -> RunSearchDto {
    RunSearchDto {
        work_id: work,
        query: query.to_string(),
        case_sensitive: false,
        whole_word: false,
        diacritic_sensitive: false,
        facets: vec![],
        search_body: true,
        search_titles: true,
        search_synopsis: true,
        search_labels: true,
        search_comments: true,
        search_resources: true,
        include_trashed: true,
    }
}

fn results(ctx: &AppContext, work: EntityId) -> Vec<MatchField> {
    let wi = work_info_commands::get_all_work_info(ctx)
        .unwrap()
        .into_iter()
        .find(|wi| wi.work == Some(work))
        .expect("the work has a WorkInfo");
    let ids = search_commands::get_search_relationship(
        ctx,
        &wi.search,
        &SearchRelationshipField::Results,
    )
    .unwrap();
    search_result_commands::get_search_result_multi(ctx, &ids)
        .unwrap()
        .into_iter()
        .flatten()
        .map(|r| r.match_field)
        .collect()
}

fn wait(ctx: &AppContext, op: &str) {
    let completion = ctx
        .long_operation_manager
        .lock()
        .unwrap()
        .completion_signal();
    assert!(
        completion.wait_for(op, Some(Duration::from_secs(30))),
        "the operation did not finish within 30s"
    );
}

fn total_words(p: &Project) -> i64 {
    let op = progress_management_commands::count_words(&p.ctx, &CountWordsDto { work_id: p.work })
        .expect("count_words");
    wait(&p.ctx, &op);
    progress_management_commands::get_count_words_result(&p.ctx, &op)
        .expect("count result")
        .expect("a finished count has a result")
        .total_word_count
}

fn save(p: &Project) {
    let op = work_management_commands::save_work(
        &p.ctx,
        &SaveWorkDto {
            work_id: p.work,
            file_name: p.dir.to_string_lossy().to_string(),
            overwrite: true,
            media_root: String::new(),
        },
    )
    .expect("save_work");
    wait(&p.ctx, &op);
    work_management_commands::get_save_work_result(&p.ctx, &op)
        .expect("save result")
        .expect("a finished save has a result");
}

/// The saved project, opened again in a store of its own; and the scene, found by title.
fn reopen(p: &Project) -> (AppContext, EntityId) {
    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: p.dir.to_string_lossy().to_string(),
        },
    )
    .expect("load_work");
    let scene = binder_item_commands::get_all_binder_item(&ctx)
        .unwrap()
        .into_iter()
        .find(|i| i.title == "The lamp")
        .expect("the scene reads back")
        .id;
    (ctx, scene)
}

/// A word that is only in the scratchpad finds nothing, with every scope on. The same
/// search finds a word of the scene text, so the search itself works.
#[test]
fn a_word_only_in_a_scratchpad_is_not_found() {
    let p = project("search", Some(SCRATCHPAD));
    let out =
        search_management_commands::run_search(&p.ctx, &search(p.work, "lighthouse")).unwrap();
    assert_eq!(out.match_count, 0, "the scratchpad is not searched");
    assert!(results(&p.ctx, p.work).is_empty());

    let out = search_management_commands::run_search(&p.ctx, &search(p.work, "keeper")).unwrap();
    assert_eq!(
        out.match_count, 2,
        "once in the scene text, once in the synopsis"
    );
    let fields = results(&p.ctx, p.work);
    assert!(fields.contains(&MatchField::Body));
    assert!(fields.contains(&MatchField::Synopsis));
    assert!(
        !fields.contains(&MatchField::Scratchpad),
        "the scratchpad's own 'keeper' is never reported"
    );
}

/// Replace All rewrites the scene text and the synopsis, and leaves the scratchpad word
/// for word as it was, though it holds the same word.
#[test]
fn replace_all_never_touches_a_scratchpad() {
    let p = project("replace", Some(SCRATCHPAD));
    let stack = Some(undo_redo_commands::create_new_stack(&p.ctx));
    search_management_commands::run_search(&p.ctx, &search(p.work, "keeper")).unwrap();
    let out = search_management_commands::replace_in_project(
        &p.ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: p.work,
            replacement: "warden".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");
    assert_eq!(out.occurrences_replaced, 2);
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::SceneText).as_deref(),
        Some("The warden climbed the stair and lit the lamp.")
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some(SCRATCHPAD),
        "the scratchpad keeps its 'keeper'"
    );
}

/// The word count of the project is the same with a scratchpad as without one.
#[test]
fn a_scratchpad_adds_nothing_to_the_word_count() {
    let without = project("count-without", None);
    let with = project("count-with", Some(SCRATCHPAD));
    let words = total_words(&without);
    assert_eq!(words, 9, "the scene text's nine words");
    assert_eq!(total_words(&with), words);
}

/// A written scratchpad is saved with the project and comes back word for word as the
/// scene's own row, apart from its scene text.
#[test]
fn a_scratchpad_is_saved_and_reopened_with_its_item() {
    let p = project("reopen", Some(SCRATCHPAD));
    save(&p);
    let files: Vec<String> = walk(&p.dir)
        .into_iter()
        .filter(|f| f.ends_with(".scratchpad.djot"))
        .collect();
    assert_eq!(files.len(), 1, "one scratchpad file: {files:?}");

    let (ctx, scene) = reopen(&p);
    assert_eq!(
        row_of(&ctx, scene, ContentRole::ScratchpadText).as_deref(),
        Some(SCRATCHPAD)
    );
    assert_eq!(
        row_of(&ctx, scene, ContentRole::SceneText).as_deref(),
        Some(SCENE)
    );
}

/// A blank scratchpad is not written, so the project reopens without the row and keeps
/// the read floor of a project that never had one.
#[test]
fn a_blank_scratchpad_is_left_out_of_the_saved_project() {
    let p = project("blank", Some("  \n"));
    save(&p);
    assert!(
        !walk(&p.dir).iter().any(|f| f.ends_with(".scratchpad.djot")),
        "no scratchpad file for a blank scratchpad"
    );
    let floor = skrib_format::peek_manifest(&p.dir.to_string_lossy())
        .expect("manifest")
        .format_min_read_version;
    assert!(floor.is_some_and(|f| f < 18), "floor {floor:?}");

    let (ctx, scene) = reopen(&p);
    assert_eq!(row_of(&ctx, scene, ContentRole::ScratchpadText), None);
    assert_eq!(
        row_of(&ctx, scene, ContentRole::SynopsisText).as_deref(),
        Some(SYNOPSIS)
    );
}

fn walk(dir: &std::path::Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(path.to_string_lossy().into_owned());
            }
        }
    }
    out
}
