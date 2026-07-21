// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared binder plumbing for the item-editing view-models.
//!
//! [`OutlineViewModel`](super::OutlineViewModel), [`StreamViewModel`](super::StreamViewModel),
//! [`CorkboardViewModel`](super::CorkboardViewModel) and
//! [`OverviewViewModel`](super::OverviewViewModel) are four views over the *same* ordered
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

use bastyde::text_document::{MoveMode, TextDocument};

use frontend::AppContext;
use frontend::binder_item_management::{MoveDto, MovePlace};
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{BinderItemDto, CreateBinderItemDto, UpdateBinderItemDto};

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

/// `{id -> (indent, sub_role)}` for a binder's items — the data [`crate::binder::placement`]
/// walks to turn "put it after this one" into a concrete `(index, indent)`.
pub(crate) fn item_meta(app_ctx: &AppContext, order: &[u64]) -> ItemMeta {
    binder_item_commands::get_binder_item_multi(app_ctx, order)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .map(|it| (it.id, (it.indent, it.sub_role)))
        .collect()
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
    let anchor_indent = meta.get(&anchor_id).map(|(i, _)| *i).unwrap_or(0);
    let (index, indent) =
        placement::insertion_point_for_item(&order, &meta, pos, anchor_indent, rec.relation);

    let dto = CreateBinderItemDto {
        title: title.to_string(),
        role,
        sub_role,
        activated: true,
        is_exportable: true,
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

/// The content roles whose text `item_id` would **lose** by becoming `target` (empty rows
/// never count). Non-empty means the conversion must be refused: a chapter holding prose
/// cannot become a Part, which has nowhere to put it.
pub(crate) fn promote_content_loss(
    app_ctx: &AppContext,
    item_id: u64,
    target: skribisto_model::PromoteTarget,
) -> Vec<ContentRole> {
    let (target_role, target_sub_role) = target.combo();
    let content_ids = binder_item_commands::get_binder_item_relationship(
        app_ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap_or_default();
    let non_empty: Vec<ContentRole> = content_commands::get_content_multi(app_ctx, &content_ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
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
        Ok(items) => items.into_iter().flatten().filter(|it| it.activated).count(),
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
        updated_at: it.updated_at,
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
        indent: it.indent,
        word_count_goal: it.word_count_goal,
        char_count_goal: it.char_count_goal,
        dict_language: it.dict_language.clone(),
        aliases: it.aliases.clone(),
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

/// Split `doc` at char offset `caret` into two Djot strings, preserving inline formatting,
/// via fragment extraction into fresh documents.
///
/// **A split at either boundary returns `Err`, not an empty half.** `caret` is clamped into
/// range, but an empty selection produces an empty fragment, which `insert_fragment` rejects
/// ("Invalid fragment_data JSON") — so `caret == 0` and `caret >= len` both fail. Every
/// caller treats `Err` as "do nothing", which makes splitting at the very start or end a
/// silent no-op. That is the sane outcome (neither would produce two useful halves), but it
/// falls out of a serialization failure rather than a deliberate guard — so if a caller ever
/// needs to *distinguish* "nothing to split" from "the split failed", this is the place to
/// add an explicit boundary check rather than relying on the error.
pub(crate) fn split_djot(doc: &TextDocument, caret: usize) -> anyhow::Result<(String, String)> {
    let n = doc.character_count();
    let caret = caret.min(n);

    let extract = |from: usize, to: usize| -> anyhow::Result<String> {
        let c = doc.cursor();
        c.set_position(from, MoveMode::MoveAnchor);
        c.set_position(to, MoveMode::KeepAnchor);
        let frag = c.selection();
        let tmp = TextDocument::new();
        tmp.cursor().insert_fragment(&frag)?;
        Ok(tmp.to_djot()?)
    };

    let before = extract(0, caret)?;
    let after = extract(caret, n)?;
    Ok((before, after))
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
            is_exportable: false,
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
            tags: vec![44],
        };

        let out = update_item_dto(&src);

        assert_eq!(out.id, 7);
        assert_eq!(
            out.uid,
            common::uid::fixture_uid(7),
            "the durable identity must survive an edit unchanged -- re-minting              it here would orphan every reference to the row"
        );
        assert_eq!(out.created_at, created);
        assert_eq!(out.updated_at, updated);
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

    /// Splitting cuts exactly at the caret and loses nothing.
    #[test]
    fn split_djot_cuts_at_the_caret() {
        let doc = TextDocument::new();
        doc.cursor().insert_text("Hello world");

        let (before, after) = split_djot(&doc, 5).expect("split");
        assert_eq!(before.trim(), "Hello");
        assert_eq!(after.trim(), "world");
    }

    /// Splitting at either boundary fails rather than yielding an empty half — an empty
    /// fragment cannot be serialized. Callers rely on this: they treat `Err` as "do
    /// nothing", so a split at the very start or end is a silent no-op. Pinned here so the
    /// day someone makes empty fragments legal, the callers get revisited too.
    #[test]
    fn split_djot_refuses_a_boundary_split() {
        let doc = TextDocument::new();
        doc.cursor().insert_text("Hello");

        assert!(split_djot(&doc, 0).is_err(), "nothing before the caret");
        assert!(split_djot(&doc, 5).is_err(), "nothing after the caret");
        // A caret past the end clamps to the end, so it fails the same way.
        assert!(split_djot(&doc, 9_999).is_err(), "clamped to the end");
    }
}
