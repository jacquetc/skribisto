// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//
// Custom implementation — hand-maintained, do NOT blanket-regenerate.
//
//! `occurrences_for_result` — the individual hits inside **one** result row.
//!
//! A `SearchResult` is one matching *field* of one item, carrying how many times
//! the query occurs in it and a snippet cut around the first. That is what a
//! results list needs. A results *tree* opens a row and asks for the rest.
//!
//! ## Why the query is not an argument
//!
//! It is read from the `Search` the row belongs to. A caller passing its own is
//! exactly how the second level of a tree would come to contradict the first —
//! a badge saying nine and eleven rows underneath it. The row is checked to
//! belong to that same `Search` for the same reason.
//!
//! ## Why the offsets are not stored instead
//!
//! Because there can be an unbounded number of them. The row set is capped at
//! `RESULT_CAP` fields, but a field holds as many hits as the prose holds; a
//! one-letter query over a long manuscript is millions. Deriving them for the one
//! row a writer opened costs a single field's scan, and the parse and fold it
//! needs are already in the corpus cache from the search that produced the row.
//!
//! ## The matching is the same matching
//!
//! Same `MatchOptions`, same per-item locale resolved through the same
//! `language::tags_in_binder`, same `crate::matching`, same `crate::snippet::cut`.
//! Not by calling `run_search` — this codebase does not let a use case call a use
//! case — but by using the modules it uses. Anything less and the tree would
//! quote its matches differently from the list above it.
use crate::OccurrencesForResultDto;
use crate::OccurrencesForResultResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::search::SearchRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{
    Binder, BinderItem, Comment, CommentReply, Content, Footnote, MatchField, Search, SearchResult,
    Work, WorkInfo,
};
use common::types::EntityId;
use std::collections::HashMap;
use text_document::matching::{FoldLocale, MatchOptions};

use crate::corpus_cache;
use crate::field_text::FieldText;

/// Most occurrences one row will report.
///
/// A review budget, not a memory guard, and the number is chosen as one: past a
/// few hundred rows under a single scene nobody is reading them, they are being
/// scrolled past. The parent row keeps saying how many there really are, and
/// `truncated` says the list stops short — the writer is never told a smaller
/// number than the truth, only shown fewer of them.
///
/// Unticking the parent row still refuses **every** hit in the field, listed or
/// not, which is why that exclusion is stored per row rather than per occurrence.
const OCCURRENCE_CAP: usize = 500;

pub trait OccurrencesForResultUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn OccurrencesForResultUnitOfWorkTrait>;
}

// The action list is hand-trimmed to what the code below actually calls, exactly
// as `run_search_uow` records for itself: the generator re-renders the full generic
// Get/GetMulti/Snapshot/Restore set for every related entity, which would compile
// and then carry a dozen methods nobody calls. Snapshot/Restore are absent on
// purpose — nothing here writes.
//
// Reference (the generator's full vocabulary):
// Create, CreateMulti, Get, GetMulti, Update (scalar-only), UpdateMulti (scalar-only),
// UpdateWithRelationships, UpdateWithRelationshipsMulti,
// Remove, RemoveMulti, GetRelationship, GetRelationshipsFromRightIds,
// SetRelationship, SetRelationshipMulti
//
// You have here a read-write unit of work trait.
//
// RO means Read Only.
// Do not mix read-only and write actions in the same unit of work.
//
// Exactly the same macros must be set in the use case uow trait file in ../units_of_work/occurrences_for_result_uow.rs
//
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "WorkInfo", action = "GetAll")]
#[macros::uow_action(entity = "Search", action = "Get")]
#[macros::uow_action(entity = "Search", action = "GetRelationship")]
#[macros::uow_action(entity = "SearchResult", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "GetMulti")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "Get")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Comment", action = "Get")]
#[macros::uow_action(entity = "CommentReply", action = "Get")]
#[macros::uow_action(entity = "Footnote", action = "Get")]
pub trait OccurrencesForResultUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_occurrences_for_result_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct OccurrencesForResultUseCase {
    uow_factory: Box<dyn OccurrencesForResultUnitOfWorkFactoryTrait>,
}

impl OccurrencesForResultUseCase {
    pub fn new(uow_factory: Box<dyn OccurrencesForResultUnitOfWorkFactoryTrait>) -> Self {
        OccurrencesForResultUseCase { uow_factory }
    }

    pub fn execute(
        &mut self,
        dto: &OccurrencesForResultDto,
    ) -> Result<OccurrencesForResultResultDto> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let work_id = dto.work_id as EntityId;
        let work: Work = uow
            .get_all_work()?
            .into_iter()
            .find(|w| w.id == work_id)
            .ok_or_else(|| anyhow!("work {work_id} is not open"))?;
        let work_info: WorkInfo = uow
            .get_all_work_info()?
            .into_iter()
            .find(|wi| wi.work == Some(work_id))
            .ok_or_else(|| anyhow!("work {work_id} is not open"))?;
        let search: Search = uow
            .get_search(&work_info.search)?
            .ok_or_else(|| anyhow!("occurrences_for_result: WorkInfo has no Search"))?;

        let result_id = dto.result_id as EntityId;
        // The row must belong to *this* Work's Search. Without the check a caller
        // could open a row from another open project and be answered from this
        // one's prose, which would be wrong quietly rather than loudly.
        let rows =
            uow.get_search_relationship(&work_info.search, &SearchRelationshipField::Results)?;
        if !rows.contains(&result_id) {
            return Err(anyhow!(
                "occurrences_for_result: result {result_id} is not in work {work_id}'s search"
            ));
        }
        let row: SearchResult = uow
            .get_search_result(&result_id)?
            .ok_or_else(|| anyhow!("occurrences_for_result: no result {result_id}"))?;

        // An empty query produced no rows at all, so reaching here with one means the
        // row outlived its search. Nothing to report rather than an error: the panel
        // is mid-keystroke, not broken.
        if search.query.is_empty() {
            uow.commit()?;
            return Ok(OccurrencesForResultResultDto::default());
        }

        let locale = Self::locale_of(&mut uow, &work, row.binder_item_id)?;
        let options = MatchOptions {
            case_sensitive: search.case_sensitive,
            diacritic_sensitive: search.diacritic_sensitive,
            whole_word: search.whole_word,
            locale,
        };
        let text = Self::field_text(&mut uow, &row, &options)?;

        // Through `FieldText::hits`, which is the same call `run_search` made to
        // write this row's count and the same one `replace_in_project` makes to
        // resolve an offset back to a hit. The offsets below leave here, are shown,
        // are dismissed and are handed back to a replace — so they had better come
        // from the list every other reader of them is reading.
        let hits = text.hits(&search.query, options);
        let truncated = hits.len() > OCCURRENCE_CAP;

        let source = text.as_str();
        let mut char_starts = Vec::new();
        let mut char_lens = Vec::new();
        let mut snippets_before = Vec::new();
        let mut snippets_match = Vec::new();
        let mut snippets_after = Vec::new();
        for &(start, len) in hits.iter().take(OCCURRENCE_CAP) {
            let (before, matched, after) = crate::snippet::cut(source, start, len);
            char_starts.push(start as i64);
            char_lens.push(len as i64);
            snippets_before.push(before);
            snippets_match.push(matched);
            snippets_after.push(after);
        }

        uow.commit()?;
        uow.publish_occurrences_for_result_event(vec![result_id], None);

        Ok(OccurrencesForResultResultDto {
            char_starts,
            char_lens,
            snippets_before,
            snippets_match,
            snippets_after,
            truncated,
        })
    }

    /// The text of the field one result row stands for.
    ///
    /// Which text that is comes from `match_field`, and the mapping has to be the
    /// one `run_search` used to write the row: a row that reported `Synopsis` and is
    /// answered here from the body would put the tree's occurrences in a document
    /// the badge above them never counted.
    ///
    /// Prose goes through the corpus cache, which is where the row's own scan left
    /// it — the parse and the fold are already done and this is a lookup, not work.
    fn field_text(
        uow: &mut Box<dyn OccurrencesForResultUnitOfWorkTrait>,
        row: &SearchResult,
        options: &MatchOptions,
    ) -> Result<FieldText> {
        // The cache is keyed on what folding actually depends on, not on the whole
        // option set — the same derivation `run_search` hands it, so a field it
        // already folded is a hit here rather than a re-parse.
        let fold_spec = options.fold_spec();
        let item_id = row.binder_item_id;
        match row.match_field {
            // A title or a label lives on the item itself: a dozen characters, no
            // markup, folded on the spot.
            MatchField::Title | MatchField::Label => {
                let item: BinderItem = uow
                    .get_binder_item(&item_id)?
                    .ok_or_else(|| anyhow!("occurrences_for_result: no item {item_id}"))?;
                let text = match row.match_field {
                    MatchField::Title => item.title,
                    _ => item.label,
                };
                Ok(FieldText::Plain(text))
            }
            // Prose on a `Content` row of this item. Which row is decided by the same
            // role mapping that decided which field the result reported.
            MatchField::Body | MatchField::Epigraph | MatchField::Synopsis => {
                let content_ids = uow.get_binder_item_relationship(
                    &item_id,
                    &BinderItemRelationshipField::Contents,
                )?;
                let content: Content = uow
                    .get_content_multi(&content_ids)?
                    .into_iter()
                    .flatten()
                    .find(|c| {
                        crate::match_field::field_of_role(&c.role) == Some(row.match_field.clone())
                    })
                    .ok_or_else(|| {
                        anyhow!(
                            "occurrences_for_result: item {item_id} has no {:?} content",
                            row.match_field
                        )
                    })?;
                Ok(FieldText::Prose(corpus_cache::corpus_for(
                    &content.data,
                    &fold_spec,
                )))
            }
            // A comment thread's own body, a reply's, or a footnote's — each on its
            // own entity, each prose the writer typed and expects to find again.
            MatchField::Comment => {
                let id = row.comment_id;
                let comment: Comment = uow
                    .get_comment(&id)?
                    .ok_or_else(|| anyhow!("occurrences_for_result: no comment {id}"))?;
                Ok(FieldText::Prose(corpus_cache::corpus_for(
                    &comment.body,
                    &fold_spec,
                )))
            }
            MatchField::CommentReply => {
                let id = row.reply_id;
                let reply: CommentReply = uow
                    .get_comment_reply(&id)?
                    .ok_or_else(|| anyhow!("occurrences_for_result: no reply {id}"))?;
                Ok(FieldText::Prose(corpus_cache::corpus_for(
                    &reply.body,
                    &fold_spec,
                )))
            }
            MatchField::Footnote => {
                let id = row.footnote_id;
                let footnote: Footnote = uow
                    .get_footnote(&id)?
                    .ok_or_else(|| anyhow!("occurrences_for_result: no footnote {id}"))?;
                Ok(FieldText::Prose(corpus_cache::corpus_for(
                    &footnote.body,
                    &fold_spec,
                )))
            }
        }
    }

    /// The language one item's prose folds under.
    ///
    /// Resolved through `language::tags_in_binder` over the binder's **ordered**
    /// stream, exactly as `run_search` and `replace_in_project` resolve it: an
    /// item's tag, else the nearest Book's, else the Work's. Reading the whole
    /// item list to answer about one of them is not waste — inheritance is what is
    /// being resolved, and it cannot be answered from the item alone.
    fn locale_of(
        uow: &mut Box<dyn OccurrencesForResultUnitOfWorkTrait>,
        work: &Work,
        item_id: EntityId,
    ) -> Result<FoldLocale> {
        let binder_ids = uow.get_work_relationship(&work.id, &WorkRelationshipField::Binders)?;
        for binder in uow.get_binder_multi(&binder_ids)?.into_iter().flatten() {
            let item_ids =
                uow.get_binder_relationship(&binder.id, &BinderRelationshipField::BinderItems)?;
            if !item_ids.contains(&item_id) {
                continue;
            }
            let items: Vec<BinderItem> = uow
                .get_binder_item_multi(&item_ids)?
                .into_iter()
                .flatten()
                .collect();
            let mut tags: HashMap<EntityId, Vec<String>> = HashMap::new();
            crate::language::tags_in_binder(&work.dict_language, &items, &mut tags);
            return Ok(tags.get(&item_id).map_or(FoldLocale::Root, |tag| {
                FoldLocale::from_tag(crate::language::primary(tag))
            }));
        }
        // The item is not in any binder of this work. Untailored folding is the same
        // answer a missing tag gets, and for the same reason: a search must not fail
        // because a row is stale.
        Ok(FoldLocale::Root)
    }
}
