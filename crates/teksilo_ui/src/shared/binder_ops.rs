// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared binder plumbing for the item-editing view-models.
//!
//! [`OutlineViewModel`](crate::binder::OutlineViewModel),
//! [`StreamViewModel`](crate::stream::StreamViewModel),
//! [`CorkboardViewModel`](crate::corkboard::CorkboardViewModel) and
//! [`OverviewViewModel`](crate::overview::OverviewViewModel) are four views over the *same* ordered
//! `Binder.binder_items` stream, so they each need the same handful of backend
//! round-trips: find the binder owning an item, read its order + `(indent, sub_role)` map,
//! create the model's recommended neighbour, move one item relative to another, patch a
//! single scalar field back, and answer the two questions that gate a type conversion.
//! Before this module each of them carried its own copy — the bodies had gone
//! byte-identical, which meant an ordering fix had to be applied three times or not at all.
//!
//! **Why free functions rather than a trait or a base type.** The three view-models reach
//! their handles differently (`StreamViewModel`/`CorkboardViewModel` hold an `Rc<Inner>`, so
//! it is `self.inner.app_ctx`; `OutlineViewModel` holds its fields directly, so it is
//! `self.app_ctx`). Taking `&AppContext` + `&AppIds` explicitly sidesteps that difference
//! entirely and keeps every function callable from a plain unit test with no view-model at
//! all.
//!
//! **Layering.** This sits *below* the view-models and *above* `frontend`: it is allowed to
//! issue `frontend::commands` calls, and it owns no state of its own. The pure ordering math
//! it builds on lives further down still, in [`crate::binder::placement`] (UI-side) and the
//! `binder_ordering` crate (backend-side) — neither of which may touch a unit of work, which
//! is precisely why these backend-touching helpers could not live there.

use teksilo::text_document::{MoveMode, MoveOperation, TextDocument};

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, MoveDto, MovePlace, SplitSceneDto};
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, comment_commands,
    content_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::undo_redo::UndoLabel;
use frontend::direct_access::{
    BinderDto, BinderItemDto, CreateBinderItemDto, UpdateBinderDto, UpdateBinderItemDto,
};
use frontend::direct_access::{CommentDto, ContentDto};

use skribisto_model::SubRoleExt;

use crate::app_ids::AppIds;
use crate::binder::placement::{self, ItemMeta};

// ── Backend reads ────────────────────────────────────────────────────────────

/// The open project's chapter storage mode — which encoding `CreateType` resolves a
/// chapter to (a `Folder`+children, or one flat `ChapterScene` row).
pub(crate) fn chapter_mode(app_ctx: &AppContext, ids: &AppIds) -> skribisto_model::ChapterMode {
    ids.work_id
        .get()
        .and_then(|id| work_commands::get_work(app_ctx, &id).ok().flatten())
        .map(|w| w.chapter_mode)
        .unwrap_or_default()
}

/// Fetch one item, or `None` if it is gone (trashed and purged, or never existed).
pub(crate) fn item_dto(app_ctx: &AppContext, id: u64) -> Option<BinderItemDto> {
    binder_item_commands::get_binder_item(app_ctx, &id)
        .ok()
        .flatten()
}

/// Find the binder owning `id`, its ordered items, and `id`'s position within them.
///
/// A `Work` may hold several binders and the model has no back-link from an item to its
/// binder, so this scans them in order — the same walk the backend does.
pub(crate) fn locate(
    app_ctx: &AppContext,
    ids: &AppIds,
    id: u64,
) -> Option<(u64, Vec<u64>, usize)> {
    let work_id = ids.work_id.get()?;
    let binders =
        work_commands::get_work_relationship(app_ctx, &work_id, &WorkRelationshipField::Binders)
            .ok()?;
    for binder in binders {
        let order = binder_commands::get_binder_relationship(
            app_ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        if let Some(pos) = order.iter().position(|&x| x == id) {
            return Some((binder, order, pos));
        }
    }
    None
}

/// Whether `candidate` lies inside `root`'s subtree — `root` itself included.
///
/// Containment is positional, not a parent link: a subtree is `root` plus every
/// following item of strictly greater indent (see [`binder_ordering::subtree_of`]),
/// so this is the only way to ask the question. `false` when the two are in
/// different binders, or either is unknown.
///
/// The guard a move-into needs: dropping a container onto one of its own
/// descendants is not rejected by the backend, it just writes an ordering the
/// binder can never represent.
pub(crate) fn subtree_contains(
    app_ctx: &AppContext,
    ids: &AppIds,
    root: u64,
    candidate: u64,
) -> bool {
    let Some((_binder, order, _pos)) = locate(app_ctx, ids, root) else {
        return false;
    };
    let indent: std::collections::HashMap<u64, i64> = item_meta(app_ctx, &order)
        .into_iter()
        .map(|(id, (_role, indent, _sub))| (id, indent))
        .collect();
    binder_ordering::subtree_of(&order, &indent, root).contains(&candidate)
}

/// `{id -> (role, indent, sub_role)}` for a binder's items — the data
/// [`crate::binder::placement`] walks to turn "put it after this one" into a concrete
/// `(index, indent)`, and [`go_targets`] walks to resolve each row's
/// `skribisto_model::GoKind`.
pub(crate) fn item_meta(app_ctx: &AppContext, order: &[u64]) -> ItemMeta {
    binder_item_commands::get_binder_item_multi(app_ctx, order)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .map(|it| (it.id, (it.role, it.indent, it.sub_role)))
        .collect()
}

/// Resolve every [`placement::GoTargets`] field for `focused_id`, walking its binder's
/// flat order. One `locate` + `item_meta` backend read feeds the pure walk in
/// [`placement::go_targets_in`], which is where the actual traversal (and its unit
/// tests) live — this wrapper exists only to fetch the data that pure function needs,
/// mirroring every other function in this "Backend reads" section.
pub(crate) fn go_targets(
    app_ctx: &AppContext,
    ids: &AppIds,
    focused_id: u64,
) -> placement::GoTargets {
    let Some((_binder, order, pos)) = locate(app_ctx, ids, focused_id) else {
        return placement::GoTargets::default();
    };
    let meta = item_meta(app_ctx, &order);
    placement::go_targets_in(&order, &meta, pos)
}

// ── Backend writes ───────────────────────────────────────────────────────────

/// Create the model's default recommendation for `anchor_id`, placed by the relation it
/// recommends — through the shared `binder_placement` math, so the outline, the stream and
/// the corkboard all place a new item identically.
///
/// Silently does nothing when the anchor has no recommendation, when the resulting
/// combination fails the constraint matrix, or when the anchor cannot be located — all
/// three mean "there is no valid item to create here", which is not an error the user
/// needs told about.
pub(crate) fn create_by_recommendation(
    app_ctx: &AppContext,
    ids: &AppIds,
    anchor_id: u64,
    anchor_role: &BinderItemRole,
    anchor_sub_role: &BinderItemSubRole,
    title: &str,
) {
    let Some(rec) = skribisto_model::recommendations(anchor_role, anchor_sub_role)
        .into_iter()
        .next()
    else {
        return;
    };
    let (role, sub_role) = rec.create_type.combo(chapter_mode(app_ctx, ids));
    if skribisto_model::validate_item(&role, &sub_role, &[]).is_err() {
        return;
    }
    let Some((binder, order, pos)) = locate(app_ctx, ids, anchor_id) else {
        return;
    };
    let meta = item_meta(app_ctx, &order);
    let anchor_indent = meta.get(&anchor_id).map(|(_role, i, _sr)| *i).unwrap_or(0);
    let (index, indent) =
        placement::insertion_point_for_item(&order, &meta, pos, anchor_indent, rec.relation);

    let is_note = sub_role == BinderItemSubRole::Note;
    let dto = CreateBinderItemDto {
        status: None,
        title: title.to_string(),
        role,
        sub_role,
        activated: true,
        // Out of the export, like every note, whichever door it came through. This one is
        // the corkboard's and the stream's "+", and it recommends `Note`/`NoteFolder` off
        // a note anchor, so without this the same note is exportable or not depending on
        // which button made it.
        is_exportable: !is_note,
        indent,
        ..Default::default()
    };
    let _ = binder_item_commands::create_binder_item(
        app_ctx,
        ids.stack_id.get(),
        &dto,
        binder,
        index as i32,
    );
}

/// Move `id` to sit `place` relative to `target` (both plain items, never binders).
pub(crate) fn move_relative(
    app_ctx: &AppContext,
    ids: &AppIds,
    id: u64,
    target: u64,
    place: MovePlace,
) {
    let _ = binder_item_management_commands::move_items(
        app_ctx,
        ids.stack_id.get(),
        &MoveDto {
            item_ids: vec![id],
            target_id: Some(target),
            target_is_binder: false,
            move_place: place,
        },
    );
}

// ── Type conversion ("Convert to ▸") and its two guards ──────────────────────
//
// Both guards are *presentation*: the `promote` use case re-validates everything and
// refuses a lossy or illegal conversion on its own. They exist so the refusal arrives as
// an explanation the writer can act on, instead of a menu item that quietly does nothing.

/// Every `Content` row hanging off `item_id`, or empty when it has none.
///
/// The fetch is two round-trips — a relationship read, then a multi-get — and it had
/// been written out twice, here and in `models::manuscript_digest::live_prose`. The two
/// want different things *from* the rows (which roles hold text, versus one role's text),
/// but the way the rows are reached is the same, and a change to that half — an added
/// filter, different error handling — would otherwise have to reach both or leave them
/// disagreeing about what a row holds.
///
/// Absent and unreadable both come back empty: a row with no content rows is ordinary,
/// and neither caller can act on the difference.
pub(crate) fn contents_of(app_ctx: &AppContext, item_id: u64) -> Vec<ContentDto> {
    let content_ids = binder_item_commands::get_binder_item_relationship(
        app_ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap_or_default();
    content_commands::get_content_multi(app_ctx, &content_ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .collect()
}

/// The content roles whose text `item_id` would **lose** by becoming `target` (empty rows
/// never count). Non-empty means the conversion must be refused: a chapter holding prose
/// cannot become a Part, which has nowhere to put it.
pub(crate) fn promote_content_loss(
    app_ctx: &AppContext,
    item_id: u64,
    target: skribisto_model::PromoteTarget,
) -> Vec<ContentRole> {
    let (target_role, target_sub_role) = target.combo();
    let non_empty: Vec<ContentRole> = contents_of(app_ctx, item_id)
        .into_iter()
        .filter(|c| !c.data.trim().is_empty())
        .map(|c| c.role)
        .collect();
    skribisto_model::promote_content_loss(&target_role, &target_sub_role, &non_empty)
}

/// The number of child items that block converting `item_id` into `target` — non-zero
/// only when a container would become a leaf (a chapter folder → a flat chapter) while it
/// still holds **live** items. Trashed descendants do not block: the prompt the caller
/// shows offers trashing as one of the two ways out, so it must accept the result.
///
/// Locates the item through [`locate`] — i.e. by walking the *backend's* binders — rather
/// than through any one view's tree. A view-scoped lookup answers `0` for an item its
/// tree is not currently showing (filtered by a search, scoped to another binder), and
/// `0` here reads as "nothing blocks this", silently waving through the very conversion
/// the guard exists to stop.
pub(crate) fn demote_blocked_children(
    app_ctx: &AppContext,
    ids: &AppIds,
    item_id: u64,
    target: skribisto_model::PromoteTarget,
) -> usize {
    let Some(dto) = item_dto(app_ctx, item_id) else {
        return 0;
    };
    // Only a container → leaf conversion is gated on emptiness.
    if !(dto.role == BinderItemRole::Folder && target.combo().0 == BinderItemRole::Item) {
        return 0;
    }
    let Some((_binder, order, pos)) = locate(app_ctx, ids, item_id) else {
        return 0;
    };
    let meta = item_meta(app_ctx, &order);
    let end = placement::subtree_end(&order, &meta, pos, dto.indent);
    let span = &order[pos + 1..end];
    // The span is topological, so it still counts *trashed* descendants: `activated =
    // !trashed` and a trashed row keeps its slot and indent in the binder order. Counting
    // the raw span therefore blocked a chapter the writer had already emptied — while the
    // prompt was telling them to "move or trash them first". Only live rows block.
    match binder_item_commands::get_binder_item_multi(app_ctx, span) {
        Ok(items) => items
            .into_iter()
            .flatten()
            .filter(|it| it.activated)
            .count(),
        // A failed read must not answer `0`: that reads as "nothing blocks this" and waves
        // through the conversion this guard exists to stop. Fall back to the whole span.
        Err(_) => span.len(),
    }
}

/// Convert `item_id` to `target` (undoable), reporting whether it took. The use case
/// re-validates the target against the item's current type, so this is safe to call even
/// from a stale menu; the demote-empty guard is the caller's job.
pub(crate) fn promote(
    app_ctx: &AppContext,
    ids: &AppIds,
    item_id: u64,
    target: skribisto_model::PromoteTarget,
) -> bool {
    binder_item_management_commands::promote(
        app_ctx,
        ids.stack_id.get(),
        &frontend::binder_item_management::PromoteDto {
            item_id,
            target: target.code(),
        },
    )
    .is_ok()
}

/// Every type `item_id` may become — the rows of the "Convert to ▸" submenu. Empty when
/// the item is gone or its type has no paired alternative.
pub(crate) fn promote_targets_of(
    app_ctx: &AppContext,
    item_id: u64,
) -> Vec<skribisto_model::PromoteTarget> {
    match item_dto(app_ctx, item_id) {
        Some(dto) => skribisto_model::promote_targets(&dto.role, &dto.sub_role),
        None => Vec::new(),
    }
}

// ── Pure helpers ─────────────────────────────────────────────────────────────

/// Build a scalar-only `UpdateBinderItemDto` from a fetched item.
///
/// Relationships (`References`, `Tags`, `Contents`) are deliberately absent: this is the
/// DTO used to patch **one field** back, and carrying stale relationship vectors through a
/// read-modify-write would let a concurrent edit be clobbered by whatever the reader saw.
pub(crate) fn update_item_dto(it: &BinderItemDto) -> UpdateBinderItemDto {
    UpdateBinderItemDto {
        id: it.id,
        created_at: it.created_at,
        // Stamped here, not by the callers. `updated_at` is caller-maintained
        // all the way down — `BinderItemWriteUoW::update_multi` persists
        // whatever the DTO carries — so every writer that forgot silently left
        // the row claiming it was last touched by some earlier edit. Four of
        // them did: the label writers in Overview / Corkboard / Stream and the
        // outline's indent write. Putting the stamp in the read-modify-write
        // vehicle itself makes forgetting impossible, which matters now that
        // "last modified" is about to become visible to writers.
        updated_at: chrono::Utc::now(),
        // Carried through unchanged: `uid` is the item's durable identity,
        // never re-minted by an edit.
        uid: it.uid.clone(),
        title: it.title.clone(),
        sub_title: it.sub_title.clone(),
        role: it.role.clone(),
        sub_role: it.sub_role.clone(),
        label: it.label.clone(),
        activated: it.activated,
        is_favorite: it.is_favorite,
        is_exportable: it.is_exportable,
        exclude_from_numbering: it.exclude_from_numbering,
        indent: it.indent,
        word_count_goal: it.word_count_goal,
        char_count_goal: it.char_count_goal,
        dict_language: it.dict_language.clone(),
        aliases: it.aliases.clone(),
    }
}

/// The `Binder` counterpart of [`update_item_dto`] — same contract, same reason.
///
/// A `Binder` carries far less (`uid`, `name`, `activated`), so the one caller
/// that needed it hand-rolled the struct and, having nothing to copy the rule
/// from, carried `updated_at` through unchanged: renaming a binder left it
/// claiming it had not been touched. Give the binder a vehicle too and the
/// asymmetry that caused it is gone.
pub(crate) fn update_binder_dto(b: &BinderDto) -> UpdateBinderDto {
    UpdateBinderDto {
        id: b.id,
        created_at: b.created_at,
        updated_at: chrono::Utc::now(),
        // Durable identity: carried, never re-minted.
        uid: b.uid.clone(),
        name: b.name.clone(),
        activated: b.activated,
    }
}

/// Does this `(role, sub_role)` carry scene prose? The **constraint matrix** decides —
/// not `SubRoleExt::carries_scene()` — because the matrix is the source of truth and the
/// backend gates split/merge on exactly this predicate.
pub(crate) fn is_prose_bearing(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    skribisto_model::content_allowed(role, sub_role, &ContentRole::SceneText)
}

/// Does this `(role, sub_role)` carry a synopsis? Same rule, other role — it is what
/// decides whether a row gets an editor in the Full Synopsis flavour.
pub(crate) fn is_synopsis_bearing(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    skribisto_model::content_allowed(role, sub_role, &ContentRole::SynopsisText)
}

/// Does this row open a structural section? Such a row must never be merged *away*: it
/// would delete the boundary (and, for a chapter folder, orphan its child scenes).
pub(crate) fn opens_a_section(sub_role: &BinderItemSubRole) -> bool {
    sub_role.opens_chapter() || sub_role.opens_part() || sub_role.opens_book()
}

/// Split `doc` at the caret offset `caret` into two Djot strings, each half whole blocks
/// that keep their formatting, block and inline.
///
/// **The blanks at the cut go with neither half.** The spaces and tabs touching the caret
/// inside the paragraph it cuts are left out of both halves ([`cut_at`]): a writer splitting
/// at "Hello| world" gets "Hello" and "world", not a scene ending on a space and a new one
/// opening with it. `text-document` keeps the blanks a paragraph opens or closes with through
/// every save, so left in they would stay in both scenes. A paragraph's indentation is
/// the exception: with nothing but blanks before the caret and words after it, the
/// paragraph moves to the new scene whole, its leading blanks as the writer typed them.
/// Blanks anywhere else, and any other space character (a no-break space, an ideographic
/// space), are text and are kept.
///
/// **A paragraph cut in two is two paragraphs of its kind**, the way Enter breaks one: a
/// heading, a list item or a quotation keeps its heading level, its list or its quotation
/// on both halves. The cut is made in a copy of the document, with a paragraph break put
/// where it falls, and each half is then taken whole blocks at a time. A selection that
/// ends inside a paragraph carries no block format (`text-document` pastes such a piece
/// into the paragraph it lands in), so taken as a piece the half of a heading came out a
/// plain paragraph, and so did a heading or a list item the caret ended.
///
/// **A table is never cut.** A caret anywhere in one cuts in front of it, and the table
/// moves to the new scene whole, as an indented paragraph does. A selection that starts or
/// ends inside a table takes the whole table, and one that starts there loses what follows
/// the table, so a split in a cell used to put the table in both scenes and drop the
/// scene's text after it. A half that starts with a table is taken from the paragraph break
/// in front of it, and the half before it is made by deleting everything from that break on,
/// which takes the table whole and leaves the paragraph before it as it was.
///
/// **A split with nothing on one side returns `Err`, not an empty half:** at the very start
/// or end of the text, or with only blanks and empty paragraphs between the caret and that
/// edge. Every caller treats `Err` as "do nothing", so such a split is a silent no-op, which
/// is the sane outcome: neither would produce two scenes. The same goes for a cut that falls
/// between two tables, which no half can be taken at without cutting one of them.
pub(crate) fn split_djot(doc: &TextDocument, caret: usize) -> anyhow::Result<SceneSplit> {
    let end = end_of(doc);
    let cut = cut_at(doc, caret.min(end))?;
    // What each half holds, read off the document before anything is copied: a half of
    // blanks and paragraph breaks alone is no scene.
    let (kept_to, moved_from) = cut.halves();
    if is_blank(&doc.text_at(0, kept_to)?) {
        anyhow::bail!("nothing before the caret to leave in the scene");
    }
    if is_blank(&doc.text_at(moved_from, end.saturating_sub(moved_from))?) {
        anyhow::bail!("nothing after the caret to move to a new scene");
    }

    // The cut is made on a copy, so the writer's document is not touched.
    let scratch = exact_copy(doc, end)?;
    let (before_break, after_break) = cut.apply(&scratch)?;
    let scratch_end = end_of(&scratch);

    // The block right after `after_break` opens the new scene. On a table's anchor the
    // caret's block is the table's first cell.
    let opens_with_table = in_table(&scratch, after_break + 1)?;
    let before = if opens_with_table {
        if in_table(&scratch, before_break)? {
            anyhow::bail!("the cut falls between two tables");
        }
        let before = exact_copy(&scratch, scratch_end)?;
        let c = before.cursor();
        c.set_position(before_break, MoveMode::MoveAnchor);
        c.set_position(scratch_end, MoveMode::KeepAnchor);
        c.remove_selected_text()?;
        before.to_djot()?
    } else {
        // Taken up to the start of the next block, so the last block is taken past its
        // paragraph break, whole, with its block format.
        extract(&scratch, 0, before_break + 1)?
    };
    let after = if opens_with_table {
        extract(&scratch, after_break, scratch_end)?
    } else {
        extract(&scratch, after_break + 1, scratch_end)?
    };
    // Where the moved text starts in `doc`, past the paragraph break when the cut fell at
    // one, and how many paragraphs stand in front of it: where a comment on it moves from.
    let first_moved = cut.first_moved();
    let blocks_before = doc
        .blocks()
        .iter()
        .filter(|block| block.position() < first_moved)
        .count();
    Ok(SceneSplit {
        before,
        after,
        moved_from: first_moved,
        blocks_before,
    })
}

/// A scene's text cut in two by [`split_djot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SceneSplit {
    /// The Djot left in the scene.
    pub before: String,
    /// The Djot moved to the new scene.
    pub after: String,
    /// Where the moved text starts in the document that was cut, in its cursor positions:
    /// what was anchored there or later moved with it.
    pub moved_from: usize,
    /// How many paragraphs of the document that was cut stand before the moved text.
    pub blocks_before: usize,
}

/// The comments a split moves to the new scene: the live ones anchored wholly in the text
/// from `moved_from` on, a paragraph comment on a paragraph that moved included.
///
/// A comment over the cut stays with the scene it opens in, where its first words are.
pub(crate) fn comments_moved_by_split(
    live: &[crate::comments::session::LiveAnchor],
    moved_from: usize,
) -> Vec<u64> {
    let mut moved: Vec<u64> = live
        .iter()
        .filter(|anchor| anchor.start >= moved_from)
        .map(|anchor| anchor.comment_id)
        .collect();
    moved.sort_unstable();
    moved.dedup();
    moved
}

/// Split `dto.source_id` with `split_scene`, and carry every comment in `comments` to the
/// new scene's `role` text, all as one step of the project's undo history. `split` is where
/// the text was cut ([`split_djot`]).
///
/// A comment is tied to the text it was written on by that text's `Content` row, and the
/// split gives the moved words a row of their own. Left pointing at the old one, a comment
/// on them found its words gone at the next opening and was shown as having lost its text,
/// while its words were in the next scene. Undoing the split takes the comments back.
pub(crate) fn split_scene_carrying_comments(
    app_ctx: &AppContext,
    ids: &AppIds,
    stack: Option<u64>,
    dto: &SplitSceneDto,
    role: ContentRole,
    comments: &[u64],
    split: &SceneSplit,
) -> anyhow::Result<()> {
    if comments.is_empty() {
        return binder_item_management_commands::split_scene(app_ctx, stack, dto);
    }
    undo_redo_commands::begin_composite_labeled(
        app_ctx,
        stack,
        Some(UndoLabel::act("split_scene")),
    )?;
    let outcome = (|| -> anyhow::Result<()> {
        binder_item_management_commands::split_scene(app_ctx, stack, dto)?;
        // The split puts the new scene right after the one it cut.
        let (_, order, position) = locate(app_ctx, ids, dto.source_id)
            .ok_or_else(|| anyhow::anyhow!("the split scene is in no binder"))?;
        let new_scene = *order
            .get(position + 1)
            .ok_or_else(|| anyhow::anyhow!("no scene after the one split"))?;
        let text = row_of(app_ctx, new_scene, &role)?;
        // The moved text opens the new scene: what stood at `moved_from` stands at 0.
        let back = |n: usize| i64::try_from(n).map(|n| -n).unwrap_or(i64::MIN);
        carry_comments(
            app_ctx,
            stack,
            comments,
            text,
            back(split.moved_from),
            back(split.blocks_before),
        )
    })();
    // Closed whatever happened: what was done is one step the writer can take back.
    undo_redo_commands::end_composite(app_ctx);
    outcome
}

/// Merge `dto.source_id` into `dto.target_id` with `merge_two_scenes`, and carry every
/// comment on the source's text to the target's text of the same kind, where that text now
/// is, all as one step of the project's undo history.
///
/// The merge moves the source's words to the end of the target and trashes the source.
/// A comment left on the source's `Content` row went into the trash with it, while the
/// words it was written on were live in the target, which showed none of the comments on
/// them. Every comment on the source's row goes, one whose words were already lost
/// included: they were lost from that text, which is the target's now.
pub(crate) fn merge_scenes_carrying_comments(
    app_ctx: &AppContext,
    stack: Option<u64>,
    dto: &MergeTwoScenesDto,
) -> anyhow::Result<()> {
    // What moves and how far, read before the merge changes anything.
    let target_rows = contents_of(app_ctx, dto.target_id);
    let source_rows = contents_of(app_ctx, dto.source_id);
    let comment_ids = work_commands::get_work_relationship(
        app_ctx,
        &dto.work_id,
        &WorkRelationshipField::Comments,
    )?;
    let comments: Vec<CommentDto> = comment_commands::get_comment_multi(app_ctx, &comment_ids)?
        .into_iter()
        .flatten()
        .collect();
    let mut carried: Vec<(ContentRole, Vec<u64>, (i64, i64))> = Vec::new();
    for role in [ContentRole::SceneText, ContentRole::SynopsisText] {
        let Some(source) = source_rows.iter().find(|row| row.role == role) else {
            continue;
        };
        // The merge leaves a text of the source's alone when it holds nothing, as
        // `merge_two_scenes` decides it, and a comment on it stays where it is.
        if source.data.trim().is_empty() {
            continue;
        }
        let moved: Vec<u64> = comments
            .iter()
            .filter(|comment| comment.content == Some(source.id))
            .map(|comment| comment.id)
            .collect();
        if moved.is_empty() {
            continue;
        }
        let ahead = match target_rows.iter().find(|row| row.role == role) {
            Some(target) => text_ahead_of_a_join(&target.data)?,
            None => (0, 0),
        };
        carried.push((role, moved, ahead));
    }
    if carried.is_empty() {
        return binder_item_management_commands::merge_two_scenes(app_ctx, stack, dto);
    }
    undo_redo_commands::begin_composite_labeled(
        app_ctx,
        stack,
        Some(UndoLabel::act("merge_two_scenes")),
    )?;
    let outcome = (|| -> anyhow::Result<()> {
        binder_item_management_commands::merge_two_scenes(app_ctx, stack, dto)?;
        for (role, moved, (chars, blocks)) in &carried {
            let text = row_of(app_ctx, dto.target_id, role)?;
            carry_comments(app_ctx, stack, moved, text, *chars, *blocks)?;
        }
        Ok(())
    })();
    undo_redo_commands::end_composite(app_ctx);
    outcome
}

/// The id of `item`'s text of kind `role`.
fn row_of(app_ctx: &AppContext, item: u64, role: &ContentRole) -> anyhow::Result<u64> {
    contents_of(app_ctx, item)
        .into_iter()
        .find(|row| row.role == *role)
        .map(|row| row.id)
        .ok_or_else(|| anyhow::anyhow!("the scene holds no text of that kind"))
}

/// How far the text a merge appends after `djot` stands from the start: the cursor
/// positions and the paragraphs of `djot` as the editor reads it, plus the paragraph break
/// the join puts between the two. Nothing for a text of blanks, which the join replaces
/// with the appended one (`merge_two_scenes`' `join_text`).
fn text_ahead_of_a_join(djot: &str) -> anyhow::Result<(i64, i64)> {
    if djot.trim().is_empty() {
        return Ok((0, 0));
    }
    let doc = TextDocument::new();
    doc.set_djot_sync(djot.trim_end_matches(['\n', '\r']))?;
    let chars = i64::try_from(end_of(&doc) + 1)?;
    let blocks = i64::try_from(doc.blocks().len())?;
    Ok((chars, blocks))
}

/// Point each of `comments` at the text `content`, its stored place moved by `chars`
/// positions and `blocks` paragraphs, to where its words now stand there.
///
/// The place is only where the anchoring looks first (`skribisto_model::comment_anchor`):
/// one that no longer holds the quote is searched for. Moved with the words, it holds it,
/// and a quote as short as one word is not found at the same offset of the other text,
/// where other words stand.
fn carry_comments(
    app_ctx: &AppContext,
    stack: Option<u64>,
    comments: &[u64],
    content: u64,
    chars: i64,
    blocks: i64,
) -> anyhow::Result<()> {
    for id in comments {
        let Some(mut comment) = comment_commands::get_comment(app_ctx, id)? else {
            continue;
        };
        comment.content = Some(content);
        comment.range_start = comment.range_start.saturating_add_signed(chars);
        comment.block_ordinal_hint = comment.block_ordinal_hint.saturating_add_signed(blocks);
        comment.updated_at = chrono::Utc::now();
        comment_commands::update_comment_with_relationships(app_ctx, stack, &comment)?;
    }
    Ok(())
}

/// The cursor position at the end of `doc`'s main text. `character_count()` is not that
/// position: it leaves out the separator between each two paragraphs.
fn end_of(doc: &TextDocument) -> usize {
    let c = doc.cursor();
    c.move_position(MoveOperation::End, MoveMode::MoveAnchor, 1);
    c.position()
}

/// Whether `text` holds nothing a scene would show: spaces, tabs and paragraph breaks
/// alone. A picture, a note reference or a table stands in the text as a character of its
/// own, and counts.
fn is_blank(text: &str) -> bool {
    text.chars().all(|c| matches!(c, ' ' | '\t' | '\n'))
}

/// `doc`'s main text from `from` to `to`, as the Djot of a document of its own.
fn extract(doc: &TextDocument, from: usize, to: usize) -> anyhow::Result<String> {
    let c = doc.cursor();
    c.set_position(from, MoveMode::MoveAnchor);
    c.set_position(to, MoveMode::KeepAnchor);
    let tmp = TextDocument::new();
    tmp.cursor().insert_fragment(&c.selection())?;
    Ok(tmp.to_djot()?)
}

/// A document holding `doc`'s main text, `end` being where it ends, at the same positions,
/// so a cut found in `doc` falls in the same place in it.
///
/// Read from `doc`'s Djot first, which is what a save stores and a reload reads. That drops
/// an empty paragraph closing the text, which a copy of the whole text keeps; a copy of the
/// whole text, pasted into a new document, puts an empty paragraph in front of a table that
/// opens it. The first of the two to hold the same text at the same positions is used, and
/// with neither, the split is refused rather than made at the wrong place.
fn exact_copy(doc: &TextDocument, end: usize) -> anyhow::Result<TextDocument> {
    let text = doc.text_at(0, end)?;
    let holds_it = |copy: &TextDocument| {
        end_of(copy) == end && copy.text_at(0, end).is_ok_and(|copied| copied == text)
    };
    let read = TextDocument::new();
    read.set_djot_sync(&doc.to_djot()?)?;
    if holds_it(&read) {
        return Ok(read);
    }
    let c = doc.cursor();
    c.set_position(0, MoveMode::MoveAnchor);
    c.set_position(end, MoveMode::KeepAnchor);
    let pasted = TextDocument::new();
    pasted.cursor().insert_fragment(&c.selection())?;
    if holds_it(&pasted) {
        return Ok(pasted);
    }
    anyhow::bail!("no copy of the scene holds it at the positions it has")
}

/// Whether a caret at `position` stands in a table: in one of its cells, or on its anchor.
fn in_table(doc: &TextDocument, position: usize) -> anyhow::Result<bool> {
    let block = doc.block_at_caret(position)?;
    Ok(doc
        .block_by_id(block.block_id)
        .is_some_and(|block| block.table_cell().is_some()))
}

/// Where a split falls, in the document's cursor positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cut {
    /// In front of the block starting at `start`, which moves whole: the caret was at its
    /// start or in its indentation, or anywhere in the table whose anchor is at `start`.
    Before { start: usize },
    /// A paragraph of blanks alone, or an empty one, from `start` to `end`, which goes with
    /// neither half.
    Dropped { start: usize, end: usize },
    /// At `at`, the end of a paragraph's text, the blanks from there to `end` going with
    /// neither half.
    AtEnd { at: usize, end: usize },
    /// Inside a paragraph: its text up to `keep` stays, from `moved` on it moves, and the
    /// blanks between the two go with neither half.
    Inside { keep: usize, moved: usize },
}

impl Cut {
    /// Where the text left behind ends and where the text moved starts.
    fn halves(self) -> (usize, usize) {
        match self {
            Cut::Before { start } => (start, start),
            Cut::Dropped { start, end } => (start, end),
            Cut::AtEnd { at, end } => (at, end),
            Cut::Inside { keep, moved } => (keep, moved),
        }
    }

    /// Where the moved text's first character stands: past the paragraph break when the cut
    /// ends a paragraph, whether its last words or its blanks alone.
    fn first_moved(self) -> usize {
        match self {
            Cut::Before { start } => start,
            Cut::Inside { moved, .. } => moved,
            Cut::Dropped { end, .. } | Cut::AtEnd { end, .. } => end + 1,
        }
    }

    /// Make the cut in `scratch`, a copy of the document, so it falls between two blocks,
    /// and say where: the paragraph break after the last block left behind, and the one in
    /// front of the first block moved. They differ only when a paragraph goes with neither
    /// half.
    fn apply(self, scratch: &TextDocument) -> anyhow::Result<(usize, usize)> {
        let remove = |from: usize, to: usize| -> anyhow::Result<()> {
            if from < to {
                let c = scratch.cursor();
                c.set_position(from, MoveMode::MoveAnchor);
                c.set_position(to, MoveMode::KeepAnchor);
                c.remove_selected_text()?;
            }
            Ok(())
        };
        match self {
            Cut::Before { start } => {
                let at = start
                    .checked_sub(1)
                    .ok_or_else(|| anyhow::anyhow!("nothing before the caret"))?;
                Ok((at, at))
            }
            Cut::Dropped { start, end } => {
                remove(start, end)?;
                let before = start
                    .checked_sub(1)
                    .ok_or_else(|| anyhow::anyhow!("nothing before the caret"))?;
                Ok((before, start))
            }
            Cut::AtEnd { at, end } => {
                remove(at, end)?;
                Ok((at, at))
            }
            Cut::Inside { keep, moved } => {
                remove(keep, moved)?;
                // A paragraph break, the way Enter makes one: the new paragraph takes the
                // heading level, the list and the quotation of the one it is cut from.
                let c = scratch.cursor();
                c.set_position(keep, MoveMode::MoveAnchor);
                c.insert_block()?;
                Ok((keep, keep))
            }
        }
    }
}

/// Where a split at `caret` falls ([`Cut`]).
///
/// Only the caret's paragraph is looked at, and only spaces, tabs and the line breaks of a
/// code block count as blanks: they are what the writer puts between words and lines. A caret with nothing but blanks before it in its
/// paragraph and words after it (at the paragraph's very start, or inside its indentation)
/// cuts in front of the whole paragraph, so the indentation the writer typed opens the new
/// scene with it. A paragraph of blanks alone goes with neither half. A caret in a table
/// cuts in front of the table.
fn cut_at(doc: &TextDocument, caret: usize) -> anyhow::Result<Cut> {
    let block = doc.block_at_caret(caret)?;
    if let Some(cell) = doc
        .block_by_id(block.block_id)
        .and_then(|block| block.table_cell())
    {
        return Ok(Cut::Before {
            start: table_anchor(doc, &cell.table)?,
        });
    }
    let text: Vec<char> = doc.text_at(block.start, block.length)?.chars().collect();
    let at = caret.saturating_sub(block.start).min(text.len());
    // A line break stands inside a paragraph only in a code block, where it ends a line.
    let is_blank = |c: &&char| matches!(**c, ' ' | '\t' | '\n');
    let blanks_before = text[..at].iter().rev().take_while(is_blank).count();
    let blanks_after = text[at..].iter().take_while(is_blank).count();
    let block_end = block.start + text.len();
    let keep = block.start + at - blanks_before;
    let moved = block.start + at + blanks_after;
    Ok(if keep == block.start && moved == block_end {
        Cut::Dropped {
            start: block.start,
            end: block_end,
        }
    } else if keep == block.start {
        Cut::Before { start: block.start }
    } else if moved == block_end {
        Cut::AtEnd {
            at: keep,
            end: block_end,
        }
    } else {
        Cut::Inside { keep, moved }
    })
}

/// Where `table`'s anchor stands, the one character the table occupies in the main text,
/// in front of its first cell and a paragraph break.
fn table_anchor(
    doc: &TextDocument,
    table: &teksilo::text_document::TextTable,
) -> anyhow::Result<usize> {
    let first_cell = (0..table.columns())
        .find_map(|column| table.cell(0, column))
        .and_then(|cell| cell.blocks().first().map(|block| block.position()))
        .ok_or_else(|| anyhow::anyhow!("a table with no first cell"))?;
    let anchor = first_cell
        .checked_sub(2)
        .filter(|&anchor| {
            doc.text_at(anchor, 2)
                .is_ok_and(|text| text == "\u{fffc}\n")
        })
        .ok_or_else(|| anyhow::anyhow!("no anchor in front of the table"))?;
    Ok(anchor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::BinderItemRole::{Folder, Item};
    use frontend::common::entities::BinderItemSubRole::{
        Book, ChapterScene, Note, Part, Scene, Text,
    };

    /// The prose/synopsis predicates read the constraint matrix, so they agree with what
    /// the backend will accept: both encodings of a chapter carry prose; a part and a book
    /// carry only a synopsis.
    #[test]
    fn prose_and_synopsis_predicates_follow_the_constraint_matrix() {
        assert!(is_prose_bearing(&Item, &Scene));
        assert!(is_prose_bearing(&Item, &ChapterScene));
        assert!(is_prose_bearing(&Folder, &ChapterScene));
        assert!(!is_prose_bearing(&Folder, &Part));
        assert!(!is_prose_bearing(&Folder, &Book));

        // Every stream row has a synopsis — that is what makes the Full Synopsis stream a
        // complete outline with no holes.
        assert!(is_synopsis_bearing(&Item, &Scene));
        assert!(is_synopsis_bearing(&Item, &ChapterScene));
        assert!(is_synopsis_bearing(&Folder, &ChapterScene));
        assert!(is_synopsis_bearing(&Folder, &Part));
        assert!(is_synopsis_bearing(&Folder, &Book));
    }

    /// The three section openers, and nothing else. A plain scene or note may be merged
    /// away; a chapter/part/book opener may not.
    #[test]
    fn only_chapter_part_and_book_open_a_section() {
        assert!(opens_a_section(&ChapterScene));
        assert!(opens_a_section(&Part));
        assert!(opens_a_section(&Book));
        assert!(!opens_a_section(&Scene));
        assert!(!opens_a_section(&Note));
        assert!(!opens_a_section(&Text));
    }

    /// `update_item_dto` is a read-modify-write vehicle: every scalar must survive the
    /// round-trip, or patching one field would silently reset another.
    #[test]
    fn update_item_dto_carries_every_scalar_across() {
        let created = chrono::DateTime::from_timestamp(1_700_000_000, 0).expect("ts");
        let updated = chrono::DateTime::from_timestamp(1_700_009_999, 0).expect("ts");
        let src = BinderItemDto {
            id: 7,
            uid: common::uid::fixture_uid(7),
            created_at: created,
            updated_at: updated,
            title: "Chapter One".into(),
            sub_title: "a beginning".into(),
            role: Item,
            sub_role: Scene,
            label: "draft".into(),
            activated: true,
            is_favorite: true,
            status: None,
            is_exportable: false,
            exclude_from_numbering: false,
            indent: 3,
            word_count_goal: 1200,
            char_count_goal: 6000,
            dict_language: vec!["fr-FR".to_string()],
            // A list of primitives, not a relationship: it *is* a scalar as far as the
            // patch DTO is concerned and must survive, or renaming an item would wipe
            // the aliases the mention index depends on.
            aliases: vec!["Lizzy".into(), "Miss Bennet".into()],
            // The relationship vectors below are exactly what must NOT survive into the
            // update DTO — it has no fields for them.
            contents: vec![41, 42],
            references: vec![43],
            point_of_view: vec![],
            books: vec![],
            tags: vec![44],
        };

        let out = update_item_dto(&src);

        assert_eq!(out.id, 7);
        assert_eq!(
            out.uid,
            common::uid::fixture_uid(7),
            "the durable identity must survive an edit unchanged -- re-minting              it here would orphan every reference to the row"
        );
        assert_eq!(
            out.created_at, created,
            "creation time is history — an edit must never rewrite it"
        );
        assert!(
            out.updated_at > updated,
            "the vehicle stamps `updated_at`; carrying the old value through is \
             what left `last modified` lying after a label edit or an indent"
        );
        assert_eq!(out.title, "Chapter One");
        assert_eq!(out.sub_title, "a beginning");
        assert_eq!(out.role, Item);
        assert_eq!(out.sub_role, Scene);
        assert_eq!(out.label, "draft");
        assert!(out.activated);
        assert!(out.is_favorite);
        assert!(!out.is_exportable);
        assert_eq!(out.indent, 3);
        assert_eq!(out.word_count_goal, 1200);
        assert_eq!(out.char_count_goal, 6000);
        assert_eq!(out.dict_language, vec!["fr-FR".to_string()]);
        assert_eq!(out.aliases, vec!["Lizzy", "Miss Bennet"]);
    }

    /// The binder vehicle keeps creation history and refreshes the edit stamp.
    ///
    /// Renaming a binder used to carry `updated_at` straight through, because
    /// the call site hand-rolled the struct with no vehicle to copy the rule
    /// from. Pinned here so the binder half cannot drift from the item half.
    #[test]
    fn update_binder_dto_stamps_the_edit_and_keeps_the_creation_time() {
        let created = chrono::DateTime::from_timestamp(1_700_000_000, 0).expect("ts");
        let updated = chrono::DateTime::from_timestamp(1_700_009_999, 0).expect("ts");
        let src = frontend::direct_access::BinderDto {
            id: 3,
            uid: common::uid::fixture_uid(3),
            created_at: created,
            updated_at: updated,
            name: "Manuscript".into(),
            activated: true,
            binder_items: vec![11, 12],
        };

        let out = update_binder_dto(&src);

        assert_eq!(out.id, 3);
        assert_eq!(out.uid, common::uid::fixture_uid(3));
        assert_eq!(out.name, "Manuscript");
        assert!(out.activated);
        assert_eq!(
            out.created_at, created,
            "creation time is history — a rename must never rewrite it"
        );
        assert!(
            out.updated_at > updated,
            "a rename is a modification and must say so"
        );
    }

    /// The two halves [`super::split_djot`] cuts.
    fn split_djot(doc: &TextDocument, caret: usize) -> anyhow::Result<(String, String)> {
        super::split_djot(doc, caret).map(|split| (split.before, split.after))
    }

    /// A document holding `text`, one paragraph per line, as the writer typed it.
    fn typed(text: &str) -> TextDocument {
        let doc = TextDocument::new();
        doc.set_plain_text(text).expect("seed the document");
        doc
    }

    /// The text the editor shows for `djot`, one paragraph per line.
    fn shown(djot: &str) -> String {
        let doc = TextDocument::new();
        doc.set_djot_sync(djot).expect("the half reads back");
        doc.to_plain_text().expect("plain text")
    }

    /// Splitting mid-paragraph cuts at the caret, and the space at the cut goes with
    /// neither half, whichever side of the caret it sat on. `text-document` keeps a
    /// paragraph's edge blanks through a save since 1.12.3 (1.12.2 dropped them), so a
    /// space left in would open the new scene, or end the old one, for good.
    #[test]
    fn split_djot_cuts_at_the_caret() {
        let doc = typed("Hello world");

        // "Hello| world": the space is after the caret.
        let (before, after) = split_djot(&doc, 5).expect("split");
        assert_eq!((before.as_str(), after.as_str()), ("Hello", "world"));

        // "Hello |world": the space is before it.
        let (before, after) = split_djot(&doc, 6).expect("split");
        assert_eq!((before.as_str(), after.as_str()), ("Hello", "world"));
    }

    /// Every space and tab touching the caret goes, and nothing else: a no-break space is
    /// a character the writer chose, and blanks elsewhere in the text are theirs too.
    #[test]
    fn split_djot_drops_only_the_spaces_and_tabs_at_the_cut() {
        let doc = typed("Ends on two spaces  \nHello \t world");
        let caret = "Ends on two spaces  \nHello \t".chars().count() - 1;
        let (before, after) = split_djot(&doc, caret).expect("split");
        assert_eq!(shown(&before), "Ends on two spaces  \nHello");
        assert_eq!(shown(&after), "world");

        let doc = typed("Hello\u{a0}world");
        let (before, after) = split_djot(&doc, 6).expect("split");
        assert_eq!(shown(&before), "Hello\u{a0}");
        assert_eq!(shown(&after), "world");
    }

    /// At the very start of a paragraph the paragraph is not cut: it opens the new scene
    /// whole, and a tab it opens with is the writer's indentation, kept as typed. A caret
    /// inside that indentation cuts in front of the paragraph the same way.
    #[test]
    fn split_djot_at_a_paragraph_start_keeps_its_indentation() {
        let doc = typed("Hello\n\tWorld");
        for caret in [6, 7] {
            let (before, after) = split_djot(&doc, caret).expect("split");
            assert_eq!(shown(&before), "Hello", "caret {caret}");
            assert_eq!(shown(&after), "\tWorld", "caret {caret}");
        }
    }

    /// At the end of a paragraph the blanks it ends on are at the cut, so the old scene
    /// does not end on them, and the next paragraph opens the new scene as it is.
    #[test]
    fn split_djot_at_a_paragraph_end_leaves_its_trailing_blanks_behind() {
        let doc = typed("Hello  \n\tWorld");
        for caret in [5, 7] {
            let (before, after) = split_djot(&doc, caret).expect("split");
            assert_eq!(shown(&before), "Hello", "caret {caret}");
            assert_eq!(shown(&after), "\tWorld", "caret {caret}");
        }
    }

    /// A paragraph of nothing but blanks, with the caret anywhere in it, is all cut:
    /// the old scene ends on the paragraph before it and the new one opens on the next.
    #[test]
    fn split_djot_in_a_paragraph_of_blanks_leaves_it_out_of_both_halves() {
        let doc = typed("Hello\n \t \nWorld");
        for caret in [6, 7, 9] {
            let (before, after) = split_djot(&doc, caret).expect("split");
            assert_eq!(shown(&before), "Hello", "caret {caret}");
            assert_eq!(shown(&after), "World", "caret {caret}");
        }
    }

    /// The new scene gets the whole rest of the text. The end used to be read from
    /// `character_count()`, which counts no paragraph break, so every break in the scene
    /// cost the new scene one of its last characters.
    #[test]
    fn split_djot_keeps_the_last_words_of_a_scene_of_many_paragraphs() {
        let doc = typed("One\nTwo\nThree\nFour");
        let (before, after) = split_djot(&doc, 2).expect("split");
        assert_eq!(shown(&before), "On");
        assert_eq!(shown(&after), "e\nTwo\nThree\nFour");
    }

    /// Splitting at either boundary fails rather than yielding an empty half, as does a
    /// split with only blanks between the caret and that boundary. Callers rely on this:
    /// they treat `Err` as "do nothing", so such a split is a silent no-op.
    #[test]
    fn split_djot_refuses_a_boundary_split() {
        let doc = typed("Hello");
        assert!(split_djot(&doc, 0).is_err(), "nothing before the caret");
        assert!(split_djot(&doc, 5).is_err(), "nothing after the caret");
        // A caret past the end clamps to the end, so it fails the same way.
        assert!(split_djot(&doc, 9_999).is_err(), "clamped to the end");

        let doc = typed("  Hello  ");
        assert!(
            split_djot(&doc, 2).is_err(),
            "only indentation before the caret"
        );
        assert!(split_djot(&doc, 7).is_err(), "only blanks after the caret");

        let doc = typed("Hello\n   ");
        assert!(
            split_djot(&doc, 9).is_err(),
            "only a paragraph of blanks after the caret"
        );
    }

    /// A document holding `djot`, as the editor reads it.
    fn read(djot: &str) -> TextDocument {
        let doc = TextDocument::new();
        doc.set_djot_sync(djot).expect("seed the document");
        doc
    }

    /// A split with the caret in a table cuts in front of the table, which moves whole to
    /// the new scene with everything after it. A selection from inside a table takes the
    /// table whole and drops what follows it, so a split in a cell used to leave the table
    /// in both scenes and lose the text after it.
    #[test]
    fn split_djot_in_a_table_moves_the_table_whole_and_loses_nothing() {
        let table = "| a | b |\n|---|---|\n| c | d |";
        let doc = read(&format!("Before\n\n{table}\n\nAfter\n"));
        // 6 ends "Before", 7 is the table's anchor, 9 to 16 are its cells, 17 opens "After".
        for caret in 6..=16 {
            let halves = split_djot(&doc, caret).expect("split");
            assert_eq!(
                halves,
                ("Before".to_string(), format!("{table}\n\nAfter")),
                "caret {caret}"
            );
        }
        assert_eq!(
            split_djot(&doc, 17).expect("split"),
            (format!("Before\n\n{table}"), "After".to_string())
        );

        // A table opening the scene has nothing in front of it to leave behind.
        let doc = read(&format!("{table}\n\nAfter\n"));
        for caret in 0..=9 {
            assert!(split_djot(&doc, caret).is_err(), "caret {caret}");
        }
        assert_eq!(
            split_djot(&doc, 10).expect("split"),
            (table.to_string(), "After".to_string())
        );
    }

    /// A paragraph cut in two is two paragraphs of its kind, as Enter makes them, and one
    /// the caret ends keeps its kind: a heading, a list item, a quotation and a code block
    /// stay what they are on both halves. Taken as pieces of a paragraph, each came out a
    /// plain paragraph (a code block as a code span per line).
    #[test]
    fn split_djot_keeps_the_kind_of_the_paragraph_it_cuts() {
        for (djot, caret, before, after) in [
            ("# Title\n\nBody text\n", 2, "# Ti", "# tle\n\nBody text"),
            ("# Title\n\nBody text\n", 5, "# Title", "Body text"),
            (
                "- one\n- two\n- three\n",
                1,
                "- o",
                "- ne\n\n- two\n\n- three",
            ),
            ("- one\n- two\n- three\n", 3, "- one", "- two\n\n- three"),
            ("> one\n>\n> two\n", 1, "> o", "> ne\n>\n> two"),
            ("> one\n>\n> two\n", 3, "> one", "> two"),
            (
                "Text\n\n```\ncode one\ncode two\n```\n\nEnd\n",
                13,
                "Text\n\n```\ncode one\n```",
                "```\ncode two\n```\n\nEnd",
            ),
        ] {
            assert_eq!(
                split_djot(&read(djot), caret).expect("split"),
                (before.to_string(), after.to_string()),
                "{djot:?} at {caret}"
            );
        }
    }

    /// A half holding nothing but empty paragraphs, or paragraphs of blanks, is no scene:
    /// the caret at the end of a scene's last words, over the empty paragraph Enter left
    /// after them, or in front of its first words, under an empty paragraph, splits
    /// nothing. It used to make a new scene with no words in it, or empty the old one.
    #[test]
    fn split_djot_refuses_a_half_of_empty_paragraphs() {
        for (text, caret) in [
            ("Hello\n", 5),
            ("Hello\n   ", 5),
            ("\nHello", 1),
            ("   \nWorld", 4),
        ] {
            assert!(
                split_djot(&typed(text), caret).is_err(),
                "{text:?} at {caret}: {:?}",
                split_djot(&typed(text), caret)
            );
        }
    }

    /// The comments a split moves are the ones anchored at the moved text's first character
    /// or later: a comment over the cut stays where its first words are.
    #[test]
    fn a_split_moves_the_comments_anchored_in_the_moved_text() {
        use crate::comments::session::LiveAnchor;
        let anchor = |comment_id, start, end| LiveAnchor {
            comment_id,
            start,
            end,
            is_paragraph: false,
            resolved: false,
        };
        let live = [
            anchor(1, 0, 5),
            anchor(2, 8, 14),
            anchor(3, 10, 12),
            anchor(4, 20, 25),
        ];
        assert_eq!(comments_moved_by_split(&live, 10), vec![3, 4]);
        assert_eq!(comments_moved_by_split(&live, 0), vec![1, 2, 3, 4]);
        assert!(comments_moved_by_split(&live, 26).is_empty());
    }

    /// A comment goes with the words it was written on: split into a new scene, where it
    /// is found at the first place its anchoring looks, taken back by one undo with the
    /// split, and merged back into the scene it came from, where it is found again. It used
    /// to stay on the text it was cut from, where the next opening reported it had lost its
    /// words, and a merge left it on the scene the merge trashed.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_comment_goes_with_its_words_through_a_split_and_back_through_a_merge() {
        use frontend::binder_item_management::{MergeTwoScenesDto, SplitSceneDto};
        use teksilo::widgets::rich_text::RichTextEditor;

        let project = crate::test_support::RealProject::empty_novel();
        let ctx = project.app_ctx.clone();
        let ids = project.ids.clone();
        let stack = ids.stack_id.get();
        let docs = crate::models::OpenDocsStore::new(ctx.clone());
        docs.set_comments(crate::comments::CommentsViewModel::new(
            crate::models::CommentsListModel::new(ctx.clone(), ids.clone()),
            ctx.clone(),
            ids.stack_id.clone(),
        ));
        let (scene, _) = project.scenes()[0];
        let open = docs.open(scene).expect("the scene opens");
        let prose = open.main.as_ref().expect("a scene has prose");
        let handle = RichTextEditor::editor(prose.doc.clone()).handle();
        handle.insert_text("The house was quiet.");
        handle.insert_block();
        handle.insert_text("Then the lamp went out.");
        open.flush(stack).expect("the scene is written");
        let old_text = row_of(&ctx, scene, &ContentRole::SceneText).expect("its text");

        let text = prose.doc.to_addressable_text().expect("addressable text");
        let at = |word: &str| {
            let byte = text.find(word).expect("the word is in the scene");
            text[..byte].chars().count()
        };
        let binding = open
            .comment_binding_main()
            .expect("comments reach the scene");
        let on_house = binding
            .add_range(at("house"), at("house") + 5)
            .expect("a comment on the first paragraph");
        let on_lamp = binding
            .add_range(at("lamp"), at("lamp") + 4)
            .expect("a comment on the second");
        let comment = |id| {
            comment_commands::get_comment(&ctx, &id)
                .expect("reading a comment")
                .expect("the comment exists")
        };
        // What the anchoring reads a comment against: its text, as the editor reads it.
        let words_at = |djot: &str, id: u64| {
            let doc = TextDocument::new();
            doc.set_djot_sync(djot).expect("the text reads back");
            let text: Vec<char> = doc
                .to_addressable_text()
                .expect("addressable text")
                .chars()
                .collect();
            let row = comment(id);
            let start = row.range_start as usize;
            let end = start + row.range_length as usize;
            text.get(start..end)
                .map(|chars| chars.iter().collect::<String>())
        };

        // Split at the start of the second paragraph, as the stream's split does.
        let split = super::split_djot(&prose.doc, at("Then")).expect("split");
        let moved = comments_moved_by_split(&binding.live(), split.moved_from);
        assert_eq!(moved, vec![on_lamp]);
        split_scene_carrying_comments(
            &ctx,
            &ids,
            stack,
            &SplitSceneDto {
                source_id: scene,
                before_text: split.before.clone(),
                after_text: split.after.clone(),
                before_synopsis: String::new(),
                after_synopsis: String::new(),
                new_title: "Second".to_string(),
            },
            ContentRole::SceneText,
            &moved,
            &split,
        )
        .expect("the split");
        let (second, _) = project.scenes()[1];
        let new_text = row_of(&ctx, second, &ContentRole::SceneText).expect("the new text");
        assert_eq!(
            comment(on_lamp).content,
            Some(new_text),
            "moved with its words"
        );
        assert_eq!(words_at(&split.after, on_lamp).as_deref(), Some("lamp"));
        assert_eq!(
            comment(on_house).content,
            Some(old_text),
            "left with its own"
        );

        // One undo takes back the split and the move together.
        undo_redo_commands::undo(&ctx, stack).expect("undo");
        assert_eq!(project.scenes().len(), 1, "the split is undone");
        assert_eq!(
            comment(on_lamp).content,
            Some(old_text),
            "and the move with it"
        );
        undo_redo_commands::redo(&ctx, stack).expect("redo");
        assert_eq!(comment(on_lamp).content, Some(new_text));

        // Merged back, it comes home, found on its words in the merged text.
        merge_scenes_carrying_comments(
            &ctx,
            stack,
            &MergeTwoScenesDto {
                work_id: project.work_id,
                target_id: scene,
                source_id: second,
            },
        )
        .expect("the merge");
        assert_eq!(
            comment(on_lamp).content,
            Some(old_text),
            "back with its words"
        );
        let merged = content_commands::get_content(&ctx, &old_text)
            .expect("reading the merged text")
            .expect("the merged text")
            .data;
        assert_eq!(words_at(&merged, on_lamp).as_deref(), Some("lamp"));
        assert_eq!(words_at(&merged, on_house).as_deref(), Some("house"));
    }
}
