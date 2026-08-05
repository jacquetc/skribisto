// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Pure transforms between store entities, the on-disk [`WorkBundle`], and the
//! neutral [`LoadedWork`] graph the materialiser consumes. No I/O here.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use common::entities::{
    Asset, Binder, BinderItem, BinderTag, ChapterMode, Content, DictWord, NoteTemplate,
    ProgressSnapshot, QuoteStyle, SmartPunctuation, TextReplacementRule, TrashInfo, Work,
};
use skribisto_model::content_allowed;
use std::collections::BTreeMap;

use super::bundle::*;
use super::loaded::*;
use super::media::{asset_relpath, extension_for};
use super::slug::{
    binder_dir_name, note_template_relpath, prose_file_name, prose_kind, prose_relpath,
};

fn fmt_dt(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(s)
        .with_context(|| format!("parsing datetime '{s}'"))?
        .with_timezone(&Utc))
}

/// The on-disk name of a [`QuoteStyle`].
///
/// Written as a string rather than relying on serde's enum encoding so that a
/// variant added by a later build cannot make the whole bundle unreadable to an
/// earlier one — the worst case degrades to [`quote_style_from_name`]'s fallback
/// instead of a hard deserialization error, which is the same posture every
/// other additive field in this crate takes.
fn quote_style_name(style: &QuoteStyle) -> &'static str {
    match style {
        QuoteStyle::LocaleDefault => "locale_default",
        QuoteStyle::CurlyDouble => "curly_double",
        QuoteStyle::Guillemets => "guillemets",
        QuoteStyle::LowHigh => "low_high",
    }
}

/// Read a [`QuoteStyle`] back, falling back to the locale default.
///
/// An unknown name means the bundle was written by a build that knows a style
/// this one does not. Falling back to `LocaleDefault` is the honest answer: it
/// is what the locale would have chosen anyway, so the prose stays typographically
/// sane rather than silently adopting some other house style.
fn quote_style_from_name(name: &str) -> QuoteStyle {
    match name {
        "curly_double" => QuoteStyle::CurlyDouble,
        "guillemets" => QuoteStyle::Guillemets,
        "low_high" => QuoteStyle::LowHigh,
        // Covers "locale_default", the empty string a `#[serde(default)]` yields
        // for a field written before this existed, and anything unrecognised.
        _ => QuoteStyle::LocaleDefault,
    }
}

// ---------------------------------------------------------------------------
// store entities -> WorkBundle (save path)
// ---------------------------------------------------------------------------

/// Build a [`WorkBundle`] from already-fetched, ordered store data.
///
/// Content rows are filtered through `skribisto_model::content_allowed`, so an
/// invalid `(role, sub_role, content_role)` triple can never be written; valid
/// rows are split into inline title contents vs `.djot` prose blobs.
#[allow(clippy::too_many_arguments)]
pub fn from_entities(
    work: &Work,
    tags: &[BinderTag],
    dict_words: &[DictWord],
    text_replacement_rules: &[TextReplacementRule],
    note_templates: &[NoteTemplate],
    assets: &[Asset],
    // Asset bytes keyed by content hash, read from the project's media
    // directory by the caller — this crate resolves no paths of its own.
    asset_bytes: BTreeMap<String, Vec<u8>>,
    // Deliberately **not** a slice, though every neighbour here is one: this is
    // a one-to-one child, and taking `&Option<_>` means a caller cannot pass it
    // in the wrong positional slot — the two `&[...]` parameters on either side
    // would have accepted each other silently.
    smart_punctuation: Option<&SmartPunctuation>,
    trash_infos: &[TrashInfo],
    paces: &[PaceWithChildren],
    progress_snapshots: &[ProgressSnapshot],
    comments: &[CommentWithReplies],
    binders: &[BinderWithItems],
    shape: ShapeTag,
) -> WorkBundle {
    // Bucket every comment by the Content it annotates, so each prose row's sidecar
    // can be assembled in one pass below. A comment whose `content` is `None` (its
    // target was purged) has no sidecar to live in and is collected separately into
    // the bundle-root orphanage — dropping it here would destroy the note.
    let mut comments_by_content: BTreeMap<u64, Vec<CommentFile>> = BTreeMap::new();
    let mut orphan_comments: Vec<CommentFile> = Vec::new();
    for cwr in comments {
        let file = comment_to_file(cwr);
        match cwr.comment.content {
            Some(content_id) => comments_by_content
                .entry(content_id)
                .or_default()
                .push(file),
            None => orphan_comments.push(file),
        }
    }

    let mut bundled_binders = Vec::with_capacity(binders.len());

    for (index, bwi) in binders.iter().enumerate() {
        let dir = binder_dir_name(index, &bwi.binder.name);
        let item_order: Vec<u64> = bwi.items.iter().map(|i| i.item.id).collect();

        let mut items = Vec::with_capacity(bwi.items.len());
        for iwc in &bwi.items {
            let item = &iwc.item;
            let mut inline_contents = Vec::new();
            let mut prose_refs = Vec::new();
            let mut prose = BTreeMap::new();
            let mut item_comments: BTreeMap<u64, Vec<CommentFile>> = BTreeMap::new();

            for c in &iwc.contents {
                if !content_allowed(&item.role, &item.sub_role, &c.role) {
                    continue; // invalid for this item — never serialise it
                }
                match prose_kind(&c.role) {
                    None => inline_contents.push(InlineContent {
                        file_id: c.id,
                        created_at: fmt_dt(&c.created_at),
                        updated_at: fmt_dt(&c.updated_at),
                        activated: c.activated,
                        role: c.role.clone(),
                        text: c.data.clone(),
                    }),
                    Some(_) => {
                        let name = prose_file_name(c.id, &item.title, &c.role)
                            .expect("prose_kind matched");
                        prose_refs.push(ProseRef {
                            file_id: c.id,
                            created_at: fmt_dt(&c.created_at),
                            updated_at: fmt_dt(&c.updated_at),
                            activated: c.activated,
                            role: c.role.clone(),
                            path: prose_relpath(&dir, &name),
                        });
                        prose.insert(c.id, c.data.clone());
                        // Only prose rows get a sidecar. A comment somehow attached
                        // to a title row (which the UI never creates — comments are
                        // prose-only by design) has nowhere to go and is treated as
                        // an orphan rather than silently dropped.
                        if let Some(list) = comments_by_content.remove(&c.id) {
                            item_comments.insert(c.id, list);
                        }
                    }
                }
            }

            items.push(BundledItem {
                item: BinderItemFile {
                    file_id: item.id,
                    uid: item.uid,
                    created_at: fmt_dt(&item.created_at),
                    updated_at: fmt_dt(&item.updated_at),
                    title: item.title.clone(),
                    sub_title: item.sub_title.clone(),
                    role: item.role.clone(),
                    sub_role: item.sub_role.clone(),
                    label: item.label.clone(),
                    activated: item.activated,
                    is_favorite: item.is_favorite,
                    is_exportable: item.is_exportable,
                    exclude_from_numbering: item.exclude_from_numbering,
                    indent: item.indent,
                    word_count_goal: item.word_count_goal,
                    char_count_goal: item.char_count_goal,
                    dict_language: item.dict_language.clone(),
                    aliases: item.aliases.clone(),
                    inline_contents,
                    prose_refs,
                    reference_ids: item.references.clone(),
                    point_of_view_ids: item.point_of_view.clone(),
                    tag_ids: item.tags.clone(),
                },
                prose,
                comments: item_comments,
            });
        }

        bundled_binders.push(BundledBinder {
            binder: BinderFile {
                file_id: bwi.binder.id,
                uid: bwi.binder.uid,
                created_at: fmt_dt(&bwi.binder.created_at),
                updated_at: fmt_dt(&bwi.binder.updated_at),
                name: bwi.binder.name.clone(),
                activated: bwi.binder.activated,
                item_order,
            },
            items,
        });
    }

    // Anything still bucketed here names a Content that was never written — it was
    // filtered out by `content_allowed`, it is a title row, or it simply is not in
    // this tree any more. Its comments have no sidecar, so they join the orphanage
    // rather than disappear at save time.
    for (_content_id, list) in std::mem::take(&mut comments_by_content) {
        orphan_comments.extend(list);
    }

    WorkBundle {
        manifest: ProjectManifest {
            format_version: FORMAT_VERSION,
            // Left unset on purpose: `folder_io::write_folder` computes and stamps the
            // read floor at the manifest commit, for every write path at once. Setting
            // a value here would be dead — overwritten a moment later — and would
            // suggest producers are each responsible for it, which is the arrangement
            // that lets one of them silently forget.
            format_min_read_version: None,
            shape,
            work: WorkFile {
                file_id: work.id,
                created_at: fmt_dt(&work.created_at),
                updated_at: fmt_dt(&work.updated_at),
                title: work.title.clone(),
                author_name: work.author_name.clone(),
                dict_language: work.dict_language.clone(),
                tag_ids: work.tags.clone(),
                dict_word_ids: work.dict_words.clone(),
                unique_id: work.unique_id.clone(),
                chapter_flat: matches!(work.chapter_mode, ChapterMode::Flat),
                text_replacement_rule_ids: work.text_replacement_rules.clone(),
                custom_replacement_rules_enabled: work.custom_replacement_rules_enabled,
                number_chapters: work.number_chapters,
                part_resets_chapter: work.part_resets_chapter,
                smart_punctuation: smart_punctuation.map(|sp| SmartPunctuationFile {
                    created_at: fmt_dt(&sp.created_at),
                    updated_at: fmt_dt(&sp.updated_at),
                    override_app_default: sp.override_app_default,
                    dashes: sp.dashes,
                    ellipsis: sp.ellipsis,
                    quotes: sp.quotes,
                    quote_style: quote_style_name(&sp.quote_style).to_string(),
                    pre_punctuation_spacing: sp.pre_punctuation_spacing,
                    dialogue_marker: sp.dialogue_marker,
                }),
            },
            binder_order: binders.iter().map(|b| b.binder.id).collect(),
            // A regular save. The backup path re-stamps these via `mark_as_backup`.
            kind: BundleKind::Regular,
            backup_of: None,
            backup_created_at: None,
        },
        tags: tags
            .iter()
            .map(|t| BinderTagFile {
                file_id: t.id,
                created_at: fmt_dt(&t.created_at),
                updated_at: fmt_dt(&t.updated_at),
                name: t.name.clone(),
                color: t.color.clone(),
                details: t.details.clone(),
                discoverable: t.discoverable,
            })
            .collect(),
        dict_words: dict_words
            .iter()
            .map(|w| DictWordFile {
                file_id: w.id,
                created_at: fmt_dt(&w.created_at),
                updated_at: fmt_dt(&w.updated_at),
                word: w.word.clone(),
            })
            .collect(),
        text_replacement_rules: text_replacement_rules
            .iter()
            .map(|r| TextReplacementRuleFile {
                file_id: r.id,
                created_at: fmt_dt(&r.created_at),
                updated_at: fmt_dt(&r.updated_at),
                trigger: r.trigger.clone(),
                replacement: r.replacement.clone(),
                enabled: r.enabled,
            })
            .collect(),
        // The manifest row carries only the metadata + the blob path; the body itself
        // goes in `note_template_bodies` and is written as a sibling `.djot`, so an
        // exploded project diffs a template edit as a prose change rather than as one
        // enormous re-quoted RON string.
        note_templates: note_templates
            .iter()
            .map(|t| NoteTemplateFile {
                file_id: t.id,
                created_at: fmt_dt(&t.created_at),
                updated_at: fmt_dt(&t.updated_at),
                name: t.name.clone(),
                starred: t.starred,
                path: note_template_relpath(t.id, &t.name),
            })
            .collect(),
        note_template_bodies: note_templates
            .iter()
            .map(|t| (t.id, t.body.clone()))
            .collect(),
        // Asset rows come from the store; their bytes come from the caller,
        // which read them out of the project's media directory. An asset with
        // no bytes supplied is dropped from the bundle rather than written as a
        // dangling row: `folder_io` would fail the whole save on a missing blob,
        // and losing one unreadable image is better than losing the save.
        assets: assets
            .iter()
            .filter(|a| asset_bytes.contains_key(&a.content_hash))
            .map(|a| AssetFile {
                file_id: a.id,
                created_at: fmt_dt(&a.created_at),
                updated_at: fmt_dt(&a.updated_at),
                content_hash: a.content_hash.clone(),
                file_name: a.file_name.clone(),
                mime_type: a.mime_type.clone(),
                width: a.width as u32,
                height: a.height as u32,
                byte_size: a.byte_size,
                alt: a.alt.clone(),
                is_cover: a.is_cover,
                path: asset_relpath(&a.content_hash, &extension_for(&a.mime_type)),
            })
            .collect(),
        asset_bytes,
        trash_infos: trash_infos
            .iter()
            .map(|ti| TrashInfoFile {
                file_id: ti.id,
                created_at: fmt_dt(&ti.created_at),
                updated_at: fmt_dt(&ti.updated_at),
                trashed_at: fmt_dt(&ti.trashed_at),
                origin_binder_id: ti.origin_binder_id,
                trashed_binder: ti.trashed_binder,
                trashed_binder_item: ti.trashed_binder_item,
            })
            .collect(),
        paces: paces
            .iter()
            .map(|pwc| PaceFile {
                file_id: pwc.pace.id,
                created_at: fmt_dt(&pwc.pace.created_at),
                updated_at: fmt_dt(&pwc.pace.updated_at),
                book_item: pwc.pace.book_item,
                start_date: fmt_dt(&pwc.pace.start_date),
                end_date: fmt_dt(&pwc.pace.end_date),
                weekday_mask: pwc.pace.weekday_mask,
                active: pwc.pace.active,
                holidays: pwc
                    .holidays
                    .iter()
                    .map(|h| HolidayFile {
                        file_id: h.id,
                        created_at: fmt_dt(&h.created_at),
                        updated_at: fmt_dt(&h.updated_at),
                        label: h.label.clone(),
                        start_date: fmt_dt(&h.start_date),
                        end_date: h.end_date.as_ref().map(fmt_dt),
                    })
                    .collect(),
                milestones: pwc
                    .milestones
                    .iter()
                    .map(|ms| MilestoneFile {
                        file_id: ms.id,
                        created_at: fmt_dt(&ms.created_at),
                        updated_at: fmt_dt(&ms.updated_at),
                        label: ms.label.clone(),
                        target_item: ms.target_item,
                        target_date: fmt_dt(&ms.target_date),
                        target_word_count: ms.target_word_count,
                    })
                    .collect(),
            })
            .collect(),
        progress_snapshots: progress_snapshots
            .iter()
            .map(|s| ProgressSnapshotFile {
                file_id: s.id,
                created_at: fmt_dt(&s.created_at),
                updated_at: fmt_dt(&s.updated_at),
                day: fmt_dt(&s.day),
                total_word_count: s.total_word_count,
                total_char_count: s.total_char_count,
                book_item_ids: s.book_item_ids.clone(),
                book_word_counts: s.book_word_counts.clone(),
            })
            .collect(),
        orphan_comments,
        binders: bundled_binders,
    }
}

/// One store `Comment` (plus its ordered replies) as its on-disk row.
///
/// The `content` link is deliberately absent from [`CommentFile`]: for a live
/// comment the sidecar's own location names the Content, and for an orphan the id
/// would be meaningless anyway, since every `EntityId` is re-minted on the next
/// load. What survives is the quote selector, which is what re-anchoring actually
/// uses.
fn comment_to_file(cwr: &CommentWithReplies) -> CommentFile {
    let c = &cwr.comment;
    CommentFile {
        file_id: c.id,
        created_at: fmt_dt(&c.created_at),
        updated_at: fmt_dt(&c.updated_at),
        kind: c.kind.clone(),
        author_name: c.author_name.clone(),
        body: c.body.clone(),
        resolved: c.resolved,
        orphaned: c.orphaned,
        orphan_reason: c.orphan_reason.clone(),
        range_start: c.range_start,
        range_length: c.range_length,
        quote_prefix: c.quote_prefix.clone(),
        quote_exact: c.quote_exact.clone(),
        quote_exact_truncated: c.quote_exact_truncated,
        quote_suffix: c.quote_suffix.clone(),
        block_ordinal_hint: c.block_ordinal_hint,
        replies: cwr
            .replies
            .iter()
            .map(|r| CommentReplyFile {
                file_id: r.id,
                created_at: fmt_dt(&r.created_at),
                updated_at: fmt_dt(&r.updated_at),
                author_name: r.author_name.clone(),
                body: r.body.clone(),
            })
            .collect(),
    }
}

/// Stamp a freshly-built [`WorkBundle`] as a point-in-time backup copy.
///
/// Called only from the backup path (never `save_work`/`save_as`), so the shared
/// [`from_entities`] stays backup-agnostic. Call this **after** computing the
/// content fingerprint — otherwise `backup_created_at` makes every backup's
/// fingerprint unique and defeats skip-if-unchanged.
pub fn mark_as_backup(bundle: &mut WorkBundle, backup_of: String, created_at: DateTime<Utc>) {
    bundle.manifest.kind = BundleKind::Backup;
    bundle.manifest.backup_of = Some(backup_of);
    bundle.manifest.backup_created_at = Some(created_at.to_rfc3339());
}

// ---------------------------------------------------------------------------
// WorkBundle -> neutral LoadedWork (load path)
// ---------------------------------------------------------------------------

/// Turn a parsed [`WorkBundle`] into the entity-typed [`LoadedWork`] graph the
/// materialiser consumes. Ids stay as file ids (remapped on materialise).
pub fn bundle_to_loaded(bundle: WorkBundle, absolute_path: &str) -> Result<LoadedWork> {
    let m = &bundle.manifest;

    let work = Work {
        id: m.work.file_id,
        created_at: parse_dt(&m.work.created_at)?,
        updated_at: parse_dt(&m.work.updated_at)?,
        title: m.work.title.clone(),
        author_name: m.work.author_name.clone(),
        dict_language: m.work.dict_language.clone(),
        // Empty for pre-v2 bundles; healed (freshly minted) in `materialize`.
        unique_id: m.work.unique_id.clone(),
        chapter_mode: if m.work.chapter_flat {
            ChapterMode::Flat
        } else {
            ChapterMode::Folder
        },
        custom_replacement_rules_enabled: m.work.custom_replacement_rules_enabled,
        // `true` for a bundle written before the field existed — see `WorkFile`'s
        // `default_true`, the one non-`false` legacy default in this format.
        number_chapters: m.work.number_chapters,
        part_resets_chapter: m.work.part_resets_chapter,
        // Empty by design: `LoadedWork` carries the children in its own ordered
        // vectors, and `materialize` wires the real store ids on afterwards.
        binders: Vec::new(),
        tags: Vec::new(),
        dict_words: Vec::new(),
        text_replacement_rules: Vec::new(),
        note_templates: Vec::new(),
        assets: Vec::new(),
        // Zero for the same reason the vectors are empty — `materialize` mints
        // the row and writes its store id back. Unlike them, zero is not a
        // valid resting state: a `Work` whose one-to-one child is still 0 has a
        // dangling relationship, so `materialize` must create a default row for
        // a bundle that carries none (every bundle written before this field
        // existed).
        smart_punctuation: 0,
        trash_infos: Vec::new(),
        paces: Vec::new(),
        comments: Vec::new(),
    };

    let tags = bundle
        .tags
        .iter()
        .map(|t| {
            Ok(BinderTag {
                id: t.file_id,
                created_at: parse_dt(&t.created_at)?,
                updated_at: parse_dt(&t.updated_at)?,
                name: t.name.clone(),
                color: t.color.clone(),
                details: t.details.clone(),
                discoverable: t.discoverable,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let dict_words = bundle
        .dict_words
        .iter()
        .map(|w| {
            Ok(DictWord {
                id: w.file_id,
                created_at: parse_dt(&w.created_at)?,
                updated_at: parse_dt(&w.updated_at)?,
                word: w.word.clone(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let text_replacement_rules = bundle
        .text_replacement_rules
        .iter()
        .map(|r| {
            Ok(TextReplacementRule {
                id: r.file_id,
                created_at: parse_dt(&r.created_at)?,
                updated_at: parse_dt(&r.updated_at)?,
                trigger: r.trigger.clone(),
                replacement: r.replacement.clone(),
                enabled: r.enabled,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // The body comes from the blob map, not the manifest row.
    //
    // A listed row with no blob is a **hard error**, exactly as a missing prose blob is,
    // and deliberately not a silent empty body. Degrading quietly here would be the worse
    // failure by far: the writer opens a project whose templates look blank, autosave
    // rewrites `templates.ron` from that empty state moments later, and the text is gone
    // for good. Failing the load leaves every byte on disk and is recoverable.
    let note_templates = bundle
        .note_templates
        .iter()
        .map(|t| {
            Ok(NoteTemplate {
                id: t.file_id,
                created_at: parse_dt(&t.created_at)?,
                updated_at: parse_dt(&t.updated_at)?,
                name: t.name.clone(),
                body: bundle
                    .note_template_bodies
                    .get(&t.file_id)
                    .cloned()
                    .ok_or_else(|| {
                        anyhow::anyhow!("missing body blob for note template {}", t.file_id)
                    })?,
                starred: t.starred,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let assets = bundle
        .assets
        .iter()
        .map(|a| {
            Ok(Asset {
                id: a.file_id,
                created_at: parse_dt(&a.created_at)?,
                updated_at: parse_dt(&a.updated_at)?,
                content_hash: a.content_hash.clone(),
                file_name: a.file_name.clone(),
                mime_type: a.mime_type.clone(),
                width: u64::from(a.width),
                height: u64::from(a.height),
                byte_size: a.byte_size,
                alt: a.alt.clone(),
                is_cover: a.is_cover,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut references: Vec<(u64, u64)> = Vec::new();
    let mut point_of_view: Vec<(u64, u64)> = Vec::new();
    let mut loaded_binders = Vec::with_capacity(bundle.binders.len());

    for bb in &bundle.binders {
        let binder = Binder {
            id: bb.binder.file_id,
            // An empty uid here would collapse every row onto one key downstream.
            uid: bb.binder.uid,
            created_at: parse_dt(&bb.binder.created_at)?,
            updated_at: parse_dt(&bb.binder.updated_at)?,
            name: bb.binder.name.clone(),
            activated: bb.binder.activated,
            // Empty by design: `LoadedBinder.items` carries the order.
            binder_items: Vec::new(),
        };

        let mut items = Vec::with_capacity(bb.items.len());
        for bi in &bb.items {
            let f = &bi.item;
            let mut contents: Vec<Content> = Vec::new();
            for ic in &f.inline_contents {
                contents.push(Content {
                    id: ic.file_id,
                    created_at: parse_dt(&ic.created_at)?,
                    updated_at: parse_dt(&ic.updated_at)?,
                    activated: ic.activated,
                    role: ic.role.clone(),
                    data: ic.text.clone(),
                });
            }
            for pr in &f.prose_refs {
                let data = bi.prose.get(&pr.file_id).cloned().ok_or_else(|| {
                    anyhow::anyhow!(
                        "missing prose blob for content {} ({})",
                        pr.file_id,
                        pr.path
                    )
                })?;
                contents.push(Content {
                    id: pr.file_id,
                    created_at: parse_dt(&pr.created_at)?,
                    updated_at: parse_dt(&pr.updated_at)?,
                    activated: pr.activated,
                    role: pr.role.clone(),
                    data,
                });
            }

            for dst in &f.reference_ids {
                references.push((f.file_id, *dst));
            }
            for dst in &f.point_of_view_ids {
                point_of_view.push((f.file_id, *dst));
            }

            items.push(LoadedItem {
                item: BinderItem {
                    id: f.file_id,
                    // See the binder above.
                    uid: f.uid,
                    created_at: parse_dt(&f.created_at)?,
                    updated_at: parse_dt(&f.updated_at)?,
                    title: f.title.clone(),
                    sub_title: f.sub_title.clone(),
                    role: f.role.clone(),
                    sub_role: f.sub_role.clone(),
                    label: f.label.clone(),
                    activated: f.activated,
                    is_favorite: f.is_favorite,
                    is_exportable: f.is_exportable,
                    exclude_from_numbering: f.exclude_from_numbering,
                    indent: f.indent,
                    word_count_goal: f.word_count_goal,
                    char_count_goal: f.char_count_goal,
                    dict_language: f.dict_language.clone(),
                    aliases: f.aliases.clone(),
                    // Empty by design: `LoadedItem` carries the contents and the tag
                    // ids, and cross-item `references` are collected separately above.
                    contents: Vec::new(),
                    references: Vec::new(),
                    point_of_view: Vec::new(),
                    tags: Vec::new(),
                },
                contents,
                tag_ids: f.tag_ids.clone(),
            });
        }

        loaded_binders.push(LoadedBinder { binder, items });
    }

    let trash_infos = bundle
        .trash_infos
        .iter()
        .map(|ti| {
            Ok(LoadedTrash {
                created_at: parse_dt(&ti.created_at)?,
                updated_at: parse_dt(&ti.updated_at)?,
                trashed_at: parse_dt(&ti.trashed_at)?,
                origin_binder_id: ti.origin_binder_id,
                trashed_binder: ti.trashed_binder,
                trashed_binder_item: ti.trashed_binder_item,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // Ids (`book_item` / `target_item`) stay as file ids here; the materialiser remaps
    // them against the same `item_map` it uses for trash-info back-links.
    let paces = bundle
        .paces
        .iter()
        .map(|p| {
            Ok(LoadedPace {
                created_at: parse_dt(&p.created_at)?,
                updated_at: parse_dt(&p.updated_at)?,
                book_item: p.book_item,
                start_date: parse_dt(&p.start_date)?,
                end_date: parse_dt(&p.end_date)?,
                weekday_mask: p.weekday_mask,
                active: p.active,
                holidays: p
                    .holidays
                    .iter()
                    .map(|h| {
                        Ok(LoadedHoliday {
                            created_at: parse_dt(&h.created_at)?,
                            updated_at: parse_dt(&h.updated_at)?,
                            label: h.label.clone(),
                            start_date: parse_dt(&h.start_date)?,
                            end_date: h.end_date.as_deref().map(parse_dt).transpose()?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
                milestones: p
                    .milestones
                    .iter()
                    .map(|ms| {
                        Ok(LoadedMilestone {
                            created_at: parse_dt(&ms.created_at)?,
                            updated_at: parse_dt(&ms.updated_at)?,
                            label: ms.label.clone(),
                            target_item: ms.target_item,
                            target_date: parse_dt(&ms.target_date)?,
                            target_word_count: ms.target_word_count,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // `book_item_ids` stay as file ids here; the materialiser remaps them against
    // `item_map` (index-paired with `book_word_counts`, dropping unresolvable ids in
    // lockstep) — same posture as the pace/trash back-links.
    let progress_snapshots = bundle
        .progress_snapshots
        .iter()
        .map(|s| {
            Ok(LoadedProgressSnapshot {
                created_at: parse_dt(&s.created_at)?,
                updated_at: parse_dt(&s.updated_at)?,
                day: parse_dt(&s.day)?,
                total_word_count: s.total_word_count,
                total_char_count: s.total_char_count,
                book_item_ids: s.book_item_ids.clone(),
                book_word_counts: s.book_word_counts.clone(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // `None` here flows all the way to the materialiser, which is what lets it
    // tell a pre-feature bundle from one whose writer switched everything off.
    let smart_punctuation = m
        .work
        .smart_punctuation
        .as_ref()
        .map(|sp| -> Result<SmartPunctuation> {
            Ok(SmartPunctuation {
                // Remapped to a fresh store id at materialise time, like every
                // other file id in this graph.
                id: 0,
                created_at: parse_dt(&sp.created_at)?,
                updated_at: parse_dt(&sp.updated_at)?,
                override_app_default: sp.override_app_default,
                dashes: sp.dashes,
                ellipsis: sp.ellipsis,
                quotes: sp.quotes,
                quote_style: quote_style_from_name(&sp.quote_style),
                pre_punctuation_spacing: sp.pre_punctuation_spacing,
                dialogue_marker: sp.dialogue_marker,
            })
        })
        .transpose()?;

    // Comments arrive from two places and are flattened into one list here, each
    // remembering the content file id it annotates. The per-Content sidecars supply
    // the anchored ones; the bundle-root orphanage supplies those whose Content is
    // already gone (`content: None`) — kept rather than dropped, so an orphan the
    // writer has not dealt with yet survives a save/load cycle intact.
    let mut comments: Vec<LoadedComment> = Vec::new();
    for bb in &bundle.binders {
        for bi in &bb.items {
            for (content_file_id, list) in &bi.comments {
                for cf in list {
                    comments.push(comment_from_file(cf, Some(*content_file_id))?);
                }
            }
        }
    }
    for cf in &bundle.orphan_comments {
        comments.push(comment_from_file(cf, None)?);
    }

    Ok(LoadedWork {
        assets,
        work,
        tags,
        dict_words,
        text_replacement_rules,
        note_templates,
        smart_punctuation,
        binders: loaded_binders,
        trash_infos,
        paces,
        progress_snapshots,
        comments,
        references,
        point_of_view,
        absolute_path: absolute_path.to_string(),
    })
}

/// One on-disk [`CommentFile`] as a [`LoadedComment`], carrying the content **file
/// id** it annotates (or `None` for an orphan). Ids stay as file ids here; the
/// materialiser remaps them, exactly as it does for pace/trash back-links.
fn comment_from_file(cf: &CommentFile, content: Option<u64>) -> Result<LoadedComment> {
    Ok(LoadedComment {
        created_at: parse_dt(&cf.created_at)?,
        updated_at: parse_dt(&cf.updated_at)?,
        content,
        kind: cf.kind.clone(),
        author_name: cf.author_name.clone(),
        body: cf.body.clone(),
        resolved: cf.resolved,
        orphaned: cf.orphaned,
        orphan_reason: cf.orphan_reason.clone(),
        range_start: cf.range_start,
        range_length: cf.range_length,
        quote_prefix: cf.quote_prefix.clone(),
        quote_exact: cf.quote_exact.clone(),
        quote_exact_truncated: cf.quote_exact_truncated,
        quote_suffix: cf.quote_suffix.clone(),
        block_ordinal_hint: cf.block_ordinal_hint,
        replies: cf
            .replies
            .iter()
            .map(|r| {
                Ok(LoadedCommentReply {
                    created_at: parse_dt(&r.created_at)?,
                    updated_at: parse_dt(&r.updated_at)?,
                    author_name: r.author_name.clone(),
                    body: r.body.clone(),
                })
            })
            .collect::<Result<Vec<_>>>()?,
    })
}
