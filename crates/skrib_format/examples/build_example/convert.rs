// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `convert` — a plain-text public-domain novel plus its TOML becomes a `.skrib`.
//!
//! Produces the manuscript and nothing else: binder, book folder, one
//! `Item/ChapterScene` per chapter with its prose and footnotes, and the paratext
//! pages. Synopses, the story bible and every relationship are `enrich`'s job, so
//! that the two bundled examples share one code path for the half they have in
//! common — Starforgers arrives as a bundle already and never runs this.

use std::collections::BTreeMap;

use anyhow::{Context, Result, ensure};
use common::entities::{
    BinderItemRole as Role, BinderItemSubRole as SubRole, ContentRole, GoalUnit,
};
use skrib_format::{
    BinderFile, BinderItemFile, BinderTagFile, BundleKind, BundledBinder, BundledItem,
    FORMAT_VERSION, FootnoteFile, ProjectManifest, ProseRef, ShapeTag, SkribShape,
    SmartPunctuationFile, WorkBundle, WorkFile, binder_dir_name, markdown_to_djot, prose_file_name,
    prose_relpath,
};

use skribisto_model::scene_break;

use crate::source_text::{self, RawChapter};
use crate::spec::{Spec, uid_for};

/// Hands out the sequential `file_id`s the format keys rows by.
#[derive(Default)]
struct Ids {
    item: u64,
    content: u64,
    footnote: u64,
}

impl Ids {
    fn item(&mut self) -> u64 {
        self.item += 1;
        self.item
    }
    fn content(&mut self) -> u64 {
        self.content += 1;
        self.content
    }
    fn footnote(&mut self) -> u64 {
        self.footnote += 1;
        self.footnote
    }
}

pub fn run(spec_path: &str, source_path: &str, out_path: &str) -> Result<()> {
    let spec = Spec::load(spec_path)?;
    let source = std::fs::read_to_string(source_path)
        .with_context(|| format!("reading the source text {source_path}"))?;

    let chapters = source_text::parse(&source, &spec.source)?;
    check_titles(&spec, &chapters)?;

    let bundle = build(&spec, &chapters)?;
    write(out_path, &bundle)?;

    let words: usize = chapters
        .iter()
        .flat_map(|c| &c.paragraphs)
        .map(|p| p.split_whitespace().count())
        .sum();
    println!(
        "convert: {out_path} — {} chapters, {words} words, {} paratext pages, {} tags",
        chapters.len(),
        spec.paratext.len(),
        spec.tag.len()
    );
    Ok(())
}

/// Every chapter the source yielded must be the one the TOML says it is.
///
/// This is the check that makes the parser's "the heading is everything up to the
/// first blank line" rule safe. Comparing on letters and digits alone lets the
/// TOML hold the title as a reader should see it — mixed case, with the emphasis
/// markers around a ship's name removed — while still catching a heading block that
/// swallowed a paragraph or a chapter that landed one slot out.
fn check_titles(spec: &Spec, chapters: &[RawChapter]) -> Result<()> {
    ensure!(
        spec.chapter.len() == chapters.len(),
        "the source has {} chapters but the TOML declares {}",
        chapters.len(),
        spec.chapter.len()
    );
    for (i, raw) in chapters.iter().enumerate() {
        let declared = &spec.chapter[i];
        ensure!(
            declared.number == Some(raw.number),
            "[[chapter]] #{} declares number {:?}, but it is chapter {} of the source",
            i + 1,
            declared.number,
            raw.number
        );
        let (found, want) = (letters(&raw.heading), letters(&declared.title));
        ensure!(
            found == want,
            "chapter {} heading mismatch — the source reads\n  {}\nbut the TOML declares\n  {}",
            raw.number,
            raw.heading,
            declared.title
        );
    }
    Ok(())
}

/// Letters and digits, folded to lower case and stripped of their accents.
///
/// The accent folding is not laxness, it is the period's typography: Hetzel sets
/// chapter headings in capitals, and French capitals of 1873 carry no accents, so
/// the heading above chapter II reads `OU PASSEPARTOUT…` where the volume's own
/// table of contents reads `Où Passepartout…`. Comparing the two literally would
/// reject the book. NFD splits each accented letter into its base plus a combining
/// mark, and a combining mark is not alphanumeric, so the filter below removes it.
fn letters(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    s.nfd()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn build(spec: &Spec, chapters: &[RawChapter]) -> Result<WorkBundle> {
    let project = &spec.book.unique_id;
    let now = spec.book.timestamp.clone();
    let mut ids = Ids::default();

    let tags: Vec<BinderTagFile> = spec
        .tag
        .iter()
        .enumerate()
        .map(|(i, t)| BinderTagFile {
            file_id: i as u64 + 1,
            uid: uid_for(project, "tag", &t.name),
            created_at: now.clone(),
            updated_at: now.clone(),
            name: t.name.clone(),
            color: t.color.clone(),
            details: t.details.clone(),
            discoverable: t.discoverable,
            creates_in: None,
            note_template: None,
        })
        .collect();

    let binder_dir = binder_dir_name(0, &spec.book.manuscript_binder);
    let mut items: Vec<BundledItem> = Vec::new();

    // ── front matter ──────────────────────────────────────────────────────────
    let section =
        |items: &mut Vec<BundledItem>, ids: &mut Ids, which: &str, folder: &str| -> Result<()> {
            let pages: Vec<_> = spec
                .paratext
                .iter()
                .filter(|p| p.section == which)
                .collect();
            if pages.is_empty() {
                return Ok(());
            }
            let indent = if folder.is_empty() {
                0
            } else {
                items.push(BundledItem {
                    item: BinderItemFile {
                        title: folder.to_string(),
                        role: Role::Folder,
                        sub_role: SubRole::Paratext,
                        indent: 0,
                        ..blank(ids.item(), uid_for(project, "paratext-folder", which), &now)
                    },
                    prose: BTreeMap::new(),
                    comments: BTreeMap::new(),
                    footnotes: BTreeMap::new(),
                });
                1
            };
            for page in pages {
                let item_id = ids.item();
                let uid = uid_for(project, "paratext", &page.title);
                let content_id = ids.content();
                let mut prose = BTreeMap::new();
                prose.insert(content_id, markdown_to_djot(&page.body)?);
                items.push(BundledItem {
                    item: BinderItemFile {
                        title: page.title.clone(),
                        role: Role::Item,
                        sub_role: SubRole::Paratext,
                        indent,
                        prose_refs: vec![prose_ref(
                            content_id,
                            uid_for(project, "paratext-content", &page.title),
                            &now,
                            ContentRole::ParatextText,
                            &binder_dir,
                            uid,
                            &page.title,
                        )?],
                        ..blank(item_id, uid, &now)
                    },
                    prose,
                    comments: BTreeMap::new(),
                    footnotes: BTreeMap::new(),
                });
            }
            Ok(())
        };
    section(&mut items, &mut ids, "front", &spec.book.front_matter)?;

    // ── the book ──────────────────────────────────────────────────────────────
    let book_uid = uid_for(project, "book", &spec.book.book_folder);
    let book_id = ids.item();
    let mut book_prose = BTreeMap::new();
    let mut book_refs = Vec::new();
    if !spec.book.synopsis.trim().is_empty() {
        let content_id = ids.content();
        book_prose.insert(content_id, markdown_to_djot(&spec.book.synopsis)?);
        book_refs.push(prose_ref(
            content_id,
            uid_for(project, "book-synopsis", &spec.book.book_folder),
            &now,
            ContentRole::SynopsisText,
            &binder_dir,
            book_uid,
            &spec.book.book_folder,
        )?);
    }
    items.push(BundledItem {
        item: BinderItemFile {
            title: spec.book.book_folder.clone(),
            role: Role::Folder,
            sub_role: SubRole::Book,
            indent: 0,
            prose_refs: book_refs,
            ..blank(book_id, book_uid, &now)
        },
        prose: book_prose,
        comments: BTreeMap::new(),
        footnotes: BTreeMap::new(),
    });

    for (raw, declared) in chapters.iter().zip(&spec.chapter) {
        let item_id = ids.item();
        let key = format!("chapter-{}", raw.number);
        let uid = uid_for(project, "item", &key);
        let content_id = ids.content();

        let (djot, notes) = prose_of(spec, raw, &mut ids, &now, project)?;
        let mut prose = BTreeMap::new();
        prose.insert(content_id, djot);
        let mut footnotes = BTreeMap::new();
        if !notes.is_empty() {
            footnotes.insert(content_id, notes);
        }

        items.push(BundledItem {
            item: BinderItemFile {
                title: declared.title.clone(),
                role: Role::Item,
                sub_role: SubRole::ChapterScene,
                indent: 1,
                prose_refs: vec![prose_ref(
                    content_id,
                    uid_for(project, "content", &key),
                    &now,
                    ContentRole::SceneText,
                    &binder_dir,
                    uid,
                    &declared.title,
                )?],
                ..blank(item_id, uid, &now)
            },
            prose,
            comments: BTreeMap::new(),
            footnotes,
        });
    }

    section(&mut items, &mut ids, "back", &spec.book.back_matter)?;

    let binder = BinderFile {
        file_id: 1,
        uid: uid_for(project, "binder", &spec.book.manuscript_binder),
        created_at: now.clone(),
        updated_at: now.clone(),
        name: spec.book.manuscript_binder.clone(),
        activated: true,
        item_order: items.iter().map(|b| b.item.file_id).collect(),
    };

    Ok(WorkBundle {
        // Empty here, and filled by `enrich` — which is where every other piece of
        // editorial vocabulary is laid down (tags, the story bible, points of view). The
        // documented pipeline for a converted example is `convert` then `enrich`, so this
        // is a starting state rather than a decision: `ensure_statuses` writes the ladder
        // the spec declares on the pass that follows.
        statuses: Vec::new(),
        manifest: ProjectManifest {
            format_version: FORMAT_VERSION,
            format_min_read_version: Some(7),
            shape: ShapeTag::Zip,
            work: WorkFile {
                file_id: 1,
                created_at: now.clone(),
                updated_at: now.clone(),
                title: spec.book.title.clone(),
                author_name: spec.book.author.clone(),
                dict_language: spec.book.language.clone(),
                tag_ids: tags.iter().map(|t| t.file_id).collect(),
                dict_word_ids: Vec::new(),
                unique_id: project.clone(),
                chapter_flat: spec.book.chapter_flat,
                text_replacement_rule_ids: Vec::new(),
                custom_replacement_rules_enabled: false,
                goal_unit: GoalUnit::Words,
                smart_punctuation: Some(SmartPunctuationFile {
                    created_at: now.clone(),
                    updated_at: now.clone(),
                    override_app_default: false,
                    dashes: false,
                    ellipsis: false,
                    quotes: false,
                    quote_style: "locale_default".to_string(),
                    pre_punctuation_spacing: false,
                    dialogue_marker: false,
                }),
                number_chapters: spec.book.number_chapters,
                part_resets_chapter: false,
            },
            binder_order: vec![binder.file_id],
            kind: BundleKind::Regular,
            backup_of: None,
            backup_created_at: None,
        },
        tags,
        dict_words: Vec::new(),
        text_replacement_rules: Vec::new(),
        note_templates: Vec::new(),
        assets: Vec::new(),
        note_template_bodies: BTreeMap::new(),
        asset_bytes: BTreeMap::new(),
        trash_infos: Vec::new(),
        paces: Vec::new(),
        progress_snapshots: Vec::new(),
        orphan_comments: Vec::new(),
        orphan_footnotes: Vec::new(),
        history: Default::default(),
        binders: vec![BundledBinder { binder, items }],
        carried: BTreeMap::new(),
    })
}

/// One chapter's paragraphs as stored Djot, plus the footnotes lifted out of them.
///
/// The conversion runs the text through `markdown_to_djot`, which parses it with
/// text-document and re-serialises from the document model. That is deliberately
/// not a hand-rolled escaper: Gutenberg's `_italics_` is Markdown emphasis and must
/// survive as emphasis, while every *other* character Djot treats as punctuation has
/// to come out escaped, and the only way to be sure both hold is to let the same
/// parser the app uses do it.
fn prose_of(
    spec: &Spec,
    raw: &RawChapter,
    ids: &mut Ids,
    now: &str,
    project: &str,
) -> Result<(String, Vec<FootnoteFile>)> {
    let mut body: Vec<String> = Vec::new();
    let mut notes: Vec<FootnoteFile> = Vec::new();

    for paragraph in &raw.paragraphs {
        if let Some(tier) = scene_break::tier_of_plain_line(paragraph) {
            // A bare `* * *` is a Djot *thematic break*: the document model cannot
            // hold one, so the parser discards it and an unescaped marker would
            // simply vanish. `canonical_djot` is the exact escaping the editor
            // itself persists, and Markdown reads `\*` as a literal asterisk — so
            // handing it in escaped is also what brings it back out escaped.
            body.push(scene_break::canonical_djot(tier).to_string());
            continue;
        }
        if spec.source.footnotes
            && let Some((label, text)) = split_footnote(paragraph)
        {
            notes.push(FootnoteFile {
                file_id: ids.footnote(),
                uid: uid_for(project, "footnote", &format!("{}-{label}", raw.number)),
                created_at: now.to_string(),
                updated_at: now.to_string(),
                label: label.clone(),
                body: markdown_to_djot(text)?,
            });
            continue;
        }
        body.push(paragraph.clone());
    }

    // The reference is rewritten to an alphanumeric sentinel *before* the parse and
    // back to `[^label]` after it, because `[1]` is a link in Markdown and `[^1]` is
    // a footnote in Djot: neither survives being handed to the other parser as-is,
    // and letters and digits are the one thing no escaper touches.
    let mut markdown = body.join("\n\n");
    for note in &notes {
        markdown = markdown.replace(&format!("[{}]", note.label), &sentinel(&note.label));
    }
    let mut djot = markdown_to_djot(&markdown)?;
    for note in &notes {
        ensure!(
            djot.contains(&sentinel(&note.label)),
            "chapter {} carries footnote [{}] whose reference is nowhere in the prose",
            raw.number,
            note.label
        );
        djot = djot.replace(&sentinel(&note.label), &format!("[^{}]", note.label));
    }
    Ok((djot, notes))
}

fn sentinel(label: &str) -> String {
    format!("SKRIBFOOTNOTE{label}REF")
}

/// `[3] the note itself` → `("3", "the note itself")`.
fn split_footnote(paragraph: &str) -> Option<(String, &str)> {
    let rest = paragraph.strip_prefix('[')?;
    let (label, rest) = rest.split_once(']')?;
    (!label.is_empty() && label.chars().all(|c| c.is_ascii_digit()))
        .then(|| (label.to_string(), rest.trim_start()))
}

/// A row with everything at its default, so each call site states only what it means.
fn blank(file_id: u64, uid: uuid::Uuid, now: &str) -> BinderItemFile {
    BinderItemFile {
        status_id: None,
        file_id,
        uid,
        created_at: now.to_string(),
        updated_at: now.to_string(),
        title: String::new(),
        sub_title: String::new(),
        role: Role::Item,
        sub_role: SubRole::Text,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        exclude_from_numbering: false,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: Vec::new(),
        inline_contents: Vec::new(),
        prose_refs: Vec::new(),
        reference_ids: Vec::new(),
        point_of_view_ids: Vec::new(),
        book_ids: Vec::new(),
        tag_ids: Vec::new(),
    }
}

#[allow(clippy::too_many_arguments)]
fn prose_ref(
    file_id: u64,
    uid: uuid::Uuid,
    now: &str,
    role: ContentRole,
    binder_dir: &str,
    item_uid: uuid::Uuid,
    slug_source: &str,
) -> Result<ProseRef> {
    let name = prose_file_name(item_uid, slug_source, &role)
        .with_context(|| format!("{role:?} is not a prose role"))?;
    Ok(ProseRef {
        file_id,
        uid,
        created_at: now.to_string(),
        updated_at: now.to_string(),
        activated: true,
        role,
        path: prose_relpath(binder_dir, &name),
    })
}

fn write(path: &str, bundle: &WorkBundle) -> Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    skrib_format::write_bundle(path, SkribShape::ZipFile, bundle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_footnote_paragraph_splits_into_label_and_body() {
        assert_eq!(
            split_footnote("[12]  Le traitement."),
            Some(("12".to_string(), "Le traitement."))
        );
        assert_eq!(split_footnote("[a] pas un chiffre"), None);
        assert_eq!(split_footnote("de la prose ordinaire"), None);
    }

    #[test]
    fn letters_ignores_case_punctuation_emphasis_and_accents() {
        assert_eq!(
            letters("OU LE PATRON DE LA _TANKADERE_ RISQUE."),
            letters("Où le patron de la Tankadère risque")
        );
        // It must still be a real comparison: two different headings stay different.
        assert_ne!(letters("Où le patron"), letters("Où le patronne"));
    }

    /// The one assumption the whole prose path rests on: an escaped marker written
    /// as Markdown comes back out of the Djot serialiser still escaped, and still
    /// the exact byte sequence the editor persists when an author types the mark.
    #[test]
    fn an_escaped_scene_break_survives_the_round_trip() {
        let djot = markdown_to_djot("Avant.\n\n\\* \\* \\*\n\nApres.").unwrap();
        assert!(
            djot.contains(scene_break::canonical_djot(
                scene_break::SceneBreakTier::Minor
            )),
            "the marker did not survive as canonical Djot: {djot:?}"
        );
    }

    /// Emphasis must survive (it is the author's italics) while a stray bracket must
    /// not become a link.
    #[test]
    fn emphasis_survives_and_brackets_are_escaped() {
        let djot = markdown_to_djot("Le _Mongolia_ [pas un lien] arrive.").unwrap();
        assert!(djot.contains("_Mongolia_"), "{djot:?}");
        assert!(djot.contains("\\[pas un lien\\]"), "{djot:?}");
    }

    /// The sentinel has to pass through both parsers untouched, or the footnote
    /// reference lands in the prose as literal noise.
    #[test]
    fn the_footnote_sentinel_is_inert_in_both_parsers() {
        let s = sentinel("1");
        let djot = markdown_to_djot(&format!("cent mille{s}.")).unwrap();
        assert!(djot.contains(&s), "{djot:?}");
    }
}
