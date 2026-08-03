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

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Result, anyhow};
use common::entities::{BinderItem, BinderItemSubRole, Content, ContentRole};
use skrib_format::Gathered;
use skribisto_model::SubRoleExt;
use skribisto_model::language;
use skribisto_model::scene_break::{self, SceneBreakTier};
use text_document::{
    DocxExportOptions, EpubExportOptions, PdfExportOptions, TextDirection, TextDocument,
};

use crate::headings::{self, Level};
use crate::preset::{
    DirectionMode, ExportFormat, HeadingLanguage, HeadingScheme, LineSpacing, PageSize, Preset,
    SceneBreak,
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
}

/// What a render produced, for the result DTO / a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderStats {
    pub items: usize,
    pub words: usize,
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
    let (doc, _, _) = assemble(req, &|_| {}, &AtomicBool::new(false))?;
    text_render(&doc, req.format)
}

/// Render an HTML preview (used by the UI live preview regardless of the chosen format).
pub fn render_preview_html(req: &RenderRequest) -> Result<String> {
    let (doc, _, _) = assemble(req, &|_| {}, &AtomicBool::new(false))?;
    Ok(doc.to_html()?)
}

/// Assemble the compiled document for the UI live preview: the *same* `TextDocument`
/// [`render_to_file`] would render (generated headings + scene breaks + prose, with
/// per-block direction), handed back so the panel can show it in a read-only editor. This
/// is why the preview and the committed export cannot diverge — one assembly path.
pub fn render_preview_document(req: &RenderRequest) -> Result<TextDocument> {
    let (doc, _, _) = assemble(req, &|_| {}, &AtomicBool::new(false))?;
    Ok(doc)
}

/// Render to `path`, honouring `progress` (0..1) and `cancel`. Handles every format: text
/// formats are assembled + written; DOCX is written by text-document's own writer.
pub fn render_to_file(
    req: &RenderRequest,
    path: &Path,
    progress: &dyn Fn(f32),
    cancel: &AtomicBool,
) -> Result<RenderStats> {
    let (doc, stats, langs) = assemble(req, progress, cancel)?;
    if cancel.load(Ordering::Relaxed) {
        return Err(anyhow!("operation cancelled"));
    }
    match req.format {
        f if f.is_text() => {
            let text = text_render(&doc, f)?;
            fs::write(path, text).map_err(|e| anyhow!("writing '{}': {e}", path.display()))?;
        }
        ExportFormat::Docx => {
            let out = path.to_string_lossy().into_owned();
            let opts = docx_options(
                req.preset,
                &req.gathered.work.title,
                &req.gathered.work.author_name,
            );
            doc.to_docx_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing DOCX '{out}': {e:#}"))?;
        }
        ExportFormat::Epub => {
            let out = path.to_string_lossy().into_owned();
            let w = &req.gathered.work;
            // The book's language drives the EPUB `dc:language` + reading direction; fall back
            // to the work-level language when the document carries none. `dc:language` takes
            // ONE tag, so this must be the primary, not the whole (possibly multi-tag) field.
            let primary = skribisto_model::language::primary(&w.dict_language);
            let lang = if primary.is_empty() {
                req.work_lang.to_string()
            } else {
                primary.to_string()
            };
            let opts = EpubExportOptions {
                title: w.title.clone(),
                author: w.author_name.clone(),
                rtl: is_rtl_row(req.preset, &lang),
                language: lang,
            };
            doc.to_epub_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing EPUB '{out}': {e:#}"))?;
        }
        ExportFormat::Pdf => {
            let out = path.to_string_lossy().into_owned();
            let w = &req.gathered.work;
            // Same single-tag requirement as the EPUB arm above.
            let primary = skribisto_model::language::primary(&w.dict_language);
            let lang = if primary.is_empty() {
                req.work_lang.to_string()
            } else {
                primary.to_string()
            };
            // Effective languages of the included rows (computed once by `assemble`) drive
            // which RTL faces to embed.
            let (page_w, page_h) = pdf_page_mm(req.preset.page_size);
            let m = &req.preset.margin;
            let in_to_mm = |i: f32| i * 25.4;
            let opts = PdfExportOptions {
                page_width_mm: page_w,
                page_height_mm: page_h,
                margin_top_mm: in_to_mm(m.top_in),
                margin_bottom_mm: in_to_mm(m.bottom_in),
                margin_left_mm: in_to_mm(m.left_in),
                margin_right_mm: in_to_mm(m.right_in),
                // The family name must match the bytes actually fed (substitute-aware).
                font_family: crate::fonts::pdf_body_family(req.preset),
                font_bytes: crate::fonts::pdf_font_bytes(req.preset, &langs),
                font_size_pt: req.preset.font_size_pt,
                // Typst `leading` (the gap *between* lines, not a line-height multiple — Typst
                // has no direct multiple), in em. Approximated as `multiple - 0.35`, anchoring
                // single on Typst's own 0.65em default and scaling up for 1½ / double so a
                // "double-spaced" manuscript reads visibly more open than 1½.
                line_spacing: match req.preset.line_spacing {
                    LineSpacing::Single => 0.65,
                    LineSpacing::OneAndHalf => 1.15,
                    LineSpacing::Double => 1.65,
                },
                first_line_indent_mm: (req.preset.first_line_indent_in > 0.0)
                    .then(|| in_to_mm(req.preset.first_line_indent_in)),
                paragraph_spacing_pt: (req.preset.paragraph_spacing_pt > 0.0)
                    .then_some(req.preset.paragraph_spacing_pt),
                justify: req.preset.justify,
                base_rtl: is_rtl_row(req.preset, &lang),
                lang: Some(lang),
                title: (!w.title.trim().is_empty()).then(|| w.title.clone()),
                author: (!w.author_name.trim().is_empty()).then(|| w.author_name.clone()),
                include_preamble: true,
            };
            doc.to_pdf_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing PDF '{out}': {e:#}"))?;
        }
        other => return Err(anyhow!("{other:?} export is not implemented yet")),
    }
    progress(1.0);
    Ok(stats)
}

/// A preset [`PageSize`] as (width, height) in millimetres — the unit `PdfExportOptions` uses.
fn pdf_page_mm(size: PageSize) -> (f32, f32) {
    match size {
        PageSize::A4 => (210.0, 297.0),
        PageSize::Letter => (215.9, 279.4),
        PageSize::A5 => (148.0, 210.0),
    }
}

/// One inch in twips (DOCX's twentieth-of-a-point length unit).
const TWIPS_PER_IN: f32 = 1440.0;

/// Map a [`Preset`] onto DOCX page geometry + base typography (all in DOCX units), plus a
/// manuscript running header from the Work's author + title. Only DOCX (and later PDF) honour
/// these; the free text formats ignore them. This is where the manuscript presets become
/// *effective* — page size, margins, font, double-spacing, first-line indent, ragged/justified
/// alignment, and page-numbered header all flow from here; per-block RTL is emitted by the
/// exporter itself from each block's direction, so it needs no option.
fn docx_options(preset: &Preset, work_title: &str, work_author: &str) -> DocxExportOptions {
    let (page_w, page_h) = match preset.page_size {
        // Twips = inch × 1440. A4 = 210×297 mm, A5 = 148×210 mm, US Letter = 8.5×11 in.
        PageSize::A4 => (11906u32, 16838u32),
        PageSize::Letter => (12240, 15840),
        PageSize::A5 => (8391, 11906),
    };
    let m = &preset.margin;
    let in_to_twips = |i: f32| (i * TWIPS_PER_IN).round() as i32;
    DocxExportOptions {
        page_width_twips: Some(page_w),
        page_height_twips: Some(page_h),
        margin_top_twips: Some(in_to_twips(m.top_in)),
        margin_bottom_twips: Some(in_to_twips(m.bottom_in)),
        margin_left_twips: Some(in_to_twips(m.left_in)),
        margin_right_twips: Some(in_to_twips(m.right_in)),
        font_family: (!preset.font_family.trim().is_empty()).then(|| preset.font_family.clone()),
        font_half_points: Some((preset.font_size_pt * 2.0).round().max(2.0) as usize),
        line_spacing_twips: Some(match preset.line_spacing {
            LineSpacing::Single => 240,
            LineSpacing::OneAndHalf => 360,
            LineSpacing::Double => 480,
        }),
        first_line_indent_twips: (preset.first_line_indent_in > 0.0)
            .then(|| in_to_twips(preset.first_line_indent_in)),
        paragraph_spacing_after_twips: (preset.paragraph_spacing_pt > 0.0)
            .then(|| (preset.paragraph_spacing_pt * 20.0).round() as i32),
        justify: preset.justify,
        page_numbers: true,
        running_header: manuscript_header(work_title, work_author),
    }
}

/// The right-aligned running header text — `"Author / TITLE"`, or one side, or `None` when both
/// are blank (the page number is emitted regardless).
fn manuscript_header(title: &str, author: &str) -> Option<String> {
    let (title, author) = (title.trim(), author.trim());
    match (author.is_empty(), title.is_empty()) {
        (true, true) => None,
        (false, true) => Some(author.to_string()),
        (true, false) => Some(title.to_uppercase()),
        (false, false) => Some(format!("{author} / {}", title.to_uppercase())),
    }
}

fn text_render(doc: &TextDocument, format: ExportFormat) -> Result<String> {
    Ok(match format {
        ExportFormat::Djot => doc.to_djot()?,
        // The *indented* plain text: `.txt` has no markup to mark quoted matter, so an
        // epigraph (or any block quotation) would otherwise dissolve into the body. The
        // flush `to_plain_text()` is the addressable view search computes offsets against
        // and deliberately stays unindented — this is a file being written out, so it
        // wants the presentation form.
        ExportFormat::PlainText => doc.to_plain_text_indented()?,
        ExportFormat::Markdown => doc.to_markdown()?,
        ExportFormat::Html => doc.to_html()?,
        ExportFormat::Latex => doc.to_latex("article", true)?,
        other => return Err(anyhow!("{other:?} is not a text format")),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Assembly
// ─────────────────────────────────────────────────────────────────────────────

fn assemble(
    req: &RenderRequest,
    progress: &dyn Fn(f32),
    cancel: &AtomicBool,
) -> Result<(
    TextDocument,
    RenderStats,
    std::collections::BTreeSet<String>,
)> {
    let rows = flatten(req);
    let preset = req.preset;

    // The effective languages of the included rows — the PDF arm uses these to decide which
    // RTL faces to embed, so it needn't re-flatten the tree just to recompute them.
    let langs: std::collections::BTreeSet<String> = rows.iter().map(|r| r.lang.clone()).collect();

    // Heading levels are dense over the structural levels actually present, so a
    // chapter-only export starts at h1 and a book+chapter export (no parts) uses h1/h2.
    let mut present_depths: Vec<u8> = rows
        .iter()
        .filter_map(|r| level_of(&r.item.sub_role))
        .map(depth)
        .collect();
    present_depths.sort_unstable();
    present_depths.dedup();

    let mut out = String::new();
    let mut counters = Counters::default();
    let mut words = 0usize;
    // Block attributes a scene break has queued for the *next* prose block: the
    // suppressed first-line indent, plus the extra leading a `BlankLine` break
    // renders as. It outlives the row loop because the paragraph following a
    // break may belong to the next item.
    let mut pending_attrs: Vec<String> = Vec::new();

    let work_rtl = is_rtl_row(preset, req.work_lang);

    // Optional title page (front matter) for a book export.
    if preset.book_title_page && rows.iter().any(|r| r.item.sub_role.opens_book()) {
        let w = &req.gathered.work;
        if !w.title.is_empty() {
            push_heading(&mut out, 1, &w.title, work_rtl);
        }
        // The author's name is *data*, never markup — escaped so a name that
        // happens to start like a list marker survives intact.
        if !w.author_name.trim().is_empty() {
            push_para(
                &mut out,
                &escape_block_leading(w.author_name.trim()),
                work_rtl,
            );
        }
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

        let heading_lang = match &preset.heading_language {
            HeadingLanguage::Fixed(l) => l.clone(),
            HeadingLanguage::Auto => row.lang.clone(),
        };
        let row_rtl = is_rtl_row(preset, &row.lang);
        let heading_rtl = is_rtl_row(preset, &heading_lang);
        let mut contributed = false;

        // 1. A structural heading, if this item opens a level; else a scene title, if kept.
        if let Some(level) = level_of(&row.item.sub_role) {
            counters.bump(level);
            // Opening a structural level restarts the flow, so whatever a
            // trailing break queued for "the next paragraph" stops here. This
            // keys off the *structure*, not off whether a heading actually
            // printed: a preset whose chapter scheme is `None` emits no text,
            // and gating on that would let a break bleed across the seam.
            pending_attrs.clear();
            // A book is titled, not numbered — and the title page (if on) already carries
            // it, so the opener then emits nothing. Parts/chapters use their own schemes.
            let scheme = match level {
                Level::Book if preset.book_title_page => HeadingScheme::None,
                Level::Book => HeadingScheme::TitleOnly,
                Level::Part => preset.part_heading,
                Level::Chapter => preset.chapter_heading,
            };
            if let Some(text) = heading_text(row, level, &counters, &heading_lang, preset, scheme) {
                let lvl = present_depths
                    .iter()
                    .position(|&d| d == depth(level))
                    .map(|p| (p + 1).min(6))
                    .unwrap_or(1) as u8;
                push_heading(&mut out, lvl, &text, heading_rtl);
                contributed = true;
            }
        } else if preset.include_scene_titles && row.item.sub_role.carries_scene() {
            // A plain titled Scene (never a ChapterScene — that already emitted its chapter
            // heading above): the title heading stands in for the scene break.
            let t = row.item.title.trim();
            if !t.is_empty() {
                push_heading(&mut out, scene_title_level, t, row_rtl);
                pending_attrs.clear();
                contributed = true;
            }
        }

        // 2. The epigraph, if this row heads a book/part/chapter and the preset keeps it.
        //    Between the heading and the prose, which is where CMOS puts it: after the
        //    chapter number/title, before the body text.
        if preset.include_epigraphs
            && let Some(epi) = content_of(row.contents, ContentRole::EpigraphText)
            && !epi.trim().is_empty()
        {
            // Never scanned for break markers: an epigraph is quoted matter, not the
            // scene flow a break divides. Its words are not the manuscript's either, so
            // the count is discarded exactly as the synopsis's is below — an epigraph
            // must not move a pace target.
            let (_, emitted) = push_prose(&mut out, epi, row_rtl, preset, false, &mut pending_attrs);
            if emitted {
                contributed = true;
                // New Hart's Rule: the first line after a heading, an epigraph or a
                // section break carries no first-line indent. Queued the same way a
                // scene break queues it, and consumed by this row's own prose below —
                // `push_prose` only lets manuscript prose (`scan_markers`) inherit the
                // queue, so a synopsis can never swallow it by mistake.
                //
                // **Only when this row has prose of its own.** A Part or a Book has none,
                // so the queue would outlive the row and land on whatever prose came
                // next — a following Scene's opening paragraph, which is a different
                // paragraph entirely and is entitled to its indent. Relying on "the next
                // structural heading clears it" is not enough: the binder is
                // organisational, so nothing guarantees a heading row comes next.
                if main_prose_role(&row.item.sub_role)
                    .is_some_and(|role| content_of(row.contents, role).is_some())
                {
                    pending_attrs.push("text_indent=0".to_string());
                }
            }
        }

        // 3. The main prose (a scene's SceneText, a note's NoteText) — appended verbatim
        //    since it is already Djot.
        if let Some(role) = main_prose_role(&row.item.sub_role)
            && let Some(prose) = content_of(row.contents, role)
        {
            // Only a scene's own prose is scanned for break markers. A
            // marker typed into a Note is just literal text — the model is
            // about scene flow.
            let scan = row.item.sub_role.carries_scene();
            // A scene whose whole prose is a single break marker emits no
            // prose at all, so it must not be counted as an emitted item —
            // the marker is furniture, and the row contributed nothing.
            let (w, emitted) =
                push_prose(&mut out, prose, row_rtl, preset, scan, &mut pending_attrs);
            words += w;
            contributed |= emitted;
        }

        // 4. The synopsis, if the preset keeps it.
        if preset.include_synopses
            && let Some(syn) = content_of(row.contents, ContentRole::SynopsisText)
        {
            // A synopsis is commentary, not the scene's prose — never
            // scanned for markers, and its words are not the manuscript's.
            let (_, emitted) =
                push_prose(&mut out, syn, row_rtl, preset, false, &mut pending_attrs);
            contributed |= emitted;
        }

        if contributed {
            emitted_items += 1;
        }
        report(i);
    }

    let doc = TextDocument::new();
    if !out.trim().is_empty() {
        doc.set_djot(&out)?
            .wait()
            .map_err(|e| anyhow!("parsing the compiled document: {e:#}"))?;
    }
    // Document text direction from the export's language (v1 is whole-document; a mixed
    // LTR/RTL book uses its dominant language here — per-scene direction is a refinement).
    if let Some(dir) = document_direction(req, &rows) {
        doc.set_text_direction(dir)?;
    }

    Ok((
        doc,
        RenderStats {
            items: emitted_items,
            words,
        },
        langs,
    ))
}

/// Flatten the frozen tree into the included rows, in document order, each with its
/// resolved language. `include` decides membership; document order decides sequence (so a
/// Choose… selection still exports top-to-bottom). `activated` is a defensive guard — a
/// trashed item never exports even if its id reaches the set.
fn flatten<'a>(req: &'a RenderRequest) -> Vec<Row<'a>> {
    let want: std::collections::HashSet<u64> = req.include.iter().copied().collect();
    let mut rows = Vec::new();
    // `tags_in_binder` speaks lists; the renderer speaks one tag per row. Built once,
    // outside the loop, and via the shared parser so an empty work language yields no
    // tags rather than a list holding one empty string.
    let work_langs = language::parse_legacy_list(req.work_lang);
    for bwi in &req.gathered.binders {
        let items: Vec<BinderItem> = bwi.items.iter().map(|iwc| iwc.item.clone()).collect();
        let mut langs = std::collections::HashMap::new();
        language::tags_in_binder(&work_langs, &items, &mut langs);
        for iwc in &bwi.items {
            if want.contains(&iwc.item.id) && iwc.item.activated {
                let lang = langs
                    .get(&iwc.item.id)
                    .map(|l| language::primary(l).to_string())
                    .filter(|l| !l.is_empty())
                    .unwrap_or_else(|| req.work_lang.to_string());
                rows.push(Row {
                    item: &iwc.item,
                    contents: &iwc.contents,
                    lang,
                });
            }
        }
    }
    rows
}

// ─────────────────────────────────────────────────────────────────────────────
// Djot builders
// ─────────────────────────────────────────────────────────────────────────────

/// The `direction` attribute pair for a row, or `None` for LTR. The single
/// encoding of the key — [`dir_attr`] and every attribute set below build from
/// it, so the spelling lives in one place.
fn dir_pair(rtl: bool) -> Option<&'static str> {
    rtl.then_some("direction=rtl")
}

/// A Djot block-attribute line that sets direction, or empty. `{direction=rtl}` on the line
/// before a block is what text-document's own Djot exporter writes and its importer reads.
fn dir_attr(rtl: bool) -> String {
    match dir_pair(rtl) {
        Some(pair) => format!("{{{pair}}}\n"),
        None => String::new(),
    }
}

fn push_heading(out: &mut String, level: u8, text: &str, rtl: bool) {
    out.push_str(&dir_attr(rtl));
    for _ in 0..level.clamp(1, 6) {
        out.push('#');
    }
    out.push(' ');
    out.push_str(text.trim());
    out.push_str("\n\n");
}

fn push_para(out: &mut String, text: &str, rtl: bool) {
    out.push_str(&dir_attr(rtl));
    out.push_str(text.trim());
    out.push_str("\n\n");
}

/// Append a row's prose, block by block. The prose itself is already Djot and
/// goes in verbatim — the exporter generates furniture, it never rewrites what
/// the author wrote.
///
/// Every block is inspected when `scan_markers` is set: a scene-break marker the
/// author placed is replaced by the preset's rendering for its tier (and does
/// not count as prose), while an ordinary block is emitted with whatever block
/// attributes apply — RTL direction, plus anything a preceding break queued for
/// it. They are merged into **one** `{...}` line, because a block carries a
/// single attribute set.
///
/// Returns `(words, emitted_prose)` — the word count with markers excluded, and
/// whether any actual prose block reached the document. A scene whose entire
/// content is a break marker emits furniture but no prose, and must not be
/// counted as an emitted item.
fn push_prose(
    out: &mut String,
    djot: &str,
    rtl: bool,
    preset: &Preset,
    scan_markers: bool,
    pending: &mut Vec<String>,
) -> (usize, bool) {
    let trimmed = djot.trim();
    // Fast path — and a correctness guard, not just an optimisation. Splitting
    // on `"\n\n"` is not a Djot block split: a list with an indented
    // continuation, or a fenced block containing a blank line, is one construct
    // spanning several chunks. When there is nothing to scan for and no
    // attribute to attach, the prose goes in verbatim exactly as it always did,
    // so those constructs cannot be disturbed at all.
    if trimmed.is_empty() {
        return (0, false);
    }
    let could_hold_marker = scan_markers && scene_break::might_contain_marker(trimmed);
    if !rtl && pending.is_empty() && !could_hold_marker {
        out.push_str(trimmed);
        out.push_str("\n\n");
        return (trimmed.split_whitespace().count(), true);
    }
    let mut words = 0usize;
    let mut emitted_prose = false;
    // Non-marker blocks are flushed as one contiguous, verbatim run rather than
    // one at a time. `split("\n\n")` is not a Djot block split — a fenced block
    // containing a blank line is one construct spanning several chunks — so
    // rejoining a run reverses the split exactly and cannot disturb it. Only the
    // first block of a run carries the queued attributes.
    let blocks: Vec<&str> = trimmed.split("\n\n").collect();
    let mut offsets: Vec<(usize, usize)> = Vec::with_capacity(blocks.len());
    let mut cursor = 0usize;
    for b in &blocks {
        offsets.push((cursor, cursor + b.len()));
        cursor += b.len() + 2;
    }
    let mut run_start: Option<usize> = None;
    let flush = |out: &mut String, range: Option<(usize, usize)>, pending: &mut Vec<String>| {
        let Some((a, b)) = range else { return false };
        let text = trimmed[a..b.min(trimmed.len())].trim_end();
        if text.trim().is_empty() {
            return false;
        }
        let mut attrs: Vec<String> = Vec::new();
        attrs.extend(dir_pair(rtl).map(str::to_string));
        // Only manuscript prose inherits what a break queued. A synopsis or a
        // note between two scenes is commentary, not the flow the break divides.
        if scan_markers {
            attrs.append(pending);
        }
        if !attrs.is_empty() {
            out.push('{');
            out.push_str(&attrs.join(" "));
            out.push_str("}\n");
        }
        out.push_str(text);
        out.push_str("\n\n");
        true
    };
    for (i, block) in blocks.iter().enumerate() {
        let tier = if scan_markers {
            scene_break::tier_of_djot_block(block)
        } else {
            None
        };
        let Some(tier) = tier else {
            // RTL needs its attribute on every block, so it cannot batch.
            if rtl {
                if flush(out, Some(offsets[i]), pending) {
                    emitted_prose = true;
                }
            } else {
                run_start.get_or_insert(offsets[i].0);
            }
            words += block.split_whitespace().count();
            continue;
        };
        if flush(out, run_start.take().map(|a| (a, offsets[i].0)), pending) {
            emitted_prose = true;
        }
        let style = match tier {
            SceneBreakTier::Minor => &preset.scene_break,
            SceneBreakTier::Major => &preset.major_scene_break,
        };
        push_scene_break(out, style, preset, rtl, pending);
    }
    if flush(out, run_start.take().map(|a| (a, trimmed.len())), pending) {
        emitted_prose = true;
    }
    (words, emitted_prose)
}

/// The gap a `BlankLine` break opens above the paragraph that follows it, in
/// logical pixels — one empty line *at this preset's body size*, so it stays
/// "about a blank line" whether the manuscript is set at 8pt or 24pt.
///
/// It has to be a real margin: Djot cannot express "one blank line" on its own,
/// since consecutive blank lines collapse into the ordinary block separator.
/// 96 px = 72 pt, hence the 4/3.
fn blank_line_gap_px(preset: &Preset) -> i64 {
    let multiple = match preset.line_spacing {
        LineSpacing::Single => 1.0,
        LineSpacing::OneAndHalf => 1.5,
        LineSpacing::Double => 2.0,
    };
    // A preset that already spaces paragraphs contributes part of the gap on its
    // own; only the remainder is needed here, or a spaced preset would open a
    // visibly double break.
    let line_px = (preset.font_size_pt as f64) * 4.0 / 3.0 * multiple;
    let already_px = (preset.paragraph_spacing_pt as f64) * 4.0 / 3.0;
    (line_px - already_px).round().max(1.0) as i64
}

/// Render a scene break the author marked, and queue the block attributes that
/// belong on the paragraph after it.
///
/// Both visible tiers suppress that paragraph's first-line indent, which is what
/// print typography actually does at a scene break — the indent would otherwise
/// read as an ordinary new paragraph and undo the separation.
fn push_scene_break(
    out: &mut String,
    sep: &SceneBreak,
    preset: &Preset,
    rtl: bool,
    pending: &mut Vec<String>,
) {
    // The most recent break wins: two markers in a row must not stack their
    // attributes onto the same following paragraph.
    pending.clear();
    match sep {
        // Rendered as nothing at all, so the following paragraph is ordinary too.
        SceneBreak::None => {}
        SceneBreak::BlankLine => {
            pending.push(format!("top_margin={}", blank_line_gap_px(preset)));
            pending.push("text_indent=0".to_string());
        }
        SceneBreak::Glyph(g) => {
            // Centred, as a dinkus is set in print — and carrying the row's own
            // direction, so the glyph line does not fall back to LTR inside an
            // otherwise right-to-left manuscript.
            let mut attrs = vec!["alignment=center".to_string()];
            attrs.extend(dir_pair(rtl).map(str::to_string));
            out.push('{');
            out.push_str(&attrs.join(" "));
            out.push_str("}\n");
            // Escape a leading Djot block-marker so the glyph stays literal text — an
            // unescaped `#` is a heading, `* * *` / `---` a (dropped) thematic break.
            out.push_str(&escape_block_leading(g.trim()));
            out.push_str("\n\n");
            pending.push("text_indent=0".to_string());
        }
    }
}

/// Escape a leading Djot block marker so the line is read as a plain paragraph.
///
/// Two different families of marker can hijack the start of a line, and they need
/// *different* escapes — a backslash only escapes punctuation in Djot, so putting one
/// in front of a letter or digit leaves a literal backslash on the page:
///
/// * a **single special character** (`#` heading, `>` quote, `*`/`-`/`+` bullet, …).
///   Escape the character itself: `\* Star` → `* Star`.
/// * a **two-character list marker** — an alphanumeric followed by `.` or `)`, which
///   Djot reads as an ordered list (numeric `1.`, alphabetic `A.`, roman `i.`).
///   Escape the *punctuation*, not the leading character: `A\. Writer` → `A. Writer`,
///   whereas `\A. Writer` renders the backslash literally and `A. Writer` silently
///   loses the `A.` — the marker is consumed and only "Writer" survives.
fn escape_block_leading(s: &str) -> String {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        // Ordered-list marker: escape the `.`/`)` that makes it one.
        (Some(c), Some(p)) if c.is_alphanumeric() && (p == '.' || p == ')') => {
            let mut out = String::with_capacity(s.len() + 1);
            out.push(c);
            out.push('\\');
            out.push_str(&s[c.len_utf8()..]);
            out
        }
        (Some(c), _) if "#*-_+>~:|=`".contains(c) => format!("\\{s}"),
        _ => s.to_string(),
    }
}

/// Whether a row's text lays out right-to-left, honouring a forced preset direction.
fn is_rtl_row(preset: &Preset, lang: &str) -> bool {
    match preset.direction {
        DirectionMode::ForceRtl => true,
        DirectionMode::ForceLtr => false,
        DirectionMode::Auto => language::is_rtl(lang),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Small helpers
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default)]
struct Counters {
    book: usize,
    part: usize,
    chapter: usize,
}
impl Counters {
    fn bump(&mut self, level: Level) {
        match level {
            Level::Book => self.book += 1,
            Level::Part => self.part += 1,
            Level::Chapter => self.chapter += 1,
        }
    }
    fn number(&self, level: Level) -> usize {
        match level {
            Level::Book => self.book,
            Level::Part => self.part,
            Level::Chapter => self.chapter,
        }
    }
}

fn level_of(sr: &BinderItemSubRole) -> Option<Level> {
    if sr.opens_book() {
        Some(Level::Book)
    } else if sr.opens_part() {
        Some(Level::Part)
    } else if sr.opens_chapter() {
        Some(Level::Chapter)
    } else {
        None
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
    } else {
        None
    }
}

fn content_of(contents: &[Content], role: ContentRole) -> Option<&str> {
    contents
        .iter()
        .find(|c| c.role == role && c.activated && !c.data.is_empty())
        .map(|c| c.data.as_str())
}

/// The item's own title content (ChapterTitle / PartTitle / BookTitle), falling back to the
/// binder-tree title.
fn title_of<'a>(row: &'a Row) -> Option<&'a str> {
    for role in [
        ContentRole::ChapterTitle,
        ContentRole::PartTitle,
        ContentRole::BookTitle,
    ] {
        if let Some(t) = content_of(row.contents, role) {
            return Some(t);
        }
    }
    (!row.item.title.is_empty()).then_some(row.item.title.as_str())
}

fn heading_text(
    row: &Row,
    level: Level,
    counters: &Counters,
    lang: &str,
    preset: &Preset,
    scheme: HeadingScheme,
) -> Option<String> {
    let numbered = || headings::numbered(lang, level, counters.number(level), preset.digit_style);
    match scheme {
        HeadingScheme::None => None,
        HeadingScheme::Numbered => Some(numbered()),
        HeadingScheme::TitleOnly => title_of(row)
            .map(str::to_string)
            .or_else(|| Some(numbered())),
        HeadingScheme::NumberAndTitle => Some(match title_of(row) {
            Some(t) => format!("{} — {t}", numbered()),
            None => numbered(),
        }),
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
mod tests {
    use super::*;
    use common::entities::{Binder, BinderItem, BinderItemRole, BinderItemSubRole as SR, Work};
    use skrib_format::{BinderWithItems, ItemWithContents};

    use crate::preset::builtin_presets;

    fn c(id: u64, role: ContentRole, data: &str) -> Content {
        Content {
            id,
            activated: true,
            role,
            data: data.to_string(),
            ..Default::default()
        }
    }

    fn iwc(id: u64, sub_role: SR, lang: &str, contents: Vec<Content>) -> ItemWithContents {
        ItemWithContents {
            item: BinderItem {
                id,
                role: BinderItemRole::Item,
                sub_role,
                // Split, not wrapped: `iwc(.., "", ..)` must mean "no language, so
                // inherit" — wrapping made it `[""]`, which reads as tagged.
                dict_language: language::parse_legacy_list(lang),
                is_exportable: true,
                activated: true,
                ..Default::default()
            },
            contents,
        }
    }

    fn gathered(items: Vec<ItemWithContents>, work_lang: &str) -> Gathered {
        Gathered {
            work: Work {
                id: 1,
                title: "My Novel".into(),
                author_name: "A. Writer".into(),
                dict_language: language::parse_legacy_list(work_lang),
                ..Default::default()
            },
            tags: vec![],
            dict_words: vec![],
            text_replacement_rules: vec![],
            note_templates: vec![],
            smart_punctuation: None,
            trash_infos: vec![],
            paces: vec![],
            progress_snapshots: vec![],
            comments: vec![],
            binders: vec![BinderWithItems {
                binder: Binder {
                    id: 10,
                    ..Default::default()
                },
                items,
            }],
            work_info: None,
        }
    }

    fn preset(id: &str) -> Preset {
        builtin_presets().into_iter().find(|p| p.id == id).unwrap()
    }

    /// A flat one-book fixture: book title, one chapter with two scenes.
    fn flat_book() -> Gathered {
        gathered(
            vec![
                iwc(
                    100,
                    SR::BookBegin,
                    "en",
                    vec![c(1, ContentRole::BookTitle, "My Novel")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![
                        c(2, ContentRole::ChapterTitle, "Storms"),
                        c(3, ContentRole::SceneText, "The wind rose over the hills."),
                    ],
                ),
                iwc(
                    102,
                    SR::Scene,
                    "en",
                    vec![c(4, ContentRole::SceneText, "She walked on into the dark.")],
                ),
            ],
            "en",
        )
    }

    /// A chapter carrying an epigraph, so the ordering and word-count rules have
    /// something to bite on. The attribution rides inside the same blockquote, which is
    /// what keeps it attached to its quotation through every writer.
    fn book_with_epigraph() -> Gathered {
        gathered(
            vec![
                iwc(
                    100,
                    SR::BookBegin,
                    "en",
                    vec![c(1, ContentRole::BookTitle, "My Novel")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![
                        c(2, ContentRole::ChapterTitle, "Storms"),
                        c(
                            5,
                            ContentRole::EpigraphText,
                            "> Salt is the only honest preservative.\n>\n> {alignment=right}\n> — M. Ferrand",
                        ),
                        c(3, ContentRole::SceneText, "The wind rose over the hills."),
                    ],
                ),
            ],
            "en",
        )
    }

    fn req<'a>(
        g: &'a Gathered,
        include: &'a [u64],
        p: &'a Preset,
        f: ExportFormat,
    ) -> RenderRequest<'a> {
        RenderRequest {
            gathered: g,
            include,
            preset: p,
            format: f,
            work_lang: "en",
            explicit_selection: false,
        }
    }

    #[test]
    fn renders_a_flat_book_to_html_with_headings_and_prose() {
        let g = flat_book();
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        assert!(html.contains("My Novel"), "book title: {html}");
        assert!(
            html.contains("Chapter 1 — Storms"),
            "chapter heading: {html}"
        );
        assert!(html.contains("The wind rose over the hills."), "{html}");
        assert!(html.contains("She walked on into the dark."), "{html}");
        // The prose must NOT become a heading.
        assert!(
            !html.contains("<h2>The wind"),
            "prose leaked into a heading: {html}"
        );
    }

    #[test]
    fn every_text_format_renders_the_prose() {
        let g = flat_book();
        let p = preset("neutral");
        for f in [
            ExportFormat::Djot,
            ExportFormat::PlainText,
            ExportFormat::Markdown,
            ExportFormat::Html,
            ExportFormat::Latex,
        ] {
            let out = render_to_string(&req(&g, &[100, 101, 102], &p, f)).unwrap();
            assert!(out.contains("She walked on into the dark"), "{f:?}: {out}");
        }
    }

    #[test]
    fn chapter_heading_follows_content_language_by_default() {
        // The built-in manuscript presets no longer force a language — headings follow each
        // scene's own resolved language (the fixture's scenes are "en").
        let g = flat_book();
        let p = preset("manuscript-fr");
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        assert!(
            html.contains("Chapter 1"),
            "content-language heading: {html}"
        );
        assert!(!html.contains("Chapitre 1"));
    }

    #[test]
    fn a_fixed_heading_language_overrides_the_content_language() {
        // A preset MAY still pin a heading language (`HeadingLanguage::Fixed`); when it does,
        // that wins over the scene's own language.
        let g = flat_book();
        let p = Preset {
            heading_language: HeadingLanguage::Fixed("fr".to_string()),
            ..preset("manuscript-fr")
        };
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        assert!(html.contains("Chapitre 1"), "forced french heading: {html}");
        assert!(!html.contains("Chapter 1"));
    }

    #[test]
    fn an_arabic_work_sets_rtl_direction() {
        let g = gathered(
            vec![iwc(
                200,
                SR::Scene,
                "ar",
                vec![c(9, ContentRole::SceneText, "نص عربي هنا.")],
            )],
            "ar",
        );
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[200], &p, ExportFormat::Html)).unwrap();
        assert!(
            html.contains("rtl"),
            "RTL direction should reach the HTML: {html}"
        );
    }

    /// `flat_book`, but the second scene's prose opens with an author-placed
    /// minor break marker — in the ESCAPED form the editor really persists.
    fn book_with_marker(marker: &str) -> Gathered {
        gathered(
            vec![
                iwc(
                    100,
                    SR::BookBegin,
                    "en",
                    vec![c(1, ContentRole::BookTitle, "My Novel")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![
                        c(2, ContentRole::ChapterTitle, "Storms"),
                        c(3, ContentRole::SceneText, "The wind rose over the hills."),
                    ],
                ),
                iwc(
                    102,
                    SR::Scene,
                    "en",
                    vec![c(
                        4,
                        ContentRole::SceneText,
                        &format!("{marker}\n\nShe walked on into the dark."),
                    )],
                ),
            ],
            "en",
        )
    }

    #[test]
    fn adjacent_scene_items_do_not_break_by_default() {
        // The binder is organisational: two adjacent scenes say nothing about typography,
        // so without a marker the prose must run straight on, even with a glyph configured.
        let g = flat_book();
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        p.major_scene_break = SceneBreak::Glyph("+++".to_string());
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(
            !txt.contains("###"),
            "no marker, so no break may appear: {txt}"
        );
        assert!(
            !txt.contains("+++"),
            "no marker, so no break may appear: {txt}"
        );
    }

    #[test]
    fn an_authored_marker_renders_as_the_presets_glyph() {
        let g = book_with_marker("\\* \\* \\*");
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(
            txt.contains("###"),
            "the marker must render as the glyph: {txt}"
        );
        assert!(
            !txt.contains("* * *"),
            "the marker itself must be consumed, not printed: {txt}"
        );
    }

    // ── the author on the compiled title page ──

    /// A book exported with a title-page preset carries the writer's name.
    /// The whole point of storing `author_name`: it must reach the page.
    #[test]
    fn a_title_page_preset_prints_the_author_under_the_title() {
        let g = flat_book();
        let p = preset("manuscript-shunn");
        assert!(p.book_title_page, "fixture preset must have a title page");
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("A. Writer"), "the author must appear: {txt}");
        assert!(txt.contains("My Novel"), "the title must appear: {txt}");
        assert!(
            txt.find("My Novel") < txt.find("A. Writer"),
            "the author belongs under the title: {txt}"
        );
    }

    /// The name is **optional**, and blank must mean "omit" — not an empty line
    /// where a name would be, and not the string "Untitled" or similar.
    #[test]
    fn an_empty_author_is_omitted_from_the_title_page() {
        let mut g = flat_book();
        g.work.author_name = String::new();
        let p = preset("manuscript-shunn");
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("My Novel"), "the title still appears: {txt}");
        assert!(
            !txt.contains("A. Writer"),
            "a cleared author must not linger: {txt}"
        );
    }

    /// An author whose name starts like an ordered-list marker must survive — see
    /// [`escape_block_leading`] for why the escape goes before the `.`, not the letter.
    #[test]
    fn an_author_named_like_a_list_marker_is_not_eaten() {
        for name in ["A. Writer", "1. Writer", "i. Writer", "J.R.R. Writer"] {
            let mut g = flat_book();
            g.work.author_name = name.to_string();
            let p = preset("manuscript-shunn");
            let txt =
                render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
            assert!(
                txt.contains(name),
                "the author {name:?} must appear verbatim, got: {txt}"
            );
            assert!(
                !txt.contains('\\'),
                "no escape may leak onto the page for {name:?}: {txt}"
            );
        }
    }

    /// The escape helper itself, per marker family — a backslash belongs before
    /// punctuation only.
    #[test]
    fn block_leading_escapes_pick_the_right_character() {
        // Single special character: escape it directly.
        assert_eq!(escape_block_leading("* Star"), "\\* Star");
        assert_eq!(escape_block_leading("# Sharp"), "\\# Sharp");
        // Ordered-list markers: escape the punctuation, not the alphanumeric.
        assert_eq!(escape_block_leading("A. Writer"), "A\\. Writer");
        assert_eq!(escape_block_leading("1. Thing"), "1\\. Thing");
        assert_eq!(escape_block_leading("i) Roman"), "i\\) Roman");
        // Nothing special: left alone.
        assert_eq!(escape_block_leading("Plain name"), "Plain name");
    }

    /// The running header degrades sensibly rather than printing a stray
    /// separator when only one of the two halves is set.
    #[test]
    fn the_running_header_handles_a_missing_author_or_title() {
        assert_eq!(
            manuscript_header("My Novel", "A. Writer").as_deref(),
            Some("A. Writer / MY NOVEL")
        );
        assert_eq!(
            manuscript_header("My Novel", "  ").as_deref(),
            Some("MY NOVEL"),
            "no author: no leading separator"
        );
        assert_eq!(
            manuscript_header("", "A. Writer").as_deref(),
            Some("A. Writer"),
            "no title: no trailing separator"
        );
        assert_eq!(manuscript_header("", "").as_deref(), None);
    }

    #[test]
    fn the_two_tiers_render_distinctly() {
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        p.major_scene_break = SceneBreak::Glyph("+++".to_string());

        let minor = render_to_string(&req(
            &book_with_marker("\\* \\* \\*"),
            &[100, 101, 102],
            &p,
            ExportFormat::PlainText,
        ))
        .unwrap();
        assert!(minor.contains("###") && !minor.contains("+++"), "{minor}");

        let major = render_to_string(&req(
            &book_with_marker("\\# # #"),
            &[100, 101, 102],
            &p,
            ExportFormat::PlainText,
        ))
        .unwrap();
        assert!(major.contains("+++") && !major.contains("###"), "{major}");
    }

    #[test]
    fn a_marker_mid_scene_breaks_inside_one_item() {
        // The case a per-item flag or a marker item structurally cannot express:
        // a viewpoint shift inside one scene, with no binder change at all.
        let g = gathered(
            vec![iwc(
                100,
                SR::Scene,
                "en",
                vec![c(
                    1,
                    ContentRole::SceneText,
                    "She closed the door.\n\n\\* \\* \\*\n\nDawn found him waiting.",
                )],
            )],
            "en",
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let txt = render_to_string(&req(&g, &[100], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("She closed the door."), "{txt}");
        assert!(txt.contains("###"), "mid-scene break must render: {txt}");
        assert!(txt.contains("Dawn found him waiting."), "{txt}");
    }

    #[test]
    fn every_accepted_spelling_of_a_marker_is_recognised() {
        // Guards the escaped forms specifically: these are the bytes that
        // actually reach `Content.data`, not idealised raw strings.
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        p.major_scene_break = SceneBreak::Glyph("+++".to_string());
        for (marker, expected) in [
            ("\\* \\* \\*", "###"),
            ("\\*\\*\\*", "###"),
            ("\\*", "###"),
            ("\\#", "###"),
            ("\\# # #", "+++"),
            ("\\#\\#\\#", "+++"),
        ] {
            let g = book_with_marker(marker);
            let txt =
                render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
            assert!(
                txt.contains(expected),
                "marker {marker:?} → {expected:?}: {txt}"
            );
        }
    }

    #[test]
    fn prose_that_merely_resembles_a_marker_is_left_alone() {
        // Exact-match by intent: ordinary prose containing an asterisk must not
        // be silently eaten and replaced by a scene break.
        let g = book_with_marker("He was \\*emphatic\\* about it.");
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("emphatic"), "prose must survive: {txt}");
        assert!(!txt.contains("###"), "prose must not become a break: {txt}");
    }

    #[test]
    fn a_blank_line_break_is_no_longer_identical_to_none() {
        // `BlankLine` renders as real leading plus a suppressed indent on the following
        // paragraph — what most of the world's publishing traditions actually use.
        let g = book_with_marker("\\* \\* \\*");
        let mut blank = preset("neutral");
        blank.scene_break = SceneBreak::BlankLine;
        let mut none = preset("neutral");
        none.scene_break = SceneBreak::None;

        let with_blank =
            render_to_string(&req(&g, &[100, 101, 102], &blank, ExportFormat::Html)).unwrap();
        let with_none =
            render_to_string(&req(&g, &[100, 101, 102], &none, ExportFormat::Html)).unwrap();
        assert_ne!(
            with_blank, with_none,
            "BlankLine must differ from None:\n{with_blank}"
        );
        assert!(
            with_blank.contains("margin-top"),
            "BlankLine must open a real gap: {with_blank}"
        );
    }

    #[test]
    fn a_break_suppresses_the_next_paragraphs_first_line_indent() {
        let g = book_with_marker("\\* \\* \\*");
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        assert!(
            html.contains("text-indent: 0px"),
            "the paragraph after a break must not be indented: {html}"
        );
    }

    #[test]
    fn a_glyph_break_is_centred() {
        let g = book_with_marker("\\* \\* \\*");
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        assert!(
            html.contains("text-align: center"),
            "a dinkus is centred in print: {html}"
        );
    }

    #[test]
    fn a_marker_does_not_count_as_prose_words() {
        let plain = flat_book();
        let marked = book_with_marker("\\* \\* \\*");
        let p = preset("neutral");
        let words_of = |g: &Gathered| {
            assemble(
                &req(g, &[100, 101, 102], &p, ExportFormat::PlainText),
                &|_| {},
                &AtomicBool::new(false),
            )
            .unwrap()
            .1
            .words
        };
        let (a, b) = (words_of(&plain), words_of(&marked));
        assert_eq!(a, b, "a scene break is furniture, not three words");
    }

    #[test]
    fn a_break_does_not_leak_across_a_chapter_boundary() {
        // A marker ending one chapter must not style the first paragraph of the next,
        // even when the preset emits no chapter heading to reset the flow.
        let g = gathered(
            vec![
                iwc(
                    100,
                    SR::ChapterScene,
                    "en",
                    vec![c(1, ContentRole::SceneText, "End of one.\n\n\\* \\* \\*")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![c(2, ContentRole::SceneText, "Start of two.")],
                ),
            ],
            "en",
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::BlankLine;
        p.chapter_heading = HeadingScheme::None;
        let html = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Html)).unwrap();
        assert!(
            !html.contains("margin-top"),
            "a trailing break must not carry into the next chapter: {html}"
        );
    }

    #[test]
    fn multi_paragraph_prose_survives_block_splitting_in_both_directions() {
        // `push_prose` now walks every block in both LTR and RTL. Nothing may be
        // lost or reordered by that — no existing test covered multi-block prose.
        for (lang, marker) in [("en", "\\* \\* \\*"), ("ar", "\\* \\* \\*")] {
            let g = gathered(
                vec![iwc(
                    100,
                    SR::Scene,
                    lang,
                    vec![c(
                        1,
                        ContentRole::SceneText,
                        &format!("One.\n\nTwo.\n\n{marker}\n\nThree.\n\nFour."),
                    )],
                )],
                lang,
            );
            let mut p = preset("neutral");
            p.scene_break = SceneBreak::Glyph("###".to_string());
            let txt = render_to_string(&req(&g, &[100], &p, ExportFormat::PlainText)).unwrap();
            for para in ["One.", "Two.", "Three.", "Four."] {
                assert!(txt.contains(para), "lang={lang} lost {para}: {txt}");
            }
            assert!(txt.contains("###"), "lang={lang}: {txt}");
        }
    }

    #[test]
    fn docx_options_map_the_manuscript_preset() {
        // Shunn: Times New Roman 12pt, double-spaced, 0.5" first-line indent, A4, ragged.
        let p = preset("manuscript-shunn");
        let o = docx_options(&p, "The Lighthouse", "Mara Vane");
        assert_eq!(o.font_family.as_deref(), Some("Times New Roman"));
        assert_eq!(o.font_half_points, Some(24), "12pt → 24 half-points");
        assert_eq!(o.line_spacing_twips, Some(480), "double spacing");
        assert_eq!(o.first_line_indent_twips, Some(720), "0.5\" → 720 twips");
        assert_eq!(o.page_width_twips, Some(11906), "A4 width");
        assert_eq!(o.margin_left_twips, Some(1440), "1\" margins by default");
        assert!(!o.justify, "Shunn manuscripts are ragged-right");
        assert!(o.page_numbers, "manuscript pages are numbered");
        assert_eq!(
            o.running_header.as_deref(),
            Some("Mara Vane / THE LIGHTHOUSE"),
            "author / TITLE running header"
        );
    }

    #[cfg(feature = "pdf")]
    #[test]
    fn pdf_export_writes_a_valid_pdf() {
        // manuscript-shunn names "Times New Roman" (no bundled bytes) → exercises the
        // EB Garamond substitution + a real embedded font.
        let g = flat_book();
        let p = preset("manuscript-shunn");
        let path = std::env::temp_dir().join(format!("skrib-export-{}.pdf", std::process::id()));
        let stats = render_to_file(
            &req(&g, &[100, 101, 102], &p, ExportFormat::Pdf),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(path.exists(), "pdf file should be written");
        let bytes = std::fs::read(&path).unwrap();
        assert!(bytes.starts_with(b"%PDF-"), "valid PDF magic bytes");
        assert!(bytes.len() > 500, "non-trivial PDF");
        assert!(stats.items >= 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn epub_export_writes_a_non_empty_file() {
        let g = flat_book();
        let p = preset("neutral");
        let path = std::env::temp_dir().join(format!("skrib-export-{}.epub", std::process::id()));
        let stats = render_to_file(
            &req(&g, &[100, 101, 102], &p, ExportFormat::Epub),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(path.exists(), "epub file should be written");
        assert!(
            std::fs::metadata(&path).unwrap().len() > 0,
            "epub should be non-empty"
        );
        assert!(stats.items >= 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn docx_export_writes_a_non_empty_file() {
        let g = flat_book();
        let p = preset("neutral");
        let path = std::env::temp_dir().join(format!("skrib-export-{}.docx", std::process::id()));
        let stats = render_to_file(
            &req(&g, &[100, 101, 102], &p, ExportFormat::Docx),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(path.exists(), "docx file should be written");
        assert!(
            std::fs::metadata(&path).unwrap().len() > 0,
            "docx should be non-empty"
        );
        assert!(stats.items >= 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_trashed_row_in_the_include_set_is_dropped_defensively() {
        let mut g = flat_book();
        g.binders[0].items[2].item.activated = false; // scene 102 trashed
        let p = preset("neutral");
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("The wind rose"));
        assert!(
            !txt.contains("She walked on"),
            "trashed scene must be excluded: {txt}"
        );
    }

    fn titled(mut iwc: ItemWithContents, title: &str) -> ItemWithContents {
        iwc.item.title = title.to_string();
        iwc
    }

    /// A book with a chapter (one scene) and a research note swept in after it.
    fn book_with_note() -> Gathered {
        gathered(
            vec![
                iwc(
                    100,
                    SR::BookBegin,
                    "en",
                    vec![c(1, ContentRole::BookTitle, "My Novel")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![
                        c(2, ContentRole::ChapterTitle, "Storms"),
                        c(3, ContentRole::SceneText, "The wind rose over the hills."),
                    ],
                ),
                iwc(
                    102,
                    SR::Note,
                    "en",
                    vec![c(4, ContentRole::NoteText, "Research: local weather.")],
                ),
            ],
            "en",
        )
    }

    #[test]
    fn a_swept_note_is_dropped_unless_the_preset_or_an_explicit_pick_keeps_it() {
        let g = book_with_note();
        let mut p = preset("neutral"); // include_notes = false
        // Swept into a whole-book export: the note is dropped by default.
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("The wind rose"), "{txt}");
        assert!(
            !txt.contains("Research"),
            "a swept note must be dropped by default: {txt}"
        );
        // The preset keeps notes → included.
        p.include_notes = true;
        let with =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(
            with.contains("Research"),
            "include_notes should keep the note: {with}"
        );
        // An explicit pick (Export Note / a checked item) overrides the toggle being off.
        p.include_notes = false;
        let ids = [102u64];
        let explicit = RenderRequest {
            explicit_selection: true,
            ..req(&g, &ids, &p, ExportFormat::PlainText)
        };
        let only = render_to_string(&explicit).unwrap();
        assert!(
            only.contains("Research"),
            "an explicit note pick overrides the toggle: {only}"
        );
    }

    #[test]
    fn scene_titles_are_emitted_only_when_the_preset_keeps_them() {
        let g = gathered(
            vec![
                titled(
                    iwc(
                        300,
                        SR::Scene,
                        "en",
                        vec![c(1, ContentRole::SceneText, "Alpha prose.")],
                    ),
                    "Opening",
                ),
                titled(
                    iwc(
                        301,
                        SR::Scene,
                        "en",
                        vec![c(2, ContentRole::SceneText, "Closing prose.")],
                    ),
                    "Ending",
                ),
            ],
            "en",
        );
        let mut p = preset("neutral"); // include_scene_titles = false
        let without = render_to_string(&req(&g, &[300, 301], &p, ExportFormat::Html)).unwrap();
        assert!(
            !without.contains("Opening"),
            "scene titles must not leak when off: {without}"
        );
        // Turned on: the titles head each scene (h1 here — no structural levels present).
        p.include_scene_titles = true;
        let with = render_to_string(&req(&g, &[300, 301], &p, ExportFormat::Html)).unwrap();
        assert!(
            with.contains("Opening") && with.contains("Ending"),
            "scene titles: {with}"
        );
        assert!(
            with.contains("<h1"),
            "scene titles head at h1 with no structural levels: {with}"
        );
    }

    #[test]
    fn a_synopsis_does_not_steal_the_breaks_styling() {
        // A break at the end of a scene belongs to the NEXT scene's opening
        // paragraph. A synopsis is commentary sitting between them, and must not
        // consume the queued attributes on its way past.
        let g = gathered(
            vec![
                iwc(
                    100,
                    SR::Scene,
                    "en",
                    vec![
                        c(1, ContentRole::SceneText, "End of one.\n\n\\* \\* \\*"),
                        c(2, ContentRole::SynopsisText, "A synopsis line."),
                    ],
                ),
                iwc(
                    101,
                    SR::Scene,
                    "en",
                    vec![c(3, ContentRole::SceneText, "Start of two.")],
                ),
            ],
            "en",
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::BlankLine;
        p.include_synopses = true;
        let html = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Html)).unwrap();
        let syn = html.split("A synopsis line").next().unwrap();
        assert!(
            !syn.ends_with("text-indent: 0px\">"),
            "the synopsis must not carry the break's styling: {html}"
        );
        assert!(
            html.contains("margin-top") && html.contains("Start of two."),
            "the next scene's opening paragraph must carry it: {html}"
        );
    }

    #[test]
    fn a_multi_block_construct_survives_alongside_a_marker() {
        // `split("\\n\\n")` is not a Djot block split: a fenced block containing a
        // blank line is ONE construct spanning two chunks. Scanning for markers
        // must not tear it apart — hence contiguous non-marker blocks are flushed
        // as a single verbatim run. The fence holds a `#` so the scan really runs.
        let g = gathered(
            vec![iwc(
                100,
                SR::Scene,
                "en",
                vec![c(
                    1,
                    ContentRole::SceneText,
                    "```\n# code a\n\ncode b\n```\n\n\\* \\* \\*\n\nAfter.",
                )],
            )],
            "en",
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let html = render_to_string(&req(&g, &[100], &p, ExportFormat::Html)).unwrap();
        assert!(
            html.matches("<pre>").count() == 1,
            "the fenced block must stay one construct: {html}"
        );
        assert!(html.contains("code a") && html.contains("code b"), "{html}");
        assert!(html.contains("###"), "the marker still renders: {html}");
    }

    #[test]
    fn a_marker_only_scene_is_not_counted_as_an_emitted_item() {
        let g = gathered(
            vec![iwc(
                100,
                SR::Scene,
                "en",
                vec![c(1, ContentRole::SceneText, "\\* \\* \\*")],
            )],
            "en",
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let stats = assemble(
            &req(&g, &[100], &p, ExportFormat::PlainText),
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap()
        .1;
        assert_eq!(stats.items, 0, "a marker is furniture, not an emitted item");
    }

    #[test]
    fn a_glyph_break_keeps_the_rows_direction_in_rtl() {
        let g = gathered(
            vec![iwc(
                100,
                SR::Scene,
                "ar",
                vec![c(
                    1,
                    ContentRole::SceneText,
                    "\u{0623}.\n\n\\* \\* \\*\n\n\u{0628}.",
                )],
            )],
            "ar",
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let html = render_to_string(&req(&g, &[100], &p, ExportFormat::Html)).unwrap();
        let glyph_para = html
            .split("###")
            .next()
            .and_then(|s| s.rfind("<p").map(|i| s[i..].to_string()))
            .unwrap_or_default();
        assert!(
            glyph_para.contains("rtl"),
            "the glyph line must not fall back to LTR: {html}"
        );
    }

    #[test]
    fn the_blank_line_gap_scales_with_body_size() {
        let g = book_with_marker("\\* \\* \\*");
        let gap = |pt: f32| {
            let mut p = preset("neutral");
            p.scene_break = SceneBreak::BlankLine;
            p.font_size_pt = pt;
            let html =
                render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
            html.split("margin-top: ")
                .nth(1)
                .and_then(|s| s.split("px").next())
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0)
        };
        assert!(
            gap(24.0) > gap(8.0),
            "a blank-line break must be about one line at the preset's own size"
        );
    }

    #[test]
    fn every_builtin_preset_exports_every_format_with_both_tiers() {
        // The end-to-end guard: a manuscript carrying both tiers must survive the
        // whole stack — recogniser → assembled Djot → text-document parse → each
        // renderer — under every regional style we ship, not just in unit tests.
        let g = gathered(
            vec![
                iwc(
                    100,
                    SR::BookBegin,
                    "en",
                    vec![c(1, ContentRole::BookTitle, "My Novel")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![
                        c(2, ContentRole::ChapterTitle, "Storms"),
                        c(
                            3,
                            ContentRole::SceneText,
                            "The wind rose.\n\n\\* \\* \\*\n\nShe waited.",
                        ),
                    ],
                ),
                iwc(
                    102,
                    SR::Scene,
                    "en",
                    vec![c(4, ContentRole::SceneText, "\\# # #\n\nA year passed.")],
                ),
            ],
            "en",
        );
        let text_formats = [
            ExportFormat::PlainText,
            ExportFormat::Markdown,
            ExportFormat::Html,
            ExportFormat::Djot,
        ];
        // A unique directory per run: a fixed shared path would let two
        // concurrent `cargo test` invocations overwrite each other's output and
        // let the first to finish delete files the second is still asserting on.
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path();
        for p in builtin_presets() {
            for f in text_formats {
                let out = render_to_string(&req(&g, &[100, 101, 102], &p, f))
                    .unwrap_or_else(|e| panic!("{} / {f:?}: {e:#}", p.id));
                // No trailing period in the needles: Markdown/Djot escape it as
                // `\.`, which is correct output, not a loss.
                assert!(out.contains("The wind rose"), "{} / {f:?}: {out}", p.id);
                assert!(out.contains("She waited"), "{} / {f:?}: {out}", p.id);
                assert!(out.contains("A year passed"), "{} / {f:?}: {out}", p.id);
            }
            // PDF rides the opt-in `pdf` feature (Typst is a heavy dependency),
            // so it joins the matrix only when that feature is on.
            #[cfg(feature = "pdf")]
            let binary = vec![ExportFormat::Docx, ExportFormat::Epub, ExportFormat::Pdf];
            #[cfg(not(feature = "pdf"))]
            let binary = vec![ExportFormat::Docx, ExportFormat::Epub];
            for f in binary {
                let path = dir.join(format!("{}-{f:?}", p.id));
                let stats = render_to_file(
                    &req(&g, &[100, 101, 102], &p, f),
                    &path,
                    &|_| {},
                    &AtomicBool::new(false),
                )
                .unwrap_or_else(|e| panic!("{} / {f:?}: {e:#}", p.id));
                assert!(
                    std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) > 0,
                    "{} / {f:?} wrote nothing",
                    p.id
                );
                assert!(stats.words > 0, "{} / {f:?} counted no words", p.id);
            }
        }
    }

    #[test]
    fn empty_prose_emits_no_block_separator() {
        // A Scene whose SceneText row exists but holds only whitespace must add
        // nothing at all to the assembled Djot — not even a bare separator.
        let g = gathered(
            vec![iwc(
                100,
                SR::Scene,
                "en",
                vec![c(1, ContentRole::SceneText, "   \n\n  ")],
            )],
            "en",
        );
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[100], &p, ExportFormat::Djot)).unwrap();
        assert!(
            out.trim().is_empty(),
            "empty prose must emit nothing: {out:?}"
        );
    }

    #[test]
    fn the_blank_line_gap_accounts_for_paragraph_spacing() {
        // A preset that already spaces paragraphs supplies part of the gap, so
        // the break must add only the remainder rather than doubling it.
        let g = book_with_marker("\\* \\* \\*");
        let gap = |spacing_pt: f32| {
            let mut p = preset("neutral");
            p.scene_break = SceneBreak::BlankLine;
            p.paragraph_spacing_pt = spacing_pt;
            let html =
                render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
            html.split("margin-top: ")
                .nth(1)
                .and_then(|s| s.split("px").next())
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0)
        };
        assert!(
            gap(6.0) < gap(0.0),
            "an already-spaced preset must not get the full extra gap"
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Epigraphs
    // ─────────────────────────────────────────────────────────────────────────

    /// Heading, then epigraph, then prose — CMOS §13.36's order, and the order the
    /// editor page shows, so what the writer sees is what the export writes.
    #[test]
    fn an_epigraph_renders_between_the_heading_and_the_prose() {
        let g = book_with_epigraph();
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
        let heading = out.find("Storms").expect("chapter heading");
        let epi = out.find("Salt is the only").expect("epigraph");
        let prose = out.find("The wind rose").expect("prose");
        assert!(
            heading < epi && epi < prose,
            "expected heading < epigraph < prose, got {heading}/{epi}/{prose} in:\n{out}"
        );
    }

    /// Quoted matter is not the author's word count. An epigraph that moved the total
    /// would inflate every pace goal and progress snapshot in the project, silently.
    #[test]
    fn an_epigraph_adds_no_words_to_the_manuscript() {
        let p = preset("neutral");

        let with = book_with_epigraph();
        let without = {
            let mut g = book_with_epigraph();
            g.binders[0].items[1]
                .contents
                .retain(|c| c.role != ContentRole::EpigraphText);
            g
        };

        let words = |g: &Gathered| {
            assemble(
                &req(g, &[100, 101], &p, ExportFormat::Djot),
                &|_| {},
                &AtomicBool::new(false),
            )
            .unwrap()
            .1
            .words
        };
        assert_eq!(
            words(&with),
            words(&without),
            "the epigraph must not be counted"
        );
        assert!(words(&with) > 0, "the fixture must count its real prose");
    }

    /// The preset toggle actually removes it — the clean-submission case.
    #[test]
    fn the_preset_can_drop_the_epigraph() {
        let g = book_with_epigraph();
        let mut p = preset("neutral");
        assert!(
            p.include_epigraphs,
            "epigraphs ship by default: authored matter is finished-book content"
        );

        p.include_epigraphs = false;
        let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
        assert!(!out.contains("Salt is the only"), "dropped: {out}");
        assert!(out.contains("The wind rose"), "prose stays: {out}");
    }

    /// A preset written before the field existed has no `include_epigraphs` key. Plain
    /// `#[serde(default)]` would read that absence as `false` and silently strip
    /// epigraphs from every custom style while the built-ins kept them — a divergence
    /// nobody would think to look for.
    #[test]
    fn a_preset_saved_before_the_field_existed_still_keeps_epigraphs() {
        // Exactly what one looks like: a real preset serialized, with the key that did
        // not exist yet removed.
        let mut v = serde_json::to_value(preset("neutral")).unwrap();
        let obj = v.as_object_mut().unwrap();
        assert!(
            obj.remove("include_epigraphs").is_some(),
            "the field must be serialized, or this test proves nothing"
        );

        let old: Preset = serde_json::from_value(v).expect("an older preset must still load");
        assert!(
            old.include_epigraphs,
            "absence must read as on, not as a silent opt-out"
        );
    }

    /// The first line after an epigraph carries no first-line indent (New Hart's Rule),
    /// queued through the same mechanism a scene break uses.
    #[test]
    fn the_paragraph_after_an_epigraph_is_not_indented() {
        let g = book_with_epigraph();
        let mut p = preset("neutral");
        p.first_line_indent_in = 0.5;
        let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
        let prose = out.find("The wind rose").expect("prose");
        let before = &out[..prose];
        let attrs = before.rfind('{').expect("an attribute block before the prose");
        assert!(
            before[attrs..].contains("text_indent=0"),
            "the prose after an epigraph must carry text_indent=0, got: {:?}",
            &before[attrs..]
        );
    }

    /// Switching the plain-text export to the indented walk changes every `.txt` that
    /// contains a blockquote, not only the epigraphs — a quoted letter inside a scene
    /// starts arriving indented too. That is the correct reading of a blockquote in a
    /// format with no markup, so it is pinned deliberately rather than left to be
    /// discovered as a surprise by someone whose manuscript already used one.
    #[test]
    fn the_plain_text_export_also_indents_a_blockquote_inside_ordinary_prose() {
        let g = gathered(
            vec![iwc(
                300,
                SR::Scene,
                "en",
                vec![c(
                    1,
                    ContentRole::SceneText,
                    "She unfolded it.\n\n> Come at once. Bring the key.\n\nThe hand was her \
                     mother's.",
                )],
            )],
            "en",
        );
        let p = preset("neutral");
        let txt = render_to_string(&req(&g, &[300], &p, ExportFormat::PlainText)).unwrap();

        let quoted = txt
            .lines()
            .find(|l| l.contains("Come at once"))
            .expect("the quoted letter must survive");
        assert!(
            quoted.starts_with(' '),
            "a blockquote in ordinary prose is set in too, got {quoted:?}"
        );
        for flush in ["She unfolded it.", "The hand was her"] {
            assert!(
                txt.lines()
                    .any(|l| l.contains(flush) && !l.starts_with(' ')),
                "surrounding prose must stay flush ({flush}): {txt}"
            );
        }
    }

    /// A Part or a Book has no prose of its own, so its epigraph must NOT queue an
    /// indent reset: the queue would outlive the row and land on the next row's opening
    /// paragraph, which is a different paragraph and is entitled to its indent. The
    /// binder is organisational, so nothing guarantees a heading row comes between them
    /// to clear it.
    #[test]
    fn a_proseless_rows_epigraph_does_not_suppress_the_next_rows_indent() {
        let g = gathered(
            vec![
                iwc(
                    200,
                    SR::Part,
                    "en",
                    vec![
                        c(1, ContentRole::PartTitle, "Part One"),
                        c(2, ContentRole::EpigraphText, "> A part-level epigraph."),
                    ],
                ),
                // Deliberately a bare Scene, not a ChapterScene: no structural heading
                // follows, so nothing clears a leaked queue.
                iwc(
                    201,
                    SR::Scene,
                    "en",
                    vec![c(3, ContentRole::SceneText, "The wind rose over the hills.")],
                ),
            ],
            "en",
        );
        let mut p = preset("neutral");
        p.first_line_indent_in = 0.5;
        let out = render_to_string(&req(&g, &[200, 201], &p, ExportFormat::Djot)).unwrap();

        let prose = out.find("The wind rose").expect("prose");
        let before = &out[..prose];
        let leaked = before
            .rfind('{')
            .is_some_and(|a| before[a..].contains("text_indent=0"));
        assert!(
            !leaked,
            "the Part's epigraph must not reach the next row's paragraph: {out}"
        );
    }

    /// An RTL row's epigraph must carry the direction attribute like any other prose on
    /// that row — the bug that appears the moment the epigraph is pushed straight into
    /// the buffer instead of through `push_prose`.
    #[test]
    fn an_epigraph_inherits_its_rows_direction() {
        let mut g = book_with_epigraph();
        g.binders[0].items[1].item.dict_language = language::parse_legacy_list("he");
        g.work.dict_language = language::parse_legacy_list("he");
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
        let epi = out.find("Salt is the only").expect("epigraph");
        let before = &out[..epi];
        let attrs = before.rfind('{').expect("an attribute block before the epigraph");
        assert!(
            before[attrs..].contains("direction=rtl"),
            "an RTL row's epigraph must be marked rtl, got: {:?}",
            &before[attrs..]
        );
    }

    /// `.txt` has no markup for quoted matter, so the epigraph must arrive indented —
    /// the whole reason the plain-text export uses the indented walk.
    #[test]
    fn the_plain_text_export_sets_the_epigraph_in() {
        let g = book_with_epigraph();
        let p = preset("neutral");
        let txt = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::PlainText)).unwrap();
        let line = txt
            .lines()
            .find(|l| l.contains("Salt is the only"))
            .expect("the epigraph must be in the plain text");
        assert!(
            line.starts_with(' '),
            "the epigraph line must be indented, got {line:?}"
        );
        assert!(
            txt.lines()
                .any(|l| l.contains("The wind rose") && !l.starts_with(' ')),
            "the body prose must stay flush: {txt}"
        );
    }

}
