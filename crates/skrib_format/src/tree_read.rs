// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Read the open Work tree into one ordered, relationship-hydrated snapshot.
//!
//! Relocated out of `work_management::work_io` so it can be shared **without a
//! feature-to-feature dependency**: `save_work` / `save_as` / `backup_now` (in
//! `work_management`) and `export_work` (in `export_management`) each implement the thin
//! [`TreeReader`] over their own generated UoW trait, and one [`gather`] does the ordered
//! read + relationship hydration once. This is the same "shared crate, no feature
//! coupling" precedent as the rest of `skrib_format`.
//!
//! Export lists fewer entities than save (no `WorkInfo` / `TrashInfo` / `DictWord` /
//! `TextReplacementRule`), so [`TreeReader::all_work_info`], [`TreeReader::all_trash_info`],
//! [`TreeReader::dict_multi`] and [`TreeReader::text_replacement_rule_multi`] have
//! **default no-op** bodies — an implementor that has no such generated getter simply
//! omits them.
//!
//! [`TreeReader::note_template_multi`] is the one exception to that convenience: it is
//! **required**, so the analysis readers carry an explicit empty stub. See its own doc
//! for why a silent default would be dangerous rather than merely convenient here.

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Result, anyhow};
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::comment::CommentRelationshipField;
use common::direct_access::footnote::FootnoteRelationshipField;
use common::direct_access::pace::PaceRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::direct_access::work_info::WorkInfoRelationshipField;
use common::entities::{
    Asset, Binder, BinderItem, BinderTag, Comment, CommentReply, Content, DictWord, Footnote,
    Holiday, Milestone, NoteTemplate, Pace, ProgressSnapshot, SmartPunctuation,
    TextReplacementRule, TrashInfo, Work, WorkInfo,
};
use common::long_operation::OperationProgress;
use common::types::EntityId;

use crate::{
    BinderWithItems, CommentWithReplies, FootnoteWithContent, ItemWithContents, PaceWithChildren,
};

/// The read surface needed to snapshot the Work subtree. Implemented for each use case's
/// `dyn …UnitOfWorkTrait` (the generated method names are identical).
///
/// `all_work_info`/`all_trash_info`/`dict_multi`/`text_replacement_rule_multi` cover
/// entities that only *save* lists; an implementor that does not read them (e.g.
/// `export_work`) leaves them defaulted — likewise the Paces/Comments/ProgressSnapshots
/// groups further down, each gated by its own `reads_*` flag.
pub trait TreeReader {
    fn all_work(&self) -> Result<Vec<Work>>;
    fn all_work_info(&self) -> Result<Vec<WorkInfo>> {
        Ok(Vec::new())
    }
    fn all_trash_info(&self) -> Result<Vec<TrashInfo>> {
        Ok(Vec::new())
    }
    fn work_rel(&self, id: &EntityId, field: &WorkRelationshipField) -> Result<Vec<EntityId>>;
    fn binder_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Binder>>>;
    fn binder_rel(&self, id: &EntityId, field: &BinderRelationshipField) -> Result<Vec<EntityId>>;
    fn item_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderItem>>>;
    fn item_rel(&self, id: &EntityId, field: &BinderItemRelationshipField)
    -> Result<Vec<EntityId>>;
    fn tag_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderTag>>>;
    fn dict_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<DictWord>>> {
        Ok(Vec::new())
    }
    fn text_replacement_rule_multi(
        &self,
        _ids: &[EntityId],
    ) -> Result<Vec<Option<TextReplacementRule>>> {
        Ok(Vec::new())
    }
    /// The per-project note templates.
    ///
    /// **Required, deliberately undefaulted** — unlike its two neighbours above. A
    /// defaulted `Ok(Vec::new())` is exactly the shape of bug this feature is most
    /// exposed to: a save path that forgot to override it would compile, run, and
    /// silently write every project with zero templates, destroying them on the first
    /// save. Making it required turns that into a compile error, and costs the three
    /// analysis readers (export / mentions / word count) one explicit empty stub each.
    fn note_template_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<NoteTemplate>>>;
    /// Image metadata rows. **Required and undefaulted for the same reason as
    /// `note_template_multi`**, and the stakes are the same: a save path that
    /// forgot to override it would write every project with zero assets, and the
    /// exploded writer's prune would then delete every image file on disk.
    fn asset_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Asset>>>;
    /// The punctuation house style. Singular, not a `_multi`, because the
    /// relationship is one-to-one — there is exactly one row or none.
    ///
    /// Defaulted to `None` like the list readers above, so `export_work` (which
    /// has no reason to read settings) needs no implementation.
    fn smart_punctuation(&self, _id: &EntityId) -> Result<Option<SmartPunctuation>> {
        Ok(None)
    }
    fn content_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Content>>>;

    // ── Paces (save-only). Export never serialises the writing plan, so it leaves
    // `reads_paces` false and these getters defaulted. ──
    fn reads_paces(&self) -> bool {
        false
    }
    fn pace_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<Pace>>> {
        Ok(Vec::new())
    }
    fn pace_rel(&self, _id: &EntityId, _field: &PaceRelationshipField) -> Result<Vec<EntityId>> {
        Ok(Vec::new())
    }
    fn holiday_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<Holiday>>> {
        Ok(Vec::new())
    }
    fn milestone_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<Milestone>>> {
        Ok(Vec::new())
    }

    // ── Footnotes. **Required and undefaulted**, unlike the comment triad below.
    //
    // Comments may default to nothing because export must *not* carry them — a
    // working note has no place in the compiled manuscript, and leaving the flag
    // false makes that structural. A footnote is the opposite: it is prose that
    // belongs in the book, so every path must answer, and the export path most of
    // all. Defaulting it would let a reader silently write a project with zero
    // notes — and the exploded writer's prune would then delete every sidecar off
    // disk. Undefaulted, forgetting one is a compile error, which is the same
    // reason `asset_multi` above is undefaulted. ──
    fn footnote_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Footnote>>>;
    fn footnote_rel(
        &self,
        id: &EntityId,
        field: &FootnoteRelationshipField,
    ) -> Result<Vec<EntityId>>;

    // ── Comments. Defaulted off, so a reader that has no use for them cannot accidentally
    // pay for them; the save path and the export path both switch them on deliberately.
    //
    // The rule this used to state — "export never reads comments" — was right about the
    // manuscript and wrong about the file. A comment must still never reach the *compiled
    // document*: it is a working note, not prose, and Scrivener draws the same line. It does
    // now reach the exported `.docx`/`.odt`, as a **side payload** of anchored ranges the
    // writer attaches beside the text, because that is the editorial round trip — a draft goes
    // out for comment and comes back with the editor's remarks, which Skribisto reads.
    //
    // So the guarantee is unchanged where it matters and is still structural: nothing here
    // puts a comment into the prose. What changed is only that the export path now asks for
    // them. Which *formats* carry them is decided far from here, by
    // `ExportFormat::carries_comments` — DOCX and ODT, the two Skribisto can also read
    // comments back from. ──
    fn reads_comments(&self) -> bool {
        false
    }
    fn comment_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<Comment>>> {
        Ok(Vec::new())
    }
    fn comment_rel(
        &self,
        _id: &EntityId,
        _field: &CommentRelationshipField,
    ) -> Result<Vec<EntityId>> {
        Ok(Vec::new())
    }
    fn comment_reply_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<CommentReply>>> {
        Ok(Vec::new())
    }

    // ── ProgressSnapshots (save-only, and only when a WorkInfo is present). Export never
    // reads WorkInfo, so its `work_info` is None and these are never called. ──
    fn work_info_rel(
        &self,
        _id: &EntityId,
        _field: &WorkInfoRelationshipField,
    ) -> Result<Vec<EntityId>> {
        Ok(Vec::new())
    }
    fn progress_snapshot_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<ProgressSnapshot>>> {
        Ok(Vec::new())
    }
}

pub struct Gathered {
    pub work: Work,
    pub tags: Vec<BinderTag>,
    pub dict_words: Vec<DictWord>,
    pub text_replacement_rules: Vec<TextReplacementRule>,
    pub note_templates: Vec<NoteTemplate>,
    pub assets: Vec<Asset>,
    /// `None` when the reader does not read settings (export), or when the row
    /// genuinely does not resolve — never fabricated here, so the writer can
    /// record its absence faithfully.
    pub smart_punctuation: Option<SmartPunctuation>,
    pub trash_infos: Vec<TrashInfo>,
    pub paces: Vec<PaceWithChildren>,
    pub progress_snapshots: Vec<ProgressSnapshot>,
    /// Empty on the export path, which never reads comments (see `reads_comments`).
    pub comments: Vec<CommentWithReplies>,
    pub footnotes: Vec<FootnoteWithContent>,
    pub binders: Vec<BinderWithItems>,
    pub work_info: Option<WorkInfo>,
}

/// Read one open Work (its tags/dict-words/trash, and every binder → item →
/// content in order), hydrating each entity's relationship id vectors (which `get` does not
/// populate). Honours `cancel`; reports `progress`.
///
/// `work_id` reads exactly that Work — the fix for the "which Work?" bug
/// (Phase 0 of the multi-Work migration): with more than one Work open, picking
/// `all_work().next()` picked whichever row a `HashMap` happened to iterate
/// first, not the one the caller actually asked to save/export/back up/scan/
/// count. Phase 0.5 closed the last two holdouts
/// (`mention_management::scan_mentions`, `progress_management::count_words`)
/// by giving both a `work_id` of their own, so every caller now has a real id
/// to pass and the `Option` this parameter used to carry is gone.
pub fn gather<R: TreeReader + ?Sized>(
    reader: &R,
    work_id: EntityId,
    progress: &(dyn Fn(OperationProgress) + Send),
    cancel: &AtomicBool,
) -> Result<Gathered> {
    let mut work = reader
        .all_work()?
        .into_iter()
        .find(|w| w.id == work_id)
        .ok_or_else(|| anyhow!("work {work_id} is not open"))?;
    let work_id = work.id;
    // WorkInfo has no reverse `WorkRelationshipField` (it is a WEAK, one-way
    // `many_to_one` FROM WorkInfo — see `work_management::work_io`'s doc comment),
    // but it already carries the hydrated `.work` back-pointer, so a client-side
    // filter is correct and — bounded by "number of open Works" — cheap.
    let work_info = reader
        .all_work_info()?
        .into_iter()
        .find(|wi| wi.work == Some(work_id));

    work.tags = reader.work_rel(&work_id, &WorkRelationshipField::Tags)?;
    work.dict_words = reader.work_rel(&work_id, &WorkRelationshipField::DictWords)?;
    work.text_replacement_rules =
        reader.work_rel(&work_id, &WorkRelationshipField::TextReplacementRules)?;
    work.note_templates = reader.work_rel(&work_id, &WorkRelationshipField::NoteTemplates)?;
    work.assets = reader.work_rel(&work_id, &WorkRelationshipField::Assets)?;
    // A one-to-one relationship still comes back as a vector — take the first,
    // and treat an empty one as "no row", which is what a Work loaded from a
    // pre-feature bundle looks like before the materialiser heals it.
    work.smart_punctuation = reader
        .work_rel(&work_id, &WorkRelationshipField::SmartPunctuation)?
        .first()
        .copied()
        .unwrap_or(0);
    work.binders = reader.work_rel(&work_id, &WorkRelationshipField::Binders)?;
    // Unlike Tags/DictWords/Binders above, `all_trash_info()` has no by-id fetch
    // (it is the whole-store `get_all_trash_info()`, `TreeReader`'s only defaulted
    // Vec-returning read besides `all_work_info`) — so TrashInfo is scoped by
    // filtering that store-wide read down to exactly this Work's own ids, the
    // same relationship `Work.trash_infos` already exists for and every OTHER
    // TrashInfo consumer in this codebase (trash_management) walks correctly.
    work.trash_infos = reader.work_rel(&work_id, &WorkRelationshipField::TrashInfos)?;

    let tags = fetch_multi(&work.tags, |ids| reader.tag_multi(ids))?;
    let dict_words = fetch_multi(&work.dict_words, |ids| reader.dict_multi(ids))?;
    let text_replacement_rules = fetch_multi(&work.text_replacement_rules, |ids| {
        reader.text_replacement_rule_multi(ids)
    })?;
    let note_templates = fetch_multi(&work.note_templates, |ids| reader.note_template_multi(ids))?;
    let assets = fetch_multi(&work.assets, |ids| reader.asset_multi(ids))?;
    // Skip the read entirely for an unwired Work rather than asking for id 0,
    // which no store row can have.
    let smart_punctuation = if work.smart_punctuation == 0 {
        None
    } else {
        reader.smart_punctuation(&work.smart_punctuation)?
    };
    // `all_trash_info()` is store-wide and `HashMap`-backed, so its iteration
    // order is NOT stable across insertions into that same shared table (e.g.
    // another Work's TrashInfo rows being inserted when it is loaded into the
    // same store). Collecting straight off that iterator — even after
    // filtering down to this Work's own ids — let this Work's own
    // `content_fingerprint()` change purely because a second, unrelated Work
    // was opened in the same process, with this Work's actual content
    // unchanged. Building an id→entity lookup and then walking
    // `work.trash_infos` (the relationship vector above, whose order is
    // deterministic — the same idiom `tags`/`dict_words` already use via
    // `fetch_multi`) makes the result depend only on this Work's own
    // relationship order, never on the store's internal hash layout.
    let trash_lookup: std::collections::HashMap<EntityId, TrashInfo> = reader
        .all_trash_info()?
        .into_iter()
        .map(|t| (t.id, t))
        .collect();
    let trash_infos = work
        .trash_infos
        .iter()
        .filter_map(|id| trash_lookup.get(id).cloned())
        .collect();
    let paces = if reader.reads_paces() {
        hydrate_paces(reader, &work_id)?
    } else {
        Vec::new()
    };
    let comments = if reader.reads_comments() {
        hydrate_comments(reader, &work_id)?
    } else {
        Vec::new()
    };
    // Unconditional: every reader carries footnotes, including export.
    let footnotes = hydrate_footnotes(reader, &work_id)?;
    // The deliberate reach through WorkInfo: it is otherwise dropped before serialisation,
    // but its ProgressSnapshots must round-trip. Only save has a WorkInfo (export's is None).
    let progress_snapshots = match &work_info {
        Some(wi) => {
            let ids =
                reader.work_info_rel(&wi.id, &WorkInfoRelationshipField::ProgressSnapshots)?;
            fetch_multi(&ids, |ids| reader.progress_snapshot_multi(ids))?
        }
        None => Vec::new(),
    };

    let binder_entities = fetch_multi(&work.binders, |ids| reader.binder_multi(ids))?;
    let count = binder_entities.len().max(1);
    let mut binders = Vec::with_capacity(binder_entities.len());
    for (idx, mut binder) in binder_entities.into_iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err(anyhow!("operation cancelled"));
        }
        let item_ids = reader.binder_rel(&binder.id, &BinderRelationshipField::BinderItems)?;
        binder.binder_items = item_ids.clone();

        let item_entities = fetch_multi(&item_ids, |ids| reader.item_multi(ids))?;
        let mut items = Vec::with_capacity(item_entities.len());
        for mut item in item_entities {
            item.contents = reader.item_rel(&item.id, &BinderItemRelationshipField::Contents)?;
            item.references =
                reader.item_rel(&item.id, &BinderItemRelationshipField::References)?;
            item.point_of_view =
                reader.item_rel(&item.id, &BinderItemRelationshipField::PointOfView)?;
            item.tags = reader.item_rel(&item.id, &BinderItemRelationshipField::Tags)?;
            let contents = fetch_multi(&item.contents, |ids| reader.content_multi(ids))?;
            items.push(ItemWithContents { item, contents });
        }
        binders.push(BinderWithItems { binder, items });
        progress(OperationProgress::new(
            10.0 + 80.0 * (idx as f32 + 1.0) / count as f32,
            Some("Reading project…".to_string()),
        ));
    }

    Ok(Gathered {
        work,
        tags,
        dict_words,
        text_replacement_rules,
        note_templates,
        assets,
        smart_punctuation,
        trash_infos,
        paces,
        progress_snapshots,
        comments,
        footnotes,
        binders,
        work_info,
    })
}

/// Read `Work.comments` and each comment's ordered reply thread into
/// [`CommentWithReplies`]. Only called on the save path (`reads_comments()` true).
///
/// `comment.content` is hydrated here too: it is a weak `many_to_one`, so an empty
/// result means the anchored Content is gone — which is exactly what
/// `from_entities` turns into an entry in the bundle-root orphanage rather than a
/// dropped note.
/// Read `Work.footnotes` and each note's annotated Content.
///
/// No children to walk and no anchor to resolve — a footnote is its label and its
/// prose, and where it sits in the book is decided by the reference in the
/// document, not by anything stored here.
fn hydrate_footnotes<R: TreeReader + ?Sized>(
    reader: &R,
    work_id: &EntityId,
) -> Result<Vec<FootnoteWithContent>> {
    let ids = reader.work_rel(work_id, &WorkRelationshipField::Footnotes)?;
    let entities = fetch_multi(&ids, |ids| reader.footnote_multi(ids))?;
    let mut out = Vec::with_capacity(entities.len());
    for mut footnote in entities {
        footnote.content = reader
            .footnote_rel(&footnote.id, &FootnoteRelationshipField::Content)?
            .into_iter()
            .next();
        out.push(FootnoteWithContent { footnote });
    }
    Ok(out)
}

fn hydrate_comments<R: TreeReader + ?Sized>(
    reader: &R,
    work_id: &EntityId,
) -> Result<Vec<CommentWithReplies>> {
    let comment_ids = reader.work_rel(work_id, &WorkRelationshipField::Comments)?;
    let comment_entities = fetch_multi(&comment_ids, |ids| reader.comment_multi(ids))?;
    let mut comments = Vec::with_capacity(comment_entities.len());
    for mut comment in comment_entities {
        comment.content = reader
            .comment_rel(&comment.id, &CommentRelationshipField::Content)?
            .into_iter()
            .next();
        comment.replies = reader.comment_rel(&comment.id, &CommentRelationshipField::Replies)?;
        let replies = fetch_multi(&comment.replies, |ids| reader.comment_reply_multi(ids))?;
        comments.push(CommentWithReplies { comment, replies });
    }
    Ok(comments)
}

/// Read `Work.paces` and each pace's Holiday/Milestone children into ordered
/// [`PaceWithChildren`]. Only called on the save path (`reads_paces()` true).
fn hydrate_paces<R: TreeReader + ?Sized>(
    reader: &R,
    work_id: &EntityId,
) -> Result<Vec<PaceWithChildren>> {
    let pace_ids = reader.work_rel(work_id, &WorkRelationshipField::Paces)?;
    let pace_entities = fetch_multi(&pace_ids, |ids| reader.pace_multi(ids))?;
    let mut paces = Vec::with_capacity(pace_entities.len());
    for mut pace in pace_entities {
        pace.book_item = reader
            .pace_rel(&pace.id, &PaceRelationshipField::BookItem)?
            .into_iter()
            .next();
        pace.holidays = reader.pace_rel(&pace.id, &PaceRelationshipField::Holidays)?;
        pace.milestones = reader.pace_rel(&pace.id, &PaceRelationshipField::Milestones)?;
        let holidays = fetch_multi(&pace.holidays, |ids| reader.holiday_multi(ids))?;
        let milestones = fetch_multi(&pace.milestones, |ids| reader.milestone_multi(ids))?;
        paces.push(PaceWithChildren {
            pace,
            holidays,
            milestones,
        });
    }
    Ok(paces)
}

fn fetch_multi<T>(
    ids: &[EntityId],
    get: impl FnOnce(&[EntityId]) -> Result<Vec<Option<T>>>,
) -> Result<Vec<T>> {
    let fetched = get(ids)?;
    // NOTE: `zip` truncates to the shorter side, so a reader answering with
    // fewer rows than it was asked for yields a silently partial tree rather
    // than an error — which is how `export_work` shipped books with every image
    // missing for as long as its `asset_multi` was stubbed to `Ok(vec![])`.
    // Rejecting a length mismatch here does not work: the analysis readers
    // (export / mentions / word count) return exactly that empty vec on purpose
    // to opt out of `note_template_multi`, so the check would fail every export
    // of a project that owns a template. Telling a deliberate opt-out apart from
    // a failed read needs the trait to say which it is, not a length comparison.
    let mut out = Vec::with_capacity(fetched.len());
    for (id, opt) in ids.iter().zip(fetched) {
        out.push(opt.ok_or_else(|| anyhow!("entity {id} vanished mid-read"))?);
    }
    Ok(out)
}
