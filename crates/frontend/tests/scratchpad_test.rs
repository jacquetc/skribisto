// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The scratchpad (`ContentRole::ScratchpadText`) end to end through the backend: the
//! writer's private notes beside an item, saved and reopened with the project, and
//! never part of the book.
//!
//! Each test builds its own project, with one scene carrying a scene text, a synopsis
//! and a scratchpad, and checks one place a scratchpad must leave alone unless asked:
//! the search and Replace All, which reach it only through its own toggle, and the word
//! count, which never does.
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

/// Every scope on: body, titles, synopses, labels, comments, resources. Scratchpads
/// stay off, as the interface keeps them until the writer turns them on.
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
        search_scratchpad: false,
        include_trashed: true,
    }
}

/// [`search`] with the scratchpad toggle on.
fn search_with_scratchpads(work: EntityId, query: &str) -> RunSearchDto {
    RunSearchDto {
        search_scratchpad: true,
        ..search(work, query)
    }
}

/// [`search_with_scratchpads`] with every other prose scope off: only scratchpads.
fn search_scratchpads_only(work: EntityId, query: &str) -> RunSearchDto {
    RunSearchDto {
        search_body: false,
        search_titles: false,
        search_synopsis: false,
        search_labels: false,
        search_comments: false,
        search_resources: false,
        ..search_with_scratchpads(work, query)
    }
}

/// The current result rows: id and field.
fn result_rows(ctx: &AppContext, work: EntityId) -> Vec<(EntityId, MatchField)> {
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
        .map(|r| (r.id, r.match_field))
        .collect()
}

/// Replace every listed occurrence of the current search with `replacement`, leaving
/// out the result rows in `excluded`, from a window whose Scratchpad switch is on.
fn replace_all(
    p: &Project,
    stack: Option<u64>,
    replacement: &str,
    excluded: Vec<EntityId>,
) -> search_management::ReplaceInProjectResultDto {
    replace_all_from(p, stack, replacement, excluded, true)
}

/// [`replace_all`], from a window whose Scratchpad switch is `switch`.
fn replace_all_from(
    p: &Project,
    stack: Option<u64>,
    replacement: &str,
    excluded: Vec<EntityId>,
    switch: bool,
) -> search_management::ReplaceInProjectResultDto {
    search_management_commands::replace_in_project(
        &p.ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: p.work,
            replacement: replacement.to_string(),
            preserve_case: true,
            excluded_result_ids: excluded,
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
            search_scratchpad: switch,
        },
    )
    .expect("replace_in_project")
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

/// With its toggle off, a word that is only in the scratchpad finds nothing, every other
/// scope on. The same search finds a word of the scene text, so the search itself works.
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

/// With the toggle off, Replace All rewrites the scene text and the synopsis, and leaves
/// the scratchpad word for word as it was, though it holds the same word.
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
            search_scratchpad: false,
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

/// With the toggle on, a word only in the scratchpad is found there and nowhere else, and
/// a word in all three texts is found in each, the scratchpad reported as its own field.
/// The `Search` keeps a record of the toggle, and the occurrences of the scratchpad row
/// are read from the scratchpad, not from the scene text.
#[test]
fn the_scratchpad_toggle_finds_a_word_in_a_scratchpad() {
    let p = project("search-on", Some(SCRATCHPAD));
    let out = search_management_commands::run_search(
        &p.ctx,
        &search_with_scratchpads(p.work, "lighthouse"),
    )
    .unwrap();
    assert_eq!(out.match_count, 1, "found once, in the scratchpad");
    assert_eq!(out.item_count, 1);
    assert_eq!(results(&p.ctx, p.work), vec![MatchField::Scratchpad]);

    let wi = work_info_commands::get_all_work_info(&p.ctx)
        .unwrap()
        .into_iter()
        .find(|wi| wi.work == Some(p.work))
        .unwrap();
    let record = search_commands::get_search(&p.ctx, &wi.search)
        .unwrap()
        .expect("the work's search");
    assert!(record.search_scratchpad, "the Search records the toggle");

    let out =
        search_management_commands::run_search(&p.ctx, &search_with_scratchpads(p.work, "keeper"))
            .unwrap();
    assert_eq!(
        out.match_count, 3,
        "the scene text, the synopsis and the scratchpad"
    );
    let rows = result_rows(&p.ctx, p.work);
    let pad = rows
        .iter()
        .find(|(_, f)| *f == MatchField::Scratchpad)
        .expect("the scratchpad is reported")
        .0;
    let occurrences = search_management_commands::occurrences_for_result(
        &p.ctx,
        &search_management::OccurrencesForResultDto {
            work_id: p.work,
            result_id: pad,
        },
    )
    .expect("occurrences_for_result");
    // "Is the keeper too old?": the word sits seven characters in, where the scene's own
    // "The keeper climbed" has it at four.
    assert_eq!(occurrences.char_starts, vec![7]);
    assert_eq!(occurrences.snippets_match, vec!["keeper".to_string()]);

    // Only the scratchpad toggle: the scene and its synopsis are not reported.
    let out =
        search_management_commands::run_search(&p.ctx, &search_scratchpads_only(p.work, "keeper"))
            .unwrap();
    assert_eq!(out.match_count, 1);
    assert_eq!(results(&p.ctx, p.work), vec![MatchField::Scratchpad]);
}

/// With the toggle on, Replace All rewrites the scratchpad with the rest, and one undo
/// puts all three back.
#[test]
fn replace_all_with_the_toggle_on_rewrites_the_scratchpad() {
    let p = project("replace-on", Some(SCRATCHPAD));
    let stack = Some(undo_redo_commands::create_new_stack(&p.ctx));
    search_management_commands::run_search(&p.ctx, &search_with_scratchpads(p.work, "keeper"))
        .unwrap();
    let out = replace_all(&p, stack, "warden", vec![]);
    assert_eq!(out.occurrences_replaced, 3);
    assert_eq!(
        out.items_changed, 1,
        "one item: its text, synopsis and notes"
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some("Is the warden too old? Ask about the *lighthouse* records.")
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::SceneText).as_deref(),
        Some("The warden climbed the stair and lit the lamp.")
    );

    undo_redo_commands::undo(&p.ctx, stack).expect("undo");
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some(SCRATCHPAD)
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::SceneText).as_deref(),
        Some(SCENE)
    );
}

/// A scratchpad hit rewrites the scratchpad and only it: searching scratchpads alone,
/// the scene text and the synopsis of the same item keep their "keeper". And the
/// writer can leave the scratchpad row out of a replace that finds it.
#[test]
fn a_scratchpad_hit_never_rewrites_the_scene_beside_it() {
    let p = project("replace-only", Some(SCRATCHPAD));
    let stack = Some(undo_redo_commands::create_new_stack(&p.ctx));
    search_management_commands::run_search(&p.ctx, &search_scratchpads_only(p.work, "keeper"))
        .unwrap();
    let out = replace_all(&p, stack, "warden", vec![]);
    assert_eq!(out.occurrences_replaced, 1);
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some("Is the warden too old? Ask about the *lighthouse* records.")
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::SceneText).as_deref(),
        Some(SCENE),
        "the scene text beside the notes is untouched"
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::SynopsisText).as_deref(),
        Some(SYNOPSIS)
    );

    // Found with the rest, but left out by the writer.
    let p = project("replace-excluded", Some(SCRATCHPAD));
    search_management_commands::run_search(&p.ctx, &search_with_scratchpads(p.work, "keeper"))
        .unwrap();
    let pad: Vec<EntityId> = result_rows(&p.ctx, p.work)
        .into_iter()
        .filter(|(_, f)| *f == MatchField::Scratchpad)
        .map(|(id, _)| id)
        .collect();
    assert_eq!(pad.len(), 1);
    let out = replace_all(&p, None, "warden", pad);
    assert_eq!(out.occurrences_replaced, 2);
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some(SCRATCHPAD)
    );
}

/// The Work has one stored search, shared by its windows. Window A searched with its
/// Scratchpad switch on, so the stored results hold A's scratchpad rows; window B, its
/// switch off, runs Replace All over them. The scratchpad is not B's to rewrite: the
/// replace touches the scene text and the synopsis and leaves it word for word.
#[test]
fn a_replace_from_a_window_with_the_switch_off_never_reaches_a_scratchpad() {
    let p = project("replace-other-window", Some(SCRATCHPAD));
    search_management_commands::run_search(&p.ctx, &search_with_scratchpads(p.work, "keeper"))
        .unwrap();
    assert!(results(&p.ctx, p.work).contains(&MatchField::Scratchpad));

    let out = replace_all_from(&p, None, "warden", vec![], false);
    assert_eq!(
        out.occurrences_replaced, 2,
        "the scene text and the synopsis"
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some(SCRATCHPAD),
        "the scratchpad keeps its 'keeper'"
    );
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::SceneText).as_deref(),
        Some("The warden climbed the stair and lit the lamp.")
    );
}

/// Both must say yes: a caller asking for scratchpads over a stored search that did not
/// look in them (its record says the switch was off) rewrites none either.
#[test]
fn a_replace_needs_the_stored_search_to_have_looked_in_scratchpads_too() {
    let p = project("replace-stored-off", Some(SCRATCHPAD));
    search_management_commands::run_search(&p.ctx, &search_with_scratchpads(p.work, "keeper"))
        .unwrap();
    let wi = work_info_commands::get_all_work_info(&p.ctx)
        .unwrap()
        .into_iter()
        .find(|wi| wi.work == Some(p.work))
        .unwrap();
    let mut record = search_commands::get_search(&p.ctx, &wi.search)
        .unwrap()
        .expect("the work's search");
    record.search_scratchpad = false;
    search_commands::update_search(&p.ctx, &record.into()).unwrap();

    let out = replace_all_from(&p, None, "warden", vec![], true);
    assert_eq!(out.occurrences_replaced, 2);
    assert_eq!(
        row_of(&p.ctx, p.scene, ContentRole::ScratchpadText).as_deref(),
        Some(SCRATCHPAD)
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
