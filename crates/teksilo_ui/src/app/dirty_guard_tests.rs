// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use frontend::commands::{
    comment_reply_commands, footnote_commands, note_template_commands, smart_punctuation_commands,
};
use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason, QuoteStyle};
use frontend::direct_access::{
    CommentRelationshipDto, CreateCommentDto, CreateCommentReplyDto, CreateFootnoteDto,
    CreateNoteTemplateDto, CreateSmartPunctuationDto, CreateWorkDto, WorkRelationshipDto,
};

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

/// A Work with one comment (with one reply), one footnote and one note
/// template, wired exactly as the live models wire them.
fn work_with_annotations(ctx: &AppContext) -> (u64, u64, u64, u64, u64) {
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

    // A template is the fourth kind edited entirely outside the manuscript
    // editors, and the fourth to need its own attribution arm: without one the
    // `_ => true` fallback would let a preset applied in one project dirty every
    // other open project.
    let note_template = note_template_commands::create_orphan_note_template(
        ctx,
        None,
        &CreateNoteTemplateDto {
            uid: Default::default(),
            created_at: now(),
            updated_at: now(),
            name: "Character sheet".into(),
            body: "# Character sheet\n".into(),
            starred: false,
        },
    )
    .expect("create note template")
    .id;
    work_commands::set_work_relationship(
        ctx,
        None,
        &WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::NoteTemplates,
            right_ids: vec![note_template],
        },
    )
    .expect("wire note template onto work");

    (work, comment, reply, footnote, note_template)
}

/// The positive half: my own comment, reply, footnote and note-template
/// events all attribute to my Work — through the direct read for all but the
/// reply, and the two-hop walk for that one.
#[test]
fn my_own_annotation_events_belong_to_my_work() {
    let ctx = AppContext::new();
    let (work, comment, reply, footnote, note_template) = work_with_annotations(&ctx);

    use DirectAccessEntity::{Comment, CommentReply, Footnote, NoteTemplate};
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
    assert!(mutation_ids_belong_to_work(
        &ctx,
        work,
        NoteTemplate(EntityEvent::Updated),
        &[note_template]
    ));
}

/// The guarding half: a sibling Work's events must not mark mine dirty —
/// the exact multi-window scenario the guard exists for (a mutation in
/// window B's project re-arming window A's autosave).
#[test]
fn a_sibling_works_annotation_events_do_not_belong_to_mine() {
    let ctx = AppContext::new();
    let (mine, ..) = work_with_annotations(&ctx);
    let (_, their_comment, their_reply, their_footnote, their_template) =
        work_with_annotations(&ctx);

    use DirectAccessEntity::{Comment, CommentReply, Footnote, NoteTemplate};
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
    assert!(!mutation_ids_belong_to_work(
        &ctx,
        mine,
        NoteTemplate(EntityEvent::Updated),
        &[their_template]
    ));
}

/// The six kinds edited from panes and dialogs rather than a manuscript editor,
/// wired the way the live models wire them: a pace under the Work with a
/// milestone and a holiday under it, a text replacement rule, an image row, and
/// the Work's own punctuation settings.
struct Planning {
    work: u64,
    pace: u64,
    milestone: u64,
    holiday: u64,
    rule: u64,
    asset: u64,
    smart_punctuation: u64,
}

fn work_with_planning(ctx: &AppContext) -> Planning {
    use frontend::commands::{
        asset_commands, holiday_commands, milestone_commands, pace_commands,
        text_replacement_rule_commands,
    };
    use frontend::common::entities::MilestoneKind;
    use frontend::direct_access::{
        CreateAssetDto, CreateHolidayDto, CreateMilestoneDto, CreatePaceDto,
        CreateTextReplacementRuleDto,
    };
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
            title: "P".into(),
            smart_punctuation,
            ..Default::default()
        },
    )
    .expect("create work")
    .id;
    let pace = pace_commands::create_pace(
        ctx,
        None,
        &CreatePaceDto {
            created_at: now(),
            updated_at: now(),
            book_item: None,
            start_date: now(),
            end_date: now(),
            weekday_mask: 31,
            active: true,
            holidays: Vec::new(),
            milestones: Vec::new(),
        },
        work,
        -1,
    )
    .expect("create pace")
    .id;
    let milestone = milestone_commands::create_milestone(
        ctx,
        None,
        &CreateMilestoneDto {
            created_at: now(),
            updated_at: now(),
            label: "Halfway".into(),
            target_item: None,
            target_date: now(),
            target_word_count: Some(40_000),
            kind: MilestoneKind::BookCumulative,
        },
        pace,
        -1,
    )
    .expect("create milestone")
    .id;
    let holiday = holiday_commands::create_holiday(
        ctx,
        None,
        &CreateHolidayDto {
            created_at: now(),
            updated_at: now(),
            label: "August".into(),
            start_date: now(),
            end_date: None,
        },
        pace,
        -1,
    )
    .expect("create holiday")
    .id;
    let rule = text_replacement_rule_commands::create_text_replacement_rule(
        ctx,
        None,
        &CreateTextReplacementRuleDto {
            created_at: now(),
            updated_at: now(),
            trigger: "->".into(),
            replacement: "→".into(),
            enabled: true,
        },
        work,
        -1,
    )
    .expect("create rule")
    .id;
    let asset = asset_commands::create_asset(
        ctx,
        None,
        &CreateAssetDto {
            created_at: now(),
            updated_at: now(),
            content_hash: "abc123".into(),
            file_name: "cover.png".into(),
            mime_type: "image/png".into(),
            width: 1,
            height: 1,
            byte_size: 1,
            alt: String::new(),
            is_cover: true,
        },
        work,
        -1,
    )
    .expect("create asset")
    .id;
    Planning {
        work,
        pace,
        milestone,
        holiday,
        rule,
        asset,
        smart_punctuation,
    }
}

/// Every one of the six attributes to its own Work: the four direct children
/// through one relationship read, the two grandchildren through the paces.
#[test]
fn my_own_planning_and_settings_events_belong_to_my_work() {
    let ctx = AppContext::new();
    let p = work_with_planning(&ctx);
    use DirectAccessEntity::{
        Asset, Holiday, Milestone, Pace, SmartPunctuation, TextReplacementRule,
    };
    let mine = |entity, ids: &[u64]| mutation_ids_belong_to_work(&ctx, p.work, entity, ids);
    assert!(mine(Pace(EntityEvent::Updated), &[p.pace]));
    assert!(mine(Milestone(EntityEvent::Updated), &[p.milestone]));
    assert!(mine(Holiday(EntityEvent::Updated), &[p.holiday]));
    assert!(mine(TextReplacementRule(EntityEvent::Updated), &[p.rule]));
    assert!(mine(Asset(EntityEvent::Updated), &[p.asset]));
    assert!(mine(
        SmartPunctuation(EntityEvent::Updated),
        &[p.smart_punctuation]
    ));
}

/// …and none of a sibling Work's do — the multi-window scenario every arm
/// exists for, where the `_ => true` fallback would have marked every open
/// project dirty for one project's pace edit.
#[test]
fn a_sibling_works_planning_and_settings_events_do_not_belong_to_mine() {
    let ctx = AppContext::new();
    let mine = work_with_planning(&ctx);
    let theirs = work_with_planning(&ctx);
    use DirectAccessEntity::{
        Asset, Holiday, Milestone, Pace, SmartPunctuation, TextReplacementRule,
    };
    let is_mine = |entity, ids: &[u64]| mutation_ids_belong_to_work(&ctx, mine.work, entity, ids);
    assert!(!is_mine(Pace(EntityEvent::Updated), &[theirs.pace]));
    assert!(!is_mine(
        Milestone(EntityEvent::Updated),
        &[theirs.milestone]
    ));
    assert!(!is_mine(Holiday(EntityEvent::Updated), &[theirs.holiday]));
    assert!(!is_mine(
        TextReplacementRule(EntityEvent::Updated),
        &[theirs.rule]
    ));
    assert!(!is_mine(Asset(EntityEvent::Updated), &[theirs.asset]));
    assert!(!is_mine(
        SmartPunctuation(EntityEvent::Updated),
        &[theirs.smart_punctuation]
    ));
}
