// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Assemble an export scope into **one** `TextDocument` and render it to a format.
//!
//! The whole book is compiled to a single Djot string — generated headings (`#` syntax),
//! scene breaks, and each scene's own Djot prose appended verbatim — then parsed once with
//! `set_djot`. Djot's parser gives clean headings-vs-paragraphs structure (a programmatic
//! block build would have every block inherit the previous one's heading level). A single
//! `to_<format>` call then renders it, so every format reads the same parsed graph and none
//! can diverge. Text direction is set on the document from the export's language.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Result, anyhow};
use common::entities::{BinderItem, BinderItemSubRole, Content, ContentRole};
use skrib_format::Gathered;
use skribisto_model::SubRoleExt;
use skribisto_model::footnote_numbering::{self, FootnoteRestart};
use skribisto_model::language;
use skribisto_model::numbering::{self, Numbered, NumberingRules};
use skribisto_model::scene_break::{self, SceneBreakTier};
use text_document::{
    DocxExportOptions, EpubExportOptions, MarkdownExportOptions, PdfExportOptions,
    PlainTextExportOptions, TextDirection, TextDocument,
};

use skrib_format::media::escape_djot_alt;

use crate::headings::{self, Level};
use crate::preset::{
    DirectionMode, EpigraphPlacement, ExportFormat, FootnoteNumbering, HeadingLanguage,
    HeadingScheme, ImageHandling, LineSpacing, PageSize, Preset, SceneBreak,
};

/// Everything a render needs: the frozen tree, the ordered ids to include, the style, the
/// format, and the Work's fallback language.
pub struct RenderRequest<'a> {
    pub gathered: &'a Gathered,
    pub include: &'a [u64],
    pub preset: &'a Preset,
    pub format: ExportFormat,
    pub work_lang: &'a str,
    /// The include set was *explicitly chosen* (Export Scene / Export Note, or a Choose…
    /// checkbox tree) rather than swept from a structural scope. When set, note items in the
    /// set always render, overriding `preset.include_notes` — the user pointed at them.
    pub explicit_selection: bool,
    /// Where the project's image bytes live.
    ///
    /// Supplied by the caller for the same reason `text-document` takes them
    /// rather than opening files itself: an export that resolved paths of its
    /// own would depend on the working directory and could reach outside the
    /// project. Empty means "no media", and every image degrades to its alt
    /// text — which is also what a scope with no images costs.
    pub media_dir: &'a std::path::Path,
}

/// Bytes for every image the included rows reference, keyed by the `src` the
/// prose carries.
///
/// Read per export rather than cached: an export is already a long operation
/// dominated by compiling, and a stale cache would silently ship the previous
/// version of a picture the writer just replaced.
/// `only` restricts the result to the paths a *scope* actually names — used by
/// the loose text formats, where a chapter export must not drop the whole book's
/// photographs into the writer's folder. `None` collects the Work's whole asset
/// set, which is what the container formats want: they package what they
/// reference and ignore the rest.
fn collect_images(
    gathered: &Gathered,
    media_dir: &std::path::Path,
    only: Option<&[String]>,
) -> text_document::ExportImages {
    let mut out = text_document::ExportImages::new();
    if media_dir.as_os_str().is_empty() {
        return out;
    }
    for asset in &gathered.assets {
        let ext = skrib_format::media::extension_for(&asset.mime_type);
        let relpath = skrib_format::media::asset_relpath(&asset.content_hash, &ext);
        if only.is_some_and(|want| !want.contains(&relpath)) {
            continue;
        }
        let Ok(bytes) = std::fs::read(media_dir.join(format!("{}.{ext}", asset.content_hash)))
        else {
            // A missing file is not an export failure: the image degrades to
            // its description, which is a smaller loss than refusing to produce
            // the manuscript at all.
            continue;
        };
        out.insert(
            relpath,
            text_document::ExportImage::new(bytes, asset.mime_type.clone()),
        );
    }
    out
}

/// What [`assemble`] built: the parsed book, plus the two things the format
/// arms need that cannot be recovered from the document afterwards.
struct Assembled {
    doc: TextDocument,
    stats: RenderStats,
    /// Effective languages of the included rows — the PDF arm uses these to
    /// decide which RTL faces to embed.
    langs: std::collections::BTreeSet<String>,
    /// The asset paths this *scope* actually names, scanned off the compiled
    /// Djot before it was parsed.
    ///
    /// Not the same set as the Work's assets: exporting one chapter must not
    /// write the whole book's photographs beside it. Recovering this from the
    /// rendered output instead would mean re-finding each `src` through the
    /// escaping rules of six different formats.
    image_refs: Vec<String>,
    /// Every `Content` whose prose reached the compiled document, in emission order.
    ///
    /// The record a comment needs to be rebased (see [`comment_rebase`]). It cannot be
    /// recovered afterwards: the compiled document is one flat stream with no memory of which
    /// row each paragraph came from, and the rows are not simply concatenated — headings, a
    /// title page, epigraphs and scene-break glyphs are interleaved, and out-of-scope rows are
    /// omitted entirely. Recorded here, while `assemble` still knows.
    emitted: Vec<EmittedContent>,
}

/// One `Content` row's prose, as it was handed to the compiled document.
#[derive(Debug, Clone)]
pub(crate) struct EmittedContent {
    /// The `Content` row's own id — what a `Comment` points at.
    pub(crate) content_id: u64,
    /// The uid of the `BinderItem` this content belongs to — what a **round-trip row mark**
    /// spells, so a returning file can be matched back onto the binder it came from.
    ///
    /// The item's, not the content's: a `Content` id is re-minted on every `load_work` and would
    /// name nothing after a save/reload, while `BinderItem.uid` is the project's one durable
    /// handle on a row. Nil for a row created before identity was minted, which
    /// [`row_marks`] skips rather than writing a mark that names nothing.
    pub(crate) item_uid: uuid::Uuid,
    /// The row's stored Djot, *before* `push_prose` transformed it.
    ///
    /// Deliberately the stored form rather than the emitted form: it is the string the
    /// comment's quote was captured against, so it is what the anchor engine must be shown.
    /// The transformation `push_prose` applies (a scene-break marker becoming the preset's
    /// rendering) is exactly why the window search downstream is block-wise and tolerant
    /// rather than a whole-row string comparison.
    djot: String,
}

impl EmittedContent {
    /// This row's prose as plain text — what a round-trip digest is taken over.
    ///
    /// The *stored* Djot's plain text, matching what `document_ingest` will report for the same
    /// row when the file comes back: both sides go through
    /// [`skribisto_model::round_trip::normalize`], which cancels the whitespace and
    /// invisible-character differences a trip through an editor introduces. A row whose Djot
    /// cannot be parsed digests as empty rather than failing the export — a mark is an aid, not
    /// content.
    fn djot_plain(&self) -> String {
        skrib_format::djot_plain_text(&self.djot)
            .map(|(text, _)| text)
            .unwrap_or_default()
    }
}

/// What a render produced, for the result DTO / a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderStats {
    pub items: usize,
    pub words: usize,
    /// Comments written into the exported file. Always 0 for a format that does not
    /// [carry comments](ExportFormat::carries_comments).
    pub comments_written: usize,
    /// Comments that belonged in this export and could not be placed, so were dropped.
    ///
    /// **Not** a count of every comment missing from the file: a comment on a row outside the
    /// export scope was never a candidate and is not counted. This is the number the writer
    /// deserves to be told about, and nothing more — a warning that fires on a perfectly good
    /// scoped export teaches people to ignore it.
    pub comments_orphaned: usize,
}

/// One included item, resolved: the entity, its content rows, and its effective language.
struct Row<'a> {
    item: &'a BinderItem,
    contents: &'a [Content],
    lang: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Public entry points
// ─────────────────────────────────────────────────────────────────────────────

/// Render a text format (Djot / plain text / Markdown / HTML / LaTeX) to a `String`.
pub fn render_to_string(req: &RenderRequest) -> Result<String> {
    if !req.format.is_text() {
        return Err(anyhow!("{:?} is not a text format", req.format));
    }
    let built = assemble(req, &|_| {}, &AtomicBool::new(false))?;
    // No path to write beside, so no images can be placed: a string render
    // references them as the prose does and leaves resolving that to whoever
    // receives the string.
    text_render(
        &built.doc,
        req.format,
        ImageHandling::CopyBeside,
        &images_for_html(req, &built),
    )
}

fn text_render(
    doc: &TextDocument,
    format: ExportFormat,
    handling: ImageHandling,
    html_images: &text_document::ExportImages,
) -> Result<String> {
    let omit = handling == ImageHandling::Omit;
    Ok(match format {
        ExportFormat::Djot => doc.to_djot_with_options(text_document::DjotExportOptions {
            omit_images: omit,
            ..Default::default()
        })?,
        // The *presentation* plain text, not the addressable one. `.txt` has no markup to
        // mark quoted matter, so an epigraph would otherwise dissolve into the body, and
        // no page concept, so a chapter boundary would vanish entirely. The flush
        // `to_plain_text()` is the view search computes offsets against and deliberately
        // stays bare — this is a file being written out, so it wants everything.
        ExportFormat::PlainText => doc.to_plain_text_with(PlainTextExportOptions::presentation())?,
        // Markdown's page break is raw HTML, which is why text-document keeps it opt-in.
        // Opting in here is safe *and* correct: the compiled document only carries a break
        // where the chosen style asked for one, so the knob that governs it is the style's,
        // shared with every other format, rather than a second one hidden in this arm.
        ExportFormat::Markdown => doc.to_markdown_with(MarkdownExportOptions {
            page_breaks: true,
            omit_images: omit,
        })?,
        // HTML is the one referencing format that can also carry its images, so
        // it is the only one where `Embed` means anything.
        ExportFormat::Html => doc.to_html_with_options(text_document::HtmlExportOptions {
            image_mode: match handling {
                ImageHandling::CopyBeside => text_document::HtmlImageMode::Reference,
                ImageHandling::Embed => text_document::HtmlImageMode::DataUri,
                ImageHandling::Omit => text_document::HtmlImageMode::Omit,
            },
            images: html_images.clone(),
        })?,
        // Spelled out in full rather than with `..Default::default()`: LaTeX carries no
        // comments (there is no LaTeX importer, so they would leave and never come home), so
        // unlike `docx_options`/`odt_options` this struct has nothing left to default.
        ExportFormat::Latex => doc.to_latex_with_options(text_document::LatexExportOptions {
            document_class: "article".into(),
            include_preamble: true,
            omit_images: omit,
        })?,
        other => return Err(anyhow!("{other:?} is not a text format")),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Assembly
// ─────────────────────────────────────────────────────────────────────────────

fn assemble(req: &RenderRequest, progress: &dyn Fn(f32), cancel: &AtomicBool) -> Result<Assembled> {
    let rows = flatten(req);
    let preset = req.preset;

    // The effective languages of the included rows — the PDF arm uses these to decide which
    // RTL faces to embed, so it needn't re-flatten the tree just to recompute them.
    let langs: std::collections::BTreeSet<String> = rows.iter().map(|r| r.lang.clone()).collect();

    // Heading levels are dense over the structural levels actually present, so a
    // chapter-only export starts at h1 and a book+chapter export (no parts) uses h1/h2.
    let mut present_depths: Vec<u8> = rows
        .iter()
        .filter_map(|r| numbering::level_of(&r.item.sub_role))
        .map(depth)
        .collect();
    present_depths.sort_unstable();
    present_depths.dedup();

    let mut out = String::new();
    // Every structural row's number, from the whole manuscript — so a scoped export
    // ("Export Chapter") reports the chapter's real number rather than renumbering from
    // one, and so no two export scopes can ever disagree about it.
    let numbers = manuscript_numbers(req);
    let mut words = 0usize;
    // Block attributes a scene break has queued for the *next* prose block: the
    // suppressed first-line indent, plus the extra leading a `BlankLine` break
    // renders as. It outlives the row loop because the paragraph following a
    // break may belong to the next item.
    let mut pending_attrs: Vec<String> = Vec::new();
    // Every Content whose prose actually reaches `out`, in the order it reaches it — the
    // record `comment_rebase` walks with its monotone cursor.
    let mut emitted_contents: Vec<EmittedContent> = Vec::new();

    let work_rtl = is_rtl_row(preset, req.work_lang);

    // A title page is built *after* the body — it carries the manuscript's word count,
    // which is only known once the rows have been walked — and prepended. Decided up
    // front all the same, because the body's first block has to know a page is coming
    // above it.
    let title_page = preset.book_title_page && rows.iter().any(|r| r.item.sub_role.opens_book());

    // The cover, on the same terms: it opens the *book*, so an export that does
    // not include the book's opening has no business carrying it. EPUB is the
    // exception — it has a real cover slot in the package, filled by the format
    // arm, and putting the picture inline as well would show it twice.
    let cover = preset
        .book_cover
        .then(|| cover_relpath(req))
        .flatten()
        .filter(|_| {
            req.format != ExportFormat::Epub && rows.iter().any(|r| r.item.sub_role.opens_book())
        });

    // A page break queued for the next block emitted, whatever that turns out to be: a
    // structural heading, or the row's own prose when the preset suppresses the heading.
    // It survives a row that emits nothing (a Book opener under a title page emits no
    // heading at all), so the break lands on the next thing that *is* printed rather than
    // being lost at that seam.
    let mut pending_break = false;
    // Whether anything will be printed above the block about to be emitted. A page break
    // on the very first block would open on a blank page in the formats that take it
    // literally, and the title page counts here even though `out` is still empty.
    let mut anything_above = title_page || cover.is_some();
    // With a cover but no title page, the body's own first block is what has to
    // break away from it — nothing else will.
    if cover.is_some() && !title_page {
        pending_break = true;
    }

    let total = rows.len().max(1);
    let report = |p: usize| progress(0.9 * (p as f32 + 1.0) / total as f32);
    // Scene titles (if kept) sit one heading level below the deepest structural level
    // present; a title-less scene export starts them at h1.
    let scene_title_level = (present_depths.len() as u8 + 1).clamp(1, 6);
    let mut emitted_items = 0usize;
    for (i, row) in rows.iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err(anyhow!("operation cancelled"));
        }
        // A note swept into a structural scope is dropped unless the preset keeps notes, or
        // the selection was explicit (Export Note / a checked Choose… item — the user
        // pointed at it, which overrides the toggle).
        let is_note = matches!(row.item.sub_role, BinderItemSubRole::Note);
        if is_note && !preset.include_notes && !req.explicit_selection {
            report(i);
            continue;
        }

        // A paratext is the author's, but it is not the story. The preset can leave it
        // out — the clean-submission case, where an editor wants the manuscript and not
        // the acknowledgements. Decided here, beside the note gate, so the row is skipped
        // whole.
        //
        // Note that a paratext emits **no heading of its own**. Its title is a binder
        // label the writer chose to find it by — "Copyright", "front matter (draft)" —
        // not a line of the book. A writer who wants a heading on the page writes one in
        // the prose, where they control its wording and its level. The same reasoning
        // keeps a `Folder/Paratext` from emitting anything at all.
        let is_paratext = matches!(row.item.sub_role, BinderItemSubRole::Paratext);
        if is_paratext && !preset.include_paratexts {
            report(i);
            continue;
        }
        // A paratext has no heading to carry the break, so it queues one for its own first
        // prose block below — and, at the bottom of the loop, another for whatever follows
        // it. What comes after the last paratext of a run is usually ordinary prose (a
        // prologue, an opening scene) with no structural opener of its own to break on, so
        // without that second arming the front matter's last page runs into the body.
        if is_paratext && preset.paratext_starts_page {
            pending_break = true;
        }

        let heading_lang = match &preset.heading_language {
            HeadingLanguage::Fixed(l) => l.clone(),
            HeadingLanguage::Auto => row.lang.clone(),
        };
        let row_rtl = is_rtl_row(preset, &row.lang);
        let heading_rtl = is_rtl_row(preset, &heading_lang);
        let mut contributed = false;

        let level = numbering::level_of(&row.item.sub_role);
        if let Some(level) = level {
            // Opening a structural level restarts the flow, so whatever a
            // trailing break queued for "the next paragraph" stops here. This
            // keys off the *structure*, not off whether a heading actually
            // printed: a preset whose chapter scheme is `None` emits no text,
            // and gating on that would let a break bleed across the seam.
            pending_attrs.clear();
            // Armed on the *structure*, and armed here — above everything this row emits —
            // because with the epigraph above the heading it is the epigraph, not the
            // title, that opens the page.
            if match level {
                Level::Book => preset.book_starts_page,
                Level::Part => preset.part_starts_page,
                Level::Chapter => preset.chapter_starts_page,
            } {
                pending_break = true;
            }
        }

        // The epigraph, prepared once and emitted on whichever side of the heading the
        // style asks for. Every convention that writes the rule down puts it after the
        // title; putting it above is a designer's choice with real currency, so the style
        // decides and the default is the convention (see `EpigraphPlacement`).
        let epigraph = preset
            .include_epigraphs
            .then(|| content_of(row.contents, ContentRole::EpigraphText))
            .flatten()
            .filter(|e| !e.trim().is_empty());
        let epigraph_leads =
            epigraph.is_some() && preset.epigraph_placement == EpigraphPlacement::BeforeHeading;

        // 1a. The epigraph, when it opens the chapter.
        if epigraph_leads && let Some(epi) = epigraph {
            let brk = take_break(&mut pending_break, &mut anything_above);
            contributed |= push_epigraph(
                &mut out,
                epi,
                &brk,
                row_rtl,
                preset,
                &mut pending_attrs,
                row,
            );
        }

        // 1b. A structural heading, if this item opens a level; else a scene title, if kept.
        if let Some(level) = level {
            // A book is titled, not numbered — and the title page (if on) already carries
            // it, so the opener then emits nothing. Parts/chapters use their own schemes.
            let scheme = match level {
                Level::Book if preset.book_title_page => HeadingScheme::None,
                Level::Book => HeadingScheme::TitleOnly,
                Level::Part => preset.part_heading,
                Level::Chapter => preset.chapter_heading,
            };
            let number = numbers.get(&row.item.id).map(Numbered::number);
            if let Some(text) = heading_text(row, level, number, &heading_lang, preset, scheme) {
                let lvl = present_depths
                    .iter()
                    .position(|&d| d == depth(level))
                    .map(|p| (p + 1).min(6))
                    .unwrap_or(1) as u8;
                let extra = take_break(&mut pending_break, &mut anything_above);
                push_heading(&mut out, lvl, &text, heading_rtl, &extra);
                contributed = true;
            }
        } else if preset.include_scene_titles && row.item.sub_role.carries_scene() {
            // A plain titled Scene (never a ChapterScene — that already emitted its chapter
            // heading above): the title heading stands in for the scene break.
            let t = row.item.title.trim();
            if !t.is_empty() {
                let extra = take_break(&mut pending_break, &mut anything_above);
                push_heading(&mut out, scene_title_level, t, row_rtl, &extra);
                pending_attrs.clear();
                contributed = true;
            }
        }

        // 2. The epigraph, when it follows the heading — where CMOS, French, German and
        //    Russian practice all put it: after the chapter number/title, before the body.
        if !epigraph_leads && let Some(epi) = epigraph {
            contributed |=
                push_epigraph(&mut out, epi, &[], row_rtl, preset, &mut pending_attrs, row);
        }

        // 3. The main prose (a scene's SceneText, a note's NoteText) — appended verbatim
        //    since it is already Djot.
        if let Some(role) = main_prose_role(&row.item.sub_role)
            && let Some(prose_row) = row_content_of(row.contents, role)
        {
            let prose = prose_row.data.as_str();
            // Only a scene's own prose is scanned for break markers. A
            // marker typed into a Note is just literal text — the model is
            // about scene flow.
            let scan = row.item.sub_role.carries_scene();
            // A scene whose whole prose is a single break marker emits no
            // prose at all, so it must not be counted as an emitted item —
            // the marker is furniture, and the row contributed nothing.
            //
            // Any page break still queued lands here: this row printed no heading to
            // carry it (a paratext never has one; a chapter under `HeadingScheme::None`
            // has none either), so its opening paragraph is what opens the page.
            let lead = take_break(&mut pending_break, &mut anything_above);
            let (w, emitted) = push_prose(
                &mut out,
                prose,
                row_rtl,
                preset,
                scan,
                &mut pending_attrs,
                &lead,
            );
            // A paratext's prose rides this same step — it is the page's whole content —
            // but its words are not the manuscript's. Counting them would drift every
            // pace goal in the project by the length of the front matter, with nothing
            // looking wrong.
            if !matches!(row.item.sub_role, BinderItemSubRole::Paratext) {
                words += w;
            }
            // Recorded only when the row genuinely printed something. A scene whose whole
            // prose is a single break marker emits nothing, and listing it here would give
            // the rebase a window to hunt for that is not in the document — which is how a
            // later row's comment ends up matching an earlier row's text.
            if emitted {
                emitted_contents.push(EmittedContent {
                    content_id: prose_row.id,
                    item_uid: row.item.uid,
                    djot: prose.to_string(),
                });
            }
            contributed |= emitted;
        }

        // 4. The synopsis, if the preset keeps it.
        if preset.include_synopses
            && let Some(syn_row) = row_content_of(row.contents, ContentRole::SynopsisText)
        {
            // A synopsis is commentary, not the scene's prose — never
            // scanned for markers, and its words are not the manuscript's.
            let (_, emitted) = push_prose(
                &mut out,
                &syn_row.data,
                row_rtl,
                preset,
                false,
                &mut pending_attrs,
                &[],
            );
            contributed |= emitted;
            // A synopsis carries comments of its own, and they are a *different* window from
            // the scene's: two remarks quoting the same sentence, one on the prose and one on
            // the summary of it, must not be able to claim each other's position. Recorded
            // only when `include_synopses` is on — with it off the synopsis is not in the
            // document, and a comment on it is out of scope rather than orphaned.
            if emitted {
                emitted_contents.push(EmittedContent {
                    content_id: syn_row.id,
                    item_uid: row.item.uid,
                    djot: syn_row.data.clone(),
                });
            }
        }

        // Re-armed *after* the row: the break this row consumed was its own, and the page
        // it opened has to end somewhere too.
        if is_paratext && preset.paratext_starts_page {
            pending_break = true;
        }

        if contributed {
            emitted_items += 1;
        }
        report(i);
    }

    // The title page, now that the word count is known. Built separately and prepended so
    // it can carry it — an editor reads that number before anything else on the page.
    if title_page {
        out.insert_str(
            0,
            &render_title_page(req, preset, work_rtl, words, cover.is_some()),
        );
    }

    // And the cover above even that: the first thing in the book, centred. The
    // page break that keeps it on a sheet of its own is carried by whatever
    // follows — the title page above, or the body's first block (armed before
    // the row loop).
    //
    // The alt text is the book's title. A cover is not decorative, and "Cover"
    // tells someone reading with a screen reader nothing they do not already
    // know from the file they opened.
    if let Some(relpath) = &cover {
        let alt = escape_djot_alt(req.gathered.work.title.trim());
        let mut block = String::new();
        push_para(
            &mut block,
            &format!("![{alt}]({relpath})"),
            work_rtl,
            &["alignment=center".to_string()],
        );
        out.insert_str(0, &block);
    }

    // Footnote definitions, and the numbers the *manuscript* gives them.
    //
    // The bodies have to be appended to the compiled Djot or nothing downstream has
    // them: a reference alone parses fine, but every writer would then render a
    // marker pointing at a note that is not in the document.
    //
    // The markers are pushed separately, because they are a fact about the book
    // rather than about this document. Compile one chapter and its own reading
    // order would number the notes from one — disagreeing with the badge the writer
    // is looking at in the editor, for the same note.
    let notes = footnote_bodies(req, &rows);
    if !notes.definitions.is_empty() {
        if !out.trim_end().is_empty() {
            out.push_str("\n\n");
        }
        out.push_str(&notes.definitions.join("\n\n"));
    }

    let doc = TextDocument::new();
    if !out.trim().is_empty() {
        doc.set_djot(&out)?
            .wait()
            .map_err(|e| anyhow!("parsing the compiled document: {e:#}"))?;
    }
    if !notes.markers.is_empty() {
        doc.set_footnote_markers(notes.markers);
    }
    // Document text direction from the export's language (v1 is whole-document; a mixed
    // LTR/RTL book uses its dominant language here — per-scene direction is a refinement).
    if let Some(dir) = document_direction(req, &rows) {
        doc.set_text_direction(dir)?;
    }

    Ok(Assembled {
        doc,
        stats: RenderStats {
            items: emitted_items,
            words,
            // Filled by `render_to_file` once the format is known — `assemble` builds the
            // document, and whether comments ride along is a property of the writer.
            comments_written: 0,
            comments_orphaned: 0,
        },
        langs,
        image_refs: skrib_format::media::referenced_paths(&out),
        emitted: emitted_contents,
    })
}

/// Resolve the footnotes of the rows this export includes.
///
/// Numbering comes from [`skribisto_model::footnote_numbering`], run over the
/// **whole** gathered tree rather than over `rows` — the same rule chapter numbers
/// follow, and for the same reason: exporting chapter five must number its notes as
/// the book numbers them.
///
/// Only notes actually referenced by an included row are emitted. A note whose
/// reference sits in a chapter this export leaves out has nothing pointing at it
/// here, and printing it would put an unreferenced note at the foot of the page.
///
/// **Which marker a wanted label draws does not come from the row that noticed
/// it.** `number_map` is keyed by `(item_id, label)`, so a label duplicated
/// across two items (a scene copied verbatim without reminting its footnote
/// labels — see `binder_item_management::duplicate_uc.rs`) has two independent
/// entries. Looking one up by *this row's own* item id used to mean a scoped
/// export could pick a different entry — and print a different number — than
/// the whole-manuscript collapse the editor's live badge uses, for the exact
/// same citation, with no edit in between. `label_homes` is the one collapse
/// both share (see its doc for the rule); a row here only ever decides whether a
/// label is *wanted*, never which number it prints.
fn footnote_bodies(req: &RenderRequest, rows: &[Row]) -> CompiledNotes {
    let preset = req.preset;
    if !preset.include_footnotes || req.gathered.footnotes.is_empty() {
        return CompiledNotes {
            definitions: Vec::new(),
            markers: std::collections::HashMap::new(),
        };
    }

    let by_label: std::collections::HashMap<&str, &str> = req
        .gathered
        .footnotes
        .iter()
        .map(|f| (f.footnote.label.as_str(), f.footnote.body.as_str()))
        .collect();
    let labels: Vec<String> = by_label.keys().map(|l| l.to_string()).collect();

    // Prose per item, from the whole tree — numbering must not see the selection.
    let mut prose_by_item: std::collections::HashMap<u64, Vec<&str>> =
        std::collections::HashMap::new();
    for bwi in &req.gathered.binders {
        for iwc in &bwi.items {
            let mut datas: Vec<&str> = Vec::new();
            for c in &iwc.contents {
                if c.activated {
                    datas.push(c.data.as_str());
                }
            }
            if !datas.is_empty() {
                prose_by_item.insert(iwc.item.id, datas);
            }
        }
    }

    let numbered = footnote_numbering::number_map(
        &crate::item_metas(req.gathered),
        &labels,
        |id| prose_by_item.get(&id).cloned().unwrap_or_default(),
        match preset.footnote_numbering {
            FootnoteNumbering::Continuous => FootnoteRestart::Continuous,
            FootnoteNumbering::PerChapter => FootnoteRestart::PerChapter,
            FootnoteNumbering::PerBook => FootnoteRestart::PerBook,
        },
    );

    // The one collapse from `number_map`'s per-item entries to the marker each
    // label actually prints — computed once, over the whole tree, so it agrees
    // with the editor's own badge (see the doc comment above and
    // `label_homes`'s own doc for why this cannot be re-derived per row).
    let homes = footnote_numbering::label_homes(&numbered);

    // Which of them this export actually references.
    let mut wanted: Vec<(usize, String)> = Vec::new();
    let mut markers = std::collections::HashMap::new();
    for row in rows {
        for content in row.contents {
            if !content.activated {
                continue;
            }
            for (_, label) in footnote_numbering::references_in(&content.data, &labels) {
                let Some((_, n)) = homes.get(&label) else {
                    continue;
                };
                if markers.contains_key(&label) {
                    continue;
                }
                markers.insert(label.clone(), n.number.to_string());
                wanted.push((n.ordinal, label));
            }
        }
    }
    wanted.sort_unstable();

    let definitions = wanted
        .into_iter()
        .filter_map(|(_, label)| {
            let body = by_label.get(label.as_str())?;
            let body = body.trim();
            if body.is_empty() {
                return None;
            }
            // Continuation lines indented, which is what keeps a multi-paragraph
            // note attached to its definition instead of ending it.
            let mut lines = body.lines();
            let mut out = format!("[^{label}]: {}", lines.next().unwrap_or_default());
            for line in lines {
                out.push('\n');
                if !line.is_empty() {
                    out.push_str("    ");
                    out.push_str(line);
                }
            }
            Some(out)
        })
        .collect();

    CompiledNotes {
        definitions,
        markers,
    }
}

fn depth(level: Level) -> u8 {
    match level {
        Level::Book => 0,
        Level::Part => 1,
        Level::Chapter => 2,
    }
}

fn main_prose_role(sr: &BinderItemSubRole) -> Option<ContentRole> {
    if sr.carries_scene() {
        Some(ContentRole::SceneText)
    } else if matches!(sr, BinderItemSubRole::Note) {
        Some(ContentRole::NoteText)
    } else if matches!(sr, BinderItemSubRole::Paratext) {
        Some(ContentRole::ParatextText)
    } else {
        None
    }
}

/// Put `{semantic_role=epigraph}` on the first block of each blockquote in `djot`.
///
/// The author writes an ordinary blockquote; the marker is the compiler's business, not
/// something to make them type. Each quotation in the row gets its own — several
/// epigraphs on one node are several blockquotes, and each is an epigraph in its own
/// right, so marking only the first would leave the rest as plain quotations.
///
/// Text that is not a blockquote is returned untouched: an epigraph field holding a bare
/// paragraph (someone typed a quotation without the `>`) still exports as what it is
/// rather than acquiring a claim the markup cannot carry.
/// `extra` rides the **first** quotation's attribute line only — it is the page break that
/// opens the chapter, and a chapter opens once however many quotations follow. It has to go
/// *inside* the quote: an attribute line in front of a `>` block is attached to no block at
/// all by the importer (block attributes are read for standalone paragraphs and headings),
/// so a break written there would vanish without trace.
///
/// Returns whether any quotation was found, so a caller that wanted to hand `extra` over
/// knows whether it was taken — an epigraph typed as a bare paragraph has no quotation to
/// put it on, and the caller must place it the ordinary way instead.
fn mark_epigraph(djot: &str, extra: &[String]) -> (String, bool) {
    let mut out = String::with_capacity(djot.len() + 32);
    let mut in_quote = false;
    let mut marked = false;
    for line in djot.lines() {
        let is_quote_line = line.trim_start().starts_with('>');
        if is_quote_line && !in_quote {
            // Match the line's own `>` prefix so the attribute lands inside the quote,
            // and at the same nesting depth as the text it describes.
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            let mut attrs = vec!["semantic_role=epigraph".to_string()];
            if !marked {
                attrs.extend(extra.iter().cloned());
            }
            out.push_str(&indent);
            out.push_str(&format!("> {{{}}}\n", attrs.join(" ")));
            marked = true;
        }
        in_quote = is_quote_line;
        out.push_str(line);
        out.push('\n');
    }
    (out, marked)
}

fn content_of(contents: &[Content], role: ContentRole) -> Option<&str> {
    row_content_of(contents, role).map(|c| c.data.as_str())
}

/// The same lookup as [`content_of`], but keeping the whole row.
///
/// Separate because a comment hangs off a **`Content`**, not off a `BinderItem`: a remark on a
/// scene's prose and one on the same scene's synopsis are anchored against two different
/// strings, and only the row's own id distinguishes them. `content_of` deliberately keeps
/// returning `&str` — every existing caller wants the prose and nothing else, and threading an
/// id through them would be noise.
fn row_content_of(contents: &[Content], role: ContentRole) -> Option<&Content> {
    contents
        .iter()
        .find(|c| c.role == role && c.activated && !c.data.is_empty())
}

/// The item's own title content (ChapterTitle / PartTitle / BookTitle), falling back to the
/// binder-tree title.
///
/// A title that is *blank* — empty, or nothing but whitespace — is `None`, not `Some("  ")`.
/// The old `.is_empty()` test measured byte length, so a title of three spaces reached the
/// composer as a real title and produced the heading `"Chapter 3 —    "`: a dangling em
/// dash and trailing spaces. Blank is blank.
fn title_of<'a>(row: &'a Row) -> Option<&'a str> {
    for role in [
        ContentRole::ChapterTitle,
        ContentRole::PartTitle,
        ContentRole::BookTitle,
    ] {
        if let Some(t) = content_of(row.contents, role).filter(|t| !t.trim().is_empty()) {
            return Some(t.trim());
        }
    }
    let own = row.item.title.trim();
    (!own.is_empty()).then_some(own)
}

/// Compose one structural heading.
///
/// `number` is `None` when the row holds no ordinal — because the writer excluded it from
/// numbering (a prologue), or because the manuscript is unnumbered. Every scheme then
/// falls back to the title, and a scheme that cannot produce anything at all returns
/// `None` rather than inventing a numeral.
fn heading_text(
    row: &Row,
    level: Level,
    number: Option<usize>,
    lang: &str,
    preset: &Preset,
    scheme: HeadingScheme,
) -> Option<String> {
    let title = title_of(row);
    let numbered = number.map(|n| headings::numbered(lang, level, n, preset.digit_style));
    match scheme {
        HeadingScheme::None => None,
        // An unnumbered row under a numbers-only scheme still needs naming — a prologue
        // typeset as a blank chapter opener would be a hole in the book.
        HeadingScheme::Numbered => numbered.or_else(|| title.map(str::to_string)),
        // `TitleOnly` degrading to a bare number when the title is empty is deliberate and
        // long-standing — an untitled chapter is better opened by "Chapter 3" than by
        // nothing at all — but it *is* a degrade, and worth naming as one here rather than
        // leaving a reader of this match to infer it from an `or_else`.
        HeadingScheme::TitleOnly => title.map(str::to_string).or(numbered),
        HeadingScheme::NumberAndTitle => match (numbered, title) {
            // A title that already *is* the number is not a title to append: writers name
            // their chapters "Chapter 5" (and every project this app creates starts out
            // that way) purely because nothing else ever showed them the number. See
            // `headings::is_redundant_number_title` for exactly what counts as saying the
            // same thing, and for why a *different* number is left visibly doubled.
            (Some(n), Some(t))
                if !headings::is_redundant_number_title(t, lang, level, number.unwrap_or(0)) =>
            {
                // Folded to spaces: a heading is one line (see `Preset::heading_separator`).
                let sep = preset.heading_separator.replace(['\n', '\r'], " ");
                Some(format!("{n}{sep}{t}"))
            }
            (Some(n), _) => Some(n),
            (None, Some(t)) => Some(t.to_string()),
            (None, None) => None,
        },
    }
}

fn document_direction(req: &RenderRequest, rows: &[Row]) -> Option<TextDirection> {
    match req.preset.direction {
        DirectionMode::ForceRtl => Some(TextDirection::RightToLeft),
        DirectionMode::ForceLtr => Some(TextDirection::LeftToRight),
        DirectionMode::Auto => {
            let lang = if !req.work_lang.is_empty() {
                req.work_lang
            } else {
                rows.first().map(|r| r.lang.as_str()).unwrap_or("")
            };
            language::is_rtl(lang).then_some(TextDirection::RightToLeft)
        }
    }
}

#[cfg(test)]
mod tests;

/// Rebasing a comment's row-local anchor onto the compiled document — the one conversion the
/// whole comment-export feature rests on. See the module's own docs for why it is a
/// re-resolution and not arithmetic.
mod emit;
mod output;

pub use output::render_to_file;

use emit::*;
use output::*;

pub mod comment_rebase;
