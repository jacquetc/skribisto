// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation (hand-maintained — do NOT blanket-regenerate): the write
// half of document import. It takes the plan the writer reviewed and accepted and
// creates it, as one undo entry.
//
// Synchronous, and deliberately so. Qleany drops `undoable` under
// `long_operation`, and the expensive part — reading and converting files —
// already happened in `analyze_document_import`. Rows arrive with their Djot
// already built, so this opens a transaction, creates, splices and commits
// without parsing anything.
//
// Undo is `snapshot_binder` / `restore_binder`, the same pair `duplicate_uc`
// uses. It costs a walk of the whole destination binder rather than of what was
// imported; that is the accepted trade for reusing a proven primitive instead of
// building a new one.
//
// Three traps this has to step around, none of which the backend will catch:
//
//  1. `is_exportable` and `activated` both default to `false`, because their DTO
//     derives `Default` and `bool::default()` is false. A row created without
//     setting them is present in the binder and invisible to export and to
//     chapter numbering — with no gap left behind, since chapters renumber to
//     fill it. They are set explicitly on every row below.
//  2. `validate_item` is called only by the UI. Nothing at this layer rejects an
//     illegal `(role, sub_role)`, so this checks the combination itself.
//  3. `content_allowed` is enforced only at save time, where it *drops the row
//     silently*. Prose that its type cannot hold is refused here, while the
//     writer can still see why.
//
// Imported comments are created here, in the same transaction and therefore the
// same undo entry as the rows they annotate. That is not a convenience: a comment
// belongs to a passage, and an undo that took back the passage while leaving the
// note behind would leave a comment pointing into a manuscript that no longer has
// the sentence it was about.
//
// Their anchors are copied across field for field and never reinterpreted. The
// plan already proved every quote against the very Djot being stored here (see
// `document_ingest::plan::anchor_comments`), so second-guessing it at this layer
// could only make it worse.
//
// The `#[macros::uow_action(...)]` list below is hand-trimmed to what this
// actually does, and must stay in lockstep with the identical list in
// ../units_of_work/apply_document_import_uow.rs.

use crate::ApplyDocumentImportDto;
use crate::ApplyDocumentImportResultDto;
use crate::dtos::{
    ApplyImportRow, ApplyImportRows, DropPosition, ImportComment, ImportCommentKind,
    ImportOrphanReason, ImportReply,
};
use crate::kind_mapping::kind_to_create_type;
use anyhow::{Result, anyhow};
use binder_ordering::{DropPlace, anchor_for_binder_target, insert_block, resolve_item_target};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::comment::CommentRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{
    BinderItem, Comment, CommentAnchorKind, CommentOrphanReason, CommentReply, Content,
    ContentRole, Work,
};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use skribisto_model::{allowed_content, content_allowed, is_valid_combination};

pub trait ApplyDocumentImportUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ApplyDocumentImportUnitOfWorkTrait>;
}

// `Work` for `chapter_mode` (how a Chapter is encoded, with no per-item
// override), and for its `comments` collection, which is where an imported comment
// hangs. `Binder` to read the destination order, splice into it, and snapshot it
// for undo. `BinderItem` + `Content` to create the rows themselves. `Comment` +
// `CommentReply` for the editor's notes that came with them.
//
// The undo snapshot is still `Binder`-scoped, and that is deliberate rather than an
// oversight: `Comment` hangs off `Work`, so a comment created here is *not* inside
// the snapshot. `undo` therefore removes it from `Work.comments` explicitly — see
// the note there.
#[macros::uow_action(entity = "Work", action = "Get")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
#[macros::uow_action(entity = "Comment", action = "CreateOrphan")]
#[macros::uow_action(entity = "Comment", action = "SetRelationship")]
#[macros::uow_action(entity = "CommentReply", action = "CreateOrphan")]
pub trait ApplyDocumentImportUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_apply_document_import_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct ApplyDocumentImportUseCase {
    uow_factory: Box<dyn ApplyDocumentImportUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
    /// The `Work` the import landed in, and the comments it created there.
    ///
    /// Kept because `Comment` hangs off `Work` and the undo snapshot is
    /// `Binder`-scoped, so restoring the binder does not unlink them. Undo detaches
    /// these ids by hand; redo puts them back.
    work_id: EntityId,
    created_comment_ids: Vec<EntityId>,
}

impl ApplyDocumentImportUseCase {
    pub fn new(uow_factory: Box<dyn ApplyDocumentImportUnitOfWorkFactoryTrait>) -> Self {
        ApplyDocumentImportUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
            work_id: 0,
            created_comment_ids: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        dto: &ApplyDocumentImportDto,
    ) -> Result<ApplyDocumentImportResultDto> {
        let ApplyImportRows::Create(rows) = &dto.rows else {
            return Err(anyhow!("apply_document_import: no rows to create"));
        };
        let rows: Vec<&ApplyImportRow> = rows
            .iter()
            .filter(|r| !matches!(r, ApplyImportRow::Empty))
            .collect();
        if rows.is_empty() {
            return Err(anyhow!("apply_document_import: no rows to create"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let work = uow
            .get_work(&dto.work_id)?
            .ok_or_else(|| anyhow!("apply_document_import: work {} not found", dto.work_id))?;
        let chapter_mode = work.chapter_mode.clone();

        let snap_before = uow.snapshot_binder(&[dto.binder_id])?;
        let order =
            uow.get_binder_relationship(&dto.binder_id, &BinderRelationshipField::BinderItems)?;

        // Everything the import creates gets one timestamp, the way every other
        // creation path in this codebase does — a row's `created_at` records the
        // import, not how long the parsing took.
        // Where the block actually goes, resolved here rather than trusted from the
        // plan: the binder can move between the writer reviewing an import and
        // accepting it, and the plan's own indent was only ever a preview.
        let (base_indent, insertion_anchor) = resolve_destination(uow.as_mut(), dto, &order)?;
        // The plan's rows sit at whatever depth the preview assumed. Shift the whole
        // block so its shallowest row lands at the depth just resolved — one offset for
        // all of them, so the tree the writer reviewed keeps its shape.
        let shift = indent_shift(&rows, base_indent);

        let now = chrono::Utc::now();
        let mut created_ids: Vec<EntityId> = Vec::with_capacity(rows.len());

        let mut created_comment_ids: Vec<EntityId> = Vec::new();

        for row in &rows {
            let ApplyImportRow::Create {
                indent,
                kind,
                title,
                djot,
                comments,
            } = row
            else {
                continue;
            };

            let (role, sub_role) = kind_to_create_type(kind).combo(chapter_mode.clone());

            // Trap 2: nothing below this layer checks the combination.
            if !is_valid_combination(&role, &sub_role) {
                return Err(anyhow!(
                    "apply_document_import: '{title}' resolves to an invalid {role:?}/{sub_role:?}"
                ));
            }

            // Trap 3: prose whose row cannot hold it would be dropped silently at
            // the next save. Refuse it here, where the writer can still be told.
            let content_id = if djot.trim().is_empty() {
                None
            } else {
                let content_role = prose_role_for(&role, &sub_role).ok_or_else(|| {
                    anyhow!(
                        "apply_document_import: '{title}' is a {role:?}/{sub_role:?}, \
                         which holds no prose"
                    )
                })?;
                let created = uow.create_orphan_content(&Content {
                    created_at: now,
                    updated_at: now,
                    activated: true,
                    role: content_role,
                    data: djot.clone(),
                    ..Default::default()
                })?;
                Some(created.id)
            };

            let item = uow.create_orphan_binder_item(&BinderItem {
                // A created row mints its own durable identity. Everything
                // persisted about an item keys on this — including, since the
                // prose-filename fix, its own file on disk.
                uid: common::uid::new_uid(),
                created_at: now,
                updated_at: now,
                title: title.clone(),
                role,
                sub_role,
                // Trap 1: both default to false, and a row that is neither
                // exportable nor activated is present but invisible to export and
                // to chapter numbering.
                activated: true,
                is_exportable: true,
                indent: *indent + shift,
                ..Default::default()
            })?;

            if let Some(content_id) = content_id {
                uow.set_binder_item_relationship(
                    &item.id,
                    &BinderItemRelationshipField::Contents,
                    &[content_id],
                )?;
            }

            // A comment points into prose, so a row that stored none can hold none.
            // That is not a loss to hide: the analyse half only ever attaches
            // comments to rows whose prose it anchored them against, so reaching
            // here with comments and no content would mean the two halves disagree.
            if !comments.is_empty() {
                let Some(content_id) = content_id else {
                    return Err(anyhow!(
                        "apply_document_import: '{title}' carries {} comment(s) but no prose \
                         to anchor them to",
                        comments.len()
                    ));
                };
                for comment in comments {
                    created_comment_ids.push(create_comment(
                        uow.as_mut(),
                        comment,
                        content_id,
                        now,
                    )?);
                }
            }

            created_ids.push(item.id);
        }

        // Comments hang off the Work, not off the row they annotate — appended to
        // whatever is already there rather than replacing it, since an import is not
        // the only thing that ever made a comment in this project.
        if !created_comment_ids.is_empty() {
            let mut all =
                uow.get_work_relationship(&dto.work_id, &WorkRelationshipField::Comments)?;
            all.extend_from_slice(&created_comment_ids);
            uow.set_work_relationship(&dto.work_id, &WorkRelationshipField::Comments, &all)?;
        }

        // Splice the new block in as one run, before the resolved insertion anchor
        // (or at the end when there is none). Keeping it contiguous is what makes the
        // import one movable, one undoable thing rather than rows scattered through
        // the binder.
        let new_order = insert_block(&order, &created_ids, insertion_anchor);
        uow.set_binder_relationship(
            &dto.binder_id,
            &BinderRelationshipField::BinderItems,
            &new_order,
        )?;

        let snap_after = uow.snapshot_binder(&[dto.binder_id])?;
        uow.commit()?;
        uow.publish_apply_document_import_event(created_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        self.work_id = dto.work_id;
        self.created_comment_ids = created_comment_ids;
        Ok(ApplyDocumentImportResultDto { created_ids })
    }
}

impl ApplyDocumentImportUseCase {
    /// Add or remove this import's comments from `Work.comments`.
    ///
    /// `Comment` hangs off `Work`, and the undo snapshot is `Binder`-scoped — so
    /// restoring the binder puts the rows back exactly and leaves the comments
    /// dangling in the Work, attached to `Content` rows that no longer exist. That
    /// is the shape of the bug the trash feature's own notes warn about: an index
    /// that outlives what it indexes.
    ///
    /// Detaching by id rather than by snapshot is deliberate too. A `Work`-wide
    /// snapshot would take back comments the writer made *after* the import, which
    /// an undo of the import has no business touching.
    fn set_comments_attached(
        &self,
        uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
        attached: bool,
    ) -> Result<()> {
        if self.created_comment_ids.is_empty() || self.work_id == 0 {
            return Ok(());
        }
        let current = uow.get_work_relationship(&self.work_id, &WorkRelationshipField::Comments)?;
        let mut next: Vec<EntityId> = current
            .into_iter()
            .filter(|id| !self.created_comment_ids.contains(id))
            .collect();
        if attached {
            next.extend_from_slice(&self.created_comment_ids);
        }
        uow.set_work_relationship(&self.work_id, &WorkRelationshipField::Comments, &next)?;
        Ok(())
    }
}

/// Create one imported comment and its replies, and return its id.
///
/// Every anchor field is copied straight across. `document_ingest::plan` already
/// resolved the quote against this exact Djot with the same matcher the editor uses,
/// so anything decided again here could only be a second, disagreeing opinion.
fn create_comment(
    uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
    comment: &ImportComment,
    content_id: EntityId,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<EntityId> {
    let ImportComment::Found {
        kind,
        author_name,
        created_at,
        body,
        resolved,
        orphaned,
        orphan_reason,
        range_start,
        range_length,
        quote_prefix,
        quote_exact,
        quote_exact_truncated,
        quote_suffix,
        block_ordinal_hint,
        replies,
    } = comment
    else {
        return Err(anyhow!("apply_document_import: an empty comment row"));
    };

    // The author's own date, when the source carried a readable one. A comment
    // stamped with the moment of import would tell the writer nothing they did not
    // already know, and would lose the one fact the file actually recorded.
    let written = parse_created_at(created_at).unwrap_or(now);

    let mut reply_ids: Vec<EntityId> = Vec::with_capacity(replies.len());
    for reply in replies {
        let ImportReply::Found {
            author_name,
            created_at,
            body,
        } = reply
        else {
            continue;
        };
        let at = parse_created_at(created_at).unwrap_or(written);
        let created = uow.create_orphan_comment_reply(&CommentReply {
            created_at: at,
            updated_at: at,
            author_name: author_name.clone(),
            body: body.clone(),
            ..Default::default()
        })?;
        reply_ids.push(created.id);
    }

    let created = uow.create_orphan_comment(&Comment {
        created_at: written,
        updated_at: written,
        kind: anchor_kind(kind),
        author_name: author_name.clone(),
        body: body.clone(),
        resolved: *resolved,
        orphaned: *orphaned,
        orphan_reason: orphan_reason_of(orphan_reason),
        range_start: (*range_start).max(0) as u64,
        range_length: (*range_length).max(0) as u64,
        quote_prefix: quote_prefix.clone(),
        quote_exact: quote_exact.clone(),
        quote_exact_truncated: *quote_exact_truncated,
        quote_suffix: quote_suffix.clone(),
        block_ordinal_hint: (*block_ordinal_hint).max(0) as u64,
        ..Default::default()
    })?;

    uow.set_comment_relationship(
        &created.id,
        &CommentRelationshipField::Content,
        &[content_id],
    )?;
    if !reply_ids.is_empty() {
        uow.set_comment_relationship(&created.id, &CommentRelationshipField::Replies, &reply_ids)?;
    }
    Ok(created.id)
}

/// RFC 3339, and the two shapes producers write without an offset.
fn parse_created_at(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&chrono::Utc));
    }
    for format in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M"] {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(value, format) {
            return Some(naive.and_utc());
        }
    }
    None
}

fn anchor_kind(kind: &ImportCommentKind) -> CommentAnchorKind {
    match kind {
        ImportCommentKind::Range => CommentAnchorKind::Range,
        ImportCommentKind::Paragraph => CommentAnchorKind::Paragraph,
        ImportCommentKind::Document => CommentAnchorKind::Document,
    }
}

fn orphan_reason_of(reason: &ImportOrphanReason) -> CommentOrphanReason {
    match reason {
        ImportOrphanReason::NotOrphaned => CommentOrphanReason::NotOrphaned,
        ImportOrphanReason::TextNotFound => CommentOrphanReason::TextNotFound,
        ImportOrphanReason::Ambiguous => CommentOrphanReason::Ambiguous,
        ImportOrphanReason::TargetDeleted => CommentOrphanReason::TargetDeleted,
    }
}

/// The `ContentRole` a row of this type stores its prose under, or `None` when
/// the type holds no prose at all.
///
/// Asks `skribisto_model` rather than deciding: the constraint matrix is the one
/// authority on what a combination may carry, and a second copy of that judgement
/// here would drift.
fn prose_role_for(
    role: &common::entities::BinderItemRole,
    sub_role: &common::entities::BinderItemSubRole,
) -> Option<ContentRole> {
    for candidate in [
        ContentRole::SceneText,
        ContentRole::NoteText,
        ContentRole::ParatextText,
    ] {
        if content_allowed(role, sub_role, &candidate) {
            return Some(candidate);
        }
    }
    // Nothing prose-shaped is allowed here. `allowed_content` is consulted so the
    // reason is visible in a debugger rather than inferred.
    let _ = allowed_content(role, sub_role);
    None
}

/// How far to move the whole accepted block so its shallowest row sits at
/// `base_indent`.
///
/// One offset for every row, never a per-row rewrite: the tree the writer reviewed
/// has to arrive with the shape they reviewed, and only its depth may move. The plan's
/// own indents were a preview computed against the destination as it looked during
/// analysis; this re-bases them onto the destination as it is at the moment of writing.
fn indent_shift(rows: &[&ApplyImportRow], base_indent: i64) -> i64 {
    let shallowest = rows
        .iter()
        .filter_map(|row| match row {
            ApplyImportRow::Create { indent, .. } => Some(*indent),
            ApplyImportRow::Empty => None,
        })
        .min()
        .unwrap_or(0);
    base_indent - shallowest
}

/// Where the accepted block lands: `(base indent, the row to insert before)`.
///
/// **The reason this is not a one-line splice.** It used to be: find the anchor in
/// the order, insert immediately after it. But a container's children *follow* it in
/// the same flat list, so inserting after the anchor row put the import between a
/// chapter and its own scenes — and since parentage is derived purely from indent
/// ("the nearest preceding row with a smaller indent"), those scenes silently
/// re-parented onto the imported book. `binder_ordering::resolve_item_target` steps
/// over the anchor's whole subtree, which is the same answer `restore_items_to`
/// already resolves for the identical question.
///
/// An anchor that is not in this binder is an error rather than a silent append: the
/// writer chose a destination, and quietly putting a hundred rows somewhere else is
/// the "it worked, just not where you said" outcome this whole feature exists to
/// avoid.
fn resolve_destination(
    uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
    dto: &ApplyDocumentImportDto,
    order: &[EntityId],
) -> Result<(i64, Option<EntityId>)> {
    let place = match dto.drop_position {
        DropPosition::Before => DropPlace::Before,
        DropPosition::After => DropPlace::After,
        DropPosition::Into => DropPlace::Into,
    };
    let nothing_moving = std::collections::HashSet::new();

    if dto.anchor_item_id == 0 {
        return Ok((0, anchor_for_binder_target(order, place, &nothing_moving)));
    }
    if !order.contains(&dto.anchor_item_id) {
        return Err(anyhow!(
            "apply_document_import: anchor {} is not in this binder",
            dto.anchor_item_id
        ));
    }

    // Indents for the whole destination order — `resolve_item_target` needs them to
    // find where the anchor's subtree ends.
    let items = uow.get_binder_item_multi(order)?;
    let indent: std::collections::HashMap<EntityId, i64> = items
        .iter()
        .flatten()
        .map(|item| (item.id, item.indent))
        .collect();
    let anchor = items
        .iter()
        .flatten()
        .find(|item| item.id == dto.anchor_item_id)
        .ok_or_else(|| {
            anyhow!(
                "apply_document_import: anchor {} could not be read",
                dto.anchor_item_id
            )
        })?;

    resolve_item_target(
        order,
        &indent,
        anchor.id,
        anchor.indent,
        anchor.role == common::entities::BinderItemRole::Folder,
        place,
        &nothing_moving,
    )
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for ApplyDocumentImportUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("apply_document_import: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        self.set_comments_attached(uow.as_mut(), false)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("apply_document_import: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        self.set_comments_attached(uow.as_mut(), true)?;
        uow.commit()?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create(indent: i64) -> ApplyImportRow {
        ApplyImportRow::Create {
            indent,
            kind: crate::dtos::ImportRowKind::Scene,
            title: "row".into(),
            djot: String::new(),
            comments: Vec::new(),
        }
    }

    /// The block moves as one piece: its shallowest row lands at the resolved depth
    /// and every other row keeps its distance from it.
    ///
    /// (There is no `splice_after` to test any more. It inserted immediately after the
    /// anchor *row*, which put an import between a container and its own children —
    /// `binder_ordering::resolve_item_target` steps over the anchor's subtree instead,
    /// and is tested in that crate. The end-to-end consequence is pinned by
    /// `importing_into_a_chapter_never_re_parents_what_was_already_there`.)
    #[test]
    fn the_block_is_rebased_onto_the_destination_as_one_piece() {
        let rows = [create(0), create(1), create(2), create(1)];
        let refs: Vec<&ApplyImportRow> = rows.iter().collect();

        assert_eq!(indent_shift(&refs, 0), 0, "already at the top level");
        assert_eq!(
            indent_shift(&refs, 3),
            3,
            "a nested destination pushes it down"
        );
    }

    /// A plan that was previewed against a deep destination and applied to a shallow
    /// one moves *up*, rather than keeping a depth nothing under it justifies.
    #[test]
    fn a_block_previewed_deeper_than_it_lands_moves_up() {
        let rows = [create(2), create(3)];
        let refs: Vec<&ApplyImportRow> = rows.iter().collect();
        assert_eq!(indent_shift(&refs, 0), -2);
    }

    #[test]
    fn an_empty_block_needs_no_shift() {
        assert_eq!(indent_shift(&[], 4), 4);
    }

    #[test]
    fn a_scene_stores_its_prose_as_scene_text() {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        assert_eq!(
            prose_role_for(&BinderItemRole::Item, &BinderItemSubRole::Scene),
            Some(ContentRole::SceneText)
        );
    }

    #[test]
    fn a_book_holds_no_prose_and_says_so() {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        assert_eq!(
            prose_role_for(&BinderItemRole::Folder, &BinderItemSubRole::Book),
            None
        );
    }
}
