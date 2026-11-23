// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The open Work, read into the compiler's `Gathered` shape from the live store.

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, content_commands, footnote_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::footnote::FootnoteRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{Binder, BinderItem, Content, Footnote, Work};
use skrib_format::{BinderWithItems, FootnoteWithContent, Gathered, ItemWithContents};

/// Read the open Work subtree into a `Gathered` from the frontend read commands,
/// converting each DTO to its entity. The client analogue of the backend's frozen
/// `gather`, and what both the Export panel and a stream read as exported hand the
/// compiler, so the two compile the same tree the backend would.
///
/// `with_footnotes` reads the footnote bodies too. The Export panel needs them (its
/// orphan-note preflight reads that list); a reading of the stream does not, since it
/// shows each row's own editor rather than compiled text.
pub(crate) fn gather_work(
    ctx: &AppContext,
    work_id: u64,
    with_footnotes: bool,
) -> anyhow::Result<Gathered> {
    let work_dto =
        work_commands::get_work(ctx, &work_id)?.ok_or_else(|| anyhow::anyhow!("no open work"))?;
    let work: Work = work_dto.into();
    let binder_ids =
        work_commands::get_work_relationship(ctx, &work.id, &WorkRelationshipField::Binders)?;
    let binder_dtos = binder_commands::get_binder_multi(ctx, &binder_ids)?;
    let mut binders = Vec::new();
    for binder_dto in binder_dtos.into_iter().flatten() {
        let binder: Binder = binder_dto.into();
        let item_ids = binder_commands::get_binder_relationship(
            ctx,
            &binder.id,
            &BinderRelationshipField::BinderItems,
        )?;
        let item_dtos = binder_item_commands::get_binder_item_multi(ctx, &item_ids)?;
        let mut items = Vec::new();
        for item_dto in item_dtos.into_iter().flatten() {
            let item: BinderItem = item_dto.into();
            let content_ids = binder_item_commands::get_binder_item_relationship(
                ctx,
                &item.id,
                &BinderItemRelationshipField::Contents,
            )?;
            let contents: Vec<Content> = content_commands::get_content_multi(ctx, &content_ids)?
                .into_iter()
                .flatten()
                .map(Content::from)
                .collect();
            items.push(ItemWithContents { item, contents });
        }
        binders.push(BinderWithItems { binder, items });
    }
    // Footnotes DO reach the compiled manuscript (unlike comments), and the
    // orphan-footnote export preflight below reads exactly this list — an empty
    // one here means that warning can never fire, whatever the project holds.
    //
    // The DTO's own `content` scalar is not the relationship's source of truth
    // (see `footnote_numbering::read_work`'s sibling note and
    // `hydrate_footnotes` in `skrib_format::tree_read`), so it is re-read from
    // the relationship the same way both of those do.
    let mut footnotes = Vec::new();
    let footnote_ids = if with_footnotes {
        work_commands::get_work_relationship(ctx, &work.id, &WorkRelationshipField::Footnotes)?
    } else {
        Vec::new()
    };
    let footnote_dtos = footnote_commands::get_footnote_multi(ctx, &footnote_ids)?;
    for footnote_dto in footnote_dtos.into_iter().flatten() {
        let mut footnote: Footnote = footnote_dto.into();
        footnote.content = footnote_commands::get_footnote_relationship(
            ctx,
            &footnote.id,
            &FootnoteRelationshipField::Content,
        )?
        .into_iter()
        .next();
        footnotes.push(FootnoteWithContent { footnote });
    }
    Ok(Gathered {
        // Export writes no bundle, so it has no reason to read the ladder — the same
        // stance `status_multi` takes on the three analysis readers.
        statuses: Vec::new(),
        assets: Vec::new(),
        // Resources are research, not the book: an export carries none.
        resources: Vec::new(),
        work,
        tags: Vec::new(),
        dict_words: Vec::new(),
        text_replacement_rules: Vec::new(),
        // Templates are authoring furniture, never part of the compiled manuscript.
        note_templates: Vec::new(),
        // Punctuation settings shape prose as it is typed; by export time the
        // substitutions are already in the text, so there is nothing to read.
        smart_punctuation: None,
        trash_infos: Vec::new(),
        // Export only needs the item stream for scope resolution, not the writing plan
        // or the progress history.
        paces: Vec::new(),
        progress_snapshots: Vec::new(),
        // Comments are working notes and never reach a compiled manuscript, so
        // the export path has no reason to read them (see `TreeReader::reads_comments`).
        comments: Vec::new(),
        footnotes,
        binders,
        work_info: None,
    })
}
