// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use frontend::commands::{comment_reply_commands, footnote_commands, smart_punctuation_commands};
use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason, QuoteStyle};
use frontend::direct_access::{
    CommentRelationshipDto, CreateCommentDto, CreateCommentReplyDto, CreateFootnoteDto,
    CreateSmartPunctuationDto, CreateWorkDto, WorkRelationshipDto,
};

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

/// A Work with one comment (with one reply) and one footnote, wired exactly
/// as the live models wire them.
fn work_with_annotations(ctx: &AppContext) -> (u64, u64, u64, u64) {
    // Each Work owns exactly one SmartPunctuation (one_to_one, strong), so
    // the `0` placeholder a defaulted DTO carries would collide on this
    // helper's second call under the generated uniqueness check — the same
    // note `frontend/tests/multi_work_scoping_test.rs` records.
    let smart_punctuation = smart_punctuation_commands::create_orphan_smart_punctuation(
        ctx,
        None,
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
    let work = work_commands::create_orphan_work(
        ctx,
        None,
        &CreateWorkDto {
            statuses: Vec::new(),
            created_at: now(),
            updated_at: now(),
            title: "W".into(),
            smart_punctuation,
            ..Default::default()
        },
    )
    .expect("create work")
    .id;

    let comment = comment_commands::create_orphan_comment(
        ctx,
        None,
        &CreateCommentDto {
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            content: None,
            kind: CommentAnchorKind::Range,
            author_name: "Jane".into(),
            author_initials: "J".into(),
            body: "note".into(),
            resolved: false,
            orphaned: false,
            orphan_reason: CommentOrphanReason::NotOrphaned,
            range_start: 0,
            range_length: 4,
            quote_prefix: String::new(),
            quote_exact: "lamp".into(),
            quote_exact_truncated: false,
            quote_suffix: String::new(),
            block_ordinal_hint: 0,
            replies: vec![],
        },
    )
    .expect("create comment")
    .id;
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::Comments,
            right_ids: vec![comment],
        },
    )
    .expect("wire comment onto work");

    let reply = comment_reply_commands::create_orphan_comment_reply(
        ctx,
        None,
        &CreateCommentReplyDto {
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            author_name: "Marc".into(),
            author_initials: "M".into(),
            body: "keep it".into(),
        },
    )
    .expect("create reply")
    .id;
    comment_commands::set_comment_relationship(
        ctx,
        None,
        &CommentRelationshipDto {
            id: comment,
            field: CommentRelationshipField::Replies,
            right_ids: vec![reply],
        },
    )
    .expect("wire reply onto comment");

    let footnote = footnote_commands::create_orphan_footnote(
        ctx,
        None,
        &CreateFootnoteDto {
            created_at: now(),
            updated_at: now(),
            uid: Default::default(),
            content: None,
            label: "fn1".into(),
            body: "a note".into(),
        },
    )
    .expect("create footnote")
    .id;
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::Footnotes,
            right_ids: vec![footnote],
        },
    )
    .expect("wire footnote onto work");

    (work, comment, reply, footnote)
}

/// The positive half: my own comment, reply and footnote events all
/// attribute to my Work — through the direct read for the first and third,
/// and the two-hop walk for the reply.
#[test]
fn my_own_annotation_events_belong_to_my_work() {
    let ctx = AppContext::new();
    let (work, comment, reply, footnote) = work_with_annotations(&ctx);

    use DirectAccessEntity::{Comment, CommentReply, Footnote};
    assert!(mutation_ids_belong_to_work(
        &ctx,
        work,
        Comment(EntityEvent::Updated),
        &[comment]
    ));
    assert!(mutation_ids_belong_to_work(
        &ctx,
        work,
        CommentReply(EntityEvent::Updated),
        &[reply]
    ));
    assert!(mutation_ids_belong_to_work(
        &ctx,
        work,
        Footnote(EntityEvent::Updated),
        &[footnote]
    ));
}

/// The guarding half: a sibling Work's events must not mark mine dirty —
/// the exact multi-window scenario the guard exists for (a mutation in
/// window B's project re-arming window A's autosave).
#[test]
fn a_sibling_works_annotation_events_do_not_belong_to_mine() {
    let ctx = AppContext::new();
    let (mine, ..) = work_with_annotations(&ctx);
    let (_, their_comment, their_reply, their_footnote) = work_with_annotations(&ctx);

    use DirectAccessEntity::{Comment, CommentReply, Footnote};
    assert!(!mutation_ids_belong_to_work(
        &ctx,
        mine,
        Comment(EntityEvent::Updated),
        &[their_comment]
    ));
    assert!(!mutation_ids_belong_to_work(
        &ctx,
        mine,
        CommentReply(EntityEvent::Updated),
        &[their_reply]
    ));
    assert!(!mutation_ids_belong_to_work(
        &ctx,
        mine,
        Footnote(EntityEvent::Updated),
        &[their_footnote]
    ));
}
