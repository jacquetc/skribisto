// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! M1 acceptance: the `Comment` / `CommentReply` entities exist, carry their anchor
//! payload, thread their replies in order, survive undo/redo, and — the property the
//! whole feature is built on — **orphan rather than vanish** when the prose they were
//! anchored to is deleted.
//!
//! There is deliberately no `comment_management` feature crate and no hand-written CRUD
//! here. Everything below goes through the *generated* per-entity commands, which are
//! already undoable via `stack_id`. That is the same lesson `tag_management_test.rs`
//! records at the top of its own file: reach for a use case only when generic CRUD
//! genuinely cannot express the operation (batching N writes into one undo entry), not
//! by reflex.
//!
//! The orphan test is the one that matters. `Comment.content` is a weak, optional
//! `many_to_one` — the same shape as `TrashInfo.trashed_binder_item` and
//! `Milestone.target_item` — precisely so that removing the anchored `Content` leaves
//! the comment standing with a dangling reference we can *render* ("this comment lost
//! its text") instead of silently destroying the writer's note. A strong relationship
//! would cascade-delete it, which is the Google-Docs/Confluence failure mode this
//! design exists to avoid.

use frontend::AppContext;
use frontend::commands::{
    comment_commands, comment_reply_commands, content_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::comment::CommentRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason, ContentRole};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    CommentRelationshipDto, CreateCommentDto, CreateCommentReplyDto, CreateContentDto,
    CreateWorkDto, UpdateCommentDto, WorkRelationshipDto,
};

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

struct Fixture {
    ctx: AppContext,
    setup: u64,
    work: EntityId,
    content: EntityId,
}

/// A Work with one prose `Content` row, built on a dedicated undo stack so the arrange
/// phase never pollutes the stack the action under test runs on.
fn make_fixture() -> Fixture {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);

    let work = work_commands::create_orphan_work(
        &ctx,
        Some(setup),
        &CreateWorkDto {
            created_at: now(),
            updated_at: now(),
            title: "The Lighthouse".into(),
            ..Default::default()
        },
    )
    .expect("create work")
    .id;

    let content = content_commands::create_orphan_content(
        &ctx,
        Some(setup),
        &CreateContentDto {
            created_at: now(),
            updated_at: now(),
            activated: true,
            role: ContentRole::SceneText,
            data: "The lamp guttered, and then it did not.".into(),
        },
    )
    .expect("create content")
    .id;

    Fixture {
        ctx,
        setup,
        work,
        content,
    }
}

/// A range comment anchored to the fixture's prose, wired onto the Work.
///
/// Every anchor field is given a **non-default** value on purpose: an all-default row
/// round-trips equal even when a field has been dropped somewhere along the way, which
/// is exactly how a persistence bug hides (the same reasoning `skrib_format`'s own
/// fixtures record for `chapter_mode` and `smart_punctuation`).
fn mk_comment(fx: &Fixture, stack: u64, body: &str) -> EntityId {
    let id = comment_commands::create_orphan_comment(
        &fx.ctx,
        Some(stack),
        &CreateCommentDto {
            created_at: now(),
            updated_at: now(),
            content: Some(fx.content),
            kind: CommentAnchorKind::Range,
            author_name: "Jane".into(),
            body: body.into(),
            resolved: false,
            orphaned: false,
            orphan_reason: CommentOrphanReason::NotOrphaned,
            range_start: 4,
            range_length: 4,
            quote_prefix: "The ".into(),
            quote_exact: "lamp".into(),
            quote_exact_truncated: false,
            quote_suffix: " guttered".into(),
            block_ordinal_hint: 0,
            replies: vec![],
        },
    )
    .expect("create comment")
    .id;

    let mut ids = work_comments(fx);
    ids.push(id);
    work_commands::set_work_relationship(
        &fx.ctx,
        Some(stack),
        &WorkRelationshipDto {
            id: fx.work,
            field: WorkRelationshipField::Comments,
            right_ids: ids,
        },
    )
    .expect("wire comment onto work");
    id
}

fn work_comments(fx: &Fixture) -> Vec<EntityId> {
    work_commands::get_work_relationship(
        &fx.ctx,
        &fx.work,
        &WorkRelationshipField::Comments,
    )
    .expect("read work comments")
}

#[test]
fn a_comment_round_trips_its_anchor_payload() {
    let fx = make_fixture();
    let id = mk_comment(&fx, fx.setup, "Is this too on-the-nose?");

    let got = comment_commands::get_comment(&fx.ctx, &id)
        .expect("get comment")
        .expect("comment exists");

    assert_eq!(got.body, "Is this too on-the-nose?");
    assert_eq!(got.author_name, "Jane");
    assert_eq!(got.content, Some(fx.content));
    assert_eq!(got.kind, CommentAnchorKind::Range);
    assert!(!got.resolved);
    assert!(!got.orphaned);
    assert_eq!(got.orphan_reason, CommentOrphanReason::NotOrphaned);

    // The anchor itself: a quote selector plus a position hint, in document-absolute
    // CHARACTER offsets. Asserted field by field because every one of them is load
    // bearing at re-anchor time.
    assert_eq!(got.range_start, 4);
    assert_eq!(got.range_length, 4);
    assert_eq!(got.quote_prefix, "The ");
    assert_eq!(got.quote_exact, "lamp");
    assert!(!got.quote_exact_truncated);
    assert_eq!(got.quote_suffix, " guttered");
    assert_eq!(got.block_ordinal_hint, 0);

    assert_eq!(work_comments(&fx), vec![id]);
}

#[test]
fn creating_a_comment_is_undoable_and_redoable() {
    let fx = make_fixture();
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let id = mk_comment(&fx, stack, "Tighten this.");
    assert!(
        comment_commands::get_comment(&fx.ctx, &id)
            .expect("get")
            .is_some()
    );

    // Two pushes landed on this stack (the create, then the relationship wiring), so
    // unwinding the create takes two undos. Asserting the intermediate state as well
    // pins that the relationship undo is a real, separate entry rather than a no-op.
    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo wiring");
    assert!(
        work_comments(&fx).is_empty(),
        "undoing the wiring should detach the comment from the Work"
    );

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo create");
    assert!(
        comment_commands::get_comment(&fx.ctx, &id)
            .expect("get")
            .is_none(),
        "undoing the create should remove the comment row"
    );

    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo create");
    undo_redo_commands::redo(&fx.ctx, Some(stack)).expect("redo wiring");
    let back = comment_commands::get_comment(&fx.ctx, &id)
        .expect("get")
        .expect("comment is back after redo");
    assert_eq!(back.body, "Tighten this.");
    assert_eq!(back.quote_exact, "lamp", "redo must restore the anchor too");
}

#[test]
fn resolving_a_comment_is_undoable() {
    let fx = make_fixture();
    let id = mk_comment(&fx, fx.setup, "Cut?");
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);

    let cur = comment_commands::get_comment(&fx.ctx, &id)
        .expect("get")
        .expect("exists");
    // `UpdateCommentDto` deliberately carries no relationship fields — resolving a
    // comment must not be able to silently re-point its anchor as a side effect.
    comment_commands::update_comment(
        &fx.ctx,
        Some(stack),
        &UpdateCommentDto {
            id: cur.id,
            created_at: cur.created_at,
            updated_at: now(),
            kind: cur.kind,
            author_name: cur.author_name,
            body: cur.body,
            resolved: true,
            orphaned: cur.orphaned,
            orphan_reason: cur.orphan_reason,
            range_start: cur.range_start,
            range_length: cur.range_length,
            quote_prefix: cur.quote_prefix,
            quote_exact: cur.quote_exact,
            quote_exact_truncated: cur.quote_exact_truncated,
            quote_suffix: cur.quote_suffix,
            block_ordinal_hint: cur.block_ordinal_hint,
        },
    )
    .expect("resolve");

    assert!(
        comment_commands::get_comment(&fx.ctx, &id)
            .expect("get")
            .expect("exists")
            .resolved
    );

    undo_redo_commands::undo(&fx.ctx, Some(stack)).expect("undo resolve");
    assert!(
        !comment_commands::get_comment(&fx.ctx, &id)
            .expect("get")
            .expect("exists")
            .resolved,
        "reopening via undo must restore the unresolved state"
    );
}

#[test]
fn replies_thread_in_order() {
    let fx = make_fixture();
    let comment = mk_comment(&fx, fx.setup, "Too on-the-nose?");

    let mut reply_ids = Vec::new();
    for body in ["Maybe.", "No — keep it.", "Agreed, keeping."] {
        let r = comment_reply_commands::create_orphan_comment_reply(
            &fx.ctx,
            Some(fx.setup),
            &CreateCommentReplyDto {
                created_at: now(),
                updated_at: now(),
                author_name: "Jane".into(),
                body: body.into(),
            },
        )
        .expect("create reply")
        .id;
        reply_ids.push(r);
    }

    comment_commands::set_comment_relationship(
        &fx.ctx,
        Some(fx.setup),
        &CommentRelationshipDto {
            id: comment,
            field: CommentRelationshipField::Replies,
            right_ids: reply_ids.clone(),
        },
    )
    .expect("wire replies");

    let got = comment_commands::get_comment_relationship(
        &fx.ctx,
        &comment,
        &CommentRelationshipField::Replies,
    )
    .expect("read replies");

    assert_eq!(
        got, reply_ids,
        "replies are an ordered_one_to_many: insertion order is the thread's chronology"
    );

    let bodies: Vec<String> = comment_reply_commands::get_comment_reply_multi(&fx.ctx, &got)
        .expect("get replies")
        .into_iter()
        .map(|r| r.expect("reply exists").body)
        .collect();
    assert_eq!(bodies, ["Maybe.", "No — keep it.", "Agreed, keeping."]);
}

#[test]
fn deleting_the_anchored_content_orphans_the_comment_instead_of_destroying_it() {
    let fx = make_fixture();
    let id = mk_comment(&fx, fx.setup, "This line is the whole scene.");

    // The writer deletes the scene's prose row outright.
    content_commands::remove_content(&fx.ctx, Some(fx.setup), &fx.content).expect("remove content");

    // The comment must still exist. This is the entire reason `Comment.content` is a
    // weak `many_to_one` and not a strong parent link.
    let survivor = comment_commands::get_comment(&fx.ctx, &id)
        .expect("get comment")
        .expect("a comment must OUTLIVE the content it was anchored to");
    assert_eq!(survivor.body, "This line is the whole scene.");

    // ...and it is still reachable from the Work, so the docks can list it and offer
    // "relink" or "delete". An orphan the user cannot see is the Confluence bug.
    assert_eq!(work_comments(&fx), vec![id]);

    // The relationship itself is now empty, which is precisely the signal the re-anchor
    // pass turns into `orphan_reason: TargetDeleted`.
    let still_pointing = comment_commands::get_comment_relationship(
        &fx.ctx,
        &id,
        &CommentRelationshipField::Content,
    )
    .expect("read content relationship");
    assert!(
        still_pointing.is_empty(),
        "the weak reference should resolve to nothing once the target is gone, \
         rather than keeping a live id that no longer exists"
    );
}

#[test]
fn removing_the_work_cascades_to_its_comments() {
    let fx = make_fixture();
    let id = mk_comment(&fx, fx.setup, "Doomed along with its Work.");

    // `Work.comments` is a STRONG one_to_many, unlike `Comment.content` — closing the
    // book really does take its marginalia with it.
    work_commands::remove_work(&fx.ctx, Some(fx.setup), &fx.work).expect("remove work");

    assert!(
        comment_commands::get_comment(&fx.ctx, &id)
            .expect("get")
            .is_none(),
        "a strong parent must cascade-remove its comments"
    );
}

// ---------------------------------------------------------------------------
// M8 — what the destructive item operations do to a comment
// ---------------------------------------------------------------------------
//
// These are an *audit*, not a feature: they pin the behaviour that falls out of
// the sidecar design, so a later change to duplicate/purge cannot silently alter
// it. Each one states the property in its name.

/// Duplicating an item must NOT duplicate its comments.
///
/// This is the property that the in-band-marker designs could not have: they
/// store the anchor inside `Content.data`, and `duplicate_uc` copies that string
/// verbatim (`data: c.data.clone()`), so the copy would carry live markers and
/// two scenes would claim the same threads. Anchoring in a sidecar makes the
/// correct behaviour the *default* rather than something a strip-and-re-mint step
/// has to remember to do.
#[test]
fn duplicating_a_content_row_does_not_carry_its_comments_along() {
    let fx = make_fixture();
    let original = mk_comment(&fx, fx.setup, "Only on the original.");

    // Duplicate the Content row the way `duplicate_uc` does — a verbatim copy of
    // the data string, with no reference to the comment store at all.
    let source = content_commands::get_content(&fx.ctx, &fx.content)
        .expect("read source")
        .expect("source exists")
        .data;
    let copy = content_commands::create_orphan_content(
        &fx.ctx,
        Some(fx.setup),
        &CreateContentDto {
            created_at: now(),
            updated_at: now(),
            activated: true,
            role: ContentRole::SceneText,
            data: source,
        },
    )
    .expect("duplicate content")
    .id;

    // The copy has no comments; the original keeps its one.
    let on_copy = comment_commands::get_all_comment(&fx.ctx)
        .expect("all comments")
        .into_iter()
        .filter(|c| {
            comment_commands::get_comment_relationship(
                &fx.ctx,
                &c.id,
                &CommentRelationshipField::Content,
            )
            .unwrap_or_default()
            .contains(&copy)
        })
        .count();
    assert_eq!(on_copy, 0, "a duplicated scene must start unannotated");

    let still = comment_commands::get_comment_relationship(
        &fx.ctx,
        &original,
        &CommentRelationshipField::Content,
    )
    .expect("read anchor");
    assert_eq!(
        still,
        vec![fx.content],
        "and the original's own thread must not have moved"
    );
}

/// Purging a trashed item — which removes its `Content` rows outright — leaves the
/// comment standing and merely anchorless.
///
/// `purge::apply_purge` calls `remove_content_multi`, so this is the same code
/// path the trash's "Delete Forever" and "Empty Trash" both end in. The comment
/// surviving is what puts it in the docks' "no home" bucket instead of destroying
/// a note the writer never chose to delete.
#[test]
fn purging_the_anchored_content_leaves_a_disposable_orphan() {
    let fx = make_fixture();
    let id = mk_comment(&fx, fx.setup, "Survives the purge.");

    content_commands::remove_content_multi(&fx.ctx, Some(fx.setup), &[fx.content])
        .expect("purge content");

    let survivor = comment_commands::get_comment(&fx.ctx, &id)
        .expect("get")
        .expect("the comment outlives the purge");
    assert_eq!(survivor.body, "Survives the purge.");
    assert!(
        comment_commands::get_comment_relationship(
            &fx.ctx,
            &id,
            &CommentRelationshipField::Content,
        )
        .expect("read anchor")
        .is_empty(),
        "its anchor resolves to nothing, which is what marks it orphaned"
    );

    // And it is still deletable — the whole point. A comment the UI will neither
    // reopen nor remove is the Confluence bug this design exists to avoid.
    comment_commands::remove_comment(&fx.ctx, Some(fx.setup), &id).expect("delete orphan");
    assert!(
        comment_commands::get_comment(&fx.ctx, &id)
            .expect("get")
            .is_none()
    );
}
