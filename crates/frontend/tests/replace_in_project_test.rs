// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `replace_in_project`: the replace pipe, and its safety net.
//!
//! `run_search` → the writer reviews and unticks some rows → `replace_in_project`
//! rewrites the rest through the Djot parser → **one `undo` puts the manuscript back**.
//!
//! What these tests pin is the part that must never regress: that a bulk rewrite of
//! someone's prose is a single, reversible action, that it goes through the parser rather
//! than string-surgering the markup, and that it refuses to guess when the text moved
//! under it.

use frontend::AppContext;
use frontend::commands::{
    content_commands, search_commands, search_management_commands, search_result_commands,
    undo_redo_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::search::SearchRelationshipField;
use search_management::{ReplaceInProjectDto, RunSearchDto};
use work_management::LoadWorkDto;

fn fixture_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn loaded_ctx() -> AppContext {
    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: String::new(),
            file_name: fixture_path(),
        },
    )
    .expect("load_work");
    ctx
}

fn work_id(ctx: &AppContext) -> u64 {
    work_commands::get_all_work(ctx)
        .expect("get_all_work")
        .pop()
        .unwrap()
        .id
}

fn search(ctx: &AppContext, query: &str) -> RunSearchDto {
    RunSearchDto {
        work_id: work_id(ctx),
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
        include_trashed: false,
    }
}

fn result_ids(ctx: &AppContext) -> Vec<u64> {
    let wi = frontend::commands::work_info_commands::get_all_work_info(ctx)
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    search_commands::get_search_relationship(ctx, &wi.search, &SearchRelationshipField::Results)
        .unwrap()
}

/// Every `Content` in the store, as one blob. The bluntest possible way to ask "did the
/// prose actually change, and did it actually change back".
fn all_prose(ctx: &AppContext) -> String {
    let mut all: Vec<String> = content_commands::get_all_content(ctx)
        .unwrap()
        .into_iter()
        .map(|c| c.data)
        .collect();
    all.sort();
    all.join("\u{1}")
}

/// The whole safety story, end to end: rewrite the manuscript, then take it back.
///
/// If this ever fails, Replace All must not ship — a bulk rewrite a writer cannot undo
/// is the single most destructive thing this app could do to them.
#[test]
fn a_replace_is_one_undoable_step() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let before = all_prose(&ctx);
    assert!(
        before.contains("ipsum"),
        "the fixture must contain the word"
    );

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "IPSUM-REPLACED".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert!(
        out.items_changed > 0,
        "the replace must have touched fields"
    );
    assert!(out.occurrences_replaced > 0);
    assert!(
        out.skipped_stale.is_empty(),
        "nothing moved under us: {:?}",
        out.skipped_stale
    );

    let after = all_prose(&ctx);
    assert!(
        after.contains("IPSUM-REPLACED"),
        "the replacement must be in the prose"
    );
    assert_ne!(before, after, "the manuscript must actually have changed");

    // ── and back, in ONE step ────────────────────────────────────────────────────
    undo_redo_commands::undo(&ctx, stack).expect("undo");

    let restored = all_prose(&ctx);
    assert_eq!(
        restored, before,
        "one undo must restore the manuscript EXACTLY — every scene, byte for byte"
    );
    assert!(
        !restored.contains("IPSUM-REPLACED"),
        "no trace of the replacement may survive the undo"
    );

    // Redo puts it back, so the writer can change their mind twice.
    undo_redo_commands::redo(&ctx, stack).expect("redo");
    assert_eq!(all_prose(&ctx), after, "redo must reapply the whole batch");
}

/// A row the writer unticked must be left alone. This is what makes "rename Aurélien,
/// but not in the dialogue where the nickname is right" possible at all.
#[test]
fn excluded_rows_are_left_untouched() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let ids = result_ids(&ctx);
    assert!(ids.len() >= 2, "need at least two rows to exclude one");

    // Exclude the first row; everything else gets rewritten.
    let excluded = ids[0];
    let excluded_row = search_result_commands::get_search_result(&ctx, &excluded)
        .unwrap()
        .unwrap();

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "ZZZ".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![excluded],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert_eq!(
        out.items_changed as usize,
        ids.len() - 1,
        "every row except the excluded one must have been rewritten"
    );

    // The excluded field must still contain the original word.
    let item_contents = {
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        let cids = frontend::commands::binder_item_commands::get_binder_item_relationship(
            &ctx,
            &excluded_row.binder_item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap();
        content_commands::get_content_multi(&ctx, &cids)
            .unwrap()
            .into_iter()
            .flatten()
            .map(|c| c.data)
            .collect::<Vec<_>>()
            .join(" ")
    };
    assert!(
        item_contents.to_lowercase().contains("ipsum"),
        "the excluded row's field must still hold the original word, got: {item_contents:?}"
    );
}

/// If the text moved between the writer reviewing it and committing, the use case must
/// SKIP that field and say so — never rewrite words it was not shown.
#[test]
fn a_field_that_moved_under_us_is_skipped_and_reported() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let ids = result_ids(&ctx);
    let row = search_result_commands::get_search_result(&ctx, &ids[0])
        .unwrap()
        .unwrap();

    // Simulate the writer editing that very scene after reviewing the results: add one
    // more occurrence, so its count no longer matches what the row recorded.
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    let cids = frontend::commands::binder_item_commands::get_binder_item_relationship(
        &ctx,
        &row.binder_item_id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    let mut victim = content_commands::get_content_multi(&ctx, &cids)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| c.data.to_lowercase().contains("ipsum"))
        .expect("the row's field");
    victim
        .data
        .push_str("\n\nAnd one more ipsum, typed after the search ran.");
    content_commands::update_content(
        &ctx,
        stack,
        &frontend::content::dtos::UpdateContentDto {
            uid: victim.uid,
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: victim.data.clone(),
        },
    )
    .expect("update_content");

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "QQQ".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert!(
        !out.skipped_stale.is_empty(),
        "the edited field must be reported as stale, not silently rewritten"
    );

    let reloaded = content_commands::get_content(&ctx, &victim.id)
        .unwrap()
        .unwrap();
    assert!(
        !reloaded.data.contains("QQQ"),
        "a field that moved under us must be left completely alone"
    );
}

/// `preserve_case` was a DTO field nothing read — so a character rename would have
/// lowercased every capitalised occurrence. It is the whole point of a rename: renaming
/// Aurélien must leave AURÉLIEN as AURÉLIAN, not aurélian.
#[test]
fn preserve_case_keeps_the_case_it_found() {
    use frontend::commands::binder_item_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    // Pick a SCENE-TEXT content deliberately. `get_all_binder_item` iterates a HashMap,
    // so "the first item with a Content row" is nondeterministic — and if it landed on a
    // title Content, a body-scoped search would never read it and the test would pass or
    // fail depending on hash order.
    let victim = binder_item_commands::get_all_binder_item(&ctx)
        .unwrap()
        .into_iter()
        .filter(|i| i.activated)
        .find_map(|i| {
            let cids = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &i.id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap();
            content_commands::get_content_multi(&ctx, &cids)
                .unwrap()
                .into_iter()
                .flatten()
                .find(|c| c.role == frontend::common::entities::ContentRole::SceneText)
        })
        .expect("an item with a SceneText content row");
    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            uid: victim.uid,
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: "Aurélien, AURÉLIEN and aurélien walked on.".to_string(),
        },
    )
    .unwrap();

    let mut q = search(&ctx, "aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    search_management_commands::run_search(&ctx, &q).expect("run_search");

    search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    let out = content_commands::get_content(&ctx, &victim.id)
        .unwrap()
        .unwrap()
        .data;
    assert!(
        out.contains("Aurélian"),
        "Titlecase must stay titlecase: {out:?}"
    );
    assert!(
        out.contains("AURÉLIAN"),
        "ALL CAPS must stay ALL CAPS: {out:?}"
    );
    assert!(
        out.contains("aurélian"),
        "lowercase stays lowercase: {out:?}"
    );
    assert!(
        !out.to_lowercase().contains("aurélien"),
        "no occurrence of the old name may survive: {out:?}"
    );
}

/// **A2: the splice happens inside the document, not on the markup.**
///
/// `find_and_replace` splices at the offsets the parser reports and the exporter
/// re-serialises — so a rename never rewrites the query where it happens to appear in
/// markup (a link's URL, an image path) and never drops formatting under the match.
#[test]
fn a_rename_spares_the_markup_and_keeps_the_styling() {
    use frontend::commands::binder_item_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    // A scene where the name is emphasised in the prose AND appears inside a link's URL.
    let victim = binder_item_commands::get_all_binder_item(&ctx)
        .unwrap()
        .into_iter()
        .filter(|i| i.activated)
        .find_map(|i| {
            let cids = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &i.id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap();
            content_commands::get_content_multi(&ctx, &cids)
                .unwrap()
                .into_iter()
                .flatten()
                .find(|c| c.role == frontend::common::entities::ContentRole::SceneText)
        })
        .expect("a scene with body text");

    let source = "She called *Aurélien* home, then read \
                  [the note](https://example.test/Aurélien-notes).";
    content_commands::update_content(
        &ctx,
        None,
        &frontend::content::dtos::UpdateContentDto {
            uid: victim.uid,
            id: victim.id,
            created_at: victim.created_at,
            updated_at: victim.updated_at,
            activated: victim.activated,
            role: victim.role.clone(),
            data: source.to_string(),
        },
    )
    .unwrap();

    let mut q = search(&ctx, "Aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    search_management_commands::run_search(&ctx, &q).expect("run_search");

    search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    let out = content_commands::get_content(&ctx, &victim.id)
        .unwrap()
        .unwrap()
        .data;

    assert!(
        out.contains("*Aurélian*"),
        "the emphasis under the renamed name must survive — the old string rewrite dropped \
         it: {out:?}"
    );
    assert!(
        out.contains("https://example.test/Aurélien-notes"),
        "the link's DESTINATION is markup, not prose. The writer never typed it into their \
         sentence and cannot see it; a rename must not reach inside it: {out:?}"
    );
    assert!(
        !out.contains("*Aurélien*"),
        "the prose occurrence must actually have been renamed: {out:?}"
    );
}

/// Anchor a thread (plus one reply) to the first Content row in the fixture, and
/// hand back `(comment_id, reply_id)`. Mirrors `run_search_test.rs`'s own
/// `seed_comment` — the two test binaries are compiled independently and this
/// repo duplicates small fixtures like this one rather than adding a shared
/// `tests/common` module for a single helper.
fn seed_comment(ctx: &AppContext, work: u64, body: &str, reply_body: &str) -> (u64, u64) {
    use frontend::commands::{binder_item_commands, comment_commands, comment_reply_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::direct_access::comment::CommentRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason};
    use frontend::direct_access::comment::dtos::CreateCommentDto;
    use frontend::direct_access::comment_reply::dtos::CreateCommentReplyDto;
    use frontend::direct_access::work::dtos::WorkRelationshipDto;

    let now = chrono::Utc::now();
    let content = {
        use frontend::commands::binder_commands;
        use frontend::common::direct_access::binder::BinderRelationshipField;
        let binders =
            work_commands::get_work_relationship(ctx, &work, &WorkRelationshipField::Binders)
                .expect("work binders");
        binders
            .into_iter()
            .flat_map(|b| {
                binder_commands::get_binder_relationship(
                    ctx,
                    &b,
                    &BinderRelationshipField::BinderItems,
                )
                .expect("binder items")
            })
            .find_map(|item| {
                binder_item_commands::get_binder_item_relationship(
                    ctx,
                    &item,
                    &BinderItemRelationshipField::Contents,
                )
                .ok()
                .and_then(|c| c.first().copied())
            })
            .expect("this work has at least one Content row")
    };

    let comment = comment_commands::create_orphan_comment(
        ctx,
        None,
        &CreateCommentDto {
            uid: Default::default(),
            created_at: now,
            updated_at: now,
            content: Some(content),
            kind: CommentAnchorKind::Range,
            author_name: "Jane".into(),
            author_initials: "J".into(),
            body: body.into(),
            resolved: false,
            orphaned: false,
            orphan_reason: CommentOrphanReason::NotOrphaned,
            range_start: 0,
            range_length: 4,
            quote_prefix: String::new(),
            quote_exact: "The ".into(),
            quote_exact_truncated: false,
            quote_suffix: String::new(),
            block_ordinal_hint: 0,
            replies: vec![],
        },
    )
    .expect("create comment")
    .id;

    let reply = comment_reply_commands::create_orphan_comment_reply(
        ctx,
        None,
        &CreateCommentReplyDto {
            uid: common::uid::fixture_uid(6001),
            created_at: now,
            updated_at: now,
            author_name: "Marc".into(),
            author_initials: "M".into(),
            body: reply_body.into(),
        },
    )
    .expect("create reply")
    .id;
    comment_commands::set_comment_relationship(
        ctx,
        None,
        &frontend::direct_access::comment::dtos::CommentRelationshipDto {
            id: comment,
            field: CommentRelationshipField::Replies,
            right_ids: vec![reply],
        },
    )
    .expect("wire reply onto comment");

    let mut ids =
        work_commands::get_work_relationship(ctx, &work, &WorkRelationshipField::Comments)
            .expect("work comments");
    ids.push(comment);
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::Comments,
            right_ids: ids,
        },
    )
    .expect("wire comment onto work");
    (comment, reply)
}

/// **M-S4.** A comment's own body is Djot now, not a plain string, and a rename
/// touching one must go through the same document-splice pipeline prose gets —
/// see the `Comment`/`CommentReply` arm's own doc for why a raw string rewrite
/// would be wrong here. Two things prove that:
///
/// * The occurrence count `run_search` reports (scanned through
///   `corpus_cache::corpus_for`, i.e. the **parsed** prose) must still match what
///   `replace_in_project` re-derives by parsing the same body into a
///   `BatchDocument` — if the two disagreed, every formatted comment would be
///   reported stale and Replace All would silently skip it.
/// * The emphasis marker around the renamed word must survive the splice, not
///   be corrupted or dropped by it — the exact failure mode a plain
///   `str::replace` over the raw Djot would risk (`*Aurélien*` losing its `*`s,
///   or the replacement landing inside the marker instead of the word).
#[test]
fn a_replace_inside_a_formatted_comment_body_does_not_corrupt_its_djot_markers() {
    let ctx = loaded_ctx();
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));
    let work = work_id(&ctx);

    let (comment_id, reply_id) = seed_comment(
        &ctx,
        work,
        "Is *Aurélien* really the right name here?",
        "I still think *Aurélien* works.",
    );

    let mut q = search(&ctx, "Aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    q.search_body = false;
    q.search_comments = true;
    search_management_commands::run_search(&ctx, &q).expect("run_search");

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work,
            replacement: "Aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert!(
        out.skipped_stale.is_empty(),
        "the comment/reply occurrence count must agree between run_search and \
         replace_in_project, or every formatted comment would be reported stale \
         and never actually rewritten: {:?}",
        out.skipped_stale
    );
    assert_eq!(
        out.occurrences_replaced, 2,
        "one occurrence in the comment, one in the reply"
    );

    let comment = frontend::commands::comment_commands::get_comment(&ctx, &comment_id)
        .unwrap()
        .unwrap();
    assert!(
        comment.body.contains("*Aurélian*"),
        "the emphasis around the renamed word must survive the splice: {:?}",
        comment.body
    );
    assert!(
        !comment.body.contains("Aurélien"),
        "the old name must actually be gone: {:?}",
        comment.body
    );

    let reply = frontend::commands::comment_reply_commands::get_comment_reply(&ctx, &reply_id)
        .unwrap()
        .unwrap();
    assert!(
        reply.body.contains("*Aurélian*"),
        "the reply's own emphasis must survive too: {:?}",
        reply.body
    );

    // …and it undoes in the same one step as a prose rename.
    undo_redo_commands::undo(&ctx, stack).expect("undo");
    let restored = frontend::commands::comment_commands::get_comment(&ctx, &comment_id)
        .unwrap()
        .unwrap();
    assert!(
        restored.body.contains("*Aurélien*"),
        "undo must put the comment's exact original Djot back: {:?}",
        restored.body
    );
}

/// A manuscript of `n` one-paragraph scenes, every one of them naming Aurélien — the shape
/// of the operation this feature exists for: renaming a character who is *in the book*.
///
/// Built rather than loaded from the bundled fixture, which is a handful of scenes: the
/// thing under test is what happens when the result set is bigger than a list a person
/// would scroll, and no small fixture can produce that.
fn manuscript_of(scenes: usize) -> AppContext {
    use frontend::commands::{binder_item_commands, work_commands};
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
    use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
    use work_management::{NewWorkDto, NewWorkTemplate};

    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-bigrename-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["fr-FR".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap();
    let binder =
        work_commands::get_work_relationship(&ctx, &work.id, &WorkRelationshipField::Binders)
            .unwrap()
            .pop()
            .expect("the new Work must have a binder");

    let now = chrono::Utc::now();
    let items: Vec<CreateBinderItemDto> = (0..scenes)
        .map(|i| CreateBinderItemDto {
            uid: common::uid::fixture_uid(i as u64),
            created_at: now,
            updated_at: now,
            title: format!("Scène {i}"),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
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
        })
        .collect();
    let created = binder_item_commands::create_binder_item_multi(&ctx, None, &items, binder, -1)
        .expect("create_binder_item_multi");

    for (i, item) in created.iter().enumerate() {
        content_commands::create_content_multi(
            &ctx,
            None,
            &[CreateContentDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                activated: true,
                role: ContentRole::SceneText,
                data: format!(
                    "Aurélien traversa la forêt, et le vent portait l'odeur du sel ({i})."
                ),
            }],
            item.id,
            -1,
        )
        .expect("create_content_multi");
    }

    let _ = std::fs::remove_dir_all(&dir);
    ctx
}

/// **A manuscript-wide rename must not be refused for being manuscript-wide.**
///
/// A result row is one matching *field*, and `run_search` stops at `RESULT_CAP` and reports
/// `truncated` — which the UI reads as "this scan cannot honestly claim completeness" and
/// switches Replace All **off** (`SearchReplaceViewModel::can_replace_all`). That cap used
/// to be 300, which is the size of a list a person can scroll rather than the size of a
/// novel: renaming a character who appears in 400 of a book's scenes tripped it, and the one
/// operation the search panel exists for went dark exactly when it was worth doing, leaving
/// the writer to retype the name scene by scene.
///
/// 400 scenes here, every one a hit — comfortably past the old cap — and the whole rename
/// must still go through, in one reversible step.
#[test]
fn a_rename_across_more_scenes_than_the_old_cap_still_goes_through() {
    const SCENES: usize = 400;

    let ctx = manuscript_of(SCENES);
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    let mut q = search(&ctx, "Aurélien");
    q.search_titles = false;
    q.search_synopsis = false;
    let scan = search_management_commands::run_search(&ctx, &q).expect("run_search");

    assert!(
        !scan.truncated,
        "{SCENES} matching scenes must produce a COMPLETE scan — a truncated one disables \
         Replace All, which is the whole feature"
    );
    assert_eq!(
        scan.item_count as usize, SCENES,
        "every scene names Aurélien, so every scene must be listed"
    );

    let before = all_prose(&ctx);
    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Aurélian".to_string(),
            preserve_case: true,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert_eq!(
        out.items_changed as usize, SCENES,
        "the rename must reach every scene, not the first few hundred"
    );
    assert_eq!(out.occurrences_replaced as usize, SCENES);
    assert!(
        out.skipped_stale.is_empty(),
        "nothing moved under us: {:?}",
        out.skipped_stale
    );

    let after = all_prose(&ctx);
    assert!(
        !after.contains("Aurélien"),
        "not one occurrence may be left behind — a rename that misses some is worse than one \
         that is refused, because nothing says which"
    );

    // …and it is still ONE undo, however many scenes it crossed. The snapshot is a
    // structural clone of the Work's entity tree, so 400 scenes cost what 4 do.
    undo_redo_commands::undo(&ctx, stack).expect("undo");
    assert_eq!(
        all_prose(&ctx),
        before,
        "one undo must put all {SCENES} scenes back, byte for byte"
    );
}

// ── replacing ONE occurrence, not one field ────────────────────────────────
//
// A result row is one matching *field*, and until the results tree there was no
// way to say anything finer than "this whole scene's prose". The tree lists the
// hits inside a row one by one and offers to replace one of them, and the tests
// below are about the two ways that goes silently wrong.

/// One scene whose prose is `text`, in a throwaway project.
///
/// Built rather than loaded because these tests turn on *how many* times the query
/// occurs in a single field, and the bundled fixture's counts are whatever its
/// prose happens to say.
fn one_scene(tag: &str, text: &str) -> (AppContext, u64) {
    use frontend::commands::{binder_item_commands, work_commands};
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
    use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};
    use work_management::{NewWorkDto, NewWorkTemplate};

    let ctx = AppContext::new();
    let dir = std::env::temp_dir().join(format!("skrib-one-scene-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["fr-FR".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap();
    let binder =
        work_commands::get_work_relationship(&ctx, &work.id, &WorkRelationshipField::Binders)
            .unwrap()
            .pop()
            .expect("the new Work must have a binder");

    let now = chrono::Utc::now();
    let created = binder_item_commands::create_binder_item_multi(
        &ctx,
        None,
        &[CreateBinderItemDto {
            uid: common::uid::fixture_uid(1),
            created_at: now,
            updated_at: now,
            title: "Scène".to_string(),
            sub_title: String::new(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
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
        }],
        binder,
        -1,
    )
    .expect("create_binder_item_multi");

    let content = content_commands::create_content_multi(
        &ctx,
        None,
        &[CreateContentDto {
            uid: Default::default(),
            created_at: now,
            updated_at: now,
            activated: true,
            role: ContentRole::SceneText,
            data: text.to_string(),
        }],
        created[0].id,
        -1,
    )
    .expect("create_content_multi")[0]
        .id;

    let _ = std::fs::remove_dir_all(&dir);
    (ctx, content)
}

/// Where every hit of `query` starts in the one result row, as the tree sees them.
fn offsets(ctx: &AppContext, row: u64) -> Vec<i64> {
    search_management_commands::occurrences_for_result(
        ctx,
        &search_management::OccurrencesForResultDto {
            work_id: work_id(ctx),
            result_id: row,
        },
    )
    .expect("occurrences_for_result")
    .char_starts
}

/// The scene's prose, verbatim, by the id it was created under.
///
/// Named rather than searched for: `get_all_content` is every `Content` in the
/// store and its order is the store's, so "the first non-empty one" is a coin
/// toss between this scene and the new project's own seeded rows -- which is a
/// test that passes alone and fails beside its neighbours.
fn prose(ctx: &AppContext, content: u64) -> String {
    content_commands::get_content(ctx, &content)
        .unwrap()
        .expect("the scene's content")
        .data
}

/// **Replace this one, and only this one.**
///
/// The whole point of the results tree's per-hit replace. Before this the DTO
/// carried the caller's picks and the use case never read them, so "replace this
/// occurrence" quietly rewrote every occurrence in the field.
#[test]
fn only_the_named_occurrence_is_replaced() {
    let (ctx, content) = one_scene(
        "only-one",
        "Anna partit. Anna revint. Anna repartit. Anna resta. Anna dormit.",
    );
    let stack = Some(undo_redo_commands::create_new_stack(&ctx));

    search_management_commands::run_search(&ctx, &search(&ctx, "Anna")).expect("run_search");
    let rows = result_ids(&ctx);
    assert_eq!(rows.len(), 1, "one scene, one matching field");
    let row = rows[0];
    let at = offsets(&ctx, row);
    assert_eq!(at.len(), 5, "five hits to choose between");

    let out = search_management_commands::replace_in_project(
        &ctx,
        stack,
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Bérénice".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![row],
            only_occurrence_starts: vec![at[2]],
        },
    )
    .expect("replace_in_project");

    assert_eq!(out.occurrences_replaced, 1, "exactly the one named");
    assert!(out.skipped_stale.is_empty());
    assert_eq!(
        prose(&ctx, content),
        "Anna partit. Anna revint. Bérénice repartit. Anna resta. Anna dormit.",
        "the third hit, and no other"
    );
}

/// **The reason it is `only_*` and not "exclude the others".**
///
/// A results tree lists at most `OCCURRENCE_CAP` hits of a field, and a field can
/// hold more. Turning "replace this one" into "skip the other n-1" is therefore a
/// list of the hits the caller could *see*, and every hit past the cap is one
/// nobody named -- so every one of them gets rewritten. That is what "it replaced
/// far more than the one I clicked" was, and it is invisible until a scene is long
/// enough, which no small fixture is.
///
/// Both halves are asserted: the naming works, and the excluding is what breaks.
#[test]
fn naming_one_hit_survives_a_field_with_more_hits_than_a_tree_will_list() {
    // Comfortably past OCCURRENCE_CAP (500), which is the most a row will list.
    const HITS: usize = 560;
    let text = (0..HITS)
        .map(|i| format!("Anna {i}."))
        .collect::<Vec<_>>()
        .join(" ");

    let (ctx, _) = one_scene("past-cap", &text);
    search_management_commands::run_search(&ctx, &search(&ctx, "Anna")).expect("run_search");
    let row = result_ids(&ctx)[0];

    // What a caller can actually enumerate: the row stops at the cap, and says so.
    let listed = offsets(&ctx, row);
    assert_eq!(
        listed.len(),
        500,
        "the row lists at most OCCURRENCE_CAP hits, which is the whole premise"
    );

    // Named directly: one hit, whatever the field holds.
    let named = search_management_commands::replace_in_project(
        &ctx,
        Some(undo_redo_commands::create_new_stack(&ctx)),
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Bérénice".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![row],
            only_occurrence_starts: vec![listed[0]],
        },
    )
    .expect("replace_in_project");
    assert_eq!(
        named.occurrences_replaced, 1,
        "naming the hit is immune to how many the field holds"
    );

    // And the shape that was wrong, on a fresh copy: the very same intent sent as
    // "skip the 499 others I can see" rewrites every hit past the cap as well.
    let (ctx, _) = one_scene("past-cap-excl", &text);
    search_management_commands::run_search(&ctx, &search(&ctx, "Anna")).expect("run_search");
    let row = result_ids(&ctx)[0];
    let listed = offsets(&ctx, row);
    let others: Vec<i64> = listed.iter().skip(1).copied().collect();
    let excluded = search_management_commands::replace_in_project(
        &ctx,
        Some(undo_redo_commands::create_new_stack(&ctx)),
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Bérénice".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![row; others.len()],
            excluded_occurrence_starts: others,
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");
    assert_eq!(
        excluded.occurrences_replaced,
        (HITS - 499) as u64,
        "an exclusion list can only name what it can see, and the rest are replaced \
         -- which is exactly why the caller must not express this as one"
    );
}

/// A dismissed hit stays dismissed: the rest of the field is rewritten around it.
#[test]
fn excluded_occurrences_are_left_alone() {
    let (ctx, content) = one_scene(
        "excluded",
        "Anna partit. Anna revint. Anna repartit. Anna resta.",
    );
    search_management_commands::run_search(&ctx, &search(&ctx, "Anna")).expect("run_search");
    let row = result_ids(&ctx)[0];
    let at = offsets(&ctx, row);

    let out = search_management_commands::replace_in_project(
        &ctx,
        Some(undo_redo_commands::create_new_stack(&ctx)),
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Bérénice".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![row, row],
            excluded_occurrence_starts: vec![at[1], at[3]],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert_eq!(out.occurrences_replaced, 2);
    assert_eq!(
        prose(&ctx, content),
        "Bérénice partit. Anna revint. Bérénice repartit. Anna resta.",
        "the two that were refused, and only those, are still there"
    );
}

/// **A row that survives its replace is corrected, not thrown away.**
///
/// The first version wiped the entire result set on every replace, which is how a
/// writer who rewrote one hit was handed an empty panel: the surface above cannot
/// tell "your search found nothing" from "your search was discarded", so the tree
/// they had open collapsed to nothing and their scroll position went with it.
#[test]
fn a_row_with_hits_left_is_restated_and_the_untouched_rows_stay() {
    let (ctx, _) = one_scene(
        "restated",
        "Anna partit. Anna revint. Anna repartit. Anna resta.",
    );
    search_management_commands::run_search(&ctx, &search(&ctx, "Anna")).expect("run_search");
    let row = result_ids(&ctx)[0];
    let at = offsets(&ctx, row);
    let before = search_result_commands::get_search_result(&ctx, &row)
        .unwrap()
        .unwrap();
    assert_eq!(before.occurrence_count, 4);

    search_management_commands::replace_in_project(
        &ctx,
        Some(undo_redo_commands::create_new_stack(&ctx)),
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Bérénice".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![row],
            only_occurrence_starts: vec![at[0]],
        },
    )
    .expect("replace_in_project");

    assert_eq!(
        result_ids(&ctx),
        vec![row],
        "the row is still there, with the id the panel is holding"
    );
    let after = search_result_commands::get_search_result(&ctx, &row)
        .unwrap()
        .unwrap();
    assert_eq!(
        after.occurrence_count, 3,
        "and it now says how many are actually left"
    );
    // The snippet was cut around the FIRST remaining hit, which is no longer the
    // one it was cut around before.
    assert!(
        after.snippet_before.contains("Bérénice"),
        "the snippet is re-cut from the prose as it now stands, got {:?}",
        after.snippet_before
    );

    // The occurrences the tree fetches next agree with the count above.
    assert_eq!(offsets(&ctx, row).len(), 3);
}

/// A field with nothing of the query left in it loses its row, and only its row.
#[test]
fn a_row_with_nothing_left_is_dropped_while_its_neighbours_stay() {
    let ctx = loaded_ctx();
    search_management_commands::run_search(&ctx, &search(&ctx, "ipsum")).expect("run_search");
    let rows = result_ids(&ctx);
    assert!(rows.len() >= 2, "need a neighbour to leave alone");
    let target = rows[0];
    let neighbours: Vec<u64> = rows.iter().skip(1).copied().collect();

    search_management_commands::replace_in_project(
        &ctx,
        Some(undo_redo_commands::create_new_stack(&ctx)),
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "ZZZ".to_string(),
            preserve_case: false,
            excluded_result_ids: neighbours.clone(),
            excluded_occurrence_rows: vec![],
            excluded_occurrence_starts: vec![],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    let left = result_ids(&ctx);
    assert!(
        !left.contains(&target),
        "the rewritten field has nothing to review any more"
    );
    for id in &neighbours {
        assert!(
            left.contains(id),
            "a row this replace never looked at must survive it untouched"
        );
    }
}

/// An offset that lands on no hit refuses the **whole row** rather than guessing.
///
/// The tempting reading is "that hit is gone, so there is nothing to skip" -- and
/// under it a stale exclusion turns into a replacement of a hit the writer refused.
/// Nothing distinguishes a hit that vanished from a hit that moved, so the honest
/// answer is that the list no longer says which hits it meant.
#[test]
fn an_offset_naming_no_hit_refuses_the_row() {
    let (ctx, content) = one_scene("stale-offset", "Anna partit. Anna revint. Anna resta.");
    search_management_commands::run_search(&ctx, &search(&ctx, "Anna")).expect("run_search");
    let row = result_ids(&ctx)[0];
    let before = prose(&ctx, content);

    let out = search_management_commands::replace_in_project(
        &ctx,
        Some(undo_redo_commands::create_new_stack(&ctx)),
        &ReplaceInProjectDto {
            work_id: work_id(&ctx),
            replacement: "Bérénice".to_string(),
            preserve_case: false,
            excluded_result_ids: vec![],
            excluded_occurrence_rows: vec![row],
            // Between two real hits, on no hit at all.
            excluded_occurrence_starts: vec![7],
            only_occurrence_rows: vec![],
            only_occurrence_starts: vec![],
        },
    )
    .expect("replace_in_project");

    assert_eq!(out.occurrences_replaced, 0);
    assert_eq!(out.skipped_stale, vec![row], "refused, and said so");
    assert_eq!(prose(&ctx, content), before, "and wrote nothing");
}
